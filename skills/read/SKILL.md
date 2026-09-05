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
- recover relevant prior decisions and inspect the exact checkout, host, ref,
  service, route, API, or data source needed for this question
- identify the owner, deciding evidence, and freshness boundary; use a fuller
  invariant and consumer map only when the investigation needs it
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
- stop when the requested question is answered with sufficient evidence; widen
  the investigation only for a material unresolved question or requested coverage

Lead with the finding and its consequence, then the evidence and material gaps.
Include a map, performance detail, or next slices when useful or requested; a
small check does not require a fixed report template. Preserve the objective and
unfinished gates of any broader task. A read-only phase grants no new mutation
authority and does not erase prior authorization; the latest user restriction
still controls what can resume.
