# META QUANT — Technology Stack

## Final baseline

| Area | Decision |
|---|---|
| Python | 3.12.x reference environment |
| Platforms | macOS ARM64 + Linux x86-64 first-class |
| Rust | Stable / 2024 edition |
| Data processing | Polars + NumPy; pandas only where interoperability requires it |
| Statistics | SciPy + statsmodels |
| ML | XGBoost primary; LightGBM experimental |
| Kalman/state-space | statsmodels or small tested implementation |
| Historical | Parquet |
| Analytical query | DuckDB |
| Columnar interchange | Apache Arrow |
| Operational DB | PostgreSQL |
| Async I/O | Tokio where needed |
| Runtime | deterministic Rust event loop |
| Python/Rust | PyO3/maturin at coarse boundaries |
| API/control | FastAPI |
| Frontend | React + TypeScript + Vite (when UI is built) |
| Config | TOML |
| Python packaging | pyproject.toml + uv.lock |
| Rust packaging | Cargo.lock + rust-toolchain.toml |
| Testing | pytest + Hypothesis + cargo test + Criterion |
| Quality | Ruff + mypy + rustfmt + Clippy |
| Security/deps | pip-audit + cargo-audit/cargo-deny |
| Containers | Docker Compose initially |
| CI | GitHub Actions |
| Logging | structured logs from day one |
| Metrics | essential metrics early; Prometheus/Grafana later |

## Explicitly deferred

Kafka, Redis, Kubernetes, Spark, Cassandra, TimescaleDB, GPU clusters, FPGA, exchange colocation and microservice sprawl are not part of the initial architecture.

They may be introduced only when a measurable requirement creates a reason.
