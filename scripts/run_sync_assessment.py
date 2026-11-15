#!/usr/bin/env python3
"""Classify synchronization for an explicitly scoped Git target."""

import argparse
import json
import os
import re
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path

if __package__:
    from scripts.verify_policy import verify as verify_policy
else:
    from verify_policy import verify as verify_policy

ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "contracts" / "sync-assessment.json"
SCHEMA = ROOT / "schemas" / "sync-receipt.schema.json"


@dataclass(frozen=True)
class Target:
    root: Path
    remote: str = "origin"
    branch: str = "main"
    policy: bool = False
    remote_url: str = field(init=False, repr=False)

    def __post_init__(self) -> None:
        object.__setattr__(self, "root", self.root.resolve(strict=True))
        for value in (self.remote, self.branch):
            if not re.fullmatch(r"[A-Za-z0-9_.-]+(?:/[A-Za-z0-9_.-]+)*", value) or value.startswith("-"):
                raise ValueError("invalid remote or branch name")
        if git(self, "rev-parse", "--show-toplevel") != str(self.root):
            raise ValueError("target root must be the exact checkout root")
        for ref in (self.ref, self.tracking_ref):
            if not succeeds(self, "check-ref-format", ref):
                raise ValueError("invalid target ref")
        object.__setattr__(self, "remote_url", git(self, "remote", "get-url", self.remote))

    @property
    def ref(self) -> str:
        return f"refs/heads/{self.branch}"

    @property
    def tracking_ref(self) -> str:
        return f"refs/remotes/{self.remote}/{self.branch}"

    def identity(self) -> dict[str, str]:
        return {"root": str(self.root), "remote": self.remote, "branch": self.branch}


def run(target: Target, command: list[str], timeout: int = 30, input_text: str | None = None):
    # Repository selection comes from the target, including inside a launched adapter.
    local_git_environment = {
        "GIT_DIR", "GIT_WORK_TREE", "GIT_COMMON_DIR", "GIT_INDEX_FILE", "GIT_NAMESPACE",
        "GIT_OBJECT_DIRECTORY", "GIT_ALTERNATE_OBJECT_DIRECTORIES", "GIT_GRAFT_FILE",
        "GIT_SHALLOW_FILE", "GIT_CONFIG", "GIT_CONFIG_COUNT", "GIT_CONFIG_PARAMETERS",
        "GIT_PREFIX", "GIT_IMPLICIT_WORK_TREE", "GIT_NO_REPLACE_OBJECTS", "GIT_REPLACE_REF_BASE",
    }
    environment = {k: v for k, v in os.environ.items() if k not in local_git_environment}
    environment.update(GIT_OPTIONAL_LOCKS="0", GIT_TERMINAL_PROMPT="0",
                       GIT_NO_REPLACE_OBJECTS="1", PYTHONDONTWRITEBYTECODE="1")
    return subprocess.run(
        command, cwd=target.root, env=environment, input=input_text, text=True,
        capture_output=True, timeout=timeout, check=False,
    )


def git(target: Target, *arguments: str) -> str:
    result = run(target, ["git", *arguments])
    if result.returncode:
        raise RuntimeError(result.stderr.strip() or "Git observation failed")
    return result.stdout.strip()


def snapshot(target: Target) -> dict[str, str]:
    if target.remote_url != git(target, "remote", "get-url", target.remote):
        raise RuntimeError("configured remote changed")
    # Git revision lookup can fall back to a different ref when the exact one is absent.
    row = git(target, "for-each-ref", "--count=1", "--format=%(refname) %(objectname)", target.tracking_ref)
    tracking_ref, _, tracking_commit = row.partition(" ")
    return {
        "status": git(target, "status", "--porcelain=v1"),
        "checkout": git(target, "rev-parse", "--symbolic-full-name", "HEAD"),
        "local": git(target, "show-ref", "--verify", "--hash", target.ref),
        "tracking": tracking_commit if tracking_ref == target.tracking_ref else "",
    }


def succeeds(target: Target, *arguments: str) -> bool | None:
    result = run(target, ["git", *arguments])
    return True if result.returncode == 0 else False if result.returncode == 1 else None


def classify(target: Target, state: dict[str, str], remote: str) -> str:
    if state["checkout"] != target.ref:
        return "inconclusive"
    if state["status"]:
        return "dirty"
    local = state["local"]
    if local == remote:
        return "up-to-date"
    if remote != state["tracking"]:
        return "inconclusive"
    local_first = succeeds(target, "merge-base", "--is-ancestor", local, remote)
    remote_first = succeeds(target, "merge-base", "--is-ancestor", remote, local)
    if local_first is True:
        return "fast-forward-available"
    if remote_first is True:
        return "ahead"
    return "diverged" if local_first is False and remote_first is False else "inconclusive"


