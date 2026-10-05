---
name: montecarlo
description: Use when the user tags $montecarlo, simulation, scenario analysis, or asks to model uncertain economics, liquidity, protocol risk, RPC/load, throughput, capacity, pricing, PnL, or tail outcomes before architecture or product decisions.
---

# Monte Carlo

Use this skill when a decision depends on uncertain distributions rather than one deterministic estimate. Prefer simple transparent models first.

## Simulation Workflow

1. Define the decision:
   - what choice the simulation should inform
   - what action changes based on the result
2. Define variables and distributions:
   - fixed inputs
   - uncertain inputs
   - correlations
   - bounds and impossible states
3. Define metrics:
   - expected value
   - p5/p50/p95/p99
   - probability of breach
   - worst-case or tail loss
   - break-even thresholds
   - sensitivity by variable
4. Run or sketch the model:
   - use a script when reproducibility matters
   - keep assumptions visible
   - use deterministic seeds when comparing scenarios
   - avoid pretending rough assumptions are precise
5. Interpret:
   - decision regions
   - efficient frontier
   - dominant risk drivers
   - what would falsify the model
   - what real measurement should replace the assumption

## Common Use Cases

- RPC credit budget and provider routing.
- Indexer catch-up throughput under variable block/event load.
- PCDS/CDS pricing, underwriter payoff, liquidity, and default risk.
- Account/history materialization cost under account-count growth.
- Frontend payload/memory distribution under large accounts or protocols.

## Output Shape

1. Decision and action threshold.
2. Assumptions and distributions.
3. Simulation method.
4. Results table and optional chart.
5. Sensitivity and failure modes.
6. Recommendation: proceed, redesign, measure, or run monitored experiment.

If the input assumptions are too weak, return a measurement plan before simulation-driven decisions.
