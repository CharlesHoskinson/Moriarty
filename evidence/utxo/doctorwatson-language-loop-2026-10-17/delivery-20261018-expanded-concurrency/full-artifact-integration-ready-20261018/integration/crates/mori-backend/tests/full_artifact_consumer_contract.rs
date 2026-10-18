//! Whole Artifact consumer checks; exact hash/ABI/IR goldens remain separate.
use mori_backend::{Artifact, Target, source_projection::compile_reported};
use serde_json::{Value, json};
use std::collections::BTreeSet;

macro_rules! source {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/financial-report-contract/",
            $name
        ))
    };
}
macro_rules! expectation {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/full-artifact-consumer-contract/",
            $name
        ))
    };
}

fn object(value: &Value, fields: &[&str]) {
    let actual: BTreeSet<_> = value.as_object().expect("object").keys().map(String::as_str).collect();
    let expected: BTreeSet<_> = fields.iter().copied().collect();
    assert_eq!(actual, expected, "whole wire object must retain every declared field");
}
fn array(value: &Value) -> &[Value] {
    value.as_array().expect("array")
}
fn text(value: &Value) -> &str {
    value.as_str().expect("string")
}
fn uint(value: &Value) -> u64 {
    let value = value.as_u64().expect("unsigned integer");
    assert!(value <= u64::from(u32::MAX));
    value
}
fn optional_text(value: &Value) {
    if !value.is_null() {
        text(value);
    }
}
fn asset(value: &Value) {
    object(value, &["network", "issuer", "reference", "scale"]);
    for field in ["network", "issuer", "reference"] {
        text(&value[field]);
    }
    assert!(uint(&value["scale"]) <= u64::from(u8::MAX));
}
fn optional_asset(value: &Value) {
    if !value.is_null() {
        asset(value);
    }
}
fn declared_asset(value: &Value, facts: &Value) {
    asset(value);
    assert!(array(&facts["assets"]).iter().any(|item| &item["identity"] == value));
}
fn identity(value: &Value) {
    object(value, &["name", "asset", "debtor", "creditor"]);
    asset(&value["asset"]);
    for field in ["name", "debtor", "creditor"] {
        text(&value[field]);
    }
}
fn same_members(actual: &[Value], expected: &[Value]) {
    assert_eq!(actual.len(), expected.len());
    for item in expected {
        assert_eq!(actual.iter().filter(|value| *value == item).count(), 1);
    }
}
fn duty_identities(facts: &Value) -> Vec<Value> {
    array(&facts["sourceWideUniqueDuties"]).iter().map(|duty| {
        json!({"name": duty["name"], "asset": duty["asset"],
               "debtor": duty["debtor"], "creditor": duty["creditor"]})
    }).collect()
}
fn reference(bindings: &[Value], position: &Value, ty: &str, expected_asset: &Value) {
    let binding = bindings.get(usize::try_from(uint(position)).unwrap()).expect("ABI reference exists");
    assert_eq!(binding["ty"], json!(ty));
    assert_eq!(&binding["asset"], expected_asset);
    assert!(binding["constant"].is_null());
    // Role/name spelling and the choice of position need an independent ABI oracle.
}

fn source_declarations(program: &mori_core::Program, facts: &Value) {
    assert_eq!(program.name, text(&facts["program"]));
    assert_eq!(program.language, text(&facts["languageLiteral"]));
    let assets: Vec<_> = program.assets.iter().map(|item| {
        json!({"alias": item.name, "identity": item.identity})
    }).collect();
    let expected_assets: Vec<_> = array(&facts["assets"]).iter().map(|item| {
        json!({"alias": item["alias"], "identity": item["identity"]})
    }).collect();
    same_members(&assets, &expected_assets);
    let states: Vec<_> = program.states.iter().map(|item| serde_json::to_value(item).unwrap()).collect();
    let expected_states: Vec<_> = array(&facts["states"]).iter().map(|item| {
        let initial = match text(&item["type"]) {
            "UInt" => json!({"type": "UInt", "value": item["initializerLiteral"]}),
            "Bool" => json!({"type": "Bool", "value": match text(&item["initializerLiteral"]) {
                "true" => true,
                "false" => false,
                _ => panic!("fixture contains a Bool literal"),
            }}),
            _ => panic!("fixture contains a supported declared type"),
        };
        json!({"name": item["name"], "ty": item["type"], "initial": initial})
    }).collect();
    same_members(&states, &expected_states);
    assert_eq!(program.actions.len(), array(&facts["actions"]).len());
    for action in &program.actions {
        let expected = array(&facts["actions"]).iter().find(|item| item["name"] == action.name).unwrap();
        let inputs: Vec<_> = action.inputs.iter().map(|item| serde_json::to_value(item).unwrap()).collect();
        let expected_inputs: Vec<_> = array(&expected["inputs"]).iter().map(|item| {
            json!({"name": item["name"], "ty": item["type"]})
        }).collect();
        same_members(&inputs, &expected_inputs);
        assert_eq!(action.statements.len() as u64, uint(&expected["syntacticStatementCount"]));
    }
    // Declaration projections supplement the complete Program equality in observe.
}

