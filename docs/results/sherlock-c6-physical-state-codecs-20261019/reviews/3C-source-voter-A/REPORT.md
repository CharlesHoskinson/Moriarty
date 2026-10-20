# Chunk 3C review, voter A: `c6_phys_state.rs` (52,511 B) and `mod.rs` (1,243 B)

**Overall verdict: AGREE.** No owner correction is mandatory. There is one targeted ABSTAIN, on the runtime claims of tests 3 and 4 (§5). Two design recommendations should be resolved before ABI publication (R1 and R2 in §6).

I reviewed source only. I compiled, ran and edited nothing, and did not recompute any hash. The byte counts and SHA-256 pins are the supplied values. The registration delta is consistent: 1220 B plus `"pub mod c6_phys_state;\n"` (23 B) is 1243 B, and the anchor line appears once.

---

## 1. DESIGN: **AGREE**, with R1 and R2

### Framing

The section is one 41-lane header followed by three children, contiguous, with no padding and nothing trailing.

- **Offsets.** `offsets()` starts at 41 and accumulates with `checked_add`, so the offsets are monotone and the children cannot overlap.
- **Lane map.**

  | Lanes | Content |
  |---|---|
  | 0 | magic |
  | 1 | version |
  | 2 | `STAT` (`0x5354_4154`) |
  | 3 | child count, which must be 3 |
  | 4..=9 | (tag, length) pairs in the fixed order IMAG, DATA, HELP |
  | 10 | ext-presence bool |
  | 11 | reserved, zero |
  | 12..=40 | capacity descriptor |

- **Tags.** STAT, `IMAG` (`0x494D_4147`), DATA, HELP and PROG are pairwise distinct.
- **Order errors.** Because the order is fixed, a reordered or duplicated child is reported as `BadChildTag` at its fixed position.
- **Redundant lengths.** Each directory length is fully determined by the child's own counts. A declared length that disagrees with the child's content is still refused, because each child decoder applies its own exact-length check to exactly the declared slice. The resulting error is the child's own `Truncated` or `TrailingLanes` at the reported offset. The section is therefore unique and no boundary is inferred.
- **IMAG is directory-only.** The image child carries no tag lane of its own (its lane 2 is the control tag). Feeding a DATA or HELP body into the image slot fails `decode_control` with `BadTag`, which is acceptable.

### Length prediction

`plan_state_lengths` computes the following:

| Child | Formula |
|---|---|
| Image | 48 + 13·frames |
| Data | 7 + 5·data + 2·fields + 5·env + 2·slots |
| Helper | 31 + 7·tasks |

These match Core `section_len`, 3A `table_offsets` (widths `[5,2,5,2]`) and 3B `section_len` exactly. `ChildLengthDrift` is a correct self-check, and with these formulas it is unreachable.

### Capacity descriptor (29 lanes)

I checked all 27 `C6PhysCaps` fields against the declaration order, followed by `max_data_field_slots` and `max_helper_tasks`. All are converted losslessly with `u(usize)`. An array-length mismatch would be a compile error, which guards against silent miscounts.

- **Absent vs present-zero ext.** `from_parts` takes `Option<ExtCapacitiesC>` and computes caps from the effective value. Absent and present-zero produce identical caps but differ in lane 10, which matches the test.
- **What the context binds.** `C6StateContext` is documented as an independent caller assertion. It does not identify the layout: two layouts with equal caps but a different `layout_version` or `field_id` produce identical descriptors. The doc is right not to infer admission identity from matching caps.
- **`retained_layout_agrees_with_host`.** It compares `layout_version`, `field_id`, the three width lists in stored order, `scalar_layout_id`, `bounds` (both `layout.bounds` and `admission.bounds`), `capacities` and `boundary_capacities_present`. That is every `LayoutSnapshot` field. Ext caps have no host counterpart, as stated.
- **But no normal API calls it.** `encode_state`, `decode_state`, `host_with_decoded_state` and `compare_uncached_witness` all accept any context. This is a stated limitation, not a defect (see O1).

### Representation only

