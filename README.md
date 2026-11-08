# Harness

Harness is the source-controlled operating policy and skill library for agent
work. [AGENTS.md](AGENTS.md) governs commitments, authority, hygiene, service
boundaries, verification, delivery, and recovery. Skills provide task-specific
methods. Project repositories and runtimes own their state and direct oracles.

## Operating model

```text
concrete outcome and authority
  -> outcome-first structural design
  -> incremental-first execution at the owner
  -> integration/invariant verification and scoped performance evidence
  -> two completion passes
  -> authorized delivery and cleanup
```

Prefer sustainable structural solutions over small patches that preserve the
cause. Use keyed/event-driven updates and bound full fallbacks. Prefer isolated
microservices without fragmenting coherent responsibilities. Use one agent;
independent tool calls may run concurrently within the task budget.

Reports start with TLDR and concrete measured before/after/target where available.
Estimates and unavailable values remain explicit. A proposal, passing package
check, merged source, installed package, and observed runtime outcome are distinct.

## Source ownership and installation

| Component | Owner and boundary |
|---|---|
| [AGENTS.md](AGENTS.md) | Shared operating rules; current snapshot must match byte for byte |
| [Policy history](agents/README.md) | Immutable sequential releases with recorded digests |
| [Skills](skills/) | Domain methods and selectively loaded references |
| [Blueprint catalog](docs/blueprints/README.md) | Concise retrieval descriptions and source provenance; external documents remain independently owned |
| [Decisions](docs/decisions/README.md) and [evaluations](docs/evaluations/) | Historical rationale and scoped evidence, not release or activation authority |
| [Canonical verifier](scripts/verify_harness.py) | Policy, release, package, and regression checks; not proof of downstream behavioral improvement |
| Git and pull requests | Source admission and change review |
| Tasks and project systems | Outcome evidence, unfinished obligations, runtime and recovery ownership |

The repository is the release source. Editing it does not automatically authorize
updating user-level skills or activating a policy in another checkout. Before
editing a checkout referenced by global policy links, inspect those links; prepare
unreleased changes in an isolated checkout when activation is excluded.

Presence in `skills/` proves tracked source, not installation or discovery. Check
the [activation profile](contracts/skill-activation.txt) against an accepted checkout:

```sh
rustc --edition=2021 scripts/verify_release.rs -o /tmp/harness-release-guard
/tmp/harness-release-guard activation "$PWD" /home/ubuntu/.codex/skills
```

The profile covers 21 intended release packages. This compares complete packages,
including metadata and references. It rejects
missing/extra/changed files; it does not install packages or prove session reload.
Walks are bounded to 256 entries, 16 path levels, and 16 MiB per package; nested
symlinks are rejected. A fresh consumer check establishes discovery. Activation
and behavioral qualification remain separate from source admission.

For explicitly authorized repository-scoped discovery, expose the library under
`.agents/skills` using the host's supported skill discovery mechanism. Preserve
existing links and independently installed skills. An existing session may need
reload; do not claim that source edits changed its instructions.

## Skill selection

Use the narrowest method that changes the task's decisions. Shared rules live in
AGENTS.md rather than being copied into every skill. Descriptions select skills;
references supply conditional detail.

