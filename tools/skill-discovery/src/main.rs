//! Local, read-only check of changed Harness skills against a fresh Codex listing.
#[cfg(unix)]
mod monitor;
#[allow(dead_code)]
#[path = "../../../scripts/verify_release.rs"]
mod release;
#[cfg(unix)]
mod remote;

use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    io::{BufRead, BufReader, Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    sync::mpsc,
    time::Duration,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const MAX_RESPONSE: u64 = 8 * 1024 * 1024;
const MAX_PACKAGE: u64 = 16 * 1024 * 1024;
const TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Debug)]
struct Unprofiled(String);
impl std::fmt::Display for Unprofiled {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "new skill is not in the activation profile: {}", self.0)
    }
}
impl std::error::Error for Unprofiled {}

#[cfg(unix)]
fn isolate(command: &mut Command) {
    use std::os::unix::process::CommandExt;
    command.process_group(0);
}
#[cfg(not(unix))]
fn isolate(_: &mut Command) {}

fn stop(child: &mut Child) {
    #[cfg(unix)]
    {
        extern "C" {
            fn kill(pid: i32, signal: i32) -> i32;
        }
        // Every child started here owns a new process group.
        let _ = unsafe { kill(-(child.id() as i32), 9) };
    }
    let _ = child.kill();
    let _ = child.wait();
}

fn run(root: &Path, binary: &Path, args: &[&str]) -> Result<Vec<u8>> {
    run_limit(root, binary, args, MAX_PACKAGE)
}

fn run_limit(root: &Path, binary: &Path, args: &[&str], limit: u64) -> Result<Vec<u8>> {
    let deadline = std::time::Instant::now() + TIMEOUT;
    let mut command = Command::new(binary);
    command
        .args(args)
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env("GIT_TERMINAL_PROMPT", "0")
        .env("GH_PROMPT_DISABLED", "1")
        .current_dir(root)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    isolate(&mut command);
    let mut child = command.spawn()?;
    let stdout = child.stdout.take().ok_or("missing git stdout")?;
    let (tx, rx) = mpsc::sync_channel(1);
    std::thread::spawn(move || {
        let mut bytes = Vec::new();
        let result = stdout
            .take(limit + 1)
            .read_to_end(&mut bytes)
            .map(|_| bytes);
        let _ = tx.send(result);
    });
    let out = match rx.recv_timeout(deadline.saturating_duration_since(std::time::Instant::now())) {
        Ok(Ok(out)) => out,
        _ => {
            stop(&mut child);
            return Err("command output unavailable or timed out".into());
        }
    };
    if out.len() > limit as usize {
        stop(&mut child);
        return Err("command output exceeded limit".into());
    }
    loop {
        if let Some(status) = child.try_wait()? {
            if !status.success() {
                return Err(format!("command failed: {status}").into());
            }
            return Ok(out);
        }
        if std::time::Instant::now() >= deadline {
            stop(&mut child);
            return Err("command exit timed out".into());
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let mut full = vec!["--no-replace-objects"];
    full.extend_from_slice(args);
    run(root, Path::new("git"), &full)
}

fn profile(bytes: &[u8]) -> Result<BTreeSet<String>> {
    let mut names = BTreeSet::new();
    for line in std::str::from_utf8(bytes)?.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if !line
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
            || !names.insert(line.to_owned())
        {
            return Err("invalid or duplicate activation profile entry".into());
        }
    }
    if names.is_empty() {
        return Err("empty activation profile".into());
    }
    Ok(names)
}

fn released(root: &Path, commit: &str) -> Result<Vec<String>> {
    if commit.len() != 40 || !commit.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("provide the exact 40-character merged commit".into());
    }
    let resolved = git(
        root,
        &["rev-parse", "--verify", &format!("{commit}^{{commit}}")],
    )?;
    if std::str::from_utf8(&resolved)?.trim() != commit {
        return Err("merged commit identity mismatch".into());
    }
    git(
        root,
        &[
            "merge-base",
            "--is-ancestor",
            commit,
            "refs/remotes/origin/main",
        ],
    )?;
    let parent = format!("{commit}^1");
    let old = profile(&git(
        root,
        &["show", &format!("{parent}:contracts/skill-activation.txt")],
    )?)?;
    let new = profile(&git(
        root,
        &["show", &format!("{commit}:contracts/skill-activation.txt")],
    )?)?;
    let mut selected: BTreeSet<String> = new.difference(&old).cloned().collect();
    let mut unprofiled = BTreeSet::new();
    let diff = git(
        root,
        &[
            "diff",
            "--name-only",
            "-z",
            "--no-renames",
            &parent,
            commit,
            "--",
            "skills",
        ],
    )?;
    for path in diff.split(|b| *b == 0).filter(|p| !p.is_empty()) {
        let path = std::str::from_utf8(path)?;
        let mut parts = path.split('/');
        if parts.next() == Some("skills") {
            if let Some(name) = parts.next() {
                if new.contains(name) {
                    selected.insert(name.to_owned());
                } else {
                    unprofiled.insert(name.to_owned());
                }
            }
        }
    }
    for name in unprofiled {
        let skill = format!("skills/{name}/SKILL.md");
        let before = git(root, &["ls-tree", "-r", "-z", &parent, "--", &skill])?;
        let after = git(root, &["ls-tree", "-r", "-z", commit, "--", &skill])?;
        if before.is_empty() && !after.is_empty() {
            return Err(Unprofiled(name).into());
        }
        eprintln!("existing source-only or retired skill not in activation profile: {name}");
    }
    Ok(selected.into_iter().collect())
}

