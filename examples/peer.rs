use std::env;

use tonic::transport::Channel;

pub mod rendezvous {
    tonic::include_proto!("rendezvous");
}

use rendezvous::rendezvous_client::RendezvousClient;
use rendezvous::RegisterRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let peer_id = env::args()
        .nth(1)
        .unwrap_or_else(|| "peer".to_string());

    println!("Starting peer: {peer_id}");

    let server = "http://127.0.0.1:50051";

    println!("Connecting to rendezvous server at {server}");

    let mut client = RendezvousClient::connect(server.to_string()).await?;

    println!("Connected!");

    let request = RegisterRequest {
        peer_id: peer_id.clone(),
    };

    let response = client.register(request).await?;

    println!("Server response: {}", response.into_inner().message);

    Ok(())
}