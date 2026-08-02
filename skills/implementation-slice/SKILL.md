---
name: implementation-slice
description: Implement one bounded subsystem change with focused verification.
---

# implementation-slice

Use the smallest useful form of:

map -> architecture checkpoint when needed -> implementation -> verification

Before editing, state the owned directory or subsystem, allowed files,
non-goals, acceptance criteria, and whether deployment is authorized. Inspect
the dirty worktree, relevant tests, existing patterns, and runtime/config
assumptions. Preserve unrelated changes and keep the edit set additive and
reviewable.

- repair the owning invariant or contract; do not add speculative fallbacks
- preserve explicit partial, stale, and unavailable states
- do not deploy, restart, migrate, or write production data without explicit
  authorization
- add the smallest tests for success and material failure/boundary paths
- stop and produce a map or contract if the slice lacks a required budget or
  public interface

Verify focused behavior first, then broader checks in proportion to risk. Report
files changed, checks and durations, coverage availability, performance impact,
rollback/deployment status, and anything that remains unverified.
