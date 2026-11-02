---
name: latency-optimization
description: Diagnose, design, implement, or verify latency and jitter reductions in APIs, services, indexers, data pipelines, user-facing request paths, and CPU-bound kernels. Use when asked to optimize response time, hot paths, p95 or p99 latency, fanout, database or RPC delays, caching, precomputation, allocations, synchronization, write amplification, or low-latency architecture. Enforce a frozen workload, end-to-end measurement, correctness and freshness parity, resource accounting, and evidence before CPU-level micro-optimization.
---

# Latency Optimization

Recover the performance contract: observable outcome, exact measurement boundary,
workload, owning service, correctness/freshness/durability/recovery invariants,
artifact and environment, target, resource limits, authority, and recovery.
Do not invent a latency target. Production profiling, load tests, deployment,
restarts, and persistent tuning retain their separate authority requirements.

## Measure, attribute, qualify

Separate request serving, confirmed-input publication, historical catch-up,
and compute-kernel workloads. Establish whether cost scales with the full dataset
or affected keys and dependencies before optimizing individual operations.
Measure complete valid results, completed work, failures, and resource consumption.
A hot component can consume CPU while elapsed time is dominated by waiting elsewhere.

Begin with existing measurements and scoped resource counters. Select a diagnostic
tool for an unanswered question, not because it is installed. Tool availability
is not proof that permissions, events, symbols, or instrumentation work.

| Question | Start with | Escalate to |
|---|---|---|
| Complete-operation performance | Existing integration workload and service measurements | Bounded workload driver |
| CPU, memory, I/O or pressure | /proc, cgroup counters, sysstat, pressure counters | Targeted profiler |
| CPU attribution | perf stat, scoped perf record/report | One suitable viewer |
| Scheduler delay or blocking | Resource and scheduling evidence | perf sched or targeted BPF |
| Allocation or retention | Bounded lifecycle measurements | Heaptrack or focused allocation tooling |
| Async-task behavior | Existing task spans | Tokio Console instrumentation |
| Query cost | Query ID, query log, EXPLAIN | Query profiler |
| Compilation | Cargo --timings | Compiler self-profile |
| Browser rendering/retention | Relevant interaction in browser tools | React or heap profiling |
| GPU behavior | Bounded CPU/GPU timeline | Nsight Systems, then targeted Compute |

These are optional tools, not mandatory dependencies or installation requests.
Use current tool documentation and installed-version support for the selected mode.

## Bound collection

Before collection identify the question, exact PID/cgroup/query/interaction,
workload, artifact, tool/version, permissions, collection window, selected events
or sampling rate, output bound, retention, and stop condition. Do not collect
system-wide data or cross another service's boundary without authority. Restricted
access does not authorize weakening host controls.

Check compatibility and symbol/unwinding prerequisites before an expensive run.
Include instrumentation builds, profiler CPU/memory, buffers, output, retention,
and analysis in cumulative costs. GPU replay and allocator/async instrumentation
can change execution. Bound telemetry labels, trace queues, and payload capture;
observability must not become an unbounded workload.

Choose sampling and duration from effect size, variability, tail requirements,
and temporal dependence. Small samples may be diagnostic, not qualification.
Report uncertainty; a tool printing a percentile does not establish stability.
Do not average or sum service percentiles into an end-to-end percentile; aggregate
compatible observations/histograms only where their semantics permit it.

Choose an arrival/concurrency model matching the actual workload. Closed-loop
clients reduce arrivals as responses slow. Use independently scheduled arrivals
when representative, and retain driver saturation, dropped arrivals, failures,
and unfinished work. Report lost samples and unavailable counters explicitly.

## Change the owning mechanism

Use the root outcome-first and incremental-first rules. Eliminate duplicate
computation, parsing, serialization, validation, scans, and nested retries before
local CPU tuning. Reuse authoritative results without losing fresh acceptance
checks. Coalesce affected updates and bound fallbacks.

Moving work off the response path does not remove its resource or recovery cost.
Preserve persist-before-publish requirements. Do not introduce another cache,
service, ring buffer, scheduler, or compatibility path without a demonstrated need.
Profile representative release artifacts before low-level optimization. Avoid
busy-spinning, realtime scheduling, user-space networking, or manual branchless
rewrites without a demonstrated requirement and CPU/fairness/resource budget.

## Verify and report

After changing the owner, compare equivalent end-to-end runs and independent
correctness/failure evidence. Control workload, concurrency, cache state, artifact,
environment, and run order where material. Separate build costs from runtime costs.
Measure without materially perturbing diagnostic instrumentation while retaining
required operating telemetry. Record CPU time, peak/retained memory, network/RPC,
database work, writes, freshness, and degradation where relevant.

Reject claimed gains that drop required behavior, hide failures, breach ceilings,
or improve only a component timer. Collect another profile only for unresolved
uncertainty or a changed mechanism. Apply root completion passes without automatic
reprofiling. Start with TLDR and before/after/target, sample and workload details,
correctness/resource evidence, deployment state, and material limitations.
