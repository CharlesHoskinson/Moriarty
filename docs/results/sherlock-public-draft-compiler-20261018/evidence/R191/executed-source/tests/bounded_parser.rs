// Source-only test proposal. Integrator must select wiring after fresh review.
use super::*;

fn limits() -> DraftParseBudget {
    DraftParseBudget {
        max_source_bytes: mori_lexer::MAX_SOURCE_BYTES,
        max_tokens: mori_lexer::MAX_TOKENS,
        max_functions: 3,
        max_parameters_per_function: 3,
        max_total_parameters: 4,
        max_ast_occurrences: 64,
        max_parser_recursion_depth: 64,
        max_structural_ast_depth: 16,
    }
}

fn source(entry: &str) -> String {
    format!("language \"moriarty-sherlock-draft/1\"; program Demo {{ entry {entry}; }}")
}

fn exceeds(source: &str, budget: DraftParseBudget, resource: DraftBudgetResource,
    limit: usize, observed: usize)
{
    assert_eq!(parse_draft_source_bounded(source, budget),
        Err(DraftError::Budget(DraftBudgetExceeded { resource, limit, observed })));
}

#[test]
fn bounded_positive_matches_legacy_program_and_provenance() {
    for entry in ["1 + 2 * 3", "let x = cons(1, nil) in match x { cons(h, t) => h, nil => 0 }",
        "if true { leaf(1) } else { branch(leaf(2), leaf(3)) }"]
    {
        let text = source(entry);
        assert_eq!(parse_draft_source_bounded(&text, limits()), parse_draft_source(&text));
        assert!(parse_draft_source_bounded(&text, limits()).is_ok());
    }
    let text = "language \"moriarty-sherlock-draft/1\"; program Demo { fn f(x: UInt): UInt { f(x) } entry f(1); }";
    assert_eq!(parse_draft_source_bounded(text, limits()), parse_draft_source(text));
    assert!(parse_draft_source_bounded(text, limits()).is_ok());
}

