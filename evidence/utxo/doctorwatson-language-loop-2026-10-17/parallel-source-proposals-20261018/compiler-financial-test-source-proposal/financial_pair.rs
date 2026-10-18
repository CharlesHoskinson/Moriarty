//! Complete compiler output checks for the shared financial specimens.
use midnight_zkir_v3::IrSource;
use mori_backend::Target;
use serde_json::Value;

macro_rules! financial_source {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/financial-pair/",
            $name
        ))
    };
}

macro_rules! compiler_oracle {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/compiler-financial-pair/",
            $name
        ))
    };
}

fn assert_complete_compilation(source: &str, target_json: &str, expected_json: &str) {
    let target = Target::from_json(target_json).expect("compiler test Target must decode");
    let expected: Value = serde_json::from_str(expected_json)
        .expect("independent complete Artifact oracle must decode");
    let program = mori_core::parse(source).expect("shared financial source must parse");
    let actual = mori_backend::compile(&program, &target)
        .expect("financial source must compile for the explicit test Target");

    assert_eq!(
        serde_json::to_value(&actual).expect("complete Artifact must serialize"),
        expected
    );

    let expected_circuits = expected["circuits"].as_array().unwrap();
    assert_eq!(actual.circuits.len(), expected_circuits.len());
    for (circuit, expected_circuit) in actual.circuits.iter().zip(expected_circuits) {
        let reopened = IrSource::load(circuit.zkir.as_bytes())
            .expect("compiler-emitted financial IR must reopen");
        let expected_zkir = expected_circuit["zkir"].as_str().unwrap();
        let expected_ir = IrSource::load(expected_zkir.as_bytes())
            .expect("independent frozen financial IR must reopen");
        assert_eq!(reopened, expected_ir);
    }
}

#[test]
fn installment_invoice_matches_complete_compiler_oracle() {
    assert_complete_compilation(
        financial_source!("installment-invoice.mori"),
        compiler_oracle!("invoice/target.json"),
        compiler_oracle!("invoice/artifact.json"),
    );
}

#[test]
fn conditional_reserve_matches_complete_compiler_oracle() {
    assert_complete_compilation(
        financial_source!("conditional-reserve.mori"),
        compiler_oracle!("reserve/target.json"),
        compiler_oracle!("reserve/artifact.json"),
    );
}
