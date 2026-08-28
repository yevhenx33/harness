---
name: credit-notes-protocol
description: "Analyze, explain, design, review, or implement RLD Credit Notes: fully funded collateral-default protection, one perpetual market per defined collateral risk, the normalized Credit Index, custom-maturity rate hedges and synthetic bonds, prefunded TWAP execution, lending integrations, Credit Liquidity Vaults, and deterministic default settlement. Use for RLD credit, Credit Note Hubs, onchain CDS, collateral protection, credit-spread discovery, fixed yield or borrowing costs, protected loops, rate/default hedges, JTM, GhostRouter, or solver-routed execution."
---

# Credit Notes Protocol

Use the current memo's model without silently filling its open design gaps. Read [references/protocol-model.md](references/protocol-model.md) before substantive analysis, design, or implementation. For matching, limit orders, ghost liquidity, or solver routing, also read [references/jit-matching-engine.md](references/jit-matching-engine.md); consult the bundled [original PDF](references/04-the-just-in-time-matching-engine.pdf) when exact JTM language or diagrams matter.

## Establish source scope

1. Treat [RLD Memo Current](https://app.notion.com/p/3c7c2f1bc76a802ea53fc1c16159086d) as the canonical source.
2. For exact wording, current figures, or work that may depend on paper revisions, fetch the Notion page again before answering.
3. If Notion is unavailable, use the bundled snapshot and label it with its fetch date.
4. Separate three kinds of statements explicitly:
   - memo-defined mechanism or invariant;
   - inference required to make the design implementable;
   - external or time-sensitive claim requiring independent verification.
5. Do not merge mechanics from `RLD Credit Notes v2`, `RLD Paper D2`, the former `Credit Notes Protocol` page, or copied drafts unless the user asks for comparison.
6. Treat JTM as a specialized execution design. The current memo owns liability, Credit Index, maturity, protection-pool, and settlement semantics.
7. Treat product readiness, market figures, incidents, integrations, and the ten-year objective as dated memo claims unless independently verified.

## Model the market

Describe one Credit Note Hub and one perpetual market per objectively defined collateral risk. Record at least:

- insured asset and reference value;
- objective default rule and observation window;
- eligible protection collateral;
- payout cap `K` per note;
- amortization rate `F` and time origin;
- oracle, stale-data behavior, and settlement finality;
- withdrawal timelock;
- active note supply and protection collateral;
- note price, live protection, normalized premium, guarded TWAP, and annualization convention;
- lending spokes consuming the spread;
- borrower, lender, and underwriter hedge objectives.

Keep protection capital, trading liquidity, prefunded execution accounts, rate-swap margin, and lending liquidity in separate accounting domains. The Hub shares risk pricing, protection capacity, and settlement; each lending venue retains its loan, collateral, liquidity, floating base rate, LTV, maturity, liquidation, oracle, permissions, and rate model.

## Apply the core math

Use the memo's notation unless the task defines another convention:

```text
A(t) = exp(-F t)
L(t) = K * A(t)
initial note capacity N0 = C / K
used capacity = N * K * A(t)
free capacity = C - N * K * A(t)
solvency: N * K * A(t) <= C
notes for fixed coverage H before maturity T: Q(t; H, T) = H / (K * A(t))
increment over delta_t: delta_Q = Q(t) * (exp(F * delta_t) - 1)
normalized premium p(t) = P(t) / L(t) = P(t) / (K * A(t))
Credit Index r_CN(t) = annualize(TWAP_w[p(t)])
loan rate = base rate + liquidity rate + r_CN + venue margin
swap payment = D * (r_floating - r_fixed) * days / 365
claim per note at default = K * A(t_default)
total claims = N * K * A(t_default)
payout ratio = min(1, available protection capital / total claims)
```

Show units, time basis, rounding, and assumptions. The memo defines normalization by live protection and a guarded TWAP, but not the exact annualization rule, quote convention, guard parameters, or manipulation bounds. Do not invent them.

## Create maturity through execution

All notes for the same collateral risk share one perpetual market. A buyer selects coverage `H` and maturity `T`; a prefunded TWAP account replenishes the amortizing position until `T`, then stops and closes or sells the remainder subject to execution liquidity. Maturity belongs to the execution schedule, not a dated Credit Note issuance.

For rate products, keep the default claim distinct from the interest-rate swap. A floating-rate lender shorts the rate to target fixed yield; a floating-rate borrower goes long the rate to target fixed borrowing cost. State fees, basis risk, margin, settlement cadence, closeout, and execution slippage before calling either outcome fixed.

## Check protocol invariants

- Back every active note liability with eligible protection collateral under the stated accounting model.
- Never rehypothecate protection collateral into lending positions.
- Require `withdrawal timelock >= oracle window + settlement finality window`.
- Freeze minting and withdrawals during a qualifying default or unsafe oracle state.
- Snapshot active balances before claims calculation.
- Cap each claim by remaining amortized liability and pay pro rata if collateral is insufficient.
- Define default and settlement rules before issuance; avoid discretionary insurance language.
- Preserve lending-spoke isolation; importing a spread must not merge lending liquidity or risk controls.
- Normalize price by live protection before treating it as a credit signal.
- Stop coverage replenishment at maturity and define the liquidity-dependent unwind.
- Do not let the Credit Liquidity Vault back default claims or protection capital market-make.

## Apply the JTM execution layer

Use JTM only when the design actually relies on its matching layer. Preserve its priority order:

1. net opposing activated ghost flow internally at the oracle TWAP;
2. fill directional ghost remainders against incoming taker flow;
3. expose only the residual to a clearing auction for external solvers.

Model TWAP and limit engines as spokes that publish pre-aggregated virtual balances to a shared router. Back every published ghost balance with escrowed assets or an atomic, exclusive reservation; never publish unminted free capacity as note inventory. Keep JTM optional and execution accounting distinct from Credit Note liability, the Credit Liquidity Vault, swap margin, and lending accounting. Verify oracle-TWAP integrity, activation rules, accumulator solvency, partial fills, pro-rata allocation, auction parameters, and solver liveness before making performance or MEV claims.

## Review designs critically

Test at least these failure surfaces when relevant:

- oracle manipulation, ambiguity, staleness, and delayed confirmation;
- adverse underwriter withdrawal before a default becomes final;
- protection-collateral correlation with the insured event;
- note-price or normalization manipulation feeding the Credit Index and lending rates;
- thin liquidity, stale spreads, and TWAP execution failure;
- ghost-balance accounting errors, activation manipulation, solver failure, and clearing-auction leakage;
- mismatch between debt, note liability, coverage target, execution maturity, and hedge ratio;
- discontinuities at default, snapshot gaming, and secondary-market transfers;
- rounding, time-origin, precision, and exponential-decay implementation;
- insolvency, pro-rata settlement, execution escrow, and excess-collateral release;
- Liquidity Vault inventory loss, adverse selection, and accounting leakage into protection capital;
- rate-swap margin failure, basis risk, settlement gaps, and maturity closeout failure;
- governance authority over hub parameters and emergency actions.

Treat `floating loan + long rate position ~= fixed borrowing cost` and `floating deposit + short rate position ~= fixed yield` as economic constructions. They require precise margin, cash-flow, settlement, basis, fee, and execution specifications before implementation. Only Credit Notes provide the predefined collateral-default payout.

## Produce decision-ready output

For explanations, lead with shared perpetual liquidity, execution-created maturity, and the two Credit Note states: floating credit-risk reference in normal markets and funded claim at default. For designs or reviews, return:

1. verdict or proposed mechanism;
2. actors, state transitions, and fund flows;
3. formulas and a numeric example when useful;
4. invariants and failure modes;
5. unresolved choices or deviations from the memo;
6. canonical Notion source link and fetch date.

Do not present memo-sourced market figures, product statuses, incidents, integrations, or competitor characterizations as independently verified facts.
