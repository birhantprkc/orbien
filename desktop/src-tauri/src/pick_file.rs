use serde::Deserialize;
use std::process::Command;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileFilter {
    #[allow(dead_code)]
    pub name: String,
    pub extensions: Vec<String>,
}

#[tauri::command]
pub fn pick_file(
    title: Option<String>,
    filters: Option<Vec<FileFilter>>,
) -> Result<Option<String>, String> {
    let title = title.unwrap_or_else(|| "Select file".into());
    let filters = filters.unwrap_or_default();

    #[cfg(target_os = "macos")]
    {
        return pick_macos(&title, &filters);
    }
    #[cfg(target_os = "windows")]
    {
        return pick_windows(&title, &filters);
    }
    #[cfg(target_os = "linux")]
    {
        return pick_linux(&title, &filters);
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        let _ = (title, filters);
        Err("file picker is not supported on this platform".into())
    }
}

#[cfg(target_os = "macos")]
fn pick_macos(title: &str, filters: &[FileFilter]) -> Result<Option<String>, String> {
    let mut script = format!(
        "try\nset theFile to choose file with prompt \"{}\"",
        escape_applescript(title)
    );
    let exts: Vec<String> = filters
        .iter()
        .flat_map(|f| f.extensions.iter().cloned())
        .filter(|e| !e.is_empty())
        .collect();
    if !exts.is_empty() {
        let _ = exts;
    }
    script.push_str("\nPOSIX path of theFile\non error number -128\nreturn \"\"\nend try");

    let out = Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|e| format!("osascript: {e}"))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        if err.contains("-128") || out.stdout.is_empty() {
            return Ok(None);
        }
        return Err(format!("osascript failed: {err}"));
    }
    let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if path.is_empty() {
        Ok(None)
    } else {
        Ok(Some(path))
    }
}

#[cfg(target_os = "macos")]
fn escape_applescript(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(target_os = "windows")]
fn pick_windows(title: &str, filters: &[FileFilter]) -> Result<Option<String>, String> {
    let mut filter = String::new();
    for f in filters {
        if f.extensions.is_empty() {
            continue;
        }
        let patterns = f
            .extensions
            .iter()
            .map(|e| {
                let e = e.trim_start_matches('.');
                if e == "*" {
                    "*.*".into()
                } else {
                    format!("*.{e}")
                }
            })
            .collect::<Vec<_>>()
            .join(";");
        if !filter.is_empty() {
            filter.push('|');
        }
        filter.push_str(&format!("{} ({})|{}", f.name, patterns, patterns));
    }
    if filter.is_empty() {
        filter = "All files (*.*)|*.*".into();
    } else if !filter.to_ascii_lowercase().contains("*.*") {
        filter.push_str("|All files (*.*)|*.*");
    }

    let ps = format!(
        r#"Add-Type -AssemblyName System.Windows.Forms; $d = New-Object System.Windows.Forms.OpenFileDialog; $d.Title = '{title}'; $d.Filter = '{filter}'; $d.Multiselect = $false; if ($d.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {{ $d.FileName }} else {{ '' }}"#,
        title = escape_ps(title),
        filter = escape_ps(&filter),
    );

    let out = Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps])
        .output()
        .map_err(|e| format!("powershell: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "powershell failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if path.is_empty() {
        Ok(None)
    } else {
        Ok(Some(path))
    }
}

#[cfg(target_os = "windows")]
fn escape_ps(s: &str) -> String {
    s.replace('\'', "''")
}

#[cfg(target_os = "linux")]
fn pick_linux(title: &str, filters: &[FileFilter]) -> Result<Option<String>, String> {
    if let Ok(path) = try_zenity(title, filters) {
        return Ok(path);
    }
    if let Ok(path) = try_kdialog(title, filters) {
        return Ok(path);
    }
    Err("no file dialog available (install zenity or kdialog)".into())
}

#[cfg(target_os = "linux")]
fn try_zenity(title: &str, filters: &[FileFilter]) -> Result<Option<String>, String> {
    let mut cmd = Command::new("zenity");
    cmd.args(["--file-selection", "--title", title]);
    for f in filters {
        if f.extensions.is_empty() {
            continue;
        }
        let patterns = f
            .extensions
            .iter()
            .map(|e| format!("*.{}", e.trim_start_matches('.')))
            .collect::<Vec<_>>()
            .join(" ");
        cmd.args(["--file-filter", &format!("{} | {}", f.name, patterns)]);
    }
    let out = cmd.output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Ok(None);
    }
    let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if path.is_empty() {
        Ok(None)
    } else {
        Ok(Some(path))
    }
}

#[cfg(target_os = "linux")]
fn try_kdialog(title: &str, filters: &[FileFilter]) -> Result<Option<String>, String> {
    let mut filter = String::new();
    for f in filters {
        if f.extensions.is_empty() {
            continue;
        }
        let patterns = f
            .extensions
            .iter()
            .map(|e| format!("*.{}", e.trim_start_matches('.')))
            .collect::<Vec<_>>()
            .join(" ");
        if !filter.is_empty() {
            filter.push('\n');
        }
        filter.push_str(&format!("{} ({})", patterns, f.name));
    }
    let mut cmd = Command::new("kdialog");
    cmd.args(["--getopenfilename", ".", &filter, "--title", title]);
    let out = cmd.output().map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Ok(None);
    }
    let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if path.is_empty() {
        Ok(None)
    } else {
        Ok(Some(path))
    }
}
