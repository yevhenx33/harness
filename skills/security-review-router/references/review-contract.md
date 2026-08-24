# Shared Security Review Contract

Every L1 method returns evidence against an exact target. The method may suggest another method, but it never starts it.

## Required header

- **Method:** exact L1 skill name.
- **Target:** repository, component, document, artifact, or endpoint.
- **Revision:** immutable commit, diff range, artifact digest, document version, or explicit `unavailable` with consequence.
- **Scope:** included and excluded paths, components, interfaces, environments, and time window.
- **Authority:** read-only operations and any separately authorized safe dynamic checks.
- **Primary question:** one threat, invariant, path, property, or artifact-chain claim the review can falsify.

## Evidence record

For each material observation record the source location or command, direct observation, supported claim, confidence boundary, and a falsifier or missing evidence. Derived output, scanner output, a passing test, and a successful command are evidence inputs, not independent proof by themselves.

## Findings

Each finding contains:

- title and exact location;
- status: `validated`, `plausible`, `invalid`, or `blocked`;
- attacker or failure prerequisite;
- violated expectation or property;
- shortest supported path to consequence;
- evidence and falsifier;
- severity only when reachability and impact are supported;
- authoritative remediation boundary, without applying a fix.

Do not render unknown, partial, stale, inaccessible, or contradictory evidence as safe, complete, or zero.

## Required footer

- **Coverage:** reviewed owners, paths, states, and applicable boundary cases.
- **Gaps:** unreviewed scope, assumptions, unavailable evidence, and inconclusive claims.
- **Suggested next method:** zero or one explicit `$security-method-...` invocation with the named evidence gap it would close.
- **Dynamic effects:** commands or effects performed; state `none` for a read-only review.

A review with no validated findings reports its actual coverage and gaps; it does not claim the target is secure.
