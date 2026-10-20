# Chunk 3B review (voter A): `c6_phys_helper.rs` 63237 B `069faf60…` plus `mod.rs` 1220 B `00b2037e…`

**Overall: AGREE.** I found no required owner correction. I ABSTAIN only on the owner-reachable test's runtime claims (§6).

I worked from source only. I compiled nothing, ran no tests, edited nothing, and recomputed no hashes. Pins and byte counts are taken as supplied. The registration delta is self-consistent: `pub mod c6_phys_helper;\n` is 24 bytes, and 1196 + 24 = 1220.

## 1. DESIGN: **AGREE**

The proposed physical section is coherent with C6P1 and does not rewrite Chunk 1. Adopting the HELP tag and the 23/4/7 widths as profile is a separate root/owner decision. This vote supports the proposal; it does not adopt it.

**Header.** The header is `[magic, version 1, 0x4845_4C50 "HELP", count]`. This follows the 3A precedent: same magic and version, with the tag at lane 2.
- HELP differs from DATA (`0x4441_5441`) and PROG (`0x5052_4F47`).
- A Chunk 1 image section has its control tag (≤10) at lane 2, so `decode_helper_tables` fails it with `BadTag`.
- That separation from Chunk 1 is incidental, not designed. Explicit outer framing remains 3C work.

**Record widths.** I recounted every variant's lane usage:

| Record | Lanes used per variant | Max |
|---|---|---|
| Helper | Scalar 17, EnvGet 12, EnvPut 13, Find 10, Bind 18, Append 15, Validate 8, Equal 10, Match 21, Plan 23, Build 18 | 23 (Plan) |
| Result | 4 max (PlanMatched: tag, arm, env×2) | 4 |
| Task | 7 max (EqualTask) | 7 |

No reader can index past its fixed array.

**Field order.** Encode and decode follow `shared.rs` field declaration order for all 11 helpers, 19 results and 3 tasks; I checked each.

**Tag numbering.**
- Helper tags equal the `HelperKind` ordinal + 1.
- Result tags equal the `HelperResultTagC` ordinal + 1.
- `ScalarStaged` tags 0..=6, task tags 0..=2, and the `SiteTag`, `MatchPhase` and `TaskVisit` maps are prefix-identical to the frozen raw-104 schema ordinals.

**Conventions shared with Chunk 1 and 3A.**
- `RefC` is `[0|1, i]`.
- Env and optional index are `[0,0]` / `[1,v]` with a zero-padding check.
- Slices are raw `[start, len]`.
- u8 values above 255 give `UsizeOverflow`.
- Counters and origins are full u128 and uncapped.

**New conventions in 3B.**
- `Option<RefC>` takes three lanes, `[present, reftag, i]`, with None requiring two zero lanes.
- Task tags start at 0 because a task is never absent; helper and result tags start at 1 because 0 means None. This asymmetry is justified.

**Preserved distinctly, never collapsed:**
- `HelperCommon.site`, `origin_reference_cursor` (full u128) and `parent_resume`.
- `parent_resume` uses env-style encoding but is never passed to `cap_env`; the field-caps test asserts `Some(usize::MAX)` is accepted.
- `reg_a` and `U128Value.n` (full u128).
- Every phase, status and enum value in its own lane.

**Tasks and task cursor.** Tasks are stored bottom-to-top with `top_task = tasks.last()`, matching the frame convention. No task cursor is stored: `StepEventC.task_cursor_*` equals `tasks.len()`, so the count lane is the only source.

The section does not touch the Chunk 1 13/17/48 layout, the 10 frames, the 11 controls, origins or the raw stack.

## 2. SOURCE_SECURITY: **APPROVE** (representation codec only)

**Inverse in both directions.**
- Every lane is either fully determined by the decoded value, or required to be zero.
- Specifically: None helper/result means all lanes zero; Option None pads; staged None pads its payload; bools are 0/1; every enum is a closed literal map; u8 fields go through `rd_u8`.
- So `decode∘encode = id` on tables that pass caps, and `encode∘decode = id` on accepted lanes.
- `Rd` reads ascending lanes. Rust evaluates struct-literal fields in source order, so lane order is guaranteed. `NonZeroPadding.lane` is record-relative, as in 3A.

