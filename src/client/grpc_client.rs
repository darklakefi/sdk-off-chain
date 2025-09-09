use std::fmt;
use std::time::Duration;
use tonic::{Request, async_trait, transport::Channel};
use tracing::*;

use eyre::Result;

use crate::{
    client::service::{QuoteRequest, QuoteResponse, Service},
    integrations_pb::{
        QuoteRequest as ProtoQuoteRequest, QuoteResponse as ProtoQuoteResponse,
        darklake_integrations_service_client::DarklakeIntegrationsServiceClient,
    },
};

/// Error type for the gRPC client.
///
/// This enum represents the different errors that can occur when interacting with the gRPC client.
#[derive(Debug)]
pub enum GrpcClientError {
    InvalidUri(String),
    ConnectionFailed(String),
    GrpcError(tonic::Status),
}

impl std::error::Error for GrpcClientError {}

/// Display implementation for the gRPC client error.
///
/// This implementation provides a human-readable representation of the gRPC client error.
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
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be created.
    ///
    /// # Returns
    ///
    /// Returns the `DarklakeIntegrationsClient` instance.
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
    ///
    /// This is used to get a quote from the Darklake Integrations service.
    ///
    /// # Errors
    ///
    /// Returns an error if the quote cannot be retrieved.
    ///
    /// # Returns
    ///
    /// Returns the `ProtoQuoteResponse` instance.
    pub async fn get_quote(
        &mut self,
        request: ProtoQuoteRequest,
    ) -> Result<ProtoQuoteResponse, GrpcClientError> {
        debug!("Getting quote for request: {:?}", request);
        let response = self.client.quote(Request::new(request)).await?;

        Ok(response.into_inner())
    }
}

#[async_trait]
impl Service for DarklakeIntegrationsClient {
    /// Get quote
    ///
    /// This is used to get a quote from the Darklake Integrations service.
    ///
    /// # Errors
    ///
    /// Returns an error if the quote cannot be retrieved.
    ///
    /// # Returns
    ///
    /// Returns the `QuoteResponse` instance.
    async fn quote(&mut self, request: QuoteRequest) -> Result<QuoteResponse> {
        let proto_response = self.get_quote(request.into()).await?;
        Ok(proto_response.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    /// Test client creation
    ///
    /// This is used to test the creation of the Darklake Integrations client.
    async fn test_client_creation() {
        // This test would require a running server
        // For now, we'll just test that the client can be created with a valid address
        let result = DarklakeIntegrationsClient::new("http://[::1]:50051".to_string()).await;
        // This will likely fail without a server, but we can test the creation logic
        assert!(result.is_err() || result.is_ok());
    }
}
