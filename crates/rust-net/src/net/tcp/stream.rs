use std::io::{self, Read, Write};
use super::socket::TcpSocket;

pub struct TcpStream {
    inner: TcpSocket,
}

impl TcpStream {
    pub(crate) fn new(socket: TcpSocket) -> Self {
        Self {
            inner: socket,
        }
    }
}

impl Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}

impl Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
