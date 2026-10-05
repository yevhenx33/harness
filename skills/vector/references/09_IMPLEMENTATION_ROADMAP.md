# Implementation Roadmap

This roadmap converges the current Aave V3 Rust indexer and the target modular
multi-protocol vector platform.

## Current Baseline

The Aave V3 Rust indexer already provides a working protocol-line prototype:

```text
confirmed block/range processing
Hypersync or eth_getLogs event discovery
same-block Multicall3 witnesses
in-memory RiskEngine
reserve/account/exposure/risk state
dirty account recompute
account_risk_vector_snapshots
WAL and watermark safety
async ClickHouse writer
rollups
direct RPC validation
continuous runtime scripts
```

Relevant files:

```text
backend/aave-indexer-rs/src/live.rs
backend/aave-indexer-rs/src/risk.rs
backend/aave-indexer-rs/src/state.rs
backend/aave-indexer-rs/src/schema.rs
backend/aave-indexer-rs/src/wal.rs
backend/aave-indexer-rs/src/writer.rs
backend/aave-indexer-rs/src/events.rs
```

## Target End State

```text
shared vector runtime library
isolated protocol-line binaries
adapter interfaces for events/witnesses/risk kernels
durable slot registry snapshots
full vector checkpoint manifests
published L1 protocol-vector contract
L0 cross-protocol aggregator
memory-first live API
consistent ClickHouse table groups across protocol lines
benchmark and validation harnesses
```

## Phase 1: Formalize Aave As Protocol Line

Goal: make the current Aave line match the blueprint without broad refactors.

Tasks:

```text
document Aave ProtocolSpec fields
name Aave witness groups required vs optional
add durable slot registry snapshot design
add snapshot manifest rows around existing account_risk_vector_snapshots
publish explicit L1 protocol vector from current aggregate
add vector epoch metadata to status output
make direct validation report reference vector epoch
```

Acceptance:

```text
Aave still runs in canonical continuous mode
existing correctness checks pass
existing account_risk_vector_snapshots still write
new manifest can validate current risk vector hash
L1 protocol vector can be read by a test L0 process
```

## Phase 2: Extract Shared Interfaces

Goal: separate protocol-specific adapter code from reusable runtime mechanics.

Tasks:

```text
define ProtocolSpec trait
define EventDecoder trait
define WitnessPlanner trait
define WitnessDecoder trait
define VectorMutation enum
define RiskKernel trait
define ProtocolVectorPublisher
define CheckpointCodec
define SnapshotManifest builder
```

Do this conservatively:

```text
extract interfaces next to Aave implementation
keep Aave behavior unchanged
avoid premature generic abstractions for protocol math
write adapter-level tests around Aave outputs
```

Acceptance:

```text
Aave adapter implements the traits
Aave live runtime output is unchanged
tests prove witness/mutation equivalence
no regression in benchmark timings
```

## Phase 3: Shared Vector Runtime

Goal: move reusable vector storage, dirty planning, scheduling, hashing, and
checkpointing behind stable APIs.

Tasks:

```text
implement SlotRegistry
implement AccountVectorStore
implement PositionVectorStore
implement AssetVectorStore
implement MarketVectorStore
implement RiskVectorStore
implement ExposureIndex
implement DirtyPlanner
implement parallel RiskScheduler
implement layer hash functions
implement checkpoint encode/decode
```

Acceptance:

```text
slot ids survive restart
checkpoint decode recreates same hashes
dirty planning matches Aave current behavior
sparse benchmark stays under target
dense benchmark stays under target
```

## Phase 4: L1 Publication And L0 Aggregator

Goal: publish protocol vectors and aggregate them without protocol coupling.

Tasks:

```text
define L1ProtocolVector DTO
write protocol_vector_latest table or memory publisher
add Aave L1 publication
implement L0 aggregator process/library
compute cross_protocol_vector
write cross_protocol_vector_snapshots
add stale protocol handling
add platform status view
```

Acceptance:

```text
L0 reads Aave L1 vector without Aave internals
L0 works with one protocol and mocked additional protocols
stale protocol state is reported, not fatal
L0 does not call RPC
L0 snapshot hashes source L1 vectors
```

## Phase 5: Memory-First API Path

Goal: stop treating ClickHouse as live risk serving path.

Tasks:

```text
define ProtocolMemoryEpoch read view
define CrossProtocolMemoryEpoch read view
add account lookup by slot registry
add protocol overview from L1 vector
add reserve/market pages from memory vectors and indexes
add cross-protocol dashboard from L0 vector
include vector epoch metadata in responses
keep ClickHouse for history and compatibility
```

Acceptance:

```text
current account risk endpoint can serve from memory
protocol overview can serve from memory
cross-protocol overview can serve from L0 memory
historical charts still read ClickHouse rollups
parity checks compare memory vs current rows during migration
```

## Phase 6: Add Second Protocol Line

Goal: prove the architecture is not Aave-only.

Recommended candidates:

```text
Spark:
  closest Aave-like adapter

Morpho:
  proves L3 market-id/share model
```

For Spark:

```text
reuse Aave-like canonical unit model
replace deployment constants and reserve/oracle config
validate scaled balances
publish separate L1 vector
```

For Morpho:

```text
define market id slot registry
store shares as canonical account units
fetch market totals and oracle witnesses
implement share-to-assets risk kernel
publish separate L1 vector
```

Acceptance:

```text
second protocol has its own RPC key and WAL
second protocol does not share Aave live state
L0 aggregates Aave plus second protocol
failure in one line does not block the other
```

## Phase 7: Complete Storage Convergence

Goal: make every protocol line follow the same durable table groups.

Tasks:

```text
standardize table names
add slot_registry_snapshots
add account_vector_snapshots
add position_vector_snapshots
add asset_reserve_vector_snapshots
add market_vector_snapshots
add protocol_vector_snapshots
add snapshot_manifests
add recovery validation status
```

Acceptance:

```text
all protocol lines can recover from latest manifest
all protocol lines can rebuild from facts/witnesses
manifest hashes prevent stale layer mixing
historical rollups reference source vector hashes
```

## Phase 8: Operational Hardening

Tasks:

```text
per-protocol dashboards
per-protocol RPC budget alerts
watermark lag alerts
witness failure alerts
checkpoint verification alerts
L0 stale protocol alerts
benchmark CI or scheduled benchmark runs
recovery drills
schema migration runbooks
```

Acceptance:

```text
operators can identify which protocol line is degraded
one-line recovery does not require stopping the platform
checkpoint hash failures have documented fallback
RPC key exhaustion is visible before live failure
```

## Migration Rules

Do:

```text
keep Aave working while extracting shared pieces
add manifests around existing snapshots before changing recovery
validate memory vs current rows before API cutover
add one protocol line at a time
keep old current-risk tables during migration
prefer compatibility layers over risky rewrites
```

Do not:

```text
merge all protocols into one ingestion process
change Aave canonical units during interface extraction
remove current rows before memory API parity is proven
make L0 aware of Aave-specific internals
share RPC keys across protocols
```

## Definition Of Done

The unified architecture is implemented when:

```text
Aave runs as one protocol line
at least one non-Aave protocol runs as another isolated line
each line has a dedicated RPC key and WAL
each line publishes L1 vectors
L0 aggregates L1 vectors only
current APIs can serve from memory vectors
ClickHouse stores facts, witnesses, snapshots, manifests, and rollups
checkpoint recovery works
direct validation exists per protocol
Monte Carlo targets are met
```
