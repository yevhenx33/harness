from __future__ import annotations

import json
import asyncio
import sys
import unittest
from pathlib import Path
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from scripts.run_sync_assessment import assess, classify, codex_command, final_receipt  # noqa: E402
from scripts.serve_sync_status import StatusState  # noqa: E402


class SyncAssessmentTest(unittest.TestCase):
    def test_classifies_clean_equal_state(self) -> None:
        state = {"status": "", "main": "abc", "origin_main": "abc"}
        self.assertEqual(classify(state, "abc"), "up-to-date")

    def test_dirty_state_wins(self) -> None:
        state = {"status": "?? local", "main": "abc", "origin_main": "abc"}
        self.assertEqual(classify(state, "abc"), "dirty")

    def test_unknown_remote_object_is_inconclusive(self) -> None:
        state = {"status": "", "main": "abc", "origin_main": "abc"}
        self.assertEqual(classify(state, "def"), "inconclusive")

    def test_reads_structured_final_receipt(self) -> None:
        receipt = {"task_id": "harness-sync-assessment-v1", "sync_state": "up-to-date"}
        stream = json.dumps(
            {"type": "item.completed", "item": {"type": "agent_message", "text": json.dumps(receipt)}}
        )
        self.assertEqual(final_receipt(stream), receipt)

    def test_global_authority_flags_precede_exec(self) -> None:
        command = codex_command("codex")
        self.assertLess(command.index("--sandbox"), command.index("exec"))
        self.assertLess(command.index("--ask-for-approval"), command.index("exec"))
        self.assertIn("--ignore-user-config", command)
        self.assertEqual(command[-1], "-")

    def test_up_to_date_path_skips_codex(self) -> None:
        state = {"status": "", "head": "abc", "main": "abc", "origin_main": "abc", "policy": "p"}
        with (
            patch("scripts.run_sync_assessment.snapshot", return_value=state),
            patch("scripts.run_sync_assessment.remote_main", return_value="abc"),
            patch("scripts.run_sync_assessment.integrity", return_value="integrity ok"),
            patch("scripts.run_sync_assessment.codex_command") as codex,
        ):
            result = assess()
        self.assertEqual(result["decision_owner"], "host")
        codex.assert_not_called()

    def test_memory_state_preserves_last_result_on_refresh_failure(self) -> None:
        state = StatusState()
        with patch("scripts.serve_sync_status.assessment.assess", return_value={"status": "verified win"}):
            asyncio.run(state.refresh(1))
        with patch("scripts.serve_sync_status.assessment.assess", side_effect=RuntimeError("offline")):
            asyncio.run(state.refresh(1))
        view = state.view()
        self.assertEqual(view["service_state"], "ready")
        self.assertEqual(view["sequence"], 1)
        self.assertEqual(view["result"], {"status": "verified win"})
        self.assertEqual(view["error"], "offline")

    def test_dirty_path_skips_codex(self) -> None:
        state = {"status": "?? local", "head": "abc", "main": "abc", "origin_main": "abc", "policy": "p"}
        with (
            patch("scripts.run_sync_assessment.snapshot", return_value=state),
            patch("scripts.run_sync_assessment.remote_main", return_value="abc"),
            patch("scripts.run_sync_assessment.integrity", return_value="integrity ok"),
            patch("scripts.run_sync_assessment.codex_command") as codex,
        ):
            result = assess()
        self.assertEqual(result["host_state"], "dirty")
        self.assertEqual(result["decision_owner"], "host")
        codex.assert_not_called()


if __name__ == "__main__":
    unittest.main()
