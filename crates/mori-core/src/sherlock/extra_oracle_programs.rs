//! Typed inputs for the two additional frozen normal K-A oracle cases.
//! These builders contain source programs only; expectations stay in the
//! independently authored literal tables.

use super::{BinaryOp, Expr, ExprRef, Function, Parameter, Program, Type, Value};
use std::sync::Arc;

fn expr(expression: Expr) -> ExprRef {
    Arc::new(expression)
}

fn uint(value: u128) -> ExprRef {
    expr(Expr::Literal(Value::UInt(value)))
}

fn var(name: &str) -> ExprRef {
    expr(Expr::Var(name.into()))
}

fn binary(op: BinaryOp, left: ExprRef, right: ExprRef) -> ExprRef {
    expr(Expr::Binary(op, left, right))
}

fn call(name: &str, arguments: Vec<ExprRef>) -> ExprRef {
    expr(Expr::Call(name.into(), arguments))
}

pub(super) fn extra_program(case_id: &str) -> Option<Program> {
    match case_id {
        "N07" => Some(Program {
            functions: vec![Function {
                name: "17".into(),
                parameters: vec![
                    Parameter {
                        name: "0".into(),
                        ty: Type::UInt,
                    },
                    Parameter {
                        name: "1".into(),
                        ty: Type::UInt,
                    },
                    Parameter {
                        name: "2".into(),
                        ty: Type::UInt,
                    },
                ],
                return_type: Type::UInt,
                body: binary(
                    BinaryOp::Add,
                    binary(
                        BinaryOp::Add,
                        var("0"),
                        binary(BinaryOp::Mul, uint(10), var("1")),
                    ),
                    binary(BinaryOp::Mul, uint(100), var("2")),
                ),
            }],
            entry: call("17", vec![uint(1), uint(2), uint(3)]),
        }),
        "N08" => Some(Program {
            functions: vec![Function {
                name: "18".into(),
                parameters: vec![Parameter {
                    name: "0".into(),
                    ty: Type::UInt,
                }],
                return_type: Type::UInt,
                body: expr(Expr::If(
                    binary(BinaryOp::Eq, var("0"), uint(0)),
                    uint(1),
                    binary(
                        BinaryOp::Mul,
                        var("0"),
                        call("18", vec![binary(BinaryOp::Sub, var("0"), uint(1))]),
                    ),
                )),
            }],
            entry: call("18", vec![uint(2)]),
        }),
        _ => None,
    }
}
