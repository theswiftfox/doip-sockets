use super::DoipUdpPayload;
use crate::SocketConfig;
use doip_codec::{DecodeError, DoipCodec, EncodeError};
use doip_definitions::{
    header::{DoipPayload, DoipVersion},
    message::DoipMessage,
};
use futures::{SinkExt, StreamExt};
use std::{io, net::SocketAddr};
use tokio::net::{ToSocketAddrs, UdpSocket as TokioUdpSocket};
use tokio_util::udp::UdpFramed;

/// Simple implementation of a UDP Socket with DoIP Frames
///
/// Applying only the most simple methods on this struct it is able to act as
/// a simple UDP socket. If extended functionality is required you can access the
/// inner Tokio UDP Socket, or raise a Issue on GitHub.
pub struct UdpSocket {
    io: UdpFramed<DoipCodec, TokioUdpSocket>,
    config: SocketConfig,
}

impl UdpSocket {
    /// Bind the socket to a local address
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<UdpSocket> {
        let sock = TokioUdpSocket::bind(addr).await?;

        Ok(UdpSocket {
            io: UdpFramed::new(sock, DoipCodec),
            config: SocketConfig::default(),
        })
    }

    /// Connect to a remote address
    pub async fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        self.io.get_ref().connect(addr).await
    }

    /// Receive a DoIP Frame from the socket queue
    pub async fn recv(&mut self) -> Option<Result<(DoipMessage, SocketAddr), DecodeError>> {
        self.io.next().await
    }

    /// Send a DoIP Frame
    pub async fn send<A: DoipUdpPayload + DoipPayload + 'static>(
        &mut self,
        payload: A,
        addr: SocketAddr,
    ) -> Result<(), EncodeError> {
        let msg = DoipMessage::new(self.config.protocol_version, Box::new(payload));
        self.io.send((msg, addr)).await
    }

    /// Get a reference to the inner Tokio UDP Socket
    pub fn get_socket_ref(&self) -> &TokioUdpSocket {
        self.io.get_ref()
    }

    /// Access the inner Tokio UDP Socket, consumes the DoIP UDP Socket
    pub fn into_socket(self) -> TokioUdpSocket {
        self.io.into_inner()
    }

    /// Change the protocol version on the socket
    pub fn set_protocol_version(&mut self, version: DoipVersion) {
        self.config.protocol_version = version
    }
}

#[cfg(test)]
mod test_udp_socket {
    use std::net::ToSocketAddrs;

    use doip_definitions::{header::PayloadType, message::VehicleIdentificationRequest};

    use super::UdpSocket;

    #[tokio::test]
    async fn test_read_write() {
        const TESTER_ADDR1: &str = "127.0.0.1:8080";
        const TESTER_ADDR2: &str = "127.0.0.1:8081";
        let socket_addr = TESTER_ADDR2.to_socket_addrs().unwrap().next().unwrap();
        let routing_activation = VehicleIdentificationRequest {};

        let mut sock1 = UdpSocket::bind(TESTER_ADDR1).await.unwrap();

        let mut sock2 = UdpSocket::bind(TESTER_ADDR2).await.unwrap();

        sock1.connect(TESTER_ADDR2).await.unwrap();
        sock1.send(routing_activation, socket_addr).await.unwrap();

        let (res, addr) = sock2.recv().await.unwrap().unwrap();

        assert!(res.header.payload_type == PayloadType::VehicleIdentificationRequest);
        assert!(res.header.payload_length == 0);
        assert!(addr == TESTER_ADDR1.to_socket_addrs().unwrap().next().unwrap());
    }
}
