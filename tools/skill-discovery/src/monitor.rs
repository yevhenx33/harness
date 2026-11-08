//! A bounded, one-shot poller. Scheduling and activation are external.
use super::*;
use std::{
    fs::OpenOptions,
    os::unix::{
        fs::{MetadataExt, OpenOptionsExt},
        io::AsRawFd,
    },
    path::PathBuf,
};

const CONTEXT: &str = "harness/local-skill-discovery";
const MAX_PENDING: usize = 128;
const MAX_PER_RUN: usize = 2;

extern "C" {
    fn flock(fd: i32, operation: i32) -> i32;
    fn geteuid() -> u32;
}

struct Lock(fs::File);
impl Lock {
    fn acquire(dir: &Path) -> Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .open(dir.join("monitor.lock"))?;
        // The descriptor releases the kernel lock even if the process crashes.
        if unsafe { flock(file.as_raw_fd(), 2 | 4) } != 0 {
            return Err("monitor already running or lock unavailable".into());
        }
        Ok(Self(file))
    }
}
impl Drop for Lock {
    fn drop(&mut self) {
        let _ = unsafe { flock(self.0.as_raw_fd(), 8) };
    }
}

struct Config {
    repo: String,
    actor: String,
    state_dir: PathBuf,
    cwd: PathBuf,
    installed: PathBuf,
    baseline: String,
}

fn sha(s: &str) -> bool {
    s.len() == 40
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn component(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with('-')
        && !s.contains("..")
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}
fn bounded_json(path: &Path) -> Result<Value> {
    let meta = fs::symlink_metadata(path)?;
    if !meta.file_type().is_file()
        || meta.len() > 64 * 1024
        || meta.uid() != unsafe { geteuid() }
        || meta.mode() & 0o022 != 0
    {
        return Err("invalid or oversized monitor file".into());
    }
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
fn field<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    v.get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing configuration field: {key}").into())
}
impl Config {
    fn load(path: &Path) -> Result<Self> {
        let v = bounded_json(path)?;
        if v.get("version").and_then(Value::as_u64) != Some(1) {
            return Err("unsupported monitor configuration".into());
        }
        let repo = field(&v, "repo")?.to_owned();
        let parts: Vec<_> = repo.split('/').collect();
        let actor = field(&v, "actor")?.to_owned();
        let baseline = field(&v, "baseline")?.to_owned();
        if parts.len() != 2
            || !parts.iter().all(|s| component(s))
            || !component(&actor)
            || !sha(&baseline)
        {
            return Err("invalid repository, actor, or baseline".into());
        }
        let path = |key| -> Result<PathBuf> {
            let p = PathBuf::from(field(&v, key)?);
            if !p.is_absolute() {
                return Err(format!("{key} must be absolute").into());
            }
            Ok(p)
        };
        let state_dir = fs::canonicalize(path("state_dir")?)?;
        let meta = fs::metadata(&state_dir)?;
        if !meta.is_dir() || meta.uid() != unsafe { geteuid() } || meta.mode() & 0o077 != 0 {
            return Err("state directory must be private and user-owned".into());
        }
        Ok(Self {
            repo,
            actor,
            baseline,
            state_dir,
            cwd: path("cwd")?,
            installed: path("installed")?,
        })
    }
}

#[derive(Debug)]
struct State {
    last: String,
    pending: Vec<String>,
}
fn load_state(dir: &Path, baseline: &str) -> Result<State> {
    let path = dir.join("state.json");
    match fs::symlink_metadata(&path) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(State {
                last: baseline.to_owned(),
                pending: vec![],
            });
        }
        Err(e) => return Err(e.into()),
        Ok(_) => (),
    }
    let v = bounded_json(&path)?;
    if v.get("version").and_then(Value::as_u64) != Some(1) {
        return Err("unsupported state version".into());
    }
    let last = field(&v, "last")?.to_owned();
    let values = v
        .get("pending")
        .and_then(Value::as_array)
        .ok_or("invalid pending queue")?;
    let pending: Vec<String> = values
        .iter()
        .map(|v| v.as_str().unwrap_or("").to_owned())
        .collect();
    if !sha(&last)
        || pending.len() > MAX_PENDING
        || pending.iter().any(|s| !sha(s))
        || pending.iter().collect::<BTreeSet<_>>().len() != pending.len()
    {
        return Err("invalid monitor state".into());
    }
    Ok(State { last, pending })
}
fn save_state(dir: &Path, state: &State) -> Result<()> {
    let tmp = dir.join("state.json.tmp");
    match fs::symlink_metadata(&tmp) {
        Ok(meta) if meta.file_type().is_file() && meta.uid() == unsafe { geteuid() } => {
            fs::remove_file(&tmp)?
        }
        Ok(_) => return Err("invalid stale state file".into()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
        Err(e) => return Err(e.into()),
    }
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&tmp)?;
    serde_json::to_writer(
        &mut f,
        &json!({"version":1,"last":state.last,"pending":state.pending}),
    )?;
    f.write_all(b"\n")?;
    f.sync_all()?;
    fs::rename(&tmp, dir.join("state.json"))?;
    fs::File::open(dir)?.sync_all()?;
    Ok(())
}

