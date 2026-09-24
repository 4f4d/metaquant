# ADR-008 — Laya as Optional Contextual Component

**Status:** Accepted (experimental, optional)
**Date:** 2026-09-24
**Related:** docs/NEWS_SHOCK_SPEC.md, docs/ML_SPEC.md sec 7-10, docs/INTERFACES.md sec 9, AGENTS.md sec 5

## Context

Market news and policy events are unstructured text. An LLM/System-1 decision model (Laya) could classify event type, relevance, affected entities, and severity, supplying contextual features to the trade-quality layer and shock guard. Current public Laya checkpoints are not finance-specialized; usefulness to META QUANT is empirical, not asserted.

## Decision

- Laya is **optional and experimental**, behind a clean `ContextClassifier` interface (`docs/INTERFACES.md`).
- Potential uses: event-type classification, relevance, affected-entity/sector classification, severity/impact classification, contextual uncertainty, semantic features for the ML layer.
- **Must not:** override hard risk limits, authorize a trade, change maximum exposure, bypass kill switches, override regulatory constraints, determine position size, or directly control order submission.
- Core system remains functional if Laya is unavailable (`context_unavailable` state).
- Architecture: `News/Event -> pre-filter/dedup -> Laya (optional) -> structured contextual features -> trade-quality/shock models -> Risk (authoritative)`.

## Alternatives considered

- **No semantic context:** baseline; numeric shock detection alone.
- **Laya as primary signal/risk authority:** rejected — violates deterministic-risk invariant.
- **Always-on per-event LLM:** rejected — latency, cost, non-determinism; use batched, hierarchical classification instead.

## Consequences

- Record Laya version / model hash / configuration for reproducibility.
- Require A/B or ablation: no-Laya vs Laya-contextual vs Laya + quantitative ML.
- Do not assume finance performance; calibrate and measure on META QUANT data.

## Validation

- Interface test: system passes when Laya mock is down.
- Ablation experiment E8/E9 measures incremental value.
