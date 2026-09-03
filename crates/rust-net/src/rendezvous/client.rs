use tonic::transport::{Channel, Endpoint};

use super::proto::rendezvous_client::RendezvousClient as TonicRendezvousClient;
use super::proto::{RegisterRequest, RegisterResponse};

pub struct RendezvousClient {
    client: TonicRendezvousClient<Channel>,
}

impl RendezvousClient {
    pub async fn connect(
        endpoint: impl AsRef<str>,
    ) -> Result<Self, tonic::transport::Error> {
        let channel = Endpoint::from_shared(endpoint.as_ref().to_owned())?
            .connect()
            .await?;
        Ok(Self {
            client: TonicRendezvousClient::new(channel),
        })
    }

    pub async fn register(
        &mut self,
        peer_id: impl Into<String>,
    ) -> Result<RegisterResponse , tonic::Status> {
        let request = RegisterRequest {
            peer_id: peer_id.into(),
        };

        let response = self.client.register(request).await?;

        Ok(response.into_inner())
    }
}
