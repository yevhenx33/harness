# Vector Data Model

The vector data model is the core runtime representation. It must support sparse
production accounts, dense stress cases, deterministic recovery, fast dirty
planning, and compact checkpoints.

## Design Goals

```text
1M accounts per platform target
500 asset/market slots per protocol line
block-by-block updates
sparse account positions by default
dense full-recompute fallback under one block interval
deterministic slot ids
compact checkpoint encoding
memory-first serving indexes
protocol-specific risk kernels on shared storage primitives
```

## Slot Registries

Slot registries map external identities to stable numeric ids.

```text
account_address -> account_index
asset_address -> asset_slot
reserve_address -> reserve_slot
market_id -> market_slot
position_identity -> position_slot
```

The registry is append-only within a protocol line:

```text
new identity:
  assign next id

existing identity:
  return existing id

deleted/closed position:
  mark inactive or zeroed
  do not reuse id during the checkpoint epoch
```

Registry snapshots are required for deterministic recovery. Rebuilding indexes
from query order alone is not sufficient once live discovery appends accounts in
runtime order.

## Registry Snapshot Shape

```text
slot_registry_snapshot {
  protocol_id
  deployment_id
  chain_id
  from_block
  to_block
  registry_type
  encoding
  value_count
  registry_bytes
  registry_hash
  producer_version
  schema_version
}
```

Recommended registry encodings:

```text
account registry:
  account_index ordered addresses

asset registry:
  asset_slot ordered addresses and token metadata

market registry:
  market_slot ordered protocol market ids

position registry:
  position_slot ordered tuple(account_index, market_slot, asset_slot, side, sub_id)
```

## Vector Layers

### L2 Asset/Reserve Vector

L2 stores asset-level and reserve-level state.

```text
asset_reserve_vector[asset_slot] = {
  asset_address
  symbol
  decimals
  price_raw
  price_usd
  oracle_address
  supply_index
  borrow_index
  liquidation_threshold
  ltv
  supply_cap
  borrow_cap
  total_supply
  total_debt
  last_updated_block
  state_hash
}
```

For Aave/Spark, this closely matches a reserve. For protocols where markets are
not asset-reserve pairs, L2 is still the normalized asset/price/config layer.

### L3 Market/Product Vector

L3 stores protocol-specific market state.

```text
market_vector[market_slot] = {
  protocol_market_id
  market_kind
  collateral_asset_slots
  debt_asset_slots
  oracle_slots
  risk_params
  liquidity_params
  rate_or_share_state
  vault_state
  total_assets
  total_shares
  last_updated_block
  market_hash
}
```

Examples:

```text
Aave:
  market_slot can initially equal reserve_slot

Morpho:
  market_slot is market id
  state includes total supply assets/shares and borrow assets/shares

Fluid:
  market_slot is vault/product id
  state includes vault exchange/risk state

Compound III:
  market_slot is Comet instance
  state includes base indexes and collateral configs
```

### L4 Account/Position Vector

L4 account state is canonical non-accruing account truth.

Sparse account representation:

```text
account_vector[account_index] = {
  account_address
  account_config
  first_position_slot
  position_count
  last_touched_block
  account_hash
}

position_vector[position_slot] = {
  account_index
  market_slot
  asset_slot
  side
  canonical_amount
  collateral_enabled
  debt_enabled
  sub_position_id
  last_touched_block
  position_hash
}
```

Dense stress representation can be generated or represented as fixed account x
slot arrays, but production should stay sparse unless the protocol truly has a
dense account model.

### L4 Account Risk Vector

Risk is derived and stored separately from canonical account units.

```text
account_risk_vector[account_index] = {
  collateral_usd
  debt_usd
  liquidation_value_usd
  ltv
  health_factor
  risk_flags
  active_collateral_count
  active_debt_count
  risk_hash
}
```

Risk vectors can be recomputed from L2/L3 and account/position vectors.

### L1 Protocol Vector

```text
protocol_vector = {
  protocol_id
  deployment_id
  chain_id
  block_number
  account_count
  position_count
  asset_count
  market_count
  total_collateral_usd
  total_debt_usd
  total_liquidation_value_usd
  risky_accounts
  health_factor_buckets
  top_risky_account_indexes
  exposure_summaries
  dependency_hashes
  protocol_vector_hash
}
```

### L0 Cross-Protocol Vector

```text
cross_protocol_vector = {
  chain_id
  block_alignment_mode
  protocol_count
  stale_protocol_count
  total_collateral_usd
  total_debt_usd
  total_liquidation_value_usd
  risky_accounts_by_protocol
  top_global_risky_accounts
  correlated_asset_exposures
  source_protocol_vector_hashes
  cross_protocol_vector_hash
}
```

## Memory Layout

