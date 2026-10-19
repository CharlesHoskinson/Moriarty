//! Expected trees, source coordinates and paths below are handwritten literals.
//! These tests prepare source; they never evaluate a Sherlock expression.
use crate::sherlock::{
    Arm, BinaryOp, Expr, ExprRef, Function, Parameter, Pattern, Program, Type, UnaryOp, Value,
};
use crate::sherlock_draft::*;
use std::sync::Arc;

pub(crate) const FACTORIAL: &str = include_str!("specimens/factorial.mori");
pub(crate) const TREEFOLD: &str = include_str!("specimens/treefold.mori");

fn e(expression: Expr) -> ExprRef {
    Arc::new(expression)
}
fn u(n: u128) -> ExprRef {
    e(Expr::Literal(Value::UInt(n)))
}
fn b(value: bool) -> ExprRef {
    e(Expr::Literal(Value::Bool(value)))
}
fn v(name: &str) -> ExprRef {
    e(Expr::Var(name.into()))
}
fn binary(op: BinaryOp, left: ExprRef, right: ExprRef) -> ExprRef {
    e(Expr::Binary(op, left, right))
}
fn unary(op: UnaryOp, value: ExprRef) -> ExprRef {
    e(Expr::Unary(op, value))
}
fn call(name: &str, args: Vec<ExprRef>) -> ExprRef {
    e(Expr::Call(name.into(), args))
}
fn parameter(name: &str, ty: Type) -> Parameter {
    Parameter {
        name: name.into(),
        ty,
    }
}

pub(crate) fn factorial_expected() -> Program {
    Program {
        functions: vec![Function {
            name: "fact".into(),
            parameters: vec![parameter("n", Type::UInt)],
            return_type: Type::UInt,
            body: e(Expr::If(
                binary(BinaryOp::Eq, v("n"), u(0)),
                u(1),
                binary(
                    BinaryOp::Mul,
                    v("n"),
                    call("fact", vec![binary(BinaryOp::Sub, v("n"), u(1))]),
                ),
            )),
        }],
        entry: call("fact", vec![u(4)]),
    }
}

pub(crate) fn treefold_expected() -> Program {
    Program {
        functions: vec![Function {
            name: "fold".into(),
            parameters: vec![parameter("t", Type::Tree)],
            return_type: Type::UInt,
            body: e(Expr::Match(
                v("t"),
                vec![
                    Arm {
                        pattern: Pattern::Leaf("x".into()),
                        body: v("x"),
                    },
                    Arm {
                        pattern: Pattern::Branch("l".into(), "r".into()),
                        body: binary(
                            BinaryOp::Add,
                            call("fold", vec![v("l")]),
                            call("fold", vec![v("r")]),
                        ),
                    },
                ],
            )),
        }],
        entry: call(
            "fold",
            vec![binary(
                BinaryOp::Branch,
                unary(UnaryOp::Leaf, u(3)),
                unary(UnaryOp::Leaf, u(4)),
            )],
        ),
    }
}

// Coordinates are 1-based line numbers, 0-based UTF-8 byte columns, half-open.
// This helper translates literal coordinates only; it does not lex/parse source.
fn span(source: &str, line: usize, start: usize, end: usize) -> SourceSpan {
    let offset: usize = source
        .split_inclusive('\n')
        .take(line - 1)
        .map(str::len)
        .sum();
    SourceSpan {
        start: (offset + start) as u32,
        end: (offset + end) as u32,
    }
}
fn location(function_index: Option<usize>, path: &[usize]) -> SourceLocation {
    SourceLocation {
        function_index,
        child_path: path.to_vec(),
    }
}
fn site(
    source: &str,
    owner: Option<usize>,
    path: &[usize],
    line: usize,
    start: usize,
    end: usize,
    kind: &'static str,
) -> ExpressionSite {
    ExpressionSite {
        source: location(owner, path),
        span: span(source, line, start, end),
        kind,
    }
}
fn binder(source: &str, name: &str, line: usize, start: usize, end: usize) -> BinderSite {
    BinderSite {
        name: name.into(),
        span: span(source, line, start, end),
    }
}

