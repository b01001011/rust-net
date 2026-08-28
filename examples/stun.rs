use rust_net::discover_public_address;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = discover_public_address().await?;
    
    println!("Public address: {addr}");

    Ok(())
}
