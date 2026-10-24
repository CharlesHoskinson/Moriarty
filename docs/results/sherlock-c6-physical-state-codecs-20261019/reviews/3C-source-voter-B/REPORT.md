# Chunk 3C review, voter B: `c6_phys_state.rs` and registration

**Overall: AGREE.** I found no mandatory owner correction. I ABSTAIN only on the runtime claims of tests 3 and 4 (see §7).

This is a source-only review. I compiled nothing, ran no tests, edited nothing, and recomputed no hashes. Byte counts and SHA-256 values are taken as supplied. One check is arithmetic only: the registration delta is 1220 + `"pub mod c6_phys_state;\n"` (23 B) = 1243, which matches. The anchor is unique in the supplied parent.

## Verdicts

| Surface | Verdict |
|---|---|
| DESIGN | **AGREE**. Coherent with C6P1. Adopting the profile/ABI remains Root's decision. |
| SOURCE_SECURITY | **AGREE**. Representation codec only. |
| NORMAL_API_COMPILE | **AGREE**. By source trace only; no type-check is claimed. |
| Runtime of tests 3 and 4 | **Targeted ABSTAIN** |

---

## 1. Design

**Framing.** The section has a fixed 41-lane header followed by three children laid out contiguously.

- **Header lanes 0–3:** magic, version, `STAT`, and a child count of 3.
- **Directory lanes 4–9:** a fixed-order directory with tags IMAG, DATA, HELP.
- **Lanes 10–11:** ext-presence and a reserved lane.
- **Lanes 12–40:** the 29-lane capacity descriptor.

The design is sound:
- `STAT` (`0x5354_4154`) and the directory-only `IMAG` (`0x494D_4147`) are distinct from DATA, HELP and PROG.
- Each child keeps its own magic/version, and the data and helper children keep their own tag. That costs 9 redundant lanes but reuses the frozen parent codecs byte-for-byte.
- Nothing in Chunk 1 changes: the 13/17/48 layout, the 10 frames, 11 controls, origins and the raw bottom-to-top stack are all untouched.

**Descriptor.** I recounted all 29 lanes:
- 27 `C6PhysCaps` fields in exact declaration order (`max_frames` … `max_template_bytes`).
- Then `max_data_field_slots` and `max_helper_tasks`.
- Spot values: lane 12 is `max_frames`, lane 22 is `max_arity`, lanes 39/40 are the two `C6StateCaps` extras. The test literals `(64, 8, 1024, 64)` match `test_caps`.

**Ext presence.**
- `from_parts` takes `Option<ExtCapacitiesC>` (Copy) and computes the effective caps itself.
- Absent and present-zero ext produce equal caps but a different lane 10.
- `from_layout` and `from_parts` derive the context identically, and `real_ctx` asserts that.

**What the context is, and is not.** It is an independent caller assertion. It is not bound to `host.admission`, and `LayoutSnapshot` really has no ext field.

- Matching caps do not establish identity. The descriptor does not cover `layout_version`, `step_cap`, `max_reference_work`, `serialization_bytes`, widths, or the 13 finance/resume ext fields. Two different layouts can therefore share a descriptor. The source never claims otherwise.
- `retained_layout_agrees_with_host` compares all 7 `LayoutSnapshot` fields, plus `admission.bounds`.
- **Limitation:** no normal API calls it. `encode_state`, `decode_state`, `state_lanes_from_host` and `compare_uncached_witness` all accept any context. Only test 3 calls the check. This is a limitation, not a defect, because the codec is representation-only (see O3).

**Inherited policies, reused unchanged:**
- The `max(max_types, 4)` builtin-TypeId rule (Chunk 1 frames/controls; 3B `Scalar.expected` and `PairTask.sort`).
- Plain `max_types` for user-nominal 3A Constructor and 3B `Build.type_id`.
- **Retained gap NB1 (from 3B voter A):** `Materialize.type_id` and `Build.type_id` cap the same value under different policies. They diverge only when `type_rows < 4`, which is unreachable for Build. 3C adds no new domain gap.

**Root counters are not normalized.** `alloc`, `active`, `seg_active`, `seg_alloc` and `seg_consumed_work` are carried independently of vector lengths. Test 2 asserts the 21-vs-3 case. The module doc correctly assigns SI-05/SI-10/StepInv to Chunk 4.

