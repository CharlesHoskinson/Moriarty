//! Normal-program comparisons against frozen, independently authored literals.
//! `check-reference.py` verifies literal JSON transcription before compiling/running;
//! none of these expectations use
//! the machine, a second evaluator, or a compiler to calculate an answer.

use super::reference_display::{KDisplay, environment_to_k};
use super::*;
use std::sync::Arc;

struct ExpectedState {
    charged_steps: u128,
    remaining_fuel: u128,
    depth: usize,
    control: &'static str,
    environment: &'static str,
    frames: &'static [&'static str],
    via_rule_start_line: Option<usize>,
}

struct ExpectedPrefix {
    initial_fuel: u128,
    boundary_state_index: usize,
    remaining_fuel: u128,
    output: &'static str,
    publication_rule_start_line: usize,
}

struct ExpectedCase {
    case_id: &'static str,
    input_sha256: &'static str,
    code: &'static str,
    entry: &'static str,
    expected_value: &'static str,
    initial_fuel: u128,
    stack_limit: usize,
    pending_output: &'static str,
    states: &'static [ExpectedState],
    publication_pre_state_index: usize,
    publication_rule_start_line: usize,
    publication_charge: u8,
    publication_control: &'static str,
    publication_output: &'static str,
    prefixes: &'static [ExpectedPrefix],
}

include!("normal_oracles.rs");
include!("extra_normal_oracles.rs");

#[path = "extra_oracle_programs.rs"]
mod extra_oracle_programs;

fn expr(expression: Expr) -> ExprRef {
    Arc::new(expression)
}
fn uint(value: u128) -> ExprRef {
    expr(Expr::Literal(Value::UInt(value)))
}
fn boolean(value: bool) -> ExprRef {
    expr(Expr::Literal(Value::Bool(value)))
}
fn var(name: &str) -> ExprRef {
    expr(Expr::Var(name.into()))
}
fn add(left: ExprRef, right: ExprRef) -> ExprRef {
    expr(Expr::Binary(BinaryOp::Add, left, right))
}
fn call(name: &str, arguments: Vec<ExprRef>) -> ExprRef {
    expr(Expr::Call(name.into(), arguments))
}
fn no_functions(entry: ExprRef) -> Arc<Program> {
    Arc::new(Program {
        functions: vec![],
        entry,
    })
}

// Six typed ASTs are written independently of the oracle serializer. Comparing
// their display terms with the JSON also checks which inputs are exercised.
fn normal_program(case_id: &str) -> Arc<Program> {
    match case_id {
        "N01" => no_functions(uint(7)),
        "N02" => no_functions(add(uint(2), uint(3))),
        "N03" => no_functions(expr(Expr::If(
            expr(Expr::Unary(UnaryOp::Not, boolean(false))),
            uint(9),
            uint(4),
        ))),
        "N04" => no_functions(expr(Expr::Let(
            "0".into(),
            uint(7),
            add(expr(Expr::Let("0".into(), uint(4), var("0"))), var("0")),
        ))),
        "N05" => Arc::new(Program {
            functions: vec![Function {
                name: "11".into(),
                parameters: vec![Parameter {
                    name: "0".into(),
                    ty: Type::Bool,
                }],
                return_type: Type::UInt,
                body: expr(Expr::If(
                    var("0"),
                    call("11", vec![boolean(false)]),
                    uint(6),
                )),
            }],
            entry: call("11", vec![boolean(true)]),
        }),
        "N06" => no_functions(expr(Expr::Match(
            expr(Expr::Literal(Value::List(vec![4]))),
            vec![
                Arm {
                    pattern: Pattern::Nil,
                    body: uint(0),
                },
                Arm {
                    pattern: Pattern::Cons("1".into(), "2".into()),
                    body: expr(Expr::Match(
                        expr(Expr::Literal(Value::Tree(Tree::Leaf(9)))),
                        vec![
                            Arm {
                                pattern: Pattern::Var("3".into()),
                                body: var("1"),
                            },
                            Arm {
                                pattern: Pattern::Leaf("4".into()),
                                body: uint(99),
                            },
                        ],
                    )),
                },
            ],
        ))),
        _ => Arc::new(
            extra_oracle_programs::extra_program(case_id)
                .unwrap_or_else(|| panic!("unknown literal input {case_id}")),
        ),
    }
}

#[allow(clippy::too_many_arguments)]
fn compare_snapshot(
    actual: &Snapshot,
    case: &ExpectedCase,
    expected: &ExpectedState,
    initial_fuel: u128,
    remaining_fuel: u128,
    control: &str,
    output: &str,
    context: &str,
) {
    assert_eq!(actual.program.to_k(), case.code, "{context}: code");
    assert_eq!(actual.program.entry.to_k(), case.entry, "{context}: entry");
    assert_eq!(
        actual
            .control
            .as_ref()
            .map(KDisplay::to_k)
            .unwrap_or_else(|| ".K".into()),
        control,
        "{context}: control"
    );
    assert_eq!(
        environment_to_k(&actual.environment),
        expected.environment,
        "{context}: environment"
    );
    let frames: Vec<String> = actual.frames.iter().map(KDisplay::to_k).collect();
    assert_eq!(
        frames, expected.frames,
        "{context}: complete top-first frames"
    );
    assert_eq!(actual.depth(), expected.depth, "{context}: depth");
    assert_eq!(
        actual.stack_limit, case.stack_limit,
        "{context}: stack limit"
    );
    assert_eq!(actual.initial_fuel, initial_fuel, "{context}: initial fuel");
    assert_eq!(
        actual.remaining_fuel, remaining_fuel,
        "{context}: remaining fuel"
    );
    assert_eq!(
        actual.charged_steps(),
        expected.charged_steps,
        "{context}: work"
    );
    assert_eq!(actual.outcome.to_k(), output, "{context}: complete outcome");
}

