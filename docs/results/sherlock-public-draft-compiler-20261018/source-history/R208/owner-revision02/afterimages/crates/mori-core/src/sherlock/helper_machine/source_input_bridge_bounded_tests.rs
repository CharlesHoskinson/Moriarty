//! Focused security-owner unit tests for `source_input_bridge::from_program_bounded`.
//! SOURCE_ONLY / UNRUN. Uses iterative comparison over `b::CoreProgram` because
//! `CoreProgram`/`CoreExpr`/`CoreTree`/`CoreValue` intentionally derive no
//! `Debug` or `PartialEq` (C8).

use std::sync::Arc;

use crate::sherlock as a;
use super::b;
use super::{
     check_add, check_limit, from_program, from_program_bounded, SourceCopyBudget,
    SourceCopyBudgetExceeded, SourceCopyError, SourceCopyResource,
};

fn expect_ok(program: &a::Program, budget: &SourceCopyBudget) -> b::CoreProgram {
    match from_program_bounded(program, budget) {
        Ok(copied) => copied,
        Err(err) => panic!("expected Ok(CoreProgram), got {:?}", err),
    }
}

fn expect_err(program: &a::Program, budget: &SourceCopyBudget) -> SourceCopyBudgetExceeded {
    match from_program_bounded(program, budget) {
        Err(SourceCopyError::Budget(exceeded)) => exceeded,
        Ok(_) => panic!("expected SourceCopyError::Budget, got Ok(CoreProgram)"),
    }
}

fn assert_core_patterns_eq(left: &b::CorePattern, right: &b::CorePattern) {
    match (left, right) {
        (b::CorePattern::PVar { x: lx }, b::CorePattern::PVar { x: rx }) => assert_eq!(lx, rx),
        (b::CorePattern::PNil, b::CorePattern::PNil) => {}
        (b::CorePattern::PCons { x: lx, y: ly }, b::CorePattern::PCons { x: rx, y: ry }) => {
            assert_eq!(lx, rx);
            assert_eq!(ly, ry);
        }
        (b::CorePattern::PLeaf { x: lx }, b::CorePattern::PLeaf { x: rx }) => assert_eq!(lx, rx),
        (b::CorePattern::PBranch { x: lx, y: ly }, b::CorePattern::PBranch { x: rx, y: ry }) => {
            assert_eq!(lx, rx);
            assert_eq!(ly, ry);
        }
        _ => panic!("CorePattern variant mismatch"),
    }
}

fn assert_core_trees_eq(left: &b::CoreTree, right: &b::CoreTree) {
    let mut stack = vec![(left, right)];
    while let Some((l, r)) = stack.pop() {
        match (l, r) {
            (b::CoreTree::Leaf { n: ln }, b::CoreTree::Leaf { n: rn }) => assert_eq!(ln, rn),
            (
                b::CoreTree::Branch { left: ll, right: lr },
                b::CoreTree::Branch { left: rl, right: rr },
            ) => {
                stack.push((lr.as_ref(), rr.as_ref()));
                stack.push((ll.as_ref(), rl.as_ref()));
            }
            _ => panic!("CoreTree structure mismatch"),
        }
    }
}

fn assert_core_values_eq(left: &b::CoreValue, right: &b::CoreValue) {
    match (left, right) {
        (b::CoreValue::UInt { n: ln }, b::CoreValue::UInt { n: rn }) => assert_eq!(ln, rn),
        (b::CoreValue::Bool { b: lb }, b::CoreValue::Bool { b: rb }) => assert_eq!(lb, rb),
        (b::CoreValue::List { elements: le }, b::CoreValue::List { elements: re }) => {
            assert_eq!(le, re);
        }
        (b::CoreValue::Tree { tree: lt }, b::CoreValue::Tree { tree: rt }) => {
            assert_core_trees_eq(lt, rt);
        }
        _ => panic!("CoreValue variant mismatch"),
    }
}

