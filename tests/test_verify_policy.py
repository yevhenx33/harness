from __future__ import annotations

import shutil
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from scripts.verify_policy import verify  # noqa: E402


class PolicyIntegrityTest(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name) / "repository"
        shutil.copytree(ROOT, self.root, ignore=shutil.ignore_patterns(".git", "__pycache__"))

    def tearDown(self) -> None:
        self.temporary.cleanup()

    def errors(self) -> str:
        return "\n".join(verify(self.root))

    def test_real_repository_is_valid(self) -> None:
        self.assertEqual(verify(ROOT), [])

    def test_rejects_root_snapshot_mismatch(self) -> None:
        policy = self.root / "AGENTS.md"
        policy.write_text(policy.read_text() + "\nchanged\n")
        self.assertIn("does not match current snapshot", self.errors())

    def test_rejects_wrong_hash(self) -> None:
        index = self.root / "agents" / "README.md"
        text = index.read_text()
        index.write_text(text.replace("v006 c2fc", "v006 0000"))
        self.assertIn("hash mismatch for v006", self.errors())

    def test_rejects_missing_sequence_number(self) -> None:
        (self.root / "agents" / "versions" / "v003.md").unlink()
        self.assertIn("versions are not sequential", self.errors())

    def test_rejects_duplicate_current_version(self) -> None:
        index = self.root / "agents" / "README.md"
        text = index.read_text()
        old = "| [`v005`](versions/v005.md) | Superseded |"
        index.write_text(text.replace(old, "| [`v005`](versions/v005.md) | Current |"))
        self.assertIn("expected exactly one current policy version", self.errors())

    def test_rejects_broken_local_link(self) -> None:
        readme = self.root / "README.md"
        readme.write_text(readme.read_text() + "\n[missing](docs/missing.md)\n")
        self.assertIn("broken local link", self.errors())


if __name__ == "__main__":
    unittest.main()
