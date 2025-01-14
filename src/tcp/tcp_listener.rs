use std::{io, net::SocketAddr};
use tokio::net::TcpListener as TokioTcpListener;

use super::TcpStream;

/// A TCP socket server, listening for connections
pub struct TcpListener {
    io: TokioTcpListener,
}

impl TcpListener {
    /// Initialised a new TcpListener
    pub fn new(io: TokioTcpListener) -> Self {
        TcpListener { io }
    }

    /// Accepts an new incoming connection from the listener
    pub async fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let (stream, addr) = self.io.accept().await?;

        Ok((TcpStream::new(stream), addr))
    }
}
