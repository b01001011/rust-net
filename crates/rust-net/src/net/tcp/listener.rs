use std::io;
use std::net::TcpListener as StdTcpListener;

pub struct TcpListener {
    inner: StdTcpListener,
}

impl TcpListener {
    pub fn bind(addr: &str) -> io::Result<Self> {
        let inner = StdTcpListener::bind(addr)?;

        Ok(Self { inner })
    }

    pub fn accept(&self) -> io::Result<(super::stream::TcpStream, std::net::SocketAddr)> {
        let (stream, addr) = self.inner.accept()?;

        Ok((
            super::stream::TcpStream {
                inner: stream,
            },
            addr,
        ))
    }
}
