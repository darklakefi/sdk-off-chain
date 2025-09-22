// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::{
    CheckTradeStatusRequest, TradeStatus,
    integrations_pb::{
        SendSignedTransactionRequest as ProtoSendSignedTransactionRequest,
        SendSignedTransactionResponse as ProtoSendSignedTransactionResponse,
    },
};
use rand::distr::{Alphanumeric, SampleString};
use tokio::sync::mpsc;

/// SendSignedTransactionRequest
///
/// This struct is used to send a wallet signed transaction to the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct SendSignedTransactionRequest {
    /// The signed transaction.
    pub signed_transaction: String,
    /// The tracking id for the trade.
    pub tracking_id: String,
    /// The trade id.
    pub trade_id: String,
}

impl SendSignedTransactionRequest {
    pub fn builder(
        signed_transaction: &str,
        trade_id: &str,
    ) -> SendSignedTransactionRequestBuilder {
        SendSignedTransactionRequestBuilder::new(signed_transaction, trade_id)
    }
}

/// SendSignedTransactionRequestBuilder
///
/// This struct is used to build a SendSignedTransactionRequest.
#[derive(Debug, Clone)]
pub struct SendSignedTransactionRequestBuilder {
    /// The signed transaction.
    pub signed_transaction: String,
    /// The tracking id for the trade.
    pub tracking_id: Option<String>,
    /// The trade id.
    pub trade_id: String,
}

impl SendSignedTransactionRequestBuilder {
    pub fn new(signed_transaction: &str, trade_id: &str) -> Self {
        Self {
            signed_transaction: signed_transaction.to_string(),
            tracking_id: None,
            trade_id: trade_id.to_string(),
        }
    }

    pub fn tracking_id(mut self, tracking_id: &str) -> Self {
        self.tracking_id = Some(tracking_id.to_string());
        self
    }

    pub fn build(self) -> SendSignedTransactionRequest {
        let tracking_id: String;
        if self.tracking_id.is_none() {
            tracking_id = Alphanumeric.sample_string(&mut rand::rng(), 12);
        } else {
            tracking_id = self.tracking_id.unwrap();
        }
        SendSignedTransactionRequest {
            signed_transaction: self.signed_transaction,
            tracking_id: tracking_id,
            trade_id: self.trade_id,
        }
    }
}

/// Convert from SendSignedTransactionRequest to ProtoSendSignedTransactionRequest
///
/// This function is used to convert from SendSignedTransactionRequest to ProtoSendSignedTransactionRequest.
impl From<SendSignedTransactionRequest> for ProtoSendSignedTransactionRequest {
    fn from(request: SendSignedTransactionRequest) -> Self {
        Self {
            signed_transaction: request.signed_transaction,
            tracking_id: request.tracking_id,
            trade_id: request.trade_id,
        }
    }
}

/// SendSignedTransactionResponse
///
/// This struct holds the response of a signed transaction sent to the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct SendSignedTransactionResponse {
    /// Whether the transaction was sent successfully.
    pub success: bool,
    /// The trade id.
    pub trade_id: String,
    /// The error logs.
    pub error_logs: Vec<String>,
}

/// Convert from ProtoSendSignedTransactionResponse to SendSignedTransactionResponse
///
/// This function is used to convert from ProtoSendSignedTransactionResponse to SendSignedTransactionResponse.
impl From<ProtoSendSignedTransactionResponse> for SendSignedTransactionResponse {
    fn from(response: ProtoSendSignedTransactionResponse) -> Self {
        Self {
            success: response.success,
            trade_id: response.trade_id,
            error_logs: response.error_logs,
        }
    }
}

/// SendSignedTransactionAndCheckStatusRequest
///
/// This struct is used to send a wallet signed transaction to the Darklake Integrations service and check the status of the trade.
#[derive(Debug, Clone)]
pub struct SendSignedTransactionAndCheckStatusRequest {
    /// The signed transaction.
    pub signed_transaction: String,
    /// The tracking id for the trade.
    pub tracking_id: String,
    /// The trade id.
    pub trade_id: String,
    /// The channel to send the send signed transaction response to.
    pub tx_response: mpsc::Sender<SendSignedTransactionResponse>,
    /// The channel to send the trade status to.
    pub tx_status: Option<mpsc::Sender<TradeStatus>>,
    /// The interval in milliseconds to check the trade status.
    pub interval_millis: Option<u64>,
    /// The maximum number of attempts to check the trade status.
    pub max_attempts: Option<u32>,
}

