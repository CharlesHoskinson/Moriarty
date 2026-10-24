//! Finite parser-boundary checks under the standing Sol parser-depth exception.
//! No malformed witness, owner admission guard, evaluator or runtime tests.
use mori_core::sherlock_nominal_source::{parse_nominal_source_bounded,DraftParseBudget,
    DraftError,DraftBudgetResource as R,DraftBudgetExceeded};
const LITERAL:&str="language \"moriarty-sherlock-nominal/1\"; program Literal { entry 7; }";
fn budget()->DraftParseBudget {
    DraftParseBudget {max_source_bytes:4096,max_tokens:1024,max_functions:8,
        max_parameters_per_function:8,max_total_parameters:32,max_ast_occurrences:256,
        max_parser_recursion_depth:64,max_structural_ast_depth:32}
}
fn error(source:&str,b:DraftParseBudget,resource:R,limit:usize,observed:usize) {
    match parse_nominal_source_bounded(source,b) {
        Err(DraftError::Budget(actual))=>assert_eq!(actual,DraftBudgetExceeded {resource,limit,observed}),
        Err(other)=>panic!("unexpected parser diagnostic: {other:?}"),
        Ok(_)=>panic!("expected explicit caller parser extent"),
    }
}
#[test]
fn source_bytes_and_eof_inclusive_tokens_keep_exact_existing_resources() {
    let mut b=budget();b.max_source_bytes=LITERAL.len();b.max_tokens=11;
    assert!(parse_nominal_source_bounded(LITERAL,b).is_ok());
    b.max_source_bytes=LITERAL.len()-1;error(LITERAL,b,R::SourceBytes,LITERAL.len()-1,LITERAL.len());
    b=budget();b.max_tokens=10;error(LITERAL,b,R::Tokens,10,11);
    b.max_tokens=0;error(LITERAL,b,R::Tokens,0,1);
}
#[test]
fn expression_occurrences_and_structural_depth_have_inclusive_boundaries() {
    let mut b=budget();b.max_ast_occurrences=1;b.max_structural_ast_depth=1;
    assert!(parse_nominal_source_bounded(LITERAL,b).is_ok());
    b.max_ast_occurrences=0;error(LITERAL,b,R::AstOccurrences,0,1);
    b=budget();b.max_structural_ast_depth=0;error(LITERAL,b,R::StructuralAstDepth,0,1);
    let binary="language \"moriarty-sherlock-nominal/1\"; program Binary { entry 1 + 2 * 3; }";
    b=budget();b.max_ast_occurrences=5;b.max_structural_ast_depth=3;
    assert!(parse_nominal_source_bounded(binary,b).is_ok());
    b.max_ast_occurrences=4;error(binary,b,R::AstOccurrences,4,5);
    b=budget();b.max_structural_ast_depth=2;error(binary,b,R::StructuralAstDepth,2,3);
}
#[test]
fn parser_descent_includes_parentheses_and_nested_type_applications() {
    let mut b=budget();b.max_parser_recursion_depth=2;
    assert!(parse_nominal_source_bounded(LITERAL,b).is_ok());
    b.max_parser_recursion_depth=1;error(LITERAL,b,R::ParserRecursionDepth,1,2);
    let nested="language \"moriarty-sherlock-nominal/1\"; program Nested { type Box<T> { Box(value:T); } fn f(b:Box<Box<UInt>>):UInt { 1 } entry 0; }";
    b=budget();b.max_parser_recursion_depth=3;
    assert!(parse_nominal_source_bounded(nested,b).is_ok());
    b.max_parser_recursion_depth=2;error(nested,b,R::ParserRecursionDepth,2,3);
    let parentheses="language \"moriarty-sherlock-nominal/1\"; program Parens { entry (7); }";
    b=budget();b.max_parser_recursion_depth=3;error(parentheses,b,R::ParserRecursionDepth,3,4);
}
#[test]
fn function_and_signature_parameter_extents_keep_existing_meanings() {
    let source="language \"moriarty-sherlock-nominal/1\"; program Function { fn f(x:UInt):UInt { x } entry f(1); }";
    let mut b=budget();b.max_functions=1;b.max_parameters_per_function=1;b.max_total_parameters=1;
    assert!(parse_nominal_source_bounded(source,b).is_ok());
    b.max_functions=0;error(source,b,R::Functions,0,1);
    b=budget();b.max_parameters_per_function=0;error(source,b,R::ParametersPerFunction,0,1);
    b=budget();b.max_total_parameters=0;error(source,b,R::TotalParameters,0,1);
}
#[test]
fn nested_nominal_patterns_count_each_occurrence_with_an_explicit_budget() {
    let source="language \"moriarty-sherlock-nominal/1\"; program Pattern { type Box<T> { Box(value:T); } entry match 7 { Box<UInt>::Box(Box<UInt>::Box(x)) => 0, wildcard => 1 }; }";
    // Match, literal scrutinee, two literal arm bodies, three nested patterns,
    // and one wildcard: eight occurrences; type annotation nodes are token bounded.
    let mut b=budget();b.max_ast_occurrences=8;
    assert!(parse_nominal_source_bounded(source,b).is_ok());
    b.max_ast_occurrences=7;error(source,b,R::AstOccurrences,7,8);
}
