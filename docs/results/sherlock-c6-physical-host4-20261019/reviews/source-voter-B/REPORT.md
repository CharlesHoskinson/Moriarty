### Verdict Summary

- **Reviewer role**: Independent non-author Reviewer B (executed by Gemini 4 Argon HIGH via AGY fallback under standing user-authorized outage routing following Opus spend-limit error; source/review only, zero tools/commands/filesystem/runtime execution).
- **`DESIGN`**: **AGREE** (0 mandatory changes)
- **`SOURCE_SECURITY`**: **AGREE** (0 mandatory changes)
- **`NORMAL_API_COMPILE`**: **TARGETED ABSTAIN on truncated prerequisite bodies (`<truncated 1069008 bytes>`) / AGREE on all visible Chunk 4 + 3C + 3B source and interfaces** (0 mandatory changes to candidate source)
- **Overall Verdict**: **AGREE on `c6_phys_host.rs` (`a172431f…`) and `helper_machine/mod.rs` (`c3174ffb…`) design, security, and visible API; TARGETED ABSTAIN on the 1,069,008-byte truncated prerequisite suffix and unexecuted compiler/test runtime.**

---

### 1. `DESIGN` Assessment — **AGREE**

#### 1.1 Whole-Host Framing (`HOST` Schema 1) and Non-Overlapping Children
- **Header layout (`HOST_HEADER_LANES = 14`, lines 62–90, 342–363, 387–424)**:
  - Lanes `0..5`: `[C6_PHYS_MAGIC, C6_PHYS_VERSION, HOST_SECTION_TAG (0x484F_5354), C6_HOST_SCHEMA (1), HOST_CHILD_COUNT (4)]`.
  - Lanes `5..13`: fixed-order 4-child directory `(tag, exact_len)` for `Metadata` (`META_SECTION_TAG = 0x4D45_5441`, min `3`), `Program` (`PROGRAM_SECTION_TAG`, min `PROGRAM_HEADER_LANES`), `Context` (`CTX_SECTION_TAG = 0x4354_5854`, min `22`), and `State` (`STATE_SECTION_TAG = 0x5354_4154`, min `STATE_HEADER_LANES = 41`).
  - Lane `13`: reserved zero padding (`zero(HOST_S, lanes, 13, 14)`).
- **Exact non-overlapping child slices**:
  - `decode_host` initializes `o = [HOST_HEADER_LANES; 5]` and computes `o[i + 1] = add(o[i], declared)?` using checked `usize` addition (`C6PhysError::UsizeOverflow`). Because every `declared >= child.minimum_lanes() > 0`, `14 = o[0] < o[1] < o[2] < o[3] < o[4]` strictly.
  - After checking `lanes.len() < o[4]` (`Truncated`) and `lanes.len() > o[4]` (`TrailingLanes`), the slices `&lanes[o[0]..o[1]]` (`META`), `&lanes[o[1]..o[2]]` (`PROG`), `&lanes[o[2]..o[3]]` (`CTX`), and `&lanes[o[3]..o[4]]` (`STATE`) partition `lanes[14..]` contiguously without overlap.
  - `PROG` bytes are the exact output of `encode_program(&host.admission.view, &ctx.caps().phys)` and `STATE` bytes are the exact output of 3C `encode_state(&C6OwnedState::from_image(&host.image), &ctx)`.

#### 1.2 Standalone Inverse and Diagnostic Authority Boundary
- **`decode_host` (lines 387–424)** reconstructs a complete owned `C6DecodedHost { host: HostSnapshot, config: C6HostConfig }` from `&[u128]`, `&C6HostConfig`, and `&C6HostLimits` alone, without any pre-existing `HostSnapshot` clone.
- **`image_from_parts(run_id, state)` (lines 374–384)** exhaustively destructures `C6OwnedState { root, frames, data, helper }`, `C6Root` (all 16 registers including `seg_active`, `seg_alloc`, `seg_consumed_work` reassembled via `SegBase::from_parts`), `C6DataTables` (all 4 tables), and `C6HelperTables` (all 3 tables), combining them with the decoded `RunIdC` into `ImageSnapshot`. Any future field added to `ImageSnapshot`, `C6OwnedState`, `C6Root`, `C6DataTables`, or `C6HelperTables` is a compile-time error here.
- **No conditional clone masquerade**: Unlike 3C's `host_with_decoded_state` (which retains `admission` and `image.run_id` from a caller clone), `decode_host` decodes `IdentityC`, `BoundsC`, `LayoutSnapshot`, `ProgramSnapshot` (`view`), and `RunIdC` from the input lanes.
- **Authority boundary**: Decoded carriers (`C6HostMetadata`, `C6DecodedHost`, `C6HostCapture`, `C6OwnerBoundHost`) construct only owned diagnostic snapshots (`HostSnapshot`, `AdmissionSnapshot`, `ImageSnapshot`). None constructs a sealed `Admitted`, `MachineImage`, `PreImage`, checkpoint import, `StepInv` proof, or native transition authority.

