mod proxy_protocol;
mod xff;

pub use proxy_protocol::{
    addrs_from_start_work, build_proxy_protocol_header, parse_proxy_protocol_version,
    try_consume_proxy_protocol, ParsedProxyHeader, PpConsume, PROXY_PROTOCOL_MAX_HEADER,
};
pub use xff::apply_x_forwarded_for;
