// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::integrations_pb::{
    QuoteRequest as ProtoQuoteRequest, QuoteResponse as ProtoQuoteResponse,
};

/// Quote request
///
/// This struct is used to request a quote from the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct QuoteRequest {
    /// The mint address of the token X.
    pub token_mint_x: String,
    /// The mint address of the token Y.
    pub token_mint_y: String,
    /// The amount of token X to swap.
    pub amount_in: u64,
    /// Whether to swap token X to token Y.
    pub is_swap_x_to_y: bool,
}

/// Quote response
///
/// This struct is used to return a quote from the Darklake Integrations service.
/// The quote returns Darklake controlled fees in the `fee_amount` field and the `fee_pct` field.
/// All the transfer fees imposed by the token contracts are calculated, but not reported in the above fields.
#[derive(Debug, Clone)]
pub struct QuoteResponse {
    /// The mint address of the token X.
    pub token_mint_x: String,
    /// The mint address of the token Y.
    pub token_mint_y: String,
    /// The amount of token X to swap.
    pub amount_in: u64,
    /// The amount of token Y to receive.
    pub amount_out: u64,
    /// The fee amount.
    pub fee_amount: u64,
    /// The fee percentage.
    pub fee_pct: f64,
    /// Whether to swap token X to token Y.
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
