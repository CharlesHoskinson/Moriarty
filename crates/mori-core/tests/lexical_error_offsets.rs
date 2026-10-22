//! Literal UTF-8 byte positions frozen before this test and the adapter fix.
//! Pure public Core parsing/profile diagnostics; no evaluator or SDK calls.
use mori_core::Error;
use serde_json::json;

struct Case {
    source: &'static str,
    offset: u32,
    message: &'static str,
}

const CASES: &[Case] = &[
    Case {
        source: "@",
        offset: 0,
        message: "I do not recognize the character `@`.",
    },
    Case {
        source: "language \"moriarty/1\";\n/* café */ @\n",
        offset: 35,
        message: "I do not recognize the character `@`.",
    },
    Case {
        source: "language \"moriarty/1\";\r\n/* café 😀 */ @\r\n",
        offset: 41,
        message: "I do not recognize the character `@`.",
    },
    Case {
        source: "language \"moriarty/2\";\r\n/* café 😀 */ @\r\n",
        offset: 41,
        message: "I do not recognize the character `@`.",
    },
    Case {
        source: "language \"moriarty/1\";\n/* café */ /* unfinished",
        offset: 35,
        message: "This block comment never ends.",
    },
    Case {
        source: "language \"moriarty/1\";\n/* café */ \"unfinished",
        offset: 35,
        message: "This string never ends.",
    },
];

fn assert_lex_error(error: Error, case: &Case) {
    assert_eq!(error.code, "mori::core::lex", "{:?}", case.source);
    assert_eq!(error.message, case.message, "{:?}", case.source);
    assert_eq!(error.offset, Some(case.offset), "{:?}", case.source);
    assert_eq!(error.step, None);
    assert_eq!(
        serde_json::to_value(error).unwrap(),
        json!({
            "code": "mori::core::lex",
            "message": case.message,
            "offset": case.offset,
            "step": null,
        }),
    );
}

#[test]
fn parse_preserves_literal_lexer_utf8_byte_offsets() {
    for case in CASES {
        assert_lex_error(mori_core::parse(case.source).unwrap_err(), case);
    }
}

#[test]
fn language_preserves_literal_lexer_utf8_byte_offsets() {
    for case in CASES {
        assert_lex_error(mori_core::language(case.source).unwrap_err(), case);
    }
}

#[test]
fn staged_load_preserves_the_same_core2_lexical_byte_offset() {
    let case = &CASES[3];
    assert_lex_error(mori_core::stage::load(case.source).unwrap_err(), case);
}

#[test]
fn existing_syntax_error_keeps_its_token_byte_offset() {
    let source = "language \"moriarty/1\";\n/* café */ agreement ;\n";
    let error = mori_core::parse(source).unwrap_err();
    assert_eq!(
        serde_json::to_value(error).unwrap(),
        json!({
            "code": "mori::core::syntax",
            "message": "Expected a name.",
            "offset": 45,
            "step": null,
        }),
    );
    assert_eq!(mori_core::language(source).unwrap(), "moriarty/1");
}

#[test]
fn unlocated_validation_error_remains_none_instead_of_zero() {
    let source = "language \"moriarty/1\"; agreement MissingAction {}";
    let error = mori_core::parse(source).unwrap_err();
    assert_eq!(
        serde_json::to_value(error).unwrap(),
        json!({
            "code": "mori::core::program",
            "message": "Program needs at least one action.",
            "offset": null,
            "step": null,
        }),
    );
}

#[test]
fn existing_positive_source_and_profile_detection_remain_admitted() {
    let source = "language \"moriarty/1\"; agreement Plain { action ping() { requires true; } }";
    let program = mori_core::parse(source).unwrap();
    assert_eq!(program.name, "Plain");
    assert_eq!(program.language, "moriarty/1");
    assert_eq!(program.semantics, "moriarty-core/1");
    assert_eq!(program.actions.len(), 1);
    assert_eq!(program.actions[0].name, "ping");
    assert_eq!(mori_core::language(source).unwrap(), "moriarty/1");
    assert_eq!(
        mori_core::language("language \"moriarty/2\";").unwrap(),
        "moriarty/2",
    );
}
