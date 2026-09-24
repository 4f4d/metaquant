// SPDX-License-Identifier: Apache-2.0
//! mq-core — canonical event types, clock, and market-state contracts.
//! So that the one event model (ADR-003) is shared across backtest/paper/shadow/live.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Canonical instrument identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Instrument {
    pub instrument_id: String,
    pub exchange: String, // NSE | BSE
    pub segment: String,  // EQ | FUT
    pub symbol: String,
    pub expiry: Option<DateTime<Utc>>,
    pub tick_size: Decimal,
    pub lot_size: i64,
}

/// Monotonic timestamp in nanoseconds since UNIX epoch (UTC canonical).
pub type TimestampNs = u64;

/// All market-data events flow through `MarketEvent`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketEvent {
    pub event_id: String,
    pub event_type: String,
    pub event_timestamp_utc_ns: TimestampNs,
    pub ingestion_timestamp_utc_ns: TimestampNs,
    pub exchange: String,
    pub segment: String,
    pub instrument_id: String,
    pub sequence_number: Option<u64>,
    pub payload: serde_json::Value,
}

/// Quote — integer ticks for execution determinism; float is research-only (DATA_SPEC sec 3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    pub instrument_id: String,
    pub exchange: String,
    pub timestamp_ns: TimestampNs,
    pub bid_price_ticks: i64,
    pub bid_quantity: i64,
    pub ask_price_ticks: i64,
    pub ask_quantity: i64,
}

/// Trade print.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub instrument_id: String,
    pub exchange: String,
    pub timestamp_ns: TimestampNs,
    pub price_ticks: i64,
    pub quantity: i64,
    pub aggressor_side: Option<String>,
}

/// Order-book delta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBookUpdate {
    pub instrument_id: String,
    pub exchange: String,
    pub timestamp_ns: TimestampNs,
    pub bids: Vec<(i64, i64)>, // (price_ticks, qty)
    pub asks: Vec<(i64, i64)>,
}

/// CAS auction state (ADR-010).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionState {
    pub instrument_id: String,
    pub exchange: String,
    pub timestamp_ns: TimestampNs,
    pub session_type: String, // continuous | closing_auction
    pub reference_price_ticks: Option<i64>,
    pub indicative_equilibrium_price_ticks: Option<i64>,
    pub indicative_tradable_quantity: Option<i64>,
    pub cumulative_buy_quantity: Option<i64>,
    pub cumulative_sell_quantity: Option<i64>,
    pub imbalance_quantity: Option<i64>,
    pub imbalance_side: Option<String>,
    pub status: String,
}

/// External contextual event (news/policy).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsEvent {
    pub event_id: String,
    pub timestamp_ns: TimestampNs,
    pub source: String,
    pub raw_text: String,
    pub entities: Vec<String>,
    pub sectors: Vec<String>,
}

/// Opportunity produced by a strategy — audit-carrying (INTERFACES sec 4).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub opportunity_id: String,
    pub strategy_id: String,
    pub detected_at_ns: TimestampNs,
    pub instrument_legs: Vec<Leg>,
    pub gross_edge_bps: Decimal,
    pub config_hash: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leg {
    pub instrument_id: String,
    pub side: String, // Buy | Sell
    pub quantity: i64,
}

/// Order and fill types (EXECUTION_SPEC).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub instrument_id: String,
    pub side: String,
    pub order_type: String,
    pub price_ticks: Option<i64>,
    pub quantity: i64,
    pub created_at_ns: TimestampNs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fill {
    pub fill_id: String,
    pub order_id: String,
    pub instrument_id: String,
    pub price_ticks: i64,
    pub quantity: i64,
    pub timestamp_ns: TimestampNs,
}

/// Risk decision — authoritative (ADR-006).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskDecision {
    pub decision: String, // Approved | Rejected
    pub reason_codes: Vec<String>,
    pub timestamp_ns: TimestampNs,
}

/// Returns crate version — used by reproducibility harness (MQ-FR-013).
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_semver() {
        assert!(version().contains('.'));
    }

    #[test]
    fn quote_ticks_are_integers() {
        let q = Quote {
            instrument_id: "RELIANCE".into(),
            exchange: "NSE".into(),
            timestamp_ns: 1,
            bid_price_ticks: 10000,
            bid_quantity: 100,
            ask_price_ticks: 10005,
            ask_quantity: 200,
        };
        assert!(q.bid_price_ticks < q.ask_price_ticks);
    }
}