fn compare_case(case_id: &str) {
    let case = CASES
        .iter()
        .chain(EXTRA_CASES.iter())
        .find(|case| case.case_id == case_id)
        .expect("frozen case");
    let program = normal_program(case_id);
    assert_eq!(PROFILE, ORACLE_PROFILE);
    assert_eq!(K_SOURCE_SHA256, ORACLE_SOURCE_SHA256);
    assert_eq!(PROFILE, EXTRA_ORACLE_PROFILE);
    assert_eq!(K_SOURCE_SHA256, EXTRA_ORACLE_SOURCE_SHA256);
    assert_eq!(program.to_k(), case.code, "{case_id}: source code term");
    assert_eq!(
        program.entry.to_k(),
        case.entry,
        "{case_id}: source entry term"
    );
    assert_eq!(case.input_sha256.len(), 64);

    for prefix in case.prefixes {
        let context = format!("{case_id}, fuel {}", prefix.initial_fuel);
        let mut machine = Machine::new(program.clone(), prefix.initial_fuel, case.stack_limit);
        let mut prior = machine.snapshot();
        let initial = &case.states[0];
        compare_snapshot(
            &prior,
            case,
            initial,
            prefix.initial_fuel,
            prefix.initial_fuel,
            initial.control,
            case.pending_output,
            &format!("{context}, startup post-state"),
        );

        for index in 1..=prefix.boundary_state_index {
            let expected = &case.states[index];
            let step = machine.step().expect("expected charged transition");
            assert_eq!(
                step.before, prior,
                "{context}, state {index}: complete pre-state"
            );
            assert_eq!(
                step.charge, 1,
                "{context}, state {index}: charged transition"
            );
            assert_eq!(
                Some(step.rule_start_line),
                expected.via_rule_start_line,
                "{context}, state {index}: actual via-rule source line"
            );
            // This is the frozen JSON's stated prefix reconstruction, using its
            // literal charged_steps. It computes no program work or result.
            let remaining = prefix.initial_fuel - expected.charged_steps;
            compare_snapshot(
                &step.after,
                case,
                expected,
                prefix.initial_fuel,
                remaining,
                expected.control,
                case.pending_output,
                &format!("{context}, state {index}"),
            );
            assert_eq!(
                machine.snapshot(),
                step.after,
                "{context}: machine post-state"
            );
            prior = step.after;
        }

        let boundary = &case.states[prefix.boundary_state_index];
        compare_snapshot(
            &prior,
            case,
            boundary,
            prefix.initial_fuel,
            prefix.remaining_fuel,
            boundary.control,
            case.pending_output,
            &format!("{context}, prefix boundary"),
        );
        let publication = machine.step().expect("expected free terminal publication");
        assert_eq!(
            publication.before, prior,
            "{context}: publication pre-state"
        );
        assert_eq!(publication.charge, 0, "{context}: free publication");
        assert_eq!(
            publication.rule_start_line, prefix.publication_rule_start_line,
            "{context}: actual publication source line"
        );
        compare_snapshot(
            &publication.after,
            case,
            boundary,
            prefix.initial_fuel,
            prefix.remaining_fuel,
            ".K",
            prefix.output,
            &format!("{context}, publication"),
        );
        assert_eq!(
            machine.snapshot(),
            publication.after,
            "{context}: published machine state"
        );
        assert!(
            machine.step().is_none(),
            "{context}: terminal state has no next transition"
        );
        assert_eq!(
            machine.snapshot(),
            publication.after,
            "{context}: terminal state unchanged"
        );

        // Exercise run() through the same production implementation as step().
        let mut run_machine = Machine::new(program.clone(), prefix.initial_fuel, case.stack_limit);
        assert_eq!(
            run_machine.run(),
            publication.after,
            "{context}: run/step complete state"
        );

        if prefix.initial_fuel == case.initial_fuel {
            assert_eq!(
                prefix.boundary_state_index,
                case.publication_pre_state_index
            );
            assert_eq!(
                prefix.publication_rule_start_line,
                case.publication_rule_start_line
            );
            assert_eq!(case.publication_charge, 0);
            assert_eq!(case.publication_control, ".K");
            assert_eq!(prefix.output, case.publication_output);
            assert_eq!(prefix.output, format!("uiDone({})", case.expected_value));
            // Retain each original full-trace fuel literal as a separate check.
            for state in case.states {
                assert_eq!(
                    state.remaining_fuel,
                    case.initial_fuel - state.charged_steps
                );
            }
        }
    }
}

#[test]
fn normal_n01_literal() {
    compare_case("N01");
}
#[test]
fn normal_n02_addition() {
    compare_case("N02");
}
#[test]
fn normal_n03_conditional() {
    compare_case("N03");
}
#[test]
fn normal_n04_lexical_restoration() {
    compare_case("N04");
}
#[test]
fn normal_n05_named_recursion() {
    compare_case("N05");
}
#[test]
fn normal_n06_ordered_list_tree_match() {
    compare_case("N06");
}

#[test]
fn normal_n07_three_ordered_arguments() {
    compare_case("N07");
}

#[test]
fn normal_n08_recursive_factorial() {
    compare_case("N08");
}

#[test]
fn frozen_normal_oracle_membership() {
    assert_eq!(
        CASES.iter().map(|case| case.case_id).collect::<Vec<_>>(),
        ["N01", "N02", "N03", "N04", "N05", "N06"]
    );
    assert_eq!(
        CASES.iter().map(|case| case.prefixes.len()).sum::<usize>(),
        92
    );
}
