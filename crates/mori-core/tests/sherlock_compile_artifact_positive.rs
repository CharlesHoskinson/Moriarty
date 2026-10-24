//! Ordinary positive format/API checks against the production mori-core rlib.
//! Compile this as a separate test harness: never enable cfg(test) on mori-core.
//! Full admitted tables, event/image/work expectations remain independently
//! owned; these checks neither duplicate those tables nor request owner.run.
use mori_core::sherlock::helper_machine::shared::RunIntC;
use mori_core::sherlock_compile_artifact::config::{U128Decimal, UsizeDecimal};
use mori_core::sherlock_compile_artifact::{self as host, CompileArtifact, Observation};
use serde_json::{Value, json};
use std::collections::BTreeSet;

const CONFIG: &str = include_str!("sherlock_compile_artifact_positive/config.json");
const QUOTE: &str = include_str!("sherlock_compile_artifact_positive/fixed-quote.mori");
const ELIGIBILITY: &str = include_str!("sherlock_compile_artifact_positive/fixed-eligibility.mori");
const TARGET: &str = "{\"kind\":\"public-host-test-descriptor\"}\n";

fn config(source: &str) -> host::config::ExecutionConfig {
    host::parse_config(source).unwrap_or_else(|failure| {
        panic!(
            "positive config failed: {}",
            serde_json::to_string(&failure).unwrap()
        )
    })
}
fn compile_only(source: &str, action: &str) -> CompileArtifact {
    host::compile_host(source, action, CONFIG, TARGET, false).unwrap_or_else(|failure| {
        panic!(
            "positive preparation failed: {}",
            serde_json::to_string(&failure).unwrap()
        )
    })
}
fn keys(value: &Value) -> BTreeSet<&str> {
    value
        .as_object()
        .expect("expected output object")
        .keys()
        .map(String::as_str)
        .collect()
}
fn common_contract(artifact: &CompileArtifact) {
    assert_eq!(artifact.format, "sherlock-host-artifact/1");
    assert_eq!(artifact.execution_profile, "sherlock-helper-microstep/1");
    assert_eq!(artifact.input_policy.kind, "parameter-free");
    assert!(artifact.input_policy.declared.is_empty());
    assert!(artifact.input_policy.supplied.is_empty());
    assert!(matches!(
        &artifact.observation,
        Observation::NotRequested {
            evaluation_performed: false
        }
    ));
    assert_eq!(artifact.evidence.host_observation, "unrun");
    assert_eq!(artifact.evidence.native, "unrun");
    assert_eq!(artifact.evidence.target, "unrun");
    assert_eq!(artifact.evidence.acceptance, "none");
    assert_eq!(artifact.plans.witness.status, "not-implemented");
    assert_eq!(artifact.plans.constraint.status, "not-implemented");
    assert_eq!(artifact.plans.export.status, "unsupported");
    assert!(artifact.plans.witness.artifact.is_none());
    assert!(artifact.plans.constraint.artifact.is_none());
    assert!(artifact.plans.export.artifact.is_none());
    let view = &artifact.admission["program_view"];
    // Exact public ProgramView membership, not independently expected table contents.
    assert_eq!(
        keys(view),
        BTreeSet::from([
            "entry",
            "expressions",
            "occurrences",
            "occurrence_paths",
            "children",
            "functions",
            "parameters",
            "arms",
            "patterns",
            "constants",
            "constant_fields",
            "names",
            "pattern_children",
            "types",
            "type_args",
            "constructors",
            "ctor_fields",
            "decl_param_counts",
            "decl_ctor_counts",
            "decl_templates",
            "plan_nodes",
            "plan_cases",
            "field_paths",
        ])
    );
    assert_eq!(
        keys(&artifact.admission["identity"]),
        BTreeSet::from([
            "profile_id",
            "grammar_domain",
            "finance",
            "layout_version",
            "layout_digest",
            "bounds_digest",
            "field_id",
            "program_membership_digest",
        ])
    );
    assert_eq!(
        keys(&artifact.admission["layout"]),
        BTreeSet::from([
            "layout_version",
            "field_id",
            "widths",
            "scalar_layout_id",
            "bounds",
            "capacities",
            "boundary_capacities_present",
        ])
    );
    // Dedicated Core/1 preparation initializes these eleven current tables empty.
    for field in [
        "pattern_children",
        "types",
        "type_args",
        "constructors",
        "ctor_fields",
        "decl_param_counts",
        "decl_ctor_counts",
        "decl_templates",
        "plan_nodes",
        "plan_cases",
        "field_paths",
    ] {
        assert!(
            view[field]
                .as_array()
                .expect("present current ProgramView table")
                .is_empty(),
            "unexpected nonempty {field} in dedicated compiler fixture"
        );
    }
    assert!(view["arms"].as_array().unwrap().is_empty());
    assert!(view["patterns"].as_array().unwrap().is_empty());
    assert_eq!(artifact.admission["status"], "admitted");
    assert_eq!(artifact.admission["representation"], "HostOnly");
    assert!(view["entry"].is_string());
    let entry = view["entry"].as_str().unwrap().parse::<usize>().unwrap();
    assert!(view["expressions"][entry].get("Call").is_some());
    let functions = view["functions"].as_array().unwrap();
    assert_eq!(functions.len(), 1);
    let body = functions[0]["body"]
        .as_str()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    assert!(view["expressions"][body].get("Let").is_some());
    assert!(artifact.preparation.inputs.is_empty());
    assert_eq!(
        artifact.preparation.emit.role,
        "value-projection; ordered emit effect remains in Core"
    );
    let exported = serde_json::to_value(artifact).unwrap();
    assert_eq!(
        exported["observation"],
        json!({"status":"not-requested", "evaluation_performed":false})
    );
    assert_eq!(exported["config"]["format"], "sherlock-host-config/1");
}
fn contains_operator(artifact: &CompileArtifact, operator: &str) -> bool {
    artifact.admission["program_view"]["expressions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|row| row.get("Binary").and_then(|row| row.get("op")) == Some(&json!(operator)))
}

#[test]
fn explicit_config_roundtrip_preserves_all_positive_fields() {
    let original: Value = serde_json::from_str(CONFIG).unwrap();
    let parsed = config(CONFIG);
    assert_eq!(serde_json::to_value(parsed).unwrap(), original);
}

#[test]
fn single_bounds_and_each_host_extent_map_to_the_real_type() {
    let mut value: Value = serde_json::from_str(CONFIG).unwrap();
    // Distinct format-valid numbers detect accidental field swaps. No admission
    // or run uses this mapping-only input; it is not a host policy envelope.
    for (index, (_, field)) in value["bounds"]
        .as_object_mut()
        .unwrap()
        .iter_mut()
        .enumerate()
    {
        *field = json!((index + 1).to_string());
    }
    for (index, (_, field)) in value["host"]
        .as_object_mut()
        .unwrap()
        .iter_mut()
        .enumerate()
    {
        *field = json!((index + 101).to_string());
    }
    for (index, (_, field)) in value["layout"]["capacities"]
        .as_object_mut()
        .unwrap()
        .iter_mut()
        .enumerate()
    {
        *field = json!((index + 201).to_string());
    }
    let parsed = config(&serde_json::to_string(&value).unwrap());
    let owner = parsed.consumer().expect("positive config mapping");
    assert_eq!(owner.bounds, owner.layout.bounds);
    macro_rules! bounds { ($($field:ident),+ $(,)?) => { $(
        assert_eq!(owner.bounds.$field, parsed.bounds.$field.0);
    )+ }; }
    bounds!(
        code_rows,
        function_rows,
        code_child_slots,
        parameter_slots,
        pattern_rows,
        match_plan_rows,
        match_arm_rows,
        type_rows,
        constructor_rows,
        constructor_field_slots,
        max_arity,
        constant_nodes,
        constant_field_slots,
        data_nodes,
        data_field_slots,
        environment_cells,
        frames,
        helper_tasks,
        serialization_bytes,
        max_reference_work,
        max_helper_work,
        step_cap
    );
    macro_rules! host_bounds { ($($field:ident),+ $(,)?) => { $(
        assert_eq!(owner.host.$field, parsed.host.$field.0);
    )+ }; }
    host_bounds!(
        max_source_actions,
        max_source_expressions,
        max_source_statements,
        max_action_inputs,
        max_supplied_inputs,
        max_projection_occurrences,
        max_generated_ast_nodes,
        max_ast_nesting,
        max_input_name_bytes,
        max_total_input_name_bytes
    );
    macro_rules! capacities { ($($field:ident),+ $(,)?) => { $(
        assert_eq!(owner.layout.capacities.$field, parsed.layout.capacities.$field.0);
    )+ }; }
    capacities!(name_rows, name_bytes, occurrence_path_slots, value_slots);
    assert_eq!(
        owner.layout.layout_version,
        (
            parsed.layout.layout_version[0].clone(),
            parsed.layout.layout_version[1].clone()
        )
    );
    assert!(owner.layout.field_id.is_none());
    assert!(owner.layout.widths.is_none());
    assert!(owner.layout.scalar_layout_id.is_none());
}

#[test]
fn canonical_positive_decimal_strings_roundtrip_without_precision_loss() {
    for text in ["0".to_owned(), "1".to_owned(), usize::MAX.to_string()] {
        let value: UsizeDecimal = serde_json::from_value(json!(text)).unwrap();
        assert_eq!(value.0.to_string(), text);
        assert_eq!(serde_json::to_value(value).unwrap(), json!(text));
    }
    for text in ["0".to_owned(), "1".to_owned(), u128::MAX.to_string()] {
        let value: U128Decimal = serde_json::from_value(json!(text)).unwrap();
        assert_eq!(value.0.to_string(), text);
        assert_eq!(serde_json::to_value(value).unwrap(), json!(text));
    }
}

#[test]
fn declared_run_integer_classes_map_without_collapsing_them() {
    for (field, expected) in [
        (json!({"kind":"Nat", "value":"17"}), RunIntC::Nat(17)),
        (json!({"kind":"Negative"}), RunIntC::Negative),
        (json!({"kind":"NatAboveU128"}), RunIntC::NatAboveU128),
    ] {
        let mut value: Value = serde_json::from_str(CONFIG).unwrap();
        value["fuel"] = field.clone();
        value["stack"] = field;
        let parsed = config(&serde_json::to_string(&value).unwrap());
        let owner = parsed.consumer().expect("schema-defined run input class");
        assert_eq!(owner.fuel, expected);
        assert_eq!(owner.stack, expected);
    }
}

#[test]
fn fixed_quote_compile_only_retains_lets_and_arithmetic() {
    let artifact = compile_only(QUOTE, "quote");
    common_contract(&artifact);
    assert_eq!(artifact.source.agreement, "FixedQuote");
    assert_eq!(artifact.preparation.lets.len(), 2);
    assert!(contains_operator(&artifact, "Mul"));
    assert!(contains_operator(&artifact, "Add"));
}

#[test]
fn fixed_eligibility_compile_only_retains_boolean_source_lowering() {
    let artifact = compile_only(ELIGIBILITY, "inspect");
    common_contract(&artifact);
    assert_eq!(artifact.source.agreement, "FixedEligibility");
    assert_eq!(artifact.preparation.lets.len(), 3);
    assert!(contains_operator(&artifact, "Lt"));
    assert!(
        artifact
            .preparation
            .occurrences
            .iter()
            .any(|site| site.node_kind.starts_with("lowering-"))
    );
}

#[test]
fn exact_fixture_bytes_bind_source_and_config_identities() {
    // Computed independently from these fixture bytes with Python hashlib;
    // these are file identities, not expected semantic output or owner tables.
    let artifact = compile_only(QUOTE, "quote");
    assert_eq!(
        artifact.source.source_digest,
        "3b3cf2165602524b4e9585293e3caffa05788f2f2aaeb6c2bfaa9eedd2e2fb4d"
    );
    assert_eq!(
        artifact.config.file_digest,
        "ad1448ba95d1f5c7b201b9d309e0d1a125d97fda2d7fc9b4425597b3470862ad"
    );
}
