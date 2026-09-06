# PCD report integration and revised implementation order

Date: 2026-09-06. Status: S2 design revision. Authority: user instruction to
read, graph and apply the supplied report, followed by the clarification that
Midnight supplies Halo2 and recursion. This changes the approved mock sprint;
it does not restart A4/A5 or establish a working proof backend.

## Source and evidence boundary

SRC-0043 is the complete [user report](../../raw/reports/pcd-2026-09-06/PCD.md),
preserved byte-for-byte with its [receipt](../../raw/reports/pcd-2026-09-06/receipt.json).
SHA-256: `f7e837fbafb68a21d34fd0ea7a46b7db0d82624bd6d2ac95d2d89353eab4d6ff`.
It is a secondary research synthesis with opaque citation markers and no
resolvable source URLs. Its benchmarks, security-advisory details and maturity
judgments are reported claims, not reproduced findings. The
[Graphify graph](../../deliverables/pcd-report-integration-2026-09-06/graphify-out/graph.html)
maps the report; EXTRACTED means explicit in that source, not independently true.

Two consequential research directions were checked against primary abstracts:
[holography accumulation](https://eprint.iacr.org/2026/538) explicitly addresses
private prover-state transfer in recursive proving; [TCT v2](https://arxiv.org/abs/2408.06478v2)
describes reusable semantic theorems checked for transactions. These motivate
handoff and certificate-reuse investigations; neither establishes performance
or compatibility for Moriarty. Acquisition limitations are retained in
[SRC-0044's receipt](../../raw/pcd-supplement-2026-09-06/receipt.json).

SRC-0045 is the [pinned Midnight review](2026-09-06-midnight-native-recursion.md):
`midnight-zk` at `695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7` implements
in-circuit verification and IVC. The inspected ledger uses proofs/circuits
0.7/6.2 while aggregation uses 0.8/7.0; IVC changes are marked Unreleased.
No native proof or ledger compatibility run was performed.

## Decisions from the report

| Report proposal | Disposition for Moriarty | Concrete consequence |
| --- | --- | --- |
| Typed Proof-Carrying Transaction Claim Envelope, report lines 151–343 | Adopt as the proposed interface structure | Name each predicate, evidence class, mandatory status, trusted verifier, dependencies and applicability. A proof badge alone is insufficient. |
| Execution validity differs from intended effects, lines 764–818 | Adopt as a first acceptance falsifier | A correctly executed swap paying the wrong recipient must fail the independently signed intent/effect relation. Cover fee, asset, approval and undeclared-write mutations too. |
| Reusable contract/path certificates | Adapt | Retain all-domain contract properties. Reuse a path certificate only after checking its exact program/specification, guard, path and assumptions; it cannot stand in for a lifetime theorem. |
| Recursion only where history matters | Adapt to the user's stronger requirement | Moriarty's accepted state changes require certified transition and predecessor-history compliance, including constrained genesis. Simple subclaims may use ordinary checks inside that required relation. PCD stays a core acceptance function. |
| Optional execution proofs and ordinary-execution fallback | Restrict to acceleration | Fallback must implement the identical acceptance predicate and still check mandatory intent, contract and PCD evidence. Missing required history proof never becomes ordinary acceptance. |
| Stateless cross-party proving, witness availability | Adopt as an early feasibility gate | Test Alice → Bob and a branch/join with independent private witnesses. Record every artifact the successor needs, its confidentiality and recovery owner. Succinct public proofs alone do not establish safe handoff. |
| Compact forbids recursive calls, lines 370 and 857 | Correct the inference boundary | Language-level recursion and recursive proof verification are separate. Prioritize Midnight-native Halo2/recursion; determine the actual application/ledger adapter instead of inferring absence from Compact syntax. |
| Generic transfer first, other ledgers and long calendar roadmap | Do not substitute for target scope | ACTUS LAM dues/settlement and DeFi swap remain the first shared semantic examples. Keep all 18 ACTUS types, 277 fixtures and 72 DeFi rows visible. Other ledgers are later adapters. |
| Broad zkVM, post-quantum and consensus-envelope investigations | Defer | No generic VM, new consensus rule or new cryptosystem in this sprint. Avoid a benchmark campaign without a concrete target decision. |

## Acceptance semantics added to the design

The deployment policy fixes a finite permitted set of claim types,
specifications and verifier/key versions. A transaction cannot choose an
arbitrary verifier that declares itself authoritative. Unknown mandatory claims,
missing evidence and unresolved dependencies reject. Optional evidence can be
ignored only under the signed policy; it grants no safety guarantee when ignored.

Use two commitments to prevent a proof/signature cycle:

1. Define `TxCore` without enclosing intent/claim/envelope roots, signatures or
   evidence. Define `ClaimSpec` without its own ID, enclosing intent/manifest
   roots, signatures or evidence hashes/bytes. It may bind `H(TxCore)` and
   earlier dependency IDs in the bounded acyclic graph. Claim specs include
   claim modes, dependencies, predicate/program identities, intended effects,
   state references, budgets, validity intervals and permitted verifier profiles.
   Compute claim IDs, then the ordered manifest root. Define
   `intentDigest = H(domain, IntentCore, H(TxCore), manifestRoot)`; IntentCore
   contains no enclosing commitment or signature. The user signs that digest.
2. `BoundClaim` attaches the now-defined intent digest and claim ID to the public
   evidence statement. It is not rehashed into ClaimSpec or manifestRoot.
   Proofs bind that signed statement and check its authorization. The final
   envelope additionally commits to ordered evidence descriptors and sidecars.
   Sidecar stripping cannot remove a mandatory obligation from the signed root.
   Final ledger signatures follow the target's encoding rules without being
   included in their own signed preimage.

Dependency descriptors form a bounded acyclic graph. Claim IDs derive from
proof-independent descriptors, avoiding mutual evidence-hash commitments.
Genesis is an explicit base case, never an absent-proof exemption. Every join
checks predecessor identity/output position, program/policy compatibility,
joint effects and consumed budgets. Proof aggregation does not establish these
properties unless the outer relation checks them. All recursive assumptions or
accumulator obligations must be discharged at final verification.

Keep five separate conclusions: execution correctness; this execution's
property; all executions within a certified domain; applicability to current
canonical state; and external truth. Each claim declares its assumptions.
Oracle attestations, finality, uniqueness and data availability stay explicit
boundaries. Proving declared state membership does not prove that the declared
read/write footprint is complete.

Recursive verification does not make the DSL Turing-complete. Agreement
evaluation still has finite types, acyclic calls, bounded folds, finite epochs
and a decreasing lifecycle measure. Add explicit claim count, dependency depth,
proof bytes, sidecar bytes, verifier work and predecessor fan-in to the profile.
Cryptographic recursion checks prior evidence under that finite profile; it is
not an unbounded user-program loop. Termination and invariant correctness remain
different obligations, and finite does not mean cheap to verify exhaustively.

## Revised sprint sequence and exit evidence

The later [intents report amendment](2026-09-06-intents-report-integration.md)
adds R2b between the exact-plan prototype and general authority acceptance.
R2b separates outcome IntentIR from PlanIR and receipts, binds aggregate gross
authority, net goals, validity/replay and residual obligations, and requires
compiler/adapter refinement. Read-only R3 interface work can proceed alongside
it; a narrower native financial proof cannot stand in for these predicates.
The [R3 specification](../../experiments/moriarty-native-ivc-r3/README.md) fixes
the first financial episode and proposed stopping/resource contract; it has not run.

These are dependency gates, with no automatic retry loop or arbitrary calendar
deadlines. The mock is executable illustration; the remaining proof work is
specified-only until its command, pins and resource ceiling are recorded.

| Gate | Work | Required evidence |
| --- | --- | --- |
| R0: report intake | Preserve report, graph concepts, reconcile decisions and Midnight recursion boundary | Source receipt, queryable graph, revised controlling plans and contradiction disposition. |
| R1: developer mock | Finish the already approved local loan/swap workspace and show proposed typed claims | Imported versus calculated results, demo evidence, real verification unavailable; browser/model checks and explicit unimplemented predicates. |
| R2: shared semantic slice and claim codec | One loan due/settlement pair and one AMM swap use the same bounded transition/effect types; define canonical signed claim manifest | Exact encoding vectors and an independent effect checker reject ledger-valid/intent-invalid recipient, asset, fee and undeclared-effect cases. All-field ACTUS comparison remains a separate gate. |
| R3: native proof boundary | Pin Midnight Halo2/recursion components, setup/curve/transcript, public-input order and target verifier entry point; prove one local step and one authenticated predecessor extension | First distinguish native Rust IVC success from ledger acceptance: both require separate receipts. Real positive proof at the actual target boundary; missing/altered proof, wrong key/domain/spec/state/effect and unsatisfied recursive dependency reject. No mock compiler or unchecked accumulator counts. |
| R4: private multi-party PCD | Extend to bounded Alice → Bob, then A → B,C → D split/join with separate private witnesses | Successful successor proving without predecessor secrets; correct branch provenance and policy composition; forged genesis, duplicate input and mixed-policy join reject; missing handoff data reports unavailable. Ledger conflict tested separately. |
| R5: properties and target expansion | Connect checked contract certificates to acceptance, then expand ACTUS/DeFi packages | Named assumptions/guards, independent semantics, all required result fields; no source-gap exclusions or unsupported lifetime claims. |
| R6: optional acceleration | Measure a genuine repeated-execution workload only if needed | Same validity predicate under fallback; witness, proving, aggregation, verification, bytes and stale-state retries measured separately. No claimed savings from proof size alone. |

R2's claim definitions and the read-only R3 interface study can proceed together.
R3/R4 are isolated cryptographic and ledger-format compatibility experiments,
not full Moriarty acceptance. Until R5 connects and checks every mandatory
contract certificate, real Moriarty verification stays unavailable; a successful
test ledger call cannot be exported as a certified Moriarty transaction.
Do not select an external recursive stack merely because its examples are easy
to run. A native adapter failure identifies the missing interface; it does not
justify removing mandatory history evidence. Holography/folding comparisons are
contingencies for an observed native limitation, not new parallel projects.

## Developer-facing consequences

The mock and subsequent SDK distinguish `IntentEffects`, `ContractInvariant`,
`TransitionValidity` and `HistoryCompliance`. A required private-authorization
or dependency claim is included when the package demands it. Each shows its
predicate scope, evidence kind, verifier profile, freshness and status.
`SimulatedEvidence` never becomes cryptographic or formal evidence.

The next functional API adds `describeClaims`, `checkIntentEffects` and
`verifyRequiredClaims` around the existing prepare/sign/prove/verify/submit
workflow. These are proposed interfaces, not implemented exports today. Policy
and verifier selection precede signing; remote witness disclosure stays explicit.

Open decisions: concrete numeric profiles; canonical codec vectors; the checked
contract-certificate kernel; exact Midnight recursive verifier integration;
private handoff artifacts; and complete effect projection. The report sharpens
these obligations rather than closing them.
