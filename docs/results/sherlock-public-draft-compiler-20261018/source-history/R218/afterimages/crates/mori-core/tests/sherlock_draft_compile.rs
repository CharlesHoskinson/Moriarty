//! SOURCE_ONLY / UNRUN. Written before the R218 public producer body.
//! R115 EXPECTED is independently sealed; no subject outputs define expectations.
use mori_core::sherlock_compile_artifact::compile_draft_host;
use serde_json::{json, Value};

const CONFIG: &str = include_str!("fixtures/draft-compile/config.json");
const TARGET: &str = include_str!("fixtures/draft-compile/target.json");
const EXPECTED: &str = include_str!("fixtures/draft-compile/EXPECTED.json");
const CONFIG_SHA256: &str = "0763c35fedd5aa8807bfac55f4d6ea0264cbaf5f6ed587a0fe8e08672e02f50f";
const TARGET_SHA256: &str = "b7b9069acc831ae962d941c75a75c7bc2780e4029a569209458601f9b3f8d56d";

fn compare_public_preparation(source: &str, case: &str) {
    let all: Value = serde_json::from_str(EXPECTED).expect("sealed independent JSON");
    let expected = &all[case];
    let artifact = match compile_draft_host(source, CONFIG, TARGET) {
        Ok(artifact) => artifact,
        Err(failure) => panic!("public draft preparation failed: {}",
            serde_json::to_string(&failure).expect("failure serialization")),
    };
    // The complete four R112 comparison fields, including all12 ProgramView
    // fields and every nested ordered cell; no context stripping or defaults.
    assert_eq!(artifact.source.source_digest, expected["source_digest"].as_str().unwrap());
    assert_eq!(serde_json::to_value(&artifact.source).unwrap(), expected["source"]);
    assert_eq!(serde_json::to_value(&artifact.preparation).unwrap(), expected["preparation"]);
    assert_eq!(&artifact.admission["program_view"], &expected["program_view"]);
    assert_eq!(artifact.format, "sherlock-host-artifact/2");
    assert_eq!(artifact.execution_profile, "sherlock-helper-microstep/1");
    assert_eq!(artifact.config.file_digest, CONFIG_SHA256);
    let mut config: Value = serde_json::from_str(CONFIG).unwrap();
    config.as_object_mut().unwrap().insert("file_digest".into(), Value::String(CONFIG_SHA256.into()));
    assert_eq!(serde_json::to_value(&artifact.config).unwrap(), config);
    assert_eq!(artifact.target.file_digest, TARGET_SHA256);
    assert_eq!(artifact.target.descriptor, serde_json::from_str::<Value>(TARGET).unwrap());
    assert_eq!(serde_json::to_value(&artifact.input_policy).unwrap(),
        json!({"kind":"closed-entry","declared":[],"supplied":[]}));
    assert_eq!(serde_json::to_value(&artifact.observation).unwrap(),
        json!({"status":"not-requested","evaluation_performed":false}));
    assert_eq!(artifact.plans.witness.status, "missing");
    assert_eq!(artifact.plans.constraint.status, "missing");
    assert_eq!(artifact.plans.export.status, "unsupported");
    assert_eq!(artifact.evidence.host_observation, "genesis-only");
    assert_eq!(artifact.evidence.native, "unrun");
    assert_eq!(artifact.evidence.target, "unrun");
    assert_eq!(artifact.evidence.acceptance, "unestablished");
}

#[test]
fn public_chain_total_prepares_admitted_complete_independent_tables() {
    compare_public_preparation(include_str!("fixtures/draft-compile/chain-total.mori"), "chain_total");
}

#[test]
fn public_branch_total_prepares_admitted_complete_independent_tables() {
    compare_public_preparation(include_str!("fixtures/draft-compile/branch-total.mori"), "branch_total");
}
