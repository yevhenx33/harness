---
name: implementation-slice
description: Implement one bounded subsystem change with focused verification.
---

# implementation-slice

Use the smallest useful form of:

map -> architecture checkpoint -> implementation -> verification

- own only the directory or subsystem named by the user
- preserve unrelated user changes
- reuse existing patterns and avoid speculative abstractions
- do not deploy, restart, migrate, or write production data unless explicitly
  authorized
- add the smallest tests that prove changed observable behavior
- report files changed, checks run, coverage availability, performance impact,
  and deployment status
