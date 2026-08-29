---
name: audience-expertise
description: Calibrate explanations, documents, presentations, and technical answers to a selected audience domain-expertise level from E0 to E10. Use for beginner-to-frontier variants, audience-specific rewriting, or explicit expertise calibration. Do not use merely to change tone, formality, grammar, or length.
---

# Audience Expertise

Adapt content to what the target audience can already understand while preserving
the meaning, evidence status, uncertainty, constraints, and decision consequences.

## Preserve the invariant

Change presentation depth, never claim truth. At every level:

- keep material facts, dates, quantities, assumptions, risks, and limitations;
- keep `verified`, `measured`, `designed`, `hypothesis`, `target`, `unknown`, and
  similar evidence states distinct;
- explain a necessary technical term instead of replacing it with an inaccurate
  simplification;
- treat an analogy as a learning aid, not evidence;
- never equate expertise with intelligence, age, seniority, or worth.

An advanced audience may receive more compression, but compression must not
remove premises needed to evaluate the conclusion.

## Resolve the communication contract

Collect or infer:

- `expertise`: exactly one level from E0 through E10;
- `audience`: who will receive the communication;
- `goal`: what they should understand, decide, or do;
- `domain`: the subject in which expertise is being measured;
- `format` and `length`: independent of expertise;
- `prior_context`: facts or concepts the audience is known to share;
- `required_claims`: content whose meaning or qualification must survive.

An explicit user-selected level is authoritative. If the user supplies only an
audience, infer the nearest level and state the assumption only when material.
Do not infer domain expertise from a job title alone. An executive can be E1 in
one domain and E9 in another.

## Apply the explicit scale

### E0 - First contact

- **Assume:** no exposure to the domain or its vocabulary.
- **Provide:** the concrete purpose first, ordinary adult language, one mental
  model at a time, definitions for every domain term, and one clear example.
- **Avoid:** unexplained acronyms, notation, implementation detail, and multiple
  branching edge cases unless essential to correctness.
- **Target outcome:** the audience can state what it is, why it matters, and one
  important limitation.

### E1 - Oriented beginner

- **Assume:** recognition of the topic, but no reliable model of how it works.
- **Provide:** the main actors, their roles, a short cause-and-effect sequence,
  one familiar contrast, and one normal limitation.
- **Avoid:** relying on domain terms before defining them.
- **Target outcome:** the audience can distinguish the main components and
  describe the basic flow.

### E2 - Foundational learner

- **Assume:** familiarity with basic terms, but not the mechanism.
- **Provide:** the standard input-to-outcome path, why each major step exists, a
  representative example, and the most common misconception.
- **Avoid:** notation that does not materially improve understanding.
- **Target outcome:** the audience can follow a standard example and explain why
  its result occurs.

### E3 - Informed beginner

- **Assume:** understanding of the overview and core vocabulary, with little
  practical experience.
- **Provide:** the complete normal path, first-order tradeoffs and dependencies,
  the most common failure condition, and simple notation or diagrams if useful.
- **Avoid:** mixing required rules with optional conventions.
- **Target outcome:** the audience can ask informed questions or perform a basic
  task with guidance.

### E4 - Junior practitioner

- **Assume:** training or limited hands-on experience.
- **Provide:** standard terminology, inputs, outputs, ownership, normal workflow,
  common failures, basic recovery, and a concrete operational example.
- **Avoid:** elementary definitions unless the context shows a gap.
- **Target outcome:** the audience can perform a routine task with a checklist and
  recognize common errors.

### E5 - Working practitioner

- **Assume:** regular work in the domain and command of standard workflows.
- **Provide:** mechanisms, assumptions, choices, tradeoffs, validation criteria,
  material failure paths, and context-specific judgment.
- **Avoid:** introductory history and routine 101-level explanation.
- **Target outcome:** the audience can independently choose among standard
  options and verify the result.

### E6 - Experienced practitioner

- **Assume:** substantial operational experience and the ability to evaluate
  ordinary implementations.
- **Provide:** edge cases, integrations, failure containment, recovery,
  performance consequences, and comparison of plausible alternatives.
- **Avoid:** repeating standard explanations except to establish a needed premise.
- **Target outcome:** the audience can review, troubleshoot, or improve a
  non-trivial implementation.

### E7 - Senior specialist

- **Assume:** deep domain fluency and ownership of consequential decisions.
- **Provide:** the primary invariant, ownership boundaries, failure domains,
  non-obvious evidence, second-order effects, and decision-relevant alternatives.
- **Avoid:** tutorials and decorative jargon.
- **Target outcome:** the audience can make or review an architectural,
  operational, or strategic decision.

### E8 - Domain expert

- **Assume:** mastery of conventional knowledge, standard patterns, and major
  debates.
- **Provide:** precise deltas from accepted practice, boundary conditions,
  disputed assumptions, second-order effects, and empirical or source-level
  support for non-obvious claims.
- **Avoid:** reteaching consensus except to establish an exact contrast.
- **Target outcome:** the audience can judge whether a novel approach is correct,
  useful, and materially different.

### E9 - Authority or researcher

- **Assume:** the ability to create, audit, or extend domain knowledge.
- **Provide:** thesis, method, assumptions, derivations, falsification conditions,
  counterexamples, uncertainty, reproducibility details, and relation to current
  methods or literature.
- **Avoid:** collapsing observation, inference, hypothesis, and forecast.
- **Target outcome:** the audience can reproduce, falsify, defend, or extend the
  claim.

### E10 - Frontier peer or co-designer

- **Assume:** the ability to advance the field and shared domain vocabulary, but
  never unstated project context.
- **Provide:** only the new contribution, decisive reasoning, necessary premises,
  hidden assumptions, adversarial counterarguments, unsolved constraints,
  irreducible uncertainty, and research gaps.
- **Avoid:** ritual pedagogy, repeated consensus, and confidence unsupported by
  evidence.
- **Target outcome:** the audience can co-design, rigorously challenge, or advance
  the work.

## Handle mixed audiences

Do not average different audiences into an artificial middle level. Use one of:

1. a shared explanation at the lowest level needed for comprehension, with
   optional expert callouts;
2. a primary-audience decision summary followed by a higher-level appendix;
3. separate, explicitly labeled variants when audiences have different goals.

For example: `E3 main explanation; E6 implementation notes; E9 research
appendix`.

## Calibrate the result

Before returning the content, verify:

1. The answer assumes only knowledge available at the selected level.
2. Every necessary but unfamiliar term is explained.
3. No fact, qualification, or uncertainty was weakened.
4. The mechanism is deep enough for the audience's intended action.
5. The audience can tell what to understand, decide, or do next.

Do not announce the level mechanically. State it when the user requested
multiple variants, when the level was inferred and material, or when the label
helps the deliverable.
