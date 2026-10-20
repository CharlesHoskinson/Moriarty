## Overall

**APPROVE for the bounded, unexecuted ScalarEntry leaf.**

No mandatory production, consumer, or test-source fix is established from the supplied source.

This is not compile, test, proof, native, full-StepInv, or full-goal approval. Full completion remains **OPEN**. Native cryptographic execution is a typed **ABSTAIN** below.

Reviewer identity: **Muse Wonder 1.0 HIGH**, independent non-author review under the stated fallback. No tools or execution were used. Membership hashes were not treated as approval evidence.

## Declared coverage

The decision object is the five supplied subject files:

- [c6_scalar_entry_step.rs](c6_scalar_entry_step.rs)
- [c6_scalar_entry_consumer.rs](c6_scalar_entry_consumer.rs)
- [c6_scalar_entry_step_tests.rs](c6_scalar_entry_step_tests.rs)
- [c6_scalar_entry_consumer_raw_tests.rs](c6_scalar_entry_consumer_raw_tests.rs)
- The supplied parent [mod.rs](mod.rs)

I also used the supplied Core selector, Scalar row-effect, ConstructorFields read, physical codecs, canonical composed emitters, owner observation, and SDK preprocessing sources needed to check mappings and failure order.

No called body needed for the bounded emitter/consumer judgment was found missing. I do not claim whole-crate, whole-SDK, native-gadget, proof, or verifier qualification.

## DESIGN — APPROVE

The five-site and eleven-shape design matches the actual Core dispatch:

- In [owner_runtime](owner_runtime), `owner_site` maps a `Return` control with a nonempty frame stack to the top frame. `owner_helper_plan` selects Scalar for `RetRight` except `Eq`, for `RetUnary`, `RetIf`, and `RetCall`, and for unchecked `ConstructorFieldsF`.
- `impl_constructor_scalar` requires `BHelperScalarEntry`, `RetCtorFields`, absent helper/result, `Return` control, and `checked == false`. It copies `field_type` exactly and does not call an arity checker.
- The general Scalar entry effect in [scalar row effects](scalar%20row%20effects) covers the other four sites. `RightF{Eq}` belongs to Equal. `ConstructorFieldsF{checked:true}` belongs to Append.

In `C6ScalarEntryStepEmitter::emit`, the dynamic top-frame derivation uses `pre_image[L_FRAME_COUNT - 1]` in bottom-to-top order. The selector correctly requires exactly one of:

- `RightF` with `Add/Sub/Mul/Lt/Cons/Branch`;
- `UnaryF` with `Leaf/Not`;
- `IfF`;
- `ReturnF`;
- `ConstructorFieldsF{checked:false}`.

The installed mapping is also correct against [c6_phys_helper.rs](c6_phys_helper.rs):

- Sites: `RetRight=11`, `RetUnary=12`, `RetIf=13`, `RetCall=16`, `RetCtorFields=25`.
- Modes: `Add=0`, `Sub=1`, `Mul=2`, `Lt=3`, `Cons=4`, `Branch=5`, `Leaf=6`, `Not=7`, `TestBool=8`, `TestType=9`.
- Right operands are `(left, Some(Return value))`.
- Unary, If, Return, and ConstructorFields operands are `(Return value, None)`.
- Return expected type is copied from `ReturnF.return_type`.
- ConstructorFields expected type is copied from `ConstructorFieldsF.field_type`, including nominal values.
- Common site, pre-state `reference_cursor`, `parent_resume=None`, `First`, `reg_a=0`, and `staged=None` are installed.

The driver also has the intended distinction:

- Fuel zero is not treated as Entry P3.
- P3 is `helper_work == retained admission max_helper_work`.
- P3 produces `Exhausted{HelperWork}`, installs no helper, and charges nothing.
- Applied Entry charges remaining/consumed/helper work by one, adds zero helper visits, and leaves the reference cursor unchanged.
- META uses retained admission and layout bindings through `C6PushFamilyParams`, rather than deriving `max_helper_work` from layout bounds.

## SECURITY_RELATION — APPROVE, conditionally bounded

The emitted relation judges the actual supplied lanes, not merely an honest producer trace.

In particular:

