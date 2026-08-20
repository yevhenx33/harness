#!/usr/bin/env python3
"""Compare one read-only Codex sync assessment with a host oracle."""

import json
import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
CONTRACT = ROOT / "contracts" / "sync-assessment.json"
SCHEMA = ROOT / "schemas" / "sync-receipt.schema.json"


def run(command: list[str], timeout: int = 30, input_text: str | None = None):
    environment = os.environ | {"GIT_OPTIONAL_LOCKS": "0", "PYTHONDONTWRITEBYTECODE": "1"}
    return subprocess.run(
        command,
        cwd=ROOT,
        env=environment,
        input=input_text,
        text=True,
        capture_output=True,
        timeout=timeout,
        check=False,
    )


def git(*arguments: str) -> str:
    result = run(["git", *arguments])
    if result.returncode:
        raise RuntimeError(result.stderr.strip() or "git observation failed")
    return result.stdout.strip()


def snapshot() -> dict[str, str]:
    return {
        "status": git("status", "--porcelain=v1"),
        "head": git("rev-parse", "HEAD"),
        "main": git("rev-parse", "main"),
        "origin_main": git("rev-parse", "refs/remotes/origin/main"),
        "policy": git("hash-object", "AGENTS.md"),
    }


def succeeds(*arguments: str) -> bool | None:
    result = run(["git", *arguments])
    return True if result.returncode == 0 else False if result.returncode == 1 else None


def classify(state: dict[str, str], remote: str) -> str:
    if state["status"]:
        return "dirty"
    local = state["main"]
    if local == remote:
        return "up-to-date"
    if remote != state["origin_main"]:
        return "inconclusive"
    local_first = succeeds("merge-base", "--is-ancestor", local, remote)
    remote_first = succeeds("merge-base", "--is-ancestor", remote, local)
    if local_first is True:
        return "fast-forward-available"
    if remote_first is True:
        return "ahead"
    return "diverged" if local_first is False and remote_first is False else "inconclusive"


def remote_main() -> str:
    result = run(["git", "ls-remote", "--heads", "origin", "main"])
    if result.returncode or not result.stdout.strip():
        raise RuntimeError(result.stderr.strip() or "origin/main unavailable")
    return result.stdout.split()[0]


def integrity() -> str:
    result = run([sys.executable, "scripts/verify_policy.py"])
    if result.returncode:
        raise RuntimeError(result.stderr.strip() or "policy verification failed")
    return result.stdout.strip()


def final_receipt(stream: str) -> dict[str, object]:
    for line in reversed(stream.splitlines()):
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        item = event.get("item", {})
        if event.get("type") == "item.completed" and item.get("type") == "agent_message":
            return json.loads(item["text"])
    raise RuntimeError("Codex returned no structured final receipt")


def codex_command(codex: str) -> list[str]:
    return [
        codex, "--sandbox", "read-only", "--ask-for-approval", "never", "exec",
        "-C", str(ROOT), "--ignore-user-config", "--ephemeral", "--json",
        "--output-schema", str(SCHEMA), "-",
    ]


def host_receipt(state: str, local: str, remote: str, verified: str) -> dict[str, object]:
    return {
        "task_id": "harness-sync-assessment-v1",
        "sync_state": state,
        "evidence": [f"Host classified {state}; main={local}; live origin/main={remote}.", verified],
        "risk": "Live remote state is a point-in-time observation.",
        "next": "Follow policy authority gates before any state-changing operation.",
    }


def assess(timeout: int = 120, codex: str = "codex") -> dict[str, object]:
    with ThreadPoolExecutor(max_workers=3) as pool:
        state_task, remote_task, integrity_task = (
            pool.submit(snapshot), pool.submit(remote_main), pool.submit(integrity))
        before, remote, integrity_before = state_task.result(), remote_task.result(), integrity_task.result()
    baseline = classify(before, remote)
    receipt = host_receipt(baseline, before["main"], remote, integrity_before)
    owner = "host"
    if baseline == "diverged":
        contract = json.loads(CONTRACT.read_text(encoding="utf-8"))
        prompt = (
            "Execute this contract exactly; return only the schema receipt. Do not mutate anything. "
            "Independently inspect the repository and do not trust any stated sync result. Contract: "
            + json.dumps(contract, separators=(",", ":"))
        )
        result = run(codex_command(codex), timeout, prompt)
        if result.returncode:
            raise RuntimeError(result.stderr.strip() or result.stdout.strip() or "Codex failed")
        receipt, owner = final_receipt(result.stdout), "codex"
        after, integrity_after = snapshot(), integrity()
        if before != after or integrity_before != integrity_after:
            raise RuntimeError("primary invariant failed: repository or integrity changed")
    if receipt.get("sync_state") != baseline:
        raise RuntimeError(f"oracle disagreement: host={baseline}, codex={receipt.get('sync_state')}")
    return {
        "status": "verified win",
        "host_state": baseline,
        "live_remote": remote,
        "decision_owner": owner,
        "codex_receipt": receipt,
        "integrity": integrity_before,
        "repository_unchanged": True,
    }


def main() -> int:
    try:
        print(json.dumps(assess(), separators=(",", ":")))
        return 0
    except (OSError, RuntimeError, subprocess.TimeoutExpired) as error:
        print(json.dumps({"status": "inconclusive", "error": str(error)}, separators=(",", ":")))
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
