# Independent language semantics recommendation

19 September 2026. Advisory proposal; no implementation, proof, build, deployment or canonical edit. I read all 3,027 lines of `evidence-packet.md` in chunks, the repository development skill and AGENTS, and ran the read-only status CLI. I have not independently revalidated the underlying source studies or September 11 native interfaces. Paths below identify supplied sources, not newly reproduced facts. This report's proposed semantic notation is not implemented syntax.

Moriarty should have one normative bounded stage relation, one owner-authenticated intention relation, and one history semantics, with explicit mappings from existing profiles. Financial products become libraries over this basis. The target remains native Midnight Halo2-derived PLONK/KZG on pinned ZKIRv3; Lean is not a dependency. The federated kernel proposes and coordinates evidence and execution. It cannot enlarge intention, supply missing correspondence, or authorize public deployment.

## 1. What the current evidence establishes

The supplied `ROADMAP.md` reports local source/5 lifecycle support, 125 expression cases, 104 lifecycle cases and six Unicode probes, plus scoped Preview loan payment/exit and swap trade/close. These are useful foundations. They do not establish a common general source-to-ZKIRv3 pipeline, mandatory native history proofs, private composition or complete conformance. The whole-language review identifies distinct Core representations and a legacy Compact mapper excluding authority/proof/settlement acceptance. Its reserve and ID-cap findings are scoped local semantics, not demonstrated deployed losses. [Sources: `ROADMAP.md`; `/home/charl/research/moriarty-whole-design-2026-09-19/REVIEW.md`, D1–D4.]

The status CLI reports SP01.6 loan-swap-subset, completed atomic-prepare/atomic-accept/rp01-mc02 stages, and outstanding native/general/composition/finance/release stages. It blocks actionful implementation admission on stale inputs and missing accounting/resource state, with no pending transactions. That operational block does not block this review or contradict the scoped historical financial receipts.

## 2. One small semantic basis

Use a total first-order Core with products/sums, nominal identifiers, bounded collections, explicit case analysis, bounded iteration, immutable pre-state and explicit state transitions. Reserve richer source abstractions for elaboration into that Core. Do not freeze a primitive count: the older estimate of roughly fifteen is not an evaluated completeness or proof-cost result. Parameterize contracts and libraries by checked interfaces and versions, not application-name tags.

A useful proposed judgment is:

`Stage_v(program, signedPolicy, authenticatedPrestate, candidate, evidence) -> Outcome`

where each outcome carries `status, poststate, completeEffects, remainingAuthority, residualDuties, cost, disclosures`. Status distinguishes precommit rejection from accepted success and accepted partial/failure outcomes. An unbalanced proposal is not an accepted partial workflow. A successful stage need not terminate the workflow.

The core's resource contexts must remain separate:

| Context | Required semantics | Consequence |
|---|---|---|
| Pure values and ordinary facts | Copyable, with typed numeric/domain meaning | Copying a fact cannot create spending permission |
| Spendable resources and consumption receipts | Explicit unique identity and ledger-enforced consumption rules | Reusing a proof never duplicates an asset or nonce |
| Authority | Affine, scoped and attenuable; unused authority may expire | No helper inherits unrelated ambient rights; grants have cumulative consumption state |
| Liabilities and operational duties | Persistent authenticated state; only explicit transition rules discharge, transfer or amend them | Weakening, local return or refund cannot erase debt |
| Reservations and recovery capacity | Named allocations with their own permitted consumers | Ordinary work cannot silently spend closure reserve |
| Evidence | Statement-indexed authenticity, domain, subject, version, freshness and disclosure | Authentic document bytes are distinct from truth of a document predicate |

“Persistent” liabilities here means non-erasure across state evolution, not unrestricted duplication of a linear token. A reusable proof that debt exists is distinct from the authoritative debt record and the right to modify it.

Type/effect judgments should track asset/domain, reads/writes, supply effects, authority requirements, liability evolution, external requests, disclosure and work. Static checks discharge structural facts; circuit checks discharge witness-dependent predicates; the ledger enforces currentness and unique consumption. Environmental claims remain explicit premises. No single check substitutes for the others.

