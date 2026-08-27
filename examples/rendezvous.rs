use tonic::{Request, Response, Status};
use tonic::transport::Server;

pub mod rendezvous {
    tonic::include_proto!("rendezvous");
}

use rendezvous::rendezvous_server::{Rendezvous, RendezvousServer};
use rendezvous::{RegisterRequest, RegisterResponse};

#[derive(Debug, Default)]
struct RendezvousService;

#[tonic::async_trait]
impl Rendezvous for RendezvousService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let peer_id = request.into_inner().peer_id;

        let response: RegisterResponse = RegisterResponse {
            message:format!("Hello, peer {peer_id}!"),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    println!("Rendezvous server listening on {addr}");

    Server::builder()
        .add_service(RendezvousServer::new(RendezvousService))
        .serve(addr)
        .await?;

    Ok(())
}
