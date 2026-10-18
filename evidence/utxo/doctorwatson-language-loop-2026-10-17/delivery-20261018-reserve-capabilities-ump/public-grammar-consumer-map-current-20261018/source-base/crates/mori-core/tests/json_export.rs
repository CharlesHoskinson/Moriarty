#[test]
fn escaped_output_roundtrips_with_exact_strings() {
    let value = serde_json::json!({"label": "café \u{202e} 😀", "amount": "7"});
    let output = mori_core::json::export(&value).unwrap();
    assert!(output.is_ascii());
    assert!(output.contains("\\u202e"));
    assert!(output.contains("\\ud83d\\ude00"));
    assert_eq!(mori_core::json::parse(&output).unwrap(), value);
}

#[test]
fn bounded_export_checks_escaped_bytes_including_print_newline() {
    let value = serde_json::json!({"label": "\u{80}".repeat(200_000)});
    let raw = serde_json::to_string_pretty(&value).unwrap();
    assert!(raw.len() < 1_048_576);
    assert!(mori_core::json::parse(&raw).is_ok());
    assert!(mori_core::json::export(&value).is_err());
    let exact = "x".repeat(1_048_574); // JSON quotes make exactly1MiB.
    assert!(mori_core::json::parse(&serde_json::to_string(&exact).unwrap()).is_ok());
    assert!(
        mori_core::json::export(&exact).is_err(),
        "CLI final newline counts toward the public reader cap"
    );
}
