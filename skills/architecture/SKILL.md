---
name: architecture
description: Design a bounded architecture slice before implementation.
---

# architecture

Use this skill for changes that cross boundaries or affect APIs, schemas,
storage, migrations, dependencies, security, money, concurrency, or data loss.

Separate current evidence from target architecture. Define the smallest required
slice across these sovereign planes when applicable:

- trust and admission: principal, policy, authority, expiry, and replay boundary
- capability: explicit operations, resources, budgets, revocation, and deny path
- execution: replaceable worker, input snapshot, lifecycle, and output contract
- verification/commit: independent oracle, invariant, ownership, and atomicity
- evidence/learning: receipt, provenance, projection, retrieval, and reverification

For every slice name the owner, consumers, public contracts, alternatives,
compatibility, recovery, and observability. Budget product and review LOC, time,
latency endpoints, memory, storage, bytes, network fanout, retries, concurrency,
queues, test runtime, and operational risk using named growing dimensions.

Define failure cells with one commit owner, blast radius, containment, detection,
retry or compensation, and recovery responsibility. A cell is implementation-
ready only when its authority, inputs, state, outputs, material failures, oracle,
resource ceiling, and rollback are explicit.

Prefer ownership partitioning, delta processing, bounded coordination, and
independently verifiable slices. Split work before the approval gate when one
slice would exceed it. Do not present an unbuilt kernel, schema, or enforcement
boundary as current capability.