fn gh(root: &Path, binary: &Path, args: &[&str]) -> Result<Value> {
    Ok(serde_json::from_slice(&run_limit(
        root,
        binary,
        args,
        1024 * 1024,
    )?)?)
}
trait StatusApi {
    fn head(&self, c: &Config) -> Result<String>;
    fn next(&self, c: &Config, last: &str, head: &str) -> Result<Vec<String>>;
    fn check(&self, c: &Config, commit: &str, codex: &Path) -> Result<Vec<String>>;
    fn verify_actor(&self, c: &Config) -> Result<()>;
    fn current(&self, c: &Config, commit: &str) -> Result<Option<(String, String)>>;
    fn post(&self, c: &Config, commit: &str, state: &str, description: &str) -> Result<()>;
}
struct GitHub<'a>(&'a Path);
impl StatusApi for GitHub<'_> {
    fn head(&self, c: &Config) -> Result<String> {
        remote::Client::new(self.0, &c.repo, &c.state_dir).head()
    }
    fn next(&self, c: &Config, last: &str, head: &str) -> Result<Vec<String>> {
        remote::Client::new(self.0, &c.repo, &c.state_dir).next(last, head)
    }
    fn check(&self, c: &Config, commit: &str, codex: &Path) -> Result<Vec<String>> {
        remote::Client::new(self.0, &c.repo, &c.state_dir).check(
            commit,
            &c.cwd,
            &c.installed,
            codex,
        )
    }
    fn verify_actor(&self, c: &Config) -> Result<()> {
        let actor = gh(&c.state_dir, self.0, &["api", "user"])?;
        if actor.get("login").and_then(Value::as_str) != Some(&c.actor) {
            return Err("GitHub authenticated actor mismatch".into());
        }
        Ok(())
    }
    fn current(&self, c: &Config, commit: &str) -> Result<Option<(String, String)>> {
        current_status(c, self.0, commit)
    }
    fn post(&self, c: &Config, commit: &str, state: &str, description: &str) -> Result<()> {
        let endpoint = format!("repos/{}/statuses/{commit}", c.repo);
        let state_arg = format!("state={state}");
        let context_arg = format!("context={CONTEXT}");
        let description_arg = format!("description={description}");
        gh(
            &c.state_dir,
            self.0,
            &[
                "api",
                "--method",
                "POST",
                &endpoint,
                "-f",
                &state_arg,
                "-f",
                &context_arg,
                "-f",
                &description_arg,
            ],
        )?;
        Ok(())
    }
}
fn current_status(c: &Config, gh_bin: &Path, commit: &str) -> Result<Option<(String, String)>> {
    for page in 1..=5 {
        let endpoint = format!(
            "repos/{}/commits/{commit}/statuses?per_page=100&page={page}",
            c.repo
        );
        let v = gh(&c.state_dir, gh_bin, &["api", &endpoint])?;
        let statuses = v.as_array().ok_or("GitHub status list missing")?;
        for s in statuses {
            if s.get("context").and_then(Value::as_str) == Some(CONTEXT) {
                if s.pointer("/creator/login").and_then(Value::as_str) != Some(&c.actor)
                    || !s
                        .get("url")
                        .and_then(Value::as_str)
                        .unwrap_or("")
                        .ends_with(&format!("/statuses/{commit}"))
                {
                    return Err("conflicting status context or author".into());
                }
                return Ok(Some((
                    field(s, "state")?.to_owned(),
                    field(s, "description")?.to_owned(),
                )));
            }
        }
        if statuses.len() < 100 {
            return Ok(None);
        }
    }
    Err("GitHub status pagination limit exceeded".into())
}
fn publish(
    c: &Config,
    api: &impl StatusApi,
    commit: &str,
    state: &str,
    description: &str,
) -> Result<()> {
    let expected = (state.to_owned(), description.to_owned());
    if api.current(c, commit)?.as_ref() == Some(&expected) {
        return Ok(());
    }
    api.post(c, commit, state, description)?;
    if api.current(c, commit)?.as_ref() != Some(&expected) {
        return Err("GitHub status not independently confirmed".into());
    }
    Ok(())
}
fn evaluate(
    c: &Config,
    api: &impl StatusApi,
    codex: &Path,
    commit: &str,
) -> (bool, bool, &'static str) {
    match api.check(c, commit, codex) {
        Ok(names) if names.is_empty() => (true, false, "No profiled skill changed"),
        Ok(names) => {
            eprintln!("{commit}: verified {}", names.join(", "));
            (true, false, "Local skill discovery verified")
        }
        Err(e) => {
            eprintln!("{commit}: local discovery unverified: {e}");
            if e.downcast_ref::<Unprofiled>().is_some() {
                (false, false, "New skill missing from activation profile")
            } else {
                (false, true, "Local discovery unverified; see host journal")
            }
        }
    }
}

