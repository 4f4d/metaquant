# META QUANT — AI Agent Engineering Rules

These rules apply to every AI coding agent working in this repository
— including OpenCode, Muse Spark, and similar autonomous agents.

> Read this file before writing any code.

## 0. Operating contract

The repository documentation is the engineering contract. If the user's short
prompt contradicts the documentation or the authoritative decisions in the
initialization task, the documentation wins. Do not silently invent
architecture. Document the conflict as an **OPEN QUESTION / ADR candidate**.

## 1. Context before code

Before modifying code:

1. Read `README.md` (docs/README.md is the doc map, root README.md is the project overview).
2. Read `PROJECT_CONTEXT.md`.
3. Read `docs/SRS.md`.
4. Read the subsystem specification relevant to your task
   (`STRATEGY_SPEC.md`, `EXECUTION_SPEC.md`, `RISK_SPEC.md`, `ML_SPEC.md`,
   `NEWS_SHOCK_SPEC.md`, `AUCTION_SPEC.md`, `DATA_SPEC.md`, etc.).
5. Read `docs/ARCHITECTURE.md` and `docs/INTERFACES.md`.
6. Read applicable `docs/ADR/*`.
7. Inspect existing interfaces, schemas, configs, and tests before creating new ones.

Do not implement from the user's short prompt alone when a project
specification exists.

## 2. Architectural authority

- Do not silently change the architecture.
- Do not replace a technology because another tool is fashionable.
- Do not add Kafka, Redis, Kubernetes, Spark, Hadoop, Cassandra, TimescaleDB,
  FPGA, GPU infrastructure, microservices, or a new ML architecture without an
  explicit ADR and evidence of need.
- Do not add generic directional stock prediction as a core strategy.
- Do not turn ML into a fourth strategy — ML is a trade-quality / contextual
  layer (`docs/ML_SPEC.md`).
- Do not turn the closing-auction / CAS subsystem into a fourth strategy — it
  is an auction-aware market-regime / feature subsystem (`docs/AUCTION_SPEC.md`).
- If a requirement appears inconsistent or impossible, stop and report the
  conflict rather than improvising a silent change.

## 3. Financial correctness

- Never use future information in backtests (no look-ahead bias).
- Never use a historical fill price that could not have been known at decision time.
- Never calculate arbitrage P&L from mid-prices unless the execution model
  explicitly defines that assumption. Separate **gross edge** from
  **net executable edge**.
- Every strategy result must state its cost, slippage, latency, liquidity and
  fill assumptions.
- Never report gross strategy P&L as though it were realizable net profit.
- Never bypass a risk limit because a model has high confidence.
- Walk-forward / out-of-sample validation is required for research conclusions.
- Document survivorship-bias and universe-change handling where applicable.

## 4. Runtime boundaries — Python / Rust split

| Plane | Language | Responsibility |
|-------|----------|---------------|
| Research, statistics, feature engineering, training, batch analytics, notebooks, experiment orchestration | **Python 3.12.x** | `python/meta_quant/` |
| Deterministic event runtime, market state, order-book, order/position state machines, execution simulation, performance-sensitive risk | **Rust (stable, 2024 edition)** | `crates/*` |
| Control / API plane | **FastAPI + React** | `services/api/`, `web/` |

Rules:

- Do not call Python once per market event from the Rust hot path.
- PyO3 / maturin is for **coarse-grained / batch** operations, not per-tick
  callbacks. Pattern: `Rust engine -> batch result -> Python`.
- FastAPI is a control/API plane, not the market-event execution loop.
- Performance optimisation must be **benchmark-driven** (Criterion / pytest-benchmark).
  Do not move code to Rust merely because "Rust is faster".
- Keep `rust-toolchain.toml` as the canonical Rust version.

## 5. Risk authority

The deterministic risk engine is **authoritative**.

Models may produce:
- signals,
- scores,
- probabilities,
- contextual classifications.

