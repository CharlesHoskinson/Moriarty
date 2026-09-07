---
id: moriarty.architecture.decision
type: decision
title: Moriarty architecture decision
status: active
updated_at: 2026-09-07T16:50:22.642457+00:00
sources:
  - SRC-0070
  - SRC-0071
  - SRC-0072
  - SRC-0004
  - SRC-0005
  - SRC-0006
  - SRC-0007
  - SRC-0009
  - SRC-0015
  - SRC-0016
  - SRC-0017
  - SRC-0031
  - SRC-0033
created: 2026-09-02
updated: 2026-09-07
tags:
  - moriarty
  - research
---

# Moriarty architecture decision

Moriarty is a new bounded financial-agreement language, not a renamed copy of
Marlowe and not a general-purpose Compact dialect.

## Provisional decision

**CLM-0131.** The agreement-Core-plus-intent-envelope architecture is the
current candidate. It is not the prompt version 1.3 architecture freeze. S02
must compare all four required architectures against the S01 intent-safety
interface before selection. Sources: SRC-0031 at XML W1 and S02, and SRC-0033
under `Four execution representations`; created and reviewed 2026-09-03 through
2026-09-04; authority normative task input and reviewed experiment design;
scope current research candidate and S02 selection; evidence recommendation;
reproduction not applicable; confidence high; lifecycle status S2.

The recommended pipeline is:

```text
Moriarty source
  -> typed elaboration and finite resource/lifetime certificate
  -> canonical Moriarty Core
  -> readable generated Compact + correspondence manifest
  -> compactc
  -> ZKIR 3 + generated TypeScript + proving/verifier artifacts
  -> Midnight ledger and wallet
```

Direct source-to-ZKIR generation is deferred. ZKIR is a typed straight-line
circuit IR with guarded impacts and no source-level financial concepts. It is
evolving and ledger-coupled. Generating Compact preserves a reviewable backend
artifact and reuses the supported compiler, source maps, runtime bindings, and
ledger operations.

## Kernel boundary

Moriarty Core retains finite continuations, explicit actions and waits,
accounting, value conservation, explicit timeouts, typed warnings/errors, and
a decreasing structural measure. It adds visibility and trust information that
Marlowe V1 lacks. Every datum is classified as `public`, `private`, `committed`,
or `revealed`; every oracle or external effect has a named capability and
assurance boundary.

The surface language contains modules, named definitions, schedules,
token-indexed amounts, durations, records, packages, bounded compile-time
loops/folds, and templates. These features must elaborate away. The deployable
manifest records Core hash, source hash, compiler versions, maximum lifetime,
maximum transition count, maximum accounts/obligations, continuation roots,
visibility policy, external capabilities, and backend artifact hashes.

## Midnight realization

A long-lived agreement is a sequence of bounded one-transition proofs, not one
circuit that executes the whole lifetime. Sealed ledger fields bind the Core
hash, version, parties/capabilities, token policy, deadlines, and resource
limits. Mutable public state contains phase, sequence number, accounting or
obligation commitments, and bounded continuation roots.

Private witness callbacks are unverified TypeScript. A Moriarty-generated
circuit must constrain every witness result. Compact's `disclose()` authorizes
a flow past the compiler's privacy analysis; it does not by itself make that
flow semantically safe. Moriarty therefore generates `disclose()` only from an
explicit source visibility transition.

Timeouts are permissionless exported transitions guarded by ledger block-time
predicates. They do not execute autonomously. Liveness still depends on a
submitter, transaction construction, proof generation, data availability, and
ledger acceptance.

Moriarty Core forbids unbounded `Map`, `Set`, and `List` state. It uses fixed
vectors or fixed-depth Merkle commitments with certified cardinality. A
Merkleized continuation gains integrity from its root but still needs a
replicated availability protocol.

## Compilation choice

Ahead-of-time specialization is the V0/V1 backend. A universal interpreter
circuit would preserve a single audited evaluator, but its proving key and cost
would be determined by a worst-case AST, depth, account count, and action set.
It would also require proving bounded decoding and dispatch. Specialized
circuits are more practical now, provided the compiler is not silently trusted:
Moriarty must emit translation evidence and run an independent Core-versus-
generated-Compact trace validator.

Cross-contract calls, arbitrary minting, and general external Compact code stay
outside the initial Core. Current Compact implementation discovery uses
generated contract metadata, and cyclic call behavior is not a suitable basis
for a high-assurance kernel. A later composition layer can use signed capability
manifests and hash allowlists without claiming that external code inherits
Moriarty proofs.

## DeFi product architecture

The 12 DeFiFormal areas and the first report's M4+ mapping remain benchmark and
migration artifacts, not Core variants. Moriarty uses M2+M3 for human-facing
classification: F1 exchange, F2 credit, F3 derivatives, F4 consensus-position
claims, F5 tokenized off-chain claims, F6 delegated asset management, and P
prediction markets over mandatory facets. M5 formal behavior controls the
kernel and assurance model. Each deployed manifest records family, facets,
formal behavior, resource bounds, and external capability assumptions.

The strongest candidate additions to the Marlowe-derived kernel are a typed
party or authority sort and a bounded mandate. Conditional-token split/merge is
a separately gated prediction-market extension. A strategy level belongs in
the typed surface and package layer unless a bounded semantics and demand case
justify promotion. The family taxonomy must never add a constructor merely
because a market aggregator has a label for it.

This separation is essential for Compact/ZKIR assurance. A proof can establish
conservation, authorization, collateral thresholds, payout rules, and bounded
state transitions. It cannot establish an off-chain custodian's solvency, an
oracle publisher's honesty, a relayer network's liveness, legal enforceability,
or discretionary investment quality. Those dependencies remain named and
auditable without entering the trusted semantic kernel.


## CLM-0918: Midnight-centric language and target-first semantic challenges

The [user clarification](../raw/assignments/moriarty-report-reconciliation-2026-09-07.md) makes Midnight the implementation target. Compact, native proofs, private state and ledger acceptance constrain source and semantic design. Other-chain report examples are comparative financial behaviors. The [report reconciliation](../openspec/REPORT-RECONCILIATION-2026-09-07.md) requires early financial/intent and backend decisions within MC01-MC08. The atomic agreement profile is an initial subset; outcome-intent authoring, nominal-liability authority and temporal workflows require versioned extensions.

Metadata: SRC-0070, SRC-0071, SRC-0072; observed 2026-09-07; secondary reports plus normative user input and repository observation at `3eb0e0acf5b07a224ad876886e54837c82c84b86`; proposed language/plan scope S2; source review reproduced, implementation/proof acceptance not reproduced; confidence high for the recorded scope, unresolved for backend feasibility.
