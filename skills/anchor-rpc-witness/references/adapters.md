# Adapter Recipes

Use this reference to map indexed facts into generic witness specs. These are examples, not mandatory account-specific logic.

## ERC20 Balance

Indexed fact:

```text
holder, token, indexed_balance_raw
```

Witness:

```text
target = token
call = balanceOf(address)(uint256)
arg = holder
compare = exact uint256
```

## ERC20 Metadata

Witness:

```text
decimals()(uint8)
symbol()(string)
name()(string)
```

String returns are dynamic. Use a task-local decoder or `cast call` for small checks instead of the generic static harness.

## Oracle Value

Witness examples:

```text
latestRoundData()
getAssetPrice(address)
price(address)
```

Compare raw oracle answers exactly when the indexed value is stored in the same units. Compare derived USD/decimal values with explicit tolerance.

## Lending Scaled Units

Witness examples:

```text
scaledBalanceOf(address)(uint256)
getReserveNormalizedIncome(address)(uint256)
getReserveNormalizedVariableDebt(address)(uint256)
```

Compare scaled units exactly or near-exactly. Compare projected balances only after confirming the index used at the same anchor block.

## AMM Pool State

Witness examples:

```text
slot0()
liquidity()
getReserves()
```

Prefer exact raw pool state fields. Derived price or TVL checks need explicit formulas and tolerances.

## Storage Slot

For invariants that map directly to storage, use `eth_getStorageAt` at the anchor block. Decode storage separately from contract ABI calls and compare bytes or integer words exactly.
