// SPDX-License-Identifier: Apache-2.0
//! mq-adapters — BrokerAdapter boundary (INTERFACES sec 8).
//! Default is simulated; live is deferred and requires separate approval.

use mq_core::{Fill, Order};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("not connected")]
    NotConnected,
    #[error("live trading not enabled (requires approval)")]
    LiveNotEnabled,
}

pub trait BrokerAdapter {
    fn place_order(&mut self, order: Order) -> Result<String, AdapterError>;
    fn cancel_order(&mut self, order_id: &str) -> Result<(), AdapterError>;
    fn fills(&self) -> Vec<Fill>;
}

/// Simulated broker — used for BACKTEST / PAPER / SHADOW (SRS sec 3).
pub struct SimulatedBroker {
    fills: Vec<Fill>,
    next_fill_price_ticks: i64,
}

impl SimulatedBroker {
    pub fn new(fill_price_ticks: i64) -> Self {
        Self {
            fills: Vec::new(),
            next_fill_price_ticks: fill_price_ticks,
        }
    }
}

impl BrokerAdapter for SimulatedBroker {
    fn place_order(&mut self, order: Order) -> Result<String, AdapterError> {
        let fill = Fill {
            fill_id: format!("fill-{}", order.order_id),
            order_id: order.order_id.clone(),
            instrument_id: order.instrument_id.clone(),
            price_ticks: self.next_fill_price_ticks,
            quantity: order.quantity,
            timestamp_ns: order.created_at_ns + 1_000_000, // 1ms simulated latency
        };
        self.fills.push(fill);
        Ok(order.order_id)
    }

    fn cancel_order(&mut self, _order_id: &str) -> Result<(), AdapterError> {
        Ok(())
    }

    fn fills(&self) -> Vec<Fill> {
        self.fills.clone()
    }
}

/// Live adapter stub — always rejects until explicitly enabled.
pub struct LiveBrokerStub;

impl BrokerAdapter for LiveBrokerStub {
    fn place_order(&mut self, _order: Order) -> Result<String, AdapterError> {
        Err(AdapterError::LiveNotEnabled)
    }

    fn cancel_order(&mut self, _order_id: &str) -> Result<(), AdapterError> {
        Err(AdapterError::LiveNotEnabled)
    }

    fn fills(&self) -> Vec<Fill> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mq_core::Order;

    #[test]
    fn simulated_broker_fills() {
        let mut b = SimulatedBroker::new(10000);
        let o = Order {
            order_id: "o1".into(),
            instrument_id: "RELIANCE".into(),
            side: "Buy".into(),
            order_type: "Market".into(),
            price_ticks: None,
            quantity: 100,
            created_at_ns: 1,
        };
        b.place_order(o).unwrap();
        assert_eq!(b.fills().len(), 1);
    }

    #[test]
    fn live_stub_rejects() {
        let mut b = LiveBrokerStub;
        let o = Order {
            order_id: "o1".into(),
            instrument_id: "RELIANCE".into(),
            side: "Buy".into(),
            order_type: "Market".into(),
            price_ticks: None,
            quantity: 100,
            created_at_ns: 1,
        };
        assert!(b.place_order(o).is_err());
    }
}
