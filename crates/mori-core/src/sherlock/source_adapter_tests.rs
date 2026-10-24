//! Source-static projection tests and fixed scalar evaluation compatibility checks.
use super::*;

const WEIGHTED: &str = include_str!("source_adapter_fixtures/weighted-total.mori");
const THRESHOLD: &str = include_str!("source_adapter_fixtures/threshold-check.mori");

fn e(value: Expr) -> ExprRef {
    Arc::new(value)
}
fn var(name: &str) -> ExprRef {
    e(Expr::Var(name.into()))
}
fn uint(value: u128) -> ExprRef {
    e(Expr::Literal(Value::UInt(value)))
}
fn binary(op: BinaryOp, left: ExprRef, right: ExprRef) -> ExprRef {
    e(Expr::Binary(op, left, right))
}

#[test]
fn weighted_source_retains_dynamic_input_order_and_exact_body() {
    let source = crate::parse(WEIGHTED).unwrap();
    let mapped = prepare_scalar_action(&source, 0).unwrap();
    assert_eq!(
        mapped.function,
        Function {
            name: "quote".into(),
            parameters: vec![
                Parameter {
                    name: "units".into(),
                    ty: Type::UInt
                },
                Parameter {
                    name: "price".into(),
                    ty: Type::UInt
                },
            ],
            return_type: Type::UInt,
            body: e(Expr::Let(
                "gross".into(),
                binary(BinaryOp::Mul, var("units"), var("price")),
                e(Expr::Let(
                    "result".into(),
                    binary(BinaryOp::Add, var("gross"), uint(3)),
                    var("result")
                ))
            )),
        }
    );
    assert_eq!(
        mapped
            .report
            .inputs
            .iter()
            .map(|input| input.index)
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
    assert_eq!(
        mapped
            .report
            .lets
            .iter()
            .map(|site| (site.statement_index, site.expression))
            .collect::<Vec<_>>(),
        vec![(0, 2), (1, 5)]
    );
    assert_eq!(
        (
            mapped.report.emit.statement_index,
            mapped.report.emit.expression
        ),
        (2, 6)
    );
    assert_eq!(mapped.report.emit.name, "total");
    let actual: Vec<_> = mapped
        .report
        .occurrences
        .iter()
        .map(|site| {
            (
                site.statement_index,
                site.expression,
                site.body_path.clone(),
            )
        })
        .collect();
    assert_eq!(
        actual,
        vec![
            (0, None, vec![]),
            (0, Some(2), vec![0]),
            (0, Some(0), vec![0, 0]),
            (0, Some(1), vec![0, 1]),
            (1, None, vec![1]),
            (1, Some(5), vec![1, 0]),
            (1, Some(3), vec![1, 0, 0]),
            (1, Some(4), vec![1, 0, 1]),
            (2, Some(6), vec![1, 1]),
        ]
    );
    assert!(
        mapped
            .report
            .occurrences
            .iter()
            .all(|site| site.action_index == 0)
    );
}

#[test]
fn threshold_source_preserves_predicate_type_and_unary_occurrence() {
    let source = crate::parse(THRESHOLD).unwrap();
    let mapped = prepare_scalar_action(&source, 0).unwrap();
    assert_eq!(
        mapped.function,
        Function {
            name: "inspect".into(),
            parameters: vec![
                Parameter {
                    name: "observed".into(),
                    ty: Type::UInt
                },
                Parameter {
                    name: "threshold".into(),
                    ty: Type::UInt
                },
            ],
            return_type: Type::Bool,
            body: e(Expr::Let(
                "below".into(),
                binary(BinaryOp::Lt, var("observed"), var("threshold")),
                e(Expr::Let(
                    "result".into(),
                    e(Expr::Unary(UnaryOp::Not, var("below"))),
                    var("result")
                ))
            )),
        }
    );
    assert_eq!(mapped.report.return_type, crate::Type::Bool);
    assert_eq!(mapped.report.emit.name, "atOrAbove");
    let actual: Vec<_> = mapped
        .report
        .occurrences
        .iter()
        .map(|site| {
            (
                site.statement_index,
                site.expression,
                site.body_path.clone(),
            )
        })
        .collect();
    assert_eq!(
        actual,
        vec![
            (0, None, vec![]),
            (0, Some(2), vec![0]),
            (0, Some(0), vec![0, 0]),
            (0, Some(1), vec![0, 1]),
            (1, None, vec![1]),
            (1, Some(4), vec![1, 0]),
            (1, Some(3), vec![1, 0, 0]),
            (2, Some(5), vec![1, 1]),
        ]
    );
}

#[test]
fn report_serializes_actual_mapping_and_explicit_coverage() {
    for specimen in [WEIGHTED, THRESHOLD] {
        let source = crate::parse(specimen).unwrap();
        let report = source_projection_report(&source);
        let json = serde_json::to_value(&report).unwrap();
        assert_eq!(json["format"], FORMAT);
        assert_eq!(json["source_language"], crate::LANGUAGE);
        assert_eq!(json["source_semantics"], crate::SEMANTICS);
        assert_eq!(json["reference_profile"], PROFILE);
        assert_eq!(json["evaluation_performed"], false);
        assert_eq!(json["source_function_support"], false);
        assert_eq!(json["source_branch_support"], false);
        assert_eq!(json["source_recursion_support"], false);
        assert_eq!(json["source_data_support"], false);
        assert_eq!(json["actions"][0]["status"], "prepared");
        assert_eq!(
            json["actions"][0]["report"]["inputs"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        assert_eq!(
            json["actions"][0]["report"]["lets"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
        assert!(
            !json["actions"][0]["report"]["occurrences"]
                .as_array()
                .unwrap()
                .is_empty()
        );
    }
}

#[test]
fn ordinary_valid_core_outside_projection_remains_explicitly_unmapped() {
    let source = crate::parse("language \"moriarty/1\"; agreement Outside { action compare(a: UInt, b: UInt) { emit \"first\"(a); emit \"second\"(b); } }").unwrap();
    crate::validate(&source).unwrap();
    let report = source_projection_report(&source);
    let ActionProjection::Unsupported { diagnostic, .. } = &report.actions[0] else {
        panic!("ordered source effects remain outside a single final-value projection");
    };
    assert_eq!(diagnostic.construct, "nonfinal-emit");
    assert_eq!(diagnostic.statement_index, Some(0));
    assert_eq!(diagnostic.expression, None);
    crate::validate(&source).unwrap();
}

#[test]
fn source_order_is_retained_across_distinct_action_reports() {
    let source = crate::parse("language \"moriarty/1\"; agreement Ordered { action z_first(x: UInt) { emit \"first\"(x); } action a_second(y: Bool) { emit \"second\"(not y); } }").unwrap();
    let report = source_projection_report(&source);
    let actual: Vec<_> = report
        .actions
        .iter()
        .map(|projection| match projection {
            ActionProjection::Prepared { report } => (report.action_index, report.action.as_str()),
            ActionProjection::Unsupported { .. } => {
                panic!("both ordinary actions have projections")
            }
        })
        .collect();
    assert_eq!(actual, vec![(0, "z_first"), (1, "a_second")]);
}

#[test]
fn shared_subexpression_fanout_stops_at_the_preparation_budget() {
    // Validated Core DAG of linear size whose per-edge projection would otherwise
    // materialize 2^15-1 occurrences. Preparation must stop at the maintainer
    // budget with per-site Unsupported metadata, never a compiler refusal.
    let mut expressions = vec![crate::Expr::Literal {
        value: crate::Value::UInt(crate::UInt(1)),
    }];
    for i in 0..14u16 {
        expressions.push(crate::Expr::Binary {
            op: crate::BinaryOp::Add,
            lhs: i,
            rhs: i,
        });
    }
    let program = Program {
        language: crate::LANGUAGE.into(),
        semantics: crate::SEMANTICS.into(),
        name: "Fanout".into(),
        states: vec![],
        assets: vec![],
        expressions,
        actions: vec![crate::Action {
            name: "blow".into(),
            inputs: vec![],
            statements: vec![Statement::Emit {
                name: "out".into(),
                value: 14,
            }],
        }],
    };
    crate::validate(&program).unwrap(); // Core accepts the DAG (depth 14, 15 exprs).
    let error = prepare_scalar_action(&program, 0).unwrap_err();
    assert_eq!(error.construct, "projection-occurrence-budget");
    assert_eq!(error.code, "projection-occurrence-budget-exhausted");
    assert_eq!(error.action_index, 0);
    assert_eq!(error.statement_index, Some(0));
    let report = source_projection_report(&program);
    let ActionProjection::Unsupported { diagnostic, .. } = &report.actions[0] else {
        panic!("fan-out must remain unmapped under the preparation budget");
    };
    assert_eq!(diagnostic.construct, "projection-occurrence-budget");
    crate::validate(&program).unwrap(); // Reporting stays independent of Core validation.
}

fn eager_boolean_source(expression: &str) -> Program {
    crate::parse(&format!(
        "language \"moriarty/1\"; agreement Eager {{ action inspect() {{ emit \"value\"({expression}); }} }}"
    ))
    .expect("fixed scalar source parses")
}

fn run_projected_scalar(source: &Program) -> crate::sherlock::Outcome {
    let mapped = prepare_scalar_action(source, 0).expect("fixed scalar source projects");
    let entry = e(Expr::Call(mapped.function.name.clone(), vec![]));
    let program = Arc::new(crate::sherlock::Program {
        functions: vec![mapped.function],
        entry,
    });
    // Fixed tiny programs only. Machine receives 128 fuel and a 32-frame stack
    // limit; run_steps caps the observed records at 128, including publication.
    let mut machine = crate::sherlock::Machine::new(program, 128, 32);
    machine.run_steps(128);
    machine.snapshot().outcome
}

// Core eval_cached evaluates lhs, then rhs, before applying either boolean op.
// Thus 0 - 1 fails even when the left Bool determines the logical result.
// The independent arithmetic expectation is Core underflow / K-A error 10;
// neither evaluator supplies the other evaluator's expected outcome.
fn assert_eager_boolean_right_underflow(
    expression: &str,
    control_expression: &str,
    expected_control: bool,
) {
    let control = eager_boolean_source(control_expression);
    let execution = crate::simulate(&control, "inspect", &crate::Scenario::default())
        .expect("the non-underflowing sibling must emit its exact boolean result");
    assert_eq!(
        execution.effects,
        vec![crate::Effect::Emit {
            name: "value".into(),
            value: crate::Value::Bool(expected_control),
        }]
    );
    assert_eq!(execution.steps.len(), 1);
    assert_eq!(
        execution.steps[0].value,
        Some(crate::Value::Bool(expected_control))
    );
    assert_eq!(
        run_projected_scalar(&control),
        crate::sherlock::Outcome::Done(Value::Bool(expected_control))
    );

    let source = eager_boolean_source(expression);
    let core_error = crate::simulate(&source, "inspect", &crate::Scenario::default())
        .expect_err("the right operand must fail before a boolean value is emitted");
    assert_eq!(
        core_error,
        crate::Error {
            code: "mori::core::underflow".into(),
            message: "UInt subtraction is negative.".into(),
            offset: None,
            step: Some(0),
        }
    );
    assert_eq!(
        run_projected_scalar(&source),
        crate::sherlock::Outcome::Rejected(10)
    );
}

#[test]
fn false_and_preserves_right_operand_underflow_in_core_and_projection() {
    assert_eager_boolean_right_underflow("false and (0 - 1 == 0)", "false and (1 - 1 == 0)", false);
}

#[test]
fn true_or_preserves_right_operand_underflow_in_core_and_projection() {
    assert_eager_boolean_right_underflow("true or (0 - 1 == 0)", "true or (1 - 1 == 0)", true);
}
