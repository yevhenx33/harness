---
name: security-method-attack-path
description: Use explicitly to prove or falsify a security path from attacker-controlled source through guards and state transitions to a consequential sink. Do not use for broad discovery without a candidate path.
---

# Attack-Path Analysis

## Selection boundary

Best fit: a candidate weakness, exposed entry point, or threat scenario needs source-to-sink reachability and impact analysis.

Reject this method for broad unknown-unknown discovery or when no attacker position, source, or sink can be stated. Use threat modeling or risk audit first.

## Inputs and authority

Resolve target/revision, attacker capability, entry point, controlled data or action, guards, privilege transitions, state changes, sink, expected property, and claimed impact. Read-only tracing is default; dynamic reproduction requires explicit local authority.

Expected cost: medium; work follows one bounded path and stops at a broken or unobservable edge.

## Workflow

1. State the path as attacker prerequisite, source, transitions, sink, and consequence.
2. Trace every transformation, validator, authorization check, queue, store, and asynchronous boundary.
3. Record branch conditions and deployment assumptions that enable or break the path.
4. Seek the narrowest independent witness of reachability or non-reachability.
5. Calibrate impact to the demonstrated terminal state, not the weakness label.

Read [references/l2-playbook.md](references/l2-playbook.md) when tracing. Read [references/l3-lineage.md](references/l3-lineage.md) only for attack-graph/taint lineage or limitations. Format results with `../security-review-router/references/review-contract.md`.

## Stop conditions

Stop when a required edge is unobservable, attacker control is unproven, the sink is non-consequential, or a guard definitively breaks the chain. Report the last proven edge and missing evidence.