**Decode first-error order** matches the module doc, steps 1–10:
1. Under 31 lanes: `Truncated`.
2. Magic.
3. Version.
4. Tag.
5. Count via `rd`.
6. Count against `max_helper_tasks`.
7. Checked `section_len` (`UsizeOverflow`), then `Truncated`, then `TrailingLanes`.
8. Helper record: structure, then caps.
9. Result record: structure, then caps.
10. Tasks in order.

**Allocation.** There is no allocation before the exact-length check. `with_capacity(n)` is bounded both by the cap and by the input length.

**Encoder.** It checks count, helper, result and task caps before `with_capacity`. Its order matches the decoder's.

**Panics and work.** No unsafe code. No arithmetic is unchecked, and all slice-end math is checked. No input can cause a panic. Work is O(n).

**Authority.** No `Admitted` or `MachineImage` is constructed. `replace_tables_of` and `host_with_decoded_helper_tables` touch only the three fields of a caller-owned clone.

**Caps are representation-only.** They prove neither membership nor admission. Bad site/kind combinations, SI-08/09/21/22 violations, and a present `parent_resume` all decode by design; that is admission work (Chunk 4).

## 3. Per-field caps against producer domains

I traced every field against its producer in the supplied family and owner code. All are correct.

| Field | Cap | Producer justification |
|---|---|---|
| Find.row_cursor | count ≤ max_functions | FindEmpty sits at `n = functions.len() ≤ function_rows` |
| Find.found, FindFound | index | — |
| EnvGet.cursor, EnvPut.new_root, Bind.dup_cursor, EnvUpdated.cell | index < max_env_cells | Equivalent to env IDs; `new_root = pre.active.env`, and a cap refusal blocks any write |
| Bind.params | parameter table, len ≤ max_arity | V3 |
| Bind.args, Append.src, Build.src | value slots, len ≤ max_arity | — |
| AppendDone.seq | value slots, len ≤ max_arity | len = completed.len + 1 ≤ call/ctor arity ≤ max_arity |
| Plan.path | `max_field_paths`, no arity cap | V13 does not arity-cap path length |
| Bind.p_cursor, a_cursor | count against **own** slice | Correct when params.len ≠ args.len (Arity/Propagate) |
| Append.src_cursor, Build.src_cursor, Plan.path_cursor | count against own slice | Exhausted cursor sits at len |
| Append.dest_start | count | = active.value_slots |
| Build.field_start | count | Entry may sit exactly at the cap |
| Match.arm | index < max_arms | ArmId from `arms.start` with len > 0 |
| Match.x / y | names only when past Arm and the tag binds them | Identical to the owner's own SC-05 rule in `check_written_code_ids`. PVar's y, PNil's x/y, Arm-phase zeros and reserved tags stay raw |
| Scalar.expected, PairTask.sort | `max(max_types, 4)` | Builtin sorts exist under DedicatedSorts, where `type_rows` may be < 4 |
| Build.type_id | plain `max_types` | Sound: generic admission forces 4 ≤ id < types.len() ≤ type_rows. See NB1 |
| Task cursors | count ≤ max_arity | Producers keep cursor < arity; inclusive cap is permissive but sound (NB3) |
| parent_resume, origin, reg_a, U128Value.n, u8 codes | uncapped | No narrowing to scalar or modulus |

## 4. NORMAL_API_COMPILE: **APPROVE** (source trace, not an observed compile)

**Toolchain premise.** The workspace sets edition **2024** and rust-version 1.99, not the edition 2021 assumed in earlier 3A reviews. The code uses let-else, const-generic arrays, by-value array `IntoIterator` and prelude `TryInto`. I found nothing sensitive to the 2024 edition: no `gen` identifiers, no `expr` fragments, and the tail-temporary uses are copies only.

**Imports.**
- From 3A, all `pub(crate)`: `cap_count`, `cap_env`, `cap_index`, `cap_ref`, `get_env`, `get_ref`, `put_env`, `put_ref`, `rd`, `rd_bool`, `u`, `zero`, `C6StateCaps`.
- From Core, all `pub`: `C6PhysError`, `C6Result`, `C6_PHYS_MAGIC`, `C6_PHYS_VERSION`.
- `rd_u8` and `cap_slice` are local definitions with no name clash.
- Every `shared` enum and variant exists with exactly the listed names.

**`lane_enum!`.** The generated matches are exhaustive. `$ty::$v` with an ident fragment is valid. `Ok(BindStatus::Ok)` resolves `Ok` to `Result::Ok`. Every generated function is used, so there are no `dead_code` warnings.

