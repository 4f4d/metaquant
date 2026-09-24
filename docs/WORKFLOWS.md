# META QUANT — End-to-End Workflows

This document explains what happens in the system, step by step, without assuming prior quant-engineering knowledge.

## 1. Historical backtest workflow

```mermaid
flowchart TD
    A[Select dataset + version] --> B[Validate data]
    B --> C[Replay events in timestamp order]
    C --> D[Update market state]
    D --> E[Evaluate session/regime]
    E --> F[Run strategy]
    F --> G[Candidate opportunity]
    G --> H[Feature generation]
    H --> I[Optional ML/context scoring]
    I --> J[Deterministic risk]
    J --> K[Execution simulation]
    K --> L[Fill / rejection]
    L --> M[Position + P&L]
    M --> N[Record event + metrics]
    N --> C
    N --> O[Experiment report]
```

## 2. Paper workflow

```mermaid
flowchart TD
    A[Live/Paper feed] --> B[Normalize]
    B --> C[Market state]
    C --> D[Strategies]
    D --> E[Risk]
    E --> F[Simulated execution]
    F --> G[Simulated fills]
    G --> H[Paper position/P&L]
    H --> I[Monitoring + journal]
```

No live order is submitted.

## 3. Shadow workflow

Shadow mode is more operationally realistic than paper mode because it exercises decision and routing logic against the live environment while preventing actual order submission.

```mermaid
flowchart LR
    A[Live feed] --> B[Runtime]
    B --> C[Strategy]
    C --> D[Context/ML]
    D --> E[Risk]
    E --> F[Shadow order]
    F --> G[Execution simulation]
    G --> H[Compare hypothetical vs market outcome]
```

## 4. Future live workflow

Live trading is deferred and must be separately approved.

```mermaid
flowchart TD
    A[Market feed] --> B[Runtime]
    B --> C[Strategy]
    C --> D[Risk authority]
    D --> E[BrokerAdapter]
    E --> F[Broker / exchange path]
    F --> G[Exchange acknowledgement / fill]
    G --> H[Reconciliation]
    H --> I[Position + audit state]
```

## 5. Basis arbitrage workflow

```mermaid
flowchart LR
    A[Spot + futures state] --> B[Fair-value / carry model]
    B --> C[Observed basis]
    C --> D[Deviation]
    D --> E[Cost + slippage estimate]
    E --> F[Candidate]
    F --> G[Optional auction/context features]
    G --> H[ML quality score]
    H --> I[Risk]
    I --> J[Two-leg execution simulation]
    J --> K[Convergence / exit]
```

## 6. Statistical arbitrage workflow

```mermaid
flowchart TD
    A[Instrument universe] --> B[Pair / basket selection]
    B --> C[Stationarity / cointegration tests]
    C --> D[Estimate hedge ratio]
    D --> E[Construct spread]
    E --> F[Spread statistics / z-score]
    F --> G[Entry condition]
    G --> H[Trade-quality model]
    H --> I[Risk]
    I --> J[Long/short execution]
    J --> K[Convergence / exit / timeout]
```

## 7. NSE–BSE workflow

```mermaid
flowchart LR
    A[NSE quote] --> C[Timestamp alignment]
    B[BSE quote] --> C
    C --> D[Executable cross-exchange spread]
    D --> E[Fees + slippage + liquidity]
    E --> F[Leg-risk check]
    F --> G[Candidate]
    G --> H[Risk]
    H --> I[Two-leg execution simulation]
```

This strategy is primarily intraday and extremely sensitive to timestamp quality and execution assumptions.

## 8. News/policy-shock workflow

```mermaid
flowchart TD
    A[News / filing / policy event] --> B[Deduplicate / normalize]
    B --> C[Entity + sector mapping]
    C --> D[Semantic classification]
    D --> E[Market shock confirmation]
    E --> F[Shock state]
    F --> G[Strategy restriction / feature]
    G --> H[Re-check after stabilization]
```

### Example

A policy announcement may be classified by the contextual model as relevant to the automotive sector. Quantitative measurements then confirm whether volatility, spread width, volume, correlation or liquidity also changed materially. The resulting shock state may reduce or suspend new entries in affected strategies.

## 9. Closing-auction workflow

```mermaid
flowchart TD
    A[3:00-3:15 cash trades] --> B[VWAP reference]
    B --> C[CAS reference/indicative state]
    C --> D[Auction imbalance + quantities]
    D --> E[3:15-3:35 CAS evolution]
    E --> F[Final cash auction result]
    F --> G[3:35-3:40 derivative window]
    G --> H[Measure basis/convergence]
    H --> I[Cost-aware opportunity analysis]
```

This is initially an event-study/research feature. The system must not assume profitability in advance.

## 10. Failure workflow

```mermaid
flowchart TD
    A[Runtime anomaly] --> B{Critical?}
    B -->|No| C[Log + continue]
    B -->|Yes| D[Fail closed]
    D --> E[Stop new risk-taking]
    E --> F[Persist state]
    F --> G[Operator alert]
    G --> H[Recover / replay / reconcile]
```
