# Python — Research / ML / Analytics Plane

Python owns research, statistics, feature engineering, training, batch analytics,
notebooks, and experiment orchestration (ADR-001).

Boundary to Rust is coarse-grained batch only — no per-tick PyO3 callbacks (ADR-004).

```
python/meta_quant/
  research/     # stats, cointegration, walk-forward
  features/     # Polars + NumPy
  models/       # XGBoost (primary), LightGBM (comparison)
  analytics/    # P&L, risk metrics, reports
  experiments/  # reproducibility harness (MQ-FR-013)
  data/         # Parquet/DuckDB access (ADR-002)
  context/      # news/shock guard, optional Laya (ADR-008/011)
```