fn circuit(value: &Value, facts: &Value, expected_action: &Value) {
    object(value, &["action", "zkir", "context_hash", "public_inputs", "authorization_bindings",
        "private_inputs", "instruction_count", "effect_frame", "balance_frame", "liability_frame", "replay_steps"]);
    assert_eq!(value["action"], expected_action["name"]);
    text(&value["context_hash"]); // No independent digest or context-limb value is asserted.
    let bindings = array(&value["public_inputs"]);
    for binding in bindings {
        object(binding, &["role", "name", "ty", "constant", "asset"]);
        text(&binding["role"]);
        text(&binding["name"]);
        assert!(matches!(text(&binding["ty"]), "UInt" | "Bool"));
        optional_text(&binding["constant"]);
        optional_asset(&binding["asset"]);
        if !binding["asset"].is_null() {
            declared_asset(&binding["asset"], facts);
        }
    }
    // Positive Artifact decoding below is the real consumer of these typed arrays.
    array(&value["authorization_bindings"]);
    for binding in array(&value["private_inputs"]) {
        object(binding, &["name", "ty"]);
        text(&binding["name"]);
        assert!(matches!(text(&binding["ty"]), "Native" | "Point<Jubjub>"));
    }
    let mut balances = Vec::new();
    for frame in array(&value["balance_frame"]) {
        object(frame, &["asset", "account", "pre_position", "post_position"]);
        asset(&frame["asset"]);
        text(&frame["account"]);
        for position in ["pre_position", "post_position"] {
            reference(bindings, &frame[position], "UInt", &frame["asset"]);
        }
        balances.push(json!({"asset": frame["asset"], "account": frame["account"]}));
    }
    let expected_balances = expected_action.get("touchedExactBalanceRows").map(array).unwrap_or(&[]);
    same_members(&balances, expected_balances);
    let duties = duty_identities(facts);
    let mut actual_duties = Vec::new();
    for frame in array(&value["liability_frame"]) {
        object(frame, &["name", "asset", "debtor", "creditor", "pre_presence_position",
            "pre_amount_position", "post_presence_position", "post_amount_position"]);
        let duty = json!({"name": frame["name"], "asset": frame["asset"],
            "debtor": frame["debtor"], "creditor": frame["creditor"]});
        identity(&duty);
        actual_duties.push(duty);
        for (position, ty) in [("pre_presence_position", "Bool"), ("pre_amount_position", "UInt"),
            ("post_presence_position", "Bool"), ("post_amount_position", "UInt")] {
            reference(bindings, &frame[position], ty, &frame["asset"]);
        }
    }
    same_members(&actual_duties, &duties);
    assert_eq!(array(&value["effect_frame"]).len(), array(&expected_action["effectStatementLines"]).len());
    for frame in array(&value["effect_frame"]) {
        object(frame, &["step", "kind", "name", "asset", "from", "to", "public_positions", "liability", "operation"]);
        uint(&frame["step"]);
        text(&frame["kind"]);
        for field in ["name", "from", "to"] {
            optional_text(&frame[field]);
        }
        optional_asset(&frame["asset"]);
        if !frame["asset"].is_null() {
            declared_asset(&frame["asset"], facts);
        }
        if !frame["liability"].is_null() {
            identity(&frame["liability"]);
            assert!(duties.contains(&frame["liability"]));
        }
        if !frame["operation"].is_null() {
            assert!(matches!(text(&frame["operation"]), "create_or_accrue" | "discharge"));
        }
        for position in array(&frame["public_positions"]) {
            assert!(usize::try_from(uint(position)).unwrap() < bindings.len());
        }
        // Exact effect-field encoding/order/step indices are not a frozen golden here.
    }
    assert_eq!(uint(&value["replay_steps"]), uint(&expected_action["syntacticStatementCount"]));

    let ir: Value = serde_json::from_str(text(&value["zkir"])).expect("complete IR JSON");
    object(&ir, &["version", "inputs", "outputs", "do_communications_commitment", "instructions"]);
    object(&ir["version"], &["major", "minor"]);
    assert_eq!(ir["version"], json!({"major": 3, "minor": 0}));
    ir["do_communications_commitment"].as_bool().expect("boolean");
    array(&ir["inputs"]);
    array(&ir["outputs"]);
    assert_eq!(uint(&value["instruction_count"]) as usize, array(&ir["instructions"]).len());
    // Pinned SDK disk->serde version adapter; this is codec consistency, not an IR oracle.
    let mut serde_ir = ir;
    serde_ir["version"] = json!(0);
    let decoded: midnight_zkir_v3::IrSource = serde_json::from_value(serde_ir.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), serde_ir);
}

