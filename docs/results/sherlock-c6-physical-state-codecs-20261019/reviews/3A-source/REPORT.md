# Chunk 3A M1 correction: fresh independent review (source and API only)

I read the inline packet only. I compiled nothing, ran no tests, executed nothing and computed no hashes. Nothing here approves any native, execution, proof, key, profile or accounting operation.

## 0. Pins

| Item | Bytes | SHA-256 | Coverage |
|---|---|---|---|
| 3A candidate `c6_phys_data.rs` (after) | 28,329 | `804173e01f2316c7786735ee959bf235fa235b67a914af124907a8c00bbd8bb4` | Whole file read |
| 3A predecessor (before) | 28,264 | `a0e8515c376c0edcf3161268833f7e1170fb2dbc12b7926f74a2ca92209eff2c` | Not supplied as a body; known via OLD/NEW and inverse facts |
| `mod.rs` registration | 1,196 | `ddc59431df64bb832285b8c015c7de8d900e9c86636ac2eed392b8c715c90862` | Whole file; unchanged |
| Corrected Core `c6_phys.rs` | 97,378 | `98941c4a592a9a58e59ee862c72f89e931cdbb74d7fbb91fbd11a8ac37c6fb17` | Not installed. Excerpts: public consts, errors, caps and budgets `[2771,9706)`; whole `fixtures` `[58226,62965)`; private `type_index_cap` `[15183,15248)` |
| `shared.rs` | 49,413 | `a71299883c658d8647fab932929c4044e4281d309d94870bc26c3456a106b6d2` | Whole file |
| `complete_observation.rs` | 25,752 | `2891c4f00454cf6bb14ec3ff03000ccd0e36eb4b5acd45fd6040504d93855c5b` | Whole file |
| `adt.rs` | 1,139 | `bc4c239e…` | Whole file |
| `ExtCapacitiesC` decl and impl | — | from `025de7cd…` | Declaration block |
| `PROGRAM_SECTION_TAG` | — | from `f4d4ae58…` | The constant |

## 1. SOURCE_SECURITY: **APPROVE**

This approval covers the representation codec only, within the scope of the prior 78740 approval.

**Why production is preserved**
- The only hunk sits inside `#[cfg(test)] mod tests`, within `refusals_follow_the_declared_first_error_order`. It cannot reach a non-test build.
- The byte arithmetic matches a single-hunk replacement. By my count, OLD is 106 B (8-space indent + 97 chars + LF) and NEW is 171 B (87 + 74 + 10). The +65 B delta equals 28,329 − 28,264.
- In the after-image, the closure text `|section, needed, limit|` no longer appears. `fn cap<T>(` appears exactly once. Every other `cap(` occurrence is a call site.
- Assertion macro counts match the supplied 47/7/3 by my own tally:
  - `assert_eq!`: 11 in test 1, 30 in test 2, 6 in test 3.
  - `assert!`: 1 in the helper, 1 in test 2, 5 in test 3.
  - `assert_ne!`: 3.
- The byte identity of the production prefix (15,877 B, `8c3477d5…`) is a supplied fact. I cannot recompute the hash.

**Independent spot re-check of the unchanged production code** (not just carried over from 78740)
- **Header and exact length.**
  - `len < 7` is checked before any `lanes[0..=6]` read.
  - Counts are read via `rd` before `count_caps`, then `table_offsets` uses checked arithmetic.
  - The length must then be exact, and `decode_rows` uses `get(from..to)` with monotone offsets.
  - `with_capacity` is bounded by the input length.
- **Row canonicality.** Every row tag pads all of its unused lanes:
  - U128 and Bool: lanes 2..5.
  - Nil: lanes 1..5.
  - Cons, Leaf and Branch: lanes 1..3.
  - Env `None`: lane 4.
  - Lanes are read in ascending order.
  - `NonZeroPadding.lane` is row-relative, because `zero` enumerates the fixed row array.
- **Encoder.** All count and row caps are checked before `table_offsets` and before `with_capacity`.
- **Authority.** No `Admitted` or `MachineImage` construction exists. `replace_tables_of` touches only the four caller-owned `ImageSnapshot` fields.

No mandatory security finding is introduced.

## 2. NORMAL_API_COMPILE: **APPROVE**

This is a source trace of the 3A file and its registration against the supplied declarations. It is not an observed compile.

### M1 resolution

```rust
fn cap<T>(section: &'static str, needed: usize, limit: usize) -> C6Result<T> {
    Err(C6PhysError::CapacityExceeded { section, needed, limit })
}
```

