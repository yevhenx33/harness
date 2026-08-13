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

Build a layered, evidence-backed map:

1. inputs, events, RPC, and external sources
2. workers, ingestion, and retry/reconciliation paths
3. state, storage, schemas, and materialization
4. serving APIs, current state, closed history, and live tails
5. frontend or downstream consumers
6. operations, health, monitoring, and recovery

For each layer, identify ownership, contracts, consumers, coupling, invariants,
security boundaries, failure modes, and ambiguous or missing edges. Include
resource notes where relevant: CPU, memory, disk, bytes, query/RPC fanout,
retries, rebuilds, copies, queues, concurrency, and failure amplification.

Name each growing dimension (`n` records, `m` edges, `p` partitions, `f` fanout,
`r` retries, `d` delta) and state observed or expected work in those variables.
Mark request-path versus background work, bounded versus unbounded queues,
ordering and idempotency requirements, commit ownership, backpressure, and
partial-failure containment.

Preserve partial and unavailable states rather than inventing completeness. List
tests, available coverage, direct oracles, and the smallest independently
verifiable implementation slices.
