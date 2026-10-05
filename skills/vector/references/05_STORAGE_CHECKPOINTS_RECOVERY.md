# Storage, Checkpoints, And Recovery

The database is durable truth for facts, witnesses, snapshots, recovery, history,
audits, and analytics. It is not the live risk calculator.

## Storage Model

```text
real-time truth:
  in-memory vectors inside each protocol line

durable truth:
  event facts
  same-block RPC witnesses
  WAL segments
  canonical raw current rows
  vector checkpoints
  snapshot manifests
  rollups
```

Current rows are convenience state. Facts, witnesses, WAL, and snapshots are the
recovery foundation.

## Protocol Namespace

Each protocol line should own a separate ClickHouse database or namespace:

```text
aave_indexer
morpho_indexer
fluid_indexer
euler_indexer
compound_indexer
```

If one physical ClickHouse cluster is shared, isolation still exists at the
database/table-prefix and writer level.

## Recommended Protocol Tables

```text
event_facts
touched_entities
rpc_witnesses
price_rpc_witnesses
pipeline_watermarks
indexer_runs
wal_checkpoints
slot_registry_snapshots
account_vector_snapshots
position_vector_snapshots
account_risk_vector_snapshots
asset_reserve_vector_snapshots
market_vector_snapshots
protocol_vector_snapshots
snapshot_manifests
account_position_current_raw
market_state_current_raw
asset_price_current_raw
index_epochs
price_epochs
market_state_epochs
protocol_risk_rollups
market_rollups
account_risk_current_debug
```

Not every table must exist on day one, but every protocol line should converge to
this shape.

## L0 Platform Tables

```text
protocol_vector_latest
protocol_vector_snapshots
cross_protocol_vector_latest
cross_protocol_vector_snapshots
cross_protocol_asset_exposure_snapshots
cross_protocol_account_exposure_snapshots
platform_indexer_runs
platform_alert_events
```

## Append-Only Facts

Event and witness facts should be append-only:

```text
event_facts:
  immutable decoded logs

rpc_witnesses:
  immutable same-block call results

price_rpc_witnesses:
  optional split table for high-volume price witnesses
```

Do not mutate facts to repair state. Write repair facts or repair run records
with provenance.

## Current Raw State

Current raw state tables are useful for:

```text
bootstrap
debugging
compatibility APIs
latest exact state inspection
fallback rebuild if snapshots are missing
```

Examples:

```text
account_position_current_raw
market_state_current_raw
asset_price_current_raw
reserve_index_current
protocol_risk_current
```

Rules:

```text
do not treat current risk rows as canonical history
do not rewrite all account-risk rows on every index-only block
prefer argMax latest-state views over FINAL in hot queries
include source hashes and producer versions
```

## WAL

The WAL records vector-runtime mutation intent and commit boundaries.

Recommended records:

```text
range_begin
block_begin
event_batch_written
witness_batch_written
vector_mutations_applied
risk_recompute_completed
snapshot_written
rollup_written
block_commit
range_commit
watermark_commit
recovery_replay_begin
recovery_replay_commit
```

WAL records should form a hash chain:

```text
wal_record_hash = hash(previous_wal_hash, record_type, block, payload_hash)
```

The WAL is protocol-line local. L0 does not write protocol WAL.

## Vector Checkpoints

Vector checkpoints are compact snapshots of memory vectors.

Protocol-line checkpoint types:

```text
slot_registry_snapshot
account_vector_snapshot
position_vector_snapshot
account_risk_vector_snapshot
asset_reserve_vector_snapshot
market_vector_snapshot
protocol_vector_snapshot
```

L0 checkpoint types:

```text
cross_protocol_vector_snapshot
cross_protocol_asset_exposure_snapshot
```

## Snapshot Manifest

Every checkpoint group should have a manifest:

