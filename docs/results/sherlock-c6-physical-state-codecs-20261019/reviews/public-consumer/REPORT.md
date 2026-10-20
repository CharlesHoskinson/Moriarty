# Fresh non-author review: public 3A external consumer (source and public API only)

I read the inline packet only. I built, ran and imported nothing, and computed no hashes. Byte counts and SHA-256 values below are the supplied facts, which I did not recompute. Nothing here approves any compile, run or other operation.

## 0. Pins

| Item | Bytes | SHA-256 | Coverage I read |
|---|---|---|---|
| New consumer `crates/mori-core/tests/sherlock_c6_phys_data_public_consumer.rs` | 20,285 | `ec6760e877263de69d7cc0f7247bef3d6d2fc78316cb183403a69d12df5e0011` | Whole file (the single fence in the author response) |
| Production 3A `c6_phys_data.rs`, including the original 3 tests | 28,329 | `804173e01f2316c7786735ee959bf235fa235b67a914af124907a8c00bbd8bb4` | Whole file |
| Core `c6_phys.rs` | 97,378 | `98941c4a592a9a58e59ee862c72f89e931cdbb74d7fbb91fbd11a8ac37c6fb17` | Excerpts only: `fixtures` `[58226,62965)`; public consts, errors, caps and budgets `[2771,9706)` |
| `helper_machine/mod.rs` registration | 1,196 | `ddc59431…0862` | Whole file; unchanged |
| `lib.rs`, `sherlock/mod.rs`, `shared.rs`, `complete_observation.rs`, `owner_host.rs`, `owner_ext_source_program.rs`, `owner_ext_t3_uses.rs`, `owner_ext_type_builder.rs`, `owner_ext_admit.rs`, `adt.rs` | as supplied | as supplied | Whole files |
| `PROGRAM_SECTION_TAG` | 51 | `189b460c…` (from `f4d4ae58…`) | The constant line only |
| Author final response | 27,775 | `ccbd150f…ebff` | Whole |

## 1. SOURCE_SECURITY: **APPROVE**

Scope: the new test-only file and its public-API reach. This approval covers nothing else.

**Production stays untouched.**
- The file is a separate `tests/` crate. It cannot alter the production rlib, the 3A file, the Core file, the registration or Cargo.
- The inverse is removal of this one file.

**The consumer gains no authority beyond the public API.**
- `Admitted` (private `body`) and `MachineImage` (all fields private) cannot be constructed outside the crate, and the compiler enforces this. The consumer only receives them as owned values from `admit_ext_program` and `host_owner_engine().admit_run`.
- Image mutation happens only through the public `record_step`, which calls the fixed owner's `step`.
- `replace_tables_of` writes only to a caller-owned `HostSnapshot` clone. It never touches a `MachineImage`.

**No codec, owner or evaluator logic is reimplemented.**
- Every encode, decode, cap, capture and step call goes to the actual public `mori_core` item.
- `SECTION` and `DATA_S` appear only on the expected side of assertions. They are never passed to production code.

**No dangerous operations.** The file has no `unsafe`, I/O, environment access, process spawning, network access, `include!` or `#[path]`. Its only failure modes are panics and assertion failures.

**No introduced mandatory security finding.**

## 2. NORMAL_API_COMPILE: **APPROVE**, with one narrow ABSTAIN on Cargo linkage

This is a source trace against the supplied declarations. It is not an observed compile.

### Module chain and normal-build (`cfg(test)` = false) availability

**Public module chain.** Every link is `pub` and none is cfg-gated:
- `lib.rs` declares `pub mod sherlock`.
- `sherlock/mod.rs` declares `pub mod helper_machine`.
- `helper_machine/mod.rs` declares `pub mod` for `c6_phys`, `c6_phys_program`, `c6_phys_data`, `complete_observation` and `shared`.
- `shared` declares `pub mod` for `adt`, `owner_ext_source_program`, `owner_ext_t3_uses`, `owner_ext_type_builder` and `owner_host`, plus the inline `pub mod i128_class`.
- `shared` re-exports `ExtCapacitiesC` and `admit_ext_program` with `pub use owner_ext_admit::{…}`.

