# L2 Playbook: Red-Team Audit

## Tools

- Use `rg`, route and identity maps, configuration inspection, and dependency or deployment manifests to enumerate the bounded surface.
- Use existing local tests, clients, debuggers, and observability only when they stay inside the rules of engagement.
- Treat scanners and ATT&CK mappings as hypothesis generators, not proof of a working chain.

## Practices

Keep an objective ledger: attacker prerequisite, action, expected transition, observed evidence, control encountered, consequence, and confidence. Prefer one short end-to-end chain over a list of disconnected weaknesses.

## Protocol

1. Pin target, objective, rules, stop signal, and recovery owner.
2. Enumerate the reachable surface from the admitted attacker position.
3. Generate abuse chains across identity, data, control, service, and recovery boundaries.
4. Challenge each chain at its least-supported edge.
5. Validate only the minimum harmless action needed to demonstrate the consequence.
6. Separate validated findings, plausible hypotheses, invalid paths, and untested coverage.

## Evidence and falsification

A finding needs an admitted attacker, reachable first action, supported intermediate edges, and demonstrated or independently entailed consequence. Falsify it with an effective guard, unreachable deployment state, unforgeable identity, unavailable prerequisite, or recovery control that prevents the claimed outcome.

## Dynamic boundary and failure behavior

No credential collection, persistence, destructive payload, production mutation, denial of service, external callback, or lateral movement is permitted unless each effect is explicitly authorized. Stop on unexpected data, impact, or scope expansion and preserve only the minimum evidence.