- All supplied META, PROG, CTX, image, DATA, and helper lanes receive unconditional 128-bit constraints.
- META, PROG, and CTX pre/post equality is unconditional.
- DATA pre/post equality is unconditional.
- Applied and P3 helper installation is constrained lane-by-lane.
- The post-helper canonical block validates the installed Scalar record, Ref spaces and caps, optional Ref padding, expected-TypeId cap, phase, staged lanes, and unused Scalar lanes.
- Preservation covers all nonmutable image lanes. This includes frames, control, env root, helper visits, reference cursor, slots-related image lanes, active/alloc lanes, DATA, result, and tasks.
- A wrong site, mode, full Ref, expected presence/value, common cursor, phase, `reg_a`, staged value, counter, or preserved lane is therefore not accepted merely because a host producer generated the rest honestly.

Concrete source-level counterexamples are correctly rejected by construction:

- Changing the top Right operator to `Eq=3` makes `right_valid` false.
- Changing ConstructorFields `checked` to `1` makes `ctor_valid` false.
- Swapping operand A with the Return Ref changes the constrained post-helper Ref lanes.
- Installing `Sub` for a genuine `Add` frame changes `post_helper[REC + 5]` from the dynamically derived mode.
- Installing an expected value on a Right frame conflicts with `post_expected_present=0` and the post-helper canonical form.

The ungated receipt issue is handled correctly. `C6ScalarEntryStepBlock.post_*` values are derived even under disabled or P3 execution, but `build_c6_scalar_entry_step_circuit` does not bind them to post-state or LedgerCall output. The LedgerCall output uses only the constrained post-image, post-DATA, and post-helper statement operands.

### Retained security premises

The relation does **not** discharge:

- SI08, SI15, SI17, SI20, SI21, SI22, or full `StepInv`;
- full pre-frame/control canonicality beyond the selected fields and the installed-helper validation;
- program membership or the top frame’s program/origin coherence;
- predecessor or successor composition;
- authenticated provenance of a publicly constructible `CompleteStepRecord`;
- native issuer, SourceSpec, P1, rollback, or ledger authentication.

Those are genuine premises of the accepted leaf relation. They are bounded limitations, not hidden producer-trust substitutions, because the emitter and consumer state them explicitly.

## SOURCE_API — APPROVE

The consumer API keeps the routes distinct:

- `witness_from_host_pair` is an independent-post route. It permits producer refusal for prefix, retained-config, geometry, codec, or length errors, but it does not semantically regenerate or filter an encodable wrong post.
- `witness_from_owner_record` validates internal event/snapshot consistency only. It correctly does not claim authenticated provenance.
- `record_scalar_entry_step` calls `complete_observation::record_step` exactly once. Its non-rollback behavior is explicit and tested.
- `expected_entry_helper` is confined to the owner-record route. It is not used as a host-pair semantic filter.
- Both `LocalPublic` and `LedgerCall` use the same emitted block and statement order.
- LedgerCall recomputes `currentstatement/currentpostCC` on each check.
- `check_c6_scalar_entry_step_circuit` is only SDK `ProofPreimage::check` preprocessing. It does not claim synthesis, proving, MockProver, key generation, measured degree `k`, or native execution.

One bounded API limitation remains: `check_c6_scalar_entry_step_circuit` compares witness and circuit geometry, lengths, and meta offset, but does not require complete `C6PushFamilyParams` equality. The result is therefore relative to the supplied circuit and its retained parameters. It is not a pinned-circuit authentication API. This is a limitation of circuit authentication scope, not a defect in the honestly built relation.

## TEST_SOURCE / MEANINGFULNESS — APPROVE as bounded source coverage

The 20 tests are source-meaningful, although entirely **UNRUN and uncompiled**.

The seven positive tests use genuine admission and owner stepping:

- `all_eleven_shapes_both_modes` covers the eleven shapes in both statement modes.
- `builtin_returnf_expected_0_to_3_both_modes` covers all builtin ReturnF expectations.
- `nominal_returnf_and_ctorfields_exact_both_modes` covers nominal ReturnF and a genuine nonempty ConstructorFields frame.
- The target-aware search prevents the inner `Add` in `Mul(Add(1,2),3)` from being mistaken for the outer Data-left Mul.
- The original wrapper post-state is compared with a fresh replay post-state.
- Expected helper values are derived in the test from the actual pre-state and the frozen predicate table, not copied from the wrapper witness.
- Exact P3 uses the genuine pre-state helper-work value as the separate admission maximum.
- Wrong-rule wrapper refusal checks that exactly one owner step persists. Terminal refusal checks that no step occurs.
- Owner-record mutations and a genuinely distinct Add/Sub crossed record are rejected.
- An encodable wrong installed mode reaches the SDK checker in both modes.

