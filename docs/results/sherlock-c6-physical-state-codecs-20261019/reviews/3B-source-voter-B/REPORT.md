# Chunk 3B review, voter B: `c6_phys_helper.rs` (63,237 B) and `mod.rs` (1,220 B)

This is a source-only review. I executed, compiled and edited nothing, and computed no hashes. The byte counts and SHA-256 values in the pins are supplied facts that I did not recompute. The +24 B `mod.rs` delta does check out arithmetically: 1196 + `"pub mod c6_phys_helper;\n"` = 1220.

## Verdicts

| Surface | Verdict |
|---|---|
| DESIGN | **AGREE**. Coherent with C6P1; profile adoption remains Root's decision. |
| SOURCE_SECURITY | **AGREE** |
| NORMAL_API_COMPILE | **AGREE**, by source trace only. The toolchain premises in §6 apply. |
| **Overall** | **AGREE** |

No owner corrections are required. The optional items are in §9.

---

## 1. Design decision on the proposed physical section

**The proposal is coherent with the existing profile.**

- **Header.** It reuses the C6P1 magic and version at lanes 0 and 1. It puts a section tag at lane 2, as Chunk 2 (`0x5052_4F47`) and 3A (`0x4441_5441`) do. `0x4845_4C50` ("HELP") is distinct from both.
- **Confusion with Chunk 1.** Chunk 1's lane 2 is the control tag, which is at most 10. A HELP stream fed to `decode_image_section` therefore fails with `BadTag{control}`. A Chunk 1 stream fed to `decode_helper_tables` fails the tag check.
- **Core is not modified.** No file under `c6_phys` is edited. The 48-lane root, the 17-lane control and the 13-lane frame layouts, all 10 frames, all 11 controls, `control_origin` and the frame stack are untouched.
- **Conventions carried over:**
  - RefC is `[tag, i]`.
  - Env and non-env options are `[0,0]` or `[1,v]`, the same shape as Chunk 1's `Enter.function`.
  - Slices are raw.
  - Enum tags are declaration ordinals.
  - Records are ascending, and padding lanes are reported with record-relative lane numbers.
  - The task stack is bottom-to-top and the cursor is derived, not stored, mirroring Chunk 1's frame count.
- **One new sub-encoding.** `Option<RefC>` is three lanes, `[presence, tag, i]`. It composes existing conventions and is unambiguous.
- **Widths are tight.** I checked them independently:
  - The Plan helper fills all 23 lanes.
  - `PlanMatched` fills all 4 result lanes.
  - `EqualTask` fills all 7 task lanes.
- **Approval scope.** This source approval does not adopt `HELP` or the 23/4/7 widths into the profile. Freezing those in C6P1 is Root's decision.

**Deliberate deviation (documented, acceptable):**
- Chunk 1 checks root structure and caps before the exact-length check.
- 3B follows 3A instead: count, then count cap, then length, then each record checked structurally and against caps in turn.
- 3C must declare the outer first-error order with this difference in view.

**Preservation** (all verified):
- `HelperCommon` keeps `site`, the full-u128 `origin_reference_cursor` and `parent_resume` as `[0,0]` or `[1,v]` with no cap.
- `ScalarStaged` is two lanes, and the `U128Value.n` payload is full width.
- `reg_a` is full width.
- Each phase and status enum has its own literal map. Same-named variants are not collapsed: `PlanPhase::Bind ≠ MatchPhase::Bind`, and `TriStatus` is shared only where `shared.rs` shares it.

## 2. Layout and inverse trace

| Record | Variant widths (lanes) |
|---|---|
| Helper (head = 5) | Scalar 17, EnvGet 12, EnvPut 13, Find 10, Bind 18, Append 15, Validate 8, Equal 10, Match 21, Plan 23, Build 18 |
| Result | Max 4 (`PlanMatched` = tag + arm + 2-lane env) |
| Task | Validate 5, Equal 7, Pair 5 |

**Order matches between encoder and decoder.** For every variant, the `Wr` write order and the `Rd` struct-literal field order match. Struct-literal fields evaluate in the order written, so lanes are read in ascending order. The field order also matches `shared.rs` declaration order for all 11 helper variants, 19 result variants and 3 task variants.

**Every enum map is exhaustive and in declaration order:**
- SiteTag (27 variants)
- ScalarMode, ScalarPhase, NodeKind, BoolSelector, TriStatus
- BindPhase, BindStatus, AppendPhase
- MatchPhase (8), PlanPhase (6), PlanAfterWalk, BuildPhase
- PatternTag (7), TaskVisitC

