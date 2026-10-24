## Top finding

Conditional source approval for the unexecuted ScalarEntry leaf only. I identify no mandatory production/consumer counterexample in the inline source. Compile/test execution, native proving, and full-goal closure remain unestablished.

**Reviewer identity/coverage:** Muse Wonder 1.0 HIGH, independent non-author source review under the stated fallback. I used only the inline material. That includes the five subject files (`c6_scalar_entry_step.rs` 27,238 B, `c6_scalar_entry_consumer.rs` 32,587 B, `c6_scalar_entry_step_tests.rs` 43,367 B / 7 tests, `c6_scalar_entry_consumer_raw_tests.rs` 44,878 B / 13 tests, parent `mod.rs` 20,581 B), the called canonical/push/Core/SDK bodies listed in the membership (C1 canonical emitters, Core selector/family/`impl_constructor_scalar`/owner driver/`complete_observation`, physical codecs, producer/lowering bodies, SDK IR/Fr/`ProofPreimage`/`transient_commit`). No called production body needed for this leaf was hash-only or missing in the prompt. Missing premises are execution premises, not source bodies: no compiler, test, MockProver, synthesis, native gadget, proof, or Git result was supplied or run. Hashes were not used as approval.

---

## DESIGN — AGREE / APPROVE, bounded leaf

The five-site/eleven-shape design matches the actual Core paths supplied:

| Area | Judgment |
|---|---|
| Sites/modes | RightF `Add/Sub/Mul/Lt/Cons/Branch`, UnaryF `Leaf/Not`, IfF `TestBool`, ReturnF `TestType`, unchecked ConstructorFieldsF `TestType`: correctly encoded in `C6ScalarEntryStepEmitter::emit`. |
| Exclusions | `RightF{Eq}` does not become ScalarEntry; `ConstructorFieldsF{checked:true}` does not become ScalarEntry. This matches `owner_helper_plan`, `plan_scalar_lowered`, and `impl_constructor_scalar`. |
| Identity copying | Operand A/B, expected TypeId, common site, `origin_reference_cursor = pre.reference_cursor`, `parent_resume = None`, `phase = First`, `reg_a = 0`, `staged = None` are copied from actual pre lanes, not from a host flag. |
| Driver | Applied: remaining−1 / consumed+1 / helper_work+1, helper visits +0, reference cursor +0. P3 at the original separate admission `max_helper_work`: uncharged `Exhausted{HelperWork}`, helper remains absent, only outcome changes. Fuel 0 remains the separate publish path, not Entry. |
| Constructor arity | Neither Core `impl_constructor_scalar` nor this emitter invents an SI15 arity call. Correctly disclosed as an upstream/downstream premise, not fabricated validity. |

**Bounded limitation, not a design defect for this leaf:** the emitter discharges the installation/copying connection. It does not discharge SI08, SI15, SI17, SI20, SI21, SI22, full `StepInv`, program membership, predecessor/successor composition, native issuance, or cryptographic provenance.

## SECURITY_RELATION — AGREE / APPROVE for the stated leaf relation; typed ABSTAIN outside it

The accepted relation is stronger than “an honest producer would do this”:

- Pre-state gates require helper/result absent, task count 0, Running outcome, positive remaining work, `ControlC::Return`, and positive depth `<= max_frames`.
- The top frame is selected dynamically at `frame_count - 1`. Tag/op derivation uses the actual frame lanes: Right op lanes, Unary op lanes, If tag, Return tag, ConstructorFields tag plus `checked == false`.
- Exactly-one selected shape is enforced by `selected == 1` under `enable`.
- Site/mode derivation is chained from those same booleans: RetRight 11, RetUnary 12, RetIf 13, RetCall 16, RetCtorFields 25; modes Add 0 … TestType 9.
- Operand A is RightF left Ref or the Return value Ref; operand B is present exactly for RightF and copies the Return value Ref; expected is present exactly for ReturnF/ConstructorFieldsF and copies `return_type`/`field_type` exactly, builtin `0..=3` and nominal `>=4` alike at the emitter level.
- Installed post-helper lanes are constrained in the emitted block; the consumer correctly does **not** bind post-state to the ungated `post_*` receipt operands in `C6ScalarEntryStepBlock`.
- Composed bound META, PROG/CTX canonicality plus expected-CTX binding, pre DATA canonicality with unconditional DATA equality, and pre/post HELPER canonicality are actually emitted. Therefore an encodable wrong site, mode, Ref space/index, expected TypeId, origin cursor, phase, `reg_a`, staged lane, counter charge, helper/result/task installation, or preservation lane reaches SDK preprocessing rejection rather than being filtered by producer semantics.
- P3 is correctly separated: `disposition_is_refused_helper_work` is tied to `pre.helper_work == retained admission max_helper_work`; post outcome is Exhausted/HelperWork, post helper equals pre helper, counters are uncharged. The actual helper-work maximum comes from retained `C6PushFamilyParams` admission bounds and is also bound through bound-META admission-bounds lanes; it is not substituted from layout bounds.
- Disabled mode still requires schema/carrier constants, unconditional META/PROG/CTX and DATA equality, unconditional 128-bit lane ranges, and gated full image/helper equality.

