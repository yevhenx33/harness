# L3 Lineage: Risk-Based Audit

## Pressure

General code review cannot inspect every path equally, while defect counts alone do not express reachability or consequence. Security assurance therefore combined systematic secure-code review with explicit risk prioritization and verification guidance.

## Method response

The method maps assets and exposure first, ranks plausible failure paths, then spends evidence budget on the paths most capable of violating security properties. It retains broad coverage language but refuses to equate sampling with completeness.

## Current shape

Modern practice combines manual tracing, targeted static or dynamic tools, threat context, and explicit coverage accounting. Tools produce leads; the auditor owns prioritization and the final evidence claim.

## Inherited strengths

- Works when the target is broad and the vulnerability class is unknown.
- Makes review cost and uncovered risk visible.
- Provides a default method from which narrower methods can be selected.

## Known failure modes

- Ranking by generic severity lists rather than the deployed system.
- Spending time on tool output that lacks reachability or impact.
- Treating reviewed samples as repository-wide assurance.
- Using a threat model so abstract that it cannot rank code paths.

## Primary anchors

- [OWASP Secure Code Review Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Secure_Code_Review_Cheat_Sheet.html)
- [NIST IR 8397: Developer Verification of Software](https://csrc.nist.gov/pubs/ir/8397/final)
- [NIST SSDF SP 800-218](https://csrc.nist.gov/pubs/sp/800/218/final)
- [Toward Effective Secure Code Reviews](https://arxiv.org/abs/2311.16396)
- [Less is More: Supporting Vulnerability Detection During Code Review](https://arxiv.org/abs/2202.04586)

The full evidence registry is maintained outside runtime skills at `/home/ubuntu/research/security-method-library/source-registry.jsonl`.
