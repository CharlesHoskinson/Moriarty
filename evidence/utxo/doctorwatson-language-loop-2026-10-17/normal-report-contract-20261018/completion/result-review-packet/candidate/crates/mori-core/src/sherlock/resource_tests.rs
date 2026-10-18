use super::reference::Machine;
use super::reference_types::*;
use std::sync::Arc;

fn program(functions: Vec<Function>, entry: Expr) -> Arc<Program> {
    Arc::new(Program {
        functions,
        entry: Arc::new(entry),
    })
}

fn collect_trace(
    prog: Arc<Program>,
    fuel: u128,
    stack_limit: usize,
) -> (Vec<(usize, u8)>, Snapshot) {
    let mut machine = Machine::new(prog, fuel, stack_limit);
    let mut steps = Vec::new();
    while let Some(record) = machine.step() {
        assert!(record.before.remaining_fuel <= record.before.initial_fuel);
        assert!(record.after.remaining_fuel <= record.after.initial_fuel);
        assert_eq!(
            record.after.checked_charged_steps(),
            Some(record.after.charged_steps())
        );
        steps.push((record.rule_start_line, record.charge));
    }
    let final_snap = machine.snapshot();
    assert_eq!(
        final_snap.checked_charged_steps(),
        Some(final_snap.charged_steps())
    );
    (steps, final_snap)
}

#[test]
fn outcome_is_resource_limit_matches_frozen_a1_partition() {
    assert!(Outcome::Exhausted.is_resource_limit());
    assert!(Outcome::Rejected(4).is_resource_limit());

    assert!(!Outcome::Pending.is_resource_limit());
    assert!(!Outcome::Done(Value::UInt(0)).is_resource_limit());
    assert!(!Outcome::Done(Value::Bool(true)).is_resource_limit());
    assert!(!Outcome::Done(Value::List(vec![1, 2])).is_resource_limit());
    assert!(!Outcome::Done(Value::Tree(Tree::Leaf(9))).is_resource_limit());

    for code in 0..=u8::MAX {
        assert_eq!(
            Outcome::Rejected(code).is_resource_limit(),
            code == 4,
            "unexpected A1 classification for Rejected({code})"
        );
    }
}

#[test]
fn frame_cap_boundary_off_by_one_and_error4_publication() {
    // Let("0", Unary(Not, Bool(true)), Var("0")) requires depth 1 for Let and depth 2 for Unary.
    let nested = program(
        vec![],
        Expr::Let(
            "0".into(),
            Arc::new(Expr::Unary(
                UnaryOp::Not,
                Arc::new(Expr::Literal(Value::Bool(true))),
            )),
            Arc::new(Expr::Var("0".into())),
        ),
    );

    // stack_limit = 0: first Push fails at depth 0 == 0 via rule 140 -> 82.
    let (steps_0, snap_0) = collect_trace(nested.clone(), 10, 0);
    assert_eq!(steps_0, vec![(114, 1), (140, 1), (82, 0)]);
    assert_eq!(snap_0.outcome, Outcome::Rejected(4));
    assert!(snap_0.outcome.is_resource_limit());
    assert_eq!(snap_0.depth(), 0);
    assert_eq!(snap_0.charged_steps(), 2);

    // stack_limit = 1: first Push succeeds (depth 1), second Push fails at depth 1 == 1.
    let (steps_1, snap_1) = collect_trace(nested.clone(), 10, 1);
    assert_eq!(
        steps_1,
        vec![(114, 1), (135, 1), (120, 1), (140, 1), (82, 0)]
    );
    assert_eq!(snap_1.outcome, Outcome::Rejected(4));
    assert!(snap_1.outcome.is_resource_limit());
    assert_eq!(snap_1.depth(), 1);
    assert_eq!(snap_1.charged_steps(), 4);

    // stack_limit = 2: both Pushes succeed (reaching depth 2) and run finishes with Done(Bool(false)).
    let (steps_2, snap_2) = collect_trace(nested, 10, 2);
    assert_eq!(
        steps_2,
        vec![
            (114, 1),
            (135, 1),
            (120, 1),
            (135, 1),
            (102, 1),
            (157, 1),
            (223, 1),
            (145, 1),
            (108, 1),
            (195, 1),
            (80, 0),
        ]
    );
    assert_eq!(snap_2.outcome, Outcome::Done(Value::Bool(false)));
    assert!(!snap_2.outcome.is_resource_limit());
    assert_eq!(snap_2.depth(), 0);
    assert_eq!(snap_2.charged_steps(), 10);
}

