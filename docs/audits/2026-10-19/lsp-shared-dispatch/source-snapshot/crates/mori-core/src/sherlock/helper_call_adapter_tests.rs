//! Child module of reference_oracles_tests: reuse its exact frozen-JSON consumer.
//! All Invoke inputs below are hand-transcribed, never obtained by stepping an
//! evaluator. No expected boundary is calculated by this adapter or Machine.
use super::super::helper_call_adapter as adapter;
use super::super::program_image::{ExprId, SourceLocator, SourceOwner, prepare_program_image};
use super::*;

fn check(actual: &Snapshot, case: &ExpectedCase, index: usize, context: &str) {
    let expected = &case.states[index];
    compare_snapshot(
        actual,
        case,
        expected,
        case.initial_fuel,
        expected.remaining_fuel,
        expected.control,
        case.pending_output,
        context,
    );
}

#[allow(clippy::too_many_arguments)]
fn frozen_call(
    case_id: &str,
    start: usize,
    name: &str,
    values: Vec<Value>,
    environment: Environment,
    frames: Vec<Frame>,
    owner: SourceOwner,
    path: Vec<usize>,
    expected_work: (u128, u128, u128),
) {
    let case = CASES
        .iter()
        .chain(EXTRA_CASES.iter())
        .find(|case| case.case_id == case_id)
        .unwrap();
    let program = normal_program(case_id);
    let before = Snapshot {
        program: program.clone(),
        control: Some(Control::Invoke(name.into(), values.clone())),
        environment: environment.clone(),
        frames: frames.clone(),
        stack_limit: case.stack_limit,
        remaining_fuel: case.states[start].remaining_fuel,
        initial_fuel: case.initial_fuel,
        outcome: Outcome::Pending,
    };
    check(&before, case, start, "independent typed Invoke input");
    let image = Arc::new(prepare_program_image(&program));
    let locator = SourceLocator {
        owner,
        ast_path: path,
    };
    // Explicit source-origin association, not an inferred dynamic trace origin.
    let origin = ExprId(
        image
            .source_map
            .iter()
            .position(|source| source == &locator)
            .unwrap(),
    );
    let mut context = adapter::CallContext::new(image.clone(), before.clone(), origin).unwrap();
    assert_eq!(context.proposed_profile(), "sherlock-helper-microstep/1");
    assert!(Arc::ptr_eq(context.resolve_program(0).unwrap(), &program));
    assert_eq!(context.resolve_source(0).unwrap(), image.as_ref());
    assert_eq!(context.resolve_origin(0).unwrap(), &(origin, locator));
    assert_eq!(context.resolve_environment(0).unwrap(), &environment);
    assert_eq!(
        context
            .resolve_frames(adapter::helper::Slice {
                start: 0,
                len: frames.len()
            })
            .unwrap(),
        frames.as_slice()
    );
    for (index, value) in values.iter().enumerate() {
        assert_eq!(
            context
                .resolve_value(adapter::helper::ValueRef(index))
                .unwrap(),
            value
        );
    }
    let mut completions = 0;
    let mut events = 0;
    while let Some(event) = context.step().unwrap() {
        events += 1;
        assert!(
            events <= expected_work.0,
            "no event beyond the independent literal count"
        );
        assert_eq!(event.helper_event.work_delta, 1);
        if let Some(record) = event.reference_completion {
            check(
                &record.before,
                case,
                start + completions,
                "complete pre-boundary",
            );
            completions += 1;
            check(
                &record.after,
                case,
                start + completions,
                "complete post-boundary",
            );
            assert_eq!(record.charge, 1);
            assert_eq!(
                record.rule_start_line,
                if completions == 1 { 200 } else { 204 }
            );
            assert_eq!(
                Some(record.rule_start_line),
                case.states[start + completions].via_rule_start_line
            );
            assert_eq!(
                record.after.checked_charged_steps(),
                Some(case.states[start + completions].charged_steps)
            );
        }
        // Every helper prefix erases to the current unchanged A boundary;
        // H never reduces A fuel or enters A charged_steps().
        check(
            &context.snapshot().unwrap(),
            case,
            start + completions,
            "helper prefix erasure",
        );
        check(
            &context.resume_snapshot().unwrap(),
            case,
            if completions == 0 { start } else { start + 1 },
            "full saved resume",
        );
    }
    assert_eq!(completions, 2);
    assert_eq!(
        context.work(),
        adapter::helper::Work {
            consumed: expected_work.0,
            helper: expected_work.1,
            helper_visits: expected_work.2,
            reference_cursor: case.states[start + 2].charged_steps,
        }
    );
    assert!(context.resolve_result_slot(0).unwrap().is_some());
    assert!(context.step().unwrap().is_none());
}

#[test]
fn frozen_n05_outer_and_recursive_bool_bindings() {
    frozen_call(
        "N05",
        4,
        "11",
        vec![Value::Bool(true)],
        Environment::new(),
        vec![],
        SourceOwner::Entry,
        vec![],
        (12, 10, 4),
    );
    frozen_call(
        "N05",
        16,
        "11",
        vec![Value::Bool(false)],
        Environment::from([("0".into(), Value::Bool(true))]),
        vec![Frame::Return(Type::UInt, Environment::new())],
        SourceOwner::Function(0),
        vec![1],
        (12, 10, 4),
    );
}