Use exact bounded arithmetic, explicit failure, named units and directed rounding. Every integer-to-field representation needs range constraints that exclude modular wraparound masquerading as financial equality. For each authenticated domain/asset, token supply and holdings have their own conservation relation. For each liability, opening plus authorized creation/accrual minus explicit repayment/waiver/discharge equals closing. Waiver is not payment; default is not extinguishment. Gross debit and fee attribution survive netting even if final balances match. [Sources: product contract; MPLR-004/017/019/025; DeFiFormal composition and libraries reviews.]

DeFiFormal's Price(base,quote) is quote-per-base, whereas Moriarty Price<A,B,S> is base-per-quote. A port needs a typed orientation conversion with zero, reciprocal, scale, width and rounding conditions; renaming types or applying an approximate reciprocal is insufficient. Its rational accounting and trusted capability grants are reference models, not a native consent theorem. Its claims library and several financial families remain planned at the supplied pin.

## 3. Authenticated intention is a relation over all permitted outcomes

Keep three objects explicit: developer program contract, owner-authorized policy, and solver candidate. Define completion as membership in the intersection of their allowed behaviors, including the allowed policy holes. Composition of signed policies means intersection of constraints, not a union of privileges or an accidental Cartesian product of recipients/assets.

Canonical intent should bind program/semantic identity; precise assets or a substitution predicate; recipients; gross debit; fee denominations and caps; successful net outcome; liability and operational-duty limits; execution domains; evidence and disclosure; stage/phase authority; expiry and revocation; replay state; partial fulfillment; recovery and amendments. An owner may authorize multiple valid routes. No solver reputation or maintainer registry is needed to choose among them. A changed objective or widened grant needs applicable amendment authority.

For the MPLR-035 example, an 11-A total debit cap includes the at-most-1-A fee for a 10-A trade. It is not a 12-A envelope. A successful result must deliver at least 20 B to the signed recipient. Permitted failed/partial outcomes have their own signed bounds and residual state; applying the final success minimum to every pending stage would make staged workflows unusable. Conversely, labeling a delivery failure “partial” must not authorize an otherwise forbidden debit.

Define distinct rights to initiate, complete previously authorized work, reconcile evidence, recover, disclose and amend policy. Ordinary initiation expiry must not accidentally grant unrestricted recovery or prevent recording an authenticated late result. Reconciliation can update knowledge without itself authorizing a fresh transfer. Specify revocation order and which prior obligations survive it. [Sources: product contract; whole-language D2/D6; MPLR-008/018/019/028/031/035; APSS permission explanation.]

## 4. Histories: reconcile ledger induction and native recursive certificates

Adopt one abstract history compliance specification with multiple evidence realizations, rather than calling every mode “PCD” without qualification.

| History segment | Recommended evidence realization | Required boundary |
|---|---|---|
| Ordinary Midnight state lineage | Ledger acceptance induction over certified stage entry points | Constrained genesis, authenticated current head or absence read, unique consumption, immutable or consent-preserving verifier migration, complete phase effects |
| Off-ledger bounded segment | Native certificate proving the same versioned local relation from an authenticated anchor | Actual inner-proof interface, bound verifier/key, predecessor compatibility and final accumulator/decider verification |
| Portable recursive history | Native recursive compliance certificate, when feasible on the pinned interface | Legitimate base, well-founded sequence/DAG, full statement binding, independently verified final proof; no claim that contract-call proof bytes are valid inner proofs |
| Private split/join | Bounded multi-parent relation and explicit resource partition/recombination | Shared ancestry accounted once; no double spending, budget duplication or lost duties; witness availability and privacy separately justified |
| Foreign observation | Typed imported evidence under a named trust/finality policy | A foreign statement is not a Moriarty transition proof; its truth and completeness assumptions remain visible |

The older decision's ledger-head induction is valuable, but its categorical rejection of general DAG/per-transaction recursive history cannot remain governing while MC03/MC06 and the newer product retain recursive and split/join obligations. Supersede that prohibition with this scoped mode distinction. Do not mark MC03 complete with ledger induction alone, or rename generic aggregation historical PCD. Preserve its actual two-step native recursive proof requirement and MC06's separate-party split/join requirement unless the user explicitly changes them.

