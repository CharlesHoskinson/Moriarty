# Independent Non-Author Actual-Result & Publication-Eligibility Review: Generic Nominal Product Model v2 (`--depth 1` 15-Child Tail Execution)

- **Reviewer Identity**: `Gemini 4 Argon (High)` (Independent Non-Author Ordinary Semantic / Model-Source Reviewer during observed Opus outage)
- **Conversation ID**: `31395ab6-a9f6-4c1e-95db-aec02eedb3d9` (Continuation Turn 5 — `END-NOMINAL-V2-ACTUAL-RESULT-ADOPTION-REVIEW`)
- **Original Reviewer CWD**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-nominal-product-model-v2-review-operation-20261018/provider-ready-01/fresh-provider-cwd`
- **Target Execution Package**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-nominal-product-model-v2-initial-simplification-tail-20261018`
- **Constraint Compliance**: Zero tool calls, zero shell commands, zero filesystem reads or writes, zero subagent delegations, zero model queries, and no new design vote. All findings and verdicts derive exclusively from the inline bodies supplied in this prompt and the byte-identical source/oracle/comparator bodies read inline in earlier turns of this same conversation (`31395ab6-a9f6-4c1e-95db-aec02eedb3d9`).

---

## 1. Readable Coverage, Prior Same-CID Read Basis, and Unread Raw Artifact Limits

### 1.1 Complete Inline Bodies Read in This Continuation (Turn 5)

All 13 inline bodies supplied in this continuation were read in full:

| # | Relative Path / Artifact | Size (B) | SHA-256 Digest | Read Status |
| :--- | :--- | ---: | :--- | :--- |
| 1 | `actual/RESULT.json` | 48,158 (in readback) | Verified via `ACTUAL-RESULT-READBACK.json` | **Read in full inline** (all 15 phase receipts, `pins_before`, `nullary_checkpoint`, `offline_result`, `all_phases_exit_zero: true`, `completed: true`) |
| 2 | `actual-result-review/ACTUAL-RESULT-READBACK.json` | — | Inline summary | **Read in full inline** (`individual_receipt_json_roundtrip_all_15_match_exact_bytes: true`, `pin_count: 60`, `membership_row_count: 95`) |
| 3 | `actual-result-review/RECEIPT-STREAMS-AND-OPAQUE-ARTIFACTS.json` | — | Inline manifest | **Read in full inline** (all 15 `receipt.json` paths/bytes/hashes, all 30 `stdout`/`stderr` stream digests, and 14 `OPAQUE_UNREAD_BODY` descriptors) |
| 4 | `actual-result-review/PIN-MEMBERSHIP.lossless.json` | — | Reconstructs `3e9ef2da...` | **Read in full inline** (95 rows `0..94` under `shared_runtime_root` + `/nix/store/bc47.../domains.md`, reconstructing the 60-entry `actual/FROZEN-EXECUTION-PINS.json` `sha256=3e9ef2dad4c24b0e4ce9d7731fed28cc870cfaeb731e16269e19484371c21f55`) |
| 5 | `comparison/COMPARISON-MANIFEST.json` | 4,050 | `dafb04dcb9f688ca9cb07f99433f91af8133fb38e89b461467705f0c97303014` | **Read in full inline** (exact 7-case mapping of expected JSON, `.term` query, `.kast` parsed input, actual `result.kore`, and actual `print/stdout`) |
| 6 | `actual/08-offline-complete-tapes/stdout` | 2,994 | `31e938c6f0c3b7f0bfe9bb608569adff0066e9efa89e85f5586042d9a3642bab` | **Read in full inline** (`all_cases_equal: true`, 7 case reports with `complete_tape_equal: true` and `first_difference: null`) |
| 7 | `actual/ROOT-NULLARY-CHECKPOINT-READBACK.json` | 590 | `589574feb5addc9925c3a87ff6a8d8c7a42b854a5c042b986bf695863e5c43b0` | **Read in full inline** (Root pause verification of `07-zero-field-constructor-print/stdout`, `31,204B`, `sha256=8ce5d44c...`) |
| 8 | `actual/PRE-PIN-CHECK.json` | — | Inline | **Read in full inline** (`{"count": 60, "all_match": true, "mismatches": []}`) |
| 9 | `actual/ROOT-SELECTION.json` | 260 | `c190fd830a9c3db3cd0174e1e8fda7f64b25396b81c44481d6fa3c76fa3c0002` | **Read in full inline** (links review `56def31a...` and `PROPOSED-OPERATIONS.json` `d33af5cd...`) |
| 10 | `review/EXACT-OPERATIONS-LOSSLESS-PROJECTION.json` | 5,802 | `61305b6885336248891b26b051bdb45f2cc460cc24b56a988cfdd73d5da4c886` | **Read in full inline** (lossless projection of `PROPOSED-OPERATIONS.json` `18,621B`, `sha256=d33af5cddf0adf3a1f65916148d6f7fb226ba76ddda787a25f7c60d57b47ddad`) |
| 11 | `provider-ready-initial-simplification-tail/REPORT.md` | 13,431 | `56def31a85de8b8017254e6eb66b51bf66d872c1846b8061a60fb06e7dc6d881` | **Read in full inline** (prior Turn 4.5 review report) |
| 12 | `context/ACTUAL-REDUCE-STDERR.txt` | 81 | `89aa4052ef5a90ddda5be6bcfeef74de60902648bf2d937e19c30a1e9de44771` | **Read in full inline** (historical `--depth 0` `WarnDepthLimitExceeded` stderr from predecessor package) |
| 13 | `context/EXISTING-LAUNCH-CALLABLE-CLOSURE.py.txt` | 4,228 | `996aadee935833bf1115725e506e06b7f3750c2c9d7d76770a2398fbbfda728b` | **Read in full inline** (`run_bounded_command` / `_terminate_process_group`) |