fn assert_core_exprs_eq(left: &b::CoreExpr, right: &b::CoreExpr) {
    let mut stack = vec![(left, right)];
    while let Some((l, r)) = stack.pop() {
        match (l, r) {
            (b::CoreExpr::Literal { value: lv }, b::CoreExpr::Literal { value: rv }) => {
                assert_core_values_eq(lv, rv);
            }
            (b::CoreExpr::Var { name: ln }, b::CoreExpr::Var { name: rn }) => assert_eq!(ln, rn),
            (
                b::CoreExpr::Let { name: ln, value: lv, body: lb },
                b::CoreExpr::Let { name: rn, value: rv, body: rb },
            ) => {
                assert_eq!(ln, rn);
                stack.push((lb.as_ref(), rb.as_ref()));
                stack.push((lv.as_ref(), rv.as_ref()));
            }
            (
                b::CoreExpr::Binary { op: lop, left: ll, right: lr },
                b::CoreExpr::Binary { op: rop, left: rl, right: rr },
            ) => {
                assert_eq!(lop, rop);
                stack.push((lr.as_ref(), rr.as_ref()));
                stack.push((ll.as_ref(), rl.as_ref()));
            }
            (
                b::CoreExpr::Unary { op: lop, operand: lo },
                b::CoreExpr::Unary { op: rop, operand: ro },
            ) => {
                assert_eq!(lop, rop);
                stack.push((lo.as_ref(), ro.as_ref()));
            }
            (
                b::CoreExpr::If { cond: lc, then_expr: lt, else_expr: le },
                b::CoreExpr::If { cond: rc, then_expr: rt, else_expr: re },
            ) => {
                stack.push((le.as_ref(), re.as_ref()));
                stack.push((lt.as_ref(), rt.as_ref()));
                stack.push((lc.as_ref(), rc.as_ref()));
            }
            (
                b::CoreExpr::Call { name: ln, args: la },
                b::CoreExpr::Call { name: rn, args: ra },
            ) => {
                assert_eq!(ln, rn);
                assert_eq!(la.len(), ra.len());
                for (l_arg, r_arg) in la.iter().zip(ra.iter()).rev() {
                    stack.push((l_arg, r_arg));
                }
            }
            (
                b::CoreExpr::Match { scrutinee: ls, arms: la },
                b::CoreExpr::Match { scrutinee: rs, arms: ra },
            ) => {
                assert_eq!(la.len(), ra.len());
                for (l_arm, r_arm) in la.iter().zip(ra.iter()).rev() {
                    assert_core_patterns_eq(&l_arm.pattern, &r_arm.pattern);
                    stack.push((&l_arm.body, &r_arm.body));
                }
                stack.push((ls.as_ref(), rs.as_ref()));
            }
            _ => panic!("CoreExpr variant mismatch"),
        }
    }
}

fn assert_core_programs_eq(left: &b::CoreProgram, right: &b::CoreProgram) {
    assert_eq!(left.functions.len(), right.functions.len());
    for (lf, rf) in left.functions.iter().zip(right.functions.iter()) {
        assert_eq!(lf.name, rf.name);
        assert_eq!(lf.return_type, rf.return_type);
        assert_eq!(lf.parameters.len(), rf.parameters.len());
        for (lp, rp) in lf.parameters.iter().zip(rf.parameters.iter()) {
            assert_eq!(lp.name, rp.name);
            assert_eq!(lp.sort, rp.sort);
        }
        assert_core_exprs_eq(&lf.body, &rf.body);
    }
    assert_core_exprs_eq(&left.entry, &right.entry);
}

fn generous_budget() -> SourceCopyBudget {
    SourceCopyBudget {
        max_functions: 16,
        max_parameters_per_function: 16,
        max_total_parameters: 64,
        max_expr_nodes: 256,
        max_copy_depth: 32,
        max_match_arms: 64,
        max_name_bytes: 1024,
        max_list_elements: 128,
        max_tree_nodes: 128,
        max_tree_depth: 32,
    }
}

