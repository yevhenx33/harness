#!/usr/bin/env python3
"""Generic same-block RPC witness checker for static ABI calls.

Inputs:
  --spec  JSON invariant spec
  --facts JSONL indexed facts, one claim per line

The harness intentionally supports a small static ABI subset. For dynamic
returns or protocol-specific decoding, build a task-local adapter using this
script as the outer comparison/reporting pattern.
"""

from __future__ import annotations

import argparse
import json
import math
import os
import re
import subprocess
import sys
import time
import urllib.error
import urllib.request
from decimal import Decimal, InvalidOperation
from typing import Any


HEX_RE = re.compile(r"^0x[0-9a-fA-F]*$")


def load_json(path: str) -> dict[str, Any]:
    with open(path, "r", encoding="utf-8") as handle:
        return json.load(handle)


def load_facts(path: str) -> list[dict[str, Any]]:
    rows: list[dict[str, Any]] = []
    with open(path, "r", encoding="utf-8") as handle:
        for line_no, line in enumerate(handle, 1):
            line = line.strip()
            if not line:
                continue
            try:
                rows.append(json.loads(line))
            except json.JSONDecodeError as exc:
                raise SystemExit(f"{path}:{line_no}: invalid JSONL row: {exc}") from exc
    return rows


def normalize_address(value: Any) -> str:
    text = str(value).strip().lower()
    if not re.match(r"^0x[0-9a-f]{40}$", text):
        raise ValueError(f"invalid address: {value!r}")
    return text


def int_from_any(value: Any) -> int:
    if isinstance(value, bool):
        return int(value)
    if isinstance(value, int):
        return value
    text = str(value).strip()
    if text.startswith(("0x", "0X")):
        return int(text, 16)
    return int(text)


def word(value: int) -> str:
    if value < 0:
        value = (1 << 256) + value
    if value < 0 or value >= 1 << 256:
        raise ValueError("ABI integer word out of uint256 range")
    return f"{value:064x}"


def encode_static_arg(arg_type: str, value: Any) -> str:
    if arg_type == "address":
        return "0" * 24 + normalize_address(value)[2:]
    if arg_type.startswith("uint"):
        return word(int_from_any(value))
    if arg_type.startswith("int"):
        return word(int_from_any(value))
    if arg_type == "bool":
        return word(1 if bool(value) else 0)
    if arg_type == "bytes32":
        text = str(value).strip()
        if not HEX_RE.match(text) or len(text) != 66:
            raise ValueError(f"invalid bytes32: {value!r}")
        return text[2:].lower()
    raise ValueError(f"unsupported static arg type: {arg_type}")


def decode_static_return(return_type: str, data: str, index: int = 0) -> Any:
    if not HEX_RE.match(data):
        raise ValueError(f"invalid hex return data: {data!r}")
    body = data[2:]
    start = index * 64
    chunk = body[start : start + 64]
    if len(chunk) != 64:
        raise ValueError("return data too short")
    if return_type.startswith("uint"):
        return str(int(chunk, 16))
    if return_type.startswith("int"):
        value = int(chunk, 16)
        if value >= 1 << 255:
            value -= 1 << 256
        return str(value)
    if return_type == "bool":
        return bool(int(chunk, 16))
    if return_type == "address":
        return "0x" + chunk[-40:].lower()
    if return_type == "bytes32":
        return "0x" + chunk.lower()
    raise ValueError(f"unsupported static return type: {return_type}")


