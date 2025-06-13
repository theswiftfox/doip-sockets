use crate::{new_header, SocketConfig};
use doip_codec::{DoipCodec, Error as CodecError};
use doip_definitions::{header::ProtocolVersion, message::DoipMessage, payload::DoipPayload};
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
    /// Creates a new UDP Socket from an `std::net::UdpSocket`
    /// This can be used in conjunction with `socket2`’s `Socket`` interface to configure a socket before it’s handed off, such as setting options like `reuse_address`
    pub fn from_std(sock: std::net::UdpSocket) -> io::Result<UdpSocket> {
        let sock = TokioUdpSocket::from_std(sock)?;

        Ok(UdpSocket {
            io: UdpFramed::new(sock, DoipCodec {}),
            config: SocketConfig::default(),
        })
    }

    /// Bind the socket to a local address
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<UdpSocket> {
        let sock = TokioUdpSocket::bind(addr).await?;

        Ok(UdpSocket {
            io: UdpFramed::new(sock, DoipCodec {}),
            config: SocketConfig::default(),
        })
    }

    /// Connect to a remote address
    pub async fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<()> {
        self.io.get_ref().connect(addr).await
    }

    /// Receive a DoIP Frame from the socket queue
    pub async fn recv(&mut self) -> Option<Result<(DoipMessage, SocketAddr), CodecError>> {
        self.io.next().await
    }

    /// Send a DoIP Frame
    pub async fn send(&mut self, payload: DoipPayload, addr: SocketAddr) -> Result<(), CodecError> {
        let msg = DoipMessage {
            header: new_header(self.config.protocol_version, &payload),
            payload,
        };
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
    pub fn set_protocol_version(&mut self, version: ProtocolVersion) {
        self.config.protocol_version = version
    }
}

#[cfg(test)]
mod test_udp_socket {
    use std::net::ToSocketAddrs;

    use doip_definitions::{
        header::PayloadType,
        payload::{DoipPayload, VehicleIdentificationRequest},
    };

    use super::UdpSocket;

    #[tokio::test]
    async fn test_read_write() {
        const TESTER_ADDR1: &str = "127.0.0.1:8080";
        const TESTER_ADDR2: &str = "127.0.0.1:8081";
        let socket_addr = TESTER_ADDR2.to_socket_addrs().unwrap().next().unwrap();
        let routing_activation =
            DoipPayload::VehicleIdentificationRequest(VehicleIdentificationRequest {});

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
