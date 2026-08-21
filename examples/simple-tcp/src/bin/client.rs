use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    println!("Connecting to 127.0.0.1:8080...");

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    println!("Connected!");

    stream.write_all(b"Hello from client!")?;

    println!("Message sent");

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    println!(
        "Received: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

    Ok(())
}