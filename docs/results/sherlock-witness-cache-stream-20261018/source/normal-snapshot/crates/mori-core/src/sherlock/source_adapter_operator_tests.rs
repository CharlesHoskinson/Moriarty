//! Author-owned ordinary structural tests, SOURCE_ONLY / UNRUN.
//! These assert the public lowering design, not independent values/work/traces.
//! No evaluator, Machine, owner engine or private oracle is called/read.
use super::*;

fn node(value: Expr) -> ExprRef { Arc::new(value) }
fn name(value: &str) -> ExprRef { node(Expr::Var(value.into())) }
fn bool_node(value: bool) -> ExprRef { node(Expr::Literal(Value::Bool(value))) }
fn primitive(op: BinaryOp, lhs: ExprRef, rhs: ExprRef) -> ExprRef {
    node(Expr::Binary(op, lhs, rhs))
}
fn negate(value: ExprRef) -> ExprRef { node(Expr::Unary(UnaryOp::Not, value)) }
fn eager_pair(body: ExprRef) -> ExprRef {
    node(Expr::Let("$0l".into(), name("left"),
        node(Expr::Let("$0r".into(), name("right"), body))))
}
fn scalar_source(operator: &str, ty: &str) -> Program {
    crate::parse(&format!(
        "language \"moriarty/1\"; agreement Operators {{ action inspect(left: {ty}, right: {ty}) {{ emit \"value\"(left {operator} right); }} }}"
    )).unwrap()
}

#[test]
fn every_existing_core_scalar_binary_operator_has_a_projection() {
    let cases = [
        (CoreOp::Add, "+", "UInt"), (CoreOp::Sub, "-", "UInt"),
        (CoreOp::Mul, "*", "UInt"), (CoreOp::Eq, "==", "UInt"),
        (CoreOp::Ne, "not =", "UInt"), (CoreOp::Lt, "<", "UInt"),
        (CoreOp::Le, "<=", "UInt"), (CoreOp::Gt, ">", "UInt"),
        (CoreOp::Ge, ">=", "UInt"), (CoreOp::And, "and", "Bool"),
        (CoreOp::Or, "or", "Bool"),
    ];
    for (operator, spelling, ty) in cases {
        let source = scalar_source(spelling, ty);
        assert!(matches!(source.expressions.last(),
            Some(CoreExpr::Binary { op, .. }) if *op == operator));
        let mapped = prepare_scalar_action(&source, 0).unwrap();
        assert_eq!(mapped.report.return_type,
            if matches!(operator, CoreOp::Add | CoreOp::Sub | CoreOp::Mul) {
                crate::Type::UInt
            } else { crate::Type::Bool });
        assert_eq!(mapped.report.emit.role, "value-projection; ordered emit effect remains in Core");
        assert!(matches!(&source_projection_report(&source).actions[0],
            ActionProjection::Prepared { .. }));
    }
    for spelling in ["==", "not ="] {
        let source = scalar_source(spelling, "Bool");
        assert_eq!(prepare_scalar_action(&source, 0).unwrap().report.return_type, crate::Type::Bool);
    }
}

#[test]
fn missing_scalar_operators_have_exact_eager_existing_ast_lowerings() {
    let l = "$0l";
    let r = "$0r";
    let cases = [
        ("not =", "UInt", negate(primitive(BinaryOp::Eq, name("left"), name("right")))),
        (">=", "UInt", negate(primitive(BinaryOp::Lt, name("left"), name("right")))),
        (">", "UInt", eager_pair(primitive(BinaryOp::Lt, name(r), name(l)))),
        ("<=", "UInt", eager_pair(negate(primitive(BinaryOp::Lt, name(r), name(l))))),
        ("and", "Bool", eager_pair(node(Expr::If(name(l), name(r), bool_node(false))))),
        ("or", "Bool", eager_pair(node(Expr::If(name(l), bool_node(true), name(r))))),
    ];
    for (operator, ty, expected) in cases {
        let source = scalar_source(operator, ty);
        assert_eq!(prepare_scalar_action(&source, 0).unwrap().function.body, expected);
    }
}

fn generated_paths(value: &ExprRef, path: Vec<usize>, output: &mut Vec<Vec<usize>>) {
    output.push(path.clone());
    let children: Vec<&ExprRef> = match value.as_ref() {
        Expr::Literal(_) | Expr::Var(_) => vec![],
        Expr::Let(_, value, body) | Expr::Binary(_, value, body) => vec![value, body],
        Expr::Unary(_, value) => vec![value],
        Expr::If(condition, yes, no) => vec![condition, yes, no],
        _ => panic!("scalar projection emitted an unrelated expression form"),
    };
    for (position, child) in children.into_iter().enumerate() {
        let mut child_path = path.clone(); child_path.push(position);
        generated_paths(child, child_path, output);
    }
}

#[test]
fn lowering_reports_cover_actual_generated_nodes_and_source_origins_in_preorder() {
    for (operator, ty) in [("not =", "UInt"), (">=", "UInt"), (">", "UInt"),
        ("<=", "UInt"), ("and", "Bool"), ("or", "Bool")] {
        let source = scalar_source(operator, ty);
        let mapped = prepare_scalar_action(&source, 0).unwrap();
        let mut paths = Vec::new();
        generated_paths(&mapped.function.body, vec![], &mut paths);
        assert_eq!(mapped.report.occurrences.iter().map(|site| site.body_path.clone())
            .collect::<Vec<_>>(), paths);
        for site in &mapped.report.occurrences {
            assert_eq!(site.action_index, 0);
            assert_eq!(site.statement_index, 0);
            assert_eq!(site.source_expression.as_ref(),
                site.expression.map(|id| &source.expressions[usize::from(id)]));
            if site.node_kind.starts_with("lowering-") {
                assert_eq!(site.expression, Some(2));
            }
        }
        let source_leaves: Vec<_> = mapped.report.occurrences.iter()
            .filter(|site| site.node_kind == "name")
            .map(|site| site.expression).collect();
        assert_eq!(source_leaves, vec![Some(0), Some(1)]);
        let serialized = serde_json::to_value(&mapped.report).unwrap();
        assert_eq!(serialized["occurrences"].as_array().unwrap().len(), paths.len());
    }
}