fn observe(source: &str, facts: &str, frozen_program: &str) {
    let facts: Value = serde_json::from_str(facts).unwrap();
    let expected_target: Value = serde_json::from_str(expectation!("target.expected.json")).unwrap();
    let target = Target::from_json(expectation!("target.expected.json")).unwrap();
    let program = mori_core::parse(source).unwrap();
    let expected_program: Value = serde_json::from_str(frozen_program).unwrap();
    assert_eq!(serde_json::to_value(&program).unwrap(), expected_program,
        "the complete independently frozen Program must match before compiling");
    source_declarations(&program, &facts);
    // Same Core/1 entry point as the public CLI, including its separate metadata wrapper.
    let artifact = compile_reported(&program, &target).unwrap().artifact;
    let serialized = serde_json::to_string_pretty(&artifact).unwrap();
    let wire: Value = serde_json::from_str(&serialized).unwrap();
    let reopened: Artifact = serde_json::from_value(wire.clone()).expect("whole public Artifact decodes");
    assert_eq!(serde_json::to_value(reopened).unwrap(), wire);
    object(&wire, &["format", "abi", "relation", "authorization_mode", "program_hash", "semantics",
        "target", "circuits", "resources", "gates"]);
    assert_eq!(wire["format"], json!("moriarty-zkir-artifact/3"));
    assert_eq!(wire["abi"], json!("moriarty-public-abi/3"));
    assert_eq!(wire["relation"], json!("moriarty-zkir-relation/3"));
    assert_eq!(wire["authorization_mode"], json!("unsigned"));
    text(&wire["program_hash"]); // Exact canonical Program hash remains independently unasserted.
    assert_eq!(wire["semantics"], expected_program["semantics"]); // Frozen independent literal.
    assert_eq!(wire["target"], expected_target);
    object(&wire["target"], &["profile", "network", "anchor", "ledger_revision", "zkir_version",
        "max_instructions", "max_artifact_bytes"]);
    let actual_names: Vec<_> = array(&wire["circuits"]).iter().map(|item| item["action"].clone()).collect();
    let expected_names: Vec<_> = array(&facts["actions"]).iter().map(|item| item["name"].clone()).collect();
    same_members(&actual_names, &expected_names);
    for value in array(&wire["circuits"]) {
        let expected_action = array(&facts["actions"]).iter().find(|item| item["name"] == value["action"]).unwrap();
        circuit(value, &facts, expected_action);
    }
    let resources = &wire["resources"];
    object(resources, &["instructions", "zkir_bytes", "artifact_bytes", "public_values", "max_replay_steps",
        "circuit_k", "fraud_max_k", "cost_measured", "liability_registry_entries", "max_liability_registry_entries"]);
    for field in ["instructions", "zkir_bytes", "artifact_bytes", "public_values", "max_replay_steps",
        "liability_registry_entries", "max_liability_registry_entries"] {
        uint(&resources[field]);
    }
    assert!(resources["circuit_k"].is_null() && resources["fraud_max_k"].is_null());
    assert_eq!(resources["cost_measured"], json!(false));
    assert_eq!(resources["liability_registry_entries"], facts["uniqueDutyCount"]);
    assert_eq!(resources["max_liability_registry_entries"], json!(16));
    let max_steps = array(&facts["actions"]).iter().map(|item| uint(&item["syntacticStatementCount"])).max().unwrap();
    assert_eq!(uint(&resources["max_replay_steps"]), max_steps);
    // Measure the exact complete public CLI serialization, including its own size field.
    assert_eq!(uint(&resources["artifact_bytes"]), serialized.len() as u64);
    for gate in array(&wire["gates"]) {
        object(gate, &["predicate", "status"]);
        text(&gate["predicate"]);
        text(&gate["status"]);
    }
    // Exact program/context hashes and limb values, ABI layout, IR bytes/semantics,
    // gate vocabulary and remaining aggregate resource totals need separate oracles.
}

#[test]
fn invoice_whole_artifact_reopens_and_preserves_independent_source_facts() {
    observe(source!("installment-invoice.mori"), expectation!("installment-invoice.source-facts.json"),
        source!("installment-invoice.program.expected.json"));
}
#[test]
fn reserve_whole_artifact_reopens_and_preserves_five_exact_balance_rows() {
    observe(source!("conditional-reserve.mori"), expectation!("conditional-reserve.source-facts.json"),
        source!("conditional-reserve.program.expected.json"));
}
