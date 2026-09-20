---
title: "Anoma design lessons for Moriarty"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, anoma, pl-theory]
---

# Anoma design lessons for Moriarty

## User clarification — native Midnight proofs

On 2026-09-19 the user confirmed that Moriarty must use Midnight's existing proof stack and does not need Lean. Lean is not a compiler, developer-tool, proving or deployment dependency. References to Juvix's Lean machinery describe comparative evidence only; they create no Moriarty Lean workstream.

Moriarty must encode signed intention, authority, financial conditions and residual duties into the constraints checked by Midnight's native stack through pinned ZKIRv3. The native cryptographic proof proves the encoded relation. Language and compiler correctness must ensure that this relation expresses the intended behavior. That responsibility remains without prescribing a separate theorem-prover backend. See the [product contract](../../../docs/MORIARTY-PRODUCT-CONTRACT.md) and existing [native architecture evidence](../../decisions/pcd-midnight-native-architecture.md).

Research synthesis, 19 September 2026. This document proposes design and experiments; it does not report an implemented Moriarty feature, a completed proof, an executed benchmark or a certified Anoma deployment.

Anoma is strong prior art for proof-checked intention/resource composition. Its separation of resource logic, structural compliance, balance, solver completion and ledger acceptance directly informs Moriarty. The strongest counterargument to building another system is that many desired conditional exchanges can already be expressed as application resource predicates and completed by solvers. The evidence does not establish the full requested conjunction: an openly deployable financial programming language whose executable code carries proof of authenticated formal intention through actual Midnight ZKIRv3, with explicit committed partial stages, conditional settlement and complete residual duties. That remaining conjunction is a research obligation, not a novelty proof or a claim that Anoma cannot express it.

Moriarty's target remains permissionless language deployment and solver participation. Any developer may submit a supported program and any solver may propose a satisfying completion. Application-specific counterparties, authority, private-data access and evidence policies can constrain a particular intention. A project membership list must not replace proof checking. AI is a witness-search and planning component; neither model confidence nor solver identity establishes authorization or correctness.

## Evidence and version boundary

This synthesis uses the [architecture study](../../../deliverables/anoma-study-2026-09-19/studies/architecture/REPORT.md) and its AR01–07 claims, the [security study](../../../deliverables/anoma-study-2026-09-19/studies/security/REPORT.md) and ARM-SEC-01–22 claims, and the [language study](../../../deliverables/anoma-study-2026-09-19/studies/language/REPORT.md) and JPL-01–12 claims. Each claim ledger contains source paths, exact hashes and locators. The [acquisition report](../../../deliverables/anoma-study-2026-09-19/ACQUISITION-REPORT.md) and [repository manifest](../../../deliverables/anoma-study-2026-09-19/repo-manifest.json) describe 19 pinned shallow repositories, 4,943 tracked files and 410 documentation/PDF files. The finite documentation closure records 1,473 requests: 1,295 successful responses and 178 HTTP 404s. Acquisition counts do not imply full reading coverage.

The selected ARM is `c72c1d3d05908ed63f2f6920c18de2691ce830b4`; EVM adapter is `3b1cb0299a097318281abff23e58a0a0c49f30db`; Juvix is `0ea34229b79b8ec266dd27e4fd3e199f8d5d7a3a`. These are independently acquired heads, not a tested release tuple. Rust ARM and Solidity adapter differ in compliance representation, nonce/message construction and verifier formats. Generic-call pins ARM rc.4 while the acquired ARM declares rc.5. The older Juvix Anoma library has its own API and stdlib dependency. Its `Proof := Resource` alias is not the current ARM's cryptographic proof type. Component compatibility must be demonstrated before an integration experiment. Historical audit findings and scope must remain attached to their audited versions.

## Four distinctions the language must preserve

