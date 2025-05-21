use doip_definitions::{
    header::ProtocolVersion,
    payload::{
        AliveCheckRequest, AliveCheckResponse, DiagnosticMessage, DiagnosticMessageAck,
        DiagnosticMessageNack, GenericNack, RoutingActivationRequest, RoutingActivationResponse,
    },
};

use crate::SocketConfig;

mod ssl_stream;
mod tcp_listener;
mod tcp_socket;
mod tcp_split;
mod tcp_stream;
pub use crate::tcp::ssl_stream::*;
pub use crate::tcp::tcp_listener::*;
pub use crate::tcp::tcp_socket::*;
pub use crate::tcp::tcp_split::*;
pub use crate::tcp::tcp_stream::*;

/// Helper Trait which assists in applying LSP hints to the send and receive of
/// sockets.
pub trait DoipTcpPayload {}

impl DoipTcpPayload for GenericNack {}
impl DoipTcpPayload for RoutingActivationRequest {}
impl DoipTcpPayload for RoutingActivationResponse {}
impl DoipTcpPayload for AliveCheckRequest {}
impl DoipTcpPayload for AliveCheckResponse {}
impl DoipTcpPayload for DiagnosticMessage {}
impl DoipTcpPayload for DiagnosticMessageAck {}
impl DoipTcpPayload for DiagnosticMessageNack {}

impl Default for SocketConfig {
    fn default() -> Self {
        Self {
            protocol_version: ProtocolVersion::DefaultValue,
        }
    }
}
