# Independent Reviewer A Assessment — Chunk 4 (`c6_phys_host.rs` + `helper_machine/mod.rs`)

- **Reviewer Identity:** `Gemini 4 Argon HIGH` (via AGY fallback under standing user-authorized outage routing following Opus monthly spend-limit error; acting in the assigned Reviewer A slot).
- **Candidate Reviewed:**
  - `crates/mori-core/src/sherlock/helper_machine/c6_phys_host.rs` (`56,823` bytes, SHA256 `a172431ffbefc9ceca3f595a54f9a4bf8314cba63f7e3b5b3c1cf5e0da677547`, 599 lines — 100% present in inline prompt).
  - `crates/mori-core/src/sherlock/helper_machine/mod.rs` (`1,265` bytes, SHA256 `c3174ffbca050ec5db725facff6593c2c328065023144b9651d9a0464f74041a`, 45 lines — 100% present in inline prompt).
  - `4-OWNER-CONTRACT-INTERFACE.txt` (`6,685` bytes, SHA256 `f006c8ba1f02ce128683f6223cf04206c0ae7f512d88b6b02b60f652b13f1853`) and `4-OWNER-RESULT.json` (`907` bytes).
- **Prerequisites Inspected from Inline Prompt:**
  - `NEW-3C-c6_phys_state.rs` (`52,511` bytes, SHA256 `6bc3f8370b2a6f579ce665a6ebc8bdb127ae8d1557c309cc9f35fe869f075eaa` — 100% present).
  - `3C-OWNER-CONTRACT-INTERFACE.txt` (`5,901` bytes, SHA256 `54ed62f2805810683b3513b60eb28b0996945969176191e3009c54d1a549bde9` — 100% present).
  - `00-NEW-c6_phys_helper.rs` (`63,237` bytes, SHA256 `069faf60e63928e77586f130e5d6144adf61c1c1f44875e9bc32dea95d71dddc` — 100% of production code present; prompt transport truncated `<truncated 1069008 bytes>` inside `#[cfg(test)] mod tests` at `admitted_with_layout`).
- **Execution Discipline:** Passive source/design/API review only. Zero tools, commands, filesystem reads/writes, compilers, or tests were executed.

---

## 1. Summary Verdict

| Dimension | Verdict | Basis |
| :--- | :--- | :--- |
| **DESIGN** | **AGREE** | Sound 14-lane `HOST` v1 framing over 4 fixed contiguous children (`META`, `PROG`, `CTX`, `STATE`), complete standalone `HostSnapshot` reconstruction without prior host state, exact canonicality across all metadata/context/program/state fields, and explicit non-authority boundaries. |
| **SOURCE_SECURITY** | **AGREE** | Trusted caller `C6HostConfig` (`layout`, full 16-field `Option<ExtCapacitiesC>`, `C6CodecBudgets`) is verified before `PROG` or `STATE` decode; `bind_decoded_host` enforces AB `Arc::ptr_eq` via `capture` first, checks retained layout agreement, and requires both structural and re-encoded lane equality of `ProgramSnapshot` rather than trusting `Unselected`/opaque digests. |
| **NORMAL_API_COMPILE** | **AGREE** *(with targeted ABSTAIN on transport-truncated prerequisite bodies)* | All types, visibilities, private-field boundaries (`C6StateContext`, `C6OwnerBoundHost`), non-`Debug` error handling (`HostViolation`), struct literal evaluation order, and the unique `mod.rs` registration hunk resolve cleanly against the visible 3C and 3B modules. Targeted **ABSTAIN** is recorded solely for prerequisite bodies in the transport-truncated 1,069,008-byte tail (`c6_phys.rs`, `c6_phys_program.rs`, `c6_phys_data.rs`, `complete_observation.rs`, `shared/*`, `witness_plan.rs`). |
| **RUNTIME / QA** | **ABSTAIN** | All 5 new `c6_phys_host` tests, the 4 `c6_phys_state` (3C) tests, and the 6 `c6_phys_helper` (3B) tests remain unrun in this review; Root owns normal whole-`mori-core` compilation and test execution. |
| **OVERALL** | **AGREE** | **0 mandatory owner source corrections required.** Ready for Root normal whole-Core compile and test qualification. |

