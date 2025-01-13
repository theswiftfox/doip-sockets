#![warn(unused_extern_crates)]
#![warn(missing_docs)]
//! Simple socket implementations which wrap Tokio Sockets with a codec for
//! Diagnostics over Internet Protocol.
//!
//! This crate is built for those wants strong type checking for sending and receiving DoIP messages
//! from sockets. Each UDP or TCP IO has typed alongside it the supported messaging formats so only
//! those payloads can be sent. This reduces the number of error paths which users code can go down.
//!
//! Each socket implementation is light, there is not a 1:1 with the Tokio sockets,
//! however users can access the inner Framed and then Tokio sockets through helper methods.
//!
//! Along side the sockets this crate include the ability to create a config
//! currently of which is solely limited to the version of the protocol used,
//! however can be extended in future version.

use doip_definitions::header::DoipVersion;
mod error;

/// Simple TCP Stream and Split implentation for a TCP Stream allowing the conversion of a
/// socket into a stream for Codec use, or the creating of a new TCP Stream
/// from scratch.
pub mod tcp;

/// Simple UDP Socket implementation for UDP communication.
pub mod udp;

/// Configuration for UDP and TCP Sockets
///
/// This provides the methods within each struct with constants which can be set
/// during a typical usage with DoIP such as the protocol_version.
#[derive(Debug, Copy, Clone)]
pub struct SocketConfig {
    protocol_version: DoipVersion,
}
