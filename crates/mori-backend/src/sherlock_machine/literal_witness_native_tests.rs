//! Native circuit-emission, SDK-check and resource-gated native proof tests, separated
//! from the SDK-free host normal test module. The ordinary (non-ignored) tests, including
//! the Sherlock child carrier cases, exercise the real A102 witness producer on positive
//! and semantically corrupted transitions with at most circuit emission and the
//! nonproving SDK `ProofPreimage::check`; the ordinary child proof-bound case uses no
//! producer or transition and refuses on proof size alone. They never invoke `ir.k()`,
//! SRS loading, keygen, proving or native verification. The `#[ignore]`d P13 literal
//! prove/verify and P14 frozen verify-only cases, and the `#[ignore]`d Sherlock child
//! producer, frozen verify-only and frozen negative selectors, do invoke `ir.k()`, pinned
//! SRS loading, key generation and native verification, and the two producer cases also
//! prove. Each needs its explicit inputs and separate resource admission.

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

use crate::proving::{ProofOptions, MAX_K};
use crate::sherlock_machine::literal_bridge::{
    prove_literal_transition, verify_literal_transition, LiteralProofBundle,
};
use std::path::PathBuf;

pub const A102_PROVE_SRS_DIR_ENV: &str = "MORIARTY_A102_PROVE_SRS_DIR";
pub const A102_PROVE_MAX_K_ENV: &str = "MORIARTY_A102_PROVE_MAX_K";
pub const A102_PROVE_OUTPUT_DIR_ENV: &str = "MORIARTY_A102_PROVE_OUTPUT_DIR";
pub const A102_PROVE_REPORT_SCHEMA: &str = "moriarty-sherlock-a102-prove-verify-report/1";
pub const A102_PROOF_BUNDLE_FILENAME: &str = "a102_literal_proof_bundle.json";
pub const A102_VERIFICATION_REPORT_FILENAME: &str = "a102_literal_verification_report.json";
pub const A102_NATIVE_ERROR_FILENAME: &str = "a102_native_error.json";

struct A102ProveInputs {
    srs_dir: PathBuf,
    max_k: u8,
    output_dir: PathBuf,
}

fn required_env(name: &str) -> String {
    let raw = std::env::var_os(name).unwrap_or_else(|| panic!("{name} must be set explicitly"));
    let text = raw
        .into_string()
        .unwrap_or_else(|_| panic!("{name} must be valid UTF-8"));
    assert!(!text.is_empty(), "{name} must not be empty");
    text
}

/// Reads and validates every explicit native input before any native work.
/// No defaults, downloads, SRS generation or degree guessing.
fn read_a102_prove_inputs() -> A102ProveInputs {
    let srs_dir = PathBuf::from(required_env(A102_PROVE_SRS_DIR_ENV));
    assert!(srs_dir.is_absolute(), "{A102_PROVE_SRS_DIR_ENV} must be absolute");
    let srs_meta = fs::metadata(&srs_dir)
        .unwrap_or_else(|e| panic!("{A102_PROVE_SRS_DIR_ENV} {}: {e}", srs_dir.display()));
    assert!(srs_meta.is_dir(), "{A102_PROVE_SRS_DIR_ENV} must be an existing directory");
    let raw_k = required_env(A102_PROVE_MAX_K_ENV);
    assert!(
        raw_k.len() <= 2 && raw_k.bytes().all(|b| b.is_ascii_digit()),
        "{A102_PROVE_MAX_K_ENV} must be a plain decimal degree"
    );
    let max_k: u8 = raw_k.parse().expect("validated decimal degree");
    assert!(
        (1..=MAX_K).contains(&max_k),
        "{A102_PROVE_MAX_K_ENV} must be within source validation range 1..={MAX_K}"
    );
    let output_dir = PathBuf::from(required_env(A102_PROVE_OUTPUT_DIR_ENV));
    assert!(output_dir.is_absolute(), "{A102_PROVE_OUTPUT_DIR_ENV} must be absolute");
    assert!(
        !output_dir.starts_with(&srs_dir),
        "{A102_PROVE_OUTPUT_DIR_ENV} must not be inside the SRS directory"
    );
    match fs::symlink_metadata(&output_dir) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Ok(_) => panic!("{A102_PROVE_OUTPUT_DIR_ENV} {} already exists", output_dir.display()),
        Err(e) => panic!("{A102_PROVE_OUTPUT_DIR_ENV} {}: {e}", output_dir.display()),
    }
    A102ProveInputs { srs_dir, max_k, output_dir }
}

fn sync_dir(dir: &Path) {
    fs::File::open(dir)
        .and_then(|d| d.sync_all())
        .unwrap_or_else(|e| panic!("failed to sync directory {}: {e}", dir.display()));
}

/// create_new write, file sync_all, then directory sync_all; returns the SHA-256.
fn write_synced_artifact(dir: &Path, name: &str, bytes: &[u8]) -> String {
    let path = dir.join(name);
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .unwrap_or_else(|e| panic!("failed to create_new {}: {e}", path.display()));
    file.write_all(bytes)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", path.display()));
    file.sync_all()
        .unwrap_or_else(|e| panic!("failed to sync {}: {e}", path.display()));
    drop(file);
    sync_dir(dir);
    hex(&Sha256::digest(bytes))
}

fn persist_native_error(dir: &Path, stage: &str, message: &str) {
    let record = serde_json::json!({
        "schema": A102_PROVE_REPORT_SCHEMA,
        "stage": stage,
        "returned_ok": false,
        "error_debug": message,
        "open_obligations": OPEN_OBLIGATIONS,
    });
    let bytes = serde_json::to_vec_pretty(&record).expect("serialize native error record");
    write_synced_artifact(dir, A102_NATIVE_ERROR_FILENAME, &bytes);
}

