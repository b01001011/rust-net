use std::io;
use std::net::SocketAddr;
use socket2::Domain;

use super::socket::TcpSocket;
use super::stream::TcpStream;

pub struct TcpListener {
    inner: TcpSocket,
}

impl TcpListener {
    pub fn bind(addr: SocketAddr) -> io::Result<Self> {
        let domain = match addr {
            SocketAddr::V4(_) => Domain::IPV4,
            SocketAddr::V6(_) => Domain::IPV6,
        };

        let socket = TcpSocket::new(domain)?;
        socket.set_reuse_address(true);
        socket.bind(&addr)?;
        socket.listen(1024)?;

        Ok(Self { 
            inner: socket, 
        })
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let (socket, addr) = self.inner.accept()?;
        
        Ok((
            TcpStream::new(socket),
            addr,
        ))
    }
}
