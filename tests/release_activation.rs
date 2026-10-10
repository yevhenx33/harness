#[allow(dead_code)]
#[path = "../scripts/verify_release.rs"]
mod guard;
use std::{
    fs,
    path::PathBuf,
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let id = NEXT.fetch_add(1, Ordering::Relaxed);
        let name = format!("harness-guard-{}-{id}", std::process::id());
        let root = std::env::temp_dir().join(name);
        fs::create_dir(&root).unwrap();
        let fixture = Self(root);
        fixture.write("agents/versions/v001.md", "first");
        fixture.write("agents/versions/v002.md", "second");
        fixture.current(2);
        for args in [
            &["init", "-q"][..],
            &["config", "user.name", "Fixture"],
            &["config", "user.email", "fixture@example.invalid"],
            &["add", "."],
            &["commit", "-qm", "baseline"],
            &["branch", "baseline"],
        ] {
            fixture.git(args);
        }
        fixture
    }
    fn git(&self, args: &[&str]) {
        let options = [
            "-c",
            "core.hooksPath=/dev/null",
            "-c",
            "commit.gpgsign=false",
        ];
        let output = Command::new("git")
            .args(options)
            .args(args)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .current_dir(&self.0)
            .output()
            .unwrap();
        assert!(output.status.success(), "Git fixture failed: {output:?}");
    }
    fn write(&self, path: &str, value: &str) {
        let path = self.0.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, value).unwrap();
    }
    fn current(&self, number: u16) {
        let row = format!("| [`v{number:03}`](versions/v{number:03}.md) | Current |\n");
        self.write("agents/README.md", &row);
    }
    fn skill(&self) {
        self.write("contracts/skill-activation.txt", "sample\n");
        for prefix in ["skills", "installed"] {
            self.write(&format!("{prefix}/sample/SKILL.md"), "skill");
            self.write(&format!("{prefix}/sample/agents/openai.yaml"), "metadata");
            self.write(&format!("{prefix}/sample/references/owner.md"), "invariant");
        }
    }
    fn installed(&self) -> PathBuf {
        self.0.join("installed")
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}

#[test]
fn unchanged_and_next_release_pass() {
    let f = Fixture::new();
    guard::release(&f.0, "baseline").unwrap();
    f.write("agents/versions/v003.md", "third");
    f.current(3);
    guard::release(&f.0, "baseline").unwrap();
}
#[test]
fn rollback_and_multiple_new_versions_fail() {
    let f = Fixture::new();
    f.current(1);
    assert!(guard::release(&f.0, "baseline").is_err());
    f.write("agents/versions/v003.md", "third");
    f.write("agents/versions/v004.md", "fourth");
    f.current(4);
    assert!(guard::release(&f.0, "baseline").is_err());
}
#[test]
fn historical_rewrite_and_removal_fail() {
    let f = Fixture::new();
    f.write("agents/versions/v001.md", "rewritten");
    assert!(guard::release(&f.0, "baseline").is_err());
    fs::remove_file(f.0.join("agents/versions/v001.md")).unwrap();
    assert!(guard::release(&f.0, "baseline").is_err());
}
#[test]
fn unavailable_or_unrelated_baseline_fails() {
    let f = Fixture::new();
    for baseline in ["", "--help", "missing"] {
        assert!(guard::release(&f.0, baseline).is_err());
    }
    f.git(&["checkout", "--orphan", "unrelated"]);
    f.git(&["commit", "-qm", "unrelated"]);
    assert!(guard::release(&f.0, "baseline").is_err());
}
#[test]
fn packages_require_every_file_and_reject_drift() {
    let f = Fixture::new();
    f.skill();
    guard::skills(&f.0, Some(&f.installed())).unwrap();
    for path in [
        "SKILL.md",
        "agents/openai.yaml",
        "references/owner.md",
        "extra.txt",
    ] {
        let target = f.installed().join("sample").join(path);
        let previous = fs::read(&target).ok();
        fs::write(&target, "drift").unwrap();
        assert!(guard::skills(&f.0, Some(&f.installed())).is_err(), "{path}");
        fs::remove_file(&target).unwrap();
        if let Some(previous) = previous {
            assert!(guard::skills(&f.0, Some(&f.installed())).is_err());
            fs::write(target, previous).unwrap();
        }
    }
    assert!(guard::skills(&f.0, Some(&f.0.join("missing"))).is_err());
}

#[test]
fn package_resource_limits_fail_closed() {
    let f = Fixture::new();
    f.skill();
    let installed = f.installed();
    let big = fs::File::create(f.installed().join("sample/large")).unwrap();
    big.set_len(16 * 1024 * 1024 + 1).unwrap();
    let error = guard::skills(&f.0, Some(&installed)).unwrap_err();
    assert!(error.to_string().contains("limit"), "{error}");
    fs::remove_file(f.installed().join("sample/large")).unwrap();
    for n in 0..257 {
        f.write(&format!("installed/sample/{n}"), "");
    }
    let error = guard::skills(&f.0, Some(&installed)).unwrap_err();
    assert!(error.to_string().contains("limit"), "{error}");
}
#[test]
fn invalid_profiles_and_nested_symlinks_fail() {
    let f = Fixture::new();
    f.skill();
    for profile in ["", "../sample\n", "sample\nsample\n"] {
        f.write("contracts/skill-activation.txt", profile);
        assert!(guard::skills(&f.0, None).is_err());
    }
    f.write("contracts/skill-activation.txt", "sample\n");
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink("SKILL.md", f.installed().join("sample/alias")).unwrap();
        assert!(guard::skills(&f.0, Some(&f.installed())).is_err());
    }
}
