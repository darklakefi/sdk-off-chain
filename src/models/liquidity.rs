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
    pub token_mint_x: String,
    pub token_mint_y: String,
    pub user_address: String,
    pub amount_lp: u64,
    pub max_amount_x: u64,
    pub max_amount_y: u64,
    pub ref_code: String,
    pub label: String,
}

/// Add liquidity response
///
/// This struct is used to response to add liquidity from the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct AddLiquidityResponse {
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
    pub token_mint_x: String,
    pub token_mint_y: String,
    pub user_address: String,
    pub amount_lp: u64,
    pub min_amount_x: u64,
    pub min_amount_y: u64,
    pub ref_code: String,
    pub label: String,
}

/// Remove liquidity response
///
/// This struct is used to response to remove liquidity from the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct RemoveLiquidityResponse {
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