```text
snapshot_manifest {
  manifest_id
  protocol_id
  deployment_id
  chain_id
  from_block
  to_block
  block_hash
  block_timestamp
  checkpoint_window_blocks
  slot_registry_hash
  event_fact_hash
  rpc_witness_hash
  price_epoch_hash
  index_epoch_hash
  asset_vector_hash
  market_vector_hash
  account_vector_hash
  position_vector_hash
  account_risk_vector_hash
  protocol_vector_hash
  encoding_versions
  producer_version
  schema_version
  validation_status
  inserted_at
}
```

The manifest is the dependency contract that prevents mixing stale and fresh
layers.

## Checkpoint Cadence

Recommended initial cadence:

```text
account risk vector:
  every 5 committed blocks

slot registry:
  every checkpoint window if changed

account/position vectors:
  every checkpoint window or coarser if raw facts can replay cheaply

asset/market vectors:
  every checkpoint window if changed

protocol vector:
  every committed block or checkpoint window, depending on write volume

rollups:
  hourly/daily/weekly boundaries
```

The phrase "minute checkpoint" should be avoided unless the system actually uses
wall-clock minute boundaries. Ethereum block intervals vary. Prefer "5-block
checkpoint window" when using block count.

## Encoding

For large vectors, use versioned binary encodings rather than JSON:

```text
f64_le_v1
fixed_decimal_le_v1
u32_index_le_v1
address20_packed_v1
zstd_vector_bundle_v1
```

Each snapshot row should include:

```text
encoding
value_count
vector_bytes
vector_hash
producer_version
schema_version
```

## Recovery Flow

Startup recovery:

```text
1. Load latest valid snapshot manifest.
2. Load slot registry snapshots referenced by manifest.
3. Load vector snapshots referenced by manifest.
4. Verify vector hashes.
5. Load WAL checkpoint metadata.
6. Replay WAL after manifest.to_block if available.
7. Replay facts/witnesses after manifest.to_block if WAL is unavailable or incomplete.
8. Recompute risk vectors to latest committed watermark.
9. Verify protocol vector hash.
10. Resume live loop from watermark + 1.
```

If snapshot verification fails:

```text
mark manifest invalid
fall back to previous valid manifest
or rebuild from bootstrap anchor plus facts/witnesses
write a new valid checkpoint
```

## Rebuild From Facts And Witnesses

Full rebuild:

```text
1. Load bootstrap static metadata.
2. Initialize empty slot registries.
3. Replay event facts in block/log order.
4. Replay witness facts in deterministic witness order.
5. Apply vector mutations.
6. Recompute risk at required checkpoints.
7. Write fresh snapshots and manifests.
8. Compare final vector hashes with previous current state if available.
```

This path must be slower but correct.

## Watermark Contract

The watermark is advanced only after the commit gate passes.

Watermark row:

```text
pipeline_watermark {
  service
  pipeline
  protocol_id
  deployment_id
  chain_id
  block_number
  block_hash
  block_timestamp
  manifest_id
  wal_path
  wal_offset
  protocol_vector_hash
  status
  producer_version
  inserted_at
}
```

On restart, the protocol line trusts the latest watermark only if the referenced
WAL and manifest are valid or rebuildable.

## Rollups

Rollups are API/history outputs, not recovery-critical vector state.

Recommended rollups:

```text
protocol_risk_rollups
market_rollups
asset_exposure_rollups
health_factor_bucket_rollups
ltv_bucket_rollups
flow_rollups
```

Rollups should include source vector hashes so they can be audited.

## Hot Query Rule

Avoid `FINAL` in hot ClickHouse paths. Prefer:

```text
argMax latest-state views
materialized latest tables
ReplacingMergeTree only for compatibility/debug state
time-bucketed rollups for charts
memory vectors for live current risk
```

## Acceptance Criteria

Storage and recovery are acceptable when:

```text
facts and witnesses are append-only
slot registries are checkpointed
vector snapshots have manifests
manifests include dependency hashes
watermarks reference valid durable state
restart can load from checkpoint
failed checkpoint verification falls back safely
full rebuild from facts/witnesses is possible
current rows are not required for live risk serving
```
