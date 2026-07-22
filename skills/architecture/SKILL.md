---
name: architecture
description: Design a bounded architecture slice before implementation.
---

# architecture

Use this skill for changes that cross boundaries or affect APIs, schemas,
storage, migrations, dependencies, security, money, concurrency, or data loss.

Define:

- subsystem responsibilities and contracts
- latency, memory, storage, network, test-runtime, and LOC budgets
- trust boundaries and failure recovery
- compatibility, migration, rollback, and observability
- independently deployable implementation slices
- verification for each slice

Split work before the approval gate when a single slice would exceed it.
