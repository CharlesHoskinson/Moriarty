#[test]
fn language_header_diagnostics_name_both_supported_core_profiles() {
    for (source, expected) in [
        (
            r#"language "moriarty/1"; agreement Example {}"#,
            "moriarty/1",
        ),
        (
            r#"language "moriarty/2"; agreement Example {}"#,
            "moriarty/2",
        ),
    ] {
        assert_eq!(language(source).unwrap(), expected);
    }

    let expected = r#"Expected language "moriarty/1" or "moriarty/2""#;
    for (source, code, offset, suffix) in [
        (
            r#"language "other";"#,
            "mori::core::profile",
            16,
            r#"; found "other"."#,
        ),
        (
            "",
            "mori::core::syntax",
            0,
            r#". Expected "language"; found ""."#,
        ),
        (
            "agreement Empty {}",
            "mori::core::syntax",
            0,
            r#". Expected "language"; found "agreement"."#,
        ),
        (
            "language ;",
            "mori::core::syntax",
            9,
            ". Expected a quoted string.",
        ),
    ] {
        let error = language(source).unwrap_err();
        assert_eq!(error.code, code, "{source:?}");
        assert_eq!(error.offset, Some(offset), "{source:?}");
        assert_eq!(error.message, format!("{expected}{suffix}"), "{source:?}");
    }
}
