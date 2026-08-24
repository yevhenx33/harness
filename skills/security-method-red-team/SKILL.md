---
name: security-method-red-team
description: Use explicitly for attacker-led discovery of feasible abuse chains across a bounded system when the weakness is not yet known. Do not use for an unauthorized live target or a single already-defined source-to-sink path.
---

# Red-Team Audit

## Selection boundary

Best fit: a bounded system, attacker objective, and allowed surface are known, but the exploitable weakness or multi-step abuse chain is not.

Reject this method for routine broad assurance without an attacker objective, a single candidate path that attack-path analysis can decide, or a bounded PoC whose flag is the oracle. Never use it against an external or production target without explicit authorization.

## Inputs and authority

Resolve the exact target/revision, attacker objective and capabilities, allowed identities and interfaces, rules of engagement, prohibited effects, time budget, detection expectations, evidence policy, stop signal, and recovery owner. Default to read-only analysis of local artifacts.

Expected cost: high; the objective, rules, time, and tested chains must remain explicitly bounded.

## Workflow

1. Translate the attacker objective into observable success and explicit forbidden effects.
2. Map reachable entry points, trust transitions, identities, data, controls, and recovery paths.
3. Form the smallest plausible abuse chains and rank them by prerequisite cost and consequence.
4. Test the weakest uncertain link with static evidence first and a safe local check only when authorized.
5. Report supported chains, broken chains, detection gaps, and the next method needed to close one named evidence gap.

Read [references/l2-playbook.md](references/l2-playbook.md) during the exercise. Read [references/l3-lineage.md](references/l3-lineage.md) only for adversary-emulation lineage, comparison, or limitations. Format results with `../security-review-router/references/review-contract.md`.

## Stop conditions

Stop at the first out-of-scope identity, system, persistence effect, sensitive-data access, lateral movement, or unavailable recovery control. A creative scenario without a proven chain is a hypothesis, not a finding.