**Precise retained premises the relation still relies on:** non-selected frame lanes and non-top frames, control origin/env/stack/alloc/active/segment coherence, full frame/control canonicality, SI08/SI15/SI17/SI20/SI21/SI22/full `StepInv`, program membership and provenance, and successor composition. Those must remain explicit caller/composition obligations. If this leaf were claimed as a standalone full machine-step proof without those premises, that broader claim would be CHANGES_REQUIRED; as a precisely scoped installation leaf, they are bounded limitations.

**Typed abstention:** I make no judgment that SDK cryptographic P2R/native gadget derivation, native constraint execution, proving, verifier degree/k, ledger deployment, or stagenet acceptance is satisfied. `check_c6_scalar_entry_step_circuit` is `ProofPreimage::check` / IR preprocessing only.

## SOURCE_API — AGREE / APPROVE, source-only

The public surface is coherent with the supplied APIs:

- Parent `mod.rs` registers and re-exports `C6ScalarEntryStepEmitter`, `emit_c6_scalar_entry_step_block`, `build_c6_scalar_entry_step_circuit`, `check_c6_scalar_entry_step_circuit`, `record_scalar_entry_step`, and the `witness_scalar_entry_from_host_pair` / `witness_scalar_entry_from_owner_record` aliases used by the positive tests.
- `record_scalar_entry_step` calls `complete_observation::record_step` exactly once. Its non-rollback contract is explicit and accurate: an `Err` after that one step does not mean “no step happened.”
- `witness_from_host_pair` is the independent-post route. It refuses prefix/config/geometry/encoding problems as producer errors, but it does not run a host semantic/StepInv filter and does not regenerate post from pre. A semantically wrong encodable post can reach the checker.
- `validate_owner_record` checks the actual event/snapshot fields: rule, HelperEnter, Scalar kind, disposition, deltas, visits/reference-cursor zero, allocation zero, outcome/boundary, source occurrence with checked path arithmetic, depth/task cursors, helper phase/cursor before/after, full preservation, applied charges, and P3 non-installation. It correctly claims only internal consistency of a publicly constructible record, not authenticated provenance.
- `run_id` is not in the physical root statement; no cryptographic `run_id` binding is claimed. Correct.
- Both `LocalPublic` and `LedgerCall` use the same statement predicate and statement order. `LedgerCall` output uses only constrained post image/DATA/helper operands; its commitment is recomputed from the current statement and current post-flat lanes on every check.
- Emitter namespace discipline is real in source: five composed reserved segments are refused in scopes and caller operands; retained emitters advance serials. The free function is documented single-shot per scope.

No API repair is required for this leaf. A circuit-authentication/pinned-`k` claim would be a separate delivery item; none is made, and instruction count is not equated with degree.

## TEST_SOURCE / MEANINGFULNESS — AGREE for delivered predicates; bounded coverage only

The 20 test bodies are meaningful as source, and remain entirely UNRUN/uncompiled. Zero passive parse errors would not be typechecking; I do not treat the tests as passed.

Delivered, by source inspection:

- Positive tests admit through real public Core/Ext admission and `admit_run`, search with real `record_step`, stop **before** Entry, use target-aware `is_target_entry_pre`, take the selected step once through the wrapper and once through a fresh raw replay, and assert wrapper POST equals replay POST. No fabricated `MachineImage`, synthesized positive `HostSnapshot`, or tuned POST is used for positive baselines.
- All 11 shapes are tabled, plus builtin ReturnF expected `0..=3`, nominal ReturnF `TypeIdC(4)`, and genuine nonempty nominal ConstructorFieldsF with exact `TypeIdC(4)`, `TypeIdC(5)`, `CtorIdC(5)`, index 0. Inner Add history is genuinely traversed before the outer Mul/Data case.
- Exact P3 is learned from a genuine generous-bound pre-state and replayed with the separate admission bound set to that value; layout bounds are not substituted.
- Owner-record mutations and a genuinely distinct Add/Sub crossed record are present; the file correctly limits this to internal consistency, never authenticated provenance.
- One encodable wrong installed mode is asserted to reach SDK check in both modes.
- Raw tests include genuine RightAdd, CtorFields, and MulDataLeft producers; private-child raw Fr access; paired positives; wrong helper/counter/preservation/selector/domain mutations; current-statement/current-post-CC recomputation through `check_c6_scalar_entry_step_circuit`; coherent selector-companion pass; gate-0 coherent equality; both-mode `Fr >= 2^128` gate-0 companion; producer-refusal categories; namespace serial test.

Necessary qualifications:

