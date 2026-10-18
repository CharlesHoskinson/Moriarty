//! The checker never panics, on damaged real programs or random input.

use proptest::prelude::*;

use crate::check;

/// Real programs to damage, so random edits reach declarations and names.
const EXAMPLES: &[&str] = &[
    include_str!("../../fixtures/amm.mori"),
    include_str!("../../fixtures/lending.mori"),
    include_str!("../../fixtures/repayment.mori"),
    include_str!("../../fixtures/lesson-transfer.mori"),
];

/// Names and declarations to insert, so damage often still parses.
const FRAGMENTS: &[&str] = &[
    "x",
    "USD",
    "alice",
    "missing",
    "transfer",
    "const x = 1;",
    "const x = y;",
    "action a uses x;",
    "action transfer uses x;",
    "intent x = { };",
    " ",
];

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
    fn check_never_panics(source in prop_oneof![damaged_example(), any::<String>()]) {
        if let Ok(ast) = mori_parser::parse(&source) {
            let result = check(&ast);
            for diagnostic in &result.diagnostics {
                let _ = diagnostic.to_string();
            }
        }
    }
}