**Items whose normal-build presence follows from unconditional production use.**
- **Used by non-test code in `c6_phys_data`:** `C6PhysError`, `C6Result`, `C6PhysCaps`, `C6CodecBudgets`, `C6_PHYS_MAGIC` and `C6_PHYS_VERSION`.
- **`C6PhysCaps::test_caps`:** no `cfg` appears inside the excerpt, and the non-test `C6StateCaps::test_caps` calls it.
- **Whole-file reads, nothing cfg-gated:** all `c6_phys_data` public items, all `complete_observation` items, `owner_host::host_owner_engine`, and the source carriers.

**Cfg-gated item that is correctly not used.** `c6_phys::fixtures` is `#[cfg(test)] pub(crate)`, so it is unreachable from this crate. The consumer does not reference it.

### Import resolution
- **All crate-root imports resolve to `pub` items.** Every import is used, so `unused_imports` should not fire.
- **All `fixtures` imports resolve.** These are `KIntC`, `ExtDomain`, `ExtProgram`, `ExtExpr`, `ExtFunction`, `ExtParamSrc`, `ExtValue`, `CtorDeclSrc`, `CtorFieldSrc`, `TypeDeclSrc`, `TypeExprSrc`, `host_owner_engine`, `Admitted`, `BoundsC`, `CoreValue`, `ExtCapacitiesC`, `FinanceInput`, `LayoutCapacitiesC`, `LayoutSchemaC`, `MachineImage`, `OwnerEngine`, `RunIntC` and `admit_ext_program`. Each one is used.
- **No dead code.** The consumer uses `FUEL`, `must`, `lit`, `box_uint`, `nominal_program`, `admitted_run`, `caps`, `f`, `all_variants`, `every_lane_significant`, `SECTION` and `DATA_S`.

### Carriers and struct literals across the crate boundary
- **Struct-update syntax across crates is legal.** It requires all fields to be visible, which holds here:
  - `BoundsC` has 22 fields, all `pub`. The fixture lists all 22 by name, and test 2 uses `..BoundsC::default()` (derived `Default`).
  - `LayoutCapacitiesC` has 4 `pub` fields and derives `Default`.
  - `ExtCapacitiesC` has 16 `pub` fields and uses `..all_zero()`.
- **`LayoutSchemaC`:** all 8 fields are `pub` and the fixture names all 8. `widths: None` needs no `WidthTable` constructor.
- **Source carriers:** the struct literals for `ExtProgram`, `ExtFunction`, `ExtParamSrc`, `TypeDeclSrc`, `CtorDeclSrc` and `CtorFieldSrc` set only `pub` fields.
  - `Vec::new()` infers the element types of `assets`, `states` and `inputs`.
  - `.into()` infers `String`.
- **Newtypes and caps:**
  - `TypeIdC(pub usize)` and `CtorIdC(pub usize)` can be constructed.
  - `C6StateCaps` and `C6CodecBudgets` have `pub` fields.
  - `C6StateCaps` derives `Copy`, so reusing `caps` after `let mut tight = caps;` is legal.
- **Drop restrictions.** Only `TypeExprSrc`, `ExtExpr`, `ExtValue` and `ExtPattern` implement `Drop`. The consumer only builds these and moves them whole; it never moves out of a field by pattern, so E0509 cannot arise.
  - The author's claim that "Drop impls on these" covers `ExtProgram` and the `*Src` structs is imprecise but harmless (NB3).

### Owner calls
- **`host_owner_engine()`** returns `impl OwnerEngine` with no captured lifetimes.
  - `.admit_run(&Admitted, &RunIntC, &RunIntC, &FinanceInput)` resolves because `OwnerEngine` is in scope. It is the only in-scope trait with an `admit_run` method.
  - It returns `Result<MachineImage, Tooling>`, and `must` places no bound on `E`.
  - The `&RunIntC::Nat(..)` temporaries live until the end of the statement.
