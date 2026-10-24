//! SOURCE ONLY / UNCOMPILED / UNRUN. One finite nonproving library unit consumer.
//! Uses existing public source preparation and real A102->A80 producer. No IR emit,
//! SDK check, keygen, prove, verify, network, arbitrary evaluator or golden reader.
//! The backend library still links its existing SDK dependencies when built.
//! Outputs are actual producer observations, NOT independent expected values.
use crate::sherlock_machine::literal_complete_bridge::produce_checked_literal_complete_execution_request;
use crate::sherlock_machine::literal_host::{
    ImageValues, LiteralCapacities, LiteralSourceIdentity,
};
use mori_core::sherlock::{Expr, K_SOURCE_SHA256, Value as ReferenceValue};
use mori_core::sherlock_compile_artifact::draft_reference::compile_draft_with_reference;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    sync::Arc,
};

const SOURCE: &str = include_str!("fixtures/literal_complete_capture_literal-seven.mori");
const CONFIG: &str = include_str!("fixtures/literal_complete_capture_draft-config.json");
const TARGET: &str = include_str!("fixtures/literal_complete_capture_target.json");
const LEGACY_K: &[u8] = include_bytes!("../../../../models/sherlock/k/main.k");
const REFERENCE_SOURCE: &[u8] = include_bytes!("../../../mori-core/src/sherlock/reference.rs");
const HASHED_MODULE_SOURCE: &[u8] = include_bytes!("mod.rs");
const A_INITIAL_FUEL: u128 = 1;
const A_STACK_LIMIT: usize = 0;
const OUTPUT_DIR_ENV: &str = "MORIARTY_LITERAL_COMPLETE_CAPTURE_OUTPUT_DIR";
const SAFE_INPUT_FILE: &str = "literal_complete_k_input.json";
const OBSERVATION_FILE: &str = "literal_complete_actual_images_and_statement.json";

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn sha(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

// Same decimal-string image retention convention as literal_witness_native_tests.
// serde_json's default Number must not narrow the actual UInt128 context/lanes.
fn image_json(image: &ImageValues) -> Value {
    json!({
        "root": image.root.iter().map(u128::to_string).collect::<Vec<_>>(),
        "tables": image.tables.iter().map(|table| table.iter().map(|row|
            row.iter().map(u128::to_string).collect::<Vec<_>>()
        ).collect::<Vec<_>>()).collect::<Vec<_>>()
    })
}

// Data encoding only: write exact actual u128 integers as JSON decimal tokens.
// This separate safe input is accepted by the existing Python renderer and
// contains NO intermediate/Done/envelope/statement from the actual producer.
fn numbers(out: &mut Vec<u8>, values: impl IntoIterator<Item = u128>) {
    out.push(b'[');
    let mut separator = "";
    for value in values {
        write!(out, "{separator}{value}").expect("write integer JSON to Vec");
        separator = ",";
    }
    out.push(b']');
}

fn image_input(out: &mut Vec<u8>, image: &ImageValues) {
    out.extend_from_slice(b"{\"root\":");
    numbers(out, image.root.iter().copied());
    out.extend_from_slice(b",\"tables\":[");
    let mut table_separator = "";
    for table in &image.tables {
        write!(out, "{table_separator}[").expect("write table framing to Vec");
        let mut row_separator = "";
        for row in table {
            out.extend_from_slice(row_separator.as_bytes());
            numbers(out, row.iter().copied());
            row_separator = ",";
        }
        out.push(b']');
        table_separator = ",";
    }
    out.extend_from_slice(b"]}");
}

fn safe_input(
    context: [u128; 2],
    caps: &LiteralCapacities,
    program: &ImageValues,
    init: &ImageValues,
) -> Vec<u8> {
    let mut out =
        b"{\"format\":\"moriarty-k-literal-complete-images-input/1\",\"context\":".to_vec();
    numbers(&mut out, context);
    out.extend_from_slice(b",\"capacities\":{\"program\":");
    numbers(&mut out, caps.program.map(|n| n as u128));
    out.extend_from_slice(b",\"state\":");
    numbers(&mut out, caps.state.map(|n| n as u128));
    write!(out, ",\"stack_limit\":{}}},\"program\":", caps.stack_limit)
        .expect("write capacity to Vec");
    image_input(&mut out, program);
    out.extend_from_slice(b",\"init\":");
    image_input(&mut out, init);
    out.extend_from_slice(b",\"observation_budget\":2}\n");
    out
}

// Same optional explicit-dir + create_new + flush retention convention as the
// existing nonproving A102 output. No default directory or overwrite fallback.
fn write_create_new(path: &Path, bytes: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap_or_else(|error| panic!("create_new {}: {error}", path.display()));
    file.write_all(bytes)
        .unwrap_or_else(|error| panic!("write {}: {error}", path.display()));
    file.flush()
        .unwrap_or_else(|error| panic!("flush {}: {error}", path.display()));
}

#[test]
fn public_source_same_arc_complete_images_capture() {
    // Exact copy of a preexisting public fixture at this new test path; no expected-image import.
    assert_eq!(SOURCE.len(), 72);
    assert_eq!(
        sha(SOURCE.as_bytes()),
        "9dfa037ef19e4b0d5f0d803c96d43e27d02b6493edfc6030947ff1437257f62a"
    );
    assert_eq!(sha(LEGACY_K), K_SOURCE_SHA256);
    let prepared = compile_draft_with_reference(SOURCE, CONFIG, TARGET)
        .unwrap_or_else(|error| panic!(
            "public source preparation refused: stage={} actual_error={} evaluation_performed={} events_forwarded={} completed={}",
            error.stage, error.actual_error, error.evaluation_performed,
            error.events_forwarded, error.completed,
        ));
    let reference = prepared.reference_program();
    assert!(reference.functions.is_empty());
    assert_eq!(*reference.entry, Expr::Literal(ReferenceValue::UInt(7)));
    assert_eq!(
        prepared.preparation().source.source_digest,
        sha(SOURCE.as_bytes())
    );
    let identity = LiteralSourceIdentity {
        k_source_sha256: sha(LEGACY_K),
        reference_source_sha256: sha(REFERENCE_SOURCE),
        program_source_sha256: sha(SOURCE.as_bytes()),
    };
    let caps = LiteralCapacities {
        program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0],
        state: [0, 0, 0, 0, 0, 1],
        stack_limit: A_STACK_LIMIT as u128,
    };
    // One real producer call; its Machine performs A102 and A80 once each.
    // This A fuel1/stack0 profile is separate from the unstepped B preparation.
    let (produced, layout, request, statement) =
        produce_checked_literal_complete_execution_request(
            Arc::clone(reference),
            &caps,
            A_INITIAL_FUEL,
            A_STACK_LIMIT,
            &identity,
            335,
        )
        .unwrap_or_else(|error| panic!("actual complete-pair producer refused: {error}"));
    assert!(Arc::ptr_eq(
        reference,
        &produced.literal_transition.prepared.full_program
    ));
    for snapshot in [
        &produced.literal_transition.step_record.before,
        &produced.literal_transition.step_record.after,
        &produced.step_record.before,
        &produced.step_record.after,
    ] {
        assert!(Arc::ptr_eq(reference, &snapshot.program));
    }
    assert_eq!(request.capacities, caps);
    assert_eq!(request.identity, identity);
    assert_eq!(layout.identity(), &identity);
    assert_eq!(layout.statement_len(), 335);
    assert_eq!(statement.len(), 335);
    let context = layout.context_limbs();
    assert_eq!(&statement[..2], &context);
    // Source/call-plane assertions only, not a second encoded state evaluator.
    assert_eq!(
        (
            produced.literal_transition.step_record.rule_start_line,
            produced.literal_transition.step_record.charge
        ),
        (102, 1)
    );
    assert_eq!(
        (
            produced.step_record.rule_start_line,
            produced.step_record.charge
        ),
        (80, 0)
    );
    assert_eq!(
        request.values.init,
        produced.literal_transition.request.values.pre
    );
    assert_eq!(
        request.values.intermediate,
        produced.literal_transition.request.values.post
    );
    assert_eq!(request.values.intermediate, produced.values.pre);
    assert_eq!(request.values.done, produced.values.post);

    let input = safe_input(
        context,
        &request.capacities,
        &request.values.program,
        &request.values.init,
    );
    let artifact = json!({
        "format": "moriarty-real-source-literal-complete-capture/1",
        "scope": "nonproving actual two-step producer; no K run or independent oracle",
        "source": SOURCE,
        "source_identity": request.identity,
        "config_sha256": sha(CONFIG.as_bytes()),
        "target_sha256": sha(TARGET.as_bytes()),
        "target_descriptor": serde_json::from_str::<Value>(TARGET).expect("target fixture JSON"),
        "counterpart_binding": "separate K source/definition receipts; not this producer context",
        "context_binds_counterpart_source": false,
        "reference_program_debug": format!("{:?}", reference.as_ref()),
        "capacities": layout.capacity_record(),
        "a_initial_fuel_dec": A_INITIAL_FUEL.to_string(),
        "a_stack_limit_dec": A_STACK_LIMIT.to_string(),
        "context_dec": context.map(|n| n.to_string()),
        "layout_hash_hex": hex(&layout.layout_hash()),
        "relation_hash_hex": hex(&layout.relation_hash()),
        "hashed_sherlock_machine_mod_sha256": sha(HASHED_MODULE_SOURCE),
        "program": image_json(&request.values.program),
        "init": image_json(&request.values.init),
        "intermediate": image_json(&request.values.intermediate),
        "done": image_json(&request.values.done),
        "literal_envelope_dec": [request.values.literal_envelope.rule_tag.to_string(),
                                  request.values.literal_envelope.charge.to_string()],
        "return_envelope_dec": [request.values.return_envelope.rule_tag.to_string(),
                                 request.values.return_envelope.charge.to_string()],
        "statement_dec": statement.iter().map(u128::to_string).collect::<Vec<_>>(),
        "lane_coordinates": layout.lanes().iter().enumerate().map(|(index, (path, lane))|
            json!({"index": index, "path": path, "name": lane.name})
        ).collect::<Vec<_>>(),
        "k_input_sha256": sha(&input),
        "same_arc_assertions": "performed for PreparedA and all four snapshots",
        "independent_full_image_expectation": "NOT_ISSUED",
        "native_proof_target_acceptance": "UNOBSERVED"
    });
    let artifact_bytes = serde_json::to_vec_pretty(&artifact).expect("serialize actual capture");
    if let Some(raw_dir) = std::env::var_os(OUTPUT_DIR_ENV) {
        let dir = Path::new(&raw_dir);
        assert!(
            !dir.as_os_str().is_empty(),
            "{OUTPUT_DIR_ENV} must not be empty when set"
        );
        fs::create_dir_all(dir).unwrap_or_else(|error| panic!("create {}: {error}", dir.display()));
        write_create_new(&dir.join(SAFE_INPUT_FILE), &input);
        write_create_new(&dir.join(OBSERVATION_FILE), &artifact_bytes);
    }
}
