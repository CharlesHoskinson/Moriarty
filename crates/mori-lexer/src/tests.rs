use std::fmt::Write;

use mori_diagnostics::Diagnostic;
use mori_diagnostics::render::render_plain;

use crate::{MAX_SOURCE_BYTES, MAX_STRING_BYTES, MAX_TOKENS, TokenIdx, Tokens, lex};

/// Lexes the source and snapshots its tokens and comments.
macro_rules! assert_lex_snapshot {
    ($source:literal) => {{
        let source = indoc::indoc!($source);
        let tokens = lex(source).unwrap_or_else(|error| panic!("expected tokens, got: {error}"));
        insta::with_settings!({
            description => format!("Code:\n\n{source}"),
            omit_expression => true,
        }, {
            insta::assert_snapshot!(dump(source, &tokens));
        });
    }};
}

/// Lexes the source and snapshots the rendered error.
macro_rules! assert_lex_error_snapshot {
    ($source:literal) => {{
        let source = indoc::indoc!($source);
        let error = lex(source).expect_err("expected a lex error");
        insta::with_settings!({
            description => format!("Code:\n\n{source}"),
            omit_expression => true,
        }, {
            insta::assert_snapshot!(render_plain(&error, "test.mori", source));
        });
    }};
}

fn dump(source: &str, tokens: &Tokens) -> String {
    let mut out = String::new();
    for index in 0..tokens.len() {
        let idx = TokenIdx::new(index);
        let span = tokens.span(idx, source);
        let kind = format!("{:?}", tokens.kind(idx));
        let range = format!("{}..{}", span.start, span.end);
        writeln!(out, "{kind:<14} {range:<10} {:?}", span.source_text(source)).unwrap();
    }
    if !tokens.comments.is_empty() {
        out.push_str("\ncomments:\n");
        for index in 0..tokens.comments.len() {
            let span = tokens.comment_span(index);
            let range = format!("{}..{}", span.start, span.end);
            writeln!(out, "{range:<10} {:?}", span.source_text(source)).unwrap();
        }
    }
    out
}

fn error_code(source: &str) -> Option<String> {
    let error = lex(source).err()?;
    Some(error.code().expect("lex errors have codes").to_string())
}

// Tokens

#[test]
fn empty_source() {
    assert_lex_snapshot!("");
}

#[test]
fn agreement_skeleton() {
    assert_lex_snapshot!(
        r#"
        profile "moriarty-beta/1";
        agreement Invoice {
          const price: Qty<USD> = 10.00 USD;
          action pay uses invoice;
        }
        "#
    );
}

#[test]
fn every_punctuation_symbol() {
    assert_lex_snapshot!("{ } [ ] ( ) : ; , = < > . + - *");
}

#[test]
fn keywords_and_identifiers() {
    assert_lex_snapshot!(
        r#"
        profile agreement action uses true false None SuccessOnly
        domain account asset const intent obligation pool instrument
        observation policy grant stage episode party share_class
        Profile none domains share_class2 USD x_1
        "#
    );
}

#[test]
fn numbers() {
    assert_lex_snapshot!("0 7 10 1_000 0.5 10.00 1_000.000_1 0.000_001");
}

#[test]
fn number_touching_asset_is_two_tokens() {
    assert_lex_snapshot!("10USD 10.5USD");
}

#[test]
fn strings() {
    assert_lex_snapshot!(
        r#"
        "" "Midnight" "Café" "tab\there" "quote \" and slash \\ \/"
        "\u00e9 \uD83D\uDE00 \b\f\n\r"
        "#
    );
}

#[test]
fn comments_are_kept_separately() {
    assert_lex_snapshot!(
        r#"
        // leading comment
        const a = 1; /* inline */ const b = 2;
        /* multi
           line */
        /**/ const c = 3; // trailing comment"#
    );
}

#[test]
fn line_comment_stops_at_carriage_return() {
    assert_lex_snapshot!("// windows\r\nconst");
}

#[test]
fn qualified_call_name() {
    assert_lex_snapshot!("amm.swap_exact_input(pool: p)");
}

// Errors

