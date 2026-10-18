//! Indented text rendering of an [`Ast`], used by snapshot tests.

use std::fmt::Write;

use mori_lexer::TokenKind;

use crate::{Ast, NodeIdx, NodeTag};

pub fn print(ast: &Ast<'_>) -> String {
    let mut out = String::new();
    node(ast, NodeIdx::ROOT, 0, &mut out);
    out
}

fn node(ast: &Ast<'_>, idx: NodeIdx, depth: usize, out: &mut String) {
    let main = ast.main_token(idx);
    let text = ast.token_text(main);
    let tag = ast.tag(idx);
    let indent = "  ".repeat(depth);

    let line = match tag {
        NodeTag::Agreement => format!("Agreement {text}"),
        NodeTag::Declaration => format!(
            "Declaration {text} {}",
            ast.token_text(ast.token_after(main, 1))
        ),
        NodeTag::Action => format!(
            "Action {} uses {}",
            ast.token_text(ast.token_after(main, 1)),
            ast.token_text(ast.token_after(main, 3)),
        ),
        NodeTag::Quantity => format!(
            "Quantity {text} {}",
            ast.token_text(ast.token_after(main, 1))
        ),
        NodeTag::Call => format!("Call {}", call_name(ast, idx)),
        NodeTag::Add
        | NodeTag::Sub
        | NodeTag::Mul
        | NodeTag::Paren
        | NodeTag::Array
        | NodeTag::Record => {
            format!("{tag:?}")
        }
        NodeTag::Type
        | NodeTag::String
        | NodeTag::Number
        | NodeTag::Bool
        | NodeTag::Tag
        | NodeTag::Reference
        | NodeTag::Field => format!("{tag:?} {text}"),
    };
    writeln!(out, "{indent}{line}").expect("writing to a String cannot fail");

    for child in ast.children(idx) {
        node(ast, child, depth + 1, out);
    }
}

/// The dotted name of a call, such as `amm.swap_exact_input`.
fn call_name(ast: &Ast<'_>, idx: NodeIdx) -> String {
    let mut token = ast.main_token(idx);
    let mut name = ast.token_text(token).to_owned();
    while ast.tokens.kind(ast.token_after(token, 1)) == TokenKind::Dot {
        token = ast.token_after(token, 2);
        name.push('.');
        name.push_str(ast.token_text(token));
    }
    name
}