- **Placement.** A block-level item after `let` statements is legal.
  - It captures no locals and uses no enclosing generics.
  - `C6Result` and `C6PhysError` resolve through the test module's `use super::*`, which re-exposes the parent's private `use super::c6_phys::{…}` imports. The existing literal-`Err` assertions already rely on this path.
  - No other `cap` exists in the value namespace. The module helpers are `cap_count`, `cap_index`, `cap_ref`, `cap_env` and `cap_fields`; the test has `caps`; there are no locals named `cap`.
- **Generics.** `T` occurs in the return type, so it is fresh at each call and needs no bound. Constructing `Err` requires nothing of `T`.
- **Inference.** `assert_eq!` evaluates `*left == *right`. The only applicable impl is the derived `impl<T: PartialEq, E: PartialEq> PartialEq for Result<T, E>`, so `?T` unifies with the left operand's `Ok` type. This is the same mechanism the file's 18 existing `Err(…)`-on-the-right assertions already depend on.
- **Arguments.** Every call passes a string literal (`&'static str`, matching `CapacityExceeded.section: &'static str`) and integer literals that infer to `usize`.
- **Debug.** `assert_eq!` needs `Result<T, C6PhysError>: Debug`. `C6DataTables` derives `Debug`, `Vec<u128>` is `Debug`, and `C6PhysError` derives `Debug`.

**All 12 uses, re-enumerated independently**

| # | Left operand | Inferred `T` | Expected value | Trace |
|---|---|---|---|---|
| 1 | `dec(set (3,257))` | `C6DataTables` | `state.data` 257/256 | Count caps: data is first |
| 2 | `dec(set (48,64),(88,7))` | `C6DataTables` | `state.data.type_id` 65/64 | Row 8 at 47..52 is checked before value-slot lane 88 |
| 3 | `dec(set (49,256))` | `C6DataTables` | `state.data.ctor_id` 257/256 | ✓ |
| 4 | `dec(set (51,9))` | `C6DataTables` | `state.data.fields` 9/8 | Length is checked before end |
| 5 | `dec(set (50,1022))` | `C6DataTables` | `state.data.fields` 1025/1024 | len 3 ≤ 8, so the end check fires |
| 6 | `dec(set (73,256))` | `C6DataTables` | `state.env.binder` 257/256 | ✓ |
| 7 | `dec(set (82,256))` | `C6DataTables` | `state.env.previous` 257/256 | Env row 1 is at 78..83 |
| 8 | `encode_data_tables(one(Data(4)), c)` | `Vec<u128>` | `state.data_fields.ref` 5/4 | `max_data_nodes` = 4 |
| 9 | `decode_data_tables(l, c)` with `l[8] = 4` | `C6DataTables` | `state.data_fields.ref` 5/4 | ✓ |
| 10 | `encode_data_tables(t, tight)` | `Vec<u128>` | `state.data_fields` 8/7 | Count cap |
| 11 | `decode_data_tables(lanes, tight)` | `C6DataTables` | `state.data_fields` 8/7 | Count cap |
| 12 | `encode_data_tables(wide, caps())` | `Vec<u128>` | `state.data.type_id` 65/64 | ✓ |

That gives 9 decode uses and 3 encode uses, which matches the owner's split.

**Data and assertion preservation**
- Every lane vector, expected variant and field value, and the `one`, `c`, `tight`, `wide` and `from_parts` constructions are unchanged relative to what 78740 traced.
- The section layout is unchanged: 7 + 50 + 16 + 15 + 8 = 96.
- The borrows remain sound: `set` borrows `lanes` shared, and `wide = t` moves `t` after its last use.

### Compatibility list, against the actual supplied Core declarations

