# Branch Protection and Repository Settings (Recommended)

This document describes the **recommended** GitHub settings to apply **manually** on the GitHub repository (do not attempt to change settings from the local clone).

## Recommended `main` Ruleset

On GitHub → Settings → Branches → Add branch protection rule (or Ruleset):

- **Branch name pattern:** `main`
- **Require a pull request before merging:** ✓ enabled
- **Require approvals:** 1
- **Require review from Code Owners:** disabled (deferred; see below)
- **Require approval from someone other than the last pusher:** enabled (when available via ruleset)
- **Require status checks to pass before merging:** ✓ enabled
  - Required checks: `python`, `rust`, `web`, `docs` (from `ci.yml`), `analyze (python)`, `analyze (javascript)` (from `codeql.yml` once enabled)
- **Require conversation resolution before merging:** ✓ enabled
- **Require signed commits:** disabled for now (revisit if supply-chain policy tightens)
- **Block force pushes:** ✓ enabled
- **Block branch deletion:** ✓ enabled
- **Require linear history:** optional; not required for this team
- **Do not require CODEOWNERS:** use ordinary PR review (3-person team works cross-cuttingly; CODEOWNERS would add overhead without benefit yet)

## Why Not CODEOWNERS Now

The team has two primary developers working across many modules rather than owning isolated paths. Ordinary PR review + the `PULL_REQUEST_TEMPLATE.md` checklist achieves review coverage with less friction. Revisit CODEOWNERS only if a sensitive path (e.g. `crates/risk/`) repeatedly lacks review.

## Settings to Enable Manually on GitHub

These cannot be set from the local repository; enable via GitHub web UI.

### Secret Scanning and Push Protection

- Settings → Code security → **Secret scanning:** Enable
- Settings → Code security → **Push protection:** Enable (blocks commits containing secrets such as `BROKER_API_KEY`)

### Dependabot

- Settings → Code security → **Dependabot alerts:** Enable
- Settings → Code security → **Dependabot security updates:** Enable
- Configuration is in `.github/dependabot.yml` (pip, cargo, npm, github-actions, docker — weekly, low volume)

### Dependency Graph

- Enabled automatically for public repositories; verify at Settings → Code security → **Dependency graph:** Enabled. Required for Dependabot and vulnerability alerts.

### CodeQL / Code Scanning

- Settings → Code security → **Code scanning:** Enable
- Default setup → Add `codeql.yml` workflow (already in `.github/workflows/codeql.yml`). Results appear under Security → Code scanning.
- Languages enabled: `python`, `javascript` (TypeScript via JS). Rust analysis via CodeQL is not in the default set; consider third-party Rust queries later if desired — not required for scaffold.

### Private Vulnerability Reporting

- Settings → Code security → **Private vulnerability reporting:** Enable (so `SECURITY.md` “Report a vulnerability” button works).

## CI Blocking Policy

- `ci.yml` jobs `python`/`rust`/`web`/`docs` should be **required** before merge.
- Security jobs `cargo deny` / `pip-audit` are currently **advisory/non-blocking** (see `ci.yml` comments) to avoid blocking scaffold while dependency advisories are triaged. Move to required once the advisory baseline is clean and `deny.toml` allowlist is finalized.
- `codeql.yml` should be required once the initial CodeQL baseline is clean.

## How to Verify

```bash
test -f LICENSE && grep -q Apache-2.0 pyproject.toml && echo "license ok"
cargo deny check   # local
gh api repos/metaquant/metaquant/branches/main/protection  # after enabling
```