pub(crate) fn factorial_provenance() -> SourceProvenance {
    use std::option::Option::{None as E, Some as F};
    SourceProvenance {
        language_span: span(FACTORIAL, 1, 9, 36),
        program_name_span: span(FACTORIAL, 2, 8, 17),
        functions: vec![FunctionSite {
            index: 0,
            span: span(FACTORIAL, 3, 2, 69),
            name_span: span(FACTORIAL, 3, 5, 9),
            parameters: vec![ParameterSite {
                binder: binder(FACTORIAL, "n", 3, 10, 11),
                type_span: span(FACTORIAL, 3, 13, 17),
            }],
            return_type_span: span(FACTORIAL, 3, 20, 24),
        }],
        expressions: vec![
            site(FACTORIAL, E, &[], 4, 8, 15, "call"),
            site(FACTORIAL, E, &[0], 4, 13, 14, "uint"),
            site(FACTORIAL, F(0), &[], 3, 27, 67, "if"),
            site(FACTORIAL, F(0), &[0], 3, 30, 36, "eq"),
            site(FACTORIAL, F(0), &[0, 0], 3, 30, 31, "var"),
            site(FACTORIAL, F(0), &[0, 1], 3, 35, 36, "uint"),
            site(FACTORIAL, F(0), &[1], 3, 39, 40, "uint"),
            site(FACTORIAL, F(0), &[2], 3, 50, 65, "mul"),
            site(FACTORIAL, F(0), &[2, 0], 3, 50, 51, "var"),
            site(FACTORIAL, F(0), &[2, 1], 3, 54, 65, "call"),
            site(FACTORIAL, F(0), &[2, 1, 0], 3, 59, 64, "sub"),
            site(FACTORIAL, F(0), &[2, 1, 0, 0], 3, 59, 60, "var"),
            site(FACTORIAL, F(0), &[2, 1, 0, 1], 3, 63, 64, "uint"),
        ],
        patterns: vec![],
    }
}

pub(crate) fn treefold_provenance() -> SourceProvenance {
    use std::option::Option::{None as E, Some as F};
    SourceProvenance {
        language_span: span(TREEFOLD, 1, 9, 36),
        program_name_span: span(TREEFOLD, 2, 8, 16),
        functions: vec![FunctionSite {
            index: 0,
            span: span(TREEFOLD, 3, 2, 88),
            name_span: span(TREEFOLD, 3, 5, 9),
            parameters: vec![ParameterSite {
                binder: binder(TREEFOLD, "t", 3, 10, 11),
                type_span: span(TREEFOLD, 3, 13, 17),
            }],
            return_type_span: span(TREEFOLD, 3, 20, 24),
        }],
        expressions: vec![
            site(TREEFOLD, E, &[], 4, 8, 38, "call"),
            site(TREEFOLD, E, &[0], 4, 13, 37, "branch"),
            site(TREEFOLD, E, &[0, 0], 4, 20, 27, "leaf"),
            site(TREEFOLD, E, &[0, 0, 0], 4, 25, 26, "uint"),
            site(TREEFOLD, E, &[0, 1], 4, 29, 36, "leaf"),
            site(TREEFOLD, E, &[0, 1, 0], 4, 34, 35, "uint"),
            site(TREEFOLD, F(0), &[], 3, 27, 86, "match"),
            site(TREEFOLD, F(0), &[0], 3, 33, 34, "var"),
            site(TREEFOLD, F(0), &[1], 3, 48, 49, "var"),
            site(TREEFOLD, F(0), &[2], 3, 67, 84, "add"),
            site(TREEFOLD, F(0), &[2, 0], 3, 67, 74, "call"),
            site(TREEFOLD, F(0), &[2, 0, 0], 3, 72, 73, "var"),
            site(TREEFOLD, F(0), &[2, 1], 3, 77, 84, "call"),
            site(TREEFOLD, F(0), &[2, 1, 0], 3, 82, 83, "var"),
        ],
        patterns: vec![
            PatternSite {
                match_source: location(F(0), &[]),
                arm_index: 0,
                span: span(TREEFOLD, 3, 37, 44),
                binders: vec![binder(TREEFOLD, "x", 3, 42, 43)],
            },
            PatternSite {
                match_source: location(F(0), &[]),
                arm_index: 1,
                span: span(TREEFOLD, 3, 51, 63),
                binders: vec![
                    binder(TREEFOLD, "l", 3, 58, 59),
                    binder(TREEFOLD, "r", 3, 61, 62),
                ],
            },
        ],
    }
}

