mod error;
mod tcp;
mod udp;

use doip_definitions::header::DoipVersion;
pub use tcp::*;
pub use udp::*;

#[derive(Debug, Copy, Clone)]
pub struct SocketConfig {
    protocol_version: DoipVersion,
}
