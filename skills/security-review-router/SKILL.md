---
name: security-review-router
description: Recommend one security review method for a repository, design, change, finding, exploit target, property, input surface, proof, or software supply chain. Use for method selection only; never start the review automatically.
---

# Security Review Router

Recommend a method and stop. Do not perform a scan, load an L1 skill, open its L2/L3 references, search the web, mutate files, or claim findings.

## Routing inputs

Extract only what is already known: exact target type, review goal, available evidence or prerequisites, budget, and explicit exclusions. Use at most three local read-only orientation calls when the target type cannot be determined from the request. Do not inspect broadly merely to improve confidence.

Read [references/method-index.yaml](references/method-index.yaml) as the authoritative method graph. Do not load sibling skills while routing.

## Decision procedure

For every eligible node, score the request using the index vocabulary:

- `+4` exact target-type match.
- `+3` exact review-goal match.
- `+2` available evidence or prerequisite match.
- `+1` budget fit.
- `-4` unmet prerequisite.
- `-5` explicit `not_for` match.

Prefer the narrower, lower-cost method on a score tie. If a material tie remains, ask one clarification and do not recommend arbitrarily. State uncertainty when essential context is absent.

Recommend at most one complement, and only when a typed `complements` edge fills a named evidence gap. Horizontal links never authorize another method to run.

## Output contract

Keep the response below 150 words and emit only:

```text
Primary method: ...
Why it fits: ...
Optional complement: ...
Why the closest alternative was rejected: ...
Next request: Use $security-method-... to review <exact target/revision> for <goal>.
```

Omit the complement line when none is necessary. If clarification is required, ask one question instead and make no method invocation. The next request is a suggestion for a future explicit user request, never an execution step.
