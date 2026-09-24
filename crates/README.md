# Crates

Rust workspace — deterministic event runtime (ADR-001).

- `core` — canonical event types, clock, schemas (`mq-core`).
- `market-data` — ingestion, normalization, replay (`mq-market-data`).
- `orderbook` — native order-book structures (hot path is not Arrow).
- `strategies` — common Strategy trait; basis / stat-arb / NSE-BSE (horizon is config, ADR-009).
- `execution` — order lifecycle, fill simulation, two-leg risk, BrokerAdapter boundary.
- `risk` — deterministic risk engine, authoritative veto (ADR-006).
- `adapters` — SimulatedBroker default; LiveBroker deferred.
