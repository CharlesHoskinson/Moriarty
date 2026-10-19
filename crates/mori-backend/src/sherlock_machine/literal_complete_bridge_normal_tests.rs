//! Source-authored unit tests for [`super::literal_complete_bridge`] (UNRUN in
//! this source-only packet; `native_authority = 0`).
//!
//! Inspects statement lane ordering/length, cryptographic domain/identity
//! separation from single-step A102 [`LiteralLayout`], host admissibility across
//! all four images (`program`, `init`, `intermediate`, `done`) and both wire
//! envelopes (`(3, 1)` and `(0, 0)`), checked 16-byte LE `UInt128` fuel charge
//! (`init.remainingFuel >= 1` and `intermediate.remainingFuel == init.remainingFuel - 1`),
//! zeroed inactive `Return` columns in `intermediate.tables[5][0]`, exact composer
//! invocation and intermediate operand aliasing in [`LiteralCompleteLayout::emit_with_block`],
//! and [`ProducedLiteralReturnPublication`] continuity checks. Note (H1): if compiled
//! and executed later, these tests invoke `produce_checked_literal_complete_execution_request`
//! (calling host witness producer `produce_a102_literal_return_publication`) and
//! `LiteralCompleteLayout::emit_with_block` / `emit` (calling `IrSource::load(zkir.as_bytes())`),
//! without native prover/verifier execution.
//! Capacities below are test specimens only, never product defaults.

use super::literal_bridge::{CircuitMode, LiteralLayout};
use super::literal_complete_bridge::*;
use super::*;
use mori_core::sherlock::{Expr, Program, Value as SherlockValue};
use std::sync::Arc;

fn specimen_capacities() -> LiteralCapacities {
    LiteralCapacities {
        program: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 4, 5, 1, 1],
        state: [1, 1, 1, 1, 1, 1],
        stack_limit: 4,
    }
}

fn specimen_identity() -> LiteralSourceIdentity {
    LiteralSourceIdentity {
        k_source_sha256: "11".repeat(32),
        reference_source_sha256: "22".repeat(32),
        program_source_sha256: "33".repeat(32),
    }
}

fn specimen_uint_program(val: u128) -> Arc<Program> {
    Arc::new(Program {
        functions: Vec::new(),
        entry: Arc::new(Expr::Literal(SherlockValue::UInt(val))),
    })
}

#[test]
fn statement_layout_ordering_length_and_identity_separation_from_a102() {
    let caps = specimen_capacities();
    let id = specimen_identity();
    let schema = literal_schema(&caps).expect("valid specimen schema");
    let prog_lanes = schema.program_header.len()
        + schema
            .program_tables
            .iter()
            .map(|t| t.capacity * t.lanes.len())
            .sum::<usize>();
    let state_lanes = schema.state_root.len()
        + schema
            .state_tables
            .iter()
            .map(|t| t.capacity * t.lanes.len())
            .sum::<usize>();
    let expected_complete_len = 2 + prog_lanes + 3 * state_lanes + 4;

    let a102 = LiteralLayout::new(&caps, &id, expected_complete_len).expect("a102 layout");
    let complete = LiteralCompleteLayout::new(&caps, &id, expected_complete_len)
        .expect("complete layout");

    assert_eq!(complete.statement_len(), expected_complete_len);
    assert_eq!(complete.statement_len(), a102.statement_len() + state_lanes + 2);
    assert_ne!(complete.layout_hash(), a102.layout_hash);
    assert_ne!(complete.relation_hash(), a102.relation_hash);
    assert_ne!(complete.context(), a102.context);
    assert_ne!(complete.context_limbs(), a102.limbs());

    let lanes = complete.lanes();
    assert_eq!(lanes[0].0, "context");
    assert_eq!(lanes[0].1.name, "limb.0");
    assert_eq!(lanes[1].0, "context");
    assert_eq!(lanes[1].1.name, "limb.1");
    assert!(lanes[2].0.starts_with("program."));
    assert!(lanes[2 + prog_lanes].0.starts_with("init."));
    assert!(lanes[2 + prog_lanes + state_lanes].0.starts_with("intermediate."));
    assert!(lanes[2 + prog_lanes + 2 * state_lanes].0.starts_with("done."));
    assert_eq!(
        lanes[expected_complete_len - 4],
        (
            "literal_envelope".to_string(),
            LiteralLane {
                name: "rule_tag".into(),
                bits: 6,
                boolean: false,
                maximum: Some(38),
            }
        )
    );
    assert_eq!(lanes[expected_complete_len - 3].0, "literal_envelope");
    assert_eq!(lanes[expected_complete_len - 3].1.name, "charge");
    assert_eq!(lanes[expected_complete_len - 2].0, "return_envelope");
    assert_eq!(lanes[expected_complete_len - 2].1.name, "rule_tag");
    assert_eq!(lanes[expected_complete_len - 1].0, "return_envelope");
    assert_eq!(lanes[expected_complete_len - 1].1.name, "charge");

    let mut other_id = id.clone();
    other_id.program_source_sha256 = "44".repeat(32);
    let other = LiteralCompleteLayout::new(&caps, &other_id, expected_complete_len)
        .expect("other layout");
    assert_ne!(complete.context(), other.context());

    let err = LiteralCompleteLayout::new(&caps, &id, expected_complete_len - 1)
        .expect_err("bound below statement_len must fail");
    assert_eq!(err.code, "mori::backend::limit");
}

