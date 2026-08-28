# RLD Credit Notes protocol model

## Contents

- [Source status](#source-status)
- [Current product thesis](#current-product-thesis)
- [Economic model](#economic-model)
- [Protocol layers and accounting boundaries](#protocol-layers-and-accounting-boundaries)
- [Normal-state mechanism](#normal-state-mechanism)
- [Credit Index and lending integration](#credit-index-and-lending-integration)
- [Custom-maturity rate products](#custom-maturity-rate-products)
- [Default state](#default-state)
- [Current product stack](#current-product-stack)
- [Use cases](#use-cases)
- [Claims requiring verification](#claims-requiring-verification)
- [Open design questions](#open-design-questions)

## Source status

- Canonical page: [RLD Memo Current](https://app.notion.com/p/3c7c2f1bc76a802ea53fc1c16159086d)
- Title returned by Notion: `RLD Memo [Current]`
- Parent: `RLD`
- Page ID: `3c7c2f1b-c76a-802e-a53f-c1c16159086d`
- Fetched from Notion: 2026-08-28
- Fetch timestamp returned by Notion: 2026-08-28T12:37:14.183Z
- Search timestamp returned for the page: 2026-08-28T12:40:00.000Z

This file is a working distillation of the current memo, not independent validation. Re-fetch the page for current product status, figures, examples, integrations, or exact language. Do not silently merge `RLD Credit Notes v2`, `RLD Paper D2`, the former `Credit Notes Protocol` page, or copied drafts.

The bundled JTM reference is a specialized execution design. It does not override this memo's Credit Note liability, Credit Index, maturity, accounting, or settlement semantics.

## Current product thesis

RLD separates a yield strategy into risks that can be priced and transferred independently:

- time and funding cost;
- liquidity and exit risk;
- floating interest-rate risk;
- collateral-default risk.

Loans remain in existing venues. Independent underwriters lock separate capital against a predefined collateral failure. All Credit Notes covering the same objectively defined collateral risk trade in one perpetual market. Their normalized market price becomes a shared credit-risk reference, while prefunded TWAP execution creates each user's selected maturity without issuing a dated Credit Note for every term.

The target outcomes are:

- lenders and allocators: fixed yield plus funded collateral protection;
- borrowers: fixed financing cost aligned with their position duration;
- underwriters: premium for fully funding defined collateral risk without funding the loan.

The memo's initial maturity range is one hour to one year. Broader “any maturity” language is a product objective, not evidence of unbounded executable liquidity.

## Economic model

The memo uses the structural-credit intuition:

```text
private credit = bond + written put on the borrower's assets
borrow rate = time and liquidity rate + collateral credit spread
```

When asset recovery falls below debt, the borrower can rationally default and leave the shortfall with the lender. Utilization curves price local funding and liquidity conditions but do not independently fund recovery from collateral failure. RLD separates the embedded put from the loan: lenders fund debt, while underwriters supply ring-fenced recovery capital.

The underwriter occupies the concentrated junior risk position. The memo states the economic constraint:

```text
underwriter yield >= lending yield
```

Compare returns per dollar of capital committed, not the protection fee as a percentage of total loan value.

The Credit Note has two states:

- normal: its market tracks the floating collateral-risk premium and supports rate hedges;
- default: it becomes a funded claim against protection capital.

The memo expresses the relationship:

```text
interest-rate swap on collateralized debt = credit-default swap on collateral
```

Treat this as an economic relationship between the loan's credit component and collateral failure, not an identity that merges swap and default accounting.

## Protocol layers and accounting boundaries

For each defined collateral risk, RLD creates one Credit Note Hub and one perpetual Credit Note market:

```text
protection capital -> Credit Notes -> Credit Index -> lending and rate products
```

Keep these domains separate:

1. protection capital and active default-claim liability;
2. Credit Note ownership and settlement eligibility;
3. market-making inventory and quote assets in the Credit Liquidity Vault;
4. prefunded TWAP deposits, fills, proceeds, and unwind state;
5. interest-rate-swap margin, periodic payments, and closeout;
6. lending liquidity, debt, collateral, liquidation, and venue risk controls.

Protection capital is ring-fenced. It cannot be lent, rehypothecated, or used as market-making liquidity. The Credit Liquidity Vault improves execution but does not back default claims. Existing lending venues retain their loans, collateral, liquidity, floating base rates, LTVs, liquidations, oracles, permissions, and local rate models.

## Normal-state mechanism

### 1. Underwriters commit capital

Underwriters deposit eligible protection capital `C` into the Hub. Their return per dollar exposed is:

```text
y_U = annual premiums earned / average protection capital locked
```

Before issuance, the Hub defines the referenced collateral, default trigger, observation window, eligible protection capital, payout cap, withdrawal delay, oracle, and settlement process.

### 2. The Hub mints amortizing Credit Notes

Definitions:

- `C`: eligible protection capital;
- `K`: maximum protection per note at issuance;
- `F`: continuous amortization rate;
- `t`: elapsed time from the note's defined start;
- `A(t)`: amortization factor;
- `L(t)`: remaining claim per note;
- `N`: active note quantity.

Equations:

```text
A(t) = exp(-F t)
L(t) = K * A(t)
N0 = C / K
used capital = N * K * A(t)
free capital = C - N * K * A(t)
solvency invariant: N * K * A(t) <= C
```

Amortization reduces existing liability and releases issuance capacity. It is not yield, a premium payment, or proof of a fixed-rate outcome.

### 3. Notes enter one perpetual market

Underwriters mint against available capacity and sell transferable Credit Notes. Let `P(t)` be the market price of one note. The market turns demand for funded collateral protection and supply of underwriting capital into an observable price.

Do not advertise free capacity as minted inventory. Any execution system must escrow notes or hold an atomic, exclusive mint reservation before presenting sell-side liquidity.

### 4. Buyers choose coverage and maturity

A buyer chooses coverage `H` and end time `T`. Before maturity:

```text
coverage = Q(t) * L(t)
Q(t; H, T) = H / (K * A(t)), for 0 <= t < T
Q(0) = H / K
delta_Q = Q(t) * (exp(F * delta_t) - 1)
```

A prefunded TWAP account gradually buys `delta_Q` as liability amortizes. At `T`, replenishment stops and the remaining position is sold or closed, subject to execution liquidity:

```text
Q(t; H, T) = 0, for t >= T
```

Maturity is created by execution, not separate dated Credit Note markets. This consolidates the base liquidity but moves path dependence, funding sufficiency, slippage, and closeout risk into the execution layer.

### 5. The Credit Liquidity Vault supports execution

The memo describes an HLP-like vault that may:

- hold Credit Note inventory and quote assets;
- fill scheduled buyer and underwriter flow;
- net opposing orders;
- absorb temporary imbalances;
- route residual flow externally.

It must quote and execute protection and rate markets without mixing their accounting. Its inventory, leverage, loss allocation, oracle, adverse-selection limits, fees, and recovery path remain implementation requirements.

## Credit Index and lending integration

Raw note price must be normalized by remaining live protection:

```text
p(t) = P(t) / L(t) = P(t) / (K * A(t))
r_CN(t) = annualize(TWAP_w[p(t)])
```

`p(t)` is premium per dollar of live protection. A guarded TWAP over window `w` smooths trades, then an annualization convention produces the Credit Index `r_CN`.

The memo does not fix the annualization rule, quote convention, guard parameters, stale-market behavior, liquidity threshold, or manipulation bounds. Specify them before deployment. If the market quotes directly in annualized units, the conversion is embedded in that quote but still needs a canonical convention.

A lending venue may consume the index:

```text
r_loan = r_base + r_liquidity + r_CN + r_margin
```

For a dollar loan, `r_base` may be an external funding reference; `r_liquidity` remains venue-local; `r_CN` prices defined collateral risk; and `r_margin` is the venue fee or operating spread. The memo identifies Aave V4 Hub/Spoke as an architectural integration example, not a deployed RLD integration unless independently verified.

## Custom-maturity rate products

The memo adds a separate perpetual interest-rate market. A swap exchanges a floating reference for an agreed fixed rate:

```text
swap payment = D * (r_floating - r_fixed) * days / 365
```

- A floating-rate borrower goes long the rate to offset rising borrowing cost.
- A floating-rate lender shorts the rate to offset falling lending yield.
- The selected maturity `T` controls the execution and settlement period, not the issuance of a new dated base instrument.

Economic targets:

```text
floating interest - long-rate PnL ~= fixed borrowing cost
floating yield + short-rate PnL ~= fixed lending yield
perpetual rate market + scheduled TWAP unwind = custom-maturity fixed rate
```

The approximation depends on fees, margin, settlement cadence, reference alignment, basis risk, execution path, liquidity, and final closeout. The rate position does not pay for collateral default. Only the Credit Note provides the predefined default claim.

## Default state

On confirmed predefined collateral failure:

1. stop new minting;
2. freeze underwriter withdrawals;
3. snapshot active Credit Note ownership;
4. move eligible protection capital into settlement;
5. freeze amortization at default time `t_d`;
6. calculate claims from remaining liability;
7. pay holders, pro rata if available capital is insufficient;
8. release excess only after settlement finality.

Equations:

```text
L_d = K * A(t_d)
total claims = N * L_d
R = min(1, C_d / (N * L_d))
holder payout = q * L_d * R
```

Safety relationship:

```text
withdrawal delay >= observation window + settlement finality window
```

Freeze minting and withdrawals during a qualifying event, stale or unsafe oracle state, dispute, or settlement. Define ownership for notes in execution escrow and protect the snapshot from transfer, activation, and last-block gaming.

## Current product stack

The 2026-08-28 memo reports the following stack. These are dated source claims; re-fetch before presenting them as current:

- **Data Node:** agent-readable exposure from transactions through accounts, markets, vaults, protocols, and assets; reported 90% ready, with cross-protocol historical PnL remaining.
- **Stress Simulator:** reproducible rate, price, liquidity, and collateral shock propagation; pending Data Node completion.
- **Credit Risk Market:** fully funded protection lifecycle; contracts reported ready, frontend remaining.
- **Hub-and-Spoke DEX:** custom-maturity rate swaps and TWAP execution against shared perpetual liquidity; reported ready.
- **Credit Liquidity Vault:** HLP-like liquidity across protection and rate markets without accounting merger; reported ready.
- **Inference Credit Network:** compute-financing vertical connecting operators, lenders, and underwriters; explicitly a hypothesis gated by contracted demand, controlled revenue, and enforceable equipment recovery.

The ten-year objective is `$1 trillion` of protected reference value, meaning linked loans or assets, not capital held by RLD. Treat it as an objective, not a forecast or current metric.

## Use cases

- **Fixed-yield synthetic bond:** floating deposit + short rate position + TWAP.
- **Fixed-rate borrowing:** floating loan + long rate position + TWAP.
- **Rate arbitrage:** directional rate exposure with TWAP closeout; mean reversion is an assumption, not a guarantee.
- **Protected looping:** leveraged yield + rate hedge + Credit Notes; distinguish financing-cost protection from collateral-default protection.
- **Optional tranching:** junior, mezzanine, or senior applications above the protocol; tranching does not change Hub liability, protection capital, Credit Index, or default rules.
- **Compute financing:** productive-asset debt with operator equity, controlled revenue, telemetry, collateral rights, and recovery; this remains a vertical hypothesis until its gates are demonstrated.

## Claims requiring verification

Treat the memo's market figures, borrower cohorts, protocol statuses, incident narratives, competitor descriptions, integration examples, and long-term objective as attributed claims unless independently verified. This includes its active-loan and interest estimates, high-LTV revenue distribution, Stream/Resolv/KelpDAO figures, status of the RLD stack, and readiness claims.

## Open design questions

- What exact annualization converts guarded normalized premium into `r_CN`?
- What quote convention, TWAP guard, staleness rule, and minimum liquidity make the Credit Index admissible?
- Is amortization global or issuance-cohort specific, and what is the canonical time origin?
- How are transfers, burns, partial fills, rounding, and cohort mixing handled?
- What additional collateral buffer, if any, is required beyond `N*K*A(t) <= C`?
- Which assets qualify as protection capital, and how is wrong-way risk bounded?
- How are oracle disputes, pauses, upgrades, and governance capture handled?
- How does a prefunded account prove it can maintain `H` until `T` under price and liquidity stress?
- What happens when the TWAP cannot replenish coverage or unwind at maturity?
- What are the rate market's margin, liquidation, settlement, funding, and closeout rules?
- Which floating reference is hedged, and how are basis risk and venue divergence measured?
- How does the Credit Liquidity Vault bound inventory risk without touching protection capital?
- Who owns a default claim for notes escrowed in a TWAP, router, auction, or vault?
- How are snapshots protected from last-block transfers and execution-state gaming?
- Which JTM assumptions, if any, are actually part of the production execution path?
