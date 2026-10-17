//! Host-only preflight precedence, bound-extent, parameterized-selection, and
//! parameter-free admission regressions for `mori_core::sherlock_compile_artifact`.

use mori_core::sherlock_compile_artifact::{
    self as host, CompileArtifact, CompileFailure, Observation,
};
use serde_json::{json, Value};

const CONFIG: &str = r#"{
  "format": "sherlock-host-config/1",
  "profile": "sherlock-helper-microstep/1",
  "grammar": "DedicatedSorts",
  "finance": "Absent",
  "bounds": {
    "code_rows": "64",
    "function_rows": "4",
    "code_child_slots": "16",
    "parameter_slots": "8",
    "pattern_rows": "8",
    "match_plan_rows": "8",
    "match_arm_rows": "8",
    "type_rows": "8",
    "constructor_rows": "8",
    "constructor_field_slots": "16",
    "max_arity": "8",
    "constant_nodes": "16",
    "constant_field_slots": "16",
    "data_nodes": "64",
    "data_field_slots": "64",
    "environment_cells": "64",
    "frames": "64",
    "helper_tasks": "64",
    "serialization_bytes": "65536",
    "max_reference_work": "4096",
    "max_helper_work": "4096",
    "step_cap": "4096"
  },
  "layout": {
    "layout_version": [
      "moriarty",
      "sherlock-helper-microstep-hostonly/1"
    ],
    "field_id": null,
    "widths": null,
    "scalar_layout_id": null,
    "capacities": {
      "name_rows": "32",
      "name_bytes": "256",
      "occurrence_path_slots": "256",
      "value_slots": "1024"
    },
    "boundary_capacities_present": false
  },
  "fuel": {
    "kind": "Nat",
    "value": "1000"
  },
  "stack": {
    "kind": "Nat",
    "value": "32"
  },
  "host": {
    "max_source_actions": "4",
    "max_source_expressions": "256",
    "max_source_statements": "64",
    "max_action_inputs": "8",
    "max_supplied_inputs": "8",
    "max_projection_occurrences": "512",
    "max_generated_ast_nodes": "512",
    "max_ast_nesting": "128",
    "max_input_name_bytes": "64",
    "max_total_input_name_bytes": "512"
  }
}
"#;

const QUOTE: &str = r#"language "moriarty/1";
agreement FixedQuote {
  action quote() {
    let base = 7 * 11;
    let total = base + 5;
    emit "quoted"(total);
  }
}
"#;

const ELIGIBILITY: &str = r#"language "moriarty/1";
agreement FixedEligibility {
  action inspect() {
    let low = 3 < 9;
    let high = 9 > 3;
    let eligible = low and high;
    emit "eligible"(eligible);
  }
}
"#;

const TARGET: &str = "{\"kind\":\"public-host-test-descriptor\"}\n";

const TWO_ACTIONS: &str = r#"language "moriarty/1";
agreement TwoActions {
  action first() {
    let a = 1 + 2;
    emit "first"(a);
  }
  action second() {
    let b = 3 + 4;
    emit "second"(b);
  }
}
"#;

fn with_host_bounds(overrides: &[(&str, &str)]) -> String {
    let mut root: Value = serde_json::from_str(CONFIG).expect("valid baseline config");
    let host_map = root["host"].as_object_mut().expect("host object");
    for (field, value) in overrides {
        host_map.insert((*field).to_owned(), Value::String((*value).to_owned()));
    }
    serde_json::to_string(&root).expect("serialize config")
}

fn compile_ok(source: &str, action: &str, config: &str) -> CompileArtifact {
    host::compile_host(source, action, config, TARGET, false).unwrap_or_else(|failure| {
        panic!(
            "expected compile_host(..., false) admission, got failure: {}",
            serde_json::to_string(&failure).unwrap()
        )
    })
}

fn compile_err(source: &str, action: &str, config: &str) -> CompileFailure {
    match host::compile_host(source, action, config, TARGET, false) {
        Ok(artifact) => panic!(
            "expected compile_host(..., false) failure, got artifact: {}",
            serde_json::to_string(&artifact).unwrap()
        ),
        Err(failure) => {
            assert!(!failure.evaluation_performed);
            assert_eq!(failure.events_forwarded, "0");
            assert!(!failure.completed);
            failure
        }
    }
}

fn assert_exceeded(failure: &CompileFailure, measure: &str, required: u128, bound: u128) {
    assert_eq!(failure.stage, "host-guard");
    assert_eq!(
        failure.actual_error,
        json!({
            "Exceeded": {
                "measure": measure,
                "required": required.to_string(),
                "bound": bound.to_string(),
            }
        })
    );
}

