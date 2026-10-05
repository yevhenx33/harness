---
name: solve-from-first-principles
description: Solve ambiguous, difficult, or optimization-heavy problems through first-principles decomposition, requirement deletion, simplification, Pareto analysis, competing hypotheses, progressive falsification, and end-to-end validation. Use when diagnosing a hard problem, seeking a performance or cost breakthrough, redesigning a system or process, planning experiments, comparing approaches under several objectives, or separating theoretical promise from real operational results.
---

# Solve from First Principles

## Aim

Find the smallest complete, verifiable, and Pareto-efficient solution to the real problem.

```text
own -> contract -> question -> delete -> simplify -> find critical few
    -> generate hypotheses -> falsify -> build Pareto frontier
    -> accelerate -> automate -> validate externally -> repeat
```

Match the user's mode. Keep inspection, mapping, research, and discussion read-only. Implement or operate only when authorized.

## Operating Rules

- Start from observable outcomes, invariants, and physical or informational limits.
- Use first principles to determine truth. Use analogies only to generate hypotheses.
- Question every requirement until its owner, purpose, and evidence are known.
- Delete before simplifying, simplify before accelerating, and automate last.
- Move fast through small, isolated, reversible experiments without weakening correctness.
- Reopen representations, boundaries, assumptions, and architectures when local tuning stalls.
- Maintain independent hypothesis families, not many variants of one idea.
- Own proof, integration, external acceptance, rollback, and recovery.
- Label results accurately as theoretical, experimental, integrated, or operational.
- Record bounded negative results so failed work narrows future search.

## 1. Define the Contract

Write down:

- **Target:** exact observable result, magnitude, and deadline.
- **Invariant:** behavior that must remain true.
- **Baseline:** current result, workload, environment, and measurement method.
- **Constraints:** separate hard limits from assumptions and historical implementation choices.
- **Objectives:** dimensions, units, direction, and priority.
- **Acceptance boundary:** real user, judge, consumer, or external system that decides success.
- **Failure budget:** tolerable failure, reversibility, and recovery requirement.

Do not optimize an undefined metric or silently change the objective.

## 2. Build the Causal and Cost Model

Map only what is needed to explain the outcome:

- inputs, outputs, state, and transformations
- dependencies and critical path
- work and capacity by resource
- correctness, trust, and compatibility boundaries
- current bottleneck and measurable lower bounds

For resource systems, calculate:

```text
resource_floor[r] = ceil(required_work[r] / capacity_per_step[r])
lower_bound = max(resource floors, dependency critical-path floor)
```

Separate total work from latency. Work removed from an idle resource may not change the result. Remeasure after every win because the bottleneck can migrate.

## 3. Question, Delete, and Simplify

For each requirement ask: who owns it, what need it protects, what proves it is current, whether it is an invariant or artifact, and how to test removing it safely. Ask the user before changing their intended outcome, safety boundary, legal duty, or an irreversible decision.

Delete first:

- unused outputs and requirements
- redundant work, conversions, state, copies, layers, synchronization, and validation
- legacy boundaries with no independent value

Simplify what remains:

- expose needed information directly through a better representation
- fuse adjacent transformations
- reuse existing mechanisms
- specialize hot paths with a fallback when needed
- exchange compute, storage, time, or network work deliberately
- precompute, cache, aggregate, materialize, or route only when total cost falls

Include lookup, invalidation, memory, source size, generation, deployment, recovery, and external-limit costs.

## 4. Find the Critical Few

Treat the 80/20 rule as a question, not a fixed ratio:

1. Rank contributors to cost, latency, defects, risk, or uncertainty.
2. Measure cumulative contribution.
3. Find the smallest set responsible for most of the outcome.
4. Focus there unless a cheaper structural deletion exists elsewhere.

Do not confuse:

- **Pareto concentration:** the few causes that dominate the outcome.
- **Pareto frontier:** solutions not dominated across multiple objectives.

## 5. Generate Competing Hypotheses

Cover independent mechanism families: deletion; representation; algorithm or semantics; boundary fusion; compute-storage exchange; caching or preprocessing; scheduling, batching, or concurrency; specialization or routing; approximation when permitted; and packaging or external-interface changes.

Give each candidate a hypothesis card:

| Field | Content |
|---|---|
| Target | Bottleneck or uncertainty |
| Mechanism | Why it should change the outcome |
| Effect | Metric and expected magnitude |
| Invariants | Preserved, changed, or uncertain |
| Cheap falsifier | Smallest test likely to kill it |
| Proof burden | Evidence required for promotion |
| Total cost | Runtime, storage, complexity, risk, and integration |
| Rollback | Safe reversal path |

Prefer a diverse portfolio over tuning around one local optimum.

## 6. Falsify Progressively

Spend evidence in increasing order of cost:

1. analytical check: units, bounds, dependency math, conservation rule, or counterexample
2. toy model preserving the claimed mechanism
3. representative simulation against a fixed baseline
4. adversarial, randomized, boundary, and failure cases
5. proof, certificate, exhaustive finite check, or independent replication as appropriate
6. sidecar or A/B run through the same real engine and comparator
7. real external acceptance test

Kill weak candidates early. For exact transformations, one mismatch rejects a candidate; randomized tests are evidence, not universal proof. State the exact scope and assumptions of every negative proof or bounded search.

## 7. Build the Pareto Frontier

Choose metrics before comparing candidates: correctness confidence, outcome, latency, compute, memory, storage, network, complexity, delivery time, compatibility, failure impact, and reversibility.

A dominates B when A is no worse on every chosen dimension and strictly better on at least one. Remove invalid and dominated candidates. Select the knee point that buys most of the desired outcome before marginal cost or risk rises sharply. State the constraints or weights behind the choice.

## 8. Accelerate, Automate, and Promote

Accelerate only the active bottleneck of a surviving design. Hold workload and environment constant, measure end to end, report sample size and distribution when noise matters, and inspect bottleneck migration. A cheaper component with no end-to-end gain is not a performance win.

Automate only the simplified, validated process. Good targets include experiment generation, invariant checks, benchmark replay, certificate verification, frontier refresh, deployment, rollback, and monitoring. Fail closed for correctness, money, security, destructive actions, and external acceptance.

Promote claims through these evidence levels:

| Level | Meaning |
|---|---|
| Idea | Plausible mechanism only |
| Analytical | Model, bound, or causal argument supports it |
| Experimental | Toy, simulation, or benchmark supports it |
| Verified | Required proof or robust replication passed |
| Integrated | Correct in the real engine against the same baseline |
| Operational | Accepted by the real external system |

Never report projected savings as measured savings or integrated success as operational success.

## Pivot, Stop, and Learn

- After repeated failures for the same reason, change hypothesis family.
- If a measured floor exceeds the target, question a requirement, representation, boundary, or architecture.
- When the bottleneck migrates, stop optimizing the old one.
- Archive dominated candidates unless they preserve unique optionality.
- Keep harness-only wins labeled experimental.
- Treat external packaging failure as unfinished work.
- Stop when the contract passes and further gains cost more than their value.

For each meaningful failure, record the hypothesis, exact test scope, conditions, result, what was ruled out, what remains possible, and what would justify reopening it.

## Response Contract

For substantial work, report:

1. problem contract and baseline
2. causal model, bottleneck, and lower bounds
3. critical few
4. hypothesis portfolio and cheap falsifiers
5. non-dominated frontier and chosen knee
6. evidence separated by level
7. decision: proceed, pivot, stop, or request a material choice
8. smallest next reversible experiment with pass and fail criteria
9. bounded negative results

Keep output proportional. Apply the same logic to small problems without manufacturing ceremony.