#[test]
fn exhaustion_and_zero_fuel_publication_precedence() {
    let lit = program(vec![], Expr::Literal(Value::UInt(7)));

    // Non-terminal Eval at 0 fuel publishes Exhausted (rule 84, charge 0).
    let (steps_ex, snap_ex) = collect_trace(lit.clone(), 0, 8);
    assert_eq!(steps_ex, vec![(84, 0)]);
    assert_eq!(snap_ex.outcome, Outcome::Exhausted);
    assert!(snap_ex.outcome.is_resource_limit());
    assert_eq!(snap_ex.charged_steps(), 0);

    // Terminal Return with empty stack at 0 remaining fuel publishes Done (rule 80 precedes 84).
    let (steps_done, snap_done) = collect_trace(lit, 1, 8);
    assert_eq!(steps_done, vec![(102, 1), (80, 0)]);
    assert_eq!(snap_done.outcome, Outcome::Done(Value::UInt(7)));
    assert!(!snap_done.outcome.is_resource_limit());
    assert_eq!(snap_done.charged_steps(), 1);

    // Error(3) at 0 remaining fuel publishes Rejected(3) (rule 82 precedes 84).
    let unbound = program(vec![], Expr::Var("0".into()));
    let (steps_err3, snap_err3) = collect_trace(unbound, 1, 8);
    assert_eq!(steps_err3, vec![(111, 1), (82, 0)]);
    assert_eq!(snap_err3.outcome, Outcome::Rejected(3));
    assert!(!snap_err3.outcome.is_resource_limit());
    assert_eq!(snap_err3.charged_steps(), 1);

    // Error(4) at 0 remaining fuel publishes Rejected(4), which is still a resource limit.
    let push_fail = program(
        vec![],
        Expr::Unary(UnaryOp::Not, Arc::new(Expr::Literal(Value::Bool(true)))),
    );
    let (steps_err4, snap_err4) = collect_trace(push_fail, 2, 0);
    assert_eq!(steps_err4, vec![(120, 1), (140, 1), (82, 0)]);
    assert_eq!(snap_err4.outcome, Outcome::Rejected(4));
    assert!(snap_err4.outcome.is_resource_limit());
    assert_eq!(snap_err4.charged_steps(), 2);
}