The chronology is not a private blind oracle. The original eleven-shape predicate was frozen before the emitter, but the detailed coverage contract and tests came later and were author-visible. No independent pre-frozen golden or test pass may be inferred.

### Raw tests and the active DATA point

The active-DATA test is meaningful as bounded first-row preservation coverage:

- `Mul(Add(1,2),3)` genuinely allocates the inner result before outer Entry.
- The test checks active count `1`, tag `0`, U128 payload `3`, exact `Data(0)`, and complete DATA preservation.
- The post-tag mutation violates the unconditional post-DATA/pre-DATA equality. Since DATA canonicality is emitted over the pre-DATA while post-DATA is bound by equality, this is a valid preservation negative for that lane.
- It does not establish exhaustive DATA-row coverage. Every other row and lane remains open.

A canonical-valid payload companion, such as changing U128 payload `3` to `4`, could strengthen attribution further, but it is **optional coverage**, not a mandatory repair. No isolated-preservation claim beyond the delivered lane is made.

The stale-commitment test is also correctly categorized. It first proves that the unchanged valid statement with its current commitment passes. It then supplies a commitment computed from a different statement. In the supplied SDK `preprocess`, instruction processing and transcript consumption occur before the final communications-commitment comparison, so this stale-only case can reach `Communications commitment mismatch`. The mutated-statement/current-commitment case remains a separate relation negative.

Producer refusals, checker-shell length refusals, SDK preprocessing failures, and stale-commitment failures are kept in separate categories.

### Test limitations

These remain bounded limitations or optional broader coverage:

- Identifier reuse is proved only for actual `.local.temp.` outputs, not all five composed prefixes or all sixteen operand groups.
- Several raw Fr mutations are LedgerCall-only and must not be extrapolated to both modes.
- Exhaustive PROG and CTX lanes are not covered.
- The 129-bit derived slice-end boundary is not isolated by a dedicated receipt.
- Every frame slot, DATA row, Const/Data variation, and wrong-post isolation case is not covered.
- No test execution, typechecking, or compilation evidence exists.

## FAILURE_STAGE_SCOPE — APPROVE

The failure stages are not conflated:

1. Host producer stage: retained-layout/config mismatch, prefix mismatch, geometry refusal, codec refusal, statement-length refusal, and reserved-operand refusal.
2. Owner-record stage: internal rule, site, disposition, counter, source-occurrence, helper, and preservation mismatch.
3. SDK preprocessing stage: the actual circuit relation rejects a semantically wrong but encodable post.
4. Commitment stage: a relation-valid statement with a stale LedgerCall commitment reaches the commitment comparison.
5. Owner execution stage: the wrapper may already have taken exactly one step before refusal; no rollback is promised.

The tests generally assert these categories through typed errors or the actual SDK preprocessing error path. They do not treat `anyErr` as proof of a specific internal constraint.

## DELIVERY_SCOPE — TYPED ABSTAIN for native and full-system claims

I abstain from approving:

- SDK native gadget derivation or native constraint execution;
- proving, verification, key generation, or ledger deployment;
- degree, performance, or measured `k` claims;
- all 39 required selectors, witnesses, constraints, and exports;
- full SI08/SI15/SI17/SI20/SI21/SI22 or StepInv closure;
- groups `1/2/4/8`, residual, finance, persistence, recursion, or stagenet acceptance.

Those items remain **REQUIRED_OPEN**, as do exhaustive operand-group, namespace, PROG/CTX, DATA-row, frame, and 129-bit-boundary coverage.

## Mandatory fixes

**None established for the bounded ScalarEntry source leaf.**

The retained StepInv/program-membership premises, incomplete exhaustive coverage, lack of circuit authentication, and absence of execution evidence prevent a broader approval. They do not convert the precisely bounded, unexecuted leaf into a failed candidate.