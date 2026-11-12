---
name: scrutiny
description: Conduct a time-bounded audit with a user-supplied duration and objective. Invoke explicitly with $scrutiny for reviews of correctness, defensive security, reliability, performance, resource use, cleanup, design, or evidence.
---

# Scrutiny

Use the allotted time to answer a specific audit question. Trace the behavior
that matters, challenge the assumptions it depends on, and show what the
evidence supports at the deadline.

## Required inputs

- **Duration.** Recover a positive, finite duration from the request or an
  existing agreement. An unambiguous deadline also supplies a duration when a
  reliable clock is available.
- **Objective.** Recover what the audit should establish, challenge, measure, or
  improve. Express it as an observable outcome or deciding question. Preserve
  stated targets, required coverage, and any requirement to retain or reconsider
  the mechanism. For several objectives, respect the user's priorities and
  evaluate each against its own evidence.

Ask for whichever required input is missing or ambiguous before starting the
timed audit. Infer the target, scope and constraints from the task when clear.
Ask only when a material ambiguity remains after checking available context.

## Bound the work

Record the start and deadline from a reliable clock. Honor any agreed start,
deadline or prerequisite, such as completion of implementation and its required
checks. Otherwise start once the required inputs and target are clear. Include
audit setup, investigation and reporting in the window. Keep the same deadline
across checkpoints, corrections, context changes and recoverable setup failures
unless the user changes it.

Use a bare duration or “spend N” as a work window. When an initial pass finishes
early, pursue a distinct unanswered question, challenge an assumption, or
verify a consequential finding. “Up to N” or “within N” allows completion as
soon as the agreed coverage and evidence are sufficient. If productive,
authorized work is exhausted early, report the reason and actual elapsed time.
When the objective requires observing behavior over a stated interval, verify
coverage of that interval and account for it in the agreed budget. Flag any
required evidence the window cannot contain and continue the feasible work.
Check remaining time at changes of direction and before expensive calls. Use
bounded operations and reserve time for the final checks and report. If the
window includes repair, allow time to verify the changes.

Identify the revision, artifact, runtime, document or observation interval under
review and track any change to it. Carry the original outcome, acceptance
criteria and invariants forward from the feature and implementation. Identify
the owner, consumer, allowed actions and what evidence would decide success or
failure. For each material invariant, state what must hold, for which entity,
and under what conditions. Mark assumptions and distinguish them from agreed
requirements. Audit work is read-only by default. Follow the task's authority
for any changes, workloads, runtime actions or external disclosure.

For an improvement objective, state whether the agreed scope calls for
optimizing the current mechanism or comparing replacements. Use the objective
and constraints to make that choice. Surface a structural limit when evidence
shows it prevents the current mechanism from meeting the objective.

## Investigate and challenge

1. **Map the material paths.** Trace the outcome backward from the consumer to
   the authoritative producer, then follow relevant transitions forward.
   At each material link, identify its input, effect, guarantee, assumption,
   and the owner or dependency that establishes it. Split a link further when
   its constituent steps have different guarantees, owners or costs. For a
   design, compare the promised outcome with what the mechanism can guarantee.
   When there is a specification, trace requirements into the implementation
   and look for consequential behavior the specification leaves unaddressed.
   Choose questions and evidence from the relevant sections of the
   [perspective guide](references/perspectives.md).
2. **Try to falsify the important claims.** Select cases that distinguish the
   expected behavior from plausible alternatives. Vary identity, ordering,
   units, limits, history or dependency state where those can change the result.
   Look for a failing combination of individually reasonable assumptions.
3. **Challenge the evidence.** Trace the exact precondition, reachability,
   cause and consumer effect for each candidate. Seek evidence that would
   refute it. Check that a test reaches the intended state and would fail on
   a meaningful wrong result. Prefer an oracle derived independently of the
   implementation. Use existing evidence or authorized checks, and retain
   limits on what was exercised or measured. If a dependency or the target
   changes, refresh only the evidence affected by that change.
4. **Use the next pass to close a gap.** Search for a confirmed cause in related
   paths and check each candidate in its own context. Inspect an omitted
   lifecycle state or validate a different assumption when that could change the
   conclusion. Choose checks that fit the remaining time. Keep a compact record
   in the task of coverage, deciding evidence, refuted leads and remaining
   questions. Retain it across context changes.

## Close the audit

Reconcile the objective and material paths against the available evidence, then
challenge the conclusion with a concrete counterexample or omitted path. For
each consequential item, give the affected version and location, expected
behavior, what the evidence establishes, necessary conditions, consequence,
owner and deciding follow-up check. Distinguish confirmed defects, improvement
hypotheses and missing verification. Identify what was established by source
reasoning, tests, runtime observation, measurement or proof under stated
assumptions. Give the elapsed time, any material interruptions and the scope of
the conclusion. Mark incomplete coverage and what would close it if the deadline
arrives first.

Report audit coverage and finding disposition separately. A read-only audit can
deliver its report with open findings. When remediation is authorized, correct
material defects at the owning boundary within the established scope and budget.
Count repair time against the window when the user includes it. Otherwise apply
the duration to the audit phase. Recheck the original failure and affected
normal, consumer and recovery paths against the final artifact. Honor any
agreed re-audit limit. Report each material item as corrected and verified,
refuted, explicitly accepted, deferred with an owner and next action, or
unresolved. Preserve outstanding obligations when time expires.