**Both directions of the inverse hold:**
- **decode then encode is the identity.** Every lane is fixed by one of: a tag, a checked 0/1 bool, a padding lane that must be zero, an option whose `None` payload must be zero, an opt_ref whose `None` must be `[0,0,0]`, a staged `None` whose payload must be 0, an exact usize via `rd`, a u8 via `rd_u8`, or a raw u128.
- **encode then decode is the identity** under caps, because the encoder and the section decoder call the same `helper_caps`, `result_caps` and `task_caps` functions.

**Strict distinctions survive:**
- `None` vs `Some(0)` encode as `[0,0]` vs `[1,0]`.
- `Const(0)` vs `Data(0)` encode as `[0,0]` vs `[1,0]`.
- `opt_ref` encodes `None`, `Some(Const(0))` and `Some(Data(0))` as `[0,0,0]`, `[1,0,0]` and `[1,1,0]`.
- Staged `None` is `[0,0]`, while `U128Value{0}` is `[1,0]`.
- An empty slice keeps its start: `(7,0) ≠ (0,0)`.

**No hostile input can cause a panic.**
- `Rd` indexes a fixed record at most 23, 4 or 7 lanes wide, as tabulated above.
- `staged` uses `at - 1`, which is safe because `at ≥ 2`.
- The `Wr` writes are statically bounded in the same way.

## 3. Caps traced against producer domains

Notation: **I** means an index check (value strictly below the cap). **C** means a count check (value at most the cap). Every row below was checked against the actual producer bodies: `environment.rs`, `structural.rs`, `adt_plan.rs`, `adt_build.rs`, `adt_constructor.rs` and `owner_adt_runtime`.

| Field | Cap | Producer domain | Verdict |
|---|---|---|---|
| `Find.row_cursor` | C `max_functions` | `FindEmpty` fires at `row_cursor == functions.len()`, which can equal `function_rows` | **C is required.** An I check would reject the reachable `FindEmpty` state. |
| `Find.found`, `FindFound` | I `max_functions` | Set from `row_cursor < len` | ✓ |
| `Append.src_cursor` | C `src.len` | `Tail` and `Done` have `src_cursor == src.len` | ✓ |
| `Build.src_cursor` | C `src.len` | `Done` has `src_cursor == src.len` | ✓ |
| `Plan.path_cursor` | C `path.len` | `plan_walk` advances to `path.len` | ✓ |
| `Bind.p_cursor`, `Bind.a_cursor` | C own slice `len` | `BindArity` fires when one cursor is at its end and the other is not | ✓ |
| `Append.dest_start`, `Build.field_start` | C `value_slots` / `data_field_slots` | Equal to `active.*` at entry. They can equal the cap before a later physical refusal. | ✓ C is correct |
| `Bind.params` | `max_parameters`, arity ≤ `max_arity` | V3 requires `params.len ≤ max_arity` | ✓ |
| `Bind.args`, `Append.src`, `Build.src` | `value_slots`, arity ≤ `max_arity` | ArgsF completed is at most `args.len − 1`. ConstructorFields completed is less than arity. Build uses exact arity. All are ≤ `max_arity`. | ✓ |
| `AppendDone.seq` | `value_slots`, arity ≤ `max_arity` | `src.len + 1 ≤ args.len` (or `fields.len`) `≤ max_arity` | ✓ |
| `Plan.path` | `max_field_paths`, no arity cap | The path is a depth. Only its elements are below `max_arity` (V13). | **Correct to omit the arity cap** |
| Raw empty starts | End must be ≤ table cap | Producer empties are `(0,0)` (normalized or `empty()`) | ✓ (raw starts are preserved) |
| `Match.arm`, `Plan.arm`, `PlanMatched.arm` | I `max_arms` | `arms.start` with `arms.len > 0`; the Select arm is in range. `arm` is unchanged through `NoMatchReady` and `Done`. | ✓ |
| `Match.x`, `Match.y` | I `max_names`, only past `Arm` and only when the tag binds the name | Entry sets `PVar`, `x = y = 0`. PNil uses 0/0. PVar and PLeaf use `y = 0`. | **The guard is required** (see the counterexample after this table) |
| EnvGet cursor, EnvPut `new_root`, Bind `dup_cursor`, `EnvUpdated.cell` | I `max_env_cells` | Env ids | ✓ |
| `Scalar.expected`, `PairTask.sort` | I `max(max_types, 4)` | ReturnF and ConstructorFields types, and the builtin parameter sorts | **Builtin cap required** (see the counterexample after this table) |
| `Build.type_id` | I plain `max_types`; `ctor_id` I `max_constructors` | `adt_checked_ctor_arity` rejects types below 4; ≥4 is `< types.len ≤ type_rows` | ✓ consistent with 3A Constructor rows |
| Task cursor | C `max_arity` | Reachable cursors satisfy `cursor < arity ≤ max_arity` (SI-22) | ✓ Looser than reachable, which is an acceptable superset |
| `parent_resume`, origin, `reg_a`, `U128Value.n`, u8 codes | No cap | Reserved or raw | ✓ Not capped against env cells, and not narrowed to scalar or modulus ranges |

