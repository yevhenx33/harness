#!/usr/bin/env python3
"""Generate normalized narrative-attention vectors and budget tables."""

from __future__ import annotations

import argparse
import csv
import json
import math
import sys


FAMILIES = ("equal", "linear-up", "linear-down", "cosine", "compound")


def normalize(values: list[float]) -> list[float]:
    values = [max(value, 1e-9) for value in values]
    total = sum(values)
    return [value / total for value in values]


def gaussian(index: int, center: float, width: float) -> float:
    return math.exp(-((index - center) ** 2) / (2 * width**2))


def make_vector(
    family: str,
    count: int,
    amplitude: float,
    slope: float,
    cycles: float,
    phase: float,
    peaks: list[tuple[float, float]],
) -> list[float]:
    if family == "equal":
        raw = [1.0] * count
    elif family == "linear-up":
        raw = [1.0 + slope * (i / max(count - 1, 1) - 0.5) for i in range(count)]
    elif family == "linear-down":
        raw = [1.0 - slope * (i / max(count - 1, 1) - 0.5) for i in range(count)]
    else:
        default_cycles = (count - 1) / 2 if count > 1 else 0
        wave_cycles = cycles if cycles >= 0 else default_cycles
        wave = [
            math.cos(2 * math.pi * wave_cycles * i / max(count - 1, 1) + phase)
            for i in range(count)
        ]
        if family == "cosine":
            raw = [1.0 + amplitude * value for value in wave]
        elif family == "compound":
            raw = [
                1.0
                + slope * (i / max(count - 1, 1) - 0.5)
                + amplitude * wave[i]
                + sum(strength * gaussian(i, center, max(count / 10, 0.75)) for center, strength in peaks)
                for i in range(count)
            ]
        else:
            raise ValueError(f"unknown family: {family}")
    return normalize(raw)


def parse_peak(value: str) -> tuple[float, float]:
    try:
        position, strength = value.split(":", 1)
        return float(position) - 1, float(strength)
    except ValueError as exc:
        raise argparse.ArgumentTypeError("peak must be SECTION_NUMBER:STRENGTH") from exc


def rows(args: argparse.Namespace) -> list[dict[str, object]]:
    sections = [part.strip() for part in args.sections.split(",") if part.strip()]
    if not sections:
        raise SystemExit("--sections must contain at least one name")
    families = [part.strip() for part in args.families.split(",") if part.strip()]
    unknown = set(families) - set(FAMILIES)
    if unknown:
        raise SystemExit(f"unknown families: {', '.join(sorted(unknown))}")

    result: list[dict[str, object]] = []
    for family in families:
        vector = make_vector(
            family,
            len(sections),
            args.amplitude,
            args.slope,
            args.cycles,
            args.phase,
            args.peak,
        )
        for index, (section, weight) in enumerate(zip(sections, vector), start=1):
            result.append(
                {
                    "family": family,
                    "index": index,
                    "section": section,
                    "weight": round(weight, 6),
                    "percent": round(weight * 100, 2),
                    "words": round(weight * args.words) if args.words is not None else None,
                    "seconds": round(weight * args.seconds) if args.seconds is not None else None,
                    "slides": round(weight * args.slides, 2) if args.slides is not None else None,
                }
            )
    return result


def print_markdown(data: list[dict[str, object]]) -> None:
    optional = [key for key in ("words", "seconds", "slides") if data[0][key] is not None]
    columns = ["family", "section", "percent", *optional]
    print("| " + " | ".join(columns) + " |")
    print("| " + " | ".join("---" if key in ("family", "section") else "---:" for key in columns) + " |")
    for row in data:
        print("| " + " | ".join(str(row[key]) for key in columns) + " |")


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sections", required=True, help="Comma-separated section names")
    parser.add_argument("--families", default=",".join(FAMILIES), help="Comma-separated curve families")
    parser.add_argument("--words", type=int, help="Total word budget")
    parser.add_argument("--seconds", type=int, help="Total duration in seconds")
    parser.add_argument("--slides", type=float, help="Total slide budget")
    parser.add_argument("--amplitude", type=float, default=0.3, help="Oscillation amplitude")
    parser.add_argument("--slope", type=float, default=0.2, help="Linear end-to-end change")
    parser.add_argument("--cycles", type=float, default=-1, help="Wave cycles; default alternates sections")
    parser.add_argument("--phase", type=float, default=0.0, help="Wave phase in radians")
    parser.add_argument("--peak", action="append", type=parse_peak, default=[], help="Compound peak SECTION:STRENGTH")
    parser.add_argument("--format", choices=("markdown", "json", "csv"), default="markdown")
    args = parser.parse_args()
    data = rows(args)

    if args.format == "json":
        json.dump(data, sys.stdout, indent=2)
        print()
    elif args.format == "csv":
        writer = csv.DictWriter(sys.stdout, fieldnames=data[0].keys())
        writer.writeheader()
        writer.writerows(data)
    else:
        print_markdown(data)


if __name__ == "__main__":
    main()
