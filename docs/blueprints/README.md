# Blueprint catalog

Harness tracks the complete source inventory; the separate blueprint repository
owns the documents. This catalog includes Markdown under `blueprints/` and
`patterns/`, recursively, except directory README files. Templates, source
indexes, and decision records are supporting material, not blueprint entries.
Distinct paths remain distinct even when titles match.

Use the installed `blueprint-methods` skill to retrieve at most three relevant
documents from `/home/ubuntu/blueprints`, or your checkout of the source repository.
Read selected documents completely before applying them. Transfer mechanisms and
failure boundaries; a blueprint provides a reasoning lens, not authority.

For missing compositions, shared control assumptions, or stalled searches, read
`blueprints/composition-coverage-before-optimization.md`. Its PK9 lesson separates
construction coverage, parameter recovery, candidate retention, and final
verification. Conditional recovery does not establish independent discovery.

Refresh after adding, changing, or removing source documents:
`python3 scripts/blueprint_catalog.py --source /home/ubuntu/blueprints --refresh`.
Audit current completeness and byte identity with the same command without
`--refresh`. A mismatch or unavailable source fails explicitly. Refresh writes
only this catalog and its manifest; it does not publish or modify source files.
CI validates the manifest and rendered catalog without the external checkout.
CI alone cannot establish current source completeness; run the source audit when
maintaining the library and before releasing a catalog update.

The [manifest](catalog.json) records paths, titles, SHA-256 values, and local Git
states at capture. Links pin committed content to the captured revision. Modified
and untracked entries require the local source checkout and have no content link.
Commit state is not remote publication evidence; refresh does not check hosting.
This inventory is not a quality or effectiveness assessment. Generated file:
change this script's header to revise guidance, then refresh.

Captured 2026-10-03 at source commit `a030fd4a2260798fc65cd91740de6a1d9d3c8998`: 45 documents.

