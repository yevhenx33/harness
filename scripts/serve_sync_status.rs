//! Standard-library Rust memory server for the harness status experiment.

use std::env;
use std::fs;
use std::hint::black_box;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixListener;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Instant;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

struct Snapshot {
    result: Option<String>,
    error: Option<String>,
    refreshed_at: SystemTime,
    refreshing: bool,
    sequence: u64,
}

impl Snapshot {
    fn new() -> Self {
        Self {
            result: None,
            error: None,
            refreshed_at: UNIX_EPOCH,
            refreshing: true,
            sequence: 0,
        }
    }

    fn response(&self) -> String {
        let ready = self.result.is_some();
        let refreshed_ms = self
            .refreshed_at
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let age_ms = SystemTime::now()
            .duration_since(self.refreshed_at)
            .unwrap_or_default()
            .as_millis();
        let state = if ready {
            "ready"
        } else if self.error.is_some() {
            "error"
        } else {
            "verifying"
        };
        let verified = if ready {
            refreshed_ms.to_string()
        } else {
            "null".to_string()
        };
        let age = if ready {
            age_ms.to_string()
        } else {
            "null".to_string()
        };
        let result = self.result.as_deref().unwrap_or("null");
        let error = self
            .error
            .as_ref()
            .map(|value| format!("\"{}\"", json_escape(value)))
            .unwrap_or_else(|| "null".to_string());
        format!(
            "{{\"service_state\":\"{state}\",\"sequence\":{},\"verified_at_unix_ms\":{},\"age_ms\":{},\"refreshing\":{},\"result\":{},\"error\":{}}}\n",
            self.sequence, verified, age, self.refreshing, result, error
        )
    }
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn assessment() -> Result<String, String> {
    let output = Command::new("python3")
        .arg("scripts/run_sync_assessment.py")
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .output()
        .map_err(|error| error.to_string())?;
    if output.status.success() {
        String::from_utf8(output.stdout)
            .map(|value| value.trim_end().to_owned())
            .map_err(|error| error.to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn refresh(state: &RwLock<Snapshot>) {
    state.write().unwrap().refreshing = true;
    let result = assessment();
    let mut snapshot = state.write().unwrap();
    match result {
        Ok(value) => {
            snapshot.result = Some(value);
            snapshot.error = None;
            snapshot.refreshed_at = SystemTime::now();
            snapshot.sequence += 1;
        }
        Err(error) => snapshot.error = Some(error),
    }
    snapshot.refreshing = false;
}

fn argument(name: &str, default: &str) -> String {
    let values: Vec<String> = env::args().collect();
    values
        .iter()
        .position(|value| value == name)
        .and_then(|index| values.get(index + 1))
        .cloned()
        .unwrap_or_else(|| default.to_string())
}

fn benchmark_local_reads(state: &RwLock<Snapshot>, reads: u64) {
    let started = Instant::now();
    for _ in 0..reads {
        let snapshot = state.read().unwrap();
        black_box((snapshot.sequence, snapshot.result.as_ref().map(String::len)));
    }
    let elapsed = started.elapsed().as_nanos();
    println!(
        "{{\"reads\":{reads},\"elapsed_ns\":{elapsed},\"ns_per_read\":{:.3}}}",
        elapsed as f64 / reads as f64
    );
}

struct SocketGuard(PathBuf);
impl Drop for SocketGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = PathBuf::from(argument("--socket", "/tmp/harness-sync-status-rust.sock"));
    let interval = argument("--interval", "5").parse::<f64>()?;
    let max_requests = argument("--max-requests", "0").parse::<u64>()?;
    let benchmark_reads = argument("--benchmark-local", "0").parse::<u64>()?;
    if socket.exists() {
        return Err(format!("socket already exists: {}", socket.display()).into());
    }
    let state = Arc::new(RwLock::new(Snapshot::new()));
    refresh(&state);
    if benchmark_reads > 0 {
        benchmark_local_reads(&state, benchmark_reads);
        return Ok(());
    }
    let listener = UnixListener::bind(&socket)?;
    fs::set_permissions(&socket, fs::Permissions::from_mode(0o600))?;
    let _guard = SocketGuard(socket);
    let stopping = Arc::new(AtomicBool::new(false));
    let refresh_state = Arc::clone(&state);
    let refresh_stop = Arc::clone(&stopping);
    thread::spawn(move || {
        while !refresh_stop.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs_f64(interval));
            if !refresh_stop.load(Ordering::Relaxed) {
                refresh(&refresh_state);
            }
        }
    });
    let mut served = 0;
    'connections: for connection in listener.incoming() {
        let mut stream = connection?;
        stream.write_all(state.read().unwrap().response().as_bytes())?;
        served += 1;
        if max_requests > 0 && served >= max_requests {
            break;
        }
        let mut reader = BufReader::new(stream.try_clone()?);
        loop {
            let mut request = String::new();
            if reader.read_line(&mut request)? == 0 {
                break;
            }
            stream.write_all(state.read().unwrap().response().as_bytes())?;
            served += 1;
            if max_requests > 0 && served >= max_requests {
                break 'connections;
            }
        }
    }
    stopping.store(true, Ordering::Relaxed);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_json_errors() {
        assert_eq!(json_escape("a\n\"b\\"), "a\\n\\\"b\\\\");
    }

    #[test]
    fn response_exposes_freshness_and_result() {
        let snapshot = Snapshot {
            result: Some("{\"status\":\"verified win\"}\n".trim_end().to_string()),
            error: None,
            refreshed_at: SystemTime::now(),
            refreshing: false,
            sequence: 2,
        };
        let response = snapshot.response();
        assert!(response.contains("\"service_state\":\"ready\""));
        assert!(response.contains("\"sequence\":2"));
        assert!(response.contains("\"result\":{\"status\":\"verified win\"}"));
        assert_eq!(response.lines().count(), 1);
    }
}