pub fn tick(config_path: &Path, codex: &Path, gh_bin: &Path) -> Result<()> {
    let c = Config::load(config_path)?;
    let _lock = Lock::acquire(&c.state_dir)?;
    poll(&c, codex, &GitHub(gh_bin))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::{Cell, RefCell},
        os::unix::fs::PermissionsExt,
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        root: PathBuf,
        source: PathBuf,
        cache: PathBuf,
        c: Config,
        codex: PathBuf,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    fn command(root: &Path, args: &[&str]) -> String {
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
            "harness-monitor-test-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
        fs::create_dir(&root).unwrap();
        let source = root.join("source");
        fs::create_dir(&source).unwrap();
        command(&source, &["init", "-q"]);
        command(&source, &["config", "user.name", "Fixture"]);
        command(
            &source,
            &["config", "user.email", "fixture@example.invalid"],
        );
        fs::create_dir_all(source.join("contracts")).unwrap();
        fs::write(source.join("contracts/skill-activation.txt"), "old\n").unwrap();
        command(&source, &["add", "."]);
        command(&source, &["commit", "-qm", "base"]);
        let baseline = command(&source, &["rev-parse", "HEAD"]);
        let remote = root.join("remote.git");
        command(&root, &["init", "--bare", "-q", remote.to_str().unwrap()]);
        command(
            &source,
            &["remote", "add", "origin", remote.to_str().unwrap()],
        );
        command(&source, &["push", "-q", "origin", "HEAD:refs/heads/main"]);
        let cache = root.join("cache.git");
        command(
            &root,
            &[
                "clone",
                "--bare",
                "-q",
                remote.to_str().unwrap(),
                cache.to_str().unwrap(),
            ],
        );
        let state_dir = root.join("state");
        fs::create_dir(&state_dir).unwrap();
        let installed = root.join("installed");
        fs::create_dir(&installed).unwrap();
        let codex = root.join("fake-codex");
        let listing = json!({"id":2,"result":{"data":[{"cwd":root,"errors":[],"skills":[{"name":"new","path":installed.join("new/SKILL.md"),"enabled":true,"scope":"user"}]}]}}).to_string();
        fs::write(&codex, format!("#!/bin/sh\nread x\nprintf '%s\\n' '{{\"id\":1,\"result\":{{}}}}'\nread x\nread x\nprintf '%s\\n' '{}'\n", listing)).unwrap();
        fs::set_permissions(&codex, fs::Permissions::from_mode(0o700)).unwrap();
        let c = Config {
            repo: "owner/repo".into(),
            actor: "fixture".into(),
            state_dir,
            cwd: root.clone(),
            installed,
            baseline,
        };
        Fixture {
            root,
            source,
            cache,
            c,
            codex,
        }
    }
    fn add_skill(f: &Fixture) -> String {
        fs::write(
            f.source.join("contracts/skill-activation.txt"),
            "old\nnew\n",
        )
        .unwrap();
        fs::create_dir_all(f.source.join("skills/new")).unwrap();
        fs::write(
            f.source.join("skills/new/SKILL.md"),
            "---\nname: new\ndescription: test\n---\n",
        )
        .unwrap();
        command(&f.source, &["add", "."]);
        command(&f.source, &["commit", "-qm", "new skill"]);
        command(&f.source, &["push", "-q", "origin", "HEAD:refs/heads/main"]);
        command(&f.source, &["rev-parse", "HEAD"])
    }
    fn install(f: &Fixture) {
        fs::create_dir_all(f.c.installed.join("new")).unwrap();
        fs::copy(
            f.source.join("skills/new/SKILL.md"),
            f.c.installed.join("new/SKILL.md"),
        )
        .unwrap();
    }
    struct FakeApi {
        cache: PathBuf,
        statuses: RefCell<BTreeMap<String, (String, String)>>,
        posts: Cell<usize>,
        ambiguous: Cell<bool>,
        bad_actor: Cell<bool>,
    }
    impl FakeApi {
        fn new(f: &Fixture) -> Self {
            Self {
                cache: f.cache.clone(),
                statuses: RefCell::new(BTreeMap::new()),
                posts: Cell::new(0),
                ambiguous: Cell::new(false),
                bad_actor: Cell::new(false),
            }
        }
    }
    impl StatusApi for FakeApi {
        fn head(&self, _: &Config) -> Result<String> {
            git(
                &self.cache,
                &[
                    "fetch",
                    "--no-tags",
                    "origin",
                    "refs/heads/main:refs/remotes/origin/main",
                ],
            )?;
            Ok(String::from_utf8(git(
                &self.cache,
                &["rev-parse", "refs/remotes/origin/main"],
            )?)?
            .trim()
            .to_owned())
        }
        fn next(&self, _: &Config, last: &str, head: &str) -> Result<Vec<String>> {
            git(&self.cache, &["merge-base", "--is-ancestor", last, head])?;
            let range = format!("{last}..{head}");
            let list = String::from_utf8(git(
                &self.cache,
                &["rev-list", "--first-parent", "--reverse", &range],
            )?)?;
            let commits: Vec<String> = list.lines().take(2).map(str::to_owned).collect();
            if let Some(first) = commits.first() {
                if String::from_utf8(git(&self.cache, &["rev-parse", &format!("{first}^1")])?)?
                    .trim()
                    != last
                {
                    return Err("history gap".into());
                }
            }
            Ok(commits)
        }
        fn check(&self, c: &Config, commit: &str, codex: &Path) -> Result<Vec<String>> {
            check(&self.cache, commit, &c.cwd, &c.installed, codex, TIMEOUT)
        }
        fn verify_actor(&self, _: &Config) -> Result<()> {
            if self.bad_actor.get() {
                Err("actor unavailable".into())
            } else {
                Ok(())
            }
        }
        fn current(&self, _: &Config, sha: &str) -> Result<Option<(String, String)>> {
            Ok(self.statuses.borrow().get(sha).cloned())
        }
        fn post(&self, _: &Config, sha: &str, state: &str, description: &str) -> Result<()> {
            self.posts.set(self.posts.get() + 1);
            self.statuses
                .borrow_mut()
                .insert(sha.to_owned(), (state.to_owned(), description.to_owned()));
            if self.ambiguous.replace(false) {
                return Err("ambiguous mutation".into());
            }
            Ok(())
        }
    }

    #[test]
    fn missing_install_reports_failure_then_recovers_without_duplicate_posts() {
        let f = fixture();
        let api = FakeApi::new(&f);
        let new = add_skill(&f);
        poll(&f.c, &f.codex, &api).unwrap();
        assert_eq!(api.statuses.borrow()[&new].0, "failure");
        let s = load_state(&f.c.state_dir, &f.c.baseline).unwrap();
        assert_eq!(s.last, new);
        assert_eq!(s.pending, vec![new.clone()]);
        poll(&f.c, &f.codex, &api).unwrap();
        assert_eq!(api.posts.get(), 1);
        install(&f);
        poll(&f.c, &f.codex, &api).unwrap();
        assert_eq!(api.statuses.borrow()[&new].0, "success");
        assert!(load_state(&f.c.state_dir, &f.c.baseline)
            .unwrap()
            .pending
            .is_empty());
        assert_eq!(api.posts.get(), 2);
    }
    #[test]
    fn new_unprofiled_skill_fails_without_endless_retry_then_profile_fix_passes() {
        let f = fixture();
        let api = FakeApi::new(&f);
        fs::create_dir_all(f.source.join("skills/new")).unwrap();
        fs::write(
            f.source.join("skills/new/SKILL.md"),
            "---\nname: new\ndescription: test\n---\n",
        )
        .unwrap();
        command(&f.source, &["add", "."]);
        command(&f.source, &["commit", "-qm", "unprofiled"]);
        command(&f.source, &["push", "-q", "origin", "HEAD:refs/heads/main"]);
        let old = command(&f.source, &["rev-parse", "HEAD"]);
        poll(&f.c, &f.codex, &api).unwrap();
        assert_eq!(
            api.statuses.borrow()[&old].1,
            "New skill missing from activation profile"
        );
        assert!(load_state(&f.c.state_dir, &f.c.baseline)
            .unwrap()
            .pending
            .is_empty());
        fs::write(
            f.source.join("contracts/skill-activation.txt"),
            "old\nnew\n",
        )
        .unwrap();
        command(&f.source, &["add", "."]);
        command(&f.source, &["commit", "-qm", "profile"]);
        command(&f.source, &["push", "-q", "origin", "HEAD:refs/heads/main"]);
        let fixed = command(&f.source, &["rev-parse", "HEAD"]);
        install(&f);
        poll(&f.c, &f.codex, &api).unwrap();
        assert_eq!(api.statuses.borrow()[&fixed].0, "success");
        assert_eq!(api.statuses.borrow()[&old].0, "failure");
    }
    #[test]
    fn github_status_uses_latest_matching_context_and_verifies_actor() {
        let f = fixture();
        let fake = f.root.join("fake-gh");
        let entry = |state: &str, actor: &str| json!({"context":CONTEXT,"state":state,"description":"known","creator":{"login":actor},"url":format!("https://api.github.com/repos/owner/repo/statuses/{}",f.c.baseline)});
        let data = json!([entry("success", "fixture"), entry("failure", "fixture")]).to_string();
        fs::write(&fake, format!("#!/bin/sh\nprintf '%s\\n' '{}'\n", data)).unwrap();
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o700)).unwrap();
        assert_eq!(
            current_status(&f.c, &fake, &f.c.baseline)
                .unwrap()
                .unwrap()
                .0,
            "success"
        );
        let data = json!([entry("success", "another-user")]).to_string();
        fs::write(&fake, format!("#!/bin/sh\nprintf '%s\\n' '{}'\n", data)).unwrap();
        assert!(current_status(&f.c, &fake, &f.c.baseline).is_err());
    }
    #[test]
    fn github_command_posts_and_reconciles_an_ambiguous_write() {
        let f = fixture();
        let fake = f.root.join("fake-gh");
        let status = f.root.join("status.json");
        let writes = f.root.join("writes");
        fs::write(&status, "[]").unwrap();
        let entry = json!([{"context":CONTEXT,"state":"success","description":"verified","creator":{"login":"fixture"},"url":format!("https://api.github.com/repos/owner/repo/statuses/{}",f.c.baseline)}]);
        let script = format!(
            "#!/bin/sh\nif [ \"$2\" = --method ]; then\n  [ \"$3\" = POST ] || exit 2\n  [ \"$4\" = repos/owner/repo/statuses/{} ] || exit 3\n  [ \"$6\" = state=success ] || exit 4\n  [ \"$8\" = context={} ] || exit 5\n  [ \"${{10}}\" = description=verified ] || exit 6\n  printf '%s\\n' '{}' > '{}'\n  printf x >> '{}'\n  exit 1\nfi\ncat '{}'\n",
            f.c.baseline, CONTEXT, entry, status.display(), writes.display(), status.display()
        );
        fs::write(&fake, script).unwrap();
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o700)).unwrap();
        let api = GitHub(&fake);
        assert!(publish(&f.c, &api, &f.c.baseline, "success", "verified").is_err());
        publish(&f.c, &api, &f.c.baseline, "success", "verified").unwrap();
        assert_eq!(fs::read(&writes).unwrap(), b"x");
    }
    #[test]
    fn ambiguous_status_write_reconciles_before_replaying() {
        let f = fixture();
        let api = FakeApi::new(&f);
        let new = add_skill(&f);
        install(&f);
        api.ambiguous.set(true);
        assert!(poll(&f.c, &f.codex, &api).is_err());
        assert_eq!(
            load_state(&f.c.state_dir, &f.c.baseline).unwrap().last,
            f.c.baseline
        );
        assert_eq!(api.posts.get(), 1);
        poll(&f.c, &f.codex, &api).unwrap();
        assert_eq!(api.posts.get(), 1);
        assert_eq!(load_state(&f.c.state_dir, &f.c.baseline).unwrap().last, new);
    }
    #[test]
    fn backlog_progress_is_ordered_and_bounded() {
        let f = fixture();
        let api = FakeApi::new(&f);
        let first = add_skill(&f);
        install(&f);
        for i in 0..3 {
            fs::write(f.source.join("note"), i.to_string()).unwrap();
            command(&f.source, &["add", "note"]);
            command(&f.source, &["commit", "-qm", "note"]);
        }
        command(&f.source, &["push", "-q", "origin", "HEAD:refs/heads/main"]);
        let head = command(&f.source, &["rev-parse", "HEAD"]);
        poll(&f.c, &f.codex, &api).unwrap();
        assert_eq!(api.statuses.borrow().len(), 2);
        assert_eq!(api.statuses.borrow()[&first].0, "success");
        assert_ne!(
            load_state(&f.c.state_dir, &f.c.baseline).unwrap().last,
            head
        );
        poll(&f.c, &f.codex, &api).unwrap();
        assert_eq!(api.statuses.borrow().len(), 4);
        assert_eq!(
            load_state(&f.c.state_dir, &f.c.baseline).unwrap().last,
            head
        );
    }
    #[test]
    fn unavailable_actor_and_corrupt_state_never_advance() {
        let f = fixture();
        let api = FakeApi::new(&f);
        add_skill(&f);
        api.bad_actor.set(true);
        assert!(poll(&f.c, &f.codex, &api).is_err());
        assert_eq!(api.posts.get(), 0);
        assert!(!f.c.state_dir.join("state.json").exists());
        fs::write(f.c.state_dir.join("state.json"), "invalid").unwrap();
        assert!(poll(&f.c, &f.codex, &api).is_err());
        assert_eq!(api.posts.get(), 0);
    }
    #[test]
    fn lock_excludes_concurrent_monitor() {
        let f = fixture();
        let _lock = Lock::acquire(&f.c.state_dir).unwrap();
        assert!(Lock::acquire(&f.c.state_dir).is_err());
    }
    #[test]
    fn rewritten_remote_cannot_skip_history() {
        let f = fixture();
        let api = FakeApi::new(&f);
        let first = add_skill(&f);
        install(&f);
        poll(&f.c, &f.codex, &api).unwrap();
        command(&f.source, &["checkout", "--orphan", "replacement"]);
        command(
            &f.source,
            &["commit", "--allow-empty", "-qm", "new history"],
        );
        command(
            &f.source,
            &["push", "-q", "--force", "origin", "HEAD:refs/heads/main"],
        );
        assert!(poll(&f.c, &f.codex, &api).is_err());
        assert_eq!(
            load_state(&f.c.state_dir, &f.c.baseline).unwrap().last,
            first
        );
    }
}

