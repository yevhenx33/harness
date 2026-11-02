---
name: architecture
description: Use when the user tags $architecture or asks for architecture design, redesign, system decomposition, protocol-line architecture, scalability planning, performance architecture, or VPS/indexer platform architecture. Focuses on correctness, resource efficiency, operational boundaries, and implementable slices.
metadata:
  short-description: Architecture design with performance constraints
---

# Architecture

Apply the root outcome-first design pass before a new feature. Map the concrete
contract, existing owners, execution path, and governing resource constraints
before selecting mechanisms. Separate required outcomes from implementation
assumptions and preserve user-selected architecture.

Propose a lean structural design that removes unnecessary work and state. Prefer
independently deployable microservices with coherent responsibilities, owned
state, published contracts, resource budgets, and failure isolation. Reuse the
existing owning service before creating another. Shared deployment infrastructure
does not grant shared write ownership.

Define stable keys, incoming events, affected dependencies, update ownership,
invalidation, and bounded full recovery. Include acknowledgement, backpressure,
late events, correction, deletion, ordering, idempotency, and restart where material.
Do not move a full scan into another service or bypass its private state.

Compare the suggested approach with the recommendation: work/state removed,
producer-to-consumer path, interface changes, latency/freshness/throughput,
CPU/memory/disk/RPC costs, and migration/recovery consequences. Include index
maintenance and cross-service communication, serialization, consistency,
deployment, and failure costs. Label assumptions and estimates.

Define the architecture falsifier, affected stop scope, and resume condition.
Test the riskiest assumption with a bounded probe before dependent implementation.
Distinguish repairable implementation errors from a defeated architecture.

Plan migration with parity, cutover, rollback, and retirement of superseded paths.
Reuse valid evidence; validate changed assumptions and affected contracts. Run a
monitored experiment when acceptance requires new runtime evidence, not after
every patch. Report the design, material tradeoffs, evidence gaps, and next action;
use diagrams or budget tables only when they clarify a decision.