#### 1.3 Exhaustive Field, Domain, and Canonicality Coverage
- **`IdentityC` (lines 189–200, 249–256)**:
  - `profile_id: u8`: encoded as `u128::from(*profile_id)`; decoded via `u8::try_from(profile)` (`> 255` refused with `UsizeOverflow`).
  - `grammar_domain`: `DedicatedSorts => 0`, `GenericNominalExtensionReserved => 1` (`> 1` refused with `BadTag`).
  - `finance`: `Absent => 0`, `TypedExtensionReserved => 1` (`> 1` refused with `BadTag`).
  - `layout_version: (String, String)`, `field_id: Option<(String, String)>`: exact string pair and optional pair (`0` / `1`).
  - `layout_digest`, `bounds_digest`, `program_membership_digest` (`DigestC`): `Unselected => [0]`; `Selected { format, bytes } => [1, format.0, format.1, bytes.len(), ..bytes]`. Format pair and opaque byte payload are preserved distinctly (`Unselected` vs `Selected` with empty `bytes: vec![]`).
- **`BoundsC` (lines 201–213, 257–266)**:
  - All 22 fields encoded in declaration order: 19 `usize` fields checked via `rd(META_S, v)` and 3 full `u128` work limits (`max_reference_work`, `max_helper_work`, `step_cap`) carried as full `u128`.
  - `admission.bounds` (`m.bounds`) and `layout.bounds` (`m.layout.bounds`) are encoded and decoded independently (44 lanes total) without cross-normalization.
- **`LayoutSnapshot` and `WidthsSnapshot` (lines 214–225, 267–283, 293–297)**:
  - Preserves `layout_version`, `field_id`, `widths`, `scalar_layout_id`, `bounds`, `capacities` (all 4 `LayoutCapacitiesC` fields), and `boundary_capacities_present` (`rd_bool`).
  - `widths`: `None => [0]`; `Some(WidthsSnapshot)` => `[1, index_widths, counter_widths, record_widths]`, preserving raw order, duplicate names, `None` vs `Some` with three empty vectors, and exact `u32` widths (`u32::try_from(w)` refuses `>= 2^32` with `UsizeOverflow`).
- **Strings and Byte Blobs (lines 168–180, 224–244)**:
  - Length lane followed by one lane per byte (`0..=255`; `> 255` refused with `BadTag`).
  - `MetaReader::text` decodes via `String::from_utf8(bytes)` (invalid UTF-8 refused with `BadTag { section: "host.meta", tag: u(len) }`; valid Unicode and embedded NUL `\0` preserved).
- **`RunIdC` and `RunIntC` (lines 226–230, 280–284, 298)**:
  - `run_id.identity` is encoded as a complete independent `IdentityC` (distinct from `admission.identity`).
  - `fuel_input` and `stack_limit_input` use 2-lane `RunIntC` encoding: `Negative => [0, 0]`, `Nat(n) => [1, n]`, `NatAboveU128 => [2, 0]`. Non-zero second lane when tag is `0` or `2` is refused with `NonZeroPadding { section: "host.meta", lane: self.at - 1, value: n }`.
  - Negative-input genesis (`RunIntC::Negative` with zero `initial_fuel`/`stack_limit` registers and `ControlC::Error { code: 1 }`) preserves `RunIntC::Negative` in `run_id` without normalizing from registers.
