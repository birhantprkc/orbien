mod quantity;
mod stream;
mod token_bucket;

use std::sync::Arc;

pub use quantity::{parse_bandwidth_limit, BandwidthLimitMode};
pub use stream::{maybe_limit, LimitedStream};
pub use token_bucket::BandwidthLimiter;

pub fn limiter_if_mode(
    bandwidth_limit: &str,
    bandwidth_limit_mode: &str,
    want: BandwidthLimitMode,
) -> anyhow::Result<Option<Arc<BandwidthLimiter>>> {
    let bytes = parse_bandwidth_limit(bandwidth_limit)?;
    if bytes == 0 {
        return Ok(None);
    }
    if BandwidthLimitMode::parse(bandwidth_limit_mode) != want {
        return Ok(None);
    }
    Ok(Some(Arc::new(BandwidthLimiter::new(bytes))))
}
