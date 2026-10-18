# doctorwatson<UTXO>: Cardano delivery and EARS closure

Advisory design, 2026-10-17. Machine slug: `doctorwatson-utxo`. No implementation, product acceptance, execution admission or security approval is supplied. The broader Preprod goal remains paused. This design can inform a later authorized implementation; it does not start that implementation.

## Recommendation and alternatives

Use the proposed local evidence reader, but make the first product delivery an actual artifact-consumption increment. EU-NAT-028 is a bounded bootstrap for honest reporting, not an indefinite delivery track. Incorporate EU-CON-003 failure classification in that bootstrap rather than treating it as another successive factory project. Afterward, choose the first missing behavior on the Rust → Cardano artifact → validated claim output → consumption/composition → settlement path. If that path is blocked, expose the exact dependency and pursue independently useful source work; never replace the blocked outcome with another reporting closure.

| Alternative | Tradeoff | Disposition |
|---|---|---|
| Small UTXO reader plus three consumer increments below | Fastest route to useful consumer behavior; requires explicit new-profile refinements and honest partial results | Recommend |
| Generalize Sherlock's full checker first | Shared maintenance may eventually help, but interpreter-specific paths and 35/25/68 membership delay UTXO behavior | Defer until both concrete readers expose a demonstrated common seam |
| Reuse legacy /5, Preview or a fixed C2 validator as current compiler closure | Superficially reduces missing rows, but changes the predicate being accepted | Reject; retain historical evidence under its original identity |

Sherlock's reader explicitly validates membership/currentness and does not decide semantic acceptance [S1]. Its implementation loop already says to stop factory expansion unless a real closure attempt exposes a defect [S2]. No scheduler, provider polling, Foreman runtime, hosted CI or new public developer permissions are needed.

## User-ordered build, EARS update and remaining-roadmap audit

The latest steering fixes the sequence: finish the design; build the bounded `doctorwatson<UTXO>` adaptation; then reconcile **all** retained EARS against actual implementation and evidence; then audit the remaining roadmap through its full completion criteria. The first build should provide membership/scenario retention, profile/target fields, evidence/currentness separation, per-row gaps and a selected gate. Do not wait for a general framework, native result or three product increments before performing the requested EARS reconciliation and roadmap audit.

The post-build EARS update is an exhaustive status/traceability reconciliation, not bulk normative rewriting. For each original clause and every named scenario, record consumed source, observed or unrun evidence, applicability/target, owner acceptance, first unmet predicate and dependency. Preserve unchanged clause text, pre-scenario trace/acceptance references and design-interface bindings; proposed additive route amendments require their own review. A shortened coverage-index sentence is not the full normative clause. The roadmap audit must return a coverage manifest for all retained IDs/scenarios/families and U/M/MC/SP/G exits; separate current accepted, stale, partial, absent, unsupported, unassessed and execution-blocked findings; enumerate unresolved contradictions and uninspected source families; produce a dependency-ordered set of real consumer gaps through final acceptance, without turning counts into completion percentages. An uninspected family makes the audit incomplete, not green.

The three increments below are the first proposed product slices for that audit's resulting backlog. Completing the audit is a deliverable distinct from completing the remaining product roadmap. The paused Preprod goal and zero native authority remain unchanged; the new steering does not turn an audit row into execution permission.

## What the sources establish

**Repository observations.** The five UTXO packages retain 119 requirements and 238 scenarios, with 20 families. CS01–11 and their 16 scenarios are separate, as are 18 Midnight PCD clauses with 38 scenarios. These are not a combined completion denominator. The baseline's historical status is specified-only. The original 42-predicate catalogue is still a provenance gap; the separate 143 versioned catalogue rows do not reconstruct it [U1, U2].

The current Cardano experiment selects `moriarty/1` and `moriarty-core/1`, `SettlementHashV2`, `GrossDebitAndFees`, `NotBound` cap and `TouchedOnly` frame. Its source says `moriarty/2` reaches the stage loader but the selected compiler refuses it with `StagedNotLowered` [C1]. This is the selected Cardano consumer's scope, not a statement that every current Rust or Sherlock consumer is Core/1. Record the real selected executable and route; `latest Rust` is not a profile.

The K2 CLI's `compile` and `inspect` branches call the selected compiler; that path includes probe synthesis. They are not ordinary report commands. The `prove` and `verify` branches are separate native consumers [C2]. A local invoice-partial proof exists as captured evidence. It uses declared unauthenticated context and does not establish F03, general family compatibility or ledger truth [C3]. No command in this design re-executes it.

