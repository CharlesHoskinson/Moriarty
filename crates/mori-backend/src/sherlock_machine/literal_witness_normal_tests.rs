//! Normal source-only unit tests for the A102 witness producer and logical-lane
//! adapter.
//!
//! Scope and evidence qualification:
//! - Derives all checks from public `mori_core::sherlock`, `layout_a`, and
//!   `mori_backend::sherlock_machine` types, `prepare_a`, `Machine::step()`,
//!   and `encode_observed_a`.
//! - Does not import private oracle fixtures or assert fabricated expected vectors.
//! - Agreement between `produce_a102_literal_transition`, `LiteralLayout::statement`,
//!   and `decode_observed_a` (on `encode_observed_a`-produced `pre_state` only)
//!   is internal host consistency, not independent oracle or native proof evidence.

use super::literal_host::{
    literal_schema, LiteralCapacities, LiteralLayout, LiteralSourceIdentity, RULE_CHARGE,
    RULE_START_LINE, RULE_TAG,
};
use super::literal_witness::{
    build_checked_literal_transition_request, build_literal_transition_request,
    core_program_capacities, core_state_capacities, produce_a102_literal_transition,
    program_block_to_image_values, state_block_to_image_values,
};

// Explicit caller test bound for host statement construction; not VERIFIER_MAX_DEGREE or a resource grant.
const HOST_TEST_MAX_STATEMENT_LANES: usize = 8_192;
use mori_core::sherlock::layout_a::{
    decode_observed_a, encode_observed_a, prepare_a, AImage, ControlRow, DataNode, DecodeMode,
    StateFailure, ValueRef,
};
use mori_core::sherlock::{
    Arm, BinaryOp, Control, Environment, Expr, Frame, Function, Outcome, Parameter, Pattern,
    Program, Snapshot, Tree, Type, UnaryOp, Value, K_SOURCE_SHA256, PROFILE,
};
use std::sync::Arc;

fn sample_identity() -> LiteralSourceIdentity {
    LiteralSourceIdentity {
        k_source_sha256: K_SOURCE_SHA256.to_string(),
        reference_source_sha256: "11".repeat(32),
        program_source_sha256: "22".repeat(32),
    }
}

fn minimal_literal_capacities(k: usize, kf: usize) -> LiteralCapacities {
    LiteralCapacities {
        // N, NB, O, OP, X, XC, F, P, R, A, T, Q, QF, K, KF
        program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, k, kf],
        // D, DF, E, V, S, C
        state: [0, 0, 0, 0, 0, 1],
        stack_limit: 8,
    }
}

