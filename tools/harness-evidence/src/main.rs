mod archive;
mod router;
mod run;

use serde::de::DeserializeOwned;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::process::ExitCode;

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const INPUT_LIMIT: u64 = 1024 * 1024;

fn regular(path: &Path) -> Result<File> {
    let file = File::options()
        .read(true)
        .custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW)
        .open(path)?;
    if !file.metadata()?.is_file() {
        return Err("input must be a regular file".into());
    }
    Ok(file)
}

fn json<T: DeserializeOwned>(file: &Path) -> Result<T> {
    let mut bytes = Vec::new();
    regular(file)?
        .take(INPUT_LIMIT + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > INPUT_LIMIT {
        return Err("JSON input exceeds 1 MiB".into());
    }
    Ok(serde_json::from_slice(&bytes)?)
}

fn output(value: impl serde::Serialize) -> Result<()> {
    serde_json::to_writer_pretty(io::stdout().lock(), &value)?;
    println!();
    Ok(())
}

fn execute() -> Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    match args.as_slice() {
        [command, plan, observed] if command == "check-run" => {
            output(run::check(&json(Path::new(plan))?, &json(Path::new(observed))?)?)
        }
        [command, plan, evidence] if command == "router-replay" => {
            output(router::replay(&json(Path::new(plan))?, Path::new(evidence))?)
        }
        [command, manifest] if command == "archive" => {
            output(archive::scan(json(Path::new(manifest))?, Path::new(manifest))?)
        }
        _ => Err("usage: harness-evidence check-run PLAN OBSERVED | router-replay PLAN RETAINED_GATE_DIR | archive MANIFEST".into()),
    }
}

fn main() -> ExitCode {
    match execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("harness-evidence: {error}");
            ExitCode::from(2)
        }
    }
}