The C2 artifact helper constructs structural Cardano data. Its reviewed r2 ordinary tests do not establish Rust admission equivalence, circuit/key correspondence or ledger acceptance, and its published GREEN explicitly grants no CP08 builder reliance [C4]. The R4 contract remains conditional, has public functions including `plan_outputs` and `main`, and separates the Opus-owned checks from the ordinary renderer. Its output contexts are `FABRICATED_LOCAL_VM_TEST`; CB412 preserves the non-claims [C5]. The source-based C2 mint consumer calls `claim.issue`, while its spending consumer calls `claim.release`; S2′ delegates settlement to the separate shard consumers [C6, C7]. These are callsite observations, not correctness judgments.

**Critical mismatch.** EUC-012 requires generated Aiken, checked blueprint, UPLC bytes, Cardano credential, source digest and dependency manifest. EUC-013 requires the actual generated spending entrypoint. The current Rust relation → proof → fixed C2/S2′ design is not automatically that generated-validator route. A fixed C2 mint pass cannot close an original generated-spend scenario. Add explicit versioned Rust/Groth16 route predicates through EU-CON-018; preserve the original EUC clauses and named scenarios open until their own obligations are met or a separately accepted normative reconciliation explicitly disposes of them. No such disposition is made here.

## Profile and target reconciliation

A row key is the tuple `(catalogue, requirement, exact scenario, source profile, semantics profile, consumer route, target id)`. Store an opaque stable ID separately; never use display labels to decide applicability. A clause can have several target rows without any of those rows silently replacing its original scope.

Each mapping records: original clause/scenario bytes and digest; exact applicable subpredicate; supported grammar/action/field domain; original and selected ABI/lifecycle versions; consumer paths and source closure; explicit adapter and its separate evidence; excluded fields and refusal boundaries; proposed amendment and owner disposition. Allowed initial dispositions are `unassessed`, `direct-applicability-proposed`, `adapter-required`, `legacy-only`, and `versioned-refinement-required`. `unsupported` is an implementation result, not permission to erase the obligation.

| Proposed target id | Binding and status |
|---|---|
| `dwu-t-legacy-fa5-cardano-preview-v1` | Original /5 → expression /4 → lifecycle /1 and the original CS10 Preview tuple; historical evidence only until its own currentness and scope are checked |
| `dwu-t-legacy-fa6-cardano-local-v1` | Original /6 → expression /5 → lifecycle /2; no inherited /5 T1 admission |
| `dwu-t-rust-core1-k2-local-v1` | Selected `moriarty/1`, `moriarty-core/1`, K2 ABI v2 relation and exact selector/compiler closure; local, unauthenticated-context evidence |
| `dwu-t-rust-core2-k2-refusal-v1` | Selected staged source consumer and recorded `StagedNotLowered` boundary; refusal evidence, no Core/1 fallback |
| `dwu-t-rust-core1-c2-local-v1` | Proposed explicit adapter to C2 r2/R4 and the pinned fixed C2/S2′/VI/VS artifacts; `qualification=unestablished` |
| `dwu-t-rust-core1-cardano-preview-v1` | Proposed target-specific receipt scope; network tuple and permission to execute remain absent here |
| `dwu-t-rust-core1-cardano-preprod-v1` | Proposed separate eventual goal target; `qualification=unestablished`, `goal=paused` |

Do not fill network magic, genesis, era/protocol-parameter hash, node/CLI versions, deployed script hashes, catalogue anchor, finality policy or actual outrefs with assumed constants. A target id is a label, not evidence. Local, Preview and Preprod observations remain separate. The original CS09/CS10 progression and any later current-target decision remain visible; this design does not jump or restart either. Midnight Preview/Fenrir and retained PCD APIs remain distinct targets and obligations.

## Bootstrap: bounded EU-NAT-028 reporting

Proposed row `dwu-bootstrap-family-report-v1` consumes the existing family inventory and explicit evidence declarations. It preserves all 20 family identities in order with independent source, admission, compiled, native and private status; representation/UPLC/target detail is additional, not merged into compiled/native. EU-NAT-028's **positive scenario is more than a row count**: each asserted native pass must have exact source and ledger readbacks. Its negative scenario retains prototype/template/partial-send/unit-fixture substitutions as unverified. A synthetic reporting fixture exercises the reader only and must not masquerade as a real native-positive result [E1].

