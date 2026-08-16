use std::path::{Path, PathBuf};

pub fn path_for(config_path: &Path) -> PathBuf {
    let mut p = config_path.to_path_buf();
    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| format!("{e}.session_id"))
        .unwrap_or_else(|| "session_id".into());
    p.set_extension(ext);
    p
}

fn read_valid_id(path: &Path) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| {
            !s.is_empty() && s.len() <= 64 && s.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
        })
}

pub fn load(config_path: &Path) -> String {
    read_valid_id(&path_for(config_path)).unwrap_or_default()
}

pub fn save(config_path: &Path, session_id: &str) -> std::io::Result<()> {
    let path = path_for(config_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, session_id)
}