**Incomplete candidate versus committed partial stage.** An unbalanced candidate may need another solver-provided action before it can pass transaction acceptance. It has not thereby committed half a financial workflow. The selected ARM rejects a delta witness at verification; candidate composition combines actions/witnesses, while accepted balance requires the appropriate proof. A committed partial workflow instead consists of individually accepted transitions. Its first stage may create an escrow or obligation resource; later stages settle, refund or compensate according to policy. Already accepted effects and unresolved duties remain explicit. Foreign effects cannot be erased by abandoning a candidate or reverting a later local transaction.

**Proof aggregation versus historical PCD.** Aggregating action/compliance proofs for one transaction changes proof packaging. Historical PCD requires a relation connecting legitimate base states and predecessors, with well-founded lineage, unique consumption, cumulative limits and remaining duties. Recursion is useful only if the recursively proved statement contains those properties. Neither arrangement proves external truth, foreign-chain finality or eventual completion without separate assumptions.

**Proof templates versus discharged obligations.** Juvix `--verify` emits Lean equivalence statements containing `sorry`; the library supplies meaningful IR semantics and verification machinery, but the generated obligations are not completed certificates. RISC0 guest execution receipts prove a different statement. Even a completed compiler-equivalence proof establishes preservation of the source meaning, not that the source fully expresses the owner's signed intention. Moriarty needs both policy satisfaction and exact source-to-ZKIRv3 correspondence, with closed and pinned dependencies.

**Resource balance versus residual-duty completeness.** A balanced transaction establishes a conservation relation under specified resource kinds and cryptographic assumptions. It does not establish fair market value, service delivery, absence of hidden fees or discharge of every obligation. Assets, liability amounts, refund entitlements, authority and operational duties need typed relations; permission units cannot be added to token quantities. Net balance cannot justify dropping gross obligations or an unresolved external leg.

## Concrete transfer matrix

| Anoma/Juvix evidence | Transfer to Moriarty | Required adaptation or limit | Existing research |
|---|---|---|---|
| Resource logic is checked against its declared circuit and public resource/action instance (ARM-SEC-07). | Bind each mandatory policy predicate to the artifact and exact accepted effects. | A trivial predicate can still prove the wrong policy; enforce signed intention completeness. | MPLR-018,022,023,029 |
| Compliance, logic and delta have separate judgments (ARM-SEC-03–13). | Separate structural validity, authorization, policy satisfaction and typed conservation. | None substitutes for external delivery or complete liabilities. | MPLR-008,017,019,025,026 |
| Solvers compose precommit candidates; delta witness is not accepted balance (security report). | Permit bounded solver-selected completions of signed holes and alternatives. | Preserve signed constraints, effects, disclosure and duties across all additions. | Proposed MPLR-035 below; complements 019,023,031 |
| Persistent roots/nullifiers need adapter state checks (ARM-SEC-14–18). | Bind proofs to authenticated ledger state and unique consumption. | A valid membership proof at an unrecognized root or reused spend remains invalid. | MPLR-011,024,027,029 |
| Generic-call logic binds forwarder and encoded call payload (ARM-SEC-19; AR05). | Bind exact external-call bytes and declared effect interpretation. | Correct bytes/acknowledgement do not prove intended economics or foreign completion. | MPLR-006,014,024,026,033 |
| Per-transaction aggregation binds journal encoding and compliance key (ARM-SEC-08–13; AR03). | Treat aggregated statements as typed evidence with explicit identity. | Add genuine predecessor/base/duty relations for history PCD. | MPLR-022,027,030 |
| SDK/prover handles private resource inputs; public fields remain (AR07; security privacy findings). | Declare solver/prover visibility, public statement and permitted disclosures. | ZK toward verifier does not hide plaintext from its prover; event suppression cannot erase calldata. | MPLR-013,029,030,031 |
| Juvix admits termination/positivity/coverage bypasses (JPL-01–04). | Define a proof-relevant admitted profile and bounded resource certificates. | Reject or separately certify escapes and imports; totality alone does not bound cost. | MPLR-020,021,023 |
| Juvix emits Nockma, Rust guest projects and Lean templates (JPL-05–09). | Separate executable artifact, execution proof and correctness proof. | Supply exact pinned ZKIRv3 lowering and discharge all obligations; no assumed backend portability. | MPLR-012,014,020,022 |
| Versioned circuit IDs, domains and migration logic (AR03,06; ARM-SEC-20). | Sign/bind a compatibility manifest and stage migration policy. | No independent-HEAD compatibility assumption; domains need unambiguous encoding. | MPLR-008,022,028 |
| Simulation reverts; real execution verifies proofs; adapter has emergency stop (ARM-SEC-14,22). | Label simulations distinctly and expose verifier/availability trust. | Simulation is not settlement; permissionless submission is distinct from governance availability. | MPLR-012,016,024,026 |

