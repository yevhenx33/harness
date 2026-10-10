---
name: budget
description: Use when the user tags $budget or asks to quantify performance/resource limits before architecture or implementation. Covers CPU, memory, disk, RPC, database scans, payload size, latency, freshness, fanout, write amplification, and VPS/container budgets.
---

# Budget

Use this skill to make performance and resource limits explicit before code or production changes. Prefer measured current-state baselines over guesses.

## Budget Workflow

1. Define the slice:
   - protocol, service, endpoint, panel, worker, or table
   - expected users/requests/blocks/accounts/markets
   - steady state and catch-up state
   - workload classes that change cost, such as ordinary updates and rollovers
   - complete measurement boundary through persistence and requested delivery
2. Capture current baseline when possible:
   - latency p50/p95/p99
   - CPU and memory
   - disk footprint and growth
   - RPC calls and retries
   - ClickHouse query shape and rows scanned
   - payload size and WebSocket frame size
3. Set target budgets:
   - freshness and lag
   - request latency
   - process CPU/RSS
   - query scan limits
   - write rows per block/window
   - RPC calls per block/window/day
   - startup/recovery time
   - build and verification cost, temporary artifacts, backups, and retained state
   - cumulative attempt, retry, remote-call, and concurrency allowances
4. Stress the worst case:
   - price/index/factor changes
   - mass account movement
   - RPC latency and timeout
   - restart and checkpoint replay
   - historical catch-up
   - frontend range or selector changes
5. Define enforcement:
   - tests
   - status fields
   - Prometheus metrics
   - alerts
   - stop/rollback criteria

Distinguish desired targets, hard caps, and agreed architecture stop conditions.
Name growing dimensions and estimate time, space, I/O, and fanout bounds before
scaling the workload. Count waiting, queueing, retries, and deferred work in the
complete operation; stage timings are diagnostics, and summed stage quantiles
are not measured end-to-end quantiles. A faster partial path cannot satisfy a
budget that includes omitted work. Report target misses even after improvement;
when a stop condition is reached, hold the affected work without resetting its
allowance or authorizing rollback.

## Budget Table

Use a table like:

| Area | Current | Target | Hard limit | Measurement | Owner |
|---|---:|---:|---:|---|---|
| API payload | | | | curl size / browser | |
| p95 latency | | | | logs / Prometheus | |
| RPC calls/day | | | | router metrics | |
| ClickHouse rows scanned | | | | query log / EXPLAIN | |
| RSS | | | | docker stats | |

## Output Shape

1. Slice and assumptions.
2. Current baseline.
3. Target and hard-limit budgets.
4. Worst-case fanout analysis.
5. Instrumentation and enforcement.
6. Recommendation: proceed, narrow scope, simulate, or redesign.

If the budget cannot be measured yet, state what mock, dry run, or monitored experiment would make it measurable.
