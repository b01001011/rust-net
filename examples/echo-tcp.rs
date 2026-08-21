use rust_net::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

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
