# L3 Lineage: CTF / Exploit-First Review

## Pressure

Security review often ends with a plausible weakness whose practical reachability remains disputed. Capture-the-flag exercises made the terminal condition explicit and rewarded a working proof against an intentionally isolated target.

## Method response

Exploit-first review adopts the strong oracle and tight boundary of a CTF: the proof either reaches a predetermined state under fixed rules or it does not.

## Current shape

The approach is valuable for training, product security challenges, regression proofs, and bounded exploit validation. Its conclusions remain limited to the modeled target, configuration, and attacker position.

## Inherited strengths

- Produces a direct, reproducible oracle.
- Forces candidate paths to survive real execution constraints.
- Creates compact regression material when retained safely.

## Known failure modes

- Generalizing challenge success to a different deployment.
- Mistaking a crash for controlled impact.
- Spending excessive time on exploit mechanics after the property is proven.
- Allowing competitive framing to weaken authorization or safety.

## Primary anchors

- [MITRE eCTF 2026: Attack Flags](https://rules.ectf.mitre.org/2026/flags/attack_flags.html)
- [MITRE eCTF Rules](https://rules.ectf.mitre.org/)
- [OWASP CTF](https://ctf.owasp.org/)
- [NIST SP 800-115: Technical Guide to Information Security Testing](https://csrc.nist.gov/pubs/sp/800/115/final)
