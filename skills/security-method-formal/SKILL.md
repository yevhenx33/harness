---
name: security-method-formal
description: Use explicitly for high-assurance analysis of a bounded algorithm, protocol, or state property with an explicit model and proof obligation. Do not use when the property, model boundary, or implementation correspondence is undefined.
---

# Formal-Methods Review

## Selection boundary

Best fit: the decisive security question can be expressed as a bounded mathematical property over a model whose assumptions and correspondence to the implementation can be reviewed.

Reject this method for broad unknown-unknown discovery, rapidly changing unspecified designs, or claims whose dominant risk lies outside the model. Use invariant review first when the property or transition set is still being formed.

## Inputs and authority

Resolve the exact target/revision, property, model, initial states, transitions, environment and attacker assumptions, abstraction boundary, refinement or correspondence argument, trusted computing base, solver/tool version, resource budget, and expected counterexample or proof artifact. New tooling or material compute requires explicit authority.

Expected cost: high; model scope, solver resources, and correspondence review are bounded before reproduction.

## Workflow

1. State the proof obligation and the exact claim it would support.
2. Audit definitions, assumptions, abstractions, trusted axioms, and excluded behavior.
3. Map each modeled state and transition to the authoritative implementation or specification.
4. Reproduce the proof or counterexample with the pinned toolchain when authorized.
5. Challenge vacuity, inconsistent assumptions, incomplete state spaces, and model/implementation drift.

Read [references/l2-playbook.md](references/l2-playbook.md) during analysis. Read [references/l3-lineage.md](references/l3-lineage.md) only for provenance, technique comparison, or limitations. Format results with `../security-review-router/references/review-contract.md`.

## Stop conditions

Stop when the property is ambiguous, assumptions make it vacuously true, the model lacks correspondence to the reviewed revision, or resource bounds prevent reproducibility. Never extend a proof claim to unmodeled code, deployment, operations, or human behavior.
