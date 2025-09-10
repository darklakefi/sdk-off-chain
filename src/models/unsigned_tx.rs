// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use rand::distr::{Alphanumeric, SampleString};

use crate::integrations_pb::{
    CreateUnsignedTransactionRequest as ProtoCreateUnsignedTransactionRequest,
    CreateUnsignedTransactionResponse as ProtoCreateUnsignedTransactionResponse,
};

/// CreateUnsignedTransactionRequest
///
/// This struct is used to request an unsigned transaction to return to the wallet for the signing.
#[derive(Debug, Clone)]
pub struct CreateUnsignedTransactionRequest {
    /// The wallet address of the user.
    pub user_address: String,
    /// The mint address of the token X.
    pub token_mint_x: String,
    /// The mint address of the token Y.
    pub token_mint_y: String,
    /// The amount of token to swap.
    pub amount_in: u64,
    /// The minimum amount of token to receive.
    pub min_out: u64,
    /// The tracking id for the trade.
    pub tracking_id: String,
    /// Whether to swap token X to token Y.
    pub is_swap_x_to_y: bool,
}

impl CreateUnsignedTransactionRequest {
    pub fn builder() -> CreateUnsignedTransactionRequestBuilder {
        CreateUnsignedTransactionRequestBuilder::new()
    }
}

/// Convert from CreateUnsignedTransactionRequest to ProtoCreateUnsignedTransactionRequest
///
/// This function is used to convert from CreateUnsignedTransactionRequest to ProtoCreateUnsignedTransactionRequest.
impl From<CreateUnsignedTransactionRequest> for ProtoCreateUnsignedTransactionRequest {
    fn from(request: CreateUnsignedTransactionRequest) -> Self {
        Self {
            user_address: request.user_address,
            token_mint_x: request.token_mint_x,
            token_mint_y: request.token_mint_y,
            amount_in: request.amount_in,
            min_out: request.min_out,
            tracking_id: request.tracking_id,
            is_swap_x_to_y: request.is_swap_x_to_y,
        }
    }
}

/// CreateUnsignedTransactionResponse
///
/// This struct holds the response of an unsigned transaction to return to the wallet for the signing.
#[derive(Debug, Clone)]
pub struct CreateUnsignedTransactionResponse {
    /// The unsigned transaction.
    pub unsigned_transaction: String,
    /// The order id.
    pub order_id: String,
    /// The trade id.
    pub trade_id: String,
}

/// Convert from ProtoCreateUnsignedTransactionResponse to CreateUnsignedTransactionResponse
///
/// This function is used to convert from ProtoCreateUnsignedTransactionResponse to CreateUnsignedTransactionResponse.
impl From<ProtoCreateUnsignedTransactionResponse> for CreateUnsignedTransactionResponse {
    fn from(response: ProtoCreateUnsignedTransactionResponse) -> Self {
        Self {
            unsigned_transaction: response.unsigned_transaction,
            order_id: response.order_id,
            trade_id: response.trade_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateUnsignedTransactionRequestBuilder {
    /// The wallet address of the user.
    pub user_address: String,
    /// The mint address of the token X.
    pub token_mint_x: String,
    /// The mint address of the token Y.
    pub token_mint_y: String,
    /// The amount of token to swap.
    pub amount_in: u64,
    /// The minimum amount of token to receive.
    pub min_out: u64,
    /// The tracking id for the trade.
    pub tracking_id: Option<String>,
    /// Whether to swap token X to token Y.
    pub is_swap_x_to_y: Option<bool>,
}

impl CreateUnsignedTransactionRequestBuilder {
    pub fn new() -> Self {
        Self {
            user_address: String::new(),
            token_mint_x: String::new(),
            token_mint_y: String::new(),
            amount_in: 0,
            min_out: 0,
            tracking_id: None,
            is_swap_x_to_y: None,
        }
    }

    pub fn user_address(mut self, user_address: &str) -> Self {
        self.user_address = user_address.to_string();
        self
    }

    pub fn token_mint_x(mut self, token_mint_x: &str) -> Self {
        self.token_mint_x = token_mint_x.to_string();
        self
    }

    pub fn token_mint_y(mut self, token_mint_y: &str) -> Self {
        self.token_mint_y = token_mint_y.to_string();
        self
    }

    pub fn amount_in(mut self, amount_in: u64) -> Self {
        self.amount_in = amount_in;
        self
    }

    pub fn min_out(mut self, min_out: u64) -> Self {
        self.min_out = min_out;
        self
    }

    pub fn tracking_id(mut self, tracking_id: &str) -> Self {
        self.tracking_id = Some(tracking_id.to_string());
        self
    }

    pub fn is_swap_x_to_y(mut self, is_swap_x_to_y: bool) -> Self {
        self.is_swap_x_to_y = Some(is_swap_x_to_y);
        self
    }

    pub fn build(self) -> CreateUnsignedTransactionRequest {
        let tracking_id: String;
        if self.tracking_id.is_none() {
            tracking_id = Alphanumeric.sample_string(&mut rand::rng(), 12);
        } else {
            tracking_id = self.tracking_id.unwrap();
        }
        let is_swap_x_to_y: bool;
        if self.is_swap_x_to_y.is_none() {
            is_swap_x_to_y = true;
        } else {
            is_swap_x_to_y = self.is_swap_x_to_y.unwrap();
        }
        CreateUnsignedTransactionRequest {
            user_address: self.user_address,
            token_mint_x: self.token_mint_x,
            token_mint_y: self.token_mint_y,
            amount_in: self.amount_in,
            min_out: self.min_out,
            tracking_id: tracking_id,
            is_swap_x_to_y: is_swap_x_to_y,
        }
    }
}
