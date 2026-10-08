use std::fmt::Write;
use std::path::Path;

use crate::{Change, diff, format};

/// Formats the source, snapshots the result, and checks that formatting the
/// result again changes nothing.
macro_rules! assert_format_snapshot {
    ($source:literal) => {{
        let source = indoc::indoc!($source);
        let formatted = format(source).unwrap_or_else(|error| panic!("format failed: {error}"));
        let again = format(&formatted).unwrap_or_else(|error| panic!("output does not parse: {error}"));
        assert_eq!(again, formatted, "formatting is not idempotent");
        insta::with_settings!({
            description => format!("Code:\n\n{source}"),
            omit_expression => true,
        }, {
            insta::assert_snapshot!(formatted);
        });
    }};
}

/// Diffs the source against its formatted form, as `--check` shows it.
macro_rules! assert_diff_snapshot {
    ($source:literal) => {{
        let source = indoc::indoc!($source);
        let formatted = format(source).unwrap_or_else(|error| panic!("format failed: {error}"));
        insta::with_settings!({
            description => format!("Code:\n\n{source}"),
            omit_expression => true,
        }, {
            insta::assert_snapshot!(render_diff(source, &formatted));
        });
    }};
}

fn render_diff(source: &str, formatted: &str) -> String {
    let mut out = String::new();
    for hunk in diff(source, formatted) {
        writeln!(out, "Diff in test.mori:{}:", hunk.line).unwrap();
        for (change, line) in hunk.lines {
            let sign = match change {
                Change::Context => ' ',
                Change::Removed => '-',
                Change::Added => '+',
            };
            writeln!(out, "{sign}{line}").unwrap();
        }
    }
    out
}

// Layout

#[test]
fn empty_agreement() {
    assert_format_snapshot!(
        r#"
        profile   "moriarty-beta/1" ;
        agreement   Empty{ }
        "#
    );
}

#[test]
fn spacing_is_normalized() {
    assert_format_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
        const   price:Qty< USD >=10.00   USD ;
            const total=( price+fee )*2-1 USD;
        action  pay uses   invoice ;
        }
        "#
    );
}

#[test]
fn short_records_stay_on_one_line() {
    assert_format_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Records {
          account alice = {
            domain: Preview,
            id: "alice",
          };
          const empty = {};
        }
        "#
    );
}

#[test]
fn long_records_break_with_trailing_commas() {
    assert_format_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Records {
          asset USD = { domain: Preview, id: "usd", scale: 2, representation: "native", symbol: "USD" };
        }
        "#
    );
}

#[test]
fn long_calls_break_and_nest() {
    assert_format_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          intent pay = { domain: Preview, asset: USD, signer: alice, key: "key1", nonce: "n1", pre_head: "h0", valid: rounds(domain: Preview, from: 0, to: 10), gross_cap: price + fee, fee_cap: fee, net_floor: price, operation: transfer(from: alice, to: bob, fee_to: treasury, value: price, fee: fee), source_hash: "src1", policy_digest: "policy1", failure: SuccessOnly, observations: [], disclosures: [], retained_effects: [], retained_duties: [], delegation: None, recovery: None };
          const swap = amm.swap_exact_input(pool: pool, owner: alice, input: 10 USD, output_asset: GOLD);
        }
        "#
    );
}

#[test]
fn lists() {
    assert_format_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Lists {
          const none = [ ];
          const few = [ USD,GOLD, ];
          const many = [100000 USD, 200000 USD, 300000 USD, 400000 USD, 500000 USD, 600000 USD];
        }
        "#
    );
}

#[test]
fn numbers_and_strings_keep_their_spelling() {
    assert_format_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Spelling {
          const big = 1_000_000;
          const exact = 10.50 USD;
          const escaped = "caf\u00e9 \"quoted\"";
        }
        "#
    );
}

// Comments

#[test]
fn comments_stay_where_they_were_written() {
    assert_format_snapshot!(
        r#"
        // The invoice agreement.
        profile "moriarty-beta/1"; // the only profile
        agreement Invoice { // body follows
          // Prices.
          const price = 10.00 /* list */ USD;
          const fee = 0.10 USD; // flat fee
          account alice = {
            domain: Preview, // home domain
            id: "alice",
            // more fields later
          };
          /* trailing note */
        }
        // after the agreement
        "#
    );
}

#[test]
fn line_comment_breaks_a_short_record() {
    assert_format_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Records {
          const r = { a: 1, // first
            b: 2 };
        }
        "#
    );
}

// Blank lines

#[test]
fn exactly_one_blank_line_between_items() {
    assert_format_snapshot!(
        r#"
        profile "moriarty-beta/1";

        agreement Invoice {

          const a = 1;
          const b = 2;



          const c = 3;

          // Section.

          const d = 4;

        }
        "#
    );
}

// Check diffs

#[test]
fn diff_shows_each_changed_region() {
    assert_diff_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const a=1;
          const b = 2;
          const c = 3;
          const d = 4;
          const e = 5;
          const f = 6;
          const g = 7;
          const h=8;
        }
        "#
    );
}

#[test]
fn formatted_source_has_no_diff() {
    let source = "profile \"moriarty-beta/1\";\n\nagreement A {\n  const a = 1;\n}\n";
    assert_eq!(format(source).unwrap(), source);
    assert!(diff(source, source).is_empty());
}

/// Every fixture formats to a stable result with the same tokens and comments.
#[test]
fn fixtures_format() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures");
    for entry in std::fs::read_dir(&fixtures).expect("fixtures directory exists") {
        let path = entry.expect("directory entry is readable").path();
        let source = std::fs::read_to_string(&path).expect("fixture is readable");
        let Ok(formatted) = format(&source) else {
            assert!(
                path.ends_with("invalid.mori"),
                "{} failed to format",
                path.display()
            );
            continue;
        };
        assert_eq!(
            format(&formatted).unwrap(),
            formatted,
            "{} is not idempotent",
            path.display()
        );
        assert_eq!(
            crate::proptests::shape(&source),
            crate::proptests::shape(&formatted),
            "{} changed its tokens or comments",
            path.display()
        );
    }
}

#[test]
fn comment_before_a_comma_moves_after_it() {
    assert_format_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Records {
          const r = { a: 1 // first
            , b: 2 /* second */ , c: 3 };
        }
        "#
    );
}