---

## 2. DESIGN Review

### 2.1 Consequential Whole-Host Framing (`HOST` Schema 1)
- **Header Layout (`c6_phys_host.rs:54-80, 389-397, 420-441`):**
  - Fixed 14-lane header (`HOST_HEADER_LANES = 14`):
    - Lane `0`: `C6_PHYS_MAGIC`
    - Lane `1`: `C6_PHYS_VERSION`
    - Lane `2`: `HOST_SECTION_TAG` (`0x484F_5354`, `"HOST"`)
    - Lane `3`: `C6_HOST_SCHEMA` (`1`)
    - Lane `4`: `HOST_CHILD_COUNT` (`4`)
    - Lanes `5..=12`: Fixed-order 4-entry directory `(tag, exact_length)` for `Metadata` (`0x4D45_5441`, min `3`), `Program` (`PROGRAM_SECTION_TAG`, min `PROGRAM_HEADER_LANES`), `Context` (`0x4354_5854`, min `22`), and `State` (`0x5354_4154`, min `41`).
    - Lane `13`: Reserved padding lane, enforced zero via `zero(HOST_S, lanes, 13, 14)`.
- **Non-Overlapping Contiguous Children & Refusal Precedence (`c6_phys_host.rs:420-453`):**
  - `decode_host` enforces a deterministic, allocation-free outer check sequence before any child decoder runs:
    1. `admit_total(lanes.len(), limits)` (`C6HostError::LimitExceeded { scope: "host.total_lanes", needed, limit }`).
    2. `lanes.len() < 14` (`Outer(Truncated { section: "host", needed: 14, actual })`).
    3. `BadMagic` (lane 0), `BadVersion` (lane 1), `BadTag` on `"host"` (lane 2), `BadTag` on `"host.schema"` (lane 3), `BadSectionCount` (lane 4).
    4. For each child `0..4` in order: `BadChildTag`, `rd(HOST_S, lanes[at + 1])` (`Outer(UsizeOverflow)`), `declared < child.minimum_lanes()` (`ChildLengthRange`), and checked cumulative offset `o[i + 1] = add(o[i], declared)?` (`Outer(UsizeOverflow)`).
    5. Reserved lane 13 zero check (`Outer(NonZeroPadding)`).
    6. Exact total slice match: `lanes.len() < o[4]` (`Outer(Truncated)`) and `lanes.len() > o[4]` (`Outer(TrailingLanes)`).
  - Children occupy disjoint contiguous slices `o[0]..o[1]` (`META`), `o[1]..o[2]` (`PROG`), `o[2]..o[3]` (`CTX`), and `o[3]..o[4]` (`STATE`). Parent `PROG` and `STATE` payloads are exact, unmodified outputs of `encode_program` and `encode_state`.
- **Standalone Inverse & Non-Authority (`c6_phys_host.rs:402-416, 451-453`):**
  - Unlike 3C's `host_with_decoded_state` (`c6_phys_state.rs:256-261`), which clones an existing `HostSnapshot` and retains its `admission` and `image.run_id`, `decode_host` takes only `(lanes, config, limits)` and reconstructs a complete `HostSnapshot` (`AdmissionSnapshot { identity, bounds, view, layout }` and `ImageSnapshot` via exhaustive `image_from_parts(run_id, state)`).
  - `C6DecodedHost` wraps the owned diagnostic `HostSnapshot` and `C6HostConfig`; it never constructs or seals `Admitted`, `MachineImage`, `StepInv`, or native proof artifacts.

### 2.2 Exhaustive Field, Domain, and Canonicality Assessment
- **`IdentityC` (`c6_phys_host.rs:170-179, 276-284`):**
  - `profile_id: u8`: encoded as `u128::from(*profile_id)` and decoded via `u8::try_from(profile)` refusing `>= 256` with `C6PhysError::UsizeOverflow { section: "host.meta", value: profile }`.
  - `grammar_domain`: strict binary tag (`0 => DedicatedSorts`, `1 => GenericNominalExtensionReserved`, `> 1 => BadTag`).
  - `finance`: strict binary tag (`0 => Absent`, `1 => TypedExtensionReserved`, `> 1 => BadTag`).
  - `layout_version: (String, String)` and `field_id: Option<(String, String)>` (`0 => None`, `1 => Some(pair)`, `> 1 => BadTag`).
  - `layout_digest`, `bounds_digest`, `program_membership_digest` (`DigestC`):
    - `0 => DigestC::Unselected`
    - `1 => DigestC::Selected { format: (String, String), bytes: Vec<u8> }` (`> 1 => BadTag`).
    - Selected digest format strings and raw `bytes` (1 lane per byte, each checked `<= 255`) roundtrip losslessly as opaque data.