- **`CTX` Section (`CTX_LANES = 22`, lines 304–339)**:
  - Lanes `0..3`: magic, version, `CTX_SECTION_TAG`.
  - Lane `3`: `ext` presence boolean (`0` for `None`, `1` for `Some`, checked via `rd_bool`).
  - Lanes `4..20`: all 16 `ExtCapacitiesC` fields in declaration order (`plan_case_slots`, `field_path_slots`, `pattern_child_slots`, `input_rows`, `asset_rows`, `state_rows`, `account_rows`, `liability_rows`, `await_site_rows`, `await_binder_slots`, `balance_cells`, `journal_rows`, `pending_op_rows`, `observations_per_invocation` as `usize` at `4..18`, `max_lifetime_work` as full `u128` at lane `18`, and `resume_rows` as `usize` at lane `19`). When `ext` is `None`, `zero(CTX_S, l, 4, 20)` enforces that all 16 lanes are `0` (`NonZeroPadding`).
  - Lanes `20..22`: `C6CodecBudgets { type_expr_nodes, template_bytes }`.

---

### 2. `SOURCE_SECURITY` Assessment — **AGREE**

#### 2.1 Context Authority and Pre-Decode Enforcement
- In `decode_host` (lines 410–419), `META` and `CTX` are decoded first, and their decoded `(meta.layout, ext, budgets)` are compared against the caller's trusted `config: &C6HostConfig` **before** `decode_program` or `decode_state` is invoked:
  ```rust
  for (same, field) in [(meta.layout == config.layout, "layout"), (ext == config.ext, "context.ext"),
      (budgets == config.budgets, "context.budgets")] {
      if !same { return Err(C6HostError::ConfigMismatch { field }); }
  }
  let ctx = config.state_context(limits);
  ```
- Program capacities (`&ctx.caps().phys`) and 3C state capacities (`&ctx`) derive exclusively from the caller's `config` and `limits`, never from untrusted header lanes.
- **Collision resistance over 3C descriptor**: 3C's 29-lane descriptor only projects `plan_case_slots`, `field_path_slots`, and `pattern_child_slots` from `ExtCapacitiesC`. Two configurations with identical 3C descriptors and `ext_present == true` but different reserved financial/persistent capacities (e.g., `journal_rows = 1` vs `0`) are refused by `decode_host` with `Err(C6HostError::ConfigMismatch { field: "context.ext" })`. Likewise, `ext = None` vs `ext = Some(ExtCapacitiesC::all_zero())` is distinguished at `CTX` lane 3 and refused.
- In `capture_host` (lines 444–446), `C6StateContext::from_layout(retained, budgets, state_limits) != config.state_context(limits)` cross-checks that `from_layout` and `from_parts` agree identically.

#### 2.2 Allocation and Work Contract Across All Public Helpers
- **`decode_host` (lines 387–424)**:
  1. Checks `admit_total(lanes.len(), limits)` at line 389 before reading any lane or allocating.
  2. Validates all 14 outer header lanes, directory tags, minimum child lengths, checked offset sums, reserved lane 13, and exact total length (`o[4] == lanes.len()`) with **zero heap allocations**.
  3. In `decode_metadata` (lines 285–302): every `blob` checks `len <= limits.max_string_bytes`, cumulative `bytes <= limits.max_metadata_bytes`, and `len <= self.remaining()` **before** `Vec::with_capacity(len)` (lines 224–231). Every `width_list` checks cumulative `width_entries <= limits.max_width_entries` and `n <= self.remaining()` **before** `Vec::with_capacity(n)` (lines 267–271).
  4. `decode_context` performs zero heap allocations.
  5. After `ConfigMismatch` verification, `decode_program` is bounded by `ctx.caps().phys` (derived from `config.budgets` and `config.layout`), and `decode_state` checks `limits.max_state_lanes` (`ctx.admit_total`) and header descriptor before allocating table vectors bounded by `ctx.caps`.
