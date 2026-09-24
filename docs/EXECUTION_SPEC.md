# META QUANT — Execution Specification

## 1. Purpose

The execution layer converts an approved signal into simulated or future broker-routed orders while modelling the difference between theoretical and realizable trading.

## 2. Order lifecycle

```mermaid
stateDiagram-v2
    [*] --> Created
    Created --> PendingRisk
    PendingRisk --> RejectedByRisk
    PendingRisk --> Approved
    Approved --> Submitted
    Submitted --> PartiallyFilled
    Submitted --> Filled
    Submitted --> Rejected
    PartiallyFilled --> PartiallyFilled
    PartiallyFilled --> Filled
    PartiallyFilled --> Cancelled
    Submitted --> Cancelled
    RejectedByRisk --> [*]
    Filled --> [*]
    Cancelled --> [*]
```

## 3. Two-leg execution

For arbitrage strategies, treat each hedge as a logical parent trade containing two child legs.

```text
ArbitrageOrder
    parent_id
    leg_A
    leg_B
    max_legging_time
    hedge_policy
    execution_status
```

The engine must represent partial/failure states explicitly rather than assuming atomic fills.

## 4. Fill model

At minimum support:

- market/limit semantics as defined by the simulation,
- queue/liquidity assumptions,
- partial fills,
- rejected orders,
- slippage,
- latency,
- market impact assumptions,
- cancellation/timeout.

## 5. Cost model

```mermaid
flowchart LR
    A[Gross spread] --> B[Exchange charges]
    B --> C[Brokerage / applicable fees]
    C --> D[Bid-ask crossing]
    D --> E[Slippage]
    E --> F[Market impact]
    F --> G[Financing / carry where applicable]
    G --> H[Net edge]
```

The exact charge schedule is configuration/data, not hard-coded strategy mathematics.

## 6. Latency model

Where historical timestamps permit, separate:

- market-event timestamp,
- ingestion time,
- strategy decision time,
- simulated order submission time,
- simulated acknowledgement time,
- simulated fill time.

Do not claim live exchange latency from a historical dataset that cannot support it.

## 7. Broker abstraction

```mermaid
flowchart TB
    E[Execution Engine] --> A[BrokerAdapter]
    A --> B[Broker implementation A]
    A --> C[Broker implementation B]
    A --> D[Future direct/exchange adapter]
```

Strategy code shall not depend directly on a broker-specific SDK.
