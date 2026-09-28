---
id: moriarty.architecture.decision
type: decision
title: Moriarty architecture decision
status: active
updated_at: 2026-09-28T20:15:00Z
sources:
  - SRC-0111
  - SRC-0112
  - SRC-0113
  - SRC-0109
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
updated: 2026-09-28
tags:
  - moriarty
  - research
---

# Moriarty architecture decision

## Source syntax and K direction

**CLM-0922.** The selected source extension is `.mori`; the successor specification uses EBNF, separate lexical rules, static judgments and K operational semantics. The [design dossier](../deliverables/defi-language-design-2026-09-07/README.md) recommends financial blocks with immutable locals and explicit pre/post state. Source: SRC-0097, current user direction; 2026-09-07; normative instruction plus design recommendation; S2; not reproduced; confidence high for direction and medium for surface usability. This is not an implemented successor profile.

**CLM-0923.** Resource identity preservation does not establish correct financial amounts or residual-debt preservation. The [semantic proposal](../deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md) separates these claims and separates acquisition/authorization expiry, due dates and execution bounds. Sources: SRC-0091, sections 3.2 and 6.2; SRC-0081, sections 3-4; SRC-0092, section 6. Source facts with scoped design inference; consulted 2026-09-07; S2; not reproduced; confidence high for distinctions.

The [surface comparison](../deliverables/defi-language-design-2026-09-07/SYNTAX-COMPARISON.md) is specified-only. Syntax studies inform choices but do not establish that braces, Lisp or layout is universally superior. The `.mori` migration preserves current source bytes; new comments, Boolean precedence, `pre`/`next`/`post` and type extensions require a new profile. That September 7 description was a historical planning status. The current [bounded Transfer/Repay K result](../deliverables/bounded-k-2026-09-09/README.md) executes the narrow funded-repayment projection; the [README small-step explanation](../README.md) describes its scope. Full successor K semantics and all-layer correspondence remain open; the archived ZKIR definition is separate.

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

**Superseded in part, 2026-09-11.** For `Release`, `JoinFrom`, `Migrate`, `ImportFrom` and `Reclaim` only, the [PCD integration amendment](attachments/historical-evidence/openspec/PCD-INTEGRATION-2026-09-11.md) allows claimed cross-contract calls between Moriarty contracts from Compact 0.33 on ledger 9, pending the user's confirmation (decision PD4). External non-Moriarty calls stay excluded.

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

## TypeScript-style language research and PL convergence

**CLM-0940.** The [research report](../deliverables/language-design-2026-09-09/REPORT.md), [recommended checklist](../deliverables/language-design-2026-09-09/FEATURE-CHECKLIST.md) and [PL convergence](../deliverables/language-design-2026-09-09/CONVERGENCE.md) recommend an authoritative `.mori` surface with TypeScript-style declarations. Static inert embedding is deferred; unrestricted TypeScript execution is excluded. Pure pre-state, single-write unreadable `next`, suffix `post`, explicit nominal public types, inferred conservative effect summaries and separately signed quantitative authority remain proposed requirements. Identified unchanged duties carry forward; domain rules and authorized resolution govern changes. No general linear calculus or universal static detection is established. SRC-0109; S2 recommendation, confidence medium; no participant study or full language acceptance.

**CLM-0941.** Current Elm commands/subscriptions differ from historical FRP. Unbounded queues and productive streams do not establish finite financial lifetime work. Unison handlers can resume multiple times, while immutable references do not establish authority or proof validity. Distinct source/Core/build/output/claim/execution identities and bounded dependencies are recommended. The [complete proposed payment data](../deliverables/language-design-2026-09-09/PAYMENT-FIXTURE.md) connects debt, custody, allocation, residual authority/work and prepared status without inventing admitted syntax. SRC-0109; scoped source facts plus S2 inference; no execution reproduced.

