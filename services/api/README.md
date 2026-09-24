# Services / API

FastAPI control plane — not the market-event execution loop.

- `meta_quant_api/main.py` — health/version endpoints (scaffold).
- Future: run control, experiment metadata, PostgreSQL-backed journal.

Run:

```bash
uv run uvicorn meta_quant_api.main:app --reload --port 8000
```
