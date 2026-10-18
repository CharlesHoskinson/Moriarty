//! Native circuit-emission and nonproving SDK `ProofPreimage::check` consumer tests
//! separated from the SDK-free host normal test module. Exercises the real A102
//! witness producer on positive and semantically corrupted transitions without
//! invoking `ir.k()`, SRS loading, keygen, proving, or native verification.

use crate::proving::hex;
use crate::sherlock_machine::literal_bridge::{
    check_literal_statement, check_literal_transition, local_max_statement_lanes, CircuitMode,
     ImageValues, LiteralConstraintCheck, LiteralLayout, LiteralSourceIdentity,
    BINDING_VERSION, OPEN_OBLIGATIONS, PROFILE, RULE_CHARGE, RULE_START_LINE, RULE_TAG, SCOPE,
    STATEMENT_FORMAT,
};
use crate::sherlock_machine::literal_host::LiteralLane;
use crate::sherlock_machine::literal_step::{LiteralCapacities, LiteralTable};
use crate::sherlock_machine::literal_witness::{
    build_checked_literal_transition_request, core_program_capacities, ProducedLiteralTransition,
};
use crate::{LEDGER_REVISION, ZKIR_VERSION};
use mori_core::sherlock::layout_a::prepare_a;
use mori_core::sherlock::{
    Arm, BinaryOp, Expr, Function, Parameter, Pattern, Program, Tree, Type, UnaryOp, Value,
    K_SOURCE_SHA256,
};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

pub const A102_CHECK_OUTPUT_DIR_ENV: &str = "MORIARTY_A102_CHECK_OUTPUT_DIR";
pub const A102_CHECK_OUTPUT_SCHEMA: &str = "moriarty-sherlock-a102-nonproving-check-output/1";
pub const A102_ZKIR_FILENAME: &str = "a102_local_public.zkir.json";
pub const A102_PROGRAM_BYTES_FILENAME: &str = "a102_program_l4.bin";
pub const A102_STATEMENT_OUTPUT_FILENAME: &str = "a102_statement_and_transition.json";

const REFERENCE_SOURCE_BYTES: &[u8] =
    include_bytes!("../../../mori-core/src/sherlock/reference.rs");
const EXPECTED_REFERENCE_SOURCE_BYTES_LEN: usize = 16_840;
const EXPECTED_REFERENCE_SOURCE_SHA256: &str =
    "cc68879db15d23d336736e39112521be52f3abfe948d5df4e4d3d97ae9ec7f81";
const EXPECTED_STATEMENT_LANES: usize = 2077;
const EXPECTED_PROGRAM_LANES: usize = 707;
const EXPECTED_STATE_LANES: usize = 683;
const EXPECTED_ENVELOPE_LANES: usize = 2;
const EXPECTED_IR_OUTPUTS: usize = 685;

fn fixture_program_and_capacities() -> (Arc<Program>, LiteralCapacities) {
    let fn0 = Function {
        name: "helper_list".to_string(),
        parameters: vec![
            Parameter {
                name: "xs".to_string(),
                ty: Type::List,
            },
            Parameter {
                name: "flag".to_string(),
                ty: Type::Bool,
            },
        ],
        return_type: Type::UInt,
        body: Arc::new(Expr::Match(
            Arc::new(Expr::Var("xs".to_string())),
            vec![
                Arm {
                    pattern: Pattern::Nil,
                    body: Arc::new(Expr::If(
                        Arc::new(Expr::Unary(
                            UnaryOp::Not,
                            Arc::new(Expr::Var("flag".to_string())),
                        )),
                        Arc::new(Expr::Literal(Value::UInt(100))),
                        Arc::new(Expr::Literal(Value::UInt(200))),
                    )),
                },
                Arm {
                    pattern: Pattern::Cons("h".to_string(), "t".to_string()),
                    body: Arc::new(Expr::Let(
                        "sum".to_string(),
                        Arc::new(Expr::Binary(
                            BinaryOp::Add,
                            Arc::new(Expr::Var("h".to_string())),
                            Arc::new(Expr::Literal(Value::UInt(1))),
                        )),
                        Arc::new(Expr::Var("sum".to_string())),
                    )),
                },
            ],
        )),
    };
    let fn1 = Function {
        name: "helper_tree".to_string(),
        parameters: vec![Parameter {
            name: "tr".to_string(),
            ty: Type::Tree,
        }],
        return_type: Type::Tree,
        body: Arc::new(Expr::Call(
            "helper_tree".to_string(),
            vec![Arc::new(Expr::Literal(Value::Tree(Tree::Leaf(55))))],
        )),
    };

    let program = Arc::new(Program {
        functions: vec![fn0, fn1],
        entry: Arc::new(Expr::Literal(Value::List(vec![9]))),
    });

    let caps = LiteralCapacities {
        // N, NB, O, OP, X, XC, F, P, R, A, T, Q, QF, K, KF
        program: [10, 64, 16, 32, 16, 16, 3, 4, 4, 4, 5, 5, 6, 10, 6],
        // D, DF, E, V, S, C capacity envelope preserved; the produced A102 transition
        // steps Machine::new's initial empty environment/frames on entry List([9]).
        state: [18, 9, 8, 4, 4, 3],
        stack_limit: 32,
    };

    (program, caps)
}

