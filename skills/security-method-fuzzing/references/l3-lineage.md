# L3 Lineage: Fuzzing Review

## Pressure

Input spaces for parsers and protocols are too large for example tests. Random testing exposed unexpected behavior, while coverage guidance, symbolic assistance, grammar models, sanitizers, and continuous services made exploration more directed and observable.

## Method response

Fuzzing repeatedly generates or mutates inputs, observes execution feedback, and relies on explicit oracles to discover behaviors humans did not enumerate.

## Current shape

Modern fuzzing ranges from property-based tests to coverage-guided native fuzzers, whitebox techniques, differential testing, and continuously operated open-source campaigns. Campaign quality depends more on harness, oracle, reachability, and triage than raw execution count.

## Inherited strengths

- Explores large adversarial input spaces cheaply once harnessed.
- Produces concrete, reproducible counterexamples.
- Supports continuous regression discovery.

## Known failure modes

- Fuzzing a shallow harness that never reaches security-sensitive behavior.
- Using crashes as the only oracle for logical flaws.
- Counting executions or coverage as security assurance.
- Retaining duplicate, flaky, or unminimized candidates.

## Primary anchors

- [NIST: Fuzz Testing for Software Assurance](https://www.nist.gov/publications/fuzz-testing-software-assurance)
- [Microsoft Research: Automated Whitebox Fuzz Testing](https://www.microsoft.com/en-us/research/publication/automated-whitebox-fuzz-testing/)
- [USENIX Security 2023: On the Reliability of Coverage-Based Fuzzer Benchmarking](https://www.usenix.org/conference/usenixsecurity23/presentation/gorz)
- [Google Security Blog: Announcing OSS-Fuzz](https://security.googleblog.com/2016/12/announcing-oss-fuzz-continuous-fuzzing.html)
