# ADR-004 — PyO3 / Maturin Boundary (Coarse-Grained, Not Per-Tick)

**Status:** Accepted
**Date:** 2026-09-24
**Related:** ADR-001, ADR-008, docs/ARCHITECTURE.md sec 4

## Context

ADR-001 separates Python (research/ML) and Rust (runtime). An FFI boundary is needed, but its granularity determines performance, determinism, and correctness. A per-market-event Python callback would negate the benefits of a Rust runtime.

## Decision

- **PyO3 / maturin** is permitted **only for coarse-grained / batch** operations.
- Pattern: `Rust engine -> batch result -> Python` (or batch inputs/models into Rust).
- **Anti-pattern (forbidden):** `Rust event -> Python -> Rust -> Python` for every tick/event.

## Alternatives considered

- **Per-tick PyO3 callbacks:** rejected — latency, GIL contention, non-determinism.
- **No Python interop (Rust-only ML):** rejected — would force ML retraining/inference into Rust without justification.
- **IPC/microservice per event:** rejected — same granularity fault as per-tick FFI; see ADR-007.

## Consequences

- Inference/scoring that depends on Python models occurs on batches or via serialized artefacts loaded in Rust.
- Laya/news-context scoring, if Python-hosted, is batched, not inline per market tick.
- Benchmarks must justify any boundary change.

## Validation

- Criterion benchmark showing no per-event Python invocation.
- CI check that hot-path crate does not depend on `pyo3` as a per-event feature.