- **`admit_ext_program(ExtProgram, ExtDomain, &BoundsC, &LayoutSchemaC, Option<LocatorC>)`** returns `Result<(Admitted, PreparationReportC), ExtTypeBuildError>`.
  - `None` infers.
  - `ExtDomain::GENERIC_ABSENT` is a `pub const`.
  - AD-0a holds by construction, because `layout.bounds` is the same `Copy` value as `bounds`.
- **`record_step`** returns `Result<ObservedStep, CaptureStepError>`.
  - `ObservedStep` has exactly two variants, `Event` and `Terminal { .. }`, and is not `#[non_exhaustive]`, so the match is exhaustive.
  - `Err(_)` needs no `Debug`.
  - `image` is bound `mut`.
- **`capture`** returns `Result<HostSnapshot, HostViolation>` and is used through `must`.

### Observation fields used by test 3
- **Admission side:**
  - `host.admission.bounds` is a `BoundsC`.
  - `host.admission.layout` provides `capacities`, `bounds` and `layout_version: (String, String)`.
- **Image side:**
  - `host.image` provides `data`, `data_fields`, `env`, `value_slots`, `alloc.data_nodes`, `active.data_fields`, `reference_cursor: u128`, `frames` and `result: Option<HelperResultC>`.
  - All of these are `pub`, and `HelperResultC::EnvMissing` exists.
- **Derives:** `HostSnapshot` derives `Clone`, `Debug`, `PartialEq` and `Eq`. Inside Core, the `TypeDeclSrc` equality and debug output come from manual `impl`s in `complete_observation`.

### Generic and inference points
- **M1 `fn cap<T>`.** It is a block item at the crate root. It resolves `C6Result` and `C6PhysError` through the crate-root `use`, because block items see module-scope names.
  - The crate root, its imports and the test block contain no other `cap`.
  - `T` is fresh at each call and unifies with the left operand's `Ok` type through the derived `PartialEq` for `Result<T, E>`.
- **All 12 `cap(...)` uses re-enumerated.** Nine infer `T = C6DataTables` (the decode side) and three infer `T = Vec<u128>` (the encode side):

  | # | Expression | Inferred `T` |
  |---|---|---|
  | 1 | `dec(set(3, 257))` | `C6DataTables` |
  | 2 | `dec(set((48, 64), (88, 7)))` | `C6DataTables` |
  | 3 | `dec(set(49, 256))` | `C6DataTables` |
  | 4 | `dec(set(51, 9))` | `C6DataTables` |
  | 5 | `dec(set(50, 1022))` | `C6DataTables` |
  | 6 | `dec(set(73, 256))` | `C6DataTables` |
  | 7 | `dec(set(82, 256))` | `C6DataTables` |
  | 8 | `encode_data_tables(one(Data(4)), c)` | `Vec<u128>` |
  | 9 | `decode_data_tables(l, c)` | `C6DataTables` |
  | 10 | `encode_data_tables(t, tight)` | `Vec<u128>` |
  | 11 | `decode_data_tables(lanes, tight)` | `C6DataTables` |
  | 12 | `encode_data_tables(wide, caps())` | `Vec<u128>` |

- **`every_lane_significant`.** `decode_data_row` and `decode_env_row` coerce to `fn(&[u128; N]) -> C6Result<T>` (`N` = 5 for both). The `decode_ref_row` closure is non-capturing and coerces with `N = 2`.
- **Slice comparisons.** Comparing `lanes[a..b]` with `[_; k]` uses `impl PartialEq<[B; N]> for [A]`.
- **Shadowing in test 3.** `let caps = C6StateCaps::from_parts(…)` shadows the crate function `caps()` only from that point on. The `[ext+]` lines then read the local value, as intended.
- **Shadowing in test 2.** No local is named `caps`, so the `[ext+]` calls `caps()` reach the crate function.
- **Borrows and moves** are unchanged from the originals.
  - `set`, `dec` and `differs` take only shared borrows of `lanes`.
  - `t` moves into `wide` (test 2) and into the final `m` (test 1) after its last use.
  - The `[ext+]` lines use only `caps()`, the local `caps`, and `host`. They take no new mutable borrows and perform no moves.

