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
        Some("mori::lex::source_too_large")
    );
}

#[test]
fn token_bound_counts_eof_but_not_comments() {
    let within = ";".repeat(MAX_TOKENS - 1);
    assert_eq!(error_code(&within), None);
    assert_eq!(lex(&within).unwrap().len(), MAX_TOKENS);

    let over = ";".repeat(MAX_TOKENS);
    assert_eq!(
        error_code(&over).as_deref(),
        Some("mori::lex::too_many_tokens")
    );

    let commented = format!("{within}{}", "/**/".repeat(100));
    assert_eq!(error_code(&commented), None);
}

#[test]
fn string_bound_counts_decoded_utf8_bytes() {
    let at_limit = format!("\"{}\"", "a".repeat(MAX_STRING_BYTES));
    assert_eq!(error_code(&at_limit), None);

    let over = format!("\"{}\"", "a".repeat(MAX_STRING_BYTES + 1));
    assert_eq!(
        error_code(&over).as_deref(),
        Some("mori::lex::string_too_long")
    );

    // `é` is two bytes in UTF-8, so 513 of them exceed the limit.
    let escaped = format!("\"{}\"", "\\u00e9".repeat(MAX_STRING_BYTES / 2 + 1));
    assert_eq!(
        error_code(&escaped).as_deref(),
        Some("mori::lex::string_too_long")
    );
}

#[test]
fn identifier_bound() {
    assert_eq!(error_code(&"a".repeat(64)), None);
    assert_eq!(
        error_code(&"a".repeat(65)).as_deref(),
        Some("mori::lex::name_too_long")
    );
}
#[test]
fn leading_token_survives_invalid_body_without_accepting_it() {
    let source = "// preamble\n/* profile */ language \"moriarty/1\"; @";
    assert_eq!(crate::first_token_text(source).unwrap(), "language");
    assert!(crate::lex(source).is_err());
    assert_eq!(crate::first_token_text(" /* comment */ ").unwrap(), "");
    assert_eq!(
        crate::first_token_text("language_beta @").unwrap(),
        "language_beta"
    );
    let oversized = format!("language {}", "x".repeat(crate::MAX_SOURCE_BYTES));
    assert_eq!(crate::first_token_text(&oversized).unwrap(), "language");
    assert!(crate::lex(&oversized).is_err());
}

// Normal public API expectations frozen before this append; see EXPECTED-NORMAL-API.json.

