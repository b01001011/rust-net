use std::io::{Read, Write};
use std::net::TcpListener;

fn main()-> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("TCP server listening on 127.0.0.1:8080");

    let (mut stream, address) = listener.accept()?;

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    println!(
        "Received: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

    stream.write_all(b"Hello from server!")?;

    println!("Response sent");

    Ok(())
}
