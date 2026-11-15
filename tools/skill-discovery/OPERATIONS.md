# Local skill discovery monitor

These instructions describe a proposed deployment. Committing this repository or
merging a PR does not install or start the monitor. Publication and activation
each require explicit authority. The monitor reports a post-merge GitHub commit
status; it cannot block a merge or report while this host or GitHub is offline.

## What it owns

The optional Linux user-level timer runs one bounded process per target. It follows
the configured GitHub branch from a pinned baseline and examines commits in
first-parent order. For a new or changed skill in the selected profile, it compares
the installed package with the merged GitHub objects and asks a fresh Codex app
server for its enabled listing at the configured cwd, Codex home, and scope. A
new skill missing from all profiles fails. A commit without a relevant skill change gets a
success status stating that no profiled skill changed. It never installs a skill
or runs code from the source repository. An already-open desktop selector may
still need refreshing; this check does not prove that UI refreshed.

The monitor derives the `harness/local-skill-discovery/<digest>` status context from
the version, machine ID and effective user, target name, repository, branch, actor,
profile, scope, canonical runtime directories, and baseline. The checkpoint
uses the same context and rejects state from another target before contacting
GitHub. It verifies that the authenticated actor matches the configured actor before any
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
rewritten branch history, unavailable GitHub, permissions failure, or rate limit
also holds progress. Review the journal and monitor freshness; a missing status
is not a pass. The baseline and checkpoint must not be reset to skip history.

## Prepare and activate after approval

1. Merge the reviewed implementation through a Harness PR. Verify the merged
   commit, repository checks, and the prior binary/configuration if upgrading.
   Build the binary from that exact merged commit with the locked dependencies;
   verify its identity and retain the previous binary for rollback. Install it at
   `~/.local/lib/harness-skill-discovery/harness-skill-discovery`, outside any
   task worktree. Replace an existing binary atomically only after preserving it.
2. Check for existing units, configuration and state before writing. Fill in the
   reviewed [configuration](systemd/config.example.json) for this target. The
   single-target unit expects `~/.config/harness-skill-discovery/config.json`
   with mode `0600` and a user-owned mode `0700` directory at
   `~/.local/state/harness-skill-discovery`. Set `state_dir` to its absolute
   path. Give additional targets distinct config files, private state directories,
   and unit names with matching `ExecStart` and `ReadWritePaths` entries.
   Select an explicit trusted first-parent baseline on the configured branch.
   An empty state directory starts after that commit and publishes statuses
   for the subsequent commits. Choose the predecessor of the first commit
   that needs a result.
3. Create mode `0600` `~/.config/harness-skill-discovery/environment` with the
   actual `PATH` containing the trusted `codex` and `gh` executables. Confirm
   `GH_HOST=github.com gh api user` is the configured actor without printing
   credentials. Verify the configured cwd, installed root, and Codex home are
   the intended directories and provide the expected discovery scope.
4. Install the reviewed [service](systemd/harness-skill-discovery.service) and
   [timer](systemd/harness-skill-discovery.timer) under
   `~/.config/systemd/user/`, without overwriting unrelated units. Verify the
   units and their sandbox paths. After explicit activation authority, run
   `systemctl --user daemon-reload` and enable and start only
   `harness-skill-discovery.timer`. The user manager must remain available when
   the user is logged out if scheduled runs are required then.
5. Verify the timer is enabled and active, inspect its first service result and
   journal, and independently read the status on the merged implementation
   commit. This verifies scheduling and status publication, but a no-skill-change
   result does not prove a new-skill check. Run the documented read-only
   `remote-check` against a known skill release and verify its fresh Codex listing
   separately. Record the actual monitored host, cwd, merged identity, status and
   remaining limitations in the release review.

## Failure, recovery and retirement

Keep a copy of the prior binary, config, and complete state directory when
upgrading from a v1 monitor. A v1 checkpoint has no target identity, so v2 rejects
it. Use a fresh private state directory with a reviewed baseline and consider
the old pending commits when selecting it. The same procedure applies when
changing a field that determines the target context.

Use `systemctl --user status harness-skill-discovery.timer` and
`journalctl --user -u harness-skill-discovery.service` to inspect scheduling and
diagnostics. Compare the checkpoint's `last` with the configured branch on
GitHub and read pending commits' statuses under the checkpoint's context.
Resolve authentication, network, profile,
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

To roll back an upgrade, stop only this timer and allow its service to finish.
Restore the preserved binary, units, configuration, and corresponding checkpoint.
Reload the user manager and restart only the timer. If retiring the monitor,
disable and stop only its timer and service, preserve the checkpoint and release
evidence according to the owner's retention decision, and leave existing GitHub
statuses intact. Neither rollback nor retirement implies deleting installed
skills, production data, unrelated units, or recovery assets.
