//! Literal normal representation expectations, written before the draft builder.
//! No evaluator, source-call expansion, native encoding or security judgment.

use super::super::{Arm, Function};
use super::*;
use std::sync::Arc;

fn e(value: Expr) -> ExprRef {
    Arc::new(value)
}
fn u(value: u128) -> ExprRef {
    e(Expr::Literal(Value::UInt(value)))
}
fn v(name: &str) -> ExprRef {
    e(Expr::Var(name.into()))
}
fn b(op: BinaryOp, left: ExprRef, right: ExprRef) -> ExprRef {
    e(Expr::Binary(op, left, right))
}
fn call(name: &str, args: Vec<ExprRef>) -> ExprRef {
    e(Expr::Call(name.into(), args))
}
fn slice(start: usize, len: usize) -> TableSlice {
    TableSlice { start, len }
}
fn row(payload: ExprPayload, start: usize, len: usize) -> ExprRow {
    ExprRow {
        payload,
        children: slice(start, len),
    }
}
fn loc(owner: SourceOwner, path: &[usize]) -> SourceLocator {
    SourceLocator {
        owner,
        ast_path: path.to_vec(),
    }
}
fn scalar(value: u128, expression: usize) -> ConstantRow {
    ConstantRow {
        payload: ConstantPayload::UInt(value),
        fields: slice(0, 0),
        source: ConstantLocator {
            expression: ExprId(expression),
            field_path: vec![],
        },
    }
}

#[test]
fn n07_has_literal_ordered_layout() {
    use BinaryOp::*;
    use ExprPayload::*;
    let program = Program {
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
            body: b(
                Add,
                b(Add, v("0"), b(Mul, u(10), v("1"))),
                b(Mul, u(100), v("2")),
            ),
        }],
        entry: call("17", vec![u(1), u(2), u(3)]),
    };
    let image = prepare_program_image(&program);
    assert_eq!(
        image.expressions,
        vec![
            row(Call("17".into()), 0, 3),
            row(Literal(ConstantId(0)), 3, 0),
            row(Literal(ConstantId(1)), 3, 0),
            row(Literal(ConstantId(2)), 3, 0),
            row(Binary(Add), 3, 2),
            row(Binary(Add), 5, 2),
            row(Var("0".into()), 7, 0),
            row(Binary(Mul), 7, 2),
            row(Literal(ConstantId(3)), 9, 0),
            row(Var("1".into()), 9, 0),
            row(Binary(Mul), 9, 2),
            row(Literal(ConstantId(4)), 11, 0),
            row(Var("2".into()), 11, 0),
        ]
    );
    assert_eq!(
        image.ordered_children,
        vec![
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
            ExprId(12)
        ]
    );
    assert_eq!(
        image.functions,
        vec![FunctionRow {
            source_table_index: 0,
            name: "17".into(),
            parameters: slice(0, 3),
            return_type: Type::UInt,
            body: ExprId(4),
        }]
    );
    assert_eq!(
        image.ordered_parameters,
        vec![
            Parameter {
                name: "0".into(),
                ty: Type::UInt
            },
            Parameter {
                name: "1".into(),
                ty: Type::UInt
            },
            Parameter {
                name: "2".into(),
                ty: Type::UInt
            },
        ]
    );
    assert_eq!(
        image.constants,
        vec![
            scalar(1, 1),
            scalar(2, 2),
            scalar(3, 3),
            scalar(10, 8),
            scalar(100, 11)
        ]
    );
    assert!(image.constant_fields.is_empty());
    assert!(image.match_arms.is_empty());
    assert_eq!(image.entry, ExprId(0));
    assert_eq!(
        image.source_map,
        vec![
            loc(SourceOwner::Entry, &[]),
            loc(SourceOwner::Entry, &[0]),
            loc(SourceOwner::Entry, &[1]),
            loc(SourceOwner::Entry, &[2]),
            loc(SourceOwner::Function(0), &[]),
            loc(SourceOwner::Function(0), &[0]),
            loc(SourceOwner::Function(0), &[0, 0]),
            loc(SourceOwner::Function(0), &[0, 1]),
            loc(SourceOwner::Function(0), &[0, 1, 0]),
            loc(SourceOwner::Function(0), &[0, 1, 1]),
            loc(SourceOwner::Function(0), &[1]),
            loc(SourceOwner::Function(0), &[1, 0]),
            loc(SourceOwner::Function(0), &[1, 1]),
        ]
    );
    assert_eq!(reconstruct(&image), program);
}