## 2. Framing, offsets, first-error order and limits

**Decode order.** I traced `decode_state` against the module doc's steps 1–8, in this exact order:

1. `LimitExceeded`, before any read.
2. Fewer than 41 lanes: `Truncated`.
3. Magic, version, tag.
4. Child count.
5. For each child, in lane order: tag, then `rd` length (`UsizeOverflow`), then `ChildLengthRange` against the parent minimum (48/7/31).
6. Ext lane: `rd_bool`, then `ContextMismatch`.
7. Reserved lane: `zero`, reported with lane index 11.
8. Descriptor lanes 12..40.
9. `offsets()` with checked add.
10. `Truncated`, then `TrailingLanes`.
11. Exact slices handed to the image, data and helper decoders.

No allocation happens before step 10, and child allocation is bounded by the slice length.

**Arithmetic is all checked:**
- `plan_state_lengths` uses `checked_mul` and `checked_add`, with widths 13 and 5/2/5/2 and 7, which equal the parents' `section_len` and `table_offsets`.
- `offsets` uses `checked_add`.
- `lanes.len() - o[3]` only runs behind the `>` guard.
- `u(usize)` is lossless.

**Encode order:** plan → offsets → limit → image, data and helper encoders (each with its parent cap order) → a `ChildLengthDrift` self-check → one `with_capacity(o[3])`. `ChildLengthDrift` is unreachable under the frozen parents.

**Canonical domain.** Encoder and decoder accept the same domain:
- Every header lane is fully determined: constants, child lengths forced by each child's exact-length check, `ctx.ext_present`, 0, and `ctx.descriptor()`.
- Children are canonical by their parents' reviewed codecs.
- The limit and caps are applied identically on both sides.
- So `decode∘encode = id` on cap-admissible states, and `encode∘decode = id` on accepted lanes.
- Reordered and duplicated children fail as fixed-position tag errors. Overlap is impossible.
- Declared boundaries are used exactly as declared; a mismatch is refused by the child's own exact-length check and reported with its offset, never inferred.

**Allocation versus `max_total_lanes`.** The limit bounds the decode input and the encode output only. Other allocations are not bounded by it:
- `state_lanes_from_host` → `C6OwnedState::from_image` clones every image table *before* the limit check.
- `host_with_decoded_state` clones the whole host, including the `ProgramSnapshot`, after decoding.
- `compare_uncached_witness` builds two plans and runs three evaluations. These unfold DAG-shared values with no sharing (`decode::value`), copy environment chains (O(E²)), and can grow super-linearly in the host's data DAG.

None of this is a defect: every one of these is proportional to a host the caller already owns, and no doc says the limit bounds them. The `C6StateLimits` doc ("bound on the whole section") is accurate. It should be stated on the witness API (O4).

## 3. `replace_state_of` and the inverse

- The destructuring is exhaustive: `C6Root` (16 fields), `C6DataTables` (4), `C6HelperTables` (3) and `ImageSnapshot` (23 fields, `run_id: _`, no `..`). Adding a field anywhere becomes a compile error.
- `let C6DataTables { data, .. } = data;` shadowing is legal; the right-hand side binds before the pattern.
- Destructuring the `&mut ImageSnapshot` gives `ref mut` bindings, with no explicit `ref`/`&`, so it is edition-2024 clean.
- `SegBase` is rebuilt with the public `from_parts`.
- `admission` lives outside `ImageSnapshot`, so it is retained structurally. `run_id` is retained explicitly.
- Every full-u128 counter and origin, raw empty-slice start, order, multiplicity and Option presence survives. The tests exercise `MAX` values, `rest.start = 8` with len 0, `None` versus `Some(0)`, and task duplication.
- The inverse only operates on a caller-supplied clone. It never synthesizes a host, and no PRE-as-POST relabeling occurs.

## 4. Uncached witness integration (actual bodies traced)

