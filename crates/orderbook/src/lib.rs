// SPDX-License-Identifier: Apache-2.0
//! mq-orderbook — native Rust order-book (hot path is not Arrow/Parquet).

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BookError {
    #[error("invalid level: price_ticks={0} qty={1}")]
    InvalidLevel(i64, i64),
}

/// Simple L2 book view — price_ticks are integer ticks (DATA_SPEC sec 3).
#[derive(Debug, Clone, Default)]
pub struct OrderBook {
    pub bids: Vec<(i64, i64)>,
    pub asks: Vec<(i64, i64)>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_update(
        &mut self,
        bids: Vec<(i64, i64)>,
        asks: Vec<(i64, i64)>,
    ) -> Result<(), BookError> {
        for (p, q) in &bids {
            if *p <= 0 || *q < 0 {
                return Err(BookError::InvalidLevel(*p, *q));
            }
        }
        for (p, q) in &asks {
            if *p <= 0 || *q < 0 {
                return Err(BookError::InvalidLevel(*p, *q));
            }
        }
        self.bids = bids;
        self.asks = asks;
        // Keep sorted: bids descending, asks ascending
        self.bids.sort_by_key(|b| std::cmp::Reverse(b.0));
        self.asks.sort_by_key(|a| a.0);
        Ok(())
    }

    pub fn best_bid(&self) -> Option<(i64, i64)> {
        self.bids.first().copied()
    }

    pub fn best_ask(&self) -> Option<(i64, i64)> {
        self.asks.first().copied()
    }

    pub fn spread_ticks(&self) -> Option<i64> {
        match (self.best_bid(), self.best_ask()) {
            (Some((bid, _)), Some((ask, _))) => Some(ask - bid),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spread_is_ask_minus_bid() {
        let mut book = OrderBook::new();
        book.apply_update(vec![(10000, 100)], vec![(10005, 200)])
            .unwrap();
        assert_eq!(book.spread_ticks(), Some(5));
    }

    #[test]
    fn rejects_negative_price() {
        let mut book = OrderBook::new();
        assert!(book.apply_update(vec![(-1, 10)], vec![]).is_err());
    }

    #[test]
    fn bbo_is_sorted() {
        let mut book = OrderBook::new();
        book.apply_update(vec![(10001, 10), (10000, 20)], vec![(10006, 5), (10005, 5)])
            .unwrap();
        assert_eq!(book.best_bid(), Some((10001, 10)));
        assert_eq!(book.best_ask(), Some((10005, 5)));
    }
}