#[test]
fn decode_string_returns_frozen_normal_utf8_bytes() {
    let cases: &[(&str, &str, &[u8])] = &[
        (r###"empty"###, r###""""###, &[]),
        (r###"ascii"###, r###""Midnight""###, &[77, 105, 100, 110, 105, 103, 104, 116]),
        (r###"quote"###, r###""\"""###, &[34]),
        (r###"backslash"###, r###""\\""###, &[92]),
        (r###"solidus"###, r###""\/""###, &[47]),
        (r###"short_controls"###, r###""\b\f\n\r\t""###, &[8, 12, 10, 13, 9]),
        (r###"hex_ascii"###, r###""\u0041\u0062\u0030""###, &[65, 98, 48]),
        (r###"hex_nul"###, r###""\u0000""###, &[0]),
        (r###"bmp_escaped"###, r###""\u00e9""###, &[195, 169]),
        (r###"bmp_raw"###, r###""é""###, &[195, 169]),
        (r###"pair_upper"###, r###""\uD83D\uDE00""###, &[240, 159, 152, 128]),
        (r###"pair_lower"###, r###""\ud83d\ude00""###, &[240, 159, 152, 128]),
        (r###"astral_raw"###, r###""😀""###, &[240, 159, 152, 128]),
        (r###"music_pair"###, r###""\uD834\uDD1E""###, &[240, 157, 132, 158]),
        (r###"escaped_crlf"###, r###""\r\n""###, &[13, 10]),
        (r###"mixed_newline"###, r###""a\nb""###, &[97, 10, 98]),
        (r###"combining_escaped"###, r###""e\u0301""###, &[101, 204, 129]),
        (r###"combining_raw"###, r###""é""###, &[101, 204, 129]),
        (r###"raw_and_escaped_solidus"###, r###""path/to\/file""###, &[112, 97, 116, 104, 47, 116, 111, 47, 102, 105, 108, 101]),
        (r###"literal_unicode_spelling"###, r###""\\u0041""###, &[92, 117, 48, 48, 52, 49]),
        (r###"literal_n_spelling"###, r###""\\n""###, &[92, 110]),
        (r###"three_slashes_then_quote"###, r###""\\\"""###, &[92, 34]),
        (r###"five_slashes_then_quote"###, r###""\\\\\"""###, &[92, 92, 34]),
        (r###"four_slashes_before_end"###, r###""\\\\""###, &[92, 92]),
    ];
    for &(name, source, expected) in cases {
        let tokens = lex(source).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(tokens.len(), 2, "{name}: exactly one string and EOF");
        let first = TokenIdx::new(0);
        assert_eq!(tokens.kind(first), crate::TokenKind::String, "{name}");
        assert_eq!(tokens.kind(TokenIdx::new(1)), crate::TokenKind::Eof, "{name}");
        let quoted = tokens.span(first, source).source_text(source);
        assert_eq!(quoted, source, "{name}: preserve the complete quoted input");
        assert_eq!(crate::decode_string(quoted).as_bytes(), expected, "{name}");
    }
}

#[test]
fn token_kind_public_classification_matches_every_variant() {
    use crate::TokenKind::*;
    let cases = [
        (Ident, false, false, true),
        (Number, false, false, false),
        (String, false, false, false),
        (KwProfile, true, false, true),
        (KwAgreement, true, false, true),
        (KwAction, true, false, true),
        (KwUses, true, false, true),
        (KwTrue, true, false, true),
        (KwFalse, true, false, true),
        (KwNone, true, false, true),
        (KwSuccessOnly, true, false, true),
        (KwDomain, true, true, true),
        (KwAccount, true, true, true),
        (KwAsset, true, true, true),
        (KwConst, true, true, true),
        (KwIntent, true, true, true),
        (KwObligation, true, true, true),
        (KwPool, true, true, true),
        (KwInstrument, true, true, true),
        (KwObservation, true, true, true),
        (KwPolicy, true, true, true),
        (KwGrant, true, true, true),
        (KwStage, true, true, true),
        (KwEpisode, true, true, true),
        (KwParty, true, true, true),
        (KwShareClass, true, true, true),
        (LBrace, false, false, false),
        (RBrace, false, false, false),
        (LBracket, false, false, false),
        (RBracket, false, false, false),
        (LParen, false, false, false),
        (RParen, false, false, false),
        (Colon, false, false, false),
        (Semicolon, false, false, false),
        (Comma, false, false, false),
        (Eq, false, false, false),
        (Lt, false, false, false),
        (Gt, false, false, false),
        (Dot, false, false, false),
        (Plus, false, false, false),
        (Minus, false, false, false),
        (Star, false, false, false),
        (Eof, false, false, false),
    ];
    for (kind, keyword, declaration, word) in cases {
        assert_eq!(kind.is_keyword(), keyword, "{kind:?}: keyword");
        assert_eq!(kind.is_declaration_kind(), declaration, "{kind:?}: declaration");
        assert_eq!(kind.is_word(), word, "{kind:?}: word");
    }
}

#[test]
fn keyword_lookup_and_lexing_preserve_exact_spelling() {
    use crate::TokenKind;
    let cases = [
        (r###"profile"###, TokenKind::KwProfile),
        (r###"agreement"###, TokenKind::KwAgreement),
        (r###"action"###, TokenKind::KwAction),
        (r###"uses"###, TokenKind::KwUses),
        (r###"true"###, TokenKind::KwTrue),
        (r###"false"###, TokenKind::KwFalse),
        (r###"None"###, TokenKind::KwNone),
        (r###"SuccessOnly"###, TokenKind::KwSuccessOnly),
        (r###"domain"###, TokenKind::KwDomain),
        (r###"account"###, TokenKind::KwAccount),
        (r###"asset"###, TokenKind::KwAsset),
        (r###"const"###, TokenKind::KwConst),
        (r###"intent"###, TokenKind::KwIntent),
        (r###"obligation"###, TokenKind::KwObligation),
        (r###"pool"###, TokenKind::KwPool),
        (r###"instrument"###, TokenKind::KwInstrument),
        (r###"observation"###, TokenKind::KwObservation),
        (r###"policy"###, TokenKind::KwPolicy),
        (r###"grant"###, TokenKind::KwGrant),
        (r###"stage"###, TokenKind::KwStage),
        (r###"episode"###, TokenKind::KwEpisode),
        (r###"party"###, TokenKind::KwParty),
        (r###"share_class"###, TokenKind::KwShareClass),
    ];
    for (source, expected) in cases {
        assert_eq!(TokenKind::keyword(source), Some(expected), "{source}");
        let tokens = lex(source).unwrap();
        assert_eq!(tokens.len(), 2, "{source}");
        assert_eq!(tokens.kind(TokenIdx::new(0)), expected, "{source}");
        assert_eq!(tokens.kind(TokenIdx::new(1)), TokenKind::Eof, "{source}");
        assert_eq!(tokens.span(TokenIdx::new(0), source).source_text(source), source);
    }
    for source in [
        r###"Profile"###,
        r###"profiled"###,
        r###"none"###,
        r###"successonly"###,
        r###"shareclass"###,
        r###"share_Class"###,
        r###"X_7"###,
    ] {
        assert_eq!(TokenKind::keyword(source), None, "{source}");
        let tokens = lex(source).unwrap();
        assert_eq!(tokens.len(), 2, "{source}");
        assert_eq!(tokens.kind(TokenIdx::new(0)), TokenKind::Ident, "{source}");
        assert_eq!(tokens.kind(TokenIdx::new(1)), TokenKind::Eof, "{source}");
    }
}

#[test]
fn punctuation_lookup_preserves_every_public_symbol() {
    use crate::TokenKind;
    let cases = [
        (b'{', TokenKind::LBrace),
        (b'}', TokenKind::RBrace),
        (b'[', TokenKind::LBracket),
        (b']', TokenKind::RBracket),
        (b'(', TokenKind::LParen),
        (b')', TokenKind::RParen),
        (b':', TokenKind::Colon),
        (b';', TokenKind::Semicolon),
        (b',', TokenKind::Comma),
        (b'=', TokenKind::Eq),
        (b'<', TokenKind::Lt),
        (b'>', TokenKind::Gt),
        (b'.', TokenKind::Dot),
        (b'+', TokenKind::Plus),
        (b'-', TokenKind::Minus),
        (b'*', TokenKind::Star),
    ];
    for (byte, expected) in cases {
        assert_eq!(TokenKind::punctuation(byte), Some(expected));
        let bytes = [byte];
        let source = std::str::from_utf8(&bytes).unwrap();
        let tokens = lex(source).unwrap();
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens.kind(TokenIdx::new(0)), expected);
        assert_eq!(tokens.kind(TokenIdx::new(1)), TokenKind::Eof);
        assert_eq!(tokens.span(TokenIdx::new(0), source).source_text(source), source);
    }
}

#[test]
fn first_token_text_preserves_raw_prefix_without_validating_later_body() {
    let cases = [
        ("blank", " \t\r\n", "", true),
        ("comments", "// preamble\r\n/* retained */ ", "", true),
        ("keyword_before_invalid_tail", "// preamble\r\n/* note */ profile @", "profile", false),
        ("identifier_before_invalid_string", "Name_7 \"\\q\"", "Name_7", false),
        ("raw_string_prefix", "\"a\\/b\" agreement", "\"a\\/b\"", true),
        ("number_and_next_word", "10USD", "10", true),
    ];
    for (name, source, expected, whole_lex_succeeds) in cases {
        assert_eq!(crate::first_token_text(source).unwrap(), expected, "{name}");
        assert_eq!(lex(source).is_ok(), whole_lex_succeeds, "{name}: full lex is separate");
    }
}
