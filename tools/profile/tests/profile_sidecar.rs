#![cfg(feature = "recursion-phase-profile")]
use midnight_proofs::profile_result;
use mori_recursion_profile::sidecar::Sidecar;
use serde_json::json;

#[test]
fn writer_preserves_failure_and_never_overwrites_an_existing_run() {
    let path =
        std::env::temp_dir().join(format!("profile-sidecar-test-{}.json", std::process::id()));
    let writer = Sidecar::start(&path).unwrap();
    let partial: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(partial["schema"], "recursion.phase-incomplete/1");
    assert!(Sidecar::start(&path).is_err());
    let result: Result<(), ()> = profile_result!(Command, Diagnostic, Err(()));
    writer
        .finish(
            json!({"id":"A2","verification":{"stock_verifier":"not_run"}}),
            result.is_ok(),
        )
        .unwrap();
    let data: serde_json::Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(data["schema"], "recursion.phase-events/1");
    assert_eq!(data["workload"]["exit"], 1);
    assert_eq!(data["profile_valid"], false);
    assert_eq!(data["events"][0]["status"], 2);
    assert!(
        data["observation"]["measurements"]
            .as_array()
            .unwrap()
            .len()
            > 0
    );
    assert!(data["sidecar_flush_wall_ns"].is_number());

    for metadata in [
        json!({"witness":"CANARY"}),
        json!({"note":"rng_seed = CANARY"}),
    ] {
        let writer = Sidecar::start(&path).unwrap();
        assert!(writer.finish(metadata, true).is_err());
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(!text.contains("CANARY"));
        assert!(text.contains("recursion.phase-incomplete/1"));
        std::fs::remove_file(&path).unwrap();
    }
}