| Check | Result |
|---|---|
| `C6PhysCaps` | `pub`, derives `Clone, Copy, Debug, PartialEq, Eq`, which satisfies `C6StateCaps`'s derives. The eight fields 3A reads are present as `pub usize`. |
| `from_parts(&BoundsC, &LayoutCapacitiesC, &ExtCapacitiesC, C6CodecBudgets) -> Self` | `pub fn`; `max_types = bounds.type_rows` is unchanged. |
| `test_caps` | `pub const fn`, callable from 3A's `const fn`. Values are 64, 256, 8, 256, 256, 256, 256 and 1024, so test 2's 65/64, 257/256, 9/8, 101/96 and 1025/1024 hold (1024 comes from 3A's own `max_data_field_slots`). |
| `C6CodecBudgets` | Fields `type_expr_nodes` and `template_bytes` are `pub`. |
| `C6PhysError`, 9 variants used | Names and field names/types match exactly. Derives `Clone, Debug, PartialEq, Eq`. 3A needs no `Copy`. |
| `C6Result<T>` | `= Result<T, C6PhysError>`. |
| Magic and version | `0x4336_5048_5953_31` and `1`. |
| `fixtures` | `#[cfg(test)] pub(crate) mod` under `pub mod c6_phys`. `admitted_run(usize) -> (Admitted, MachineImage)` and `must<T, E>(Result<T, E>, &str) -> T` are both `pub(crate)` with no bound on `E`. 3A does not import `fixtures::caps`. |
| Fixture data | `constant_nodes`, `data_nodes`, `data_field_slots` and `environment_cells` are all 128, so the Const↔Data flip stays under equal caps. `ext_capacities: Some(…)` is unread by 3A (test 3 passes `all_zero()`; U2 remains). |
| Test 3 field paths | `host.admission.bounds`, `.layout.capacities`, `image.alloc/active/reference_cursor/frames/result` and `HelperResultC::EnvMissing` all exist as used. |
| `PROGRAM_SECTION_TAG` | `0x5052_4F47` from `c6_phys_program`, registered `pub mod`. It differs from `DATA_SECTION_TAG` (`0x4441_5441`). |
| `shared` and `adt` imports | `BoundsC` and `LayoutCapacitiesC` derive `Default`. `ExtCapacitiesC` is re-exported, with `const fn all_zero` and `effective_for_extended_domain(Option<Self>)`, and is `Copy`. `TypeIdC` and `CtorIdC` are `pub usize` newtypes. |

**Toolchain premises (unchanged from 78740)**
- Edition 2021.
- rustc ≥ 1.63, for explicit generic arguments alongside `impl FnMut`.
- Inline format arguments, which need rustc ≥ 1.58.

## 3. Introduced mandatory findings

**None.** M1 is resolved by construction, and the correction introduces nothing else.

## 4. Unchanged gaps and nonblocking findings

**N6, the explicit choice, reviewed in representation-only scope: ACCEPTED as stated.**
- The Core's private `type_index_cap` (`max_types.max(4)`) is used only at its five frame/control sites. 3A does not reference it.
- 3A's `state.data.type_id` check stays on plain `max_types`. That is sound for the codec: builtin sorts 0..=3 are encoded as the U128/Bool/Nil/Cons/Leaf/Branch tags, never as Constructor rows.
- The two policies coincide when `max_types ≥ 4`. Below that, 3A is only stricter.
- The claim that reachable Constructor rows have `type_id ≥ 4` and `< types.len() ≤ type_rows` remains a premise from the admission internals. I did not re-verify it here.
- No sixth correction is warranted. Optionally, a later revision could add a one-line comment at `data_row_caps` recording the divergence from the Core doc's "five TypeId index fields". This is not required, and no hunk is requested.

**Other nonblocking items**

| ID | Status |
|---|---|
| N1 | `unused_mut` on the non-capturing `fields` closure is still present, as intentionally retained. It is a warning only, unless the crate denies warnings. |
| N2–N5, N7 | Unchanged as recorded by 78740. |
| U1 | Addressed in the corrected Core by `type_index_cap`. Not re-reviewed here. |
| U2–U4 | Unchanged. |
| New, minor | A block item after statements trips only the pedantic `clippy::items_after_statements` lint, which is off by default. |
| 3B and 3C | Declaration gaps remain exactly as in 78740 §6. |

## 5. Source vs actual qualification limits

- **All three tests are UNRUN.** No whole-crate or normal-Core compilation was observed.
- **Test 3's runtime claims are premises.** It performs real generic admission and up to 4096 owner steps. Unverified claims:
  - a terminal state is reached;
  - a Constructor row is present;
  - both Const and Data refs appear in `data_fields`;
  - env and value slots are nonempty;
  - `n ≥ 2`;
  - the flip stays within caps.
- **Core-internal compilation is outside this verdict.** That includes the fixture's own imports and call signatures (`admit_ext_program`, `host_owner_engine().admit_run`, `CoreValue`, `ExtDomain::GENERIC_ABSENT`) and every Core module body beyond the supplied excerpts.
- **Bodies not read** (premises): `owner_prepare`, `owner_runtime`, `families`, `prepare_ext_*`, `check_generic_program_view`, `witness_plan`, `resource_report`, `owner_host`.
- **Supplied facts I could not recompute:** hashes, the production-prefix identity, the old-anchor uniqueness count, and rustfmt parse acceptance. I verified them only through the delta arithmetic, after-image inspection and macro tallies above.
- **Still OPEN, as in 78740:**
  - cross-section membership and counter binding (Chunk 4);
  - ctor-belongs-to-type checks and arity checks;
  - env acyclicity;
  - native, constraint, proof, key and profile work;
  - finance, accounting and public permissionless scope.
- **Separate records.** The two earlier Core source approvals are separate records and are not compile results. Selecting the actual Core qualification remains with root.