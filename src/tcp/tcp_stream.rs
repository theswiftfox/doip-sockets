use std::io::{self};

use doip_codec::{DecodeError, DoipCodec};
use doip_definitions::{header::ProtocolVersion, message::DoipMessage, payload::DoipPayload};
use futures::{SinkExt, StreamExt};
use tokio::net::{TcpStream as TokioTcpStream, ToSocketAddrs};
use tokio_util::codec::{Framed, FramedRead, FramedWrite};

use crate::{error::SocketSendError, new_header};

use super::{
    tcp_split::{TcpStreamReadHalf, TcpStreamWriteHalf},
    SocketConfig,
};
/// Simple implementation of a TCP Stream
///
/// Applying only the most simple methods on this struct it is able to act as
/// a simple TCP stream. If extended functionality is required you can access the
/// inner Tokio TCP Stream, or raise a Issue on GitHub.
#[derive(Debug)]
pub struct TcpStream {
    io: Framed<TokioTcpStream, DoipCodec>,
    config: SocketConfig,
}

impl TcpStream {
    /// Creates a new TCP Stream from a Tokio TCP Stream
    pub fn new(io: TokioTcpStream) -> Self {
        TcpStream {
            io: Framed::new(io, DoipCodec {}),
            config: SocketConfig {
                protocol_version: ProtocolVersion::Iso13400_2012,
            },
        }
    }

    /// Creates a new TCP Stream given a remote address
    pub async fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        match TokioTcpStream::connect(addr).await {
            Ok(stream) => Ok(Self::apply_codec(stream)),
            Err(err) => Err(err),
        }
    }

    fn apply_codec(stream: TokioTcpStream) -> TcpStream {
        TcpStream {
            io: Framed::new(stream, DoipCodec {}),
            config: SocketConfig {
                protocol_version: ProtocolVersion::Iso13400_2012,
            },
        }
    }

    /// Send a DoIP frame to the sink
    pub async fn send(&mut self, payload: DoipPayload) -> Result<(), SocketSendError> {
        let msg = DoipMessage {
            header: new_header(self.config.protocol_version, &payload),
            payload,
        };

        match self.io.send(msg).await {
            Ok(_) => Ok(()),
            Err(err) => Err(SocketSendError::EncodeError(err)),
        }
    }

    /// Read a DoIP frame off the stream
    pub async fn read(&mut self) -> Option<Result<DoipMessage, DecodeError>> {
        self.io.next().await
    }

    /// Converts a standard library TCP Stream to a DoIP Framed TCP Stream
    pub fn from_std(stream: std::net::TcpStream) -> io::Result<TcpStream> {
        let stream = TokioTcpStream::from_std(stream)?;
        Ok(Self::apply_codec(stream))
    }

    /// Splits the TCP Stream into a Read Half and Write Half
    pub fn into_split(
        self,
    ) -> (
        TcpStreamReadHalf<TokioTcpStream>,
        TcpStreamWriteHalf<TokioTcpStream>,
    ) {
        let stream: TokioTcpStream = self.io.into_inner();

        let (r_half, w_half) = tokio::io::split(stream);

        let read = FramedRead::new(r_half, DoipCodec {});
        let write = FramedWrite::new(w_half, DoipCodec {});

        (
            TcpStreamReadHalf::new(read, Some(self.config)),
            TcpStreamWriteHalf::new(write, Some(self.config)),
        )
    }

    /// Get a reference to the inner Tokio TCP Stream
    pub fn get_stream_ref(&self) -> &TokioTcpStream {
        self.io.get_ref()
    }

    /// Access the inner Tokio TCP Stream, consumes the DoIP TCP Stream
    pub fn into_socket(self) -> TokioTcpStream {
        self.io.into_inner()
    }
}

#[cfg(test)]
mod test_tcp_stream {
    use doip_codec::Encoder;
    use doip_definitions::{
        message::DoipMessage,
        payload::{
            ActivationCode, ActivationType, DoipPayload, RoutingActivationRequest,
            RoutingActivationResponse,
        },
    };
    use tokio::io::AsyncReadExt;

