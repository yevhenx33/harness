---
name: security-method-variant-analysis
description: Use explicitly to find and validate sibling instances of a known vulnerability root cause across code or configuration. Do not use when the seed finding is unvalidated or reduced only to a keyword.
---

# Variant Analysis

## Selection boundary

Best fit: one vulnerability or security bug has a validated root cause, and the question is where the same causal structure appears elsewhere.

Reject this method when there is no validated seed, the requested search is merely lexical, or the goal is broad unknown-unknown discovery. Triage or attack-path analysis should establish the seed before expansion.

## Inputs and authority

Resolve the exact target/revision, seed finding, minimal root-cause predicate, attacker prerequisite, affected and fixed examples, search scope, languages/configurations, expected sinks, budget, and validation oracle. The default procedure is read-only.

Expected cost: medium; query expansion stops when the predicate or candidate volume can no longer be validated within budget.

## Workflow

1. Reduce the seed to a structural predicate: controlled source, missing or misplaced guard, transition, sink, and consequence.
2. Extract positive and negative examples that distinguish the cause from surface syntax.
3. Search from the narrowest structural representation outward.
4. Validate each candidate independently; do not inherit severity from the seed.
5. Refine the predicate until false positives and false negatives are explicit, then report covered and uncovered variants.

Read [references/l2-playbook.md](references/l2-playbook.md) during the search. Read [references/l3-lineage.md](references/l3-lineage.md) only for taxonomy, provenance, or limitations. Format results with `../security-review-router/references/review-contract.md`.

## Stop conditions

Stop if the root cause cannot be stated independently of one identifier, the search scope is unbounded, or validation cannot distinguish a sibling vulnerability from harmless similarity. Report the unresolved predicate instead of a candidate count.
