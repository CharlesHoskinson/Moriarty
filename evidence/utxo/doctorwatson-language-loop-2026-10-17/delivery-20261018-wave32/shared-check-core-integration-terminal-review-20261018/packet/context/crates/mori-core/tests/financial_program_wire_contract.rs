//! Complete independently frozen Core/1 wires, without selecting the SDK backend.
use mori_core::{Program, parse, v1};
use serde_json::Value;

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/financial-report-contract/",
            $name
        ))
    };
}

fn consume(source: &str, frozen_program: &str) {
    let expected: Value = serde_json::from_str(frozen_program)
        .expect("independently frozen complete Program JSON must decode");
    let actual = parse(source).expect("exact source must parse through the real Core consumer");
    let bytes = serde_json::to_vec(&actual).expect("complete Program must serialize");
    let actual_wire: Value = serde_json::from_slice(&bytes).expect("complete wire must be JSON");
    assert_eq!(
        actual_wire, expected,
        "every Program field and nested cell must match"
    );

    // Actual shared public decoding routes through the frozen Core/1 DTO.
    let reopened: Program =
        serde_json::from_slice(&bytes).expect("public Program wire must reopen");
    assert_eq!(
        reopened, actual,
        "reopening must preserve the complete parsed Program"
    );
    assert_eq!(serde_json::to_value(&reopened).unwrap(), expected);

    // This is a codec/adapter check, separate from the independent equality above.
    let frozen_v1: v1::Program = serde_json::from_str(frozen_program).unwrap();
    let converted: Program = frozen_v1.into();
    assert_eq!(converted, actual);
}

#[test]
fn invoice_complete_core_wire_matches_frozen_32_node_program_and_reopens() {
    consume(
        fixture!("installment-invoice.mori"),
        fixture!("installment-invoice.program.expected.json"),
    );
}

#[test]
fn reserve_complete_core_wire_matches_frozen_33_node_program_and_reopens() {
    consume(
        fixture!("conditional-reserve.mori"),
        fixture!("conditional-reserve.program.expected.json"),
    );
}
