# META QUANT — Core Interfaces and Contracts

The exact language syntax may change; these semantic interfaces do not.
Refer to `schemas/README.md` for canonical logical schemas and to
`docs/ARCHITECTURE.md` for plane ownership.

## 1. MarketDataProvider / MarketDataFeed

**Responsibility:** Produce `MarketEvent`s in timestamp order from a source (historical file, paper feed, future broker).

| Aspect | Contract |
|---|---|
| Inputs | feed configuration, instrument universe, time range |
| Outputs | `MarketEvent` stream (QuoteEvent, TradeEvent, OrderBookUpdate, AuctionEvent) |
| Invariants | events emitted in non-decreasing `event_timestamp_utc_ns`; `sequence_number` gaps detectable where supplied; no future event emitted before current decision time in replay |
| Failure | `connect()`/`disconnect()` errors explicit; stale feed raises `RiskEvent`/health signal, not silent data |

```text
MarketDataFeed
    connect() -> Result<()>
    disconnect() -> Result<()>
    next_event() -> Option<MarketEvent>
    health() -> FeedHealth  // heartbeat, staleness, sequence_gap
```

Implementations: `HistoricalReplayFeed` (Parquet/Arrow), `PaperFeed` (live ingest, simulated orders). `LiveBrokerFeed` is deferred behind `BrokerAdapter`.

## 2. MarketState

**Responsibility:** Central mutable view of latest valid market truth consumed by strategies.

| Aspect | Contract |
|---|---|
| Inputs | `MarketEvent` stream, instrument metadata, session calendar |
| Outputs | queryable state for strategy `evaluate()` |
| Invariants | never exposes partially applied event; UTC canonical time; derived venue-local time is cached, not authoritative; missing data returns explicit `None`, not fabricated fill |
| Failure | stale-data flag propagated to `RiskEngine`; `MarketState::is_stale()` check mandatory before signal generation |

```text
MarketState
    apply(event: MarketEvent) -> Result<()>
    get_quote(instrument_id) -> Option<Quote>
    get_order_book(instrument_id) -> Option<OrderBookView>
    get_auction_state(instrument_id) -> Option<AuctionState>
    get_session_state() -> SessionState
    last_update_ns() -> u64
    is_stale(threshold_ns: u64) -> bool
```

Hot-path representation: native Rust structures (integer ticks/quantities); Arrow/Parquet only at ingestion/interchange boundary (`docs/DATA_SPEC.md`).

## 3. Strategy

**Responsibility:** Pure relative-value logic: detect candidate opportunity from `MarketState` without broker or risk side-effects.

```text
Strategy
    initialize(config: StrategyConfig, instruments: [Instrument]) -> Result<()>
    on_event(event: MarketEvent, state: MarketState) -> Vec<Opportunity>
    // or evaluate(state, timestamp) -> Vec<Opportunity>
    on_fill(fill: FillEvent) -> ()
    on_session_change(session: SessionState) -> ()
    reset() -> ()
```

| Aspect | Contract |
|---|---|
| Inputs | `MarketEvent`, `MarketState`, configuration (thresholds, horizons, universe) |
| Outputs | `Opportunity` / `Signal` objects containing assumptions + state snapshot for audit |
| Invariants | no direct order submission; no broker SDK import; horizon is a config property, not a global; no look-ahead (only state up to `event_timestamp_utc_ns`) |
| Failure | invalid config rejected at `initialize()`; runtime error returns empty vec + structured log, never panic in replay path |

All three families (Basis, StatArb, NSE-BSE) implement this contract. CAS and News contexts are injected as regime/feature inputs, not as strategies.

## 4. Opportunity / Signal

Logical value object produced by a strategy; consumed by `OpportunityEvaluator` / `TradeQualityModel` and `RiskEngine`.

```text
Opportunity
    opportunity_id: string
    strategy_id: string  // basis | stat_arb | nse_bse
    instrument_leg(s): InstrumentId + side + quantity hint
    detected_at_ns: u64
    market_snapshot: { quotes, spread, basis/deviation reference }
    gross_edge_bps: decimal  // explicit, not net
    cost_assumption_id: string
    horizon_hint: enum { intraday, short_term, medium_term }
    config_hash: string
    metadata: map
```

Net edge is derived downstream by execution/cost model; never implied by `gross_edge`.

## 5. OpportunityEvaluator / TradeQualityModel

**Responsibility:** Rank/filter candidates produced by strategies using quantitative and optional contextual features.

```text
TradeQualityModel
    evaluate(features: FeatureVector, context: Option<ContextFeatures>) -> QualityScore
    // QualityScore { score: f64 [0,1], calibrated_probability?: f64, model_version, model_hash, reasons[] }
```

| Aspect | Contract |
|---|---|
| Inputs | `FeatureVector` (spread, z-score, vol, liquidity, time-to-expiry, regime, shock state, auction state) |
| Outputs | `QualityScore` with version/hash for reproducibility (`docs/ML_SPEC.md`) |
| Invariants | deterministic given same inputs+model; never overrides risk; calibration measured on validation set |
| Failure | model unavailable -> `context_unavailable` not rejection of entire pipeline; fallback to rule-only evaluation |

Python owns training; Rust loads a serialised artifact and scores via coarse-grained batch call (ADR-001, ADR-004).

