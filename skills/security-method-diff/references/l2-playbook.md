# L2 Playbook: Security Diff

## Tools

- Use `git diff`, `git show`, `git diff --stat`, and rename-aware views to freeze the target.
- Use `rg` and language-aware navigation to trace changed symbols into callers, validators, persistence, and external interfaces.
- Run existing focused tests or analyzers against the frozen revision when they can distinguish old from new behavior.

## Practices

Review deleted and configuration files, not only added lines. Identify whether the change alters trust, validation, authorization, defaults, error handling, concurrency, cryptography, dependency resolution, or recovery. Separate a security regression from an unrelated pre-existing weakness.

## Protocol

1. Record repository path, base, head, dirty state, and diff hash or equivalent identity.
2. Classify each changed behavior, then rank security-sensitive ones.
3. Trace each ranked change through unchanged supporting code.
4. Compare prior and current invariants; seek a minimal counterexample.
5. Validate only findings caused or exposed by the change.
6. Report changed-file coverage and supporting-code depth.

## Evidence and falsification

Evidence must connect a changed line or removed guard to a reachable security consequence. Falsify by demonstrating preserved validation, unchanged effective behavior, unreachable deployment configuration, or a test/specification that distinguishes the versions.

## Dynamic boundary and failure behavior

Do not check out, reset, format, or clean the user's worktree. Use non-mutating Git reads. Run tests only when they do not rewrite tracked files; stop if the target diff changes during review.