/// One existing-API prove plus one independently reconstructed verify of the
/// produced Literal(List([9])) transition. Not a soundness, ledger or resource claim.
#[test]
#[ignore = "native SRS/keygen/prove/verify; requires explicit MORIARTY_A102_PROVE_* inputs and separate resource admission"]
fn native_a102_literal_proof_roundtrips_and_independently_verifies() {
    let inputs = read_a102_prove_inputs();

    assert_eq!(REFERENCE_SOURCE_BYTES.len(), EXPECTED_REFERENCE_SOURCE_BYTES_LEN);
    let reference_source_sha256 = hex(&Sha256::digest(REFERENCE_SOURCE_BYTES));
    assert_eq!(reference_source_sha256, EXPECTED_REFERENCE_SOURCE_SHA256);
    let (program, caps) = fixture_program_and_capacities();
    let prog_caps = core_program_capacities(&caps).expect("program capacities conversion");
    let preparatory = prepare_a(program.clone(), prog_caps).expect("preparatory prepare_a");
    let program_bytes = preparatory
        .program_bytes()
        .expect("deterministic PreparedA::program_bytes encoding");
    assert!(!program_bytes.is_empty());
    let id = LiteralSourceIdentity {
        k_source_sha256: K_SOURCE_SHA256.to_string(),
        reference_source_sha256,
        program_source_sha256: hex(&Sha256::digest(&program_bytes)),
    };

    // Single Machine::step production; no SDK check is replayed.
    let (produced, statement) = build_checked_literal_transition_request(
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
    assert_eq!(produced_program_bytes, program_bytes);
    assert_eq!(statement.len(), EXPECTED_STATEMENT_LANES);

    // Host emission only (no ir.k); the bridge re-emits inside prepare.
    let layout = LiteralLayout::new(
        &produced.request.capacities,
        &produced.request.identity,
        local_max_statement_lanes(),
    )
    .expect("layout construction");
    assert_eq!(layout.statement(&produced.request.values).expect("statement"), statement);
    let (_, zkir) = layout.emit(CircuitMode::LocalPublic).expect("local circuit emission");

    let out = inputs.output_dir.as_path();
    fs::create_dir(out).unwrap_or_else(|e| panic!("failed to create {}: {e}", out.display()));
    sync_dir(out.parent().expect("absolute output directory has a parent"));
    let program_sha256 = write_synced_artifact(out, A102_PROGRAM_BYTES_FILENAME, &program_bytes);
    let zkir_sha256 = write_synced_artifact(out, A102_ZKIR_FILENAME, zkir.as_bytes());

    let options = ProofOptions { params_dir: inputs.srs_dir.clone(), max_k: inputs.max_k };
    // Native call 1 of 2. No retry, fallback or cap change on failure.
    let bundle = prove_literal_transition(&produced.request, &options).unwrap_or_else(|e| {
        persist_native_error(out, "prove_literal_transition", &format!("{e:?}"));
        panic!("prove_literal_transition failed: {e:?}")
    });
    let bundle_text = serde_json::to_string_pretty(&bundle).expect("serialize LiteralProofBundle");
    let bundle_sha256 =
        write_synced_artifact(out, A102_PROOF_BUNDLE_FILENAME, bundle_text.as_bytes());
    assert!(!bundle.proof_hex.is_empty() && bundle.proof_hex.len() % 2 == 0);
    let public_values: Vec<String> = statement.iter().map(u128::to_string).collect();
    assert_eq!(bundle.claim.public_values, public_values);
    assert_eq!(bundle.claim.identity, id);
    assert_eq!(bundle.claim.context_hash, layout.context_hex());
    assert_eq!(bundle.claim.zkir_hash, zkir_sha256);
    assert!((1..=inputs.max_k).contains(&bundle.claim.circuit_k));

    let persisted = fs::read_to_string(out.join(A102_PROOF_BUNDLE_FILENAME))
        .expect("read persisted proof bundle");
    assert_eq!(persisted, bundle_text);
    let roundtripped = LiteralProofBundle::from_json(&persisted)
        .unwrap_or_else(|e| panic!("LiteralProofBundle::from_json failed: {e:?}"));
    assert_eq!(roundtripped.claim, bundle.claim);
    assert_eq!(roundtripped.proof_hex, bundle.proof_hex);
    drop(bundle);

    // Native call 2 of 2: independent reconstruction of claim, circuit and VK.
    let verification = verify_literal_transition(&produced.request, &roundtripped, &options)
        .unwrap_or_else(|e| {
            persist_native_error(out, "verify_literal_transition", &format!("{e:?}"));
            panic!("verify_literal_transition failed: {e:?}")
        });
    let claim = &roundtripped.claim;
    assert_eq!(verification.context_hash, claim.context_hash);
    assert_eq!(verification.zkir_hash, claim.zkir_hash);
    assert_eq!(verification.circuit_k, claim.circuit_k);
    assert_eq!(verification.proof_bytes as usize * 2, roundtripped.proof_hex.len());
    assert_eq!(verification.open_obligations, OPEN_OBLIGATIONS.to_vec());

    let report = serde_json::json!({
        "schema": A102_PROVE_REPORT_SCHEMA,
        "verify_returned_ok": true,
        "verification": verification,
        "identity": {
            "format": claim.format,
            "binding": claim.binding,
            "profile": claim.profile,
            "scope": claim.scope,
            "ledger_revision": claim.ledger_revision,
            "zkir_version": claim.zkir_version,
            "source": claim.identity,
            "capacities": claim.capacities,
            "layout_hash": claim.layout_hash,
            "relation_hash": claim.relation_hash,
            "context_hash": claim.context_hash,
            "zkir_hash": claim.zkir_hash,
            "circuit_k": claim.circuit_k,
        },
        "fixture": {
            "entry": "Literal(List([9]))",
            "initial_fuel_dec": "50",
            "snapshot_stack_limit": 12,
            "selected_expr_id": produced.selected_expr_id,
        },
        "inputs": {"srs_dir": inputs.srs_dir.display().to_string(), "max_k": inputs.max_k},
        "artifacts": {
            "program_l4": {"file": A102_PROGRAM_BYTES_FILENAME, "bytes": program_bytes.len(), "sha256": program_sha256},
            "zkir": {"file": A102_ZKIR_FILENAME, "bytes": zkir.len(), "sha256": zkir_sha256},
            "proof_bundle": {"file": A102_PROOF_BUNDLE_FILENAME, "bytes": bundle_text.len(), "sha256": bundle_sha256},
        },
        "public_input_accounting": {
            "claim_public_values_lanes": claim.public_values.len(),
            "native_statement_lanes_with_binding": claim.public_values.len() + 1,
            "binding_lane": "reconstructed privately inside literal_bridge prepare; not returned or persisted",
            "public_values_are_complete_native_public_input_vector": false,
        },
        "verifier_key": "not exported: VK/PK stay private inside literal_bridge; no extra keygen was run",
        "native_calls_by_consumer": {"prove_literal_transition": 1, "verify_literal_transition": 1, "extra_ir_k": 0, "tampered_checks": 0},
        "resource_observation": "UNESTABLISHED",
        "open_obligations": OPEN_OBLIGATIONS,
    });
    let report_bytes = serde_json::to_vec_pretty(&report).expect("serialize verification report");
    write_synced_artifact(out, A102_VERIFICATION_REPORT_FILENAME, &report_bytes);
    println!(
        "A102 prove + independent verify returned Ok: k={} proof_bytes={} out={}",
        claim.circuit_k,
        verification.proof_bytes,
        out.display()
    );
}

use std::io::Read;

pub const A102_VERIFY_BUNDLE_PATH_ENV: &str = "MORIARTY_A102_VERIFY_BUNDLE_PATH";
pub const A102_VERIFY_SRS_DIR_ENV: &str = "MORIARTY_A102_VERIFY_SRS_DIR";
pub const A102_VERIFY_MAX_K_ENV: &str = "MORIARTY_A102_VERIFY_MAX_K";
pub const A102_VERIFY_OUTPUT_DIR_ENV: &str = "MORIARTY_A102_VERIFY_OUTPUT_DIR";
pub const A102_VERIFY_SCHEMA: &str = "moriarty-sherlock-a102-verify-only/1";
pub const A102_VERIFY_INPUT_FILENAME: &str = "a102_verify_only_input.json";
pub const A102_VERIFY_REPORT_FILENAME: &str = "a102_verify_only_report.json";
pub const A102_VERIFY_ERROR_FILENAME: &str = "a102_verify_only_error.json";
/// The existing 1 MiB bounded JSON profile (mori_core::json).
const A102_BUNDLE_READ_CAP: u64 = 1_048_576;
// Frozen prior-run byte identities: input identity only, never semantic goldens.
const A102_FROZEN_BUNDLE_BYTES: usize = 31_390;
const A102_FROZEN_BUNDLE_SHA256: &str =
    "5ec12ca7495db2201e9fafcf4af50a2478794910b33c499412541f1ca728b4bc";
const A102_FROZEN_ZKIR_BYTES: usize = 3_972_464;
const A102_FROZEN_ZKIR_SHA256: &str =
    "0dc0f0f4cc76ee0bad8c419ab06bfd906861e7605024622b7d39b43c92617ead";
const A102_FROZEN_PROGRAM_BYTES: usize = 707;
const A102_FROZEN_PROGRAM_SHA256: &str =
    "4da320d50aa736af5c03bb1ba74810b0e2b93c01e71fa5fb330c43225be1fedc";

struct A102VerifyInputs {
    bundle_path: PathBuf,
    srs_dir: PathBuf,
    max_k: u8,
    output_dir: PathBuf,
}

/// Reads and validates every explicit verify-only input before any native work.
/// No defaults, downloads, SRS generation or degree guessing.
fn read_a102_verify_inputs() -> A102VerifyInputs {
    let bundle_path = PathBuf::from(required_env(A102_VERIFY_BUNDLE_PATH_ENV));
    assert!(bundle_path.is_absolute(), "{A102_VERIFY_BUNDLE_PATH_ENV} must be absolute");
    let bundle_meta = fs::symlink_metadata(&bundle_path)
        .unwrap_or_else(|e| panic!("{A102_VERIFY_BUNDLE_PATH_ENV} {}: {e}", bundle_path.display()));
    assert!(
        bundle_meta.file_type().is_file(),
        "{A102_VERIFY_BUNDLE_PATH_ENV} must be an existing regular file, not a symlink"
    );
    let srs_dir = PathBuf::from(required_env(A102_VERIFY_SRS_DIR_ENV));
    assert!(srs_dir.is_absolute(), "{A102_VERIFY_SRS_DIR_ENV} must be absolute");
    let srs_meta = fs::metadata(&srs_dir)
        .unwrap_or_else(|e| panic!("{A102_VERIFY_SRS_DIR_ENV} {}: {e}", srs_dir.display()));
    assert!(srs_meta.is_dir(), "{A102_VERIFY_SRS_DIR_ENV} must be an existing directory");
    let raw_k = required_env(A102_VERIFY_MAX_K_ENV);
    assert!(
        raw_k.len() <= 2 && raw_k.bytes().all(|b| b.is_ascii_digit()) && !raw_k.starts_with('0'),
        "{A102_VERIFY_MAX_K_ENV} must be a canonical decimal degree"
    );
    let max_k: u8 = raw_k.parse().expect("validated decimal degree");
    assert!(
        (1..=MAX_K).contains(&max_k),
        "{A102_VERIFY_MAX_K_ENV} must be within source validation range 1..={MAX_K}"
    );
    let output_dir = PathBuf::from(required_env(A102_VERIFY_OUTPUT_DIR_ENV));
    assert!(output_dir.is_absolute(), "{A102_VERIFY_OUTPUT_DIR_ENV} must be absolute");
    let bundle_dir = bundle_path.parent().expect("absolute bundle path has a parent");
    assert!(
        !output_dir.starts_with(&srs_dir) && !output_dir.starts_with(bundle_dir),
        "{A102_VERIFY_OUTPUT_DIR_ENV} must be outside the SRS and bundle directories"
    );
    match fs::symlink_metadata(&output_dir) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Ok(_) => panic!("{A102_VERIFY_OUTPUT_DIR_ENV} {} already exists", output_dir.display()),
        Err(e) => panic!("{A102_VERIFY_OUTPUT_DIR_ENV} {}: {e}", output_dir.display()),
    }
    let parent = output_dir.parent().expect("absolute output directory has a parent");
    let parent_meta = fs::metadata(parent)
        .unwrap_or_else(|e| panic!("{A102_VERIFY_OUTPUT_DIR_ENV} parent {}: {e}", parent.display()));
    assert!(
        parent_meta.is_dir() && !parent_meta.permissions().readonly(),
        "{A102_VERIFY_OUTPUT_DIR_ENV} parent must be an existing writable directory"
    );
    A102VerifyInputs { bundle_path, srs_dir, max_k, output_dir }
}