- **`encode_host` (lines 342–363)**:
  - Checks `host.admission.layout != config.layout` first.
  - Calls `encode_metadata` (bounded by `limits.max_string_bytes`, `limits.max_metadata_bytes`, `limits.max_width_entries`), `encode_program` (bounded by `ctx.caps().phys`), `encode_context` (fixed 22 lanes), and `encode_state` (clones state tables in `C6OwnedState::from_image`, then checks `limits.max_state_lanes` and child caps).
  - Checks `admit_total(total, limits)` at line 355 before allocating the assembled `out` vector.
  - **Honesty of allocation contract**: As explicitly documented in `4-OWNER-CONTRACT-INTERFACE.txt` ("`encode_program` allocates before the outer total-lane check. That allocation is bounded by the caller-config program caps") and `c6_phys_host.rs` module docs (which claim pre-allocation `max_total_lanes` order only for `decode_host`), `encode_host` cannot pre-compute variable-length `META`/`PROG` lane counts without inspecting/encoding their variable-width strings and templates; child allocations during `encode_host` are bounded by the caller's metadata limits, program caps/budgets, and `max_state_lanes`, while `max_total_lanes` bounds the final assembled section. No source correction is required.
- **`compare_full_host_witness` (lines 534–551)**:
  - As established in R1/R2, `max_total_lanes` bounds only physical lane vectors; it does not bound witness DAG unfolding or $O(E^2)$ environment chain traversal inside `construct_plan_for` / `evaluate_uncached`.

#### 2.3 Security-Critical Owner Membership Binding (`bind_decoded_host` and `C6OwnerBoundHost`)
- **Private carrier (`C6OwnerBoundHost`, lines 455–460)**:
  - All fields (`decoded`, `owner`, `config`, `current_image_equal`) are private to `c6_phys_host`.
  - `bind_decoded_host` is the sole constructor.
  - `C6HostCapture` (lines 430–434) keeps its fields private and labels `C6HostCapture::from_snapshot` as `C6HostProvenance::CallerSnapshot` vs `capture_host` as `C6HostProvenance::OwnerCapture`.
- **Check order in `bind_decoded_host` (lines 474–501)**:
  1. Calls `capture(admitted, image)` **first** (line 476), enforcing the AB `Arc::ptr_eq` check between `admitted` and `image` before any snapshot field is trusted. An unrelated `(Admitted, MachineImage)` pair is immediately rejected with `Err(C6BindError::Observation(_))`, while an `Arc::clone` of the same `Admitted` succeeds.
  2. Calls `retained_layout_agrees_with_host(retained, &owner)` (line 477), verifying all layout fields and `owner.admission.bounds == retained.bounds`.
  3. Derives `config = C6HostConfig::from_retained_layout(retained, budgets)` and checks `config == decoded.config` (`"context"`).
  4. Compares `admission.identity`, `admission.bounds`, `admission.layout`, `admission.view` (full `ProgramSnapshot` structural equality, including unused functions, generic declaration templates, origins, plans, markers, and tables), `run_id.identity`, `run_id.fuel_input`, `run_id.stack_limit_input`, plus whole-struct `d.admission == o.admission && drun == orun`.
  5. Re-encodes both `o.admission.view` and `d.admission.view` with `encode_program(..., &phys)` under the caller-derived `phys` caps and requires exact lane equality (`"admission.view.lanes"`).
  6. Records `current_image_equal = d.image == o.image` honestly without enforcing state equality.
- **Unselected digests and residual state surface**:
  - Current owner admissions produce `DigestC::Unselected` for `layout_digest`, `bounds_digest`, and `program_membership_digest`, and `DigestC::Selected` bytes are unverified opaque data. `bind_decoded_host` never trusts digests for membership; it enforces full structural and encoded-lane equality of `ProgramSnapshot` against the live AB-bound owner capture.
  - When `current_image_equal == false`, `decoded_host().image` is known only to be a cap-valid decoded state section paired with the bound program and run inputs; it does **not** establish `StepInv`, reachability, valid transition history, counter/table-length consistency (`SI-05`/`SI-10`), checkpoint validity, or native proof authority. The docstrings state this accurately.