def remote_head(target: Target) -> str:
    result = run(target, ["git", "ls-remote", "--exit-code", "--refs", target.remote, target.ref])
    fields = result.stdout.strip().split()
    if (result.returncode or len(fields) != 2 or fields[1] != target.ref
            or not re.fullmatch(r"[0-9a-f]{40}|[0-9a-f]{64}", fields[0])):
        raise RuntimeError("configured remote branch is unavailable")
    return fields[0]


def integrity(target: Target) -> str | None:
    if not target.policy:
        return None
    errors = verify_policy(target.root)
    if errors:
        raise RuntimeError("policy verification failed: " + "; ".join(errors))
    return "policy-integrity: ok"


def final_receipt(stream: str) -> dict[str, object]:
    for line in reversed(stream.splitlines()):
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        if not isinstance(event, dict):
            continue
        item = event.get("item", {})
        if (event.get("type") == "item.completed" and isinstance(item, dict)
                and item.get("type") == "agent_message"):
            if not isinstance(item.get("text"), str):
                break
            receipt = json.loads(item["text"])
            if isinstance(receipt, dict):
                return receipt
            break
    raise RuntimeError("Codex returned no structured final receipt")


def codex_command(target: Target, codex: str) -> list[str]:
    return [
        codex, "--sandbox", "read-only", "--ask-for-approval", "never", "exec",
        "-C", str(target.root), "--ignore-user-config", "--ephemeral", "--json",
        "--output-schema", str(SCHEMA), "-",
    ]


def assess(timeout: int = 120, codex: str | None = None, *, target: Target) -> dict[str, object]:
    before, remote, verified = snapshot(target), remote_head(target), integrity(target)
    baseline = classify(target, before, remote)
    required = {
        "task_id": "harness-sync-assessment-v2",
        "target": target.identity(),
        "local_commit": before["local"],
        "remote_commit": remote,
        "sync_state": baseline,
    }
    receipt = {
        **required, "evidence": [f"Host classified {baseline}."],
        "risk": "Remote state is a point-in-time observation.",
        "next": "Follow policy authority gates before any state-changing operation.",
    }
    owner = "host"
    if codex is not None and baseline == "diverged":
        contract = json.loads(CONTRACT.read_text(encoding="utf-8"))
        contract.update(target=target.identity(), local_ref=target.ref, tracking_ref=target.tracking_ref)
        prompt = (
            "Return only the schema receipt. Independently observe the exact target and its "
            "local and remote commits. Do not mutate anything. "
            + json.dumps(contract, separators=(",", ":"))
        )
        answer = run(target, codex_command(target, codex), timeout, prompt)
        if answer.returncode:
            raise RuntimeError("optional Codex assessment failed")
        receipt, owner = final_receipt(answer.stdout), "codex"
        if any(receipt.get(key) != value for key, value in required.items()):
            raise RuntimeError("oracle disagreement on target, commits, or sync state")
        evidence = receipt.get("evidence")
        if (receipt.keys() != {*required, "evidence", "risk", "next"}
                or not isinstance(evidence, list) or not evidence
                or not all(isinstance(entry, str) for entry in evidence)
                or not isinstance(receipt.get("risk"), str) or not isinstance(receipt.get("next"), str)):
            raise RuntimeError("optional Codex receipt does not match the schema")
        if remote_head(target) != remote or integrity(target) != verified:
            raise RuntimeError("target changed during independent assessment")
    if snapshot(target) != before:
        raise RuntimeError("checkout changed during assessment")

    return {
        "status": "inconclusive" if baseline == "inconclusive" else "verified win",
        "target": target.identity(),
        "host_state": baseline,
        "live_remote": remote,
        "decision_owner": owner,
        "receipt": receipt,
        "integrity": verified,
        "repository_unchanged": True,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True, help="exact checkout root")
    parser.add_argument("--remote", default="origin")
    parser.add_argument("--branch", default="main")
    parser.add_argument("--verify-policy", action="store_true")
    parser.add_argument("--codex", metavar="EXECUTABLE", help="opt in to independent Codex assessment on divergence")
    parser.add_argument("--timeout", type=int, default=120)
    args = parser.parse_args()
    try:
        if args.timeout <= 0:
            raise ValueError("timeout must be positive")
        result = assess(args.timeout, args.codex, target=Target(args.root, args.remote, args.branch, args.verify_policy))
        print(json.dumps(result, separators=(",", ":")))
        return 1 if result["status"] == "inconclusive" else 0
    except (OSError, ValueError, RuntimeError, subprocess.TimeoutExpired) as error:
        print(json.dumps({"status": "inconclusive", "error": str(error)}, separators=(",", ":")))
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
