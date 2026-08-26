use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiPrefs {
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_locale() -> String {
    "zh".into()
}

fn default_theme() -> String {
    "light".into()
}

impl Default for UiPrefs {
    fn default() -> Self {
        Self {
            locale: default_locale(),
            theme: default_theme(),
        }
    }
}

impl UiPrefs {
    pub fn locale_index(&self) -> i32 {
        if self.locale.eq_ignore_ascii_case("en") {
            1
        } else {
            0
        }
    }

    pub fn theme_index(&self) -> i32 {
        if self.theme.eq_ignore_ascii_case("dark") {
            1
        } else {
            0
        }
    }

    pub fn set_locale_index(&mut self, index: i32) {
        self.locale = if index == 1 { "en".into() } else { "zh".into() };
    }

    pub fn set_theme_index(&mut self, index: i32) {
        self.theme = if index == 1 {
            "dark".into()
        } else {
            "light".into()
        };
    }
}

pub fn default_prefs_path() -> PathBuf {
    let base = std::env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."));
    #[cfg(target_os = "macos")]
    {
        let dir = base.join("Library/Application Support/com.orbien.desktop");
        let _ = fs::create_dir_all(&dir);
        return dir.join("ui-prefs.toml");
    }
    #[cfg(not(target_os = "macos"))]
    {
        let dir = base.join(".config").join("orbien");
        let _ = fs::create_dir_all(&dir);
        dir.join("ui-prefs.toml")
    }
}

pub fn load() -> UiPrefs {
    let path = default_prefs_path();
    match load_from(&path) {
        Ok(prefs) => prefs,
        Err(e) => {
            tracing::debug!(path = %path.display(), ?e, "ui prefs not loaded; using defaults");
            UiPrefs::default()
        }
    }
}

fn load_from(path: &PathBuf) -> Result<UiPrefs> {
    if !path.is_file() {
        return Ok(UiPrefs::default());
    }
    let raw = fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    let prefs: UiPrefs =
        toml::from_str(&raw).with_context(|| format!("parse {}", path.display()))?;
    Ok(normalize(prefs))
}

pub fn save(prefs: &UiPrefs) -> Result<()> {
    let path = default_prefs_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("mkdir {}", parent.display()))?;
    }
    let body = toml::to_string_pretty(prefs).context("serialize ui prefs")?;
    fs::write(&path, body).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

fn normalize(mut prefs: UiPrefs) -> UiPrefs {
    if !prefs.locale.eq_ignore_ascii_case("en") {
        prefs.locale = "zh".into();
    } else {
        prefs.locale = "en".into();
    }
    if prefs.theme.eq_ignore_ascii_case("dark") {
        prefs.theme = "dark".into();
    } else {
        prefs.theme = "light".into();
    }
    prefs
}
