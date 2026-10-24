//! Source-level normal wiring and refusal tests for `sherlock_literal_complete`.
//! Native-touching SDK entry points (`derive_keys`, `derive_keys_for_request`,
//! `prove_call`, `verify_call`, `build_call_transaction`, and `ContractState`
//! proof-verifying consumers) remain UNRUN (`native_authority == 0`).

use super::*;
use midnight_transient_crypto::proofs::{KeyLocation, ProofPreimage};
use mori_backend::sherlock_machine::literal_complete_bridge::{
    COMPLETE_KEY_LOCATION, COMPLETE_OPEN_OBLIGATIONS, ImageValues, LiteralCapacities,
    LiteralCompleteExecutionRequest, LiteralCompleteLayout, LiteralCompleteValues,
    LiteralEnvelopeValues, LiteralSchema, LiteralSourceIdentity,
};
use std::borrow::Cow;

fn sample_identity() -> LiteralSourceIdentity {
    LiteralSourceIdentity {
        k_source_sha256: "11".repeat(32),
        reference_source_sha256: "22".repeat(32),
        program_source_sha256: "33".repeat(32),
    }
}

fn sample_capacities() -> LiteralCapacities {
    LiteralCapacities {
        program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0],
        state: [0, 0, 0, 0, 0, 1],
        stack_limit: 8,
    }
}

fn zero_image(
    root_len: usize,
    tables: &[mori_backend::sherlock_machine::literal_complete_bridge::LiteralTable],
) -> ImageValues {
    ImageValues {
        root: vec![0; root_len],
        tables: tables
            .iter()
            .map(|t| vec![vec![0; t.lanes.len()]; t.capacity])
            .collect(),
    }
}

fn set_state_root(schema: &LiteralSchema, image: &mut ImageValues, name: &str, value: u128) {
    let idx = schema
        .state_root
        .iter()
        .position(|l| l.name == name)
        .expect("fixed L4 state_root lane");
    image.root[idx] = value;
}

fn set_state_table(
    schema: &LiteralSchema,
    image: &mut ImageValues,
    table: usize,
    row: usize,
    name: &str,
    value: u128,
) {
    let idx = schema.state_tables[table]
        .lanes
        .iter()
        .position(|l| l.name == name)
        .expect("fixed L4 state_table lane");
    image.tables[table][row][idx] = value;
}

fn sample_request_and_layout() -> (LiteralCompleteLayout, LiteralCompleteExecutionRequest) {
    let capacities = sample_capacities();
    let identity = sample_identity();
    let layout = LiteralCompleteLayout::new(&capacities, &identity, 4096).unwrap();
    let schema = layout.schema();

    let mut program = zero_image(schema.program_header.len(), &schema.program_tables);
    program.root = vec![0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0];
    program.tables[10] = vec![vec![0, 0, 0], vec![1, 0, 0], vec![2, 0, 2], vec![3, 2, 2]];
    program.tables[11] = vec![
        vec![2, 0, 0, 0],
        vec![2, 1, 0, 2],
        vec![3, 0, 2, 1],
        vec![3, 1, 3, 2],
    ];
    program.tables[12] = vec![vec![0], vec![2], vec![0], vec![3], vec![3]];

    let mut init = zero_image(schema.state_root.len(), &schema.state_tables);
    set_state_root(schema, &mut init, "controlLength", 1);
    set_state_root(schema, &mut init, "stackLimit", 4);
    set_state_root(schema, &mut init, "remainingFuel.0", 5);
    set_state_root(schema, &mut init, "initialFuel.0", 5);

    let mut intermediate = zero_image(schema.state_root.len(), &schema.state_tables);
    set_state_root(schema, &mut intermediate, "controlLength", 1);
    set_state_root(schema, &mut intermediate, "stackLimit", 4);
    set_state_root(schema, &mut intermediate, "remainingFuel.0", 4);
    set_state_root(schema, &mut intermediate, "initialFuel.0", 5);
    set_state_table(schema, &mut intermediate, 5, 0, "tag", 1);
    set_state_table(schema, &mut intermediate, 5, 0, "valueRef.space", 0);
    set_state_table(schema, &mut intermediate, 5, 0, "valueRef.index", 0);

    let mut done = zero_image(schema.state_root.len(), &schema.state_tables);
    set_state_root(schema, &mut done, "controlLength", 0);
    set_state_root(schema, &mut done, "outcomeTag", 1);
    set_state_root(schema, &mut done, "outcomeValue.space", 0);
    set_state_root(schema, &mut done, "outcomeValue.index", 0);
    set_state_root(schema, &mut done, "stackLimit", 4);
    set_state_root(schema, &mut done, "remainingFuel.0", 4);
    set_state_root(schema, &mut done, "initialFuel.0", 5);

    let values = LiteralCompleteValues::new(program, init, intermediate, done);
    let request = LiteralCompleteExecutionRequest {
        capacities,
        identity,
        values,
    };
    (layout, request)
}