    use crate::{new_header, tcp::tcp_stream::TcpStream};

    #[tokio::test]
    async fn test_connect() {
        const TESTER_ADDR: &str = "127.0.0.1:0";

        let listener = tokio::net::TcpListener::bind(TESTER_ADDR).await.unwrap();
        let addr = listener.local_addr().unwrap();

        let stream = TcpStream::connect(addr).await;
        dbg!(&stream);

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

        let routing_activation_payload =
            DoipPayload::RoutingActivationRequest(RoutingActivationRequest {
                source_address: [0x0e, 0x80],
                activation_type: ActivationType::Default,
                buffer: [0, 0, 0, 0],
            });
        let routing_activation = DoipMessage {
            header: new_header(stream.config.protocol_version, &routing_activation_payload),
            payload: routing_activation_payload.clone(),
        };

        let mut codec = doip_codec::DoipCodec {};
        let mut bytes = Vec::<u8>::new();
        codec.to_bytes(routing_activation, &mut bytes).unwrap();

        let _ = &stream.send(routing_activation_payload).await;

        let mut buffer = [0; 64];
        let read = socket.read(&mut buffer).await.unwrap();

        assert_eq!(&buffer[8..read], &bytes[8..read]);

        // Cleanup
        drop(socket);
    }

    #[tokio::test]
    async fn test_read() {
        const TESTER_ADDR: &str = "127.0.0.1:0";
        let routing_activation = DoipPayload::RoutingActivationRequest(RoutingActivationRequest {
            source_address: [0x0e, 0x80],
            activation_type: ActivationType::Default,
            buffer: [0, 0, 0, 0],
        });

        let listener = tokio::net::TcpListener::bind(TESTER_ADDR).await;
        let listener = listener.unwrap();

        let client = TcpStream::connect(listener.local_addr().unwrap()).await;
        let mut client = client.unwrap();

        let (socket, _) = listener.accept().await.unwrap();
        let mut server = TcpStream::new(socket);

        let _ = &client.send(routing_activation).await;
        let _ = server.read().await.unwrap().unwrap();

        let routing_activation_res =
            DoipPayload::RoutingActivationResponse(RoutingActivationResponse {
                logical_address: [0x0e, 0x80],
                source_address: [0x14, 0x11],
                activation_code: ActivationCode::SuccessfullyActivated,
                buffer: [0, 0, 0, 0],
            });

        let _ = server.send(routing_activation_res.clone()).await;
        let echo = client.read().await.unwrap().unwrap();

        assert_eq!(echo.payload, routing_activation_res)
    }

    #[tokio::test]
    async fn test_into_split() {
        const TESTER_ADDR: &str = "127.0.0.1:0";
        let routing_activation = DoipPayload::RoutingActivationRequest(RoutingActivationRequest {
            source_address: [0x0e, 0x80],
            activation_type: ActivationType::Default,
            buffer: [0, 0, 0, 0],
        });

        let listener = tokio::net::TcpListener::bind(TESTER_ADDR).await;
        let listener = listener.unwrap();

        let client = TcpStream::connect(listener.local_addr().unwrap()).await;
        let client = client.unwrap();
        let (mut read, mut write) = client.into_split();

        let (socket, _) = listener.accept().await.unwrap();
        let mut server = TcpStream::new(socket);

        let _ = write.send(routing_activation).await;
        let _ = server.read().await.unwrap().unwrap();

        let routing_activation_res =
            DoipPayload::RoutingActivationResponse(RoutingActivationResponse {
                logical_address: [0x0e, 0x80],
                source_address: [0x14, 0x11],
                activation_code: ActivationCode::SuccessfullyActivated,
                buffer: [0, 0, 0, 0],
            });

        let _ = server.send(routing_activation_res.clone()).await;
        let echo = read.read().await.unwrap().unwrap();

        assert_eq!(echo.payload, routing_activation_res)
    }
}