- **Dual Independent `BoundsC` (`c6_phys_host.rs:180-191, 226, 231, 285-296`):**
  - All 22 fields of `BoundsC` are destructured exhaustively: 19 `usize` bounds (checked via `rd(META_S, v)`) followed by the 3 full `u128` work bounds (`max_reference_work`, `max_helper_work`, `step_cap`, read/written via `raw()`/`v()` without `usize` truncation).
  - `host.admission.bounds` and `host.admission.layout.bounds` are encoded and decoded independently in `META`.
- **`LayoutSnapshot` & Raw Ordered Width Lists (`c6_phys_host.rs:192-203, 227-235, 297-314, 332-336`):**
  - Every `LayoutSnapshot` field is preserved: `layout_version`, `field_id`, `widths`, `scalar_layout_id`, `bounds` (22 fields), `capacities` (4 `usize` fields), and `boundary_capacities_present` (`rd_bool` `0 | 1`).
  - `widths: Option<WidthsSnapshot>` distinguishes `None` (`[0]`) from `Some` with three empty vectors (`[1, 0, 0, 0]`).
  - Within `Some(WidthsSnapshot)`, each of `index_widths`, `counter_widths`, and `record_widths` is encoded as a length-prefixed sequence of `(String, u32)` pairs in raw slice order, preserving duplicate keys and unsorted order verbatim. Each width lane is range-checked via `u32::try_from(w)` (`UsizeOverflow` on `>= 2^32`).
- **Strings, Bytes, UTF-8, NUL, and Unicode (`c6_phys_host.rs:154-162, 256-274`):**
  - Strings and byte blobs are encoded as `[len, b_0, ..., b_{len-1}]`. Each byte lane is checked via `u8::try_from(lane)` (`BadTag { section: "host.meta", tag: lane }` on `> 255`), and strings are validated by `String::from_utf8(bytes)` (`BadTag { section: "host.meta", tag: u(len) }` on invalid UTF-8). Valid UTF-8 containing embedded NUL (`\0`) and multi-byte Unicode codepoints (`ν`) is preserved exactly.
- **`RunIdC`, 3-Class `RunIntC`, and Invalid-Genesis Non-Normalization (`c6_phys_host.rs:204-208, 236-238, 315-321, 337`):**
  - `run_id.identity: IdentityC` is encoded and decoded independently of `admission.identity`.
  - `fuel_input` and `stack_limit_input` encode all 3 `RunIntC` variants: `Negative` (`[0, 0]`), `Nat(n)` (`[1, n]` for any `u128`), and `NatAboveU128` (`[2, 0]`), rejecting non-zero payloads on tags `0` and `2` with `NonZeroPadding { section: "host.meta", lane: self.at - 1, value: n }`.
  - When genesis fails on a negative fuel/stack input (`initial_fuel == 0`, `stack_limit == 0`, `control == ControlC::Error { code: 1 }`), `META` preserves `RunIntC::Negative` in `run_id` while `STATE` preserves `0` in `C6Root` without normalizing one from the other.
- **Full `ProgramSnapshot` and Exhaustive `ImageSnapshot` (`c6_phys_host.rs:386, 409-416, 449-450`):**
  - `PROG` delegates directly to `encode_program`/`decode_program`, preserving unused functions, generic declaration templates, origins, match plans, markers, and all program tables.
  - `image_from_parts` exhaustively destructures `C6OwnedState { root, frames, data, helper }`, `C6Root` (all 16 fields, reconstructing `seg_base: SegBase::from_parts(seg_active, seg_alloc, seg_consumed_work)`), `C6DataTables` (4 fields), and `C6HelperTables` (3 fields) into `ImageSnapshot`. Stored `alloc`/`active` counters remain decoupled from table lengths; no silent normalization or admission occurs.

