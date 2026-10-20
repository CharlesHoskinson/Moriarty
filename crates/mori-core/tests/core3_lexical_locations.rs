//! Source-author integration checks extending the existing frozen /1-/2
//! lexer-location literals to the actual Core3 loader and header adapter.
//! No action evaluation, compiler, SDK, VM or proof is invoked.
use mori_core::{Error, language_v3, stage};

#[test]
fn core3_load_and_extended_header_retain_all_existing_lexical_location_literals() {
    let cases = [
        ("@\n", 23, "I do not recognize the character `@`."),
        ("é\n", 23, "`é` is not an ASCII character."),
        ("agreement P { action run() { emit(\"bad\\q\", 0); } }\n", 61, "`\\q` is not an escape I know."),
        ("agreement P { action run() { emit(\"open", 57, "This string never ends."),
        ("/* café */ @\n", 35, "I do not recognize the character `@`."),
        ("agreement P { action run() { emit(\"café\", 0); } } @\n", 74, "I do not recognize the character `@`."),
    ];
    for (tail, offset, message) in cases {
        let source = format!("language \"moriarty/3\";\n{tail}");
        let expected = Error { code:"mori::core::lex".into(), message:message.into(), offset:Some(offset), step:None };
        assert_eq!(stage::load_v3(&source).unwrap_err(), expected, "{source}");
        assert_eq!(language_v3(&source).unwrap_err(), expected, "{source}");
    }
}

#[test]
fn core3_unspanned_lexical_guard_preserves_absence_and_success_keeps_profile() {
    let source = " ".repeat(65_537);
    let expected = Error { code:"mori::core::lex".into(),
        message:"This file is 65537 bytes, but a source file can be at most 65536 bytes.".into(),
        offset:None, step:None };
    assert_eq!(stage::load_v3(&source).unwrap_err(), expected);
    assert_eq!(language_v3(&source).unwrap_err(), expected);
    let source = "language \"moriarty/3\"; agreement P { action run() {} }";
    assert_eq!(language_v3(source).unwrap(), "moriarty/3");
    assert_eq!(stage::load_v3(source).unwrap().program().language, "moriarty/3");
}
