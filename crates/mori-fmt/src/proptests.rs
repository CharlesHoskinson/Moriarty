//! Formatting never panics, keeps the program, and is stable.

use mori_lexer::{TokenIdx, TokenKind};
use proptest::prelude::*;

use crate::format;

/// Real programs to damage, so random edits reach every layout.
const EXAMPLES: &[&str] = &[
    include_str!("../../fixtures/amm.mori"),
    include_str!("../../fixtures/lending.mori"),
    include_str!("../../fixtures/repayment.mori"),
    include_str!("../../fixtures/lesson-transfer.mori"),
];

/// Pieces to insert: comments, blank lines and syntax.
const FRAGMENTS: &[&str] = &[
    "// note\n",
    "/* note */",
    "/* two\nlines */",
    "\n\n",
    " ",
    ",",
    "const x = 1;",
    "{ a: 1 }",
    "[1, 2]",
];

/// Every token's kind and text, and every comment's text.
pub type Shape = (Vec<(TokenKind, String)>, Vec<String>);

/// What formatting must not change: every token except commas, which lists
/// print themselves, and the text of every comment, in order, ignoring
/// trailing spaces.
pub fn shape(source: &str) -> Option<Shape> {
    let tokens = mori_lexer::lex(source).ok()?;
    let kinds = (0..tokens.len())
        .map(TokenIdx::new)
        .filter(|&token| tokens.kind(token) != TokenKind::Comma)
        .map(|token| {
            (
                tokens.kind(token),
                tokens.span(token, source).source_text(source).to_owned(),
            )
        })
        .collect();
    let comments = (0..tokens.comments.len())
        .map(|index| {
            // Formatting drops trailing spaces at the end of every line.
            let text = tokens.comment_span(index).source_text(source);
            text.lines()
                .map(str::trim_end)
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect();
    Some((kinds, comments))
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

proptest! {
    #[test]
    fn format_keeps_the_program_and_is_stable(source in prop_oneof![damaged_example(), any::<String>()]) {
        if let Ok(formatted) = format(&source) {
            prop_assert_eq!(shape(&formatted), shape(&source));
            let again = format(&formatted);
            prop_assert!(again.is_ok(), "formatted output does not parse");
            prop_assert_eq!(again.unwrap(), formatted);
        }
    }
}
