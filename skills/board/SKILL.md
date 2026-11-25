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

Group rows by owner. After the table, add a numbered **Dispatch order** with
one short line per step. Start each step only after all rows in the previous
step meet their handoff or acceptance criteria. Group row IDs in one step
only when prerequisites, write scopes, and shared budgets allow parallel work.
Keep specific gates in the table's Blocker column.

### Example dispatch order

Start each step after all tasks in the previous step are done. The table lists
any additional gates.

1. **R-01 + A-01 in parallel** — implement recovery and prepare the Aave check.
2. **A-02** — verify recovery through Aave.
3. **R-02** — retire the old recovery path.

The user assigns each implementation task to a dedicated Codex chat with its
own branch and worktree. The assigned agent works within its row's scope and
records the action, result, evidence, budget used, and next gate after each step.
Where concurrent row edits are unsupported, the agent gives the update to the
user for the shared artifact. Surface scope conflicts and failed gates in the
row. Worktree verification earns Ready for integration with the exact candidate
revision or diff and check results. The user decides what moves to the canonical
branch and when. Check the combined candidate against applicable repository
and consumer gates before integration. Use local checks where permitted.
