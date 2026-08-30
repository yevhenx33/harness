---
name: structural-factorization
description: Build a structural factor model for a coupled system, mechanism, product, workflow, or research problem by separating its invariant core, independently varying factors, explicit interactions, composition rules, owners, and failure boundaries. Use when decomposition or factorization is the requested outcome, or when blended ownership or repeated whole-system work prevents a direct solution. Do not use for routine local edits, simple one-owner bugs, general architecture planning without a factorization question, or broad optimization that primarily requires hypothesis search.
---

# Structural Factorization

Find the smallest complete structure that preserves the requested outcome while
making ownership, variation, interaction, cost, and failure explicit.

Factorization here is not limited to mathematics or data processing. It means
separating what remains invariant from what varies, then defining exactly how
the factors compose into the whole.

## Boundary

- Remain read-only unless the user explicitly authorizes implementation.
- Preserve the user's requested outcome; do not replace it with a preferred
  mechanism.
- Use this skill only when the problem has meaningful coupling, repeated work,
  blended concepts, or ambiguous ownership.
- Stop when a direct local fix already owns the complete problem.

## Freeze the objective

Before redesign, state:

- the observable outcome and primary invariant;
- the evidence and oracle that can falsify success;
- the current owner, consumers, constraints, and failure boundary;
- the relevant budget and non-goals.

Do not redefine success after discovering an attractive decomposition.

## Build the atomic map

Decompose the current system into atoms with one meaning each:

- actors and owners;
- state and representations;
- inputs, outputs, and flows;
- constraints and resources;
- interfaces and consumers;
- transitions, ordering, and failure modes.

Separate independent axes such as time, risk, price, ownership, freshness,
availability, proof, policy, and settlement. An inventory is not a structural
map until every material atom has an owner, consumer, relationship, and failure
boundary.

## Derive the factor model

Express the candidate structure as:

```text
whole = invariant core + variable factors + interactions + composition rules
```

For every part, name:

| Part | Required question |
|---|---|
| Invariant core | What must remain true and canonical? |
| Factor | What changes independently, and who owns its value and version? |
| Interaction | Where do factors cease to be independent? |
| Composition | How are factors combined into the observable result? |
| Provenance | What evidence establishes each input without changing its meaning? |
| Frontier | What ordering or completeness boundary makes the result safe to consume? |
| Projection | Which derived views can be rebuilt or replaced without changing truth? |

Use only the parts that materially exist. Mark provenance, frontier, projection,
or a cost model not applicable with a reason; do not invent structure to complete
the template.

Do not call a decomposition factorized when hidden interaction terms, duplicated
ownership, or an unbounded reconciliation step still determine the result.

## Locate the governing mechanism

Trace a visible symptom to the narrowest authoritative mechanism that creates it.
For a greenfield problem, identify the narrowest boundary that must create the
intended outcome. For repeated work, multiply each cost by its execution
frequency and include fanout, retries, rebuilds, copies, coordination, and peak
live state.

Apply the governing repository reduction order first:

```text
question -> delete -> simplify -> fix the owner -> falsify and verify
-> disposition findings -> accelerate -> automate
```

Within the surviving structural design, delete repeated work, reuse authoritative
state, and fix ownership before precomputation, deferral, or local optimization.

An inversion is useful only when it changes causality, for example:

- full rebuild to bounded projection;
- implicit shared ownership to one explicit commit owner;
- downstream cleanup to upstream invalid-state prevention;
- blended state to invariant basis plus independently versioned factors;
- unordered completion to a monotonic committed frontier.

## Express the smallest direct structure

Choose one canonical representation and one authoritative transition path for
each fact. Keep composition and failure visible. Reject a new abstraction unless
it removes duplicated work, isolates a real boundary, or serves multiple current
consumers.

If a parallel replacement is required, define its parity oracle, cutover,
rollback, and removal condition for the superseded path.

## Preserve material decisions

When the factorization establishes or changes a material architecture,
ownership, interface, state, ordering, security, recovery, or freeze boundary,
produce or update the repository's architecture decision record. Read
[references/architecture-decision-record.md](references/architecture-decision-record.md)
for the schema and lifecycle.

Treat the ADR as the causal record of why the boundary exists. Keep decision
status separate from evidence state, name falsifying or revisit triggers, and
supersede an accepted record instead of rewriting its history. Do not create an
ADR for routine local choices or use one as a substitute for specifications,
proof, implementation, or runtime evidence.

## Falsify the complete result

Verify the coupled objective, not a favorable component metric. Cover the
material workload classes, including normal, dense or adversarial, boundary,
restart or recovery, shared-contention, and long-range behavior when applicable.

Check that:

- the factorization preserves all information needed by the invariant;
- interaction and ordering rules are complete;
- work scales with affected factors rather than hidden total state;
- proof or metadata changes do not masquerade as semantic changes;
- local savings do not move cost into another owner or failure boundary;
- uncertainty remains explicit and recovery is exact.

Classify the result as verified, no-gain, invalid, blocked, or inconclusive.

## Required output

Return the smallest useful form of:

1. frozen objective, invariant, and oracle;
2. atomic ownership and flow map;
3. invariant core, factors, interactions, composition, provenance, frontier,
   and projections;
4. governing constraint and any current repeated cost;
5. chosen structural mechanism and rejected alternatives;
6. target complexity where work grows, otherwise the dominant cost or resource
   model;
7. falsification, failure, and recovery plan;
8. ADR or ADR update for each material decision, otherwise why none is needed;
9. smallest independently verifiable next slice.

Use a diagram or table only when it materially clarifies relationships.