/// Read-only bounded read: opened regular file, at most cap + 1 bytes, then the
/// exact frozen size and SHA-256. Returns (text, sha256, permissions_readonly).
fn read_frozen_a102_bundle(path: &Path) -> (String, String, bool) {
    #[cfg(unix)]
    let opened = {
        use std::os::unix::fs::OpenOptionsExt;
        OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW)
            .open(path)
    };
    #[cfg(not(unix))]
    let opened = fs::File::open(path);
    let file = opened.unwrap_or_else(|e| panic!("failed to open {}: {e}", path.display()));
    let meta = file
        .metadata()
        .unwrap_or_else(|e| panic!("failed to stat {}: {e}", path.display()));
    assert!(meta.is_file(), "frozen bundle must be a regular file");
    assert!(meta.len() <= A102_BUNDLE_READ_CAP, "frozen bundle exceeds the 1 MiB profile");
    let permissions_readonly = meta.permissions().readonly();
    let mut bytes = Vec::new();
    file.take(A102_BUNDLE_READ_CAP + 1)
        .read_to_end(&mut bytes)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    assert!(bytes.len() as u64 <= A102_BUNDLE_READ_CAP, "frozen bundle read overflowed the cap");
    assert_eq!(bytes.len(), A102_FROZEN_BUNDLE_BYTES, "frozen bundle byte length");
    let sha256 = hex(&Sha256::digest(&bytes));
    assert_eq!(sha256, A102_FROZEN_BUNDLE_SHA256, "frozen bundle SHA-256");
    let text = String::from_utf8(bytes).expect("frozen bundle must be UTF-8");
    (text, sha256, permissions_readonly)
}

fn persist_a102_verify_error(dir: &Path, message: &str) {
    let record = serde_json::json!({
        "schema": A102_VERIFY_SCHEMA,
        "record": "verify-only-returned-err",
        "stage": "verify_literal_transition",
        "returned_ok": false,
        "error_debug": message,
        "open_obligations": OPEN_OBLIGATIONS,
    });
    let bytes = serde_json::to_vec_pretty(&record).expect("serialize verify-only error record");
    write_synced_artifact(dir, A102_VERIFY_ERROR_FILENAME, &bytes);
}