- The stored `alloc`, `active` and segment-base counters are carried exactly, independent of vector lengths. The test asserts `active.data == 21` while `data.len() == 3`.
- Depth and `task_live` are not stored.
- Nothing is normalized and nothing is relabelled as StepInv.
- `max_types` policies are inherited unchanged. Builtin-capable fields use `max(max_types, 4)`: frames, controls, `Scalar.expected` and `PairTask.sort`. User-nominal fields use plain `max_types`: data Constructor and `Build.type_id`.
- 3B NB1 (Build vs Materialize policy divergence when `type_rows < 4`) remains unresolved and inherited. 3C adds nothing new there.

### R1 (design, recommended before ABI publication): codec budgets are bound into the state context

Descriptor lanes 37 and 38 carry `max_type_expr_nodes` and `max_template_bytes`. Core documents these as "codec work budgets, not admitted limits", and no state child reads them.

**Counterexample.** Take one admitted state and two contexts built from the same retained layout, one with budgets (0, 0) and one with (4096, 8192).
- `encode_state` produces lanes that differ only at lanes 37 and 38.
- Decoding the first encoding with the second context returns `ContextMismatch { lane: 37, expected: 4096, actual: 0 }`.

So the state bytes are a function of (state, caps, tooling budget), not of (state, caps). Eleven of the 29 lanes are program-only caps that the state section never uses:
- occurrences, occurrence paths, patterns, pattern children
- ctor_fields, decl_templates, plan cases, constant fields
- name_bytes, and the two budgets

This is consistent and documented, so it is not a source defect. Before Chunk 4/5 treat these lanes as a public context, choose one of:
- drop the budgets (and optionally the unused program caps), or
- fix the budgets to a profile constant, or
- state explicitly that context identity includes tooling budgets.

### R2 (honest scope of the witness comparison)

`compare_uncached_witness` requires `first_host_difference(host, rebuilt) == None` before it plans anything. The supplied `construct_plan_for` / `decode::build` and `evaluate_uncached` / `dag::evaluate` are pure functions of `&HostSnapshot`: they use `BTreeMap`, `BTreeSet` and Kahn ordering on the smallest ready ID, with no address, time or Arc identity. Once the hosts are equal, the following are therefore implied by determinism:

- `PlanMismatch`
- `EvaluationMismatch` for both the Decoded and Cross sides
- both `check_live_outs` calls (§4)

None of them can detect a codec defect that the whole-host equality did not already detect. The reachable failures are only these:
- `OriginalPlan` or `OriginalEvaluation`, meaning the existing witness pipeline cannot handle a reachable state;
- `HostMismatch`.

The comparison is still useful as reachability coverage of `witness_plan` across real generic-ADT owner states and as a determinism check. It must not be cited as preservation evidence independent of host equality. I recommend that the module doc say so.

---

## 2. SOURCE_SECURITY: **AGREE** (representation and diagnostic only)

### `decode_state` first-error order

The code matches the doc exactly:

1. `admit_total(len)` returns `LimitExceeded` before any lane is read.
2. Fewer than 41 lanes returns `Truncated { "state", 41, len }`.
3. `BadMagic`.
4. `BadVersion`.
5. `BadTag`.
6. Lane 3 returns `BadSectionCount`.
7. For each child in order: its tag (`BadChildTag`), then `rd` on its length (`UsizeOverflow`), then its minimum (`ChildLengthRange`, minimums 48/7/31).
8. Lane 10: `rd_bool` (`BadBool`), then a comparison with the context (`ContextMismatch`).
9. Lane 11: `zero` reports the absolute lane 11.
10. Lanes 12..=40: `ContextMismatch`.
11. Offsets (`UsizeOverflow`), then `Truncated`, then `TrailingLanes`.
12. Children decoded on exact slices `[o0..o1]`, `[o1..o2]`, `[o2..o3]`, in the order image, data, helper, each wrapped as `Child { child, offset, error }`.

### No panic on hostile input

- Header indexing happens only after the `len ≥ 41` check.
- Slices are safe because `o[3] == len` and the offsets are monotone.
- Every addition is checked.
- `unsafe` is forbidden in the crate.

