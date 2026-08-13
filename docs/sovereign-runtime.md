# Sovereign runtime target architecture

## Status

Everything in this document is **target architecture** unless explicitly marked
as current. The repository currently contains an operating policy, immutable
policy snapshots, five workflow skills, and policy-integrity automation. It does
not contain a runtime kernel, wire protocol, signer, capability broker, worker
node, commit service, receipt journal, or confidential-inference system.

This document defines boundaries for future design. It deliberately does not
publish an intent schema or receipt schema before implementation and threat-model
evidence exist.

## Design objective

The target runtime accepts an authenticated, bounded intent; grants only the
capabilities required for that intent; executes it on a replaceable worker;
independently verifies the result; commits only an accepted result; and records a
tamper-evident receipt. Models remain replaceable. Authority, evidence, policy,
and learning remain owner-controlled state.

```text
signed intent
    |
    v
trust root -> admission -> capability boundary -> replaceable worker
                    |                         |
                    v                         v
               rejection              candidate result
                                              |
                                              v
                                  verification/commit gate
                                     |                |
                                  reject           commit
                                     |                |
                                     +-------> receipt journal
                                                      |
                                                      v
                                         asynchronous projections
```

## Sovereign planes

### 1. Trust and authority plane - target

The trust root will authenticate principals, policy versions, worker identities,
and signed intents. Admission will bind an intent to one policy version, owner,
scope, invariants, oracle, capabilities, budgets, expiry, and recovery boundary.
Authentication will prove who requested work; it will not by itself authorize
the work.

The trust root must be small, auditable, offline-recoverable, and independent of
any model provider. Key rotation, revocation, clock assumptions, replay defense,
and emergency recovery require explicit protocols before this plane exists.

### 2. Capability plane - target

Admission will project the accepted contract into short-lived, least-privilege
capabilities. A worker will receive no ambient authority. Capabilities will name
resources and operations, carry budgets and expiry, and be independently
revocable. Read, write, deploy, publish, destructive, and production-data powers
will remain distinct.

The capability boundary must deny by default and produce evidence for grants,
use, expiry, revocation, and rejected escalation. A model request cannot widen a
capability; only a new admitted intent can do so.

### 3. Execution plane - target

A replaceable worker will execute the smallest complete slice against an input
snapshot. The worker may be a hosted model, self-hosted model, deterministic
program, or bounded composition. It will own no durable authority or canonical
learning state.

Execution contracts will bound time, compute, memory, I/O, network fanout,
retries, concurrency, output size, and retained state. Cancellation and lease
expiry will terminate authority as well as scheduling. Worker loss must not make
canonical state ambiguous.

### 4. Verification and commit plane - target

Worker output will be a candidate, never a committed result. An independent gate
will evaluate the declared oracle, primary invariant, failure paths, provenance,
budgets, and policy version. Verification strength will match consequence.

Commit will be atomic at the authoritative boundary or explicitly compensating.
The gate will reject stale inputs, expired authority, duplicated commits,
conflicting ownership, incomplete evidence, and budget breaches. Proposal,
verification, acceptance, and activation will be separate states.

### 5. Evidence and learning plane - target

An append-oriented receipt journal will record the admitted intent reference,
policy and artifact identities, capability projection, relevant evidence,
candidate and commit identities, cost, outcome class, recovery state, and
learning projections. The eventual wire format remains intentionally undefined.

Receipts will project asynchronously into searchable memory, blueprints,
anti-patterns, and skills. Projection failure must not alter the committed task
outcome. Retrieval will be bounded and provenance-preserving; prior learning will
be reverified against current inputs before reuse.

## Signed intent boundary - target

A signed intent will be immutable after admission. It will reference rather than
embed large artifacts and will bind their content identities. It must distinguish
the requested outcome from a proposed mechanism and distinguish task authority
from operational authority.

The design must resolve canonical encoding, signature domain separation, replay
scope, delegation, expiry, revocation, multi-party approval, and schema evolution
before any public wire contract is accepted. Until then, the policy task contract
is governance text, not a signed protocol.

## Asynchronous execution - target

Parallel work will be expressed as child intents with bounded inputs,
capabilities, budgets, deadlines, and output contracts. The parent will own
integration and the final invariant. Shared mutable ownership will require a
single commit authority or an explicit concurrency protocol.

Queues will be bounded. Backpressure will propagate to admission rather than
creating unbounded hidden work. Retries will be idempotent and budgeted; duplicate
delivery will be assumed. A task whose lease expires will lose commit authority
even if its worker later returns.

## Failure cells - target

The runtime will isolate failures into cells with explicit blast radius and
recovery ownership:

| Cell | Contained failure | Required response |
|---|---|---|
| Trust root | bad signature, replay, revoked identity | deny admission; preserve evidence |
| Admission | invalid contract, policy conflict, budget refusal | reject without capability grant |
| Capability broker | overgrant, expired or failed revocation | fail closed; quarantine affected leases |
| Worker | timeout, crash, malformed or adversarial output | discard candidate; revoke lease |
| Verifier | oracle unavailable, disagreement, stale evidence | no commit; classify inconclusive or blocked |
| Commit owner | conflict, partial write, uncertain activation | contain writes; recover from owner journal |
| Receipt journal | unavailable or integrity failure | stop commits requiring durable receipt |
| Learning projector | stale, duplicate, or poisoned projection | isolate projection; preserve task outcome |

No cell may turn uncertainty into plausible success. Cross-cell recovery will use
stable identities and idempotent operations rather than best-effort inference.

## Confidential inference boundary - target

Confidential inference may place a replaceable worker inside a hardware- or
software-attested boundary. Signed intent, encrypted inputs, model and runtime
identity, capability projection, output encryption, and verifier evidence must be
bound to the same attested execution claim.

Attestation will not prove model correctness, prevent authorized data exfiltration,
or replace output verification. Metadata leakage, prompt retention, side channels,
rollback attacks, operator access, key release, and attestation-root compromise
remain separate threats. No confidential-inference claim is current in this
repository.

## Scalability readiness

The target design will scale by partitioning ownership and evidence, not by
weakening invariants. Each growing dimension must be named: intents, artifacts,
workers, capabilities, receipts, projections, tenants, and bytes. Admission and
commit stay bounded per intent; journals partition by stable owner; projections
consume deltas; retrieval is indexed and limited; fanout and retries are explicit
budget terms.

Before implementation, representative `n`, `2n`, and `4n` workloads must define
expected time, space, I/O, and queue growth. Control-plane availability and
recovery objectives must be path-specific.

## Evolution gates

Each target component becomes current only after a separate accepted change
defines its owner, threat model, public contract, failure cells, recovery,
resource model, and direct verification. The first executable slice should be the
smallest boundary that can deny unauthorized work and prove that denial. A CLI,
distributed scheduler, or generalized orchestration layer is not a prerequisite.