/// One existing-API verify of the frozen A102 bundle against an independently
/// reconstructed request. No proving, PK keygen or check replay. Not a soundness,
/// ledger or resource claim.
#[test]
#[ignore = "native SRS/keygen_vk/verify of a frozen A102 bundle; requires explicit MORIARTY_A102_VERIFY_* inputs and separate resource admission"]
fn native_a102_frozen_bundle_verify_only_independently_reconstructs_and_verifies() {
    let inputs = read_a102_verify_inputs();
    let (bundle_text, bundle_sha256, permissions_readonly) =
        read_frozen_a102_bundle(&inputs.bundle_path);
    let bundle = LiteralProofBundle::from_json(&bundle_text)
        .unwrap_or_else(|e| panic!("LiteralProofBundle::from_json failed: {e:?}"));

    // Request reconstruction never reads the bundle claim.
    assert_eq!(REFERENCE_SOURCE_BYTES.len(), EXPECTED_REFERENCE_SOURCE_BYTES_LEN);
    let reference_source_sha256 = hex(&Sha256::digest(REFERENCE_SOURCE_BYTES));
    assert_eq!(reference_source_sha256, EXPECTED_REFERENCE_SOURCE_SHA256);
    let (program, caps) = fixture_program_and_capacities();
    let prog_caps = core_program_capacities(&caps).expect("program capacities conversion");
    let preparatory = prepare_a(program.clone(), prog_caps).expect("preparatory prepare_a");
    let program_bytes = preparatory
        .program_bytes()
        .expect("deterministic PreparedA::program_bytes encoding");
    assert_eq!(program_bytes.len(), A102_FROZEN_PROGRAM_BYTES);
    let program_sha256 = hex(&Sha256::digest(&program_bytes));
    assert_eq!(program_sha256, A102_FROZEN_PROGRAM_SHA256);
    let id = LiteralSourceIdentity {
        k_source_sha256: K_SOURCE_SHA256.to_string(),
        reference_source_sha256,
        program_source_sha256: program_sha256.clone(),
    };
    let (produced, statement) = build_checked_literal_transition_request(
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
    assert_eq!(produced_program_bytes, program_bytes);
    assert_eq!(statement.len(), EXPECTED_STATEMENT_LANES);
    let layout = LiteralLayout::new(
        &produced.request.capacities,
        &produced.request.identity,
        local_max_statement_lanes(),
    )
    .expect("layout construction");
    assert_eq!(layout.statement(&produced.request.values).expect("statement"), statement);
    // Host emission only (no ir.k); the bridge re-emits inside prepare.
    let (_, zkir) = layout.emit(CircuitMode::LocalPublic).expect("local circuit emission");
    let zkir_sha256 = hex(&Sha256::digest(zkir.as_bytes()));
    assert_eq!(zkir.len(), A102_FROZEN_ZKIR_BYTES);
    assert_eq!(zkir_sha256, A102_FROZEN_ZKIR_SHA256);

    // Refusal-only host joins before native work; verify's exact claim match stays authority.
    let claim = &bundle.claim;
    let public_values: Vec<String> = statement.iter().map(u128::to_string).collect();
    assert_eq!(claim.public_values, public_values);
    assert_eq!(claim.identity, id);
    assert_eq!(claim.capacities, layout.record());
    assert_eq!(claim.layout_hash, hex(&layout.layout_hash));
    assert_eq!(claim.relation_hash, hex(&layout.relation_hash));
    assert_eq!(claim.context_hash, layout.context_hex());
    assert_eq!(claim.zkir_hash, zkir_sha256);
    assert!((1..=inputs.max_k).contains(&claim.circuit_k), "claim circuit_k exceeds the explicit cap");
    let proof_hex = &bundle.proof_hex;
    assert!(
        !proof_hex.is_empty()
            && proof_hex.len() % 2 == 0
            && proof_hex.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')),
        "proof hex must be nonempty even-length lowercase"
    );

    let reconstructed = serde_json::json!({
        "source_identity": id,
        "capacities": layout.record(),
        "profile": PROFILE,
        "layout_hash": hex(&layout.layout_hash),
        "relation_hash": hex(&layout.relation_hash),
        "context_hash": layout.context_hex(),
        "program_l4": {"bytes": program_bytes.len(), "sha256": program_sha256},
        "zkir": {"bytes": zkir.len(), "sha256": zkir_sha256},
        "statement_lanes": statement.len(),
        "fixture": {
            "entry": "Literal(List([9]))",
            "initial_fuel_dec": "50",
            "snapshot_stack_limit": 12,
            "capacity_stack_limit_dec": caps.stack_limit.to_string(),
            "selected_expr_id": produced.selected_expr_id,
        },
    });
    let bundle_input = serde_json::json!({
        "path": inputs.bundle_path.display().to_string(),
        "bytes": bundle_text.len(),
        "sha256": bundle_sha256,
        "read_cap_bytes": A102_BUNDLE_READ_CAP,
        "permissions_readonly": permissions_readonly,
        "identity_role": "frozen input byte identity, not a semantic golden; opened read-only, never rewritten or copied",
        "claim_circuit_k": claim.circuit_k,
        "proof_hex_chars": proof_hex.len(),
    });
    let inputs_json = serde_json::json!({
        "srs_dir": inputs.srs_dir.display().to_string(),
        "max_k": inputs.max_k,
    });

    let out = inputs.output_dir.as_path();
    fs::create_dir(out).unwrap_or_else(|e| panic!("failed to create {}: {e}", out.display()));
    sync_dir(out.parent().expect("absolute output directory has a parent"));
    let setup = serde_json::json!({
        "schema": A102_VERIFY_SCHEMA,
        "record": "input-identity-before-native-call; not a result",
        "bundle_input": bundle_input,
        "reconstructed": reconstructed,
        "inputs": inputs_json,
        "planned_consumer_calls": {"verify_literal_transition": 1, "prove_literal_transition": 0, "check_literal_transition": 0, "extra_ir_k": 0, "tampered_checks": 0},
        "open_obligations": OPEN_OBLIGATIONS,
    });
    let setup_bytes = serde_json::to_vec_pretty(&setup).expect("serialize verify-only input record");
    let setup_sha256 = write_synced_artifact(out, A102_VERIFY_INPUT_FILENAME, &setup_bytes);

    let options = ProofOptions { params_dir: inputs.srs_dir.clone(), max_k: inputs.max_k };
    // Sole native call. No retry, fallback or cap change on failure.
    let verification = verify_literal_transition(&produced.request, &bundle, &options)
        .unwrap_or_else(|e| {
            persist_a102_verify_error(out, &format!("{e:?}"));
            panic!("verify_literal_transition failed: {e:?}")
        });
    assert_eq!(verification.qualification, "local-native-a102-literal-transition-proof-unqualified");
    assert_eq!(verification.context_hash, layout.context_hex());
    assert_eq!(verification.zkir_hash, zkir_sha256);
    assert_eq!(verification.circuit_k, claim.circuit_k);
    assert!((1..=inputs.max_k).contains(&verification.circuit_k));
    assert_eq!(verification.proof_bytes as usize * 2, proof_hex.len());
    assert_eq!(verification.open_obligations, OPEN_OBLIGATIONS.to_vec());

    let report = serde_json::json!({
        "schema": A102_VERIFY_SCHEMA,
        "record": "verify-only-returned-ok",
        "verify_returned_ok": true,
        "qualification": "independently reconstructed local native verify of a frozen A102 bundle; not full soundness, refinement, ledger or stagenet",
        "verification": verification,
        "profile": PROFILE,
        "circuit_k": verification.circuit_k,
        "bundle_input": bundle_input,
        "reconstructed": reconstructed,
        "input_record": {"file": A102_VERIFY_INPUT_FILENAME, "bytes": setup_bytes.len(), "sha256": setup_sha256},
        "inputs": inputs_json,
        "binding_lane": "reconstructed privately inside literal_bridge prepare; not returned or persisted",
        "verifier_key": "not exported: VK stays private inside verify_literal_transition; no extra keygen",
        "native_calls_by_consumer": {"verify_literal_transition": 1, "prove_literal_transition": 0, "extra_ir_k": 0, "tampered_checks": 0},
        "resource_observation": "UNESTABLISHED",
        "open_obligations": OPEN_OBLIGATIONS,
    });
    let report_bytes = serde_json::to_vec_pretty(&report).expect("serialize verify-only report");
    write_synced_artifact(out, A102_VERIFY_REPORT_FILENAME, &report_bytes);
    println!(
        "A102 frozen-bundle verify-only returned Ok: k={} proof_bytes={} out={}",
        verification.circuit_k,
        verification.proof_bytes,
        out.display()
    );
}

// UNRUN Sherlock A102 Poseidon recursive-child cases, appended after the P14 prefix
// (unchanged except its corrected top doc). They reuse the real fixture, producer, SDK
// check and P14 bounded-read and synced-output helpers above.

use crate::proving::MAX_PROOF_BYTES;
use crate::sherlock_child::{
    check_child_statement, child_witness, field_hex, verify_sherlock_child, ChildClaim, ChildKey,
    SherlockChildProof, CHILD_BINDING_VERSION, CHILD_OPEN_OBLIGATIONS,
};
use crate::sherlock_machine::literal_bridge::{
    CapacityRecord, LiteralTransitionRequest, LiteralValues,
};
use crate::sherlock_machine::literal_host::decode_digest;
use std::path::Component;
use midnight_transient_crypto::curve::Fr;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::ir_types::IrValue;
use midnight_zkir_v3::{Identifier, Instruction};

pub const A102_CHILD_PROVE_SRS_DIR_ENV: &str = "MORIARTY_A102_CHILD_PROVE_SRS_DIR";
pub const A102_CHILD_PROVE_MAX_K_ENV: &str = "MORIARTY_A102_CHILD_PROVE_MAX_K";
pub const A102_CHILD_PROVE_OUTPUT_DIR_ENV: &str = "MORIARTY_A102_CHILD_PROVE_OUTPUT_DIR";
pub const A102_CHILD_VERIFY_INPUT_DIR_ENV: &str = "MORIARTY_A102_CHILD_VERIFY_INPUT_DIR";
pub const A102_CHILD_VERIFY_SRS_DIR_ENV: &str = "MORIARTY_A102_CHILD_VERIFY_SRS_DIR";
pub const A102_CHILD_VERIFY_MAX_K_ENV: &str = "MORIARTY_A102_CHILD_VERIFY_MAX_K";
pub const A102_CHILD_VERIFY_OUTPUT_DIR_ENV: &str = "MORIARTY_A102_CHILD_VERIFY_OUTPUT_DIR";
pub const A102_CHILD_NEGATIVE_INPUT_DIR_ENV: &str = "MORIARTY_A102_CHILD_NEGATIVE_INPUT_DIR";
pub const A102_CHILD_NEGATIVE_SRS_DIR_ENV: &str = "MORIARTY_A102_CHILD_NEGATIVE_SRS_DIR";
pub const A102_CHILD_NEGATIVE_MAX_K_ENV: &str = "MORIARTY_A102_CHILD_NEGATIVE_MAX_K";
pub const A102_CHILD_NEGATIVE_OUTPUT_DIR_ENV: &str = "MORIARTY_A102_CHILD_NEGATIVE_OUTPUT_DIR";
pub const A102_CHILD_REPORT_SCHEMA: &str = "moriarty-sherlock-a102-poseidon-child-report/1";
pub const A102_CHILD_VK_FILENAME: &str = "child.vk";
pub const A102_CHILD_PROOF_FILENAME: &str = "child.proof";
pub const A102_CHILD_INSTANCE_FILENAME: &str = "child.instance.json";
pub const A102_CHILD_CLAIM_FILENAME: &str = "child.claim.json";
pub const A102_CHILD_INPUT_FILENAME: &str = "child.input.json";
pub const A102_CHILD_REPORT_FILENAME: &str = "child.report.json";
pub const A102_CHILD_ERROR_FILENAME: &str = "child.error.json";
pub const A102_CHILD_PHASE_RECONSTRUCTED_FILENAME: &str = "child.phase.1-reconstructed.json";
pub const A102_CHILD_PHASE_JOINED_FILENAME: &str = "child.phase.2-joined.json";
pub const A102_CHILD_PHASE_CONTROL_FILENAME: &str = "child.phase.3-control-returned.json";
/// Fenrir `read_relation` caps processed key bytes at 1 MiB; JSON uses the P14 cap.
const A102_CHILD_VK_READ_CAP: u64 = 1 << 20;

fn child_instance(checked: &LiteralConstraintCheck, binding: Fr) -> Vec<Fr> {
    std::iter::once(binding).chain(checked.statement.iter().map(|v| Fr::from(*v))).collect()
}

#[test]
fn a102_child_carrier_is_exactly_the_checked_local_public_statement() {
    let (_, checked, _) = build_positive_fixture();
    let binding = Fr::from(0x5348_4552_4c4f_434bu64);
    let skips = check_child_statement(&checked.ir, &checked.statement, binding)
        .expect("SDK check under a nonzero binding");
    let instance = child_instance(&checked, binding);
    let w = child_witness(&checked.ir, &instance, skips).expect("pinned carrier");
    assert_eq!(w.memory.len(), EXPECTED_STATEMENT_LANES);
    for (i, v) in checked.statement.iter().enumerate() {
        assert_eq!(
            w.memory.get(&Identifier(format!("%sherlock.a102.statement.{i}"))),
            Some(&IrValue::Native(Fr::from(*v)))
        );
    }
    assert_eq!(w.pis.len(), EXPECTED_STATEMENT_LANES + 1);
    assert_eq!(w.pis[0], binding.0);
    assert_eq!(w.binding_input, binding.0);
    assert!(w.comm_comm.is_none());
    assert_eq!(w.pi_skips, vec![None; EXPECTED_STATEMENT_LANES]);
    assert_eq!(field_hex(&Fr::from(1u64)), format!("01{}", "00".repeat(31)));
}

#[test]
fn a102_child_carrier_refuses_shape_mode_skips_and_unchecked_statements() {
    let (produced, checked, _) = build_positive_fixture();
    let binding = Fr::from(7u64);
    let skips = check_child_statement(&checked.ir, &checked.statement, binding).unwrap();
    let instance = child_instance(&checked, binding);
    assert!(child_witness(&checked.ir, &instance[..instance.len() - 1], skips.clone()).is_err());
    let mut longer = instance.clone();
    longer.push(Fr::from(0u64));
    assert!(child_witness(&checked.ir, &longer, skips.clone()).is_err());
    let mut skipped = skips.clone();
    skipped[0] = Some(1);
    assert!(child_witness(&checked.ir, &instance, skipped).is_err());
    let mut guarded = checked.ir.clone();
    let code = Arc::make_mut(&mut guarded.instructions);
    let at = code.iter().position(|i| matches!(i, Instruction::Impact { .. })).unwrap();
    if let Instruction::Impact { guard, .. } = &mut code[at] {
        *guard = Operand::Immediate(Fr::from(0u64));
    }
    assert!(child_witness(&guarded, &instance, skips.clone()).is_err());
    let (ledger, _) = checked.layout.emit(CircuitMode::LedgerCall).unwrap();
    assert!(child_witness(&ledger, &instance, skips).is_err());
    let fuel0 = column_index(&checked.layout.schema.state_root, "remainingFuel.0");
    let mut bad = produced.request.clone();
    bad.values.post.root[fuel0] = 48;
    let bad_statement = checked.layout.statement(&bad.values).unwrap();
    assert!(check_child_statement(&checked.ir, &bad_statement, binding).is_err());
    assert_ne!(CHILD_BINDING_VERSION, BINDING_VERSION);
}

/// Absolute and free of `.`/`..`; where it exists, also canonical (no symlinked component).
fn assert_canonical_absolute(name: &str, path: &Path, exists: bool) {
    assert!(
        path.is_absolute()
            && path.components().all(|c| matches!(c, Component::RootDir | Component::Normal(_))),
        "{name} must be an absolute path without `.` or `..` components"
    );
    if exists {
        let real = fs::canonicalize(path)
            .unwrap_or_else(|e| panic!("{name} {}: {e}", path.display()));
        assert!(real.as_path() == path, "{name} must be canonical, without symlinked components");
    }
}

struct A102ChildInputs {
    input_dir: Option<PathBuf>,
    srs_dir: PathBuf,
    max_k: u8,
    output_dir: PathBuf,
}

/// Reads and validates every explicit child input before any native work: canonical
/// absolute paths, a canonical decimal degree, an existing writable output parent and an
/// output directory that is exactly NotFound. No defaults, downloads or degree guessing.
fn read_a102_child_inputs(
    input_env: Option<&str>,
    srs_env: &str,
    max_k_env: &str,
    output_env: &str,
) -> A102ChildInputs {
    let input_dir = input_env.map(|env| {
        let dir = PathBuf::from(required_env(env));
        assert_canonical_absolute(env, &dir, true);
        assert!(fs::metadata(&dir).is_ok_and(|m| m.is_dir()), "{env} must be an existing directory");
        dir
    });
    let srs_dir = PathBuf::from(required_env(srs_env));
    assert_canonical_absolute(srs_env, &srs_dir, true);
    assert!(fs::metadata(&srs_dir).is_ok_and(|m| m.is_dir()), "{srs_env} must be an existing directory");
    let raw_k = required_env(max_k_env);
    assert!(
        raw_k.len() <= 2 && raw_k.bytes().all(|b| b.is_ascii_digit()) && !raw_k.starts_with('0'),
        "{max_k_env} must be a canonical decimal degree"
    );
    let max_k: u8 = raw_k.parse().expect("validated decimal degree");
    assert!(
        (1..=MAX_K).contains(&max_k),
        "{max_k_env} must be within source validation range 1..={MAX_K}"
    );
    let output_dir = PathBuf::from(required_env(output_env));
    assert_canonical_absolute(output_env, &output_dir, false);
    let disjoint = |other: &Path| !output_dir.starts_with(other) && !other.starts_with(&output_dir);
    assert!(
        disjoint(srs_dir.as_path()) && input_dir.as_deref().is_none_or(disjoint),
        "{output_env} must be disjoint from the SRS and input directories"
    );
    match fs::symlink_metadata(&output_dir) {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
        Ok(_) => panic!("{output_env} {} already exists", output_dir.display()),
        Err(e) => panic!("{output_env} {}: {e}", output_dir.display()),
    }
    let parent = output_dir.parent().expect("absolute output directory has a parent");
    assert_canonical_absolute(output_env, parent, true);
    let parent_meta = fs::metadata(parent)
        .unwrap_or_else(|e| panic!("{output_env} parent {}: {e}", parent.display()));
    assert!(
        parent_meta.is_dir() && !parent_meta.permissions().readonly(),
        "{output_env} parent must be an existing writable directory"
    );
    A102ChildInputs { input_dir, srs_dir, max_k, output_dir }
}

/// P14 reconstruction for native child selectors: one `Machine::step`, no SDK check
/// replay, and no bundle, claim or key input.
fn reconstruct_a102_child_request() -> (ProducedLiteralTransition, Vec<u128>, String) {
    assert_eq!(REFERENCE_SOURCE_BYTES.len(), EXPECTED_REFERENCE_SOURCE_BYTES_LEN);
    let reference_source_sha256 = hex(&Sha256::digest(REFERENCE_SOURCE_BYTES));
    assert_eq!(reference_source_sha256, EXPECTED_REFERENCE_SOURCE_SHA256);
    let (program, caps) = fixture_program_and_capacities();
    let prog_caps = core_program_capacities(&caps).expect("program capacities conversion");
    let program_bytes = prepare_a(program.clone(), prog_caps)
        .expect("preparatory prepare_a")
        .program_bytes()
        .expect("deterministic PreparedA::program_bytes encoding");
    assert_eq!(program_bytes.len(), A102_FROZEN_PROGRAM_BYTES);
    let program_sha256 = hex(&Sha256::digest(&program_bytes));
    assert_eq!(program_sha256, A102_FROZEN_PROGRAM_SHA256);
    let id = LiteralSourceIdentity {
        k_source_sha256: K_SOURCE_SHA256.to_string(),
        reference_source_sha256,
        program_source_sha256: program_sha256.clone(),
    };
    let (produced, statement) = build_checked_literal_transition_request(
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
    assert_eq!(produced_program_bytes, program_bytes);
    assert_eq!(statement.len(), EXPECTED_STATEMENT_LANES);
    (produced, statement, program_sha256)
}

/// Bounded read-only read mirroring `read_frozen_a102_bundle`: a no-follow regular file of
/// one to `cap` bytes, read through `take(cap + 1)`.
fn read_bounded_child_file(path: &Path, cap: u64) -> Vec<u8> {
    #[cfg(unix)]
    let opened = {
        use std::os::unix::fs::OpenOptionsExt;
        OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW)
            .open(path)
    };
    #[cfg(not(unix))]
    let opened = fs::File::open(path);
    let file = opened.unwrap_or_else(|e| panic!("failed to open {}: {e}", path.display()));
    let meta = file
        .metadata()
        .unwrap_or_else(|e| panic!("failed to stat {}: {e}", path.display()));
    assert!(
        meta.is_file() && meta.len() <= cap,
        "{} must be a regular file of at most {cap} bytes",
        path.display()
    );
    let mut bytes = Vec::new();
    file.take(cap + 1)
        .read_to_end(&mut bytes)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    assert!(
        !bytes.is_empty() && bytes.len() as u64 <= cap,
        "{} must hold one to {cap} bytes",
        path.display()
    );
    bytes
}

/// Bounded frozen producer artifacts. Their bytes are inputs and identities only; the
/// claim, instance and key are rebuilt by the consumer and compared, never trusted.
fn read_frozen_a102_child(dir: &Path) -> (SherlockChildProof, serde_json::Value) {
    let mut files = serde_json::Map::new();
    let mut read = |name: &str, cap: u64| {
        let bytes = read_bounded_child_file(&dir.join(name), cap);
        let identity = serde_json::json!({"bytes": bytes.len(), "sha256": hex(&Sha256::digest(&bytes))});
        files.insert(name.to_string(), identity);
        bytes
    };
    let vk_bytes = read(A102_CHILD_VK_FILENAME, A102_CHILD_VK_READ_CAP);
    let proof = read(A102_CHILD_PROOF_FILENAME, MAX_PROOF_BYTES as u64);
    let instance_text = read(A102_CHILD_INSTANCE_FILENAME, A102_BUNDLE_READ_CAP);
    let claim_text = read(A102_CHILD_CLAIM_FILENAME, A102_BUNDLE_READ_CAP);
    let hexes: Vec<String> = serde_json::from_slice(&instance_text).expect("child instance JSON");
    let instance: Vec<Fr> = hexes
        .iter()
        .map(|h| {
            let bytes = decode_digest(h).expect("32-byte lowercase lane hex");
            let lane = Fr::from_le_bytes(&bytes).expect("canonical field lane");
            assert_eq!(&field_hex(&lane), h, "lane hex must be canonical");
            lane
        })
        .collect();
    let claim: ChildClaim = serde_json::from_slice(&claim_text).expect("child claim JSON");
    (SherlockChildProof { claim, instance, vk_bytes, proof }, serde_json::Value::Object(files))
}

/// Refusal-only host joins of frozen artifacts before native work. The exact claim,
/// instance and key comparison stays with the native consumer.
fn join_frozen_a102_child(
    child: &SherlockChildProof,
    request: &LiteralTransitionRequest,
    statement: &[u128],
    max_k: u8,
) {
    let public_values: Vec<String> = statement.iter().map(u128::to_string).collect();
    assert_eq!(child.claim.public_values, public_values, "frozen claim statement");
    assert_eq!(child.claim.identity, request.identity, "frozen claim source identity");
    assert!((1..=max_k).contains(&child.claim.circuit_k), "frozen claim circuit_k exceeds the explicit cap");
    assert_eq!(child.instance.len(), EXPECTED_STATEMENT_LANES + 1, "frozen instance lanes");
    assert!(
        child.instance[1..].iter().zip(statement).all(|(lane, v)| *lane == Fr::from(*v)),
        "frozen instance statement lanes"
    );
}

fn a102_child_open_obligations() -> Vec<&'static str> {
    OPEN_OBLIGATIONS.iter().chain(CHILD_OPEN_OBLIGATIONS.iter()).copied().collect()
}

