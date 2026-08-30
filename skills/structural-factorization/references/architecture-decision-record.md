# Architecture Decision Records

Use an architecture decision record (ADR) to preserve why a material structural
boundary exists, which alternatives were rejected, what evidence supports it,
and what would require the decision to be reopened.

## Trigger

Create or supersede an ADR when a decision materially affects an authoritative
owner, public interface, persistent representation, security or authority
boundary, concurrency or ordering rule, recovery model, or implementation
freeze. A costly or plausible rejected alternative also warrants a record when
future reintroduction could violate the primary invariant.

Do not create an ADR for a routine implementation detail, a temporary local
choice, or a decision that is both cheap and safe to reverse.

## Location and identity

Follow the repository's established convention. If none exists, use:

```text
docs/decisions/README.md
docs/decisions/ADR-NNN-short-title.md
```

Give every record a stable sequential ID. Keep the index compact: ID, title,
decision status, evidence state, and supersession link.

## Required record

```markdown
# ADR-NNN: Short decision title

- Decision status: Proposed | Accepted | Superseded | Rejected
- Evidence state: Designed | Reference-proven | Implementation-tested |
  Production-verified
- Date:
- Owner:
- Scope:
- Supersedes:
- Superseded by:

## Context and governing constraint

What forced a choice, including the simplest viable baseline and relevant
budget or failure boundary.

## Primary invariant and consumers

What must remain true, which authority owns it, and which consumers depend on
the decision.

## Decision

The chosen mechanism and the boundary it establishes.

## Consequences

Accepted costs, limitations, operational effects, and recovery or migration
consequences if the decision changes.

## Rejected alternatives

Alternatives considered and the specific reason each failed the invariant,
oracle, cost, authority, or recovery requirement.

## Dependencies and proof obligations

Upstream assumptions, downstream decisions, direct success or failure oracles,
and the specification, reference model, tests, implementation, or receipts that
provide evidence.

## Falsifiers and revisit triggers

Observable conditions that invalidate an assumption, breach the budget, reveal
a better mechanism, or otherwise require a new decision.
```

Use only applicable fields, but never omit the invariant, owner, chosen
mechanism, consequences, evidence state, and revisit condition.

## Lifecycle

Decision status and evidence state are independent. `Accepted` means the design
is the current baseline; it does not mean the design has been proved or
deployed. Advance evidence only when its named oracle succeeds:

```text
Designed -> Reference-proven -> Implementation-tested -> Production-verified
```

Do not imply that every decision must reach every evidence state. A design with
no production deployment may correctly stop at `Implementation-tested`.

Once accepted, preserve the record. If context or evidence changes the choice,
create a new ADR, mark the old one `Superseded`, and link both directions. Never
edit old reasoning into agreement with the replacement.

## Retrieval and reuse

Before changing a recorded boundary, retrieve the governing ADR and its
dependencies, then reverify drift-prone context and evidence. Keep ADRs
project-local. Promote a mechanism to a cross-project blueprint or skill only
after repeated verified use shows that its invariant and workload generalize.