### Canonical domains agree

The decoder accepts L exactly when both of these hold:
- L's header equals `state_header(dir(L), ctx)`;
- each child slice is accepted by its child decoder.

Each child decoder accepts only its canonical encoding, with exact length (Core, 3A and 3B inverse). So the declared lengths equal the child canonical lengths, which in turn equal `plan_state_lengths(decoded)`. It follows that `encode_state(decode_state(L)) == L`. That encode uses the same limit, since `o[3] = len(L) ≤ limit`, and the same caps.

In the other direction, `decode_state(encode_state(s)) == s` whenever encoding succeeds.

On precedence: the encoder reports `LimitExceeded` before child caps, and the decoder reports `LimitExceeded` before anything else. Both agree.

### Allocation and work claims for every public wrapper

| API | Bounded by `max_total_lanes`? | Precise statement |
|---|---|---|
| `decode_state` | **Yes** | The limit check runs before any read. Nothing is allocated before the children decode. Child allocations are bounded by their slices, whose lengths sum to `len ≤ limit`. |
| `encode_state` | Output, yes | The plan and limit run before the child encoders. The three child outputs sum to `o[3] ≤ limit`, and `out` is `o[3]`. Peak usage is about 2·`o[3]` lanes, plus the caller-owned state. |
| `state_lanes_from_host` | **No (prefix)** | `from_image` clones all six image tables before planning or the limit check. The size is proportional to the caller's host. Same pattern as 3A N2 and 3B NB2. |
| `roundtrip_state_owned` | Yes | Bounded the same way as encode and decode. |
| `host_with_decoded_state` | Decode only | After decoding, `host.clone()` copies the whole caller host, including `AdmissionSnapshot` and the old image tables that are about to be replaced (O2). |
| `compare_uncached_witness` | **No** | It clones and encodes, decodes, clones the host, builds two plans and runs three evaluations. Plan construction is not bounded by the limit (see below). |

**Witness work is unbounded by the limit.** This is a property inherited from the supplied `decode::Builder::value`, which builds a "fully unfolded occurrence tree; repeated raw references produce repeated semantic occurrences".

**Counterexample.** Consider data rows `B₀ = Leaf`, `Bₖ₊₁ = Branch(Bₖ, Bₖ)`, created by repeated `let t = … in t <op> t`. The rows are about 5·k data lanes, but construction creates 2ᵏ plan nodes. With k ≈ 40, well inside `data_nodes`, the state section stays under 1,000 lanes while `construct_plan_for` does not terminate in practice.

Separately, `environment_cells` rebuilds every chain, which is O(E²).

This is a documented limitation of the existing implementation and not a 3C defect. However, `max_total_lanes` must not be described as bounding `compare_uncached_witness`.

### Authority

- Nothing constructs `Admitted`, `MachineImage`, `RunIdC`, `IdentityC` or `AdmissionSnapshot`.
- `replace_state_of` writes only the caller-owned image. It keeps `run_id` and never touches `admission`.
- A decoded nonzero segment base is materialized only through the public `SegBase::from_parts`.
- A diagnostic clone with arbitrary untrusted lanes is never an owner state.

### `replace_state_of` is exhaustive

- **Root.** `C6Root` is destructured over all 16 fields, with no `..`.
- **Image.** `ImageSnapshot` is destructured over all 23 fields: `run_id: _` plus the 22 written fields, with no `..`.
- **Tables.** `C6DataTables` is destructured over 4 fields and `C6HelperTables` over 3.

As a result, adding any field to `C6Root`, `ImageSnapshot` or either table type is a compile error until the field is handled.

- All full-width u128 counters and the origins are moved unchanged.
- Raw empty slices, `None` vs `Some(0)`, `Const` vs `Data`, order and multiplicity all pass through the child codecs, which preserve them by construction.

`first_host_difference` lists `admission` and all 23 image fields, with a final whole-value fallback. It is not exhaustive by destructuring, but the fallback "snapshot" covers any future field.

---

## 3. NORMAL_API_COMPILE: **AGREE** (source trace; no typecheck observed)

