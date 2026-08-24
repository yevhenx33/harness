# L3 Lineage: Red-Team Audit

## Pressure

Checklist and control reviews can miss how an attacker composes individually acceptable behaviors across boundaries. Penetration testing established controlled technical testing; red-team exercises added objectives, adaptive adversary behavior, and evaluation of detection and response.

## Method response

The method begins with an attacker objective, constrains it through rules of engagement, and searches for the shortest feasible abuse chain rather than maximizing tool output.

## Current shape

Modern red-team work uses threat intelligence and adversary emulation to make behavior realistic, while preserving explicit authorization, safety controls, evidence standards, and recovery.

## Inherited strengths

- Exposes compositional failures spanning systems and controls.
- Tests whether defensive assumptions survive an attacker-led sequence.
- Reveals detection and recovery gaps alongside prevention gaps.

## Known failure modes

- Treating ATT&CK technique coverage as evidence of exploitability.
- Optimizing for surprise or volume instead of the stated objective.
- Crossing authority boundaries in pursuit of realism.
- Reporting isolated observations as a complete attack chain.

## Primary anchors

- [NIST Glossary: Red Team Exercise](https://csrc.nist.gov/glossary/term/red_team_exercise)
- [NIST SP 800-115: Technical Guide to Information Security Testing](https://csrc.nist.gov/pubs/sp/800/115/final)
- [MITRE: Adversary Emulation and Red Teaming](https://attack.mitre.org/resources/get-started/adversary-emulation-and-red-teaming)
- [Google Security Blog: Chrome Security Review](https://security.googleblog.com/2023/07/a-look-at-chromes-security-review.html)
