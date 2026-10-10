#!/usr/bin/env python3
"""Run the canonical Harness verification suite."""

from __future__ import annotations

import os
import subprocess
import sys
from tempfile import TemporaryDirectory
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
CHECK_TIMEOUT_SECONDS = 300
CHECKS = (
    ("policy-integrity", (sys.executable, "scripts/verify_policy.py")),
    ("skill-integrity", (sys.executable, "scripts/verify_skills.py")),
    ("tests", (sys.executable, "-m", "unittest", "discover", "-s", "tests", "-v")),
)


def verify() -> list[str]:
    with TemporaryDirectory(prefix="harness-verification-") as build:
        guard, tests = str(Path(build) / "guard"), str(Path(build) / "tests")
        checks = (
            ("rust-build", ("rustc", "--edition=2021", "scripts/verify_release.rs", "-o", guard)),
            ("release-boundary", (guard, "release", str(ROOT), os.environ.get("HARNESS_RELEASE_BASE", "main"))),
            ("activation-source", (guard, "skills", str(ROOT))),
            ("rust-test-build", ("rustc", "--edition=2021", "--test", "tests/release_activation.rs", "-o", tests)),
            ("release-activation-tests", (tests,)),
        ) + CHECKS
        return run_checks(checks)


def run_checks(checks: tuple) -> list[str]:
    environment = os.environ | {"PYTHONDONTWRITEBYTECODE": "1"}
    failures: list[str] = []
    for name, command in checks:
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
    print(f"harness-verification: ok ({len(CHECKS) + 5} checks)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
