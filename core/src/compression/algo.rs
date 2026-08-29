use anyhow::{bail, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CompressionAlgo {
    #[default]
    None,
    Lz4,
}

impl CompressionAlgo {
    pub fn parse(raw: &str) -> Result<Self> {
        let s = raw.trim();
        if s.is_empty() || s.eq_ignore_ascii_case("none") {
            return Ok(Self::None);
        }
        if s.eq_ignore_ascii_case("lz4") {
            return Ok(Self::Lz4);
        }
        bail!("unsupported compression {raw:?}; use \"none\" | \"lz4\"")
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Lz4 => "lz4",
        }
    }

    pub fn is_none(self) -> bool {
        matches!(self, Self::None)
    }

    pub fn wire_str(self) -> String {
        match self {
            Self::None => String::new(),
            Self::Lz4 => "lz4".into(),
        }
    }
}