#[test]
fn n08_keeps_named_recursion_without_unrolling() {
    use BinaryOp::*;
    use ExprPayload::*;
    let program = Program {
        functions: vec![Function {
            name: "18".into(),
            parameters: vec![Parameter {
                name: "0".into(),
                ty: Type::UInt,
            }],
            return_type: Type::UInt,
            body: e(Expr::If(
                b(Eq, v("0"), u(0)),
                u(1),
                b(Mul, v("0"), call("18", vec![b(Sub, v("0"), u(1))])),
            )),
        }],
        entry: call("18", vec![u(2)]),
    };
    let image = prepare_program_image(&program);
    assert_eq!(
        image.expressions,
        vec![
            row(Call("18".into()), 0, 1),
            row(Literal(ConstantId(0)), 1, 0),
            row(If, 1, 3),
            row(Binary(Eq), 4, 2),
            row(Var("0".into()), 6, 0),
            row(Literal(ConstantId(1)), 6, 0),
            row(Literal(ConstantId(2)), 6, 0),
            row(Binary(Mul), 6, 2),
            row(Var("0".into()), 8, 0),
            row(Call("18".into()), 8, 1),
            row(Binary(Sub), 9, 2),
            row(Var("0".into()), 11, 0),
            row(Literal(ConstantId(3)), 11, 0),
        ]
    );
    assert_eq!(
        image.ordered_children,
        vec![
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
            ExprId(12)
        ]
    );
    assert_eq!(
        image.functions,
        vec![FunctionRow {
            source_table_index: 0,
            name: "18".into(),
            parameters: slice(0, 1),
            return_type: Type::UInt,
            body: ExprId(2),
        }]
    );
    assert_eq!(
        image.ordered_parameters,
        vec![Parameter {
            name: "0".into(),
            ty: Type::UInt
        }]
    );
    assert_eq!(
        image.constants,
        vec![scalar(2, 1), scalar(0, 5), scalar(1, 6), scalar(1, 12)]
    );
    assert!(image.constant_fields.is_empty());
    assert!(image.match_arms.is_empty());
    assert_eq!(image.entry, ExprId(0));
    assert_eq!(
        image.source_map,
        vec![
            loc(SourceOwner::Entry, &[]),
            loc(SourceOwner::Entry, &[0]),
            loc(SourceOwner::Function(0), &[]),
            loc(SourceOwner::Function(0), &[0]),
            loc(SourceOwner::Function(0), &[0, 0]),
            loc(SourceOwner::Function(0), &[0, 1]),
            loc(SourceOwner::Function(0), &[1]),
            loc(SourceOwner::Function(0), &[2]),
            loc(SourceOwner::Function(0), &[2, 0]),
            loc(SourceOwner::Function(0), &[2, 1]),
            loc(SourceOwner::Function(0), &[2, 1, 0]),
            loc(SourceOwner::Function(0), &[2, 1, 0, 0]),
            loc(SourceOwner::Function(0), &[2, 1, 0, 1]),
        ]
    );
    assert_eq!(reconstruct(&image), program);
}

