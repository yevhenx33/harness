# L2 Playbook: Risk-Based Audit

## Tools

- Use `rg`, repository manifests, routes, schemas, policies, and deployment configuration to map exposed and privileged surfaces.
- Use Git history only to clarify ownership or security-relevant intent; history is not proof of current behavior.
- Run existing focused tests, analyzers, and dependency checks only when they address a ranked hypothesis. Do not install a scanner merely to increase apparent coverage.

## Practices

Build a compact risk register with `surface`, `asset`, `attacker position`, `impact`, `reachability`, `uncertainty`, and `review status`. Rank consequential reachable paths above stylistic weakness counts. Trace one primary invariant from its owner through every material consumer.

Keep coverage explicit: reviewed, sampled, blocked, or not reviewed. A clean high-risk slice does not clear an unreviewed repository.

## Protocol

1. Freeze target, revision, policy, authority, and budget.
2. Map assets and attack surfaces before opening low-risk implementation details.
3. Rank slices; record why each top slice outranks the next.
4. Inspect the owner and follow data/control flow across boundaries.
5. Validate candidates with code reachability, an existing test, safe local reproduction, or an independent specification.
6. Produce findings, negative evidence, coverage gaps, and one optional next method.

## Evidence and falsification

A reportable finding needs an attacker-controlled condition, violated security expectation, reachable behavior, and bounded impact. Falsify by proving input exclusion, unreachable control flow, effective validation, non-applicable deployment assumptions, or an invariant that blocks impact.

## Dynamic boundary and failure behavior

Dynamic checks must remain local, authorized, bounded, and reversible. Stop on missing environment authority, ambiguous production coupling, unstable fixtures, or destructive setup. Report the blocked hypothesis and the lowest-risk next observation.