---

## 3. SOURCE_SECURITY Review

### 3.1 Context Authority (`C6HostConfig` vs Untrusted Lanes)
- **Why 3C's Descriptor Is Insufficient Alone:**
  - In `c6_phys_state.rs:117-120`, `C6StateContext::from_parts` applies `ExtCapacitiesC::effective_for_extended_domain(ext)`, which projects only 3 extension fields (`plan_case_slots`, `field_path_slots`, `pattern_child_slots`) into `C6PhysCaps` while ignoring the 13 reserved financial/persistent fields of `ExtCapacitiesC` (`journal_rows`, `asset_rows`, `max_lifetime_work`, etc.). Moreover, `HostSnapshot.admission.layout` (`LayoutSnapshot`) omits `ext_capacities` and `C6CodecBudgets`.
- **Chunk 4 Resolution (`c6_phys_host.rs:123-139, 343-376, 442-450`):**
  - `C6HostConfig` retains the caller's full `layout: LayoutSnapshot` (with raw ordered width lists), `ext: Option<ExtCapacitiesC>` (all 16 fields, keeping `None` distinct from `Some(ExtCapacitiesC::all_zero())`), and `budgets: C6CodecBudgets`.
  - `CTX` (22 fixed lanes) encodes the presence boolean (`lane 3`), all 16 `ExtCapacitiesC` fields in declaration order (`lanes 4..20`, with `max_lifetime_work` as full `u128` at lane 18, and `zero(CTX_S, l, 4, 20)` enforced when `lane 3 == 0`), and `budgets` (`lanes 20..22`).
  - In `decode_host` (`lines 442-450`), `META` and `CTX` are decoded first and compared against the caller's `config` (`meta.layout == config.layout`, `ext == config.ext`, `budgets == config.budgets`) **before** `decode_program` or `decode_state` is invoked.
  - Consequently:
    - Any collision where two configs share identical 3C `descriptor()` and `caps()` but differ in a reserved `ExtCapacitiesC` field (e.g., `journal_rows`) or in `None` vs `Some(all_zero)` is deterministically refused as `C6HostError::ConfigMismatch { field: "context.ext" }`.
    - Any difference in `layout_version`, `field_id`, raw width order/duplicates, `scalar_layout_id`, `bounds`, `capacities`, or `boundary_capacities_present` is refused as `ConfigMismatch { field: "layout" }`.
    - Any difference in `C6CodecBudgets` is refused as `ConfigMismatch { field: "context.budgets" }`.
    - Capacities passed to `decode_program` and `decode_state` (`ctx = config.state_context(limits)`) come exclusively from the trusted caller `config` and `limits`, never from untrusted input lanes.
  - In `capture_host` (`lines 475-478`), `C6StateContext::from_layout(retained, budgets, state_limits) != config.state_context(limits)` is also explicitly verified.

