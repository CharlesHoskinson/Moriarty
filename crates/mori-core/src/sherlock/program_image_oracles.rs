//! Independent literal expectations, frozen without reading or running the subject.
//! Inputs N07/N08 transcribed from extra-normal-programs.json; two further inputs
//! exercise physical sharing and whole-source function order. Helpers construct
//! literal fields only; no helper derives, traverses, or evaluates an image.
use super::*;
use std::sync::Arc;

fn s(start: usize, len: usize) -> TableSlice {
    TableSlice { start, len }
}
fn r(payload: ExprPayload, start: usize, len: usize) -> ExprRow {
    ExprRow {
        payload,
        children: s(start, len),
    }
}
fn p(owner: SourceOwner, ast_path: Vec<usize>) -> SourceLocator {
    SourceLocator { owner, ast_path }
}
fn c(
    payload: ConstantPayload,
    start: usize,
    len: usize,
    expression: usize,
    field_path: Vec<usize>,
) -> ConstantRow {
    ConstantRow {
        payload,
        fields: s(start, len),
        source: ConstantLocator {
            expression: ExprId(expression),
            field_path,
        },
    }
}
fn a(expr: Expr) -> ExprRef {
    Arc::new(expr)
}
fn u(value: u128) -> ExprRef {
    a(Expr::Literal(Value::UInt(value)))
}
fn v(name: &str) -> ExprRef {
    a(Expr::Var(name.into()))
}

