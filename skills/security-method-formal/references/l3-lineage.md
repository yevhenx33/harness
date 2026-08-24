# L3 Lineage: Formal-Methods Review

## Pressure

Testing samples executions and conventional review can overlook rare interleavings or subtle protocol states. Mathematical specification, theorem proving, model checking, and symbolic reasoning were developed to make bounded correctness claims exhaustive relative to explicit assumptions.

## Method response

Formal review asks whether a stated property follows from a precisely defined model and whether that model corresponds closely enough to the implementation for the claim to matter.

## Current shape

Methods range from lightweight specification and bounded model checking to SMT-backed verification and machine-checked proofs. Practical assurance increasingly combines formal artifacts with conventional review of modeling assumptions, code generation, deployment, and the trusted computing base.

## Inherited strengths

- Exhaustively checks a bounded state space or proof obligation.
- Produces precise counterexamples and explicit assumptions.
- Raises assurance for compact, stable, high-consequence components.

## Known failure modes

- Proving the wrong or vacuous property.
- Omitting the transition that contains the real defect.
- Assuming model-to-code correspondence without evidence.
- Presenting timeout, bounded success, or solver output as universal proof.

## Primary anchors

- [NIST IR 8539: Security Property Verification](https://csrc.nist.gov/pubs/ir/8539/final)
- [NIST Design/Modeling Verification Tools](https://www.nist.gov/itl/csd/secure-systems-and-applications/designmodeling-verification-tools)
- [NIST: Verification and Test Methods for Access Control Policies](https://www.nist.gov/publications/verification-and-test-methods-access-control-policiesmodels)
- [NIST IR 8397: Guidelines on Minimum Standards for Developer Verification](https://csrc.nist.gov/pubs/ir/8397/final)
