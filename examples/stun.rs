use tokio::net::UdpSocket;
use tokio::net::lookup_host;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = UdpSocket::bind("0.0.0.0:0").await?;

    let server = lookup_host("stun.l.google.com:19302")
        .await?
        .next()
        .ok_or("Could not resolve STUN server")?;

    let request: [u8; 20] = [
        0x00, 0x01,
        0x00, 0x00,
        0x21, 0x12, 0xA4, 0x42,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x00, 0x02,
    ];

    socket.send_to(&request, server).await?;

    let mut buf = [0u8; 1024];
    let (len, peer) = socket.recv_from(&mut buf).await?;

    println!("Received {} bytes from {}", len, peer);
    println!("{:02x?}", &buf[..len]);

    Ok(())
}
