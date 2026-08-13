---
name: read
description: Read-only investigation, audit, inspection, and reporting.
---

# read

Use this skill when the task is explicitly read-only or asks for a check, map,
audit, inspection, status, or report.

- do not edit files, commit, deploy, restart services, migrate, or mutate
  production state
- inspect only the requested scope and nearest applicable instructions
- identify the exact checkout, host, ref, service, route, API, and data source
- freeze the primary invariant, owner, consumer, oracle, and freshness boundary
- for live systems, trace the actual producer-to-consumer path rather than
  trusting labels, nearby source, or top-level health alone
- use built-in Codex memory only for a bounded lookup of task-relevant prior
  evidence; preserve provenance and reverify drift-prone claims in current state
- use bounded commands and read-only queries; redact secrets from output
- distinguish observed, verified, inferred, stale, hypothetical, and unavailable
  evidence
- preserve partial or unavailable values; do not turn missing evidence into zero
  or healthy
- name growing dimensions, fanout, retries, copies, queues, and async boundaries
  when they affect cost, latency, completeness, or failure amplification
- report ownership, boundaries, invariants, evidence age, tests, coverage, risks,
  unknowns, performance impact, and the smallest verifiable next slices

Report in this order: scope and mutation boundary, inspected map, findings,
performance notes, prior evidence reverified or rejected, gaps and next slices,
and anything not verified.
