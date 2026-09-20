# Anoma-informed Moriarty research plan

Status: research design, 19 September 2026. This plan reconciles the architecture, security and language studies with existing MPLR-001 through MPLR-034. It allocates no IDs and claims no completed formalization, test, proof, compiler port or deployment. The target remains Midnight ZKIR v3; Anoma's RISC0/Nockma targets are comparative evidence.

The central experiment is an authenticated conditional exchange with a ten-unit total spending budget, including at most one unit of fees. An unregistered solver may propose a solution. The owner may submit an unfunded request, later fund it, permit a partial fill, and receive a delivery or recovery outcome under the signed policy. Every accepted stage must account for actual assets, liabilities, delegated authority and outstanding duties. Precommit incomplete intents and committed pending workflows are different types.

## Reconciliation: prefer refinements, not duplicate requirements

| Study candidate | Existing MPLRs | Refinement to investigate; allocation disposition |
|---|---|---|
| Precommit composition versus committed stages | 001,002,003,004,005,007,010,011,024 | Candidate delta is not accepted stage state. Add discriminating examples and types; no new ID needed. |
| Exact image, journal, kind-table and adapter binding | 014,020,022,026,028,030 | Extend the acceptance tuple and implementation correspondence relation; no new generic binding requirement. |
| Application logic / compliance / balance separation | 017,023,025,027,029 | Require complete policy/effect coverage in addition to conservation; do not equate net balance with gross duties. Refinement. |
| Ephemeral resource admissibility | 015,017,023,027,029 | Define supported ephemeral roles and prove each admissible transition; zero-value padding must remain usable. Could be a named sub-obligation, but insufficient basis for another top-level ID. |
| Admission profile and dependency closure | 014,015,020,021,022 | Bound language fragment, flags, imported primitives and resource cost; retain permissionless authors. Refinement. |
| Closed proof obligations | 012,014,020,022,023,027 | Generated Lean text, successful compilation and execution receipts have separate assurance statuses. Refinement. |
| Stage authority and canonical signed policy | 008,016,018,019,022,031 | Resource-key knowledge and proof generation do not automatically establish owner's consent or recipient obligation consent. Refinement. |
| Privacy observers and witness availability | 013,028,029,030 | Explicit verifier/solver/prover/operator/calldata/event disclosure and continuation recovery. Refinement. |
| External calls and finality | 003,005,006,010,024,026,030,033 | Return bytes, payment finality and service delivery are separate evidence judgments. Refinement. |
| Cumulative budgets and logical obligation identity | 004,009,011,017,032,034 | Count pending reservations, fees, effects and retry identity across stages; do not reset caps per proof. Refinement. |

All 34 existing IDs are represented. The security/language candidates above fit existing requirements. The final synthesis additionally proposes constraint-preserving solver completion (tentative MPLR-035): an explicit relation from authenticated incomplete specifications, through authorized hole filling and policy intersection, to a complete candidate. This is a defensible refinement-specific obligation beyond merely checking the final predicates, but not an allocated ID. E2/E4/E7 must compare independently signed policies and preserve permitted alternatives without widening either authorization. A valid alternative route is the positive witness; unauthorized fee, recipient, disclosure or domain substitution is hostile. Register maintainers may give this relation its own entry or attach it explicitly to MPLR-022/023/031; do not silently count both as independent guarantees.

## Candidate formal contract

First define a version tuple `V = (source language/profile, dependency closure, compiler, certified primitive set, target semantic version, artifact hash, circuit/image identity, public-input encoding, verifier identity/configuration, resource-kind interpretation, adapter/network identity, migration policy)`. Not every backend uses every field, but omitted fields must be explicitly inapplicable rather than silently unbound. Anoma's current independent repository heads are evidence for this obligation, not a compatible tuple to copy.

Define canonical authenticated intention `I`, state `S`, stage `k`, branch `b`, complete effect frame `E`, authority `A`, remaining obligations `O`, evidence `D` and resource bound `B`. A candidate judgment is:

`Accept(V,I,k,b,S,A,O,D,w) -> Exists S',A',O',E. PermittedStep(I,k,b,S,A,O,D,S',A',O',E) and TargetEffects(V,w)=E and Bound(V,w,B)`.

This notation is a research sketch. The actual theorem must quantify over **every accepting adversarial witness** and state its cryptographic, ledger, hardware and external-observation assumptions. Proving that the intended witness generator produces good witnesses is insufficient. A separate existence theorem or constructed witness establishes non-vacuity; general completeness needs its own quantified statement.

For recursion, define an authenticated base with legitimate resource origin and declared initial duties. Each step must bind compatible predecessor statements, consume prior authority/state once, account for concurrent reservations, and preserve or discharge every duty. A terminal judgment must distinguish delivery-complete from refund-complete or compensated branches. Batch proof aggregation alone proves none of this history relation. Safety is conditional on named assumptions; eventual completion requires explicit liveness assumptions and fairness or recovery rules.

