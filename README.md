# Harness

Harness is a versioned operating policy and modular skill library for disciplined
Codex work.

It helps turn a request into a bounded workflow:

```text
request
  -> authority and success contract
  -> narrowest applicable skill
  -> bounded work
  -> direct verification
  -> evidence-backed result
```

Harness is not an autonomous-agent runtime. It currently ships instructions,
skills, integrity checks, and one bounded policy-sync experiment. Capability
enforcement, signed intents, durable receipts, confidential inference, and a
runtime kernel remain target architecture.

## What exists today

| Surface | Authoritative owner | Purpose |
|---|---|---|
| Operating policy | [`AGENTS.md`](AGENTS.md) | Authority, scope, budgets, invariants, verification, recovery, and reporting |
| Policy history | [`agents/README.md`](agents/README.md) | Immutable, hashed releases of the operating policy |
| General workflows | [`skills/`](skills/) | Read, map, architecture, implementation, and GitHub procedures |
| Security review pack | [`security-review-router`](skills/security-review-router/SKILL.md) | Select one specialized security method without starting a review |
| Policy integrity | [`verify_policy.py`](scripts/verify_policy.py) | Verify policy versions, hashes, root equality, sequencing, and Markdown links |
| Skill integrity | [`verify_skills.py`](scripts/verify_skills.py) | Verify the security graph, package structure, routing contracts, and invocation boundaries |
| Policy-sync experiment | [`run_sync_assessment.py`](scripts/run_sync_assessment.py) | Read-only comparison of local and remote policy state |
| Target runtime design | [`sovereign-runtime.md`](docs/sovereign-runtime.md) | Future architecture, explicitly separated from current capability |

## Operating policy

`AGENTS.md` is the current policy and governs work inside this repository. It
defines:

- authority boundaries between reading, changing, publishing, deploying, and
  destructive operations;
- the task contract: outcome, scope, owner, invariant, oracle, authority, budget,
  failures, non-goals, and recovery;
- reduction before automation;
- computational and operational cost controls;
- independent verification and outcome classification;
- concise, consequence-calibrated reporting.

Every accepted policy version is stored unchanged under `agents/versions/`.
The root policy must be byte-identical to the single version marked Current.

## General workflow skills

| Skill | Use it for |
|---|---|
| [`read`](skills/read/SKILL.md) | Read-only investigation and audit |
| [`map`](skills/map/SKILL.md) | Ownership, data-flow, dependency, and runtime mapping |
| [`architecture`](skills/architecture/SKILL.md) | Bounded design across security, storage, money, concurrency, or public contracts |
| [`implementation-slice`](skills/implementation-slice/SKILL.md) | One small implementation with direct verification and recovery |
| [`github`](skills/github/SKILL.md) | Repository identity, branches, pull requests, rules, and publication boundaries |
| [`effective-writing`](skills/effective-writing/SKILL.md) | Review, revise, or draft prose by testing whether it does its intended job |
| [`audience-expertise`](skills/audience-expertise/SKILL.md) | Calibrate content to an explicit audience domain-expertise level from E0 to E10 |

## Domain skills

| Skill | Use it for |
|---|---|
| [`credit-notes-protocol`](skills/credit-notes-protocol/SKILL.md) | RLD Credit Notes, Credit Index, custom-maturity rate hedges, funded protection, and settlement |

## Security Review Method Pack

The security pack is a library of independent instruction-only skills.

```text
L0  security-review-router
      |
      +-- authoritative method-index.yaml
      |
L1  selected security method
      |
L2  tools, practices, protocol, and evidence rules
      |
L3  method lineage, strengths, limitations, and primary sources
```

Methods are connected through typed horizontal links:

- `complements`
- `precedes`
- `validates`
- `escalates_to`
- `alternative_to`

The graph lives in
[`method-index.yaml`](skills/security-review-router/references/method-index.yaml).
All methods use the same
[`review-contract.md`](skills/security-review-router/references/review-contract.md).

### Available methods

