# Ingest And Witness Architecture

Event ingestion discovers what changed. Witnesses prove exact same-block state.
They are separate concerns and must stay separate.

## Rule

```text
Events identify touched state.
Witnesses provide canonical same-block state.
```

Do not use event payloads alone as account truth unless the protocol's event is
explicitly sufficient and the adapter marks it as such.

## Isolated Ingestion Lines

Each protocol line owns:

```text
preferred event source credentials
RPC fallback config
dedicated protocol RPC API key
Multicall batch size
RPC concurrency
retry policy
rate-limit policy
failure thresholds
event watermark
witness metrics
```

Example environment shape:

```text
AAVE_RPC_URL
AAVE_HYPERSYNC_API_TOKEN
AAVE_WAL_DIR
AAVE_CLICKHOUSE_DATABASE

MORPHO_RPC_URL
MORPHO_HYPERSYNC_API_TOKEN
MORPHO_WAL_DIR
MORPHO_CLICKHOUSE_DATABASE
```

Protocol lines may share provider infrastructure only if they still have separate
API keys and rate limits.

## Event Source Strategy

Preferred:

```text
indexed event source such as Hypersync
```

Fallback:

```text
eth_getLogs through the protocol's dedicated RPC key
```

The fallback path must produce the same normalized event facts as the preferred
source.

## Event Fact Contract

Event facts should include:

```text
protocol_id
deployment_id
chain_id
block_number
block_hash
block_timestamp
transaction_hash
transaction_index
log_index
contract_address
event_name
topic0
topics
data
decoded_fields
touched_accounts
touched_assets
touched_markets
touched_positions
event_hash
source_type
producer_version
inserted_at
```

Event facts are append-only.

## Touched State

Decoded events become a touched-state graph:

```text
TouchedState {
  accounts
  assets
  reserves
  markets
  account_market_pairs
  account_asset_pairs
  position_identities
  broad_market_updates
  broad_protocol_updates
}
```

The touched-state graph is used only to plan witnesses and dirty recompute. It is
not itself final state.

## Witness Planning

Witness planning must be deterministic for:

```text
same protocol spec
same block
same touched state
same vector state
```

Witness calls should be grouped by:

```text
market/reserve state
asset price state
account position state
account config state
protocol config state
debug/compatibility state
```

Required witness examples:

```text
Aave:
  oracle getAssetPrice
  pool getReserveNormalizedIncome
  pool getReserveNormalizedVariableDebt
  pool getReserveData where config changed
  aToken scaledBalanceOf(account)
  debt token scaledBalanceOf(account)

Morpho:
  market total supply/borrow assets and shares
  oracle price
  account supply/borrow shares
  account collateral

Fluid:
  vault/product state
  oracle state
  account vault position

Compound III:
  base indexes
  collateral prices
  account base principal
  account collateral balances
  market config
```

Optional/debug witness examples:

```text
balanceOf when scaledBalanceOf is canonical
redundant resolver data
human-readable metadata
```

## Same-Block Consistency

Witnesses must be fetched at the block being committed:

```text
eth_call blockTag = block_number
Multicall3 aggregate3 at block_number
```

Using latest state for a historical block is not acceptable.

## Multicall Batching

The runtime should expose knobs:

```text
multicall_batch_size
rpc_concurrency
max_retries
request_timeout_ms
required_failure_policy
```

Batch planning should be deterministic, but execution can be concurrent.

RPC metrics per block:

```text
planned_calls
executed_calls
successful_calls
failed_required_calls
failed_optional_calls
batches
batch_latency_p50_ms
batch_latency_p95_ms
provider_error_counts
rate_limit_count
```

## Witness Fact Contract

Witness facts should include:

```text
run_id
protocol_id
deployment_id
chain_id
block_number
block_hash
block_timestamp
witness_group
target_address
call_selector
calldata
return_data
decoded_value
rpc_success
rpc_error
required
witness_hash
producer_version
inserted_at
```

Witness facts are durable truth for exact chain state used by the vector runtime.

## Failure Handling

Required witness failure:

```text
write failure diagnostics
do not apply partial required state as canonical
do not advance watermark
retry according to policy
keep protocol line isolated
```

Optional witness failure:

```text
write diagnostics
continue if ProtocolSpec allows it
mark missing optional witness in run details
```

Event source failure:

```text
try fallback if configured
if fallback succeeds, continue
if no source succeeds, do not advance watermark
```

RPC rate limiting:

```text
reduce concurrency or batch size if adaptive policy is enabled
pause protocol line if required witness success cannot be achieved
do not affect other protocol lines
```

## Witness To Mutation

WitnessDecoder outputs protocol-neutral mutation categories:

```text
PriceUpdate
IndexUpdate
MarketStateUpdate
AccountPositionUpdate
AccountConfigUpdate
ProtocolConfigUpdate
```

Each mutation contains enough protocol-specific payload for the risk kernel but
uses shared ids once passed through slot registries.

## Ordering

Within a block:

```text
events ordered by transaction_index, log_index
witnesses grouped by deterministic planner order
mutations applied in deterministic category order
```

Recommended mutation order:

```text
1. asset/reserve metadata
2. price updates
3. market/index/config updates
4. account config updates
5. account position updates
6. zero/delete position updates
7. dirty planning
8. risk recompute
```

## Security And Secrets

RPC keys are protocol-line secrets. They should not be shared through L0.

Do not log:

```text
RPC API keys
full provider URLs with credentials
private token values
```

It is acceptable to log:

```text
provider label
status code
rate-limit counters
request class
batch size
latency
```

## Acceptance Criteria

Ingest and witness architecture is acceptable when:

```text
each protocol line has a dedicated RPC key
event fallback produces equivalent facts
witness plans are deterministic
required witnesses use same-block eth_call
required witness failures block watermark advancement
witness facts are durable and hashable
decoded witnesses become normalized mutations
RPC degradation in one line does not affect others
```