fn poll(c: &Config, codex: &Path, api: &impl StatusApi) -> Result<()> {
    let mut state = load_state(&c.state_dir, &c.baseline)?;
    let head = api.head(c)?;
    if !sha(&head) {
        return Err("invalid remote main identity".into());
    }
    let commits = api.next(c, &state.last, &head)?;
    if commits.len() > MAX_PER_RUN || commits.iter().any(|s| !sha(s)) {
        return Err("backlog exceeds limit or is malformed".into());
    }
    if commits.is_empty() && head != state.last {
        return Err("main history gap".into());
    }
    if !commits.is_empty() || !state.pending.is_empty() {
        api.verify_actor(c)?;
    }
    if !state.pending.is_empty() {
        let commit = state.pending[0].clone();
        let (ok, retry, description) = evaluate(&c, api, codex, &commit);
        publish(
            &c,
            api,
            &commit,
            if ok { "success" } else { "failure" },
            description,
        )?;
        state.pending.remove(0);
        if retry {
            state.pending.push(commit);
        }
        save_state(&c.state_dir, &state)?;
    }
    for commit in commits.iter().take(MAX_PER_RUN) {
        if state.pending.len() >= MAX_PENDING {
            return Err("pending discovery queue full; manual reconciliation required".into());
        }
        let (ok, retry, description) = evaluate(&c, api, codex, commit);
        publish(
            &c,
            api,
            commit,
            if ok { "success" } else { "failure" },
            description,
        )?;
        if retry {
            state.pending.push(commit.to_owned());
        }
        state.last = commit.to_owned();
        save_state(&c.state_dir, &state)?;
    }
    if !c.state_dir.join("state.json").exists() {
        save_state(&c.state_dir, &state)?;
    }
    if state.last != head {
        eprintln!("additional commits remain for later polling");
    }
    Ok(())
}
