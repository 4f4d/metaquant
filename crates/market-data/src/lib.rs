//! mq-market-data — ingestion, normalization, replay.
//! Historical: Parquet/Arrow interchange; hot state: native Rust structs.

use mq_core::{MarketEvent, TimestampNs};

/// Feed health — heartbeat, staleness, sequence gaps (INTERFACES sec 1).
#[derive(Debug, Clone)]
pub struct FeedHealth {
    pub last_event_ns: Option<TimestampNs>,
    pub is_stale: bool,
    pub sequence_gaps: u64,
}

/// Trait for all market-data feeds (historical replay, paper, future live).
pub trait MarketDataFeed {
    fn next_event(&mut self) -> Option<MarketEvent>;
    fn health(&self) -> FeedHealth;
}

/// In-memory replay feed for deterministic backtest (ADR-003).
pub struct InMemoryReplayFeed {
    events: Vec<MarketEvent>,
    cursor: usize,
    gaps: u64,
}

impl InMemoryReplayFeed {
    pub fn new(mut events: Vec<MarketEvent>) -> Self {
        events.sort_by_key(|e| e.event_timestamp_utc_ns);
        Self {
            events,
            cursor: 0,
            gaps: 0,
        }
    }
}

impl MarketDataFeed for InMemoryReplayFeed {
    fn next_event(&mut self) -> Option<MarketEvent> {
        if self.cursor >= self.events.len() {
            return None;
        }
        let ev = self.events[self.cursor].clone();
        self.cursor += 1;
        // Detect sequence gap if sequence_number present and not monotonic
        if self.cursor >= 2 {
            let prev = &self.events[self.cursor - 2];
            let curr = &self.events[self.cursor - 1];
            if let (Some(a), Some(b)) = (prev.sequence_number, curr.sequence_number)
                && b != a + 1
            {
                self.gaps += 1;
            }
        }
        Some(ev)
    }

    fn health(&self) -> FeedHealth {
        FeedHealth {
            last_event_ns: self
                .events
                .get(self.cursor.saturating_sub(1))
                .map(|e| e.event_timestamp_utc_ns),
            is_stale: false,
            sequence_gaps: self.gaps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn ev(ts: u64, seq: Option<u64>) -> MarketEvent {
        MarketEvent {
            event_id: format!("ev-{ts}"),
            event_type: "QuoteEvent".into(),
            event_timestamp_utc_ns: ts,
            ingestion_timestamp_utc_ns: ts,
            exchange: "NSE".into(),
            segment: "EQ".into(),
            instrument_id: "RELIANCE".into(),
            sequence_number: seq,
            payload: json!({}),
        }
    }

    #[test]
    fn replay_is_timestamp_ordered() {
        let mut feed =
            InMemoryReplayFeed::new(vec![ev(3, Some(3)), ev(1, Some(1)), ev(2, Some(2))]);
        assert_eq!(feed.next_event().unwrap().event_timestamp_utc_ns, 1);
        assert_eq!(feed.next_event().unwrap().event_timestamp_utc_ns, 2);
        assert_eq!(feed.next_event().unwrap().event_timestamp_utc_ns, 3);
        assert!(feed.next_event().is_none());
    }

    #[test]
    fn sequence_gap_detected() {
        let mut feed = InMemoryReplayFeed::new(vec![ev(1, Some(1)), ev(2, Some(3))]);
        feed.next_event();
        feed.next_event();
        assert_eq!(feed.health().sequence_gaps, 1);
    }
}