/// Fresh output directory, synced parent and synced input record before any native call.
fn begin_a102_child_output(out: &Path, record: &serde_json::Value) -> String {
    fs::create_dir(out).unwrap_or_else(|e| panic!("failed to create {}: {e}", out.display()));
    sync_dir(out.parent().expect("absolute output directory has a parent"));
    let bytes = serde_json::to_vec_pretty(record).expect("serialize child input record");
    write_synced_artifact(out, A102_CHILD_INPUT_FILENAME, &bytes)
}

fn persist_a102_child_error(dir: &Path, selector: &str, stage: &str, message: &str) {
    let record = serde_json::json!({
        "schema": A102_CHILD_REPORT_SCHEMA,
        "selector": selector,
        "record": "native-stage-failed",
        "stage": stage,
        "returned_ok": false,
        "error_debug": message,
        "retry": "none; the failed selection is consumed",
        "open_obligations": a102_child_open_obligations(),
    });
    let bytes = serde_json::to_vec_pretty(&record).expect("serialize child error record");
    write_synced_artifact(dir, A102_CHILD_ERROR_FILENAME, &bytes);
}

/// Synced phase marker at a boundary this test observes directly: one distinct
/// `create_new` file per phase, never rewritten. It records only what the named call
/// returned; it cannot attest internal stages that `ChildKey` does not expose.
fn persist_a102_child_phase(dir: &Path, file: &str, selector: &str, phase: serde_json::Value) -> String {
    let record = serde_json::json!({
        "schema": A102_CHILD_REPORT_SCHEMA,
        "selector": selector,
        "record": "phase-boundary; not a final result",
        "phase": phase,
        "open_obligations": a102_child_open_obligations(),
    });
    let bytes = serde_json::to_vec_pretty(&record).expect("serialize child phase record");
    write_synced_artifact(dir, file, &bytes)
}

