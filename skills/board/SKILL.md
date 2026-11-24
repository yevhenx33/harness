---
name: board
description: Turn a set of problems into an owner-based task table for the user to dispatch. Use when invoked as $board or asked to organize problems into independently verifiable assignments.
---

# Board

Convert the supplied problems and evidence into one Markdown table. Planning is
read-only by default. The user dispatches the rows and coordinates integration.

| Problem | Blocker | Invariants | Expected Outcome | Budget (latency, disk, memory, CPU) | Owner | Write Scope | Status |
|---|---|---|---|---|---|---|

- **Problem** gives a stable task ID and states the observed gap. Use one row per
  independently verifiable task and preserve existing IDs when updating a board.
- **Blocker** names what must clear before work or verification can proceed.
  Reference prerequisite row IDs or name the shared resource or decision.
- **Invariants** state what must hold through the affected transitions and failures.
- **Expected Outcome** gives the observable result and acceptance check. Name
  the consumer and its check when acceptance crosses a module boundary.
- **Budget** separates limits, targets, and measured baselines. Use TBD for
  relevant unknowns.
- **Owner** names the module responsible for the behavior or state. Include the
  dedicated Codex chat once the user assigns it.
- **Write Scope** bounds writable paths, contracts, and runtime state. Record
  the dedicated branch and worktree once assigned.
- **Status** records the current state and links to evidence when available.
  Use Proposed, Ready, In progress, Blocked, Ready for integration, or Integrated.

Group rows by owner. After the table, call out required ordering and safe
parallel work. Rows can overlap when prerequisites are satisfied, writes are
independent, and combined resource use fits the shared budgets. Account for
shared dependencies such as live services, ports, locks, and provider quotas.

The user assigns each implementation task to a dedicated Codex chat with its
own branch and worktree. The assigned agent works within its row's scope and
records the action, result, evidence, budget used, and next gate after each step.
Where concurrent row edits are unsupported, the agent gives the update to the
user for the shared artifact. Surface scope conflicts and failed gates in the
row. Worktree verification earns Ready for integration with the exact candidate
revision or diff and check results. The user decides what moves to the canonical
branch and when. Check the combined candidate against applicable repository
and consumer gates before integration. Use local checks where permitted.
