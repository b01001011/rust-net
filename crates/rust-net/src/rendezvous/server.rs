use std::net::SocketAddr;

use tonic::{Request, Response, Status};
use tonic::transport::Server;

use super::proto::rendezvous_server::{
    Rendezvous, 
    RendezvousServer as TonicRendezvousServer,
};
use super::proto::{RegisterRequest, RegisterResponse};

#[derive(Debug, Default)]
struct RendezvousService;

#[tonic::async_trait]
impl Rendezvous for RendezvousService {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let peer_id = request.into_inner().peer_id;
        let response = RegisterResponse {
            message: format!("Hello, peer {peer_id}!"),
        };

        Ok(Response::new(response))
    }
}

pub struct RendezvousServer {
    addr: SocketAddr,
}

impl RendezvousServer {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn run(self) -> Result<(), tonic::transport::Error> {
        println!("Rendezvous server listening on {}", self.addr);

        Server::builder()
            .add_service(TonicRendezvousServer::new(RendezvousService))
            .serve(self.addr)
            .await
    }
}