use mori_core::parse;
use serde_json::Value;

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/program-ir-contract/",
            $name
        ))
    };
}

#[test]
fn counter_full_program_matches_independent_eleven_node_oracle() {
    let program = parse(fixture!("counter.mori")).expect("Counter source must parse");
    let actual = serde_json::to_value(&program).expect("Counter Program must serialize");
    let expected: Value = serde_json::from_str(fixture!("counter.program.expected.json"))
        .expect("frozen Counter Program oracle must decode");

    assert_eq!(actual, expected, "complete Counter Program must match");
}

#[test]
fn empty_frame_full_program_retains_uninvoked_global_declaration() {
    let program = parse(fixture!("empty-frame.mori")).expect("empty-frame source must parse");
    let actual = serde_json::to_value(&program).expect("empty-frame Program must serialize");
    let expected: Value = serde_json::from_str(fixture!("empty-frame.program.expected.json"))
        .expect("frozen empty-frame Program oracle must decode");

    assert_eq!(actual, expected, "complete empty-frame Program must match");
}