| Document | Source path | Kind | Local source state |
|---|---|---|---|
| [Constraint Reframing](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-constraint-reframing.md) | `blueprints/aerospace-constraint-reframing.md` | blueprints | committed |
| [Deletion-First Engineering](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-deletion-first.md) | `blueprints/aerospace-deletion-first.md` | blueprints | committed |
| [Demand-Side Vertical Integration](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-demand-flywheel.md) | `blueprints/aerospace-demand-flywheel.md` | blueprints | committed |
| [Design Freeze & Incremental Evolution](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-design-freeze.md) | `blueprints/aerospace-design-freeze.md` | blueprints | committed |
| [Massively Parallel Engine-Out Architecture](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-engine-out.md) | `blueprints/aerospace-engine-out.md` | blueprints | committed |
| [Flight Envelope Protection](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-envelope-protection.md) | `blueprints/aerospace-envelope-protection.md` | blueprints | committed |
| [The Factory Is The Product](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-factory-is-product.md) | `blueprints/aerospace-factory-is-product.md` | blueprints | committed |
| [Fault Isolation & Containment Zones](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-fault-isolation.md) | `blueprints/aerospace-fault-isolation.md` | blueprints | committed |
| [Mission Assurance Review Lifecycle](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-mission-assurance.md) | `blueprints/aerospace-mission-assurance.md` | blueprints | committed |
| [Rapid Iterative Hardware Development](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-rapid-iteration.md) | `blueprints/aerospace-rapid-iteration.md` | blueprints | committed |
| [Reusability-First Architecture](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-reusability-first.md) | `blueprints/aerospace-reusability-first.md` | blueprints | committed |
| [Vertical Integration as Strategic Architecture](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/aerospace-vertical-integration.md) | `blueprints/aerospace-vertical-integration.md` | blueprints | committed |
| [Constraint as Forcing Function](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/apple-constraint-forcing.md) | `blueprints/apple-constraint-forcing.md` | blueprints | committed |
| [Superlinear Switching Costs Through Device Graph Integration](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/apple-device-graph.md) | `blueprints/apple-device-graph.md` | blueprints | committed |
| [The Discipline of Omission](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/apple-discipline-of-omission.md) | `blueprints/apple-discipline-of-omission.md` | blueprints | committed |
| Experience-Backward Product Design | `blueprints/apple-experience-backward-product-design.md` | blueprints | untracked |
| [Manufacturing Process as Design Language](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/apple-process-as-design.md) | `blueprints/apple-process-as-design.md` | blueprints | committed |
| The Taste Layer | `blueprints/apple-taste-layer.md` | blueprints | modified |
| [The Walled Garden as Quality Guarantee](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/apple-walled-garden.md) | `blueprints/apple-walled-garden.md` | blueprints | committed |
| Composition Coverage Before Optimization | `blueprints/composition-coverage-before-optimization.md` | blueprints | untracked |
| Atomic Inversion Brutalism | `blueprints/cross-industry-atomic-inversion-brutalism.md` | blueprints | untracked |
| [Cross-Industry Context Path Design](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/cross-industry-context-path-design.md) | `blueprints/cross-industry-context-path-design.md` | blueprints | committed |
| [Runtime Manifest Control Plane](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/engineering/01-runtime-manifest-control-plane.md) | `blueprints/engineering/01-runtime-manifest-control-plane.md` | blueprints | committed |
| [Page-Model GraphQL For Analytics Frontends](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/engineering/02-page-model-graphql.md) | `blueprints/engineering/02-page-model-graphql.md` | blueprints | committed |
| [Asynchronous Event-Driven Data Indexing Architecture](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/engineering/03-collector-processor-preaggregation.md) | `blueprints/engineering/03-collector-processor-preaggregation.md` | blueprints | committed |
| [Two-Pass Event Router With Scoped Projections](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/engineering/04-two-pass-event-router.md) | `blueprints/engineering/04-two-pass-event-router.md` | blueprints | committed |
| [Precomputed Snapshot Materialization](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/engineering/05-precomputed-snapshot-materialization.md) | `blueprints/engineering/05-precomputed-snapshot-materialization.md` | blueprints | committed |
| [Phase-Based Orchestration With Crash Recovery](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/engineering/06-phase-based-orchestration.md) | `blueprints/engineering/06-phase-based-orchestration.md` | blueprints | committed |
| [Deployment-Driven Bootstrap With Oracle Anchoring](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/engineering/07-deployment-driven-bootstrap.md) | `blueprints/engineering/07-deployment-driven-bootstrap.md` | blueprints | committed |
| [Accounting-Custody Separation In Smart Contracts](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/engineering/08-accounting-custody-separation.md) | `blueprints/engineering/08-accounting-custody-separation.md` | blueprints | committed |
| [Hub-Spoke Execution Engine](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/engineering/09-hub-spoke-execution-engine.md) | `blueprints/engineering/09-hub-spoke-execution-engine.md` | blueprints | committed |
| [Deny-By-Default API Edge](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/engineering/10-deny-by-default-api-edge.md) | `blueprints/engineering/10-deny-by-default-api-edge.md` | blueprints | committed |
| [Readiness and Observability Contract](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/engineering/11-readiness-observability-contract.md) | `blueprints/engineering/11-readiness-observability-contract.md` | blueprints | committed |
| [Asynchronous Event-Driven Data Indexing Architecture](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/indexer-processor-collector.md) | `blueprints/indexer-processor-collector.md` | blueprints | committed |
| [Burning Generality for Speed](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/nvidia-burn-generality.md) | `blueprints/nvidia-burn-generality.md` | blueprints | committed |
| [The Hidden Tax of Heterogeneity](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/nvidia-heterogeneity-tax.md) | `blueprints/nvidia-heterogeneity-tax.md` | blueprints | committed |
| [Hardware-Software Co-Design](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/nvidia-hw-sw-codesign.md) | `blueprints/nvidia-hw-sw-codesign.md` | blueprints | committed |
| [Information Topology](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/nvidia-information-topology.md) | `blueprints/nvidia-information-topology.md` | blueprints | committed |
| [Latency-Throughput Inversion](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/nvidia-latency-throughput.md) | `blueprints/nvidia-latency-throughput.md` | blueprints | committed |
| [The Memory Wall](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/nvidia-memory-wall.md) | `blueprints/nvidia-memory-wall.md` | blueprints | committed |
| [Paradigm Bet](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/nvidia-paradigm-bet.md) | `blueprints/nvidia-paradigm-bet.md` | blueprints | committed |
| [Platform Moat Through Developer Ecosystem](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/nvidia-platform-moat.md) | `blueprints/nvidia-platform-moat.md` | blueprints | committed |
| [Scaling Law as Demand Guarantee](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/blueprints/nvidia-scaling-law-demand.md) | `blueprints/nvidia-scaling-law-demand.md` | blueprints | committed |
| Repeated-Region Efficiency | `blueprints/openai-repeated-region-efficiency.md` | blueprints | untracked |
| [Context Path Prompting](https://github.com/yevhenx33/blueprints/blob/a030fd4a2260798fc65cd91740de6a1d9d3c8998/patterns/context-path-prompting.md) | `patterns/context-path-prompting.md` | patterns | committed |
