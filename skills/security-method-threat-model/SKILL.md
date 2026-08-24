---
name: security-method-threat-model
description: Use explicitly to identify assets, attacker positions, data flows, trust boundaries, and security assumptions before detailed review. Do not use when the task already supplies a complete threat model and asks for exploit validation.
---

# Threat-Model-Driven Review

## Selection boundary

Best fit: the review cannot yet answer who attacks, what matters, where trust changes, or which assumptions make an impact plausible.

Reject this method when a validated finding only needs reachability or exploit proof, or when the exact outcome is a diff assessment. Reuse a current threat model when its repository and revision still match.

## Inputs and authority

Resolve the product purpose, exact revision, deployment, users and operators, assets, identities, data flows, external systems, trust boundaries, attacker capabilities, and excluded threats. Treat supplied diagrams and documents as hypotheses to verify against code and configuration.

Expected cost: low; stop at a decision-ready model rather than scanning implementation for findings.

## Workflow

1. Pin the target and describe the system in one bounded paragraph.
2. Inventory assets, actors, entry points, trust boundaries, privileged actions, and failure consequences.
3. Trace material data/control flows across boundaries.
4. State security properties and assumptions in falsifiable language.
5. Rank threat scenarios by capability, reachability, and impact.
6. Recommend one downstream method only when a scenario needs deeper evidence.

Read [references/l2-playbook.md](references/l2-playbook.md) to create the model. Load [references/l3-lineage.md](references/l3-lineage.md) only for framework choice or limitations. Format results with `../security-review-router/references/review-contract.md`.

## Stop conditions

Stop and label uncertainty when deployment, identity, asset, or boundary facts cannot be derived. Do not fill missing facts with generic STRIDE labels or portray an unverified diagram as current architecture.
