use crate::{append_log_line, SessionInner};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

const TARGET_TRIPLE: &str = env!("ORBIEN_TARGET_TRIPLE");

pub fn resolve_orbien(override_path: &str) -> Result<PathBuf, String> {
    if !override_path.trim().is_empty() {
        let p = PathBuf::from(override_path.trim());
        if p.is_file() {
            return Ok(p);
        }
        return Err(format!(
            "orbien not found at configured path: {}",
            p.display()
        ));
    }

    if let Ok(env_path) = std::env::var("ORBIEN_PATH") {
        let p = PathBuf::from(env_path.trim());
        if p.is_file() {
            return Ok(p);
        }
    }

    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(exe) = std::env::current_exe() {
        let exe = exe.canonicalize().unwrap_or(exe);
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join(format!("orbien{}", exe_suffix())));
            candidates.push(dir.join(format!(
                "orbien-{}{}",
                TARGET_TRIPLE,
                exe_suffix()
            )));
            if let Some(contents) = dir.parent() {
                candidates.push(
                    contents
                        .join("Resources")
                        .join(format!("orbien{}", exe_suffix())),
                );
                candidates.push(contents.join("MacOS").join(format!("orbien{}", exe_suffix())));
            }
        }
    }

    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
        "binaries/orbien-{}{}",
        TARGET_TRIPLE,
        exe_suffix()
    )));

    let rels = [
        "orbien",
        "orbien.exe",
        "target/release/orbien",
        "target/debug/orbien",
        "target/release/orbien.exe",
        "target/debug/orbien.exe",
        "dist/orbien",
        "dist/orbien.exe",
    ];

    if let Ok(cwd) = std::env::current_dir() {
        for rel in rels {
            candidates.push(cwd.join(rel));
        }
        if let Some(found) = find_in_ancestors(&cwd, "target/release/orbien") {
            candidates.push(found);
        }
        if let Some(found) = find_in_ancestors(&cwd, "target/debug/orbien") {
            candidates.push(found);
        }
        if let Some(found) = find_in_ancestors(&cwd, "dist/orbien") {
            candidates.push(found);
        }
    }

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            if let Some(found) = find_in_ancestors(dir, "target/release/orbien") {
                candidates.push(found);
            }
            if let Some(found) = find_in_ancestors(dir, "dist/orbien") {
                candidates.push(found);
            }
        }
    }

    for c in &candidates {
        if c.is_file() {
            return Ok(c.clone());
        }
    }

    if let Ok(path) = which("orbien") {
        return Ok(path);
    }

    Err(
        "orbien sidecar not found next to the app. Reinstall the desktop package, \
         or set Config → Orbien Binary Path to your local `orbien` (e.g. \
         /path/to/target/release/orbien)."
            .into(),
    )
}

fn exe_suffix() -> &'static str {
    if TARGET_TRIPLE.contains("windows") {
        ".exe"
    } else {
        ""
    }
}

fn find_in_ancestors(start: &Path, rel: &str) -> Option<PathBuf> {
    let mut dir = start.to_path_buf();
    for _ in 0..10 {
        let cand = dir.join(rel);
        if cand.is_file() {
            return Some(cand);
        }
        if !dir.pop() {
            break;
        }
    }
    None
}

fn which(name: &str) -> Result<PathBuf, ()> {
    let Ok(path_env) = std::env::var("PATH") else {
        return Err(());
    };
    for dir in std::env::split_paths(&path_env) {
        let p = dir.join(name);
        if p.is_file() {
            return Ok(p);
        }
        #[cfg(windows)]
        {
            let p_exe = dir.join(format!("{name}.exe"));
            if p_exe.is_file() {
                return Ok(p_exe);
            }
        }
    }
    Err(())
}

pub fn spawn_orbien(
    bin: &Path,
    config: &Path,
    logs: Arc<Mutex<SessionInner>>,
) -> Result<Child, String> {
    let mut child = Command::new(bin)
        .arg("-c")
        .arg(config)
        .env("NO_COLOR", "1")
        .env("RUST_LOG_STYLE", "never")
        .env("CLICOLOR", "0")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::null())
        .spawn()
        .map_err(|e| format!("{}: {e}", bin.display()))?;

    if let Some(out) = child.stdout.take() {
        let logs_o = Arc::clone(&logs);
        thread::spawn(move || {
            let reader = BufReader::new(out);
            for line in reader.lines().flatten() {
                append_log_line(&logs_o, format!("[orbien] {line}"));
            }
        });
    }
    if let Some(err) = child.stderr.take() {
        let logs_e = Arc::clone(&logs);
        thread::spawn(move || {
            let reader = BufReader::new(err);
            for line in reader.lines().flatten() {
                append_log_line(&logs_e, format!("[orbien:err] {line}"));
            }
        });
    }

    Ok(child)
}

pub fn stop_child(mut child: Child) -> Result<(), String> {
    child.kill().map_err(|e| e.to_string())?;
    let _ = child.wait();
    Ok(())
}