#[test]
fn config_and_zero_host_bounds_precede_parse_and_unknown_action_scan() {
    let mut bad_layout: Value = serde_json::from_str(CONFIG).unwrap();
    bad_layout["layout"]["boundary_capacities_present"] = Value::Bool(true);
    let boundary_err = compile_err(
        "not valid moriarty source",
        "nonexistent_action",
        &serde_json::to_string(&bad_layout).unwrap(),
    );
    assert_eq!(boundary_err.stage, "config");
    assert_eq!(
        boundary_err.actual_error,
        json!({"message": "layout.boundary_capacities_present must be false in sherlock-host-config/1"})
    );

    let zero_bounds_cfg = with_host_bounds(&[
        ("max_source_statements", "0"),
        ("max_input_name_bytes", "0"),
    ]);
    let zero_err = compile_err("not valid moriarty source", "nonexistent_action", &zero_bounds_cfg);
    assert_eq!(zero_err.stage, "host-guard");
    assert_eq!(
        zero_err.actual_error,
        json!({"InvalidHostBounds": {"field": "max_source_statements"}})
    );

    let parse_err = compile_err("not valid moriarty source", "nonexistent_action", CONFIG);
    assert_eq!(parse_err.stage, "parse");
}

#[test]
fn global_counts_precede_unknown_action_scan_and_respect_order_and_boundaries() {
    let five_actions = r#"language "moriarty/1";
agreement FiveActions {
  action a1() { emit "e1"(1); }
  action a2() { emit "e2"(2); }
  action a3() { emit "e3"(3); }
  action a4() { emit "e4"(4); }
  action a5() { emit "e5"(5); }
}
"#;
    let default_actions_err = compile_err(five_actions, "unknown_action", CONFIG);
    assert_exceeded(&default_actions_err, "SourceActions", 5, 4);

    // TWO_ACTIONS has 2 actions, 8 flat expressions, and 4 statements summed across both actions.
    let all_exceeded_cfg = with_host_bounds(&[
        ("max_source_actions", "1"),
        ("max_source_expressions", "7"),
        ("max_source_statements", "3"),
    ]);
    let actions_first = compile_err(TWO_ACTIONS, "unknown_action", &all_exceeded_cfg);
    assert_exceeded(&actions_first, "SourceActions", 2, 1);

    let exprs_second_cfg = with_host_bounds(&[
        ("max_source_actions", "2"),
        ("max_source_expressions", "7"),
        ("max_source_statements", "3"),
    ]);
    let exprs_second = compile_err(TWO_ACTIONS, "unknown_action", &exprs_second_cfg);
    assert_exceeded(&exprs_second, "SourceExpressions", 8, 7);

    // Each action in TWO_ACTIONS has only 2 statements, but global preflight sums all actions (4 > 3).
    let stmts_third_cfg = with_host_bounds(&[
        ("max_source_actions", "2"),
        ("max_source_expressions", "8"),
        ("max_source_statements", "3"),
    ]);
    let stmts_third = compile_err(TWO_ACTIONS, "unknown_action", &stmts_third_cfg);
    assert_exceeded(&stmts_third, "SourceStatements", 4, 3);

    // At the exact global boundaries (2, 8, 4), global preflight passes and unknown action scan runs.
    let exact_global_cfg = with_host_bounds(&[
        ("max_source_actions", "2"),
        ("max_source_expressions", "8"),
        ("max_source_statements", "4"),
    ]);
    let unknown_err = compile_err(TWO_ACTIONS, "unknown_action", &exact_global_cfg);
    assert_eq!(unknown_err.stage, "select");
    assert_eq!(
        unknown_err.actual_error,
        json!({"message": "unknown action: unknown_action"})
    );
}

