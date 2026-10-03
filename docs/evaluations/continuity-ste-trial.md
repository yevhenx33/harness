# Continuity and STE writing: local trial

At trial close, this was a local skill candidate for a possible future release.
The official policy remains v019. No policy snapshot, installed skill, or
discovery path changed during the trial. The candidate adds handoffs and artifact-bound evidence to existing
methods, and STE-inspired technical writing to effective-writing. It does not
claim ASD-STE100 compliance or provide a new runtime.

## Frozen comparison

Six requests and their semantic oracles were frozen before editing. Two fresh
agents received the same requests, policy, tool access, and inherited model and
effort settings. Each agent received either baseline or candidate skills. The
oracle and the other arm's outputs were withheld. Cases ran as one batch per
arm; grading was parent review, not blind. Exact effective model identifiers,
token cost, and execution time were not independently recorded.

The [record](continuity-ste-trial.json) retains requests, expected decisions,
instruction hashes, exact responses, discrepancies, and remaining checks.

| Case | Observation |
|---|---|
| Continuation | Both retain the correction, authority, and unfinished browser checks. Neither names immediate independent work; the input did not establish what work was available. Retain this limitation. |
| Artifact change | Both keep historical tests separate from unknown activation and current consumer behavior. No observed decision gain. |
| Predicate gate | Both reject false output with exit code zero. The candidate explicitly asks to execute passing and failing inputs before trusting the correction. |
| Technical report | Both retain the block values, times, unknown durability, unmeasured savings, and target designs. The candidate leads with the verification gap. Comprehension was not measured. |
| Diagram | Both distinguish observed and inferred edges, retain evidence and snapshot references, and reject screenshot instructions. Rendering was not tested. |
| Grammar only | Both preserve the founder's voice and missing-data meaning. They make different valid number/article corrections. |

## Narrow correction and supplemental probe

The first continuation responses prompted a two-line clarification: name
available independent work while holding dependent acceptance. A new frozen
case explicitly supplied accessible source and a focused test. Two fresh agents
used the same supplemental input. Both chose that immediate work and retained
browser acceptance, deployment exclusion, and unrelated edits. The candidate
used one sentence for each procedural action. The original results remain intact;
the supplemental result does not turn them into clean passes.

## Evidence state and recovery

The canonical policy, skill, and test checks passed, including all 23 tests.
Each modified skill passed package validation. The final source hashes match
the tested instruction snapshots; all 19 policy snapshots remain unchanged.
Observed decision parity is a verified no-gain for the artifact and supplemental
continuation probes. The predicate response
contains useful extra guidance. Operational effectiveness, reader comprehension,
and cost improvement remain inconclusive or unmeasured. No release is qualified.

Next checks are a real authorized task resumed in a fresh session, comprehension
and total-cost measurement, rendered diagram inspection, and source-to-active
skill parity before activation. Those are future release gates, not completed
workflows. Recovery is to discard only this uncommitted task diff; the main
checkout and installed skills remain unchanged. The branch preserves the trial
while these observations are reviewed.

## Production promotion: 2026-10-03

The user authorized production promotion through a PR. The release adopts the
three instruction-only skill updates with the trial limits disclosed; it does
not claim measured behavioral improvement or create a v020 policy snapshot.
The trial's original requests, outputs, hashes, and observations remain intact.

A fresh agent resumed the actual release assessment from these repository
artifacts. It inspected the diff, checked all final skill hashes against the
record, confirmed v019 equality, and produced a release handoff without assuming
publication or activation. Implementation-slice v2 has supplemental-probe
coverage; its original six-case batch used v1. This is one observed handoff,
not a controlled comparison of operational effectiveness.

Both recorded diagrams rendered with Mermaid 11.12.0 in an isolated browser
context. Browser DOM inspection and visual review showed the snapshot labels,
inferred connection, and unchecked later updates. This verifies these fixture
diagrams, not a live producer-to-consumer flow.

Reader comprehension, total cost, and comparative workflow effectiveness remain
unmeasured follow-up questions; they are not claimed by this release. Publication
requires current canonical verification and the required PR check. Activation
requires merged-source instruction parity and fresh-consumer evidence. Git and
the PR retain publication and activation receipts. Recovery restores the saved
installed entrypoints and uses a reconciliation PR for any source reversal;
historical policy snapshots remain unchanged.