fn committed_package(
    root: &Path,
    commit: &str,
    name: &str,
) -> Result<BTreeMap<std::path::PathBuf, Vec<u8>>> {
    let path = format!("skills/{name}/");
    let tree = git(
        root,
        &["ls-tree", "-r", "-z", "--full-tree", commit, "--", &path],
    )?;
    let mut files = BTreeMap::new();
    let mut total = 0;
    for entry in tree.split(|b| *b == 0).filter(|e| !e.is_empty()) {
        let tab = entry
            .iter()
            .position(|b| *b == b'\t')
            .ok_or("invalid git tree entry")?;
        let (header, full) = (&entry[..tab], &entry[tab + 1..]);
        let header = std::str::from_utf8(header)?;
        let mut parts = header.split(' ');
        let mode = parts.next().ok_or("missing git mode")?;
        let kind = parts.next().ok_or("missing git object kind")?;
        let oid = parts.next().ok_or("missing git object ID")?;
        if !matches!(mode, "100644" | "100755")
            || kind != "blob"
            || parts.next().is_some()
            || oid.len() != 40
            || !oid.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(format!("unsupported committed package entry: {name}").into());
        }
        let full = std::str::from_utf8(full)?;
        let relative = full.strip_prefix(&path).ok_or("unexpected git tree path")?;
        let relative = std::path::PathBuf::from(relative);
        if relative.components().count() > 16 || files.len() >= 256 {
            return Err("committed skill package entry limit exceeded".into());
        }
        let bytes = git(root, &["cat-file", "blob", oid])?;
        total += bytes.len() as u64;
        if total > MAX_PACKAGE || files.insert(relative, bytes).is_some() {
            return Err("committed skill package byte limit or duplicate path".into());
        }
    }
    if !files.contains_key(Path::new("SKILL.md")) {
        return Err(format!("missing committed skill: {name}").into());
    }
    Ok(files)
}

fn package_matches_commit(root: &Path, commit: &str, installed: &Path, name: &str) -> Result<()> {
    let target = release::package(&installed.join(name))
        .map_err(|e| format!("installed skill unavailable or invalid ({name}): {e}"))?;
    if committed_package(root, commit, name)? != target {
        return Err(format!("installed skill differs from merged commit: {name}").into());
    }
    Ok(())
}

struct Server {
    child: Child,
    lines: mpsc::Receiver<std::result::Result<Value, String>>,
    timeout: Duration,
}

