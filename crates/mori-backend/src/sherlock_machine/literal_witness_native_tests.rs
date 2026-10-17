//! Native circuit-emission test separated from the SDK-free host normal test module.
//! Retains the `local_max_statement_lanes()` and `LiteralLayout::emit(CircuitMode::LocalPublic)`
//! check on the produced A102 transition request without weakening or running native proving.

use crate::sherlock_machine::literal_bridge::{
    local_max_statement_lanes, CircuitMode, LiteralLayout, LiteralSourceIdentity,
};
use crate::sherlock_machine::literal_step::LiteralCapacities;
use crate::sherlock_machine::literal_witness::{
    build_checked_literal_transition_request, produce_a102_literal_transition,
};
use mori_core::sherlock::{
    Arm, BinaryOp, Expr, Function, Parameter, Pattern, Program, Tree, Type, UnaryOp, Value,
    K_SOURCE_SHA256,
};
use std::sync::Arc;

#[test]
fn produced_a102_request_emits_local_public_zkir_within_local_verifier_lane_bound() {
    let id = LiteralSourceIdentity {
        k_source_sha256: K_SOURCE_SHA256.to_string(),
        reference_source_sha256: "11".repeat(32),
        program_source_sha256: "22".repeat(32),
    };
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

    // Verify statement construction and ZKIR emission over the produced request.
    let layout =
        LiteralLayout::new(&caps, &id, local_max_statement_lanes()).expect("layout construction");
    let statement = layout
        .statement(&produced.request.values)
        .expect("statement validation");
    assert_eq!(statement.len(), layout.statement_len());
    let (_ir, zkir) = layout
        .emit(CircuitMode::LocalPublic)
        .expect("local circuit emission");
    assert!(!zkir.is_empty());

    let (_, checked_statement) = build_checked_literal_transition_request(
        program,
        &caps,
        50,
        12,
        &id,
        local_max_statement_lanes(),
    )
    .expect("checked request within local_max_statement_lanes");
    assert_eq!(checked_statement, statement);
}