## 6. RiskEngine (authoritative)

**Responsibility:** Gate every candidate before execution; deterministic veto is final.

```text
RiskEngine
    evaluate(
        candidate: Opportunity | Order,
        account: AccountState,
        market: MarketState,
        shock_state: ShockState
    ) -> RiskDecision

RiskDecision
    decision: Approved | Rejected
    reason_codes: [string]  // e.g. MAX_GROSS_EXPOSURE, STALE_DATA, ABNORMAL_SPREAD
    checked_limits: { limit_name, value, threshold }
    timestamp_ns: u64
```

Supported checks (limits are configuration, not hard-coded — `docs/RISK_SPEC.md`):
max position, gross/net exposure (global/per-strategy/per-instrument), per-order notional, stale-data, abnormal spread, liquidity, legging timeout, model integrity, kill switch, shock restriction, regulatory/instrument restriction.

Failure behaviour: any integrity failure => fail-closed (no new risk); `RiskDecision::Rejected` persisted to operational journal/PostgreSQL.

## 7. ExecutionEngine

**Responsibility:** Convert approved `Signal`/`Opportunity` into `Order` lifecycle and simulated fills; model gap between gross and net edge.

```text
ExecutionEngine
    submit(order: Order) -> OrderId
    cancel(order_id: string) -> Result<()>
    on_market_event(event: MarketEvent) -> Vec<FillEvent | OrderEvent>
    on_timer(event: TimerEvent) -> Vec<FillEvent | OrderEvent>
    get_order(order_id) -> Option<OrderView>
```

Order lifecycle (`docs/EXECUTION_SPEC.md`): Created -> PendingRisk -> Approved -> Submitted -> PartiallyFilled/Filled/Rejected/Cancelled -> terminal. Two-leg parent orders expose `max_legging_time` and `hedge_policy`; atomic fill is never assumed.

Cost/slippage/latency/impact are explicit model parameters versioned in experiment metadata.

## 8. BrokerAdapter (future live boundary)

**Responsibility:** Isolate broker/exchange-specific SDKs. Strategies and `ExecutionEngine` never import broker types.

```text
BrokerAdapter
    place_order(order: Order) -> Result<BrokerOrderId>
    cancel_order(order_id: string) -> Result<()>
    get_orders() -> Vec<BrokerOrderView>
    get_positions() -> Vec<Position>
    get_account_state() -> AccountState
```

Current builds use a `SimulatedBrokerAdapter`; a real adapter is a future separately approved crate. Default config runs BACKTEST/PAPER/SHADOW only.

## 9. NewsProvider / ContextClassifier

**Responsibility:** Detect, deduplicate, and classify external contextual events; supply features to trade-quality and shock state.

```text
NewsProvider
    ingest(event: RawNewsEvent) -> NormalizedNewsEvent

ContextClassifier  // Laya is one implementation
    classify(event: NormalizedNewsEvent) -> ContextClassification

ContextClassification
    event_type: string
    affected_entities: [InstrumentId]
    affected_sectors: [string]
    materiality_class: enum { low, medium, high }
    uncertainty_score: f64
    model_version: string
    model_hash: string
    timestamp_ns: u64
```

Laya constraints (ADR-008): optional, behind clean interface, version/hash recorded, never authorizes orders, never overrides hard risk limits, system runs without it (`context_unavailable`).

## 10. ShockState / AuctionState

Shock and auction are regime inputs, not order producers.

```text
ShockState  // NORMAL | WATCH | RESTRICT | SUSPEND_NEW_ENTRIES | RECOVERY
    state: enum
    scope: { instruments | sectors | global }
    reason: string
    expires_at_ns?: u64

AuctionState  // per instrument, CAS 15:15-15:35 IST subset
    session_type: enum { continuous, closing_auction }
    reference_price_ticks?: i64
    indicative_equilibrium_price_ticks?: i64
    indicative_tradable_quantity?: i64
    cumulative_buy_quantity?: i64
    cumulative_sell_quantity?: i64
    imbalance_quantity?: i64
    imbalance_side?: enum
    status: enum
```

Both feed `MarketState` and are consumed as features by strategies and `TradeQualityModel`.

## 11. Event identity and observability

Every `MarketEvent`, `Signal`, `RiskDecision`, `OrderEvent`, `FillEvent`, `PositionEvent` requiring audit/replay carries: `event_id`, `event_type`, `event_timestamp_utc_ns`, `ingestion_timestamp_utc_ns`, `source`, `config_hash`/`strategy_version` where applicable. Structured logs + metrics are emitted at state transitions; metrics surface is Prometheus-compatible in later operational phase (`docs/OPERATIONS.md`).

## Ownership summary

| Interface | Primary owner | Language |
|---|---|---|
| MarketDataFeed, MarketState, RiskEngine, ExecutionEngine, Order/Position state machines | Runtime plane | Rust |
| Strategy logic (math), feature engineering, model training, experiment orchestration | Research plane | Python |
| TradeQualityModel training + batch scoring | Research (train) + Runtime (score via batch boundary) | Python -> Rust |
| BrokerAdapter | Adapter plane (isolated crate) | Rust |
| ContextClassifier (Laya) | Context plane | Python/service behind interface |
| Control/monitoring API | Control plane | FastAPI + React |
