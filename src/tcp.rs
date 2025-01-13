use doip_definitions::{
    header::DoipVersion,
    message::{
        AliveCheckRequest, AliveCheckResponse, DiagnosticMessage, DiagnosticMessageAck,
        DiagnosticMessageNack, GenericNack, RoutingActivationRequest, RoutingActivationResponse,
    },
};

use crate::SocketConfig;

mod tcp_split;
mod tcp_stream;
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
            protocol_version: DoipVersion::DefaultValue,
        }
    }
}
