# Audit Perspectives

Choose the perspective that serves the objective. A combined review can reuse
its map of owners and paths while applying a different deciding check to each
outcome. Use other questions when they expose a dependency that matters to the
requested outcome.

## Correctness and defensive security

- Derive safety and progress properties from the contract. Establish their
  identity and scope, such as per recipient, per event, per account, or system
  wide. A sum can balance while an individual destination is wrong. In the
  OpenZeppelin Miden Standard review, tests found both expected assets somewhere
  among the outputs while one intended payment note received neither asset.
  [Miden Standard audit](https://www.openzeppelin.com/news/miden-standard-diff-audit)
- Inspect the owning checks and trust boundaries. Follow acceptance through
  durable effects. Compare the state read by a guard with the state and
  authority in force when the operation commits. Review defensive behavior
  through source, approved evidence, and safe verification of intended
  contracts.
- Choose cases that distinguish the intended formula from a competing one.
  In the Orga and Merk review, a 50% test value masked a swap between the
  fraction kept and the fraction deducted. An asymmetric fixture separates
  those formulas. [Orga and Merk report](https://raw.githubusercontent.com/trailofbits/publications/master/reviews/2024-11-orgaandmerk-securityreview.pdf)
- Inspect whether a proof's assumptions exclude the bad state or a test always
  exits before the relevant transition. Certora's Notional postmortem shows
  how a vacuous rule passed while providing no protection against the intended
  double-counting case. [Notional postmortem](https://www.certora.com/blog/post-mortem-analysis-of-the-notional-finance-vulnerability)

## Reliability and operations

- Follow a fact through admission, processing, durable commit, publication,
  consumption and recovery. Check which event advances progress and when an
  acknowledgement becomes valid. Examine failure before and after each
  consequential state transition.
- Compare lifetimes and ownership across boundaries. Trace timeout, retry,
  cancellation, restart, stale evidence, partial results and competing workers
  where they matter. Check what is retained or invalidated when recovery changes
  the generation, provider or artifact.
- Verify visible effects at the consumer. Use a durable read, replay result,
  state reconciliation or other independent witness suited to the objective.
  Explain which transitions existing evidence covers and which still need proof.

## Latency and resource efficiency

- Freeze the workload, environment, comparison artifact and end-to-end
  measurement boundary. Identify the critical path or dominant work, including
  repeated work, size, fanout, contention and retries. Use component evidence to
  attribute cost. Measure whole outcomes on comparable valid work to qualify a
  gain.
- For each material step, identify the work it does, what requires it, and what
  could be removed, reused or performed incrementally. Pursue the changes with
  a credible effect on the objective. Account for moved work, background costs,
  peak and retained resources, freshness and failure behavior. ChainSecurity's
  Euler review found that a timestamp shortcut relied on an upstream guarantee
  the dependency did not enforce.
  [Euler price-oracle audit](https://www.chainsecurity.com/reports/Euler/ChainSecurity_Euler_PriceOracles_Audit.pdf)
- When the scope permits reading evidence but no new measurement, separate
  measured costs from hypotheses and name the workload and comparison that
  would decide them. Use the user's acceptance threshold if provided.

## Cleanup

- Establish the owner's contract and active consumers of each retirement
  candidate. Check source references, live dependencies, startup and rollback,
  retention and recovery. Preserve a distinction between unreferenced in a
  source search, unobserved in a sample, and proven safe to retire.
- Identify precise candidates and why removal would improve the objective.
  Quantify recoverable resources where directly supported. Preserve required
  artifacts, shared dependencies and unrelated state. Remove only with explicit
  authority.

## Design challenge and structural alternatives

- Hold the intended outcome and hard invariants fixed. Separate them from
  choices about ownership, representation, timing and coordination. For an
  incremental objective, trace improvements within the agreed mechanism and
  explain any constraint that limits the gain. For a redesign objective,
  challenge the assumptions that make the current work necessary and compare
  viable alternatives against the incumbent under the same acceptance criteria.
- Consider deletion, consolidation at the owner, or inversion of a data or
  control flow when that changes the cause. For each serious alternative,
  account for removed and added work, migration, compatibility, failure and
  recovery. Name the evidence that would invalidate it. A read-only proposal
  can establish a deciding experiment without claiming a measured win.
- Revisit changed assumptions after an authorized redesign. In Trail of Bits'
  Liquidium report, the follow-up verified corrections to original findings
  and expressly left end-to-end review of the new architecture open.
  [Liquidium report and fix review](https://raw.githubusercontent.com/trailofbits/publications/master/reviews/2025-11-liquidium-cross-chain-lending-securityreview.pdf)

## Documents and research evidence

- Trace a conclusion through premises, definitions, data, calculations and
  citations. Inspect relevant tables, diagrams and appendices. Record unreadable
  portions and evidence gaps. Distinguish retrieval from close review.
- Check the original source and version when a checklist or dataset suggests a
  concern. Recalculate important claims independently and try a concrete
  counterexample. Keep an exact distinction between contradictory evidence,
  unsupported assumptions, scope limitations and alternative interpretations.

## Example objectives for one system

- **Correctness.** “Spend 20 minutes checking whether the event relay preserves
  event identity across reconnect and replay.” Trace the durable cursor through
  the consumer and compare event identities across the transition. A total
  event count cannot detect a missing event paired with a duplicate.
- **Incremental improvement.** “Spend 20 minutes finding replay work the relay
  can remove within its current protocol.” Locate repeated reads or decoding
  under the same replay workload. Keep ordering, freshness and recovery as
  acceptance criteria and account for any work moved to another stage.
- **Structural alternative.** “Spend 20 minutes assessing whether a different
  checkpoint ownership model can eliminate duplicate replay.” Compare which
  owner establishes durable progress in each design. Trace reconnect and
  failure between durable commit and acknowledgement in each design.

## Method sources

OpenZeppelin establishes scope against a frozen version, checks invariants and
reviews remediated issues against the fixes. Carry the version and the intended
properties through both review phases. [Audit readiness guide](https://www.openzeppelin.com/readiness-guide),
[audit process](https://www.openzeppelin.com/faqs/securing-onchain-systems)

Trail of Bits traces calls to see what establishes each caller assumption, then
checks both requirements that lack an implementation and implementation behavior
the specification omits. [Audit context building](https://github.com/trailofbits/skills/blob/main/plugins/audit-context-building/skills/audit-context-building/SKILL.md),
[spec-to-code compliance](https://github.com/trailofbits/skills/blob/main/plugins/spec-to-code-compliance/skills/spec-to-code-compliance/SKILL.md)
