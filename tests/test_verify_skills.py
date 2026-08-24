from __future__ import annotations

import shutil
import sys
import tempfile
import unittest
from pathlib import Path

import yaml


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from scripts.verify_skills import verify  # noqa: E402


class SkillIntegrityTest(unittest.TestCase):
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

    def test_rejects_unresolved_graph_edge(self) -> None:
        index_path = self.root / "skills/security-review-router/references/method-index.yaml"
        index = yaml.safe_load(index_path.read_text())
        index["nodes"][0]["links"]["precedes"].append("missing-method")
        index_path.write_text(yaml.safe_dump(index, sort_keys=False))
        self.assertIn("unresolved precedes edge: risk-audit -> missing-method", self.errors())

    def test_rejects_nonreciprocal_symmetric_edge(self) -> None:
        index_path = self.root / "skills/security-review-router/references/method-index.yaml"
        index = yaml.safe_load(index_path.read_text())
        index["nodes"][0]["links"]["complements"].remove("threat-model")
        index_path.write_text(yaml.safe_dump(index, sort_keys=False))
        self.assertIn("non-reciprocal complements edge: threat-model -> risk-audit", self.errors())

    def test_rejects_implicit_method_invocation(self) -> None:
        agent = self.root / "skills/security-method-risk-audit/agents/openai.yaml"
        agent.write_text(agent.read_text().replace("allow_implicit_invocation: false", "allow_implicit_invocation: true"))
        self.assertIn("security-method-risk-audit must disable implicit invocation", self.errors())

    def test_rejects_frontmatter_name_drift(self) -> None:
        skill = self.root / "skills/security-method-risk-audit/SKILL.md"
        skill.write_text(skill.read_text().replace("name: security-method-risk-audit", "name: security-method-risk-auditor"))
        self.assertIn("frontmatter name mismatch for security-method-risk-audit", self.errors())

    def test_rejects_missing_layer_file(self) -> None:
        (self.root / "skills/security-method-risk-audit/references/l2-playbook.md").unlink()
        self.assertIn("security-method-risk-audit must contain exactly its four declared files", self.errors())


if __name__ == "__main__":
    unittest.main()
