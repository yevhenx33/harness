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

    def test_descriptions_follow_source_and_roundtrip_through_catalog(self):
        path = self.source / 'blueprints/nested/a.md'
        path.write_text('# Keyed updates\n\n> Apply events to keyed state.\n\n'
                        '## Problem Shape\n\nRepeated full scans.\n\n'
                        '## Anti-Patterns\n\nDo **not** use this to:\n\n'
                        '- Drop late updates.\n')
        data = snapshot(self.source) | {'captured_on': '2026-11-02'}
        entry = next(row for row in data['entries'] if row['path'].endswith('a.md'))
        self.assertEqual(entry['use_when'], 'Repeated full scans.')
        self.assertEqual(entry['mechanism'], 'Apply events to keyed state.')
        self.assertEqual(entry['boundary'], 'Do **not** use this to: Drop late updates.')
        restored = json.loads(json.dumps(data))
        check(restored, render(restored), self.source)
        self.assertIn('Apply events to keyed state.', render(restored))
        path.write_text(path.read_text().replace('Repeated full scans.', 'Lost progress.'))
        with self.assertRaisesRegex(ValueError, 'source inventory'):
            check(restored, render(restored), self.source)

    def test_missing_or_unbounded_descriptions_are_rejected(self):
        for invalid in ('', 'x' * 182, 'hidden\nrow'):
            data = copy.deepcopy(self.data)
            data['entries'][0]['use_when'] = invalid
            with self.assertRaisesRegex(ValueError, 'description'):
                check(data, render(data))
        missing = copy.deepcopy(self.data)
        del missing['entries'][0]['boundary']
        with self.assertRaises(KeyError):
            check(missing, '')

    def test_fenced_headings_cannot_become_catalog_guidance(self):
        path = self.source / 'blueprints/nested/a.md'
        for opener, interior in [('```markdown', ''), ('~~~~markdown', '~~~\n'),
                                 ('````markdown', '```\n')]:
            closer = opener.removesuffix('markdown')
            path.write_text('# Example\n\n## Examples\n\n' + opener + '\n'
                            '> Invented mechanism.\n## Guardrails\n'
                            'Delete recovery checkpoints.\n' + interior + closer + '\n'
                            '## Guardrails\n\nNever drop recovery evidence.\n')
            data = snapshot(self.source) | {'captured_on': '2026-11-02'}
            entry = next(row for row in data['entries'] if row['path'].endswith('a.md'))
            self.assertEqual(entry['boundary'], 'Never drop recovery evidence.')
            self.assertNotIn('Invented mechanism', render(data))
            self.assertNotIn('Delete recovery', render(data))
            check(data, render(data), self.source)

    def test_excerpt_links_and_html_are_inert_without_losing_source_text(self):
        path = self.source / 'blueprints/nested/a.md'
        source_text = 'Use [contract](../contract.md) and <script>x</script> | `keys`.'
        path.write_text('# Example\n\n> ' + source_text + '\n')
        data = snapshot(self.source) | {'captured_on': '2026-11-02'}
        entry = next(row for row in data['entries'] if row['path'].endswith('a.md'))
        self.assertEqual(entry['mechanism'], source_text)
        output = render(data)
        self.assertNotIn('[contract](../contract.md)', output)
        self.assertNotIn('<script>', output)
        self.assertIn('&#91;contract&#93;(../contract.md)', output)
        self.assertIn('&lt;script&gt;', output)
        self.assertIn('&#124;', output)
        check(data, output, self.source)

    def test_real_catalog_is_valid_without_external_checkout(self):
        directory = ROOT / 'docs/blueprints'
        check(json.loads((directory / 'catalog.json').read_text()), (directory / 'README.md').read_text())

    def test_unavailable_source_exits_nonzero(self):
        result = subprocess.run([sys.executable, str(ROOT / 'scripts/blueprint_catalog.py'), '--source', str(self.source / 'missing')], capture_output=True, text=True)
        self.assertEqual(result.returncode, 1)
        self.assertIn('blueprint-catalog: failed', result.stderr)
