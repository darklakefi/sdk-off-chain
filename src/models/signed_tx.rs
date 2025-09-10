// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::integrations_pb::{
    SendSignedTransactionRequest as ProtoSendSignedTransactionRequest,
    SendSignedTransactionResponse as ProtoSendSignedTransactionResponse,
};
use rand::distr::{Alphanumeric, SampleString};

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
    pub fn builder() -> SendSignedTransactionRequestBuilder {
        SendSignedTransactionRequestBuilder::new()
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
    pub fn new() -> Self {
        Self {
            signed_transaction: String::new(),
            tracking_id: None,
            trade_id: String::new(),
        }
    }

    pub fn signed_transaction(mut self, signed_transaction: &str) -> Self {
        self.signed_transaction = signed_transaction.to_string();
        self
    }

    pub fn tracking_id(mut self, tracking_id: &str) -> Self {
        self.tracking_id = Some(tracking_id.to_string());
        self
    }

    pub fn trade_id(mut self, trade_id: &str) -> Self {
        self.trade_id = trade_id.to_string();
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
