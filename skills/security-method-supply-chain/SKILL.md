---
name: security-method-supply-chain
description: Use explicitly to review dependencies, CI, builds, releases, provenance, signing, and artifact distribution from source to consumer. Do not use when the decisive question is only an application-code defect.
---

# Supply-Chain Review

## Selection boundary

Best fit: security depends on what enters the build, who can change it, how artifacts are produced, and how consumers verify provenance and integrity.

Reject this method when the target is only a line-level application change with no dependency, CI, build, packaging, signing, or release consequence. Use risk audit for unknown broad exposure and security diff for a bounded change.

## Inputs and authority

Resolve the exact source revision, repositories and dependencies, lockfiles, registries, identities, CI workflows and runners, build inputs, secrets, artifact stores, provenance and signing policy, release channels, consumer verification, rollback, and recovery owner. Default to read-only inspection; do not update dependencies, run releases, or rotate credentials.

Expected cost: medium; trace representative artifact paths and every privileged mutation boundary rather than executing the supply chain.

## Workflow

1. Pin the source revision and identify the authoritative source-to-artifact path.
2. Inventory mutable inputs, dependency resolution, privileged identities, build isolation, and publication rights.
3. Trace one artifact from source and lockfile through CI, build, provenance, signing, registry, and consumer verification.
4. Test whether substitution, workflow tampering, compromised dependency, secret exposure, or rollback can cross an unverified boundary.
5. Report supported attack paths, missing attestations, unverifiable states, and the narrowest control owner.

Read [references/l2-playbook.md](references/l2-playbook.md) during review. Read [references/l3-lineage.md](references/l3-lineage.md) only for SCRM/provenance lineage, comparison, or limitations. Format results with `../security-review-router/references/review-contract.md`.

## Stop conditions

Stop when the artifact, source revision, build identity, or publication path cannot be resolved. Never treat an SBOM, signature, passing CI job, or provenance file as assurance without checking who created it and whether the consumer verifies it.
