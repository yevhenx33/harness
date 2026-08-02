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
- for live systems, trace the actual producer-to-consumer path rather than
  trusting labels, nearby source, or top-level health alone
- use bounded commands and read-only queries; redact secrets from output
- distinguish observed, verified, inferred, stale, hypothetical, and unavailable
  evidence
- preserve partial or unavailable values; do not turn missing evidence into zero
  or healthy
- report ownership, boundaries, invariants, tests, coverage, risks, unknowns,
  performance/operational impact, and the smallest useful implementation slices

Report in this order: scope and mutation boundary, inspected map, findings,
performance notes, gaps and next slices, and anything not verified.
