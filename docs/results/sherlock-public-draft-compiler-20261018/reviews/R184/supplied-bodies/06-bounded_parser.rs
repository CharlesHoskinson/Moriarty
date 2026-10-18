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
    budget.max_parser_recursion_depth = 6;
    assert!(parse_draft_source_bounded(&source("((1))"), budget).is_ok());
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
