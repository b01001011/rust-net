use socket2::{Domain, Protocol, Socket, Type};

use std::io::{self, Read, Write};
use std::net::SocketAddr;

pub(crate) struct TcpSocket {
    inner: Socket,
}

impl TcpSocket {
    pub(crate) fn new(domain: Domain) -> io::Result<Self> {
        let inner = Socket::new(
            domain, 
            Type::STREAM,
            Some(Protocol::TCP),
        )?;
        Ok(Self { inner })
    }

    pub(crate) fn bind(&self, addr: &SocketAddr) -> io::Result<()> {
        self.inner.bind(&(*addr).into())
    }

    pub(crate) fn listen(&self, backlog: i32) -> io::Result<()> {
        self.inner.listen(backlog)
    }

    pub(crate) fn accept(&self) -> io::Result<(TcpSocket, SocketAddr)> {
        let (socket, addr) = self.inner.accept()?;

        Ok((
            TcpSocket { inner: socket },
            addr.as_socket()
                .expect("TCP socket address should be an IP socket address"),
        ))
    }

    pub(crate) fn set_reuse_address(&self, reuse: bool) -> io::Result<()> {
        self.inner.set_reuse_address(reuse)
    }
}

impl Read for TcpSocket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for TcpSocket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.send(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
