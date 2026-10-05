# Render Identity and Retention

Use this reference for WebSocket-fed charts, rerender audits, and browser-memory work.

## Governing invariant

Closed history is immutable. The live tail is one replaceable value. A message may change only the representation it owns.

```text
heartbeat       metadata only
current.patch   named scalar/table sections only
detail.replace  named detail state only
tail.replace    one named panel tail only
invalidate      one history cache key only
```

The chart-visible series is `closed history + current tail`, not a growing log of WebSocket frames. Never append every received tail, retain prior snapshots, or rebuild all panel arrays from a current snapshot.

## Minimal frontend representation

For each `(entity, panel, resolution)` retain:

- one immutable closed-history array;
- zero or one open-tail payload;
- the accepted history and tail version/hash; and
- one materialized chart array only when the chart library requires it.

Materialize only when closed history, the relevant tail, or resolution changes. Suitable mechanisms include `useMemo` for component-local state or a GC-safe cache keyed by immutable history and tail identities. A `WeakMap` cache must not have a permanent registry of entity keys. Never mutate arrays used as cache keys.

Preserve exact identity for all unaffected series. Avoid broad normalization after every frame: normalization that maps or spreads every row is a rebuild even if values are equal.

An equivalent or older tail version is a no-op. On a newer tail, replace the open bucket if it shares the resolution bucket; otherwise append one new open bucket. When history seals that bucket, accept the authoritative closed history and discard the superseded open tail.

## Memo boundary

Stable data alone does not prevent a chart render. Confirm every prop reaching the memoized chart is stable:

- define static series, legends, domains, and reference-line arrays outside the component;
- memoize derived filtered rows by the source-array identity and selector;
- stabilize callbacks or formatter functions when they are chart props;
- avoid inline objects, arrays, and JSX used as chart props; and
- use a memoized chart component or an equivalent subscription/selector boundary.

It is acceptable for current-value cells and the page shell to render after a scalar patch. The expensive chart must not commit unless its data, selector, dimensions, or interaction state changed.

## Strict replacement and TTL policy

Name and bound each growing dimension. Defaults are design requirements, not universal numeric TTLs:

- Open tails: one per active entity/panel/resolution; replacement, never accumulation.
- Selected history: cache by stable entity/panel/range/version; use bounded LRU/TTL or evict on the product's established policy.
- Resolution/entity change: detach obsolete subscriptions and make prior tail/materialization state collectible.
- Optional panels: fetch and retain only when visible or explicitly cached within the bound.
- Versions/dedupe keys: retain only the accepted frontier plus bounded recovery metadata.
- WebSocket queues: bounded or latest-wins; never allow UI work to lag behind an unbounded frame backlog.
- Timers/listeners/connections: exactly one owned lifecycle; clear on reconnect, dependency change, and unmount.

Do not invent TTL numbers without measuring update frequency, navigation behavior, history-fetch cost, and the repository's memory budget. Unavailable or stale data remains explicit; eviction never converts it to zero.

## Merge and ordering rules

Reject stale versions before allocation. Apply sparse patches without normalizing untouched charts. Do not derive a chart tail from `current.patch` when an explicit `tail.replace` exists; that produces duplicate work and split ownership.

On a version gap, schema mismatch, or ambiguous ordering, stop incremental commit and use the defined resync path. A reconnect must not allow an expired socket, fetch, or timer to commit after its replacement owns the page.

## Direct verification

Use strict identity tests as the primary cheap oracle:

1. Prime the page with closed histories and open tails.
2. Apply heartbeat, scalar, and detail frames.
3. Assert `next.panelRows === previous.panelRows` for every chart.
4. Apply a tail for one panel and assert only that panel changes.
5. Replay the same/older tail and assert the whole update or named series is unchanged.
6. Seal the open bucket with new closed history and assert no duplicate timestamp remains.

Also verify the real memo boundary. Prefer a render/commit counter or React profiler around the chart. DOM mutation counts can support the result but can miss canvas work or count unrelated UI.

For a representative browser soak:

- observe multiple heartbeats, current patches, detail replacements, and tail replacements;
- count unexpected REST history/current refetches;
- record WebSocket types, bytes, and panel fanout;
- sample post-GC JS heap and retained-object evidence, not RSS alone;
- confirm DOM node, document, listener, timer, and connection counts stabilize; and
- navigate or unmount, then confirm subscriptions and retained state are released.

Do not claim "no leak" from a short flat sample. State the duration, frame counts, garbage-collection method, baseline/final values, and what remains unobserved. A successful build or HTTP 200 proves neither render efficiency nor retention correctness.

## Expected report

Report the owner and identity rule, which messages changed which panels, REST refetch count, actual chart commits when observable, post-GC memory/DOM/listener trend, ordering/recovery behavior, and any unverified boundary. Separate fixed computational churn from demonstrated retained-memory leakage.
