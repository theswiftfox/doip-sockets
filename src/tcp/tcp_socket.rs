use std::{io, net::SocketAddr};

use tokio::net::TcpSocket as TokioTcpSocket;

use super::{TcpListener, TcpStream};

/// A TCP socket that has not yet been converted to a TcpStream or TcpListener
pub struct TcpSocket {
    io: tokio::net::TcpSocket,
}

impl TcpSocket {
    /// Creates a new socket configured for IPv4
    pub fn new_v4() -> io::Result<Self> {
        Ok(TcpSocket {
            io: TokioTcpSocket::new_v4()?,
        })
    }

    /// Establishes a TCP connection with a peer at the specified socket address
    pub async fn connect(self, addr: SocketAddr) -> io::Result<TcpStream> {
        let stream = self.io.connect(addr).await?;
        Ok(TcpStream::new(stream))
    }

    /// Binds the socket to the given address
    pub fn bind(&self, addr: SocketAddr) -> io::Result<()> {
        self.io.bind(addr)
    }

    /// Converts the socket into a TcpListener
    pub fn listen(self, backlog: u32) -> io::Result<TcpListener> {
        Ok(TcpListener::new(self.io.listen(backlog)?))
    }
}
