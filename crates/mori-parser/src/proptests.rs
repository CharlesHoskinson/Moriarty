//! The parser never panics, on random input or on damaged real programs.

use proptest::prelude::*;

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
    fn parse_never_panics(source in source()) {
        if let Ok(ast) = parse(&source) {
            ast.print();
        }
    }
}