#[test]
fn bounded_copy_matches_legacy_across_all_variants_and_exact_limits() {
    let shared_leaf_expr = Arc::new(a::Expr::Literal(a::Value::UInt(9)));
    let shared_tree_leaf = Arc::new(a::Tree::Leaf(42));
    let tree_val = a::Tree::Branch(Arc::clone(&shared_tree_leaf), shared_tree_leaf);

    let fn_body = Arc::new(a::Expr::Match(
        Arc::new(a::Expr::Var("xs".to_string())),
        vec![
            a::Arm {
                pattern: a::Pattern::Nil,
                body: Arc::clone(&shared_leaf_expr),
            },
            a::Arm {
                pattern: a::Pattern::Cons("h".to_string(), "t".to_string()),
                body: Arc::new(a::Expr::Binary(
                    a::BinaryOp::Add,
                    Arc::new(a::Expr::Var("h".to_string())),
                    Arc::new(a::Expr::Call(
                        "sum".to_string(),
                        vec![Arc::new(a::Expr::Var("t".to_string()))],
                    )),
                )),
            },
        ],
    ));

    let entry = Arc::new(a::Expr::Let(
        "v".to_string(),
        Arc::new(a::Expr::If(
            Arc::new(a::Expr::Unary(
                a::UnaryOp::Not,
                Arc::new(a::Expr::Literal(a::Value::Bool(false))),
            )),
            Arc::new(a::Expr::Literal(a::Value::List(vec![1, 2, 3]))),
            Arc::new(a::Expr::Literal(a::Value::Tree(tree_val))),
        )),
        Arc::new(a::Expr::Binary(
            a::BinaryOp::Add,
            shared_leaf_expr,
            Arc::new(a::Expr::Call("sum".to_string(), vec![Arc::new(a::Expr::Var("v".to_string()))])),
        )),
    ));

    let program = a::Program {
        functions: vec![a::Function {
            name: "sum".to_string(),
            parameters: vec![a::Parameter {
                name: "xs".to_string(),
                ty: a::Type::List,
            }],
            return_type: a::Type::UInt,
            body: fn_body,
        }],
        entry,
    };

    // Exact counts:
    // functions: 1 ("sum" = 3 bytes)
    // parameters: 1 ("xs" = 2 bytes)
    // entry expr nodes: Let(1) + If(1) + Unary(1) + Bool(1) + List(1) + Tree(1)
    //                   + Binary(1) + UInt(1) + Call(1) + Var("v")(1) = 10, max depth = 4
    // fn_body expr nodes: Match(1) + Var("xs")(1) + UInt(1) + Binary(1) + Var("h")(1)
    //                     + Call(1) + Var("t")(1) = 7, max depth = 4
    // total expr nodes: 17
    // match arms: 2
    // name bytes: "sum"(3) + "xs"(2) + "v"(1) + "sum"(3) + "v"(1)
    //             + "xs"(2) + "h"(1) + "t"(1) + "h"(1) + "sum"(3) + "t"(1) = 19
    // list elements: 3
    // tree nodes: Branch(1) + Leaf(1) + Leaf(1) = 3, tree depth: 2
    let exact_budget = SourceCopyBudget {
        max_functions: 1,
        max_parameters_per_function: 1,
        max_total_parameters: 1,
        max_expr_nodes: 17,
        max_copy_depth: 4,
        max_match_arms: 2,
        max_name_bytes: 19,
        max_list_elements: 3,
        max_tree_nodes: 3,
        max_tree_depth: 2,
    };

    let legacy = from_program(&program);
    let bounded = expect_ok(&program, &exact_budget);
    assert_core_programs_eq(&legacy, &bounded);
}

#[test]
fn function_and_parameter_limits_refuse_before_allocation() {
    let lit = Arc::new(a::Expr::Literal(a::Value::UInt(0)));
    let program = a::Program {
        functions: vec![
            a::Function {
                name: "f0".to_string(),
                parameters: vec![
                    a::Parameter { name: "a".to_string(), ty: a::Type::UInt },
                    a::Parameter { name: "b".to_string(), ty: a::Type::Bool },
                ],
                return_type: a::Type::UInt,
                body: Arc::clone(&lit),
            },
            a::Function {
                name: "f1".to_string(),
                parameters: vec![
                    a::Parameter { name: "c".to_string(), ty: a::Type::List },
                    a::Parameter { name: "d".to_string(), ty: a::Type::Tree },
                ],
                return_type: a::Type::UInt,
                body: Arc::clone(&lit),
            },
        ],
        entry: lit,
    };

    let mut b_fn = generous_budget();
    b_fn.max_functions = 1;
    assert_eq!(
        expect_err(&program, &b_fn),
        SourceCopyBudgetExceeded {
            resource: SourceCopyResource::Functions,
            limit: 1,
            observed: Some(2),
            function_index: None,
            child_index: None,
        }
    );

    let mut b_ppf = generous_budget();
    b_ppf.max_parameters_per_function = 1;
    assert_eq!(
        expect_err(&program, &b_ppf),
        SourceCopyBudgetExceeded {
            resource: SourceCopyResource::ParametersPerFunction,
            limit: 1,
            observed: Some(2),
            function_index: Some(0),
            child_index: None,
        }
    );

    let mut b_total = generous_budget();
    b_total.max_parameters_per_function = 2;
    b_total.max_total_parameters = 3;
    assert_eq!(
        expect_err(&program, &b_total),
        SourceCopyBudgetExceeded {
            resource: SourceCopyResource::TotalParameters,
            limit: 3,
            observed: Some(4),
            function_index: Some(1),
            child_index: None,
        }
    );
}