The hot path should avoid per-account hash maps and object graphs. Use
structure-of-arrays or compact arrays wherever possible:

```text
accounts:
  address[]
  account_config[]
  first_position[]
  position_count[]
  last_touched_block[]
  account_hash[]

positions:
  account_index[]
  market_slot[]
  asset_slot[]
  side[]
  canonical_amount[]
  flags[]
  last_touched_block[]
  position_hash[]

assets:
  price[]
  decimals[]
  supply_index[]
  borrow_index[]
  liquidation_threshold[]
  ltv[]
  state_hash[]

markets:
  market_kind[]
  asset_slot_ranges[]
  risk_param_ranges[]
  liquidity_param_ranges[]
  market_hash[]

risks:
  collateral_usd[]
  debt_usd[]
  liquidation_value_usd[]
  ltv[]
  health_factor[]
  risk_flags[]
```

Maps are acceptable at the boundary:

```text
address -> account_index
asset address -> asset_slot
market id -> market_slot
```

They should not be in the inner risk projection loop.

## Exposure Indexes

Dirty planning needs reverse indexes:

```text
asset_slot -> account indexes exposed to asset
market_slot -> account indexes exposed to market
reserve_slot -> account indexes exposed to reserve
risk_param_epoch -> affected accounts or markets
```

Implementation options:

```text
sorted account-index vectors
bitsets for high-cardinality exposure
compressed roaring bitmaps if needed
hybrid sparse vectors for small markets and bitsets for large markets
```

The API also needs serving indexes:

```text
by_health_factor
by_ltv
by_debt_usd
by_collateral_usd
by_liquidation_distance
by_asset_exposure
by_market_exposure
```

Risk-only indexes can be rebuilt from `account_risk_vector`. Exposure indexes
require account/position vectors or dedicated exposure summaries.

## Dirty Planning

Dirty accounts come from:

```text
account operation:
  touched account only

price update:
  all accounts exposed to asset

index/share/exchange-rate update:
  all accounts exposed to reserve/market/vault

market config update:
  all accounts exposed to market

liquidation/risk parameter update:
  all accounts affected by changed parameter

unknown or broad protocol event:
  protocol-specific fallback, possibly full recompute
```

Dirty planner output:

```text
DirtyPlan {
  account_indexes
  affected_asset_slots
  affected_market_slots
  reason_counts
  full_recompute
  dirty_ratio
}
```

Full recompute is acceptable when:

```text
dirty ratio exceeds threshold
changed state affects most accounts
exposure index is unavailable or suspect
recovery validation requires it
```

## Risk Kernels

The vector runtime supplies read views. The protocol-specific risk kernel
implements projection.

Aave:

```text
collateral_tokens = scaled_a_token_balance * liquidity_index / RAY
debt_tokens = scaled_debt_balance * variable_borrow_index / RAY
collateral_usd = collateral_tokens * price
debt_usd = debt_tokens * price
liquidation_value_usd = collateral_usd * liquidation_threshold
health_factor = liquidation_value_usd / debt_usd
```

Morpho:

```text
supply_assets = supply_shares * total_supply_assets / total_supply_shares
borrow_assets = borrow_shares * total_borrow_assets / total_borrow_shares
collateral_usd = collateral_amount * collateral_price
debt_usd = borrow_assets * loan_asset_price
```

Vault:

```text
assets = user_shares * vault_total_assets / vault_total_supply
usd = assets * asset_price
```

The kernel must be deterministic for the same vector inputs.

## Hashing

Hash each layer separately:

```text
slot_registry_hash
asset_vector_hash
market_vector_hash
account_vector_hash
position_vector_hash
account_risk_vector_hash
protocol_vector_hash
cross_protocol_vector_hash
```

Hash inputs should include:

```text
protocol_id
deployment_id
chain_id
block range
schema_version
producer_version
ordered vector bytes
dependency hashes
```

Never hash unordered maps directly.

## Checkpoint Encoding

Risk vector encoding should be compact and versioned:

```text
encoding = f64_le_v1:collateral,debt,liq,hf
encoding = fixed_decimal_le_v1:scale=1e8:collateral,debt,liq,hf
encoding = zstd:vector_bundle_v1
```

Account and position vector checkpoints should prefer stable binary encodings
over JSON for large snapshots.

Each checkpoint row references a snapshot manifest with dependency hashes.

## Vector Runtime Acceptance Criteria

The vector runtime is acceptable when:

```text
slot ids are stable across restart
dirty planning avoids full recompute for sparse updates
full recompute stays below block interval for dense stress target
risk projection is deterministic
all layer hashes are reproducible
checkpoint decode recreates the same vectors
serving indexes can be updated incrementally or rebuilt deterministically
protocol-specific kernels do not leak storage internals
```