### 1.2 Byte-Identical Prior Same-CID Read Basis (Turns 1–4.5) Verified via Pin Continuity

All 60 entries in `actual/FROZEN-EXECUTION-PINS.json` (`sha256=3e9ef2dad4c24b0e4ce9d7731fed28cc870cfaeb731e16269e19484371c21f55`, reconstructed losslessly from rows `0..94` of `PIN-MEMBERSHIP.lossless.json`) were verified before execution (`PRE-PIN-CHECK.json`) and after every single child (`pins_after: {"count": 60, "all_match": true}`). This connects the observed execution directly to the exact files read in full inline earlier in `31395ab6-a9f6-4c1e-95db-aec02eedb3d9`:
- **3-File K Source & Observer Closure (25,043B)**:
  - `source/nominal-product-syntax.k` (`4,490B`, `sha256=bf626f801f4ac309687ab1d1433aa8ec248baa224eb7ffe7b7ed2eb2cca6392b`, row 11)
  - `source/nominal-product.k` (`19,167B`, `sha256=0d7d4ec32206a478d868194860f717f941bc45997b99c866af1df2b679b43131`, row 12)
  - `source/normal-trace-observations.k` (`1,386B`, `sha256=f4eaa0b5ad2969f2d0ab1324fc0894aba23cfe5140d23a7a81a58d54a3219f5f`, row 13)
- **Offline Codec, Structural Parser, and Complete-Tape Comparator**:
  - `comparison/nominal_trace_codec.py` (`14,188B`, `sha256=6ddfc6238fd4c93d04caad62971b6826be2d0a9e048b91fa49bf8dd3e266d74e`, row 2)
  - `comparison/compare_complete_tapes.py` (`3,024B`, `sha256=43ac8f29cf480c94f8f53f4392f2ff44985a5be9347bb17c918b35e18e67e236`, row 1)