A common statement should bind semantics/program/property/target relation, canonical intent, execution domain, authenticated predecessor commitments, consumed resources, cumulative budgets, observations, complete effects and remaining duties. Public commitments may hide fields if equality and completeness are proved. A certificate establishes compliance; the ledger still decides whether resources can currently be consumed. Abstract DAG semantics need not force expensive arbitrary-arity circuits: use bounded joins, with the supported bound explicit.

The September 11 source notes say ledger contract-call proofs use a different transcript from proposed inner proofs and that recursion requires a deferred pairing check. Those are historical pinned observations, not current support claims. Refresh the target matrix before choosing certificate wiring. A host-verified Boolean or an unchecked accumulator cannot close native acceptance. If the deployed target lacks the interface, preserve an interface-blocked recursive milestone while continuing the correct ledger-anchored subset. [Sources: `wiki/decisions/pcd-midnight-native-architecture.md`, CLM-0946–0959; MPLR-027/029; Daml security-provability; MC03–MC06 in completion program.]

## 5. Correspondence and jets are one proof workstream

Maintain explicit representation relations for source-to-Core, Core-to-reference execution, reference-to-constraint fragment, constraint statement-to-ledger transcript, and accepted transcript-to-actual effects. Compact may be an intermediate, but emitted ZKIRv3 and actual ledger acceptance are the endpoint.

Require both directions within the admitted domain: source-valid behavior has a valid target realization, and every adversarial satisfying target witness denotes a permitted source behavior with the declared effects. The second is essential: testing honest witness generation alone leaves invalid witnesses unconstrained. A jet certificate binds reference expression, preconditions/ranges, host implementation, target constraints, versions, failure/effect behavior, evidence and cost model. Whole-program composition must discharge caller preconditions and frames; a jet cannot assume that a caller silently established a range.

Keep logical work, host performance and circuit resources separate. If logical work is observable through reserves or exhaustion, a faster jet must preserve it unless an explicit semantic version changes that policy. Certification can use compositional proofs or a sound artifact-level translation validator. The latter still needs its own soundness argument and complete checked artifact boundary. Neither strategy needs Lean.

PR17 is conditional Agda evidence for an older surface. It does not certify current ZKIRv3 wholesale. Derive or enforce WShape for every accepted adversarial witness and discharge concrete chip/implementation assumptions. Its sub-realizer/public-input conclusion must not be inflated into all-memory equivalence. [Sources: product contract; APSS certified-basis; MPLR-014/020–023.]

## 6. Stage bounds and lifecycle progress

Every accepted stage must have a checked concrete artifact bound, including late-bound modules. A potentially indefinite series of bounded stages is compatible with this design. Lifecycle termination, enabled recovery and eventual finality are separate propositions.

The 128-entry local arrays imply a finite episode unless successor state/rollover is designed. Choose either an explicitly finite episode with viable closure before exhaustion or authenticated rollover carrying replay protections, cumulative budgets, residual authority and duties. PCD compression does not automatically compact state or retain nonmembership evidence.

Recovery needs a dedicated authority and resource rule, or an admission rule proving a supported remedy remains enabled under stated assumptions. The local closure reserve example does not warrant making reserves available to every ordinary action. Even a proved enabled remedy cannot guarantee liquidity, witness availability, inclusion or foreign-chain finality. Unknown outcomes retain duties; reliable late success resolves its corresponding uncertainty. [Sources: whole-language D3/D4; MPLR-005/009/010/021; APSS applications/settlement.]

## 7. Boundary with the federated kernel

| Responsibility | Moriarty | Federated kernel / external party |
|---|---|---|
| Meaning and accepted effects | Language/Core, bounded policy relation, native constraint/ledger binding | Supplies candidates; cannot redefine meaning |
| Search | Public candidate interface and objective validation | AI routing, quotes, matching, optimization, service discovery |
| Authority | Owner policy, scoped delegation, consumed/reserved budgets, liability consent | Wallet/MPC signer implementation enforces agreed scope under explicit corruption assumptions |
| External knowledge | Typed statements, evidence policy and admissibility | Chain clients, attestations, documents, delivery observations and availability |
| Privacy | Declared observers/disclosures, supported information-flow claim | Witness transfer, MPC/TEE hosting and recovery under named assumptions |
| OWS/x402 | Exact signed effect interpretation and lifecycle/accounting contract | Wallet interoperability; scheme-specific negotiation, signing, payment and service transport |
| DeFiFormal | Source of semantic contracts and counterexamples | Formal reference library, not the deployed federated runtime or a Moriarty proof engine |