**Call path.**
- `construct_plan_for` calls `decode::build`, which emits exactly 33 output fields in `OUTPUT_FIELDS` order. I counted 13 explicit fields plus a 20-entry array.
- Each field's node is freshly added (sequence, payload, environment or slice node), never a cached `Read`. So the 34 live-outs are distinct.
- `evaluate_uncached` calls `dag::evaluate`, which runs `validate` (definitions recomputed, `live_outs == fields + output`, exactly one `Read` per binding) and then one uncached pass.

**`check_live_outs` assumptions against the actual `dag::definitions`:**
- `value == producer == id`.
- `live_out` comes from `contains`.
- `lifetime == plan.lifetime`.
- Live-out `last_use_order == order.len()`.
- The count is compared against the distinct set, so duplicate or shared live-outs would still be counted consistently.

Every check is true by construction. Only the lifetime check adds anything beyond what `validate` already enforces, and that is the requested-lifetime check. The test literals `live_out_fields == 33` and `live_outs.len() == 34` match `OUTPUT_FIELDS`.

**What is actually compared.**
- `decoded.metadata` plus the 33 named field values, which are the live-out slot values copied by `field_values`, plus the diagnostics.
- Intermediate slots are not compared. The `Metadata` read node is not a live-out, but it is compared through `decoded.metadata`.
- No claim beyond this is made.

**This comparison is not independent evidence about the codec.** It is entailed by the host check:
- Once `first_host_difference` returns `None`, the rebuilt host is structurally equal to the original. That covers all 24 image fields, the admission, and the `snapshot` fallback. It relies on the custom `TypeExprSrc` equality, which is structural.
- `construct_plan_for` and `evaluate_uncached` are deterministic pure functions (BTreeMap/BTreeSet order, smallest-ready Kahn order). So plan equality, decoded/diagnostics equality, and cross-evaluation success (the `Read == immutable_value` checks) all follow from host equality.

The genuine mismatch detector is `HostMismatch`. The rest is a useful integration smoke test (plans build and evaluate on every trace state in all three lifetimes, and the live-out assumptions hold), not a second witness of preservation. The doc's "preservation under the existing implementation only" is accurate; O1 asks it to say this explicitly.

**Failure propagation** is ordered and exact: State → HostMismatch → OriginalPlan → DecodedPlan → PlanMismatch → LiveOut → Original/Decoded/Cross evaluation → EvaluationMismatch. Nothing is skipped or retried.

## 5. Normal API compile (source trace)

**Imports resolve with their actual visibility:**
- Core: `decode_image_section`, `encode_root_and_frames`, `C6CodecBudgets`, `C6PhysError`, `C6Root`, the magic/version constants, `FRAME_LANES`, `IMAGE_SECTION_HEADER_LANES` are all `pub`.
- 3A: `rd`, `rd_bool`, `u`, `zero` are `pub(crate)`; the rest are `pub`.
- 3B: all `pub`.
- `shared::host_implied_prefix` is a `pub mod`. `SegBase::from_parts` and `genesis_uncut` are `pub const`.
- `ExtCapacitiesC` is re-exported via `pub use`.
- The `witness_plan` items used (`NodeId`, `Operation::Snapshot`, all plan/definition fields, `WitnessEvaluation`, `WitnessFailure`) are `pub`.

**Derives are satisfiable:**
- `C6StateContext` is Copy because `C6StateCaps` and `C6StateLimits` are.
- `C6WitnessError` and `C6WitnessPreservation` need `PartialEq`/`Eq` on `WitnessFailure`, `WitnessPlan`, `WitnessEvaluation` and `LifetimeUnit`, which all derive them.

**Expressions and patterns resolve:**
- `use C6WitnessError as E` with `E::State` as a function and `E::HostMismatch { .. }` is valid.
- The generic `first<T>(&Vec)` deref-coerces correctly.
- `|&(same, _)|`, `|&i|` and `for &(k, v)` are top-level reference patterns, so edition-2024 rules are respected.
- Function-local `use C6StateChild::*` shadows nothing; all locals are lowercase.
- There are no dead private items.

