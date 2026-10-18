const NATIVE_CODE: &str = "mori::command::native-unavailable";
const NATIVE_MSG: &str =
    "This mori build has no Midnight support. Rebuild with the native feature enabled.";


fn normalize_ws(text: &str) -> String {
    let mut seen_line = false;
    text.lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            if trimmed.is_empty() {
                return None;
            }
            let content = if seen_line {
                trimmed.strip_prefix('│').unwrap_or(trimmed)
            } else {
                seen_line = true;
                trimmed
            };
            Some(content)
        })
        .flat_map(str::split_whitespace)
        .collect::<Vec<_>>()
        .join(" ")
}


fn main() {

    assert_eq!(
        normalize_ws(
            "\n  × mori::command::native-unavailable: This mori build has no Midnight\n  │ support. Rebuild with the native feature enabled.\n"
        ),
        format!("× {NATIVE_CODE}: {NATIVE_MSG}")
    );
    assert_eq!(
        normalize_ws("  × header │ keep\n    │ │ body │ keep\n"),
        "× header │ keep │ body │ keep"
    );

}
