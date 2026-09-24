# META QUANT — Development Plan

## Phase 0 — Specification and repository contracts

Deliverables:
- SRS
- architecture
- data contracts
- strategy contracts
- execution/risk specs
- ADRs
- AGENTS.md
- CI skeleton

## Phase 1 — Data layer

Deliverables:
- canonical schemas
- Parquet writer/reader
- validation
- DuckDB research access
- instrument metadata handling

## Phase 2 — Event/backtest kernel

Deliverables:
- event types
- deterministic clock
- replay engine
- market state
- order/position state machines
- accounting skeleton

## Phase 3 — One vertical slice

Implement only:

```text
historical market data
→ market event
→ basis signal
→ risk
→ simulated order
→ fill
→ cost
→ position
→ P&L
→ deterministic replay
```

This is the first serious end-to-end acceptance milestone.

## Phase 4 — Execution realism

Add:
- spread crossing,
- slippage,
- partial fills,
- latency,
- two-leg execution,
- impact assumptions.

## Phase 5 — Statistical arbitrage

Add cointegration/pairs research and runtime strategy contracts.

## Phase 6 — NSE–BSE

Add timestamp alignment, venue-specific market state and cross-exchange two-leg simulation.

## Phase 7 — News/policy and auction context

Add the contextual/risk and auction-regime modules.

## Phase 8 — ML

Add calibrated trade-quality model and optional Laya contextual features.

## Phase 9 — Paper/shadow

Operate validated strategies against live/paper feeds, with no live order submission.
