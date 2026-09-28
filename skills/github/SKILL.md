---
name: github
description: Orient a GitHub repository and keep local changes uncommitted unless asked.
---

# github

Use this skill when a repository URL, GitHub repository, pull request, issue,
branch, rename, ruleset, or publication boundary is in scope.

- resolve the exact owner, repository, visibility, default branch, remote URL,
  local checkout, and ref tips before acting
- inspect local and remote status and preserve pre-existing changes
- use connector or read-only GitHub inspection for repository context
- treat rename, local move, commit, push, pull-request creation, merge, settings,
  rulesets, and other publication as separate actions requiring explicit intent
- before rename, freeze target identity and collision state; after rename verify
  new and redirect URLs, visibility, description, default branch, local path,
  remote URL, and rollback constraint
- treat force push, ref deletion, history rewrite, ruleset bypass or disablement,
  and protection weakening as destructive boundaries; resolve exact targets and
  require explicit authority immediately before the action
- prefer pull requests and required checks; verify check identity and result,
  merge method, merge commit, active rules, and consumer-visible repository state
- when a repository declares a production runtime manifest, treat pull requests
  as the source-admission boundary even if hosting cannot enforce protection:
  use a task branch/worktree, run its canonical check, merge the PR before
  activation, and never let production depend on the task worktree
- use direct `main` only with explicit break-glass authority; record the reason,
  prior artifact, recovery action, and follow immediately with a reconciliation PR
- before removing a worktree, disposition dirty/untracked state, branch ancestry,
  runtime references, and recovery evidence; ancestry or a merged branch alone is
  not proof that removal is safe
- never infer remote publication from a local commit or successful command
- report exact local path, branch and tip, files and gross diff, PR and checks,
  merge/settings state, deleted refs with prior hashes, and recovery path
