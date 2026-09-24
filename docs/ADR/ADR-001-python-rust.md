# ADR-001 — Python / Rust Split

**Status:** Accepted
**Date:** 2026-09-24
**Deciders:** Architecture baseline
**Related:** ADR-004, docs/ARCHITECTURE.md sec 2-4, docs/TECH_STACK.md

## Context

META QUANT must serve two distinct workloads: (1) quantitative research — statistics, feature engineering, model training, batch analytics, notebooks, experiment orchestration — where iteration speed and library ecosystem matter most; and (2) a deterministic event runtime — market state, order-book, order/position state machines, execution simulation, risk checks — where determinism, strong typing, and controlled performance matter most.

A single-language choice would compromise one workload. A per-tick polyglot hot path would compromise correctness and performance.

## Problem

Choose a language boundary that preserves research velocity without introducing per-event cross-language overhead or non-determinism on the market-event path.

## Decision

- **Python 3.12.x** owns research, statistics, feature engineering, model training, ML, notebooks, analytics, and experiment orchestration (`python/meta_quant/`).
- **Rust (stable, 2024 edition)** owns the deterministic event runtime, market state, order-book representation, order/position state machines, execution simulation, performance-sensitive risk logic, and eventual live components (`crates/*`).
- **Control plane** (FastAPI + React) is not the market-event execution loop.
- **PyO3 / maturin** is permitted only for **coarse-grained / batch** operations (e.g. `Rust engine -> batch result -> Python`), never per-market-event callbacks.

## Alternatives considered

- **Python-only runtime:** rejected — insufficient guarantees for deterministic ordering, state-machine correctness, and long-running replay without GIL/typing compromises.
- **Rust-only research:** rejected — would force statistical/ML prototyping into a systems language and lose the PyData ecosystem.
- **Per-tick Python callbacks from Rust (PyO3 per event):** rejected — introduces latency, GIL contention, and non-determinism; violates ADR-003.

## Consequences

- Keep `rust-toolchain.toml` as canonical Rust version.
- Performance optimisation is benchmark-driven (Criterion / pytest-benchmark); do not move code to Rust merely because "Rust is faster".
- The boundary is coarse-grained: batch inputs/models/feature sets into Rust, batch results/metrics/fills out.

## Validation

- `cargo fmt --check`, `cargo clippy`, `cargo test` for Rust.
- `uv run ruff check`, `uv run mypy` for Python.
- Deterministic replay test proves the hot path never invokes Python per tick.
