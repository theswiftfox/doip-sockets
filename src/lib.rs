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

use doip_definitions::{
    header::{DoipHeader, PayloadType, ProtocolVersion},
    payload::{
        AliveCheckResponse, DiagnosticMessageAck, DiagnosticMessageNack, DoipPayload,
        EntityStatusRequest, GenericNack, PowerInformationResponse, RoutingActivationRequest,
        RoutingActivationResponse, VehicleAnnouncementMessage, VehicleIdentificationRequestEid,
        VehicleIdentificationRequestVin,
    },
};
mod error;

/// Simple TCP Stream and Split implentation for a TCP Stream allowing the conversion of a
/// socket into a stream for Codec use, or the creating of a new TCP Stream
/// from scratch.
pub mod tcp;

/// Simple UDP Socket implementation for UDP communication.
pub mod udp;

pub use doip_codec::Error;

/// Configuration for UDP and TCP Sockets
///
/// This provides the methods within each struct with constants which can be set
/// during a typical usage with DoIP such as the protocol_version.
#[derive(Debug, Copy, Clone)]
pub struct SocketConfig {
    protocol_version: ProtocolVersion,
}

pub(crate) fn new_header(protocol_version: ProtocolVersion, payload: &DoipPayload) -> DoipHeader {
    let (payload_length, payload_type) = match payload {
        DoipPayload::GenericNack(_) => (size_of::<GenericNack>(), PayloadType::GenericNack),
        DoipPayload::VehicleIdentificationRequest(_) => {
            (0, PayloadType::VehicleIdentificationRequest)
        }
        DoipPayload::VehicleIdentificationRequestEid(_) => (
            size_of::<VehicleIdentificationRequestEid>(),
            PayloadType::VehicleIdentificationRequestEid,
        ),
        DoipPayload::VehicleIdentificationRequestVin(_) => (
            size_of::<VehicleIdentificationRequestVin>(),
            PayloadType::VehicleIdentificationRequestVin,
        ),
        DoipPayload::VehicleAnnouncementMessage(_) => (
            size_of::<VehicleAnnouncementMessage>(),
            PayloadType::VehicleAnnouncementMessage,
        ),
        DoipPayload::RoutingActivationRequest(_) => (
            size_of::<RoutingActivationRequest>(),
            PayloadType::RoutingActivationRequest,
        ),
        DoipPayload::RoutingActivationResponse(_) => (
            size_of::<RoutingActivationResponse>(),
            PayloadType::RoutingActivationResponse,
        ),
        DoipPayload::AliveCheckRequest(_) => (0, PayloadType::AliveCheckRequest),
        DoipPayload::AliveCheckResponse(_) => (
            size_of::<AliveCheckResponse>(),
            PayloadType::AliveCheckResponse,
        ),
        DoipPayload::EntityStatusRequest(_) => (
            size_of::<EntityStatusRequest>(),
            PayloadType::EntityStatusRequest,
        ),
        DoipPayload::EntityStatusResponse(_) => (0, PayloadType::EntityStatusResponse),
        DoipPayload::PowerInformationRequest(_) => (0, PayloadType::PowerInformationRequest),
        DoipPayload::PowerInformationResponse(_) => (
            size_of::<PowerInformationResponse>(),
            PayloadType::PowerInformationResponse,
        ),
        DoipPayload::DiagnosticMessage(msg) => (
            msg.source_address.len() + msg.target_address.len() + msg.message.len(),
            PayloadType::DiagnosticMessage,
        ),
        DoipPayload::DiagnosticMessageAck(_) => (
            size_of::<DiagnosticMessageAck>(),
            PayloadType::DiagnosticMessageAck,
        ),
        DoipPayload::DiagnosticMessageNack(_) => (
            size_of::<DiagnosticMessageNack>(),
            PayloadType::DiagnosticMessageNack,
        ),
    };

    let inverse_protocol_version = !(protocol_version as u8);

    DoipHeader {
        protocol_version,
        inverse_protocol_version,
        payload_type,
        payload_length: payload_length as u32,
    }
}
