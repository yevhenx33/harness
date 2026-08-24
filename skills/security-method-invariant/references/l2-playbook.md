# L2 Playbook: Invariant Review

## Tools

- Use `rg`, call graphs, schema and migration inspection, transaction boundaries, and existing state-machine or property tests.
- Use existing property-based testing, model checking, symbolic execution, or database queries only when they directly exercise the named invariant.
- Treat generated counterexamples as candidates until mapped back to the authoritative implementation and environment.

## Practices

Write the predicate before evaluating code. Keep a transition table containing actor, pre-state, input, guard, write owner, post-state, publication, failure, retry, and recovery.

## Protocol

1. Pin target, owner, representation, and invariant.
2. Enumerate initialization, normal mutation, exceptional mutation, migration, retry, reconciliation, and recovery transitions.
3. Check the predicate before and after each transition and interleaving that the concurrency model permits.
4. Trace derived views back to authoritative state.
5. Construct the shortest counterexample or state why the checked transition preserves the property.
6. Report modeled coverage and every excluded transition explicitly.

## Evidence and falsification

A violation needs a reachable initial state, allowed input or actor behavior, transition sequence, and post-state that falsifies the predicate. Falsify a candidate with an unreachable pre-state, effective guard, atomicity guarantee, impossible ordering, or authoritative readback that preserves the property.

## Dynamic boundary and failure behavior

Use only local fixtures, existing models, or explicitly authorized disposable environments. Never probe financial, authorization, or production state destructively. Stop if the state owner or reset cannot be verified.
