use super::{CheckAnalysis, analyze_source};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Case {
    name: String,
    source: String,
    file: String,
    expected_cli: Value,
    expected_cli_text: Option<String>,
}

#[test]
fn complete_cli_reports_match_frozen_public_expectations() {
    let cases: Vec<Case> = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../fixtures/mcp/check-cases.json"
    )))
    .unwrap();
    assert_eq!(cases.len(), 8);
    for case in cases {
        let analysis = analyze_source(&case.file, &case.source);
        if let (CheckAnalysis::Beta { report, .. }, Some(expected)) =
            (&analysis, &case.expected_cli_text)
        {
            assert_eq!(serde_json::to_string_pretty(report).unwrap(), *expected);
        }
        assert_eq!(
            analysis.is_accepted(),
            case.expected_cli["status"] == "accepted",
            "{} decision",
            case.name
        );
        assert_eq!(
            analysis.cli_report(&case.file, &case.source).unwrap(),
            case.expected_cli,
            "{} complete public report",
            case.name
        );
    }
}