#### 2.4 Full-Host Witness Comparison (`compare_full_host_witness`)
- Calls `encode_host`, standalone `decode_host(...).into_host()`, `first_host_difference(host, &decoded)`, re-encode `encode_host(&decoded, ...) == lanes`, 3C `compare_uncached_witness` (including `check_live_outs` on the 33 snapshot fields + output node = 34 live-outs), `construct_plan_for` on both `host` and standalone `decoded`, and `evaluate_uncached` on `(plan, host)`, `(decoded_plan, &decoded)`, and cross `(plan, &decoded)`.
- Because `host == decoded` is verified first and `construct_plan_for` / `evaluate_uncached` are deterministic functions of `(&HostSnapshot, LifetimeUnit)`, witness preservation is entailed by whole-host roundtrip equality under the existing uncached witness implementation (R2), not an independent global conformance or soundness proof.

---

### 3. `NORMAL_API_COMPILE` Assessment — **TARGETED ABSTAIN on Truncated Suffix / AGREE on Visible Source**

#### 3.1 Verified Directly in the Provided Inline Source
1. **Registration (`crates/mori-core/src/sherlock/helper_machine/mod.rs`, `c3174ffb…`)**:
   - Single-line addition `pub mod c6_phys_host;` between `pub mod c6_phys_state;` and `pub mod witness_plan;`.
   - `#![forbid(unsafe_code)]` is satisfied (`c6_phys_host.rs` contains no `unsafe`).
2. **Cross-check against complete `NEW-3C-c6_phys_state.rs` (`6bc3f837…`) and `00-NEW-c6_phys_helper.rs` (`069faf60…`)**:
   - All imports from `super::c6_phys_state` (`compare_uncached_witness`, `decode_state`, `encode_state`, `retained_layout_agrees_with_host`, `C6OwnedState`, `C6StateContext`, `C6StateError`, `C6StateLimits`, `C6WitnessError`, `C6WitnessPreservation`, `STATE_HEADER_LANES`, `STATE_SECTION_TAG`) and `super::c6_phys_helper::C6HelperTables` exist with `pub` visibility and exact matching signatures.
   - `C6StateContext` private fields (`caps`, `ext_present`, `limits`) are accessed only through `pub` methods `C6StateContext::from_parts`, `C6StateContext::from_layout`, `ctx.caps()`, and `ctx.descriptor()`.
   - `C6OwnedState` (`pub root`, `pub frames`, `pub data`, `pub helper`) and `C6HelperTables` (`pub helper`, `pub result`, `pub tasks`) field names and types match the exhaustive destructuring in `image_from_parts` (lines 374–384).
3. **Internal Rust resolution in `c6_phys_host.rs`**:
   - `use super::*;` in `#[cfg(test)] mod tests` brings all private `use` declarations from the parent module into scope.
   - Struct literal field expressions in `decode_metadata` (`IdentityC`, `BoundsC`, `WidthsSnapshot`, `LayoutSnapshot`, `LayoutCapacitiesC`, `RunIdC`) and tuple expressions `(self.text()?, self.text()?)` evaluate in written left-to-right order in Rust, matching `MetaWriter`.
   - `C6HostCaptureError` (line 436) and `C6BindError` (line 462) wrap `HostViolation` (non-`Debug`, non-`PartialEq`) and therefore omit `#[derive(Debug, PartialEq, Eq)]`; all test call sites (`must_capture` at line 732, `bind` match helpers at lines 820–837) use pattern matching (`match` / `matches!`) and never require `C6HostCaptureError: Debug` or `C6BindError: Debug`.

#### 3.2 Targeted Abstention Due to Inline Prompt Truncation (`<truncated 1069008 bytes>`)
The inline prompt was truncated at `<truncated 1069008 bytes>` at line 695 of `00-NEW-c6_phys_helper.rs`. Because tool/filesystem access is forbidden, I must **ABSTAIN** from claiming direct source inspection of the truncated prerequisite files (`c6_phys.rs`, `c6_phys_program.rs`, `c6_phys_data.rs`, `complete_observation.rs`, `shared/*`, `witness_plan.rs`, and the 3,500B Root QA disposition). For `c6_phys_host.rs` to compile cleanly in `mori-core`, those truncated modules must provide the following exact items (consistent with 3C/3B usage):