#[test]
fn normal_request_layout_opening_and_preimage_wiring_succeeds() {
    let (layout, request) = sample_request_and_layout();
    let built = build_layout_for_request(&request, layout.statement_len()).unwrap();
    assert_eq!(built.context(), layout.context());
    assert_eq!(built.layout_hash(), layout.layout_hash());
    assert_eq!(built.relation_hash(), layout.relation_hash());

    let comm_rand = Fr::from(37u64);
    let opening = SherlockCompleteCallOpening::from_request(&layout, &request, comm_rand).unwrap();
    opening.admit_request(&layout, &request).unwrap();

    let (statement, input, commitment, fields) =
        compute_expected_call_statement(&layout, &opening).unwrap();
    assert_eq!(statement.len(), layout.statement_len());
    assert_eq!(fields.len(), statement.len());
    assert_eq!(&statement[..2], &layout.context_limbs());

    let preimage = ProofPreimage {
        inputs: fields.clone(),
        private_transcript: vec![],
        public_transcript_inputs: vec![],
        public_transcript_outputs: vec![],
        binding_input: Fr::from(0u64),
        communications_commitment: Some((commitment, comm_rand)),
        key_location: KeyLocation(Cow::Borrowed(COMPLETE_KEY_LOCATION)),
    };
    let admitted =
        admit_call_preimage(&layout, COMPLETE_KEY_LOCATION, &opening, &preimage).unwrap();
    assert_eq!(admitted, statement);
    assert_ne!(input, AlignedValue::from(()));
}

#[test]
fn refuses_noncanonical_data_space_done_and_mismatched_requests_or_preimages() {
    let (layout, mut request) = sample_request_and_layout();
    let comm_rand = Fr::from(19u64);
    let opening = SherlockCompleteCallOpening::from_request(&layout, &request, comm_rand).unwrap();

    // Mutating done.outcomeValue.space to Data(1) with activeData == 0 is refused fail-closed.
    let mut canonical_claim = opening.clone();
    set_state_root(
        layout.schema(),
        &mut canonical_claim.values.done,
        "outcomeValue.space",
        1,
    );
    assert_eq!(
        compute_expected_call_statement(&layout, &canonical_claim)
            .unwrap_err()
            .code,
        "sherlock-statement"
    );

    // Request-opening and request-layout mismatches are refused.
    let mut wrong_val_req = request.clone();
    wrong_val_req.values.literal_envelope = LiteralEnvelopeValues {
        rule_tag: 0,
        charge: 0,
    };
    assert_eq!(
        opening
            .admit_request(&layout, &wrong_val_req)
            .unwrap_err()
            .code,
        "request-opening-mismatch"
    );

    request.identity.program_source_sha256 = "44".repeat(32);
    assert_eq!(
        SherlockCompleteCallOpening::from_request(&layout, &request, comm_rand)
            .unwrap_err()
            .code,
        "request-layout-mismatch"
    );

    // Builtin midnight/ key locations and tampered preimage commitments are refused.
    let (_, _, commitment, fields) = compute_expected_call_statement(&layout, &opening).unwrap();
    let mut preimage = ProofPreimage {
        inputs: fields,
        private_transcript: vec![],
        public_transcript_inputs: vec![],
        public_transcript_outputs: vec![],
        binding_input: Fr::from(0u64),
        communications_commitment: Some((commitment, comm_rand)),
        key_location: KeyLocation(Cow::Borrowed(COMPLETE_KEY_LOCATION)),
    };
    assert_eq!(
        admit_call_preimage(&layout, "midnight/zswap/spend", &opening, &preimage)
            .unwrap_err()
            .code,
        "entry-point"
    );
    preimage.communications_commitment = Some((commitment, Fr::from(99u64)));
    assert_eq!(
        admit_call_preimage(&layout, COMPLETE_KEY_LOCATION, &opening, &preimage)
            .unwrap_err()
            .code,
        "sherlock-preimage"
    );
}

#[test]
fn composed_closure_and_obligations_preserve_all_bridge_and_ledger_qualifications() {
    let closure = composed_ledger_source_closure();
    assert_eq!(closure.len(), 64);
    assert_ne!(closure, crate::proving::source_closure());
    let obligations = composed_open_obligations();
    assert_eq!(
        obligations.len(),
        COMPLETE_OPEN_OBLIGATIONS.len() + COMPOSED_LEDGER_ADDITIONAL_OBLIGATIONS.len()
    );
    for required in COMPLETE_OPEN_OBLIGATIONS {
        assert!(obligations.contains(&required));
    }
}
