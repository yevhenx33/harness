# L3 Lineage: Supply-Chain Review

## Pressure

Software assurance expanded beyond source code as attackers targeted dependencies, build systems, update channels, and trusted vendors. Supply-chain risk management addressed organizational dependencies; reproducible builds, SBOMs, provenance, and signing made technical artifact history more inspectable.

## Method response

The method treats the source-to-consumer chain as the review target and asks which identities can mutate each edge and which independent checks prevent an untrusted artifact from being accepted.

## Current shape

Modern practice combines dependency governance, isolated and hermetic builds, least-privilege CI, provenance attestations, signing, transparent distribution, consumer policy enforcement, and rehearsed recovery.

## Inherited strengths

- Connects repository controls to the artifact users actually execute.
- Exposes hidden privilege in CI, registries, and release automation.
- Separates artifact inventory from verified provenance and enforcement.

## Known failure modes

- Treating an SBOM as integrity or provenance proof.
- Signing an artifact without protecting or identifying the signer.
- Producing attestations that consumers never verify.
- Auditing dependencies while ignoring workflow and release compromise.

## Primary anchors

- [NIST SP 800-204D: Software Supply Chain Security in DevSecOps CI/CD](https://csrc.nist.gov/pubs/sp/800/204/d/final)
- [NIST Cybersecurity Supply Chain Risk Management](https://csrc.nist.gov/Projects/Cyber-Supply-Chain-Risk-Management)
- [Google Security Blog: Introducing SLSA](https://security.googleblog.com/2021/06/introducing-slsa-end-to-end-framework.html)
- [MITRE ATT&CK T1195: Supply Chain Compromise](https://attack.mitre.org/techniques/T1195/)
