# Agent instruction versions

The root [`AGENTS.md`](../AGENTS.md) is the authoritative current policy. Files
under [`versions/`](versions/) are immutable, full-content snapshots of accepted
production versions. They are named `vNNN.md`, rather than `AGENTS.md`, so they
do not act as nested repository instructions.

## Versions

| Version | Status | Effective date | Original source | Change |
|---|---|---|---|---|
| [`v013`](versions/v013.md) | Current | 2026-09-01 | User-assigned tiered frontier search | Route explicit tiers into effort, blind operators, adjudication, and verification |
| [`v012`](versions/v012.md) | Superseded | 2026-08-30 | Evidence-aware architecture decisions | Preserve material decision lineage, proof state, and revisit triggers |
| [`v011`](versions/v011.md) | Superseded | 2026-08-21 | User-facing concise reporting | Hide routine internal execution details while preserving material risks and decisions |
| [`v010`](versions/v010.md) | Superseded | 2026-08-21 | 50:50 implementation gate | Require five measurable simplicity and five measurable elegance deliverables |
| [`v009`](versions/v009.md) | Superseded | 2026-08-19 | Concise communication format | Make routine agent communication compact without requiring a literal label |
| [`v008`](versions/v008.md) | Superseded | 2026-08-19 | TL;DR communication format | Make routine agent communication compact without weakening evidence or receipts |
| [`v007`](versions/v007.md) | Superseded | 2026-08-17 | Zero-tolerance quality gate | Reject waste and false-success mechanisms at admission |
| [`v006`](versions/v006.md) | Superseded | 2026-08-13 | Harness policy foundation | Consolidate sovereign policy, efficiency, and learning controls |
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
v007 702dc03ceffadee86771d81ca7f54666bf869ab7c0ce9b34803740c6f133d13f
v008 1b128f9b88f7acf5e979dbf39b391d7bd4a843073247bc3b9983543189a13d51
v009 097e5512082e70fefc53936372710c20481e82655ca65f9ef94aa2ff97bf7578
v010 bf7685f0873c8310fbff0bb43638782c336c1b9992ccf5995c86c7f9c7e2dd91
v011 e481090ef40e5c5e8fdf7264c58f3fc4abc9268c86b26c51a10b40acea411ffe
v012 dc83714284302740457db4e72cb9e0ac6f14d623098fe8ce17bd85cc5b9caeb5
v013 ebcc0d2f8c6a4a89b4cfaf5a4f8326bc02b099bf13a1acb5ff3ed558c3bb23e4
```

For every future root-policy change:

1. Assign the next sequential version.
2. Make the policy change in root `AGENTS.md`.
3. Copy the final file byte-for-byte to `versions/vNNN.md`.
4. Add the version here and mark the previous version superseded.
5. Verify the current snapshot with `cmp` and `sha256sum`.

Never modify a historical snapshot. A correction creates a new version.
