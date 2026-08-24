---
name: security-method-diff
description: Use explicitly to review a pull request, commit, branch range, or working-tree patch for security-relevant behavior changes. Do not use as a substitute for a full repository audit.
---

# Security Diff Review

Use this method only after an explicit invocation.

## Selection boundary

Best fit: the authoritative target is a Git change set and the outcome is a security assessment of behavior introduced, removed, or made reachable by that change.

Reject this method for an unbounded repository audit, design-only proposal, or known bug-class sweep. A diff may require reading unchanged supporting code, but that does not widen the claim to unchanged repository coverage.

## Inputs and authority

Resolve the exact base and head revisions or working-tree state. Record generated, renamed, deleted, vendored, configuration, schema, and dependency files in scope. Review is read-only unless the user separately authorizes a fix.

Expected cost: low; work scales with the diff plus the minimum unchanged owner and consumer context needed to decide changed behavior.

## Workflow

1. Freeze the exact diff and record both revisions.
2. Explain the security-relevant behavior change before judging individual lines.
3. Follow changed inputs, authorization, state, serialization, error, concurrency, and dependency behavior into unchanged owners and consumers.
4. Validate candidate regressions against both old and new behavior.
5. Report findings attributable to the change and separate pre-existing observations.

Read [references/l2-playbook.md](references/l2-playbook.md) to run the method. Read [references/l3-lineage.md](references/l3-lineage.md) only for history, limitations, or method comparison. Format results with `../security-review-router/references/review-contract.md`.

## Stop conditions

Stop if the base is ambiguous, the patch changes while under review, generated artifacts cannot be traced to their source, or required supporting code is unavailable. Do not invent a stable range.