**Test-side resolution:**
- `use super::*` imports the parent's private imports and functions (RFC 1560 globs). That covers `first_host_difference` and private context fields such as `c.limits` and `narrow.caps.phys`, which the child module can access.
- `LayoutSchemaC::clone` comes from the crate-wide `impl Clone` in `owner_prepare`.
- `admit_ext_program` is re-exported with signature `(ExtProgram, ExtDomain, &BoundsC, &LayoutSchemaC, Option<LocatorC>)`.
- `host_owner_engine().admit_run` needs `OwnerEngine` in scope, and it is imported. R3–R5 pass: 4096 ≤ 10⁶, 64 ≤ 64, 4096 ≤ 4096.
- `fixtures::{must, FUEL}` are `#[cfg(test)] pub(crate)`, and `must` places no bound on `E`.
- The `[(&str, fn(&mut C6OwnedState)); 29]` array literal coerces its non-capturing closures through the expected fn-pointer type.
- `tasks.push(tasks[0])` is accepted under two-phase borrows.
- `Err(_)` is used for `CaptureStepError`, which has no `Debug`.

## 6. The four tests, against the literal codecs (all UNRUN)

**Test 1: framing and first-error order.** I hand-traced every expectation and all of them match.

- **Directory and length:** 74/35/45 and total 195.
- **Header literal:** `[MAGIC, VERSION, STAT, 3, IMAG, 74, DATA, 35, HELP, 45, 1, 0]`.
- **Child placement:** the image child at 41..115, its frame 1 at 102..115, `L_INITIAL_FUEL == MAX`.
- **Limit:** 194 versus 195 on both encode and decode.
- **Header and directory errors:**
  - 40 lanes → `Truncated` (41/40).
  - Magic, version, and tag (`"state"`).
  - Child count 2.
  - The reordered and duplicate child tags.
  - Data length `u128::MAX` → `UsizeOverflow`.
  - Data length 6 → `ChildLengthRange` (6/7).
  - Ext lane 2 → `BadBool`, which precedes the reserved-lane error.
  - Reserved lane → `NonZeroPadding` at 11.
- **Context mismatches:**
  - Absent ext → mismatch at lane 10, which precedes the reserved-lane error.
  - `max_frames` → lane 12 (63/64).
  - `max_data_field_slots` → lane 39 (1/1024), while encoding reports `Child{Data, 115, state.data_fields 2/1}`.
- **Offset and length errors:**
  - Image length `usize::MAX` → offset overflow.
  - 75 → 196/195 `Truncated`, which precedes the frame body.
  - Trailing 1.
  - 87/22 → image `TrailingLanes` 13.
  - 61/48 → image `Truncated` 74/61.
- **Child body errors:**
  - Lanes 89 + 122 → the frame `BadTag` is reported first.
  - Lane 122 → `state.data` `BadTag` 7.
  - Lane 117 → `state_data` HELP.
  - Lane 154 → `state.helper` 12.
  - `stack_limit` 65 → `Child{Image, 41}`.

**Test 2: 29 mutations, external retention, nonzero SegBase.**
- All 29 mutated states pass the test caps.
- Every one of the 16 root fields, both frame properties and all seven tables is perturbed. Sub-fields are partial (one component each of `alloc`, `active` and the segment fields), but test 1's lane equality covers the layout.
- Empty tables give 127 lanes. `None` versus `Some(0)` is checked.
- The real-host part works as claimed:
  - genesis `Eval{0}` versus synthetic `Eval{5}` → `"image.control"`;
  - `admission` and `run_id` are retained;
  - `SegBase` is nonzero via `from_parts`;
  - restoring from the original image recovers the host exactly.
- The synthetic state is labelled unadmitted, and no admission rule is turned into codec acceptance.
- Weak spot: the "distinct framed bytes" comment for absent versus present-zero ext is not asserted here. Test 1 does exercise the mismatch.

**Test 3: owner trace and whole-host clone-replace.** It uses real `admit_ext_program`, `admit_run`, `capture` and `record_step`, with a context built from the retained original layout.
- Every boundary goes through encode → decode → re-encode → whole-host equality.
- `retained_layout_agrees_with_host` is checked positive and negative (the `"bounds"` mismatch), and the `run_id` mismatch is checked.
- The `before != after ⟺ host before != after` assertion is trivially true, since every event charges or publishes.
- My trace of `let p = mk(7, 8) in p` supports a nonempty `ArgsF.completed` (second argument), `ReturnF`, `ConstructorFieldsF.completed` (`MkPair` while evaluating `MkBox(y)`), Bind with `PairTask`, Append, Build, Scalar with `expected` set, a result, an env, and Constructor data. Halting is well within fuel 4096 and 4096 steps.

