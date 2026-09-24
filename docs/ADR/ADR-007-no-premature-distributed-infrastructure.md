# ADR-007 — No Premature Distributed Infrastructure

**Status:** Accepted
**Date:** 2026-09-24
**Related:** docs/TECH_STACK.md, docs/ARCHITECTURE.md sec 3

## Context

Trading-platform marketing often suggests Kafka, Redis, Kubernetes, Spark, Cassandra, TimescaleDB, GPU clusters, FPGA, or exchange colocation by default. META QUANT's current problem is quantitative correctness and realistic event/execution simulation — not distributed scale.

## Decision

Do **not** introduce Kafka, Redis, Kubernetes, Spark, Hadoop, Cassandra, TimescaleDB, FPGA, GPU infrastructure, or a microservice sprawl in the initial architecture. Initial stack: Docker Compose, Parquet + DuckDB + PostgreSQL, single-process Rust runtime, FastAPI control plane, XGBoost on tabular features.

## Alternatives considered

- **Kafka for market events:** rejected — replay needs deterministic ordering, not a distributed log, at current scale.
- **Redis for hot state:** rejected — native Rust structures already own hot state; Redis adds operational cost without measured need.
- **Kubernetes / Spark / Hadoop:** rejected — no measured throughput requirement justifies them.

## Consequences

- Infrastructure may be introduced **only** through an explicit ADR supported by measured requirements (profiling, benchmark, operational incident).
- Documentation must not imply distributed capabilities the system does not have.

## Validation

- Architecture review checks for premature deps; `cargo-deny` / `pip-audit` keep dependency surface minimal.
