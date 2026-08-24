---
name: security-method-architecture
description: Use explicitly to review whether a software design, service boundary, or cross-component architecture can preserve required security properties. Do not use for a line-level diff or standalone exploit proof.
---

# Security Architecture Review

## Selection boundary

Best fit: the security question is owned by component boundaries, identity, data placement, privilege separation, failure containment, or recovery design rather than one implementation defect.

Reject this method when assets and attackers are still unknown; run threat modeling first. Use invariant or formal review when the decisive question is a specific state property.

## Inputs and authority

Resolve the exact design/revision, components, owners, interfaces, identities, data stores, deployment boundaries, security properties, operational failures, and recovery owner. Review documents and current code/configuration; do not assume diagrams are authoritative.

Expected cost: medium; analysis crosses material boundaries but does not attempt line-by-line repository coverage.

## Workflow

1. Pin the architecture evidence and identify one primary security invariant.
2. Map input, owner, transition, storage, output, and recovery across every material boundary.
3. Test whether privilege, validation, isolation, confidentiality, integrity, availability, and audit ownership are placed at authoritative boundaries.
4. Look for split ownership, implicit trust, confused deputies, insecure fallback, and recovery paths that bypass normal controls.
5. Compare the smallest viable alternatives and state the tradeoff.

Read [references/l2-playbook.md](references/l2-playbook.md) during review. Read [references/l3-lineage.md](references/l3-lineage.md) only for provenance or limitations. Format results with `../security-review-router/references/review-contract.md`.

## Stop conditions

Stop when the authoritative owner or deployment boundary cannot be identified. Report the decision gap; do not compensate with speculative infrastructure or an invented diagram.
