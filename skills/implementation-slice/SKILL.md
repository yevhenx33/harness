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

- repair the primary invariant at its owner and remove superseded exceptions
- delete or reuse before adding; keep the complete slice within admitted files,
  review LOC, runtime resource, latency, and operational budgets
- preserve explicit partial, stale, and unavailable states
- do not deploy, restart, migrate, or write production data without explicit
  authorization
- define success at the affected consumer, including later updates and
  unavailable states where relevant; preserve exact requested UI copy and layout
- use existing checks when sufficient; add regression tests for uncovered
  material behavior, not assertions that mirror cosmetic source changes
- name growing dimensions and verify the chosen time, space, I/O, fanout, retry,
  copy, and queue bounds; use `n`, `2n`, and `4n` scaling where material
- hold dependent actions if authority, an oracle, a required budget, recovery,
  or a material interface decision is missing; continue bounded investigation

Verify focused behavior first, then required broader checks in proportion to
risk. Inspect the affected render for visual edits. Repeat checks only after a
new change, failure, unresolved concern, or for an explicit observation window.
Continue through authorized delivery, retaining remaining gates across follow-up
messages. For release work, distinguish source, build, activation, and consumer
evidence. Keep the receipt internally and report the result, completion state,
and material limitations at the detail requested by the user.
