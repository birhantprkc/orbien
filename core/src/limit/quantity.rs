use anyhow::{bail, Result};

pub fn mbps_to_bytes_per_sec(mbps: f64) -> u64 {
    if mbps <= 0.0 || !mbps.is_finite() {
        return 0;
    }
    (mbps * 1_000_000.0 / 8.0).round() as u64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BandwidthLimitSide {
    Client,
    Server,
}

impl BandwidthLimitSide {
    pub fn parse(s: &str) -> Self {
        if s.trim().eq_ignore_ascii_case("server") {
            Self::Server
        } else {
            Self::Client
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Client => "client",
            Self::Server => "server",
        }
    }
}

pub fn parse_bandwidth_mbps(raw: &str) -> Result<f64> {
    let s = raw.trim();
    if s.is_empty() {
        return Ok(0.0);
    }
    let n: f64 = s
        .parse()
        .map_err(|_| anyhow::anyhow!("invalid bandwidth (Mbps number expected), got {s:?}"))?;
    if !n.is_finite() || n < 0.0 {
        bail!("invalid bandwidth value: {s:?}");
    }
    Ok(n)
}
