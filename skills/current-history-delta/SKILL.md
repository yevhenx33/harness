---
name: current-history-delta
description: Design, review, implement, or verify efficient panel analytics using the Current + Selected History + Delta Stream model. Use for chart/info APIs, range selectors, WebSocket deltas, rollup-plus-tail assembly, 24h changes, render-churn or memory-growth investigations, and bounded frontend/backend retention.
---

# Current History Delta

## Overview

Use the Current + Selected History + Delta Stream model to keep analytics pages live without repeatedly downloading and re-rendering full page snapshots. Separate latest state, selected history, and live deltas into independently cached and updated streams.

## Model

Treat a page as a composition of streams, not one snapshot:

```text
current endpoint       latest scalars, tables, daily 24h changes
history endpoint       one panel, one selected range, closed rollup rows
websocket delta stream heartbeat, current patches, tail replacement, invalidation
optional endpoint      expensive or hidden panels loaded only when needed
```

Default API shape:

```text
GET /page/current
GET /page/history?panel=<panel>&range=<range>
GET /page/optional?panel=<panel>&...
WS  /page/ws
```

Default WebSocket messages:

```json
{ "type": "heartbeat", "anchor": {}, "status": "OK" }
```

```json
{ "type": "tail.replace", "panel": "assets", "rows": [], "tailVersion": "..." }
```

```json
{ "type": "current.patch", "sections": { "overview": {}, "table": [] }, "currentVersion": "..." }
```

```json
{ "type": "history.invalidate", "panel": "assets", "range": "1m", "historyVersion": "..." }
```

## Invariants

- Fetch selected history once per visible panel/range.
- Never refetch a full page snapshot for normal WebSocket messages.
- Apply `heartbeat` to metadata only.
- Apply `tail.replace` by replacing or appending the latest in-progress point for that panel.
- Apply `current.patch` only to changed current sections.
- Refetch history only on selector changes or matching `history.invalidate`.
- Load expensive optional streams only when their panel/tab is visible or requested.
- Preserve unchanged array/object identities so chart libraries do not rebuild.
- Derive displayed 24h changes from daily/current-daily data, independent of selected chart range.
- Keep one immutable closed-history array plus at most one replaceable open tail per entity, panel, and resolution.
- Only the message that owns a domain fact may change its identity: `tail.replace` changes its named panel; scalar, detail, status, and unrelated messages do not.
- Bound every retained dimension: history cache keys, open tails, versions, queues, reconnect timers, listeners, and optional-panel state.

## Range Contract

Use one resolution for every point in a selected range:

```text
1w  -> hourly rows
1m  -> hourly rows
1y  -> daily rows
all -> weekly rows
```

Do not mix daily and weekly rows inside one returned series. If a page needs a live in-progress point, send it as a separate tail stream or mark it explicitly.

## Frontend Workflow

1. On initial page load, fetch `current` plus the histories for visible panels only.
2. On selector change, fetch only that panel/range and keep other panel data stable.
3. On WebSocket `heartbeat`, update freshness/status without touching chart arrays.
4. On WebSocket `tail.replace`, update only the target panel tail.
5. On WebSocket `current.patch`, merge only patched current sections.
6. On WebSocket `history.invalidate`, refetch the matching cached history key only if it is visible or already cached.
7. Use full snapshot fetch only as a recovery path after reconnect failure, schema mismatch, or unrecoverable version gap.

For any live-chart implementation, rerender audit, or browser-memory investigation, read [references/render-identity-and-retention.md](references/render-identity-and-retention.md) before changing code.

## API Design Checks

- Include stable versions or hashes per stream: `currentVersion`, `historyVersion`, `tailVersion`.
- Support conditional fetches with `ETag` or equivalent version checks for history.
- Keep response payloads panel-scoped; avoid returning hidden chart panels or all flow windows by default.
- Bound tail payload size.
- Keep daily 24h changes in `current`, not in selected history responses.
- Make optional panels addressable by stable cache keys.

## Performance Budget

Use these targets unless the repo has stricter budgets:

```text
heartbeat                 under 2 KB
tail.replace              bounded to current tail rows only
current.patch             changed sections only
history fetch             one panel and one range only
normal live update        zero full-page REST snapshot fetches
unchanged chart identity  stable across heartbeat/current-only updates
```

When auditing, report wire bytes, row counts by stream, WebSocket frame size, refetch frequency, and which React/chart arrays are recreated.

## Completion Gate

Do not call the connection efficient from payload size or heap snapshots alone. Verify at the consumer boundary that:

- heartbeat, scalar, detail, and unrelated frames retain strict (`===`) identity for every untouched chart input;
- one `tail.replace` changes only its named panel and an equivalent repeated version is a no-op;
- chart props besides data are stable enough for the memo boundary to work;
- normal deltas cause no history/full-page REST refetch;
- reconnect, selector change, invalidation, and unmount have explicit cleanup and recovery behavior; and
- a representative multi-update soak shows bounded retained state after garbage collection, with DOM nodes and listeners returning to a stable baseline.

Report inability to observe actual component commits as a verification gap; parent renders, DOM mutations, and a flat heap are useful evidence but are not substitutes for chart render/commit or strict-identity evidence.
