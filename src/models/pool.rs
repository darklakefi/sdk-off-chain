// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::integrations_pb::{
    InitPoolRequest as ProtoInitPoolRequest, InitPoolResponse as ProtoInitPoolResponse,
};

/// Init pool request
///
/// This struct is used to request a pool initialization from the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct InitPoolRequest {
    /// The mint address of the token X.
    pub token_mint_x: String,
    /// The mint address of the token Y.
    pub token_mint_y: String,
    /// The wallet address of the user.
    pub user_address: String,
    /// The amount of token X to add.
    pub amount_x: u64,
    /// The amount of token Y to add.
    pub amount_y: u64,
    /// The referral code.
    pub ref_code: String,
    /// The label to attach, 10 chars max.
    pub label: String,
}

/// Init pool response
///
/// This struct is used to response a pool initialization from the Darklake Integrations service.
#[derive(Debug, Clone)]
pub struct InitPoolResponse {
    /// Base64 encoded unsigned transaction to send to the wallet for sign & execute.
    pub unsigned_transaction: String,
}

/// Convert from ProtoInitPoolResponse to InitPoolResponse
///
/// This function is used to convert from ProtoInitPoolResponse to InitPoolResponse.
impl From<ProtoInitPoolResponse> for InitPoolResponse {
    fn from(response: ProtoInitPoolResponse) -> Self {
        Self {
            unsigned_transaction: response.unsigned_transaction,
        }
    }
}

/// Convert from InitPoolRequest to ProtoInitPoolRequest
///
/// This function is used from InitPoolRequest to ProtoInitPoolRequest.
impl From<InitPoolRequest> for ProtoInitPoolRequest {
    fn from(request: InitPoolRequest) -> Self {
        Self {
            token_mint_x: request.token_mint_x,
            token_mint_y: request.token_mint_y,
            user_address: request.user_address,
            amount_x: request.amount_x,
            amount_y: request.amount_y,
            ref_code: request.ref_code,
            label: request.label,
        }
    }
}
