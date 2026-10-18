# Independent Non-Author Continuation Audit: Minimal Cargo Registration & Exact Two-Operation Discovery Proposal

- **Reviewer Session**: `8efbfa9e-8e8f-4810-b53f-c50f4be0cbfc` (same fresh non-author reviewer continuation; zero tools, filesystem reads, shell commands, network requests, provider calls, or subject jobs invoked)
- **Original CWD**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ035-original-positive-host-clause-audit-20261018/part-01-ready/fresh-provider-cwd`
- **Execution State**: `UNRUN_FRESH_SAME8efb_SOURCE_AND_TWO_OPERATIONS_REVIEW_THEN_ROOT_SELECTION` (both operations `01` and `02` are unrun; `WW` and native `W` are untouched)
- **Assessment (A) — Source Eligibility Verdict**: **`REGISTRATION_SOURCE_ELIGIBLE_NO_DEFECTS`**
- **Assessment (B) — Exact Two-Operation Proposal Verdict**: **`TWO_DISCOVERY_OPERATIONS_ELIGIBLE_FOR_ROOT_EXECUTION`**

---

## 1. Assessment (A): Minimal Cargo Registration Source Eligibility

### 1.1 Exact Entry Mutation, Target Naming, and 9-Test Discovery Arithmetic
- **Canonical Cargo Integration Entry vs. Support Module**:
  - The actual top-level Cargo integration test target in `crates/mori-core/tests/` is `crates/mori-core/tests/sherlock_witness_reference_prefix.rs`, whereas `sherlock_witness_reference_prefix_positive.rs` is a submodule under `crates/mori-core/tests/support/`.
  - **Preimage**: `/home/charl/Moriarty/.worktrees/sherlock-witness-cache-stream-20261018/crates/mori-core/tests/sherlock_witness_reference_prefix.rs` (`bytes=240`, `sha256=8039696911d04a711c0fafc0461dc30a62293c775d356c4f4c9e63984fc91c7e`) already registers `mod positive;` (3 tests) and `mod contrast;` (2 tests).
  - **Patch & Appended Suffix**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ035-cargo-discovery-and-original-closure-20261018/REGISTRATION.patch` (`bytes=308`, `sha256=186d03c759bca021fd9985cdf14394eee679a4b706072264ca69e0e31dd96c82`) appends the exact 75-byte UTF-8 suffix:
    ```rust

    #[path = "support/sherlock_witness_reference_work_positive.rs"]
    mod work;
    ```
  - **Afterimage**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ035-cargo-discovery-and-original-closure-20261018/afterimages/crates/mori-core/tests/sherlock_witness_reference_prefix.rs` (`bytes=315`, `sha256=335899d0468a2037a1e412e7a634c4cae85d487d531a6f43e4886e5f1c213164`) has the 240-byte preimage as an exact prefix (`240 + 75 = 315` bytes) and is invertibly restored by stripping those 75 bytes.
  - **Preservation of All 5 Existing Tests (`3 + 2 + 4 = 9`)**: Preserving `mod positive;` (3 tests) and `mod contrast;` (2 tests) while adding `mod work;` (4 tests) yields **9** registered tests in total (not 7):
    1. `positive::factorial_three_complete_reference_prefix_and_pending_caller`
    2. `positive::list_two_five_complete_reference_prefix`
    3. `positive::tree_four_one_three_complete_reference_prefix`
    4. `contrast::factorial_two_complete_prefix_program_cache_and_incomplete_stream`
    5. `contrast::let_leaf_pattern_variable_complete_reference_prefix`
    6. `work::factorial_three_decoded_metadata_reference_work_and_b0`
    7. `work::list_two_five_decoded_metadata_reference_work_and_b0`
    8. `work::tree_four_one_three_decoded_metadata_reference_work_and_b0`
    9. `work::valid_program_layout_run_inputs_clear_cache_contrasts`

### 1.2 Support Module Isolation, Include-Path Resolution, and Fixture Pin Integrity
All 3 support modules and 10 opaque fixture files staged under `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ035-cargo-discovery-and-original-closure-20261018/afterimages/crates/mori-core/tests/` match their canonical/reviewed pins in `REGISTRATION-AND-DISCOVERY-CONTRACT.json` (`bytes=12758`, `sha256=09ced65018f97c1b6998baaaa940e9bdf9cc58f81fb5fe5a6c632b5c1c49c587`), `RETAINED-CONSUMER-INVERSE.json`, and `REGISTRATION-SOURCE-FREEZE.json`:

| Staged File Under `.../UIREQ035-cargo-discovery-and-original-closure-20261018/afterimages/crates/mori-core/tests/` | Bytes | SHA-256 | Verification Note |
|---|---:|---|---|
| `sherlock_witness_reference_prefix.rs` | `315` | `335899d0468a2037a1e412e7a634c4cae85d487d531a6f43e4886e5f1c213164` | Top-level entry; 240B preimage (`80396969...`) + 75B `mod work;` registration |
| `support/sherlock_witness_reference_prefix_positive.rs` | `18254` | `7391f8f8302e5441a52742962836bdb291b46f832a35099f38caf8aa6c6551cc` | Byte-identical to `WW` copy; include-path-only normalization of reviewed `abf69577...` |
| `support/sherlock_witness_reference_contrast_positive.rs` | `33049` | `e61953072f7783f3e28258f19f6db0009eda1200cd12f50e5330361cece43f97` | Byte-identical to `WW` copy; include-path-only normalization of reviewed `97975468...` |
| `support/sherlock_witness_reference_work_positive.rs` | `37862` | `b87ef1580e4f0d42cf0aebeb5f356b3793d559723e9b329c0c29493ddcfb4b8f` | Byte-identical to reviewed & 4-test-passed consumer |
| `fixtures/witness-reference-prefix/factorial3-complete-A39-prefix.json` | `464178` | `af15b0e50e6f58b456f9c4b63773eccd802771d8ea64d0a2730ae14c8a6af1d5` | Unchanged opaque independent fixture (`positive` & `work`) |
| `fixtures/witness-reference-prefix/list2-5-complete-A39-prefix.json` | `301342` | `6feb5bb6bec696425bec29ab2abc656de52ccc07a7e8b97b424175666a7494db` | Unchanged opaque independent fixture (`positive` & `work`) |
| `fixtures/witness-reference-prefix/tree4-1-3-complete-A39-prefix.json` | `716490` | `c9238deef8bb1193f2b67c39fca9c9e0a56aac950f3fa2df405745b4aa9df4c6` | Unchanged opaque independent fixture (`positive` & `work`) |
| `fixtures/witness-reference-prefix/factorial-pending-caller3.json` | `14726` | `48cb2d9de73c2af3441bac2acd4f95a79ad4cce2e68239e7bba94f8d10dafb00` | Unchanged opaque independent fixture (`positive` & `work`) |
| `fixtures/witness-reference-contrast-independent/factorial2-normal-program-contrast-A39-prefix.json` | `322314` | `11b11e9f36cb09f82eb2714a5653667b38f8fcedc05db35153a596e904024f6b` | Unchanged opaque independent fixture (`contrast`) |
| `fixtures/witness-reference-contrast-independent/let-leaf-pattern-var-complete-A39-prefix.json` | `34861` | `ddd6e079760c358d6fd3bc5ca7ea6ebc87ca9756a23e1065e8dea11646fa7b58` | Unchanged opaque independent fixture (`contrast`) |
| `fixtures/witness-reference-contrast-independent/factorial2-normal-program-contrast-A39-prefix-B0-clocks.json` | `28581` | `d384409dc97941fb7397c7a583ab2a322248ea3553219b8f526de5205c81d703` | Unchanged opaque independent fixture (`contrast`) |
| `fixtures/witness-reference-work-independent/factorial3-complete-A39-prefix-B0-clocks.json` | `38577` | `845c66a8faed3f0b985475b45ea72e778c1672165a57b19bbd334a9c18deb36e` | Unchanged opaque independent fixture (`work`) |
| `fixtures/witness-reference-work-independent/list2-5-complete-A39-prefix-B0-clocks.json` | `25490` | `87d07d213ac279671d38f3de365f08e4e8642ed47731a4c9598ee3501b504c11` | Unchanged opaque independent fixture (`work`) |
| `fixtures/witness-reference-work-independent/tree4-1-3-complete-A39-prefix-B0-clocks.json` | `37236` | `4280e5f675be648818002f5581d3b2ecc852f1ea14b164fe4f3dc3c939317159` | Unchanged opaque independent fixture (`work`) |

- **Namespace & Compilation Soundness**: Each support file is mounted as a distinct child module (`mod positive;`, `mod contrast;`, `mod work;`), preventing any symbol collision between module-private helpers (`exercise`, `variants`, `compare_expected`, etc.) or constants (`FACT`, `LIST`, `TREE`, `PENDING`, etc.). In Rust, `include_str!(...)` inside a `#[path = "support/..."]` submodule resolves relative to the submodule file's directory (`.../afterimages/crates/mori-core/tests/support/`), so all `../fixtures/...` literals resolve to the exact 10 staged fixture files above.
- **No Source Defects Identified**: Zero syntax, module-resolution, or pin-integrity defects exist in the staged registration tree.

