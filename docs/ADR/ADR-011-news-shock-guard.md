# ADR-011 — Market News & Policy Shock Guard

**Status:** Accepted
**Date:** 2026-09-24
**Related:** docs/NEWS_SHOCK_SPEC.md, docs/RISK_SPEC.md sec 4, docs/ML_SPEC.md sec 7-8, ADR-008

## Context

Sudden information/policy events can invalidate the statistical relationship a relative-value strategy depends on (volatility shock, abnormal volume, spread widening, liquidity deterioration, correlation/cointegration instability, order-book abnormalities, large displacement). The system needs to identify regime disruption without claiming to predict prices from headlines.

## Decision

Add a **cross-cutting Market News & Policy Shock Guard** whose purpose is **not** price prediction but **contextual regime disruption detection**:

`External Event -> Event Detection -> Entity/Sector Identification -> Semantic Classification -> Market Confirmation -> Shock/Regime State -> Strategy/Risk Context`

- Semantic side may optionally use Laya (ADR-008) behind `ContextClassifier`.
- Market-confirmation side uses observable proxies listed above (quantitative, not sentiment).
- Output is a `ShockState` (`NORMAL | WATCH | RESTRICT | SUSPEND_NEW_ENTRIES | RECOVERY`) scoped to instrument/sector/regime where possible; must not silently suspend the whole market without a policy rule.
- Shock state feeds `RiskEngine` and `TradeQualityModel` as context/features; risk remains authoritative.

## Alternatives considered

- **Price-prediction news model:** rejected — out-of-scope and untestable as a safety layer.
- **Semantic-only without market confirmation:** rejected — headlines alone are insufficient; require observable market confirmation.
- **No guard:** rejected — leaves strategies exposed to verifiable regime breaks.

## Consequences

- Pipeline is instrument/sector-aware; evaluation is ablation-based (E8): numeric-only vs numeric+semantic, measuring adverse-trade reduction vs good-trade rejection.
- Failure of semantic layer degrades to numeric-only guard, not to unsafe trading.

## Validation

- Classification + market-confirmation integration tests.
- Ablation experiment demonstrating risk-adjusted benefit before promotion.