#[test]
fn ordinary_lists_trees_arms_and_unused_functions_keep_occurrences() {
    let shared = u(8);
    let leaf = Arc::new(Tree::Leaf(5));
    let tree = Tree::Branch(leaf.clone(), Arc::new(Tree::Branch(leaf.clone(), leaf)));
    let patterns = vec![
        Pattern::Nil,
        Pattern::Cons("h".into(), "t".into()),
        Pattern::Var("all".into()),
        Pattern::Leaf("a".into()),
        Pattern::Branch("l".into(), "r".into()),
    ];
    let bodies = vec![
        e(Expr::Unary(
            UnaryOp::Not,
            e(Expr::Literal(Value::Bool(false))),
        )),
        b(BinaryOp::Cons, shared.clone(), v("t")),
        e(Expr::Unary(UnaryOp::Leaf, shared.clone())),
        b(
            BinaryOp::Branch,
            e(Expr::Literal(Value::Tree(tree))),
            e(Expr::Literal(Value::Tree(Tree::Leaf(9)))),
        ),
        b(BinaryOp::Lt, shared.clone(), shared),
    ];
    let program = Program {
        entry: e(Expr::Let(
            "x".into(),
            e(Expr::Literal(Value::List(vec![3, 1]))),
            e(Expr::Match(
                v("x"),
                patterns
                    .iter()
                    .cloned()
                    .zip(bodies)
                    .map(|(pattern, body)| Arm { pattern, body })
                    .collect(),
            )),
        )),
        functions: vec![
            Function {
                name: "90".into(),
                parameters: vec![
                    Parameter {
                        name: "p".into(),
                        ty: Type::Tree,
                    },
                    Parameter {
                        name: "q".into(),
                        ty: Type::List,
                    },
                ],
                return_type: Type::Bool,
                body: e(Expr::Literal(Value::Bool(true))),
            },
            Function {
                name: "20".into(),
                parameters: vec![Parameter {
                    name: "b".into(),
                    ty: Type::Bool,
                }],
                return_type: Type::List,
                body: e(Expr::Literal(Value::List(vec![]))),
            },
        ],
    };
    let image = prepare_program_image(&program);
    assert_eq!(image.entry, ExprId(0));
    assert_eq!(image.expressions.len(), 19);
    assert_eq!(
        image.ordered_children,
        vec![
            ExprId(1),
            ExprId(2),
            ExprId(3),
            ExprId(4),
            ExprId(6),
            ExprId(9),
            ExprId(11),
            ExprId(14),
            ExprId(5),
            ExprId(7),
            ExprId(8),
            ExprId(10),
            ExprId(12),
            ExprId(13),
            ExprId(15),
            ExprId(16)
        ]
    );
    assert_eq!(
        image.match_arms,
        patterns
            .into_iter()
            .zip([4, 6, 9, 11, 14])
            .map(|(pattern, body)| MatchArmRow {
                pattern,
                body: ExprId(body)
            })
            .collect::<Vec<_>>()
    );
    assert_eq!(
        image.functions,
        vec![
            FunctionRow {
                source_table_index: 0,
                name: "90".into(),
                parameters: slice(0, 2),
                return_type: Type::Bool,
                body: ExprId(17)
            },
            FunctionRow {
                source_table_index: 1,
                name: "20".into(),
                parameters: slice(2, 1),
                return_type: Type::List,
                body: ExprId(18)
            },
        ]
    );
    assert_eq!(
        image.ordered_parameters,
        vec![
            Parameter {
                name: "p".into(),
                ty: Type::Tree
            },
            Parameter {
                name: "q".into(),
                ty: Type::List
            },
            Parameter {
                name: "b".into(),
                ty: Type::Bool
            }
        ]
    );
    let constant = |payload, start, len, expression, path: &[usize]| ConstantRow {
        payload,
        fields: slice(start, len),
        source: ConstantLocator {
            expression: ExprId(expression),
            field_path: path.to_vec(),
        },
    };
    use ConstantPayload::*;
    assert_eq!(
        image.constants,
        vec![
            constant(List, 0, 2, 1, &[]),
            constant(UInt(3), 2, 0, 1, &[0]),
            constant(UInt(1), 2, 0, 1, &[1]),
            constant(Bool(false), 2, 0, 5, &[]),
            constant(UInt(8), 2, 0, 7, &[]),
            constant(UInt(8), 2, 0, 10, &[]),
            constant(TreeBranch, 2, 2, 12, &[]),
            constant(TreeLeaf(5), 4, 0, 12, &[0]),
            constant(TreeBranch, 4, 2, 12, &[1]),
            constant(TreeLeaf(5), 6, 0, 12, &[1, 0]),
            constant(TreeLeaf(5), 6, 0, 12, &[1, 1]),
            constant(TreeLeaf(9), 6, 0, 13, &[]),
            constant(UInt(8), 6, 0, 15, &[]),
            constant(UInt(8), 6, 0, 16, &[]),
            constant(Bool(true), 6, 0, 17, &[]),
            constant(List, 6, 0, 18, &[]),
        ]
    );
    assert_eq!(
        image.constant_fields,
        vec![
            ConstantId(1),
            ConstantId(2),
            ConstantId(7),
            ConstantId(8),
            ConstantId(9),
            ConstantId(10)
        ]
    );
    assert_eq!(image.source_map[7], loc(SourceOwner::Entry, &[1, 2, 0]));
    assert_eq!(image.source_map[10], loc(SourceOwner::Entry, &[1, 3, 0]));
    assert_eq!(image.source_map[15], loc(SourceOwner::Entry, &[1, 5, 0]));
    assert_eq!(image.source_map[16], loc(SourceOwner::Entry, &[1, 5, 1]));
    assert_eq!(reconstruct(&image), program);
}

