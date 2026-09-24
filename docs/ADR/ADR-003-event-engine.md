# ADR-003 — Common Deterministic Event-Driven Runtime

**Status:** Accepted
**Date:** 2026-09-24
**Related:** docs/ARCHITECTURE.md sec 5, docs/WORKFLOWS.md, docs/SRS.md FR-02/FR-03/FR-09

## Context

The platform must support BACKTEST, PAPER, SHADOW, and future LIVE modes. Reimplementing strategy logic per mode would invite correctness bugs and prevent fair comparison. Replay must be deterministic for research reproducibility and audit.

## Decision

Historical replay, backtest, paper and shadow shall share **one conceptual event model and runtime semantics**:

Market Data -> Normalization -> Market State -> Strategy -> Opportunity -> Trade Quality -> Risk -> Execution -> Order -> Fill -> Position -> P&L / Analytics / Audit

Core event types (minimum): `MarketEvent`, `QuoteEvent`, `TradeEvent`, `OrderBookUpdate`, `AuctionEvent`, `NewsEvent`, `SignalEvent`, `OrderEvent`, `FillEvent`, `PositionEvent`, `RiskEvent`, `TimerEvent`, `SystemEvent` (`docs/ARCHITECTURE.md`).

Only feed adapters and execution adapters differ between modes; strategy and risk contracts do not.

## Alternatives considered

- **Separate engines per mode:** rejected — duplicate logic, divergent semantics, untestable parity.
- **Async-everywhere runtime:** rejected — backtest/replay path must be deterministic; Tokio is scoped to network I/O, external feeds, timers, and service boundaries (`docs/TECH_STACK.md`).

## Consequences

- Backtest path is deterministic; timestamps, ordering, and fill simulation are versioned.
- Tokio powers I/O boundaries, not the core replay loop.
- `TimerEvent` and `OrderBookUpdate` semantics are preserved across modes.

## Validation

- Deterministic replay / golden tests (`tests/replay/`).
- Property tests for event ordering invariants.
