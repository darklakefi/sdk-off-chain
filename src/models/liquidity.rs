// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::integrations_pb::{
    AddLiquidityRequest as ProtoAddLiquidityRequest,
    AddLiquidityResponse as ProtoAddLiquidityResponse,
    RemoveLiquidityRequest as ProtoRemoveLiquidityRequest,
    RemoveLiquidityResponse as ProtoRemoveLiquidityResponse,
};

/// Add liquidity request
///
/// This struct is used to request to add liquidity from the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct AddLiquidityRequest {
    /// The mint address of the token X.
    pub token_mint_x: String,
    /// The mint address of the token Y.
    pub token_mint_y: String,
    /// The wallet address of the user.
    pub user_address: String,
    /// The desired amount of pool liquidity tokens.
    pub amount_lp: u64,
    /// The maximum amount of token X to add.
    pub max_amount_x: u64,
    /// The maximum amount of token Y to add.
    pub max_amount_y: u64,
    /// The referral code.
    pub ref_code: String,
    /// The label to attach, 10 chars max.
    pub label: String,
}

/// Add liquidity response
///
/// This struct is used to return an unsigned transaction to send to the wallet for the signing and execution.
#[derive(Debug, Clone)]
pub struct AddLiquidityResponse {
    /// Base64 encoded unsigned transaction to send to the wallet for sign & execute.
    pub unsigned_transaction: String,
}

/// Convert from ProtoAddLiquidityResponse to AddLiquidityResponse
///
/// This function is used to convert from ProtoAddLiquidityResponse to AddLiquidityResponse.
impl From<ProtoAddLiquidityResponse> for AddLiquidityResponse {
    fn from(response: ProtoAddLiquidityResponse) -> Self {
        Self {
            unsigned_transaction: response.unsigned_transaction,
        }
    }
}

/// Convert from AddLiquidityRequest to ProtoAddLiquidityRequest
///
/// This function is used to convert from AddLiquidityRequest to ProtoAddLiquidityRequest.
impl From<AddLiquidityRequest> for ProtoAddLiquidityRequest {
    fn from(request: AddLiquidityRequest) -> Self {
        Self {
            token_mint_x: request.token_mint_x,
            token_mint_y: request.token_mint_y,
            user_address: request.user_address,
            amount_lp: request.amount_lp,
            max_amount_x: request.max_amount_x,
            max_amount_y: request.max_amount_y,
            ref_code: request.ref_code,
            label: request.label,
        }
    }
}

/// Remove liquidity request
///
/// This struct is used to request to remove liquidity from the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct RemoveLiquidityRequest {
    /// The mint address of the token X.
    pub token_mint_x: String,
    /// The mint address of the token Y.
    pub token_mint_y: String,
    /// The wallet address of the user.
    pub user_address: String,
    /// The desired amount of pool liquidity tokens.
    pub amount_lp: u64,
    /// The minimum amount of token X to remove.
    pub min_amount_x: u64,
    /// The minimum amount of token Y to remove.
    pub min_amount_y: u64,
    /// The referral code.
    pub ref_code: String,
    /// The label to attach, 10 chars max.
    pub label: String,
}

/// Remove liquidity response
///
/// This struct is used to return an unsigned transaction to send to the wallet for the signing and execution.
#[derive(Debug, Clone)]
pub struct RemoveLiquidityResponse {
    /// Base64 encoded unsigned transaction to send to the wallet for sign & execute.
    pub unsigned_transaction: String,
}

/// Convert from ProtoRemoveLiquidityResponse to RemoveLiquidityResponse    
///
/// This function is used to convert from ProtoRemoveLiquidityResponse to RemoveLiquidityResponse.
impl From<ProtoRemoveLiquidityResponse> for RemoveLiquidityResponse {
    fn from(response: ProtoRemoveLiquidityResponse) -> Self {
        Self {
            unsigned_transaction: response.unsigned_transaction,
        }
    }
}

/// Convert from RemoveLiquidityRequest to ProtoRemoveLiquidityRequest
///
/// This function is used to convert from RemoveLiquidityRequest to ProtoRemoveLiquidityRequest.
impl From<RemoveLiquidityRequest> for ProtoRemoveLiquidityRequest {
    fn from(request: RemoveLiquidityRequest) -> Self {
        Self {
            token_mint_x: request.token_mint_x,
            token_mint_y: request.token_mint_y,
            user_address: request.user_address,
            amount_lp: request.amount_lp,
            min_amount_x: request.min_amount_x,
            min_amount_y: request.min_amount_y,
            ref_code: request.ref_code,
            label: request.label,
        }
    }
}