fn expected_expr(expression: Expr) -> ExprRef { Arc::new(expression) }
fn expected_uint(value: u128) -> ExprRef { expected_expr(Expr::Literal(Value::UInt(value))) }
fn expected_var(name: &str) -> ExprRef { expected_expr(Expr::Var(name.into())) }
fn expected_call(name: &str, arguments: Vec<ExprRef>) -> ExprRef {
    expected_expr(Expr::Call(name.into(), arguments))
}
fn expected_binary(op: BinaryOp, left: ExprRef, right: ExprRef) -> ExprRef {
    expected_expr(Expr::Binary(op, left, right))
}
fn expected_leaf(child: ExprRef) -> ExprRef { expected_expr(Expr::Unary(UnaryOp::Leaf, child)) }
fn expected_span(start: u32, end: u32) -> SourceSpan { SourceSpan { start, end } }
fn expected_binder(name: &str, start: u32, end: u32) -> BinderSite {
    BinderSite { name: name.into(), span: expected_span(start, end) }
}
fn expected_site(function_index: Option<usize>, child_path: &[usize],
    start: u32, end: u32, kind: &'static str) -> ExpressionSite
{
    ExpressionSite { source: SourceLocation { function_index, child_path: child_path.to_vec() },
        span: expected_span(start, end), kind }
}

#[test]
fn precedence_has_independently_expected_ast() {
    let parsed = parse_draft_source_bounded(&source("1 + 2 * 3"), limits()).unwrap();
    assert_eq!(parsed.program.as_ref(), &Program {
        functions: vec![],
        entry: expected_binary(BinaryOp::Add, expected_uint(1),
            expected_binary(BinaryOp::Mul, expected_uint(2), expected_uint(3))),
    });
}

// Exact fixture bytes and every provenance literal below come from the frozen
// independent R115 EXPECTED.json / DERIVATION.md, not either parser's output.
// AST shapes are transcribed from R115's independent row/child derivation.
#[test]
fn chain_total_matches_independent_ast_and_complete_provenance() {
    let text = include_str!("inputs/chain-total.mori");
    let parsed = parse_draft_source_bounded(text, limits()).unwrap();
    let expected = Program {
        functions: vec![Function {
            name: "sum".into(),
            parameters: vec![Parameter { name: "xs".into(), ty: Type::List }],
            return_type: Type::UInt,
            body: expected_expr(Expr::Match(expected_var("xs"), vec![
                Arm { pattern: Pattern::Nil, body: expected_uint(0) },
                Arm { pattern: Pattern::Cons("h".into(), "t".into()),
                    body: expected_binary(BinaryOp::Add, expected_var("h"),
                        expected_call("sum", vec![expected_var("t")])) },
            ])),
        }],
        entry: expected_call("sum", vec![expected_binary(BinaryOp::Cons, expected_uint(2),
            expected_binary(BinaryOp::Cons, expected_uint(5),
                expected_expr(Expr::Literal(Value::List(vec![])))))]),
    };
    assert_eq!(parsed.name, "ChainTotal");
    assert_eq!(parsed.program.as_ref(), &expected);
    assert_eq!(parsed.provenance, SourceProvenance {
        language_span: expected_span(9, 36),
        program_name_span: expected_span(46, 56),
        functions: vec![FunctionSite {
            index: 0, span: expected_span(61, 140), name_span: expected_span(64, 67),
            parameters: vec![ParameterSite {
                binder: expected_binder("xs", 68, 70), type_span: expected_span(72, 76),
            }],
            return_type_span: expected_span(79, 83),
        }],
        expressions: vec![
            expected_site(None, &[], 149, 173, "call"),
            expected_site(None, &[0], 153, 172, "cons"),
            expected_site(None, &[0, 0], 158, 159, "uint"),
            expected_site(None, &[0, 1], 160, 171, "cons"),
            expected_site(None, &[0, 1, 0], 165, 166, "uint"),
            expected_site(None, &[0, 1, 1], 167, 170, "nil"),
            expected_site(Some(0), &[], 90, 136, "match"),
            expected_site(Some(0), &[0], 96, 98, "var"),
            expected_site(Some(0), &[1], 108, 109, "uint"),
            expected_site(Some(0), &[2], 124, 134, "add"),
            expected_site(Some(0), &[2, 0], 124, 125, "var"),
            expected_site(Some(0), &[2, 1], 128, 134, "call"),
            expected_site(Some(0), &[2, 1, 0], 132, 133, "var"),
        ],
        patterns: vec![
            PatternSite { match_source: SourceLocation { function_index: Some(0), child_path: vec![] },
                arm_index: 0, span: expected_span(101, 104), binders: vec![] },
            PatternSite { match_source: SourceLocation { function_index: Some(0), child_path: vec![] },
                arm_index: 1, span: expected_span(111, 120),
                binders: vec![expected_binder("h", 116, 117), expected_binder("t", 118, 119)] },
        ],
    });
}

#[test]
fn branch_total_matches_independent_ast_and_complete_provenance() {
    let text = include_str!("inputs/branch-total.mori");
    let parsed = parse_draft_source_bounded(text, limits()).unwrap();
    let expected = Program {
        functions: vec![Function {
            name: "fold".into(),
            parameters: vec![Parameter { name: "t".into(), ty: Type::Tree }],
            return_type: Type::UInt,
            body: expected_expr(Expr::Match(expected_var("t"), vec![
                Arm { pattern: Pattern::Leaf("n".into()), body: expected_var("n") },
                Arm { pattern: Pattern::Branch("l".into(), "r".into()),
                    body: expected_binary(BinaryOp::Add,
                        expected_call("fold", vec![expected_var("l")]),
                        expected_call("fold", vec![expected_var("r")])) },
            ])),
        }],
        entry: expected_call("fold", vec![expected_binary(BinaryOp::Branch,
            expected_leaf(expected_uint(4)), expected_binary(BinaryOp::Branch,
                expected_leaf(expected_uint(1)), expected_leaf(expected_uint(3))))]),
    };
    assert_eq!(parsed.name, "BranchTotal");
    assert_eq!(parsed.program.as_ref(), &expected);
    assert_eq!(parsed.provenance, SourceProvenance {
        language_span: expected_span(9, 36),
        program_name_span: expected_span(46, 57),
        functions: vec![FunctionSite {
            index: 0, span: expected_span(62, 153), name_span: expected_span(65, 69),
            parameters: vec![ParameterSite {
                binder: expected_binder("t", 70, 71), type_span: expected_span(73, 77),
            }],
            return_type_span: expected_span(80, 84),
        }],
        expressions: vec![
            expected_site(None, &[], 162, 207, "call"),
            expected_site(None, &[0], 167, 206, "branch"),
            expected_site(None, &[0, 0], 174, 181, "leaf"),
            expected_site(None, &[0, 0, 0], 179, 180, "uint"),
            expected_site(None, &[0, 1], 182, 205, "branch"),
            expected_site(None, &[0, 1, 0], 189, 196, "leaf"),
            expected_site(None, &[0, 1, 0, 0], 194, 195, "uint"),
            expected_site(None, &[0, 1, 1], 197, 204, "leaf"),
            expected_site(None, &[0, 1, 1, 0], 202, 203, "uint"),
            expected_site(Some(0), &[], 91, 149, "match"),
            expected_site(Some(0), &[0], 97, 98, "var"),
            expected_site(Some(0), &[1], 112, 113, "var"),
            expected_site(Some(0), &[2], 130, 147, "add"),
            expected_site(Some(0), &[2, 0], 130, 137, "call"),
            expected_site(Some(0), &[2, 0, 0], 135, 136, "var"),
            expected_site(Some(0), &[2, 1], 140, 147, "call"),
            expected_site(Some(0), &[2, 1, 0], 145, 146, "var"),
        ],
        patterns: vec![
            PatternSite { match_source: SourceLocation { function_index: Some(0), child_path: vec![] },
                arm_index: 0, span: expected_span(101, 108),
                binders: vec![expected_binder("n", 106, 107)] },
            PatternSite { match_source: SourceLocation { function_index: Some(0), child_path: vec![] },
                arm_index: 1, span: expected_span(115, 126),
                binders: vec![expected_binder("l", 122, 123), expected_binder("r", 124, 125)] },
        ],
    });
}

#[test]
fn byte_and_token_limits_are_inclusive_and_count_eof() {
    let text = source("1");
    let mut budget = limits();
    budget.max_source_bytes = text.len();
    // language, string, ;, program, Demo, {, entry, 1, ;, }, EOF.
    budget.max_tokens = 11;
    assert!(parse_draft_source_bounded(&text, budget).is_ok());
    budget.max_source_bytes -= 1;
    exceeds(&text, budget, DraftBudgetResource::SourceBytes, text.len() - 1, text.len());
    budget.max_source_bytes = text.len();
    budget.max_tokens = 10;
    exceeds(&text, budget, DraftBudgetResource::Tokens, 10, 11);
    budget.max_tokens = 0;
    exceeds(&text, budget, DraftBudgetResource::Tokens, 0, 1);
}

#[test]
fn function_and_parameter_limits_are_separate() {
    let text = "language \"moriarty-sherlock-draft/1\"; program Demo { fn f(a: UInt, b: UInt): UInt { a } fn g(c: UInt): UInt { c } entry f(1, 2); }";
    let mut budget = limits();
    budget.max_functions = 2;
    budget.max_parameters_per_function = 2;
    budget.max_total_parameters = 3;
    assert!(parse_draft_source_bounded(text, budget).is_ok());
    budget.max_functions = 1;
    exceeds(text, budget, DraftBudgetResource::Functions, 1, 2);
    budget.max_functions = 2;
    budget.max_parameters_per_function = 1;
    exceeds(text, budget, DraftBudgetResource::ParametersPerFunction, 1, 2);
    budget.max_parameters_per_function = 2;
    budget.max_total_parameters = 2;
    exceeds(text, budget, DraftBudgetResource::TotalParameters, 2, 3);
}

#[test]
fn occurrences_include_expression_and_pattern_occurrences() {
    let text = source("match nil { nil => 1 }");
    let mut budget = limits();
    // Match, scrutinee nil, pattern nil, body UInt.
    budget.max_ast_occurrences = 4;
    assert!(parse_draft_source_bounded(&text, budget).is_ok());
    budget.max_ast_occurrences = 3;
    exceeds(&text, budget, DraftBudgetResource::AstOccurrences, 3, 4);
    let text = source("1 + 2");
    budget.max_ast_occurrences = 3;
    assert!(parse_draft_source_bounded(&text, budget).is_ok());
    budget.max_ast_occurrences = 2;
    exceeds(&text, budget, DraftBudgetResource::AstOccurrences, 2, 3);
}

#[test]
fn pratt_structural_depth_differs_from_recursive_descent_depth() {
    let mut budget = limits();
    budget.max_parser_recursion_depth = 4;
    budget.max_structural_ast_depth = 4;
    assert!(parse_draft_source_bounded(&source("1 + 2 + 3 + 4"), budget).is_ok());
    budget.max_structural_ast_depth = 3;
    exceeds(&source("1 + 2 + 3 + 4"), budget,
        DraftBudgetResource::StructuralAstDepth, 3, 4);
    // Parentheses add no AST nodes or structural depth, but do recurse.
    budget.max_structural_ast_depth = 1;
    exceeds(&source("((1))"), budget,
        DraftBudgetResource::ParserRecursionDepth, 4, 5);
    budget.max_parser_recursion_depth = 5;
    exceeds(&source("((1))"), budget,
        DraftBudgetResource::ParserRecursionDepth, 5, 6);
    budget.max_parser_recursion_depth = 6;
    assert!(parse_draft_source_bounded(&source("((1))"), budget).is_ok());
}

#[test]
fn literal_has_exact_parser_recursion_depth_two() {
    let mut budget = limits();
    budget.max_parser_recursion_depth = 0;
    exceeds(&source("1"), budget, DraftBudgetResource::ParserRecursionDepth, 0, 1);
    budget.max_parser_recursion_depth = 1;
    exceeds(&source("1"), budget, DraftBudgetResource::ParserRecursionDepth, 1, 2);
    budget.max_parser_recursion_depth = 2;
    assert!(parse_draft_source_bounded(&source("1"), budget).is_ok());
}

#[test]
fn nonbinary_constructor_has_exact_structural_depth_two() {
    let mut budget = limits();
    budget.max_structural_ast_depth = 1;
    exceeds(&source("leaf(1)"), budget, DraftBudgetResource::StructuralAstDepth, 1, 2);
    budget.max_structural_ast_depth = 2;
    assert!(parse_draft_source_bounded(&source("leaf(1)"), budget).is_ok());
}

#[test]
fn zero_limits_allow_absent_functions_and_parameters() {
    let mut budget = limits();
    budget.max_functions = 0;
    budget.max_parameters_per_function = 0;
    budget.max_total_parameters = 0;
    budget.max_ast_occurrences = 1;
    budget.max_parser_recursion_depth = 2;
    budget.max_structural_ast_depth = 1;
    assert!(parse_draft_source_bounded(&source("1"), budget).is_ok());
    budget.max_ast_occurrences = 0;
    exceeds(&source("1"), budget, DraftBudgetResource::AstOccurrences, 0, 1);
    budget.max_ast_occurrences = 1;
    budget.max_structural_ast_depth = 0;
    exceeds(&source("1"), budget, DraftBudgetResource::StructuralAstDepth, 0, 1);
}

#[test]
fn legacy_diagnostics_remain_the_original_payloads() {
    for text in [source("340282366920938463463374607431768211456"), source("1 +"), source("@")] {
        assert_eq!(parse_draft_source_bounded(&text, limits()), parse_draft_source(&text));
        assert!(parse_draft_source(&text).is_err());
    }
}
