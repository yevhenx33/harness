# L2 Playbook: Attack Path

## Tools

- Use `rg`, call-site navigation, route maps, schema queries, and configuration inspection for static path construction.
- Use language-aware taint or data-flow tools when available and already configured; treat results as candidate edges.
- Use focused logs, tests, or local PoCs only to witness disputed edges.

## Practices

Represent the path as ordered nodes: prerequisite, source, transformation, guard, privilege/state transition, sink, consequence. Distinguish data influence from control influence and potential flow from demonstrated reachability.

## Protocol

1. Pin target and attacker assumptions.
2. Name the source and consequential sink.
3. Traverse forward from source and backward from sink until the traces meet or a guard breaks them.
4. Check serialization, aliases, callbacks, queues, retries, caches, and cross-service identity.
5. Validate the weakest uncertain edge.
6. Report the complete chain, broken chain, or exact unresolved edge.

## Evidence and falsification

Every edge needs source evidence. A scanner path is not proof. Falsify with input normalization, unforgeable identity, authorization, type/state constraints, environmental exclusion, or a sink that cannot produce the claimed consequence.

## Dynamic boundary and failure behavior

Dynamic checks must use a local or explicitly authorized target and the smallest harmless payload. Stop before persistence, lateral movement, credential access, or external effects not required by the evidence claim.
