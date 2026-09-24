<!-- SPDX-License-Identifier: Apache-2.0 -->
## Summary

What changed and why.

## Requirements

- Implements: MQ-FR-___ / MQ-NFR-___
- Spec/ADR referenced: `docs/...` / `docs/ADR/ADR-___`

## Tests

- [ ] `cargo fmt --check && cargo clippy -- -D warnings && cargo test --all`
- [ ] `uv run ruff check . && uv run mypy python/ && uv run pytest -q`
- [ ] `web: npm ci && npm run build` (if web changed)

List tests added/updated:

## Documentation

- [ ] `docs/*` / `schemas/README.md` / Mermaid diagrams updated where behaviour changed
- [ ] No silent architecture change without ADR

## Verification

Commands run and result:

## Checklist

- [ ] No live-broker authority introduced (default remains BACKTEST/PAPER/SHADOW)
- [ ] No deterministic risk bypass
- [ ] No secrets committed (`configs/*.toml` ignored, `.env` not committed)
- [ ] Source files carry `SPDX-License-Identifier: Apache-2.0` where appropriate
