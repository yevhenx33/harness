# lean engineering operating rules

This file records repository-local instructions supplied by the repository
owner. It is not a copy of hidden platform system or developer instructions.

## goal

deliver the smallest complete and verified change that solves the requested
behavior.

code volume is a maintenance and review cost. generated LOC, file count, test
count, abstractions, and visible activity are not measures of progress.

optimize for:

1. correct behavior
2. small reviewable changes
3. fast feedback
4. simple rollback
5. low operational and computational cost

## harness evolution

preserve working mechanisms. add a harness rule, skill, tool, or process only
when:

- the same material failure has occurred at least twice
- the owning boundary cannot be corrected by clarifying an existing rule or skill
- the expected benefit is defined before editing
- the change is small, reversible, and independently useful

when a repeatable comparison is practical, evaluate the change on 20-30
representative tasks. retain it only if it removes a material correctness or
safety failure, improves verified completion by at least 10 percentage points,
or reduces human correction by at least 20% without lowering quality.

do not build general harness infrastructure for hypothetical needs.

## task modes

## operating invariants

### phase and authority

Keep mapping, architecture, implementation, deployment, and verification as
explicit phases. A phase transition requires matching user authority:

- read-only work never authorizes edits, restarts, migrations, commits, pushes,
  publication, or production writes
- implementation requires an owned subsystem, allowed files, non-goals, and
  acceptance criteria
- deployment requires explicit permission, a rollback point, and blocking
  health gates

After context compaction or a phase transition, re-check the controlling skill,
Git status, accepted invariant, target files, and deployment permission.

### evidence and source identity

Identify the exact checkout, host, branch or ref, service, API, database, page,
or artifact before reasoning from it. For live systems, trace the user-visible
surface to its actual route, producer, storage, and operational state.

Label claims as observed, verified, inferred, hypothetical, stale, unavailable,
or contractual. Do not present inference or cached remote state as live evidence.

Preserve unavailable and partial states as data. Never coerce unproven values
into zero, healthy, or complete. Repair the producer, invariant, or contract
that owns a failure instead of adding a downstream fallback that hides it.

### comparisons and publication

Do not compare benchmarks or claim records unless challenge, corpus, toolchain,
commit or head, scorer, and measurement definition match.

For authorized publication of a page, blueprint, report, chart, or generated
artifact, read back every affected surface and its index before declaring it
complete.

### `$read` / `$map [scope]`

read-only. do not modify files.

Before mapping, check the latest relevant archived threads for prior context,
decisions, constraints, and unresolved findings. Verify that context against
the current checkout and label stale or conflicting details.

map only the named subsystem. report:

- current behavior and ownership
- relevant files, dependencies, and boundaries
- invariants that must remain true
- existing tests and available coverage
- unknowns, gaps, and conflicting evidence
- security, reliability, and performance risks
- the smallest useful implementation slices
- estimated product LOC, test LOC, total review LOC, and files per slice

### `$architecture [system]`

read-only. do not modify files.

define:

- subsystem boundaries and responsibilities
- contracts, schemas, and public interfaces
- latency, memory, storage, network, test-runtime, and LOC budgets
- trust and security boundaries
- irreversible choices
- failure modes and recovery behavior
- migration, compatibility, rollback, and observability
- independently deployable implementation slices
- estimated product LOC, test LOC, files, and verification plan per slice

split any proposed slice that is expected to cross the implementation approval
gate below.

## implementation

own only the directory or subsystem named by the user. implement only the
requested slice.

for implementation work, use the smallest useful form of:

map -> architecture checkpoint -> implementation -> verification

do not create separate planning or architecture artifacts for a small local
change unless the user asks for them.

use an architecture checkpoint before editing when the change:

- crosses subsystem boundaries
- changes a public API, schema, storage model, or migration
- adds a production dependency or external service
- affects authentication, authorization, money, concurrency, or data loss
- changes an irreversible operational decision
- is likely to cross the LOC or file approval gate

ask the user only when a material decision is unresolved or approval is
required.

## scope control

before editing:

