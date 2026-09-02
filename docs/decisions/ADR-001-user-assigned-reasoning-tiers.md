# ADR-001: User-assigned reasoning tiers and worktree-separated frontier search

- Decision status: Superseded
- Evidence state: Designed
- Date: 2026-09-01
- Owner: user for tier assignment; Harness policy for routing
- Scope: Codex task admission, frontier candidate separation, synthesis, and
  verification
- Supersedes: none
- Superseded by: [ADR-002](ADR-002-explicit-tiered-reasoning-skill.md)

## Context and governing constraint

Routine and frontier problems should not consume the same search budget.
Automatic classification would let the model redefine the user's intended
investment. Conversely, AGENTS.md is instruction policy loaded when a run
starts; it cannot retroactively mutate an already-running context's reasoning
effort. Stochastic candidate generation also converges when approaches share
reasoning history or code state.

The design must give the user sole tier authority, apply a real Codex effort
setting before execution, preserve causally distinct approaches, and allow only
the frozen oracle to promote a result.

## Primary invariant and consumers

Only the user assigns the current problem's tier. Every candidate receives the
same contract and baseline but no sibling interpretation or implementation
before synthesis. The conductor, candidate agents, synthesizer, verifier, and
user consume this boundary.

## Decision

The task interface is an explicit T0, T1, T2, or T3 assignment, mapped to low,
medium, high, or xhigh effort. If the active context cannot apply that setting,
it remains a conductor and requests a fresh executor with an explicit override.
Execution proceeds only after runtime confirmation; otherwise effort is unknown
and the task is blocked unless the user revises the contract.

At T3, at least three operator lanes use the same raw packet and shared
atomic-mapping and brutalist disciplines while producing independent maps.
Factorization, inversion, and representation change run in separate contexts
and, for competing code, separate worktrees from one sealed baseline. Mutable
runtime resources are namespaced.

A fresh adjudicator starts from the baseline and may select, combine, or reject
candidates. A combination is a new candidate. Only a sealed final artifact goes
to a fresh verifier with the frozen oracle and no persuasive rationale.

## Consequences

- T3 pays deliberate token, worktree, disk, and coordination cost.
- Missing tiers stop substantive work rather than being inferred.
- Policy provides cooperative blindness. Enforced isolation requires a separate
  permission or sandbox receipt; worktrees are not a security boundary.
- Candidate, hybrid, and repair implementations share one admitted cap.
- The repository provides policy and a skill, not an enforced launcher,
  scheduler, or capability kernel.
- Synthesis is conditional; no-gain, invalid, and inconclusive are valid ends.

Recovery is to abort the tournament, preserve evidence, discard unaccepted
candidates, and remove worktrees only after confirming they contain no user
work. A released policy change is reversed only by a new superseding version.

## Rejected alternatives

- Model-inferred tiers: violates user ownership and invites prestige routing.
- Claiming AGENTS.md changed the running root: reports a setting that was not
  applied.
- One shared atomic map: creates a common-mode representation failure.
- Shared contexts or worktrees: contaminates both hypotheses and code.
- Forced synthesis or bulk diff merging: hides incompatibility and creates an
  unverified hybrid.
- Model consensus or eloquence as selector: neither can falsify correctness.

## Dependencies and proof obligations

The mechanism depends on Codex launch or agent-creation effort overrides, fresh
agent contexts, Git worktrees for writable candidates, and an independent task
oracle. Repository integrity proves only packaging and policy lineage. A
representative T0-T3 evaluation must measure whether routing reduces cost at
lower tiers and improves held-out quality at T3 without contamination.

Official behavior references:

- [Codex configuration](https://learn.chatgpt.com/docs/config-file/config-reference)
- [Codex custom instructions](https://learn.chatgpt.com/docs/agent-configuration/agents-md)
- [Codex subagents](https://learn.chatgpt.com/docs/agent-configuration/subagents)
- [Codex worktrees](https://learn.chatgpt.com/docs/environments/git-worktrees)

## Falsifiers and revisit triggers

Supersede this decision if Codex cannot apply or report explicit effort at the
chosen boundary; candidate contexts cannot stay blind or mutable state cannot be separated;
the third operator adds no causal diversity; the oracle cannot distinguish
candidates; or representative evaluation shows that coordination cost exceeds
the measured quality gain.
