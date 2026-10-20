//! Normal Core lexer-location regressions; parsing/loading only, no execution.
use mori_core::{Error, Program, language, parse, stage};

fn parse_profile(source: &str, profile: u8) -> Result<Program, Error> {
    if profile == 1 {
        parse(source)
    } else {
        stage::load(source).map(|staged| staged.program().clone())
    }
}

fn snapshot_errors(name: &str, source: &str, errors: &[(&str, Error)], expected: &Error) {
    let mut projections = Vec::new();
    for (adapter, error) in errors {
        assert_eq!(error, expected, "{name}/{adapter}");
        projections.push(format!(
            "{adapter}:\n  code: {}\n  message: {}\n  offset: {:?}\n  step: {:?}",
            error.code, error.message, error.offset, error.step,
        ));
    }
    insta::with_settings!({
        description => format!("Code:\n\n{source}"),
        omit_expression => true,
    }, {
        insta::assert_snapshot!(name, projections.join("\n"));
    });
}

fn snapshot_success(name: &str, source: &str, program: &Program, routed: &str) {
    assert_eq!(program.name, "P");
    assert_eq!(program.language, routed);
    insta::with_settings!({
        description => format!("Code:\n\n{source}"),
        omit_expression => true,
    }, {
        insta::assert_snapshot!(name, format!(
            "language: {}\nagreement: {}\nrouted: {routed}", program.language, program.name,
        ));
    });
}

#[test]
fn lexical_offsets_survive_both_parse_and_language_adapters() {
    // Byte positions/messages are retained from the independently frozen normal fixtures.
    let cases = [
        (
            "ascii_at",
            "@\n",
            23,
            "I do not recognize the character `@`.",
        ),
        ("non_ascii", "é\n", 23, "`é` is not an ASCII character."),
        (
            "invalid_escape",
            "agreement P { action run() { emit(\"bad\\q\", 0); } }\n",
            61,
            "`\\q` is not an escape I know.",
        ),
        (
            "unterminated_string",
            "agreement P { action run() { emit(\"open",
            57,
            "This string never ends.",
        ),
        (
            "multibyte_comment_before_at",
            "/* café */ @\n",
            35,
            "I do not recognize the character `@`.",
        ),
        (
            "multibyte_string_before_at",
            "agreement P { action run() { emit(\"café\", 0); } } @\n",
            74,
            "I do not recognize the character `@`.",
        ),
    ];
    for profile in [1, 2] {
        for (name, tail, offset, message) in cases {
            let source = format!("language \"moriarty/{profile}\";\n{tail}");
            assert!(source.is_char_boundary(offset as usize));
            if name.starts_with("multibyte_") {
                assert_ne!(source[..offset as usize].chars().count(), offset as usize);
            }
            let errors = [
                ("parse", parse_profile(&source, profile).unwrap_err()),
                ("language", language(&source).unwrap_err()),
            ];
            snapshot_errors(
                &format!("core{profile}_{name}"),
                &source,
                &errors,
                &Error {
                    code: "mori::core::lex".into(),
                    message: message.into(),
                    offset: Some(offset),
                    step: None,
                },
            );
        }
    }
}

#[test]
fn eof_syntax_offsets_and_header_routing_stay_unchanged() {
    for profile in [1, 2] {
        let source = format!("language \"moriarty/{profile}\"");
        assert_eq!(language(&source).unwrap(), format!("moriarty/{profile}"));
        snapshot_errors(
            &format!("core{profile}_eof_syntax"),
            &source,
            &[("parse", parse_profile(&source, profile).unwrap_err())],
            &Error {
                code: "mori::core::syntax".into(),
                message: "Expected \";\"; found \"\".".into(),
                offset: Some(21),
                step: None,
            },
        );
    }
}

#[test]
fn valid_core_profiles_still_parse_and_route() {
    for profile in [1, 2] {
        let source =
            format!("language \"moriarty/{profile}\";\nagreement P {{ action run() {{}} }}\n");
        let routed = language(&source).unwrap();
        assert_eq!(routed, format!("moriarty/{profile}"));
        snapshot_success(
            &format!("core{profile}_valid"),
            &source,
            &parse_profile(&source, profile).unwrap(),
            routed,
        );
    }
}

#[test]
fn unspanned_size_guard_does_not_invent_offset_zero() {
    let source = " ".repeat(65_537);
    let errors = [
        ("core1_parse", parse_profile(&source, 1).unwrap_err()),
        ("core2_parse", parse_profile(&source, 2).unwrap_err()),
        ("language", language(&source).unwrap_err()),
    ];
    snapshot_errors(
        "unspanned_source_size",
        &source,
        &errors,
        &Error {
            code: "mori::core::lex".into(),
            message: "This file is 65537 bytes, but a source file can be at most 65536 bytes."
                .into(),
            offset: None,
            step: None,
        },
    );
}
