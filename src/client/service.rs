// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use async_trait::async_trait;
use eyre::Result;

use crate::models::{
    CheckTradeStatusRequest, CheckTradeStatusResponse, CreateUnsignedTransactionRequest,
    CreateUnsignedTransactionResponse, GetTradesListByUserRequest, GetTradesListByUserResponse,
    QuoteRequest, QuoteResponse, SendSignedTransactionRequest, SendSignedTransactionResponse,
};

#[async_trait]
/// Service trait
///
/// This trait is used to define the methods that must be implemented by the service.
pub(crate) trait Service {
    async fn quote(&mut self, request: QuoteRequest) -> Result<QuoteResponse>;
    async fn create_unsigned_transaction(
        &mut self,
        request: CreateUnsignedTransactionRequest,
    ) -> Result<CreateUnsignedTransactionResponse>;
    async fn send_signed_transaction(
        &mut self,
        request: SendSignedTransactionRequest,
    ) -> Result<SendSignedTransactionResponse>;
    async fn check_trade_status(
        &mut self,
        request: CheckTradeStatusRequest,
    ) -> Result<CheckTradeStatusResponse>;
    async fn get_trades_list_by_user(
        &mut self,
        request: GetTradesListByUserRequest,
    ) -> Result<GetTradesListByUserResponse>;
}
