# Invariant Spec

Use this reference when defining a reusable anchor RPC witness check.

## JSON Spec Shape

```json
{
  "name": "erc20-balance-anchor-check",
  "anchor": {
    "chain_id": 1,
    "block": 25473710,
    "rpc_url": "http://127.0.0.1:8545",
    "rpc_url_env": "RPC_URL"
  },
  "call": {
    "target": { "column": "contract_address" },
    "signature": "balanceOf(address)(uint256)",
    "selector": "0x70a08231",
    "args": [
      { "column": "holder", "type": "address" }
    ],
    "returns": [
      { "name": "actual", "type": "uint256" }
    ]
  },
  "compare": {
    "key": ["holder", "contract_address"],
    "expected": { "column": "indexed_balance_raw", "type": "uint256" },
    "actual": { "return": "actual", "type": "uint256" },
    "tolerance": { "mode": "exact" }
  },
  "runtime": {
    "batch_size": 100,
    "timeout_seconds": 30,
    "max_mismatches": 25
  }
}
```

`rpc_url_env` is preferred for reusable specs. `rpc_url` is acceptable for local non-secret endpoints.

## Fact Rows

Facts are JSONL rows. Each row should contain:

```json
{"holder":"0x...","contract_address":"0x...","indexed_balance_raw":"123"}
```

Rows should be one claim each. If one indexed entity has multiple invariants, either emit multiple rows with an `invariant` field or run separate specs.

## Comparison Modes

`exact`:

- Use for integers, addresses, booleans, bytes, and canonical protocol units.
- Normalize integer strings and hex quantities before comparing.

`absolute`:

- Use for values where a fixed error bound is meaningful.
- Include `"value": "1000"` or a decimal string.

`relative`:

- Use for projected decimals, USD values, prices, and ratios.
- Include `"value": "1e-12"` or similar.
- Treat zero expected values carefully; fall back to absolute diff when expected is zero.

## Output Contract

Every run should report:

- invariant name
- anchor block and chain id
- source fact count
- planned call count
- successful witness count
- RPC/decode failure count
- mismatch count
- max absolute and relative diff
- top mismatches by relative or absolute diff

Do not collapse RPC/decode failures into invariant mismatches. A failed witness is unknown, not false.
