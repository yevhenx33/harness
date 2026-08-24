# L3 Lineage: Security Architecture Review

## Pressure

Many security failures are enabled by trust and ownership decisions that line-level analysis cannot repair. Architecture evaluation developed to assess whether system structure can support required qualities before and during implementation.

## Method response

The method traces security properties across components and demands explicit owners, boundaries, failure containment, and recovery. It treats implementation evidence as confirmation of the design rather than a substitute for it.

## Current shape

Security architecture review now spans identity, data, supply chain, cloud and service boundaries, administrative planes, and operational recovery. It often consumes a threat model and produces properties for invariant or formal review.

## Inherited strengths

- Finds systemic weaknesses that repeat across code paths.
- Makes ownership and recovery reviewable.
- Supports early design correction before implementation cost accumulates.

## Known failure modes

- Reviewing ideal diagrams rather than deployed topology.
- Adding controls without naming the authoritative enforcement point.
- Mistaking defense layers for independent evidence.
- Proposing a parallel system without cutover and rollback.

## Primary anchors

- [NIST: Static Analysis Is Not Enough](https://www.nist.gov/publications/static-analysis-not-enough-role-architecture-and-design-software-assurance)
- [Microsoft Security Architecture Design](https://learn.microsoft.com/en-us/azure/architecture/security/security-get-started)
- [ISO/IEC/IEEE 42030 Architecture Evaluation](https://www.iso.org/standard/73436.html)
- [NIST SSDF](https://csrc.nist.gov/pubs/sp/800/218/final)