#[test]
fn producer_adapter_and_statement_check_all_four_images_and_both_envelopes() {
    let caps = specimen_capacities();
    let id = specimen_identity();
    let program = specimen_uint_program(42);

    let (produced, layout, request, statement) =
        produce_checked_literal_complete_execution_request(
            program,
            &caps,
            5,
            2,
            &id,
            local_max_statement_lanes(),
        )
        .expect("valid two-step literal specimen");

    assert_eq!(statement.len(), layout.statement_len());
    assert_eq!(&statement[..2], &layout.context_limbs());
    assert_eq!(
        &statement[statement.len() - 4..],
        &[
            COMPLETE_LITERAL_EXECUTION_STEP1_RULE_TAG,
            COMPLETE_LITERAL_EXECUTION_STEP1_RULE_CHARGE,
            COMPLETE_LITERAL_EXECUTION_STEP2_RULE_TAG,
            COMPLETE_LITERAL_EXECUTION_STEP2_RULE_CHARGE,
        ]
    );
    assert_eq!(request.values.init, produced.literal_transition.request.values.pre);
    assert_eq!(
        request.values.intermediate,
        produced.literal_transition.request.values.post
    );
    assert_eq!(request.values.intermediate, produced.values.pre);
    assert_eq!(request.values.done, produced.values.post);

    // Wrong step-2 envelope must be refused.
    let mut bad_env = request.values.clone();
    bad_env.return_envelope.rule_tag = 3;
    let err = layout
        .statement(&bad_env)
        .expect_err("non-A80 return_envelope must fail");
    assert_eq!(err.code, "mori::backend::sherlock-envelope");

    // Claiming Data-space (space == 1) on Step-mode Done without projectA must be refused.
    let mut fake_canonical_done = request.values.clone();
    let space_idx = layout
        .schema()
        .state_root
        .iter()
        .position(|l| l.name == "outcomeValue.space")
        .expect("outcomeValue.space lane");
    fake_canonical_done.done.root[space_idx] = 1;
    let err = layout
        .statement(&fake_canonical_done)
        .expect_err("Data-space outcomeValue in Step-mode Done must fail");
    assert_eq!(err.code, "mori::backend::sherlock-representation");

    // Breaking A102.post == A80.pre continuity in ProducedLiteralReturnPublication must fail.
    let mut broken_produced = produced.clone();
    broken_produced.values.pre.root[0] = 1;
    let err = build_literal_complete_execution_request(&broken_produced)
        .expect_err("divergent intermediate state must fail");
    assert_eq!(err.code, "mori::backend::sherlock-continuity");

    // Divergent program images in ProducedLiteralReturnPublication must fail continuity.
    let mut broken_prog = produced.clone();
    broken_prog.values.program.root[0] ^= 1;
    let err = build_literal_complete_execution_request(&broken_prog)
        .expect_err("divergent program image must fail");
    assert_eq!(err.code, "mori::backend::sherlock-continuity");
}

