pub mod proto {
    tonic::include_proto!("rendezvous");
}

mod client;
mod server;

pub use client::RendezvousClient;
pub use server::RendezvousServer;
