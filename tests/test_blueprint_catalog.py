import copy
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))
from scripts.blueprint_catalog import SOURCE_URL, check, render, snapshot


class BlueprintCatalogTest(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.source = Path(self.temp.name)
        for path in ('blueprints/nested/a.md', 'patterns/b.md', 'blueprints/README.md', 'templates/blueprint-template.md'):
            target = self.source / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_text('# Same title\n')
        for args in (('init', '-q'), ('remote', 'add', 'origin', SOURCE_URL), ('add', '.'), ('-c', 'user.name=Test', '-c', 'user.email=test@example.com', 'commit', '-qm', 'fixture')):
            subprocess.run(['git', '-C', str(self.source), *args], check=True, capture_output=True)
        self.data = snapshot(self.source) | {'captured_on': '2026-10-03'}

    def test_recursive_inventory_keeps_distinct_paths_and_excludes_support(self):
        self.assertEqual({entry['path'] for entry in self.data['entries']}, {'blueprints/nested/a.md', 'patterns/b.md'})
        check(self.data, render(self.data), self.source)

    def test_missing_new_or_deleted_source_fails(self):
        path = self.source / 'blueprints/new.md'
        path.write_text('# New\n')
        with self.assertRaisesRegex(ValueError, 'source inventory'):
            check(self.data, render(self.data), self.source)
        path.unlink()
        (self.source / 'patterns/b.md').unlink()
        with self.assertRaisesRegex(ValueError, 'source inventory'):
            check(self.data, render(self.data), self.source)

    def test_changed_bytes_fail_and_drafts_have_no_remote_content_link(self):
        (self.source / 'blueprints/nested/a.md').write_text('# Changed\n')
        (self.source / 'blueprints/new.md').write_text('# Draft\n')
        with self.assertRaisesRegex(ValueError, 'source inventory'):
            check(self.data, render(self.data), self.source)
        fresh = snapshot(self.source) | {'captured_on': '2026-10-03'}
        self.assertEqual([entry['state'] for entry in fresh['entries']], ['modified', 'untracked', 'committed'])
        self.assertNotIn('[Draft]', render(fresh))
        self.assertNotIn('[Changed]', render(fresh))

    def test_duplicate_path_and_missing_rendered_row_fail(self):
        duplicate = copy.deepcopy(self.data)
        duplicate['entries'].append(duplicate['entries'][0])
        with self.assertRaisesRegex(ValueError, 'duplicate'):
            check(duplicate, render(duplicate))
        with self.assertRaisesRegex(ValueError, 'catalog differs'):
            check(self.data, render(self.data).rsplit('\n', 2)[0])

    def test_real_catalog_is_valid_without_external_checkout(self):
        directory = ROOT / 'docs/blueprints'
        check(json.loads((directory / 'catalog.json').read_text()), (directory / 'README.md').read_text())

    def test_unavailable_source_exits_nonzero(self):
        result = subprocess.run([sys.executable, str(ROOT / 'scripts/blueprint_catalog.py'), '--source', str(self.source / 'missing')], capture_output=True, text=True)
        self.assertEqual(result.returncode, 1)
        self.assertIn('blueprint-catalog: failed', result.stderr)