#[test]
fn complete_statement_enforces_u128_le_fuel_decrement_and_inactive_return_columns_zero() {
    let caps = specimen_capacities();
    let id = specimen_identity();
    let program = specimen_uint_program(42);
    let (_, layout, request, _) = produce_checked_literal_complete_execution_request(
        program,
        &caps,
        5,
        2,
        &id,
        local_max_statement_lanes(),
    )
    .expect("valid two-step literal specimen");
    let s = layout.schema();
    let values = request.values;

    let set_fuel_u128 = |img: &mut ImageValues, prefix: &str, val: u128| {
        for (byte, b) in val.to_le_bytes().into_iter().enumerate() {
            let lane_name = format!("{prefix}.{byte}");
            let idx = s
                .state_root
                .iter()
                .position(|l| l.name == lane_name)
                .expect("fixed L4 fuel byte lane");
            img.root[idx] = u128::from(b);
        }
    };

    // 1. Unchanged charged fuel (intermediate.remainingFuel == init.remainingFuel) must fail.
    let mut uncharged = values.clone();
    set_fuel_u128(&mut uncharged.init, "initialFuel", 5);
    set_fuel_u128(&mut uncharged.init, "remainingFuel", 5);
    set_fuel_u128(&mut uncharged.intermediate, "initialFuel", 5);
    set_fuel_u128(&mut uncharged.intermediate, "remainingFuel", 5);
    set_fuel_u128(&mut uncharged.done, "initialFuel", 5);
    set_fuel_u128(&mut uncharged.done, "remainingFuel", 5);
    let err = layout
        .statement(&uncharged)
        .expect_err("uncharged intermediate remainingFuel must fail");
    assert_eq!(err.code, "mori::backend::sherlock-representation");

    // 2. Wrong fuel decrement (5 -> 3 instead of 5 -> 4) must fail.
    let mut wrong_dec = uncharged.clone();
    set_fuel_u128(&mut wrong_dec.intermediate, "remainingFuel", 3);
    set_fuel_u128(&mut wrong_dec.done, "remainingFuel", 3);
    let err = layout
        .statement(&wrong_dec)
        .expect_err("wrong fuel decrement must fail");
    assert_eq!(err.code, "mori::backend::sherlock-representation");

    // 3. Zero initial fuel (init.remainingFuel == 0) and 0 -> u128::MAX underflow wrap must fail.
    let mut zero_fuel = values.clone();
    set_fuel_u128(&mut zero_fuel.init, "initialFuel", 0);
    set_fuel_u128(&mut zero_fuel.init, "remainingFuel", 0);
    set_fuel_u128(&mut zero_fuel.intermediate, "initialFuel", 0);
    set_fuel_u128(&mut zero_fuel.intermediate, "remainingFuel", 0);
    set_fuel_u128(&mut zero_fuel.done, "initialFuel", 0);
    set_fuel_u128(&mut zero_fuel.done, "remainingFuel", 0);
    let err = layout
        .statement(&zero_fuel)
        .expect_err("zero initial remainingFuel must fail");
    assert_eq!(err.code, "mori::backend::sherlock-representation");

    let mut underflow_wrap = zero_fuel.clone();
    set_fuel_u128(&mut underflow_wrap.intermediate, "remainingFuel", u128::MAX);
    set_fuel_u128(&mut underflow_wrap.done, "remainingFuel", u128::MAX);
    let err = layout
        .statement(&underflow_wrap)
        .expect_err("0 -> u128::MAX fuel underflow wrap must fail");
    assert_eq!(err.code, "mori::backend::sherlock-representation");

    // 4. Multi-byte LE borrow (256 -> 255 across remainingFuel.0 and remainingFuel.1) succeeds,
    // while high-byte corruption in remainingFuel.15 fails.
    let mut borrow_ok = values.clone();
    set_fuel_u128(&mut borrow_ok.init, "initialFuel", 256);
    set_fuel_u128(&mut borrow_ok.init, "remainingFuel", 256);
    set_fuel_u128(&mut borrow_ok.intermediate, "initialFuel", 256);
    set_fuel_u128(&mut borrow_ok.intermediate, "remainingFuel", 255);
    set_fuel_u128(&mut borrow_ok.done, "initialFuel", 256);
    set_fuel_u128(&mut borrow_ok.done, "remainingFuel", 255);
    layout
        .statement(&borrow_ok)
        .expect("valid 16-byte LE 256 -> 255 fuel borrow must succeed");

    let mut bad_high_byte = borrow_ok.clone();
    let rem15 = s
        .state_root
        .iter()
        .position(|l| l.name == "remainingFuel.15")
        .expect("remainingFuel.15 lane");
    bad_high_byte.intermediate.root[rem15] = 1;
    bad_high_byte.done.root[rem15] = 1;
    let err = layout
        .statement(&bad_high_byte)
        .expect_err("corrupted high fuel byte remainingFuel.15 must fail");
    assert_eq!(err.code, "mori::backend::sherlock-representation");

    // 5. Every inactive Return column in intermediate.tables[5][0] must be 0.
    for (col_idx, lane) in s.state_tables[5].lanes.iter().enumerate() {
        if matches!(lane.name.as_str(), "tag" | "valueRef.space" | "valueRef.index") {
            continue;
        }
        let mut dirty_ctrl = values.clone();
        dirty_ctrl.intermediate.tables[5][0][col_idx] = 1;
        let err = layout
            .statement(&dirty_ctrl)
            .expect_err("nonzero inactive Return column in intermediate ControlRows[0] must fail");
        assert_eq!(err.code, "mori::backend::sherlock-representation");
    }

    // 6. Nonzero done ControlRows[0] while done.controlLength == 0 must fail padded check.
    let mut dirty_done_ctrl = values;
    dirty_done_ctrl.done.tables[5][0][0] = 1;
    let err = layout
        .statement(&dirty_done_ctrl)
        .expect_err("nonzero done ControlRows[0] when controlLength == 0 must fail");
    assert_eq!(err.code, "mori::backend::sherlock-representation");
}

