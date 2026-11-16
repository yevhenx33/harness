---
name: rubik
description: Remove performance blockers by deriving the system's actual design dimensions, challenging assumptions behind its mechanisms, and iteratively combining structural rotations while preserving the outcome and hard invariants. Use for coupled performance or resource problems where the current mechanism or its assumptions may be redesigned; prefer a direct local fix when it satisfies the complete performance contract.
---

# Rubik

Treat the solution space as a cube whose dimensions come from the problem, not a
fixed catalog. A rotation changes a material design choice; combined rotations
can remove work that no single change can remove. The cube's rules are the
required outcome and hard invariants. Its current arrangement is not itself a
requirement.

The objective is to remove performance blockers and unnecessary work, not to
generate novelty or enumerate every combination. Question why the work exists
before accelerating it. Use inversion as one possible rotation, not a required
move. The number and meaning of dimensions can change as understanding improves.

## Freeze the outcome

Establish the observable result, requested performance objective, and complete
success and failure oracle, the workload and measurement boundary, owner and
consumers, authority, resource limits, and recovery. A working functional result
alone does not satisfy an unmet performance objective. Identify hard invariants
such as correctness, freshness,
durability, isolation, ordering, and availability only where they are required.
Separate these from adjustable preferences, existing design choices, and
unverified assumptions. Do not invent a target or relax an invariant to make a
candidate look faster.

Respect the requested scope and existing authority: analysis is read-only unless
implementation is authorized; production profiling, load, activation, and other
external mutations require their own authority. An attractive rotation does not
authorize changing a user-selected architecture or another service's boundary.

## Map the blocker and derive dimensions

Trace inputs through the owning mechanism to the consumer outcome, including
background work and failure and recovery paths. Use existing evidence first;
distinguish a measured bottleneck from a hypothesis and name evidence that would
disprove it. Where relevant account for frequency, input size, affected keys and
dependencies, fanout, copies, contention, retries, peak state, and total work.

For each material cost, ask what assumption requires the work, where it is
established, and whether it is a hard constraint or a challengeable choice.
Derive dimensions from decisions that can change the governing cost. Describe
each dimension's current position, plausible alternative positions, owner,
dependencies, and constraints. Examples, when relevant, include timing,
representation, ownership, data movement, work scope, coordination, and
verification. Do not add a dimension solely to fill a template.

Make interactions explicit: choices may be coupled, a move may require another
move, and not all combinations are valid or reversible. Treat the map as a
working model; add, merge, split, or remove dimensions when evidence changes it.

## Rotate to remove work

Challenge the causal assumptions behind the blocker:

1. Can the work or requirement be deleted without violating the outcome?
2. Can unchanged authoritative results be reused, or can work be limited to
   changed inputs and affected dependencies?
3. Can a different representation, owner, timing, scope, or interaction eliminate
   repeated scans, copies, coordination, waiting, or recovery work?
4. Only after structural opportunities are assessed, can remaining work be
   executed more efficiently?

Generate a small set of causally distinct single and combined rotations. Do not
force every dimension to reverse or search the entire combinatorial space. For
each plausible candidate, state the changed mechanism and assumptions, work
removed and added, dependencies and interactions, expected consumer effect,
resource costs, failure and recovery consequences, and falsifying evidence.
Include index maintenance, coordination, migration, and background work where
applicable. Work moved off a request path is not automatically work removed.

Compare the current arrangement and viable candidates across the complete
outcome, hard constraints, and relevant costs. Retain materially different viable
trade-offs when there is no evidence for a single winner. Reject arrangements
that weaken an invariant, duplicate authority, hide failures, exceed a budget,
or merely shift an unacceptable bottleneck. Mark unknown effects as hypotheses,
not measured improvements. Present the recommendation and deciding evidence
before implementation; respect the user's chosen architecture and boundaries.

For example, a per-request full scan might be rotated toward update-time
maintenance, keyed state, and selected-range reads. Together these may remove
repeated full scans; update fanout, index costs, freshness, restart, and recovery
still need evidence. This is an illustration, not a default architecture.

## Test, learn, and rotate again

Use bounded evidence appropriate to the decision and authority. Compare
equivalent workloads and complete valid results at the requested outcome
boundary; include correctness, resource use, failure, and recovery. Attribute
component costs without presenting them as end-to-end results. Preserve normal
operating telemetry and account for diagnostic and background overhead.

After a valid change or a new finding, identify the remaining governing
constraint, revisit affected assumptions and interactions, and choose the next
rotation only if it is justified. Preserve prior evidence when its inputs and
conditions remain valid. A valid unfavorable experiment is a result, not a reason
to retry it as if it had not happened. When evidence is insufficient to choose,
investigate the deciding uncertainty within the existing scope, authority, and
budget; continue independent authorized work where possible. Stop when the
performance objective and preserved outcome are verified, no viable candidate is
justified within the constraints, or the necessary evidence cannot be obtained
within the available authority or budget. Report the exact remaining blocker and
next evidence or decision needed.

## Output

Keep the result proportionate to the task. Show the frozen outcome and hard
invariants; current blocker and evidence; derived dimensions, assumptions, and
interactions; candidate rotations and whole-system trade-offs; the selected next
action and falsifier; and measured results or explicit uncertainty. Distinguish
proposal, implementation, verified improvement, no-gain, invalid, blocked, and
inconclusive results. Do not claim performance improvement from a design alone.
