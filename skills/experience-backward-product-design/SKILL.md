---
name: experience-backward-product-design
description: Design, critique, simplify, or redesign products by defining the intended user experience first and working backward to technology, interfaces, and scope. Use for product concepts, UX flows, feature prioritization, prototypes, product-quality reviews, hardware-software integration decisions, or requests invoking Steve Jobs-style simplicity, taste, delight, focus, or end-to-end product coherence.
---

# Experience-Backward Product Design

Use the intended user experience as the governing constraint. Make technology,
scope, and organizational boundaries serve that experience.

## Load the Blueprint

Read `/home/ubuntu/blueprints/blueprints/apple-experience-backward-product-design.md`
completely before applying this method. Load at most two related blueprints when
they materially change a decision.

## Workflow

1. **Write the experience contract.** Name the user, triggering situation,
   desired outcome, desired feeling, and what should require no explanation.
2. **Separate ends from means.** Mark each current feature, interface, workflow,
   dependency, and convention as an experience requirement or an implementation
   assumption. Challenge the assumptions.
3. **Map the whole journey.** Trace discovery, first use, core task, transitions,
   errors, recovery, and return use. Include seams between teams, systems,
   hardware, software, and external vendors.
4. **Subtract.** Remove steps, choices, controls, modes, and features that do not
   strengthen the contract. Preserve expert control or safety where invisibility
   would hide material state or risk.
5. **Resolve seams at the owning boundary.** Change or own only the layers whose
   seams materially degrade the experience. Do not equate coherence with owning
   the entire stack.
6. **Build a real end-to-end prototype.** Test the smallest working journey that
   exposes behavior, timing, transitions, and recovery. Do not validate an
   interaction with static drawings alone.
7. **Observe without coaching.** Record comprehension, completion, hesitation,
   errors, recovery, delight, and abandonment. Treat requested solutions as
   evidence about needs, not as the design specification.
8. **Critique and iterate.** Fix the largest break in the experience contract,
   then repeat. Polish micro-details only after the journey works coherently.

## Required Gates

- A new user can state what the product does and complete the core task without
  a manual or live guidance.
- Critical state, money, permissions, destructive actions, and failure modes
  remain visible even when implementation complexity is hidden.
- The prototype exercises the complete journey, including one material failure
  and recovery path.
- Every retained feature and owned layer has a named contribution to the
  experience contract.
- Observed evidence, stakeholder claims, and design inference are labeled
  separately.

## Output

Return:

1. the experience contract
2. the current journey and its largest breaks
3. deletions and challenged assumptions
4. the proposed end-to-end design
5. the smallest working prototype or experiment
6. evidence, gates, and unresolved risks

Prefer one coherent recommendation over a long feature menu. Explain what was
removed and why.

## Guardrails

- Do not use “users cannot tell us what they need” to dismiss research. Observe
  behavior and validate outcomes; avoid outsourcing invention to feature polls.
- Do not restart for aesthetic dissatisfaction alone. Restart when evidence
  shows the governing experience or architecture is wrong.
- Do not pursue delight at the expense of accessibility, correctness, safety,
  latency, or recoverability.
- Do not hide complexity that the user must understand to make an informed
  decision.
- Do not polish isolated pixels while the end-to-end journey remains broken.
