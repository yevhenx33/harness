---
name: security-method-fuzzing
description: Use explicitly to evaluate or design systematic adversarial input exploration for parsers, decoders, APIs, and other executable untrusted-input surfaces. Do not use without a harness, oracle, and bounded execution budget.
---

# Fuzzing Review

## Selection boundary

Best fit: an executable target consumes complex or adversarial inputs, many cases can be generated, and crashes or property violations can be observed through a deterministic oracle.

Reject this method for non-executable designs, an already-defined source-to-sink path, or targets without isolation, reset, observability, and resource bounds. Use variant analysis when a known structural defect is the seed.

## Inputs and authority

Resolve the exact target/revision, input boundary and grammar, harness, seed corpus, oracle, sanitizers or assertions, state reset, coverage signal, time/CPU/memory/disk limits, deduplication rule, forbidden effects, and recovery owner. Installing tools or running material compute requires explicit authority.

Expected cost: high; all executions, concurrency, retained corpus, and compute resources must fit the fixed campaign budget.

## Workflow

1. Pin the target and define the security property and failure oracle.
2. Review the harness for reachability, determinism, isolation, reset, and false positives.
3. Choose the smallest input model, seed corpus, and mutation strategy that exercises the property.
4. Run only within the admitted budget; minimize and reproduce each candidate independently.
5. Deduplicate by root cause, validate impact, and report coverage limits rather than equating runtime with assurance.

Read [references/l2-playbook.md](references/l2-playbook.md) before any campaign. Read [references/l3-lineage.md](references/l3-lineage.md) only for technique lineage, comparison, or limitations. Format results with `../security-review-router/references/review-contract.md`.

## Stop conditions

Stop when isolation or reset fails, the oracle is noisy, resource growth is unbounded, the harness misses the target path, or results cannot be reproduced. A unique crash is a candidate, not automatically a vulnerability.