#[test]
fn copy_depth_zero_and_child_depth_refusals_are_precise() {
    let program = a::Program {
        functions: vec![],
        entry: Arc::new(a::Expr::Unary(
            a::UnaryOp::Not,
            Arc::new(a::Expr::Literal(a::Value::Bool(true))),
        )),
    };

    let mut b_zero = generous_budget();
    b_zero.max_copy_depth = 0;
    assert_eq!(
        expect_err(&program, &b_zero),
        SourceCopyBudgetExceeded {
            resource: SourceCopyResource::CopyDepth,
            limit: 0,
            observed: Some(1),
            function_index: None,
            child_index: None,
        }
    );

    let mut b_one = generous_budget();
    b_one.max_copy_depth = 1;
    assert_eq!(
        expect_err(&program, &b_one),
        SourceCopyBudgetExceeded {
            resource: SourceCopyResource::CopyDepth,
            limit: 1,
            observed: Some(2),
            function_index: None,
            child_index: Some(0),
        }
    );

    // An empty Call has no child expressions and therefore succeeds at depth 1.
    let empty_call_program = a::Program {
        functions: vec![],
        entry: Arc::new(a::Expr::Call("nullary".to_string(), vec![])),
    };
    let _ = expect_ok(&empty_call_program, &b_one);
}

#[test]
fn shared_arc_expr_dag_counts_every_occurrence_without_deduplication() {
    let leaf = Arc::new(a::Expr::Literal(a::Value::UInt(1)));
    let pair = Arc::new(a::Expr::Binary(
        a::BinaryOp::Add,
        Arc::clone(&leaf),
        Arc::clone(&leaf),
    ));
    // Root Binary(1) + left pair(3) + right pair(3) = 7 occurrences total.
    let program = a::Program {
        functions: vec![],
        entry: Arc::new(a::Expr::Binary(
            a::BinaryOp::Add,
            Arc::clone(&pair),
            pair,
        )),
    };

    let mut budget = generous_budget();
    budget.max_expr_nodes = 6;
    // Order of admission:
    // 1: root Binary
    // 2, 3: left pair (child 0) and right pair (child 1) at root expansion
    // 4, 5: left pair's two children at left pair expansion
    // 6, 7: right pair's two children at right pair expansion -> child 1 fails at 7 > 6.
    assert_eq!(
        expect_err(&program, &budget),
        SourceCopyBudgetExceeded {
            resource: SourceCopyResource::ExprNodes,
            limit: 6,
            observed: Some(7),
            function_index: None,
            child_index: Some(1),
        }
    );

    budget.max_expr_nodes = 7;
    let _ = expect_ok(&program, &budget);
}

