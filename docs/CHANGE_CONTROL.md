# META QUANT — Change Control

## What requires an ADR

Create/update an ADR before changing:

- programming-language split,
- event model,
- storage architecture,
- execution semantics,
- risk authority,
- broker boundary,
- ML authority,
- supported execution modes,
- external infrastructure topology.

## What does not require an ADR

Routine changes such as:

- fixing a bug while preserving the contract,
- adding tests,
- refactoring without changing behavior,
- implementing a parameter already defined as configurable.

## Research changes

Changing a strategy threshold or model hyperparameter should be tracked as an experiment/configuration change and must not silently become a new architectural decision.