### Production imports

All of these are visible from `c6_phys_state`.

| Source | Items | Visibility |
|---|---|---|
| Core `c6_phys` | `decode_image_section`, `encode_root_and_frames`, `C6CodecBudgets`, `C6PhysError`, `C6Root`, magic, version, `FRAME_LANES`, `IMAGE_SECTION_HEADER_LANES` | `pub` |
| 3A `c6_phys_data` | `rd`, `rd_bool`, `u`, `zero` | `pub(crate)` |
| 3A `c6_phys_data` | `decode_data_tables`, `encode_data_tables`, `C6DataTables`, `C6StateCaps`, and the header/row/ref/env widths and tag | `pub` |
| 3B `c6_phys_helper` | `decode_helper_tables`, `encode_helper_tables`, `C6HelperTables`, `HELPER_FIXED_LANES`, `HELPER_SECTION_TAG`, `TASK_LANES` | `pub` |
| `complete_observation` | `HostSnapshot`, `ImageSnapshot` | `pub` |
| `shared::host_implied_prefix` | `SegBase` (the module is `pub mod`; `from_parts` is `pub const fn`) | `pub` |
| `shared` | `ExtCapacitiesC` (via `pub use`); `BoundsC`, `FrameRowC`, `LayoutCapacitiesC`, `LayoutSchemaC` | `pub` |
| `witness_plan` (registered `pub mod`) | `construct_plan_for`, `evaluate_uncached`, `LifetimeUnit`, `NodeId`, `Operation`, `WitnessEvaluation`, `WitnessFailure`, `WitnessPlan` | `pub` |

Every import is used. `LayoutSnapshot` field names and `WidthTable::*_entries()` match the supplied bodies.

### Expressions and lints

- **`descriptor()`.** It builds a 29-element `[usize; 29]` and calls `.map(u)`, passing a `const fn` item as `FnMut`. Fine.
- **`decode_state`.** `rd_bool(…).map_err(outer)? != ctx.ext_present` is correct by precedence.
- **`encode_state` / `decode_state`.** The function-local `use C6StateChild::*` imports `Image`, `Data` and `Helper` without colliding with any module item.
- **`state_header`.** `copy_from_slice` is called with length 4 and with length 29.
- **`replace_state_of`.** The `let C6DataTables { data, … } = data;` rebinding is valid, because the right-hand side resolves before the new bindings. Destructuring `image: &mut ImageSnapshot` binds `&mut` fields under the default binding mode.
- **`check_live_outs`.**
  - The `bad` closure is non-capturing, so it is reusable and its parameter types are inferred from the variant.
  - The let-else over `Option<&Operation>` binds `fields: &Vec<FieldBinding>`.
  - `BTreeSet<NodeId>` is fine because `NodeId` implements `Ord`.
- **`first_plan_difference`.** Inside, `fn first<T: PartialEq>(x: &[T], …)` receives `&Vec<_>` arguments and coerces them with generic inference.
- **Derives.** `C6StateError` and `C6WitnessError` derive `Eq`, which requires `C6PhysError`, `WitnessFailure`, `FailureKind`, `ValueType`, `SortTag` and `NodeId` to be `Eq`. All are. `C6WitnessPreservation: Eq` requires `WitnessPlan` and `WitnessEvaluation` (including `HostDiagnostics`) to be `Eq`. Both are.
- **Edition 2024.** I found nothing sensitive to the 2024 edition. The `|&(same, _)|` and `for &(k, v)` patterns are explicit reference patterns, there are no `gen` identifiers, and every tail temporary is a `Copy` value or is consumed. Every `let mut` is mutated, so there are no unused-variable warnings.

### Tests

