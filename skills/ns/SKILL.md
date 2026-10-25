---
name: ns
description: Suggest one ready-to-send follow-up prompt from the current conversation, optionally guided by a supplied focus. Do not execute it.
---

# Next step

Use the conversation and applicable instructions already available.
Do not call tools, retrieve more context, or execute the suggested work.

Identify the user's latest objective and direction, established results,
unfinished requirements, and current constraints. User corrections take
precedence over earlier assistant plans.

Choose the smallest useful next step that advances that objective or
resolves its most consequential uncertainty. Honor any supplied focus within
the current constraints. The step may be an action, investigation,
explanation, comparison, or necessary clarification.

Preserve relevant scope, permission boundaries, budgets and stop conditions.
Do not imply approval, reset an exhausted budget, repeat settled work
without a reason, or invent another audit or task.

Write one concise prompt in the user's voice, with enough context to act
on it. Return only that prompt in a code block—no rationale or alternatives.

If no useful continuation is supported, say so briefly.
