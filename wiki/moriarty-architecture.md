---
id: moriarty.architecture.decision
type: decision
title: Moriarty architecture decision
status: active
updated_at: 2026-09-09T22:50:07Z
sources:
  - SRC-0110
  - SRC-0104
  - SRC-0100
  - SRC-0101
  - SRC-0102
  - SRC-0103
  - SRC-0098
  - SRC-0099
  - SRC-0094
  - SRC-0088
  - SRC-0093
  - SRC-0090
  - SRC-0081
  - SRC-0092
  - SRC-0091
  - SRC-0086
  - SRC-0079
  - SRC-0082
  - SRC-0097
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
updated: 2026-09-09
tags:
  - moriarty
  - research
---

# Moriarty architecture decision

## Source syntax and K direction

**CLM-0922.** The selected source extension is `.mori`; the successor specification uses EBNF, separate lexical rules, static judgments and K operational semantics. The [design dossier](../deliverables/defi-language-design-2026-09-07/README.md) recommends financial blocks with immutable locals and explicit pre/post state. Source: SRC-0097, current user direction; 2026-09-07; normative instruction plus design recommendation; S2; not reproduced; confidence high for direction and medium for surface usability. This is not an implemented successor profile.

**CLM-0923.** Resource identity preservation does not establish correct financial amounts or residual-debt preservation. The [semantic proposal](../deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md) separates these claims and separates acquisition/authorization expiry, due dates and execution bounds. Sources: SRC-0091, sections 3.2 and 6.2; SRC-0081, sections 3-4; SRC-0092, section 6. Source facts with scoped design inference; consulted 2026-09-07; S2; not reproduced; confidence high for distinctions.

The [surface comparison](../deliverables/defi-language-design-2026-09-07/SYNTAX-COMPARISON.md) is specified-only. Syntax studies inform choices but do not establish that braces, Lisp or layout is universally superior. The `.mori` migration preserves current source bytes; new comments, Boolean precedence, `pre`/`next`/`post` and type extensions require a new profile. K is selected, not implemented; the archived K definition covers ZKIR.

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

The ISO catalogue metadata is retained in SRC-0098. The Marlowe paper is reused through SRC-0099 and the original SRC-0040 collection; the original PDF and receipt remain unchanged.

## Source-grounded semantic tests — 2026-09-08

**CLM-0930.** The [four-paper language analysis](../deliverables/defi-taxonomy-papers-2026-09-08/DESIGN-IMPLICATIONS.md) recommends more precise claim/share, debt, clock, authority and composition semantics within the existing successor direction. Source support: SRC-0100 PDF pp. 10–19 §§3–4; SRC-0101 PDF pp. 3–8 §§III–V; SRC-0102 PDF pp. 3–6,12; SRC-0103 PDF pp. 4–11 §§3–6. Primary descriptive studies, version dates in the [source register](../deliverables/defi-taxonomy-papers-2026-09-08/sources.json); reviewed 2026-09-08; S2 inference, not implementation or proof; confidence medium. SP02 still owns complete lexical/EBNF/static rules; SP03 owns executable bounded Moriarty K semantics. Proposed TX02 makes a partial payment leave an observable obligation across source/Core/K/evaluator; later acceptance must bind complete effects and duties to Midnight. None of the four mandatory proof claims or existing coverage obligations is waived.

The [collateral-vault supplement](../deliverables/erc4626-vault-report-2026-09-08/DESIGN-IMPLICATIONS.md), CLM-0933 in [[wiki/security|security boundaries]], adds proposed tests for donation, preview authorization, valuation purpose, asynchronous claims and nested exposure. These feed SP02/SP03/SP08–SP11 without creating an Ethereum backend or accepting new syntax. The combined research now specifies eighteen illustrative tests, all unexecuted; complete language/K and actual Midnight evidence remain separate deliverables.

## Assets, claims and transformations — 2026-09-09

**CLM-0943.** The supplied security-token report separates financial lifecycle and entitlement from identity, transfer control and chain enforcement. Its useful contribution to Moriarty is an explicit account of how wrapping, pledging, liquidation, recovery and redemption change claims and retain obligations. Source: SRC-0110, report lines 317–385 and 539–555; report date 2026-09-09; secondary descriptive synthesis; reviewed 2026-09-09; source argument, S2; not reproduced; confidence medium. Its external citations are opaque and were not independently verified.

**CLM-0944.** Recommend bounded asset/claim/encumbrance records and operation-specific transformation rules, with reusable financial and policy profiles. Keep asset quantities distinct from share units, economic exposure, legal title and nominal debt. Normal and exceptional authority must be separate. This extends the existing token-indexed amount/residual-duty direction; it does not add one Core constructor per standard. Source: SRC-0110, lines 317–473; S2 design inference/recommendation, reviewed 2026-09-09; not implemented or reproduced; confidence medium. [Three approaches, proposed semantics and eight cases](../deliverables/security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md).

Place the specification in SP01, types/EBNF in SP02, and Felleisen–Hieb reductions/K in SP03. SP07 owns scheduled servicing; SP08 owns DeFi transformations and pending claims; SP06/SP09 bind history and ledger correspondence; SP10 handles private bounded composition; SP11 conformance and SP12 developer release complete the path. SP04 requalifies affected native components. SP05 supplies Docker then Midnight Preview evidence only after the profile is admitted. The [exact sprint crosswalk](../deliverables/security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md) preserves MC/RP gates, existing fixture counts and the shared-file ownership order. These are proposed refinements, not roadmap acceptance or new grammar. See [[wiki/security#Security-token policy paths — 2026-09-09|policy-path risks]].
