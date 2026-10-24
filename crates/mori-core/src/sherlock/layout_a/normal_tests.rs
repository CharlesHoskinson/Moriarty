//! Authored before implementation. All tests UNRUN; no SDK/evaluator calls.
use super::*;
use crate::sherlock::{
    Arm, BinaryOp, Expr, Function, Parameter, Pattern, Program, Tree, Type, UnaryOp, Value,
};
use std::sync::Arc;

fn e(x: Expr) -> Arc<Expr> {
    Arc::new(x)
}
fn lit(n: u128) -> Arc<Expr> {
    e(Expr::Literal(Value::UInt(n)))
}
// Test dimensions only, not defaults or a resource admission.
fn capacities() -> Capacities {
    Capacities {
        tables: [128; 15],
        data: 128,
    }
}
fn run(entry: Arc<Expr>, functions: Vec<Function>) -> PreparedA {
    prepare_a(Arc::new(Program { entry, functions }), capacities()).unwrap()
}

#[test]
fn independent_tree_field_blocks_follow_parent_append_not_parent_reservation() {
    let p = run(
        e(Expr::Literal(Value::Tree(Tree::Branch(
            Arc::new(Tree::Leaf(7)),
            Arc::new(Tree::Leaf(9)),
        )))),
        vec![],
    );
    let b = &p.block;
    assert_eq!(b.constants.active, 5);
    assert_eq!(b.constant_fields.active, 4);
    assert_eq!(
        b.constant_fields.rows[..4],
        [
            ValueRef::constant(0),
            ValueRef::constant(2),
            ValueRef::constant(1),
            ValueRef::constant(3),
        ]
    );
    assert_eq!(
        b.constants.rows[..5]
            .iter()
            .map(|n| n.fields)
            .collect::<Vec<_>>(),
        vec![
            Slice::ZERO,
            Slice { start: 0, len: 1 },
            Slice::ZERO,
            Slice { start: 1, len: 1 },
            Slice { start: 2, len: 2 }
        ]
    );
    assert_eq!(b.constants.rows[0].uint_bytes, 7u128.to_le_bytes());
    assert_eq!(b.constants.rows[2].uint_bytes, 9u128.to_le_bytes());
    assert_eq!(b.expressions.rows[0].value, ValueRef::constant(4));
}

#[test]
fn independent_list_postorder_and_nil_false_zero_distinctions() {
    let p = run(
        e(Expr::Call(
            "f".into(),
            vec![
                e(Expr::Literal(Value::List(vec![7, 9]))),
                e(Expr::Literal(Value::List(vec![]))),
                e(Expr::Literal(Value::Bool(false))),
                lit(0),
            ],
        )),
        vec![],
    );
    assert_eq!(p.block.constants.active, 8);
    assert_eq!(
        p.block.constants.rows[..8]
            .iter()
            .map(|n| (n.kind, n.type_id, n.ctor_id))
            .collect::<Vec<_>>(),
        vec![
            (0, 0, 0),
            (0, 0, 0),
            (2, 2, 0),
            (2, 2, 1),
            (2, 2, 1),
            (2, 2, 0),
            (1, 1, 0),
            (0, 0, 0)
        ]
    );
    assert_eq!(
        p.block.constant_fields.rows[..4],
        [
            ValueRef::constant(1),
            ValueRef::constant(2),
            ValueRef::constant(0),
            ValueRef::constant(3)
        ]
    );
    assert_eq!(
        p.block.expressions.rows[1..5]
            .iter()
            .map(|r| r.value.index)
            .collect::<Vec<_>>(),
        vec![4, 5, 6, 7]
    );
}

