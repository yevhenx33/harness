# L2 Playbook: Supply-Chain Review

## Tools

- Inspect manifests, lockfiles, vendored code, checksums, SBOMs, CI definitions, runner permissions, build scripts, release workflows, provenance, signatures, registries, and deployment verification.
- Use existing package-manager audit, SBOM, signature, and provenance verification commands only as evidence inputs.
- Do not install, upgrade, publish, sign, revoke, or rotate anything without explicit authority.

## Practices

Represent the chain as source revision, resolved dependencies, build definition, runner and identity, artifact digest, attestation, signer, registry, promotion, consumer verification, and rollback. Mark every mutable or unauthenticated edge.

## Protocol

1. Pin source, dependency graph, build definition, and expected artifact.
2. Identify every identity that can alter source, workflow, runner, dependency, artifact, metadata, or release channel.
3. Verify dependency pinning and trusted resolution boundaries.
4. Trace artifact digest and provenance through signing and distribution.
5. Confirm the consumer enforces the expected identity and policy.
6. Exercise rollback and compromise assumptions analytically; report every unverified edge.

## Evidence and falsification

A supported weakness needs a feasible actor or compromised component, mutable edge, missing or bypassable verification, and consequential consumer acceptance. Falsify it with immutable resolution, isolated build identity, independently verifiable provenance, policy-enforced signatures, or consumer rejection.

## Dynamic boundary and failure behavior

Do not execute untrusted build steps or packages on the host. Use existing sandboxed inspection only when authorized. Stop on secret material, untrusted code execution, registry mutation, publication, or production promotion.