Models may **not** override:
- exposure limits,
- position limits,
- loss limits,
- instrument restrictions,
- execution constraints,
- safety / kill-switch rules,
- regulatory controls.

Laya specifically **must not**: override hard risk limits, authorize a trade,
change maximum exposure, bypass kill switches, override regulatory constraints,
determine position size, or directly control order submission
(`docs/NEWS_SHOCK_SPEC.md`, `ADR-008`).

The core trading system must remain functional if Laya is unavailable.

## 6. Reproducibility

Record, where applicable:

- Git commit SHA
- dataset / version identifier
- configuration hash
- strategy version
- model version / hash
- random seed
- training / validation / test windows
- cost-model version
- execution-model version
- software / toolchain versions
- hardware and benchmark environment

Never use informal experiment naming such as `final_final2_real_final.csv`.

## 7. Tests are mandatory

A feature is not complete until the applicable tests pass.

Required classes include:
- unit tests (`tests/unit/`)
- property / invariant tests (`tests/property/`)
- integration tests (`tests/integration/`)
- deterministic replay / golden tests (`tests/replay/`)
- strategy validation tests
- performance benchmarks where performance is claimed (`Criterion`)

Every bug fix must add a regression test.

## 8. Interface preservation

- Preserve documented interfaces in `docs/INTERFACES.md` and `schemas/`.
- Prefer editing existing interfaces over creating parallel ones.
- If an interface must change, update the spec, add a migration note, and
  update all call sites and tests in the same change.

## 9. Documentation discipline

- Update `docs/*` when behaviour changes.
- Keep Mermaid diagrams embedded in Markdown using standard `mermaid` code blocks
  and ensure they agree with the written spec.
- Required diagrams: system architecture, data flow, runtime event flow,
  backtest/paper/shadow flows, strategy flows, execution/order lifecycle,
  risk authority, ML flow, news shock, Laya integration, closing auction,
  recovery/replay (`docs/ARCHITECTURE.md`, `docs/WORKFLOWS.md`).
- Use concise sections, tables, and examples. Modular documents — not one giant
  prose file.
- For beginner-facing concepts, explain in plain language before implementation
  detail (`docs/BEGINNER_GUIDE.md`, `docs/GLOSSARY.md`).

## 10. Preservation and verification

- Inspect existing `tests/` before editing code.
- Create or update tests for every behaviour change.
- Run verification before declaring done:
  `cargo fmt --check`, `cargo clippy`, `cargo test`, `uv run ruff check`,
  `uv run mypy`, `uv run pytest`.
- Report deviations from the spec explicitly.
- Do not silently change terminology — keep it consistent across SRS, specs,
  ADRs, README, and code.

## 11. Dependency and platform hygiene

- Python environment: `uv` + `pyproject.toml` + `uv.lock`, Python 3.12.x
  (see `ADR-005`). CI may test newer versions for forward compatibility.
- Do not assume Ubuntu-only paths (`/home/...`). Support **macOS ARM64** and
  **Linux x86-64**. Use cross-platform config and environment handling.
- Docker is not required for every development task.
- Avoid unnecessary dependencies. Do not introduce a dependency without
  checking whether the existing stack already satisfies the requirement.
- Pin / audit dependencies: `pip-audit`, `cargo-audit` / `cargo-deny`.

## 12. Safety boundaries

- Never claim unverified performance or profitability.
- Never introduce live-trading behaviour (real exchange order submission) unless
  explicitly requested and approved. Default mode is BACKTEST / PAPER / SHADOW.
- Never bypass deterministic risk controls.
- Never commit secrets (API keys, broker credentials). Use environment / secret
  mechanisms (`docs/SECURITY.md`).
- Live-capable code must remain behind `BrokerAdapter` (`docs/INTERFACES.md`).

## 13. Definition of done

An implementation is not complete merely because code compiles. The agent must
provide:

- what changed,
- which requirement it implements (e.g. `MQ-FR-001`),
- tests run and their result,
- benchmarks if relevant,
- known limitations,
- any deviations from the specification.
