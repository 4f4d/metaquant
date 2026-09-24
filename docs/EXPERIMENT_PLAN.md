# META QUANT — Experiment Plan

## 1. Purpose

The architecture is fixed; the quantitative conclusions are not. Experiments determine which strategies and enhancements survive realistic assumptions.

## 2. Experiment hierarchy

### E1 — Engine correctness
Prove deterministic replay, event ordering, accounting and state transitions.

### E2 — Basis baseline
Test spot–futures relationships before advanced ML.

### E3 — Basis execution realism
Add spread crossing, costs, slippage, liquidity and latency assumptions.

### E4 — Statistical arbitrage baseline
Cointegration/pairs with strict walk-forward validation.

### E5 — Dynamic hedge ratio
Compare static, rolling and Kalman/state-space approaches.

### E6 — NSE–BSE reconstruction
Evaluate historical cross-exchange opportunities with timestamp alignment and two-leg execution constraints.

### E7 — Closing auction study
Event-study first; strategy only if an economically meaningful edge is established.

### E8 — News/policy shock guard
Numeric-only shock detection vs numeric + semantic context.

### E9 — ML uplift
Rules vs baseline ML vs XGBoost vs XGBoost + contextual features.

### E10 — Paper/shadow
Run the validated strategies without live order submission.

## 3. Standard result set

Every strategy experiment should report:

- gross P&L,
- net P&L,
- Sharpe or appropriate risk-adjusted measure,
- maximum drawdown,
- turnover,
- trade count,
- win rate,
- average holding period,
- average spread/edge,
- fill ratio,
- rejected/partial fills,
- cost contribution,
- latency sensitivity where measurable.

## 4. Ablation principle

Change one meaningful factor at a time where possible.

Example:

```text
Signal only
  ↓
+ costs
  ↓
+ slippage
  ↓
+ liquidity
  ↓
+ latency
  ↓
+ two-leg failure
  ↓
+ ML
  ↓
+ news/context
```

This reveals which part of the theoretical edge disappears under realistic conditions.

## 5. No-result rule

A strategy that loses money after realistic costs is still a valid research result. The project shall report negative findings rather than tune parameters until a desired result appears.
