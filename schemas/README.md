# Canonical Schemas

Logical schemas are the canonical semantics; JSON/Arrow/Serde definitions follow them where needed. See `docs/DATA_SPEC.md` and `docs/INTERFACES.md` for normative contracts.

## Instrument

```text
Instrument
  instrument_id: string  # e.g. RELIANCE, RELIANCE24DECFUT
  exchange: string       # NSE | BSE
  segment: string        # EQ | FUT | OPT
  symbol: string
  expiry: datetime?      # UTC; None for spot
  tick_size: decimal     # Decimal price increment; runtime uses integer ticks (price_ticks = price / tick_size)
  lot_size: int          # integer exchange units; runtime quantities are i64
  currency: string       # explicit, expected INR for NSE/BSE (DATA_SPEC §1)
```

## MarketEvent (envelope)

```text
MarketEvent
  event_id: string
  event_type: string       # QuoteEvent | TradeEvent | OrderBookUpdate | AuctionEvent | NewsEvent | TimerEvent | SystemEvent
  event_timestamp_utc_ns: u64
  ingestion_timestamp_utc_ns: u64
  exchange: string
  segment: string
  instrument_id: string
  sequence_number: u64?
  payload: object          # variant-specific
```

Invariants: `event_timestamp_utc_ns` non-decreasing in replay (`u64` nanoseconds, UTC canonical); `sequence_number` gaps detectable where supplied. `ingestion_timestamp_utc_ns` and `normalized_timestamp_utc_ns` are also UTC nanoseconds.

## Quote

```text
Quote
  instrument_id: string
  exchange: string
  timestamp_ns: u64
  bid_price_ticks: i64
  bid_quantity: i64
  ask_price_ticks: i64
  ask_quantity: i64
```

Numeric: integer ticks (`i64`) and integer quantities (`i64`, `lot_size`) for runtime; `currency` explicit on Instrument (expected INR); timestamps `u64` nanoseconds UTC; float64 only for research/ML (DATA_SPEC §1/§3, MQ-FR-016).

Instrument-master source of truth is an open question (DATA_SPEC §1); do not invent the NSE/BSE feed source.

## Trade

```text
Trade
  instrument_id: string
  exchange: string
  timestamp_ns: u64
  price_ticks: i64
  quantity: i64
  aggressor_side: string?  # Buy | Sell | null
```

## OrderBookUpdate

```text
OrderBookUpdate
  instrument_id: string
  exchange: string
  timestamp_ns: u64
  bids: [(price_ticks: i64, qty: i64)]  # sorted descending
  asks: [(price_ticks: i64, qty: i64)]  # sorted ascending
```

Hot-path representation: native Rust `mq-orderbook` (not Arrow).

## AuctionEvent / AuctionState (ADR-010)

```text
AuctionState
  instrument_id: string
  exchange: string
  timestamp_ns: u64
  session_type: string     # continuous | closing_auction
  reference_price_ticks: i64?
  indicative_equilibrium_price_ticks: i64?
  indicative_tradable_quantity: i64?
  cumulative_buy_quantity: i64?
  cumulative_sell_quantity: i64?
  imbalance_quantity: i64?
  imbalance_side: string?
  status: string           # e.g. open | closed | final
```

Fields absent from the source feed must not be fabricated.

## NewsEvent / ContextEvent

```text
NewsEvent
  event_id: string
  timestamp_ns: u64
  source: string
  raw_text: string
  entities: [string]
  sectors: [string]

ContextClassification  # via ContextClassifier (optionally Laya, ADR-008)
  event_type: string
  affected_entities: [string]
  affected_sectors: [string]
  materiality_class: low | medium | high
  uncertainty_score: float [0,1]
  model_version: string
  model_hash: string
  timestamp_ns: u64
```

Laya outputs are `context_unavailable` on failure; never authorize orders.

## Opportunity / Signal

```text
Opportunity
  opportunity_id: string
  strategy_id: string      # basis | stat_arb | nse_bse
  detected_at_ns: u64
  instrument_legs: [{ instrument_id, side: Buy|Sell, quantity: i64 }]
  gross_edge_bps: decimal  # explicit; net is downstream (MQ-FR-005)
  config_hash: string
  metadata: object         # horizon, params, snapshot refs
```

## Order / Fill

```text
Order
  order_id: string
  instrument_id: string
  side: Buy | Sell
  order_type: Market | Limit | etc
  price_ticks: i64?
  quantity: i64
  created_at_ns: u64
  status: Created | PendingRisk | Approved | Submitted | PartiallyFilled | Filled | Cancelled | Rejected | RejectedByRisk

Fill
  fill_id: string
  order_id: string
  instrument_id: string
  price_ticks: i64
  quantity: i64
  timestamp_ns: u64
```

Two-leg arbitrage: `ArbitrageOrder { parent_id, leg_A, leg_B, max_legging_time, hedge_policy }` — partial/failure states explicit (EXECUTION_SPEC sec 3).

## Position

```text
Position
  instrument_id: string
  quantity: i64
  avg_price_ticks: i64
  realized_pnl_ticks: i64
  unrealized_pnl_ticks: i64
  updated_at_ns: u64
```

## RiskSnapshot

```text
RiskSnapshot / RiskDecision
  decision: Approved | Rejected
  reason_codes: [string]   # MAX_POSITION, KILL_SWITCH, SHOCK_RESTRICT, etc.
  checked_limits: [{ limit_name, value, threshold }]
  timestamp_ns: u64
```

Risk is authoritative and deterministic (ADR-006).

## PnlSnapshot

```text
PnlSnapshot
  timestamp_ns: u64
  realized_pnl: decimal
  unrealized_pnl: decimal
  gross_pnl: decimal
  net_pnl: decimal        # after cost model
  cost_breakdown: object
  config_hash: string
  execution_model_version: string
```

Net vs gross separation is mandatory (MQ-FR-005).

## Event identity (all auditable events)

Every auditable event carries `event_id`, `event_type`, `event_timestamp_utc_ns`, `ingestion_timestamp_utc_ns`, `source`, and where applicable `config_hash`/`strategy_version` (INTERFACES sec 11).