#[test]
fn shared_arc_occurrences_are_distinct_and_header_is_complete() {
    let shared = lit(7);
    let p = run(
        e(Expr::Binary(BinaryOp::Add, shared.clone(), shared)),
        vec![],
    );
    assert_eq!(
        p.block.header.active_lengths,
        [0, 0, 3, 2, 3, 2, 0, 0, 0, 0, 4, 4, 5, 2, 0]
    );
    assert_eq!(p.block.expr_children.rows[..2], [1, 2]);
    assert_eq!(
        p.occurrences
            .iter()
            .map(|o| o.path.clone())
            .collect::<Vec<_>>(),
        vec![vec![], vec![0], vec![1]]
    );
    assert_eq!(p.block.constants.rows[0], p.block.constants.rows[1]);
    assert_eq!(p.block.occurrences.rows[0].function_index, 0);
    assert_eq!(p.block.functions.active, 0);
    // Inactive function/name/value columns encode as zero even for empty tables.
    assert!(p.program_bytes().is_ok());
}

#[test]
fn complete_syntax_variants_and_duplicate_function_parameter_order_are_retained() {
    let binary = [
        BinaryOp::Add,
        BinaryOp::Sub,
        BinaryOp::Mul,
        BinaryOp::Eq,
        BinaryOp::Lt,
        BinaryOp::Cons,
        BinaryOp::Branch,
    ];
    let mut args = vec![
        lit(u128::MAX),
        e(Expr::Var("unused".into())),
        e(Expr::Let("x".into(), lit(1), lit(2))),
    ];
    for op in binary {
        args.push(e(Expr::Binary(op, lit(3), lit(4))));
    }
    args.push(e(Expr::Unary(UnaryOp::Leaf, lit(5))));
    args.push(e(Expr::Unary(UnaryOp::Not, lit(6))));
    args.push(e(Expr::If(lit(0), lit(1), lit(2)))); // Runtime type error remains representable.
    args.push(e(Expr::Call("missing".into(), vec![])));
    let patterns = vec![
        Pattern::Var("".into()),
        Pattern::Nil,
        Pattern::Cons("h".into(), "h".into()),
        Pattern::Leaf("é".into()),
        Pattern::Branch("l".into(), "r".into()),
    ];
    args.push(e(Expr::Match(
        lit(8),
        patterns
            .into_iter()
            .map(|pattern| Arm {
                pattern,
                body: lit(9),
            })
            .collect(),
    )));
    let f = Function {
        name: "f".into(),
        parameters: vec![
            Parameter {
                name: "p".into(),
                ty: Type::UInt,
            },
            Parameter {
                name: "p".into(),
                ty: Type::Tree,
            },
        ],
        return_type: Type::Bool,
        body: e(Expr::Var("p".into())),
    };
    let p = run(e(Expr::Call("f".into(), args)), vec![f.clone(), f]);
    let ids = &p.block.expr_children.rows[..15];
    assert_eq!(
        ids.iter()
            .map(|&id| p.block.expressions.rows[id].tag)
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3, 3, 3, 3, 3, 3, 3, 4, 4, 5, 6, 7]
    );
    assert_eq!(
        ids[3..10]
            .iter()
            .map(|&id| p.block.expressions.rows[id].binary_op)
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3, 4, 5, 6]
    );
    assert_eq!(
        ids[10..12]
            .iter()
            .map(|&id| p.block.expressions.rows[id].unary_op)
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
    assert_eq!(
        p.block.patterns.rows[..5]
            .iter()
            .map(|r| r.tag)
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3, 4]
    );
    assert_eq!(
        p.names,
        vec!["", "f", "h", "l", "missing", "p", "r", "unused", "x", "é"]
    );
    assert_eq!(
        p.block.parameters.rows[..4]
            .iter()
            .map(|r| (r.name_id, r.type_id))
            .collect::<Vec<_>>(),
        vec![(5, 0), (5, 3), (5, 0), (5, 3)]
    );
    assert_eq!(
        p.block.functions.rows[0].parameters,
        Slice { start: 0, len: 2 }
    );
    assert_eq!(
        p.block.functions.rows[1].parameters,
        Slice { start: 2, len: 2 }
    );
    assert_eq!(p.block.functions.rows[1].source_table_index, 1);
    let root = p.block.functions.rows[1].body_expr_id;
    assert_eq!(p.block.occurrences.rows[root].function_index, 1);
    assert_eq!(p.block.occurrences.rows[root].path, Slice::ZERO);
    assert_eq!(
        p.block.types.rows[..4],
        [
            TypeRow {
                kind: 0,
                constructors: Slice::ZERO
            },
            TypeRow {
                kind: 1,
                constructors: Slice::ZERO
            },
            TypeRow {
                kind: 2,
                constructors: Slice { start: 0, len: 2 }
            },
            TypeRow {
                kind: 3,
                constructors: Slice { start: 2, len: 2 }
            }
        ]
    );
    assert_eq!(p.block.constructor_field_types.rows[..5], [0, 2, 0, 3, 3]);
    assert_eq!(
        p.block.constructors.rows[..4]
            .iter()
            .map(|r| r.fields)
            .collect::<Vec<_>>(),
        vec![
            Slice::ZERO,
            Slice { start: 0, len: 2 },
            Slice { start: 2, len: 1 },
            Slice { start: 3, len: 2 }
        ]
    );
}

