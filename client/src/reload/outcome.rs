use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReloadLevel {
    #[default]
    Noop,
    TunnelsOnly,
    ReconnectControl,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReloadOutcome {
    pub level: ReloadLevel,
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub updated: Vec<String>,
    pub failed: Vec<(String, String)>,
    #[serde(default)]
    pub connection_settings_changed: bool,
}

impl ReloadLevel {
    pub fn label(self) -> &'static str {
        match self {
            ReloadLevel::Noop => "noop",
            ReloadLevel::TunnelsOnly => "tunnels",
            ReloadLevel::ReconnectControl => "reconnect",
        }
    }
}

impl ReloadOutcome {
    pub fn succeeded(&self) -> bool {
        self.failed.is_empty()
    }
}