OWS possession must not expose an unrestricted key to an untrusted solver. x402 authorization, facilitator verification, submission, settlement and recipient service delivery are separate states. Concurrent pending purchases reserve aggregate budget durably. ZK validity, MPC threshold authorization and TEE attestation must refer to the same statement but retain different assumptions. [Sources: OWS/x402 explanation; MPLR-030–034.]

## 8. Proposed EARS refinements and coverage

These identifiers are report-local proposals, not new MPLR allocations. Preserve all existing MPLR entries and map their exact obligations in the canonical plan.

| Proposed EARS statement | MPLR mapping | Positive / hostile exit evidence |
|---|---|---|
| E-L1: When a supported program is submitted, validation shall depend on semantic/proof rules and owner policy without requiring project membership. | 015,016 | New independent program and second solver accepted; identical valid program without reviewer metadata accepted |
| E-L2: When a solver completes an intention, acceptance shall enforce the intersection of all authenticated constraints and preserve allowed alternative solutions. | 018,019,023,031,035 | Two allowed routes accepted; recipient, fee asset, extra fee and widened grant mutations rejected |
| E-L3: When a stage commits any effects, its authenticated frame shall preserve typed asset, liability, authority and duty evolution with cumulative budgets. | 004,009,011,017,025,029,032 | Concurrent fills within aggregate cap accepted; hidden reservation or debt deletion rejected |
| E-L4: When target fallibility retains effects, the successor shall record the exact authorized phase outcome, fees, authority consumption and residual duties. | 001,005,009,014,024 | Fallible failure retains only allowed effects; invented fee/nonce policy rejected |
| E-L5: When evidence resumes a conditional stage, acceptance shall check its bound subject, condition, origin, freshness and consumption rules. | 002,003,006,007,010,033,034 | Valid document predicate plus recipient consent releases; digest alone, wrong callback, replay and timeout-only refund reject |
| E-L6: When history evidence is accepted, it shall establish legitimate origins and compatible well-founded predecessors under the declared history mode. | 011,022,027,029 | Valid two-step certificate and join; fabricated base, arbitrary VK, cyclic bootstrap, duplicate resource and omitted duty reject |
| E-L7: When a primitive or module substitutes for another, correspondence shall preserve values, failures, effects and declared costs under discharged caller preconditions. | 014,020,021,022,026 | Certified alternate implementation accepted; carry-drop, ignored assertion and unbounded import rejected |
| E-L8: When a bounded instance approaches its state/work limit, its supported closure or successor rule shall preserve liabilities, replay state and signed cumulative bounds. | 005,009,010,021,028,029 | Final repayment/recovery at boundary succeeds where promised; rollover budget/nonce reset rejects |
| E-L9: Where private continuation is claimed, the toolchain shall state permitted observations and provide a supported witness-continuation route under named assumptions. | 013,029,030 | Independent-party continuation; forbidden disclosures and missing witness availability surfaced |
| E-L10: When tools report assurance or evolve policy/code, they shall bind the supported scope and preserve existing consent or require authorized amendment. | 012,018,022,028 | Simulated/accepted distinction; incompatible migration rejected |

Coverage: MPLR-001 through -035 all appear above. Full conformance must also retain their individual acceptance examples; mapping several IDs into one row does not merge away their distinct proof obligations.

## 9. One dependency roadmap

Consolidate the P/C/K plans as views of one dependency graph and one evidence register, rather than three new completion programs. Preserve historical identifiers and all objective MC/SP predicates.

