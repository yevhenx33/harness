from __future__ import annotations

import contextlib
import io
import subprocess
import sys
import unittest
from pathlib import Path
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from scripts.verify_harness import CHECKS, verify  # noqa: E402


class HarnessVerificationTest(unittest.TestCase):
    def test_contract_runs_policy_skills_then_tests(self) -> None:
        self.assertEqual([name for name, _ in CHECKS], ["policy-integrity", "skill-integrity", "tests"])

    def test_runs_every_check_and_reports_each_failure(self) -> None:
        results = [
            subprocess.CompletedProcess([], 1),
            subprocess.CompletedProcess([], 0),
            subprocess.CompletedProcess([], 2),
        ]
        with patch("scripts.verify_harness.subprocess.run", side_effect=results) as run:
            self.assertEqual(verify(), ["policy-integrity", "tests"])
        self.assertEqual(run.call_count, len(CHECKS))

    def test_timeout_is_explicit_and_later_checks_still_run(self) -> None:
        results = [
            subprocess.TimeoutExpired([], 300),
            subprocess.CompletedProcess([], 0),
            subprocess.CompletedProcess([], 0),
        ]
        with (
            patch("scripts.verify_harness.subprocess.run", side_effect=results) as run,
            contextlib.redirect_stderr(io.StringIO()) as stderr,
        ):
            self.assertEqual(verify(), ["policy-integrity"])
        self.assertIn("policy-integrity: failed", stderr.getvalue())
        self.assertEqual(run.call_count, len(CHECKS))


if __name__ == "__main__":
    unittest.main()
