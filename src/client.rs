// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::{
    core::config::Config,
    models::{
        CreateUnsignedTransactionRequest, CreateUnsignedTransactionResponse, QuoteRequest,
        QuoteResponse, SendSignedTransactionRequest, SendSignedTransactionResponse,
    },
};
use eyre::Result;
use tracing::*;
pub mod grpc_client;
pub mod service;

/// Client type
///
/// This enum is used to define the type of client to create.
#[derive(Debug, Clone)]
pub enum ClientType {
    Grpc,
}

/// Client for the Darklake Integrations service.
///
/// This struct is used to create a client for the Darklake Integrations service.
pub struct Client {
    service: Box<dyn service::Service>,
}

impl Client {
    /// Creates a new `Client` instance.
    ///
    /// This is used to create a new `Client` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if the client cannot be created.
    pub async fn new(config: Config) -> Result<Self> {
        debug!("Creating client with config: {:?}", config);
        let service = match config.client_type {
            ClientType::Grpc => Box::new(
                grpc_client::DarklakeIntegrationsClient::new(config.url.to_string())
                    .await
                    .map_err(|e| eyre::eyre!("Failed to create gRPC client: {}", e))?,
            ),
        };
        Ok(Self { service })
    }

    /// Gets a quote from the Darklake Integrations service.
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
    pub async fn get_quote(&mut self, request: QuoteRequest) -> Result<QuoteResponse> {
        self.service.quote(request).await
    }

    /// Creates an unsigned transaction for the Darklake Integrations service.
    ///
    /// This is used to create an unsigned transaction for the Darklake Integrations service.
    ///
    /// # Errors
    ///
    /// Returns an error if the unsigned transaction cannot be created.
    pub async fn create_unsigned_transaction(
        &mut self,
        request: CreateUnsignedTransactionRequest,
    ) -> Result<CreateUnsignedTransactionResponse> {
        self.service.create_unsigned_transaction(request).await
    }

    /// Sends a signed transaction to the Darklake Integrations service.
    ///
    /// This is used to send a signed transaction to the Darklake Integrations service.
    ///
    /// # Errors
    ///
    /// Returns an error if the signed transaction cannot be sent.
    ///
    /// # Returns
    ///
    /// Returns the `SendSignedTransactionResponse` instance.
    pub async fn send_signed_transaction(
        &mut self,
        request: SendSignedTransactionRequest,
    ) -> Result<SendSignedTransactionResponse> {
        self.service.send_signed_transaction(request).await
    }
}
