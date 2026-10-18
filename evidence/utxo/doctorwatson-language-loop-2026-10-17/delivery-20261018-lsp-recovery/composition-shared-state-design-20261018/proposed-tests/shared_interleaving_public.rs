// Proposed future crates/mori/tests/shared_interleaving_public.rs.
// The proposed consumer/feature and independent expectations are NOT present.
#![cfg(feature = "core2-shared-interleaving-draft")]
use serde_json::Value;
use std::{fs, path::PathBuf, process::Command};

#[test]
fn public_shared_consumer_preserves_every_complete_ordered_observation() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../fixtures/core-shared-interleaving");
    let expected_bytes = fs::read(dir.join("public-report.json")).unwrap();
    let expected_json: Value = serde_json::from_slice(&expected_bytes).unwrap();
    let manifest: Value =
        serde_json::from_slice(&fs::read(dir.join("mori-tests.json")).unwrap()).unwrap();
    let expected_cases: Vec<Vec<Value>> = manifest["cases"].as_array().unwrap()
        .iter().map(|case| serde_json::from_slice(
            &fs::read(dir.join(case["expected"].as_str().unwrap())).unwrap(),
        ).unwrap()).collect();
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "test"])
        .arg(&dir)
        .output().unwrap();
    assert_eq!(output.status.code(), Some(0), "{}",
        String::from_utf8_lossy(&output.stderr));
    let actual: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(actual, expected_json);
    assert_eq!(output.stdout, expected_bytes);
    assert!(String::from_utf8_lossy(&output.stderr).contains("4 passed; 0 failed"));
    let cases = actual["cases"].as_array().unwrap();
    assert_eq!(cases.len(), expected_cases.len());
    for (case, expected) in cases.iter().zip(expected_cases) {
        let stages = case["stages"].as_array().unwrap();
        assert_eq!(stages.len(), expected.len());
        for (stage, expected) in stages.iter().zip(expected) {
            assert_eq!(stage["actual"], expected);
        }
    }
}
