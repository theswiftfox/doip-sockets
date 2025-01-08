use doip_definitions::{header::DoipVersion, message::{
    AliveCheckRequest, AliveCheckResponse, DiagnosticMessage, DiagnosticMessageAck,
    DiagnosticMessageNack, GenericNack, RoutingActivationRequest, RoutingActivationResponse,
}};

pub mod tcp_split;
pub mod tcp_stream;

pub trait DoipTcpPayload {}

impl DoipTcpPayload for GenericNack {}
impl DoipTcpPayload for RoutingActivationRequest {}
impl DoipTcpPayload for RoutingActivationResponse {}
impl DoipTcpPayload for AliveCheckRequest {}
impl DoipTcpPayload for AliveCheckResponse {}
impl DoipTcpPayload for DiagnosticMessage {}
impl DoipTcpPayload for DiagnosticMessageAck {}
impl DoipTcpPayload for DiagnosticMessageNack {}

#[derive(Debug, Copy, Clone)]
pub struct SocketConfig {
    protocol_version: DoipVersion,
}
