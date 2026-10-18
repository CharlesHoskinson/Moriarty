{
  "coverage": {
    "inline_complete": true,
    "prior_context_available": true,
    "unique_pages": 16,
    "bodies": 35,
    "total_inline_bytes": 199442,
    "page_read_order_verified": [
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/001.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/002.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/003.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/004.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/005.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/006.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/007.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/008.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/009.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/010.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/011.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/012.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/013.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/014.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/015.txt",
      "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/311-shared-cli-results-report-consumer-review/pages/016.txt"
    ],
    "gaps": [
      "SOURCE_AVAILABLE_NOT_FRESHLY_READ_IN_R311: Transitive existing Core implementation files (`crates/mori-core/src/sherlock/compile_artifact/wire.rs` 51045B sha256 727f8a3a2fd01bd03ee98a3e3bb3beca1626ce9fe0396dd523232780598bec9e, `crates/mori-core/src/sherlock/helper_machine/resource_report.rs` 18780B sha256 3604be306a36d03e7f2c429823ca2886775a8295dea44582a4ddf6dc10d03a40, and the 134 unchanged normal `mori-core` and `mori-lexer` `.rs` files in the R301 138-file snapshot including `owner_runtime.rs`, `owner_si21_bind_match.rs`, `recursive_source.rs`, `draft_source.rs`, `draft_typecheck.rs`, `fragments/legacy.proposed.rs`, and `fragments/scalar.proposed.rs`) and the 570 cached third-party crate source files across the 17-package Clap closure remain SHA256-pinned source-available rather than freshly read line-by-line in this follow-up.",
      "PINNED_BINARY_AND_DEPINFO_IDENTITY_NOT_SEMANTIC_DECOMPILATION: Generated R299 `.rlib`/`.so`/test-binary/`.d` artifacts, generated R301 `libmori_core.rlib` (34971056B sha256 53db3f27a6784022d82afe57a8e4ac159e3bc156ca3b5e05aecf163b60d2ba8a) and `core-rlib.d` (39018B sha256 25a0fee1869e8a04a2636b818174ae077214a21aee1571bddb7a9801f41d7199), and the 273 normal dependency/toolchain/linker/sysroot ELF artifacts (`normalDependencyToolInventory` from R206/R279) are bound by byte/SHA256 pins and first-rule `.d` closures rather than binary decompilation or syscall tracing.",
      "WHOLE_LEDGER_BODY_NOT_FRESHLY_REVIEWED: In R305, the 647293B base (`975b6e34bb51373522fab91c6deffddf5bda2530f5156bbbd6925f63f41a2182`) and candidate (`555899cede04ff2ff8148dbb7717116fd8d0a057be21426c8caf0f9b09190074`) `current-evidence.json` ledgers are verified via exact 18-reference / 36-operation JSON Patch reconstruction (`BODY15`, `BODY18`, `BODY19`), not a fresh whole-ledger semantic review.",
      "OUTSIDE_SELECTED_CLOSURE: Full `mori` binary crate compilation/execution, global `Cli::try_parse_from` dispatch, `commands::simulate::run`, `commands::simulate_sherlock::run`, `commands::replay::run`, and `mori-backend`/`mori-ledger`/`midnight-*` SDK/ledger dependencies remain uncompiled and unexecuted."
    ]
  },
  "identity": {
    "actual_CID": "5bbfb99c-d07d-4659-81fa-29f4d817d7d9",
    "original_cwd": "/tmp/moriarty-r300-shared-cli-source-review-y9gzlvos",
    "same_actual_R300_reviewer": true,
    "requested": "gemini-4-argon-high (standing user-authorized observed-Opus-outage fallback)",
    "returned": "Gemini 4 Argon (High) (Antigravity agentic coding assistant in same CID 5bbfb99c-d07d-4659-81fa-29f4d817d7d9)",
    "prior_R300_raw_response_sha256": "547770b21f6ead5587b5bc18c340b52bd7a917952de292019cdfecd2f3aa7fc7",
    "prior_R300_result_sha256": "5ed0a854a864ae509f494022f7914ac875758f713f88faae936374e0b51cbf3a",
    "qualification": "One same-owner follow-up continuing the actual R300 review session; neither a source-author vote nor a fresh second peer reviewer.",
    "HIGH_evidence_limits": [
      "Runtime model string is self-reported from harness/system context without hardware or cryptographic provider attestation.",
      "Exact page byte/line read counts confirm full delivery of the 16 allowlisted pages but do not by themselves prove semantic comprehension.",
      "Root agent alone selects subject execution; per the current policy note, human-only beta approval gates are removed, while reviewer execution_authority=0 and native_authority=0 remain unchanged."
    ]
  },
  "decision1_actual_results": {
    "R299": {
      "classification": "VERIFIED_SCOPED_R299_17_PHASE_SHARED_ARGS_AND_CLAP_CLOSURE_PASS",
      "plan_sha256": "59d3f1ce603f3927ed1278302c3212c2e443845b012dbc3a3378d4af60ad42f0",
      "observed_phases": 17,
      "all_phases_exit_0": true,
      "pre_and_post_1611_input_pins_unchanged": true,
      "summary": "All 14 cached third-party Clap-closure compile phases (01-compile_syn through 14-compile_clap, including libclap_derive.so 9331952B sha256 9d5ec6792ae42b22ec8467df5ba3a2599facde93bd2cc66a04e590fbf0f51c10), phase 15 direct 6-source consumer compile (shared-simulate-arguments-tests 16781920B sha256 6df7ba4f741db16644a5a1a19adc7ec46b08bf9666869c1250515ecdb4628c36 linked against R264 libmori_core.rlib), phase 16 exact 1-test list (103B stdout sha256 91c7f278c3f700c6d84ca46aff5b554893394144f07c9cf4562eb972b0d1b363), and phase 17 exact 1-test execution of shared_production_simulate_arguments_and_selection_match_original_24_cases (199B stdout sha256 7699552829060a10397f483646d2662c80a79cefbac2176cd52c6a40539c8324) exited 0 with 0B stderr across all 17 phases and all first-rule .d source dependencies inside the pinned namespace (including 8 byte-verified relative-path aliases across clap_builder.d, clap_derive.d, clap.d, and shared-simulate-arguments-tests.d)."
    },
    "R301": {
      "classification": "VERIFIED_SCOPED_R301_NORMAL_CORE_COMPILE_ONLY_PASS_WITH_PRESERVED_RAW_FAIL_STOP_AND_ALIAS_CORRECTION",
      "plan_sha256": "7b266ade5cc0596613655ceb7bf8b63c3b5dac4ce9b318b677ab708a063217cd",
      "raw_capture_status_preserved": "FAIL_STOP",
      "corrected_scoped_result": "NORMAL_CORE_COMPILE_PASS_NO_RERUN",
      "compiler_invocations": 1,
      "actual_exit": 0,
      "elapsed_seconds": 4.090251816000091,
      "pre_and_post_559_input_pins_unchanged": true,
      "stdout_bytes": 0,
      "stderr_bytes": 4502,
      "stderr_sha256": "2ec3023167648376466afbaa1953f6a01713bf9b4fd5aeca08edaa353e8babe4",
      "warnings_analysis": "9 rustc warnings emitted in 4502B stderr.bin (unused variables `partial`, `loc`, `g`; unused `mut push`; dead_code `check_any_ref`, `checked_guards`, `checked_pre_image`, `SOURCE_LANGUAGE`, `parse_draft_source`, `check_program_types`), all located in unchanged pre-existing mori-core modules (`owner_si21_bind_match.rs`, `owner_runtime.rs`, `recursive_source.rs`, `draft_source.rs`, `draft_typecheck.rs`) and zero warnings in `draft_simulation_report.rs` or `compile_artifact.rs`.",
      "generated_outputs": {
        "libmori_core_rlib": {
          "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/301-new-simulation-report-normal-core-QA/build/libmori_core.rlib",
          "bytes": 34971056,
          "sha256": "53db3f27a6784022d82afe57a8e4ac159e3bc156ca3b5e05aecf163b60d2ba8a"
        },
        "core_rlib_d": {
          "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/301-new-simulation-report-normal-core-QA/build/core-rlib.d",
          "bytes": 39018,
          "sha256": "25a0fee1869e8a04a2636b818174ae077214a21aee1571bddb7a9801f41d7199",
          "source_dependency_count": 75
        }
      }
    },
    "Root_corrections": {
      "R300_coverage_convention_correction": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/300-shared-simulate-source-operation-review/ROOT-COVERAGE-CONVENTION-CORRECTION.json",
        "bytes": 24354,
        "sha256": "64a6e2e7540c21be0eb24cac793de7bc6361693bc953f03cf50bc99e648d503a",
        "assessment": "ACCEPTED_ADDITIVE_CORRECTION_ORIGINAL_FALSE_RETAINED: Original TOOL-READ-COVERAGE.json (sha256 382c88a8626d6207fcf72718026bc6e2ac79ef9bb4ca295473f243099c8df1b8) compared view_file reported line counts (LF+1) against raw LF counts before any subject launch; all 81 R300 pages had exact 1 DONE, exact byte counts, and uniform difference=1 (reported_line_count = actual_LF_count + 1), with no rerun."
      },
      "R301_depinfo_alias_correction": {
        "path": "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/301-new-simulation-report-normal-core-QA/actual-attempt-01/ROOT-DEPINFO-ALIAS-CORRECTION.json",
        "bytes": 2465,
        "sha256": "8ee351c5668442f7c4780f7eed06521f89265a2346bb3a08ec3a264481010906",
        "assessment": "ACCEPTED_ADDITIVE_CORRECTION_ORIGINAL_FAIL_STOP_RETAINED: Original R301 RESULT.json (sha256 4b7d870f06ed97dbd1271b27cddea6340784cd37a2d4439299180a1577671f79) failed post-compile only because Root's script did not normalize `families/../fragments/legacy.proposed.rs` (22227B sha256 96498b8a1e85ce7da24a0cc4bdf24e92b1e4fdb355c2dd1ec92562a53c49c1ce) and `families/../fragments/scalar.proposed.rs` (13694B sha256 beda415aaf730aeee7b0f07a8188c451a748dd6b9e429010618ad92ea8852511) in core-rlib.d. Both normalized paths match the frozen R301 input pins byte-for-byte. The dated `R299_unselected_unexecuted_by_failstop: true` field was accurate at 04:31:42Z before R299 was separately selected at 04:32:13Z and passed all 17 phases."
      }
    },
    "bindings": {
      "all_18_commands_match_reviewed_plans": true,
      "all_36_raw_streams_accounted": true,
      "all_15_depinfo_closures_within_pinned_namespaces_after_lexical_normalization": true,
      "no_artifact_substitution_between_R299_and_R301": true
    },
    "findings": [
      "R299 genuinely compiled the 14 cached Clap-closure crates (including `libclap_derive.so`) and the 6-file direct production argument consumer (`simulate_arguments.rs` + `cli_args.rs` + `cli_args/{replay,simulate,simulate_selection}.rs` + `CLI-EXPECTATIONS.json`) and passed the 1 exact test over all 24 frozen command-shape cases.",
      "R301 genuinely compiled the normal `mori-core` library (`libmori_core.rlib` sha256 53db3f27a6784022d82afe57a8e4ac159e3bc156ca3b5e05aecf163b60d2ba8a) including `compile_artifact.rs` (sha256 9c0749c098cacc523632930a85a1a3c0be1ccf0d5e4a0bccaa35a7a017edd70f) and `draft_simulation_report.rs` (sha256 5b27e4638d17ab474c9d5111437251675d5cd18c3d43cf0b4dcaf2631b1e859c) with compiler exit 0 in a single invocation."
    ],
    "limits": [
      "R299 validates only `simulate::Args::augment_args`, `FromArgMatches::from_arg_matches`, `AuthorizationArgs::present`, `logical_cap`, and `simulate::selection` on the 24 frozen cases; it does not construct or parse global `Cli`, execute `commands::simulate::run`, `commands::simulate_sherlock::run`, or `commands::replay::run`, read files, or run `run_draft_host` or `draft_simulation_report::report`.",
      "R301 is a compile/typecheck/codegen check of `mori_core` only; it executes no test, no `run_draft_host`, no `draft_simulation_report::report`, and no JSON serialization."
    ]
  },
  "decision1_installation_R305": {
    "classification": "APPROVED_CONDITIONAL_14_SOURCE_INSTALLATION_AND_GUARDED_18_REF_36_OP_LEDGER_COMPATIBILITY_PATCH",
    "exact14source_and18ref_scope": {
      "candidate_file_count": 14,
      "existing_worktree_files_modified": 5,
      "new_worktree_files_created": 9,
      "parent_compile_artifact_preimage_sha256": "17ff0a6b88b84c1669bc0594ce561834b83f7436a8028578e918788d6611c99f",
      "parent_compile_artifact_postimage_sha256": "9c0749c098cacc523632930a85a1a3c0be1ccf0d5e4a0bccaa35a7a017edd70f",
      "new_report_leaf_sha256": "5b27e4638d17ab474c9d5111437251675d5cd18c3d43cf0b4dcaf2631b1e859c",
      "deleting_2_registration_lines_restores_entire_old_parent": true,
      "all_9_existing_parent_functions_byte_equal": true,
      "affected_ledger_reference_objects": 18,
      "other_13_candidate_paths_have_zero_ledger_refs": true,
      "guarded_jsonpatch_operations": 36,
      "guarded_patch_sha256": "9d5837cf066743a430a1fe3e42ca5b9a9bff3c7adc0b39f59e1a61bb45a537a6",
      "all_18_reference_proposal_sha256": "0b77d6f63f715ee0bd58b3c6691f0ad1b36626575e8cdaea671c7f53187ac051",
      "base_ledger_sha256": "975b6e34bb51373522fab91c6deffddf5bda2530f5156bbbd6925f63f41a2182",
      "candidate_ledger_sha256": "555899cede04ff2ff8148dbb7717116fd8d0a057be21426c8caf0f9b09190074",
      "span_shifts": "1..293->1..295 (6 refs in UIREQ-002, UIREQ-022; restored body 14861B sha256 17ff0a6b...); 1..245->1..247 (4 refs in UIREQ-025; restored body 12521B sha256 8d4b8de8...); 1..241->1..243 (8 refs in UIREQ-031, SCON-001, SCON-003, SCON-012, SCON-023, SCON-025; restored body 12378B sha256 e493619e...); all other decoded ledger values and acceptance statuses unchanged."
    },
    "conditions": [
      "Before installing the 14 postimages into `@W`, Root must verify that all 5 existing worktree preimages and all 9 non-existent target paths in `@W` still match `SOURCE-PREIMAGES-POSTIMAGES.json` (`BODY17`).",
      "When updating `openspec/changes/sherlock-completion-convergence/current-evidence.json`, Root must apply the 36 guarded RFC 6902 `test`+`replace` operations (`GUARDED-SOURCE-REFS.jsonpatch` sha256 9d5837cf066743a430a1fe3e42ca5b9a9bff3c7adc0b39f59e1a61bb45a537a6) against matching live reference objects rather than blindly overwriting `current-evidence.json` with the frozen candidate afterimage.",
      "Installing the 14 source files and applying the 18-reference span/hash compatibility patch records source reference maintenance only; it does not close any open requirement (`UIREQ-031`, `SCON-001`, `SCON-003`, `SCON-012`, `SCON-023`, `SCON-025`) or establish runtime report or CLI acceptance."
    ],
    "gaps": [
      "Actual worktree installation (`actualWSourceInstallation: NOT_PERFORMED_BY_THIS_LANE`) and any subsequent pure `current-evidence.json` reader check have not yet been executed.",
      "The 647293B `current-evidence.json` ledger outside the 18 reference objects is bound by decoded-value equality and SHA256 rather than a fresh whole-ledger audit."
    ]
  },
  "decision2_R307_source_oracle": {
    "classification": "APPROVED_INDEPENDENT_PUBLIC_SIMULATION_REPORT_TEST_AND_5_FIXTURES_SOURCE_UNRUN",
    "exact_6_file_pins": {
      "EXPECTED-REPORT.json": {
        "bytes": 3864,
        "sha256": "53d2d78d93b3eaef6c6fe07ae195f7cffef1187cecf79e1210367ef0f4b51b91"
      },
      "branch-total.mori": {
        "bytes": 211,
        "sha256": "fbf56e7b9fe589471ba12cf9aa7246ac538291422e662a79db5e7924a300e109"
      },
      "chain-total.mori": {
        "bytes": 177,
        "sha256": "3cec986244abd2e43c5d52894a84c54d69383a2bec327308651ea40a3f787644"
      },
      "config.json": {
        "bytes": 1795,
        "sha256": "3406712e1327ebe87fa9c5265986a6a51ef5856f3498d99ff4a8dd958d70ac17"
      },
      "target.json": {
        "bytes": 64,
        "sha256": "b7b9069acc831ae962d941c75a75c7bc2780e4029a569209458601f9b3f8d56d"
      },
      "sherlock_draft_simulation_report_independent.rs": {
        "bytes": 7484,
        "sha256": "df9bcea320130b9a8f409450cf1c422266201fdedee8d45edb89d71fcc1791c0"
      },
      "EXPECTATION-FREEZE.json": {
        "bytes": 2902,
        "sha256": "77f850a4580f31494c9a146b8dbea606e6a565ab258f6025a26550cb4d3ec7e1"
      },
      "SOURCE-FREEZE.json": {
        "bytes": 3053,
        "sha256": "8926e4293bd23b919a233c056fa1d6f4f20943cccea090deefebfa4ec247d40a"
      }
    },
    "independence": {
      "expectations_frozen_before_test_source_and_execution": true,
      "pending_report_implementation_body_read_by_R307": false,
      "report_actual_output_read_by_R307": false,
      "sent_to_R293_R270_source_author": false,
      "qualification": "R307 authored the expectations and test from the declaration-only signature (`REPORT-API-DECLARATION.json` sha256 120803a5509717182471826963744e383cf19f779b7bb3a7df3116843297702d), the frozen R270 `OUTPUT-CONTRACT.json`, existing `wire.rs`/`resource_report.rs`, and the inherited R259/R278 independent semantic fixtures (`2+5=7` for `ChainTotal`, `4+(1+3)=8` for `BranchTotal`) and corrected `4096` capacities (`READSET.json` sha256 9e7dd4a1cd40e06a8eb1e7f5b00103e516a83df66913882b5b26ee14d94286ae). It does not claim blindness to earlier R279/R286 runtime results, and its top-level report format/qualification strings come from the shared R270 contract rather than an independent numerical derivation."
    },
    "assertion_coverage": {
      "calls_real_public_run_draft_host_and_report_and_serde_serialize": true,
      "verifies_top_level_format_qualification_and_execution_flags": true,
      "verifies_no_retained_events_array_in_report_or_execution": true,
      "verifies_whole_serialized_preparation_equals_run_preparation": true,
      "verifies_independent_source_identity_and_config_digest": true,
      "verifies_genesis_counters_and_9_nonqualification_unrun_unproved_fields": true,
      "verifies_terminal_halted_reference_resolution_and_independent_7_and_8_results": true,
      "verifies_terminal_vs_genesis_outcome_distinction_and_4096_fuel_conservation": true,
      "verifies_callback_count_equals_events_forwarded_and_summary_events": true,
      "verifies_same_run_summary_image_and_resource_report_bindings_and_9_unmeasured_groups": true
    },
    "limits": [
      "Does not supply an independently derived exact B total `consumed_work`, `helper_work`, `helper_visits`, `reference_cursor`, or `events` numerical count oracle (it binds those serialized report fields to the same run's typed `final_image`, `run.completed.summary`, and callback counter).",
      "Does not assert a full B `StepEventC` sequence/payload golden or intermediate machine images, and does not re-run the earlier R279/R286 runtime assertion cohort.",
      "Exercises only the 2 positive fixed normal sources (`chain-total.mori`, `branch-total.mori`) through the library API (`run_draft_host` -> `report` -> `serde_json::to_vec`); it does not invoke the `mori` CLI binary, `commands::simulate_sherlock::run`, or error-path report formatting (`print_failure`)."
    ],
    "findings": [
      "`sherlock_draft_simulation_report_independent.rs` is mechanically and semantically compatible with `mori_core::sherlock_compile_artifact::draft_simulation_report::report` (`5b27e4638d17ab474c9d5111437251675d5cd18c3d43cf0b4dcaf2631b1e859c`) and `run_draft_host` (`55c1029155c14f8ac318531d9d67955a5de4bb14251f59b17b7529a12dc04366`) reviewed in R300, without reading private fields or fabricating report projections."
    ]
  },
  "decision2_R310_ordinary_operation": {
    "classification": "ELIGIBLE_ORDINARY_EXTERNAL_PUBLIC_REPORT_CONSUMER_3_PHASE_OPERATION_FOR_ROOT_SELECTION",
    "exact_plan_sha256": "6c55a96d2b8438178bc1dd17a4f44253e8fd7e393f9743b17bedfd233812becb",
    "exact_review_projection_sha256": "7dd7ed1dce9418a2ed37ebda3f9ccfedc26d7bc39480c54c43cec60452885d36",
    "exact_source_composition_sha256": "e41041e9bd9de634d0230f6c94520058f5e4b37979aeb95c42d18c958c147181",
    "exact_actual_R301_artifact_binding_sha256": "706c887a9745c3f30e14579b9c800a4b72fcd48a72b3e04a8d6dfd4c7406bb7c",
    "exact_flat_input_pins_sha256": "cbbb96a93226064450c5a43103101c986a626ee481f755a635b5239112c4bf6d",
    "reused_R301_core_rlib_sha256": "53db3f27a6784022d82afe57a8e4ac159e3bc156ca3b5e05aecf163b60d2ba8a",
    "reused_R301_core_depinfo_sha256": "25a0fee1869e8a04a2636b818174ae077214a21aee1571bddb7a9801f41d7199",
    "exact3argv_scope": [
      {
        "id": "externalIndependentPublicReportConsumerCompile",
        "wallSeconds": 30,
        "argv": [
          "/usr/bin/timeout",
          "--signal=KILL",
          "30s",
          "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
          "--edition=2024",
          "--crate-name",
          "r310_public_simulation_report_independent",
          "--test",
          "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/310-public-report-consumer-ordinary-QA/snapshot/crates/mori-core/tests/sherlock_draft_simulation_report_independent.rs",
          "--extern",
          "mori_core=/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/301-new-simulation-report-normal-core-QA/build/libmori_core.rlib",
          "--extern",
          "serde_json=/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/24-compiler-qa/proposed-run/core-target/debug/deps/libserde_json-76dc87088753b61e.rlib",
          "-L",
          "dependency=/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/301-new-simulation-report-normal-core-QA/build",
          "-L",
          "dependency=/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/191-bounded-parser-ordinary-qa/build",
          "-L",
          "dependency=/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/24-compiler-qa/proposed-run/core-target/debug/deps",
          "--emit=link,dep-info=/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/310-public-report-consumer-ordinary-QA/build/public-report-consumer.d",
          "-o",
          "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/310-public-report-consumer-ordinary-QA/build/public-report-consumer-tests",
          "-C",
          "debuginfo=0",
          "-C",
          "metadata=moriarty_r310_public_report_consumer_20261018_v1"
        ]
      },
      {
        "id": "listExactOneNewPublicReportTest",
        "wallSeconds": 10,
        "argv": [
          "/usr/bin/timeout",
          "--signal=KILL",
          "10s",
          "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/310-public-report-consumer-ordinary-QA/build/public-report-consumer-tests",
          "--list"
        ]
      },
      {
        "id": "public_simulation_report_serializes_independent_results_and_same_run_bindings",
        "wallSeconds": 10,
        "argv": [
          "/usr/bin/timeout",
          "--signal=KILL",
          "10s",
          "/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/310-public-report-consumer-ordinary-QA/build/public-report-consumer-tests",
          "--exact",
          "public_simulation_report_serializes_independent_results_and_same_run_bindings",
          "--test-threads=1",
          "--nocapture"
        ]
      }
    ],
    "conditions": [
      "Root alone selects execution after verifying all 457 flat input pins (`FLAT-INPUT-PINS.json` sha256 cbbb96a93226064450c5a43103101c986a626ee481f755a635b5239112c4bf6d), including the actual R301 `libmori_core.rlib` (`53db3f27a6784022d82afe57a8e4ac159e3bc156ca3b5e05aecf163b60d2ba8a`) and `core-rlib.d` (`25a0fee1869e8a04a2636b818174ae077214a21aee1571bddb7a9801f41d7199`) and all 6 R307 snapshot copies (`SOURCE-COMPOSITION.json` sha256 e41041e9bd9de634d0230f6c94520058f5e4b37979aeb95c42d18c958c147181); do not substitute the older R264 `libmori_core.rlib`.",
      "Run serially in cwd `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/310-public-report-consumer-ordinary-QA` with exact 3-variable environment (`PATH=/usr/bin:/bin`, `LANG=C.UTF-8`, `TMPDIR=@J/tmp`), per-phase wall timeouts `30s`/`10s`/`10s`, stream cap `131072` bytes with process-group kill on cap, `retry=false`, and `fail_stop=true` under `ROOT-CAPTURE-CODE.txt` (`sha256=01b9e50b952bf700bdb5149bb8fb77943ec4f260e0b5860a4b6ff485cb568d7a`).",
      "After phase 1 (`externalIndependentPublicReportConsumerCompile`), Root must pin the newly generated `public-report-consumer-tests` binary and `public-report-consumer.d` (verifying normalized `.d` source membership stays within the 6 pinned R307 files), verify phase 2 (`--list`) outputs the exact 1-element test set `[\"public_simulation_report_serializes_independent_results_and_same_run_bindings\"]` with 0 benchmarks before running phase 3, and verify all 457 input pins and generated binary/depinfo pins remain unchanged after phase 3."
    ],
    "gaps": [
      "R310 is currently UNRUN (`FINAL_PREPARED_UNSELECTED_UNRUN_PENDING_FRESH_REVIEW`); eligibility classification does not predict PASS or approve unexecuted results.",
      "R310 tests the library-level `run_draft_host` -> `draft_simulation_report::report` -> `serde::Serialize` path on 2 fixed positive sources; it does not compile or execute the `mori` CLI binary, `commands::simulate::run`, `commands::simulate_sherlock::run`, or CLI stdout/exit-code handling."
    ]
  },
  "retained_limits": [
    "All 26 named-unproved host-to-K/Quint/Lean/circuit/A4 (`StepInv`, `decodeCanonical`, `decodeStep`, `project`) mapping and representation obligations remain unproved.",
    "All 5 owner/H1 gaps remain open: GAP-1 (B iterative Drop transient Vec/Box allocations), GAP-2 (A recursive Arc Drop bounded by structural AST/tree depth), GAP-3 (unmetered internal host heap/Vec/Arc/clone allocations in `owner_prepare` and `owner_runtime`), GAP-4 (unmetered host heap allocations in `SourceProvenance`, `DraftCompileArtifact` projection, `OwnerWire`, `ResourceReport::from_owner`, and `serde_json` outside `bounds.serialization_bytes`), and GAP-5 (no process-wide host heap-byte, native call-stack-byte, or wall-clock time admission guarantee / global H1 or untrusted-input sandboxing).",
    "All 3 runtime premises remain open: PREMISE-RT-1 (caller-owned `Vec<StepEventC>` retention in `compare_public_runtime`), PREMISE-RT-2 (uncaught sink panic in `owner_run` and `run_draft_host` unwinds the host stack without `catch_unwind`), and PREMISE-RT-3 (physical host CPU/heap/stack and serialization costs remain outside logical B work counters).",
    "No exact total B work/helper/event count oracle, no full B `StepEventC` sequence/payload golden or intermediate machine image oracle, and no general fuel/capacity sufficiency proof beyond the two fixed 4096-fuel cases.",
    "Full `mori` CLI binary compilation, global `Cli` dispatch, `commands::simulate::run` / `commands::simulate_sherlock::run` handler execution, and `mori-backend`/`mori-ledger`/`midnight-*` SDK/ledger/native/target verification remain unrun and outside scope.",
    "All existing counters, budgets, and governance limits (C4/R2/60460/stopped Amendment 10, execution_authority=0, native_authority=0) remain unchanged."
  ],
  "later_R310_actual_result_review_required": true,
  "original_EARS_closure_claim": false,
  "execution_authority": 0,
  "native_authority": 0,
  "end_marker": "END-SAME-R300-TWO-DECISIONS-NATIVE0"
}