Conditional settlement means a submitted destination request waits for the configured combination of signatures, documents, proofs, recipient actions or other supported conditions. It can be unfunded; escrow is the funded/locked variant. This differs from waiting for an asynchronous callback. A staged application can combine both, but must authenticate outcomes and preserve branch-specific authority and recovery rules. Timeout alone establishes no universal nonexecution fact; reliable evidence of success remains success. Recovery after a local failure must account for external effects that may already exist.

## One distinct proposal: MPLR-035 — Constraint-preserving solver completion

**Proposed behavior.** When a solver completes, combines or routes a partially specified signed intention, acceptance shall establish that the completed candidate refines every applicable signed constraint and permitted choice, binds all introduced effects and disclosures, and preserves existing commitments and residual duties. A completion outside that authorization requires a separately authorized amendment. Candidates remain uncommitted until ordinary acceptance succeeds; solver completion does not authorize rollback of earlier committed stages.

This proposes a relation between an authenticated incomplete specification and a fully executable candidate. It complements MPLR-019's consent to introducing obligations, MPLR-023's enforcement of mandatory predicates and MPLR-031's bounded delegated authority. Those requirements do not by themselves define which holes a solver may fill, which alternatives it may select, or how two independently signed specifications may be combined without widening either one's solution set. The proposed item should be merged into an existing entry if subsequent register review already establishes this exact relation; the requested number is proposed only here, not allocated in the vault.

A possible research judgment is `Complete(P, C, E, S)`: candidate C completes authenticated policy P under evidence E and state S, with explicitly authorized substitutions, compatible combined policies, authenticated complete effects and residual duties. This is abstract notation, not implemented syntax or a theorem. The accepted completion may choose any solution admitted by P; a solver must not be forced to pick a single developer-approved route. “Improvement” is not unrestricted permission to modify the objective, fee asset, recipients or disclosure budget.

**Positive witness.** Two independently signed offers authorize exchange of exactly 10 units of asset A for at least 20 units of asset B, an aggregate fee of at most 1 unit of A charged within an 11-unit A debit cap, an exact receiving address, accepted settlement domain and approved evidence/disclosure policy. An unregistered solver supplies a compatible counterparty and permitted route. Both signatures' constraints, all fees and all created obligations are enforced; the candidate balances and commits. The fee cap is not added a second time to the total debit cap.

**Hostile witnesses.** Keep cryptographic proofs and balance valid while changing only one property: add a second fee leg; redirect the B output; change the fee denomination; substitute an unintended circuit; combine an offer whose required domain conflicts; disclose a prohibited witness field; or erase an unresolved obligation carried from an earlier accepted stage. Each mutation must fail for its specific policy/identity/completeness reason. An unbalanced uncommitted candidate must report incomplete/rejected without claiming a committed partial effect. A valid alternative route within the signed policy must remain admissible.

**Open theory choices.** Investigate refinement and effect systems for solver-controlled holes; monotone constraint combination; linear/resource semantics for commitments; authenticated policy intersection; quantitative budgets across concurrent completions; and privacy as a relational property. Prove soundness for arbitrary accepted witnesses, plus non-vacuity through valid completions. Bind that relation to actual Midnight ZKIRv3 acceptance. Do not confuse “solver found a candidate” with a guarantee that a candidate exists, is optimal or will eventually settle.

