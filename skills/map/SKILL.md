---
name: map
description: Map a named subsystem, protocol, data flow, or dependency chain.
---

# map

Map only the scope named by the user and remain read-only unless implementation
is explicitly requested.

Build a layered, evidence-backed map:

1. inputs, events, RPC, and external sources
2. workers, ingestion, and retry/reconciliation paths
3. state, storage, schemas, and materialization
4. serving APIs, current state, closed history, and live tails
5. frontend or downstream consumers
6. operations, health, monitoring, and recovery

For each layer, identify ownership, contracts, consumers, coupling, invariants,
security boundaries, failure modes, and ambiguous or missing edges. Include
resource notes where relevant: CPU, memory, disk, query/RPC fanout, retries,
and failure amplification. Preserve partial and unavailable states rather than
inventing completeness. List tests, available coverage, and the smallest
independently verifiable implementation slices.