#[test]
fn frozen_n07_three_real_values_through_ordered_parameter_table() {
    // One source function means one row visit, hence 23 events / H21 / visits13.
    frozen_call(
        "N07",
        8,
        "17",
        vec![Value::UInt(1), Value::UInt(2), Value::UInt(3)],
        Environment::new(),
        vec![],
        SourceOwner::Entry,
        vec![],
        (23, 21, 13),
    );
}

#[test]
fn frozen_n08_preserves_both_pending_multiplications_and_all_caller_scopes() {
    frozen_call(
        "N08",
        4,
        "18",
        vec![Value::UInt(2)],
        Environment::new(),
        vec![],
        SourceOwner::Entry,
        vec![],
        (12, 10, 4),
    );
    frozen_call(
        "N08",
        32,
        "18",
        vec![Value::UInt(1)],
        Environment::from([("0".into(), Value::UInt(2))]),
        vec![
            Frame::Right(BinaryOp::Mul, Value::UInt(2)),
            Frame::Return(Type::UInt, Environment::new()),
        ],
        SourceOwner::Function(0),
        vec![2, 1],
        (12, 10, 4),
    );
    frozen_call(
        "N08",
        60,
        "18",
        vec![Value::UInt(0)],
        Environment::from([("0".into(), Value::UInt(1))]),
        vec![
            Frame::Right(BinaryOp::Mul, Value::UInt(1)),
            Frame::Return(
                Type::UInt,
                Environment::from([("0".into(), Value::UInt(2))]),
            ),
            Frame::Right(BinaryOp::Mul, Value::UInt(2)),
            Frame::Return(Type::UInt, Environment::new()),
        ],
        SourceOwner::Function(0),
        vec![2, 1],
        (12, 10, 4),
    );
}

#[test]
fn real_list_tree_values_and_saved_scope_are_resolved_without_reconstruction() {
    // Additional ordinary typed-host specimen, independent explicit expected
    // snapshots. Existing frozen JSON named-call fixtures cover UInt and Bool.
    let leaf = Arc::new(Tree::Leaf(4));
    let arguments = vec![
        Value::List(vec![4, 5]),
        Value::Tree(Tree::Branch(leaf.clone(), leaf)),
    ];
    let function = Function {
        name: "9".into(),
        parameters: vec![
            Parameter {
                name: "10".into(),
                ty: Type::List,
            },
            Parameter {
                name: "11".into(),
                ty: Type::Tree,
            },
        ],
        return_type: Type::List,
        body: Arc::new(Expr::Var("10".into())),
    };
    let program = Arc::new(Program {
        functions: vec![function.clone()],
        entry: Arc::new(Expr::Call(
            "9".into(),
            vec![
                Arc::new(Expr::Literal(Value::List(vec![4, 5]))),
                Arc::new(Expr::Literal(Value::Tree(Tree::Branch(
                    Arc::new(Tree::Leaf(4)),
                    Arc::new(Tree::Leaf(4)),
                )))),
            ],
        )),
    });
    let caller = Environment::from([
        ("10".into(), Value::UInt(99)),
        ("saved".into(), Value::Tree(Tree::Leaf(9))),
    ]);
    let frames = vec![Frame::Scope(Environment::from([(
        "outer".into(),
        Value::Bool(true),
    )]))];
    let before = Snapshot {
        program: program.clone(),
        control: Some(Control::Invoke("9".into(), arguments)),
        environment: caller,
        frames,
        stack_limit: 8,
        remaining_fuel: 10,
        initial_fuel: 12,
        outcome: Outcome::Pending,
    };
    let expected_enter = Snapshot {
        control: Some(Control::Enter(
            Some(function.clone()),
            vec![
                Value::List(vec![4, 5]),
                Value::Tree(Tree::Branch(
                    Arc::new(Tree::Leaf(4)),
                    Arc::new(Tree::Leaf(4)),
                )),
            ],
        )),
        remaining_fuel: 9,
        ..before.clone()
    };
    let expected_bound = Snapshot {
        control: Some(Control::Bound(
            Some(Environment::from([
                ("10".into(), Value::List(vec![4, 5])),
                (
                    "11".into(),
                    Value::Tree(Tree::Branch(
                        Arc::new(Tree::Leaf(4)),
                        Arc::new(Tree::Leaf(4)),
                    )),
                ),
            ])),
            Type::List,
            Arc::new(Expr::Var("10".into())),
        )),
        remaining_fuel: 8,
        ..before.clone()
    };
    let image = Arc::new(prepare_program_image(&program));
    let mut context =
        adapter::CallContext::new(image.clone(), before.clone(), image.entry).unwrap();
    let mut records = Vec::new();
    while let Some(event) = context.step().unwrap() {
        if let Some(record) = event.reference_completion {
            records.push(record);
        }
    }
    assert_eq!(
        records,
        vec![
            StepRecord {
                rule_start_line: 200,
                charge: 1,
                before: before.clone(),
                after: expected_enter.clone()
            },
            StepRecord {
                rule_start_line: 204,
                charge: 1,
                before: expected_enter,
                after: expected_bound.clone()
            }
        ]
    );
    assert_eq!(context.snapshot().unwrap(), expected_bound);
    assert_eq!(
        context.work(),
        adapter::helper::Work {
            consumed: 17,
            helper: 15,
            helper_visits: 8,
            reference_cursor: 4
        }
    );
}