- **Glob import.** `use super::*` brings in the parent's private functions (`first_host_difference`) and its private `use` imports (`BoundsC`, `LayoutCapacitiesC`, `C6CodecBudgets`, `SegBase`, `LifetimeUnit`, …), the same mechanism 3A and 3B rely on. Tests also access private `C6StateContext` fields (`c.limits`, `absent.ext_present`, `narrow.caps.phys`), which is legal from a child module.
- **Explicit imports.** `C6StateChild::{Data, Helper, Image}` shadows the glob. `RefC::Data` is always qualified.
- **Mutation table.** `[(&str, fn(&mut C6OwnedState)); 29]` deduces each closure signature from the expected fn-pointer element type, and none captures anything. `s.helper.tasks.push(s.helper.tasks[0])` compiles under two-phase borrows, since `HelperTaskC` is `Copy`.
- **Fixture APIs, checked against the actual bodies:**
  - The `LayoutSchemaC` literal has all 8 pub fields.
  - `ExtCapacitiesC { …, ..all_zero() }` is valid.
  - `admit_ext_program(ExtProgram, ExtDomain, &BoundsC, &LayoutSchemaC, Option<LocatorC>)` resolves through the `pub use`. AD-0a holds because the same `bounds` value is used.
  - `host_owner_engine().admit_run(...)` resolves because `OwnerEngine` is imported. R3 (4096 ≤ 10⁶), R4 (64 ≤ 64) and R5 (4096 ≤ 4096) all hold.
  - `fixtures::{must, FUEL}` are `#[cfg(test)] pub(crate)`, and `must` places no bound on `E`.
  - `layout.clone()` resolves to `impl Clone for LayoutSchemaC` in the private `owner_prepare` module. Trait impls are global, so module privacy does not matter, and `WidthTable: Clone` is there too.
  - `real_ctx` passes `layout.ext_capacities` by `Copy` out of `&LayoutSchemaC`, and `from_layout == from_parts` holds by construction.
  - `record_step` errors are matched as `Err(_)`, so the missing `Debug` on `CaptureStepError` is not needed.

### Transitive bodies

For compiling 3C itself, nothing further is needed. The test binary also needs the existing `witness_plan/adt_projection_tests.rs` and the families' `include!`d fragments. Those bodies are not supplied, but they are existing files that the reported Core18 build depends on. This is a premise, not a new 3C dependency.

---

## 4. The four tests (all UNRUN), checked against the literal APIs

### Test 1: outer framing, lane map, inverse and refusal precedence

I verified every literal by hand against the source.

**Lengths and header lanes:**
- The child lengths are 74, 35 and 45 (`48+2·13`; `7+15+4+5+4`; `31+14`), for a total of 195.
- The header prefix is `[magic, ver, STAT, 3, IMAG, 74, DATA, 35, HELP, 45, 1, 0]`.
- Descriptor lane 12 is 64 (frames), lane 22 is 8 (arity), lane 39 is 1024 and lane 40 is 64.

**Caps on the synthetic state (all pass under `test_caps`):**
- ConstructorFieldsF: type 5 < max(64, 4); `completed` end 3; `rest` end 7 ≤ 256.
- Data Constructor: type 5 < 64.
- PairTask: sort 3.

**Child placement:**
- `41 + L_INITIAL_FUEL = 67` holds `u128::MAX`.
- The top frame is at `102..115`.
- Data is at `115..150` and helper at `150..`.
- Inverse and re-encode both hold, and the limit of 195 is accepted while 194 is refused for both encode and decode.

**Every refusal vector matches the code path:**
- BadMagic, BadVersion, BadTag, and `BadSectionCount(2)` before the child tag.
- The reordered directory gives `BadChildTag{Image}`; the duplicate gives `BadChildTag{Helper}`.
- Length `u128::MAX` gives `UsizeOverflow`; length 6 gives `ChildLengthRange{Data, 6, 7}`.
- `BadBool` comes before reserved; reserved lane 11 gives `NonZeroPadding`.
- With the ext-absent context, the ext lane mismatch (lane 10) is reported before reserved.
- The descriptor mismatches at lanes 12 and 39.
- Encoder data cap: `state.data_fields` 2/1 at offset 115.
- Offset overflow on `usize::MAX`.
- Outer `Truncated` 196/195 beats a planted frame `BadTag` at lane 89.
- `TrailingLanes` 1.
- Declared 87/22 gives image `TrailingLanes` 13; declared 61/48 gives image `Truncated` 74/61.
- Image errors precede data errors (lanes 89 and 122).
- Data row `BadTag "state.data"`; data header `BadTag "state_data"`.
- Helper record `BadTag` 12 at offset 150.
- Encoder `root.stack_limit` 65/64 at offset 41.

