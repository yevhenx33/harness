---
name: contract
description: Use when the user tags $contract or asks to define an API, schema, data, runtime, frontend/backend, protocol-line, or service contract before implementation. Use for producer/consumer boundaries, compatibility rules, freshness semantics, validation gates, and removal plans.
---

# Contract

Use this skill to turn a mapped idea into an implementation-ready contract. Keep it read-only unless the user explicitly asks for edits.

## Contract Workflow

1. Name the contract and owner:
   - producer
   - consumer
   - runtime/service boundary
   - directory or protocol owner
   - public interface, allowed dependency direction, and shared-state writer
2. Define the payload or state shape:
   - fields, types, units, nullability
   - keys and identity
   - ordering and dedupe rules
   - versioning and additive-only fields
3. Define semantics:
   - freshness and closed/open window rules
   - memory-first versus persisted source
   - current versus selected history versus tail
   - retry, replay, idempotency, and checkpoint behavior
4. Define compatibility:
   - old readers/new writers
   - new readers/old writers
   - fallback behavior
   - deprecation and removal criteria
5. Define validation:
   - unit tests
   - integration tests
   - parity checks
   - read-only probes
   - monitored experiment metrics
   - affected consumer and dependency interactions when a shared contract changes

For each touched module boundary, identify the owning invariant, permitted
writes, failure state, direct oracle, and recovery action. Independent module
work must use compatible contract versions and explicit ownership of shared
changes; internal implementation details must not become implicit dependencies.
Check affected interactions such as retries with cancellation, cache freshness,
and publication with delayed updates when they are material to the change.

## Required Questions

- Who is allowed to write this contract?
- Who is allowed to read it?
- What is the canonical source of truth?
- What is not allowed to depend on protocol internals?
- What makes a row/message/window sealed?
- What happens when data is missing, stale, partial, or duplicated?
- What is the smallest implementation slice that proves the contract?

## Output Shape

1. Scope and owner.
2. Producer/consumer map.
3. Contract table with fields, types, units, and semantics.
4. Freshness, sealing, replay, and failure rules.
5. Compatibility and migration notes.
6. Validation gates and next implementation slice.

Resolve discoverable ambiguity through bounded inspection. If a critical owner,
source of truth, or oracle remains unresolved, hold dependent implementation and
state the missing decision; continue independent authorized work. Carry agreed
architecture stop conditions into the implementation contract.