- **Independent Oracle Expectations & Lossless Factored Representation**:
  - `source/independent-expected-tapes.k` (`62,832B`, `sha256=375cb881953d54e84cf4f70df94d6428966bf1e918f85beeb995b96fbdd778fb`, row 0, read in full in Turn 1 and proven roundtrip-equivalent to the 7 frozen JSON expectations under `expected/*.json`, rows 37–43, combined digest `d4c9aee74c507299413c4009e7a4e8545f7c076b166876a8f32bde3cd47d7826`)
- **7 Unparenthesized-Map Input Queries (`.term`) and Reused Compiled/Parsed Artifacts**:
  - `inputs/01..07-*.term` (rows 15–21, read in full in Turn 4)
  - Reused 15-file `nominal-kompiled` directory (rows 22–36) and 7 parsed `.kast` queries + receipts (rows 59–79).

### 1.3 Unread Raw Artifact Limitation (`OPAQUE_UNREAD_BODY`)

As documented in `actual-result-review/RECEIPT-STREAMS-AND-OPAQUE-ARTIFACTS.json`, 14 large generated output files are **deliberately not inline and not read by this reviewer**:
- **7 Raw Kore Outputs (`actual/01..07-*-reduce/result.kore`)**: `216,697,318` bytes (`~216.7 MB`) total across the 7 reductions.
- **7 Raw Printed Tape Outputs (`actual/01..07-*-print/stdout`)**: `4,862,169` bytes (`~4.86 MB`) total across the 7 `kprint` invocations.

Each of these 14 artifacts is marked `"read_status": "OPAQUE_UNREAD_BODY"`. Their byte sizes and streaming SHA-256 digests in `RECEIPT-STREAMS-AND-OPAQUE-ARTIFACTS.json` match the exact `actual_kore_bytes`/`actual_kore_sha256` and `actual_print_bytes`/`actual_print_sha256` reported in `actual/08-offline-complete-tapes/stdout`. Consequently:
- This review **does not claim direct human/model visual inspection** of the `~216.7 MB` raw Kore or `~4.86 MB` printed tape text.
- Rather, acceptance of the 7 actual outputs is **source-conditioned** on:
  1. Our complete prior inline audit of `nominal_trace_codec.py` (`6ddfc623...`) and `compare_complete_tapes.py` (`43ac8f29...`), which proved that the parser/normalizer fails closed on any unknown constructor/sort, unconsumed trailing suffix, or non-`ngTape` term (as empirically demonstrated when `--depth 0` failed with exit code `1`), and
  2. Our verification here that Phase 15 (`08-offline-complete-tapes`) executed that exact pinned comparator against those exact 14 SHA-256-matched files and the 7 pinned expected JSON files (`rows 37..43`), exiting `0` with `all_cases_equal: true`.

---

## 2. Verification of the Observed 15-Child `--depth 1` Execution

### 2.1 Exact Match Against Approved 15-Operation Plan (`d33af5cd...`)

Comparing `actual/RESULT.json` against `review/EXACT-OPERATIONS-LOSSLESS-PROJECTION.json` (`sha256=61305b68...`, projecting `PROPOSED-OPERATIONS.json` `sha256=d33af5cd...`):
- **Operation Count & Order**: Exactly the 15 approved operations ran in exact phase order (`phase_index` `1..15`):
  - Phases 1–2: `07-zero-field-constructor-reduce` (`kore-exec --depth 1`), `07-zero-field-constructor-print` (`kprint`).
  - Phases 3–14: `01-construct-asymmetric-{reduce,print}`, `02-construct-repeated-{reduce,print}`, `03-structural-sum-asymmetric-{reduce,print}`, `04-ordered-first-arm-{reduce,print}`, `05-structural-sum-repeated-{reduce,print}`, `06-recursive-constructor-fields-{reduce,print}`.
  - Phase 15: `08-offline-complete-tapes` (`python3 comparison/compare_complete_tapes.py --manifest comparison/COMPARISON-MANIFEST.json`).
