---
name: anchor-rpc-witness
description: General same-block RPC witness validation for indexed or materialized onchain data. Use when the user asks to cross-check, validate, audit, or compare indexed facts against direct RPC calls at a specific anchor block, including Multicall3 witness runs, archive-RPC checks, invariant checks, snapshot validation, or block-tagged read-only contract calls for any protocol or data type.
---

# Anchor RPC Witness

## Purpose

Validate indexed facts against direct same-block onchain witnesses. Treat the indexed row as a claim, the block-tagged RPC call as the witness, and the invariant as the comparison rule.

This skill is generic. Do not assume the data is account, lending, token, oracle, pool, or protocol-specific unless the current task provides an adapter or schema.

## Non-Negotiables

- Keep the workflow read-only by default. Do not mutate databases, code, services, git state, or chain state.
- Use one explicit anchor block for both indexed facts and RPC calls.
- Confirm the RPC endpoint supports archive reads at the anchor block before large fanout.
- Compare canonical/raw integer values first. Compare derived floats, USD values, or ratios only as secondary checks with explicit tolerances.
- Classify failures separately: source-missing, witness-missing, RPC failed, decode failed, invariant mismatch.
- Keep large checks bounded by chunks, retries, and clear runtime/call-count reporting.
- Report the exact anchor metadata: chain id, block number, block timestamp when available, RPC label, Multicall3 address when used, source query or source file, and invariant version.

## Workflow

1. **Define the anchor**
   - Use the user-provided block when available.
   - Otherwise choose a closed, stable block from indexed data and explain why it is safe.
   - Verify archive support with a cheap `eth_getBlockByNumber` or one small `eth_call` at that block.

2. **Extract indexed facts**
   - Use read-only queries or existing files.
   - Produce rows with stable keys, expected values, target contract, call args, and optional metadata.
   - For "all" checks, define the population precisely, such as active-at-anchor, all indexed rows in a window, all configured markets, or all contracts in a registry.

3. **Build the witness plan**
   - Define the ABI signature or selector, target address, args, return type, expected column, key columns, and comparison tolerance.
   - Prefer Multicall3 for large homogeneous read calls on chains where it is deployed and reliable.
   - Use batched JSON-RPC `eth_call` as a fallback for small or heterogeneous checks.

4. **Execute same-block calls**
   - Pass the anchor as the block tag on every witness call.
   - Chunk calls conservatively. Start small, then increase only after observing RPC stability.
   - Retry transient failures by chunk, not by restarting the whole check.

5. **Compare invariants**
   - Use exact equality for addresses, bytes, booleans, and integer canonical units unless the adapter specifies otherwise.
   - Use absolute or relative tolerance for decimal projections, prices, rates, or values derived through floating point.
   - Preserve the original indexed value, witness value, absolute diff, relative diff, key, and call metadata for mismatches.

6. **Report**
   - Lead with pass/fail counts and whether the invariant is trustworthy.
   - Include anchor metadata, checked row count, call count, failed calls, mismatch count, max diff, and representative mismatches.
   - State sampling limits if the check is not exhaustive.

## Bundled Resources

- `scripts/run_anchor_witness.py`: generic JSON spec + JSONL fact harness for static ABI `eth_call` witness checks. Use it when the indexed facts can be exported as JSONL and the witness is a static read call.
- `references/invariant-spec.md`: spec shape, comparison modes, output contract, and examples.
- `references/multicall3.md`: Multicall3 usage guidance, chunking rules, failure handling, and when to fall back.
- `references/adapters.md`: small adapter recipes for common witness classes such as ERC20 balances, oracle values, storage slots, pool state, and protocol-specific scaled units.

## Script Quick Start

Create a JSON spec and JSONL facts outside the skill directory, then run:

```bash
python3 /home/ubuntu/.codex/skills/anchor-rpc-witness/scripts/run_anchor_witness.py \
  --spec /path/to/spec.json \
  --facts /path/to/facts.jsonl
```

Use `--dry-run` first for large checks to validate call planning without making RPC calls.

For complex dynamic ABI returns, protocol-specific decoding, or native Multicall3 packing, read the references and create a task-local adapter script rather than overloading the generic harness.