impl Server {
    fn start(binary: &Path, timeout: Duration) -> Result<Self> {
        let mut command = Command::new(binary);
        command
            .args(["app-server", "--listen", "stdio://"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null());
        isolate(&mut command);
        let mut child = command.spawn()?;
        let stdout = child.stdout.take().ok_or("missing server stdout")?;
        let (tx, lines) = mpsc::channel();
        std::thread::spawn(move || {
            let mut reader = BufReader::new(stdout.take(MAX_RESPONSE + 1));
            let mut total = 0;
            loop {
                let mut line = Vec::new();
                match reader.read_until(b'\n', &mut line) {
                    Ok(0) => break,
                    Ok(n) => {
                        total += n as u64;
                        let item = if total > MAX_RESPONSE || !line.ends_with(b"\n") {
                            Err("app-server output exceeded limit or was truncated".to_owned())
                        } else {
                            serde_json::from_slice(&line)
                                .map_err(|_| "malformed app-server JSON".to_owned())
                        };
                        let stop = item.is_err();
                        if tx.send(item).is_err() || stop {
                            break;
                        }
                    }
                    Err(_) => {
                        let _ = tx.send(Err("app-server read failed".to_owned()));
                        break;
                    }
                }
            }
        });
        Ok(Self {
            child,
            lines,
            timeout,
        })
    }

    fn send(&mut self, value: Value) -> Result<()> {
        let stdin = self.child.stdin.as_mut().ok_or("missing server stdin")?;
        serde_json::to_writer(&mut *stdin, &value)?;
        stdin.write_all(b"\n")?;
        stdin.flush()?;
        Ok(())
    }

    fn response(&self, id: u64) -> Result<Value> {
        let deadline = std::time::Instant::now() + self.timeout;
        loop {
            let remaining = deadline
                .checked_duration_since(std::time::Instant::now())
                .ok_or("app-server response timeout")?;
            let value = self
                .lines
                .recv_timeout(remaining)
                .map_err(|_| "app-server response unavailable or timed out")??;
            if value.get("id").and_then(Value::as_u64) != Some(id) {
                continue;
            }
            if value.get("error").is_some() {
                return Err("app-server returned an error".into());
            }
            return value
                .get("result")
                .cloned()
                .ok_or_else(|| "app-server result missing".into());
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        stop(&mut self.child);
    }
}

fn validate_listing(listing: &Value, cwd: &Path, installed: &Path, names: &[String]) -> Result<()> {
    let rows = listing
        .get("data")
        .and_then(Value::as_array)
        .ok_or("missing listing data")?;
    let cwd = cwd.to_str().ok_or("non-UTF-8 working directory")?;
    let matches: Vec<_> = rows
        .iter()
        .filter(|r| r.get("cwd").and_then(Value::as_str) == Some(cwd))
        .collect();
    if matches.len() != 1 {
        return Err("missing or duplicate cwd listing".into());
    }
    let row = matches[0];
    if !row
        .get("errors")
        .and_then(Value::as_array)
        .ok_or("missing listing errors")?
        .is_empty()
    {
        return Err("Codex reported skill discovery errors".into());
    }
    let skills = row
        .get("skills")
        .and_then(Value::as_array)
        .ok_or("missing skill listing")?;
    for name in names {
        let entries: Vec<_> = skills
            .iter()
            .filter(|s| s.get("name").and_then(Value::as_str) == Some(name))
            .collect();
        if entries.len() != 1 {
            return Err(format!("skill missing or ambiguous: {name}").into());
        }
        let entry = entries[0];
        if entry.get("enabled").and_then(Value::as_bool) != Some(true)
            || entry.get("scope").and_then(Value::as_str) != Some("user")
        {
            return Err(format!("skill not enabled in user scope: {name}").into());
        }
        let path = entry
            .get("path")
            .and_then(Value::as_str)
            .ok_or("missing discovered path")?;
        let expected = fs::canonicalize(installed.join(name).join("SKILL.md"))?;
        if fs::canonicalize(path)? != expected {
            return Err(format!("unexpected skill path: {name}").into());
        }
    }
    Ok(())
}

fn check(
    root: &Path,
    commit: &str,
    cwd: &Path,
    installed: &Path,
    binary: &Path,
    timeout: Duration,
) -> Result<Vec<String>> {
    let names = released(root, commit)?;
    if names.is_empty() {
        return Ok(names);
    }
    let cwd = fs::canonicalize(cwd)?;
    let installed = fs::canonicalize(installed)?;
    for name in &names {
        package_matches_commit(root, commit, &installed, name)?;
    }
    discover(&cwd, &installed, &names, binary, timeout)?;
    for name in &names {
        package_matches_commit(root, commit, &installed, name)?;
    }
    Ok(names)
}

fn discover(
    cwd: &Path,
    installed: &Path,
    names: &[String],
    binary: &Path,
    timeout: Duration,
) -> Result<()> {
    let mut server = Server::start(binary, timeout)?;
    server.send(json!({"id": 1, "method": "initialize", "params": {"clientInfo": {"name": "harness-skill-check", "version": "1"}}}))?;
    server.response(1)?;
    server.send(json!({"method": "initialized", "params": {}}))?;
    server.send(
        json!({"id": 2, "method": "skills/list", "params": {"cwds": [cwd], "forceReload": true}}),
    )?;
    validate_listing(&server.response(2)?, cwd, installed, names)
}

fn main() {
    let args: Vec<_> = env::args_os().collect();
    #[cfg(unix)]
    if args.len() == 3 && args[1] == "monitor" {
        if let Err(e) = monitor::tick(Path::new(&args[2]), Path::new("codex"), Path::new("gh")) {
            eprintln!("monitor incomplete: {e}");
            std::process::exit(1);
        }
        return;
    }
    #[cfg(unix)]
    if args.len() == 6 && args[1] == "remote-check" {
        let repo = args[2].to_string_lossy();
        let commit = args[3].to_string_lossy();
        let result = (|| -> Result<Vec<String>> {
            if !remote::valid_repo(&repo) {
                return Err("invalid repository".into());
            }
            let host_cwd = env::current_dir()?;
            let api = remote::Client::new(Path::new("gh"), &repo, &host_cwd);
            let head = api.head()?;
            api.next(&commit, &head)?;
            api.check(
                &commit,
                Path::new(&args[4]),
                Path::new(&args[5]),
                Path::new("codex"),
            )
        })();
        match result {
            Ok(names) if names.is_empty() => println!("no changed user-level skill packages"),
            Ok(names) => println!("local discovery verified: {}", names.join(", ")),
            Err(e) => {
                eprintln!("local discovery not verified: {e}");
                std::process::exit(1);
            }
        }
        return;
    }
    if args.len() != 5 {
        eprintln!("usage: harness-skill-discovery REPO MERGED_COMMIT CWD INSTALLED_SKILLS_ROOT");
        std::process::exit(2);
    }
    let commit = args[2].to_string_lossy();
    match check(
        Path::new(&args[1]),
        &commit,
        Path::new(&args[3]),
        Path::new(&args[4]),
        Path::new("codex"),
        TIMEOUT,
    ) {
        Ok(names) if names.is_empty() => println!("no changed user-level skill packages"),
        Ok(names) => println!("local discovery verified: {}", names.join(", ")),
        Err(e) => {
            eprintln!("local discovery not verified: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::{
        os::unix::fs::PermissionsExt,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);

    struct Fixture {
        root: PathBuf,
        commit: String,
        installed: PathBuf,
        binary: PathBuf,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    fn run_git(root: &Path, args: &[&str]) -> String {
        let out = Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap().trim().to_owned()
    }
    fn fixture() -> Fixture {
        let root = env::temp_dir().join(format!(
            "harness-skill-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir(&root).unwrap();
        run_git(&root, &["init", "-q"]);
        run_git(&root, &["config", "user.name", "Fixture"]);
        run_git(&root, &["config", "user.email", "fixture@example.invalid"]);
        fs::create_dir_all(root.join("contracts")).unwrap();
        fs::write(root.join("contracts/skill-activation.txt"), "old\n").unwrap();
        run_git(&root, &["add", "."]);
        run_git(&root, &["commit", "-qm", "base"]);
        fs::write(root.join("contracts/skill-activation.txt"), "old\nnew\n").unwrap();
        fs::create_dir_all(root.join("skills/new")).unwrap();
        fs::write(
            root.join("skills/new/SKILL.md"),
            "---\nname: new\ndescription: test\n---\n",
        )
        .unwrap();
        run_git(&root, &["add", "."]);
        run_git(&root, &["commit", "-qm", "release"]);
        let commit = run_git(&root, &["rev-parse", "HEAD"]);
        run_git(&root, &["update-ref", "refs/remotes/origin/main", &commit]);
        let installed = root.join("installed");
        fs::create_dir_all(installed.join("new")).unwrap();
        fs::copy(
            root.join("skills/new/SKILL.md"),
            installed.join("new/SKILL.md"),
        )
        .unwrap();
        let binary = root.join("fake-codex");
        Fixture {
            root,
            commit,
            installed,
            binary,
        }
    }
    fn fake(f: &Fixture, outcome: &str) {
        let path = f.installed.join("new/SKILL.md");
        let mut skills = vec![json!({"name":"new","path":path,"enabled":true,"scope":"user"})];
        let mut errors = vec![];
        match outcome {
            "missing" => skills.clear(),
            "disabled" => skills[0]["enabled"] = json!(false),
            "duplicate" => skills.push(skills[0].clone()),
            "wrong-path" => skills[0]["path"] = json!(f.root.join("skills/new/SKILL.md")),
            "wrong-scope" => skills[0]["scope"] = json!("repo"),
            "errors" => errors.push(json!({"path":"x","message":"failed"})),
            _ => (),
        }
        let response =
            json!({"id":2,"result":{"data":[{"cwd":f.root,"errors":errors,"skills":skills}]}})
                .to_string();
        let script = match outcome {
            "malformed" => "#!/bin/sh\nread x\nprintf 'not-json\\n'\n".to_owned(),
            "timeout" => "#!/bin/sh\nread x\nexec sleep 5\n".to_owned(),
            _ => format!("#!/bin/sh\nread x\nprintf '%s\\n' '{{\"id\":1,\"result\":{{}}}}'\nread x\nread x\nprintf '%s\\n' '{}'\n", response),
        };
        fs::write(&f.binary, script).unwrap();
        fs::set_permissions(&f.binary, fs::Permissions::from_mode(0o700)).unwrap();
    }
    fn execute(f: &Fixture) -> Result<Vec<String>> {
        check(
            &f.root,
            &f.commit,
            &f.root,
            &f.installed,
            &f.binary,
            Duration::from_millis(500),
        )
    }

    #[test]
    fn release_detects_package_and_validates_fresh_listing() {
        let f = fixture();
        fake(&f, "ok");
        assert_eq!(execute(&f).unwrap(), vec!["new"]);
    }

    #[test]
    fn missing_disabled_duplicate_wrong_path_and_discovery_errors_fail() {
        for outcome in [
            "missing",
            "disabled",
            "duplicate",
            "wrong-path",
            "wrong-scope",
            "errors",
            "malformed",
            "timeout",
        ] {
            let f = fixture();
            fake(&f, outcome);
            assert!(execute(&f).is_err(), "{outcome}");
        }
    }

    #[test]
    fn missing_stale_and_extra_installed_packages_fail() {
        let f = fixture();
        fake(&f, "ok");
        fs::remove_file(f.installed.join("new/SKILL.md")).unwrap();
        assert!(execute(&f).is_err());
        fs::write(f.installed.join("new/SKILL.md"), "stale").unwrap();
        assert!(execute(&f).is_err());
        fs::copy(
            f.root.join("skills/new/SKILL.md"),
            f.installed.join("new/SKILL.md"),
        )
        .unwrap();
        fs::write(f.installed.join("new/extra"), "extra").unwrap();
        assert!(execute(&f).is_err());
        fs::remove_file(f.installed.join("new/extra")).unwrap();
        fs::write(f.root.join("skills/new/SKILL.md"), "dirty").unwrap();
        assert_eq!(execute(&f).unwrap(), vec!["new"]);
        fs::write(f.root.join("skills/new/untracked"), "extra").unwrap();
        assert_eq!(execute(&f).unwrap(), vec!["new"]);
    }

    #[test]
    fn rejects_unrelated_main_and_invalid_commit() {
        let f = fixture();
        fake(&f, "ok");
        assert!(check(
            &f.root,
            "not-a-commit",
            &f.root,
            &f.installed,
            &f.binary,
            Duration::from_millis(500)
        )
        .is_err());
        let parent = run_git(&f.root, &["rev-parse", "HEAD^1"]);
        run_git(
            &f.root,
            &["update-ref", "refs/remotes/origin/main", &parent],
        );
        assert!(execute(&f).is_err());
    }

    #[test]
    fn unchanged_release_does_not_require_codex() {
        let f = fixture();
        fs::write(f.root.join("note"), "not a skill").unwrap();
        run_git(&f.root, &["add", "note"]);
        run_git(&f.root, &["commit", "-qm", "documentation"]);
        let commit = run_git(&f.root, &["rev-parse", "HEAD"]);
        run_git(
            &f.root,
            &["update-ref", "refs/remotes/origin/main", &commit],
        );
        assert!(check(
            &f.root,
            &commit,
            &f.root,
            &f.root.join("missing-skills"),
            &f.binary,
            Duration::from_millis(500)
        )
        .unwrap()
        .is_empty());
    }
}
