//! Read-only release and installed-package gates; existing integrity checks remain required.
use std::{
    collections::BTreeMap,
    env,
    error::Error,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let output = Command::new("git")
        .arg("--no-replace-objects")
        .args(args)
        .current_dir(root)
        .output()?;
    if !output.status.success() {
        return Err(format!("Git baseline check failed ({args:?}): {}", output.status).into());
    }
    Ok(output.stdout)
}

fn version(path: &Path) -> Result<u16> {
    let name = path.file_name().ok_or("invalid snapshot path")?;
    let name = name.to_string_lossy();
    let digits = name.strip_prefix('v').and_then(|s| s.strip_suffix(".md"));
    let digits = digits.ok_or("invalid snapshot name")?;
    if digits.len() != 3 || !digits.bytes().all(|c| c.is_ascii_digit()) {
        return Err("invalid snapshot version".into());
    }
    Ok(digits.parse()?)
}

pub fn release(root: &Path, baseline: &str) -> Result<()> {
    if baseline.is_empty() || baseline.starts_with('-') {
        return Err("explicit trusted baseline required".into());
    }
    let commit = String::from_utf8(git(
        root,
        &["rev-parse", "--verify", &format!("{baseline}^{{commit}}")],
    )?)?;
    let commit = commit.trim();
    git(root, &["merge-base", "--is-ancestor", commit, "HEAD"])?;
    let tree = format!("{commit}:agents/versions");
    let historical = String::from_utf8(git(root, &["ls-tree", "-r", "--name-only", &tree])?)?;
    let mut previous = 0;
    for path in historical.lines() {
        let path = format!("agents/versions/{path}");
        previous = previous.max(version(Path::new(&path))?);
        let local = root.join(&path);
        if !fs::symlink_metadata(&local)?.file_type().is_file()
            || fs::read(&local)? != git(root, &["show", &format!("{commit}:{path}")])?
        {
            return Err(format!("historical snapshot changed: {path}").into());
        }
    }
    if previous == 0 {
        return Err("trusted baseline has no policy snapshots".into());
    }
    let mut latest = 0;
    for entry in fs::read_dir(root.join("agents/versions"))? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            return Err("snapshots must be regular files".into());
        }
        latest = latest.max(version(&entry.path())?);
    }
    if latest > previous + 1 {
        return Err("release must append only the next policy version".into());
    }
    let index = fs::read_to_string(root.join("agents/README.md"))?;
    let current: Vec<_> = index
        .lines()
        .filter(|line| line.starts_with("| [`v") && line.contains(" | Current |"))
        .collect();
    if current.len() != 1
        || !current[0].starts_with(&format!(
            "| [`v{latest:03}`](versions/v{latest:03}.md) | Current |"
        ))
    {
        return Err("Current must identify the latest policy snapshot".into());
    }
    Ok(())
}

fn package(root: &Path) -> Result<BTreeMap<PathBuf, Vec<u8>>> {
    let mut files = BTreeMap::new();
    let (mut entries, mut bytes) = (0, 0);
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let full = entry.path();
            let path = full.strip_prefix(root)?.to_path_buf();
            entries += 1;
            if entries > 256 || path.components().count() > 16 {
                return Err("skill package inventory limit exceeded".into());
            }
            let kind = entry.file_type()?;
            if kind.is_dir() {
                pending.push(full);
            } else if kind.is_file() {
                let mut data = Vec::new();
                fs::File::open(full)?
                    .take(16 * 1024 * 1024 - bytes + 1)
                    .read_to_end(&mut data)?;
                bytes += data.len() as u64;
                if bytes > 16 * 1024 * 1024 {
                    return Err("skill package byte limit exceeded".into());
                }
                files.insert(path, data);
            } else {
                return Err(format!("unsupported package entry: {}", full.display()).into());
            }
        }
    }
    if !files.contains_key(Path::new("SKILL.md")) {
        return Err(format!("missing SKILL.md in {}", root.display()).into());
    }
    Ok(files)
}

pub fn skills(root: &Path, installed: Option<&Path>) -> Result<()> {
    let profile = fs::read_to_string(root.join("contracts/skill-activation.txt"))?;
    let mut names = std::collections::BTreeSet::new();
    for name in profile.lines() {
        if name.is_empty() || name.starts_with('#') {
            continue;
        }
        if !name
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'-')
            || !names.insert(name)
        {
            return Err(format!("invalid or duplicate skill name: {name}").into());
        }
        let source = package(&root.join("skills").join(name))?;
        if let Some(installed) = installed {
            if source != package(&installed.join(name))? {
                return Err(format!("installed skill differs: {name}").into());
            }
        }
    }
    if names.is_empty() {
        return Err("activation profile is empty".into());
    }
    Ok(())
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let result = match args.as_slice() {
        [_, mode, root, baseline] if mode == "release" => release(Path::new(root), baseline),
        [_, mode, root] if mode == "skills" => skills(Path::new(root), None),
        [_, mode, root, installed] if mode == "activation" => skills(Path::new(root), Some(Path::new(installed))),
        _ => Err("usage: verify_release release ROOT TRUSTED_BASE | skills ROOT | activation ROOT INSTALLED_ROOT".into()),
    };
    match result {
        Ok(()) => println!("{}: ok", args[1]),
        Err(error) => {
            eprintln!("verification failed: {error}");
            std::process::exit(1);
        }
    }
}