Keep ordinary integrity success separate from a selected acceptance gate. Complete all-open inventories should pass integrity. Duplicate or missing membership, target/profile substitutions, mismatched scenario identity and stale declared dependencies should fail the relevant integrity/closure check. An expected absent-checker RED is evidence of absence, not checker behavior: the D7 baseline performed no checker and accepted no script-hash semantics [C8]. No security-owned D7 policy is decided by the reporting reader.

## First three product increments

These are proposed delivery slices, not new replacement milestones. The identifiers below are advisory row identifiers, not already adopted EARS IDs. Their normative additions need the existing owner review. Each original requirement remains independently addressable with every original scenario.

### 1. Source-bound Rust/Cardano artifact consumption

**Developer outcome:** one declared supported Rust relation produces a coherent public artifact bundle that the actual C2 builder consumer reads, or the consumer returns the exact refusal without publishing a success manifest. Start with the existing invoice-partial specimen; also prepare the contrasting reserve specimen through the same public route, with independently specified input/output expectations. A refused reserve action is not the second positive U2 program.

**Proposed rows:** `dwu-rc-artifact-publication-v1` (route refinement), `dwu-rc-artifact-consumption-v1` (builder), `dwu-rc-profile-refusal-v1` (supported/refused selector matrix). These link to EUC-001/002/006/008/011/012/014/016, EU-FIN-002, EU-CON-001/003/018 and UNI-018/019/021/022. EUC-012's original `Positive Publish a checked result` and `Negative Refuse stale success` remain exact originals; a Rust fixed-verifier bundle is separately assessed.

**Real consumers and dependencies:** selected K2 CLI/relation descriptor → admitted public export adapter → exact r2 `prepare_c2_issue` → proposed R4 `plan_outputs`/`main` → actual downstream artifact reader. Resolve CP08 reliance, full R4 source and actual-result review, the corrected script-hash checker and its actual consumer, real build/blueprint/credential binding, public verification-record availability, and profile/ABI reconciliation separately. Do not wire prohibited v1 into R4 or treat r2 admission as Rust acceptance. The captured R4 RED authorizes source authoring only under its recorded terms; revised test bytes and F1 retain their separate result-review status [C4, C5, C8, C9].

**Acceptance:** independent review of the exact route amendment; exact producer and consumer artifact identities; observed consumption by the real public path; failure-isolated output publication; observed specified positive and refusal cases; exact applicable source/result review. Ordinary adapter or manifest evidence may close only its subpredicate. Full EUC-012 still needs its own Aiken/build/test/blueprint observations and complete source profile scope. A manual bundle, fixture renderer alone, matching hashes or a copied manifest does not establish source-to-Cardano correspondence.

**Discriminating negatives:** ordinary reporting tests preserve old-success/new-failure separation, unsupported profile, wrong selector/action and stale dependencies. Reuse the original EUC-012 stale-blueprint scenario. Hostile filesystem/hash/encoding policy and R4/D7 tests remain the designated security owner's existing work; this design neither redesigns nor approves them. Execution authority remains zero for any compiler/build path that reaches synthesis, native or target operations.

### 2. Actual claim issue/consume correspondence with complete outcomes

**Developer outcome:** a supported Rust transition's outputs can be compared through the actual C2 issue and claim-consumption path against an independently frozen complete envelope; a mismatch is attributed to its actual layer. This slice first establishes local compiled behavior, not settlement.

**Proposed rows:** `dwu-rc-c2-issue-correspondence-v1`, `dwu-rc-c2-consume-correspondence-v1`, `dwu-rc-complete-envelope-v1`. Link to EUC-003/004/009/010/013/014, EU-LDG-009/010/013/014, EU-FIN-001/003/005/019/024/025, EU-CON-002/003/005/006 and UNI-020. Preserve the older generated-spend path and EUC-013 scenarios verbatim. In particular, its positive requires the supplied release lender balance 7 to become 107, prior IDs to retain `PriorSpend` plus `Release1`, and revision 4 to become 5; invoice resemblance does not satisfy that original [E2].

**Real consumers and dependencies:** increment 1's artifacts; actual `validators/c2.ak` mint and spend entrypoints; actual decoded transaction contexts; the selected S2′/VS/VI consumer path where claim use is asserted. Decode and execution evidence must distinguish host serialization, actual Aiken decoder, Aiken unit execution, compiled UPLC evaluation, native Groth16 verification and ledger observations. C2 `Issue` and C2 `spend` are separate outcomes. Complete isolated C2 checks do not establish composed S2′ settlement [C6, C7].

