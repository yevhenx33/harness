from __future__ import annotations

import asyncio
import json
import os
import subprocess
import sys
import unittest
from pathlib import Path
from tempfile import TemporaryDirectory
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from scripts import run_sync_assessment as assessment  # noqa: E402
from scripts.serve_sync_status import StatusState  # noqa: E402


def git(cwd: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", "-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid",
         "-c", "commit.gpgsign=false", *args],
        cwd=cwd, text=True, capture_output=True, check=True,
    )
    return result.stdout.strip()


class SyncAssessmentTest(unittest.TestCase):
    def setUp(self) -> None:
        temporary = TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)

    def target(self, name: str, remote: str, branch: str) -> assessment.Target:
        checkout = self.root / name
        checkout.mkdir()
        origin = self.root / f"{name}.git"
        git(self.root, "init", "--bare", str(origin))
        git(checkout, "init", "-b", branch)
        git(checkout, "commit", "--allow-empty", "-m", name)
        git(checkout, "remote", "add", remote, str(origin))
        git(checkout, "push", remote, f"HEAD:refs/heads/{branch}")
        return assessment.Target(checkout, remote, branch)

    def advance_remote(self, target: assessment.Target, *, fetch: bool) -> str:
        peer = self.root / "peer"
        git(self.root, "clone", "--branch", target.branch, target.remote_url, str(peer))
        git(peer, "commit", "--allow-empty", "-m", "remote change")
        git(peer, "push", "origin", f"HEAD:{target.ref}")
        if fetch:
            git(target.root, "fetch", target.remote)
        return git(peer, "rev-parse", "HEAD")

    def test_two_independent_targets_and_ambient_git_redirection(self) -> None:
        a = self.target("a", "origin", "main")
        b = self.target("b", "upstream", "release/next")
        before = git(b.root, "rev-parse", "HEAD")
        remote = self.advance_remote(b, fetch=False)
        with patch("scripts.run_sync_assessment.codex_command") as codex, patch.dict(
            os.environ, GIT_DIR=str(a.root / ".git"), GIT_WORK_TREE=str(a.root)
        ):
            b = assessment.Target(b.root, b.remote, b.branch)
            first = assessment.assess(target=a)
            second = assessment.assess(target=b)
        codex.assert_not_called()
        self.assertEqual(first["host_state"], "up-to-date")
        self.assertEqual(first["target"], a.identity())
        self.assertEqual(second["status"], "inconclusive")
        self.assertEqual(second["live_remote"], remote)
        self.assertEqual(second["target"], b.identity())

        git(b.root, "fetch", b.remote)
        cli = subprocess.run(
            [sys.executable, str(ROOT / "scripts/run_sync_assessment.py"),
             "--root", str(b.root), "--remote", b.remote, "--branch", b.branch],
            cwd=a.root, text=True, capture_output=True, check=True,
        )
        result = json.loads(cli.stdout)
        self.assertEqual(result["host_state"], "fast-forward-available")
        self.assertEqual(result["decision_owner"], "host")
        self.assertEqual(result["receipt"]["target"], b.identity())
        self.assertEqual(result["receipt"]["local_commit"], before)
        self.assertEqual(result["receipt"]["remote_commit"], remote)
        self.assertEqual(git(b.root, "rev-parse", "HEAD"), before)
        self.assertEqual(git(b.root, "status", "--porcelain"), "")
        git(b.root, "remote", "set-url", b.remote, a.remote_url)
        with self.assertRaisesRegex(RuntimeError, "configured remote changed"):
            assessment.assess(target=b)

    def test_opt_in_receipt_requires_same_target_even_for_matching_commits(self) -> None:
        a = self.target("a", "origin", "main")
        b = self.target("b", "upstream", "release/next")
        git(b.root, "commit", "--allow-empty", "-m", "local change")
        remote = self.advance_remote(b, fetch=True)
        local = git(b.root, "rev-parse", "HEAD")
        real_run = assessment.run
        receipt_target = a.identity()
        requests = []

        def run(target, command, timeout=30, input_text=None):
            if command[0] != "fake-codex":
                return real_run(target, command, timeout, input_text)
            self.assertEqual(target, b)
            self.assertLess(command.index("--sandbox"), command.index("exec"))
            self.assertLess(command.index("--ask-for-approval"), command.index("exec"))
            self.assertIn("--ignore-user-config", command)
            self.assertEqual(command[command.index("-C") + 1], str(b.root))
            contract = json.loads(input_text[input_text.index("{"):])
            self.assertEqual(contract["target"], b.identity())
            self.assertEqual(contract["local_ref"], b.ref)
            self.assertEqual(contract["tracking_ref"], b.tracking_ref)
            requests.append(contract)
            receipt = {
                "task_id": "harness-sync-assessment-v2", "target": receipt_target,
                "local_commit": local, "remote_commit": remote, "sync_state": "diverged",
                "evidence": ["fixture"], "risk": "fixture", "next": "review",
            }
            stream = json.dumps({
                "type": "item.completed", "item": {"type": "agent_message", "text": json.dumps(receipt)}
            })
            return subprocess.CompletedProcess(command, 0, stream, "")

        with patch("scripts.run_sync_assessment.run", side_effect=run):
            self.assertEqual(assessment.assess(target=b)["decision_owner"], "host")
            self.assertEqual(requests, [])
            with self.assertRaisesRegex(RuntimeError, "oracle disagreement"):
                assessment.assess(codex="fake-codex", target=b)
            receipt_target = b.identity()
            result = assessment.assess(codex="fake-codex", target=b)
            self.assertEqual(result["decision_owner"], "codex")
            self.assertEqual(result["receipt"]["target"], b.identity())
        self.assertEqual(len(requests), 2)

    def test_dirty_checked_out_target_and_wrong_branch(self) -> None:
        target = self.target("one", "origin", "main")
        (target.root / "pending").write_text("dirty")
        with patch("scripts.run_sync_assessment.codex_command") as codex:
            self.assertEqual(assessment.assess(codex="fake", target=target)["host_state"], "dirty")
            git(target.root, "switch", "-c", "unrelated")
            self.assertEqual(assessment.assess(codex="fake", target=target)["host_state"], "inconclusive")
        codex.assert_not_called()

    def test_memory_state_preserves_last_result_on_refresh_failure(self) -> None:
        target = self.target("one", "origin", "main")
        state = StatusState(target)
        result = {"status": "verified win", "target": target.identity()}
        with patch("scripts.serve_sync_status.assessment.assess", return_value=result) as assess:
            asyncio.run(state.refresh(1))
            assess.assert_called_once_with(1, target=target)
        with patch("scripts.serve_sync_status.assessment.assess", side_effect=RuntimeError("offline")):
            asyncio.run(state.refresh(1))
        view = state.view()
        self.assertEqual(view["service_state"], "ready")
        self.assertEqual(view["sequence"], 1)
        self.assertEqual(view["result"], result)
        self.assertEqual(view["error"], "offline")


if __name__ == "__main__":
    unittest.main()