**Gaps:** `ChildLengthRange` is not tested for Image or Helper, `BadChildTag` is not tested at the Data position, and no test covers a header-wide every-lane significance check.

### Test 2: 29 state mutations, external retention, nonzero SegBase

- The 29 mutations cover all 16 root fields, frame order, frame count, the raw empty `rest.start`, the four data tables, helper and result presence and payload, and task order and multiplicity. Each one stays within caps, changes the lanes and decodes back exactly.
- The empty-table encoding is 127 lanes (`41+48+7+31`), and `env_root` `None` vs `Some(0)` is distinct.
- The ext comparison (absent vs zero) is checked against the actual `effective_for_extended_domain`.
- Clone-replace into a real genesis host:
  - admission and `run_id` are retained;
  - `from_image(out)` equals the synthetic state;
  - the nonzero `SegBase` comes from `from_parts` and differs from `genesis_uncut`;
  - the first difference is `image.control` (genesis `Eval{0}` vs `Eval{5}`);
  - replacing back restores the whole host.
- The synthetic state is correctly labelled unadmitted, and no admission rule is applied to it.
- **Gap:** no test asserts at the lane level that changing `run_id` or `admission` leaves `state_lanes_from_host` unchanged.

### Test 3: real owner trace, two-argument call, nested generic Box/Pair

- **Admission.** It uses real generic admission and real `capture` / `record_step`. The context comes from the retained layout with ext present.
- **Retained-layout check.** It passes, and a deliberate `bounds.frames` mismatch is reported as "bounds", which is the first mismatching field in check order.
- **Per-boundary check.** Each boundary runs decode → `from_image` equality, re-encode equality, and whole-host clone-replace equality. `before != after` for the lanes is compared with `before != after` for the hosts. This holds because every image field except `run_id` is encoded and the codec is injective.
- **Terminal.** It must be reached within 4096 iterations and must be `Halted`. The loop panics on failure rather than skipping.
- **`Seen` flags.** These target exactly what the two-argument program adds:
  - `ArgsF` with `completed.len > 0` while the second argument is evaluated;
  - `ConstructorFieldsF` with a nonempty `completed` while `MkBox(y)` is evaluated;
  - `Bind` together with a `PairTask`.

### Test 4: uncached witness comparison over the trace, all three lifetimes

- It verifies `RecordBefore` and `RecordAfter` on every event, `Snapshot` every 8 steps, and the terminal state under all three lifetimes. Any failure panics with the step number, lifetime and full error; nothing is skipped.
- **`live_out_fields == 33`.** This matches `OUTPUT_FIELDS`: 13 image-side fields plus 20 program fields, built in that order by `decode::build`.
- **`live_outs.len() == 34`.** Each field dependency is a freshly added node: sequence, payload, environment or `values_slice`. So the live-outs are distinct, and the BTreeSet count equals the number of definitions flagged as live-out.
- **What `check_live_outs` verifies.** Producer and value equal the id, the `live_out` flag is set, the lifetime matches, `last_use_order` equals the schedule length, and the counts agree. Every one of these is a construction invariant of `dag::definitions` and is also re-verified by `dag::validate` inside `evaluate_uncached`. The check is consistent with the actual bodies but tautological under the current implementation (R2).
- **Scope of the evidence.** Evaluation exposes only `DecodedSnapshot` and `HostDiagnostics`. The test asserts the lanes, the lifetime and `metadata.seg_base`, and does not claim any intermediate slot values.
- **Weak counts.** `helpers`, `tasks`, `results` and `ctor_frames` are only asserted `> 0`. These are literal assertions, not replaced expectations.

In none of the four tests did I find `.or(actual)`, circular expected values, or false fixture labels.

---

## 5. Targeted ABSTAIN: runtime claims of tests 3 and 4 only

