# L2 Playbook: CTF / Exploit-First Review

## Tools

- Use the target's existing build, test, debugger, disassembler, protocol client, and reset mechanisms.
- Use `rg` and focused static tracing to reduce the hypothesis space before dynamic work.
- Do not install offensive tooling, connect to external infrastructure, or broaden the target without authority.

## Practices

Keep the proof minimal and deterministic: starting state, exact input, observed transition, oracle output, and reset. Separate exploit reliability from severity in a real deployment.

## Protocol

1. Record revision, environment, starting state, and exact oracle.
2. Identify the property and guard protecting the oracle.
3. Trace a candidate bypass from controlled input to the protected state.
4. Implement the smallest harmless proof.
5. Repeat from a clean reset and record negative controls.
6. Minimize the proof and state every deployment assumption.

## Evidence and falsification

The oracle must be directly observable and repeatable. Falsify the exploit with a clean control input, a fixed guard, an unreachable prerequisite, environmental non-equivalence, or failure to reproduce after reset.

## Dynamic boundary and failure behavior

Run only against the local challenge or explicitly authorized sandbox. No persistence, external callbacks, credential use, denial of service, destructive state, or access to unrelated data. Stop if isolation or reset fails.
