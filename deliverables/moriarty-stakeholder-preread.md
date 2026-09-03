# Moriarty stakeholder pre-read

**Decision workshop — research cut 2026-09-02 — target length: 8 pages**

Moriarty is the proposed language name. Marlowe is the upstream Cardano
language and migration baseline. This pre-read summarizes the evidence and
motions in the full
[decision study](moriarty-decision-study-2026-09-02.md); it does not replace the
source inventory, experiments, or formal proof obligations.

## 1. Decision in one page

Proceed only with a bounded vertical slice: a strictly finite, typed
financial-agreement DSL, a small normative Core, and an ahead-of-time Compact
backend. Generated Compact must remain reviewable. The pinned Compact compiler
then emits TypeScript and ZKIR 3 artifacts. A translation manifest must bind
source, Core, generated Compact, compiler tuple, ZKIR, and ledger versions.

Do not put arbitrary Compact, ZKIR, minting, oracle, or external-contract calls
inside the semantic kernel. Do not claim that an agreement is safe merely
because its circuit proves. Separate semantic safety, privacy, authorization,
proof availability, ledger feasibility, participant liveness, and application
security.

This direction has S3 feasibility evidence: the repository's finite escrow
compiled with Compact compiler 0.34.100 and generated three ZKIR 3.0 circuits.
The current ZKIR mock compiler accepted all three. It does not yet have a full
proof-generation run, deployment, fee benchmark, independent audit, or
Marlowe-equivalence proof.

The product thesis is narrower than “non-programmers write contracts.” The
primary user is a Midnight DApp or fintech developer who needs private or
selectively disclosed financial terms with a bounded, auditable state machine.
Continue beyond a 90-day vertical slice only if two non-toy pilot teams commit
and the backend, proof, client-verification, and cost gates pass.

The first feasibility gate is smaller than the full suite: generate Compact for
a finite Marlowe-shaped fragment plus atomic two-token swap. Stop or choose
audited Compact libraries if the output needs an unbounded collection or an
unconstrained witness callback. The gate also requires an artifact/disclosure
manifest, a translation-validation certificate, and 1,000 coverage-guided
differential traces. Full proof generation and 10,000 materially distinct traces
per application remain release gates.

## 2. Terminology and system map

- **Marlowe V1:** the immutable Cardano financial-contract system used as the
  semantic and migration baseline.
- **Moriarty surface:** typed authoring language with modules, templates,
  token-indexed amounts, time types, visibility, and compile-time schedules.
- **Moriarty Core:** small finite normative state machine after elaboration.
- **Resource certificate:** checked bounds on lifetime, transitions, state,
  actions, payouts, continuations, generated circuits, and declared effects.
- **Compact:** Midnight's smart-contract language and the first Moriarty
  backend target.
- **ZKIR:** circuit intermediate representation produced by the Compact
  toolchain; not the normative Moriarty semantics.
- **Translation validation:** independent checks that the source, Core,
  generated Compact, and backend artifacts correspond for a particular build.
- **Capability manifest:** explicit external effects and trust assumptions;
  capabilities do not inherit Moriarty's Core guarantees.

```text
Moriarty source + dependency lock
              |
       typecheck/elaborate
              v
 normative finite Core + resource/visibility certificate
              |
    reference interpretation and translation validation
              v
 generated reviewable Compact + source map + artifact manifest
              |
        pinned compactc toolchain
              v
 TypeScript client  +  ZKIR/prover/verifier artifacts
              |                    |
     local intent verifier         v
              +---------> Midnight ledger/runtime
```

Trust boundary: the mathematical Core and valid resource certificate define
the language claim. The compiler, generated Compact, Compact compiler, proof
system, client, wallet, Runtime, ledger, oracle, credential, and dependency
supply chain each require separate correspondence or operational assurance.

## 3. Evidence that changes the decision

1. Marlowe V1 has a genuinely small finite algebra and useful termination,
   conservation, validity, and quiescence results, but the theorems have
   premises and do not prove end-to-end ledger or participant liveness.