fn build_positive_fixture() -> (ProducedLiteralTransition, LiteralConstraintCheck, Vec<u8>) {
    assert_eq!(
        REFERENCE_SOURCE_BYTES.len(),
        EXPECTED_REFERENCE_SOURCE_BYTES_LEN
    );
    let reference_source_sha256 = hex(&Sha256::digest(REFERENCE_SOURCE_BYTES));
    assert_eq!(reference_source_sha256, EXPECTED_REFERENCE_SOURCE_SHA256);

    let (program, caps) = fixture_program_and_capacities();
    let prog_caps = core_program_capacities(&caps).expect("program capacities conversion");
    let preparatory_prepared =
        prepare_a(program.clone(), prog_caps).expect("preparatory prepare_a for program_bytes");
    let preparatory_program_bytes = preparatory_prepared
        .program_bytes()
        .expect("deterministic PreparedA::program_bytes encoding");
    assert!(!preparatory_program_bytes.is_empty());
    let program_source_sha256 = hex(&Sha256::digest(&preparatory_program_bytes));

    let id = LiteralSourceIdentity {
        k_source_sha256: K_SOURCE_SHA256.to_string(),
        reference_source_sha256,
        program_source_sha256,
    };

    // Produce once (single Machine::step execution) and verify preparatory bytes match.
    let (produced, checked_statement) = build_checked_literal_transition_request(
        program,
        &caps,
        50,
        12,
        &id,
        local_max_statement_lanes(),
    )
    .expect("checked request within local_max_statement_lanes");

    let produced_program_bytes = produced
        .prepared
        .program_bytes()
        .expect("produced PreparedA::program_bytes encoding");
    assert_eq!(preparatory_program_bytes, produced_program_bytes);
    assert_eq!(preparatory_prepared.block, produced.prepared.block);
    assert_eq!(preparatory_prepared.capacities, produced.prepared.capacities);

    let checked = check_literal_transition(&produced.request)
        .expect("produced A102 transition must pass SDK ProofPreimage::check");
    assert_eq!(checked.statement, checked_statement);

    (produced, checked, produced_program_bytes)
}

fn column_index(lanes: &[LiteralLane], name: &str) -> usize {
    lanes
        .iter()
        .position(|lane| lane.name == name)
        .unwrap_or_else(|| panic!("missing schema column {name}"))
}

fn image_lane_total(root: &[LiteralLane], tables: &[LiteralTable]) -> usize {
    root.len()
        + tables
            .iter()
            .map(|t| t.capacity * t.lanes.len())
            .sum::<usize>()
}

