# META QUANT Documentation Map

## Start with these

### 1. `SRS.md`
What the software must do (MQ-FR-xxx / MQ-NFR-xxx; acceptance criteria; non-goals).

### 2. `ARCHITECTURE.md`
How the pieces fit together (planes, event loop, storage, Python/Rust boundary).

### 3. `WORKFLOWS.md`
How data moves through the system end-to-end (backtest/paper/shadow/live + strategy + news + CAS).

### 4. `TECH_STACK.md`
Which technologies are selected and why (and what is explicitly deferred).

### 5. `BEGINNER_GUIDE.md`
Plain-language explanations of core concepts (basis, cointegration, execution, etc.).

## Subsystem specifications

- `DATA_SPEC.md` — time, entities, numeric representation, quality checks, leakage controls.
- `STRATEGY_SPEC.md` — common contract + basis / stat-arb / NSE-BSE + horizons + CAS/news extensions.
- `EXECUTION_SPEC.md` — order lifecycle, two-leg risk, fill/slippage/cost/latency models, broker boundary.
- `RISK_SPEC.md` — deterministic risk authority, checks, shock-state interaction, fail-closed.
- `ML_SPEC.md` — XGBoost trade-quality layer, baselines, labeling, calibration, walk-forward, Laya integration.
- `NEWS_SHOCK_SPEC.md` — shock guard pipeline, Laya role, shock policy.
- `AUCTION_SPEC.md` — CAS regime, VWAP/equilibrium/imbalance, research hypothesis, guardrails.

## Human/agent context

- `BEGINNER_GUIDE.md`
- `GLOSSARY.md`
- `INTERFACES.md` — MarketDataFeed, MarketState, Strategy, RiskEngine, ExecutionEngine, BrokerAdapter, ContextClassifier, Auction/Shock.
- `OPERATIONS.md` — startup, health checks, recovery, operational evidence.
- `SECURITY.md` — secret handling, live boundary, audit.
- `CHANGE_CONTROL.md` — what needs an ADR.
- `PROJECT_CONTEXT.md` + `AGENTS.md` (root) — mental model and agent rules.

## Research/development

- `EXPERIMENT_PLAN.md` — E1-E10 question hierarchy, standard result set, ablation.
- `DEVELOPMENT_PLAN.md` — Phases 0-9; Phase 3 vertical slice is next milestone.
- `REQUIREMENT_TRACEABILITY.md` — FR/NFR -> spec -> ADR -> implementation -> validation.

## Architecture decisions

See `ADR/` for decisions that must not be silently changed:

- ADR-001 Python/Rust split · ADR-002 Storage · ADR-003 Event engine · ADR-004 PyO3 boundary
- ADR-005 Python 3.12 + platforms · ADR-006 Risk authority · ADR-007 No premature infra
- ADR-008 Laya optional · ADR-009 Multi-horizon · ADR-010 CAS regime · ADR-011 Shock guard

Implementation must not override these without an ADR.
