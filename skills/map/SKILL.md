---
name: map
description: Map a named subsystem, protocol, data flow, or dependency chain.
---

# map

Map only the scope named by the user and remain read-only unless implementation
is explicitly requested.

Start with one primary invariant and its authoritative producer. Trace it through
every ownership, storage, interface, asynchronous, and consumer boundary. Treat
derived views and health summaries as consumers or evidence, not owners.

Recover relevant prior maps and decisions, rechecking drift-prone edges rather
than repeating full discovery. Build an evidence-backed map of applicable layers:

1. inputs, events, RPC, and external sources
2. workers, ingestion, and retry/reconciliation paths
3. state, storage, schemas, and materialization
4. serving APIs, current state, closed history, and live tails
5. frontend or downstream consumers
6. operations, health, monitoring, and recovery

For each applicable layer, identify ownership, contracts, consumers, coupling,
invariants, security boundaries, failure modes, and ambiguous or missing edges. Include
resource notes where relevant: CPU, memory, disk, bytes, query/RPC fanout,
retries, rebuilds, copies, queues, concurrency, and failure amplification.

Name each growing dimension (`n` records, `m` edges, `p` partitions, `f` fanout,
`r` retries, `d` delta) and state observed or expected work in those variables.
Mark request-path versus background work, bounded versus unbounded queues,
ordering and idempotency requirements, commit ownership, backpressure, and
partial-failure containment.

Preserve partial and unavailable states rather than inventing completeness.
Distinguish observed edges from inference and stop once the requested map and
material gaps are covered. Identify direct consumer oracles, including initial
state and subsequent updates where relevant, existing checks, and the smallest
independently verifiable next slices. Carry the original requirements and
unresolved gates into any later authorized implementation.