### 3.2 Precise Allocation and Work Contract Across All Public Helpers
| Public Helper | Allocation / Work Bound Contract & Error Precedence |
| :--- | :--- |
| `encode_metadata` | Checks `max_string_bytes` and cumulative `max_metadata_bytes` in `MetaWriter::blob` (which applies to both strings and `DigestC::Selected.bytes` — note that a selected digest byte vector `> max_string_bytes` is refused under `"host.meta.string_bytes"`) and cumulative `max_width_entries` in `MetaWriter::widths` before extending `w.out`. Does not check `max_total_lanes`. |
| `decode_metadata` | Checks header (3 lanes, magic, version, tag) before reading. In `MetaReader::blob`, checks `max_string_bytes`, `checked_add`, `max_metadata_bytes`, and `len <= self.remaining()` **before** `Vec::with_capacity(len)`. In `MetaReader::width_list`, checks `checked_add`, `max_width_entries`, and `n <= self.remaining()` **before** `Vec::with_capacity(n)`. |
| `encode_context` / `decode_context` | Fixed 22-lane stack array (`[u128; 22]`); zero heap allocations. Ascending lane validation (`0..22`). |
| `encode_host` | **Important pre-limit allocation behavior (honestly documented by owner):** Checks `host.admission.layout != config.layout` first, then clones metadata (`C6HostMetadata::from_host`), encodes `meta` (bounded by metadata limits), encodes `prog` via `encode_program` (allocating the `prog` vector bounded by caller `ctx.caps().phys`), encodes `context` (22 lanes), clones `C6OwnedState::from_image(&host.image)` and encodes `state` via `encode_state` (bounded by `limits.max_state_lanes` and `ctx.caps`), and only then checks `admit_total(total, limits)` (`limits.max_total_lanes`) at line 392 before allocating the combined `out` vector. |
| `decode_host` | Checks `admit_total(lanes.len(), limits)` (`max_total_lanes`) at line 420 **first**, then validates the 14-lane header, child minimums, checked offset sums, reserved lane, and exact total length `o[4]` with **zero heap allocations**. Then decodes `META` (bounded by metadata limits and slice length), decodes `CTX` (stack), verifies `config` equality (`layout`, `context.ext`, `context.budgets`), and only then invokes `decode_program` and `decode_state` under caller-config capacities. |
| `C6HostConfig::from_retained_layout` / `C6HostMetadata::from_host` | Pure in-memory clones of caller's `LayoutSchemaC` / `HostSnapshot`; no `C6HostLimits` check. |
| `C6HostCapture::from_snapshot` / `capture_host` | `capture_host` runs `capture(admitted, image)` (AB `Arc::ptr_eq` check first, then clones `HostSnapshot`), checks `retained_layout_agrees_with_host`, builds `C6HostConfig`, checks `from_layout == state_context`, and calls `encode_host`. `from_snapshot` calls `encode_host` then clones `host` and `config`. |
| `bind_decoded_host` | Runs `capture(admitted, image)` (AB check first, then clones owner `HostSnapshot`), checks `retained_layout_agrees_with_host`, compares `config`, `admission`, and `run_id`, and allocates two `encode_program` vectors (bounded by caller `phys` caps, not `limits.max_total_lanes`) to verify canonical program lane equality before cloning `decoded.host`. |
| `compare_full_host_witness` | Calls `encode_host`, `decode_host`, `encode_host`, `compare_uncached_witness`, two `construct_plan_for` calls, and three `evaluate_uncached` calls. **R1 bound limitation:** `limits.max_total_lanes` and `limits.max_state_lanes` do **not** bound witness DAG unfolding or `O(E^2)` environment traversal work in `construct_plan_for` / `evaluate_uncached`. |

The candidate's doc comments (`c6_phys_host.rs:17-32, 105-113, 483-488, 552-554`) and `4-OWNER-CONTRACT-INTERFACE.txt` state these preconditions and boundaries accurately; no source correction is required.

### 3.3 Owner Membership Binding (`bind_decoded_host` & `C6OwnerBoundHost`)
- **Constructor & Provenance Integrity (`c6_phys_host.rs:483-528`):**
  - `C6OwnerBoundHost` has private fields (`decoded`, `owner`, `config`, `current_image_equal`) and can only be constructed by `bind_decoded_host`.
  - `bind_decoded_host` executes:
    1. `let owner = capture(admitted, image).map_err(C6BindError::Observation)?;` — enforces the AB `Arc::ptr_eq` binding check on `(admitted, image)` before any field read.
    2. `retained_layout_agrees_with_host(retained, &owner).map_err(C6BindError::RetainedLayout)?;` — verifies that `retained: &LayoutSchemaC` matches every layout field of `owner` and `owner.admission.bounds == retained.bounds`.
    3. `C6HostConfig::from_retained_layout(retained, budgets) == decoded.config` (`Mismatch { field: "context" }`).
    4. Field-by-field and whole-struct equality of `admission.identity`, `admission.bounds`, `admission.layout`, `admission.view` (`ProgramSnapshot`), `run_id.identity`, `run_id.fuel_input`, and `run_id.stack_limit_input`.
    5. Canonical re-encoding check: `encode_program(&o.admission.view, &phys) == encode_program(&d.admission.view, &phys)` (`Mismatch { field: "admission.view.lanes" }`).