| Phase | Deliverable and prerequisites | Measurable exit | Existing ownership to retain |
|---|---|---|---|
| R0 | Normative stage/intent/history relation; precise profile embeddings; refreshed native target/interface matrix | Every supported operation maps source/Core/constraint/ledger/effect; gaps explicit; no public administrative gate | P0, C0, K0; MC01/SP semantic and native feasibility objectives |
| R1 | Small certified basis and first complete signed single-stage slice, after R0 | New two-asset program through public source to ZKIRv3; arbitrary-witness soundness obligations discharged for admitted slice; real proof/effect controls | P1/P2, K1/K2; MC01/MC02/MC04 and existing SP financial integration |
| R2 | Mandatory four-claim acceptance and native history realization; early feasibility work runs alongside R1 | Exact ledger entry point enforces invariant, intent, transition, history; two retained recursive steps independently verify; tampering rejects | MC03/MC04/MC05, SP native/mandatory objectives; P2 |
| R3 | Staged conditional settlement, persistent continuation, recovery, bounded joins and private handoff | Recipient/document condition; partial fill and late-success race; reserve/ID boundary; two principals; real target effects and preserved duties | P4, C1–C3, K2/K3; MC06/SP composition objectives |
| R4 | Optional constrained wallet/service adapters; public financial libraries | OWS/x402 budget/retry/delivery cases and exact-byte effect binding; ACTUS/DeFi source contracts mapped through native proofs/effects | K4/K5, P6/C4; MC07/SP financial conformance |
| R5 | Independent developer release and reproducible retained evidence | Clean install without project metadata; novel program and two proposal sources; actual Preview evidence plus objective negatives; all retained acceptance rows reconciled | P7; MC08 and SP release objectives |

R1 must include minimal phase-aware failure and residual state, so R3 does not retroactively change the foundation. It need not implement every asynchronous construct before proving one stage. Trust/obligation reporting from P3 begins with R0; exact advisory checking follows useful encodings. P5 synthesis comes after replayable counterexamples and usable obligations, and never becomes a prerequisite for manual authorship.

MC02 stays uncertified integration until mandatory acceptance; a general-program demonstration does not erase its loan/swap obligations. MC03's recursion is not satisfied by a ledger-head theorem. MC06 has additional split/join and private-handoff predicates. Preserve MC07's 277 ACTUS fixtures/all present fields and 72 DeFi rows, separating semantic, proof, local-target and Preview denominators. A shared profile theorem needs explicit per-row instantiation; sample proofs are insufficient. MC08 retains broader unresolved release obligations. Existing SP IDs should remain exact aliases in the final traceability register; this packet lacks their full row definitions, so I do not invent an SP-by-SP closure map.

## 10. Alternatives and unsettled decisions

1. **Ledger-only history first:** smallest useful native slice and reasonable early delivery; insufficient for retained portable recursive/split-join goals. Recommend as a clearly scoped milestone, not the final contract.
2. **Recursive proof at every stage:** offers uniform portable evidence but adds cost, interface and witness-availability dependencies. Do not require it for every on-ledger step before measuring the native path; do preserve the needed certificate workstream.
3. **Certified compiler versus translation validation:** either can provide sound correspondence. Select based on complete supported coverage and a defensible trusted base, not easier green tests. A hybrid of certified primitive lowering and checked composed artifacts is a reasonable research candidate.
4. **Explicit finite episodes versus authenticated rollover:** both can preserve bounded stages. Decide with the final-payment-at-limit counterexample and persistent service requirements; neither is currently demonstrated.
5. **Small static effect summaries versus dependent refinements:** start with structural typing/effects and explicit dynamic proof obligations. Add exact refinements where they prevent real authoring mistakes. Do not make arbitrary theorem inference a usability prerequisite.
6. **Owner-signed exact actions versus delegated sessions:** exact actions are a valid first profile. Keep scoped delegation and separate expiry/recovery rights in the normative relation so future support does not reinterpret old signatures.
7. **Private joins:** cryptographic validity alone does not supply hidden predecessor state. Decide actual handoff, selective disclosure or joint proving arrangements and their recovery/availability assumptions before claiming support.

My recommendation is firm on semantic boundaries and delivery order, conditional on native interface feasibility. The packet supports this design direction; it does not establish current deployed recursion support, a certified jet basis, a complete compiler theorem or general financial ledger acceptance. Those remain measured/proved exits, not adviser votes.