#[test]
fn contrasting_literal_forms_and_uint128_precision_produce_valid_step_requests() {
    let id = sample_identity();

    // 1. UInt128 with 16 distinct non-zero bytes and multi-byte fuel borrow across 8 zero bytes.
    let precise_uint: u128 = 0xfedc_ba98_7654_3210_0123_4567_89ab_cdef;
    let initial_fuel: u128 = 1u128 << 64; // LE bytes 0..8 are 0x00, byte 8 is 0x01.
    let stack_limit: usize = 5;
    let uint_prog = Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(Value::UInt(precise_uint))),
    });
    let uint_caps = LiteralCapacities {
        program: [0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 4, 4, 5, 2, 0],
        state: [0, 0, 0, 0, 0, 2],
        stack_limit: 16,
    };

    let (produced_uint, statement_uint) = build_checked_literal_transition_request(
        uint_prog.clone(),
        &uint_caps,
        initial_fuel,
        stack_limit,
        &id,
        HOST_TEST_MAX_STATEMENT_LANES,
    )
    .expect("UInt128 literal transition should build and pass host statement checks");

    assert_eq!(produced_uint.step_record.rule_start_line as u128, RULE_START_LINE);
    assert_eq!(u128::from(produced_uint.step_record.charge), RULE_CHARGE);
    assert_eq!(produced_uint.selected_expr_id, 0);
    assert_eq!(produced_uint.selected_constant_ref, ValueRef::constant(0));
    assert_eq!(
        produced_uint.step_record.after.control,
        Some(Control::Return(Value::UInt(precise_uint)))
    );
    assert_eq!(
        produced_uint.step_record.after.remaining_fuel,
        initial_fuel - 1
    );

    // Check exact 16-byte LE representation of UInt128 in Constants row 0 (lanes 3..19)
    // and inactive padding in Constants row 1.
    let const_row0 = &produced_uint.request.values.program.tables[13][0];
    let const_row1 = &produced_uint.request.values.program.tables[13][1];
    for (byte_idx, &b) in precise_uint.to_le_bytes().iter().enumerate() {
        assert_eq!(const_row0[3 + byte_idx], u128::from(b));
    }
    assert!(const_row1.iter().all(|&v| v == 0));

    // Check 16-byte LE remainingFuel before and after borrow in StateRoot (lanes 9..25).
    for (byte_idx, &b) in initial_fuel.to_le_bytes().iter().enumerate() {
        assert_eq!(produced_uint.request.values.pre.root[9 + byte_idx], u128::from(b));
        assert_eq!(produced_uint.request.values.pre.root[25 + byte_idx], u128::from(b));
        assert_eq!(produced_uint.request.values.post.root[25 + byte_idx], u128::from(b));
    }
    for (byte_idx, &b) in (initial_fuel - 1).to_le_bytes().iter().enumerate() {
        assert_eq!(produced_uint.request.values.post.root[9 + byte_idx], u128::from(b));
    }

    // Pre-state came from encode_observed_a over a trusted Machine snapshot, so Canonical
    // decode is within the layout_a diagnostic contract.
    let state_caps = core_state_capacities(&uint_caps).expect("state caps");
    let decoded_pre = decode_observed_a(
        &produced_uint.prepared,
        &state_caps,
        &AImage {
            profile: PROFILE,
            full_program: &produced_uint.prepared.full_program,
            program: &produced_uint.prepared.block,
            state: &produced_uint.pre_state,
        },
        DecodeMode::Canonical,
    )
    .expect("canonical pre_state decode");
    assert_eq!(decoded_pre, produced_uint.step_record.before);
    assert!(!statement_uint.is_empty());

    // 2. Bool(true) literal.
    let bool_prog = Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(Value::Bool(true))),
    });
    let bool_caps = minimal_literal_capacities(1, 0);
    let produced_bool =
        produce_a102_literal_transition(bool_prog, &bool_caps, 1, 0, &id).expect("Bool transition");
    assert_eq!(produced_bool.selected_constant_ref, ValueRef::constant(0));
    assert_eq!(produced_bool.request.values.program.tables[13][0][0], 1); // kind = Bool
    assert_eq!(produced_bool.request.values.program.tables[13][0][1], 1); // typeId = Bool
    assert_eq!(produced_bool.request.values.program.tables[13][0][19], 1); // boolValue = true
    assert_eq!(produced_bool.request.values.post.root[9], 0); // remainingFuel decremented 1 -> 0

    // 3. Nonempty List literal: postorder places children at 0..4 and root Cons at index 4.
    let list_val = Value::List(vec![42, u128::MAX - 9]);
    let list_prog = Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(list_val.clone())),
    });
    let list_caps = minimal_literal_capacities(5, 4);
    let produced_list = produce_a102_literal_transition(list_prog, &list_caps, 10, 2, &id)
        .expect("List transition");
    assert_eq!(produced_list.prepared.block.constants.active, 5);
    assert_eq!(produced_list.prepared.block.constant_fields.active, 4);
    assert_eq!(produced_list.selected_constant_ref, ValueRef::constant(4));
    assert_eq!(
        produced_list.post_state.control_rows[0],
        ControlRow {
            tag: 1,
            value: ValueRef::constant(4),
            ..ControlRow::default()
        }
    );
    assert_eq!(
        produced_list.request.values.post.tables[5][0][0..4],
        [1, 0, 0, 4]
    );

    // 4. Nested Tree literal: Leaf(7) and Leaf(u128::MAX) under Branch -> 5 Constants, 4 ConstantFields, root at 4.
    let tree_val = Value::Tree(Tree::Branch(
        Arc::new(Tree::Leaf(7)),
        Arc::new(Tree::Leaf(u128::MAX)),
    ));
    let tree_prog = Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(tree_val)),
    });
    let tree_caps = minimal_literal_capacities(5, 4);
    let produced_tree = produce_a102_literal_transition(tree_prog, &tree_caps, 3, 1, &id)
        .expect("Tree transition");
    assert_eq!(produced_tree.selected_constant_ref, ValueRef::constant(4));
    assert_eq!(produced_tree.request.values.program.tables[13][4][0], 2); // kind = Constructor
    assert_eq!(produced_tree.request.values.program.tables[13][4][1], 3); // typeId = Tree
    assert_eq!(produced_tree.request.values.program.tables[13][4][2], 3); // ctorId = Branch
}

