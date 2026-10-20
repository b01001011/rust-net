use rust_net::discovery::{
    DiscoveryService,
};

fn main() {
    println!("discovery");
    
    let service = DiscoveryService::new().expect("Failed to create service");
}

