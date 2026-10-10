---
name: me
description: Use when the user tags $me or asks for a monitored experiment, live run, bounded trial, indexer experiment, soak check, performance observation window, or goal-style experiment. Structures preflight, metrics, controlled execution, resource monitoring, and final status. Requires explicit permission before mutating services.
metadata:
  short-description: Monitored experiment workflow
---

# Monitored Experiment

Use this skill for `$me`, monitored experiments, bounded live runs, soak checks, and goal-style observations.

## Safety Contract

- Do not start, stop, restart, redeploy, or mutate services unless the user explicitly requests the experiment action.
- If the experiment is read-only observation, state that clearly.
- Keep the experiment bounded by time, blocks, rows, requests, or another concrete stop condition.
- Capture enough pre/post state to distinguish real progress from log noise.

## Experiment Workflow

1. Objective:
   - what is being tested
   - success criteria
   - failure criteria
   - maximum duration or stop condition
   - workload, artifact, environment, and full measurement boundary
   - architecture falsifier, affected stop scope, and condition for resumption
2. Preflight:
   - git/worktree state if relevant
   - service/container/process state
   - current block/offset/checkpoint/row counts
   - disk free space
   - CPU/memory snapshot
   - relevant logs baseline
3. Run/observe:
   - record start timestamp
   - use low-overhead monitoring first
   - sample CPU, memory, disk IO, network/RPC, logs, and DB progress
   - avoid tight polling loops; use sensible intervals
   - stop the bounded run at its agreed limit; preserve evidence and unfinished work
4. Postflight:
   - record end timestamp
   - compare before/after metrics
   - check errors, retries, failed rows, lag, restarts, memory growth, disk growth
5. Verdict:
   - verified win, verified no-gain, invalid, blocked, or inconclusive
   - performance cost
   - correctness signal
   - next narrow action

Compare equivalent inputs and starting state, separating workload classes when
their costs differ. Report sample count, warmup, and missing measurements; a
single run cannot establish a median or p95. Measure the complete requested
operation rather than adding stage quantiles. Verify required correctness
alongside performance; a faster result that drops required behavior is invalid.
A target miss remains unfinished, and an agreed architecture blocker holds
dependent variants until its resume condition is satisfied.

## Metrics Checklist

- Wall time and observation window.
- Blocks/events/rows processed.
- Bad runs, failed rows, failed RPC calls, retries.
- CPU percent and sustained load.
- RSS/memory growth.
- Disk usage and write amplification.
- Database table growth and query latency if relevant.
- RPC/network request volume.
- Container/process restarts and log error rate.

## Goal-Style Use

For longer work, create or use an explicit goal only when the user asks for persistent continuation. Keep goal objectives measurable and stop when success, failure, or the stop condition is reached.

## Output Shape

1. Objective and bounds.
2. Preflight baseline.
3. Observed metrics.
4. Result comparison.
5. Verdict and next step.

Do not bury failures. If progress is implementation-only and not actual replay/runtime progress, say so directly.