1. **Chronology/oracle:** the 11-shape predicate was author-visible before C1 and is not a private blind golden. The coverage contract came after C1/C2 and the tests after C2. They are exposed source expectations, not independent pre-frozen private-oracle evidence.
2. **Active DATA:** `raw_active_data_row_preservation_both_modes` does satisfy the delivered preservation predicate: genuine `Mul(Add(1,2),3)`, DATA count 1, row 0 tag U128/payload 3, exact `Data(0)`, pre DATA == post DATA, and both-mode rejection of a mutated post tag. It does **not** isolate DATA canonicality: mutating post tag `0 -> 1` while payload remains `3` is coupled — post DATA differs from pre DATA, and tag 1/Bool with payload 3 is also non-canonical. That limitation is truthfully not claimed as isolation in the inventory. A canonical-isolation companion would be optional coverage, e.g. mutate pre and post DATA identically to a non-canonical Bool row so equality holds and only canonicality can reject; it is not required to make the current preservation test truthful.
3. **DATA lane history:** the old Sol/owner `DATA_HEADER` classification was wrong. Current source is correct: `DATA_HEADER_LANES = 7`, header lanes `0..6`, first row tag at lane `7`. The RightAdd mutation is first **inactive** padded row tag; the new MulDataLeft test is the genuine active-row specimen.
4. **Namespace coverage:** `local_temp_ids` collects actual SDK `Identifier` output names, but the repeated-serial assertion proves only `.local.temp.` disjointness. It does not prove all five prefixes or all 16 operand groups. That remains REQUIRED_OPEN, correctly not overstated as delivered exhaustive coverage.
5. **Mode coverage gaps:** many raw Fr mutations are LedgerCall-only; the file does not justify extrapolating them to LocalPublic. Both-mode claims are sound only where the loops actually run both modes, such as helper-domain `reject_with`, active DATA, coherent P3, and gate-0 controls.
6. **Raw P3 carrier:** `p3_witness` in the raw file is a labelled mutated coherent carrier, not genuine owner reachability. Genuine P3 reachability is supplied separately by the positive file’s `exact_p3_at_original_separate_admission_bound`.

No test-source repair is mandatory for the bounded leaf predicates actually claimed. Exhaustive namespace/operand-group, every PROG/CTX lane, 129-bit slice-end boundary, every DATA row/frame lane, and broader per-mode isolation coverage remain mandatory for the full goal, not silently delivered here.

## FAILURE_STAGE_SCOPE — AGREE

Failure categories are kept distinct in source:

- Host/producer refusals: prefix mismatch, layout/config mismatch, geometry/length mismatch, reserved operand, scope refusal, and checker-shell length mismatch are not presented as SDK relation failures.
- SDK relation failures: `check_c6_scalar_entry_step_circuit` wraps `ProofPreimage::check` failure as `SDK ProofPreimage::check failed: …`; raw relation negatives assert that category rather than merely `anyErr`.
- Stale commitment: the SDK `ir-vm.rs` preprocess loop executes instructions first, then transcript-consumption checks, then the communications-commitment comparison at the end. The stale test therefore correctly presents an unchanged relation-valid statement with a commitment computed from a different basis and asserts the actual `Communications commitment mismatch` category. It is not counted as a relation negative. The changed-statement/current-CC companion remains a distinct relation negative.
- Wrapper refusal: wrong-rule and terminal refusals are distinguished, and the wrong-rule test asserts exactly one owner step persisted (`consumed_work + 1`), matching the documented non-rollback contract. Terminal refusal asserts no mutation.

## DELIVERY_SCOPE — typed ABSTAIN for execution/native/full-goal delivery

Not delivered or not established by this source candidate:

- Compilation, test execution, measured degree/k, performance, native proof, verifier, ledger, or stagenet result.
- Full 39-selector closure, SI08/SI15/SI17/SI20/SI21/SI22/full StepInv, predecessor/successor bridges, program membership/authentication, P1/native issuer/SourceSpec, rollback/auth claims, groups 1/2/4/8, residual/finance/persistence/recursion, and all REQUIRED_OPEN items named in the task.

`nativeC13UNSELECTED`, `c=null / S NOT_ESTABLISHED / stoppedAm10` are unchanged by this review. No resource or authority change is recommended from source review alone.

## OVERALL — AGREE / APPROVE, conditional unexecuted leaf only

**Recommendation:** approve `c6_scalar_entry_step.rs` + `c6_scalar_entry_consumer.rs` as a bounded, unexecuted ScalarEntry installation leaf, with the two test files accepted as meaningful source coverage for the predicates they actually claim.

**Mandatory source fixes identified:** none in production, consumer, or the claimed test predicates.

**Mandatory gates before adoption/execution claims:** compile the workspace and run all 20 tests; then separately satisfy the retained SI/program-membership premises and native/full-goal requirements. Those are adoption blockers, not hidden source passes.

**Optional/bounded coverage to add later:** canonical-isolated DATA companion, exhaustive five-prefix/16-group namespace vectors, every PROG/CTX/DATA/frame lane, broader both-mode raw isolation, and the 129-bit derived slice-end boundary.