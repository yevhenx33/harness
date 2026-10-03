#!/usr/bin/env python3
"""Track the independently owned blueprint library without copying its content."""
import argparse
from datetime import date
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys
from urllib.parse import quote

ROOT = Path(__file__).resolve().parents[1]
DIRECTORY = ROOT / 'docs' / 'blueprints'
SOURCE_URL = 'https://github.com/yevhenx33/blueprints'
HEADER = '''# Blueprint catalog

Harness tracks the complete source inventory; the separate blueprint repository
owns the documents. This catalog includes Markdown under `blueprints/` and
`patterns/`, recursively, except directory README files. Templates, source
indexes, and decision records are supporting material, not blueprint entries.
Distinct paths remain distinct even when titles match.

Use the installed `blueprint-methods` skill to retrieve at most three relevant
documents from `/home/ubuntu/blueprints`, or your checkout of the source repository.
Read selected documents completely before applying them. Transfer mechanisms and
failure boundaries; a blueprint provides a reasoning lens, not authority.

For missing compositions, shared control assumptions, or stalled searches, read
`blueprints/composition-coverage-before-optimization.md`. Its PK9 lesson separates
construction coverage, parameter recovery, candidate retention, and final
verification. Conditional recovery does not establish independent discovery.

Refresh after adding, changing, or removing source documents:
`python3 scripts/blueprint_catalog.py --source /home/ubuntu/blueprints --refresh`.
Audit current completeness and byte identity with the same command without
`--refresh`. A mismatch or unavailable source fails explicitly. Refresh writes
only this catalog and its manifest; it does not publish or modify source files.
CI validates the manifest and rendered catalog without the external checkout.
CI alone cannot establish current source completeness; run the source audit when
maintaining the library and before releasing a catalog update.

The [manifest](catalog.json) records paths, titles, SHA-256 values, and local Git
states at capture. Links pin committed content to the captured revision. Modified
and untracked entries require the local source checkout and have no content link.
Commit state is not remote publication evidence; refresh does not check hosting.
This inventory is not a quality or effectiveness assessment. Generated file:
change this script's header to revise guidance, then refresh.
'''


def git(source, *args):
    return subprocess.check_output(['git', '-C', str(source), *args], timeout=10).decode().strip('\n')


def snapshot(source):
    revision = git(source, 'rev-parse', 'HEAD')
    if git(source, 'remote', 'get-url', 'origin').removesuffix('.git') != SOURCE_URL:
        raise ValueError('source origin does not match the owning blueprint repository')
    tracked = set(git(source, 'ls-files', '-z', '--', 'blueprints', 'patterns').split('\0'))
    changed = set(git(source, 'diff', '--name-only', '-z', 'HEAD', '--', 'blueprints', 'patterns').split('\0'))
    entries = []
    for kind in ('blueprints', 'patterns'):
        if not (source / kind).is_dir():
            raise ValueError(f'missing source directory: {kind}')
        for path in sorted((source / kind).rglob('*.md')):
            if path.name.lower() == 'readme.md':
                continue
            if path.is_symlink():
                raise ValueError(f'symlinked source document: {path}')
            raw = path.read_bytes()
            title = next((line[2:] for line in raw.decode().splitlines() if line.startswith('# ')), '')
            relative = path.relative_to(source).as_posix()
            state = 'untracked' if relative not in tracked else 'modified' if relative in changed else 'committed'
            entries.append(dict(path=relative, title=title, kind=kind, state=state, sha256=hashlib.sha256(raw).hexdigest()))
    return dict(version=1, source_url=SOURCE_URL, source_commit=revision, entries=entries)


def validate(data):
    if data['version'] != 1 or data['source_url'] != SOURCE_URL or not re.fullmatch('[0-9a-f]{40}', data['source_commit']):
        raise ValueError('invalid catalog source identity')
    date.fromisoformat(data['captured_on'])
    paths = []
    for entry in data['entries']:
        path = PurePosixPath(entry['path'])
        if path.is_absolute() or '..' in path.parts or path.suffix != '.md' or path.parts[0] != entry['kind'] or entry['kind'] not in ('blueprints', 'patterns') or path.name.lower() == 'readme.md':
            raise ValueError('invalid catalog path or kind')
        if not entry['title'].strip() or entry['state'] not in ('committed', 'modified', 'untracked') or not re.fullmatch('[0-9a-f]{64}', entry['sha256']):
            raise ValueError('invalid catalog entry')
        paths.append(entry['path'])
    if not paths or len(paths) != len(set(paths)):
        raise ValueError('empty inventory or duplicate source path')


def render(data):
    lines = [HEADER.rstrip(), f"\nCaptured {data['captured_on']} at source commit `{data['source_commit']}`: {len(data['entries'])} documents.\n", '| Document | Source path | Kind | Local source state |', '|---|---|---|---|']
    for entry in data['entries']:
        title = entry['title'].replace('|', '\\|')
        if entry['state'] == 'committed':
            title = f"[{title}]({SOURCE_URL}/blob/{data['source_commit']}/{quote(entry['path'])})"
        lines.append(f"| {title} | `{entry['path']}` | {entry['kind']} | {entry['state']} |")
    return '\n'.join(lines) + '\n'


def check(data, text, source=None):
    validate(data)
    if text != render(data):
        raise ValueError('catalog differs from manifest; refresh it')
    if source is not None and {key: value for key, value in data.items() if key != 'captured_on'} != snapshot(source):
        raise ValueError('source inventory, content, or Git state changed; refresh the catalog')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', type=Path, help='owning blueprint repository checkout')
    parser.add_argument('--refresh', action='store_true', help='update only the Harness manifest and catalog')
    args = parser.parse_args()
    try:
        if args.refresh:
            if args.source is None:
                raise ValueError('--refresh requires --source')
            data = snapshot(args.source)
            data['captured_on'] = date.today().isoformat()
            validate(data)
            check(data, render(data), args.source)
            DIRECTORY.mkdir(parents=True, exist_ok=True)
            encoded = ',\n'.join(json.dumps(entry, ensure_ascii=False) for entry in data['entries'])
            metadata = json.dumps({key: value for key, value in data.items() if key != 'entries'}, indent=2)[:-2]
            (DIRECTORY / 'catalog.json').write_text(metadata + ',\n  "entries": [\n' + encoded + '\n  ]\n}\n')
            (DIRECTORY / 'README.md').write_text(render(data))
        data = json.loads((DIRECTORY / 'catalog.json').read_text())
        check(data, (DIRECTORY / 'README.md').read_text(), args.source)
    except (ValueError, KeyError, TypeError, IndexError, OSError, subprocess.SubprocessError) as error:
        print(f'blueprint-catalog: failed ({error})', file=sys.stderr)
        return 1
    print(f"blueprint-catalog: ok ({len(data['entries'])} documents; " + ('source parity checked)' if args.source else 'manifest and rendering checked; source freshness unchecked)'))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
