#![warn(missing_docs)]

mod error;
pub mod tcp;
pub mod udp;

use doip_definitions::header::DoipVersion;

#[derive(Debug, Copy, Clone)]
pub struct SocketConfig {
    protocol_version: DoipVersion,
}