#[test]
fn unterminated_string() {
    assert_lex_error_snapshot!(r#"const name = "Midnight"#);
}

#[test]
fn string_with_line_break() {
    assert_lex_error_snapshot!(
        r#"
        const name = "Midnight;
        const other = "x";
        "#
    );
}

#[test]
fn string_with_tab_character() {
    assert_lex_error_snapshot!("const name = \"a\tb\";");
}

#[test]
fn string_with_invalid_escape() {
    assert_lex_error_snapshot!(r#"const path = "C:\data";"#);
}

#[test]
fn string_with_short_unicode_escape() {
    assert_lex_error_snapshot!(r#"const e = "\u00e";"#);
}

#[test]
fn string_with_lone_surrogate() {
    assert_lex_error_snapshot!(r#"const e = "\uD83D!";"#);
}

#[test]
fn string_syntax_error_wins_over_lone_surrogate() {
    assert_lex_error_snapshot!(r#"const e = "\uD83D\q";"#);
}

#[test]
fn unterminated_block_comment() {
    assert_lex_error_snapshot!(
        r#"
        const a = 1;
        /* explain the fee
        const b = 2;
        "#
    );
}

#[test]
fn number_with_leading_zero() {
    assert_lex_error_snapshot!("const n = 007.50;");
}

#[test]
fn number_with_trailing_underscore() {
    assert_lex_error_snapshot!("const n = 1_000_;");
}

#[test]
fn number_with_double_underscore() {
    assert_lex_error_snapshot!("const n = 1__000;");
}

#[test]
fn number_with_underscore_after_point() {
    assert_lex_error_snapshot!("const n = 1._5;");
}

#[test]
fn number_with_trailing_point() {
    assert_lex_error_snapshot!("const n = 10.;");
}

#[test]
fn number_with_two_points() {
    assert_lex_error_snapshot!("const version = 1.2.3;");
}

#[test]
fn identifier_too_long() {
    assert_lex_error_snapshot!(
        "const a_name_that_keeps_going_and_going_and_going_and_going_well_past_the_limit = 1;"
    );
}

#[test]
fn division() {
    assert_lex_error_snapshot!("const quarter = 1 USD / 4;");
}

#[test]
fn curly_quotes() {
    assert_lex_error_snapshot!("const name = “Midnight”;");
}

#[test]
fn single_quotes() {
    assert_lex_error_snapshot!("const name = 'Midnight';");
}

#[test]
fn non_ascii_name() {
    assert_lex_error_snapshot!("const café = 1;");
}

#[test]
fn name_starting_with_underscore() {
    assert_lex_error_snapshot!("const _fee = 1;");
}

#[test]
fn unknown_symbol() {
    assert_lex_error_snapshot!("const a = 1 # note");
}

// Bounds

#[test]
fn source_bound() {
    assert_eq!(error_code(&" ".repeat(MAX_SOURCE_BYTES)), None);
    assert_eq!(
        error_code(&" ".repeat(MAX_SOURCE_BYTES + 1)).as_deref(),
        Some("BETA_SOURCE_BOUND")
    );
}

#[test]
fn token_bound_counts_eof_but_not_comments() {
    let within = ";".repeat(MAX_TOKENS - 1);
    assert_eq!(error_code(&within), None);
    assert_eq!(lex(&within).unwrap().len(), MAX_TOKENS);

    let over = ";".repeat(MAX_TOKENS);
    assert_eq!(error_code(&over).as_deref(), Some("BETA_TOKEN_BOUND"));

    let commented = format!("{within}{}", "/**/".repeat(100));
    assert_eq!(error_code(&commented), None);
}

#[test]
fn string_bound_counts_decoded_utf8_bytes() {
    let at_limit = format!("\"{}\"", "a".repeat(MAX_STRING_BYTES));
    assert_eq!(error_code(&at_limit), None);

    let over = format!("\"{}\"", "a".repeat(MAX_STRING_BYTES + 1));
    assert_eq!(error_code(&over).as_deref(), Some("BETA_STRING_BOUND"));

    // `é` is two bytes in UTF-8, so 513 of them exceed the limit.
    let escaped = format!("\"{}\"", "\\u00e9".repeat(MAX_STRING_BYTES / 2 + 1));
    assert_eq!(error_code(&escaped).as_deref(), Some("BETA_STRING_BOUND"));
}

#[test]
fn identifier_bound() {
    assert_eq!(error_code(&"a".repeat(64)), None);
    assert_eq!(
        error_code(&"a".repeat(65)).as_deref(),
        Some("BETA_IDENTIFIER_BOUND")
    );
}