### Narrow ABSTAIN: Cargo linkage
`Cargo.toml` is not in the readset. Three things therefore cannot be checked:
- that the lib target name is `mori_core` (no `[lib] name` override);
- that `tests/*.rs` auto-discovery is enabled, or that a matching `[[test]]` entry exists, with no `required-features`;
- which edition applies.

I do not guess these. The rest of the trace is independent of them.

## 3. Predicate, fixture and assertion preservation

### Test bodies, compared line by line against the original 3A bodies

| Original → external | `assert_eq` | `assert` | `assert_ne` | Result |
|---|---|---|---|---|
| `every_lane_significant` (helper) | – | 1 / 1 | – | Verbatim |
| `every_table_variant_…` → `public3a_…` | 11 / 11 | 0 / 0 | 1 / 1 (in `differs`, called 5 times) | Verbatim: same lane vectors, messages and five mutations |
| `refusals_follow_…` → `public3a_…` | 30 / 30 + 2 `[ext+]` | 1 / 1 | 0 / 0 | Verbatim through the `from_parts` equality |
| `captured_nominal_host_…` → `public3a_…` | 6 / 6 + 4 `[ext+]` | 5 / 5 | 2 / 2 | Verbatim through the `n - 1` cap assertion |
| **Total** | **47 / 47 + 6** | **7 / 7** | **3 / 3** | |

**What is preserved:**
- the same strict error variants and field values, including `Truncated` 7/6 and 101/96, `TrailingLanes` 1, `BadMagic`, `BadVersion`, `BadTag`, `BadBool`, `NonZeroPadding` at lanes 1, 2 and 4, and `UsizeOverflow` for `u128::MAX` and `usize::MAX`;
- the cap, overflow and first-error inputs at the identical lane indices;
- `one`, `c`, `tight` and `wide`;
- the exact `from_parts` mapping;
- the `0..4096` fixed-owner loop with the terminal break and `Err(_) => panic!`;
- the Constructor, Const, Data and nonempty env/value-slot predicates;
- the whole-host clone-replace `assert_ne!`/`assert_eq!` pair;
- the counter and preserved-field block (`alloc`, `active`, `reference_cursor`, `frames`, `result`);
- the Const↔Data flip;
- the `n - 1` cap.

**Order of failures.** All six `[ext+]` assertions come after the last original assertion in their test. They cannot change which original assertion fails first.

**No weakening or seeding.** No original expected value is replaced with, or derived from, implementation output beyond what the originals already did.

**The only adaptations are textual:**
- explicit `mori_core::…` paths instead of `use super::*` and `crate::…`;
- local expected-only `SECTION = "state_data"` and `DATA_S = "state.data"`, byte-equal to the private production consts;
- crate-root placement instead of `mod tests`;
- the `public3a_` name prefix.

### Fixture, compared with the Core `98941` `fixtures` block
`FUEL = 4096`, `must`, `lit`, `box_uint`, `nominal_program` and `admitted_run` match value for value. That covers:
- the `Box<A>`/`Pair` declarations;
- `mk`'s parameter and return type;
- the entry `let p = mk(7) in p`;
- all 22 bounds;
- the layout version `("c6-phys-test", "1")`;
- capacities 128 / 4096 / 4096 / 4096;
- ext caps 64 / 64 / 64 on top of `all_zero`;
- `boundary_capacities_present: false`;
- `GENERIC_ABSENT`, `Nat(FUEL)`, `Nat(stack_limit)` and `FinanceInput::Absent`.

The only difference is the use list: explicit public imports replace `use super::*`. Omitting `fixtures::caps()` is correct, since 3A never uses it.

### Strong preservation vs. observed outcome
The predicates are preserved at source level. Every runtime outcome is **UNOBSERVED** (see §5).

## 4. Findings

**Introduced mandatory findings: none.**

**Nonblocking:**

