---
name: atomic-inversion-brutalism
description: Redesign a coupled system, mechanism, product, workflow, or strategy by freezing the desired outcome, atomically mapping the current structure, inverting its governing causal mechanism, and expressing the smallest direct verifiable form. Use when the user explicitly invokes Atomic Inversion Brutalism or asks to invert control, ownership, incentives, or representation instead of tuning a downstream symptom.
---

# Atomic Inversion Brutalism

Work backward from the required outcome. Map the current mechanism into precise
atoms, identify the constraint that governs the result, invert that mechanism,
and retain only the smallest structure that preserves the complete objective.

Brutalism means directness, not carelessness. Keep invariants, validation,
failure states, recovery, and evidence explicit.

## Boundary

- Remain read-only unless the user authorizes a change.
- Preserve the user's outcome, authority, budget, and non-goals.
- Use the method when patches, blended concepts, indirect control, or local
  tuning obscure the mechanism that owns the result.
- Stop at a direct local fix when one owner already controls the complete
  problem.
- Reject inversion performed only for novelty; it must alter a causal mechanism.

## Freeze the target

State before redesign:

- the observable desired outcome and primary invariant;
- the complete success oracle and what would falsify it;
- coupled constraints that cannot regress;
- current owner, consumers, failure boundary, and exact recovery action.

Do not redefine success after discovering an attractive inversion.

## Build the atomic map

Give every material atom one meaning:

- actors, owners, and decision rights;
- authoritative facts, state, and representations;
- inputs, outputs, value or information flows, and incentives;
- constraints, resources, interfaces, transitions, and ordering;
- consumers, failure modes, uncertainty, and recovery.

For each atom, identify its owner, producer, consumer, relevant edges, and
failure boundary. Separate independent axes such as time, risk, price,
liquidity, ownership, freshness, availability, proof, policy, and settlement.
An inventory without ownership and causal edges is not an atomic map.

## Locate the governing mechanism

Trace the visible symptom to the narrowest authoritative mechanism that creates
it. Distinguish the binding constraint from a busy component or downstream
proxy. Where work repeats, include frequency, fanout, retries, rebuilds, copies,
coordination, and peak live state rather than measuring one execution in
isolation.

State the causal chain and the evidence that could disprove it before proposing
an inversion.

## Invert causality

Generate only inversions that could change the governing mechanism, such as:

```text
push -> pull
implicit -> explicit
shared ownership -> one commit owner
downstream cleanup -> upstream prevention
optimistic commit -> proposal plus authoritative verification
periodic full rebuild -> immutable facts plus bounded projection
parameter tuning -> representation change
forward accumulation -> backward design from the required outcome
```

For each candidate, predict the changed owner, transition, cost, failure mode,
and observable effect. Reject a candidate that moves the bottleneck, duplicates
truth, hides uncertainty, adds an indefinite parallel path, or weakens the
oracle.

## Express the brutalist form

Choose the smallest candidate that changes causality while preserving every
binding constraint. Prefer concrete state, one authoritative write path,
visible control flow, plain names, explicit costs, and bounded failure over
ornamental indirection.

Add an abstraction only for two current consumers or one required external
boundary. If a parallel replacement is necessary, define parity, cutover,
rollback, and the removal condition for the superseded path. Delete the old
path only after the new invariant is proven and deletion is authorized.

## Falsify the whole result

Verify the frozen outcome at the authoritative and consumer boundaries. Cover
normal operation and each material unavailable, partial, stale, duplicate,
ordering, cancellation, timeout, and recovery state that applies.

A component improvement is invalid if another binding resource, invariant, or
consumer outcome regresses. Classify unsupported or noisy evidence as
inconclusive rather than success.

## Required output

Return the smallest useful form of:

1. frozen outcome, invariant, constraints, and oracle;
2. atomic ownership and flow map;
3. governing mechanism and supporting or falsifying evidence;
4. current direction and chosen inversion;
5. smallest direct structural form and rejected alternatives;
6. material failure states, recovery, and verification result;
7. smallest independently verifiable next action.

Use a diagram or table only when it makes ownership, flows, or alternatives
materially easier to inspect.