2. Documentation, executable Haskell, Isabelle, Agda, deployed scripts, and
   services do not form one synchronized version. Isabelle lacks V1
   Merkleization, and interval comments expose an endpoint ambiguity.
3. The local Marlowe specification suite passed 59 tests, but its generator
   warned about weak non-`Close` and longer-input coverage.
4. Public Marlowe mainnet evidence for the known semantics credential is small:
   137 transactions in the observed range and 103 unspent outputs across known
   semantics hashes. Outputs are not necessarily active or valid contracts.
5. The official Marlowe V2 report is a design input, not evidence of a released
   V2. The examined active validator branch does not implement the proposed
   action-set language.
6. All 74 public `midnightntwrk` repositories and all three public
   `LFDT-Minokawa` repositories were pinned. Active Compact development moved to
   LFDT Minokawa even though the old GitHub repository is not API-archived.
7. Compact enforces static circuit structure and supports fixed bounded loops,
   but its maps, sets, and lists can grow without a Moriarty bound. Witness
   callbacks are unverified TypeScript inputs and must be constrained in the
   circuit.
8. The escrow prototype proves that an ahead-of-time route exists. Full ZKIR
   workspace integration tests still require external proof parameters through
   `MIDNIGHT_PP`; library tests alone passed 44 tests.
9. Structural V1 experiments show why bounds matter: a 1,000-period unrolled
   schedule produced 368,007 JSON bytes, 24,001 nodes, and depth 4,002; 1,000
   accounts produced 85,068 state bytes and 1,000 abstract refunds.
10. Public demand evidence is insufficient. Greater expressiveness is not a
    substitute for committed users or acceptable proof and ledger economics.
11. The authorized DeFiFormal checkout reproduces 72 corpus rows and 60
    construction specs, but all 60 verdicts are partial or inadmissible.
    Deterministic Jaccard classification recovers only 47/72 labels at 1-NN and
    50/72 at 3-NN. Moriarty therefore uses six economic families plus Prediction
    over mandatory facets for human navigation, a formal-behavior profile
    internally, and D01–D12/M4+ only as legacy benchmark views.
12. An independent tool-free advisory round with Grok, exact Fable 5.1, and
    GPT-5.6 Sol converged on a gated revision, generated Compact, and a real
    library-only fallback. A blinded second-round Fable call exhausted its
    bounded budget and is recorded as an abstention, not consensus.

## 4. Disputed assumptions

| Assumption | Strongest support | Strongest challenge | Resolution test |
|---|---|---|---|
| A new DSL is better than a Compact library | Stable semantics, reusable audits, domain diagnostics | Compiler and governance add a trusted layer | Three-contract comparative build and two pilot commitments |
| All contracts must remain finite | Enables termination and predeployment bounds | Some recurring products are naturally open-ended | Compare unrolling, rollover contracts, and certified runtime iteration |
| A small Core plus surface is sustainable | Keeps conveniences out of the trusted kernel | Elaboration correspondence is expensive | Prototype two independent interpreters and translation validation |
| Privacy is a product differentiator | Midnight can hide selected terms and witnesses | Proof cost, disclosure mistakes, and weak demand may dominate | Pilot measurements and visibility-policy user study |
| Generated Compact is the correct V2.0 backend | Reviewable and uses the supported compiler path | Version churn and specialization increase artifact burden | Pin/rebuild study across two toolchain releases |
| State compression solves scaling | Reduces some datum/state footprint | Moves cost into proofs, witnesses, storage, and availability | End-to-end state/proof/ledger benchmark |
| A universal validator is preferable | Shared audit and stable identity | AOT circuits may be cheaper and easier to specialize | AOT-versus-interpreter benchmark on three contracts |
| Runtime can be convenient and trust-minimized | Planner output can be checked locally | Coin selection, state discovery, and UI can still mislead | Malicious-planner rejection exercise |
| V1 can be “migrated” | Syntax and traces can inform translation | Immutable Cardano state cannot move to Midnight | Classified translator with human policy report; no value-move claim |
| Technical modernization is justified | Prototype and formal inheritance are promising | Adoption evidence is weak | Stop after 90 days without two written pilots |
| Twelve DeFi areas define the Core | They give a stable legacy benchmark roster | They mix products, instruments, mechanisms, and infrastructure | Use F1–F6/P plus facets for navigation, M5 behavior internally, and validate both seven family applications and 13 legacy patterns against all 72 rows |

