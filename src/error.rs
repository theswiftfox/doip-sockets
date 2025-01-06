#[derive(thiserror::Error, Debug)]
pub enum SocketSendError {
    /// Encode error from Codec
    #[error("Underlying Codec Error: {0}")]
    EncodeError(#[from] doip_codec::EncodeError),

    /// Payload Type not supported by TCP Socket
    #[error("Payload Type not supported by TCP Socket")]
    InvalidTcpPayload,
}
