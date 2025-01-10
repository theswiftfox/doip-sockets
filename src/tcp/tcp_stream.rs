use std::io::{self};

use doip_codec::{DecodeError, DoipCodec};
use doip_definitions::{
    header::{DoipPayload, DoipVersion},
    message::DoipMessage,
};
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpStream as TokioTcpStream, ToSocketAddrs};
use tokio_util::codec::{Framed, FramedRead, FramedWrite};

use crate::error::SocketSendError;

use super::{
    tcp_split::{TcpStreamReadHalf, TcpStreamWriteHalf},
    DoipTcpPayload, SocketConfig,
};

#[derive(Debug)]
pub struct TcpStream {
    io: Framed<TokioTcpStream, DoipCodec>,
    config: SocketConfig,
}

impl TcpStream {
    pub fn new(io: Framed<TokioTcpStream, DoipCodec>) -> Self {
        TcpStream {
            io,
            config: SocketConfig {
                protocol_version: DoipVersion::Iso13400_2012,
            },
        }
    }

    pub async fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        match TokioTcpStream::connect(addr).await {
            Ok(stream) => Ok(Self::apply_codec(stream)),
            Err(err) => Err(err),
        }
    }

    fn apply_codec(stream: TokioTcpStream) -> TcpStream {
        TcpStream {
            io: Framed::new(stream, DoipCodec),
            config: SocketConfig {
                protocol_version: DoipVersion::Iso13400_2012,
            },
        }
    }

    pub async fn send<A: DoipTcpPayload + DoipPayload + 'static>(
        &mut self,
        payload: A,
    ) -> Result<(), SocketSendError> {
        let msg = DoipMessage::new(self.config.protocol_version, Box::new(payload));

        match self.io.send(msg).await {
            Ok(_) => Ok(()),
            Err(err) => Err(SocketSendError::EncodeError(err)),
        }
    }

    pub async fn read(&mut self) -> Option<Result<DoipMessage, DecodeError>> {
        self.io.next().await
    }

    pub fn from_std(stream: std::net::TcpStream) -> io::Result<TcpStream> {
        let stream = TokioTcpStream::from_std(stream)?;
        Ok(Self::apply_codec(stream))
    }

    pub fn into_split(self) -> (TcpStreamReadHalf, TcpStreamWriteHalf) {
        let stream: TokioTcpStream = self.io.into_inner();

        let (r_half, w_half) = tokio::io::split(stream);

        let read = FramedRead::new(r_half, DoipCodec);
        let write = FramedWrite::new(w_half, DoipCodec);

        (
            TcpStreamReadHalf::new(read, Some(self.config)),
            TcpStreamWriteHalf::new(write, Some(self.config)),
        )
    }

    pub fn get_socket_ref(&self) -> &TokioTcpStream {
        self.io.get_ref()
    }

    pub fn into_socket(self) -> TokioTcpStream {
        self.io.into_inner()
    }
}

#[cfg(test)]
mod test_tcp_stream {
    use doip_codec::DoipCodec;
    use doip_definitions::{
        header::DoipPayload,
        message::{
            ActivationCode, ActivationType, RoutingActivationRequest, RoutingActivationResponse,
        },
    };
    use tokio::io::AsyncReadExt;
    use tokio_util::codec::Framed;

    use crate::tcp::tcp_stream::TcpStream;

    #[tokio::test]
    async fn test_connect() {
        const TESTER_ADDR: &str = "127.0.0.1:0";

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
        let listener = listener.unwrap();

        let stream = TcpStream::connect(listener.local_addr().unwrap()).await;

        let mut stream = stream.unwrap();

        let (mut socket, _) = listener.accept().await.unwrap();

        let routing_activation = RoutingActivationRequest {
            source_address: [0x0e, 0x80],
            activation_type: ActivationType::Default,
            buffer: [0, 0, 0, 0],
        };

        let bytes = routing_activation.to_bytes();

        let _ = &stream.send(routing_activation).await;

        let mut buffer = [0; 64];
        let read = socket.read(&mut buffer).await.unwrap();

        assert_eq!(&buffer[8..read], &bytes);

        // Cleanup
        drop(socket);
    }

    #[tokio::test]
    async fn test_read() {
        const TESTER_ADDR: &str = "127.0.0.1:0";
        let routing_activation = RoutingActivationRequest {
            source_address: [0x0e, 0x80],
            activation_type: ActivationType::Default,
            buffer: [0, 0, 0, 0],
        };

        let listener = tokio::net::TcpListener::bind(TESTER_ADDR).await;
        let listener = listener.unwrap();

        let client = TcpStream::connect(listener.local_addr().unwrap()).await;
        let mut client = client.unwrap();

        let (socket, _) = listener.accept().await.unwrap();
        let mut server = TcpStream::new(Framed::new(socket, DoipCodec));

        let _ = &client.send(routing_activation).await;
        let _ = server.read().await.unwrap().unwrap();

        let routing_activation_res = RoutingActivationResponse {
            logical_address: [0x0e, 0x80],
            source_address: [0x14, 0x11],
            activation_code: ActivationCode::SuccessfullyActivated,
            buffer: [0, 0, 0, 0],
        };
        let bytes = routing_activation_res.to_bytes();

        let _ = server.send(routing_activation_res).await;
        let echo = client.read().await.unwrap().unwrap();

        assert_eq!(echo.to_bytes()[8..], bytes)
    }

    #[tokio::test]
    async fn test_into_split() {
        const TESTER_ADDR: &str = "127.0.0.1:0";
        let routing_activation = RoutingActivationRequest {
            source_address: [0x0e, 0x80],
            activation_type: ActivationType::Default,
            buffer: [0, 0, 0, 0],
        };

        let listener = tokio::net::TcpListener::bind(TESTER_ADDR).await;
        let listener = listener.unwrap();

        let client = TcpStream::connect(listener.local_addr().unwrap()).await;
        let client = client.unwrap();
        let (mut read, mut write) = client.into_split();

        let (socket, _) = listener.accept().await.unwrap();
        let mut server = TcpStream::new(Framed::new(socket, DoipCodec));

        let _ = write.send(routing_activation).await;
        let _ = server.read().await.unwrap().unwrap();

        let routing_activation_res = RoutingActivationResponse {
            logical_address: [0x0e, 0x80],
            source_address: [0x14, 0x11],
            activation_code: ActivationCode::SuccessfullyActivated,
            buffer: [0, 0, 0, 0],
        };
        let bytes = routing_activation_res.to_bytes();

        let _ = server.send(routing_activation_res).await;
        let echo = read.read().await.unwrap().unwrap();

        assert_eq!(echo.to_bytes()[8..], bytes)
    }
}