**Acceptance:** applicable owner-defined relation/context and verifier mapping; independent positive envelopes for invoice and reserve through their genuinely supported actions; required failure observations with exact diagnostic/layer and first differing path; full candidate and actual-result review. Report `TouchedOnly`/`NotBound` as actual descriptor limitations. A complete frame or capped-intent claim stays blocked until its substantive missing constraints are supplied and accepted by the designated owner. Do not patch missing source behavior with host-computed effects.

**Negative coverage:** preserve EUC-013 `Negative Reject helper-only evidence`; EUC-014 `Negative Reject serializer equivalence shortcut`; EU-FIN-001's differing effect/row/identifier/obligation/diagnostic/retained field; EU-FIN-003's row ordering and unrelated-field cases; EU-FIN-005's residual duty 40 and `NONEMPTY_OBLIGATIONS`; EU-FIN-024's actual traversed work. The proposed evidence reader must refuse closure if any required scenario/result is absent. Security/cryptographic adversarial-case construction and adequacy judgments stay with Opus. No actual compiled/native checks are run by this design.

### 3. One authenticated lineage from claim output to settlement, then bounded composition

**Developer outcome:** for a separately qualified target, one actual authenticated predecessor and validated claim output are consumed by the actual settlement transaction, with complete successor state, payouts, duties, fees, authority and outcome read back. A follow-on accepted transition consumes that successor, demonstrating one real sequential composition. This is a bounded local/product target, not a shortcut to universal or recursive compliance.

**Proposed rows:** `dwu-rc-target-claim-issue-v1`, `dwu-rc-target-settlement-v1`, `dwu-rc-sequential-lineage-v1`. Instantiate each separately for Preview and Preprod; the Preprod instance remains paused. Links: EU-LDG-002/003/005/008/009/010/011/013/014/015/016/021/025/027/030/031; EU-NAT-004–018/025/026/027; EU-FIN-001/019/022/025; EU-CON-002/015/018; retained CS06/09/10/11 lineage. These links are obligations to reconcile, not a claim that a finite single-case receipt closes whole clauses.

**Real consumers and dependencies:** accepted increments 1–2, actual C2 mint/holder, S2′ and all required VI/VS pieces, a genuine source-driven builder/submit/readback consumer, authenticated runtime tuple/outrefs/state, independently frozen expectations, actual measured transaction costs and protocol limits, owner-approved proof/claim/anchor semantics, current native/ledger authority and resources, and an explicit unpause for Preprod. Any absent submit/readback consumer is missing implementation; a DoctorWatson row cannot replace it.

**Acceptance:** distinguish constructed body, evaluated body, signed-body identity, submission result, inclusion and complete readback. Apply EU-NAT-012's persisted signed-CBOR/CLI-TxId prerequisite, EU-NAT-013 returned identifier binding, EU-NAT-014 deadline provenance, EU-NAT-015 phase/outcome categories and EU-NAT-018 complete financial readback exactly within their reconciled profile. Provider success/evaluation alone remains native acceptance false and finality unknown [E3]. Preserve actual fees/collateral and protected-state outcomes on failures. These are source requirements; no signing or target action is authorized now.

**Negative coverage and retained next work:** retain the original node/phase/readback, claim, state, replay and conflict controls under the designated security owner. Reporting negatives must keep timeout unknown, preserve partial-send failure, reject missing payout/readback evidence and prevent local or Preview receipts from satisfying Preprod. After a sequential slice, S03 split/join and S04 migration/termination remain separate obligations: EU-LDG-022 consumes the parent and preserves unique duty ownership; EU-LDG-023 consumes every declared child and authority while retaining all obligations/IDs; EU-LDG-024/025 retain migration and residual-duty/retirement conditions [E4]. No single-parent result closes multi-parent PCD. The remaining disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous composition cases remain required.

## Complete evidence envelope and closure rules

Freeze expectations before observing the candidate. Expected values must be independently derived; importing the evaluator/compiler under test or merely replaying the same evaluator is insufficient. Every compared field has a declared domain, representation and frame obligation. An explicit preserved field and an absent field are different; present-zero and absent liabilities must not collapse.

The minimum envelope for each applicable transition records:

