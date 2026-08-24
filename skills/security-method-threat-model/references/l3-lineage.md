# L3 Lineage: Threat Modeling

## Pressure

Vulnerability checklists do not establish which actors, assets, and trust changes matter in a particular system. Threat modeling emerged to make those assumptions explicit before detailed assurance work.

## Method response

The method represents system flows and boundaries, then derives threats from attacker capabilities and desired security properties. Frameworks such as STRIDE supply prompts, not system truth.

## Current shape

Practice has moved toward lightweight, continuously updated models anchored to actual architecture and code. Empirical work also shows that excessive formality and maintenance cost can cause models to drift or be abandoned.

## Inherited strengths

- Establishes review priorities before tool selection.
- Makes attacker and deployment assumptions inspectable.
- Connects design decisions to downstream attack-path and red-team work.

## Known failure modes

- Diagramming components without assets or adversaries.
- Treating taxonomy completion as threat coverage.
- Reusing a model across changed revisions or deployments.
- Producing threats too abstract to test.

## Primary anchors

- [NIST SP 800-154: Data-Centric Threat Modeling](https://csrc.nist.gov/pubs/sp/800/154/ipd)
- [OWASP Threat Modeling Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html)
- [OWASP Threat Modeling Project](https://owasp.org/www-project-threat-modeling)
- [Microsoft STRIDE](https://learn.microsoft.com/en-us/archive/msdn-magazine/2006/november/uncover-security-design-flaws-using-the-stride-approach)
- [Investigating Threat Modeling Practices](https://www.usenix.org/conference/usenixsecurity25/presentation/kaur)
