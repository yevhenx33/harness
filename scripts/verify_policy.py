#!/usr/bin/env python3
"""Verify policy version, content, hash, and local-link integrity."""

from __future__ import annotations

import hashlib
import re
import sys
from pathlib import Path
from urllib.parse import unquote, urlsplit


VERSION_RE = re.compile(r"^v(?P<number>\d{3})\.md$")
ROW_RE = re.compile(
    r"^\| \[`v(?P<number>\d{3})`\]\(versions/v(?P=number)\.md\) "
    r"\| (?P<status>Current|Superseded) \|"
)
HASH_RE = re.compile(r"^v(?P<number>\d{3}) (?P<digest>[0-9a-f]{64})$")
LINK_RE = re.compile(r"!?\[[^]]*\]\((?P<target>[^)]+)\)")


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def local_link_target(root: Path, source: Path, raw: str) -> Path | None:
    target = raw.strip().strip("<>").split(maxsplit=1)[0]
    parsed = urlsplit(target)
    if parsed.scheme or parsed.netloc or target.startswith("#"):
        return None
    path = unquote(parsed.path)
    if not path:
        return None
    # Version snapshots preserve root AGENTS.md bytes, so their links retain the
    # root policy's resolution semantics.
    base = root if source.parent == root / "agents" / "versions" else source.parent
    return (base / path).resolve()


def verify(root: Path) -> list[str]:
    root = root.resolve()
    errors: list[str] = []
    index_path = root / "agents" / "README.md"
    versions_dir = root / "agents" / "versions"

    if not index_path.is_file() or not versions_dir.is_dir():
        return ["missing agents/README.md or agents/versions directory"]

    index_lines = index_path.read_text(encoding="utf-8").splitlines()
    rows = [match.groupdict() for line in index_lines if (match := ROW_RE.match(line))]
    hashes = [match.groupdict() for line in index_lines if (match := HASH_RE.match(line))]

    current = [row for row in rows if row["status"] == "Current"]
    if len(current) != 1:
        errors.append(f"expected exactly one current policy version, found {len(current)}")

    files = sorted(
        (int(match.group("number")), path)
        for path in versions_dir.glob("v*.md")
        if (match := VERSION_RE.match(path.name))
    )
    numbers = [number for number, _ in files]
    expected = list(range(1, max(numbers, default=0) + 1))
    if numbers != expected:
        errors.append(f"policy versions are not sequential: found {numbers}, expected {expected}")

    row_numbers = [int(row["number"]) for row in rows]
    if sorted(row_numbers) != numbers or len(row_numbers) != len(set(row_numbers)):
        errors.append("version index rows do not identify each snapshot exactly once")

    hash_numbers = [int(entry["number"]) for entry in hashes]
    if sorted(hash_numbers) != numbers or len(hash_numbers) != len(set(hash_numbers)):
        errors.append("integrity entries do not identify each snapshot exactly once")

    digest_by_number = {int(entry["number"]): entry["digest"] for entry in hashes}
    for number, path in files:
        recorded = digest_by_number.get(number)
        actual = sha256(path)
        if recorded != actual:
            errors.append(
                f"hash mismatch for v{number:03}: recorded {recorded or 'missing'}, actual {actual}"
            )

    if len(current) == 1:
        number = int(current[0]["number"])
        snapshot = versions_dir / f"v{number:03}.md"
        root_policy = root / "AGENTS.md"
        if not snapshot.is_file() or not root_policy.is_file():
            errors.append("root policy or current snapshot is missing")
        elif root_policy.read_bytes() != snapshot.read_bytes():
            errors.append(f"root AGENTS.md does not match current snapshot v{number:03}")

    for source in sorted(root.rglob("*.md")):
        if ".git" in source.parts:
            continue
        text = source.read_text(encoding="utf-8")
        for match in LINK_RE.finditer(text):
            target = local_link_target(root, source, match.group("target"))
            if target is not None and not target.exists():
                errors.append(
                    f"broken local link in {source.relative_to(root)}: {match.group('target')}"
                )

    return errors


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    errors = verify(root)
    if errors:
        print("policy-integrity: failed", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    versions = len(list((root / "agents" / "versions").glob("v[0-9][0-9][0-9].md")))
    print(f"policy-integrity: ok ({versions} sequential versions, links and hashes valid)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