#[test]
fn reversed_comparisons_and_boolean_ops_keep_operand_initializers_in_source_order() {
    for expression in ["(left + 1) > (right - 2)", "(left + 1) <= (right - 2)",
        "((left + 1) < right) and ((right - 2) < left)",
        "((left + 1) < right) or ((right - 2) < left)"] {
        let source = crate::parse(&format!(
            "language \"moriarty/1\"; agreement Eager {{ action inspect(left: UInt, right: UInt) {{ emit \"out\"({expression}); }} }}"
        )).unwrap();
        let mapped = prepare_scalar_action(&source, 0).unwrap();
        let Expr::Let(_, lhs, body) = mapped.function.body.as_ref() else {
            panic!("left source operand must be evaluated first");
        };
        let Expr::Let(_, rhs, _) = body.as_ref() else {
            panic!("right source operand must be evaluated before the comparison/If body");
        };
        // Compare the actual source-operand subtrees, not evaluated results.
        let mut lhs_paths = Vec::new(); generated_paths(lhs, vec![], &mut lhs_paths);
        let mut rhs_paths = Vec::new(); generated_paths(rhs, vec![], &mut rhs_paths);
        assert!(!lhs_paths.is_empty() && !rhs_paths.is_empty());
        let left_first = mapped.report.occurrences.iter()
            .position(|site| site.body_path == [0]).unwrap();
        let right_first = mapped.report.occurrences.iter()
            .position(|site| site.body_path == [1, 0]).unwrap();
        assert!(left_first < right_first);
        assert!(matches!(lhs.as_ref(), Expr::Binary(BinaryOp::Add | BinaryOp::Lt, _, _)));
        assert!(matches!(rhs.as_ref(), Expr::Binary(BinaryOp::Sub | BinaryOp::Lt, _, _)));
    }
}

fn generated_binders(value: &ExprRef, output: &mut Vec<String>) {
    match value.as_ref() {
        Expr::Let(binder, value, body) => {
            if binder.starts_with('$') { output.push(binder.clone()); }
            generated_binders(value, output); generated_binders(body, output);
        }
        Expr::Binary(_, left, right) => {
            generated_binders(left, output); generated_binders(right, output);
        }
        Expr::Unary(_, value) => generated_binders(value, output),
        Expr::If(condition, yes, no) => {
            generated_binders(condition, output); generated_binders(yes, output);
            generated_binders(no, output);
        }
        Expr::Literal(_) | Expr::Var(_) => {}
        _ => panic!("unrelated expression form"),
    }
}

#[test]
fn repeated_dag_edges_have_distinct_deterministic_hygienic_generated_symbols() {
    let source = Program {
        language: crate::LANGUAGE.into(), semantics: crate::SEMANTICS.into(),
        name: "Shared".into(), states: vec![], assets: vec![],
        expressions: vec![CoreExpr::Name { name: "left".into() },
            CoreExpr::Name { name: "right".into() },
            CoreExpr::Binary { op: CoreOp::Gt, lhs: 0, rhs: 1 },
            CoreExpr::Binary { op: CoreOp::And, lhs: 2, rhs: 2 }],
        actions: vec![crate::Action { name: "inspect".into(),
            inputs: vec![crate::InputDecl { name: "left".into(), ty: crate::Type::UInt },
                crate::InputDecl { name: "right".into(), ty: crate::Type::UInt }],
            statements: vec![Statement::Emit { name: "out".into(), value: 3 }] }],
    };
    let first = prepare_scalar_action(&source, 0).unwrap();
    assert_eq!(first, prepare_scalar_action(&source, 0).unwrap());
    let mut binders = Vec::new(); generated_binders(&first.function.body, &mut binders);
    assert!(!binders.is_empty());
    assert_eq!(binders.iter().collect::<std::collections::BTreeSet<_>>().len(), binders.len());
    assert!(binders.iter().all(|binder| binder.contains('$') && binder != "left" && binder != "right"));
    let mut paths = Vec::new(); generated_paths(&first.function.body, vec![], &mut paths);
    assert_eq!(first.report.occurrences.iter().map(|site| site.body_path.clone())
        .collect::<Vec<_>>(), paths);
}

#[test]
fn compact_symbol_supply_avoids_every_already_occupied_generated_name() {
    let source = scalar_source(">", "UInt");
    let mut supply = LoweringSymbols::new(&source);
    assert!(supply.occupied.contains("Operators"));
    assert!(supply.occupied.contains("inspect"));
    assert!(supply.occupied.contains("left") && supply.occupied.contains("right"));
    // Ordinary compiler fresh-name lifecycle: reserve existing generated names,
    // then request a colliding base. No invalid Core input is constructed.
    supply.occupied.insert("$0l".into());
    supply.occupied.insert("$0l$".into());
    assert_eq!(supply.fresh(0, "l"), "$0l$$");
    assert_eq!(supply.fresh(0, "l"), "$0l$$$");
    assert_eq!(supply.fresh(0, "r"), "$0r");
}
