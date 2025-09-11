// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::time::Duration;

use crate::{
    core::config::Config,
    models::{
        CheckTradeStatusRequest, CheckTradeStatusResponse, CreateUnsignedTransactionRequest,
        CreateUnsignedTransactionResponse, GetTradesListByUserRequest, GetTradesListByUserResponse,
        QuoteRequest, QuoteResponse, SendSignedTransactionAndCheckStatusRequest,
        SendSignedTransactionRequest, SendSignedTransactionResponse, TradeStatus,
    },
};
use eyre::Result;
use tokio::sync::mpsc;
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

    /// Check trade status
    ///
    /// This is used to check the status of a trade. This function checks just one time so it needs to be put in a loop to check the status until it is confirmed.
    ///
    /// # Errors
    ///
    /// Returns an error if the trade status cannot be checked.
    ///
    /// # Returns
    ///
    /// Returns the `CheckTradeStatusResponse` instance.
    pub async fn check_trade_status(
        &mut self,
        request: CheckTradeStatusRequest,
    ) -> Result<CheckTradeStatusResponse> {
        self.service.check_trade_status(request).await
    }

    /// Check trade status loop
    ///
    /// This is used to check the status of a trade. This function checks until the trade is completed.
    /// It also sends the status of the trade to the channel if provided.
    ///
    /// # Errors
    ///
    /// Returns an error if the trade status cannot be checked.
    ///
    /// # Returns
    ///
    /// Returns the `CheckTradeStatusResponse` instance.
    pub async fn check_trade_status_loop(
        &mut self,
        request: CheckTradeStatusRequest,
        tx: Option<mpsc::Sender<TradeStatus>>,
        interval_millis: Option<u64>,
        max_attempts: Option<u32>,
    ) -> Result<CheckTradeStatusResponse> {
        let mut attempts = 0;
        let interval = interval_millis.unwrap_or(500);
        let interval_check = Duration::from_millis(interval);
        loop {
            let tx = tx.clone();
            let response = self.check_trade_status(request.clone()).await?;
            if tx.is_some() {
                if tx.unwrap().send(response.status.clone()).await.is_err() {
                    return Err(eyre::eyre!("Receiver channel closed"));
                }
            }
            match response.status {
                TradeStatus::Confirmed => {}
                TradeStatus::Settled => {
                    return Ok(response);
                }
                TradeStatus::Failed => {
                    return Ok(response);
                }
                TradeStatus::Cancelled => {
                    return Ok(response);
                }
                TradeStatus::Unsigned => {}
                TradeStatus::Signed => {}
                TradeStatus::Slashed => {
                    return Ok(response);
                }
            }
            if max_attempts.is_some() && attempts >= max_attempts.unwrap() {
                return Err(eyre::eyre!("Max attempts reached"));
            }
            attempts += 1;
            tokio::time::sleep(interval_check).await;
        }
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
    /// Returns the `GetTradesListByUserResponse` instance.
    pub async fn get_trades_list_by_user(
        &mut self,
        request: GetTradesListByUserRequest,
    ) -> Result<GetTradesListByUserResponse> {
        self.service.get_trades_list_by_user(request).await
    }

    pub async fn send_signed_transaction_and_check_status(
        &mut self,
        request: SendSignedTransactionAndCheckStatusRequest,
    ) -> Result<CheckTradeStatusResponse> {
        let signed_response = self
            .service
            .send_signed_transaction(request.clone().into())
            .await?;
        if request
            .tx_response
            .send(signed_response.clone())
            .await
            .is_err()
        {
            return Err(eyre::eyre!(
                "Signed transaction response receiver channel closed"
            ));
        }
        let mut attempts = 0;
        let interval = request.interval_millis.unwrap_or(500);
        let interval_check = Duration::from_millis(interval);
        loop {
            let tx = request.tx_status.clone();
            let response = self
                .service
                .check_trade_status(request.clone().into())
                .await?;
            if tx.is_some() {
                if tx.unwrap().send(response.status.clone()).await.is_err() {
                    return Err(eyre::eyre!("Trade status receiver channel closed"));
                }
            }
            match response.status {
                TradeStatus::Confirmed => {}
                TradeStatus::Settled => {
                    return Ok(response);
                }
                TradeStatus::Failed => {
                    return Ok(response);
                }
                TradeStatus::Cancelled => {
                    return Ok(response);
                }
                TradeStatus::Unsigned => {}
                TradeStatus::Signed => {}
                TradeStatus::Slashed => {
                    return Ok(response);
                }
            }
            if request.max_attempts.is_some() && attempts >= request.max_attempts.unwrap() {
                return Err(eyre::eyre!("Max attempts reached"));
            }
            attempts += 1;
            tokio::time::sleep(interval_check).await;
        }
    }
}
