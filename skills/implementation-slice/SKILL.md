---
name: implementation-slice
description: Implement one bounded subsystem change with focused verification.
---

# implementation-slice

Use the smallest useful form of:

map -> architecture checkpoint when needed -> implementation -> verification

Before editing, freeze Outcome, Scope, Owner, Invariants, Oracle, Authority,
Budget, Failures, Non-goals, and Recovery. Inspect the dirty worktree, applicable
instructions, authoritative implementation, consumers, relevant tests, existing
patterns, and runtime/config assumptions. Preserve unrelated changes.

- repair the primary invariant at its owner and remove superseded exceptions
- delete or reuse before adding; keep the complete slice within admitted files,
  review LOC, runtime resource, latency, and operational budgets
- preserve explicit partial, stale, and unavailable states
- do not deploy, restart, migrate, or write production data without explicit
  authorization
- add the smallest tests for success and material failure/boundary paths
- name growing dimensions and verify the chosen time, space, I/O, fanout, retry,
  copy, and queue bounds; use `n`, `2n`, and `4n` scaling where material
- stop at admission if the slice lacks authority, an oracle, a required budget,
  recovery, or a decided public interface

Verify focused behavior first, then broader checks in proportion to risk. Report
an execution receipt with intent, owning change and exact files, evidence mapped
to claims, gross LOC and resource or complexity cost, one outcome class, current
operational state, recovery, unresolved risk, and warranted learning projection.
