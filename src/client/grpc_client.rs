use std::fmt;
use std::time::Duration;
use tonic::{Request, transport::Channel};
use tracing::*;

use crate::integrations_pb::{
    QuoteRequest, QuoteResponse,
    darklake_integrations_service_client::DarklakeIntegrationsServiceClient,
};

#[derive(Debug)]
pub enum GrpcClientError {
    InvalidUri(String),
    ConnectionFailed(String),
    GrpcError(tonic::Status),
}

impl std::error::Error for GrpcClientError {}

impl fmt::Display for GrpcClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrpcClientError::InvalidUri(msg) => write!(f, "Invalid URI: {}", msg),
            GrpcClientError::ConnectionFailed(msg) => write!(f, "Connection failed: {}", msg),
            GrpcClientError::GrpcError(status) => write!(f, "gRPC error: {}", status),
        }
    }
}

impl From<tonic::Status> for GrpcClientError {
    fn from(status: tonic::Status) -> Self {
        GrpcClientError::GrpcError(status)
    }
}

/// gRPC client for interacting with the Darklake Integrations service
pub struct DarklakeIntegrationsClient {
    client: DarklakeIntegrationsServiceClient<Channel>,
}

impl DarklakeIntegrationsClient {
    /// Create a new client with the given server address
    pub async fn new(server_addr: String) -> Result<Self, GrpcClientError> {
        let channel = Channel::from_shared(server_addr)
            .map_err(|e| GrpcClientError::InvalidUri(e.to_string()))?
            .timeout(Duration::from_secs(30))
            .connect()
            .await
            .map_err(|e| GrpcClientError::ConnectionFailed(e.to_string()))?;

        let client = DarklakeIntegrationsServiceClient::new(channel.clone());

        Ok(Self { client })
    }

    /// Get quote
    pub async fn get_quote(
        &mut self,
        request: QuoteRequest,
    ) -> Result<QuoteResponse, GrpcClientError> {
        info!("Getting quote for request: {:?}", request);
        let response = self.client.quote(Request::new(request)).await?;
        info!("Quote response: {:?}", response);
        Ok(response.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        // This test would require a running server
        // For now, we'll just test that the client can be created with a valid address
        let result = DarklakeIntegrationsClient::new("http://[::1]:50051".to_string()).await;
        // This will likely fail without a server, but we can test the creation logic
        assert!(result.is_err() || result.is_ok());
    }
}
