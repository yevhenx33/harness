# L2 Playbook: Threat Model

## Tools

- Read architecture, route, identity, deployment, schema, secret, and network configuration.
- Use `rg` and call/data-flow tracing to verify entry points and trust transitions.
- Use STRIDE, misuse cases, attack trees, privacy frameworks, or protocol-specific taxonomies only as prompts; none owns the system facts.

## Practices

Keep a table of `asset`, `actor`, `entry point`, `trust boundary`, `security property`, `threat scenario`, `assumption`, and `evidence`. Prefer concrete attacker stories over category counts. Distinguish deployment fact, code fact, assumption, and open question.

## Protocol

1. Freeze repository and deployment identity.
2. Map the smallest end-to-end system slice that owns the requested outcome.
3. Enumerate assets and attacker positions.
4. Mark every crossing where identity, validation, privilege, or integrity changes.
5. Form threats as attacker capability plus action plus violated property plus consequence.
6. Verify the top scenarios against current code/configuration.
7. Hand off to architecture, red-team, or attack-path review only when the missing evidence matches that method.

## Evidence and falsification

A threat scenario is supported when an attacker position, reachable boundary, violated property, and impact are all evidenced. Falsify by showing the attacker lacks the prerequisite, the boundary enforces the property, or the asset/consequence is absent in the deployed system.

## Dynamic boundary and failure behavior

Threat modeling is read-only. Do not probe external services. If production topology is unavailable, label the model code-derived and enumerate the deployment assumptions that remain unverified.
