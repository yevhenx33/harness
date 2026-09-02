# ADR-002: Keep tiered reasoning as an explicit skill

- Decision status: Accepted
- Evidence state: Designed
- Date: 2026-09-02
- Owner: `tiered-reasoning` skill for opt-in routing; root policy for ordinary
  task admission
- Scope: Codex reasoning-tier invocation and root-policy behavior
- Supersedes: [ADR-001](ADR-001-user-assigned-reasoning-tiers.md)
- Superseded by: none

## Context and governing constraint

ADR-001 made a T0-T3 assignment mandatory for every problem through the root
policy. That ambient admission requirement interrupts ordinary work even when
the user does not want a tiered search. The user wants the pre-v013 Harness
behavior restored while preserving tiered frontier search as an available,
explicitly selected workflow.

Historical policy snapshots and accepted decision reasoning must remain intact.
The rollback therefore requires a new policy version and decision rather than
editing v013 or deleting ADR-001.

## Primary invariant and consumers

Ordinary tasks are admitted without a reasoning-tier requirement. T0-T3 routing
applies only when the user explicitly invokes `$tiered-reasoning` with a tier.
The root policy, skill discovery layer, Codex executor, and user consume this
boundary.

## Decision

Release v014 with policy content byte-identical to v012. Preserve v013 as a
superseded immutable snapshot. Keep one `tiered-reasoning` skill accepting T0,
T1, T2, or T3, and set `policy.allow_implicit_invocation: false` in its
`agents/openai.yaml` metadata.

The skill retains its effort mapping, candidate separation, adjudication, and
verification workflow. Those mechanics do not apply to a task unless the skill
is explicitly invoked, and invoking it does not expand authority, scope,
assurance, or publication rights.

## Consequences

- Tasks without an explicit skill invocation no longer stop for a tier.
- One skill preserves a single routing contract and avoids four duplicate skill
  packages.
- Users who want the frontier workflow must invoke `$tiered-reasoning` and name
  T0, T1, T2, or T3.
- Existing contexts may retain instructions loaded before the policy change;
  the new boundary applies when the updated policy and skill metadata are loaded.
- The repository still provides instructions rather than an enforced launcher,
  scheduler, isolation boundary, or capability kernel.

Recovery is a new superseding policy version and ADR. Historical snapshots and
accepted decision records remain preserved.

## Rejected alternatives

- Delete tiered reasoning entirely: removes a requested opt-in capability.
- Split T0-T3 into four skills: duplicates the shared contract and increases
  discovery and maintenance cost without changing the owner.
- Keep mandatory tiers but infer or default them: retains ambient routing and
  removes explicit user choice.
- Rewrite v013 or ADR-001: destroys policy and decision lineage.

## Dependencies and proof obligations

Repository checks must prove sequential policy history, recorded hashes, root
and Current snapshot equality, valid links, valid skill packaging, and disabled
implicit invocation metadata. A fresh Codex context must separately confirm
that the skill is absent from ambient context and remains available through
explicit invocation; repository integrity cannot prove runtime discovery.

## Falsifiers and revisit triggers

Revisit if Codex implicitly injects the skill despite its metadata, explicit
invocation cannot select the requested effort boundary, a single T0-T3 skill
causes material routing ambiguity, or representative use shows that the retained
workflow does not improve its declared outcomes.
