# Aave V3 Runtime Alignment

This document maps the current Aave V3 Rust indexer to the finalized vector
architecture. It exists so agents can understand what is already implemented and
what still needs to be generalized.

## Summary

The Aave V3 Rust indexer is the first concrete protocol line. It already proves:

```text
isolated protocol runtime
event discovery with fallback
same-block Multicall3 witnesses
in-memory vector risk engine
dirty account recompute
compact risk vector snapshots
WAL and watermark safety
ClickHouse durable facts/current rows/rollups
direct RPC validation
```

It is not yet the full multi-protocol platform because it still needs:

```text
durable slot registry snapshots
generalized adapter traits
full vector snapshot manifests
published L1 protocol-vector contract
L0 cross-protocol aggregator
memory-first public API path
consistent table groups for all future protocols
```

## Current Module Map

```text
backend/aave-indexer-rs/src/main.rs
  CLI entry point.
  Separates schema setup, bootstrap, live indexing, check, and status.

backend/aave-indexer-rs/src/lib.rs
  Crate module boundary.

backend/aave-indexer-rs/src/live.rs
  Live runtime loop.
  Confirmed head, watermark, event fetch, witnesses, vector updates,
  risk-vector windows, WAL commits, writer calls, run metrics.

backend/aave-indexer-rs/src/events.rs
  Aave event topic and log decoding.

backend/aave-indexer-rs/src/risk.rs
  In-memory RiskEngine.
  Reserves, accounts, exposure indexes, risk projections, aggregate,
  deterministic hashes, dirty recompute.

backend/aave-indexer-rs/src/state.rs
  Snapshot/bootstrap loading from ClickHouse.
  Current projection row persistence and rollup writes.

backend/aave-indexer-rs/src/schema.rs
  Rust-owned ClickHouse schema for current vectors, current risk,
  protocol aggregate, reserve exposure, index current, snapshots, checkpoints.

backend/aave-indexer-rs/src/wal.rs
  WAL append and offset management.

backend/aave-indexer-rs/src/writer.rs
  Async ClickHouse writer.

backend/aave-indexer-rs/src/bin/aave_risk_projection_mock.rs
  Earlier Aave projection benchmark.

backend/aave-indexer-rs/src/bin/layered_vector_monte_carlo.rs
  Multi-protocol vector architecture stress simulation.
```

## Blueprint Mapping

| Unified Blueprint | Current Aave Implementation |
| --- | --- |
| Protocol line | `aave-indexer live` runtime |
| ProtocolSpec | constants and reserve metadata embedded across schema/state/live |
| EventDecoder | `events.rs` |
| TouchedStatePlanner | decoded touched accounts/reserves in `live.rs` |
| WitnessPlanner | Multicall planning in `live.rs` |
| WitnessDecoder | Multicall return decoding in `live.rs` |
| SlotRegistry | in-memory account/reserve maps in `risk.rs`, not yet checkpointed as first-class registry |
| VectorRuntime | `RiskEngine` in `risk.rs` |
| RiskKernel | scaled balance plus reserve index math in `risk.rs` |
| DirtyPlanner | exposure index affected-account recompute in `risk.rs` |
| WAL | `wal.rs` |
| AsyncWriter | `writer.rs` |
| Current raw rows | `account_vector_current`, `account_risk_current`, `reserve_index_current`, etc. |
| Compact snapshots | `account_risk_vector_snapshots` |
| SnapshotManifest | not yet first-class |
| L1 ProtocolVector | mostly `protocol_risk_current` / aggregate, not yet full published contract |
| L0 Aggregator | not yet implemented |
| Direct validation | `check` command |

## Current Runtime Flow

Current Aave flow:

```text
1. Load Aave snapshot from ClickHouse.
2. Initialize RiskEngine with reserves and account positions.
3. Open WAL.
4. Start async ClickHouse writer.
5. Read confirmed head.
6. Read event_replay watermark.
7. Select next confirmed block range.
8. Fetch block metadata.
9. Fetch Aave logs through Hypersync or RPC fallback.
10. Decode event facts and touched entities.
11. WAL append range_begin.
12. Write decoded event facts.
13. For every block:
    - fetch price and reserve index witnesses
    - update reserve market data
    - recompute accounts exposed to changed reserves
    - fetch touched account scaled-balance witnesses
    - update account vectors
    - recompute touched account risk
    - push block risk vector into checkpoint window
    - write risk vector snapshot every configured window
    - WAL append block_commit
14. Optionally persist dirty current projection rows.
15. Write protocol current/rollup state.
16. Write runtime checkpoint and indexer run.
17. Advance watermark after successful writes.
```

This matches the protocol-line blueprint, with Aave-specific event and witness
logic.