- **Command Arguments, CWD, and Environment**:
  - Every executed `argv`, `cwd` (`/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-nominal-product-model-v2-initial-simplification-tail-20261018`), `PATH`, `LC_ALL=C`, `LANG=C`, `JAVA_TOOL_OPTIONS=-XX:+UseSerialGC -Xms64m -Xmx512m -XX:ActiveProcessorCount=1`, and per-child `SHERLOCK_K_ATTEMPT_TOKEN=generic-nominal-product-model-v2-initial-simplification-tail-20261018:<operation_id>` matches the approved specification byte-for-byte.
  - All 7 `kore-exec` invocations used `--depth 1` (replacing `--depth 0`), enabling the Haskell backend's first `[Begin, Simplify, Rewrite, Simplify]` strategy bundle so `ngObserve(ngInitial(...), N)` simplifies completely through `Evaluator.evaluate`.
- **Containment, Resource Bounds, and Pin Integrity**:
  - All 15 phases completed with `returncode: 0`, `timed_out: false`, `containment_breach: false`, `swept: []`, and `remaining: []`.
  - Wall-clock durations ranged from `0.033s` (`07-zero-field-constructor-reduce`) to `10.960s` (`05-structural-sum-repeated-print`), well within the `30s` per-child wall cap (`wall_limit_seconds: 30`).
  - `pins_before` and every phase's `pins_after` report `{"count": 60, "all_match": true, "mismatches": []}`, confirming that no source, compiled definition, parsed `.kast` input, comparator script, or oracle file was mutated before, during, or after execution.
- **Empty `stderr` Across All 15 Phases vs. Historical `--depth 0` Warning**:
  - Every one of the 15 executed phases produced an empty `0`-byte `stderr` (`sha256=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`).
  - The predecessor `--depth 0` warning (`Warning (WarnDepthLimitExceeded): ... The depth limit 0 was exceeded`, `81B`, `sha256=89aa4052...`) is preserved unmodified at its original historical path (`.../generic-nominal-product-model-v2-input-map-parens-tail-20261018/actual/01-construct-asymmetric-reduce/stderr` and `context/ACTUAL-REDUCE-STDERR.txt`) and did not recur under `--depth 1`.

### 2.2 Root Nullary Checkpoint Verification (`actual/ROOT-NULLARY-CHECKPOINT-READBACK.json`)

As required by the Turn 4.5 conditional execution gate:
1. Phase 1 (`07-zero-field-constructor-reduce`) completed at `2026-10-18T17:36:23.095615+00:00` (`0.033s`, `result.kore` = `717,440B`, `sha256=4943237d8f7e4bc690ca2ff42b06b3556835be5270778f5c79902182dc4e3c3c`).
2. Phase 2 (`07-zero-field-constructor-print`) completed at `2026-10-18T17:36:28.187923+00:00` (`5.091s`, `stdout` = `31,204B`, `sha256=8ce5d44cd4671b2bb8739db0a0400dc2a1e2dcff5a6507943912bdde0ec48eb8`).
3. Root paused execution for ~43 seconds (`17:36:28Z` to `17:37:11Z`) and ran the structural parser readback (`actual/ROOT-NULLARY-CHECKPOINT-READBACK.json`, `590B`, `sha256=589574fe...`) on `07-zero-field-constructor-print/stdout`, verifying:
   - `top_constructor: "ngTape"`, `top_sort: "NGTape"`, `remaining_suffix: ""` (exhausted suffix),
   - `initial_control: "ngEval"`,
   - `returned_states: 3`,
   - `final_control: "ngHalted"`, `final_frames: "ngNoFrames"`,
   - `final_equals_last_returned_state: true`.
4. Only after confirming a genuine reduced `ngTape` with exhausted suffix did Root launch Phases 3–15 at `2026-10-18T17:37:11.653632+00:00`.

### 2.3 Complete 7-Case Offline Tape Equality (`actual/08-offline-complete-tapes/stdout`)

