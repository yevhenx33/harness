---
name: architecture
description: Use when the user tags $architecture or asks for architecture design, redesign, system decomposition, protocol-line architecture, scalability planning, performance architecture, or VPS/indexer platform architecture. Focuses on correctness, resource efficiency, operational boundaries, and implementable slices.
metadata:
  short-description: Architecture design with performance constraints
---

# Architecture Skill

Use this skill for `$architecture`, architecture redesign, platform design, protocol-line architecture, scalability, and performance-focused system planning.

## Principles

- Start from the running system and existing code before proposing design.
- Keep read-only until the user asks for implementation.
- Favor protocol-line isolation, clear ownership boundaries, measurable contracts, and small migration slices.
- Treat performance as a first-class requirement, not a follow-up.

## Architecture Workflow

1. Current-state map:
   - services/processes/containers
   - code ownership
   - databases/storage
   - data flows
   - APIs/frontends
   - monitoring/recovery
2. Requirement framing:
   - correctness invariants
   - latency and freshness targets
   - throughput/scale targets
   - durability and replay needs
   - operational constraints on VPS resources
3. Performance budget:
   - CPU budget per protocol/service
   - memory budget and resident-state model
   - disk growth, retention, and write amplification
   - RPC/network fanout
   - database query and materialization cost
   - startup/recovery time
4. Target architecture:
   - components and boundaries
   - contracts between layers
   - failure modes and recovery
   - observability and health checks
5. Migration plan:
   - narrow implementation slices
   - read-only validation before each slice
   - monitored experiment after each slice
   - rollback/stop criteria

## Performance Lens

Always ask:

- What state must be memory-first versus persisted?
- What writes happen per block/event/account?
- What queries scan history versus current/materialized state?
- What can be batched, cached, checkpointed, or compacted?
- What is the worst-case fanout during price/index/factor changes?
- What happens under RPC latency, database lag, or process restart?

## Output Shape

1. Current architecture.
2. Target architecture.
3. Performance/resource budget.
4. Risks and invariants.
5. Migration slices.
6. Monitored experiments to validate each slice.

Prefer diagrams and tables when they clarify boundaries or resource tradeoffs.
