# ADR-006 — Deterministic Risk Engine Is Authoritative

**Status:** Accepted
**Date:** 2026-09-24
**Related:** docs/RISK_SPEC.md, docs/ML_SPEC.md, docs/NEWS_SHOCK_SPEC.md, docs/INTERFACES.md sec 6

## Context

Signals, trade-quality scores, and contextual classifications are probabilistic and model-dependent. Exposure limits, loss limits, instrument restrictions, and kill switches must remain predictable, auditable, and testable. If a model could override risk, the safety invariant collapses.

## Decision

- **All model outputs are advisory/evidentiary.** Deterministic risk checks have **final veto** before any simulated or future live order.
- Risk → Trade-quality ordering is conceptually `Strategy -> Signal -> TradeQuality -> Risk -> Execution` (risk last).
- Enforced checks (config-driven limits): max position, gross/net exposure (per-instrument/per-strategy/global), per-order notional, stale-data, abnormal-spread, liquidity, legging timeout, model integrity, kill switch, shock restriction, regulatory constraints.

## Alternatives considered

- **Model-confidence overrides risk:** rejected — violates safety; never permitted even at high confidence.
- **Probabilistic risk model:** rejected — hard limits must be deterministic; statistical context may inform thresholds but not bypass them.

## Consequences

- Laya, XGBoost, or any future model may not: override hard limits, authorize trades, change max exposure, bypass kill switches, override regulatory constraints, determine position size, or directly control order submission.
- Risk failures are fail-closed and journaled to PostgreSQL.

## Validation

- Unit + property tests for limit invariants; integration tests for reject paths; kill-switch test in `tests/replay/`.
