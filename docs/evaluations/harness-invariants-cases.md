# Harness invariants qualification inputs

These are local qualification cases. The evidence base is the retained Router
stage and the measured CI and Morpho fixtures. The archive cases use synthetic
transcripts with manually specified expected events and dependencies.

## Router

The retained stage 021 of Router V5 gate-r5 observed 18 passes and one ignored
case in its first, server-auth libtest group. The retained recipe independently
names two required cases for this group. The local replay plan records those
cases, the Router owner, and the expected local candidate and test executable.
The checked-in CLI fixture uses synthetic candidate and executable identities.
Other stage 021 groups and release admission are outside this replay's scope.

The verdicts must be verified for the retained server observation, and unverified
for an SDK binary with the same case names, a stale run, and one missing required
case with other cases still green. A correctly scoped failed assertion must
remain a failure. An ignored required case must stay unverified.

The retained synthetic ownership fixture is qualified through durable state and
a loopback consumer. After ownership changes, the replacement must commit,
and stale commit, replay, or retry must leave the accepted operation intact.
The positive control and stale-writer negative control use separate scratch state.

## Hosted Rust CI

Inputs are the two cold one-job versus parallel repeated-build pairs, with all
four required fixture cases and identical executables. Time to final gate was
29/25 seconds for the control and 19/22 seconds for the candidate. Assigned runner
time was 77 and 34 seconds respectively over the two runs. The earlier cross-job
sharing design used 80 versus 68 runner-seconds and lengthened both runs.

The recommendation may favor one job for the observed fixture. A larger,
otherwise matched example with 12 seconds saved in setup and transfer and
15 seconds of lost suite overlap favors parallel execution for elapsed time,
by 3 seconds under the stated simple model. The 12/15 example is hypothetical.
A CI decision needs parity, the two distinct measures, and a crossover that
uses actual suite overlap and overhead at scale. Two pairs cannot establish
tail latency or billing.

## Morpho prepared pages

Use three fresh processes per variant on the same opening seed, closing catalog,
CPU/memory cap, and runtime. Six selected-page hashes match. Idle publication
moves from 857.6–903.6 ms to 8.3–8.7 ms and first selection after idle from
55.9–68.8 ms to 418.9–450.7 ms. Retained prepared bytes move from 42,692,250 to
35,977. The candidate spends 351–383 ms of first demand preparing shared inputs.

With a hypothetical 100 ms first-selection cap on this component boundary,
the lazy design fails the cap despite saving memory. With a hypothetical 500 ms
cap and retained prepared memory as the objective, the measured candidate fits
these samples and wins on that objective. Both caps are test inputs. Without an
agreed cap or objective the agent should keep the choice conditional. Test the
selected dependency closure under the same parity rule, including invalidation
and recovery before extending the scope of the claim.

## Archive

Use labeled plain JSONL, Zstandard, split and selected tar members. An explicit
request, an echoed file path, and a native injection are independent
observations. Copies of the identified injection count once. Missing turn
identity stays unknown. An incomplete final record preserves earlier positive
observations but cannot establish complete coverage.

Start reachability at an active fork and a required backup root. Traverse
declared links to an archived parent. An incomplete reference inventory keeps
an unrelated item unknown. Charge physical bytes once per observed inode and
report transcript bytes separately. Validate bounded reads, unsupported inputs,
and non-file entries through the read-only CLI.