def selector_from_spec(call: dict[str, Any]) -> str:
    if call.get("selector"):
        selector = str(call["selector"]).lower()
        if not HEX_RE.match(selector) or len(selector) != 10:
            raise SystemExit(f"invalid function selector: {selector!r}")
        return selector
    signature = call.get("signature")
    if not signature:
        raise SystemExit("call.selector or call.signature is required")
    try:
        result = subprocess.run(
            ["cast", "sig", str(signature)],
            check=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except (OSError, subprocess.CalledProcessError) as exc:
        raise SystemExit(
            "call.signature requires Foundry `cast`; provide call.selector instead"
        ) from exc
    selector = result.stdout.strip().lower()
    if not HEX_RE.match(selector) or len(selector) != 10:
        raise SystemExit(f"cast returned invalid selector: {selector!r}")
    return selector


def resolve_value(spec: dict[str, Any], row: dict[str, Any]) -> Any:
    if "column" in spec:
        return row[spec["column"]]
    if "literal" in spec:
        return spec["literal"]
    raise ValueError("value spec must contain column or literal")


def build_call(row: dict[str, Any], spec: dict[str, Any], selector: str) -> tuple[str, str]:
    call = spec["call"]
    target = normalize_address(resolve_value(call["target"], row))
    encoded_args = "".join(
        encode_static_arg(arg["type"], resolve_value(arg, row))
        for arg in call.get("args", [])
    )
    return target, selector + encoded_args


def rpc_batch(rpc_url: str, payloads: list[dict[str, Any]], timeout: float) -> list[dict[str, Any]]:
    request = urllib.request.Request(
        rpc_url,
        data=json.dumps(payloads).encode("utf-8"),
        headers={"content-type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            body = response.read().decode("utf-8")
    except urllib.error.URLError as exc:
        raise RuntimeError(f"RPC request failed: {exc}") from exc
    value = json.loads(body)
    if isinstance(value, dict):
        value = [value]
    by_id = {item.get("id"): item for item in value}
    return [by_id.get(payload["id"], {"id": payload["id"], "error": "missing response"}) for payload in payloads]


def decimal_from(value: Any) -> Decimal:
    try:
        return Decimal(str(value))
    except InvalidOperation as exc:
        raise ValueError(f"not a decimal value: {value!r}") from exc


def compare_values(expected: Any, actual: Any, tolerance: dict[str, Any]) -> tuple[bool, Decimal, Decimal]:
    mode = tolerance.get("mode", "exact")
    if mode == "exact":
        ok = str(expected).lower() == str(actual).lower()
        try:
            abs_diff = abs(decimal_from(expected) - decimal_from(actual))
            rel_diff = Decimal(0) if decimal_from(expected) == 0 else abs_diff / abs(decimal_from(expected))
        except ValueError:
            abs_diff = Decimal(0) if ok else Decimal(1)
            rel_diff = abs_diff
        return ok, abs_diff, rel_diff

    expected_dec = decimal_from(expected)
    actual_dec = decimal_from(actual)
    abs_diff = abs(expected_dec - actual_dec)
    rel_diff = Decimal(0) if expected_dec == 0 else abs_diff / abs(expected_dec)
    limit = decimal_from(tolerance.get("value", "0"))
    if mode == "absolute":
        return abs_diff <= limit, abs_diff, rel_diff
    if mode == "relative":
        return rel_diff <= limit, abs_diff, rel_diff
    raise ValueError(f"unsupported tolerance mode: {mode}")


def key_for(row: dict[str, Any], key_columns: list[str]) -> dict[str, Any]:
    return {column: row.get(column) for column in key_columns}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--spec", required=True)
    parser.add_argument("--facts", required=True)
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--json", action="store_true", help="emit JSON summary")
    args = parser.parse_args()

    spec = load_json(args.spec)
    rows = load_facts(args.facts)
    anchor = spec["anchor"]
    rpc_url = anchor.get("rpc_url") or os.environ.get(str(anchor.get("rpc_url_env", "")))
    if not rpc_url and not args.dry_run:
        raise SystemExit("anchor.rpc_url or anchor.rpc_url_env is required")
    block = int(anchor["block"])
    block_tag = hex(block)
    selector = selector_from_spec(spec["call"])
    runtime = spec.get("runtime", {})
    batch_size = int(runtime.get("batch_size", 100))
    timeout = float(runtime.get("timeout_seconds", 30))
    max_mismatches = int(runtime.get("max_mismatches", 25))
    returns = spec["call"].get("returns", [])
    if len(returns) != 1:
        raise SystemExit("generic harness expects exactly one static return")
    return_type = returns[0]["type"]
    return_name = returns[0].get("name", "actual")
    compare = spec["compare"]
    key_columns = compare.get("key", [])
    expected_spec = compare["expected"]
    tolerance = compare.get("tolerance", {"mode": "exact"})

    planned = []
    planning_errors = []
    for index, row in enumerate(rows):
        try:
            target, calldata = build_call(row, spec, selector)
            planned.append((index, row, target, calldata))
        except Exception as exc:  # noqa: BLE001 - report row-level planning errors
            planning_errors.append({"row": index, "key": key_for(row, key_columns), "error": str(exc)})

    if args.dry_run:
        summary = {
            "name": spec.get("name", "anchor-rpc-witness"),
            "anchor": {"chain_id": anchor.get("chain_id"), "block": block},
            "facts": len(rows),
            "planned_calls": len(planned),
            "planning_errors": len(planning_errors),
            "selector": selector,
            "dry_run": True,
        }
        print(json.dumps(summary, indent=2, sort_keys=True))
        if planning_errors[:max_mismatches]:
            print(json.dumps({"planning_error_samples": planning_errors[:max_mismatches]}, indent=2, sort_keys=True))
        return 0 if not planning_errors else 2

    start = time.monotonic()
    rpc_failures = []
    decode_failures = []
    mismatches = []
    pass_count = 0
    max_abs = Decimal(0)
    max_rel = Decimal(0)

    request_id = 1
    for offset in range(0, len(planned), batch_size):
        chunk = planned[offset : offset + batch_size]
        payloads = []
        for _, _, target, calldata in chunk:
            payloads.append(
                {
                    "jsonrpc": "2.0",
                    "id": request_id,
                    "method": "eth_call",
                    "params": [{"to": target, "data": calldata}, block_tag],
                }
            )
            request_id += 1
        try:
            responses = rpc_batch(str(rpc_url), payloads, timeout)
        except Exception as exc:  # noqa: BLE001 - classify transport failure
            for index, row, target, _ in chunk:
                rpc_failures.append({"row": index, "key": key_for(row, key_columns), "target": target, "error": str(exc)})
            continue

        for (index, row, target, _), response in zip(chunk, responses):
            if response.get("error"):
                rpc_failures.append(
                    {"row": index, "key": key_for(row, key_columns), "target": target, "error": response["error"]}
                )
                continue
            try:
                actual = decode_static_return(return_type, response["result"], 0)
            except Exception as exc:  # noqa: BLE001 - classify decode failure
                decode_failures.append(
                    {"row": index, "key": key_for(row, key_columns), "target": target, "error": str(exc)}
                )
                continue
            expected = resolve_value(expected_spec, row)
            ok, abs_diff, rel_diff = compare_values(expected, actual, tolerance)
            max_abs = max(max_abs, abs_diff)
            max_rel = max(max_rel, rel_diff)
            if ok:
                pass_count += 1
            else:
                mismatches.append(
                    {
                        "row": index,
                        "key": key_for(row, key_columns),
                        "target": target,
                        "expected": str(expected),
                        return_name: str(actual),
                        "abs_diff": str(abs_diff),
                        "rel_diff": str(rel_diff),
                    }
                )

    elapsed = time.monotonic() - start
    summary = {
        "name": spec.get("name", "anchor-rpc-witness"),
        "anchor": {"chain_id": anchor.get("chain_id"), "block": block},
        "facts": len(rows),
        "planned_calls": len(planned),
        "passed": pass_count,
        "planning_errors": len(planning_errors),
        "rpc_failures": len(rpc_failures),
        "decode_failures": len(decode_failures),
        "mismatches": len(mismatches),
        "max_abs_diff": str(max_abs),
        "max_rel_diff": str(max_rel),
        "elapsed_seconds": round(elapsed, 3),
    }

    if args.json:
        print(
            json.dumps(
                {
                    "summary": summary,
                    "planning_errors": planning_errors[:max_mismatches],
                    "rpc_failures": rpc_failures[:max_mismatches],
                    "decode_failures": decode_failures[:max_mismatches],
                    "mismatches": mismatches[:max_mismatches],
                },
                indent=2,
                sort_keys=True,
            )
        )
    else:
        print(json.dumps(summary, indent=2, sort_keys=True))
        for label, values in (
            ("planning_errors", planning_errors),
            ("rpc_failures", rpc_failures),
            ("decode_failures", decode_failures),
            ("mismatches", mismatches),
        ):
            if values:
                print(f"\n{label}:")
                for value in values[:max_mismatches]:
                    print(json.dumps(value, sort_keys=True))

    return 0 if not (planning_errors or rpc_failures or decode_failures or mismatches) else 1


if __name__ == "__main__":
    sys.exit(main())