#[test]
fn deep_let_if_call_and_match_paths_are_literal_source_positions() {
    let body = e(Expr::Let(
        "x".into(),
        lit(1),
        e(Expr::If(
            lit(0),
            e(Expr::Call("g".into(), vec![lit(2)])),
            lit(3),
        )),
    ));
    let p = run(
        e(Expr::Match(
            lit(7),
            vec![
                Arm {
                    pattern: Pattern::Var("x".into()),
                    body: e(Expr::Var("x".into())),
                },
                Arm {
                    pattern: Pattern::Var("y".into()),
                    body: e(Expr::Var("y".into())),
                },
            ],
        )),
        vec![
            Function {
                name: "a".into(),
                parameters: vec![],
                return_type: Type::List,
                body: lit(0),
            },
            Function {
                name: "g".into(),
                parameters: vec![],
                return_type: Type::Tree,
                body,
            },
        ],
    );
    assert_eq!(
        p.occurrences[..4]
            .iter()
            .map(|o| o.path.clone())
            .collect::<Vec<_>>(),
        vec![vec![], vec![0], vec![1], vec![2]]
    );
    assert_eq!(
        p.occurrences[5..]
            .iter()
            .map(|o| o.path.clone())
            .collect::<Vec<_>>(),
        vec![
            vec![],
            vec![0],
            vec![1],
            vec![1, 0],
            vec![1, 1],
            vec![1, 1, 0],
            vec![1, 2]
        ]
    );
    assert!(
        p.block.occurrences.rows[5..12]
            .iter()
            .all(|o| o.owner_tag == 1 && o.function_index == 1)
    );
    assert_eq!(
        p.block.expr_children.rows[p.block.expressions.rows[0].children.start],
        1
    );
    assert_eq!(
        p.block.arms.rows[..2]
            .iter()
            .map(|a| a.body_expr_id)
            .collect::<Vec<_>>(),
        vec![2, 3]
    );
}

