{
  "reviewer_identity": {
    "role": "fresh_non_author_designated_security_owner",
    "model": "Gemini 4 Argon (High) fallback during observed Opus outage (non-Opus, non-R67-author)",
    "native_authority": 0
  },
  "source_review": {
    "disposition": "CONDITIONALLY_APPROVED_IN_INSPECTED_SCOPE",
    "preflight_ordering_assessment": {
      "sequence": [
        "1. Config/target decode: compile_host calls parse_config(config_source) (crate::json::parse -> serde_json::from_value::<ExecutionConfig> -> config.consumer()), calls config.consumer() again, then crate::json::parse(target_source) at stage 'config'.",
        "2. Guard step 0: `validate_config(&consumer)` in `compile_host` (`current/crates/mori-core/src/sherlock/compile_artifact.rs`) rejects zero SourceHostBounds fields in declaration order, non-SherlockHelperMicrostep1 profile, non-DedicatedSorts grammar, and non-Absent finance at stage 'host-guard' before `crate::parse(source)`.",
        "3. Parse & global extents: after `crate::parse(source)` (stage 'parse'), `preflight_source_global(&program, &consumer).map_err(CompileFailure::consumer)?;` in `compile_host` bounds `SourceActions`, `SourceExpressions`, and checked-sum `SourceStatements` across all actions before action lookup.",
        "4. Action lookup & selected input extents: `program.actions.iter().position(|action| action.name == action_name)` (stage 'select' on miss) precedes `let action = preflight_action_inputs(&program, action_index, &inputs, &consumer)` in `compile_host`, which enforces `ActionInputs`, `SuppliedInputs`, per-name `InputNameBytes`, and checked-sum `TotalInputNameBytes` on the selected action before `if !action.inputs.is_empty()` serializes `\"declared_inputs\":action.inputs` (stage 'select').",
        "5. Full preflight vs early refusals: neither `preflight_source_global` nor `preflight_action_inputs` calls `crate::validate`. Refusals at config, host-guard global/input bounds, unknown action, or parameterized-action return BEFORE `crate::validate(program)`. Only parameter-free actions reach `prepare_scalar_action_host` / `run_scalar_action` -> `preflight_source`, where `crate::validate(program).map_err(ConsumerError::CoreValidation)?;` (`current/crates/mori-core/src/sherlock/helper_machine/core_action_consumer_security.rs`) and DAG `ProjectionOccurrences` precounting run prior to `prepare_scalar_action`, `check_generated`, `check_combined_generated_ast_nodes`, `source_input_bridge::from_program`, `engine.admit`, and `engine.admit_run`."
      ]
    },
    "complete_comparison_review": {
      "anchor": "`pub fn first_difference<'a>` in `current/crates/mori-core/src/sherlock/helper_machine/complete_comparison.rs`",
      "finding": "Pure read-only slice/snapshot comparator (`expected_initial != observed_initial`, per-index `before`/`event`/`after`, then length). Invokes no owner engine, allocator loop, or native/SDK code."
    }
  },
  "original_finding_dispositions": [
    {
      "id": "R23_mandatory_1_preflight_ordering_and_bounds",
      "disposition": "RESOLVED_IN_INSPECTED_SOURCE",
      "anchor": "`validate_config(&consumer)`, `preflight_source_global(&program, &consumer).map_err(CompileFailure::consumer)?;`, `let action = preflight_action_inputs(&program, action_index, &inputs, &consumer)` in `compile_host` (`current/crates/mori-core/src/sherlock/compile_artifact.rs`)",
      "analysis": "Guard step 0 (`validate_config`), global source extents (`SourceActions`, `SourceExpressions`, `SourceStatements`), and selected-action input/name-byte extents (`ActionInputs`, `SuppliedInputs`, `InputNameBytes`, `TotalInputNameBytes`) now execute in order before `action.inputs` is serialized into `CompileFailure`. By design, `crate::validate(program)` remains in `preflight_source` after the parameter-free gate so bounded parameterized actions fail with stage 'select' (`parameterized-action`) rather than `MissingInput` or later validation.",
      "remaining_correction": "None blocking. Non-security redundancy remains: `config.consumer()` is called in both `parse_config` and `compile_host`, and `validate_config` / `preflight_source_global` / `preflight_action_inputs` re-run inside `preflight_source`.",
      "uncertainty": "Low for ordering in `compile_artifact.rs` and `core_action_consumer_security.rs`; `config.consumer()` internals in `compile_artifact/config.rs` were not re-read inline."
    },
    {
      "id": "R23_mandatory_2_cli_failure_escaping_and_routing",
      "disposition": "RESOLVED_AT_CALLSITE_WITH_DOWNSTREAM_ROUTING_ABSTENTION",
      "anchor": "`match mori_core::json::export(failure) { Ok(diagnostic) => core_io::operation(diagnostic),` in `report_sherlock_failure` (`current/crates/mori/src/commands/compile.rs`) and `if character.is_ascii() && (!character.is_control() || character == '\\n')` in `export` (`current/crates/mori-core/src/json.rs`)",
      "analysis": "`report_sherlock_failure` no longer calls `serde_json::to_string` or `eprintln!`. It uses `mori_core::json::export`, enforcing the 1 MiB ceiling, depth<=32, nodes<=65536, duplicate-key rejection, and UTF-16 `\\u{unit:04x}` escaping of DEL (`\\u007f`), C1 controls, and non-ASCII/BiDi controls, then routes via `core_io::operation`.",
      "remaining_correction": "None in `report_sherlock_failure` or `json::export`.",
      "uncertainty": "Downstream handling of `core_io::operation(diagnostic)` and `mori_diagnostics::Report` / `Shell` quiet-mode rendering is outside the inline files and unestablished."
    }
  ],
  "exact_five_tests_classification": {
    "status": "ELIGIBLE_FOR_ORDINARY_HOST_PREPARATION_TEST_RUN_UNDER_CONDITIONS",
    "exact_selectors": [
      "config_and_zero_host_bounds_precede_parse_and_unknown_action_scan",
      "global_counts_precede_unknown_action_scan_and_respect_order_and_boundaries",
      "selected_input_count_and_name_byte_extents_precede_parameterized_diagnostic",
      "within_bounds_parameterized_action_is_select_unsupported_not_missing_input",
      "ordinary_parameter_free_compile_yields_actual_admission_and_occurrence_bounds"
    ],
    "reachable_production_calls": {
      "anchor": "`host::compile_host(source, action, config, TARGET, false)` in `compile_ok` and `compile_err` (`current/crates/mori-core/tests/sherlock_compile_preflight_regressions.rs`)",
      "refusal_paths": "Selectors 1, 2, 4 and the error assertions of 3 and 5 exercise `parse_config`, `crate::json::parse`, `validate_config`, `crate::parse`, `preflight_source_global`, action lookup, `preflight_action_inputs`, `parameterized-action` refusal, and `preflight_source` (`ProjectionOccurrences` after `crate::validate`), all with `observe = false`.",
      "positive_admission_path": "Only `compile_ok` in selectors 3 (`plain`) and 5 (`QUOTE`, `ELIGIBILITY`) enters `let prepared = prepare_scalar_action_host(&program, action_index, &inputs, &consumer)` -> `preflight_source` -> `prepare_scalar_action` -> `check_generated` -> `check_combined_generated_ast_nodes` -> `source_input_bridge::from_program` -> `host_owner_engine().admit` -> `engine.admit_run(..., &FinanceInput::Absent)` -> `prepared.observe()` -> `wire::admission`. Because `observe` is literal `false`, `run_scalar_action` and `OwnerEngine::run`/`step` are unreachable."
    },
    "oracle_evaluation_9_and_22": "In `preflight_source` + `binary_occurrence_overhead` (`core_action_consumer_security.rs`), `QUOTE` precounts to 9 (`7*11` Let=1+3, `base+5` Let=1+3, Emit=0+1) and `ELIGIBILITY` precounts to 22 (`3<9` Let=1+3, `9>3` Let=1+7, `low and high` Let=1+8, Emit=0+1). The precount thresholds 9 and 22 are verified from inline guard arithmetic; actual adapter occurrence materialization in `prepare_scalar_action` (`source_adapter.rs`) is an uninspected transitive step, not an adopted independent oracle.",
    "linking_vs_selected_execution": "Compiling `sherlock_compile_preflight_regressions` compiles/links `mori_core` library modules (R70 recorded exit 0; R71 lost child exits so tests remain UNRUN), whereas invoking the binary with `--exact <selector> --nocapture` executes only that single test function with `observe = false`."
  },
  "required_conditions_or_corrections": [
    "Execute only via Root ordinary synthetic host-preparation test harness using `--test sherlock_compile_preflight_regressions -- --exact <selector> --nocapture` for the five listed selectors only.",
    "Do not pass `--observe`, invoke the `mori` CLI binary, run full workspace test suites, or invoke `mori_backend`/SDK/native/prover/keygen/target operations.",
    "Treat classification as host-preparation test eligibility only, not a pass verdict, H1 closure, or native/target grant."
  ],
  "abstentions": [
    "No optional files from OPTIONAL-SOURCE-INDEX.json were read; bounded abstention on uninspected transitive implementations (`compile_artifact/config.rs`, `compile_artifact/wire.rs`, `parser.rs`, `runtime.rs`, `source_adapter.rs`, `source_input_bridge.rs`, `owner_host.rs`, and CLI `core_io`/`Shell`/`mori_diagnostics`).",
    "Abstain on R71 pass status (child exit codes unrecorded; tests remain UNRUN).",
    "Abstain on `--observe`, H1, `OwnerEngine::run`/`step`, public CLI execution, and all native/SDK/prover/target authority."
  ]
}

