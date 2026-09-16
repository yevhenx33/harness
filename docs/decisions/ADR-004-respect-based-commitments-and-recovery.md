# ADR-004: Organize work around commitments, evidence, and breach recovery

- Decision status: Accepted
- Evidence state: Designed
- Date: 2026-09-16
- Owner: root agent operating policy
- Scope: policy structure, commitment continuity, evidence-bound claims,
  breach recovery, and contextual confidence
- Supersedes: none; refines v016 and preserves ADR-003
- Superseded by: none

## Context and governing constraint

The user authorized implementing, deploying, verifying, committing, publishing
through a PR, and merging the discussed respect-based restructuring. Respect
means earned credibility from aligned commitments, actions, evidence, and
reports. Existing authority, budget, verification, and preservation obligations
must survive the reorganization.

This release changes instruction policy and target architecture. It does not
implement a capability kernel, general receipt protocol, confidence service, or
automated reputation enforcement. Acceptance records a design baseline, not
measured production behavior.

## Primary invariant, owner, and consumers

Every accepted commitment remains accountable to the user's intent, granted
authority, and observable evidence until completed or explicitly revised.
Blockers preserve unfinished obligations.

Root AGENTS.md owns the rule and versioned PR releases are its authoritative
write path. Executing agents and workflow skills consume it; task context and
receipts retain evidence. Actual system owners remain authoritative for facts
such as deployed artifacts. A task record or confidence projection cannot
overrule those owners.

## Decision and consequences

Organize the policy into authority and commitments, execution and resources,
evidence and claims, breaches and recovery, and learning and adaptation.
Preserve every v016 non-heading paragraph and every earlier immutable snapshot.
Keep governance and purpose before those five areas.

Separate user-granted authority, contextual confidence, and current readiness.
Relevant history may inform worker choice and proportional supervision within
the existing scope and budget. It never grants permission or waives a check.
Keep outcome classification unchanged and separate from breach disposition.

Treat breaches as observed mismatches without assuming intent. Correct the
record, contain affected behavior, trace the owner, repair within authority,
and restore reliance through relevant evidence. Unrelated successes, elapsed
time, and apologies cannot discharge a breach. Verification outages hold only
dependent decisions; they do not establish worker dishonesty.

The authorized restructuring admits eight repository files and the review size
needed to move existing policy and add its full immutable snapshot, ADR,
evaluation record and raw decisions, and target-design clarification. Product
LOC remains zero.
This one-release budget exception does not relax subsequent task limits.
Simplicity is 5/5 with that size exception; operational elegance is 5/5 against
the single policy owner, unchanged task interface, direct oracles, explicit
failure dispositions, and versioned recovery.

## Rejected alternatives

- A scalar respect account: unrelated successes could offset authority or
  reporting breaches and reward gaming or concealment.
- Automatic privilege promotion: confuses relevant competence with permission.
- Universal approval after failure: needlessly blocks independent authorized
  work and recreates the supervision burden this design addresses.
- A separate respect database or five new services: duplicates existing task
  evidence without a current consumer requiring another representation.
- Replacing existing obligations with virtue labels: makes the restructuring
  concise by removing the mechanisms that give it meaning.
- Self-grading, repeated model agreement, or policy hash checks as behavioral
  proof: none independently establishes an actual task outcome.

## Dependencies, proof obligations, and evidence

The [frozen comparison](../evaluations/respect-v017.md) defines representative
cases, independent expected decisions, release gates, observations, and limits.
The canonical verification command remains
`PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_harness.py`.
Compare all v016 non-heading paragraphs and earlier snapshot bytes directly.

Publish through the required policy-integrity check and squash merge. Deploy
by advancing the clean authoritative checkout to merged main and checking its
instruction consumers and current-snapshot parity. A fresh consumer probe must
identify the deployed policy and apply its relevant boundary. Existing sessions
and independently installed skill copies are outside automatic refresh.

Decision simulations support only the observed decisions. No improvement in
production reliability, speed, or supervision cost is claimed by this release.
The [runtime design](../sovereign-runtime.md) remains target architecture.

## Falsifiers, revisit triggers, and recovery

Revisit if an old obligation is lost, confidence changes authority, a blocker
erases a commitment, the agent claims more than observed, honest disclosure
causes a blanket penalty, recovery performs an unauthorized write, or measured
completion and user burden regress. A scenario discrepancy must be dispositioned
before release; insufficient evidence stays explicit.

Recover with a new immutable policy version restoring v016 behavior, a
superseding ADR, required verification, and deployment of the merged revision.
Never rewrite v016, v017, or this accepted reasoning. A confidence projection
failure must not change task truth or relax an authority boundary.
