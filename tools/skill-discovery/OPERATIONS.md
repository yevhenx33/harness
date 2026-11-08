# Local skill discovery monitor

These instructions describe a proposed deployment. Committing this repository or
merging a PR does not install or start the monitor. Publication and activation
each require explicit authority. The monitor reports a post-merge GitHub commit
status; it cannot block a merge or report while this host or GitHub is offline.

## What it owns

The user-level timer runs one bounded process on this host. It follows Harness
`main` from a pinned baseline and examines commits in first-parent order. For a
new or changed profiled skill, it compares the whole installed package with the
merged GitHub objects and asks a fresh Codex app server for its enabled user
listing in the configured working directory. A new skill missing from the
activation profile fails. A commit without a relevant skill change gets a
success status stating that no profiled skill changed. It never installs a skill
or runs code from the source repository. An already-open desktop selector may
still need refreshing; this check does not prove that UI refreshed.

The monitor uses the `harness/local-skill-discovery` status context and verifies
that the authenticated GitHub actor matches the configured actor before any
status write. An existing status under that context from another actor stops
processing. The configured `gh` account needs repository read access and commit
status write permission. Prefer a dedicated least-privilege credential where
available; the existing account can have broader privileges. Do not place a
token in the example configuration or journal.

Each run handles at most two new commits and retries at most one failed check.
Status writes are read back before the checkpoint advances. An ambiguous write
is read back before a subsequent write. The private checkpoint has at most 128
pending failures; no unbounded repository clone or queue is retained. Each remote
check has limits of 128 GitHub calls, 48 MiB of downloaded responses and a
60-second deadline checked between calls; individual commands have a 20-second
timeout. Selected package content is limited to 32 MiB in total, 16 MiB per
package and 256 entries per package. The service additionally limits memory to
512 MiB, CPU to one core and its run to 10 minutes. These are limits, not measured
resource use.

The timer runs approximately every five minutes when its previous run has
finished. After an outage it catches up in order, two commits per run. A failure
is retried automatically; a later release of the same skill may mean an older
commit can no longer match the installed version, so historical failures can
remain pending. At 128 pending failures, processing stops rather than losing
evidence. The owner must reconcile them as described below. A bad checkpoint,
rewritten main history, unavailable GitHub, permissions failure, or rate limit
also holds progress. Review the journal and monitor freshness; a missing status
is not a pass. The baseline and checkpoint must not be reset to skip history.

## Prepare and activate after approval

1. Merge the reviewed implementation through a Harness PR. Verify the merged
   commit, repository checks, and the prior binary/configuration if upgrading.
   Build the binary from that exact merged commit with the locked dependencies;
   verify its identity and retain the previous binary for rollback. Install it at
   `~/.local/lib/harness-skill-discovery/harness-skill-discovery`, outside any
   task worktree. Replace an existing binary atomically only after preserving it.
2. Check for existing units, configuration and state at the paths below; do not
   overwrite unrelated files. Create `~/.config/harness-skill-discovery` and
   `~/.local/state/harness-skill-discovery` as user-owned mode `0700` directories.
   Install the reviewed [configuration](systemd/config.example.json) as
   `~/.config/harness-skill-discovery/config.json` with mode `0600`. The pinned
   baseline is the already released `$clean` merge; confirm it is an ancestor of
   current `main`. An absent state file starts there, so intervening commits are
   checked. Preserve an existing checkpoint across upgrades.
3. Create mode `0600` `~/.config/harness-skill-discovery/environment` with the
   actual `PATH` containing the trusted `codex` and `gh` executables and
   `CODEX_HOME=/home/ubuntu/.codex`. Confirm `gh api user` is the configured
   actor without printing credentials. Verify the Codex executable and the
   configured cwd and skill root exist and are the intended host locations.
4. Install the reviewed [service](systemd/harness-skill-discovery.service) and
   [timer](systemd/harness-skill-discovery.timer) under
   `~/.config/systemd/user/`, without overwriting unrelated units. Verify the
   units and their sandbox paths. After explicit activation authority, run
   `systemctl --user daemon-reload` and enable and start only
   `harness-skill-discovery.timer`. The user manager must remain available when
   the user is logged out; on this host user lingering was enabled when checked.
5. Verify the timer is enabled and active, inspect its first service result and
   journal, and independently read the status on the merged implementation
   commit. This verifies scheduling and status publication, but a no-skill-change
   result does not prove a new-skill check. Run the documented read-only
   `remote-check` against a known skill release and verify its fresh Codex listing
   separately. Record the actual monitored host, cwd, merged identity, status and
   remaining limitations in the release review.

## Failure, recovery and retirement

Use `systemctl --user status harness-skill-discovery.timer` and
`journalctl --user -u harness-skill-discovery.service` to inspect scheduling and
diagnostics. Compare the checkpoint's `last` with GitHub `main`, check pending
commits and read their status contexts. Resolve authentication, network, profile,
package or app-server failures at their owner. A missing or wrong installation
requires a separately authorized installation; this monitor only detects it.

If a historical pending commit cannot match a later installed release, or the
pending queue is full, preserve the checkpoint and identify each affected
commit and its existing failed status. With authority to operate the monitor,
stop only this timer and wait for its service to finish, preserve a copy of the
checkpoint, and remove only specifically reconciled historical commit IDs from
the `pending` JSON array. Keep `last` unchanged; do not remove unresolved entries
or change their GitHub statuses to success. Validate the JSON and private file
ownership/mode, then restart the timer and verify forward progress. Escalate a
rewritten history or ambiguous ownership rather than skipping commits.

To roll back an upgrade, stop only this timer and allow its service to finish;
restore the preserved binary, units and configuration, retain the checkpoint,
reload the user manager, and restart only the timer. If retiring the monitor,
disable and stop only its timer and service, preserve the checkpoint and release
evidence according to the owner's retention decision, and leave existing GitHub
statuses intact. Neither rollback nor retirement implies deleting installed
skills, production data, unrelated units, or recovery assets.
