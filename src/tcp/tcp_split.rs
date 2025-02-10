use doip_codec::{DecodeError, DoipCodec};
use doip_definitions::{header::DoipPayload, message::DoipMessage};
use futures::{SinkExt, StreamExt};
use tokio::io::{AsyncRead, AsyncWrite, ReadHalf, WriteHalf};
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::error::SocketSendError;

use super::{DoipTcpPayload, SocketConfig};

/// Simple implementation of a TCP Stream Read Half
///
/// Allows for the passing of the read half being passed into a different thread
/// seperate to the write half. Will be dropped if the Write Half is dropped.
pub struct TcpStreamReadHalf<T>
where
    T: AsyncRead + AsyncWrite,
{
    io: FramedRead<ReadHalf<T>, DoipCodec>,
    #[allow(dead_code)]
    config: SocketConfig,
}

impl<T> TcpStreamReadHalf<T>
where
    T: AsyncRead + AsyncWrite,
{
    /// Creates a new TCP Stream Read Half from an existing Tokio TCP Stream and
    /// config
    pub fn new(io: FramedRead<ReadHalf<T>, DoipCodec>, config: Option<SocketConfig>) -> Self {
        TcpStreamReadHalf {
            io,
            config: config.unwrap_or_default(),
        }
    }

    /// Read from the stream
    pub async fn read(&mut self) -> Option<Result<DoipMessage, DecodeError>> {
        self.io.next().await
    }
}

/// Simple implementation of a TCP Stream Write Half
///
/// Can be used to write messages to the sink. If dropped this will close the
/// connection on the TcpStreamReadHalf.
pub struct TcpStreamWriteHalf<T>
where
    T: AsyncRead + AsyncWrite,
{
    io: FramedWrite<WriteHalf<T>, DoipCodec>,
    config: SocketConfig,
}

impl<T> TcpStreamWriteHalf<T>
where
    T: AsyncRead + AsyncWrite,
{
    /// Creates a new TCP Stream Read Half from an existing Tokio TCP Stream and
    /// config
    pub fn new(io: FramedWrite<WriteHalf<T>, DoipCodec>, config: Option<SocketConfig>) -> Self {
        TcpStreamWriteHalf {
            io,
            config: config.unwrap_or_default(),
        }
    }

    /// Send a message to the sink
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
}