## Refine the existing register instead of expanding it

- MPLR-001–007 and010: label precommit incompleteness separately from committed stages; preserve pending evidence, known outcomes, partial fills and recovery obligations; keep settle/refund competition explicit.
- MPLR-008,009,011,016,019,031,032,034: enforce stage-specific authority, unambiguous domains, aggregate budgets, unique request/spend identity and consent across solver composition. Preserve open participation.
- MPLR-012,014,020–023,028: distinguish simulation, templates, checked proofs and executable artifacts; pin admitted language profiles, compiler/primitive/target identities and migration relations.
- MPLR-013,017,024–027,029,030,033: require complete authenticated effects and residual duties, legitimate recursive origins, compatible evidence statements and explicit external settlement/privacy assumptions. Balance, aggregation and acknowledgement each retain their limited meaning.
- MPLR-015,018: keep the general financial-language objective and inspectable signed contract central; an escrow demo is an experiment, not a reduction of the product to fixed transaction templates.

These are suggested refinements for review. No existing note or product requirement is changed by this synthesis.

## Implementation research priorities and first experiment

1. Define a small reference relation for signed policy, solver holes, complete typed effects, resource state, evidence and residual duties. Specify acceptance and transition outputs before choosing syntax. Distinguish proposal, accepted stage and terminal outcome.
2. Define the bounded proof-relevant language profile and its trust/assumption manifest. Specify authenticated primitives, admissible recursion, input/cost bounds, dependency closure, verifier keys and a precise ZKIRv3 representation relation. A proof scaffold is not an accepted build artifact.
3. Construct one coherent pinned Anoma comparison fixture, using dependency-compatible circuits/journals/adapter versions rather than all 19 HEADs. Check reproducible ELF/ImageID correspondence if using ARM proof production; do not treat development-mode proofs as production assurance.
4. Build the proposed MPLR-035 positive witness in the reference model, then mutate each hostile property independently. Add a conditional funded stage and a separate unfunded pending-request case. Each accepted stage must preserve authority, budgets and an explicit obligation set.
5. Add a delayed external result. First leave its outcome unknown, then supply authenticated success; race an otherwise authorized refund against that success. The accepted state must prevent incompatible terminal branches without discarding reliable knowledge. The experiment must state what evidence/finality assumptions justify each transition.
6. Lower the same bounded relation into pinned Midnight ZKIRv3 and compare accepted effects with the reference semantics. Check complete proof discharge and preserve a valid permissionless solver case. Only after this correspondence exists investigate PCD compression across stages; its statement must bind origins, predecessors, unique resources, cumulative budgets and remaining duties.

The experiment's result record should identify source policy and artifact hashes, all inputs and assumptions, positive outputs, each single-property mutation, expected/observed rejection reason, target effects and remaining duties. Tests cannot by themselves establish universal soundness. The required proof work and measured resource costs remain separate deliverables. No step above has been executed by this research synthesis.

## Reuse and confidence

Adopt the decomposed validity model, constraint-preserving solver completion, exact call binding and versioned proof-artifact identity as concepts. Evaluate code reuse component by component: acquired licenses include MIT, Apache-2.0 and GPL-3.0 material, and some roots lack a license file. The acquisition license inventory is evidence, not a legal compatibility determination. No RISC0 receipt, Juvix library or EVM adapter is claimed to execute unchanged on Midnight.

The evidence supports Anoma as a serious prior-art baseline and makes the proposed research more precise. It neither establishes that Moriarty has solved these obligations nor that no existing platform can satisfy them with additional application work.

Filing update: the proposal above is now recorded as [MPLR-035](../mplr/MPLR-035.md), still research-draft. [Evidence](reference.md) · [Index](index.md).