**Test resolution.**
- `use super::*` exposes the parent's private items and imports, the same mechanism 3A relies on. `use HelperControlC as H;` and the block-scoped glob uses (`ScalarMode::*`, `MatchPhase::*` with `PatternTag::*`, etc.) resolve.
- No glob variant shadows a pattern binding; every binding is lowercase.
- `Copy`, `Bind` and `Ok` are never ambiguous: `BindStatus` is never glob-imported, and `H::Bind` is always qualified.

**Fixture APIs, now with their bodies supplied:**
- `LayoutSchemaC`: 8 pub fields; `widths: None`.
- `ExtCapacitiesC { .., ..all_zero() }`.
- `admit_ext_program` is a `pub use` re-export with signature `(ExtProgram, ExtDomain, &BoundsC, &LayoutSchemaC, Option<LocatorC>) -> Result<(Admitted, PreparationReportC), ExtTypeBuildError>`.
- `host_owner_engine() -> impl OwnerEngine`. `admit_run(&Admitted, &RunIntC, &RunIntC, &FinanceInput)` forwards to `owner_prepare::admit_run`, and R1–R6 pass with fuel 4096 ≤ 1e6 ≤ value_slots 4096 and stack 64 ≤ frames 64.
- `fixtures::{must, FUEL}` are `#[cfg(test)] pub(crate)`; `must` places no bound on `E`.
- `ExtProgram`, `ExtFunction`, `ExtParamSrc`, `ExtExpr::{Construct, Call, Let, Var, Literal}`, `CtorDeclSrc`, `CtorFieldSrc`, `TypeDeclSrc` and `TypeExprSrc` are all used with their actual fields.
- `C6StateCaps::from_layout` and `SliceC::empty` are pub.

**Hand-traced expectations.** All match:
- Build lane map `[11,25,MAX,1,3,5,6,2,2,2,14,0,0,0,1,1,9,1,0×5]`; Validate prefix; PlanMatched `[17,5,1,2]`; EqualTask `[1,0,3,1,3,1,2]`; PairTask `[2,4,3,1,2,0,0]`; section length 66 and 38.
- Every refusal vector:
  - (12, 0) gives `NonZeroPadding` lane 9.
  - (16, 1) gives lane 12.
  - (9, 256) with (28, 2) gives binder 257/256 before the result's `BadBool`.
  - (34, 2) gives a task `BadTag`.
  - (35, 9) gives 9/8.
  - The u8 vectors and the staged-None padding at lane 16.
- Every field-caps vector, including overflow → arity → end ordering and the max(0,4) = 4 cases.
- `every_lane_significant`: XOR 0x55 never produces an alias in any variant.
- The M1 lesson is applied: `exceeded` is a generic `fn`, not a closure.
- `v.push(v[0])` is accepted under two-phase borrows.

## 5. Tests (all six UNRUN)

1. **`literal_enum_maps_…`** Real exhaustiveness and declaration-order check. Meaningful.
2. **`all_variants_…`** Labelled SYNTHETIC. Covers all 11/19/3 kinds, every phase/status, all 14 staged values, None/Some(0), Const/Data, raw empty start and u128::MAX. It cannot establish owner reachability, and does not claim to.
3. **`declared_lane_maps_…`** Freezes literal offsets for Build, the Validate prefix, one result and two tasks only (see T1). Asserts section inverse and order/multiplicity sensitivity.
4. **`refusals_…`** Covers the full first-error order, structure-before-caps precedence and encoder parity on count.
5. **`field_specific_caps_…`** Covers inclusive vs strict caps, non-env options, path-without-arity, the Match placeholder rule and the builtin vs user TypeId policy.
6. **`owner_reachable_…`** Assessed below.

**Test 6 design is sound.** It uses real generic admission, real `capture` and real `record_step` with no synthetic host or image, and takes caps from the retained original layout and ext caps. It does whole-host clone-replace equality with no `.or(actual)` and no weakened expectations. It checks that mutations are visible and that other image state is excluded. The terminal flag, helper reachability (Scalar with `expected`, EnvGet, EnvPut, Find, Bind, Append, Validate, Build) and task reachability (ValidateTask, PairTask) are plausible for `let p = mk(7) in p` by my trace of the family and owner bodies. Equal, Match, Plan, EqualTask and most results are correctly left to the synthetic test.

