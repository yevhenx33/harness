# Architecture decisions

These records preserve historical reasons for Harness policy choices. They are
not an action log or a requirement to create an ADR for every policy release.
The policy versions and Git history record releases. Future records are for
hard-to-reverse architecture choices whose rationale a future maintainer needs.
Historical records may describe superseded policy; the current `AGENTS.md`
governs. Append observed outcome evidence without rewriting accepted rationale;
supersede a record when the choice changes.

| ID | Decision | Status | Evidence | Superseded by |
|---|---|---|---|---|
| [ADR-001](ADR-001-user-assigned-reasoning-tiers.md) | User-assigned reasoning tiers and worktree-separated frontier search | Superseded | Designed | [ADR-002](ADR-002-explicit-tiered-reasoning-skill.md) |
| [ADR-002](ADR-002-explicit-tiered-reasoning-skill.md) | Keep tiered reasoning as an explicit skill | Accepted | Designed | - |
| [ADR-003](ADR-003-task-continuity-and-proportionate-verification.md) | Preserve task continuity and scale verification to the outcome | Accepted | Designed | - |
| [ADR-004](ADR-004-respect-based-commitments-and-recovery.md) | Organize work around commitments, evidence, and breach recovery | Accepted | Designed | - |