#[test]
fn selected_input_count_and_name_byte_extents_precede_parameterized_diagnostic() {
    let nine_inputs = r#"language "moriarty/1";
agreement NineInputs {
  action plain() { emit "ok"(1); }
  action overloaded(i1: UInt, i2: UInt, i3: UInt, i4: UInt, i5: UInt, i6: UInt, i7: UInt, i8: UInt, i9: UInt) {
    emit "ok"(i1);
  }
}
"#;
    let count_err = compile_err(nine_inputs, "overloaded", CONFIG);
    assert_exceeded(&count_err, "ActionInputs", 9, 8);
    // Unselected action with 9 inputs does not block parameter-free sibling selection.
    let sibling_artifact = compile_ok(nine_inputs, "plain", CONFIG);
    assert_eq!(sibling_artifact.source.action, "plain");
    assert_eq!(sibling_artifact.admission["status"], json!("admitted"));

    let name_65 = "x".repeat(65);
    let long_name_src = format!(
        "language \"moriarty/1\";\nagreement LongName {{\n  action take({name_65}: UInt) {{ emit \"ok\"({name_65}); }}\n}}\n"
    );
    let name_err = compile_err(&long_name_src, "take", CONFIG);
    assert_exceeded(&name_err, "InputNameBytes", 65, 64);

    let two_inputs_src = r#"language "moriarty/1";
agreement PrecedenceInputs {
  action check(aaaa: UInt, bbbbb: Bool) {
    emit "ok"(aaaa);
  }
}
"#;
    // ActionInputs (2 > 1) precedes InputNameBytes (5 > 4) and TotalInputNameBytes (9 > 6).
    let both_exceeded_cfg = with_host_bounds(&[
        ("max_action_inputs", "1"),
        ("max_input_name_bytes", "4"),
        ("max_total_input_name_bytes", "6"),
    ]);
    let count_first = compile_err(two_inputs_src, "check", &both_exceeded_cfg);
    assert_exceeded(&count_first, "ActionInputs", 2, 1);

    // InputNameBytes (5 > 4) precedes TotalInputNameBytes (9 > 6) and parameterized diagnostic.
    let item_exceeded_cfg = with_host_bounds(&[
        ("max_action_inputs", "2"),
        ("max_input_name_bytes", "4"),
        ("max_total_input_name_bytes", "6"),
    ]);
    let item_second = compile_err(two_inputs_src, "check", &item_exceeded_cfg);
    assert_exceeded(&item_second, "InputNameBytes", 5, 4);

    // Individual input names at exact bound (4 <= 4 each), total (8 > 7) fails TotalInputNameBytes.
    let total_exceeded_src = r#"language "moriarty/1";
agreement TotalBytes {
  action check(aaaa: UInt, bbbb: Bool) {
    emit "ok"(aaaa);
  }
}
"#;
    let total_exceeded_cfg = with_host_bounds(&[
        ("max_action_inputs", "2"),
        ("max_input_name_bytes", "4"),
        ("max_total_input_name_bytes", "7"),
    ]);
    let total_third = compile_err(total_exceeded_src, "check", &total_exceeded_cfg);
    assert_exceeded(&total_third, "TotalInputNameBytes", 8, 7);
}

#[test]
fn within_bounds_parameterized_action_is_select_unsupported_not_missing_input() {
    let exact_bounds_src = r#"language "moriarty/1";
agreement ParameterizedCheck {
  action plain() {
    emit "ok"(1);
  }
  action quoted(aaaa: UInt, bbbb: Bool) {
    emit "quoted"(aaaa);
  }
}
"#;
    let exact_input_cfg = with_host_bounds(&[
        ("max_action_inputs", "2"),
        ("max_input_name_bytes", "4"),
        ("max_total_input_name_bytes", "8"),
    ]);
    let err = compile_err(exact_bounds_src, "quoted", &exact_input_cfg);
    assert_eq!(err.stage, "select");
    assert_ne!(err.stage, "host-guard");
    assert!(err.actual_error.get("MissingInput").is_none());
    assert_eq!(
        err.actual_error,
        json!({
            "unsupported": "parameterized-action",
            "action": "quoted",
            "action_index": "1",
            "declared_inputs": [
                {"name": "aaaa", "ty": "UInt"},
                {"name": "bbbb", "ty": "Bool"}
            ]
        })
    );
}

#[test]
fn ordinary_parameter_free_compile_yields_actual_admission_and_occurrence_bounds() {
    for (source, agreement, action, expected_occurrences) in [
        (QUOTE, "FixedQuote", "quote", 9u128),
        (ELIGIBILITY, "FixedEligibility", "inspect", 22u128),
    ] {
        let too_low_cfg = with_host_bounds(&[(
            "max_projection_occurrences",
            &(expected_occurrences - 1).to_string(),
        )]);
        let occ_err = compile_err(source, action, &too_low_cfg);
        assert_exceeded(&occ_err, "ProjectionOccurrences", expected_occurrences, expected_occurrences - 1);

        let exact_occ_cfg =
            with_host_bounds(&[("max_projection_occurrences", &expected_occurrences.to_string())]);
        let artifact = compile_ok(source, action, &exact_occ_cfg);
        assert_eq!(artifact.format, "sherlock-host-artifact/1");
        assert_eq!(artifact.execution_profile, "sherlock-helper-microstep/1");
        assert_eq!(artifact.source.agreement, agreement);
        assert_eq!(artifact.source.action, action);
        assert_eq!(artifact.source.action_index, "0");
        assert_eq!(artifact.input_policy.kind, "parameter-free");
        assert!(artifact.input_policy.declared.is_empty());
        assert!(artifact.input_policy.supplied.is_empty());
        assert!(matches!(
            artifact.observation,
            Observation::NotRequested { evaluation_performed: false }
        ));
        assert_eq!(artifact.evidence.source_preparation, "prepared-and-admitted");
        assert_eq!(artifact.evidence.host_observation, "unrun");
        assert_eq!(artifact.evidence.native, "unrun");
        assert_eq!(artifact.evidence.target, "unrun");
        assert_eq!(artifact.evidence.acceptance, "none");
        assert_eq!(artifact.admission["status"], json!("admitted"));
        assert_eq!(artifact.admission["representation"], json!("HostOnly"));
        assert!(artifact.admission["identity"].is_object());
        assert!(artifact.admission["bounds"].is_object());
        assert!(artifact.admission["program_view"].is_object());
        assert!(artifact.admission["layout"].is_object());
    }
}
