---
name: contract
description: Use when the user tags $contract or asks to define an API, schema, data, runtime, frontend/backend, protocol-line, or service contract before implementation. Use for producer/consumer boundaries, compatibility rules, freshness semantics, validation gates, and removal plans.
---

# Contract

Turn the outcome-first design into a concrete producer/consumer contract. Remain
read-only unless edits are authorized. Identify each service owner, owned state,
public interface, allowed dependency direction, and shared-contract write scope.

Define required fields, types, units, nullability, stable keys, and identity.
Specify event ordering, duplicate handling, idempotency, deletion, late events,
corrections, acknowledgement, and dependency invalidation where applicable.

Define freshness, open/closed windows, current/history/tail semantics, delta
processing, necessary conversion boundaries, retained-state lifetime, and full
recovery behavior. Preserve typed values inside compatible service boundaries.

Preserve compatibility through explicit versioning, migration, deprecation, and
removal rules. Do not assume additive-only growth or indefinite fallback support.
Cross-service changes remain within authorized scope and verify affected producer
and consumer contracts. Private implementation details are not public interfaces.

For each material boundary, identify the invariant, permitted writes, failure
state, direct oracle, and recovery action. Check interactions such as retry with
cancellation, publication with delayed updates, and invalidation after corrections.

Resolve discoverable ambiguity through bounded inspection. Carry architecture
stop/resume conditions into implementation. Report unresolved owners or semantics
and hold dependent work; use the existing task or a contract table as needed.