Phase 15 (`08-offline-complete-tapes`) completed in `0.785s` with `returncode: 0` and emitted `2,994` bytes on `stdout` (`sha256=31e938c6f0c3b7f0bfe9bb608569adff0066e9efa89e85f5586042d9a3642bab`), reporting `"all_cases_equal": true`:

| Case ID | Expected JSON (`rows 37..43`) | Actual `result.kore` Bytes & SHA-256 | Actual `print/stdout` Bytes & SHA-256 | `complete_tape_equal` | `first_difference` |
| :--- | :--- | :--- | :--- | :---: | :---: |
| `construct-asymmetric` | `01-construct-asymmetric.json` (`11,227B`, `b52bb543...`) | `4,521,552B` (`4a25d370...`) | `173,054B` (`5a76933d...`) | `true` | `null` |
| `construct-repeated` | `02-construct-repeated.json` (`11,227B`, `35629a5f...`) | `4,521,552B` (`dee8fe01...`) | `173,054B` (`3b0e9a35...`) | `true` | `null` |
| `structural-sum-asymmetric` | `03-structural-sum-asymmetric.json` (`74,974B`, `0e2ab92a...`) | `76,128,305B` (`928d8d01...`) | `1,609,142B` (`f498c6d6...`) | `true` | `null` |
| `ordered-first-arm` | `04-ordered-first-arm.json` (`10,275B`, `9a5d3b2d...`) | `3,395,088B` (`24ca32b5...`) | `152,415B` (`c4f1d94b...`) | `true` | `null` |
| `structural-sum-repeated` | `05-structural-sum-repeated.json` (`74,983B`, `844c549d...`) | `76,128,305B` (`aed0cad8...`) | `1,609,142B` (`91112f50...`) | `true` | `null` |
| `recursive-constructor-fields` | `06-recursive-constructor-fields.json` (`53,795B`, `09dbdbda...`) | `51,285,076B` (`b8d99257...`) | `1,114,158B` (`3f580e3a...`) | `true` | `null` |
| `zero-field-constructor` | `07-zero-field-constructor.json` (`2,487B`, `5b481b56...`) | `717,440B` (`4943237d...`) | `31,204B` (`8ce5d44c...`) | `true` | `null` |

Because `compare_complete_tapes.py` (`43ac8f29...`) compares the full normalized JSON object produced by `parse_kprint_tape_to_expanded_json` (`6ddfc623...`) against the full expanded expected JSON object (`expand_expected_case`), `"complete_tape_equal": true` establishes exact structural equality across:
1. The initial `NGState` (complete `NGProgram`, initial `ngEval` control, initial `Map` environment, `ngNoFrames` stack, and `ngNoEvents` trace),
2. Every intermediate returned `NGState` in exact chronological `NGStateList` order (preserving all `NGProgram` declarations, `NGControl`, current `Map` environment, ordered `NGFrames` stack including caller occurrence IDs in `ngConstructorFields` and saved caller environments in `ngRestoreEnv`, and cumulative `NGEvents` prefixes across all 19 event constructors), and
3. The final halted `NGState` (`ngHalted(V)`, `ngNoFrames`, restored top-level environment, complete event sequence) and absence of terminal stutters (`final == last(after_each_step)`).

---

## 3. Retained Qualifications & Scope Boundaries (Adopted Claim vs. `OPEN` Items)

### 3.1 Narrow Claim Eligible for Scoped Adoption / Publication

The observed execution and source-conditioned audit establish **only** the following narrow claim:

> **Scoped Adopted Claim**: Under the exact pinned 3-file K model/observer closure (`25,043B`), the reused 15-file `nominal-kompiled` definition, the 7 unparenthesized-Map source-only `.term` / `.kast` inputs, and the pinned K/Haskell toolchain (`/nix/store/bc47...` + `/nix/store/pmjs0...` with `kore-exec --depth 1`), the seven frozen positive nominal v2 cases (`construct-asymmetric`, `construct-repeated`, `structural-sum-asymmetric`, `ordered-first-arm`, `structural-sum-repeated`, `recursive-constructor-fields`, `zero-field-constructor`) reduced to complete `ngTape` outputs that match the independently frozen full-state expectations (`d4c9aee7...`) across initial state, every single-step returned state, ordered frames/fields/events, and final halted state.

