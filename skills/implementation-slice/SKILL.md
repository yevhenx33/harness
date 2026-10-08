---
name: implementation-slice
description: Implement one bounded subsystem change with focused verification.
---

# implementation-slice

Use the smallest useful form of:

map -> architecture checkpoint when needed -> implementation -> verification

Before editing, recover the admitted contract and unfinished requirements, or
freeze the smallest useful contract under the root policy. For routine work this
may be one internal sentence. Inspect changed or missing context: Git state,
applicable instructions, owner, consumers, relevant checks, existing patterns,
and runtime/config assumptions. Preserve unrelated changes; reuse valid prior
orientation, authorization, and admitted budget exceptions.

Before a material handoff, preserve the objective, granted authority, latest
user correction, owned changes, verified criteria, unfinished checks, evidence
references, and next executable action. Use the existing task or PR. Add a
task-specific file only when authorized implementation needs persistent state.
A handoff can preserve incomplete work; it does not authorize cleanup, commit,
rollback, or activation. On resumption, recheck changed facts and evidence
validity before continuing; reuse valid setup and prior checks.
When a required check is unavailable, name the next authorized independent
action if one exists. Keep dependent acceptance pending.

- repair the primary invariant at its owner and remove superseded exceptions
- delete or reuse before adding; keep the complete slice within admitted files,
  review LOC, runtime resource, latency, and operational budgets
- preserve explicit partial, stale, and unavailable states
- use Rust for backend services and operational logic; another implementation
  language requires explicit user approval, while existing tools may be invoked
- do not deploy, restart, migrate, or write production data without explicit
  authorization
- for a repository with a production runtime manifest, keep implementation in a
  task worktree, publish through a pull request, activate only the merged
  artifact, and preserve the prior artifact plus durable state for recovery
- scope release backups and restore checks to the changed component and its
  required shared records, checkpoint/WAL, configuration, and prior artifact;
  backing up the whole shared database service or VPS requires an affected wider
  scope and explicit user approval, not an older runbook or task plan
- define success at the affected consumer, including later updates and
  unavailable states where relevant; preserve exact requested UI copy and layout
- use existing checks when sufficient; add regression tests for uncovered
  material behavior, not assertions that mirror cosmetic source changes
- name growing dimensions and verify the chosen time, space, I/O, fanout, retry,
  copy, and queue bounds; use `n`, `2n`, and `4n` scaling where material
- hold dependent actions if authority, an oracle, a required budget, recovery,
  or a material interface decision is missing; continue bounded investigation

Put mechanical classification, validation, and commit decisions in
deterministic code. Bind commits to the relevant input, policy, artifact, and
owner identities. Make retries idempotent and bounded; expired or cancelled
work must not commit later. Label temporary mitigations with owner, risk,
removal condition, and recovery action.

Bind verification evidence to the criterion, checked input or artifact, and
relevant environment. Record the observation time when facts can change.
Preserve an earlier pass as history when its applicability changes; identify
the affected criterion as needing fresh evidence. For a new or uncertain gate,
check a representative material failure too. An exit code must reflect the
assertion result; a successful process can still print a failed predicate.

At touched external boundaries, handle relevant unavailable, timeout,
malformed, stale, partial, duplicate, ordering, cancellation, and retry states.
With existing coverage tooling, target 90% changed lines, 80% changed branches,
and all changed critical branches involving security, money, migration,
concurrency, destruction, or data loss. Otherwise map behavior to checks; do
not install coverage tooling merely to produce a metric. For performance work,
freeze the workload, baseline, artifact, and environment; measure at least ten
post-warmup runs, use p95 only with twenty samples, and inspect `n`, `2n`, and
`4n` scaling where work grows.

Verify focused behavior first, then required broader checks in proportion to
risk. Inspect the affected render for visual edits. Repeat checks only after a
new change, failure, unresolved concern, or for an explicit observation window.
Continue through authorized delivery, retaining remaining gates across follow-up
messages. For release work, distinguish source, build, activation, and consumer
evidence. Verify that production does not reference the task worktree and that
the active immutable artifact identifies merged source. Keep the receipt
internally and report the result, completion state, and material limitations at
the detail requested by the user.
