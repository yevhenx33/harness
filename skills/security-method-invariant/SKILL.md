---
name: security-method-invariant
description: Use explicitly to review whether authorization, accounting, protocol, or state-machine invariants hold across all relevant transitions. Do not use when the property or authoritative state owner cannot be stated.
---

# Invariant Review

## Selection boundary

Best fit: security depends on a precise property that must hold before and after every relevant transition, especially for authorization, accounting, protocols, concurrency, and state machines.

Reject this method when the assets and attackers are still unknown, the decisive issue is a broad architecture boundary, or no authoritative representation and transition set can be identified. Use threat modeling or architecture review first.

## Inputs and authority

Resolve the exact target/revision, authoritative state owner, invariant, initial states, allowed transitions, identities, inputs, concurrency and ordering model, failure states, consumers, and recovery action. Default to read-only code, schema, configuration, and test inspection.

Expected cost: medium; work scales with relevant transitions and admitted interleavings, not repository size.

## Workflow

1. Express one invariant as a falsifiable predicate over authoritative state.
2. Enumerate every transition that can create, change, publish, reconcile, or recover that state.
3. Check preconditions, authorization, atomicity, ordering, retries, cancellation, and partial failure for each transition.
4. Seek a counterexample using static traces and existing tests or models.
5. Report the minimal violating trace, proof boundary, unmodeled transitions, and next validation method if needed.

Read [references/l2-playbook.md](references/l2-playbook.md) during review. Read [references/l3-lineage.md](references/l3-lineage.md) only for state-machine lineage, comparison, or limitations. Format results with `../security-review-router/references/review-contract.md`.

## Stop conditions

Stop when ownership is split or unknown, the predicate depends on an undefined term, or a transition cannot be enumerated. Do not convert missing, stale, partial, or contradictory state into a passing invariant.
