# ADR-010 — Closing-Auction / CAS as Feature/Regime Subsystem

**Status:** Accepted
**Date:** 2026-09-24
**Related:** docs/AUCTION_SPEC.md, docs/STRATEGY_SPEC.md sec 6, docs/WORKFLOWS.md sec 9

## Context

NSE describes a Closing Auction Session (CAS) ~15:15-15:35 IST for cash equities (reference/VWAP from 15:00-15:15, indicative equilibrium, cumulative quantities, imbalance, final auction price), with derivatives trading to 15:40. The question is whether the cash auction outcome relates to derivative pricing in the residual window. This is a regime/feature question, not a new arbitrage strategy.

## Decision

- Closing-auction functionality is **not a fourth strategy**. It is an **auction-aware market-regime/feature subsystem**.
- Initial primary use: **Spot-Futures Basis Arbitrage** (event study of basis via CAS reference/indicative/final prices).
- Potential secondary use: NSE-BSE convergence around the cash close transition.
- Require granular data: 15:00-15:15 trades, CAS indicative/quantities/imbalance, final cash outcome, 15:35-15:40 derivatives. EOD-only data cannot support it.
- Settlement/market rules are **configurable**, not hard-coded; rules can change.
- No claim that CAS creates a profitable arbitrage exists before empirical validation.

## Alternatives considered

- **CAS as standalone strategy:** rejected — premature; first prove a cost-adjusted edge exists.
- **Hard-coded settlement assumptions:** rejected — fragile across rule changes.
- **Ignore CAS:** rejected — it is a natural regime boundary for the existing basis strategy.

## Consequences

- `AuctionState` and `SessionState` are regime inputs consumed by strategies and the trade-quality model.
- Experiment E7 is event-study-first; strategy promotion only if edge survives costs.

## Validation

- `AuctionState` schema tests; replay of synthetic CAS timeline; E7 experiment template.