#[test]
fn unused_functions_and_all_program_and_state_tables_map_completely() {
    let id = sample_identity();

    // Unused functions exercise Names, NameBytes, Occurrences, OccurrencePaths,
    // Expressions, ExprChildren, Functions, Parameters, Patterns, Arms, Constants, ConstantFields.
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
        // D, DF, E, V, S, C:
        // rich_snapshot below visits `env` (Bool(true) + List([9]) -> 4 Data, 2 DataFields, 2 EnvCells)
        // four times (Push.pendingFrame, Push.environment, Root.environment, FrameSlots[0])
        // plus Invoke(Tree::Leaf(55) -> 2 Data, 1 DataFields, 1 ValueSlots), yielding active [18, 9, 8, 1, 1, 2].
        state: [18, 9, 8, 4, 4, 3],
        stack_limit: 32,
    };

    let produced = produce_a102_literal_transition(program.clone(), &caps, 50, 12, &id)
        .expect("full program with unused functions should produce A102 transition");

    // Complete Program including unused functions is retained in PreparedA and ProgramBlock.
    assert_eq!(produced.prepared.full_program.functions.len(), 2);
    assert_eq!(produced.prepared.block.functions.active, 2);
    assert_eq!(produced.prepared.block.parameters.active, 3);
    assert_eq!(produced.prepared.block.patterns.active, 2);
    assert_eq!(produced.prepared.block.arms.active, 2);

    // Every one of the 15 program tables has active > 0 in this program.
    for table_idx in 0..15 {
        assert!(
            produced.request.values.program.root[table_idx + 1] > 0,
            "program table {table_idx} should have nonzero active rows"
        );
        assert_eq!(
            produced.request.values.program.tables[table_idx].len(),
            caps.program[table_idx]
        );
    }

    // Entry is ExprId 0 (List(vec![9]) -> UInt(9), Nil, Cons(9, Nil) -> root Constant index 2).
    assert_eq!(produced.selected_expr_id, 0);
    assert_eq!(produced.selected_constant_ref, ValueRef::constant(2));
    assert_eq!(produced.request.values.rule_tag, RULE_TAG);
    assert_eq!(produced.request.values.charge, RULE_CHARGE);

    // Verify host statement construction over the produced request; native emission is in literal_witness_native_tests.
    let layout =
        LiteralLayout::new(&caps, &id, HOST_TEST_MAX_STATEMENT_LANES).expect("layout construction");
    let statement = layout
        .statement(&produced.request.values)
        .expect("statement validation");
    assert_eq!(statement.len(), layout.statement_len());

    // Also test state_block_to_image_values on a StateBlock with non-empty Data, DataFields,
    // EnvCells, ValueSlots, FrameSlots, and multi-row ControlRows (Push + Invoke).
    let mut env = Environment::new();
    env.insert("flag".to_string(), Value::Bool(true));
    env.insert("xs".to_string(), Value::List(vec![9]));
    let rich_snapshot = Snapshot {
        program: program.clone(),
        control: Some(Control::Push(
            Frame::Let(
                "sum".to_string(),
                Arc::new(Expr::Var("sum".to_string())),
                env.clone(),
            ),
            Box::new(Control::Invoke(
                "helper_tree".to_string(),
                vec![Value::Tree(Tree::Leaf(55))],
            )),
            env.clone(),
        )),
        environment: env.clone(),
        frames: vec![Frame::Return(Type::UInt, env)],
        stack_limit: 12,
        remaining_fuel: 40,
        initial_fuel: 50,
        outcome: Outcome::Pending,
    };
    let state_caps = core_state_capacities(&caps).expect("state caps");
    let rich_state_block =
        encode_observed_a(&produced.prepared, &state_caps, &rich_snapshot).expect("encode rich state");
    let rich_state_image =
        state_block_to_image_values(&rich_state_block, &caps).expect("map rich StateBlock");

    // All 6 state tables have active > 0 in rich_state_image.
    for state_table_idx in 0..6 {
        assert!(
            rich_state_image.root[state_table_idx] > 0,
            "state table {state_table_idx} should have nonzero active rows"
        );
        assert_eq!(
            rich_state_image.tables[state_table_idx].len(),
            caps.state[state_table_idx]
        );
    }
    assert_eq!(&rich_state_image.root[..6], &[18, 9, 8, 1, 1, 2]);
    // ControlRows[0] is Push (tag 2) with inline pendingFrame Let (tag 0 at lane 4).
    assert_eq!(rich_state_image.tables[5][0][0], 2);
    assert_eq!(rich_state_image.tables[5][0][4], 0);
    // ControlRows[1] is Invoke (tag 3).
    assert_eq!(rich_state_image.tables[5][1][0], 3);
}

