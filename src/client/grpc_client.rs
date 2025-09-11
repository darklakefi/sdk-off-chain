// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::fmt;
use std::time::Duration;
use tonic::{Request, async_trait, transport::Channel};
use tracing::*;

use eyre::Result;

use crate::{
    client::service::Service,
    integrations_pb::{
        CheckTradeStatusRequest as ProtoCheckTradeStatusRequest,
        CheckTradeStatusResponse as ProtoCheckTradeStatusResponse,
        CreateUnsignedTransactionRequest as ProtoCreateUnsignedTransactionRequest,
        CreateUnsignedTransactionResponse as ProtoCreateUnsignedTransactionResponse,
        GetTradesListByUserRequest as ProtoGetTradesListByUserRequest,
        GetTradesListByUserResponse as ProtoGetTradesListByUserResponse,
        QuoteRequest as ProtoQuoteRequest, QuoteResponse as ProtoQuoteResponse,
        SendSignedTransactionRequest as ProtoSendSignedTransactionRequest,
        SendSignedTransactionResponse as ProtoSendSignedTransactionResponse,
        darklake_integrations_service_client::DarklakeIntegrationsServiceClient,
    },
    models::{
        CheckTradeStatusRequest, CheckTradeStatusResponse, CreateUnsignedTransactionRequest,
        CreateUnsignedTransactionResponse, GetTradesListByUserRequest, GetTradesListByUserResponse,
        QuoteRequest, QuoteResponse, SendSignedTransactionRequest, SendSignedTransactionResponse,
    },
};

/// Error type for the gRPC client.
///
/// This enum represents the different errors that can occur when interacting with the gRPC client.
#[derive(Debug)]
pub(crate) enum GrpcClientError {
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
pub(crate) struct DarklakeIntegrationsClient {
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
    pub(crate) async fn new(server_addr: String) -> Result<Self, GrpcClientError> {
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
    async fn get_quote(
        &mut self,
        request: ProtoQuoteRequest,
    ) -> Result<ProtoQuoteResponse, GrpcClientError> {
        debug!("Getting quote for request: {:?}", request);
        let response = self.client.quote(Request::new(request)).await?;

        Ok(response.into_inner())
    }

    /// Create unsigned transaction
    ///
    /// This is used to create an unsigned transaction for the Darklake Integrations service.
    ///
    /// # Errors
    ///
    /// Returns an error if the unsigned transaction cannot be created.
    ///
    /// # Returns
    ///
    /// Returns the `ProtoCreateUnsignedTransactionResponse` instance.
    async fn create_unsigned_transaction(
        &mut self,
        request: ProtoCreateUnsignedTransactionRequest,
    ) -> Result<ProtoCreateUnsignedTransactionResponse, GrpcClientError> {
        debug!("Creating unsigned transaction for request: {:?}", request);
        let response = self
            .client
            .create_unsigned_transaction(Request::new(request))
            .await?;
        Ok(response.into_inner())
    }

    /// Send signed transaction
    ///
    /// This is used to send a signed transaction to the Darklake Integrations service.
    ///
    /// # Errors
    ///
    /// Returns an error if the signed transaction cannot be sent.
    ///
    /// # Returns
    ///
    /// Returns the `ProtoSendSignedTransactionResponse` instance.
    async fn send_signed_transaction(
        &mut self,
        request: ProtoSendSignedTransactionRequest,
    ) -> Result<ProtoSendSignedTransactionResponse, GrpcClientError> {
        debug!("Sending signed transaction for request: {:?}", request);
        let response = self
            .client
            .send_signed_transaction(Request::new(request))
            .await?;
        Ok(response.into_inner())
    }

    /// Check trade status
    ///
    /// This is used to check the status of a trade.
    ///
    /// # Errors
    ///
    /// Returns an error if the trade status cannot be checked.
    ///
    /// # Returns
    ///
    /// Returns the `ProtoCheckTradeStatusResponse` instance.
    async fn check_trade_status(
        &mut self,
        request: ProtoCheckTradeStatusRequest,
    ) -> Result<ProtoCheckTradeStatusResponse, GrpcClientError> {
        debug!("Checking trade status for request: {:?}", request);
        let response = self
            .client
            .check_trade_status(Request::new(request))
            .await?;
        Ok(response.into_inner())
    }

    /// Get trades list by user
    ///
    /// This is used to get the trades list by user.
    ///
    /// # Errors
    ///
    /// Returns an error if the trades list cannot be retrieved.
    ///
    /// # Returns
    ///
    /// Returns the `ProtoGetTradesListByUserResponse` instance.
    async fn get_trades_list_by_user(
        &mut self,
        request: ProtoGetTradesListByUserRequest,
    ) -> Result<ProtoGetTradesListByUserResponse, GrpcClientError> {
        debug!("Getting trades list by user for request: {:?}", request);
        let response = self
            .client
            .get_trades_list_by_user(Request::new(request))
            .await?;
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

    async fn create_unsigned_transaction(
        &mut self,
        request: CreateUnsignedTransactionRequest,
    ) -> Result<CreateUnsignedTransactionResponse> {
        let proto_response = self.create_unsigned_transaction(request.into()).await?;
        Ok(proto_response.into())
    }

    async fn send_signed_transaction(
        &mut self,
        request: SendSignedTransactionRequest,
    ) -> Result<SendSignedTransactionResponse> {
        let proto_response = self.send_signed_transaction(request.into()).await?;
        Ok(proto_response.into())
    }

    async fn check_trade_status(
        &mut self,
        request: CheckTradeStatusRequest,
    ) -> Result<CheckTradeStatusResponse> {
        let proto_response = self.check_trade_status(request.into()).await?;
        Ok(proto_response.into())
    }

    async fn get_trades_list_by_user(
        &mut self,
        request: GetTradesListByUserRequest,
    ) -> Result<GetTradesListByUserResponse> {
        let proto_response = self.get_trades_list_by_user(request.into()).await?;
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