## Ordered experiments and acceptance evidence

These experiments are proposed. Each must save a pinned manifest, inputs, positive and hostile artifacts, exact accepted/rejected judgments, checker output and scope. A rejection caused by malformed serialization is not evidence that an economic constraint was enforced.

### E1 — Compatible artifact and statement tuple

Build one selected compatible ARM/adapter pair as a reference experiment; separately specify the corresponding Midnight tuple. Record source revisions, compiler/flags, ELF and image IDs, journal schemas, verification keys, adapter bytecode/configuration and accepted kind-table hash. Compare independently produced manifests rather than trusting filenames.

Positive: a valid transfer's exact image, canonical instance and table match the adapter. Hostile variants: swap image while retaining a genuine proof; change journal encoding; substitute a table mapping two asset kinds to one chosen point; attach raw actions to an unrelated aggregation; use an old adapter format with a new circuit. Require rejection at the intended binding boundary. Preserve positive proof generation for the matched tuple. Do not assume current unrelated HEADs interoperate. MPLR014/020/022/026/028/030.

### E2 — Policy completeness and predicate enforcement

Model a two-party exchange whose owner authorizes the exact asset/issuer, destination, minimum receipt, total spend and fee envelope. Enumerate every effect in the authenticated frame and every mandatory predicate for the selected branch.

Positive: a solver supplies a valid nontrivial exchange with a permitted fee. Hostile variants: always-true resource logic; compute `signature_ok=false` and discard it; omit a fee debit from the effect frame; replace a declared circuit with a trivial passing circuit; satisfy an empty list of listed predicates while omitting an affected resource. Distinguish the legacy helper's limitation from a claim about the ARM ledger verifier. Prove acceptance implies all mandatory predicates and complete effects; separately construct the valid witness. MPLR017/018/019/023/025/026.

### E3 — Persistent and ephemeral existence

Define persistent resource existence, nonmembership of nullifiers, and each permitted ephemeral role. Include chain-root history in state assumptions.

Positive: persistent spend at an accepted historical root with an unused nullifier; zero-quantity ephemeral padding without prior creation; a purpose-bound ephemeral constraint carrier. Hostile variants: consistent membership path to an unknown root; previously spent nullifier; duplicated consumption across actions; ephemeral resource used to bypass application mint authority; wrong derived output nonce/index. Require correct-role acceptance and reject unauthorized supply/duty changes. An unconditional rule demanding prior creation for all ephemeral resources would fail the positive witness. MPLR015/017/027/029.

### E4 — Candidate composition and committed prefix

Define `Candidate` with unresolved balance and `CommittedStage` with ledger-accepted effects and residual duties. Construct complementary candidates and compose before settlement.

Positive: two candidate deltas cancel, the full policy holds, and the final transaction is accepted. Hostile: submit the first candidate using only a delta witness and label it settled. Separately create a balanced escrow stage with a persistent obligation resource; it must be accepted as a pending workflow without pretending the business objective is complete. Require no conversion from unresolved candidate to committed state without acceptance evidence. MPLR001/002/003/004/024/027.

### E5 — Partial fill, concurrency, fees and residual duties

Start with ten units authorized in total, of which fees may consume at most one. A first fill consumes four units of principal plus0.4 fee and leaves5.6 units total budget, fee headroom0.6 and the unfilled business obligation. Split pending reservations between two independent solvers.

Positive: reserve3.3 and2.3 for later work including fees, then settle within all aggregate limits and retain any unfilled quantity. Hostile: each solver independently spends5.6; reset fee headroom after recursion; net a fee away against an unrelated credit; lose a pending reservation during retry. Require exact cumulative accounting and one logical spend identity. Closing financial budget does not itself close delivery duties. MPLR004/009/011/017/025/032/034.

### E6 — Delivery, timeout and compensation races

Use a funded pending stage with distinct authenticated payment, service-result and recipient-delivery evidence. Specify settle/refund precedence and late-result handling before execution.

Positive: delivery evidence consumes the delivery obligation and releases funds once; alternatively a permitted timeout refunds assets still controlled and records the refund branch. Hostile: refund after known successful delivery; treat absent evidence as proof no payment occurred; a late successful external result creates a second payout; cancel one join branch and erase its outstanding duty; call a new compensating transfer a rollback. Require a state transition table and model-checked finite race exploration before stronger inductive proof claims. MPLR003/005/006/007/010/033/034.

### E7 — Authority, domain separation and obligation consent

Bind solver capability to stage, effect class, expiry/revocation epoch and canonical policy. Encode context with fixed widths or explicit lengths; include network, adapter, artifact and role as required.

