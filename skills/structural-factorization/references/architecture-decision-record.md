# Decision records for durable architecture

A record exists to prevent a future maintainer from repeating a consequential
mistake. It preserves the causal reason for a durable choice, not a timeline of
work performed. Git and the change review already record actions.

## When to write one

Use the owning repository's convention. Write a record only when both apply:

1. The choice is hard to reverse and establishes or changes a durable owner,
   public contract, persistent representation, security or authority boundary,
   ordering rule, recovery model, or implementation freeze.
2. A future maintainer could plausibly choose wrongly without the governing
   constraint, rejected alternative, and revisit condition.

A rejected option may deserve a record when its later reintroduction would break
the same invariant. Discussion, a routine local choice, a policy release, or a
completed task does not by itself meet this test. Record task choices and
outcomes in the task or change review.

## Smallest useful record

Follow an existing project format. Otherwise use
`docs/decisions/ADR-NNN-short-title.md` and link it from
`docs/decisions/README.md`:

```markdown
# Decision: Short title

- Status: Proposed | Accepted | Superseded | Rejected
- Evidence: Designed | Reference-proven | Implementation-tested | Production-verified
- Owner and scope:
- Primary invariant and affected consumers:

## Why this choice was necessary
What constraint or observed failure forced a decision?

## Choice and alternatives
What mechanism was chosen, what plausible option was rejected, and why?

## Consequence and recovery
What cost or limitation is accepted? How can the choice be changed safely?

## Evidence and revisit trigger
What direct evidence supports the current state? What observation would reopen it?
```

Use only fields that clarify the decision. Do not turn a record into a
specification, test plan, runtime receipt, or proof by assertion. `Accepted`
means a design baseline; the evidence field states what has actually been
observed.

## Later outcomes

Append dated outcome evidence and links without rewriting the original reason.
A good or bad outcome is a claim about observed behavior, not about the number
of actions completed. If the choice changes, create a superseding record and
preserve the earlier reasoning. Promote a transferable mechanism or anti-pattern
to memory or the blueprint library only when evidence supports reuse.
