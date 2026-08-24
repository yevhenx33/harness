# L2 Playbook: Architecture Review

## Tools

- Inspect architecture decisions, service manifests, routes, schemas, identity policy, secrets, network policy, storage, queues, and recovery procedures.
- Use code search to verify which component actually validates, authorizes, persists, publishes, and recovers each fact.
- Use a minimal data/control-flow diagram only when it makes three or more boundaries clearer.

## Practices

Name one owner and one authoritative representation for each security-critical fact. Keep shared code separate from shared mutable authority. Treat fallback, cache, retry, migration, and administrative paths as part of the architecture.

## Protocol

1. Freeze system, revision, workload, and deployment assumptions.
2. State the primary invariant and its responsible owner.
3. Trace every producer, transition, store, interface, consumer, and recovery path.
4. Challenge trust crossings, privilege placement, failure containment, and observability.
5. Compare current design with the simplest design that preserves the property.
6. Report design findings separately from code-level candidates.

## Evidence and falsification

Support a design finding with a boundary trace and a concrete failure scenario. Falsify it with an authoritative enforcement point, independent readback, contained failure behavior, or proven non-reachability.

## Dynamic boundary and failure behavior

Do not deploy, migrate, restart, or reconfigure systems during architecture review. When live topology differs from configuration, label the discrepancy and stop short of a production claim.