Positive: an unregistered solver with a valid attenuated capability completes an allowed stage; a recipient receives positive value without being assigned an unconsented duty. Hostile: nullifier-secret possession used as authority for an unrelated owner policy; cross-network signature replay; ambiguous `(a,bc)` versus `(ab,c)` domain/message encoding; revived expired capability; transfer to a recipient used to impose a liability. Require rejection independent of solver registration. MPLR008/015/016/018/019/022/031.

### E8 — Admission and target correspondence

Compare finite fuel, structural recursion and checked ranking certificates as alternatives; do not pick a theory solely because Juvix uses it. Bound proof-relevant inputs, lists, imported functions, arithmetic and concrete target cost.

Positive: a terminating nonstructural algorithm is accepted with a checked ranking/resource certificate, or an explicitly bounded formulation under the chosen profile. Hostile: `terminating` escape for an infinite loop; disabled positivity/coverage imported under a checked entry point; late-bound code admitted after the cost certificate; generated Lean containing `sorry`; dependency drifting from a mutable branch. Distinguish parsing, compilation, execution proof and discharged correspondence. Prove source-policy satisfaction and source-to-ZKIRv3 refinement separately, with failure and guaranteed/fallible effects preserved. MPLR012/014/020/021/022/023.

### E9 — Observer-specific privacy and continuation availability

Create an observation matrix for owner, counterparty, solver, prover, relay, verifier, ledger reader and TEE operator. Cover resource witnesses, policy/circuit identities, tags, app-data, calldata, events, timing, outputs and recovery keys. State intended leakage and required witness availability.

Positive: an authorized successor obtains sufficient private evidence to continue, while the declared public observer cannot distinguish two executions beyond permitted leakage. Hostile: mark plaintext calldata `Immediately` and claim deletion; suppress an event but expose the same secret to a solver; rotate keys and strand the continuation; combine individually safe disclosures into a policy leak. Use relational tests/model reasoning first; a ZK proof alone does not hide data from its prover. MPLR013/028/029/030.

### E10 — External behavior and federated evidence

Construct an adapter that binds the exact forwarder, call bytes, expected response and declared finality rule. Give each ZK proof, MPC authorization and TEE attestation a typed statement and assumption set.

Positive: consistent evidence for one stage's exact allowed effect plus separately authenticated final delivery. Hostile: a forwarder returns expected acknowledgement without delivery; a TEE runs the wrong policy correctly; proof for X combined with threshold signature for Y; stale attestation epoch; silent weaker fallback after a service outage. Preserve local transaction rollback only for effects actually in its domain. Prove that federation preserves the specified predicates under explicit assumptions, not that hardware or thresholds establish external truth. MPLR006/024/026/030/033.

### E11 — Upgrade and recursive-history closure

Migrate a live partially fulfilled obligation between compatible artifact tuples. The migration relation must preserve beneficiary, duty amount, consumed evidence, privacy, recovery and aggregate resource bounds, or carry applicable amendment authorization.

Positive: authorized refinement preserves all live obligations and permits the remaining valid continuation. Hostile: interface-compatible update changes controller; fabricated approved genesis; cyclic predecessor proof; drop one pending resource; carry a valid proof across an incompatible circuit/schema epoch. Require well-founded lineage and authenticated initial state; keep batch aggregation distinct from this theorem. MPLR002/004/008/014/022/027/028.

## Evidence gates and completion criteria

Proceed from executable reference semantics and falsifiable examples to finite-state race models, then symbolic/inductive obligations and exact target correspondence. A chosen model's bounded exploration is not unbounded proof. A prover accepting real cryptographic proofs is a separate gate from dev-mode tests. Avoid claiming a historical audit covers excluded PCD files or current HEADs.

A useful first milestone completes E1–E4 with real positive/negative checks and explicit remaining assumptions. A staged-settlement milestone adds E5–E7 and E11, including residual duties and concurrent reservations. A target-assurance milestone discharges E8 and the actual Midnight correspondence. Privacy/federation claims require E9–E10 under named observers and assumptions. These are separable milestones; passing an earlier one cannot be advertised as completing the later ones.

Deliverables per experiment: exact tuple manifest; canonical policy and statement schema; positive witness; one-property hostile mutations; test/model/proof artifact and output; assurance label; unresolved obligations; and mapping to existing MPLRs. Publish no “certified kernel” conclusion until the advertised judgment is discharged for the concrete target and admitted arbitrary witnesses. Preserve permissionless program submission throughout.

## Evidence inputs

`studies/security/{REPORT.md,claims.json}`, `studies/language/{REPORT.md,claims.json, MPLR-candidates.md}`, `studies/architecture/{REPORT.md,claims.json}`. The plan's input manifest records hashes of these artifacts and the read-only MPLR register. This is comparative research grounded in inspected source; no source system is claimed to implement every proposed experiment or Moriarty requirement.
