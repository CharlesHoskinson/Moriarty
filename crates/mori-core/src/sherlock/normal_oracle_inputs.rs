//! Existing typed normal inputs, shared by assertions and the observation producer.
//! Source programs only: expected states/work/outcomes stay in independent JSON.

use super::*;
use std::sync::Arc;

use super::extra_oracle_programs;

pub(super) fn expr(expression: Expr) -> ExprRef {
    Arc::new(expression)
}
pub(super) fn uint(value: u128) -> ExprRef {
    expr(Expr::Literal(Value::UInt(value)))
}
pub(super) fn boolean(value: bool) -> ExprRef {
    expr(Expr::Literal(Value::Bool(value)))
}
pub(super) fn var(name: &str) -> ExprRef {
    expr(Expr::Var(name.into()))
}
pub(super) fn add(left: ExprRef, right: ExprRef) -> ExprRef {
    expr(Expr::Binary(BinaryOp::Add, left, right))
}
pub(super) fn call(name: &str, arguments: Vec<ExprRef>) -> ExprRef {
    expr(Expr::Call(name.into(), arguments))
}
pub(super) fn no_functions(entry: ExprRef) -> Arc<Program> {
    Arc::new(Program {
        functions: vec![],
        entry,
    })
}

// Six typed ASTs are written independently of the oracle serializer. Comparing
// their display terms with the JSON also checks which inputs are exercised.
pub(super) fn normal_program(case_id: &str) -> Arc<Program> {
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

