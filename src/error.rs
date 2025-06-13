#[derive(thiserror::Error, Debug)]
pub enum SocketSendError {
    /// Encode error from Codec
    #[error("Underlying Codec Error: {0}")]
    EncodeError(doip_codec::Error),

    /// Payload Type not supported by TCP Socket
    #[error("Payload Type not supported by TCP Socket")]
    InvalidTcpPayload,
}