**Counterexample for the `Match.x/y` guard.** Take `match [] { Nil => 0 }`, which has no names, so `name_rows = 0` is admissible. At the `Arm` phase, `x = 0`; at `PNil/Tag`, `x = y = 0`. Unconditional checks would reject both reachable states.

**Counterexample for the builtin TypeId cap.** Under a DedicatedSorts layout, `admit` and `check_program_view` cap types at 0, so `type_rows` can be 0. With `fn f(x: UInt)`, the `PairTask` sort is `TypeIdC(0)`, and the `RetCall` expected type is a builtin. A plain `max_types` check would reject both.

**Cap order follows lane order** in every variant. Caps choose a table and a bound only. They do not prove admission or membership, and the module says so.

## 4. First error, allocation and work

**Section decode** follows the documented steps 1 through 10:
- `rd` on the count, then the count cap, then a checked `section_len`, then `Truncated`, then `TrailingLanes`.
- Records are decoded only after that, each structurally and then against caps, in the order helper, result, tasks.
- Nothing is allocated before the length check. `with_capacity(n)` is bounded by `(len − 31) / 7`.

**Section encode** checks the task count, then the helper, result and task caps, then computes `section_len`, then allocates.

**Encoder and decoder agree on which error comes first.** For example, an excess task count is reported before a bad helper cap by both.

**Work is O(len).**

**Two surfaces with different contracts:**
- `encode_helper_record`, `encode_result_record`, `encode_task_record` and `decode_*_record` are structural only and apply no caps. This mirrors `encode_frame` and `decode_frame`.
- The `*_checked` decoders and the table and section APIs apply caps.

Both contracts are accurate.

## 5. Source security

- No `unsafe`.
- No panic paths on hostile input (§2).
- Integer conversions are checked: `u128` to `usize` via `rd`, `u128` to `u8` via `rd_u8` (`UsizeOverflow`), and slice ends via checked `add`.
- Nothing constructs `Admitted` or `MachineImage`.
- `replace_tables_of` writes exactly three fields of an `ImageSnapshot` the caller owns.
- `host_with_decoded_helper_tables` decodes before it clones.

**No security finding.**

## 6. Normal API and compile (source trace)

**Production imports** all resolve, and every one is used:
- The 3A items are all `pub(crate)` or `pub`: `u`, `rd`, `rd_bool`, `zero`, `put_ref`, `get_ref`, `put_env`, `get_env`, `cap_count`, `cap_index`, `cap_ref`, `cap_env` and `C6StateCaps`.
- From Core, `C6PhysError`, `C6Result`, and the magic and version constants are `pub`.
- `HostSnapshot`, `ImageSnapshot`, `adt::{TypeIdC, CtorIdC}` and all of the `shared` enums and carriers are `pub`.

**Local names do not collide.** The local `rd_u8` and `cap_slice` do not clash, because the Core copies are private to `c6_phys`.

**Macro and binding points:**
- The `lane_enum!` macro is defined before it is invoked.
- `$n:literal` is valid in both expression and pattern position.
- `match *h` on Copy enums binds fields by value, the same pattern Core uses.

**The four previously uninspected bodies, now checked against the supplied source:**

1. **`LayoutSchemaC`.** All 8 fields are `pub` and there is no `Default`, so the literal is complete. `ExtCapacitiesC` is the `shared` re-export, with `pub` fields and a `const fn all_zero`.
2. **`admit_ext_program`.** It is a `pub use` re-export with signature `(ExtProgram, ExtDomain, &BoundsC, &LayoutSchemaC, Option<LocatorC>) -> Result<(Admitted, PreparationReportC), ExtTypeBuildError>`.
   - AD-0a holds: `layout.bounds` is the same `bounds` value.
   - Widths are `None`.
   - `boundary_capacities_present` is `false`.