/// R1: an empty or oversized proof refuses with the proof code before any layout, k
/// selection, SRS or key work. Request and claim are deliberately unreconstructable, so a
/// regression reaches a different refusal instead of native work.
#[test]
fn a102_child_verify_refuses_unbounded_proof_before_reconstruction() {
    let empty = ImageValues { root: Vec::new(), tables: Vec::new() };
    let identity = LiteralSourceIdentity {
        k_source_sha256: String::new(),
        reference_source_sha256: String::new(),
        program_source_sha256: String::new(),
    };
    let request = LiteralTransitionRequest {
        capacities: LiteralCapacities { program: [0; 15], state: [0; 6], stack_limit: 0 },
        identity: identity.clone(),
        values: LiteralValues {
            program: empty.clone(),
            pre: empty.clone(),
            post: empty,
            rule_tag: 0,
            charge: 0,
        },
    };
    let claim = ChildClaim {
        format: String::new(),
        binding: String::new(),
        transcript: String::new(),
        statement_format: String::new(),
        profile: String::new(),
        scope: String::new(),
        ledger_revision: String::new(),
        zkir_version: String::new(),
        rule_start_line: 0,
        rule_tag: 0,
        charge: 0,
        capacities: CapacityRecord { program: Vec::new(), state: Vec::new(), stack_limit: String::new() },
        identity,
        layout_hash: String::new(),
        relation_hash: String::new(),
        context_hash: String::new(),
        zkir_hash: String::new(),
        adapter_source_sha256: String::new(),
        circuit_k: 0,
        public_values: Vec::new(),
    };
    let options = ProofOptions {
        params_dir: PathBuf::from("/nonexistent/moriarty-a102-child-proof-bound"),
        max_k: MAX_K,
    };
    for proof in [Vec::new(), vec![0u8; MAX_PROOF_BYTES + 1]] {
        let child = SherlockChildProof { claim: claim.clone(), instance: Vec::new(), vk_bytes: Vec::new(), proof };
        let err = verify_sherlock_child(&request, &child, &options).unwrap_err();
        assert_eq!(err.code, "mori::backend::proof");
    }
}