```rust
// super::c6_phys
pub const C6_PHYS_MAGIC: u128;
pub const C6_PHYS_VERSION: u128;
pub type C6Result<T> = Result<T, C6PhysError>;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct C6CodecBudgets { pub type_expr_nodes: usize, pub template_bytes: usize }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6Root {
    pub control: ControlC, pub control_origin: usize, pub env_root: Option<usize>,
    pub stack_limit: usize, pub outcome: OutcomeC, pub initial_fuel: u128,
    pub remaining_work: u128, pub consumed_work: u128, pub helper_work: u128,
    pub helper_visits: u128, pub reference_cursor: u128, pub alloc: AllocCounters,
    pub active: ActiveLengths, pub seg_active: ActiveLengths, pub seg_alloc: AllocCounters,
    pub seg_consumed_work: u128,
}
#[cfg(test)]
pub(crate) mod fixtures {
    pub(crate) const FUEL: u128;
    pub(crate) fn must<T, E>(r: Result<T, E>, msg: &str) -> T;
}

// super::c6_phys_data
pub const fn u(x: usize) -> u128;
pub fn rd(section: &'static str, value: u128) -> C6Result<usize>;
pub fn rd_bool(section: &'static str, value: u128) -> C6Result<bool>;
pub fn zero(section: &'static str, lanes: &[u128], start: usize, end: usize) -> C6Result<()>;
pub fn cap_count(section: &'static str, count: usize, limit: usize) -> C6Result<()>;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct C6DataTables {
    pub data: Vec<DataSnapshot>, pub data_fields: Vec<RefC>,
    pub env: Vec<EnvCellC>, pub value_slots: Vec<RefC>,
}

// super::c6_phys_program
pub const PROGRAM_SECTION_TAG: u128;
pub const PROGRAM_HEADER_LANES: usize;
pub fn encode_program(view: &ProgramSnapshot, caps: &C6PhysCaps) -> C6Result<Vec<u128>>;
pub fn decode_program(lanes: &[u128], caps: &C6PhysCaps) -> C6Result<ProgramSnapshot>;
```
Plus `LayoutSchemaC: Clone` (in `shared::owner_prepare`), `WidthsC::{index_width_entries, counter_width_entries, record_width_entries}`, `ExtCapacitiesC::all_zero()`, `BoundsC: Default`, `SegBase::from_parts`, `complete_observation::{capture, record_step}`, and `witness_plan::{construct_plan_for, evaluate_uncached}`.

---

### 4. Review of the Five Unrun Tests in `c6_phys_host.rs`

1. **`metadata_literal_lanes_inverse_mutations_and_refusals` (lines 590–661)**:
   - Literal 19-lane prefix `[C6_PHYS_MAGIC, C6_PHYS_VERSION, META_SECTION_TAG, 255, 1, 1, 4, 0xCE, 0xBD, 0, 0x78, 0, 0, 1, 1, 0x66, 1, 0x31, 0]` and 4-lane tail `[2, 0, 1, u128::MAX]` match `synthetic_meta()` byte-for-byte (`"ν\0x"` = `[0xCE, 0xBD, 0x00, 0x78]`, `Unselected` = `[0]`, `Selected(("f","1"), [])` = `[1, 1, 0x66, 1, 0x31, 0]`).
   - All 19 mutations produce distinct encodings and roundtrip.
   - Budget failure thresholds (`max_string_bytes: 3` -> `needed: 4` at `"ν\0x"`; `max_metadata_bytes: 5` -> `needed: 6` at `"1"`; `max_width_entries: 3` -> `needed: 4` at `record_widths`) and lane error positions (`u32::MAX` width lane occurs uniquely at `index_widths[0].1`; `NonZeroPadding` at `n - 3`) are exact.
   - `synthetic_meta()` is properly labeled `SYNTHETIC, unadmitted`.
2. **`context_section_presence_every_field_and_padding` (lines 664–689)**:
   - Verifies `None` vs `Some(ExtCapacitiesC::all_zero())`, all 16 `ExtCapacitiesC` lanes (`4..20`) significant, non-zero padding in `4..20` when `ext == None` refused with `NonZeroPadding`, `max_lifetime_work: u128::MAX`, `BadBool` on lane 3, and `Truncated` at 21 lanes.