| Skill | Method |
|---|---|
| [read](skills/read/SKILL.md), [map](skills/map/SKILL.md) | Read-only investigation, owners, flows and evidence |
| [clean](skills/clean/SKILL.md) | Clutter inventory, protected-state checks, and authorized retirement |
| [architecture](skills/architecture/SKILL.md), [contract](skills/contract/SKILL.md) | Outcome-first design, service ownership and public interfaces |
| [implementation-slice](skills/implementation-slice/SKILL.md) | Owner-level implementation, integration/invariant checks and delivery |
| [budget](skills/budget/SKILL.md), [latency-optimization](skills/latency-optimization/SKILL.md), [me](skills/me/SKILL.md) | Resource budgets, question-driven profiling and authorized bounded observation |
| [current-history-delta](skills/current-history-delta/SKILL.md) | Incremental current/history/tail transport and retention |
| [github](skills/github/SKILL.md) | Repository identity, PR admission, release and worktree cleanup |
| [structural-factorization](skills/structural-factorization/SKILL.md), [atomic-inversion-brutalism](skills/atomic-inversion-brutalism/SKILL.md), [solve-from-first-principles](skills/solve-from-first-principles/SKILL.md) | Structural alternatives, owning mechanisms and falsification |
| [vector](skills/vector/SKILL.md), [anchor-rpc-witness](skills/anchor-rpc-witness/SKILL.md) | Protocol-local architecture and same-block independent witnesses |
| [deep-review](skills/deep-review/SKILL.md) | Single-agent document coverage, challenge and global consistency |
| [effective-writing](skills/effective-writing/SKILL.md), [audience-expertise](skills/audience-expertise/SKILL.md), [attention-weighted-narrative](skills/attention-weighted-narrative/SKILL.md), [experience-backward-product-design](skills/experience-backward-product-design/SKILL.md) | Writing, audience, narrative and product methods |
| [blueprint-methods](skills/blueprint-methods/SKILL.md), [montecarlo](skills/montecarlo/SKILL.md) | Relevant mechanisms and bounded scenario models |
| [tiered-reasoning](skills/tiered-reasoning/SKILL.md) | Explicit T0-T3 effort and single-agent hypothesis comparison |
| [ns](skills/ns/SKILL.md) | Explicitly requested next prompt; never executes it |

## Blueprint retrieval

Search [catalog descriptions](docs/blueprints/README.md) for problem shape,
mechanism, and constraints before opening relevant source sections. The owning
library is `/home/ubuntu/blueprints`; Harness does not copy or modify its documents.
Descriptions and digests are retrieval/provenance evidence, not proof that a
mechanism is effective in the current task.

Refresh the existing catalog only when maintaining its source inventory or descriptions:

```sh
python3 scripts/blueprint_catalog.py --source /home/ubuntu/blueprints --refresh
```

Run without `--refresh` to audit current source parity. An unavailable or changed
source fails explicitly. CI verifies the retained manifest/rendering, not current
external freshness. Preserve dirty/untracked source status and do not publish
private source content as an incidental library update.

## Verification and policy release

Requirements: Python 3.11, Rust 1.75+, and Git history.

```sh
PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_harness.py
```

The verifier builds Rust gates/tests outside the checkout, verifies policy history,
checks profiled skill sources, and runs regression checks. `HARNESS_RELEASE_BASE`
defaults locally to `main`; CI supplies the PR base or previous push commit with
full history. Unavailable or unrelated baselines fail. Installed-package parity
uses the separate activation check above.

A policy release updates AGENTS.md, adds the next immutable snapshot, marks it
Current and its predecessor Superseded, and records all snapshot SHA-256 values
in the policy index. Historical snapshots are never rewritten. Run the canonical
checks and both completion passes; publish/merge/activate only within explicit
authority. After an authorized merge, clean up its implementation worktree under
the root preservation and recovery rules.

A passing package check proves package integrity. Comparative behavioral evidence
requires frozen tasks, matched model/effort/inputs/tools/oracles, and reporting of
completion, authority/reporting errors, interventions, unnecessary stops, and cost.
Do not claim improvement from wording changes or decision simulations alone.

The [focused v024 comparison](docs/evaluations/v024-focused-comparison.md) records
source findings, executable regressions, and a completed six-case decision
comparison, with its limits. The earlier [comparison plan](docs/evaluations/v024-comparison-plan.md) is an
unexecuted proposal; its twelve-trial procedure is not itself a release requirement.

## Scoped policy-sync diagnostic

[The sync contract](contracts/sync-assessment.json), [schema](schemas/sync-receipt.schema.json),
and [assessment tool](scripts/run_sync_assessment.py) support a bounded read-only
comparison of policy state. They do not fetch, merge, reset, commit, or push.
The [Python](scripts/serve_sync_status.py) and [Rust](scripts/serve_sync_status.rs)
status servers are experimental reference tools, not supported production services.
The checked-in contract is local-path-specific. Use only its admitted scope;
this diagnostic is not a general task receipt service.

## License

Licensed under the [Apache License 2.0](LICENSE).