pub(super) fn normal_cases() -> Vec<(&'static str, Program, ProgramImageDraft)> {
    // This one physical Arc occurs twice in the input. The expected constant
    // table below gives those two occurrences different constant IDs.
    let shared_leaf = Arc::new(Tree::Leaf(4));
    vec![
        (
            "N07",
            Program {
                entry: a(Expr::Call("17".into(), vec![u(1), u(2), u(3)])),
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
                    body: a(Expr::Binary(
                        BinaryOp::Add,
                        a(Expr::Binary(
                            BinaryOp::Add,
                            v("0"),
                            a(Expr::Binary(BinaryOp::Mul, u(10), v("1"))),
                        )),
                        a(Expr::Binary(BinaryOp::Mul, u(100), v("2"))),
                    )),
                }],
            },
            ProgramImageDraft {
                profile_id: "sherlock-k-a/1",
                expressions: vec![
                    r(ExprPayload::Call("17".into()), 0, 3),
                    r(ExprPayload::Literal(ConstantId(0)), 3, 0),
                    r(ExprPayload::Literal(ConstantId(1)), 3, 0),
                    r(ExprPayload::Literal(ConstantId(2)), 3, 0),
                    r(ExprPayload::Binary(BinaryOp::Add), 3, 2),
                    r(ExprPayload::Binary(BinaryOp::Add), 5, 2),
                    r(ExprPayload::Var("0".into()), 7, 0),
                    r(ExprPayload::Binary(BinaryOp::Mul), 7, 2),
                    r(ExprPayload::Literal(ConstantId(3)), 9, 0),
                    r(ExprPayload::Var("1".into()), 9, 0),
                    r(ExprPayload::Binary(BinaryOp::Mul), 9, 2),
                    r(ExprPayload::Literal(ConstantId(4)), 11, 0),
                    r(ExprPayload::Var("2".into()), 11, 0),
                ],
                ordered_children: vec![
                    ExprId(1),
                    ExprId(2),
                    ExprId(3),
                    ExprId(5),
                    ExprId(10),
                    ExprId(6),
                    ExprId(7),
                    ExprId(8),
                    ExprId(9),
                    ExprId(11),
                    ExprId(12),
                ],
                functions: vec![FunctionRow {
                    source_table_index: 0,
                    name: "17".into(),
                    parameters: s(0, 3),
                    return_type: Type::UInt,
                    body: ExprId(4),
                }],
                ordered_parameters: vec![
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
                match_arms: vec![],
                constants: vec![
                    c(ConstantPayload::UInt(1), 0, 0, 1, vec![]),
                    c(ConstantPayload::UInt(2), 0, 0, 2, vec![]),
                    c(ConstantPayload::UInt(3), 0, 0, 3, vec![]),
                    c(ConstantPayload::UInt(10), 0, 0, 8, vec![]),
                    c(ConstantPayload::UInt(100), 0, 0, 11, vec![]),
                ],
                constant_fields: vec![],
                source_map: vec![
                    p(SourceOwner::Entry, vec![]),
                    p(SourceOwner::Entry, vec![0]),
                    p(SourceOwner::Entry, vec![1]),
                    p(SourceOwner::Entry, vec![2]),
                    p(SourceOwner::Function(0), vec![]),
                    p(SourceOwner::Function(0), vec![0]),
                    p(SourceOwner::Function(0), vec![0, 0]),
                    p(SourceOwner::Function(0), vec![0, 1]),
                    p(SourceOwner::Function(0), vec![0, 1, 0]),
                    p(SourceOwner::Function(0), vec![0, 1, 1]),
                    p(SourceOwner::Function(0), vec![1]),
                    p(SourceOwner::Function(0), vec![1, 0]),
                    p(SourceOwner::Function(0), vec![1, 1]),
                ],
                entry: ExprId(0),
            },
        ),
        (
            "N08",
            Program {
                entry: a(Expr::Call("18".into(), vec![u(2)])),
                functions: vec![Function {
                    name: "18".into(),
                    parameters: vec![Parameter {
                        name: "0".into(),
                        ty: Type::UInt,
                    }],
                    return_type: Type::UInt,
                    body: a(Expr::If(
                        a(Expr::Binary(BinaryOp::Eq, v("0"), u(0))),
                        u(1),
                        a(Expr::Binary(
                            BinaryOp::Mul,
                            v("0"),
                            a(Expr::Call(
                                "18".into(),
                                vec![a(Expr::Binary(BinaryOp::Sub, v("0"), u(1)))],
                            )),
                        )),
                    )),
                }],
            },
            ProgramImageDraft {
                profile_id: "sherlock-k-a/1",
                expressions: vec![
                    r(ExprPayload::Call("18".into()), 0, 1),
                    r(ExprPayload::Literal(ConstantId(0)), 1, 0),
                    r(ExprPayload::If, 1, 3),
                    r(ExprPayload::Binary(BinaryOp::Eq), 4, 2),
                    r(ExprPayload::Var("0".into()), 6, 0),
                    r(ExprPayload::Literal(ConstantId(1)), 6, 0),
                    r(ExprPayload::Literal(ConstantId(2)), 6, 0),
                    r(ExprPayload::Binary(BinaryOp::Mul), 6, 2),
                    r(ExprPayload::Var("0".into()), 8, 0),
                    r(ExprPayload::Call("18".into()), 8, 1),
                    r(ExprPayload::Binary(BinaryOp::Sub), 9, 2),
                    r(ExprPayload::Var("0".into()), 11, 0),
                    r(ExprPayload::Literal(ConstantId(3)), 11, 0),
                ],
                ordered_children: vec![
                    ExprId(1),
                    ExprId(3),
                    ExprId(6),
                    ExprId(7),
                    ExprId(4),
                    ExprId(5),
                    ExprId(8),
                    ExprId(9),
                    ExprId(10),
                    ExprId(11),
                    ExprId(12),
                ],
                functions: vec![FunctionRow {
                    source_table_index: 0,
                    name: "18".into(),
                    parameters: s(0, 1),
                    return_type: Type::UInt,
                    body: ExprId(2),
                }],
                ordered_parameters: vec![Parameter {
                    name: "0".into(),
                    ty: Type::UInt,
                }],
                match_arms: vec![],
                constants: vec![
                    c(ConstantPayload::UInt(2), 0, 0, 1, vec![]),
                    c(ConstantPayload::UInt(0), 0, 0, 5, vec![]),
                    c(ConstantPayload::UInt(1), 0, 0, 6, vec![]),
                    c(ConstantPayload::UInt(1), 0, 0, 12, vec![]),
                ],
                constant_fields: vec![],
                source_map: vec![
                    p(SourceOwner::Entry, vec![]),
                    p(SourceOwner::Entry, vec![0]),
                    p(SourceOwner::Function(0), vec![]),
                    p(SourceOwner::Function(0), vec![0]),
                    p(SourceOwner::Function(0), vec![0, 0]),
                    p(SourceOwner::Function(0), vec![0, 1]),
                    p(SourceOwner::Function(0), vec![1]),
                    p(SourceOwner::Function(0), vec![2]),
                    p(SourceOwner::Function(0), vec![2, 0]),
                    p(SourceOwner::Function(0), vec![2, 1]),
                    p(SourceOwner::Function(0), vec![2, 1, 0]),
                    p(SourceOwner::Function(0), vec![2, 1, 0, 0]),
                    p(SourceOwner::Function(0), vec![2, 1, 0, 1]),
                ],
                entry: ExprId(0),
            },
        ),
        (
            "SHARED_ARC_TREE",
            Program {
                entry: a(Expr::Literal(Value::Tree(Tree::Branch(
                    shared_leaf.clone(),
                    shared_leaf,
                )))),
                functions: vec![],
            },
            ProgramImageDraft {
                profile_id: "sherlock-k-a/1",
                expressions: vec![r(ExprPayload::Literal(ConstantId(0)), 0, 0)],
                ordered_children: vec![],
                functions: vec![],
                ordered_parameters: vec![],
                match_arms: vec![],
                constants: vec![
                    c(ConstantPayload::TreeBranch, 0, 2, 0, vec![]),
                    c(ConstantPayload::TreeLeaf(4), 2, 0, 0, vec![0]),
                    c(ConstantPayload::TreeLeaf(4), 2, 0, 0, vec![1]),
                ],
                constant_fields: vec![ConstantId(1), ConstantId(2)],
                source_map: vec![p(SourceOwner::Entry, vec![])],
                entry: ExprId(0),
            },
        ),
        (
            "UNUSED_FUNCTION_TABLE_ORDER",
            Program {
                entry: u(7),
                functions: vec![
                    Function {
                        name: "8".into(),
                        parameters: vec![],
                        return_type: Type::UInt,
                        body: u(9),
                    },
                    Function {
                        name: "3".into(),
                        parameters: vec![],
                        return_type: Type::UInt,
                        body: u(4),
                    },
                ],
            },
            ProgramImageDraft {
                profile_id: "sherlock-k-a/1",
                expressions: vec![
                    r(ExprPayload::Literal(ConstantId(0)), 0, 0),
                    r(ExprPayload::Literal(ConstantId(1)), 0, 0),
                    r(ExprPayload::Literal(ConstantId(2)), 0, 0),
                ],
                ordered_children: vec![],
                functions: vec![
                    FunctionRow {
                        source_table_index: 0,
                        name: "8".into(),
                        parameters: s(0, 0),
                        return_type: Type::UInt,
                        body: ExprId(1),
                    },
                    FunctionRow {
                        source_table_index: 1,
                        name: "3".into(),
                        parameters: s(0, 0),
                        return_type: Type::UInt,
                        body: ExprId(2),
                    },
                ],
                ordered_parameters: vec![],
                match_arms: vec![],
                constants: vec![
                    c(ConstantPayload::UInt(7), 0, 0, 0, vec![]),
                    c(ConstantPayload::UInt(9), 0, 0, 1, vec![]),
                    c(ConstantPayload::UInt(4), 0, 0, 2, vec![]),
                ],
                constant_fields: vec![],
                source_map: vec![
                    p(SourceOwner::Entry, vec![]),
                    p(SourceOwner::Function(0), vec![]),
                    p(SourceOwner::Function(1), vec![]),
                ],
                entry: ExprId(0),
            },
        ),
    ]
}
