# L2 Playbook: Fuzzing Review

## Tools

- Prefer the project's existing fuzzer, property-test framework, sanitizers, corpus, and CI integration.
- Use coverage instrumentation, grammar-aware generation, dictionaries, differential oracles, or snapshotting only when they reduce a named blind spot.
- Do not install a fuzzer or start a long campaign without authority and a resource budget.

## Practices

Track executions, useful coverage, corpus growth, crashes, timeouts, hangs, peak memory, disk use, minimization success, and validated root causes. Bound parallelism, input size, per-case timeout, retained corpus, and retries.

## Protocol

1. Pin target, build, harness, oracle, and resource envelope.
2. Verify positive and negative controls against the oracle.
3. Confirm generated input reaches the intended parser, transition, or assertion.
4. Start with a minimal high-value corpus and observe saturation.
5. Reproduce and minimize every candidate from a clean state.
6. Group by root cause and state unexercised behaviors.

## Evidence and falsification

A finding needs a minimized input, stable reproduction, target reachability, violated property, and consequential behavior. Falsify it as harness error, nondeterminism, resource-only artifact outside the admitted model, duplicate root cause, or unreachable production configuration.

## Dynamic boundary and failure behavior

Run in a local or explicitly authorized isolated environment with hard CPU, memory, disk, timeout, and concurrency limits. Stop on containment failure, uncontrolled state growth, external traffic, sensitive data, or repeated non-actionable failures.