fn image_to_decimal_json(image: &ImageValues) -> serde_json::Value {
    serde_json::json!({
        "root": image.root.iter().map(u128::to_string).collect::<Vec<_>>(),
        "tables": image.tables.iter().map(|table| {
            table.iter().map(|row| {
                row.iter().map(u128::to_string).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>(),
    })
}

fn write_create_new_file(path: &Path, bytes: &[u8]) {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap_or_else(|e| panic!("failed to create_new {}: {e}", path.display()));
    file.write_all(bytes)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", path.display()));
    file.flush()
        .unwrap_or_else(|e| panic!("failed to flush {}: {e}", path.display()));
}

fn build_output_artifact_json(
    produced: &ProducedLiteralTransition,
    checked: &LiteralConstraintCheck,
    produced_program_bytes: &[u8],
) -> serde_json::Value {
    let s = &checked.layout.schema;
    let program_lanes = image_lane_total(&s.program_header, &s.program_tables);
    let pre_lanes = image_lane_total(&s.state_root, &s.state_tables);
    let post_lanes = image_lane_total(&s.state_root, &s.state_tables);
    assert_eq!(program_lanes, EXPECTED_PROGRAM_LANES);
    assert_eq!(pre_lanes, EXPECTED_STATE_LANES);
    assert_eq!(post_lanes, EXPECTED_STATE_LANES);
    assert_eq!(
        2 + program_lanes + pre_lanes + post_lanes + EXPECTED_ENVELOPE_LANES,
        EXPECTED_STATEMENT_LANES
    );

    let limbs = checked.layout.limbs();
    let lanes_json: Vec<serde_json::Value> = checked
        .layout
        .lanes
        .iter()
        .zip(&checked.statement)
        .enumerate()
        .map(|(index, ((path, lane), value))| {
            serde_json::json!({
                "index": index,
                "path": path,
                "name": lane.name,
                "bits": lane.bits,
                "boolean": lane.boolean,
                "maximum_dec": lane.maximum.map(|m| m.to_string()),
                "value_dec": value.to_string(),
            })
        })
        .collect();

    serde_json::json!({
        "schema": A102_CHECK_OUTPUT_SCHEMA,
        "qualification": "ordinary-nonproving-proofpreimage-check-unqualified",
        "format": STATEMENT_FORMAT,
        "binding": BINDING_VERSION,
        "profile": PROFILE,
        "scope": SCOPE,
        "ledger_revision": LEDGER_REVISION,
        "zkir_version": ZKIR_VERSION,
        "rule_start_line": RULE_START_LINE,
        "rule_tag": RULE_TAG,
        "charge": RULE_CHARGE,
        "identity": checked.layout.identity,
        "capacities": checked.layout.record(),
        "fixture": {
            "initial_fuel_dec": "50",
            "snapshot_stack_limit": 12,
            "selected_expr_id": produced.selected_expr_id,
            "selected_constant_ref": {
                "space": produced.selected_constant_ref.space,
                "index": produced.selected_constant_ref.index,
            },
            "prepared_entry_expr_id": produced.prepared.block.header.entry_expr_id,
            "prepared_active_lengths": produced.prepared.block.header.active_lengths,
            "prepared_names": produced.prepared.names,
        },
        "byte_counts": {
            "reference_source_bytes": REFERENCE_SOURCE_BYTES.len(),
            "program_l4_bytes": produced_program_bytes.len(),
            "zkir_bytes": checked.zkir.len(),
        },
        "digests": {
            "k_source_sha256": checked.layout.identity.k_source_sha256,
            "reference_source_sha256": checked.layout.identity.reference_source_sha256,
            "program_source_sha256": checked.layout.identity.program_source_sha256,
            "layout_hash": hex(&checked.layout.layout_hash),
            "relation_hash": hex(&checked.layout.relation_hash),
            "context_hash": checked.layout.context_hex(),
            "zkir_sha256": checked.zkir_hash,
        },
        "counts": {
            "statement_lanes": checked.statement.len(),
            "context_lanes": 2,
            "program_lanes": program_lanes,
            "pre_state_lanes": pre_lanes,
            "post_state_lanes": post_lanes,
            "envelope_lanes": EXPECTED_ENVELOPE_LANES,
            "ir_inputs": checked.ir.inputs.len(),
            "ir_public_input_instructions": checked.public_input_count,
            "ir_impact_instructions": checked.impact_count,
            "ir_outputs": checked.output_count,
            "ir_total_instructions": checked.ir.instructions.len(),
            "preimage_raw_inputs": 0,
            "preimage_private_transcript": 0,
            "preimage_public_transcript_inputs": checked.statement.len(),
            "preimage_public_transcript_outputs": checked.statement.len(),
        },
        "check_outcome": {
            "passed": true,
            "binding_input_dec": "0",
            "binding_input_qualification": "zero-fr-ordinary-check-fixture-not-native-claim",
            "do_communications_commitment": checked.ir.do_communications_commitment,
            "pi_skips_len": checked.pi_skips.len(),
            "active_unskipped_impacts": checked.pi_skips.iter().filter(|s| s.is_none()).count(),
            "skipped_impacts": checked.pi_skips.iter().filter(|s| s.is_some()).count(),
        },
        "context_limbs_dec": [limbs[0].to_string(), limbs[1].to_string()],
        "program_l4_bytes_hex": hex(produced_program_bytes),
        "program": image_to_decimal_json(&produced.request.values.program),
        "pre": image_to_decimal_json(&produced.request.values.pre),
        "post": image_to_decimal_json(&produced.request.values.post),
        "envelope": {
            "rule_tag_dec": produced.request.values.rule_tag.to_string(),
            "charge_dec": produced.request.values.charge.to_string(),
        },
        "public_values_dec": checked.statement.iter().map(u128::to_string).collect::<Vec<_>>(),
        "lanes": lanes_json,
        "open_obligations": OPEN_OBLIGATIONS,
    })
}

fn assert_corrupted_transitions_rejected(
    produced: &ProducedLiteralTransition,
    checked: &LiteralConstraintCheck,
) {
    // 1. Corrupt post remainingFuel.0 from 49 to 48 (updating both transcripts coherently).
    let fuel0_col = column_index(&checked.layout.schema.state_root, "remainingFuel.0");
    assert_eq!(produced.request.values.pre.root[fuel0_col], 50);
    assert_eq!(produced.request.values.post.root[fuel0_col], 49);
    let mut bad_fuel_req = produced.request.clone();
    bad_fuel_req.values.post.root[fuel0_col] = 48;
    let bad_fuel_statement = checked
        .layout
        .statement(&bad_fuel_req.values)
        .expect("host statement admissibility must accept in-range post remainingFuel.0 = 48");
    assert_ne!(bad_fuel_statement, checked.statement);
    assert!(
        check_literal_statement(&checked.ir, &bad_fuel_statement).is_err(),
        "SDK ProofPreimage::check must reject wrong post remainingFuel.0 decrement"
    );
    assert!(
        check_literal_transition(&bad_fuel_req).is_err(),
        "check_literal_transition must reject wrong post remainingFuel.0 decrement"
    );

    // 2. Corrupt post Control[0].valueRef.index to a different valid constant index.
    let control_lanes = &checked.layout.schema.state_tables[5].lanes;
    let control_tag_col = column_index(control_lanes, "tag");
    let control_val_idx_col = column_index(control_lanes, "valueRef.index");
    let active_constants_col =
        column_index(&checked.layout.schema.program_header, "activeConstants");
    let active_constants = produced.request.values.program.root[active_constants_col];
    assert!(active_constants >= 2);
    assert_eq!(produced.request.values.post.tables[5][0][control_tag_col], 1);
    let expected_const_idx = produced.selected_constant_ref.index as u128;
    assert_eq!(
        produced.request.values.post.tables[5][0][control_val_idx_col],
        expected_const_idx
    );
    let wrong_const_idx = if expected_const_idx == 0 { 1 } else { 0 };
    assert!(wrong_const_idx < active_constants);
    let mut bad_return_req = produced.request.clone();
    bad_return_req.values.post.tables[5][0][control_val_idx_col] = wrong_const_idx;
    let bad_return_statement = checked
        .layout
        .statement(&bad_return_req.values)
        .expect("host statement admissibility must accept in-range Return constant index");
    assert_ne!(bad_return_statement, checked.statement);
    assert!(
        check_literal_statement(&checked.ir, &bad_return_statement).is_err(),
        "SDK ProofPreimage::check must reject Return control pointing to wrong constant"
    );
    assert!(
        check_literal_transition(&bad_return_req).is_err(),
        "check_literal_transition must reject Return control pointing to wrong constant"
    );

    // 3. Corrupt preserved post.root.stackLimit from 12 to 11.
    let stack_limit_col = column_index(&checked.layout.schema.state_root, "stackLimit");
    assert_eq!(produced.request.values.pre.root[stack_limit_col], 12);
    assert_eq!(produced.request.values.post.root[stack_limit_col], 12);
    let mut bad_root_req = produced.request.clone();
    bad_root_req.values.post.root[stack_limit_col] = 11;
    let bad_root_statement = checked
        .layout
        .statement(&bad_root_req.values)
        .expect("host statement admissibility must accept in-range post stackLimit = 11");
    assert_ne!(bad_root_statement, checked.statement);
    assert!(
        check_literal_statement(&checked.ir, &bad_root_statement).is_err(),
        "SDK ProofPreimage::check must reject unpreserved post.root.stackLimit"
    );
    assert!(
        check_literal_transition(&bad_root_req).is_err(),
        "check_literal_transition must reject unpreserved post.root.stackLimit"
    );

    // 4. Corrupt context limb 0 directly on the statement with coherent transcripts.
    let mut bad_context_statement = checked.statement.clone();
    bad_context_statement[0] ^= 1;
    assert!(
        check_literal_statement(&checked.ir, &bad_context_statement).is_err(),
        "SDK ProofPreimage::check must reject corrupted context limb 0"
    );
}

#[test]
fn produced_a102_request_emits_local_public_zkir_within_local_verifier_lane_bound() {
    let (produced, checked, produced_program_bytes) = build_positive_fixture();

    let layout = LiteralLayout::new(
        &produced.request.capacities,
        &produced.request.identity,
        local_max_statement_lanes(),
    )
    .expect("layout construction");
    let statement = layout
        .statement(&produced.request.values)
        .expect("statement validation");
    assert_eq!(statement, checked.statement);
    assert_eq!(checked.statement.len(), EXPECTED_STATEMENT_LANES);
    assert_eq!(checked.layout.statement_len(), EXPECTED_STATEMENT_LANES);
    assert_eq!(&checked.statement[..2], &checked.layout.limbs()[..]);
    assert_eq!(
        checked.statement[checked.statement.len() - 2],
        RULE_TAG
    );
    assert_eq!(
        checked.statement[checked.statement.len() - 1],
        RULE_CHARGE
    );

    let (emitted_ir, emitted_zkir) = layout
        .emit(CircuitMode::LocalPublic)
        .expect("local circuit emission");
    assert_eq!(emitted_ir, checked.ir);
    assert_eq!(emitted_zkir, checked.zkir);
    assert!(!checked.zkir.is_empty());
    assert_eq!(
        checked.zkir_hash,
        hex(&Sha256::digest(checked.zkir.as_bytes()))
    );

    assert!(!checked.ir.do_communications_commitment);
    assert_eq!(checked.ir.inputs.len(), 0);
    assert_eq!(checked.public_input_count, EXPECTED_STATEMENT_LANES);
    assert_eq!(checked.impact_count, EXPECTED_STATEMENT_LANES);
    assert_eq!(checked.output_count, EXPECTED_IR_OUTPUTS);
    assert_eq!(checked.ir.outputs.len(), EXPECTED_IR_OUTPUTS);
    assert_eq!(checked.pi_skips.len(), EXPECTED_STATEMENT_LANES);
    assert!(checked.pi_skips.iter().all(Option::is_none));

    assert_corrupted_transitions_rejected(&produced, &checked);

    let artifact_json = build_output_artifact_json(&produced, &checked, &produced_program_bytes);
    let serialized_artifact =
        serde_json::to_vec_pretty(&artifact_json).expect("serialize A102 check output artifact");
    assert!(!serialized_artifact.is_empty());

    if let Some(raw_dir) = std::env::var_os(A102_CHECK_OUTPUT_DIR_ENV) {
        let out_dir = Path::new(&raw_dir);
        assert!(
            !out_dir.as_os_str().is_empty(),
            "{A102_CHECK_OUTPUT_DIR_ENV} must not be empty when set"
        );
        fs::create_dir_all(out_dir)
            .unwrap_or_else(|e| panic!("failed to create {}: {e}", out_dir.display()));
        write_create_new_file(&out_dir.join(A102_ZKIR_FILENAME), checked.zkir.as_bytes());
        write_create_new_file(
            &out_dir.join(A102_PROGRAM_BYTES_FILENAME),
            &produced_program_bytes,
        );
        write_create_new_file(
            &out_dir.join(A102_STATEMENT_OUTPUT_FILENAME),
            &serialized_artifact,
        );
    }
}

#[test]
fn semantically_corrupted_a102_transitions_reach_and_fail_sdk_preimage_check() {
    let (produced, checked, _) = build_positive_fixture();
    assert_corrupted_transitions_rejected(&produced, &checked);
}
