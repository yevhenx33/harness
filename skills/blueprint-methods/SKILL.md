---
name: blueprint-methods
description: Reference and evolve the cross-industry blueprint library, including mining current and archived Codex tasks for verified reusable practices. Use for architecture, system mapping, root-cause analysis, structural fixes, protocol or business-mechanism design, reliability, performance optimization, first-principles work, or when the user mentions blueprints, task history, atomic mapping, mechanism inversion, code brutalism, structural solutions, repeated-region efficiency, or non-obvious transferable patterns.
---

# Blueprint Methods

Use `/home/ubuntu/blueprints` as the single source of truth. Do not copy the
library into this skill.

## Reference Workflow

1. Inspect `/home/ubuntu/blueprints/README.md`.
2. Search titles, triggers, problem shapes, principles, and anti-patterns with
   `rg`. Do not load the whole library.
3. Select at most three relevant blueprints. Prefer intentionally different
   source domains when contrast could expose a non-obvious solution.
4. Read every selected blueprint completely before applying it.
5. State in commentary which blueprints are being used and what decision they
   influence.
6. Transfer the mechanism, invariant, control direction, and failure boundary.
   Do not copy domain vocabulary or surface implementation.
7. Resolve conflicts against the active task's evidence, constraints, and
   verification oracle. A blueprint is a reasoning lens, not authority.

For routine local edits with an obvious implementation, skip blueprint loading.
Do not add context without a decision it can improve.

## Default Method

When no single domain blueprint dominates, use:

```text
atomic map
  → separate independent axes
  → identify the owning mechanism and governing constraint
  → invert control, ownership, or representation
  → express the smallest direct structural form
  → verify against the complete frozen objective
```

Pair this with `Repeated-Region Efficiency` when a cost recurs inside a loop.

## Capture and Update Loop

Update `/home/ubuntu/blueprints` only when the current user request explicitly
includes blueprint maintenance or the library is inside the named task scope.
A general implementation request does not authorize a second, unrelated write.

Capture a candidate only when it is:

- a concrete mechanism rather than advice or style
- verified by a completed task, a primary source, or both
- transferable beyond its original project
- materially distinct from the current library
- explicit about constraints, failure modes, and when not to use it

Prefer, in order:

1. add evidence or a transfer rule to an existing blueprint
2. clarify an existing mechanism or anti-pattern
3. create one new blueprint only when no current owner fits

For a new blueprint:

- use `/home/ubuntu/blueprints/templates/blueprint-template.md`
- include internal task IDs when task history is material evidence
- enrich technical claims with primary sources
- distinguish verified behavior, source claims, inference, and failed experiments
- add the document to `/home/ubuntu/blueprints/README.md`
- keep one blueprint per bounded implementation slice

Failed or regressing experiments may define an anti-pattern or boundary. Never
promote them as proven practice.

## Thread Harvest

At the end of each completed non-read-only task:

1. Extract any new mechanism, invariant, failure boundary, or verification
   method from the current task.
2. Compare the candidate with the README and likely owning blueprints.
3. When earlier evidence could strengthen, contradict, or generalize it, use
   `list_threads` and `read_thread` to inspect the smallest relevant set of
   current and archived Codex tasks. Paginate only when needed.
4. Treat titles, previews, summaries, and recollection as discovery evidence.
   Promote only claims supported by the underlying task evidence.
5. If the capture criteria are satisfied and the current request authorizes the
   write, update the owning blueprint before the final response. Otherwise
   report the candidate without changing the library.

Do not rescan all tasks after every task. Search by mechanism, failure mode,
subsystem, and verification method. Preserve task IDs when provenance matters,
but never copy private task content into the library.

When a blueprint materially changes the work, name it and link its path in the
final response.

## Mutation Boundaries

- In `$read`, `$map`, audit, check-only, or otherwise read-only work, do not
  update the library. Report the candidate and proposed owner instead.
- Do not interrupt the requested task merely to polish the library.
- Do not capture task-specific constants, credentials, transient production
  state, or private data.
- Preserve unrelated blueprint worktree changes and inspect git status first.
- Apply the repository LOC and approval gates to blueprint updates.
- Do not commit, push, or deploy the library unless explicitly requested.

## Verification

After an update:

- run `git diff --check`
- verify every README link resolves locally
- verify new full blueprints contain all 11 canonical sections
- report exact handwritten LOC and files
- report external-source or runtime verification honestly

Coverage and runtime benchmarks are not applicable to Markdown-only updates.
