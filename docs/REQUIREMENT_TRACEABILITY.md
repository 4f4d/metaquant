# META QUANT — Requirement Traceability

**Version:** 0.2
**Date:** 2026-09-24
**Authority:** `docs/SRS.md` (MQ-FR-xxx / MQ-NFR-xxx)

| Requirement | Authority | Spec | ADR | Implementation | Validation |
|---|---|---|---|---|---|
| MQ-FR-001 Data ingestion | SRS 4 | DATA_SPEC | ADR-002, ADR-003 | `crates/market-data`, `python/meta_quant/data` | Unit + DuckDB round-trip + replay |
| MQ-FR-002 Market state | SRS 4 | DATA_SPEC, ARCH sec 5 | ADR-003 | `crates/core` market state | Unit + deterministic replay |
| MQ-FR-003 Strategy interface | SRS 4 | STRATEGY_SPEC sec 1 | ADR-009 | `crates/strategies`, `python/meta_quant/research` | Strategy tests + replay |
| MQ-FR-004 Opportunity | SRS 4 | STRATEGY_SPEC, INTERFACES sec 4 | — | Opportunity/Signal value objects | Audit-trail test |
| MQ-FR-005 Cost-aware | SRS 4 | EXECUTION_SPEC sec 5 | — | Cost model (crates/execution) | Gross-vs-net test |
| MQ-FR-006 Risk authority | SRS 4 | RISK_SPEC | ADR-006 | `crates/risk` | Invariant + kill-switch tests |
| MQ-FR-007 Execution sim | SRS 4 | EXECUTION_SPEC | ADR-003 | `crates/execution` | Fill/latency/partial/leg tests |
| MQ-FR-008 Position/P&L | SRS 4 | DATA_SPEC sec 3, EXEC sec 5 | — | Position/PNL (Rust) | Accounting unit tests |
| MQ-FR-009 Replay | SRS 4 | ARCH sec 5, WORKFLOWS | ADR-003 | Replay engine (`crates/core`) | Golden/deterministic tests |
| MQ-FR-010 News/policy | SRS 4 | NEWS_SHOCK_SPEC | ADR-008, ADR-011 | `python/meta_quant/context` + interface | Classification + ablation (E8) |
| MQ-FR-011 Auction/CAS | SRS 4 | AUCTION_SPEC | ADR-010 | Auction regime (`crates/market-data`) | CAS event-study (E7) |
| MQ-FR-012 ML quality | SRS 4 | ML_SPEC | ADR-001, ADR-008 | `python/meta_quant/models` | Walk-forward + calibration (E9) |
| MQ-FR-013 Reproducibility | SRS 4 | INTERFACES sec 11, DATA_SPEC | ADR-003/005 | Experiment harness + metadata | Replay + hash tests |
| MQ-FR-014 Observability | SRS 4 | OPERATIONS | — | Structured logs + metrics | Log/metric integration test |
| MQ-FR-015 BrokerAdapter | SRS 4 | INTERFACES sec 8 | — | `crates/adapters` (sim default) | Adapter isolation test |
| MQ-FR-016 Numerics | SRS 4 | DATA_SPEC sec 3 | — | Tick/quantity types | Numeric property tests |
| MQ-FR-017 Configuration | SRS 4 | configs/README | — | TOML loader (Rust + Python) | Config validation + secret check |
| MQ-NFR-001 Correctness | SRS 5 | ARCH, RISK, EXEC | ADR-006 | All planes | Unit/property/integration |
| MQ-NFR-002 No look-ahead | SRS 5 | DATA_SPEC sec 9, STRATEGY | — | Backtest harness | Look-ahead regression tests |
| MQ-NFR-003 Reproducibility | SRS 5 | DATA_SPEC, ARCH | ADR-003 | Engine + experiment IDs | Deterministic replay |
| MQ-NFR-004 Perf (benchmark) | SRS 5 | TECH_STACK | ADR-001 | Criterion / pytest-benchmark | Benchmarks |
| MQ-NFR-005 Portability | SRS 5 | TECH_STACK | ADR-005 | CI matrix | macOS + Linux CI |
| MQ-NFR-006 Resilience | SRS 5 | OPERATIONS | — | Journal + recovery | Fail-closed + recovery tests |
| MQ-NFR-007 Security | SRS 5 | SECURITY | — | Secret handling + audit | `pip-audit` + `cargo deny` |
| MQ-NFR-008 Auditability | SRS 5 | INTERFACES sec 11 | — | Journal + event IDs | Trace test |
| MQ-NFR-009 Deterministic risk | SRS 5 | RISK_SPEC | ADR-006 | `crates/risk` | Risk-authority tests |
| MQ-NFR-010 No premature infra | SRS 5 | TECH_STACK | ADR-007 | Repo-wide | Arch review |

Notes:
- Strategy horizons: ADR-009. Laya constraints: ADR-008. Event model: ADR-003.
- Experiments E1-E10 in `docs/EXPERIMENT_PLAN.md` map to validation column.
