# Protocol Adapter Guide

This guide explains how to add a new lending protocol line without changing the
platform architecture.

## Adapter Rule

Adapters are protocol-specific, but protocol lines are isolated runtime units.
Do not add a new protocol as a plugin inside another protocol's ingestion loop.

Each new protocol gets:

```text
own process/container
own event source config
own dedicated RPC key
own WAL
own ClickHouse namespace
own watermark
own vector checkpoints
own validation command
```

## Implementation Sequence

1. Define canonical account units.
2. Define market/product identity.
3. Define asset/reserve identity.
4. Define account position identity.
5. Define event topics and touched-state extraction.
6. Define required same-block witnesses.
7. Define witness decoding into canonical units.
8. Define L2/L3/L4 vector payloads.
9. Implement risk kernel.
10. Implement dirty planning rules.
11. Add checkpoint encoders and manifests.
12. Add direct RPC validation.
13. Add L1 protocol-vector publication.
14. Add API serving summaries.
15. Add historical rollups.

## Canonical Unit Checklist

For every account position, answer:

```text
What exact on-chain value does not silently accrue?
What market/index/share state converts it to current assets?
What price converts it to USD?
What risk/config params convert it to borrowing power or liquidation value?
What witness proves it at block N?
What event tells us the account/market might have changed?
```

If the adapter cannot answer these, the protocol is not ready for live risk.

## Aave/Spark Adapter

Canonical units:

```text
scaled aToken balance
scaled variable debt balance
emode category
collateral enabled flag
```

L2:

```text
reserve address
asset price
decimals
liquidity index
variable borrow index
liquidation threshold
ltv
reserve caps/config
```

L3:

```text
initially same as reserve slot
may become separate if modeling products around reserves
```

Required witnesses:

```text
getAssetPrice(asset)
getReserveNormalizedIncome(asset)
getReserveNormalizedVariableDebt(asset)
getReserveData(asset) when config changes
aToken.scaledBalanceOf(account)
variableDebtToken.scaledBalanceOf(account)
user configuration or collateral event-derived flag with validation
```

Risk projection:

```text
collateral_tokens = scaled_a_token * liquidity_index / RAY
debt_tokens = scaled_debt * variable_borrow_index / RAY
collateral_usd = collateral_tokens * price
debt_usd = debt_tokens * price
liquidation_value_usd = collateral_usd * liquidation_threshold
health_factor = liquidation_value_usd / debt_usd
```

Dirty rules:

```text
account transfer/supply/withdraw/borrow/repay:
  touched account

reserve price/index change:
  accounts exposed to reserve

reserve config/e-mode change:
  accounts exposed to reserve or e-mode category
```

## Morpho Adapter

Canonical units:

```text
supply shares
borrow shares
collateral amount
market id
```

L2:

```text
loan asset
collateral asset
oracle price inputs
decimals
```

L3:

```text
market id
total supply assets
total supply shares
total borrow assets
total borrow shares
lltv
irm/oracle config
```

Required witnesses:

```text
market total assets/shares
account supply shares
account borrow shares
account collateral
oracle price
market config
```

Risk projection:

```text
supply_assets = supply_shares * total_supply_assets / total_supply_shares
borrow_assets = borrow_shares * total_borrow_assets / total_borrow_shares
collateral_usd = collateral_amount * collateral_price
debt_usd = borrow_assets * loan_asset_price
liquidation_value_usd = collateral_usd * lltv
```

Dirty rules:

```text
account-market event:
  touched account in market

market total shares/assets change:
  accounts exposed to market

oracle/lltv/config change:
  accounts exposed to market
```

## Fluid Adapter

Canonical units depend on the product, but usually include:

```text
user vault shares or position units
debt units
collateral units
vault/product id
```

L3 is first-class:

```text
vault/product state
exchange rates
liquidity params
borrow params
liquidation params
resolver state
```

Required witnesses:

```text
vault/product state
account position state
oracle prices
risk params
```

Dirty rules:

```text
account position change:
  touched account

vault exchange/rate/state change:
  accounts exposed to vault

oracle/risk param change:
  accounts exposed to affected vault/product
```

## Euler Adapter

Canonical units:

```text
account vault balances
account liabilities
vault/share units where applicable
collateral enabled state
controller/config state
```

L3:

```text
vault/market configuration
oracle configuration
liquidation and collateral factors
interest/index state
```

Required witnesses:

```text
account position in vault
vault totals/share conversion
oracle prices
risk config
controller state
```

Dirty rules:

```text
account-vault event:
  touched account

vault/share/index/config change:
  accounts exposed to vault

oracle change:
  accounts exposed to priced asset
```

## Compound III / Comet Adapter

Canonical units:

```text
base principal
collateral balances
```

L3:

```text
Comet market
base supply index
base borrow index
collateral factors
liquidation factors
price feeds
```

Required witnesses:

```text
base indexes
account base principal
account collateral balances
collateral prices
market config
```

Dirty rules:

```text
account event:
  touched account

base index change:
  all accounts with base principal

collateral price/config change:
  accounts exposed to collateral asset
```

## Vault Adapter

Canonical units:

```text
user shares
```

L3:

```text
vault total assets
vault total supply
share price or exchange rate
asset price
vault risk params if lending-related
```

Required witnesses:

```text
balanceOf(account)
totalAssets
totalSupply
asset oracle price
vault config
```

Risk projection:

```text
assets = shares * total_assets / total_supply
usd = assets * asset_price
```

## Adapter Output Contract

Adapters must emit:

```text
DecodedEvents
TouchedState
WitnessCalls
DecodedWitnesses
VectorMutations
RiskKernel
ValidationPlan
RollupDefinitions
```

Adapters must not:

```text
write directly to another protocol's namespace
advance platform or L0 watermarks
mutate L0 state
share RPC keys with another protocol
store live risk only in ClickHouse
```

## Validation

Each adapter must include direct validation:

```text
sample accounts
fetch exact same-block direct state
compare canonical units
compare derived risk for sampled accounts
compare protocol aggregate within tolerance
report failed witnesses and mismatches
```

For Aave, this is scaled balance validation. Other protocols need equivalent
canonical-unit validation.

## Adapter Acceptance Criteria

An adapter is ready when:

```text
canonical units are clearly defined
required witnesses prove those units
event decoder identifies touched state
witness decoder emits deterministic mutations
risk kernel is deterministic
dirty planning avoids unnecessary full recompute
snapshots include adapter-specific payload versions
direct validation passes
L1 protocol vector publication is implemented
```
