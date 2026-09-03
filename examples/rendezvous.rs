use rust_net::rendezvous::RendezvousServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;

    RendezvousServer::new(addr)
        .run()
        .await?;
    
    Ok(())
}