#[test]
fn padding_and_byte_lanes_are_explicit() {
    let mut c = capacities();
    c.tables[TableId::Functions as usize] = 0;
    let p = prepare_a(
        Arc::new(Program {
            functions: vec![],
            entry: lit(0x100),
        }),
        c,
    )
    .unwrap();
    assert_eq!(
        p.block.constants.rows[0].uint_bytes,
        [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    );
    assert!(
        p.block.expressions.rows[1..]
            .iter()
            .all(|r| *r == ExpressionRow::default())
    );
    assert!(
        p.block.constants.rows[1..]
            .iter()
            .all(|r| *r == ConstantNode::default())
    );
    assert!(p.block.name_bytes.rows.iter().all(|r| *r == 0));
    assert_eq!(
        &p.program_bytes().unwrap()[..16],
        &[0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0]
    );
}

#[test]
fn independent_deep_source_specimen_matches_literal_header_paths_and_tables() {
    // Root's independent deep-source-paths.json: no parser/evaluator/candidate
    // computes these expected ids, slices or counts. The specimen is complete.
    let entry = e(Expr::Let(
        "entry".into(),
        lit(7),
        e(Expr::If(
            e(Expr::Literal(Value::Bool(true))),
            e(Expr::Call("f".into(), vec![lit(9)])),
            lit(12),
        )),
    ));
    let functions = vec![
        Function {
            name: "g".into(),
            parameters: vec![],
            return_type: Type::Bool,
            body: e(Expr::Literal(Value::Bool(false))),
        },
        Function {
            name: "f".into(),
            parameters: vec![Parameter {
                name: "x".into(),
                ty: Type::UInt,
            }],
            return_type: Type::UInt,
            body: e(Expr::Binary(
                BinaryOp::Add,
                e(Expr::Var("x".into())),
                lit(1),
            )),
        },
    ];
    let p = run(entry, functions);
    let b = &p.block;
    assert_eq!(p.names, vec!["entry", "f", "g", "x"]);
    assert_eq!(&b.name_bytes.rows[..8], b"entryfgx");
    assert_eq!(
        b.header.active_lengths,
        [4, 8, 11, 13, 11, 8, 2, 1, 0, 0, 4, 4, 5, 6, 0]
    );
    assert_eq!(b.expr_children.rows[..8], [1, 2, 3, 4, 6, 5, 9, 10]);
    assert_eq!(
        b.occurrence_paths.rows[..13],
        [0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 2, 0, 1]
    );
    assert_eq!(
        b.occurrences.rows[..11]
            .iter()
            .map(|r| (r.path.start, r.path.len))
            .collect::<Vec<_>>(),
        vec![
            (0, 0),
            (0, 1),
            (1, 1),
            (2, 2),
            (4, 2),
            (6, 3),
            (9, 2),
            (0, 0),
            (0, 0),
            (11, 1),
            (12, 1)
        ]
    );
    assert_eq!(
        b.functions.rows[..2]
            .iter()
            .map(|r| r.body_expr_id)
            .collect::<Vec<_>>(),
        vec![7, 8]
    );
    assert_eq!(b.occurrences.rows[7].owner_tag, 1);
    assert_eq!(b.occurrences.rows[7].function_index, 0);
    assert!(
        b.occurrences.rows[8..11]
            .iter()
            .all(|r| r.owner_tag == 1 && r.function_index == 1)
    );
    assert_eq!(
        b.expressions.rows[..11]
            .iter()
            .map(|r| r.tag)
            .collect::<Vec<_>>(),
        vec![2, 0, 5, 0, 6, 0, 0, 0, 3, 1, 0]
    );
    assert_eq!(
        b.constants.rows[..6]
            .iter()
            .map(|r| (r.kind, u128::from_le_bytes(r.uint_bytes), r.bool_value))
            .collect::<Vec<_>>(),
        vec![
            (0, 7, false),
            (1, 0, true),
            (0, 9, false),
            (0, 12, false),
            (1, 0, false),
            (0, 1, false)
        ]
    );
    assert!(
        b.constants.rows[..6]
            .iter()
            .all(|r| r.fields == Slice::ZERO)
    );
    assert_eq!(
        &p.program_bytes().unwrap()[..16],
        &[0, 4, 8, 11, 13, 11, 8, 2, 1, 0, 0, 4, 4, 5, 6, 0]
    );
}

#[test]
fn shared_tree_children_are_unfolded_without_interning() {
    let child = Arc::new(Tree::Leaf(7));
    let p = run(
        e(Expr::Literal(Value::Tree(Tree::Branch(
            child.clone(),
            child,
        )))),
        vec![],
    );
    assert_eq!(p.block.constants.active, 5);
    assert_eq!(p.block.constants.rows[0], p.block.constants.rows[2]);
    assert_eq!(
        p.block.constant_fields.rows[..4],
        [
            ValueRef::constant(0),
            ValueRef::constant(2),
            ValueRef::constant(1),
            ValueRef::constant(3)
        ]
    );
}