| Method | Best suited for |
|---|---|
| [`risk-audit`](skills/security-method-risk-audit/SKILL.md) | Broad repository or component review with unknown exposure |
| [`diff`](skills/security-method-diff/SKILL.md) | Pull requests, commits, patches, and changed behavior |
| [`threat-model`](skills/security-method-threat-model/SKILL.md) | Unclear assets, attackers, trust boundaries, and security requirements |
| [`architecture`](skills/security-method-architecture/SKILL.md) | Cross-component security boundaries, privilege, isolation, and recovery |
| [`attack-path`](skills/security-method-attack-path/SKILL.md) | Proving reachability and impact from attacker-controlled source to sink |
| [`red-team`](skills/security-method-red-team/SKILL.md) | Attacker-led discovery of bounded multi-step abuse chains |
| [`ctf`](skills/security-method-ctf/SKILL.md) | Isolated targets where a working exploit is the strongest oracle |
| [`variant-analysis`](skills/security-method-variant-analysis/SKILL.md) | Finding siblings of a validated vulnerability root cause |
| [`invariant`](skills/security-method-invariant/SKILL.md) | Authorization, accounting, protocols, and state-machine properties |
| [`fuzzing`](skills/security-method-fuzzing/SKILL.md) | Executable parsers, decoders, APIs, and untrusted-input surfaces |
| [`formal`](skills/security-method-formal/SKILL.md) | Bounded proof obligations and machine-checkable properties |
| [`supply-chain`](skills/security-method-supply-chain/SKILL.md) | Dependencies, CI, builds, provenance, signing, and releases |

The router recommends one method and stops. L1 methods disable implicit
invocation and run only after an explicit request.

Example:

```text
Use $security-review-router to select a method for reviewing commit abc123.

Use $security-method-diff to review commit abc123 against its parent for
security regressions.
```

## Activate the skills in Codex

This repository stores versioned skill sources under `skills/`. Codex discovers
repository-scoped skills under `.agents/skills` and supports symlinked skill
folders. See the [official Codex skill documentation](https://developers.openai.com/codex/skills).

For local use in this checkout, expose the library at that discovery boundary:

```sh
mkdir -p .agents
ln -s ../skills .agents/skills
```

If newly exposed skills do not appear, restart Codex.

Activation is separate from source ownership: editing this repository does not
automatically update independently installed user-level copies.

## Verify the repository

Requirements:

- Python 3.11
- PyYAML for security graph validation

Install the development dependency in an environment you control:

```sh
python3 -m pip install -r requirements-dev.txt
```

Run all direct checks:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_policy.py
PYTHONDONTWRITEBYTECODE=1 python3 scripts/verify_skills.py
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s tests -v
```

The checks enforce:

- one current immutable policy version;
- sequential versions and recorded SHA-256 hashes;
- byte equality between `AGENTS.md` and the current snapshot;
- valid local Markdown links;
- twelve graph-backed security methods;
- resolved and reciprocal typed links;
- disabled implicit invocation for every L1 method;
- router execution boundaries and routing-score fixtures.

GitHub runs the same integrity workflow on pull requests and pushes to `main`.

## Policy-sync experiment

The repository contains a bounded read-only experiment for comparing local
policy state with live `origin/main`:

- [`sync-assessment.json`](contracts/sync-assessment.json) defines its authority;
- [`sync-receipt.schema.json`](schemas/sync-receipt.schema.json) defines its
  narrow output;
- [`run_sync_assessment.py`](scripts/run_sync_assessment.py) owns the host
  classification.

This schema is not a general Harness receipt protocol. The diagnostic does not
fetch, merge, reset, commit, or push.

The [Python](scripts/serve_sync_status.py) and
[Rust](scripts/serve_sync_status.rs) Unix-socket status servers are experimental
reference implementations, not supported runtime services. The checked-in sync
contract is currently local-path-specific and must be parameterized before
portable use.

## Current capability versus target architecture

Current:

- instruction policy;
- immutable policy releases;
- general and security review skills;
- deterministic integrity checks;
- bounded read-only policy-sync diagnostic;
- GitHub rules and CI.

Not currently implemented:

- signed-intent protocol;
- capability broker or enforcement kernel;
- generalized receipt wire format or journal;
- worker scheduler or commit service;
- confidential-inference runtime;
- database, embeddings, or agent orchestration harness.

The future boundaries are documented in
[`docs/sovereign-runtime.md`](docs/sovereign-runtime.md). Nothing in that
document should be treated as deployed capability unless separately marked and
verified.

## Changing the policy

A policy change must be one complete release:

1. update `AGENTS.md`;
2. add the next immutable `agents/versions/vNNN.md`;
3. mark exactly that version Current;
4. mark its predecessor Superseded;
5. record every snapshot SHA-256 in `agents/README.md`;
6. run the complete verification suite;
7. publish through a pull request and the required integrity check.

Historical snapshots are never rewritten. A correction creates a new version.

## Repository principles

- Authority is explicit.
- Unknown state is never rendered as success.
- Owners and invariants come before mechanisms.
- The narrowest adequate method wins.
- Tools produce evidence, not truth.
- Verification must be able to falsify the claim.
- Target architecture is never presented as current capability.
- Learning remains provenance-preserving and reverified before reuse.

## License

Licensed under the [Apache License 2.0](LICENSE).