- **Why Full View Equality Is Required:**
  - All current owner admissions emit `DigestC::Unselected` for `layout_digest`, `bounds_digest`, and `program_membership_digest`, and `DigestC::Selected` carries only unverified opaque bytes. Therefore, `IdentityC` digests cannot authenticate program membership. Comparing the full `ProgramSnapshot` (`d.admission.view == o.admission.view`) and its canonical encoded lanes against the live AB-verified `owner` capture prevents any caller-supplied or colliding `IdentityC` (such as a program differing only in an unused function) from passing `bind_decoded_host`.
- **What Arbitrary State Remains Possible After Binding:**
  - `bind_decoded_host` does **not** enforce `d.image == o.image`; it records `current_image_equal = d.image == o.image`.
  - When `current_image_equal()` is `false`, `decoded_host().image` can hold an arbitrary decoded `C6OwnedState` (arbitrary root registers, unnormalized `alloc`/`active`/`seg_base` counters, arbitrary frames, data nodes, environments, or helper tasks) that is within the codec's syntactic/capacity domain but is **not** guaranteed to satisfy `StepInv`, reachability from genesis, or valid step transitions.
  - Neither `C6OwnerBoundHost` nor `C6HostCapture` grants `MachineImage` import, checkpoint restoration, or native proof authority.

