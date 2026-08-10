use anyhow::{bail, Result};

pub const KB: u64 = 1024;
pub const MB: u64 = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BandwidthLimitMode {
    Client,
    Server,
}

impl BandwidthLimitMode {
    pub fn parse(s: &str) -> Self {
        match s.trim().to_ascii_lowercase().as_str() {
            "server" => Self::Server,
            _ => Self::Client,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Client => "client",
            Self::Server => "server",
        }
    }
}

pub fn parse_bandwidth_limit(s: &str) -> Result<u64> {
    let s = s.trim();
    if s.is_empty() {
        return Ok(0);
    }
    if let Some(num) = s.strip_suffix("MB").or_else(|| s.strip_suffix("mb")) {
        let f: f64 = num.trim().parse()?;
        return Ok((f * MB as f64) as u64);
    }
    if let Some(num) = s.strip_suffix("KB").or_else(|| s.strip_suffix("kb")) {
        let f: f64 = num.trim().parse()?;
        return Ok((f * KB as f64) as u64);
    }
    bail!("bandwidthLimit unit not supported (use KB or MB), got {s:?}");
}