- Exact source/action/selector/program/semantics/ABI identities; complete ordinary pre/post state, field schema and ordering, IDs, revisions, nonce/history, permissions, observations and their authentication status.
- Complete financial balances, allowances, custody/reserves, ordered effects, destinations and asset identities; authorized mint/burn; opening, created/accrued, discharged and closing liabilities; initial principal, accrued debt and outstanding duties; protected/unaffected fields and frame domain.
- Per-payer gross debit, refunds, fees by recipient, minimum net receipts, dust/minimum ADA, change/reserves and separate collateral policy/outcome. Refunds do not erase gross-debit limits; fees count toward net goals.
- Actual traversed logical work, pinned operation costs, static bound, remaining work and closure reserve; separate transport revision/remaining, ordinary revision and lifecycle work. Circuit constraints/proving cost, ExUnits, transaction fees and recovery reservations are separate measurements.
- Original full result/refusal object and source diagnostic; first differing path with expected/actual values; actual command/exit and raw public evidence; original attempt and failure identity; no success inferred for later stages.

The invoice's recorded 109→72 escrow, 35 seller receipts, fee 2/gross debit 37, liability 90→55 and reserve duty 14 are one exact captured scope, not a reusable financial oracle for all new inputs [C3]. Currentness must bind the exact oracle and its independently authored provenance, not just the program source.

Keep four independent axes: implementation, observed evidence, owner semantic acceptance, and computed currentness. Add explicit execution authority and target qualification fields; neither is generated by a green row. Hash equality establishes declared byte currentness only. Authenticate and scope reviews via their own accepted process. Carry original reviewer identity and terminal status. A source review, actual-result review and target admission are not interchangeable.

Evidence classes are explicit: source-inspected, ordinary-local, representation/Aiken-unit, compiled-UPLC, native-proof, target-observed, and formal/semantic evidence where required. Three claim kinds remain separate: universal property certificate; concrete execution proof; authenticated ledger claim. The designated cryptographic owner defines their substantive predicates and dependency relations. DoctorWatson validates required declarations and records that owner's judgment; it does not invent a relation from a matching digest.

A closure gate must require the exact applicable complete predicate, all named scenarios, appropriate real consumers, required evidence classes, reviewed actual results and current dependencies. It must not close a universal statement from sampled cases. Known omitted dependency or unknown applicability means unestablished closure, even if every declared hash matches. Dependency invalidation must include selected profile, oracle, representation adapter, compiler/build, verifier/catalogue mapping, result and applicable review—not just edited code. Retain historical accepted observations when current closure becomes stale.

## Remaining coverage and useful parallel work

Retain the entire 20-family inventory in the companion JSON, not merely the first invoice/reserve cases. Keep U0–U7, M1–M17, MC/SP/G, 277 ACTUS fixtures/fields, 18 executable types, 32 taxonomy dispositions, DS01–07, 72 DeFi rows, DA01–24, supplements, three held-outs, eight intent cases and eight regression classes. Keep mandatory program properties, intent refinement, transition validity and history compliance; general recursion, private handoff, multi-parent PCD and the separate 18 Midnight clauses remain open. EUC-016 explicitly prevents Cardano compatibility from replacing global Midnight acceptance [E2].

Parallelize by real output ownership: (a) independent complete envelopes and original-scenario reconciliation; (b) ordinary artifact consumer/renderer wiring; (c) designated-owner R4/D7/claim/verification work; (d) target receipt/readback requirements and composition source mapping. One integrator owns shared ABI/CLI/schema changes. No agent may edit the independent oracle to make its own implementation pass. Dependency preparation can proceed while runtime is blocked, but a report's next-action field describes work rather than dispatching it. This task spawns no further agents.

## Decisions still required

1. Adopt the additive current-Rust fixed-verifier route and its exact supported grammar/actions; decide separately the disposition of the legacy generated-Aiken obligations. This draft supplies no automatic waiver.
2. Select the authoritative current consumer/source artifact set for r2/R4 and script-hash checks. Reconcile later adoption receipts against each frozen version instead of promoting historical author-time prose.
3. Designated security owner: settle substantive complete-frame, intent-cap, key/relation/anchor, authentic context and claim-consumption obligations, proof trust and adversarial test adequacy. Existing `TouchedOnly`, `NotBound` and unauthenticated context labels cannot be promoted by this adviser.
4. Freeze independent invoice/reserve complete envelopes and their admitted positive action selectors; map any retained family explicitly. Keep invoice-partial distinct from F03's AccrualFirst debt semantics.
5. Establish a separately approved target ordering/tuple and observable ledger consumer when execution is eventually authorized; Preprod remains paused and native authority remains zero here.

