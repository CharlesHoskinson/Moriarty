```json
{
  "review_id": "R44-B-COMPLETE-OBSERVATION-OWNER-REVIEW-20261017",
  "reviewer_role": "Designated non-author SECURITY OWNER (standing Gemini 4 Argon High fallback; primary Opus 5.5 High unavailable on observed weekly quota)",
  "source_snapshot_commit": "c7245554dd2a176303c5280a068cafc889a1113e",
  "source_decision": {
    "verdict": "APPROVE_SOURCE_COMPOSITION_WITH_SCOPE",
    "target_files": [
      {
        "path": "crates/mori-core/src/sherlock/helper_machine/complete_observation.rs",
        "full_sha256": "34b9f4a41bc7be9f68a6af509bb2df99caaa8ae4e45fa302e585a342c701a8b9",
        "pages": ["01-01.txt"],
        "source_lines": [1, 312]
      },
      {
        "path": "crates/mori-core/src/sherlock/helper_machine/mod.rs",
        "full_sha256": "e348e37ea574d2ed1ffb1bae80bd7890d7c9f70e15ecb90f92f9a947fe2f8df0",
        "pages": ["03-01.txt"],
        "source_lines": [1, 32]
      },
      {
        "path": "crates/mori-core/tests/sherlock_complete_observation_positive.rs",
        "full_sha256": "b7cd86a342e6015b04f8e035b7cb5ad7426948861b46e4be7dc9fe04ae760fab",
        "pages": ["02-01.txt"],
        "source_lines": [1, 93]
      }
    ],
    "summary": "complete_observation.rs and its Root registration in helper_machine/mod.rs (lines 12-13, `pub mod complete_observation;`) are source-correct. As a sibling of `shared`, `complete_observation` has no access to private `Admitted`, `MachineImage`, `PreImage`, `OwnerGuards`, or `ValidatedPlan` fields and alters no owner guards, selectors, invariants, work counters, or step semantics. `copy_observation` (lines 150-191) and `capture` (lines 193-195) perform a complete 1-to-1 owned copy of every public field and enum variant of `HostObservation` produced by `observe_bound_image`. `record_step` (lines 230-256) captures an owned `before` snapshot prior to mutation, invokes `host_owner_engine().step(admitted, image)` at most once with no run loop, retains the complete `StepEventC` on `StepResult::Event`, distinguishes `StepResult::Terminal` without fabricating an event, and preserves `before` plus post-step `capture` evidence on both step failure (`CaptureStepError::Step`) and post-observation failure (`CaptureStepError::AfterObservation`)."
  },
  "four_fixed_test_execution_classification": {
    "verdict": "ELIGIBLE_ORDINARY_SDK_FREE_H1_SATISFIED_WITH_CONDITIONS",
    "execution_class": "Safe-static (ordinary SDK-free host diagnostic compile and fixed-test execution, subject to mandatory conditions C1-C5)",
    "prospective_plan_sha256": "7b0a0663ba20507e7f636c20fb0d93e624e427290b1174151cc29da651e4c002",
    "test_source_sha256": "b7cd86a342e6015b04f8e035b7cb5ad7426948861b46e4be7dc9fe04ae760fab",
    "approved_test_selectors": [
      "actual_pre_is_owned_before_step_and_post_matches_returned_image",
      "present_empty_records_compare_initial_to_returned_without_terminality_claim",
      "two_actual_complete_prefix_records_are_contiguous_in_every_field",
      "different_owned_returned_metadata_reports_the_final_boundary"
    ],
    "step_and_resource_closure": {
      "input_program": "Program { functions: vec![], entry: Arc::new(Expr::Literal(Value::UInt(7))) }",
      "admission_calls": 4,
      "owner_step_calls_per_test": [2, 0, 2, 1],
      "total_owner_step_calls": 5,
      "executed_rules": [
        "RuleIdC::BHelperValidateEntry",
        "RuleIdC::BHelperValidateOccurrence"
      ],
      "run_loop_invoked": false,
      "fuel_input": "RunIntC::Nat(64)",
      "stack_limit_input": "RunIntC::Nat(8)",
      "finance_input": "FinanceInput::Absent",
      "sdk_or_native_proving_used": false
    },
    "non_transferability": "Prior H1 approval (SHA 16b1fb249c94804ad1b1e19856875e2070fd918a7d687b7262a859f59d4c8be2) covered only the two parent-A fixtures (S01/S02) and does not transfer by implication. This R44 classification covers ONLY the 4 exact tests in sherlock_complete_observation_positive.rs (SHA b7cd86a342e6015b04f8e035b7cb5ad7426948861b46e4be7dc9fe04ae760fab) on the fixed UInt(7) literal input and explicit bounds. H1 generic-caller restriction, C4 OPEN, R2 UNSATISFIED, 60460 HOLD, and native_grants = 0 remain unchanged."
  },
  "terminal_and_failure_distinctions": {
    "source_implementation": {
      "event_step": "complete_observation.rs:235-241 (`Ok(StepResult::Event { event })` -> `ObservedStep::Event(CompleteStepRecord { before, event, after })`) retains full `StepEventC` alongside owned `before` and `after` snapshots.",
      "terminal_no_next": "complete_observation.rs:243-249 (`Ok(StepResult::Terminal)` -> `ObservedStep::Terminal { before, after }`) reflects `owner_step` P0 (`owner_runtime.rs:168`, `if img.outcome != OutcomeC::Running { return Ok(StepResult::Terminal); }`) with no fabricated `StepEventC` and no inferred mutation.",
      "before_observation_failure": "complete_observation.rs:209,233 (`CaptureStepError::BeforeObservation(HostViolation)`) surfaces AB (`SI01`) or `implied_seg_base` (`SI05`) failure before `engine.step` is called.",
      "step_failure_with_post_copy": "complete_observation.rs:211-215,251-253 (`CaptureStepError::Step { violation, before: Box<HostSnapshot>, after: Result<Box<HostSnapshot>, HostViolation> }`) preserves the unchanged owner `HostViolation`, the pre-step snapshot, and the post-failure `capture(admitted, image)` result.",
      "after_observation_failure": "complete_observation.rs:217-221,237-240,245-248 (`CaptureStepError::AfterObservation { violation, before: Box<HostSnapshot>, event: Option<StepEventC> }`) preserves `before` and `Some(event)` (or `None` on `Terminal`) if post-step `capture` fails."
    },
    "test_and_metadata_coverage_limits": {
      "unrun_metadata_only": "R43 (`actual_production_metadata`, SHA 5acdcac1b57fb0fd49d786474615c24e086cf4fabb35a1003cffd7feb4188ad9, 23.000854s) and R45 (`actual_external_test_metadata`, SHA d8e2610536fb9637c9b996723443c6f9c8b83587d139531d45813d1b15e55ec0, 1.068844s) are type/borrow/derive checks only (`test_execution: \"UNRUN\"`) and establish zero linked build or runtime test/event execution.",
      "four_test_runtime_scope": "The 4 prospective tests in `sherlock_complete_observation_positive.rs` exercise only a 2-step `ObservedStep::Event` prefix (`BHelperValidateEntry`, `BHelperValidateOccurrence` while `OutcomeC::Running`), empty-slice continuity (` initial == returned`), 2-record contiguous prefix continuity, and `ContinuityBoundary::FinalReturned { final_record: 0 }`. They do not runtime-exercise `ObservedStep::Terminal`, `CaptureStepError::{BeforeObservation, Step, AfterObservation}`, or `ContinuityBoundary::{EmptyReturned, InitialBefore, Between}`."
    }
  },
  "mandatory_conditions_for_prospective_execution": [
    {
      "id": "C1_EXACT_SOURCE_AND_DEPINFO_PINNING",
      "anchor": {
        "file": "prospective_UNRUN_exact_plan",
        "full_sha256": "7b0a0663ba20507e7f636c20fb0d93e624e427290b1174151cc29da651e4c002",
        "page": "26-01.txt",
        "source_lines": [11, 37, 84],
        "literal": "\"/home/charl/Moriarty/.worktrees/sherlock-parallel-implementation-20261016/crates/mori-core/src/lib.rs\""
      },
      "requirement": "Because `production_build_argv` and `external_test_compile_argv` reference paths under `.worktrees/sherlock-parallel-implementation-20261016` (where W concurrently changed branches during packet prep), Root must verify the exact SHA-256 of every file in the `mori-core` source closure (matching snapshot commit `c7245554dd2a176303c5280a068cafc889a1113e` plus `complete_observation.rs` = `34b9f4a4...`, `helper_machine/mod.rs` = `e348e37e...`, and `sherlock_complete_observation_positive.rs` = `b7cd86a3...`) immediately before compilation and immediately after test completion, and verify that emitted `core.d` and `tests.d` reference only those hash-pinned source files."
    },
    {
      "id": "C2_FULL_DEPENDENCY_DIRECTORY_INVENTORY",
      "anchor": {
        "file": "prospective_UNRUN_exact_plan",
        "full_sha256": "7b0a0663ba20507e7f636c20fb0d93e624e427290b1174151cc29da651e4c002",
        "page": "26-01.txt",
        "source_lines": [12, 26, 40, 41, 85],
        "literal": "\"dependency=/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/24-compiler-qa/proposed-run/core-target/debug/deps\""
      },
      "requirement": "Before step 1 and after the final test step, record path, byte size, and SHA-256 for every regular file in `24-compiler-qa/proposed-run/core-target/debug/deps` (covering the 6 explicit `--extern` rlibs, all transitive rlibs, and `serde_derive`/`thiserror_impl` proc-macro `.so` files), require pre and post inventories to be identical, and ensure no pre-existing `libmori_core*` artifact resides in that `-L dependency` directory."
    },
    {
      "id": "C3_TOOLCHAIN_AND_LINKER_IDENTITY",
      "anchor": {
        "file": "prospective_UNRUN_exact_plan",
        "full_sha256": "7b0a0663ba20507e7f636c20fb0d93e624e427290b1174151cc29da651e4c002",
        "page": "26-01.txt",
        "source_lines": [7, 34],
        "literal": "\"/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc\""
      },
      "requirement": "Record `rustc -vV` (must report 1.99.0) and SHA-256 digests of the resolved `rustc` binary, `librustc_driver-*.so`, the target `lib/rustlib/x86_64-unknown-linux-gnu/lib` inventory, and the active linker (`cc` and any `rust-lld`/`gcc-ld`)."
    },
    {
      "id": "C4_TEST_BINARY_FLAGS_AND_EXACT_LIST_GATE",
      "anchor": {
        "file": "prospective_UNRUN_exact_plan",
        "full_sha256": "7b0a0663ba20507e7f636c20fb0d93e624e427290b1174151cc29da651e4c002",
        "page": "26-01.txt",
        "source_lines": [31, 45, 47, 82, 87],
        "literal": "\"limits\": \"serial; defaultopt0/debuginfo0; oneproductionbuild120s; oneexternalcompile30s; fourexacttests10seach; no broadtestfilter/no retries\""
      },
      "requirement": "Include `-C debuginfo=0` on `external_test_compile_argv` (matching `production_build_argv` and line 87 `limits`), record SHA-256 hashes of `libmori_core.rlib` and `observer-tests`, run `observer-tests --list` before executing tests to confirm only the 4 approved test names exist in the binary, and require each of the 4 `--exact --test-threads=1` runs to finish with `1 passed; 0 failed; 0 ignored; 0 measured`."
    },
    {
      "id": "C5_SANITIZED_ENVELOPE_AND_FAIL_FAST",
      "anchor": {
        "file": "governing_H1_source",
        "full_sha256": "16b1fb249c94804ad1b1e19856875e2070fd918a7d687b7262a859f59d4c8be2",
        "page": "25-01.txt",
        "source_lines": [96, 102],
        "literal": "Use `env -i` with only `PATH=/usr/bin:/bin`, `LANG=C.UTF-8` and a fresh `TMPDIR`."
      },
      "requirement": "Run each command serially from a fresh attempt directory outside the worktree under `env -i PATH=/usr/bin:/bin LANG=C.UTF-8 TMPDIR=<fresh>` (no `HOME`, `RUST*`, `CARGO*`, `RUSTC_WRAPPER`, `LD_*`), with process-group kill on timeout (120s rlib, 30s test compile, 10s per test), no `-O` or debug-assertion/overflow-check overrides, and immediate halt on first failure without retries."
    }
  ],
  "non_blocking_findings": [
    {
      "id": "NB1_UNMEASURED_HOST_CLONE_COSTS_OUTSIDE_B_COUNTERS",
      "anchor": {
        "file": "crates/mori-core/src/sherlock/helper_machine/complete_observation.rs",
        "full_sha256": "34b9f4a41bc7be9f68a6af509bb2df99caaa8ae4e45fa302e585a342c701a8b9",
        "page": "01-01.txt",
        "source_lines": [6, 150, 195, 233, 255],
        "literal": "//! Host allocation and copying costs are unmeasured and outside B work counters."
      },
      "detail": "`copy_observation` clones all `ProgramView` and `MachineImage` vectors (`expressions`, `occurrences`, `occurrence_paths`, `children`, `functions`, `parameters`, `arms`, `patterns`, `constants`, `constant_fields`, `names`, `data`, `data_fields`, `env`, `frames`, `tasks`, `value_slots`) and `LayoutSchemaC` strings/widths on both pre-step and post-step captures (`2 * O(|ProgramView| + |MachineImage|)` per `record_step`). This is explicitly disclosed at line 6 and bounded for the 4 fixed `UInt(7)` literal tests (17 total `capture` calls across 4 tests, 1 expression row, 1 constant row, 0 names, <=1 task)."
    },
    {
      "id": "NB2_HISTORICAL_UNINSTALLED_COMMENTS_IN_REGISTERED_MODULES",
      "anchor": {
        "file": "crates/mori-core/src/sherlock/helper_machine/shared.rs",
        "full_sha256": "479eb3983c81e9734d5bd427b68d735e627010368e6c992687c9ff5ff2dad3a9",
        "page": "04-01.txt",
        "source_lines": [1, 19],
        "literal": "//! helper_machine::shared — PROPOSED v2, not installed, not adopted."
      },
      "detail": "Header comments in `shared.rs:1-19`, `raw.rs:1-7`, `owner_host.rs:1-2`, `host_implied_prefix.rs:1`, `owner_prepare.rs:1`, `owner_runtime.rs:1`, `source_input_bridge.rs:1`, `legacy.proposed.rs:1-6`, and `structural.rs:1-5` retain historical `PROPOSED`/`uninstalled`/`uncompiled` labels, whereas `lib.rs:8`, `sherlock/mod.rs:11`, `helper_machine/mod.rs:7-13`, and `shared.rs:467-470` actually register them in `mori_core`. Actual registered source governs as fact; this carries no product or native acceptance."
    },
    {
      "id": "NB3_PARTIAL_RUNTIME_BRANCH_COVERAGE_IN_POSITIVE_TEST_FILE",
      "anchor": {
        "file": "crates/mori-core/tests/sherlock_complete_observation_positive.rs",
        "full_sha256": "b7cd86a342e6015b04f8e035b7cb5ad7426948861b46e4be7dc9fe04ae760fab",
        "page": "02-01.txt",
        "source_lines": [39, 92],
        "literal": "Ok(ObservedStep::Terminal { .. }) => panic!(\"expected an ordinary prefix event\"),"
      },
      "detail": "The 4 tests in `sherlock_complete_observation_positive.rs` test positive prefix event capture and `FinalReturned` / empty continuity only; `ObservedStep::Terminal`, `CaptureStepError` variants, and `ContinuityBoundary::{EmptyReturned, InitialBefore, Between}` are verified by source inspection of `complete_observation.rs:201-302` rather than exercised by these 4 tests."
    }
  ],
  "read_coverage": {
    "required_inline_read": [
      "01-01.txt (complete_observation.rs, SHA 34b9f4a41bc7be9f68a6af509bb2df99caaa8ae4e45fa302e585a342c701a8b9)",
      "02-01.txt (sherlock_complete_observation_positive.rs, SHA b7cd86a342e6015b04f8e035b7cb5ad7426948861b46e4be7dc9fe04ae760fab)",
      "03-01.txt (helper_machine/mod.rs, SHA e348e37ea574d2ed1ffb1bae80bd7890d7c9f70e15ecb90f92f9a947fe2f8df0)",
      "04-01.txt..04-03.txt (shared.rs, full SHA 479eb3983c81e9734d5bd427b68d735e627010368e6c992687c9ff5ff2dad3a9)",
      "05-01.txt..05-03.txt (raw.rs, full SHA ef9c70a41933acf2c4310dab566c83fd72087b8ea435869a8c283f6c40aff7d1)",
      "06-01.txt (owner_host.rs, SHA b6d0524431df4e73898853b073ba9c3f5a212fd83f3dd4015c6a4e1eda1d0017)",
      "25-01.txt (governing_H1_source, SHA 16b1fb249c94804ad1b1e19856875e2070fd918a7d687b7262a859f59d4c8be2)",
      "26-01.txt (prospective_UNRUN_exact_plan, SHA 7b0a0663ba20507e7f636c20fb0d93e624e427290b1174151cc29da651e4c002)",
      "27-01.txt (actual_production_metadata, SHA 5acdcac1b57fb0fd49d786474615c24e086cf4fabb35a1003cffd7feb4188ad9)",
      "28-01.txt (actual_external_test_metadata, SHA d8e2610536fb9637c9b996723443c6f9c8b83587d139531d45813d1b15e55ec0)"
    ],
    "required_view_read": [
      "07-01.txt (host_implied_prefix.rs, SHA 7a1cbeb98845afbf1f49a15887407f885e597213e31374ffad12d69804b48734)",
      "08-01.txt (source_input_bridge.rs, SHA 87be4cc95d27c74c806363f862ff9c032160af18274ba81bb03f7f8a150a9441)",
      "09-01.txt (reference_types.rs, SHA 4b4bb364a0c8182e117a9c36efb5d91cb7e405f653a6d3870f54dc03eefa317f)",
      "10-01.txt..10-04.txt (owner_prepare.rs, full SHA da558e3e0f3498fe7da646d860af2488337201ad93b3bac5ebc5389bddc5e542)",
      "11-01.txt..11-09.txt (owner_runtime.rs prefix lines 1-1516, full SHA 4c3288e086f63ecb3745f3ca1bc8bd1f093279e85a0cf2da6ab6805c0a2b54e7, selected SHA cc810adab9a2a4a9f26f27ed7167b72be6062114c11824c01ba00838c5cbe7c1)",
      "12-01.txt (families/mod.rs, SHA 7507b2020fba6ce17ace05646f3470eb9df1682410792c8b6958609eebf78f45)",
      "13-01.txt (families/legacy.rs, SHA 67f5d04573deba41b7a98d246fa33517ea5bbd7b152affe30c0fbc0576f1f379)",
      "14-01.txt..14-02.txt (fragments/legacy.proposed.rs, full SHA 96498b8a1e85ce7da24a0cc4bdf24e92b1e4fdb355c2dd1ec92562a53c49c1ce)",
      "15-01.txt (owner_si21_bind_match.rs, SHA 6a615d67f60a53b1dcfef8b742d02ca29181d8205904eb61580c9e804da029ef)",
      "16-01.txt..16-02.txt (owner_si21_closure.rs, full SHA de0c099200c847a6b9b80b021dfb5b54b792c280cfb32398e164bc672aff5e6b)"
    ],
    "optional_view_read": [
      "20-01.txt..20-02.txt (families/structural.rs, full SHA b356f61eb32e2f35aa1cab1e7b2e66ea32a96871d519f5db36c2c434e7359be6 — read to verify `plan_validate` executed by the 2-step `UInt(7)` literal prefix)",
      "21-01.txt (crates/mori-core/src/lib.rs, SHA df45168fa300562f532577739731c257d016a387fc943b25975ae46e3968a2fa — read to verify crate root module graph)",
      "22-01.txt (crates/mori-core/src/sherlock/mod.rs, SHA 467eaf6382194217dbcfe6ab99be14ccf21aa81ebbb7ab4e91820ddceff94c0f — read to verify `sherlock::helper_machine` export)"
    ],
    "optional_unreads": [
      "17-01.txt (families/scalar.rs, SHA 69dde78c695cc78c26a020c4a45a053cb35af4b7470752bd65f2fbf76b0db888 — unreachable in 2-step UInt(7) literal prefix)",
      "18-01.txt (fragments/scalar.proposed.rs, SHA beda415aaf730aeee7b0f07a8188c451a748dd6b9e429010618ad92ea8852511 — unreachable in 2-step UInt(7) literal prefix)",
      "19-01.txt (families/environment.rs, SHA 1d7d7835cffabe2e390817e6d628cfa38e5afabcc3a6693b59ab23056906e40a — unreachable in 2-step UInt(7) literal prefix)",
      "23-01.txt..23-02.txt (sherlock/source_adapter.rs, full SHA 6fe7193e3e0132009b117a43ebad1bb1a247683dfa89105567c29b0166e6d02a — not called by complete_observation or the 4 literal tests)",
      "24-01.txt..24-02.txt (crates/mori-core/src/ir.rs, full SHA 0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d — not called by complete_observation or the 4 literal tests)",
      "owner_runtime.rs lines >1516 (omitted pre-existing #[cfg(test)] body; not compiled when `--cfg test` is absent on `libmori_core.rlib`)"
    ]
  },
  "native_or_general_grants": "NONE"
}
```

### Concise Owner Review Summary

1. **Decision A — Source & Root Registration (`APPROVE_SOURCE_COMPOSITION_WITH_SCOPE`)**:
   - [`complete_observation.rs`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/44-b-complete-observation-owner-review/pages/01-01.txt#L1-L312) (`34b9f4a4...`) and [`helper_machine/mod.rs`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/44-b-complete-observation-owner-review/pages/03-01.txt#L12-L13) (`e348e37e...`) preserve all `shared` visibility boundaries: as a sibling of `shared`, `complete_observation` cannot forge or mutate `Admitted`, `MachineImage`, `PreImage`, `OwnerGuards`, or `ValidatedPlan`.
   - [`copy_observation`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/44-b-complete-observation-owner-review/pages/01-01.txt#L150-L191) and [`capture`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/44-b-complete-observation-owner-review/pages/01-01.txt#L193-L195) copy every public field and variant of [`HostObservation`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/44-b-complete-observation-owner-review/pages/06-01.txt#L36-L136) (including all 8 `ExpressionRowC` variants, all 7 `raw::ConstRow` variants with `marker_free` and `ConstFieldSlice`, all 6 `raw::DataRow` variants with `FieldSlice`, `WidthsSnapshot`, and `SegBase`).
   - [`record_step`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/44-b-complete-observation-owner-review/pages/01-01.txt#L230-L256) captures `before`, invokes `host_owner_engine().step(admitted, image)` once, retains the full `StepEventC` on `StepResult::Event`, separates `ObservedStep::Terminal { before, after }` without fabricating an event, and retains `before` plus post-step `capture` evidence in `CaptureStepError::{Step, AfterObservation}`.

2. **Decision B — Four Fixed N01 Test Classification (`ELIGIBLE_ORDINARY_SDK_FREE_H1_SATISFIED_WITH_CONDITIONS`)**:
   - [`sherlock_complete_observation_positive.rs`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/44-b-complete-observation-owner-review/pages/02-01.txt#L1-L93) (`b7cd86a3...`) under [`prospective_UNRUN_exact_plan`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/44-b-complete-observation-owner-review/pages/26-01.txt#L1-L95) (`7b0a0663...`) is **Safe-static** (SDK-free) and satisfies H1 **solely** for those 4 fixed tests (`UInt(7)` literal, `Nat(64)` fuel, `Nat(8)` stack, `FinanceInput::Absent`, `2 + 0 + 2 + 1 = 5` fixed `step` calls executing only `BHelperValidateEntry` and `BHelperValidateOccurrence`, no `run` loop), subject to mandatory conditions **C1–C5** above. No general caller, whole-crate, C4, R2, 60460, or native authority is granted.
