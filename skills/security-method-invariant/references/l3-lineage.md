# L3 Lineage: Invariant Review

## Pressure

Many severe failures are not unsafe lines of code but illegal states reached through individually plausible transitions. State-machine reasoning and security-property verification made those cross-transition obligations explicit.

## Method response

Invariant review names one authoritative predicate and searches for a reachable transition sequence that falsifies it. It connects implementation review with property-based testing and bounded formal analysis without claiming a proof beyond reviewed coverage.

## Current shape

The approach is central to access control, distributed protocols, accounting, smart contracts, lifecycle systems, and concurrency, where retries and recovery paths are as important as the nominal transition.

## Inherited strengths

- Detects illegal states that local checks miss.
- Makes authorization and accounting assumptions falsifiable.
- Produces compact counterexample traces and regression properties.

## Known failure modes

- Stating a slogan instead of a predicate.
- Checking a derived view rather than the authoritative state.
- Omitting migration, retry, cancellation, or recovery transitions.
- Claiming universal assurance from bounded transition coverage.

## Primary anchors

- [NIST IR 8539: Security Property Verification](https://csrc.nist.gov/pubs/ir/8539/final)
- [NIST: Verification and Test Methods for Access Control Policies](https://www.nist.gov/publications/verification-and-test-methods-access-control-policiesmodels)
- [USENIX Security 2021: Likely Invariants for Fuzzers](https://www.usenix.org/conference/usenixsecurity21/presentation/fioraldi)
- [OWASP Smart Contract Security Verification Standard](https://scs.owasp.org/SCSVS/)
