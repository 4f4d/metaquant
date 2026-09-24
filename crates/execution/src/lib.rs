//! mq-execution — order lifecycle, fill simulation, two-leg/legging risk.

use mq_core::{Fill, Order, TimestampNs};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("order not found: {0}")]
    OrderNotFound(String),
    #[error("invalid quantity: {0}")]
    InvalidQuantity(i64),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Created,
    PendingRisk,
    Approved,
    Submitted,
    PartiallyFilled,
    Filled,
    Cancelled,
    Rejected,
    RejectedByRisk,
}

/// Simulated execution engine — deterministic (EXECUTION_SPEC).
pub struct ExecutionEngine {
    orders: std::collections::HashMap<String, (Order, OrderStatus)>,
}

impl ExecutionEngine {
    pub fn new() -> Self {
        Self {
            orders: std::collections::HashMap::new(),
        }
    }

    pub fn submit(&mut self, order: Order) -> Result<String, ExecutionError> {
        if order.quantity <= 0 {
            return Err(ExecutionError::InvalidQuantity(order.quantity));
        }
        let id = order.order_id.clone();
        self.orders
            .insert(id.clone(), (order, OrderStatus::Submitted));
        Ok(id)
    }

    pub fn fill(&mut self, fill: Fill) -> Result<(), ExecutionError> {
        let entry = self
            .orders
            .get_mut(&fill.order_id)
            .ok_or_else(|| ExecutionError::OrderNotFound(fill.order_id.clone()))?;
        entry.1 = OrderStatus::Filled;
        Ok(())
    }

    pub fn cancel(&mut self, order_id: &str) -> Result<(), ExecutionError> {
        let entry = self
            .orders
            .get_mut(order_id)
            .ok_or_else(|| ExecutionError::OrderNotFound(order_id.into()))?;
        entry.1 = OrderStatus::Cancelled;
        Ok(())
    }

    pub fn status(&self, order_id: &str) -> Option<OrderStatus> {
        self.orders.get(order_id).map(|(_, s)| s.clone())
    }

    /// Gross vs net edge — execution never claims mid-price = fill (EXECUTION_SPEC sec 5).
    pub fn net_edge_bps(gross_bps: f64, cost_bps: f64, slippage_bps: f64) -> f64 {
        gross_bps - cost_bps - slippage_bps
    }
}

impl Default for ExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Latency bucket for simulation (EXECUTION_SPEC sec 6).
#[derive(Debug, Clone)]
pub struct LatencyModel {
    pub decision_lag_ns: TimestampNs,
    pub submit_lag_ns: TimestampNs,
    pub fill_lag_ns: TimestampNs,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mq_core::Order;

    fn order(id: &str) -> Order {
        Order {
            order_id: id.into(),
            instrument_id: "RELIANCE".into(),
            side: "Buy".into(),
            order_type: "Limit".into(),
            price_ticks: Some(10000),
            quantity: 100,
            created_at_ns: 1,
        }
    }

    #[test]
    fn submit_and_fill() {
        let mut eng = ExecutionEngine::new();
        eng.submit(order("o1")).unwrap();
        assert_eq!(eng.status("o1"), Some(OrderStatus::Submitted));
        eng.fill(Fill {
            fill_id: "f1".into(),
            order_id: "o1".into(),
            instrument_id: "RELIANCE".into(),
            price_ticks: 10000,
            quantity: 100,
            timestamp_ns: 2,
        })
        .unwrap();
        assert_eq!(eng.status("o1"), Some(OrderStatus::Filled));
    }

    #[test]
    fn net_edge_subtracts_costs() {
        assert_eq!(ExecutionEngine::net_edge_bps(10.0, 3.0, 2.0), 5.0);
    }

    #[test]
    fn rejects_zero_qty() {
        let mut eng = ExecutionEngine::new();
        let mut o = order("o2");
        o.quantity = 0;
        assert!(eng.submit(o).is_err());
    }
}