#[test]
fn emit_modes_bind_context_and_invoke_composer_with_exact_aliased_intermediate() {
    let caps = specimen_capacities();
    let id = specimen_identity();
    let layout = LiteralCompleteLayout::new(&caps, &id, local_max_statement_lanes())
        .expect("complete layout");

    let (local_ir, local_zkir, block) = layout
        .emit_with_block(CircuitMode::LocalPublic)
        .expect("local emit");
    assert!(local_ir.inputs.is_empty());
    assert!(!local_ir.do_communications_commitment);
    assert!(!local_ir.outputs.is_empty());
    assert!(local_zkir.contains(COMPLETE_STATEMENT_VARIABLE_PREFIX));

    // Shared program and exact aliased intermediate between A102 and A80.
    assert_eq!(
        block.literal_step_preserved.program,
        block.return_publication_step_preserved.program
    );
    assert_eq!(
        block.literal_step_preserved.post,
        block.return_publication_step_preserved.pre
    );
    assert_eq!(block.literal_step_preserved.pre, block.preserved.init);
    assert_eq!(
        block.return_publication_step_preserved.post,
        block.preserved.done
    );

    // Composer instructions appear contiguously after statement PublicInput/Impact and context limb ConstrainEq.
    let prefix_len = 2 * layout.statement_len() + 2;
    let instrs = &local_ir.instructions;
    assert_eq!(
        &instrs[prefix_len..prefix_len + block.instructions.len()],
        block.instructions.as_slice()
    );

    let (ledger_ir, _) = layout
        .emit(CircuitMode::LedgerCall)
        .expect("ledger call emit");
    assert_eq!(ledger_ir.inputs.len(), layout.statement_len());
    assert!(ledger_ir.do_communications_commitment);
    assert!(ledger_ir.outputs.is_empty());
    match ledger_ir.instructions.last() {
        Some(Instruction::Output { vals }) => assert!(vals.is_empty()),
        other => panic!("expected empty Output terminator, got {other:?}"),
    }
}
