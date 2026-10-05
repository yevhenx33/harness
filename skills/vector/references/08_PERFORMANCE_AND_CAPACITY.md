# Performance And Capacity

This document records the target performance model and benchmark interpretation
for the modular vector architecture.

## Target

```text
1M accounts
500 asset/market slots per protocol line
block-by-block live updates
12 second Ethereum block interval
isolated protocol lines
dedicated RPC key per line
memory-first live risk
compact checkpoints
```

## Key Conclusion

For sparse production-shaped accounts, risk computation is not the bottleneck.
RPC witness collection and durable writes dominate wall-clock time.

Dense full recompute remains well below the block interval on the current 16 CPU
host, but it is not a sub-10-ms operation.

## Benchmark Tools

Existing benchmark binaries:

```text
backend/aave-indexer-rs/src/bin/aave_risk_projection_mock.rs
backend/aave-indexer-rs/src/bin/layered_vector_monte_carlo.rs
```

Benchmark docs:

```text
../AAVE_V3_RISK_PROJECTION_MOCK.md
../LAYERED_VECTOR_MONTE_CARLO_STRESS_TEST.md
```

Run Monte Carlo help:

```bash
cargo run --release \
  --manifest-path backend/aave-indexer-rs/Cargo.toml \
  --bin layered_vector_monte_carlo -- --help
```

## Sparse Production-Shaped Scenario

Command:

```bash
cargo run --release \
  --manifest-path backend/aave-indexer-rs/Cargo.toml \
  --bin layered_vector_monte_carlo -- \
  --protocols 4 \
  --accounts-per-protocol 250000 \
  --slots 500 \
  --layout sparse \
  --avg-active-slots 12 \
  --blocks 20 \
  --checkpoint-window-blocks 5 \
  --account-touches-per-block 10000 \
  --slot-updates-per-block 8 \
  --threads 8
```

Latest local result:

```text
protocols=4
dedicated_rpc_keys=4
total_accounts=1000000
slots_per_protocol=500
materialized_sparse_positions=12002254

CPU isolated wall/block:
  p50 9.25 ms
  p95 10.92 ms
  max 11.45 ms

Risk compute/protocol-block:
  p50 4.37 ms
  p95 5.90 ms
  max 6.75 ms

CPU+estimated RPC isolated wall/block:
  p50 601.81 ms
  p95 638.48 ms
  max 648.64 ms

RPC calls/protocol-block:
  p50 18527
  p95 28441
  max 29396

Checkpoint:
  rows 16
  bytes 128000000
  encode p95 2.76 ms/row
```

Interpretation:

```text
The sparse vector hot path computes risk under 10 ms per protocol-block p95.
Estimated RPC witness time dominates.
The architecture should optimize witness volume, batching, provider latency, and
durable write backpressure before optimizing risk math.
```

## Dense Worst-Case Scenario

Command:

```bash
cargo run --release \
  --manifest-path backend/aave-indexer-rs/Cargo.toml \
  --bin layered_vector_monte_carlo -- \
  --protocols 1 \
  --accounts-per-protocol 1000000 \
  --slots 500 \
  --layout dense \
  --blocks 1 \
  --checkpoint-window-blocks 1 \
  --account-touches-per-block 0 \
  --slot-updates-per-block 500 \
  --threads 16
```

Latest local result:

```text
protocols=1
dedicated_rpc_keys=1
accounts=1000000
slots=500
full_recompute=100%

CPU isolated wall/block:
  420.41 ms

Risk compute:
  391.34 ms

CPU+estimated RPC isolated wall/block:
  451.04 ms

Checkpoint:
  1 row
  32000000 bytes
  encode 29.04 ms
```

Interpretation:

```text
The dense 1M x 500 full recompute path is far below a 12 second block interval
on the current 16 CPU host, but it is not under 10 ms.
It is memory-bandwidth and serialization sensitive.
```

## 8 vs 16 Threads

Sparse run:

```text
8 threads:
  CPU isolated p95 10.92 ms
  risk compute/protocol-block p95 5.90 ms
  CPU+estimated RPC p95 638.48 ms

16 threads:
  CPU isolated p95 15.04 ms
  risk compute/protocol-block p95 6.67 ms
  CPU+estimated RPC p95 638.61 ms
```

Dense run:

```text
8 threads:
  CPU/block 714.15 ms
  risk compute 680.46 ms

16 threads:
  CPU/block 420.41 ms
  risk compute 391.34 ms
```

Interpretation:

```text
Sparse dirty batches are already small, so 16 threads do not materially improve
p95 and can add scheduling variance.

Dense full recompute benefits materially from 16 threads, cutting risk compute by
about 42%, but scaling is not linear.
```

## Performance Budget

For each protocol line:

```text
confirmed block interval:
  12,000 ms target envelope

sparse risk compute:
  < 10 ms p95 target

dense full recompute:
  < 1,000 ms target on 16 cores

checkpoint encode:
  < 100 ms per checkpoint row target

RPC witness fetch:
  dominant external budget
  should stay below configured catch-up target

ClickHouse async write:
  should not block hot risk compute except at commit gate
```

For L0:

```text
L1 vector aggregation:
  < 10 ms target for normal protocol counts

stale protocol detection:
  per publication epoch

cross-protocol serving:
  memory-first
```

## Bottlenecks

Expected bottlenecks in order:

```text
1. RPC witness latency and rate limits
2. RPC witness volume
3. ClickHouse write latency during commit
4. checkpoint serialization for large snapshots
5. serving index maintenance for large sort/filter surfaces
6. memory bandwidth during dense full recompute
7. risk math CPU
```

## Tuning Knobs

Protocol line:

```text
max_blocks_per_tick
catch_up_target_ms
multicall_batch_size
rpc_concurrency
risk_threads
checkpoint_window_blocks
skip_debug_current_writes
dirty_ratio_full_recompute_threshold
price/index update cadence
serving index rebuild cadence
```

Platform:

```text
per-protocol resource limits
L0 aggregation cadence
stale protocol threshold
cross-protocol snapshot cadence
API cache TTL for historical views
```

## Performance Rules

Do:

```text
keep hot vectors compact
use sparse positions by default
precompute slot multipliers when possible
batch RPC through Multicall3
separate required and optional witnesses
write durable data asynchronously
checkpoint compact binary vectors
rebuild indexes from vectors when cheaper than incremental churn
```

Do not:

```text
decode JSON in the risk loop
query ClickHouse per live account request
rewrite every account-risk row on index-only blocks
use hash maps inside the per-account projection loop
share one RPC key across protocols
let L0 call protocol RPC
```

## Benchmark Expansion Needed

Future benchmark cases:

```text
multi-protocol dense stress:
  4 lines x 1M accounts x 500 slots with staggered full recompute

RPC replay benchmark:
  real provider latency distributions by protocol

checkpoint codec benchmark:
  raw binary vs compressed bundle

serving index benchmark:
  top-N and filter update costs for 1M accounts

recovery benchmark:
  checkpoint load vs facts/witness replay

ClickHouse write benchmark:
  facts/witnesses/checkpoints under live ingestion volume
```

## Acceptance Criteria

Performance is acceptable when:

```text
sparse production-shaped risk compute stays under 10 ms p95 per protocol-block
dense 1M x 500 full recompute stays under 1 second on 16 cores
checkpoint encode stays comfortably below the checkpoint window
RPC delay is isolated per protocol key
one protocol's RPC delay does not delay other protocols
L0 aggregation remains negligible compared with protocol-line work
live APIs do not require ClickHouse risk recompute
```