- **NB1: test 2 `[ext+]` #1 is partly self-referential.**
  - The `phys` side compares `C6PhysCaps::test_caps()` against `C6StateCaps::test_caps().phys`, which that function defines as the same value. The `max_data_field_slots: 1024` and `max_helper_tasks: 64` parts are real checks.
  - `[ext+]` #2 pins `max_types`, `max_arity`, `max_data_nodes`, `max_names` and `max_env_cells`, but not `max_constructors` (behind ctor_id 257/256).
  - The comment "exactly those every expectation above was derived from" therefore overstates the coverage. The assertions are additive only, and the original assertions already pin those values.
- **NB2: the test 3 `[ext+]` comment "refused or accepted" is inaccurate.** The original `data_lanes_from_host(&changed, &caps).unwrap()` requires the flip to be accepted. The `[ext+]` assertion only confirms the equal 128/128 caps afterwards.
- **NB3: one claim in the author hand-trace is imprecise.** `ExtProgram`, `ExtFunction`, `ExtParamSrc` and the `*Src` structs have no `Drop`; only `TypeExprSrc`, `ExtExpr`, `ExtValue` and `ExtPattern` do. This has no effect.
- **NB4: edition premise mismatch.**
  - 48360 recorded Edition 2021, while the author and the passive parse used 2024.
  - The consumer belongs to the same package as the originals, so it inherits the same edition.
  - I found no edition-sensitive construct: no `gen`, no macro fragments, no `impl Trait` lifetime capture, and no reference patterns that differ between editions under default binding modes.
- **NB5: string drift.** `SECTION` and `DATA_S` are copies. A production rename would surface as an assertion mismatch, not a compile error (the author noted this).
- **NB6: build-mode divergence.** The consumer exercises owner internals with `cfg(test)` = false. Any `cfg(test)`-gated behavior in the owner bodies I did not receive would make external results differ from the original unit context. That is one more reason no pass can be inherited.

## 5. Premises, scope limits and unobserved items

**Narrow declaration gaps.**
- **`PROGRAM_SECTION_TAG`.** Only the const line `[1772,1823)` was supplied. No supplied production use implies it exists in the normal build, so a preceding `#[cfg(test)]` cannot be excluded from the readset.
- **Cargo linkage.** As in §2: lib name, test discovery, features and edition.

**Core-internal premises (outside this verdict).**
- `impl OwnerEngine for CheckedOwnerEngine`.
- `Clone for LayoutSchemaC`. `admit_ext_program` and `prepare_ext_program` call `.clone()`, but `shared.rs` shows no derive.
- The derives on `SegBase`, `CtorRowExt` and `CtorFieldRowExt` that the snapshot derives need.
- The bodies of `owner_prepare`, `owner_runtime`, `families`, `host_implied_prefix`, `adt_storage`, `owner_ext_c_expression` and `prepare_ext_program_types`.

**Normal rlib: UNQUALIFIED.** No qualified normal production rlib exists for the 98941 / 804173 tree.
- The 95011 attempt stopped at 30.0016 s with a null exit and four tail operations unrun.
- The 88603 attempt also stopped at 30 s.
- The old 7.10 s Core result and the passed 18-test ELF come from a different tree. They establish neither library fit nor library existence.
- This source does not resolve or hide that build gap.

**Runtime: UNOBSERVED.** None of these has been observed:
- generic admission succeeds;
- `admit_run` succeeds;
- the terminal state is reached within 4096 steps;
- a Constructor row is present;
- both Const and Data refs appear;
- env and value slots are nonempty (which together implies `n ≥ 2`);
- the flip stays within caps;
- the host lanes are within the admitted caps;
- all 47 + 6 / 7 / 3 assertions pass.

**Separate records.** The three `public3a_*` tests and the three original unit tests are all UNRUN, and each set is its own independent record. No result passes between them.

**Operation eligibility: not approved.** Any future compile needs a genuinely changed method or a newly verified premise, plus a fresh exact-operation review and Root's selection. Retrying unchanged or raising the cap is not eligible.

**No claims** are made about native code, SDK, proof, key, context, accounting, constraints or refinement.