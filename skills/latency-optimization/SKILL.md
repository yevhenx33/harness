---
name: latency-optimization
description: Diagnose, design, implement, or verify latency and jitter reductions in APIs, services, indexers, data pipelines, user-facing request paths, and CPU-bound kernels. Use when asked to optimize response time, hot paths, p95 or p99 latency, fanout, database or RPC delays, caching, precomputation, allocations, synchronization, write amplification, or low-latency architecture. Enforce a frozen workload, end-to-end measurement, correctness and freshness parity, resource accounting, and evidence before CPU-level micro-optimization.
---

# Latency Optimization

Deliver the smallest verified reduction at the authoritative boundary. Treat
correctness, completeness, freshness, durability, and recovery as invariants;
never trade them away silently for a faster number.

## Respect authority

- Treat read, inspect, audit, review, diagnose, benchmark, and profile requests
  as read-only. Report a proposed patch without editing.
- Treat fix, implement, change, and optimize as authority for the smallest
  necessary local change only.
- Require explicit authority for deployment, restart, production load tests,
  production profiling, migrations, data writes, or persistent runtime tuning.
- Avoid sustained load or intrusive profilers on shared or production systems.

## Freeze the performance contract

Record this contract before tailoring a solution:

```text
Outcome: observable latency behavior to improve
Path: exact start and end measurement points
Workload: requests/events, data size, concurrency, hit/miss mix, and failures
Owner: component that authoritatively controls the dominant work
Invariant: correctness, freshness, completeness, durability, and recovery rules
Baseline: build/artifact, environment, configuration, and measurements
Acceptance: target distribution plus correctness and resource limits
Non-goals: adjacent throughput, infrastructure, or feature work
Operations: deployment authority, rollback, and recovery
```

If the user has not supplied a latency target, measure and rank bottlenecks
without inventing success. For a local memory-first serving path with no
project-specific target, use median below 50 ms as a soft goal and 100 ms as a
diagnostic every-sample ceiling. Do not apply those numbers blindly to remote
networks, chain confirmation, batch processing, or external providers.

## Separate paths before optimizing

Model each path independently:

- **Request serving:** client or proxy ingress to complete, valid response.
- **Live publication:** confirmed input event to validated state visible to
  consumers.
- **History or backfill:** bounded query or replay request to complete result.
- **Compute kernel:** function entry to output only after the end-to-end path is
  proven CPU-bound.

Do not optimize a component timer and claim an end-to-end win. Do not combine a
throughput-oriented catch-up path with a latency-oriented live path.

## Establish the baseline

1. Read the nearest instructions, relevant Git state, runtime configuration,
   owner code, consumers, tests, and rollback boundary.
2. Trace the actual path across proxy, API, cache or memory, database, RPC,
   queues, serialization, storage, and frontend work.
3. Identify fanout width, retry multiplication, timeout behavior, cache misses,
   locks, copies, allocations, state rebuilds, serialization, and writes.
4. Freeze workload, harness, artifact or build fingerprint, environment, and
   configuration. Warm the path before sampling.
5. Run at least 10 measured samples. Report median and range. Report p95 only
   with at least 20 samples and p99 only with at least 100. Keep maximum visible
   whenever a hard ceiling or jitter matters.
6. Capture the correctness oracle on the same input: output hash, row parity,
   invariant, canonical source readback, or independent witness.
7. Capture relevant resource costs: CPU, RSS, allocations, network and RPC
   calls, disk bytes, write amplification, queue depth, and degraded or stale
   results.

Label small sequential loopback samples as diagnostics, not production SLO
proof. Measure representative concurrency and hit/miss behavior before making a
capacity claim.

## Rank mechanisms in this order

### 1. Eliminate work

- Remove duplicate computation, repeated parsing, unnecessary serialization,
  redundant validation, and unused response fields.
- Bound scans, ranges, payloads, retained state, retries, and concurrency.
- Prevent nested retries and cache-miss stampedes.
- Delete superseded compatibility or fallback work when correctness permits.

