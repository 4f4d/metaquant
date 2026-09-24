# ADR-009 — Multi-Horizon Strategy Model

**Status:** Accepted
**Date:** 2026-09-24
**Related:** docs/STRATEGY_SPEC.md sec 5, docs/SRS.md sec 6, docs/INTERFACES.md sec 3

## Context

The three strategy families have different natural holding periods, yet the engine should not maintain separate clocks or engines per horizon. Hard-coding a single global holding period would conflate execution-sensitive intraday logic (NSE-BSE) with medium-term convergence logic (stat-arb).

## Decision

- Platform is **multi-horizon**; horizon is a **strategy/configuration property**, not a global constant.

| Strategy | Horizons | Notes |
|---|---|---|
| NSE-BSE Cross-Exchange | intraday (primary) | Short-lived, execution/timestamp sensitive |
| Spot-Futures Basis | intraday + short-term (+ optional carry-to-expiry research) | Convergence and financing matter |
| Cointegration / Stat-Arb | intraday + short-term + medium-term | Determined by measured spread dynamics |
| News/Policy Guard, Auction regime | contextual layers | No standalone holding period |

- A strategy object is independent of the engine clock; configuration selects horizon/thresholds/universe.
- Does **not** target conventional long-term directional investing. Generic directional stock prediction is explicitly not a core strategy.

## Alternatives considered

- **Single global holding period:** rejected — prevents valid intraday vs medium-term experiments.
- **Separate engine per horizon:** rejected — duplicates event semantics; configuration already suffices.
- **Directional prediction as strategy:** rejected — out of scope; ML is trade-quality, not a fourth strategy.

## Consequences

- Config files expose `horizon` and `max_holding_bars/time` per strategy instance.
- Backtests vary horizon via config, not code fork.

## Validation

- Property test: same engine handles intraday and medium-term configs without code change.
- Experiment plan E4/E5 covers horizon sensitivity.
