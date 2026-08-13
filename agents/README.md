# Agent instruction versions

The root [`AGENTS.md`](../AGENTS.md) is the authoritative current policy. Files
under [`versions/`](versions/) are immutable, full-content snapshots of accepted
production versions. They are named `vNNN.md`, rather than `AGENTS.md`, so they
do not act as nested repository instructions.

## Versions

| Version | Status | Effective date | Original source | Change |
|---|---|---|---|---|
| [`v006`](versions/v006.md) | Current | 2026-08-13 | Harness policy foundation | Consolidate sovereign policy, efficiency, and learning controls |
| [`v005`](versions/v005.md) | Superseded | 2026-08-11 | This archive change | Add full snapshot versioning |
| [`v004`](versions/v004.md) | Superseded | 2026-08-11 | `d765e48` | Add latency performance thresholds |
| [`v003`](versions/v003.md) | Superseded | 2026-08-05 | `74688eb` | Add archived-thread context to mapping |
| [`v002`](versions/v002.md) | Superseded | 2026-08-02 | `92d0955` | Generalize the operating principles |
| [`v001`](versions/v001.md) | Superseded | 2026-07-22 | `435aa1c` | Add initial repository instructions |

Only versions accepted into `main` belong in this index. Branch experiments are
drafts until merged and do not receive production version numbers.

## Integrity

```text
v001 a026ba2aeb85f4aab0254a48334b9b30b5c2933783ddcc751b65e0fa242a2cc0
v002 0597f52a5b7afdca3d13e6bee031b4ffa6dbdcc4f82fae753c043369b880dfca
v003 9dcc62e49548e7e7412d49d7c80ee8f50a0c0c7054d26175137ee21df9a209e3
v004 d66e0c6f83441c847a9244b784076de52bfeff277b811b17165c40702f1e9688
v005 8c112459ee721a1b51b9cc5f03e330bc04b09616d8fb40b7c5723904c875c0fc
v006 c2fc8057d072282b5cb7f2f2205f773f21bb983023f5aeecff7556345eec1b0c
```

For every future root-policy change:

1. Assign the next sequential version.
2. Make the policy change in root `AGENTS.md`.
3. Copy the final file byte-for-byte to `versions/vNNN.md`.
4. Add the version here and mark the previous version superseded.
5. Verify the current snapshot with `cmp` and `sha256sum`.

Never modify a historical snapshot. A correction creates a new version.