### 3.4 Full Host Witness Comparison (`compare_full_host_witness`)
- `compare_full_host_witness` (`c6_phys_host.rs:555-573`) encodes the whole host (`encode_host`), performs a genuinely standalone `decode_host(&lanes, config, limits)` (not 3C's clone-replace), checks `first_host_difference(host, &decoded)` and canonical re-encoding `encode_host(&decoded, config, limits) == lanes`, runs 3C's `compare_uncached_witness` (including `check_live_outs` over the 33 snapshot fields + output node), and then builds `construct_plan_for(host, lifetime)` and `construct_plan_for(&decoded, lifetime)` and runs original, decoded, and cross `evaluate_uncached` evaluations.
- **R2 Entailment:** Because `construct_plan_for` and `evaluate_uncached` are deterministic functions of `&HostSnapshot` and `LifetimeUnit`, once `decoded == *host` holds, plan and evaluation equality (`plan == decoded_plan == state.plan` and `evaluation == decoded_evaluation == cross`) is entailed by whole-`HostSnapshot` equality under the existing witness evaluator. It validates standalone whole-host roundtrip fidelity for witness inputs, not independent global encoder conformance or native constraint soundness.

---

## 4. NORMAL_API_COMPILE & Unrun Test Verification

### 4.1 API, Visibility, and Type Resolution
- **Registration Hunk (`helper_machine/mod.rs:20`):** Adds `pub mod c6_phys_host;` uniquely between `pub mod c6_phys_state;` and `pub mod witness_plan;`. `#![forbid(unsafe_code)]` is respected (`0` unsafe blocks).
- **3C & 3B Cross-Module Resolution:**
  - Every item imported from `super::c6_phys_state` (`compare_uncached_witness`, `decode_state`, `encode_state`, `retained_layout_agrees_with_host`, `C6OwnedState`, `C6StateContext`, `C6StateError`, `C6StateLimits`, `C6WitnessError`, `C6WitnessPreservation`, `STATE_HEADER_LANES`, `STATE_SECTION_TAG`) and `super::c6_phys_helper` (`C6HelperTables`) matches the exact public definitions in `NEW-3C-c6_phys_state.rs` and `00-NEW-c6_phys_helper.rs`.
  - `C6StateContext` has private fields in `c6_phys_state.rs:108`, and `c6_phys_host.rs` accesses it exclusively via its public constructors (`from_parts`, `from_layout`) and public accessors (`caps()`, `descriptor()`).
  - `C6CodecBudgets { type_expr_nodes, template_bytes }` field destructuring/construction (`c6_phys_host.rs:356, 375, 588`) matches its direct struct literal construction in `c6_phys_state.rs:354, 410`.
  - Non-`Debug` `HostViolation` inside `C6HostCaptureError` and `C6BindError` is handled without deriving `Debug`/`PartialEq` and without calling `.unwrap()` or `assert_eq!` on those error types in tests (`c6_phys_host.rs:468, 493, 744, 830-845`).
  - Sequential struct literal field evaluation in `MetaReader::identity`, `MetaReader::bounds`, and `decode_metadata` (`c6_phys_host.rs:282-295, 332-337`) is guaranteed left-to-right by the Rust language specification and matches `MetaWriter` / `encode_metadata` lane order.
- **Targeted ABSTAIN on Transport-Truncated Prerequisite Bodies:**
  - Because the prompt transport truncated `<truncated 1069008 bytes>` inside `00-NEW-c6_phys_helper.rs` test code, the literal bodies of `c6_phys.rs`, `c6_phys_program.rs`, `c6_phys_data.rs`, `complete_observation.rs`, `shared/*`, and `witness_plan.rs` were not delivered in the inline text. While their signatures are corroborated by `c6_phys_state.rs` and `c6_phys_helper.rs`, I record a **targeted ABSTAIN** on directly inspecting those truncated prerequisite bodies (such as `encode_program`/`decode_program` in `c6_phys_program.rs` and `WidthsSchemaC` methods in `shared`).

### 4.2 Review of the Five Unrun Tests (`c6_phys_host.rs:575-881`)
1. **`metadata_literal_lanes_inverse_mutations_and_refusals` (`lines 605-675`):**
   - **Literal prefix (`lanes[..19]`):** `[C6_PHYS_MAGIC, C6_PHYS_VERSION, META_SECTION_TAG, 255, 1, 1, 4, 0xCE, 0xBD, 0, 0x78, 0, 0, 1, 1, 0x66, 1, 0x31, 0]` matches the exact encoding of `profile_id = 255`, `grammar = 1`, `finance = 1`, `layout_version = ("ν\0x", "")` (`"ν\0x"` = `[0xCE, 0xBD, 0, 0x78]`), `layout_digest = Unselected` (`0`), and `bounds_digest = Selected { format: ("f", "1"), bytes: [] }` (`[1, 1, 0x66, 1, 0x31, 0]`).
   - **Literal suffix (`lanes[n - 4..]`):** `[2, 0, 1, u128::MAX]` matches `NatAboveU128` (`[2, 0]`) and `Nat(u128::MAX)` (`[1, u128::MAX]`).
   - **19 mutations & refusals:** All 19 field mutations produce pairwise distinct lane vectors and roundtrip; refusals verify bad tag (`lane 2`, `lane 4`, `lane 13`), `profile_id` overflow (`256` at `lane 3`), byte overflow (`0x1CE` at `lane 7`), invalid UTF-8 (`0xFF` at `lane 7` reporting byte length `4`), truncation, trailing lane, `RunIntC` non-zero padding (`lane n - 3`), `u32` width overflow (`1 << 32`), and exact `max_string_bytes` (`4 > 3`), `max_metadata_bytes` (`6 > 5`), and `max_width_entries` (`4 > 3`) refusals on both encode and decode.
2. **`context_section_presence_every_field_and_padding` (`lines 677-702`):**
   - Verifies `None` vs `Some(ExtCapacitiesC::all_zero())` (differing at `lane 3`), loops over all 16 `ExtCapacitiesC` lanes (`4..20`) to verify both significance when present and `NonZeroPadding` refusal when absent, and checks `max_lifetime_work = u128::MAX`, `BadBool` at `lane 3`, and 21-lane `Truncated`.
3. **`owner_capture_standalone_inverse_outer_refusals_and_context_binding` (`lines 748-816`):**
   - Verifies real generic nominal `Box<A>`/`Pair` capture, 14-lane header directory (`[magic, version, HOST, 1, 4, META, m, PROG, p, CTX, 22, STATE, s, 0]`), contiguous child slices, standalone `decode_host` roundtrip, `LimitExceeded` on `decode_host` and `encode_host`, `BadMagic`, `BadTag` (`"host.schema"`), `BadSectionCount`, `BadChildTag`, `ChildLengthRange` (`Context` declared `21 < 22` at directory lane 10), `NonZeroPadding` (lane 13), `TrailingLanes`, child offset error wrapping (`offset: 14`), `ConfigMismatch` on `"context.ext"` (for both `journal_rows = 1` with identical 3C caps/descriptor and `ext = None`), `"layout"`, and `"context.budgets"`, and negative-input genesis (`RunIntC::Negative` preserved alongside zeroed `initial_fuel`/`stack_limit`).
4. **`owner_binding_refuses_other_program_unrelated_pair_and_other_run_inputs` (`lines 818-847`):**
   - Verifies that two programs with identical `IdentityC` (`Unselected` digests) and identical `C6HostConfig` differing only in an unused function `unused(z)` are refused by `bind_decoded_host` with `Mismatch { field: "admission.view" }`; verifies `Arc` clone acceptance, unrelated pair AB refusal (`C6BindError::Observation`), fuel/stack/negative input mismatches, context mismatch, and post-step binding with `current_image_equal() == false`.
5. **`owner_trace_full_host_decode_preserves_uncached_witness` (`lines 849-880`):**
   - Traces the `Box`/`Pair` program to `OutcomeC::Halted` within 4096 steps, verifying `compare_full_host_witness` across `RecordBefore`, `RecordAfter`, and terminal `Snapshot` with `steps > 0 && helpers > 0 && ctor > 0`.
6. **Untested Error Branches & Open Test Obligations:**
   - Untested branches in `c6_phys_host.rs` (not claimed covered): `decode_host` header `< 14` `Truncated`, `BadVersion`, `BadTag("host")`, directory length/offset `UsizeOverflow`, body `Truncated` (`14 <= lanes.len() < o[4]`), `Child { child: Program, .. }` / `State { offset, error }` propagation, `C6HostCapture::from_snapshot`, and `C6BindError::RetainedLayout` / `Program` / `"admission.view.lanes"`.
   - **T1 Open Obligation:** The 9 `HelperControlC` and 18 `HelperResultC` literal lane vector checks in Chunk 3B (`c6_phys_helper.rs`) remain open before ABI publication and are not closed by Chunk 4 tests.

---

## 5. Mandatory Corrections, Optional Follow-ups, and Outstanding Gates

### Mandatory Owner Source Corrections
- **None (0 mandatory fixes).**

### Optional Non-Blocking Follow-ups
1. **Selected-Digest Blob Limit Naming (`c6_phys_host.rs:155, 168, 258, 274`):** `MetaWriter::digest` and `MetaReader::digest` call `self.blob(bytes)` for `DigestC::Selected { format, bytes }`, which applies `cap_count("host.meta.string_bytes", ...)` to selected digest byte arrays in addition to cumulative `"host.meta.bytes"`. If future selected digests could exceed `max_string_bytes` while staying within `max_metadata_bytes`, a dedicated `blob` helper or doc note on `max_string_bytes` can clarify that it caps any single metadata byte/string blob.
2. **Additional Outer Refusal Unit Checks:** When convenient, add unit assertions for `decode_host` `< 14` truncation, directory offset `UsizeOverflow`, and `C6HostCapture::from_snapshot` (`CallerSnapshot` provenance).

### Outstanding Runtime Qualification & Open Roadmap Gates
- **Root Normal Toolchain QA (Outstanding):** Compile `mori-core` and execute the 5 new `c6_phys_host` tests, the 4 unrun `c6_phys_state` (3C) tests, the 6 unrun `c6_phys_helper` (3B) tests, the 3 `c6_phys_data` (3A) tests, and the 18 existing Core tests under bounded resources, plus the separate `c6_a135_push` backend SDK test.
- **Still Open Across the Roadmap:**
  - T1 literal lane vectors for the remaining 9 helper and 18 result variants in 3B before ABI publication;
  - Detached cryptographic commitment / authentication for `DigestC::Selected` and checkpoint/import authority;
  - Full 35 UIREQ / 25 features (68 scenarios) and financial/persistent execution semantics (current `CTX` and `META` finance/persistent fields are representation only);
  - All 39 selectors, partition and rule constraints, typed witness and export artifacts;
  - Grouping 1/2/4/8 earliest-stop, free-publication equivalence, and run-barrier observability;
  - Finite immutable-key residual baseline and generic full-state fallback;
  - Chunk 5 native circuit constraints, emitter, public statement binding, prover, verification, and Midnight Preview ledger settlement.
