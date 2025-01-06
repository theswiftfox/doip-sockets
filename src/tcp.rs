use std::{io, net::SocketAddr};

use doip_codec::{DoipCodec, EncodeError};
use doip_definitions::message::DoipMessage;
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio_util::codec::Framed;

#[derive(Debug)]
pub struct TcpStream {
    pub io: Framed<tokio::net::TcpStream, DoipCodec>,
}

impl TcpStream {
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        match tokio::net::TcpStream::connect(addr).await {
            Ok(stream) => Ok(Self::apply_codec(stream)),
            Err(err) => Err(err),
        }
    }

    fn apply_codec(stream: tokio::net::TcpStream) -> TcpStream {
        TcpStream {
            io: Framed::new(stream, DoipCodec),
        }
    }

    pub async fn send(&mut self, msg: DoipMessage) -> Result<(), EncodeError> {
        self.io.send(msg).await
    }

    // fn peer_addr(&self) -> io::Result<SocketAddr> {
    //     self.io.get_ref().peer_addr()
    // }
}

#[cfg(test)]
mod test_tcp_stream {
    use doip_definitions::{
        header::DoipVersion,
        message::{
            ActivationType, DoipMessage, RoutingActivationRequest, VehicleIdentificationRequest,
        },
    };
    use tokio::io::AsyncReadExt;

    use crate::TcpStream;

    #[tokio::test]
    async fn test_connect() {
        const TESTER_ADDR: &str = "127.0.0.1:8080";

        let listener = tokio::net::TcpListener::bind(TESTER_ADDR).await;

        let stream = TcpStream::connect(TESTER_ADDR).await;

        assert!(stream.is_ok());

        // Cleanup
        drop(listener);
    }

    #[tokio::test]
    async fn test_send() {
        const TESTER_ADDR: &str = "127.0.0.1:0";

        let listener = tokio::net::TcpListener::bind(TESTER_ADDR).await;

        let stream = TcpStream::connect(TESTER_ADDR).await;

        let mut stream = stream.unwrap();
        let listener = listener.unwrap();

        let (mut socket, _) = listener.accept().await.unwrap();

        let routing_activation = DoipMessage::new(
            DoipVersion::Iso13400_2012,
            Box::new(RoutingActivationRequest {
                source_address: [0x0e, 0x80],
                activation_type: ActivationType::Default,
                buffer: [0, 0, 0, 0],
            }),
        );

        let bytes = routing_activation.to_bytes();

        let _ = &stream.send(routing_activation).await;

        let mut buffer = [0; 64];
        let read = socket.read(&mut buffer).await.unwrap();

        assert_eq!(&buffer[..read], &bytes);

        // Cleanup
        drop(socket);
    }
}

// use std::io;

// use doip_codec::DoipCodec;
// use tokio::net::ToSocketAddrs;
// use tokio_util::codec::Framed;

// struct TCPTEST {}

// impl TCPTEST {
//     pub async fn connect<A: ToSocketAddrs>(
//         addr: A,
//     ) -> io::Result<Framed<tokio::net::TcpStream, DoipCodec>> {
//         let stream = tokio::net::TcpStream::connect(addr).await?;
//         Ok(Framed::new(stream, DoipCodec))
//     }
// }