Grammar lessons: completeness claims name a profile. [The grammar review](../deliverables/grammar-pl-review-2026-09-09/README.md) records the repository lesson that ISO14977 meta-identifiers use letters/digits (camelCase replaces unsuitable underscores). Check grammar, lexer, parser, formatter and README agreement separately from typing and financial correspondence. The [current README](../README.md) describes the bounded small-step projection; PL-agent votes are expert critique, not developer-usability evidence.

## Assets, claims and transformations — 2026-09-09

**CLM-0943.** The supplied security-token report separates financial lifecycle and entitlement from identity, transfer control and chain enforcement. Its useful contribution to Moriarty is an explicit account of how wrapping, pledging, liquidation, recovery and redemption change claims and retain obligations. Source: SRC-0110, report lines 317–385 and 539–555; report date 2026-09-09; secondary descriptive synthesis; reviewed 2026-09-09; source argument, S2; not reproduced; confidence medium. Its external citations are opaque and were not independently verified.

**CLM-0944.** Recommend bounded asset/claim/encumbrance records and operation-specific transformation rules, with reusable financial and policy profiles. Keep asset quantities distinct from share units, economic exposure, legal title and nominal debt. Normal and exceptional authority must be separate. This extends the existing token-indexed amount/residual-duty direction; it does not add one Core constructor per standard. Source: SRC-0110, lines 317–473; S2 design inference/recommendation, reviewed 2026-09-09; not implemented or reproduced; confidence medium. [Three approaches, proposed semantics and eight cases](../deliverables/security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md).