#[test]
fn unsupported_domains_and_invalid_padding_or_capacities_are_refused() {
    let id = sample_identity();
    let caps = LiteralCapacities {
        program: [2, 8, 4, 4, 4, 4, 0, 0, 0, 0, 4, 4, 5, 2, 0],
        state: [1, 0, 1, 0, 1, 2],
        stack_limit: 4,
    };

    // 1. Non-literal entry (Expr::Binary) is refused as OutsideObservationDomain.
    let binary_prog = Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Binary(
            BinaryOp::Add,
            Arc::new(Expr::Literal(Value::UInt(1))),
            Arc::new(Expr::Literal(Value::UInt(2))),
        )),
    });
    let err = build_literal_transition_request(binary_prog, &caps, 5, 2, &id)
        .expect_err("non-literal entry must be refused");
    assert!(matches!(err, StateFailure::OutsideObservationDomain { .. }));

    // 2. Zero initial_fuel (exhaustion rule 84 instead of rule 102) is refused.
    let lit_prog = Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(Value::UInt(7))),
    });
    let err = build_literal_transition_request(lit_prog.clone(), &caps, 0, 2, &id)
        .expect_err("zero initial fuel must be refused for A102");
    assert!(matches!(err, StateFailure::OutsideObservationDomain { .. }));

    // 3. stack_limit exceeding capacities.stack_limit is refused as Capacity.
    let err = build_literal_transition_request(lit_prog.clone(), &caps, 5, 9, &id)
        .expect_err("stack_limit > capacities.stack_limit must be refused");
    assert!(matches!(
        err,
        StateFailure::Capacity {
            table: "Root.stackLimit",
            ..
        }
    ));

    // 4a. With K=2 and KF=0 in `caps`, List([1, 2]) first reserves 1 root Constants row
    // (1 <= 2) and then in Task::Visit(Input::List) reserves 2 ConstantFields slots (2 > 0)
    // before scheduling child Constants, so prepare_a refuses on ConstantFields first.
    let big_list_prog = Arc::new(Program {
        functions: vec![],
        entry: Arc::new(Expr::Literal(Value::List(vec![1, 2]))),
    });
    let err = build_literal_transition_request(big_list_prog.clone(), &caps, 5, 2, &id)
        .expect_err("List([1, 2]) must fail on ConstantFields when K=2, KF=0");
    assert!(matches!(
        err,
        StateFailure::Capacity {
            table: "ConstantFields",
            needed: 2,
            limit: 0,
            ..
        }
    ));

    // 4b. With sufficient explicit KF=4 and K=2, ConstantFields(2) succeeds on the outer
    // Cons visit and prepare_a refuses on Constants (needed=3, limit=2) when scheduling
    // the second child of the outer Cons.
    let mut kf_sufficient_caps = caps.clone();
    kf_sufficient_caps.program[14] = 4;
    let err = build_literal_transition_request(big_list_prog.clone(), &kf_sufficient_caps, 5, 2, &id)
        .expect_err("List([1, 2]) must fail on Constants when K=2, KF=4");
    assert!(matches!(
        err,
        StateFailure::Capacity {
            table: "Constants",
            needed: 3,
            limit: 2,
            ..
        }
    ));

    // 4c. Combined-invalid precedence (C1): cheap entry/stack/fuel checks run before prepare_a.
    let err = build_literal_transition_request(big_list_prog, &caps, 0, 2, &id)
        .expect_err("zero initial_fuel check must precede prepare_a capacity refusal");
    assert!(matches!(err, StateFailure::OutsideObservationDomain { .. }));

    // 5. Non-zero inactive padding row in ProgramBlock or StateBlock is refused as WrongRepresentation.
    let prog_caps = core_program_capacities(&caps).expect("prog caps");
    let prepared = prepare_a(lit_prog, prog_caps).expect("prepare");
    let mut dirty_program = prepared.block.clone();
    dirty_program.constants.rows[1].bool_value = true; // row 1 is inactive (active == 1)
    let err = program_block_to_image_values(&dirty_program, &caps)
        .expect_err("dirty inactive program row must be refused");
    assert!(matches!(
        err,
        StateFailure::WrongRepresentation {
            block: "ProgramBlock",
            table: "Constants",
            row: 1,
            ..
        }
    ));

    let mut dirty_state = produce_a102_literal_transition(
        prepared.full_program.clone(),
        &caps,
        5,
        2,
        &id,
    )
    .expect("valid transition")
    .post_state;
    dirty_state.data[0] = DataNode {
        bool_value: true,
        ..DataNode::default()
    }; // active_data == 0, so row 0 is inactive
    let err = state_block_to_image_values(&dirty_state, &caps)
        .expect_err("dirty inactive state row must be refused");
    assert!(matches!(
        err,
        StateFailure::WrongRepresentation {
            block: "StateBlock",
            table: "Data",
            row: 0,
            ..
        }
    ));

    // 6. Schema minimum-shape check refuses K == 0.
    let bad_caps = LiteralCapacities {
        program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 0, 0],
        state: [0, 0, 0, 0, 0, 1],
        stack_limit: 1,
    };
    assert!(literal_schema(&bad_caps).is_err());
    assert!(matches!(
        program_block_to_image_values(&prepared.block, &bad_caps),
        Err(StateFailure::WrongRepresentation { .. })
    ));

    // 7. build_checked_literal_transition_request enforces max_statement_lanes via
    // LiteralLayout::new before running produce_a102_literal_transition (even with initial_fuel=0).
    let limit_err = build_checked_literal_transition_request(
        prepared.full_program.clone(),
        &caps,
        0,
        2,
        &id,
        1,
    )
    .expect_err("statement lane count above explicit max_statement_lanes must be refused first");
    assert_eq!(limit_err.code, "mori::backend::limit");
}