## 5. Motions for decision

| # | Motion | Evidence for | Evidence against or condition |
|---|---|---|---|
| 1 | Moriarty remains strictly finite; every deployment has a lifetime and resource certificate. | Preserves Marlowe's strongest assurance and makes cost/liveness dependencies visible. | Excludes naturally perpetual agreements; rollover must be explicit. |
| 2 | Adopt a small normative Core plus typed surface language. | Improves authoring without expanding runtime semantics; supports domain types and source maps. | Adds elaboration and compiler-correspondence obligations. |
| 3 | V2.0 uses generated Compact, not a direct ZKIR backend. | Current end-to-end prototype works; generated source is reviewable. | Compact churn remains a dependency; direct ZKIR could later reduce layers. |
| 4 | V2.0 loops elaborate away; runtime bounded iteration is V2.1. | Keeps circuit and transition bounds simple. | Large schedules can explode; V2.1 needs measured proof/cost benefit. |
| 5 | V2.0 state is statically capped and closure uses deterministic bounded payout claims. | Avoids high-fan-out close and unbounded containers. | General compression and pull queues may improve scale but add availability/proof state. |
| 6 | V2.0 has ordered-any and atomic-all only. | Semantics are understandable and testable; covers the common branch and synchronized-action cases. | Threshold and partial collection may be required by real multi-party workflows. |
| 7 | External scripts and minting stay outside Core; later composition uses capability manifests. | Preserves the assurance boundary and makes effects auditable. | Restricts integration and can require more application transactions. |
| 8 | One mathematical Core specification is normative; implementations pass conformance and correspondence gates. | Prevents code/document/proof drift and enables independent checking. | Requires two funded formal maintainers and a practical executable reference. |
| 9 | V1 remains immutable and supported side by side; translation is classified and trace-checked. | Honest about backend differences and protects existing contracts. | Sustaining V1 LTS and two routes consumes staff. |
| 10 | Target privacy-sensitive DApp/fintech developers; require two non-toy pilots before full investment. | Matches the system's technical advantage and sets a falsifiable demand gate. | Narrows the audience and may show that a library is sufficient. |

Recommended votes: approve 1–10 as a single bounded program, with motion 10 as
the investment stop condition. Record dissent separately; do not average it
away in a score.

## 6. Decision matrix

Scores use 0–5 and the assignment weights. Burden is scored as feasibility to
maintain, so a higher number is better.

| Option | Assurance 20 | Cost 15 | Express. 12 | DX 10 | Migration 10 | Runtime 10 | Demand 8 | Maintain. 8 | Interop 7 | Weighted /5 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| V1 stabilization | 4.0 | 3.5 | 2.0 | 2.5 | 5.0 | 3.0 | 2.0 | 3.5 | 2.0 | 3.22 |
| Conservative V2 | 3.5 | 3.0 | 3.0 | 3.0 | 4.0 | 3.0 | 2.5 | 3.0 | 2.5 | 3.13 |
| Verified Core + typed surface | 4.5 | 3.5 | 4.0 | 4.5 | 3.0 | 4.0 | 3.5 | 2.5 | 3.5 | **3.84** |
| Composable protocol DSL | 3.0 | 2.5 | 4.5 | 3.5 | 2.5 | 3.5 | 3.5 | 2.0 | 5.0 | 3.32 |
| Portable/L2 semantics | 2.5 | 2.0 | 3.5 | 2.5 | 2.0 | 2.5 | 2.0 | 1.5 | 4.5 | 2.53 |

