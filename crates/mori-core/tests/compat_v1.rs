//! `/1` wire compatibility (clarification §1.4, D1 = none). Requires the
//! `ir::v1` shared hunk from OPUS-V1-WIRE.patch.
use mori_core::{Execution, Program, Scenario, Statement, Step};
use serde_json::{Value, json};

// Baseline (fe01bc8b) serde text: the variant list must not gain `Await`.
const BASELINE_FOO: &str = "unknown variant `Foo`, expected one of `Let`, `Requires`, `Ensures`, `Next`, `Emit`, `Transfer`, `Fee`, `Liability`, `Discharge`";
const BASELINE_AWAIT: &str = "unknown variant `Await`, expected one of `Let`, `Requires`, `Ensures`, `Next`, `Emit`, `Transfer`, `Fee`, `Liability`, `Discharge`";

fn program() -> Value {
    json!({
        "language": "moriarty/1",
        "semantics": "moriarty-core/1",
        "name": "Compat",
        "states": [{"name": "count", "ty": "UInt", "initial": {"type": "UInt", "value": "0"}}],
        "assets": [],
        "expressions": [
            {"kind": "Name", "name": "count"},
            {"kind": "Literal", "value": {"type": "UInt", "value": "1"}},
            {"kind": "Binary", "op": "Add", "lhs": 0, "rhs": 1}
        ],
        "actions": [{"name": "bump", "inputs": [], "statements": [
            {"kind": "Next", "state": "count", "value": 2}
        ]}]
    })
}

fn with_statement(statement: Value) -> Value {
    let mut p = program();
    p["actions"][0]["statements"]
        .as_array_mut()
        .unwrap()
        .insert(0, statement);
    p
}

fn h23_cases() -> [(Value, &'static str); 3] {
    [
        (
            json!({"kind": "Await", "label": "wait", "binders": [], "guard": 0}),
            BASELINE_AWAIT,
        ),
        (json!({"kind": "Await", "guard": "x", "extra": true}), BASELINE_AWAIT),
        (json!({"kind": "Foo", "predicate": 0}), BASELINE_FOO),
    ]
}

#[test]
fn h23_program_json_cannot_represent_await() {
    for (statement, expected) in h23_cases() {
        let source = with_statement(statement);
        let error = serde_json::from_value::<Program>(source.clone()).unwrap_err();
        assert_eq!(error.to_string(), expected);
        // Same bytes through the strict public reader used by `/1` CLI imports.
        let parsed = mori_core::json::parse(&source.to_string()).unwrap();
        let error = serde_json::from_value::<Program>(parsed).unwrap_err();
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn h23_nested_statement_step_and_execution_decode_use_wire() {
    let execution = mori_core::simulate(
        &serde_json::from_value(program()).unwrap(),
        "bump",
        &serde_json::from_value::<Scenario>(json!({
            "state": {"count": {"type": "UInt", "value": "0"}},
            "inputs": {}, "balances": [], "liabilities": []
        }))
        .unwrap(),
    )
    .unwrap();
    let encoded = serde_json::to_value(&execution).unwrap();
    for (statement, expected) in h23_cases() {
        let error = serde_json::from_value::<Statement>(statement.clone()).unwrap_err();
        assert_eq!(error.to_string(), expected);
        let mut step = encoded["steps"][0].clone();
        step["statement"] = statement.clone();
        let error = serde_json::from_value::<Step>(step.clone()).unwrap_err();
        assert_eq!(error.to_string(), expected);
        let mut whole = encoded.clone();
        whole["steps"][0] = step;
        let error = serde_json::from_value::<Execution>(whole).unwrap_err();
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn non_await_serialization_is_unchanged() {
    let source = program();
    let decoded: Program = serde_json::from_value(source.clone()).unwrap();
    assert_eq!(serde_json::to_value(&decoded).unwrap(), source);
    let scenario: Scenario = serde_json::from_value(json!({
        "state": {"count": {"type": "UInt", "value": "0"}},
        "inputs": {}, "balances": [], "liabilities": []
    }))
    .unwrap();
    let execution = mori_core::simulate(&decoded, "bump", &scenario).unwrap();
    let encoded = serde_json::to_value(&execution).unwrap();
    let back: Execution = serde_json::from_value(encoded.clone()).unwrap();
    assert_eq!(back, execution);
    assert_eq!(serde_json::to_value(&back).unwrap(), encoded);
}

#[test]
fn c7_in_memory_core1_await_is_profile_refused() {
    let mut p: Program = serde_json::from_value(program()).unwrap();
    p.actions[0].statements.insert(
        0,
        Statement::Await {
            label: "wait".into(),
            binders: vec![],
            guard: 0,
        },
    );
    // Shared Serialize still renders Await for in-memory `/2` use.
    let rendered = serde_json::to_value(&p.actions[0].statements[0]).unwrap();
    assert_eq!(rendered["kind"], "Await");
    let error = mori_core::validate(&p).unwrap_err();
    assert_eq!(error.code, "mori::core::profile");
    assert_eq!(error.message, "Await requires moriarty-core/2.");
}

// H15 (C6): `await` and `when` stay ordinary `/1` names. The fixtures were
// derived by hand from the fe01bc8b wire definitions before this test existed;
// each is the `json::export` text plus the CLI's final newline.
const H15_SOURCE: &str = r#"language "moriarty/1"; agreement Identifiers { state count: UInt = 0; action run() { let await = 1; let when = await; next count = when; } }"#;
const H15_PROGRAM: &str = include_str!("fixtures/compat_v1_identifiers/program.json");
const H15_EXECUTION: &str = include_str!("fixtures/compat_v1_identifiers/execution.json");

#[test]
fn h15_await_and_when_identifiers_keep_program_and_execution_bytes() {
    let program = mori_core::parse(H15_SOURCE).unwrap();
    assert_eq!(format!("{}\n", mori_core::json::export(&program).unwrap()), H15_PROGRAM);
    assert_eq!(serde_json::from_str::<Program>(H15_PROGRAM).unwrap(), program);
    let scenario: Scenario = serde_json::from_value(json!({
        "state": {}, "inputs": {}, "balances": [], "liabilities": []
    }))
    .unwrap();
    let execution = mori_core::simulate(&program, "run", &scenario).unwrap();
    assert_eq!(format!("{}\n", mori_core::json::export(&execution).unwrap()), H15_EXECUTION);
    assert_eq!(serde_json::from_str::<Execution>(H15_EXECUTION).unwrap(), execution);
}