Place the specification in SP01, types/EBNF in SP02, and Felleisen–Hieb reductions/K in SP03. SP07 owns scheduled servicing; SP08 owns DeFi transformations and pending claims; SP06/SP09 bind history and ledger correspondence; SP10 handles private bounded composition; SP11 conformance and SP12 developer release complete the path. SP04 requalifies affected native components. SP05 supplies Docker then Midnight Preview evidence only after the profile is admitted. The [exact sprint crosswalk](../deliverables/security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md) preserves MC/RP gates, existing fixture counts and the shared-file ownership order. These are proposed refinements, not roadmap acceptance or new grammar. See [[wiki/security#Security-token policy paths — 2026-09-09|policy-path risks]].

## Midnight-native PCD realization — 2026-09-11

The [[wiki/decisions/pcd-midnight-native-architecture|PCD decision]] places the Moriarty-to-Midnight seam at the operation key in `ContractState.operations`, over the ledger-built statement (CLM-0947).

**Generated contract shape.**

- **Entry points.** `Initialize`, `Step`, `Split`, `Join` and `Terminate`, with an optional principal-threshold `Pause`; cross-contract `Release`, `JoinFrom`, `Migrate`, `ImportFrom` and `Reclaim` from the Compact 0.33 toolchain.
- **State.** Public state holds per-head commitments; private state stays with entitled parties as encrypted openings.
- **Authority.** An empty maintenance committee with threshold at least 1 (CLM-0950).
- **Intent.** The signed intent digest binds network tag, contract, instance, head, revision, program digest, effect bounds, observation policy, certificate requirements, nonce and validity window.

**Compiler invariants.**

- Every head write reads the head or its absence.
- Read, write and native effects share one transcript section with no checkpoint between them (CLM-0949).

See the [report](attachments/historical-evidence/deliverables/pcd-midnight-native-2026-09-11/REPORT.md) §§10–12 and the [PCD roadmap](attachments/historical-evidence/openspec/PCD-ROADMAP-2026-09-11.md). S2 proposal; not implemented.

## Intent language owns conditional settlement and programmable escrow — 2026-09-28

**Owner decision, 2026-09-28.** Conditional settlement with programmable escrow belongs in the **intent language**: the conditions that govern funding, release, refund and partial progress are expressed and signed as part of the canonical intention, not left to program logic alone and not delegated to the optional Federated DeFi Kernel.

**Why this is a placement decision and not a restatement.** The architecture already describes conditional settlement ([design](../docs/MORIARTY-CONSOLIDATED-DESIGN.md) `:62` names "funded into programmable escrow" among nine workflow states, `:64` lists the condition materials, and [ROADMAP.md](../ROADMAP.md) `:24` makes two-asset programmable escrow a U3 exit) — but it never says which component carries a condition. The signed intention as frozen in the 2026-09-23 packet has twelve fields (`intentId`, `signer`, `consentPolicy`, `delegationPolicy`, `assetIdentities`, `recipients`, `grossDebitCap`, `feeCap`, `minNetOutcome`, `validity`, `replayPolicy`, `recoveryPolicy`): caps, sets and policies, and no condition, predicate, escrow or release construct. The decision assigns the missing carrier.

**What it closes.** The 2026-09-28 DeFi coverage study found several properties asserted with no component behind them; this decision supplies the component for a group of them: the conjunction and threshold rules the design assigns to the Moriarty column (`:30`) with no operator, core construct or library family; the release, refund and compensation branch of cross-domain workflow (`:72`); and the conditional-payoff shape that options, escrowed delivery and document-guarded settlement all need. See [[wiki/sessions/u0-unified-proposal-2026-09-28|the U0 study page]] and the [coverage report](../deliverables/u0-study-2026-09-28/DEFI-COVERAGE-REPORT.md).

**What it obliges, before U0-F freezes the contract.**

- **S6 must design for conditions.** The U0 plan's signed-intent abstract syntax (a Core `SignedIntent` type and a source `intent` EBNF fragment) is scheduled in the freeze phase. It must admit a condition grammar even though slice S0 uses none, or U3's escrow forces a grammar break and new signing semantics over an already-frozen intent.
- **One predicate language, not two.** The plan already proposes a small fixed predicate language for executable judgment clauses (linear integer arithmetic and set membership over relation paths). Intent conditions need a predicate language too. These should be the same language, or the product acquires two predicate surfaces with two evaluators, two certifications and two correspondence arguments.
- **The intent judgment grows.** "Intent refinement" must be defined over conditions, not only over the cap inequalities it covers today.
- **Observations become an intent-language dependency.** A condition over an observed value cannot be written while the canonical stage statement binds observations as issuer, domain, time and finality with no value and no observed-at time. The oracle gap is therefore on the critical path for escrow, not a separate category concern.
- **In-circuit authentication gets a richer object.** The proposed decision to authenticate the canonical intent digest in-circuit now covers a recursive predicate structure. U1 owns measuring that cost; the structure must be canonical and bounded before it can be digested.
- **Escrow needs custody and a release branch in the relation.** The stage relation carries effects and a failure policy; escrowed funds, their release condition and their refund path need a bound representation.

**Status.** Owner decision recorded; specified-only. No construct is implemented and no milestone has yet been assigned the grammar work. CLM-0978.

## Moriarty Intent Language MIL/1 — 2026-09-28

A [full design](../concepts/intent-language/README.md) for the intent language the 2026-09-28 owner decision requires, with a [category map](../concepts/intent-language/CATEGORY-MAP.md) over eight DeFi categories and twelve asset categories. Specified-only; no construct implemented and no milestone has accepted it.

**Shape.** Four layers: Intent (signed, digested, authenticated in-circuit) → Plan (a solver's candidate, checked not trusted) → Episode (bounded DAG of stages carrying joins, compensation and cumulative budgets) → Stage (one accepted transition on one domain). The Episode is the composite unit above the stage that the architecture review found missing.

**The decisions that carry it.**

- **One predicate language Φ** for intent conditions, judgment clauses, library preconditions and refinements. Total, bounded, no recursion, no division, each clause statically costed. The U0 plan's proposed judgment-clause language is a subset of it.
- **Time is a core type and domain-qualified.** A bare instant is a type error.
- **Observations carry a value, an observed-at instant and an evidence class** (`anchored`, `imported of Policy`, `attested of (issuer,k,n)`). The anchoring rule is a typing rule: an imported fact cannot be written where an anchored one is required. This is how cross-domain settlement stays honest without claiming global rollback.
- **Two new authority rights beside the design's six:** `issue` (supply authority) and `enforce` (a third party's bounded right against a defaulting counterparty). Four reviewers reached this independently; see [[wiki/sessions/u0-unified-proposal-2026-09-28|CLM-0982]].
- **Footprints** over `(domain, account, asset)`, obligations, supply, escrow and replay cells, declared per operation. Independence is set disjointness. This is the region notion of CLM-0980, and the one addition that cannot be retrofitted.
- **Escrow with `release`/`refund` predicates that must be exhaustive and disjoint**, checked at compile time. The nine workflow states become derived predicates rather than an enum, and an escrow with no reachable exit is rejected — which gives the design's recovery-viability obligation a component for the first time.
- **Holes with statically checked monotone completion.** A solver cannot widen a cap, add a recipient, extend a window or resurrect authority by filling a hole; it is a polarity check, not a review.
- **Conservation stated generally:** per `(domain, asset)`, Σ balance deltas = declared supply delta, zero exactly when nothing is minted. This supersedes the U0 proposal's E1, which states the zero case unconditionally ([[wiki/sessions/u0-unified-proposal-2026-09-28|CLM-0973]]).
- **Residue is a field**, so an inexpressible effect is reported rather than making the instance invalid.
- **Asset identity is structured** — domain, issuer, symbol, representation — giving wrapped, canonical, shielded and synthetic forms distinct identities with declared links.

**Asset architecture, in three sentences.** An asset's identity carries its domain, issuer and representation; its economy is one conservation equation per `(domain, asset)`; creating units always requires the `issue` right, and creating liabilities always requires consent from the party made liable. None of the three was previously expressible.

**Open and visible:** n-party clearing (batch auctions, order-book crossing), flash loans (excluded by keeping the stage atomic), a liquidation latency bound, competing-slash ordering under restaking, and the general form of policy state over reachability. The design also does not supply the family→constructor map, and adds constructors that make it more necessary.

**Placement.** The freeze work — Φ, the types, footprints, authority kinds, canonical form — is U0, as the signed-intent syntax task extended. Everything else is enabled by deciding the shape now and foreclosed by freezing the twelve-field intent instead. CLM-0986.

### MIL/1 nine-reviewer PL review — 2026-09-28

[Combined report](../concepts/intent-language/review/REVIEW-REPORT.md) · nine reviews in the same directory. Three lenses (type system, expressiveness, compilation) × three model families (Claude Opus 5.5, GPT-6 Sol, Grok 4.7).

**Unanimous: the shape is right, the document is not freezable.** Nine of nine endorse the four layers, outcome-first intents, evidence as a rejectable position, debt off the supply equation, per-domain conservation and footprints. Nine of nine refuse to hash-bind it at U0. CLM-0987.

**Three load-bearing claims are false, not incomplete** (CLM-0988):

- The **escrow meta-rule** rejects the design's own showcase and its own lending example, forbids the ordinary waiting state, does no work under the satisfiability reading, and forbids the late-success/refund race `ROADMAP.md:39` requires of U3. A deadline is a clock fact, not an exit and not a refund entitlement.
- **Monotone completion** is refuted three ways, including a filling that passes the polarity check while deleting the signer's early exit, and sign-dependent variance under `Signed<A>`. The stated implication is not a closed sentence.
- The **anchoring rule** is not a typing rule: `anchored` is a nullary tag for a domain-relative property, every eliminator erases the label, holes carry no evidence, and a private witness tag is forgeable unless bound in-circuit to an authenticated read.

**Four findings reach back into the U0 proposal itself** (CLM-0989): E1's zero-form conservation; T6's in-circuit signature primacy, which overreaches a ZKIR v3 instruction set with no Ed25519 verifier and no SHA-512; the U3 discriminator's cross-multiplied UInt256 inequalities, which do not fit the 255-bit BLS12-381 scalar against a 256-bit `u128` product; and the assumption that a compile-time solver escapes the trusted computing base, which it does not unless the circuit re-establishes the property.

**Decidability, precisely** (CLM-0990): Φ without products is difference logic or QF-LIA and solver-free; with products it is QF-BV and needs SMT. The repository's own measurement is the argument — one 128-bit division obligation cost bitvector `rlimit` 242,607,369 against 1,978 for unbounded `Int`, and 256-bit exceeded 600 seconds (`openspec/changes/aeon-refinement-integration/design.md:27-31`).

**The five open items were wrong** (CLM-0991). n-party clearing is real but its proposed carrier is refuted — every order writes the pool, so a join over disjoint footprints is the maximally wrong mechanism. Flash loans are a product cut, not a consequence of atomicity. Liquidation latency is liveness, not a language property. Competing-slash ordering is forced into the core. Policy state belongs in the core now. **Concentrated liquidity is missing from the list and is not reachable by any incremental patch to Φ.**

**Disposition.** MIL/1 is superseded pending MIL/2, to be cut against the review rather than patched in place. Five choices in that revision are the owner's: Φ₀ or Φ₁ at U0; flash loans as a restated cut or a bounded intra-stage trace; joins ledger-linked or in-circuit; whether to reserve a multi-signer stage arity now; and whether concentrated liquidity is in scope.

### MIL/2 — 2026-09-28

[Design](../concepts/intent-language/DESIGN-MIL2.md). Cut against the nine-reviewer review rather than patched, because the repairs interact. Specified-only. CLM-0992.

**Repairs.** Escrow becomes a transition relation with a legitimate pending state, per-witness in-circuit exclusivity via a constrained branch bit and a ledger tombstone, a required `priority` field for the late-success/refund race, and `deadline none` legal again; the authoring check is `after(deadline) ⇒ release_when ∨ refund_when` over Φ₀, decided by difference logic with `unknown` failing closed. Monotone completion is replaced by **acceptance refinement** — `Accepted(I[σ]) ⊆ Accepted(I)`, checked per filling — with polarity demoted to an authoring lint whose variance table excludes signed factors. Evidence becomes a **type index with a source-set effect** (`Γ ⊢ t : T ! S`), `anchored` a typing side condition bound in-circuit to an authenticated ledger read rather than a Φ proposition. Φ gains formation judgments, total evaluation into `Value | Reject`, atomic negation, `pre`/`post`, and `totalAssets` with both share-conversion directions. The footprint cell vocabulary is completed and footprints are derived and checked against declaration. The MIL/1 fallback is deleted outright.

**Five owner decisions taken, each with its reversal cost recorded.** (1) **Φ₀ frozen at U0, Φ₁ deferred** — literal-coefficient cross-multiplication stays in Φ₀, so the U3 discriminator needs no solver, while pool arithmetic and variable products wait; the repository's own bitvector measurement (rlimit 242,607,369 against 1,978; 256-bit over 600 s) decides it. (2) **Flash loans excluded as a restated product cut**, not an entailment of atomicity, with a `trace` extension point reserved. (3) **Joins ledger-linked through U3**; in-circuit parent verification stays ZR09 in U4. (4) **Multi-signer stage arity reserved now** with the U0 profile admitting one signer — the one thing reviewers agreed cannot be retrofitted. (5) **Concentrated liquidity declared out of scope**, since it needs bounded iteration and a new sort.

**Discipline change.** Every formal claim now carries `[checked]`, `[obligation]` or `[deferred]`, and §17 collects the six obligations that must be proved before the digest is hash-bound. MIL/1's failure was stating theorems it had not checked; nothing in MIL/2 is claimed proved.

**Corrections to U0 carried forward:** E1 as the general conservation law with S0 as its zero specialization; T6's in-circuit signature primacy withdrawn, since ZKIR v3 has no Ed25519 verifier and no SHA-512; the U3 cross-multiplied UInt256 limits subject to the limb rule; and SMT kept out of U0 because a compile-time solver does not escape the trusted base unless the circuit re-establishes the property.
