---
name: vector
description: Modular vector lending indexer architecture context for this workspace. Use when the user tags @vector or $vector, asks about the vector architecture, layered vector indexer, Aave V3 Rust indexer reusable blueprint, multi-protocol lending indexer platform, protocol-line isolation, dedicated RPC keys, slot registries, L0/L1/L2/L3/L4 vectors, live risk computation, vector checkpoints, recovery, or memory-first lending risk APIs.
---

# Vector

Use this skill to reason about the finalized modular vector-based lending indexer
architecture for `/home/ubuntu/data-v2`.

The skill packages the target platform design, the reusable protocol-line
blueprint, the vector data model, storage/recovery rules, serving contracts,
protocol adapter guidance, performance targets, roadmap, and Aave V3 runtime
alignment.

## Core Invariants

Keep these rules in mind before making architecture or code changes:

```text
Protocol lines are isolated.
Each protocol owns its process/container, event ingest, dedicated RPC key, WAL,
watermark, ClickHouse namespace, checkpoints, and recovery loop.

Shared code is allowed; shared live protocol state is not.

Canonical account units do not silently accrue.
Aave stores scaled balances, Morpho stores shares, Compound-style systems store
principal-like units, and vault systems store shares.

Live risk is memory-first.
ClickHouse is for facts, witnesses, snapshots, recovery, history, audits,
analytics, and compatibility/debug views.

L0 consumes published L1 protocol vectors only.
L0 must not call protocol RPC or inspect protocol internals.

Required same-block witness failures block protocol watermark advancement.

Do not rewrite every account-risk row on every index-only block.
```

## Reference Selection

Read only the reference files needed for the current request.

```text
references/OVERVIEW.md
  Read first when you need the document map or project vocabulary.

references/00_UNIFIED_ARCHITECTURE.md
  Read for the complete platform + protocol-line architecture map.

references/01_PLATFORM_ARCHITECTURE.md
  Read for L0 aggregation, process isolation, deployment boundaries, shared
  libraries, platform observability, and failure domains.

references/02_PROTOCOL_LINE_BLUEPRINT.md
  Read when designing or modifying a protocol indexer runtime. This is the
  generalized Aave V3 Rust indexer runtime.

references/03_VECTOR_DATA_MODEL.md
  Read for slot registries, L2/L3/L4 vectors, memory layout, exposure indexes,
  dirty planning, risk kernels, hashing, and checkpoint encodings.

references/04_INGEST_AND_WITNESSES.md
  Read for event ingestion, dedicated RPC keys, same-block Multicall witnesses,
  witness fact contracts, required vs optional witnesses, and failure handling.

references/05_STORAGE_CHECKPOINTS_RECOVERY.md
  Read for ClickHouse table groups, WAL, vector snapshots, snapshot manifests,
  checkpoint cadence, recovery, and watermark contracts.

references/06_SERVING_AND_APIS.md
  Read for memory-first live APIs, L1/L0 publication, serving indexes, fallback
  behavior, and compatibility mode.

references/07_PROTOCOL_ADAPTER_GUIDE.md
  Read when adding or reviewing Aave/Spark, Morpho, Fluid, Euler, Compound/Comet,
  or vault adapters.

references/08_PERFORMANCE_AND_CAPACITY.md
  Read for Monte Carlo stress-test commands, 8-thread vs 16-thread results,
  budgets, bottlenecks, and tuning knobs.

references/09_IMPLEMENTATION_ROADMAP.md
  Read when planning implementation phases or migration from current Aave runtime
  to the unified platform.

references/10_AAVE_V3_RUNTIME_ALIGNMENT.md
  Read when mapping the current Rust Aave V3 indexer modules and tables to the
  finalized architecture.
```

## Working Rules

When asked to design, review, or implement vector architecture work:

1. State whether the change belongs to the platform, a protocol line, a protocol
   adapter, the shared vector runtime, storage/recovery, serving/API, or L0.
2. Preserve protocol isolation unless the user explicitly asks for a temporary
   experiment.
3. Treat the Aave V3 Rust indexer as the first protocol-line implementation, not
   as the whole platform.
4. Keep Aave-specific logic inside the Aave adapter or risk kernel.
5. Require durable slot registry snapshots for deterministic vector recovery.
6. Require snapshot manifests when mixing vector checkpoints, facts, witnesses,
   and hashes.
7. Prefer memory vectors for live current risk and ClickHouse for historical or
   audit queries.
8. Check performance claims against the Monte Carlo numbers before asserting a
   bottleneck.

## Common Interpretations

```text
"protocol adapter"
  Protocol-specific event decoding, witness planning/decoding, canonical unit
  conversion, and risk math.

"protocol line"
  One isolated runtime for one protocol deployment or family, with its own RPC
  key, WAL, watermarks, vector state, and checkpoints.

"platform"
  The multi-protocol layer around protocol lines: shared libraries, L1 contracts,
  L0 aggregation, global status, and cross-protocol APIs.

"vector runtime"
  Shared primitives for slot registries, vector stores, dirty planning,
  recompute scheduling, hashing, checkpointing, and publication.
```
