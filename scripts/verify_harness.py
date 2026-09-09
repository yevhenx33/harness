#!/usr/bin/env python3
"""Run the canonical Harness verification suite."""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CHECK_TIMEOUT_SECONDS = 300
CHECKS = (
    ("policy-integrity", (sys.executable, "scripts/verify_policy.py")),
    ("skill-integrity", (sys.executable, "scripts/verify_skills.py")),
    ("tests", (sys.executable, "-m", "unittest", "discover", "-s", "tests", "-v")),
)


def verify() -> list[str]:
    environment = os.environ | {"PYTHONDONTWRITEBYTECODE": "1"}
    failures: list[str] = []
    for name, command in CHECKS:
        try:
            result = subprocess.run(
                command,
                cwd=ROOT,
                env=environment,
                check=False,
                timeout=CHECK_TIMEOUT_SECONDS,
            )
        except (OSError, subprocess.TimeoutExpired) as error:
            print(f"{name}: failed ({error})", file=sys.stderr)
            failures.append(name)
            continue
        if result.returncode:
            failures.append(name)
    return failures


def main() -> int:
    failures = verify()
    if failures:
        print(f"harness-verification: failed ({', '.join(failures)})", file=sys.stderr)
        return 1
    print(f"harness-verification: ok ({len(CHECKS)} checks)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
