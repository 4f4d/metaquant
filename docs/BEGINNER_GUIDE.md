# META QUANT — Beginner Guide to the Architecture

This page explains the important terms in plain language.

## What is an event-driven system?

Instead of repeatedly asking the system “what is happening?”, the system reacts to events such as:

```text
new quote arrived
trade occurred
auction state changed
order filled
timer fired
```

This makes it natural to replay historical markets in the same order that a live system would process events.

## What is basis arbitrage?

Spot and futures represent related economic exposure. The strategy estimates a fair relationship and checks whether the observed difference is large enough to survive all costs.

## What is statistical arbitrage?

A pair or basket of securities has a statistically measured relationship. When their spread deviates unusually far from its historical relationship, the strategy can take offsetting positions and look for convergence.

## What is NSE–BSE arbitrage?

The same security can be quoted separately on NSE and BSE. The strategy looks for a simultaneously executable price difference after both-leg costs and execution risk.

## Why is execution separate from the strategy?

A strategy can be correct about a price relationship and still lose money because the intended orders cannot both be filled at the expected prices.

## What does ML do?

ML estimates whether an already detected opportunity is attractive under its current conditions.

It does not replace:

- accounting,
- risk limits,
- order mechanics,
- exchange rules.

## What does Laya do?

Laya is a specialized System-1 decision model. In META QUANT it is considered for contextual text such as news and policy events. It can classify/score an event, but it does not decide whether risk limits may be violated or whether an order is allowed.

## What is the closing auction?

A call auction uses aggregate demand and supply to determine the closing price rather than continuously matching every incoming order in the same way as normal continuous trading. META QUANT treats this as a separate market regime that may affect basis and convergence behavior.

## Why are there so many documents?

Because a large system should not require every engineer or AI agent to understand everything at once.

For example:

```text
Need to modify basis strategy?
    ↓
SRS
    ↓
STRATEGY_SPEC
    ↓
relevant ADR
    ↓
code + tests
```

That is faster and safer than asking an agent to infer the architecture from source code.
