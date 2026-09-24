# Security Policy

## Supported Versions

| Version | Supported |
|---|---|
| 0.2.x (architecture baseline / scaffold) | ✓ |
| < 0.2 | ✗ |

Security support is best-effort for this pre-implementation scaffold. Post-`0.2` releases will update this table.

## What Is Security-Sensitive

- Credentials or broker secrets committed or exposed in issues/PRs/logs
- Authentication/authorization bypass
- Ability to bypass deterministic risk controls
- Live-broker authority in a non-`BrokerAdapter` path
- Dependency vulnerabilities with high severity

## What Is Not Security-Sensitive

- Quantitative strategy questions (use GitHub Discussions/Issues)
- Feature requests

## Reporting a Vulnerability

**Do not open a public issue for sensitive vulnerabilities.**

For this repository, use **GitHub Security Advisories — private reporting**:

1. Go to the repository **Security** tab → **Report a vulnerability** → **Private vulnerability reporting**.
2. Alternatively, contact a maintainer privately via GitHub.

If the repository does not yet have a designated security contact, the GitHub private advisory mechanism is the canonical channel — do not invent an email address.

Please include: component, version/commit, reproduction steps, and impact.

## Handling

- We will acknowledge receipt and investigate promptly.
- We will coordinate disclosure and a fix before any public disclosure.
- Do not publish credentials, broker secrets, API keys, or live trading credentials in any issue, PR, discussion, or commit. Use environment variables (`.env`, gitignored) and `configs/*.example.toml` placeholders.

## Current Security Posture

- **No live broker authority:** default mode is `BACKTEST` / `PAPER` / `SHADOW` only. A real `BrokerAdapter` is future-only and requires separate approval (`docs/SRS.md` §3, `docs/SECURITY.md`).
- **No credentials in source:** enforced via `.gitignore` + secret-scanning (when enabled on GitHub, see `docs/BRANCH_PROTECTION.md`).
- **Dependencies pinned:** `uv.lock` / `Cargo.lock` with `pip-audit` / `cargo-deny` in CI.

For detailed technical rules (secret handling, live boundary diagram, dependency audit), see `docs/SECURITY.md` — this file is the GitHub-discoverable entry point.
