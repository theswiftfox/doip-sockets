use doip_codec::{DecodeError, DoipCodec};
use doip_definitions::{header::DoipPayload, message::DoipMessage};
use futures::{SinkExt, StreamExt};
use tokio::{
    io::{ReadHalf, WriteHalf},
    net::TcpStream as TokioTcpStream,
};
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::error::SocketSendError;

use super::{DoipTcpPayload, SocketConfig};

pub struct TcpStreamReadHalf {
    pub io: FramedRead<ReadHalf<TokioTcpStream>, DoipCodec>,
    pub config: SocketConfig,
}

impl TcpStreamReadHalf {
    pub async fn read(&mut self) -> Option<Result<DoipMessage, DecodeError>> {
        self.io.next().await
    }
}

pub struct TcpStreamWriteHalf {
    pub io: FramedWrite<WriteHalf<TokioTcpStream>, DoipCodec>,
    pub config: SocketConfig,
}

impl TcpStreamWriteHalf {
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