#[test]
fn all_non_resource_k_a_rejections_classified_false() {
    // Error 3: unbound variable (111 -> 82).
    let (s3, r3) = collect_trace(program(vec![], Expr::Var("0".into())), 10, 8);
    assert_eq!(s3, vec![(111, 1), (82, 0)]);
    assert_eq!(r3.outcome, Outcome::Rejected(3));
    assert!(!r3.outcome.is_resource_limit());

    // Error 5: non-boolean If condition retains frame (123 -> 135 -> 102 -> 169 -> 82).
    let (s5, r5) = collect_trace(
        program(
            vec![],
            Expr::If(
                Arc::new(Expr::Literal(Value::UInt(1))),
                Arc::new(Expr::Literal(Value::UInt(2))),
                Arc::new(Expr::Literal(Value::UInt(3))),
            ),
        ),
        10,
        8,
    );
    assert_eq!(s5, vec![(123, 1), (135, 1), (102, 1), (169, 1), (82, 0)]);
    assert_eq!(r5.outcome, Outcome::Rejected(5));
    assert_eq!(r5.depth(), 1);
    assert!(!r5.outcome.is_resource_limit());

    // Error 6: return type mismatch retains Return frame (129->200->204->208->135->102->190->82).
    let fn_bad_ret = Function {
        name: "0".into(),
        parameters: vec![],
        return_type: Type::UInt,
        body: Arc::new(Expr::Literal(Value::Bool(true))),
    };
    let (s6, r6) = collect_trace(
        program(vec![fn_bad_ret], Expr::Call("0".into(), vec![])),
        10,
        8,
    );
    assert_eq!(
        s6,
        vec![
            (129, 1),
            (200, 1),
            (204, 1),
            (208, 1),
            (135, 1),
            (102, 1),
            (190, 1),
            (82, 0)
        ]
    );
    assert_eq!(r6.outcome, Outcome::Rejected(6));
    assert_eq!(r6.depth(), 1);
    assert!(!r6.outcome.is_resource_limit());

    // Error 7: missing function (129 -> 200 -> 202 -> 82).
    let (s7, r7) = collect_trace(program(vec![], Expr::Call("0".into(), vec![])), 10, 8);
    assert_eq!(s7, vec![(129, 1), (200, 1), (202, 1), (82, 0)]);
    assert_eq!(r7.outcome, Outcome::Rejected(7));
    assert!(!r7.outcome.is_resource_limit());

    // Error 8: binding failure on duplicate parameter names (131->135->102->177->102->181->200->204->206->82).
    let fn_dup_param = Function {
        name: "0".into(),
        parameters: vec![
            Parameter {
                name: "1".into(),
                ty: Type::UInt,
            },
            Parameter {
                name: "1".into(),
                ty: Type::UInt,
            },
        ],
        return_type: Type::UInt,
        body: Arc::new(Expr::Literal(Value::UInt(0))),
    };
    let (s8, r8) = collect_trace(
        program(
            vec![fn_dup_param],
            Expr::Call(
                "0".into(),
                vec![
                    Arc::new(Expr::Literal(Value::UInt(1))),
                    Arc::new(Expr::Literal(Value::UInt(2))),
                ],
            ),
        ),
        12,
        8,
    );
    assert_eq!(
        s8,
        vec![
            (131, 1),
            (135, 1),
            (102, 1),
            (177, 1),
            (102, 1),
            (181, 1),
            (200, 1),
            (204, 1),
            (206, 1),
            (82, 0)
        ]
    );
    assert_eq!(r8.outcome, Outcome::Rejected(8));
    assert!(!r8.outcome.is_resource_limit());

    // Error 9: match exhaustion after equal-name Cons binder miss (126->135->102->173->214->217->212->82).
    let (s9, r9) = collect_trace(
        program(
            vec![],
            Expr::Match(
                Arc::new(Expr::Literal(Value::List(vec![1]))),
                vec![Arm {
                    pattern: Pattern::Cons("0".into(), "0".into()),
                    body: Arc::new(Expr::Literal(Value::UInt(1))),
                }],
            ),
        ),
        10,
        8,
    );
    assert_eq!(
        s9,
        vec![
            (126, 1),
            (135, 1),
            (102, 1),
            (173, 1),
            (214, 1),
            (217, 1),
            (212, 1),
            (82, 0)
        ]
    );
    assert_eq!(r9.outcome, Outcome::Rejected(9));
    assert!(!r9.outcome.is_resource_limit());

    // Error 10: checked arithmetic underflow (117->135->102->149->102->153->225->82).
    let (s10, r10) = collect_trace(
        program(
            vec![],
            Expr::Binary(
                BinaryOp::Sub,
                Arc::new(Expr::Literal(Value::UInt(0))),
                Arc::new(Expr::Literal(Value::UInt(1))),
            ),
        ),
        10,
        8,
    );
    assert_eq!(
        s10,
        vec![
            (117, 1),
            (135, 1),
            (102, 1),
            (149, 1),
            (102, 1),
            (153, 1),
            (225, 1),
            (82, 0)
        ]
    );
    assert_eq!(r10.outcome, Outcome::Rejected(10));
    assert!(!r10.outcome.is_resource_limit());

    // Error 11: unary operator sort mismatch (120->135->102->157->225->82).
    let (s11, r11) = collect_trace(
        program(
            vec![],
            Expr::Unary(UnaryOp::Not, Arc::new(Expr::Literal(Value::UInt(1)))),
        ),
        10,
        8,
    );
    assert_eq!(
        s11,
        vec![(120, 1), (135, 1), (102, 1), (157, 1), (225, 1), (82, 0)]
    );
    assert_eq!(r11.outcome, Outcome::Rejected(11));
    assert!(!r11.outcome.is_resource_limit());
}

#[test]
fn snapshot_counter_invariant_and_checked_charged_steps() {
    let prog = program(vec![], Expr::Literal(Value::UInt(1)));
    let mut machine = Machine::new(prog.clone(), 3, 4);
    assert_eq!(machine.snapshot().checked_charged_steps(), Some(0));
    let _ = machine.run();
    assert_eq!(machine.snapshot().checked_charged_steps(), Some(1));

    // Hand-built invalid snapshot with remaining_fuel > initial_fuel must return None, never Some(0).
    let invalid = Snapshot {
        program: prog,
        control: None,
        environment: Environment::new(),
        frames: vec![],
        stack_limit: 4,
        remaining_fuel: 6,
        initial_fuel: 5,
        outcome: Outcome::Pending,
    };
    assert_eq!(invalid.checked_charged_steps(), None);
}