**Missing tests (optional; T1 recommended before adoption):**
- **T1.** Literal lane vectors for the other 9 helper kinds and 18 result kinds. Encode and decode share one author, so a mirrored field swap would pass roundtrip; only literal vectors freeze the ABI. I verified the order by reading, but this is the only check that pins it.
- **T2.** Checked refusals for result and task fields: EnvUpdated.cell, FindFound, AppendDone.seq (overflow/arity/end), PlanMatched.arm/env, BindOk/MatchMatched env, Validate/Equal task refs, PairTask binder/arg. Also encoder parity on per-field caps.
- **T3.** A `section_len` overflow vector: with `max_helper_tasks = usize::MAX` and the count lane at `usize::MAX`, the result should be `UsizeOverflow{state_helper}` before `Truncated`.
- **T4.** Cross-section confusion: PROG and Chunk 1 sections fed to `decode_helper_tables`, and HELP fed to the 3A/Chunk 1 decoders.
- **T5.** A record-level structural-vs-checked contrast: `decode_helper_record` accepts an over-cap binder that the checked decoder refuses.
- **T6.** In test 6, bind each event's `task_cursor_before/after` to lane 3 of the before/after sections and `result_tag` to the decoded result. Also assert that `host.admission.bounds` and `host.admission.layout.capacities` equal the retained layout's.
- **T7.** Owner-reachable fixtures for Equal, Match, Plan, and the remaining results. Required before any reachable-all claim; not required for this codec.

## 6. Targeted ABSTAIN: test-6 runtime claims only

Whether admission succeeds, the terminal state is reached within 4096 iterations, and the listed helpers and tasks are actually reached depends on bodies not in the packet:
- `families/fragments/legacy.proposed.rs` and `fragments/scalar.proposed.rs` (`include!`d; the legacy, Let, Scope and TestType rows).
- `owner_ext_c_expression::prepare_ext_c1_to_p`.
- `owner_ext_prepare_types::{drain_pending_keys, instantiate_declared_types}`.
- `nominal_declaration_order`, `ext_pass_b_walk`, `ext_source_traversal`, `owner_ext_c_slots`.

Core18 PASS does not establish terminal reachability for generic fixtures: the Core push test asserts no terminal, and the 3A terminal test ran in a build that timed out. This abstention covers runtime behavior only, not compile.

## 7. Findings

**Required: none.**

**Nonblocking:**

| ID | Finding |
|---|---|
| NB1 | `Build.type_id` uses plain `max_types`, while Chunk 1 caps the identical value at `Materialize.type_id` (SI-21 requires equality) with `max(max_types,4)`. The doc's "as 3A Constructor" rationale omits that Chunk 1 chose the other policy for the same user-nominal field. The two diverge only when `type_rows < 4`, which is unreachable for Build. Record this for 3C. Aligning either way is optional. |
| NB2 | `helper_lanes_from_host` clones the task list before checking caps (same as 3A N2). |
| NB3 | Task cursor caps are an inclusive count while producers keep cursor < arity. Permissive but sound, and the doc states it accurately. |
| NB4 | `rd_u8` and `cap_slice` duplicate private Core copies, a drift risk (3A N7). 3A does not export them, so this is acceptable now. |
| NB5 | The section labels `state_helper` and `state.helper` are inconsistent (3A N4 pattern). |
| NB6 | `l[16] = 0;` in the refusals test is redundant. |
| NB7 | Caps are a caller assertion. `Plan.path` depends on ext `max_field_paths`, which `LayoutSnapshot` does not carry (U2), so caps cannot be derived from a `HostSnapshot`. 3C must expose a public section-length or prefix API, since `section_len` is private and decoding is exact-length. |

## 8. Scope and what remains open

- **This vote covers:** the helper/result/task representation codec source, its API and its design coherence.
- **It does not cover:** typecheck, the six tests passing, admission, whole-host behavior, full-state constraints, qualification or performance. Backend1 and the real SDK remain separately unqualified.
- **Still OPEN:**
  - 3C: outer framing, section lengths, layout/ext cap binding, and seven-field clone-replace with uncached witness equality.
  - Chunk 4: host/program/admission and membership binding.
  - Chunk 5: native constraints.
  - Finance and persistent scope.
  - All-39 witness, constraint, export, grouping and residual closures.