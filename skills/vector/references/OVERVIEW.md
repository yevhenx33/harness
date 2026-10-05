# Modular Vector Lending Indexer Architecture

Date: 2026-06-08

This folder is the finalized architecture package for the modular vector-based
lending indexer platform. It reconciles the implemented Aave V3 Rust indexer
blueprint with the target layered vector architecture for multi-protocol support.

The goal is a system that maintains live risk in memory for every supported
lending protocol, isolates each protocol's ingestion and RPC dependencies, and
publishes compact protocol vectors that can be aggregated into cross-protocol
risk views.

Target capacity:

```text
1M accounts
500 reserve/market slots per protocol line
block-by-block live updates
memory-first live risk serving
compact durable checkpoints
protocol-isolated RPC, WAL, and recovery
```

## Reading Order

1. `00_UNIFIED_ARCHITECTURE.md`
   - The complete platform and protocol-line map.
   - Start here if you need the whole picture.

2. `01_PLATFORM_ARCHITECTURE.md`
   - Multi-protocol platform boundaries.
   - L0 aggregator, process isolation, shared libraries, deployment model.

3. `02_PROTOCOL_LINE_BLUEPRINT.md`
   - The reusable per-protocol runtime.
   - This is the generalized version of the Aave V3 Rust indexer runtime.

4. `03_VECTOR_DATA_MODEL.md`
   - Slot registries, vector layers, memory layout, dirty planning, risk kernels.

5. `04_INGEST_AND_WITNESSES.md`
   - Event ingestion, same-block witnesses, dedicated RPC keys, failure handling.

6. `05_STORAGE_CHECKPOINTS_RECOVERY.md`
   - ClickHouse shape, WAL, vector snapshots, manifests, recovery contracts.

7. `06_SERVING_AND_APIS.md`
   - Memory-first live API, historical ClickHouse reads, L0 serving contract.

8. `07_PROTOCOL_ADAPTER_GUIDE.md`
   - How to add Aave-like, Morpho-like, Fluid-like, Euler-like, vault, or Comet
     protocol lines.

9. `08_PERFORMANCE_AND_CAPACITY.md`
   - Monte Carlo stress-test results, budgets, bottlenecks, tuning guidance.

10. `09_IMPLEMENTATION_ROADMAP.md`
    - Migration path from current Aave implementation to the unified platform.

11. `10_AAVE_V3_RUNTIME_ALIGNMENT.md`
    - Concrete mapping from the current Aave V3 Rust indexer to this blueprint.

## Source Material

These docs supersede and consolidate the target architecture described in:

- `../LAYERED_VECTOR_INDEXER_ARCHITECTURE.md`
- `../AAVE_V3_RUST_INDEXER_REUSABLE_BLUEPRINT.md`
- `../AAVE_INDEXER_RS_CONTINUOUS.md`
- `../LAYERED_VECTOR_MONTE_CARLO_STRESS_TEST.md`
- `../AAVE_V3_RISK_PROJECTION_MOCK.md`

The older docs remain useful as implementation history and benchmark notes.
This folder should be treated as the target architecture reference.

## Core Decisions

1. Protocol lines are isolated.
   Each protocol has its own process/container, event ingestion, dedicated RPC
   API key, WAL, watermarks, ClickHouse namespace, and recovery loop.

2. Shared code is allowed; shared live state is not.
   Protocol lines may link the same vector runtime library and storage helpers,
   but they do not share mutable in-memory protocol state.

3. Canonical account units are non-accruing.
   Aave stores scaled balances, Morpho stores shares, Compound-style systems
   store principal-like units, vault systems store shares. Accrued balances are
   derived from market/index/price vectors.

4. Live risk is memory-first.
   APIs serving current account, market, protocol, and cross-protocol risk read
   memory vectors first. ClickHouse is used for facts, witnesses, snapshots,
   recovery, history, audits, analytics, and compatibility views.

5. L0 consumes protocol outputs only.
   The cross-protocol aggregator does not call protocol RPC and does not inspect
   protocol internals. It consumes published L1 protocol vectors and optional
   exposure summaries.

6. Watermark advancement is gated.
   A block or range is not canonical until required witnesses, vector mutation,
   WAL, durable facts, snapshots or checkpoints, and run records agree.

7. Per-account current-risk row rewrites are not the live path.
   Compatibility/debug current rows may exist, but canonical live risk is held
   in memory vectors and compact checkpoints.

## Terminology

```text
Platform:
  The multi-protocol system around isolated protocol lines.

Protocol line:
  One isolated runtime for one lending protocol deployment or protocol family.

Adapter:
  Protocol-specific code for events, witnesses, canonical units, and risk math.

Shared vector runtime:
  Reusable library for slot registries, vector storage, dirty planning,
  recompute scheduling, hashing, checkpointing, and API publication.

L0:
  Cross-protocol vector aggregation layer.

L1:
  Protocol vector layer, one published output per protocol line.

L2:
  Asset/reserve state vectors.

L3:
  Market/product/vault/pair state vectors.

L4:
  Account/position and account-risk vectors.

Witness:
  Same-block direct chain state fetched through RPC, usually via Multicall3.

Canonical unit:
  The account unit stored as truth, before accrual/share/index conversion.
```

## Architectural North Star

```text
Protocol-specific ingestion and witnesses
  -> normalized vector mutations
  -> deterministic slot registries
  -> shared vector runtime
  -> protocol-specific risk kernel
  -> L1 protocol vector
  -> L0 cross-protocol vector
  -> memory-first live API

Durable side path:
  facts + witnesses + WAL + checkpoints + manifests + rollups
  -> ClickHouse
  -> recovery, history, audits, analytics
```
