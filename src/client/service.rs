// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use async_trait::async_trait;
use eyre::Result;

use crate::integrations_pb::{
    QuoteRequest as ProtoQuoteRequest, QuoteResponse as ProtoQuoteResponse,
};

/// Quote request
///
/// This struct is used to request a quote from the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct QuoteRequest {
    pub token_mint_x: String,
    pub token_mint_y: String,
    pub amount_in: u64,
    pub is_swap_x_to_y: bool,
}

/// Quote response
///
/// This struct is used to response a quote from the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct QuoteResponse {
    pub token_mint_x: String,
    pub token_mint_y: String,
    pub amount_in: u64,
    pub amount_out: u64,
    pub fee_amount: u64,
    pub fee_pct: f64,
    pub is_swap_x_to_y: bool,
}

/// Convert from ProtoQuoteResponse to QuoteResponse
///
/// This function is used to convert from ProtoQuoteResponse to QuoteResponse.
impl From<ProtoQuoteResponse> for QuoteResponse {
    fn from(response: ProtoQuoteResponse) -> Self {
        Self {
            token_mint_x: response.token_mint_x,
            token_mint_y: response.token_mint_y,
            amount_in: response.amount_in,
            amount_out: response.amount_out,
            fee_amount: response.fee_amount,
            fee_pct: response.fee_pct,
            is_swap_x_to_y: response.is_swap_x_to_y,
        }
    }
}

/// Convert from QuoteRequest to ProtoQuoteRequest
///
/// This function is used to convert from QuoteRequest to ProtoQuoteRequest.
impl From<QuoteRequest> for ProtoQuoteRequest {
    fn from(request: QuoteRequest) -> Self {
        Self {
            token_mint_x: request.token_mint_x,
            token_mint_y: request.token_mint_y,
            amount_in: request.amount_in,
            is_swap_x_to_y: request.is_swap_x_to_y,
        }
    }
}

#[async_trait]
/// Service trait
///
/// This trait is used to define the methods that must be implemented by the service.
pub trait Service {
    async fn quote(&mut self, request: QuoteRequest) -> Result<QuoteResponse>;
}
