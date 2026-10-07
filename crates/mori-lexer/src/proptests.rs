//! The lexer never panics, and its output is well formed for any input.

use proptest::prelude::*;

use crate::{TokenIdx, TokenKind, lex};

/// Pieces that exercise every lexer rule, valid and broken.
const FRAGMENTS: &[&str] = &[
    "profile",
    "agreement",
    "const",
    "action",
    "uses",
    "asset",
    "None",
    "share_class",
    "x",
    "USD",
    "_",
    "0",
    "7",
    "007",
    "1_000",
    "1__0",
    "1_",
    "1.",
    "1.2.3",
    "10.50",
    "0._5",
    "\"",
    "\"ok\"",
    "\\",
    "\\u",
    "\\u00e",
    "\\uD83D",
    "\\uDE00",
    "\\n",
    "\\q",
    "//",
    "/*",
    "*/",
    "/",
    " ",
    "\n",
    "\r",
    "\t",
    "{",
    "}",
    "[",
    "]",
    "(",
    ")",
    ":",
    ";",
    ",",
    "=",
    "<",
    ">",
    ".",
    "+",
    "-",
    "*",
    "é",
    "“",
    "'",
    "#",
    "\u{0}",
    "\u{1F600}",
];

fn fragments() -> impl Strategy<Value = String> {
    prop::collection::vec(prop::sample::select(FRAGMENTS), 0..64).prop_map(|parts| parts.concat())
}

fn source() -> impl Strategy<Value = String> {
    prop_oneof![fragments(), any::<String>()]
}

proptest! {
    #[test]
    fn lex_never_panics_and_tokens_are_well_formed(source in source()) {
        let Ok(tokens) = lex(&source) else {
            return Ok(());
        };
        let len = u32::try_from(source.len()).unwrap();

        let last = TokenIdx::new(tokens.len() - 1);
        prop_assert_eq!(tokens.kind(last), TokenKind::Eof);
        prop_assert_eq!(tokens.start(last), len);

        let mut previous_end = 0;
        for index in 0..tokens.len() {
            let span = tokens.span(TokenIdx::new(index), &source);
            prop_assert!(span.start >= previous_end && span.end <= len);
            prop_assert!(source.is_char_boundary(span.start as usize));
            prop_assert!(source.is_char_boundary(span.end as usize));
            if index + 1 < tokens.len() {
                prop_assert!(span.start < span.end, "only end of file is empty");
            }
            previous_end = span.end;
        }

        let mut previous_end = 0;
        for index in 0..tokens.comments.len() {
            let span = tokens.comment_span(index);
            prop_assert!(span.start >= previous_end && span.start < span.end && span.end <= len);
            previous_end = span.end;
        }
    }
}
