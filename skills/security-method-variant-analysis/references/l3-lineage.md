# L3 Lineage: Variant Analysis

## Pressure

Vulnerabilities recur through copied code, parallel implementations, incomplete fixes, and equivalent state transitions. Text search finds surface resemblance but misses renamed or restructured siblings and over-reports harmless matches.

## Method response

Variant analysis turns a validated defect into a reusable causal predicate and searches progressively richer program representations for equivalent instances.

## Current shape

The method combines vulnerability taxonomies, root-cause analysis, code search, static queries, and focused validation. Modern practice also checks whether a patch removed the cause everywhere rather than only at the reported location.

## Inherited strengths

- Converts one expensive finding into broader coverage.
- Detects incomplete fixes and parallel vulnerable implementations.
- Produces reusable queries when the predicate is stable.

## Known failure modes

- Generalizing from an unvalidated seed.
- Encoding names or syntax instead of causal structure.
- Treating query hits as confirmed vulnerabilities.
- Expanding scope before measuring false positives and blind spots.

## Primary anchors

- [NIST SP 800-231: The Bugs Framework](https://csrc.nist.gov/pubs/sp/800/231/final)
- [NIST Taxonomy of Software Flaws](https://www.nist.gov/itl/ai/ai-standards-and-guidelines-group/taxonomy-software-flaws)
- [Google Project Zero: CVE-2019-1367 Root Cause Analysis](https://googleprojectzero.blogspot.com/p/rca-cve-2019-1367.html?m=1)
- [MITRE CWE Root Cause Mapping Guidance](https://cwe.mitre.org/documents/cwe_usage/guidance.html)
