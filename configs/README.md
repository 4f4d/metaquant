# Configuration

Canonical format: **TOML** (TECH_STACK).

- `*.example.toml` are committed and contain placeholders only.
- Real `*.toml` are **gitignored** and must never contain secrets.
- Secrets via environment: `BROKER_API_KEY`, `NEWS_FEED_TOKEN`, or `.env` (gitignored).
- Every limit/threshold/horizon is configuration (RISK_SPEC, ADR-009).

## Examples

- `basis.example.toml` — Spot-futures basis arbitrage.
- `stat_arb.example.toml` — Cointegration / stat-arb.
- `nse_bse.example.toml` — NSE-BSE cross-exchange.
- `paper.example.toml` — Paper/shadow run (mode = paper | shadow).

Copy and edit locally:

```bash
cp configs/paper.example.toml configs/paper.toml
# edit configs/paper.toml — do not commit
```
