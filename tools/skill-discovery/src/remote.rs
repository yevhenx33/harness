//! Read immutable GitHub objects without a persistent clone or working tree.
use super::*;
use std::cell::Cell;

const MAX_CALLS: usize = 128;
const MAX_SELECTED_BYTES: u64 = 32 * 1024 * 1024;
const MAX_TOTAL_READ: u64 = 48 * 1024 * 1024;

pub(super) struct Client<'a> {
    pub gh: &'a Path,
    pub repo: &'a str,
    pub cwd: &'a Path,
    calls: Cell<usize>,
    bytes_read: Cell<u64>,
    deadline: std::time::Instant,
}
impl<'a> Client<'a> {
    pub fn new(gh: &'a Path, repo: &'a str, cwd: &'a Path) -> Self {
        Self {
            gh,
            repo,
            cwd,
            calls: Cell::new(0),
            bytes_read: Cell::new(0),
            deadline: std::time::Instant::now() + Duration::from_secs(60),
        }
    }
    fn bytes(&self, path: &str, raw: bool) -> Result<Vec<u8>> {
        let next = self.calls.get() + 1;
        if next > MAX_CALLS || std::time::Instant::now() >= self.deadline {
            return Err("GitHub read budget exceeded".into());
        }
        self.calls.set(next);
        let endpoint = format!("repos/{}{}", self.repo, path);
        let remaining = MAX_TOTAL_READ
            .checked_sub(self.bytes_read.get())
            .ok_or("GitHub byte budget exceeded")?;
        let limit = std::cmp::min(remaining, MAX_PACKAGE);
        if limit == 0 {
            return Err("GitHub byte budget exceeded".into());
        }
        let bytes = if raw {
            run_limit(
                self.cwd,
                self.gh,
                &[
                    "api",
                    "-H",
                    "Accept: application/vnd.github.raw+json",
                    &endpoint,
                ],
                limit,
            )?
        } else {
            run_limit(self.cwd, self.gh, &["api", &endpoint], limit)?
        };
        self.bytes_read
            .set(self.bytes_read.get() + bytes.len() as u64);
        Ok(bytes)
    }
    fn json(&self, path: &str) -> Result<Value> {
        Ok(serde_json::from_slice(&self.bytes(path, false)?)?)
    }
    pub fn head(&self) -> Result<String> {
        let v = self.json("/git/ref/heads/main")?;
        let head = v
            .pointer("/object/sha")
            .and_then(Value::as_str)
            .ok_or("missing remote main")?;
        if !valid_sha(head) || v.pointer("/object/type").and_then(Value::as_str) != Some("commit") {
            return Err("invalid remote main".into());
        }
        Ok(head.to_owned())
    }
    pub fn next(&self, last: &str, head: &str) -> Result<Vec<String>> {
        if !valid_sha(last) || !valid_sha(head) {
            return Err("invalid comparison identity".into());
        }
        if last == head {
            return Ok(vec![]);
        }
        let v = self.json(&format!("/compare/{last}...{head}?per_page=2&page=1"))?;
        compare_page(&v, last, head)
    }
    fn tree(&self, id: &str, recursive: bool) -> Result<Value> {
        if !valid_sha(id) {
            return Err("invalid tree identity".into());
        }
        let suffix = if recursive { "?recursive=1" } else { "" };
        let v = self.json(&format!("/git/trees/{id}{suffix}"))?;
        if v.get("truncated").and_then(Value::as_bool) != Some(false) {
            return Err("GitHub tree unavailable or truncated".into());
        }
        if !valid_sha(v.get("sha").and_then(Value::as_str).unwrap_or("")) {
            return Err("invalid GitHub tree response".into());
        }
        Ok(v)
    }
    fn snapshot(&self, root: &Value) -> Result<Snapshot> {
        let skills_id = child(root, "skills", "tree")?;
        let contracts_id = child(root, "contracts", "tree")?;
        let contracts = self.tree(&contracts_id, false)?;
        if contracts.get("sha").and_then(Value::as_str) != Some(&contracts_id) {
            return Err("contracts tree identity mismatch".into());
        }
        let profile_id = child(&contracts, "skill-activation.txt", "blob")?;
        let profile_size = entries(&contracts)?
            .iter()
            .find(|x| x.get("path").and_then(Value::as_str) == Some("skill-activation.txt"))
            .and_then(|x| x.get("size"))
            .and_then(Value::as_u64)
            .ok_or("missing profile size")?;
        let profile_bytes = self.bytes(&format!("/git/blobs/{profile_id}"), true)?;
        if profile_bytes.len() as u64 != profile_size {
            return Err("profile blob size mismatch".into());
        }
        let names = profile(&profile_bytes)?;
        let skills_tree = self.tree(&skills_id, false)?;
        if skills_tree.get("sha").and_then(Value::as_str) != Some(&skills_id) {
            return Err("skills tree identity mismatch".into());
        }
        let mut skills = BTreeMap::new();
        for entry in entries(&skills_tree)? {
            if entry.get("type").and_then(Value::as_str) != Some("tree") {
                continue;
            }
            let name = entry
                .get("path")
                .and_then(Value::as_str)
                .ok_or("skill tree missing name")?;
            if name.contains('/')
                || name.is_empty()
                || !name
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            {
                return Err("invalid skill tree name".into());
            }
            let id = entry
                .get("sha")
                .and_then(Value::as_str)
                .ok_or("skill tree missing SHA")?;
            if !valid_sha(id) || skills.insert(name.to_owned(), id.to_owned()).is_some() {
                return Err("invalid or duplicate skill tree".into());
            }
        }
        Ok(Snapshot { names, skills })
    }
    fn package(&self, tree_id: &str) -> Result<BTreeMap<std::path::PathBuf, Vec<u8>>> {
        let tree = self.tree(tree_id, true)?;
        if tree.get("sha").and_then(Value::as_str) != Some(tree_id) {
            return Err("skill tree identity mismatch".into());
        }
        let rows = entries(&tree)?;
        if rows.len() > 256 {
            return Err("skill package entry limit exceeded".into());
        }
        let mut files = BTreeMap::new();
        let mut total = 0u64;
        for entry in rows {
            let path = entry
                .get("path")
                .and_then(Value::as_str)
                .ok_or("missing package path")?;
            let rel = std::path::PathBuf::from(path);
            if rel.is_absolute()
                || rel.components().count() > 16
                || rel
                    .components()
                    .any(|c| !matches!(c, std::path::Component::Normal(_)))
            {
                return Err("invalid package path".into());
            }
            let kind = entry
                .get("type")
                .and_then(Value::as_str)
                .ok_or("missing package type")?;
            if kind == "tree" {
                continue;
            }
            let mode = entry
                .get("mode")
                .and_then(Value::as_str)
                .ok_or("missing package mode")?;
            let id = entry
                .get("sha")
                .and_then(Value::as_str)
                .ok_or("missing package blob")?;
            let size = entry
                .get("size")
                .and_then(Value::as_u64)
                .ok_or("missing package size")?;
            total = total.checked_add(size).ok_or("package size overflow")?;
            if kind != "blob"
                || !matches!(mode, "100644" | "100755")
                || !valid_sha(id)
                || total > MAX_PACKAGE
            {
                return Err("unsupported or oversized package entry".into());
            }
            let bytes = self.bytes(&format!("/git/blobs/{id}"), true)?;
            if bytes.len() as u64 != size || files.insert(rel, bytes).is_some() {
                return Err("package blob size mismatch or duplicate path".into());
            }
        }
        if !files.contains_key(Path::new("SKILL.md")) {
            return Err("committed skill missing SKILL.md".into());
        }
        Ok(files)
    }
    pub fn check(
        &self,
        commit: &str,
        cwd: &Path,
        installed: &Path,
        codex: &Path,
    ) -> Result<Vec<String>> {
        if !valid_sha(commit) {
            return Err("invalid commit".into());
        }
        let data = self.json(&format!("/git/commits/{commit}"))?;
        if data.get("sha").and_then(Value::as_str) != Some(commit) {
            return Err("commit identity mismatch".into());
        }
        let parents = data
            .get("parents")
            .and_then(Value::as_array)
            .ok_or("missing commit parents")?;
        let parent = parents
            .first()
            .and_then(|v| v.get("sha"))
            .and_then(Value::as_str)
            .ok_or("missing first parent")?;
        if !valid_sha(parent) {
            return Err("invalid first parent".into());
        }
        let old_root = self.tree(parent, false)?;
        let new_root = self.tree(commit, false)?;
        // Most merges do not touch either input. Git tree identities let us
        // skip profile, package and consumer reads for those merges.
        if child(&old_root, "skills", "tree")? == child(&new_root, "skills", "tree")?
            && child(&old_root, "contracts", "tree")? == child(&new_root, "contracts", "tree")?
        {
            return Ok(vec![]);
        }
        let old = self.snapshot(&old_root)?;
        let new = self.snapshot(&new_root)?;
        let mut selected: BTreeSet<String> = new.names.difference(&old.names).cloned().collect();
        for (name, id) in &new.skills {
            if old.skills.get(name) != Some(id) {
                if new.names.contains(name) {
                    selected.insert(name.clone());
                } else {
                    let t = self.tree(id, false)?;
                    let has_skill = entries(&t)?
                        .iter()
                        .any(|v| v.get("path").and_then(Value::as_str) == Some("SKILL.md"));
                    let had_skill = match old.skills.get(name) {
                        Some(old_id) if has_skill => entries(&self.tree(old_id, false)?)?
                            .iter()
                            .any(|v| v.get("path").and_then(Value::as_str) == Some("SKILL.md")),
                        _ => false,
                    };
                    if has_skill && !had_skill {
                        return Err(Unprofiled(name.clone()).into());
                    }
                }
            }
        }
        let names: Vec<String> = selected.into_iter().collect();
        if names.is_empty() {
            return Ok(names);
        }
        let cwd = fs::canonicalize(cwd)?;
        let installed = fs::canonicalize(installed)?;
        let mut packages = BTreeMap::new();
        let mut total = 0u64;
        for name in &names {
            let tree = new
                .skills
                .get(name)
                .ok_or("profiled skill source missing")?;
            let source = self.package(tree)?;
            total += source.values().map(|v| v.len() as u64).sum::<u64>();
            if total > MAX_SELECTED_BYTES {
                return Err("selected packages exceed memory budget".into());
            }
            let actual = release::package(&installed.join(name))
                .map_err(|e| format!("installed skill unavailable or invalid ({name}): {e}"))?;
            if source != actual {
                return Err(format!("installed skill differs from merged commit: {name}").into());
            }
            packages.insert(name.clone(), source);
        }
        discover(&cwd, &installed, &names, codex, TIMEOUT)?;
        for name in &names {
            if packages.get(name) != Some(&release::package(&installed.join(name))?) {
                return Err(format!("installed skill changed during discovery: {name}").into());
            }
        }
        Ok(names)
    }
}
struct Snapshot {
    names: BTreeSet<String>,
    skills: BTreeMap<String, String>,
}
fn valid_sha(s: &str) -> bool {
    s.len() == 40
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub(super) fn valid_repo(s: &str) -> bool {
    let parts: Vec<_> = s.split('/').collect();
    parts.len() == 2
        && parts.iter().all(|x| {
            !x.is_empty()
                && !x.starts_with('-')
                && !x.contains("..")
                && x.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        })
}
fn entries(v: &Value) -> Result<&Vec<Value>> {
    v.get("tree")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing tree entries".into())
}
fn child(v: &Value, name: &str, kind: &str) -> Result<String> {
    let entries: Vec<_> = entries(v)?
        .iter()
        .filter(|x| x.get("path").and_then(Value::as_str) == Some(name))
        .collect();
    if entries.len() != 1 || entries[0].get("type").and_then(Value::as_str) != Some(kind) {
        return Err(format!("missing or invalid tree child: {name}").into());
    }
    let mode = entries[0].get("mode").and_then(Value::as_str).unwrap_or("");
    if (kind == "tree" && mode != "040000")
        || (kind == "blob" && !matches!(mode, "100644" | "100755"))
    {
        return Err("unsupported tree child mode".into());
    }
    let id = entries[0]
        .get("sha")
        .and_then(Value::as_str)
        .ok_or("missing tree child SHA")?;
    if !valid_sha(id) {
        return Err("invalid tree child SHA".into());
    }
    Ok(id.to_owned())
}
fn compare_page(v: &Value, last: &str, head: &str) -> Result<Vec<String>> {
    if v.pointer("/merge_base_commit/sha").and_then(Value::as_str) != Some(last) {
        return Err("main history diverged from checkpoint".into());
    }
    let rows = v
        .get("commits")
        .and_then(Value::as_array)
        .ok_or("missing comparison commits")?;
    if rows.is_empty() || rows.len() > 2 {
        return Err("comparison unavailable or malformed".into());
    }
    let mut previous = last.to_owned();
    let mut commits = Vec::new();
    for row in rows {
        let id = row
            .get("sha")
            .and_then(Value::as_str)
            .ok_or("missing comparison SHA")?;
        let parents = row
            .get("parents")
            .and_then(Value::as_array)
            .ok_or("missing comparison parents")?;
        if !valid_sha(id)
            || parents.len() != 1
            || parents[0].get("sha").and_then(Value::as_str) != Some(&previous)
        {
            return Err("nonlinear or incomplete main history".into());
        }
        previous = id.to_owned();
        commits.push(id.to_owned());
    }
    let total = v
        .get("total_commits")
        .and_then(Value::as_u64)
        .ok_or("missing total commit count")?;
    if rows.len() != std::cmp::min(total, 2) as usize
        || (total == commits.len() as u64 && commits.last().map(String::as_str) != Some(head))
    {
        return Err("comparison count or head mismatch".into());
    }
    Ok(commits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    #[test]
    fn paginated_comparison_preserves_order_and_rejects_gaps() {
        let a = "a".repeat(40);
        let b = "b".repeat(40);
        let c = "c".repeat(40);
        let d = "d".repeat(40);
        let valid = json!({"merge_base_commit":{"sha":a},"total_commits":3,"commits":[{"sha":b,"parents":[{"sha":a}]},{"sha":c,"parents":[{"sha":b}]}]});
        assert_eq!(
            compare_page(&valid, &a, &d).unwrap(),
            vec![b.clone(), c.clone()]
        );
        let mut gap = valid.clone();
        gap["commits"][1]["parents"][0]["sha"] = json!(a);
        assert!(compare_page(&gap, &a, &d).is_err());
        let mut truncated = valid.clone();
        truncated["commits"].as_array_mut().unwrap().pop();
        assert!(compare_page(&truncated, &a, &d).is_err());
        let mut diverged = valid.clone();
        diverged["merge_base_commit"]["sha"] = json!(d);
        assert!(compare_page(&diverged, &a, &d).is_err());
    }
    #[test]
    fn rejects_ambiguous_or_malformed_tree_children() {
        let a = "a".repeat(40);
        let valid = json!({"tree":[{"path":"skills","type":"tree","mode":"040000","sha":a}]});
        assert_eq!(child(&valid, "skills", "tree").unwrap(), a);
        let duplicate = json!({"tree":[{"path":"skills","type":"tree","mode":"040000","sha":a},{"path":"skills","type":"tree","mode":"040000","sha":a}]});
        assert!(child(&duplicate, "skills", "tree").is_err());
        assert!(child(&valid, "skills", "blob").is_err());
        assert!(!valid_repo("a/../b"));
    }

    #[test]
    fn adding_skill_to_existing_source_directory_requires_profile() {
        let dir = env::temp_dir().join(format!(
            "harness-remote-skill-fixture-{}",
            std::process::id()
        ));
        fs::create_dir(&dir).unwrap();
        let ids: Vec<String> = "abcdef123"
            .chars()
            .map(|c| c.to_string().repeat(40))
            .collect();
        let (
            parent,
            commit,
            old_root,
            new_root,
            old_skills,
            new_skills,
            contracts,
            profile_id,
            folder,
        ) = (
            &ids[0], &ids[1], &ids[2], &ids[3], &ids[4], &ids[5], &ids[6], &ids[7], &ids[8],
        );
        let tree = |sha: &str, entries: Value| json!({"sha":sha,"truncated":false,"tree":entries});
        let child = |name: &str, kind: &str, sha: &str| json!({"path":name,"type":kind,"mode":if kind == "tree" {"040000"} else {"100644"},"sha":sha,"size":4});
        let fixture = [
            (
                format!("/git/commits/{commit}"),
                json!({"sha":commit,"parents":[{"sha":parent}]}),
            ),
            (
                format!("/git/trees/{parent}"),
                tree(
                    old_root,
                    json!([
                        child("skills", "tree", old_skills),
                        child("contracts", "tree", contracts)
                    ]),
                ),
            ),
            (
                format!("/git/trees/{commit}"),
                tree(
                    new_root,
                    json!([
                        child("skills", "tree", new_skills),
                        child("contracts", "tree", contracts)
                    ]),
                ),
            ),
            (
                format!("/git/trees/{contracts}"),
                tree(
                    contracts,
                    json!([child("skill-activation.txt", "blob", profile_id)]),
                ),
            ),
            (
                format!("/git/trees/{old_skills}"),
                tree(old_skills, json!([child("new", "tree", folder)])),
            ),
            (
                format!("/git/trees/{new_skills}"),
                tree(new_skills, json!([child("new", "tree", new_root)])),
            ),
            (
                format!("/git/trees/{folder}"),
                tree(folder, json!([child("README.md", "blob", profile_id)])),
            ),
            (
                format!("/git/trees/{new_root}"),
                tree(new_root, json!([child("SKILL.md", "blob", profile_id)])),
            ),
        ];
        let mut script = String::from("#!/bin/sh\nfor arg do case \"$arg\" in repos/*) endpoint=\"$arg\";; esac; done\ncase \"$endpoint\" in\n");
        for (endpoint, response) in fixture {
            script.push_str(&format!(
                "  repos/owner/repo{endpoint}) printf '%s\\n' '{}' ;;\n",
                response
            ));
        }
        script.push_str(&format!(
            "  repos/owner/repo/git/blobs/{profile_id}) printf 'old\\n' ;;\n  *) exit 1 ;;\nesac\n"
        ));
        let gh = dir.join("gh");
        fs::write(&gh, script).unwrap();
        fs::set_permissions(&gh, fs::Permissions::from_mode(0o700)).unwrap();
        let result =
            Client::new(&gh, "owner/repo", &dir).check(commit, &dir, &dir, Path::new("unused"));
        assert!(result.unwrap_err().downcast_ref::<Unprofiled>().is_some());
        fs::remove_dir_all(dir).unwrap();
    }
}
