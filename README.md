# harness

Sovereign agent operating policy, verification, and recursive learning harness.

This repository currently provides a governance foundation for high-correctness,
resource-aware agent work. It defines how intent is admitted, bounded work is
executed, evidence is verified, outcomes are classified, and verified learning is
retrieved and revalidated. It does not yet implement a signed-intent protocol,
capability enforcement, a CLI, a receipt wire schema, or a sovereign runtime
kernel.

## Policy

[`AGENTS.md`](AGENTS.md) is the authoritative current operating policy. It owns
authority gates, task contracts, invariants, computational and operational
budgets, verification, outcome receipts, and the recursive learning boundary.

[`agents/README.md`](agents/README.md) indexes immutable accepted policy
snapshots and their SHA-256 hashes. The root policy must be byte-identical to the
single snapshot marked Current. Historical snapshots remain unchanged; a policy
correction creates the next version.

## Skills

Skills provide bounded workflows under the policy's authority and evidence
rules:

- [`read`](skills/read/SKILL.md): read-only investigation and reporting
- [`map`](skills/map/SKILL.md): subsystem and dependency mapping
- [`architecture`](skills/architecture/SKILL.md): bounded architecture design
- [`implementation-slice`](skills/implementation-slice/SKILL.md): focused implementation and verification
- [`github`](skills/github/SKILL.md): GitHub orientation and publication boundaries
- [`security-review-router`](skills/security-review-router/SKILL.md): select one of twelve independently invocable security review methods and stop

The security review pack keeps routing, method procedure, operational playbook,
and historical lineage in separate layers. Its typed method graph is the only
routing authority. L1 methods disable implicit invocation, so selecting a method
does not start a review. The evidence corpus used to author the pack remains
outside this repository and is not loaded during normal routing or review work.

The current milestone is deliberately policy-and-instructions-only. Future
enforceable runtime components will be documented as target architecture until
they exist and have direct operational evidence.

## Architecture and integrity

[`docs/sovereign-runtime.md`](docs/sovereign-runtime.md) defines the future
sovereign runtime boundaries. Every component not present in this repository is
explicitly marked as target architecture.

Validate policy snapshots, hashes, sequencing, root equality, and local Markdown
links with only the Python standard library:

```sh
python3 scripts/verify_policy.py
```

Validate the security skill packages, typed graph, and deterministic routing
fixtures with the pinned development dependency:

```sh
python3 -m pip install -r requirements-dev.txt
python3 scripts/verify_skills.py
```

## License

Licensed under the [Apache License 2.0](LICENSE).
