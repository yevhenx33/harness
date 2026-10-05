# Multicall3 Guidance

Use Multicall3 when checking many read-only calls at the same anchor block.

## Default Address

Common deployment:

```text
0xca11bde05977b3631167028862be2a173976ca11
```

Do not assume this address exists on every chain. Verify code exists at the anchor block with `eth_getCode`.

## Preferred Pattern

Use `aggregate3` for read-only witness checks:

```solidity
function aggregate3(Call3[] calldata calls)
  returns (Result[] memory returnData);

struct Call3 {
  address target;
  bool allowFailure;
  bytes callData;
}

struct Result {
  bool success;
  bytes returnData;
}
```

Set `allowFailure = true` for audit runs so one bad contract or missing method does not abort the whole batch. Classify unsuccessful results separately.

## Chunking

Start conservative:

```text
100 to 500 calls per Multicall3 call for large return data
500 to 1500 calls for small static returns
```

Reduce chunk size when the RPC returns payload-limit, timeout, or execution-reverted errors.

## Block Tag

The Multicall3 `eth_call` itself must use the anchor block tag. Do not use `block.number` returned inside Multicall3 as the validation anchor unless that is explicitly part of the invariant.

## Fallbacks

Use batched JSON-RPC `eth_call` when:

- the chain lacks Multicall3 at the anchor block
- calls are heterogeneous and dynamic decoding is easier one-by-one
- the RPC provider handles HTTP batch calls better than large EVM multicalls
- debugging a small mismatch set

The same-block invariant still holds as long as every `eth_call` uses the same anchor block tag.

## Failure Classes

Report separately:

- multicall transport failure
- aggregate call revert
- per-call `success = false`
- empty return data
- ABI decode failure
- invariant mismatch

Only the last one is a proven indexed-data mismatch.