impl SendSignedTransactionAndCheckStatusRequest {
    pub fn builder(
        signed_transaction: &str,
        trade_id: &str,
        tx_response: mpsc::Sender<SendSignedTransactionResponse>,
    ) -> SendSignedTransactionAndCheckStatusRequestBuilder {
        SendSignedTransactionAndCheckStatusRequestBuilder::new(
            signed_transaction,
            trade_id,
            tx_response,
        )
    }
}

/// Convert from SendSignedTransactionAndCheckStatusRequest to SendSignedTransactionRequest
///
/// This function is used to convert from SendSignedTransactionAndCheckStatusRequest to SendSignedTransactionRequest.
impl From<SendSignedTransactionAndCheckStatusRequest> for SendSignedTransactionRequest {
    fn from(request: SendSignedTransactionAndCheckStatusRequest) -> Self {
        Self {
            signed_transaction: request.signed_transaction,
            tracking_id: request.tracking_id,
            trade_id: request.trade_id,
        }
    }
}

/// Convert from SendSignedTransactionAndCheckStatusRequest to CheckTradeStatusRequest
///
/// This function is used to convert from SendSignedTransactionAndCheckStatusRequest to CheckTradeStatusRequest.
impl From<SendSignedTransactionAndCheckStatusRequest> for CheckTradeStatusRequest {
    fn from(request: SendSignedTransactionAndCheckStatusRequest) -> Self {
        Self {
            tracking_id: request.tracking_id,
            trade_id: request.trade_id,
        }
    }
}

/// SendSignedTransactionAndCheckStatusRequestBuilder
///
/// This struct is used to build a SendSignedTransactionAndCheckStatusRequest.
#[derive(Debug, Clone)]
pub struct SendSignedTransactionAndCheckStatusRequestBuilder {
    /// The signed transaction.
    pub signed_transaction: String,
    /// The tracking id for the trade.
    pub tracking_id: Option<String>,
    /// The trade id.
    pub trade_id: String,
    /// The channel to send the send signed transaction response to.
    pub tx_response: mpsc::Sender<SendSignedTransactionResponse>,
    /// The channel to send the trade status to.
    pub tx_status: Option<mpsc::Sender<TradeStatus>>,
    /// The interval in milliseconds to check the trade status.
    pub interval_millis: Option<u64>,
    /// The maximum number of attempts to check the trade status.
    pub max_attempts: Option<u32>,
}

/// SendSignedTransactionAndCheckStatusRequestBuilder
///
/// This struct is used to build a SendSignedTransactionAndCheckStatusRequest.
impl SendSignedTransactionAndCheckStatusRequestBuilder {
    pub fn new(
        signed_transaction: &str,
        trade_id: &str,
        tx_response: mpsc::Sender<SendSignedTransactionResponse>,
    ) -> Self {
        Self {
            signed_transaction: signed_transaction.to_string(),
            tracking_id: None,
            trade_id: trade_id.to_string(),
            tx_response: tx_response,
            tx_status: None,
            interval_millis: None,
            max_attempts: None,
        }
    }

    pub fn tracking_id(mut self, tracking_id: &str) -> Self {
        self.tracking_id = Some(tracking_id.to_string());
        self
    }

    pub fn tx_status(mut self, tx_status: mpsc::Sender<TradeStatus>) -> Self {
        self.tx_status = Some(tx_status);
        self
    }

    pub fn interval_millis(mut self, interval_millis: u64) -> Self {
        self.interval_millis = Some(interval_millis);
        self
    }

    pub fn max_attempts(mut self, max_attempts: u32) -> Self {
        self.max_attempts = Some(max_attempts);
        self
    }

    pub fn build(self) -> SendSignedTransactionAndCheckStatusRequest {
        let tracking_id: String;
        if self.tracking_id.is_none() {
            tracking_id = Alphanumeric.sample_string(&mut rand::rng(), 12);
        } else {
            tracking_id = self.tracking_id.unwrap();
        }
        SendSignedTransactionAndCheckStatusRequest {
            signed_transaction: self.signed_transaction,
            tracking_id: tracking_id,
            trade_id: self.trade_id,
            tx_response: self.tx_response,
            tx_status: self.tx_status,
            interval_millis: self.interval_millis,
            max_attempts: self.max_attempts,
        }
    }
}
