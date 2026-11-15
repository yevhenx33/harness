# Local Harness evidence

Build the tool with Rust 1.75+ on Linux. Supply exact local files. The tool
writes JSON to stdout and diagnostics to stderr. Its commands compare or report
evidence and do not change a gate, install skills, or retire transcripts.

~~~sh
cargo run --locked --manifest-path tools/harness-evidence/Cargo.toml -- check-run PLAN.json OBSERVED.json
cargo run --locked --manifest-path tools/harness-evidence/Cargo.toml -- router-replay \
  LOCAL_ROUTER_PLAN.json RETAINED_GATE_DIR
cargo run --locked --manifest-path tools/harness-evidence/Cargo.toml -- archive MANIFEST.json
~~~

## Test evidence

Keep the schema-1 plan tied to the reviewed requirement and owning recipe.
Each guarantee names its owner, revision-relative file or symbol, observable
outcome, and required case IDs. The plan is an object with a schema number,
an expected execution identity of the same shape as the observation below,
and a guarantees array with entries like this.

~~~json
{
  "id": "writer-ownership",
  "owner": "Publication component",
  "source": "src/publication.rs::commit",
  "outcome": "The current writer commits and superseded writes leave durable state intact.",
  "cases": ["replacement_commits", "stale_write_refused"]
}
~~~

Several guarantees may share a case. Bind the run to the exact source or
artifact, package, target, configuration, executable SHA-256, and run attempt.

An observed schema-1 record has this shape. Populate every execution field from
the runner or its verified records. The plan and the observed record have
independent producers.

~~~json
{
  "schema": 1,
  "execution": {
    "candidate": "source-and-bundle",
    "package": "service",
    "target": "bin:service",
    "configuration": "release-linux-fixture-a",
    "executable_sha256": "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
    "run": "run-123:attempt-1"
  },
  "outcome": "passed",
  "cases": [
    {"id": "replacement_commits", "outcome": "passed"},
    {"id": "stale_write_refused", "outcome": "passed"}
  ]
}
~~~

Case and run outcomes are passed, failed, skipped, or incomplete. A JSON
report is returned with exit 0 even when the verdict is failed or unverified.
Use the JSON verdict for a decision. Execution mismatches are listed at run
level. A matching execution distinguishes missing, skipped, incomplete, and
failed cases. Invalid or contradictory input exits 2.
The verifier assumes the identity and per-case evidence came from an admitted
producer. It neither authenticates that producer nor substitutes for the
behavioral assertion. Validate producer and artifact provenance at any trust
boundary.

The Router adapter is a report-only replay for the retained V5 gate-r5 stage
021. It reads the stage recipe, ledger, first server auth test group, candidate
receipt, and exact-role executable inventory. It hashes the retained test
binaries. The other three verdicts use modified observations of that run.
The replay does not execute Router, start services, or produce a live release
receipt. Use the locally retained plan and artifacts for that replay. The
repository verifies the adapter on an independently specified synthetic
recipe and evidence set. [Qualification cases](../../docs/evaluations/harness-invariants-cases.md)

## Archive manifest

Supply a frozen manifest from the archive owner. Each chat lists its required
fragments. A fragment path is absolute or relative to the manifest. A selected
tar member name is relative to the archive and exact. The tool reads JSONL,
JSONL.ZST, ordinary tar, and Tar.ZST, with lowercase suffixes. Selected tar
members can contain plain or Zstandard JSONL. The supported tar paths are
standard and GNU long names. PAX extensions and GNU sparse entries stop
coverage of that container. Links are never extracted or followed. Supported
transcript fragments begin with matching session metadata and contain
newline-terminated JSON records.

~~~json
{
  "schema": 1,
  "limits": {"seconds": 30, "stored_bytes": 134217728, "decoded_bytes": 268435456},
  "skills": ["clean", "scrutiny"],
  "chats": [
    {
      "id": "archived-parent",
      "fragments": [
        {"path": "part-1.jsonl", "native_events": true},
        {"path": "backup.tar.zst", "member": "transcripts/part-2.jsonl.zst", "native_events": true}
      ]
    }
  ],
  "references": {
    "complete": false,
    "roots": ["active-fork", "required-backup"],
    "edges": [["active-fork", "archived-parent"], ["required-backup", "active-fork"]]
  }
}
~~~

The owner declares whether each fragment supports native event observation.
Native load recognition requires matching typed selection and source metadata
on a supported user-message envelope. Explicit skill markers in user text and
skill paths in tool-call arguments are reported as requests and path
references, including paths in structured argument values. Skill catalog
metadata does not establish a load. Use an independent behavioral check
for whether a skill helped. Deduplication uses chat-local message or call
identity with the event kind and selected skill. Unidentified observations
and observations without known turn IDs have their own counters. An unresolved
copied turn is excluded from known turns. A conflicting native selection is
excluded from identified loads. A recognized observation before a truncated
record remains visible with partial coverage. Fragment issues explain malformed
records and coverage lost during a read.

Declared root-to-dependency links can cross active forks and backup
generations. Only the supplied metadata is traversed. Reachable items are
protected by reference; unreferenced items remain unknown when the reference
inventory is incomplete. A complete manifest means complete for that
snapshot. A finding of no declared reference is input to a separate cleanup
review, which owns recovery evidence, current-state rechecks, and authority.

Physical allocation is read from the named file's inode and charged once for
duplicate paths. A tar source's allocation includes the whole container.
Decoded transcript and tool-output bytes describe composition. Reading part of
a compressed archive or obtaining a compact report does not establish how many
physical bytes could be reclaimed.

The scan bounds stored and decoded work, including traversal of unselected tar
members and decompression of selected nested members. It uses cooperative time
checks on reads; use an external deadline if storage can block. The supplied
time, stored-byte, and decoded-byte limits must be positive. Manifest input,
record size, decoder window, event counters, chat population, and printed
reference paths are also bounded. A scan stopped by an input limit reports
partial or unreadable coverage and a reason. Use only the output for the
declared scope. No chat bodies or tool outputs are printed.