Uncertainty is approximately ±0.5 per raw score and greatest for demand,
proof/ledger cost, and maintenance. V1 stabilization becomes preferable if no
pilots commit, the Compact toolchain cannot offer a usable compatibility
window, or Core-to-Compact correspondence cannot be audited at acceptable
cost. A composable DSL becomes preferable only after capability semantics and
separate audit evidence are demonstrated.

## 7. Stakeholder questions

- **Formal methods:** Which proof environment has two committed maintainers?
  Which V2.0 obligations can finish in 12 months without admitted gaps?
- **Compact/ZKIR:** What toolchain compatibility window, proof-parameter
  lifecycle, and artifact reproducibility guarantee can Moriarty depend on?
- **Ledger and Runtime:** Which state, effects, credentials, fees, and
  continuations can a local client independently verify before signing?
- **Security and audit:** Is AOT specialization plus translation validation an
  auditable boundary? What compiler mutation evidence is sufficient?
- **Financial domain and product:** Which finite agreements need privacy and
  analyzability enough to choose Moriarty over handwritten Compact?
- **Application developers:** Do ordered-any and atomic-all cover the first
  pilots? Which diagnostic and source-trace failures block adoption?
- **V1 operators:** What LTS, registry, continuation, and explorer obligations
  remain for immutable contracts?
- **Governance and funding:** Who owns the normative Core, emergency response,
  release signing, independent audits, and the decision to stop?

## 8. Two-session agenda

### Session 1 — purpose and semantics (90 minutes)

1. Evidence, statuses, and contradictions — 10 min
2. Product thesis and strict-finiteness motion — 15 min
3. Core/surface and V2.0 boundary — 20 min
4. Action sets, timeouts, iteration, state, and closure — 20 min
5. Normative specification and proof obligations — 15 min
6. Votes on motions 1, 2, 4, 5, 6, and 8; record dissent — 10 min

### Session 2 — realization and investment (90 minutes)

1. Compact/ZKIR prototype and missing evidence — 15 min
2. Backend, Runtime, client verification, and capability boundary — 20 min
3. V1 coexistence and translator policy — 15 min
4. Target users, pilot commitments, budget, and stop conditions — 20 min
5. Governance, audit, release gates, and ownership — 10 min
6. Votes on motions 3, 7, 9, and 10; assign records and owners — 10 min

## 9. Unresolved-issue register

| ID | Issue | Decision needed by | Evidence or owner |
|---|---|---|---|
| U-01 | Normative prover and two-maintainer commitment | Day 30 | Formal lead |
| U-02 | Compact/ZKIR version support and proof parameters | Day 30 | Backend lead + upstream |
| U-03 | Client-verifiable credential and transaction intent model | Day 60 | Runtime/security leads |
| U-04 | AOT versus interpreter cost and proof comparison | Day 75 | Benchmark lead |
| U-05 | Exact atomic-all timeout and simultaneous-input rules | Day 45 | Language/formal leads |
| U-06 | Fixed payout cap and claim protocol | Day 60 | Ledger/formal leads |
| U-07 | V1 translation classes and first trace corpus | Day 75 | Migration lead |
| U-08 | Two written non-toy pilot commitments | Day 90 | Product lead |
| U-09 | Independent compiler/backend audit scope and quote | Day 90 | Security lead |
| U-10 | Whether the separate language beats a Compact library | Day 90 | Steering group |

## 10. Decision-record template

```yaml
id: MORIARTY-DR-000
motion: "Exact proposition decided"
date: 2026-09-02
participants: []
declared_conflicts: []
evidence_for: []
evidence_against: []
assumptions: []
decision: approved | rejected | deferred | experiment-required
dissent: []
status: proposed | accepted | superseded
owner: "Named accountable person"
deadline: YYYY-MM-DD
validation_metric: "Numeric or observable gate"
artifact_hashes: []
supersession_rule: "Condition and authority required to replace this record"
```

The immediate go/no-go is not “build all of Moriarty.” It is whether to fund a
90-day vertical slice with explicit stop conditions: three representative
contracts, two independent Core implementations, an AOT-versus-interpreter
benchmark, malicious-planner rejection, a proof-parameter plan, and two written
pilot commitments.