## Pinned references and read limits

`U`/`C`/`E` references use repository commit **1721457439dbd6d988d00c6c86d1cf648caf2a0c**. `S` uses Sherlock commit **0aecf98455dd9abc1635801166151431cba309f7**. Line ranges identify inspected bounded spans. Frozen comments/status are historical source text; later captured disposition can qualify them only for its exact scope.

| Ref | Path and lines |
|---|---|
| S1 | `models/sherlock/qa/check.py:682–914` |
| S2 | `docs/sherlock/implementation-loop.md:1–95`; `docs/DOCTORWATSON.md:1–65`; `docs/DOCTORWATSON-CI.md:1–60` |
| U1 | `docs/utxo/MAIN-INTEGRATION-2026-10-17.md:1–23` |
| U2 | `docs/utxo/current-rust-pattern-callsite-inventory-2026-10-16/README.md:1–13` |
| C1 | `experiments/moriarty-cardano-rust/k2/src/relation.rs:1–35` |
| C2 | `experiments/moriarty-cardano-rust/k2/src/main.rs:137–222`; `experiments/moriarty-cardano-rust/k2/src/claim.rs:1–105` |
| C3 | `evidence/utxo/current-rust-groth16-first-proof-2026-10-16/README.md:1–50` |
| C4 | `evidence/utxo/current-cardano-c2-r2-green-2026-10-17/README.md:1–11`; `experiments/cardano-preprod-host/README.md:1–31` |
| C5 | `evidence/utxo/current-cardano-c2-builder-r4-f1-baseline-2026-10-17/reviewed-source/R4-CONTRACT.md:1–110` (bounded excerpt, not full contract) |
| C6 | `experiments/cardano-four-piece-settlement/validators/c2.ak:1–31`; `experiments/cardano-four-piece-settlement/lib/pvc2/claim.ak:1–105` |
| C7 | `experiments/cardano-four-piece-settlement/validators/s2p.ak:22–76` |
| C8 | `evidence/utxo/pending-review-checkpoint-2026-10-17/README.md:1–11` |
| C9 | `evidence/utxo/current-cardano-c2-builder-r4-ordinary-red-acceptance-2026-10-17/README.md:1–5`; `evidence/utxo/current-cardano-c2-builder-r4-f1-baseline-2026-10-17/README.md:1–7` |
| E1 | `openspec/changes/eutxo-native-validation-and-receipts/specs/eutxo-native-validation/spec.md:315–337` |
| E2 | `openspec/changes/eutxo-compiler-correspondence/specs/eutxo-compiler/spec.md:124–177` |
| E3 | `openspec/changes/eutxo-native-validation-and-receipts/specs/eutxo-native-validation/spec.md:135–218` |
| E4 | `openspec/changes/eutxo-ledger-state-and-authority/specs/eutxo-ledger-state/spec.md:364–431` |
| E5 | `openspec/changes/eutxo-financial-pattern-validation/specs/eutxo-financial-patterns/spec.md:7–48,63–76,217–230,329–356` |
| E6 | `openspec/changes/eutxo-conformance-and-private-feasibility/specs/eutxo-conformance-private/spec.md:137–201` |
| E7 | `openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md:179–266` (heading index and entrypoint description; scenario bodies not fully reviewed) |

Read limits: this is bounded source navigation and design. The shared DESIGN/ASSESSMENT/SOURCE-INVENTORY/EARS-BASELINE were read as draft inputs, with JSON summaries and selected original clauses/scenarios; root edited the draft baseline concurrently. Their working-tree hashes are captured in the companion JSON, not attributed to the baseline commit. Several broad startup/read commands were output-truncated; only displayed bounded excerpts and explicitly parsed fields support the findings. Whole original design documents, complete runtime implementation, frozen contracts, all 292 scenario bodies, full review packets and underlying native results were not exhaustively inspected. Sherlock references were read with `git show` at the Sherlock pin; three same-named files in the UTXO working tree differ and are not substituted for those references. Startup status was read and treated as historical operational context, not current work selection. No graph existed at the normal root; no graph build/research acquisition was needed. No web retrieval, test execution, compiler import, proof/VM/ledger action, wallet/key/PK/SRS/cache access, private provider capture or historical external path traversal occurred. Public evidence summaries are cited as recorded observations, not fresh reproduction or substitute security review.
