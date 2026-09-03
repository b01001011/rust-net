use std::env;
use rust_net::rendezvous::RendezvousClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let peer_id = env::args()
        .nth(1)
        .unwrap_or_else(|| "peer".to_string());

    println!("Starting peer: {peer_id}");

    let server = "http://127.0.0.1:50051";

    println!("Connecting to rendezvous server at {server}");

    let mut client = RendezvousClient::connect(server).await?;

    println!("Connected!");

    let response = client.register(&peer_id).await?;

    println!("Server response: {}", response.message);

    Ok(())
}
