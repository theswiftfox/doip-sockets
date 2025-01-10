use doip_definitions::message::{
    EntityStatusRequest, EntityStatusResponse, GenericNack, PowerInformationRequest,
    PowerInformationResponse, VehicleAnnouncementMessage, VehicleIdentificationRequest,
    VehicleIdentificationRequestEid, VehicleIdentificationRequestVin,
};

mod udp_socket;

pub use crate::udp::udp_socket::*;

pub trait DoipUdpPayload {}

impl DoipUdpPayload for GenericNack {}
impl DoipUdpPayload for VehicleIdentificationRequest {}
impl DoipUdpPayload for VehicleIdentificationRequestEid {}
impl DoipUdpPayload for VehicleIdentificationRequestVin {}
impl DoipUdpPayload for VehicleAnnouncementMessage {}
impl DoipUdpPayload for EntityStatusRequest {}
impl DoipUdpPayload for EntityStatusResponse {}
impl DoipUdpPayload for PowerInformationRequest {}
impl DoipUdpPayload for PowerInformationResponse {}
