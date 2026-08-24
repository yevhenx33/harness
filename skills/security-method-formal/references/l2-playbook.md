# L2 Playbook: Formal-Methods Review

## Tools

- Prefer the repository's pinned model checker, SMT solver, proof assistant, symbolic executor, specifications, and reproducible commands.
- Use generated traces and solver models to inspect counterexamples and vacuity.
- Do not substitute a new formalism or install a toolchain without authority; first review the existing proof boundary.

## Practices

Maintain a claim ledger: property, quantifiers, initial states, transitions, assumptions, abstractions, trusted axioms, checked bounds, tool/version, result, implementation correspondence, and exclusions.

## Protocol

1. Pin target, model, toolchain, and proof obligation.
2. Check definitions, satisfiability, and non-vacuity with small witness properties.
3. Map model elements to code, configuration, and environment.
4. Reproduce the result and inspect counterexamples or proof dependencies.
5. Perturb assumptions and bounds to expose hidden dependence.
6. State the narrowest justified claim and every unmodeled risk.

## Evidence and falsification

A supported result needs reproducible artifacts, consistent assumptions, non-vacuity evidence, and a reviewed correspondence boundary. Falsify it with a reachable counterexample, inconsistent axiom, missing transition, incorrect abstraction, toolchain drift, or implementation behavior outside the model.

## Dynamic boundary and failure behavior

Bound solver time, memory, state space, and retained artifacts. A timeout or unknown result is inconclusive, never a proof. Stop if the model or generated artifacts could expose secrets or if computation exceeds the admitted envelope.