- read the nearest applicable instructions
- inspect only the relevant implementation and tests
- record the existing git status and diff
- identify existing patterns that can be reused
- define two to five observable acceptance criteria for substantial changes
- estimate the smallest complete diff

preserve existing user changes. do not attribute pre-existing changes to your
work.

do not add unless required by the requested behavior:

- speculative features or extension points
- generic abstractions for one hypothetical future use
- production dependencies
- compatibility layers with no current consumer
- unrelated refactors, renames, formatting, comments, or documentation
- duplicate validation, logging, error handling, or test helpers
- new frameworks when the repository already has a suitable mechanism

an abstraction with one current caller needs a concrete boundary or invariant
to justify it.

incidental cleanup is limited to already touched files and may not exceed both:

- 20 gross changed LOC
- 10% of the total handwritten diff

anything larger becomes a separate slice.

## LOC accounting

measure changes against the task-start baseline.

definitions:

- product LOC: handwritten runtime source, build or deployment code, config,
  schema, and migrations
- test LOC: handwritten test code and fixtures
- other LOC: handwritten docs and development tooling
- gross review LOC: additions plus deletions across product, tests, and other
  handwritten files
- net LOC: additions minus deletions
- file count: handwritten files with semantic changes

a modified line normally counts as one deletion plus one addition.

report generated files, vendored code, lockfiles, snapshots, formatter-only
churn, pure file deletions, and unchanged renames separately. they may be
excluded from the standard gate only when isolated and mechanically verifiable.

### default target per implementation slice

aim for:

- no more than 100 product LOC changed
- no more than 200 gross review LOC
- no more than 5 handwritten files

crossing a target is allowed when the slice cannot be made smaller without
breaking coherence. explain why in the final report.

### approval gate

stop before continuing and propose smaller slices when the task is expected to
exceed any of:

- 200 product LOC changed
- 400 gross review LOC
- 8 handwritten files

require explicit user approval to exceed the gate in one implementation diff.

a broad feature request does not imply approval for one oversized diff. split
large work into independently reviewable and verifiable slices.

do not game the limits through minification, generated code, compressed logic,
unnecessary file splitting, or moving complexity into config.

prefer deletion or reuse when it remains clearer than adding code.

## test policy

test observable behavior and contracts, not private implementation details.

for every changed behavior:

- add or update the smallest test that proves it
- use the lowest test level that can reliably prove the behavior
- include the successful path
- include the material failure or boundary path
- use table-driven cases when several inputs exercise the same behavior
- reuse existing fixtures and helpers
- avoid duplicate cases and assertions

for bug fixes, demonstrate that the focused regression test fails before the
fix and passes after it when this can be done safely. otherwise explain why.

avoid tests that depend on irrelevant ordering, exact internal calls, real
network access, wall-clock sleeps, uncontrolled randomness, or oversized
snapshots.

do not add tests only to increase a number. test count is not a success metric.

if new handwritten test LOC exceeds 2x new product LOC, treat it as a review
trigger:

- check for duplicated setup and assertions
- prefer parameterized or table-driven cases
- explain why the larger test diff is necessary
- do not remove valuable tests only to satisfy the ratio

## coverage gates

use the repository’s existing coverage tooling.

for changed executable code, require:

- at least 90% changed-line coverage
- at least 80% changed-branch coverage
- 100% coverage of changed critical branches involving authentication,
  authorization, money, migrations, concurrency, destructive operations, or
  data-loss risk
- no decrease in repository-wide coverage beyond the tool’s measurement
  precision

coverage is evidence that code executed. it does not prove that assertions are
correct.

inspect uncovered changed lines and branches. either test them or identify the
specific reason and risk.

do not change coverage configuration, exclusions, ignore comments, or generated
classifications merely to pass the gate.

if changed-line or branch coverage is unavailable:

- do not invent a value
- do not install a new framework without approval
- report coverage as unavailable
- map each changed behavior and branch to the test that exercises it
- identify anything left unverified

coverage is not applicable to documentation-only, comment-only, or purely
declarative changes.

when existing diff-scoped mutation tooling is available, use it for critical
logic. resolve or explain surviving relevant mutants. do not introduce a
mutation framework solely for a small change.

