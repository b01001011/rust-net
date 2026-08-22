use rust_net::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::net::SocketAddr;

fn main() -> std::io::Result<()> {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(addr)?;

    println!("Listening on: 127.0.0.1:8080");

    let mut buffer = [0; 1024];

    loop { 
        let (mut stream, addr) = listener.accept()?;

        println!("Connection from {addr}");

        let mut buffer = [0; 1024];

        loop {
            let n = stream.read(&mut buffer)?;

            if n == 0 {
                break;
            }

            stream.write_all(&buffer[..n])?;
        }
    }
}
