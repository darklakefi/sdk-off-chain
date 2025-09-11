// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use rand::distr::{Alphanumeric, SampleString};

use crate::integrations_pb::{
    CheckTradeStatusRequest as ProtoCheckTradeStatusRequest,
    CheckTradeStatusResponse as ProtoCheckTradeStatusResponse,
    GetTradesListByUserRequest as ProtoGetTradesListByUserRequest,
    GetTradesListByUserResponse as ProtoGetTradesListByUserResponse,
    TokenMetadata as ProtoTokenMetadata, Trade as ProtoTrade,
};

use crate::models::enums::TradeStatus;

/// CheckTradeStatusRequest
///
/// This struct is used to check the status of a trade.
#[derive(Debug, Clone)]
pub struct CheckTradeStatusRequest {
    /// The tracking id for the trade.
    pub tracking_id: String,
    /// The trade id.
    pub trade_id: String,
}

impl CheckTradeStatusRequest {
    pub fn builder(trade_id: &str) -> CheckTradeStatusRequestBuilder {
        CheckTradeStatusRequestBuilder::new(trade_id)
    }
}

/// CheckTradeStatusRequestBuilder
///
/// This struct is used to build a CheckTradeStatusRequest.
#[derive(Debug, Clone)]
pub struct CheckTradeStatusRequestBuilder {
    /// The tracking id for the trade.
    pub tracking_id: Option<String>,
    /// The trade id.
    pub trade_id: String,
}

impl CheckTradeStatusRequestBuilder {
    pub fn new(trade_id: &str) -> Self {
        Self {
            tracking_id: None,
            trade_id: trade_id.to_string(),
        }
    }

    pub fn tracking_id(mut self, tracking_id: &str) -> Self {
        self.tracking_id = Some(tracking_id.to_string());
        self
    }

    pub fn build(self) -> CheckTradeStatusRequest {
        let tracking_id: String;
        if self.tracking_id.is_none() {
            tracking_id = Alphanumeric.sample_string(&mut rand::rng(), 12);
        } else {
            tracking_id = self.tracking_id.unwrap();
        }
        CheckTradeStatusRequest {
            tracking_id: tracking_id,
            trade_id: self.trade_id,
        }
    }
}

/// Convert from CheckTradeStatusRequest to ProtoCheckTradeStatusRequest
///
/// This function is used to convert from CheckTradeStatusRequest to ProtoCheckTradeStatusRequest.
impl From<CheckTradeStatusRequest> for ProtoCheckTradeStatusRequest {
    fn from(request: CheckTradeStatusRequest) -> Self {
        Self {
            tracking_id: request.tracking_id,
            trade_id: request.trade_id,
        }
    }
}

/// CheckTradeStatusResponse
///
/// This struct holds the response of a trade status check.
#[derive(Debug, Clone)]
pub struct CheckTradeStatusResponse {
    /// The trade id.
    pub trade_id: String,
    /// The status of the trade.
    pub status: TradeStatus,
}

/// Convert from CheckTradeStatusRequest to ProtoCheckTradeStatusRequest
///
/// This function is used to convert from CheckTradeStatusRequest to ProtoCheckTradeStatusRequest.
impl From<ProtoCheckTradeStatusResponse> for CheckTradeStatusResponse {
    fn from(response: ProtoCheckTradeStatusResponse) -> Self {
        Self {
            trade_id: response.trade_id,
            status: response.status.into(),
        }
    }
}

/// GetTradesListByUserRequest
///
/// This struct is used to get the list of trades for a user.
#[derive(Debug, Clone)]
pub struct GetTradesListByUserRequest {
    /// The wallet address of the user.
    pub user_address: String,
    /// The page size.
    pub page_size: i32,
    /// The page number.
    pub page_number: i32,
}

/// GetTradesListByUserResponse
///
/// This struct holds the response of a trades list by user.
#[derive(Debug, Clone)]
pub struct GetTradesListByUserResponse {
    /// The trades list.
    pub trades: Vec<Trade>,
    /// The total pages.
    pub total_pages: i32,
    /// The current page.
    pub current_page: i32,
}

/// Convert from GetTradesListByUserRequest to ProtoGetTradesListByUserRequest
///
/// This function is used to convert from GetTradesListByUserRequest to ProtoGetTradesListByUserRequest.
impl From<GetTradesListByUserRequest> for ProtoGetTradesListByUserRequest {
    fn from(request: GetTradesListByUserRequest) -> Self {
        Self {
            user_address: request.user_address,
            page_size: request.page_size,
            page_number: request.page_number,
        }
    }
}

/// Convert from ProtoGetTradesListByUserResponse to GetTradesListByUserResponse
///
/// This function is used to convert from ProtoGetTradesListByUserResponse to GetTradesListByUserResponse.
impl From<ProtoGetTradesListByUserResponse> for GetTradesListByUserResponse {
    fn from(response: ProtoGetTradesListByUserResponse) -> Self {
        Self {
            trades: response
                .trades
                .into_iter()
                .map(|trade| trade.into())
                .collect(),
            total_pages: response.total_pages,
            current_page: response.current_page,
        }
    }
}

/// Trade
///
/// This struct is used to define a trade.
#[derive(Debug, Clone)]
pub struct Trade {
    /// The trade id.
    pub trade_id: String,
    /// The order id.
    pub order_id: String,
    /// The user address.
    pub user_address: String,
    /// The token X.
    pub token_x: Option<TokenMetadata>,
    /// The token Y.
    pub token_y: Option<TokenMetadata>,
    /// The amount in.
    pub amount_in: u64,
    /// The minimal amount out.
    pub minimal_amount_out: u64,
    /// The status.
    pub status: TradeStatus,
    /// The signature.
    pub signature: String,
    /// The created at.
    pub created_at: i64,
    /// The updated at.
    pub updated_at: i64,
    /// Whether to swap token X to token Y.
    pub is_swap_x_to_y: bool,
}

/// Convert from ProtoTrade to Trade
///
/// This function is used to convert from ProtoTrade to Trade.
impl From<ProtoTrade> for Trade {
    fn from(trade: ProtoTrade) -> Self {
        Self {
            trade_id: trade.trade_id,
            order_id: trade.order_id,
            user_address: trade.user_address,
            token_x: trade.token_x.map(|token| token.into()),
            token_y: trade.token_y.map(|token| token.into()),
            amount_in: trade.amount_in,
            minimal_amount_out: trade.minimal_amount_out,
            status: trade.status.into(),
            signature: trade.signature,
            created_at: trade.created_at,
            updated_at: trade.updated_at,
            is_swap_x_to_y: trade.is_swap_x_to_y,
        }
    }
}

/// TokenMetadata
///
/// This struct is used to define a token metadata.
#[derive(Debug, Clone)]
pub struct TokenMetadata {
    /// The name.
    pub name: String,
    /// The symbol.
    pub symbol: String,
    /// The decimals.
    pub decimals: u32,
    /// The logo URI.
    pub logo_uri: String,
    /// The address.
    pub address: String,
}

/// Convert from ProtoTokenMetadata to TokenMetadata
///
/// This function is used to convert from ProtoTokenMetadata to TokenMetadata.
impl From<ProtoTokenMetadata> for TokenMetadata {
    fn from(token: ProtoTokenMetadata) -> Self {
        Self {
            name: token.name,
            symbol: token.symbol,
            decimals: token.decimals,
            logo_uri: token.logo_uri,
            address: token.address,
        }
    }
}
