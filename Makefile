.PHONY: fmt clippy test py-lint py-test audit

fmt:
	cargo fmt --check

clippy:
	cargo clippy -- -D warnings

test:
	cargo test --all
	uv run --python 3.12 pytest -q

py-lint:
	uv run --python 3.12 ruff check .
	uv run --python 3.12 mypy python/

audit:
	uv run --python 3.12 pip-audit || true
	cargo audit || true
	cargo deny check || true