fn check_parse(source: &str, name: &str, expected: Program, provenance: SourceProvenance) {
    let parsed = parse_draft_source(source).expect("normal source preparation");
    assert_eq!(parsed.name, name);
    assert_eq!(*parsed.program, expected);
    assert_eq!(parsed.provenance, provenance);
}

#[test]
fn factorial_literal_ast_and_complete_source_provenance() {
    // Independent whole-integer target: fact(4) = 4*3*2*1 = 24; no run here.
    check_parse(
        FACTORIAL,
        "Factorial",
        factorial_expected(),
        factorial_provenance(),
    );
}

#[test]
fn treefold_literal_ast_order_patterns_and_complete_source_provenance() {
    // Independent whole-integer target: fold(branch(leaf(3),leaf(4))) = 3+4 = 7.
    check_parse(
        TREEFOLD,
        "TreeFold",
        treefold_expected(),
        treefold_provenance(),
    );
}

#[test]
fn mutual_calls_lists_let_bool_and_all_shallow_list_patterns_remain_actual_ast() {
    let source = r#"language "moriarty-sherlock-draft/1";
program Extra {
  fn first(x: UInt, xs: List): Bool { second(x, xs) }
  fn second(x: UInt, xs: List): Bool { let yes = not(false) in if x < 1 { yes } else { match xs { nil => true, cons(h, t) => first(h, t), all => false } } }
  entry first(2, cons(3, nil));
}
"#;
    let expected = Program {
        functions: vec![
            Function {
                name: "first".into(),
                parameters: vec![parameter("x", Type::UInt), parameter("xs", Type::List)],
                return_type: Type::Bool,
                body: call("second", vec![v("x"), v("xs")]),
            },
            Function {
                name: "second".into(),
                parameters: vec![parameter("x", Type::UInt), parameter("xs", Type::List)],
                return_type: Type::Bool,
                body: e(Expr::Let(
                    "yes".into(),
                    unary(UnaryOp::Not, b(false)),
                    e(Expr::If(
                        binary(BinaryOp::Lt, v("x"), u(1)),
                        v("yes"),
                        e(Expr::Match(
                            v("xs"),
                            vec![
                                Arm {
                                    pattern: Pattern::Nil,
                                    body: b(true),
                                },
                                Arm {
                                    pattern: Pattern::Cons("h".into(), "t".into()),
                                    body: call("first", vec![v("h"), v("t")]),
                                },
                                Arm {
                                    pattern: Pattern::Var("all".into()),
                                    body: b(false),
                                },
                            ],
                        )),
                    )),
                )),
            },
        ],
        entry: call(
            "first",
            vec![
                u(2),
                binary(BinaryOp::Cons, u(3), e(Expr::Literal(Value::List(vec![])))),
            ],
        ),
    };
    let parsed = parse_draft_source(source).expect("normal mutual/list source");
    assert_eq!(*parsed.program, expected);
}

#[test]
fn arithmetic_precedence_parentheses_and_lexer_comments_prepare_without_evaluation() {
    let source = r#"language "moriarty-sherlock-draft/1";
program Arithmetic {
  // Parentheses preserve a source span, without an extra AST node.
  entry (2 + 3) * 4 - 1;
}
"#;
    let parsed = parse_draft_source(source).expect("normal arithmetic source");
    assert_eq!(
        *parsed.program,
        Program {
            functions: vec![],
            entry: binary(
                BinaryOp::Sub,
                binary(BinaryOp::Mul, binary(BinaryOp::Add, u(2), u(3)), u(4)),
                u(1)
            )
        }
    );
    assert_eq!(
        parsed.provenance.expressions,
        vec![
            site(source, None, &[], 4, 8, 23, "sub"),
            site(source, None, &[0], 4, 8, 19, "mul"),
            site(source, None, &[0, 0], 4, 8, 15, "add"),
            site(source, None, &[0, 0, 0], 4, 9, 10, "uint"),
            site(source, None, &[0, 0, 1], 4, 13, 14, "uint"),
            site(source, None, &[0, 1], 4, 18, 19, "uint"),
            site(source, None, &[1], 4, 22, 23, "uint"),
        ]
    );
}