### 3.2 Retained Qualifications and Claims That Remain Explicitly `OPEN`

None of the following broader claims are established by this result; each remains **`OPEN`**:

1. **Source-Conditioned Unread Raw Artifact Qualification**:
   - The `216,697,318` bytes of raw `result.kore` and `4,862,169` bytes of raw `print/stdout` were not read inline (`OPAQUE_UNREAD_BODY`); their verification relies on streaming SHA-256 continuity and the audited fail-closed offline parser/comparator (`nominal_trace_codec.py` + `compare_complete_tapes.py`).
2. **Input-Supplied Nominal IDs vs. Canonical Elaboration (`OPEN`)**:
   - All nominal type IDs, constructor IDs, field IDs, and expression/pattern occurrence IDs in v2 are **caller-supplied model inputs** in `NGProgram`. Canonical `TypeId`/`CtorId`/`FieldId` allocation, `CtorLayout` derivation, `MatchPlan` compilation, and `workMap` elaboration remain **`OPEN`**.
3. **Single-Recursive-Sum Fixture Coverage vs. Multi-Type & Parameterized Declarations (`OPEN`)**:
   - All 7 positive cases execute over a single fixed 3-type declaration fixture (`Pair=10`, `IntList=20`, `UnitFlag=30`). Mutually recursive multi-type declarations, parameterized declarations, and broader declaration shapes remain **`OPEN`**.
4. **Positive-Only Coverage vs. Negative / Stuck / Error & Binder-Linearity Cases (`OPEN`)**:
   - All 7 cases are positive, well-typed, terminating executions. Negative/ill-typed declarations, constructor arity/type mismatches, non-exhaustive match failures, duplicate pattern binder rejection (`ngBindPatternFields` silent overwrite if duplicate `VarId`s occur), and duplicate declaration ID rejection remain **`OPEN`**.
5. **Complete-Run Budgets vs. Mid-Prefix Observer Cut Resumption (`OPEN`)**:
   - All 7 queries supply an observer budget $N$ equal to the exact step count to `ngHalted`. Mid-prefix budget exhaustion ($N < \text{steps}$) and resumption from an intermediate `ngEval`/`ngReturn` state remain **`OPEN`**.
6. **A39 / B Clock, Native / Target / PCD, Full `UIREQ-034` / `SCON-016` EARS, and Paused `Let` / `If` (`OPEN`)**:
   - `kore-exec --depth 1` is purely a K Haskell backend rewrite-strategy parameter for `[function]` evaluation of `ngObserve` and has no A39/B helper-clock, gas/fuel, native proof, PCD, Preview ledger settlement, or full `UIREQ-034` / `SCON-016` EARS conformance meaning. General `Let` and `If` tracks remain paused and **`OPEN`**.

---

## 4. Scoped Verdicts

- **Observed 15-Child `--depth 1` Execution & Complete-Tape Comparator Verdict**: **`ACCEPT`** (All 15 children exited `0` with `0`-byte `stderr`, no timeouts or containment breaches, all 60 execution pins unchanged pre- and post-each-child, Root nullary checkpoint verified on the `31,204B` print prior to dependent continuation, and `08-offline-complete-tapes` exited `0` with `all_cases_equal: true` and `first_difference: null` across all 7 cases).
- **Scoped Publication / Adoption Eligibility Verdict**: **`ACCEPT`** (Eligible to adopt and publish **solely** the narrow source-conditioned positive 7-case nominal v2 complete-tape agreement claim stated in Section 3.1, retaining all `OPAQUE_UNREAD_BODY` and Section 3.2 `OPEN` qualifications).
- **Blockers for Scoped Adoption**: **`NONE`**.