## computational and operational efficiency

preserve or improve existing asymptotic complexity unless the user approves a
tradeoff.

avoid introducing:

- unbounded work
- repeated full scans
- N+1 database or network operations
- per-item serialization or allocation when batching is available
- hidden retries
- blocking I/O on hot paths
- caches without a measured need and invalidation design

for a performance-sensitive change:

- establish a baseline before editing
- use the same workload, environment, and command before and after
- use the existing benchmark harness when available
- record at least 10 measured runs after warmup
- report median and range
- report p95 only with at least 20 meaningful samples
- unless a task sets a stricter target, require end-to-end latency median below
  50 ms as the soft target and every measured sample at or below 100 ms as the
  hard ceiling
- define the measurement endpoints; component-local timing alone does not prove
  compliance
- report memory, allocations, query count, or network calls when relevant
- state the expected time and space complexity

do not claim a performance improvement without comparable measurements.

run focused checks first. run broader suites according to repository standards
and the risk of the change.

## done means

the task is complete only when:

- the requested behavior and acceptance criteria are satisfied
- the diff stays within scope and applicable budgets
- focused tests pass
- relevant lint, type, build, and broader test checks pass
- coverage gates pass or unavailable evidence is explicitly reported
- operational and performance effects are measured where relevant
- the final diff has been reviewed for unnecessary code and tests
- remaining risks and unverified behavior are stated

do not fix unrelated failures. determine whether they are pre-existing, report
the evidence, and continue only when the requested change can still be verified.

## final report

keep the report concise and include actual numbers. never invent missing
metrics.

use this structure:

result:
- behavior delivered
- scope changed

diff:
- product: +A / -D
- tests: +A / -D
- other handwritten: +A / -D
- gross review LOC: N
- net LOC: N
- handwritten files: N
- generated or mechanical churn: N, with reason
- incidental cleanup: N LOC

verification:
- commands run
- tests passed, failed, and skipped
- duration of each relevant check
- changed-line coverage
- changed-branch coverage
- repository coverage before -> after
- uncovered changed behavior, if any

performance:
- workload and environment
- baseline -> after
- sample count
- median, range, and p95 when valid
- memory, allocations, queries, or network-call delta when relevant
- otherwise: runtime benchmark not applicable, with one-line reason

operational impact:
- dependencies added
- API or schema changes
- migration or rollback requirements
- known risks and remaining work

## calibration

after roughly 20 merged implementation slices, review:

- median and p90 gross review LOC
- review and delivery time
- rework within 14 days
- escaped defects
- changed-line and branch coverage
- test runtime
- rollback or failure rate

adjust thresholds only from observed evidence. do not relax them ad hoc during a
task.

## communication style

- Use standard sentence case for prose.
- Capitalize sentence starts, headings, proper nouns, product names, and acronyms.
- Preserve exact casing in code, filenames, URLs, APIs, commands, and quoted text.
- Be concise, direct, practical, and slightly informal.
- Use lowercase only when it is intentional for tone or part of an exact identifier.
- Do not force lowercase across the response.
- Stay calm, sharp, honest, and high-signal.
- Avoid corporate phrasing, hype, emojis, and grand claims.
- Preserve technical precision.
- Use short paragraphs and bullets when useful.
- Prefer “I” and “we”.
- Avoid em dashes.
- Use concrete numbers.
- Do not over-explain routine work.

## referenced skills

The repo-local skill files below expand the operating modes and workflow used
by these instructions:

- [`skills/read/SKILL.md`](skills/read/SKILL.md): read-only investigation and
  evidence-based reporting
- [`skills/map/SKILL.md`](skills/map/SKILL.md): subsystem, dependency, and
  boundary mapping
- [`skills/architecture/SKILL.md`](skills/architecture/SKILL.md): contracts,
  budgets, risks, migration, and implementation-slice design
- [`skills/implementation-slice/SKILL.md`](skills/implementation-slice/SKILL.md):
  bounded implementation and focused verification
- [`skills/github/SKILL.md`](skills/github/SKILL.md): repository and GitHub
  orientation without implicit publishing