### 2. Move work off the hot path

- Precompute aggregates, sorting indexes, token or filter summaries, and stable
  projections at publication time.
- Serve current state from validated immutable memory when that is the owning
  architecture.
- Refresh database-derived enrichments asynchronously or at explicit
  boundaries instead of querying them per request.
- Move nonessential logging, metrics formatting, checkpoints, and durability
  work away from the measured path only when the invariant allows it.
- Preserve fail-closed behavior; stale or partial cached data must remain
  explicit.

### 3. Reduce fanout and remote I/O

- Prefer one owned aggregate or batched request over per-source request fanout.
- Batch same-block or same-key reads and classify results locally.
- Place provider selection and retries at one authoritative layer.
- Cache by canonical identity and invalidate by version, hash, or boundary.
- Treat HTTP success as transport success only; validate row-level completeness
  and source health.

### 4. Reduce copying and publication amplification

- Keep a single writer and immutable reader snapshots when practical.
- Coalesce multiple producer updates before rebuilding shared derived state.
- Partition state by owner and share unchanged partitions instead of cloning
  the complete dataset.
- Prefer compact indexes or stable identifiers over repeated joins and sets.
- Persist bounded deltas or checkpoints only if the durability and recovery
  contract permits it; otherwise retain persist-before-publish semantics.
- Use bounded latest-state or message channels to apply backpressure and collapse
  obsolete intermediate updates. Introduce a ring buffer only when measurements
  demonstrate that a simpler bounded channel is insufficient.

### 5. Optimize data layout and CPU behavior

Proceed only after the path is local, bounded, compute-dominant, and stable.

- Profile release artifacts using representative data.
- Inspect cycles, instructions, cache misses, branch misses, context switches,
  allocations, and lock contention as hypotheses, not conclusions.
- Improve locality, remove unpredictable branches, specialize dispatch, inline,
  pin cores, or change scheduling only when a measurement attributes material
  latency to that mechanism.
- Reject busy-spinning, realtime scheduling, cache warming, user-space
  networking, and manual branchless rewrites without a strict tail-latency need
  and an explicit CPU, power, fairness, and operational budget.

## Implement the smallest owner fix

- Restore the latency invariant at the component that owns the dominant work.
- Prefer an existing cache, publication, batching, index, or snapshot mechanism
  over a new layer.
- Keep success, timeout, unavailable, stale, partial, malformed, duplicate,
  retry, and recovery behavior visible.
- Preserve unrelated work and avoid broad refactors.
- Add instrumentation only where it distinguishes competing mechanisms or proves
  acceptance. Do not build a permanent harness for a one-off unknown.

## Verify without moving the goalposts

1. Re-run the frozen workload on the changed release artifact in the same
   environment.
2. Prove output and failure parity with the baseline or canonical oracle.
3. Exercise cache hit and miss, timeout, unavailable, stale, partial, duplicate,
   and retry paths that can violate acceptance.
4. Compare end-to-end median, p95 when sufficiently sampled, maximum or range,
   CPU, RSS, network or RPC calls, and disk writes.
5. Run focused correctness tests, then broader checks proportional to risk.
6. Reject the change if it only improves a component timer, lowers throughput or
   correctness outside the contract, increases hidden degradation, breaches a
   hard ceiling, or lacks reproducible evidence.

For critical paths, use an independent correctness oracle. A benchmark and test
that share the optimized implementation's assumption are not independent.

## Report

Lead with the outcome and distinguish observation from inference:

- frozen path, workload, environment, and artifact;
- before/after median, p95, range or maximum, and sample count;
- correctness and failure-path evidence;
- CPU, memory, network/RPC, disk, and write-amplification effects;
- owning mechanism and exact changed files or proposed slice;
- deployment and rollback state;
- unresolved risk, unmeasured concurrency, or production-validation gaps.

Call a result “not working” when it produces no end-to-end gain under the frozen
contract. Preserve negative results so the same mechanism is not repeated
without new evidence.