3. **`host_owner_engine().admit_run`.** `CheckedOwnerEngine::admit_run` calls `owner_prepare::admit_run` through `PrepareDeps`.
   - Finance is Absent.
   - R3 holds: 4096 ≤ 10⁶.
   - R4 holds: 64 ≤ 64.
   - R5 holds: 4096 is not greater than `value_slots` 4096.
   - It returns a valid genesis image.
   - `OwnerEngine` is imported, so the method resolves.
4. **Core fixtures.** `fixtures` is `#[cfg(test)] pub(crate)`. `must<T, E>` places no bound on `E`. `FUEL` is a `pub(crate) const u128`.

**Test-only imports:**
- `KIntC` resolves (`pub mod i128_class`).
- `ExtDomain::GENERIC_ABSENT` resolves.
- The T3 and type-builder source carriers are `pub`.
- `capture`, `record_step` and `ObservedStep` resolve.
- `DATA_SECTION_TAG` resolves.

**Visibility and imports:**
- `use HelperControlC as H;` after `use super::*;` is the same pattern as `structural.rs`'s `use RuleIdC as RuleId;`.
- Glob imports at block level (`use AppendPhase::*`, which shadows the prelude's `Copy` only in the value namespace; `MatchPhase::*` with `PatternTag::*`, which do not overlap; `PlanPhase::*`) shadow nothing used unqualified.
- The M1 lesson from 3A is applied: `exceeded<T>` is a generic `fn` item, not a closure.

**Premises** (not observed):
- Edition 2024 (workspace), which supplies:
  - array `IntoIterator`;
  - `TryFrom<&[T]> for &[T; N]` in the prelude;
  - `let-else`;
  - two-phase borrows for `tasks.push(tasks[0])`.
- No match-ergonomics reservation is triggered: `for &(k, v)` matches by move.
- No whole-crate or normal-Core build was observed.

## 7. The six tests, checked against the literal APIs

1. **`literal_enum_maps…`** All 15 maps are exhaustive, and each next-out-of-range tag yields `BadTag`.

2. **`all_variants…`** These are synthetic vectors and are labelled as such.
   - All 11 kinds, 27 sites and 3 `parent_resume` forms.
   - All 14 staged forms, including the full Scalar mode/phase/operand/expected cross product.
   - Bind 6×2, Append 3, Match 8×7, Plan 6×2.
   - All 19 result tags and every task variant.
   - Every vector passes test caps.
   - `every_lane_significant` is sound because decode is injective.

3. **`declared_lane_maps…`** I traced every literal vector by hand:
   - The Build helper (23 lanes).
   - The Validate prefix.
   - `PlanMatched` `[17,5,1,2]`.
   - `EqualTask` `[1,0,3,1,3,1,2]`.
   - `PairTask` `[2,4,3,1,2,0,0]`.
   - Offsets 4, 27 and 31+7k.
   - Section length 66.
   - The mutations are actually distinguished by the encoding.

4. **`refusals…`** Every expected error and lane matches the source:
   - Lengths 30/31 and 45/38.
   - The magic, version and tag checks.
   - `u128::MAX` gives `UsizeOverflow`; 65/64 gives `CapacityExceeded`.
   - Record lane 8 tag 2 gives `BadTag`.
   - Lane 8 set to 0 gives `NonZeroPadding{lane: 9, value: 1}`.
   - Lane 12 padding.
   - Structure before caps: lanes 5 and 11.
   - Helper before result: binder 257/256 is reported ahead of `BadBool`.
   - Task lane 3 `BadTag`, and lane 4 at 9/8.
   - u8 lanes 1/16/15 give `UsizeOverflow`.
   - Staged `None` padding at lane 16.
   - Encoder and decoder parity on the task count.

5. **`field_specific_caps…`** All expectations hold under the mutated caps:
   - binder 255/256.
   - Find row 64 is accepted and 65 is refused; found 65/64.
   - Non-env options do not use the env cap.
   - Plan path length 100 is accepted, and arm 65/64 is refused.
   - `parent_resume` set to `usize::MAX` is accepted.
   - Append cursor 4/3.
   - Bind args: overflow, then 9/8, then 1025/1024.
   - Match at `Arm` with `max_names = 0` is accepted, and at `Field` gives `x` 1/0.
   - Scalar and Pair at 3 are accepted and at 4 give 5/4; Build `type_id` gives 1/0.

6. **`owner_reachable…`** This test uses real generic admission with a retained layout and the actual capture and `record_step` APIs. Its caps come from `from_layout`, which keeps the ext caps. Nothing is reconstructed synthetically.
   - Every captured state goes through encode, decode, re-encode, whole-host replacement and the derived count check.
   - Mutating the origin bit, duplicating a task, or adding a result changes the lanes. Clearing other image state does not.
   - Clone-and-replace restores the host exactly.
   - There is no `.or(actual)` and no weakened expected value.

   **My trace of the program run** (`Let` → `ArgsF` → `Validate`+`ValidateTask` → `Append` → `Find` → `Bind`+`PairTask` → `ReturnF` → `ConstructorFieldsF` → `EnvGet` → `Scalar`(`TestType`, expected `Some`) → `Append` → `Build` → `Scalar`(`RetCall`) → `EnvPut` → `EnvGet` → publish → `Terminal`) supports the asserted 8 helper kinds, `ValidateTask` and `PairTask`, with `mutated` set at `Validate` and termination well within 4096 steps and the available fuel.

   These runtime claims are premises until the test is actually run.

## 8. Missing tests and gaps

None of these blocks approval of the codec. All of them are absent from the tests.

1. **Owner-reachable coverage.**
   - **Helpers:** Equal, Match and Plan.
   - **Tasks:** EqualTask.
   - **Results:** EnvMissing, FindMissing, BindBad, ScalarValue, ScalarError, ScalarSelector, EqualDone, MatchMatched, MatchNoMatch, PlanMatched, PlanNoMatch and `Validated{false}`.
   - **Staged forms:** all except `None` and `TypeTest`.
   - The owner's caveat names only Equal, Match, Plan, EqualTask and the Plan/Match results as synthetic-only, which understates the gap.
   - Test 6 does not record which result tags it saw.
   - Fixtures with `==`, a dedicated `match`, and a constructor `match` would cover most of this.
   - Synthetic vectors cannot establish reachability for any of the 39 rules.
2. **Negative caps for results:** `EnvUpdated.cell`, `FindFound`, the env field of BindOk/MatchMatched/PlanMatched, `PlanMatched.arm`, the three `AppendDone.seq` failure modes, and value refs.
3. **Negative caps for tasks:** `r`, `left`/`right`, `PairTask.binder` and `arg`.
4. **Negative caps for helpers.** Most fields have none. A swapped table, such as `max_parameters` against `max_value_slots`, would currently go undetected by the tests. I verified the table choices by trace instead.
5. **Exact-error tests** (currently checked only indirectly as "refuses or differs"):
   - result and task padding;
   - env-option `None` padding;
   - a ref tag of 2;
   - the `section_len` `UsizeOverflow` vector (`max_helper_tasks = usize::MAX`).
6. **No assertion that the structural decode applies no caps** at the record level. Core has the equivalent assertion.
7. **No reachable state under a tight layout,** for example a dedicated layout with `type_rows < 4` exercising the builtin cap.

## 9. Optional findings

| ID | Item |
|---|---|
| O1 | `from_image` and `helper_lanes_from_host` clone the task list before the caps run. This is a single copy (same as 3A's N2). |
| O2 | The record encoders' doc comments do not say "infallible; caps applied by the section layer", as Core's `encode_frame` does. |
| O3 | 3C needs a public section-length or header-peek API, because `section_len` is private. |
| O4 | Test 6 duplicates Core's bounds and layout literals because `fixtures::admitted_run` does not return the layout. This is a drift risk. |
| O5 | The local `rd_u8` duplicates Core's private copy. |
| O6 | The fixed widths leave no slack, so any added field requires a version bump. |
| O7 | Correct the caveat's reachability lists to match §8.1. |

## 10. Scope and what remains to qualify

**This verdict covers only** the representation codec over `helper`, `result` and `tasks`, plus the registration line.

**It is not:**
- admission or proof of membership;
- the SI-07, SI-08, SI-21 or SI-22 state-space checks;
- whole-host or full-state constraints;
- qualification or performance.

**The source is uncompiled and the tests are unrun.** No type-check or test pass is claimed.

**Separate records, not inputs to this verdict:**
- Root's Core18 actual PASS is a separate record.
- The combined 3A build timeout is not a pass.
- Backend1 / the real SDK remain unqualified.

**Still OPEN:**
- **3C:** outer framing and section lengths with their first-error order, binding of bounds, layout and ext caps, and the seven-field clone-and-replace with uncached witness equality.
- **Chunk 4:** whole-host and program binding, admission and membership.
- **Chunk 5:** native constraints.
- **Finance and persistence.**
- **The required-coverage closures** for all 39 rules: witness, constraint, export, grouping and residualization.
- **Profile adoption** of `HELP` and the 23/4/7 widths, which is Root's decision.