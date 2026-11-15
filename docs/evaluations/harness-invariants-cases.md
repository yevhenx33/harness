# Harness invariants qualification inputs

The verification cases use synthetic plans and runner observations. The
trade-off cases use retained measurements. The archive cases use synthetic
transcripts with independently specified events and dependencies.

## Test ownership and execution

Use a plan for a fixture release unit with two named guarantees. Independently
provide the observed record for the exact candidate, package, target,
configuration, executable digest, and run attempt. Vary each execution field
individually. Identical case names from a different execution leave the planned
guarantees unverified. Skipped, missing, incomplete, and failed cases each have
a distinct interpretation. Omit one required case from an otherwise successful
matching execution. The affected guarantee stays unverified and the guarantee
supported by the remaining case is verified.

Test a writer handoff by pausing the old writer, letting the replacement publish,
and resuming the old writer. Observe durable state independently. The valid
replacement must make progress and the old writer must leave the accepted
operation intact. Both stale-write acceptance and refusing all writes fail.

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