/// Producer only: one key reconstruction and one child proof (SDK check under the real
/// binding, `setup_pk`, prove, producer self-verify). The reconstructed key, instance and
/// claim and a phase record are synced once reconstruction returns and before `prove`;
/// the proof is synced once `prove` returns, before any host assertion, and the key is
/// dropped before the report. No independent
/// reconstruction or negative case runs in this process. Not a soundness, ledger or
/// resource claim.
#[test]
#[ignore = "native SRS/keygen/prove/self-verify of the A102 Poseidon child; requires explicit MORIARTY_A102_CHILD_PROVE_* inputs and separate resource admission"]
fn native_a102_poseidon_child_producer_proves_and_persists() {
    let selector = "native_a102_poseidon_child_producer_proves_and_persists";
    let inputs = read_a102_child_inputs(
        None,
        A102_CHILD_PROVE_SRS_DIR_ENV,
        A102_CHILD_PROVE_MAX_K_ENV,
        A102_CHILD_PROVE_OUTPUT_DIR_ENV,
    );
    let (produced, statement, program_sha256) = reconstruct_a102_child_request();
    let inputs_json = serde_json::json!({
        "srs_dir": inputs.srs_dir.display().to_string(),
        "max_k": inputs.max_k,
    });
    let out = inputs.output_dir.as_path();
    let input_sha256 = begin_a102_child_output(out, &serde_json::json!({
        "schema": A102_CHILD_REPORT_SCHEMA,
        "selector": selector,
        "record": "input-identity-before-native-call; not a result",
        "inputs": inputs_json,
        "reconstructed": {
            "source_identity": produced.request.identity,
            "program_l4_sha256": program_sha256,
            "statement_lanes": statement.len(),
        },
        "planned_native_calls": {"ChildKey::reconstruct": 1, "ChildKey::prove": 1, "independent_verify": 0, "negative_verifies": 0},
        "open_obligations": a102_child_open_obligations(),
    }));
    let options = ProofOptions { params_dir: inputs.srs_dir.clone(), max_k: inputs.max_k };
    // Native calls 1 and 2 of 2. No retry, fallback or cap change on failure.
    let key = ChildKey::reconstruct(&produced.request, &options).unwrap_or_else(|e| {
        persist_a102_child_error(out, selector, "ChildKey::reconstruct", &format!("{e:?}"));
        panic!("ChildKey::reconstruct failed: {e:?}")
    });
    // Reconstructed public key, instance and claim, then a phase record, are synced before
    // `prove`. They are reconstruction outputs only, not a proof or a verification result.
    let vk_file_bytes = key.vk_bytes().len();
    let vk_sha256 = write_synced_artifact(out, A102_CHILD_VK_FILENAME, key.vk_bytes());
    let instance_hex: Vec<String> = key.instance().iter().map(field_hex).collect();
    let instance_text = serde_json::to_vec_pretty(&instance_hex).expect("serialize instance");
    let instance_sha256 = write_synced_artifact(out, A102_CHILD_INSTANCE_FILENAME, &instance_text);
    let claim_text = serde_json::to_vec_pretty(key.claim()).expect("serialize claim");
    let claim_sha256 = write_synced_artifact(out, A102_CHILD_CLAIM_FILENAME, &claim_text);
    let phase_sha256 = persist_a102_child_phase(
        out,
        A102_CHILD_PHASE_RECONSTRUCTED_FILENAME,
        selector,
        serde_json::json!({
            "record": "ChildKey::reconstruct returned Ok; ChildKey::prove not yet called or returned",
            "reconstruct_returned_ok": true,
            "prove_returned": false,
            "circuit_k": key.claim().circuit_k,
            "instance_lanes": key.instance().len(),
            "artifacts": {
                "vk": {"file": A102_CHILD_VK_FILENAME, "bytes": vk_file_bytes, "sha256": vk_sha256},
                "instance": {"file": A102_CHILD_INSTANCE_FILENAME, "bytes": instance_text.len(), "sha256": instance_sha256},
                "claim": {"file": A102_CHILD_CLAIM_FILENAME, "bytes": claim_text.len(), "sha256": claim_sha256},
            },
            "role": "reconstruction outputs only; no proof exists and nothing is verified at this boundary",
        }),
    );
    let child = key.prove().unwrap_or_else(|e| {
        persist_a102_child_error(out, selector, "ChildKey::prove", &format!("{e:?}"));
        panic!("ChildKey::prove failed: {e:?}")
    });
    // The proof is synced as soon as `prove` returns, before any host assertion.
    let proof_sha256 = write_synced_artifact(out, A102_CHILD_PROOF_FILENAME, &child.proof);
    let public_values: Vec<String> = statement.iter().map(u128::to_string).collect();
    let lanes_ok = child.instance.len() == EXPECTED_STATEMENT_LANES + 1;
    let instance_ok = child.instance.as_slice() == key.instance();
    let claim_ok = &child.claim == key.claim() && child.claim.public_values == public_values;
    let vk_ok = vk_sha256 == key.vk_sha256();
    // The pre-prove files must hold exactly the returned child's key, instance and claim.
    let persisted_ok = child.vk_bytes.as_slice() == key.vk_bytes()
        && hex(&Sha256::digest(&child.vk_bytes)) == vk_sha256
        && serde_json::to_vec_pretty(&child.instance_hex()).expect("serialize instance") == instance_text
        && serde_json::to_vec_pretty(&child.claim).expect("serialize claim") == claim_text;
    drop(key);
    let passed = lanes_ok && instance_ok && claim_ok && vk_ok && persisted_ok;
    let report = serde_json::json!({
        "schema": A102_CHILD_REPORT_SCHEMA,
        "selector": selector,
        "record": "producer-returned-ok",
        "producer_returned_ok": true,
        "host_checks": {"lanes": lanes_ok, "instance": instance_ok, "claim": claim_ok, "vk_sha256": vk_ok, "pre_prove_files_match_returned_child": persisted_ok},
        "host_checks_passed": passed,
        "claim": {
            "format": child.claim.format,
            "binding": child.claim.binding,
            "transcript": child.claim.transcript,
            "context_hash": child.claim.context_hash,
            "relation_hash": child.claim.relation_hash,
            "zkir_hash": child.claim.zkir_hash,
            "adapter_source_sha256": child.claim.adapter_source_sha256,
            "circuit_k": child.claim.circuit_k,
        },
        "producer_vk_sha256": vk_sha256,
        "producer_vk_sha256_role": "producer export only; a consumer pin comes from the separate frozen verify-only selector",
        "artifacts": {
            "proof": {"file": A102_CHILD_PROOF_FILENAME, "bytes": child.proof.len(), "sha256": proof_sha256},
            "vk": {"file": A102_CHILD_VK_FILENAME, "bytes": vk_file_bytes, "sha256": vk_sha256},
            "instance": {"file": A102_CHILD_INSTANCE_FILENAME, "lanes": child.instance.len(), "sha256": instance_sha256},
            "claim": {"file": A102_CHILD_CLAIM_FILENAME, "sha256": claim_sha256},
        },
        "input_record": {"file": A102_CHILD_INPUT_FILENAME, "sha256": input_sha256},
        "phase_records": [{"file": A102_CHILD_PHASE_RECONSTRUCTED_FILENAME, "sha256": phase_sha256}],
        "inputs": inputs_json,
        "native_calls_by_consumer": {"ChildKey::reconstruct": 1, "ChildKey::prove": 1, "independent_verify": 0, "negative_verifies": 0},
        "resource_observation": "UNESTABLISHED",
        "open_obligations": a102_child_open_obligations(),
    });
    let report_bytes = serde_json::to_vec_pretty(&report).expect("serialize child producer report");
    write_synced_artifact(out, A102_CHILD_REPORT_FILENAME, &report_bytes);
    assert!(passed, "child producer host checks failed; artifacts preserved, no retry");
    println!(
        "A102 child producer returned Ok: k={} proof_bytes={} out={}",
        child.claim.circuit_k,
        child.proof.len(),
        out.display()
    );
}

/// Frozen-child consumer: bounded producer artifacts and one independent
/// `verify_sherlock_child` (claim and instance rebuilt and compared before SRS access, key
/// rebuilt with `setup_vk` and compared bytewise, stdlib verify). No `setup_pk`, proving,
/// retry or fallback. Not a soundness, ledger or resource claim.
#[test]
#[ignore = "native SRS/keygen_vk/verify of frozen A102 child artifacts; requires explicit MORIARTY_A102_CHILD_VERIFY_* inputs and separate resource admission"]
fn native_a102_poseidon_child_frozen_verify_only_independently_reconstructs_and_verifies() {
    let selector = "native_a102_poseidon_child_frozen_verify_only_independently_reconstructs_and_verifies";
    let inputs = read_a102_child_inputs(
        Some(A102_CHILD_VERIFY_INPUT_DIR_ENV),
        A102_CHILD_VERIFY_SRS_DIR_ENV,
        A102_CHILD_VERIFY_MAX_K_ENV,
        A102_CHILD_VERIFY_OUTPUT_DIR_ENV,
    );
    let input_dir = inputs.input_dir.clone().expect("explicit input directory");
    let (child, files) = read_frozen_a102_child(&input_dir);
    let (produced, statement, program_sha256) = reconstruct_a102_child_request();
    join_frozen_a102_child(&child, &produced.request, &statement, inputs.max_k);
    let inputs_json = serde_json::json!({
        "input_dir": input_dir.display().to_string(),
        "files": files,
        "srs_dir": inputs.srs_dir.display().to_string(),
        "max_k": inputs.max_k,
    });
    let out = inputs.output_dir.as_path();
    let input_sha256 = begin_a102_child_output(out, &serde_json::json!({
        "schema": A102_CHILD_REPORT_SCHEMA,
        "selector": selector,
        "record": "input-identity-before-native-call; not a result",
        "inputs": inputs_json,
        "reconstructed": {
            "source_identity": produced.request.identity,
            "program_l4_sha256": program_sha256,
            "statement_lanes": statement.len(),
        },
        "planned_native_calls": {"verify_sherlock_child": 1, "setup_pk": 0, "prove": 0, "negative_verifies": 0},
        "open_obligations": a102_child_open_obligations(),
    }));
    let options = ProofOptions { params_dir: inputs.srs_dir.clone(), max_k: inputs.max_k };
    // Sole native call. No retry, fallback or cap change on failure.
    let verification = verify_sherlock_child(&produced.request, &child, &options).unwrap_or_else(|e| {
        persist_a102_child_error(out, selector, "verify_sherlock_child", &format!("{e:?}"));
        panic!("verify_sherlock_child failed: {e:?}")
    });
    let vk_ok = verification.vk_sha256 == hex(&Sha256::digest(&child.vk_bytes));
    let shape_ok = verification.instance_lanes == child.instance.len()
        && verification.proof_bytes as usize == child.proof.len()
        && verification.circuit_k == child.claim.circuit_k;
    let report = serde_json::json!({
        "schema": A102_CHILD_REPORT_SCHEMA,
        "selector": selector,
        "record": "verify-only-returned-ok",
        "verify_returned_ok": true,
        "host_checks": {"vk_sha256_matches_frozen_file": vk_ok, "lanes_bytes_and_k": shape_ok},
        "verification": verification,
        "fenrir_pins": {
            "SHERLOCK_CHILD_VK_SHA256": verification.vk_sha256,
            "SHERLOCK_CHILD_INSTANCE_LANES": verification.instance_lanes,
            "role": "from this independent reconstruction; valid only for this exact source, lock and k",
        },
        "input_record": {"file": A102_CHILD_INPUT_FILENAME, "sha256": input_sha256},
        "inputs": inputs_json,
        "native_calls_by_consumer": {"verify_sherlock_child": 1, "setup_pk": 0, "prove": 0, "negative_verifies": 0},
        "resource_observation": "UNESTABLISHED",
    });
    let report_bytes = serde_json::to_vec_pretty(&report).expect("serialize child verify-only report");
    write_synced_artifact(out, A102_CHILD_REPORT_FILENAME, &report_bytes);
    assert!(vk_ok && shape_ok, "child verify-only host checks failed; no retry");
    println!(
        "A102 child frozen verify-only returned Ok: k={} proof_bytes={} out={}",
        verification.circuit_k,
        verification.proof_bytes,
        out.display()
    );
}

