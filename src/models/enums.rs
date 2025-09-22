// (c) Darklake Labs PTE Ltd.
//
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::integrations_pb::TradeStatus as ProtoTradeStatus;

/// Trade status
///
/// This enum is used to define the status of a trade.
#[derive(Debug, Clone, PartialEq)]
pub enum TradeStatus {
    /// The trade is unsigned.
    Unsigned,
    /// The trade is signed.
    Signed,
    /// The trade is confirmed.
    Confirmed,
    /// The trade is settled.
    Settled,
    /// The trade is slashed.
    Slashed,
    /// The trade is cancelled.
    Cancelled,
    /// The trade failed for a system error.
    Failed,
}

/// Convert from ProtoTradeStatus to TradeStatus
///
/// This function is used to convert from ProtoTradeStatus to TradeStatus.
impl From<ProtoTradeStatus> for TradeStatus {
    fn from(status: ProtoTradeStatus) -> Self {
        match status {
            ProtoTradeStatus::Unsigned => TradeStatus::Unsigned,
            ProtoTradeStatus::Signed => TradeStatus::Signed,
            ProtoTradeStatus::Confirmed => TradeStatus::Confirmed,
            ProtoTradeStatus::Settled => TradeStatus::Settled,
            ProtoTradeStatus::Slashed => TradeStatus::Slashed,
            ProtoTradeStatus::Cancelled => TradeStatus::Cancelled,
            ProtoTradeStatus::Failed => TradeStatus::Failed,
        }
    }
}

/// Convert from i32 to TradeStatus
///
/// This function is used to convert from i32 to TradeStatus.
impl From<i32> for TradeStatus {
    fn from(status: i32) -> Self {
        match status {
            0 => TradeStatus::Unsigned,
            1 => TradeStatus::Signed,
            2 => TradeStatus::Confirmed,
            3 => TradeStatus::Settled,
            4 => TradeStatus::Slashed,
            5 => TradeStatus::Cancelled,
            6 => TradeStatus::Failed,
            _ => TradeStatus::Failed,
        }
    }
}