The following are unobserved, and the bodies they depend on are not in the packet:
- whether the two-argument generic program is admitted;
- whether it halts within 4096 iterations at `FUEL` = 4096;
- whether every `Seen` flag is reached;
- whether `construct_plan_for` and `evaluate_uncached` succeed on every reachable state under every lifetime;
- whether this finishes within the existing bounded QA budget, given that the prior combined builds were STOPPED at 30 s.

The missing bodies are `prepare_ext_c1_to_p`, `owner_ext_prepare_types`, `nominal_declaration_order`, `ext_pass_b_walk`, `ext_source_traversal`, `owner_ext_c_slots`, the `families/fragments` legacy and scalar files, and `adt_projection_tests`.

My trace of the family and owner bodies that were supplied makes all of these plausible. That trace is not execution, and Core18 PASS does not establish terminal reachability for generic programs. This abstention does not extend to compilation or to tests 1 and 2.

---

## 6. Findings

### Mandatory source or API corrections

None.

### Recommended before ABI publication or Chunk 4 binding

- **R1.** Remove the codec budgets (and possibly the unused program caps) from the descriptor, or explicitly declare them part of the context identity (§1).
- **R2.** Document that the witness comparison is implied by whole-host equality plus determinism, and that it provides witness-reachability coverage only. Also document that `max_total_lanes` does not bound it, and that `construct_plan_for` is exponential under data-node sharing (§2).

### Optional

- **O1.** Add a wrapper that derives the context from a retained layout and calls `retained_layout_agrees_with_host` first, for the normal host-facing paths.
- **O2.** Have `host_with_decoded_state` build `ImageSnapshot { run_id: host.image.run_id.clone(), …decoded }` with `admission.clone()` instead of cloning and then overwriting the old tables. This avoids a full table copy while keeping the exhaustive struct literal.
- **O3.** Tests to add:
  - lane-level `run_id` and `admission` externality;
  - `ChildLengthRange` for Image and Helper;
  - `BadChildTag` at the Data position;
  - flipping each of the 41 header lanes;
  - `LimitExceeded` together with a child cap violation;
  - a budget-lane mismatch at lane 37;
  - an ext-absent real context.
- **O4.** On owner error, print the `HostViolation` locator by pattern instead of using bare `Err(_)`.
- **O5.** Add `ChildLengthDrift` to the encoder-order doc.

### Actual runtime obligations

- A normal mori-core library and test build under the existing toolchain, offline and within the existing bounds. No unchanged retry and no cap raise is authorized.
- Run the 31 host tests (18 + 3 + 6 + 4) by exact filter.
- Run the backend `c6_a135_push` real-SDK test separately. It remains unqualified.

### Preserved obligations from prior votes

- **3B voter A T1** remains open before ABI publication: literal vectors for the other 9 helper kinds and 18 result kinds. 3C's test-1 child-slice comparisons reuse the same encoders and do not close it.
- **3B voter A T2–T7** remain optional.
- **3B voter B's limits** on result, task and staged-value coverage and on true reachability remain.
- **3B NB1** (TypeId policy divergence) is inherited.
- Neither the six 3B tests nor these four establish that every variant is reachable or executed.

### Remaining full scope (OPEN)

- **All 39 rules:** partition, selectors, rule constraints, typed witness and export.
- **Grouping:** widths 1/2/4/8 with earliest stop, free publication and run barriers.
- **Residualization:** the finite immutable-key residual baseline and the generic fallback.
- **Chunk 4:** full host, program, admission and identity, and checkpoint binding.
- **Chunk 5:** native constraints, emitter, prover and ledger.
- **Finance and persistent state.**

The state lanes, directory offsets and descriptor are not a bound public statement, native instruction or relation. `c6_lanes_to_immediates` operands are not constraints, nothing touches the A114 L4 emitter, host, schema or bridge, and legacy nine-frame `FrameSlots` is neither changed nor reused.

### Scope of this vote

This vote covers the 3C assembler and witness-comparison source, its API and its design coherence. It does not claim adoption, global encoder conformance, qualification, performance, or any compile, test or native result.