**Test 4: uncached witness, all three lifetimes.** It covers Snapshot every 8 steps, RecordBefore/RecordAfter on every event, and on the terminal step both records plus a Snapshot. It panics with the exact error and never skips. The literals 33 and 34 are correct.

None of the four tests uses `.or(actual)`, replaces an expected value with the actual one, or weakens a count.

## 7. Targeted ABSTAIN: runtime of tests 3 and 4

These outcomes depend on bodies not supplied in the packet:
- whether two-argument generic admission succeeds;
- whether a terminal Halted state is reached;
- whether each `Seen` flag is reached;
- whether `construct_plan_for` and `evaluate_uncached` succeed on every trace state.

The missing bodies are the `include!`d `fragments/legacy.proposed.rs` and `scalar.proposed.rs`, `prepare_ext_c1_to_p`, `owner_ext_prepare_types`, `nominal_declaration_order`, `ext_pass_b_walk`, `ext_source_traversal`, `owner_ext_c_slots`, and `witness_plan/adt_projection_tests.rs` as an existing run baseline. Core18 PASS does not establish these. The 3A+Core and normal-rlib 30 s STOPPED runs are not passes. Debug-build runtime of test 4 (2 plans and 3 evaluations per boundary) is unmeasured.

## 8. Findings

**Mandatory owner corrections: none.**

**Optional improvements, before ABI publication where noted:**

| ID | Recommendation |
|---|---|
| O1 | State in `compare_uncached_witness` that plan, evaluation and cross equality are entailed by whole-host equality. Optionally add a negative test, e.g. a too-tight context yielding `E::State(Child …)`. |
| O2 | Before ABI publication, decide whether the descriptor should bind codec budgets and caps the state section never reads (`max_type_expr_nodes`, `max_template_bytes`, `max_occurrences`, `max_occurrence_paths`, `max_patterns`, `max_pattern_children`, `max_ctor_fields`, `max_decl_templates`, `max_plan_cases`, `max_constant_fields`, `max_name_bytes`). Today, a change to a program-codec budget alone changes the state bytes. |
| O3 | Add a checked constructor (retained layout + host → context) that runs `retained_layout_agrees_with_host` first. |
| O4 | Document that `max_total_lanes` does not bound `from_image`/`host.clone()` or witness work. Optionally compute the directory from the image's counts before cloning. |
| O5 | The `Child.offset` alone cannot locate record-relative lanes in data/helper records. Diagnostic only. |
| O6 | Encode one state under absent versus present-zero ext and assert that only lane 10 differs. Add helper `ChildLengthRange` and image-minimum vectors. |
| O7 | `first_host_difference` duplicates Core's private copy. Comparing diagnostics would break spuriously if `measured_host_*` are ever populated. |

**Preserved obligations (not new policy):**
- 3B voter A **T1**: literal vectors for the other 9 helper kinds and 18 result kinds, required before ABI publication. T2–T7 remain optional.
- 3B voter B's limits on result/task/staged coverage and on true reachability. Equal, Match, Plan, EqualTask and most results remain synthetic-only. These tests do not show every variant is reachable or executed.

**Actual runtime obligations:**
- Whole `mori-core` lib and test-binary build.
- The 31 host tests (18 + 3 + 6 + 4) by exact filter.
- The real-SDK `c6_a135_push` backend test, run separately.

**Remaining full scope (OPEN):**
- The 41-lane state section, offsets and descriptor are not a bound public statement, native instruction or relation. `c6_lanes_to_immediates` yields operands only, and nothing connects this to A114 L4 or legacy `FrameSlots`.
- All 39 partition/selector/rule constraints, typed witness and export.
- Grouping 1/2/4/8 earliest-stop and free-publication equivalence, with run barriers.
- The finite immutable-key residual baseline and generic fallback.
- Chunk 4: full-host, program identity, checkpoint and membership binding.
- Chunk 5: native constraints, emitter, prover, ledger.
- Finance and persistence.

Nothing here is adopted, globally encoder-conformant, qualified, compiled, executed, or measured for performance.