## Current Aave Canonical Units

Aave canonical account units:

```text
scaled aToken balance
scaled variable debt balance
collateral enabled flag
emode category
```

Derived risk:

```text
collateral_tokens = scaled_a_token_balance * liquidity_index / RAY
debt_tokens = scaled_debt_balance * variable_borrow_index / RAY
collateral_usd = collateral_tokens * price
debt_usd = debt_tokens * price
liquidation_value_usd = collateral_usd * liquidation_threshold
health_factor = liquidation_value_usd / debt_usd
```

This is aligned with the canonical-unit rule.

## Current Aave Durable Tables

Current Rust schema includes:

```text
account_vector_current
account_risk_current
protocol_risk_current
reserve_exposure_current
reserve_index_current
account_risk_vector_snapshots
rollup_hourly
rollup_daily
rollup_weekly
runtime_checkpoints
event_only_tick_materializations
```

Analytics schema also includes raw facts and witnesses such as:

```text
event_facts
rpc_witnesses
price_rpc_witnesses
account_position_current_raw
reserve_state_current_raw
asset_price_current_raw
history_anchor_snapshots
pipeline_watermarks
indexer_runs
```

Target additions:

```text
slot_registry_snapshots
position_vector_snapshots
asset_reserve_vector_snapshots
market_vector_snapshots
protocol_vector_snapshots
snapshot_manifests
protocol_vector_latest
```

## Aave-Specific Logic To Keep Adapter-Local

```text
Pool/reserve model
aToken mapping
variable debt token mapping
ReserveData decoding
Aave event topic decoding
ReserveUsedAsCollateral events
UserEModeSet handling
getReserveNormalizedIncome
getReserveNormalizedVariableDebt
getAssetPrice
scaledBalanceOf
liquidation threshold and e-mode math
```

These should not leak into L0 or generic platform code.

## Reusable Logic To Extract

```text
confirmed range selector
event source fallback shape
Multicall batching machinery
required vs optional witness policy
async writer
WAL hash chain
watermark commit gate
slot registry primitives
vector checkpoint codec
snapshot manifest builder
dirty planner abstractions
parallel risk scheduler
protocol vector publisher
validation report shape
benchmark harness
```

Extraction should preserve current Aave behavior first, then support new
protocols.

## Alignment Gaps

### Slot registries

Current Aave has in-memory account and reserve indexes. The unified architecture
requires durable slot registry snapshots so ids are stable across checkpoint
recovery.

### Snapshot manifests

Current Aave writes risk vector snapshots. The unified architecture requires
manifests that hash dependencies:

```text
slot registry
events
witnesses
asset/reserve vectors
market vectors
account vectors
risk vectors
protocol vector
producer/schema versions
```

### L1 vector publication

Current aggregate state is close to an L1 vector but needs a formal publication
contract with:

```text
vector_epoch_id
dependency_hashes
asset exposure summaries
market exposure summaries
top risky account indexes
schema_version
staleness/status metadata
```

### L0 aggregation

Not yet implemented. L0 must consume Aave's published L1 vector without calling
Aave RPC or reading Aave internals.

### Memory-first API

Current API paths still lean on ClickHouse/current rows. Target live serving
should read memory vectors and L1/L0 vectors first, with ClickHouse used for
history and compatibility.

## Aave Migration Checklist

```text
1. Add ProtocolSpec struct for current Aave constants.
2. Label Aave witnesses required vs optional.
3. Add slot registry snapshot table and writer.
4. Add snapshot_manifest table.
5. Wrap account_risk_vector_snapshots with manifest_id.
6. Publish formal Aave L1ProtocolVector.
7. Add L1 latest table or memory publisher.
8. Add test L0 aggregator consuming Aave L1.
9. Add memory read view for account/protocol current risk.
10. Keep current rows in parallel for validation.
11. Cut selected API endpoints to memory after parity is stable.
```

## Safety Constraints During Migration

Do not:

```text
remove current Aave current rows yet
change scaled-balance canonical units
weaken required witness failure behavior
advance watermarks earlier than today
merge Aave with another protocol's runtime
move Aave-specific risk math into L0
```

Do:

```text
add manifests around existing snapshots
add explicit L1 publication
compare memory vs current rows
compare memory vs direct RPC samples
keep recovery path conservative
```

## Completion Criteria For Aave Alignment

Aave is fully aligned when:

```text
it is explicitly packaged as one protocol line
it has a dedicated RPC key and WAL
slot registries are durable
snapshots have manifests
L1 protocol vector publication exists
L0 can consume Aave without Aave internals
memory-first current risk read path exists
direct validation references vector epochs
current rows are compatibility/debug, not live-risk truth
```