3. **`owner_capture_standalone_inverse_outer_refusals_and_context_binding` (lines 736–807)**:
   - Verifies `OwnerCapture` provenance, 14-lane header, contiguous `[meta, prog, ctxl, state]` child slices, standalone `decode_host`, outer refusals, reserved `ext.journal_rows = 1` collision refused with `ConfigMismatch { field: "context.ext" }` despite equal 3C `descriptor()` and `caps()`, lane `HOST_HEADER_LANES + m + p + 4 + 11` (`journal_rows` in `CTX`) acceptance when paired with `reserved`, and `RunIntC::Negative` genesis preservation.
4. **`owner_binding_refuses_other_program_unrelated_pair_and_other_run_inputs` (lines 810–838)**:
   - Verifies that adding an unreferenced `unused` function preserves `IdentityC` (`DigestC::Unselected`) and `C6HostConfig` while changing `ProgramSnapshot` (`view`) and failing `bind_decoded_host` with `Mismatch { field: "admission.view" }`. Verifies `Arc::clone` acceptance, unrelated pair AB failure (`C6BindError::Observation`), fuel/stack/context mismatches, and `current_image_equal() == false` after stepping.
5. **`owner_trace_full_host_decode_preserves_uncached_witness` (lines 841–872)**:
   - Traces nested `Box`/`Pair` evaluation through `OutcomeC::Halted`, checking `compare_full_host_witness` on `RecordBefore`, `RecordAfter`, and terminal `Snapshot` without `.or(actual)` or skip-on-failure.

#### Scope of Untested Cases in `c6_phys_host.rs` (Optional Follow-Ups, Non-Blocking)
- **Metadata mutations not individually toggled in Test 1's 19-entry table**: the other 20 `BoundsC` fields, the other 3 `LayoutCapacitiesC` fields, `layout.field_id`, `layout.scalar_layout_id`, `counter_widths`/`record_widths` ordering/duplicates, `digest.format` strings, `run_id.identity` non-profile fields, and `stack_limit_input = RunIntC::Negative`.
- **Refusal branches not individually triggered in Tests 1–4**: `decode_metadata` `BadMagic`/`BadVersion`/`BadBool` (on `boundary_capacities_present`) and `Truncated` inside `blob`/`width_list`; `decode_context` `BadMagic`/`BadVersion`/`BadTag`/`TrailingLanes`; `decode_host` `Truncated` (`< 14` or `< o[4]`), `BadVersion`, `ChildLengthRange` on `Metadata`/`Program`/`State`, `Child` errors on `Program`/`Context`, and `C6HostError::State`; `C6HostCapture::from_snapshot`; `bind_decoded_host` mismatch on `"admission.identity"`, `"admission.bounds"`, `"admission.layout"`, `"run_id.identity"`, and `"admission.view.lanes"`; and `compare_full_host_witness` error variants.

---

### 5. Mandatory vs. Optional Corrections and Open Obligations

- **Mandatory source corrections to `c6_phys_host.rs` / `helper_machine/mod.rs`**: **None.**
- **Optional follow-ups**:
  1. Add unit test assertions covering `C6HostCapture::from_snapshot` (`CallerSnapshot` provenance) and the remaining `decode_host` / `bind_decoded_host` error variants listed above.
- **Runtime, qualification, and roadmap obligations still OPEN**:
  1. **Root normal toolchain execution**: Compile `mori-core` with `c6_phys_host` registered and execute the 5 new `c6_phys_host` tests, the 6 unrun 3B tests, the 4 unrun 3C tests, the 18 existing Core tests, the 3 3A tests, and the separate `c6_a135_push` SDK test (36 host tests total).
  2. **T1 literal vector obligation**: Explicit literal lane vectors for the remaining 9 `HelperControlC` variants and 18 `HelperResultC` variants in 3B remain open before ABI publication.
  3. **Detached commitment & import**: Cryptographic verification of `DigestC::Selected`, detached host authentication, and validated checkpoint/import (`StepInv`/reachability enforcement) remain open.
  4. **Downstream roadmap**: Full 35 UIREQ / 25 features / 68 scenarios, financial/persistent execution, all 39 selectors/partition/rule constraints, grouping 1/2/4/8 earliest-stop/free-publication/run-barriers, finite immutable-key residual baseline, and Chunk 5 native constraints/emitter/prover/verifier/ledger settlement remain open.
