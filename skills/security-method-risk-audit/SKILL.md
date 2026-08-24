---
name: security-method-risk-audit
description: Use explicitly for a broad risk-prioritized security review of a repository or component when the attack surface is not yet narrowed. Do not use for a specific diff, known bug variant, or proof-oriented exploit task.
---

# Risk-Based Security Audit

Use this method after `$security-review-router` recommends it or when the user explicitly invokes it.

## Selection boundary

Best fit: a repository or component needs broad review, exposure is uncertain, and review effort must follow plausible impact rather than equal file coverage.

Reject this method when the target is only a diff, a validated bug class, a bounded exploit, or a single source-to-sink question. Use the narrower method instead.

## Inputs and authority

Resolve the exact target and Git revision, security policy, deployed surfaces, trust boundaries, high-value assets, untrusted inputs, and review budget. Review is read-only. Do not install tools, mutate code, exercise external targets, or run disruptive tests without explicit authority.

Expected cost: medium; the ranked budget bounds breadth, and deeper proof is delegated only by explicit follow-up.

## Workflow

1. Pin the target revision and map entry points, assets, privileged operations, external boundaries, and recovery-critical state.
2. Rank review slices by plausible impact, reachability, exposure, and uncertainty.
3. Trace the highest-ranked slices through owner, transition, storage, and consumer boundaries.
4. Treat every candidate as a hypothesis; seek a direct falsifier before reporting it.
5. Report covered and unreviewed risk areas without implying repository-wide completeness.

Read [references/l2-playbook.md](references/l2-playbook.md) when running the review. Read [references/l3-lineage.md](references/l3-lineage.md) only for provenance, limitations, or comparison with another method.

Use the shared contract at `../security-review-router/references/review-contract.md`. Consult the router index for neighboring methods; do not execute a neighbor automatically.

## Stop conditions

Stop when the ranked budget is exhausted, the remaining areas are lower priority than the named threshold, or missing deployment/asset facts prevent responsible ranking. State the gap instead of converting unknown coverage into a clean result.
