# ADR-005: Cooperative pull-request admission for production repositories

- Decision status: Accepted
- Evidence state: Designed
- Date: 2026-09-28
- Owner: Harness policy for agent behavior; each repository for its release oracle
- Scope: production source admission when repository hosting cannot enforce rules

## Context and governing constraint

Some private repositories cannot enable server-side branch protection on their
current hosting plan. Production still needs an auditable source boundary, and a
hosting limitation must not silently turn direct pushes or task worktrees into
the normal deployment path.

## Invariant and decision

For a repository declaring a production runtime manifest, every ordinary
production change begins in a task branch/worktree, passes the repository's
canonical check, and merges through a pull request before activation. Production
cannot depend on that task worktree. Harness owns this cooperative agent rule;
the repository owns artifact provenance, activation, verification, and rollback.

Direct `main` changes require explicit break-glass authority, the prior artifact,
an exact recovery action, and an immediate reconciliation pull request. This is
not described as technical enforcement: a user or tool outside Harness can still
bypass it while hosting protection is unavailable.

## Consequences and rejected alternatives

The rule preserves reviewable history without pretending Harness is a capability
kernel. It adds no deployment service, receipt database, or automatic rollback.

Rejected: reporting unenforced GitHub protection as active; blocking all work
until a paid hosting feature exists; direct-main-by-default; and production
activation from an unmerged task worktree.

## Proof, recovery, and revisit triggers

Repository integrity proves only that the rule is packaged. A production release
must separately prove PR merge, immutable artifact identity, canonical runtime
ownership, consumer behavior, and rollback. Recover by selecting the prior
artifact with unchanged durable state and reconciling source through a PR.

Revisit when server-side protection becomes available, the repository no longer
declares a production manifest, or measured use shows that the cooperative rule
does not preserve source/runtime correspondence.
