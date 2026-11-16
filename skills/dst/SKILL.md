---
name: dst
description: Use for stateful changes where correctness or progress depends on the order of events and failures, such as timeouts, retries, cancellation, concurrent writes, or restart. Design, run, or replay deterministic simulations of the affected behavior.
---

# Deterministic Simulation Testing

Apply during design as well as verification. Start with the existing task
invariants and the smallest owning component whose behavior depends on event
order. Reuse its simulator or test runner where available. Use the real logic
under test and control the sources of nondeterminism that can affect it, such as
time, event delivery, concurrent scheduling, and storage outcomes. Keep simulated
dependencies faithful to the contract being tested, including what survives
restart. A seeded generator alone cannot make uncontrolled scheduling repeatable.

- **Safety.** Name what must hold throughout the run and check it at the relevant
  transitions and observable boundaries. Use an independent expected result
  where the implementation and its checks could share the same mistake.
- **Liveness.** Define the conditions that allow progress, such as restored I/O
  or a responsive peer. Give the component that opportunity after injecting
  faults and verify it completes the required work under those conditions.
- **Bounded exploration.** Set a seed count or range, per-run event or simulated
  time limit, and cumulative wall-time and resource limits before running.
  Generate operation and fault orderings that exercise the invariant. Classify
  a capped run as incomplete unless the stated oracle has reached a verdict.
- **Exact replay.** Retain the failing seed, code revision or executable
  identity, simulator configuration, starting fixture, and replay command.
  Confirm that replay reaches the same event order and failure. If it diverges,
  report the replay as unverified and identify uncontrolled inputs if known.
  Preserve a reproducible case as a regression when it protects a material
  guarantee.

Keep simulation local or isolated and within the task's authority. Report the
seeds explored, invariants exercised, verdict, and limits of the simulation.
DST is task-specific and adds no global release gate.
