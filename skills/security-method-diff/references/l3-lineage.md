# L3 Lineage: Security Diff

## Pressure

Repository-wide review is too expensive for every change, while line-only review misses security effects carried through unchanged code. Change review evolved toward behavior-centered analysis anchored to an exact revision pair.

## Method response

The method treats the diff as the attribution boundary and the surrounding program as evidence. It asks what behavior changed, then traces that delta into existing owners and consumers.

## Current shape

Modern diff review combines precise Git identity, reviewer attention support, targeted analysis, and regression tests. Its strongest claim is about the change, not the whole repository.

## Inherited strengths

- Fast feedback for pull requests and releases.
- Natural old-versus-new falsification oracle.
- Clear attribution and rollback boundary.

## Known failure modes

- Reviewing only added lines.
- Ignoring deleted configuration, schemas, or dependency metadata.
- Allowing the diff to move during review.
- Reporting pre-existing weaknesses as change-introduced.

## Primary anchors

- [OWASP Secure Code Review](https://cheatsheetseries.owasp.org/cheatsheets/Secure_Code_Review_Cheat_Sheet.html)
- [Less is More: Supporting Vulnerability Detection During Code Review](https://arxiv.org/abs/2202.04586)
- [NIST IR 8397](https://csrc.nist.gov/pubs/ir/8397/final)
- [GitHub security features](https://docs.github.com/en/code-security/getting-started/github-security-features)
