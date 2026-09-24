# Contributing to META QUANT

> For AI agents: also read `AGENTS.md` before any code change.

## Overview

META QUANT is an event-driven relative-value research platform. The repository is maintained by a small three-person team. Contributions are via lightweight PR review, not heavyweight governance.

## Workflow

1. **Branch:** create a feature branch from `main` (`feature/mq-fr-001-description` or `fix/...`).
2. **Context before code:** read `README.md` → `PROJECT_CONTEXT.md` → `docs/SRS.md` → relevant `docs/*_SPEC.md` → `docs/ARCHITECTURE.md` + `docs/INTERFACES.md` → applicable `docs/ADR/*`. Inspect existing interfaces/schemas/configs/tests.
3. **Config, not hard-code:** horizons, thresholds, costs, and limits are TOML configuration, not globals.
4. **Implement + test:** update/add tests for every behaviour change. Required test classes: `tests/unit/`, `tests/property/`, `tests/integration/`, `tests/replay/` (deterministic replay / golden). Run verification before pushing (see below).
5. **Docs:** update `docs/*`, `schemas/README.md`, and Mermaid diagrams when behaviour changes. Diagrams must agree with text.
6. **ADR:** architectural changes (language split, event model, storage, execution semantics, risk authority, broker boundary, ML authority, execution modes, infra topology) require an ADR before code.
7. **Financial-math changes:** require mathematical documentation, unit tests, reference-result tests, and tolerance definitions.
8. **PR:** open a PR, fill the template, request review from at least one other team member. CI must be green.

## Pull Request Expectations

- PR description references requirement ID (`MQ-FR-xxx` / `MQ-NFR-xxx`) and relevant ADR/spec.
- Lists tests run and result, benchmarks if claimed, known limitations, deviations.
- Does not silently change architecture or add dependencies without ADR/evidence.
- No strategy/broker/ML implementation beyond scaffold without spec alignment.

## Test Requirements

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo test --all
uv run ruff check . && uv run mypy python/ && uv run pytest -q
# web when built: npm ci && npm run build
```

A feature is not complete until tests pass and replay/determinism invariants hold.

## Documentation Requirements

- Keep `docs/SRS.md` authoritative; `docs/INTERFACES.md` and `schemas/` stable.
- Prefer editing existing interfaces over parallel ones; migration notes required if interface changes.
- Use concise sections, tables, examples; Mermaid code blocks for required diagrams.

## AI-Agent Contribution Expectations

- Read `AGENTS.md` in full.
- Do not bypass deterministic risk controls; do not enable live trading without explicit approval + `BrokerAdapter` boundary.
- Never commit secrets; never claim unverified profitability.

## Licensing and SPDX

- All contributions are under **Apache License 2.0**. See `LICENSE`. Copyright holder placeholder is `META QUANT Contributors` pending formal entity (see `LICENSE` appendix).
- **Source-file headers:** newly controlled source files (Rust `*.rs`, Python `*.py`, TS `*.ts/*.tsx`) should start with:
  - Rust/TS: `// SPDX-License-Identifier: Apache-2.0`
  - Python: `# SPDX-License-Identifier: Apache-2.0`
- Do not add headers to generated files, lockfiles (`uv.lock`, `Cargo.lock`), or third-party code.
- A `NOTICE` file is **not** required for the current contents (no bundled NSE data or third-party attribution inclusion). If a future release bundles attributable third-party code, add `NOTICE` per Apache-2.0 §4(d).

## Review

- `main` is protected (see `docs/BRANCH_PROTECTION.md`): PR required, 1 approving review, required CI, conversations resolved, force-push blocked.
- Small team: normal PR review is preferred over `CODEOWNERS`.

## Reporting Issues

Use `.github/ISSUE_TEMPLATE/`. For security vulnerabilities, see `SECURITY.md` (root) — do not file publicly.
