# Test Structure

| Directory | Purpose | Example |
|---|---|---|
| `tests/unit/` | Unit tests (Python) | `test_reproducibility.py` |
| `tests/integration/` | Integration tests | feed -> market-state -> strategy |
| `tests/property/` | Hypothesis property tests | invariants, numeric representation |
| `tests/replay/` | Deterministic replay / golden tests | event sequence replays identically |
| `tests/fixtures/` | Shared fixtures, sample Parquet, synthetic events | `sample_quote.json` |
| `crates/*/src` | Rust unit tests (`cargo test`) | book, risk, execution invariants |

Every bug fix must add a regression test (AGENTS.md sec 7).

Run:

```bash
uv run pytest -q            # Python
cargo test --all             # Rust
cargo fmt --check && cargo clippy -- -D warnings
uv run ruff check . && uv run mypy python/
```
