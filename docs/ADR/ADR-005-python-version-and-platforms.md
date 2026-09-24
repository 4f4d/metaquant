# ADR-005 — Python 3.12.x and Cross-Platform Support

**Status:** Accepted
**Date:** 2026-09-24
**Related:** docs/TECH_STACK.md, docs/SRS.md NFR-05

## Context

The project must choose a reference Python version and supported operating systems that balance library maturity, reproducibility, and developer environment reality (macOS ARM64 for local research, Linux x86-64 for CI/production).

## Decision

- **Python 3.12.x** is the reference environment. Pin via `.python-version`, `pyproject.toml`, and `uv.lock`.
- **macOS ARM64** and **Linux x86-64** are **first-class** supported development/runtime environments.
- Do not hard-code OS-specific paths (`/home/...`, `/usr/...`); use cross-platform config and environment handling.
- Shell scripts are portable or explicitly marked platform-specific.
- Docker is not required for every development task.

## Alternatives considered

- **Python 3.13 as reference:** rejected — eco-maturity not yet justified; CI may test it for forward compatibility but lock remains 3.12.x until a controlled migration ADR.
- **Ubuntu-only:** rejected — violates actual contributor environments and harms reproducibility.

## Consequences

- CI matrix includes 3.12.x (required) and optionally newer versions (informational).
- Dependency audit (`pip-audit`) runs against the 3.12 lock.
- `uv` is the canonical environment manager.

## Validation

- CI builds on macOS and Linux.
- `uv lock --check` and `pyproject.toml` `requires-python ==3.12.*` enforcement.
