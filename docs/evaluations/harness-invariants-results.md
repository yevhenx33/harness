# Local qualification of the Harness invariants

The candidate is based on upstream main at 4a6771d7e49c43cb0fd8714ea31ffe360ff58b5f.
The three tracks are qualified at the boundaries below. The existing v024 policy
already owns the general requirements. The candidate changes skills, a standalone
reporting tool, fixtures, and the canonical verification suite.

## Test ownership and execution

The supplied Router plan connects two server authorization guarantees to
src/auth.rs at the retained candidate and to their named cases in stage 021's
recipe. The adapter independently reads the observed group from the stage log,
reconciles the stage and development receipts, and hashes the role-selected
executables.

| Retained run or injected observation | Result |
| --- | --- |
| Server executable, matching scoped run, both required cases passed | Verified |
| SDK executable with the same passing case names | Unverified, target and executable mismatch |
| Matching cases and executable with an older attempt | Unverified, run mismatch |
| Matching execution and other passing results with one required case omitted | Unverified for the missing case, verified for the other guarantee |

The CLI tests cover all six identity fields, skipped and incomplete cases,
an observed assertion failure, and contradictory or duplicate results.
Execution mismatches leave the guarantee unverified without reporting that its
case was absent from a matching execution. A synthetic stage 021 fixture runs
all four replay controls and verifies that a changed recipe invalidates the
execution. Its recipe and output are independently specified. The private
candidate and executable identifiers stay with the retained local plan.

A portable fixture observes the durable record after takeover and rejects both
the stale-writer control and a control that refuses all writes. The retained
lifecycle oracle was also executed in scratch state. The valid writer
passed all six checks. The stale-writer control failed all three late-write
checks for commit, replay, and retry while its restart and publication checks
still passed. The recorded Router run was replayed from local evidence. Only
the synthetic lifecycle binaries were executed for this track.

## Trade-off decisions

One reviewer applied the upstream and revised guidance to the same frozen CI
and Morpho cases. The reviewer knew the version and both expected outcomes.
This qualifies the decision method on the cases. It does not estimate a change
in autonomous agent reliability, intervention rate, or cost.

| Case | Upstream walkthrough | Revised guidance walkthrough |
| --- | --- | --- |
| Hosted Rust fixture | Choose the one-job layout on both observed metrics, retaining all four required cases. Qualify long suites separately. | Same fixture choice. Record the crossover as saved serial overhead versus lost overlap under comparable runner conditions. At 12 seconds saved and 15 seconds lost, the parallel variant finishes 3 seconds sooner in the hypothetical example. |
| Morpho component probe | Keep the memory and first-selection results separate. Qualify the selected dependency closure and preserve page parity. | Same component diagnosis. A 100 ms first-selection limit rules out the measured lazy candidate. With a 500 ms limit and memory as the objective it fits these samples and wins on retained bytes. The limit and priority must be supplied before making that choice. |

The original CI CSV confirms both elapsed pairs and the 77/34 runner-seconds.
The original Morpho comparison confirms exact selected-page parity for the
matched probe and explicitly limits its scope. The deciding checks come from
the [frozen cases](harness-invariants-cases.md).

## Archive observations and retention

CLI tests passed labeled JSONL, nested Zstandard, long tar member names,
split and torn input, repeated identities, missing turns, a symlink, an
unsupported type, stored and decoded limits, transitive and cyclic references,
and physical allocation deduplication. Structured tool arguments are included
in path observations. Malformed evidence marks the fragment partial. Conflicting
native names and turns are excluded from their confirmed counts. The fixtures
also distinguish a developer-provided skill catalog from a selected skill.
The tests verify that raw output text is absent.

A single retained archived chat provided a real format check. The diagnostic
read 269,926,658 bytes and reported 269,934,592 allocated bytes. It returned a
3,323-byte report. An independent source check found the same four native
loads across the three selected skills. Report size is an output measure.
The qualification did not test physical archive compaction.

A separate metadata-only case used the recorded active fork and archived
parent relationship from the November 2 backup report. The tool returned
unreadable transcript coverage and protected-by-reference independently.
The fixture with a backup root also protected its declared target. These are
snapshot reference results; the cleanup owner retains current-state and
recovery decisions.

## Release review

The canonical suite passed nine checks, including seven release-activation,
17 skill-discovery, eight evidence CLI, and 27 Python tests. The release boundary
used the exact upstream base. A parallel preflight reproduced a Linux executable
busy error in a generated skill-discovery fixture. The fixtures now use the
stable shell interpreter and retain the outcome assertions, including the
ambiguous-write control. The executable change is test-only.

The work was locally scoped and report-only at the Router and archive boundaries. Package verification, historical replay,
synthetic execution, and local skill discovery are different observations.
