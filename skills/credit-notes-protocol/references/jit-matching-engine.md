# Just-In-Time Matching Engine

## Source and role

- Original: [04: The Just-In-Time Matching Engine](04-the-just-in-time-matching-engine.pdf)
- Pages: 5
- PDF creation date: 2026-06-06
- SHA-256: `97f1935524071261d123e7503d4f84158913d8cbeb83c8cc670484c99d54c17e`

Use this paper as the execution-layer complement to the canonical Credit Notes Protocol paper. It specifies how programmatic TWAP and limit-order flow may be matched. It does not redefine credit-note liability, protection collateral, default rules, or settlement.

Treat incident narratives, literature characterizations, and performance claims as paper claims until independently verified.

## Architecture

The JTM engine uses a hub-and-spoke execution model:

```text
TwapEngine spoke ----\
                       -> GhostRouter hub -> internal match / taker fill / auction
LimitEngine spoke ---/
```

Activated programmatic orders become fungible virtual `ghost` balances. The router intercepts flow before it reaches a passive AMM and applies strict priority:

1. **L1 internal netting:** Cancel opposing ghost streams at the oracle TWAP.
2. **L2 JIT fill:** Match the directional ghost remainder directly against incoming taker flow.
3. **L3 clearing auction:** Offer only the residual flow through a dynamic Dutch-auction discount to external solvers.

The design aims to avoid forcing unmatched TWAP flow through a constant-product curve, where makers absorb price impact and arbitrageurs capture the correction.

## Order primitives

### Continuous TWAP

A TWAP stream accrues a latent ghost balance at `sellRate * deltaTime`. The paper treats it as a dense sequence of protocol-priced instantaneous limit orders. Incoming flow can match accrued balance at the current protocol TWAP.

### Ghost Limit

A user deposits tokens at a target tick in the LimitEngine. The order remains dormant until a five-minute protocol TWAP crosses that tick, then joins the aggregate ghost pool. The paper intends this boundary to resist intra-block activation manipulation.

Execution is unidirectional: a later price reversal does not undo a completed fill. Earnings are distributed through a global accumulator.

### Pre-aggregation and allocation

Orders are aggregated on deposit or activation rather than iterated during a taker swap. The paper claims constant-time fills against aggregate virtual state. It replaces FIFO priority with pro-rata distribution among depositors sharing an activated tick.

Do not infer constant total system cost from constant-time taker matching alone; account for deposit, activation, accumulator update, claim, and tick-management costs.

## Solver routing assumption

The paper assumes external solvers will route flow through the ghost pool whenever external price and internal ghost TWAP diverge enough to create an arbitrage opportunity:

```text
P_external != P_ghost -> solver routes flow -> ghost liquidity receives L2 demand
```

This is an economic liveness assumption, not a protocol guarantee. Model gas, latency, inventory, capital, private order flow, censorship, failed delivery, and minimum profitable spread.

## Connection to credit notes

Amortizing credit-note liability requires repeated programmatic execution:

- buyers acquire additional notes to maintain constant dollar coverage;
- underwriters may sell newly freed capacity as liability decays;
- internal netting can cancel opposing scheduled flows before external execution;
- JIT taker matches and auctions can clear the remainder.

Keep three accounting domains separate:

1. protection collateral and active claim liability;
2. credit-note ownership and settlement eligibility;
3. execution deposits, ghost balances, fills, and accrued proceeds.

An execution fill must update note ownership and coverage consistently without letting router balances become protection collateral or lending liquidity.

Publish only funded execution state:

- escrow minted notes before advertising sell-side ghost inventory, or reserve mint capacity exclusively and mint atomically on fill;
- escrow buyer quote assets before advertising buy-side ghost demand;
- do not treat free protection capacity as an asset or ghost balance;
- define beneficial ownership and default-snapshot entitlement for notes held in execution escrow.

## Review checklist

- Define the TWAP oracle, sampling window, update cadence, staleness limits, and manipulation bounds.
- Specify when ghost balance accrues, activates, expires, cancels, and becomes claimable.
- Prove conservation across deposits, virtual balances, fills, fees, proceeds, refunds, and claims.
- Reconcile every ghost balance to escrowed assets or an exclusive atomic reservation.
- Specify accumulator precision, rounding direction, dust handling, and overflow bounds.
- Define price and time priority within and across activated ticks despite pro-rata allocation.
- Bound work when many ticks cross or many spokes activate in one update.
- Specify partial fills and atomic state transitions across router and spokes.
- Prevent reentrancy, callback abuse, stale approvals, and double use of virtual liquidity.
- Define L1 netting when the two sides use different limits, tenors, assets, or oracle observations.
- Define L2 taker pricing, fees, surplus allocation, and behavior when taker demand exceeds ghost liquidity.
- Define L3 auction start price, decay, duration, solver permissions, fallback, and failure recovery.
- Test solver absence, gas spikes, chain congestion, reorgs, and oracle halts.
- Quantify MEV and LVR under realistic latency; avoid absolute claims without measurements and a threat model.
- Specify emergency pause and recovery without trapping or misallocating user funds.

## Open implementation questions

- Is each spoke trusted, permissioned, or proven correct before its balances enter the router?
- How does the router prevent a malicious spoke from publishing unbacked ghost liquidity?
- Are notes minted on TWAP deposit or on fill, and how does that choice affect active liability and default snapshots?
- If the router escrows notes, who receives a claim when default occurs before execution completes?
- Can limit activation and matching occur in the same block, and what oracle observation governs both?
- Who pays execution gas for continuous accrual and tick activation?
- What is the safe fallback when no solver clears L3 residual flow?
- How are fees allocated among makers, takers, solvers, the protocol, and oracle upkeep?
- How does JTM interact with note-price formation used by lending spokes without creating a circular or manipulable rate signal?