#[test]
fn match_arms_and_pattern_binder_name_bytes_are_checked_before_allocation() {
    let lit = Arc::new(a::Expr::Literal(a::Value::UInt(1)));
    let program = a::Program {
        functions: vec![],
        entry: Arc::new(a::Expr::Match(
            Arc::clone(&lit),
            vec![
                a::Arm {
                    pattern: a::Pattern::Leaf("x".to_string()),
                    body: Arc::clone(&lit),
                },
                a::Arm {
                    pattern: a::Pattern::Branch("left_b".to_string(), "right_b".to_string()),
                    body: lit,
                },
            ],
        )),
    };

    let mut b_arms = generous_budget();
    b_arms.max_match_arms = 1;
    assert_eq!(
        expect_err(&program, &b_arms),
        SourceCopyBudgetExceeded {
            resource: SourceCopyResource::MatchArms,
            limit: 1,
            observed: Some(2),
            function_index: None,
            child_index: None,
        }
    );

    // "x" (1) + "left_b" (6) = 7; "right_b" (7) brings total to 14 > 10 on arm index 1 (child_index = Some(2)).
    let mut b_names = generous_budget();
    b_names.max_name_bytes = 10;
    assert_eq!(
        expect_err(&program, &b_names),
        SourceCopyBudgetExceeded {
            resource: SourceCopyResource::NameBytes,
            limit: 10,
            observed: Some(14),
            function_index: None,
            child_index: Some(2),
        }
    );
}

#[test]
fn list_elements_and_tree_nodes_depth_are_bounded_per_occurrence() {
    let list_expr = Arc::new(a::Expr::Literal(a::Value::List(vec![10, 20, 30])));
    let list_prog = a::Program {
        functions: vec![],
        entry: Arc::new(a::Expr::Binary(
            a::BinaryOp::Eq,
            Arc::clone(&list_expr),
            list_expr,
        )),
    };

    let mut b_list = generous_budget();
    b_list.max_list_elements = 5;
    assert_eq!(
        expect_err(&list_prog, &b_list),
        SourceCopyBudgetExceeded {
            resource: SourceCopyResource::ListElements,
            limit: 5,
            observed: Some(6),
            function_index: None,
            child_index: None,
        }
    );
    b_list.max_list_elements = 6;
    let _ = expect_ok(&list_prog, &b_list);

    let shared_leaf = Arc::new(a::Tree::Leaf(7));
    let shared_sub = Arc::new(a::Tree::Branch(Arc::clone(&shared_leaf), shared_leaf));
    // Root Branch(1) + left Branch(1) + 2 Leaves(2) + right Branch(1) + 2 Leaves(2) = 7 tree nodes, depth 3.
    let tree_prog = a::Program {
        functions: vec![],
        entry: Arc::new(a::Expr::Literal(a::Value::Tree(a::Tree::Branch(
            Arc::clone(&shared_sub),
            shared_sub,
        )))),
    };

    let mut b_tdepth = generous_budget();
    b_tdepth.max_tree_depth = 2;
    assert_eq!(
        expect_err(&tree_prog, &b_tdepth),
        SourceCopyBudgetExceeded {
            resource: SourceCopyResource::TreeDepth,
            limit: 2,
            observed: Some(3),
            function_index: None,
            child_index: Some(0),
        }
    );

    let mut b_tnodes = generous_budget();
    b_tnodes.max_tree_nodes = 6;
    assert_eq!(
        expect_err(&tree_prog, &b_tnodes),
        SourceCopyBudgetExceeded {
            resource: SourceCopyResource::TreeNodes,
            limit: 6,
            observed: Some(7),
            function_index: None,
            child_index: Some(1),
        }
    );

    b_tnodes.max_tree_nodes = 7;
    let _ = expect_ok(&tree_prog, &b_tnodes);
}

#[test]
fn checked_addition_overflow_reports_observed_none() {
    let mut counter = usize::MAX;
    let err = check_add(
        SourceCopyResource::NameBytes,
        &mut counter,
        1,
        usize::MAX,
        Some(0),
        Some(2),
    )
    .unwrap_err();
    assert_eq!(
        err,
        SourceCopyError::Budget(SourceCopyBudgetExceeded {
            resource: SourceCopyResource::NameBytes,
            limit: usize::MAX,
            observed: None,
            function_index: Some(0),
            child_index: Some(2),
        })
    );

    let depth_err = check_limit(
        SourceCopyResource::CopyDepth,
        usize::MAX,
        usize::MAX.checked_add(1),
        None,
        Some(0),
    )
    .unwrap_err();
    assert_eq!(
        depth_err,
        SourceCopyError::Budget(SourceCopyBudgetExceeded {
            resource: SourceCopyResource::CopyDepth,
            limit: usize::MAX,
            observed: None,
            function_index: None,
            child_index: Some(0),
        })
    );
}
