---
name: security-method-ctf
description: Use explicitly for a bounded, authorized target where a working local exploit or flag is the strongest success oracle. Do not use for broad assurance, production exploitation, or an unknown target boundary.
---

# CTF / Exploit-First Review

## Selection boundary

Best fit: the target is deliberately bounded, safe to exercise, and has an exact exploit, flag, or controlled-state oracle that can decide success.

Reject this method for broad repository coverage, unclear authorization, live-user or production systems, or when proof would require harmful effects. Use attack-path analysis when reachability can be decided without constructing an exploit, and fuzzing when systematic input exploration is the main need.

## Inputs and authority

Resolve the exact target/revision, challenge boundary, attacker starting state, success oracle, allowed tools and payloads, time and compute budget, forbidden effects, evidence capture, reset mechanism, and recovery owner.

Expected cost: high; exploit mechanics are time-boxed and stop once the fixed oracle is reproducibly satisfied or falsified.

## Workflow

1. Pin the target and restate the flag or exploit condition as an observable oracle.
2. Identify the smallest attack surface and the likely security property protecting the oracle.
3. Form one exploit hypothesis and trace its prerequisites before writing a payload.
4. Build the minimum reproducible local proof and confirm it from a clean reset.
5. Explain the violated property, environmental assumptions, limitations, and nearest non-exploit alternative.

Read [references/l2-playbook.md](references/l2-playbook.md) while developing the proof. Read [references/l3-lineage.md](references/l3-lineage.md) only for method history, comparison, or limitations. Format results with `../security-review-router/references/review-contract.md`.

## Stop conditions

Stop if the oracle cannot be stated, the target cannot be reset, the proof would escape the admitted environment, or the required effect is destructive. A crash, scanner alert, or suggestive output is not a successful exploit unless it satisfies the fixed oracle.