/// Frozen-child negatives: one key reconstruction (`setup_vk` only), the honest frozen
/// proof as control, then eight tampered instance or proof cases against the
/// reconstructed key, recorded before they are asserted. Synced phase records mark the
/// returns of reconstruction, the frozen join and the control verify. No proving. Not a
/// soundness, ledger or resource claim.
#[test]
#[ignore = "native SRS/keygen_vk and nine verifies over frozen A102 child artifacts; requires explicit MORIARTY_A102_CHILD_NEGATIVE_* inputs and separate resource admission"]
fn native_a102_poseidon_child_frozen_negatives_reject_tampering() {
    let selector = "native_a102_poseidon_child_frozen_negatives_reject_tampering";
    let inputs = read_a102_child_inputs(
        Some(A102_CHILD_NEGATIVE_INPUT_DIR_ENV),
        A102_CHILD_NEGATIVE_SRS_DIR_ENV,
        A102_CHILD_NEGATIVE_MAX_K_ENV,
        A102_CHILD_NEGATIVE_OUTPUT_DIR_ENV,
    );
    let input_dir = inputs.input_dir.clone().expect("explicit input directory");
    let (child, files) = read_frozen_a102_child(&input_dir);
    let (produced, statement, program_sha256) = reconstruct_a102_child_request();
    join_frozen_a102_child(&child, &produced.request, &statement, inputs.max_k);
    let inputs_json = serde_json::json!({
        "input_dir": input_dir.display().to_string(),
        "files": files,
        "srs_dir": inputs.srs_dir.display().to_string(),
        "max_k": inputs.max_k,
    });
    let out = inputs.output_dir.as_path();
    let input_sha256 = begin_a102_child_output(out, &serde_json::json!({
        "schema": A102_CHILD_REPORT_SCHEMA,
        "selector": selector,
        "record": "input-identity-before-native-call; not a result",
        "inputs": inputs_json,
        "reconstructed": {
            "source_identity": produced.request.identity,
            "program_l4_sha256": program_sha256,
            "statement_lanes": statement.len(),
        },
        "planned_native_calls": {"ChildKey::reconstruct": 1, "ChildKey::verify": 9, "setup_pk": 0, "prove": 0},
        "open_obligations": a102_child_open_obligations(),
    }));
    let options = ProofOptions { params_dir: inputs.srs_dir.clone(), max_k: inputs.max_k };
    // Native call 1: no retry, fallback or cap change on failure.
    let key = ChildKey::reconstruct(&produced.request, &options).unwrap_or_else(|e| {
        persist_a102_child_error(out, selector, "ChildKey::reconstruct", &format!("{e:?}"));
        panic!("ChildKey::reconstruct failed: {e:?}")
    });
    let reconstructed_phase_sha256 = persist_a102_child_phase(
        out,
        A102_CHILD_PHASE_RECONSTRUCTED_FILENAME,
        selector,
        serde_json::json!({
            "record": "ChildKey::reconstruct returned Ok; frozen-artifact join not yet evaluated",
            "reconstruct_returned_ok": true,
            "circuit_k": key.claim().circuit_k,
            "vk_sha256": key.vk_sha256(),
            "instance_lanes": key.instance().len(),
        }),
    );
    if key.vk_bytes() != child.vk_bytes.as_slice()
        || key.claim() != &child.claim
        || key.instance() != child.instance.as_slice()
    {
        drop(key);
        let message = "Reconstructed key, claim or instance differs from the frozen artifacts.";
        persist_a102_child_error(out, selector, "reconstruction-join", message);
        panic!("{message}")
    }
    let joined_phase_sha256 = persist_a102_child_phase(
        out,
        A102_CHILD_PHASE_JOINED_FILENAME,
        selector,
        serde_json::json!({
            "record": "reconstructed key, claim and instance equal the frozen artifacts; honest control not yet called",
            "join_passed": true,
            "control_returned": false,
        }),
    );
    let control = key.verify(&child.instance, &child.proof);
    let control_phase_sha256 = persist_a102_child_phase(
        out,
        A102_CHILD_PHASE_CONTROL_FILENAME,
        selector,
        serde_json::json!({
            "record": "honest-control ChildKey::verify returned; tampered cases not yet called",
            "control_returned_ok": control.is_ok(),
            "control_error": control.as_ref().err().map(|e| serde_json::json!({"code": e.code, "message": e.message})),
            "negatives_called": 0,
        }),
    );
    let n = child.instance.len();
    let mut cases: Vec<(String, Vec<Fr>, Vec<u8>)> = Vec::new();
    for lane in [0usize, 1, n - 1] {
        let mut bad = child.instance.clone();
        bad[lane] = bad[lane] + Fr::from(1u64);
        cases.push((format!("instance-lane-{lane}-plus-one"), bad, child.proof.clone()));
    }
    cases.push(("instance-last-lane-dropped".into(), child.instance[..n - 1].to_vec(), child.proof.clone()));
    let mut longer = child.instance.clone();
    longer.push(Fr::from(0u64));
    cases.push(("instance-zero-lane-appended".into(), longer, child.proof.clone()));
    let mut flipped = child.proof.clone();
    flipped[child.proof.len() / 2] ^= 1;
    cases.push(("proof-middle-byte-bit-flip".into(), child.instance.clone(), flipped));
    let mut trailing = child.proof.clone();
    trailing.push(0);
    cases.push(("proof-trailing-zero-byte".into(), child.instance.clone(), trailing));
    let shorter = child.proof[..child.proof.len() - 1].to_vec();
    cases.push(("proof-last-byte-dropped".into(), child.instance.clone(), shorter));
    let mut results = serde_json::Map::new();
    let mut all_rejected = true;
    for (name, instance, proof) in &cases {
        let outcome = key.verify(instance, proof);
        all_rejected &= outcome.is_err();
        let verdict = match outcome {
            Ok(()) => serde_json::json!("ACCEPTED"),
            Err(e) => serde_json::json!({"rejected": e.code, "message": e.message}),
        };
        results.insert(name.clone(), verdict);
    }
    drop(key);
    let control_json = match &control {
        Ok(()) => serde_json::json!("accepted"),
        Err(e) => serde_json::json!({"rejected": e.code, "message": e.message}),
    };
    let report = serde_json::json!({
        "schema": A102_CHILD_REPORT_SCHEMA,
        "selector": selector,
        "record": "negatives-completed",
        "honest_control": control_json,
        "negatives": results,
        "all_negatives_rejected": all_rejected,
        "phase_records": [
            {"file": A102_CHILD_PHASE_RECONSTRUCTED_FILENAME, "sha256": reconstructed_phase_sha256},
            {"file": A102_CHILD_PHASE_JOINED_FILENAME, "sha256": joined_phase_sha256},
            {"file": A102_CHILD_PHASE_CONTROL_FILENAME, "sha256": control_phase_sha256}
        ],
        "input_record": {"file": A102_CHILD_INPUT_FILENAME, "sha256": input_sha256},
        "inputs": inputs_json,
        "native_calls_by_consumer": {"ChildKey::reconstruct": 1, "ChildKey::verify": 1 + cases.len(), "setup_pk": 0, "prove": 0},
        "resource_observation": "UNESTABLISHED",
        "open_obligations": a102_child_open_obligations(),
    });
    let report_bytes = serde_json::to_vec_pretty(&report).expect("serialize child negatives report");
    write_synced_artifact(out, A102_CHILD_REPORT_FILENAME, &report_bytes);
    assert!(control.is_ok() && all_rejected, "honest control failed or a tampered case was accepted; no retry");
}