---

## 2. Assessment (B): Exact Two-Operation Discovery Proposal Eligibility

Inspection of `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ035-cargo-discovery-and-original-closure-20261018/ordinary-QA/PROPOSED.json` (`bytes=8067`, `sha256=21a020c97cf1c39a12f70cdfe06192cc90e61355e8294e1e664b19551f25c53e`) and the 509-entry `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/UIREQ035-cargo-discovery-and-original-closure-20261018/ordinary-QA/FLAT-INPUT-PINS.json` (`bytes=147741`, `sha256=ed82ec371c816b730d600176fe3fb25a48d8a8f48ebb30c5b3cd1fab4f154b7e`, reconstructed byte-for-byte in `FULL509-INPUT-PINS-LOSSLESS.json` by appending the 14 staged registration/support/fixture rows to the 495-entry base `fffd2915...`) confirms that the two-operation plan is **eligible for Root execution**:

| Op ID | Mode | Command & Binding | Resource Caps & Isolation | Required Post-Run Acceptance Checks |
|---|---|---|---|---|
| `01-compileCargoReferencePrefixRegistration` | Compile only (`coreBuilds: 0`) | `rustc --edition=2024 --crate-name witness_reference_prefix --test .../afterimages/crates/mori-core/tests/sherlock_witness_reference_prefix.rs` linking retained `libmori_core.rlib` (`4afb6609...`) and `libserde_json-76dc87088753b61e.rlib` (`050f23c5...`) -> outputs `.../ordinary-QA/build/witness-reference-prefix-registration` + `.d` | `timeout 30s`, `prlimit --cpu=25:25 --as=2147483648:2147483648 --fsize=67108864:67108864 --core=0:0`, `keptBytesPerStream=131072`, `PATH=/usr/bin:/bin`, `LANG=C.UTF-8`, `TMPDIR=.../ordinary-QA/tmp`, `serialFailStopNoRetry=true` | `actual_exit == 0`; all 509 static pins unchanged pre/post; `.d` dep-info contains the entry (`335899d0...`), all 3 support modules (`7391f8f8...`, `e6195307...`, `b87ef158...`), and all 10 opaque fixtures with zero unpinned reads; pin newly emitted binary and `.d` before Op `02`. |
| `02-listOnlyCargoReferencePrefixRegisteredTests` | Discovery only (`testBodiesRun: 0`, `oldTestReplay: 0`) | `.../ordinary-QA/build/witness-reference-prefix-registration --list --format terse` | `timeout 10s`, `prlimit --cpu=8:8 --as=1073741824:1073741824 --fsize=16777216:16777216 --core=0:0`, `keptBytesPerStream=131072`, same isolated `cwd`/`environment`, `serialFailStopNoRetry=true` | `actual_exit == 0`; all 509 static pins and newly pinned binary unchanged; `full_stream_retained == true`; parse `<name>: test` lines from `stdout.bin` (do **not** apply single-test `test_membership` summary parsing to `--list --format terse`), requiring the exact 9 unique expected test names (including all 4 `work::*` names) and 0 benchmarks or duplicates. |

### Execution Controls & Scope Boundaries Verified
1. **Unchanged Subject Capture Executor**: Both operations use the existing R435 `capture(op, phase)`, `dep_inputs`, and static pin check helpers in `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/435-resolved-signature-execution-preparation/revision02/run_selected_13.py` (`bytes=15371`, `sha256=ce0482fdc12e4146500fa4421f7e4333b14321bdaa207c123a09ee6cb34c9932`), without invoking historical `main`, any new runner framework, or single-test `test_membership` on Operation `02`'s `--list --format terse` output.
2. **Zero Core Rebuild & Zero Test Body Replay**: `libmori_core.rlib` (`4afb6609...`) is reused unchanged (`coreBuilds: 0`), and neither the 5 existing tests nor the 4 already-passed `work::*` tests execute their bodies during `--list --format terse` (`testBodiesRun: 0`, `oldTestReplay: 0`).
3. **No Defect Identified**: Both the minimal 3-line registration afterimage (`335899d0...`) and the two-operation discovery plan (`21a020c9...`) are sound and ready for Root selection.

