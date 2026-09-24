# ADR-002 — Historical Data Storage: Parquet + DuckDB + Arrow + PostgreSQL

**Status:** Accepted
**Date:** 2026-09-24
**Related:** docs/DATA_SPEC.md, docs/ARCHITECTURE.md sec 6

## Context

The system has two storage needs: large immutable historical/research datasets (ticks, quotes, trades, auction states) and mutable operational state (paper/shadow orders, positions, experiment metadata, alerts, audit records). Using one store for both would conflate a data-lake concern with an operational-database concern.

## Decision

- **Parquet** for historical data (canonical store).
- **DuckDB** for analytical querying over Parquet.
- **Apache Arrow** for columnar interchange between Python/Rust/intermediate stages.
- **PostgreSQL** for operational state: paper/shadow orders, positions, operational state, experiment metadata, alerts, durable operational records.
- **Native Rust structures** for hot-path order-book / market-state (not Arrow/Parquet objects).

PostgreSQL shall **not** become the primary historical tick-data lake.

## Alternatives considered

- **TimescaleDB / Cassandra / Spark lake:** rejected — premature distributed infrastructure without measured need (see ADR-007).
- **PostgreSQL for ticks:** rejected — wrong access pattern and cost for full-resolution replay.
- **Arrow/Parquet as hot-path book:** rejected — columnar interchange is not a runtime order-book representation.

## Consequences

- Polars + NumPy are primary processing; pandas only where library compatibility requires it.
- Data quality checks and leakage controls defined in `docs/DATA_SPEC.md` apply at the Parquet ingress boundary.

## Validation

- Data validation tests; DuckDB round-trip tests; replay from Parquet.
