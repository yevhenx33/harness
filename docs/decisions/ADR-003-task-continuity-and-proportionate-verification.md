# ADR-003: Preserve task continuity and scale verification to the outcome

- Decision status: Accepted
- Evidence state: Designed
- Date: 2026-09-05
- Owner: root agent operating policy
- Scope: task admission, continuation, verification, and read/map/implementation workflows
- Supersedes: none; refines v014 while preserving [ADR-002](ADR-002-explicit-tiered-reasoning-skill.md)
- Superseded by: none

## Context and governing constraint

A read-only audit of the user's latest 100 Codex tasks identified repeated
orientation, lost continuation requirements, mode-only stops after authorization,
and verification that did not reach the requested consumer behavior. Small visual
iterations also incurred repeated workflow reads and full builds. These are
workflow failure patterns, not measurements of a new model's performance.

The Astra migration prompted this revision. The policy remains model-neutral:
instructions should preserve intent and direct verification while allowing the
model to resolve routine uncertainty. Explicit reasoning-tier invocation remains
unchanged. The repository provides instructions, not an enforced runtime.

## Primary invariant and consumers

An admitted task retains its scope, authority, user corrections, and unfinished
acceptance gates until completion or explicit revision. Continuing a task may
neither widen its authority nor silently weaken its required result.

Root policy owns this invariant. The read, map, and implementation-slice skills,
executing agent, and user consume it. Existing task context or an appropriate
task artifact carries state; no new persistent service or authority store is added.

## Decision and consequences

Release v015 with these changes:

- Preserve authorization and admitted budget exceptions within their agreed
  scope. Ask only when a material decision, risk, scope, or authority changes.
  Planning can resolve a decision; mode availability alone cannot block an
  otherwise admitted action. Read-only restrictions remain binding.
- Retain compact state for substantial or interrupted tasks: objective, scope,
  artifact, completed evidence, remaining gates, next action, and recovery.
  Revalidate drift-prone facts on continuation and reuse valid orientation.
- Scale workflow overhead to the question, visual edit, investigation,
  production operation, or frontier experiment without mandatory routing tiers.
- Verify at the affected consumer, including subsequent updates and unavailable
  states where relevant. Keep source, build, activation, and consumer evidence
  distinct. Finish after required checks pass unless new evidence warrants more.
- Preserve numerical budgets, material regression coverage, production gates,
  and cleanup authority. Keep detailed receipts internally and honor requested
  explanation depth and research breadth.

This release changes eight files and includes the required full immutable policy
snapshot. The user-authorized version release admits that mechanical duplication
and its review-size exception; it does not relax budgets for later tasks.

## Rejected alternatives

- Mandatory tiers or mode transitions: recreate an admission ceremony without
  resolving the missing fact, decision, or authority.
- A new task database, scheduler, or capability runtime: exceeds the demonstrated
  need and adds ownership and recovery costs to an instruction-only change.
- Remove numerical budgets or consumer verification: reduces visible overhead
  by weakening the result the user requested.
- Require a full build or new test for every cosmetic edit: repeats work even
  when narrower evidence is sufficient and repository requirements permit it.
- Encode model effort presets in the root policy: model-specific tuning lacks a
  representative controlled comparison and would conflict with explicit routing.

## Dependencies and proof obligations

Repository checks must establish sequential history, hashes, root/snapshot
equality, valid links, skill packaging, and unchanged prior snapshots. Review the
candidate against historical failure scenarios: authorized work with no Plan
mode; an explicit read-only restriction; a status question during implementation;
resume with unfinished gates; a cosmetic edit with sufficient existing checks;
and backend success with a broken subsequent consumer update.

These checks establish release integrity and inspect the intended decisions;
they do not prove live model compliance or an Astra speedup. Before claiming an
operational gain, compare v014 and v015 on representative tasks with the same
model, effort, inputs, tools, environment, and success oracle. Include completion
quality, wall time, tool work, user interventions, and integration cost. Preserve
the frozen performance sampling requirements in the policy.

## Falsifiers, recovery, and revisit triggers

Revisit if continuation exceeds authority, a user restriction is ignored,
unfinished acceptance gates disappear, consumer defects escape the narrower
checks, or representative tasks show worse completion or total cost. Any such
failure requires a bounded correction or rollback, not another generic ceremony.

Recover through a new policy version restoring the v014 behavior and prior skill
content, with a superseding ADR. Do not rewrite v015 or accepted decision records.
Existing sessions can retain previously loaded instructions; merging the release
does not prove adoption in every session or separately installed skill copy.