// Test-only independent inverse of the tables. Reverse preorder consumes children;
// source sharing is intentionally reconstructed as distinct tree occurrences.
fn reconstruct(image: &ProgramImageDraft) -> Program {
    let mut constants: Vec<Option<Value>> = vec![None; image.constants.len()];
    for (id, node) in image.constants.iter().enumerate().rev() {
        let fields = &image.constant_fields[node.fields.start..node.fields.start + node.fields.len];
        let value = match node.payload {
            ConstantPayload::UInt(n) => Value::UInt(n),
            ConstantPayload::Bool(b) => Value::Bool(b),
            ConstantPayload::TreeLeaf(n) => Value::Tree(Tree::Leaf(n)),
            ConstantPayload::List => Value::List(
                fields
                    .iter()
                    .map(|c| match constants[c.0].take().unwrap() {
                        Value::UInt(n) => n,
                        _ => panic!("list field"),
                    })
                    .collect(),
            ),
            ConstantPayload::TreeBranch => {
                let left = match constants[fields[0].0].take().unwrap() {
                    Value::Tree(t) => t,
                    _ => panic!("left tree"),
                };
                let right = match constants[fields[1].0].take().unwrap() {
                    Value::Tree(t) => t,
                    _ => panic!("right tree"),
                };
                Value::Tree(Tree::Branch(Arc::new(left), Arc::new(right)))
            }
        };
        constants[id] = Some(value);
    }
    let mut expressions: Vec<Option<ExprRef>> = vec![None; image.expressions.len()];
    for (id, row) in image.expressions.iter().enumerate().rev() {
        let children: Vec<_> = image.ordered_children
            [row.children.start..row.children.start + row.children.len]
            .iter()
            .map(|c| expressions[c.0].take().unwrap())
            .collect();
        let expression = match &row.payload {
            ExprPayload::Literal(c) => Expr::Literal(constants[c.0].take().unwrap()),
            ExprPayload::Var(n) => Expr::Var(n.clone()),
            ExprPayload::Let(n) => Expr::Let(n.clone(), children[0].clone(), children[1].clone()),
            ExprPayload::Binary(op) => Expr::Binary(*op, children[0].clone(), children[1].clone()),
            ExprPayload::Unary(op) => Expr::Unary(*op, children[0].clone()),
            ExprPayload::If => Expr::If(
                children[0].clone(),
                children[1].clone(),
                children[2].clone(),
            ),
            ExprPayload::Call(n) => Expr::Call(n.clone(), children),
            ExprPayload::Match(arms) => Expr::Match(
                children[0].clone(),
                image.match_arms[arms.start..arms.start + arms.len]
                    .iter()
                    .zip(children.iter().skip(1))
                    .map(|(a, b)| Arm {
                        pattern: a.pattern.clone(),
                        body: b.clone(),
                    })
                    .collect(),
            ),
        };
        expressions[id] = Some(e(expression));
    }
    Program {
        functions: image
            .functions
            .iter()
            .map(|f| Function {
                name: f.name.clone(),
                parameters: image.ordered_parameters
                    [f.parameters.start..f.parameters.start + f.parameters.len]
                    .to_vec(),
                return_type: f.return_type,
                body: expressions[f.body.0].take().unwrap(),
            })
            .collect(),
        entry: expressions[image.entry.0].take().unwrap(),
    }
}

// Frozen by a non-author before the builder was first executed.
#[path = "program_image_oracles.rs"]
mod independent_oracles;

#[test]
fn independent_literal_complete_images() {
    for (label, input, expected) in independent_oracles::normal_cases() {
        assert_eq!(prepare_program_image(&input), expected, "{label}");
    }
}
