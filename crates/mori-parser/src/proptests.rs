//! The parser never panics, on random input or on damaged real programs.

use proptest::prelude::*;

use mori_ast::{Ast, NodeIdx, NodeTag};
use mori_lexer::TokenKind;

use crate::parse;

/// Real programs to damage, so random edits reach deep into the grammar.
const EXAMPLES: &[&str] = &[
    include_str!("../fixtures/amm.mori"),
    include_str!("../fixtures/lending.mori"),
    include_str!("../fixtures/repayment.mori"),
    include_str!("../fixtures/lesson-transfer.mori"),
];

/// Pieces of syntax, to insert into examples or to string together.
const FRAGMENTS: &[&str] = &[
    "profile \"moriarty-beta/1\";",
    "agreement A {",
    "const x = ",
    "action pay uses x;",
    "asset",
    "x",
    "USD",
    "10",
    "10.5",
    "10 USD",
    "10USD",
    "\"s\"",
    "true",
    "None",
    "amm.swap",
    "rounds(",
    "a: 1",
    "Qty<USD>",
    "{",
    "}",
    "[",
    "]",
    "(",
    ")",
    "<",
    ">",
    ":",
    ";",
    ",",
    "=",
    ".",
    "+",
    "-",
    "*",
    " ",
    "\n",
    "/* c */",
];

fn fragments() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(FRAGMENTS), 0..48).prop_map(|parts| parts.concat())
}

/// An example with one span of characters replaced by a fragment or nothing.
fn damaged_example() -> impl Strategy<Value = String> {
    (
        prop::sample::select(EXAMPLES),
        any::<prop::sample::Index>(),
        0usize..16,
        prop::option::of(prop::sample::select(FRAGMENTS)),
    )
        .prop_map(|(example, at, removed, inserted)| {
            let chars: Vec<char> = example.chars().collect();
            let start = at.index(chars.len() + 1);
            let end = (start + removed).min(chars.len());
            let mut damaged: String = chars[..start].iter().collect();
            damaged.push_str(inserted.unwrap_or(""));
            damaged.extend(&chars[end..]);
            damaged
        })
}

fn source() -> impl Strategy<Value = String> {
    prop_oneof![damaged_example(), fragments(), any::<String>()]
}

proptest! {
    #[test]
    fn parse_never_panics_and_spans_nest(source in source()) {
        if let Ok(ast) = parse(&source) {
            ast.print();
            check_spans(&ast, NodeIdx::ROOT)?;
        }
    }
}

/// A node's span starts and ends with the tokens its tag implies, and its
/// children's spans lie inside it, in order and without overlapping.
fn check_spans(ast: &Ast<'_>, node: NodeIdx) -> Result<(), TestCaseError> {
    let first = ast.first_token(node);
    let last = ast.last_token(node);
    prop_assert!(first <= last);

    let (opens, closes) = match ast.tag(node) {
        NodeTag::Agreement => (Some(TokenKind::KwAgreement), Some(TokenKind::RBrace)),
        NodeTag::Declaration | NodeTag::Action => (None, Some(TokenKind::Semicolon)),
        NodeTag::Paren => (Some(TokenKind::LParen), Some(TokenKind::RParen)),
        NodeTag::Array => (Some(TokenKind::LBracket), Some(TokenKind::RBracket)),
        NodeTag::Record => (Some(TokenKind::LBrace), Some(TokenKind::RBrace)),
        NodeTag::Call => (None, Some(TokenKind::RParen)),
        NodeTag::Type if ast.list(node).next().is_some() => (None, Some(TokenKind::Gt)),
        _ => (None, None),
    };
    if let Some(kind) = opens {
        prop_assert_eq!(ast.tokens.kind(first), kind);
    }
    if let Some(kind) = closes {
        prop_assert_eq!(ast.tokens.kind(last), kind);
    }

    let span = ast.span(node);
    let mut previous_end = span.start;
    for child in ast.children(node) {
        let child_span = ast.span(child);
        prop_assert!(child_span.start >= previous_end && child_span.end <= span.end);
        previous_end = child_span.end;
        check_spans(ast, child)?;
    }
    Ok(())
}
