# L2 Playbook: Variant Analysis

## Tools

- Start with `rg`, language-aware navigation, AST queries, and existing static-analysis facilities.
- Use CodeQL, Semgrep, compiler queries, or custom checks only when already available or explicitly authorized.
- Use focused tests or safe local witnesses to validate disputed candidates, not to replace root-cause reasoning.

## Practices

Maintain a search predicate with required and optional clauses. Track positive examples, negative examples, excluded scopes, query revisions, and the reason each candidate is valid or invalid.

## Protocol

1. Pin the seed and confirm its attack path and violated property.
2. Separate essential causal features from incidental syntax.
3. Search exact structure, then aliases, wrappers, generated paths, and equivalent transitions.
4. Review every candidate against the full predicate.
5. Expand only after the current query's error modes are understood.
6. Report validated variants, invalid candidates, blind spots, and query limits.

## Evidence and falsification

Each variant needs its own reachable path and consequence. Falsify it with an effective guard, safe data provenance, unreachable configuration, non-equivalent state transition, or harmless sink. Search-result count is never finding count.

## Dynamic boundary and failure behavior

Dynamic validation follows the same local, harmless, explicitly authorized boundary as the seed. Stop if a query produces an unreviewable candidate volume; narrow the predicate rather than sampling opportunistically.
