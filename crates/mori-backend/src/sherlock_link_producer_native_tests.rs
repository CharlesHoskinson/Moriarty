//! Ignored native fixtures for the scoped linked endpoint. UNRUN: no SRS,
//! keygen, proof or verification in this file has been executed. Each test
//! is a separate process invocation surface under the existing
//! `cargo test -p mori-backend --lib` build; Root selects the exact test,
//! parameters directory, k cap and output paths under a resource scope.
//! Success here is still only the two-child link: parent recursion, the
//! terminal decider and ledger acceptance are downstream consumers.
use super::*;
use std::path::PathBuf;

const PARAMS_ENV: &str = "MORIARTY_SHERLOCK_LINK_PARAMS_DIR";
const ATTEMPT_ENV: &str = "MORIARTY_SHERLOCK_LINK_ATTEMPT_DIR";
const ARTIFACT_ENV: &str = "MORIARTY_SHERLOCK_LINK_ARTIFACT_DIR";
const MAX_K_ENV: &str = "MORIARTY_SHERLOCK_LINK_MAX_K";

fn required_env(name: &str) -> String {
    std::env::var(name)
        .unwrap_or_else(|_| panic!("{name} must name the selected native input"))
}
fn options() -> ProofOptions {
    let max_k: u8 = required_env(MAX_K_ENV)
        .parse()
        .expect("MORIARTY_SHERLOCK_LINK_MAX_K must be a decimal u8");
    assert!((1..=MAX_K).contains(&max_k), "max_k outside 1..={MAX_K}");
    ProofOptions { params_dir: PathBuf::from(required_env(PARAMS_ENV)), max_k }
}

/// Native: compile the real source, prove and checkpoint each child, export
/// the sealed link, then re-verify the frozen export through the separate
/// consumer entrypoint in the same process. The fresh-process consumer is
/// `literal_seven_link_verify_frozen`; run it as its own invocation before
/// crediting independence.
#[test]
#[ignore = "native SRS/keygen/prove: selected resource scope and MORIARTY_SHERLOCK_LINK_* env required"]
fn literal_seven_link_prove_export() {
    let options = options();
    let attempt = PathBuf::from(required_env(ATTEMPT_ENV));
    assert!(!attempt.exists(), "attempt dir must not exist: {}", attempt.display());
    let report = prove_literal_seven_link(&attempt, &options).expect("linked producer");
    assert_eq!(
        report.phases.iter().map(|p| p.phase).collect::<Vec<_>>(),
        [
            "a102-key-reconstruct",
            "a102-prove",
            "a102-checkpoint",
            "a80-key-reconstruct",
            "a80-prove",
            "a80-checkpoint",
            "verify-linked-children",
            "write-link-dir",
        ]
    );
    assert_eq!(report.children.len(), 2);
    for child in &report.children {
        assert!((1..=options.max_k).contains(&child.circuit_k));
        assert_eq!(child.instance_lanes, report.instance_lanes);
        assert!(child.proof_bytes > 0);
    }
    assert_ne!(report.children[0].vk_sha256, report.children[1].vk_sha256);
    let link = attempt.join(report.link_dir);
    let verified = verify_literal_seven_link_dir(&link, &options).expect("frozen consumer");
    assert_eq!(verified.manifest.children[0].vk_sha256, report.children[0].vk_sha256);
    assert_eq!(verified.manifest.children[1].vk_sha256, report.children[1].vk_sha256);
    assert_eq!(verified.verification.a102().vk_sha256, report.children[0].vk_sha256);
    assert_eq!(verified.verification.a80().vk_sha256, report.children[1].vk_sha256);
    eprintln!(
        "[sherlock-link] UNQUALIFIED scoped link only: two Poseidon children; \
         parent, decider and ledger acceptance remain downstream"
    );
}

/// Native independent consumer: recompiles the source in this process and
/// verifies a previously frozen link directory. Intended as a separate
/// process invocation from the producer test above.
#[test]
#[ignore = "native SRS/keygen/verify: selected resource scope and MORIARTY_SHERLOCK_LINK_* env required"]
fn literal_seven_link_verify_frozen() {
    let options = options();
    let dir = PathBuf::from(required_env(ARTIFACT_ENV));
    let (requests, shape, _record) =
        literal_seven_linked_requests().expect("source-derived requests");
    let verified = read_link_dir(&dir, &requests, &options).expect("frozen verification");
    assert_eq!(verified.verification.shape(), shape);
    assert_eq!(verified.manifest.instance_lanes, shape.instance_lanes);
    assert_eq!(
        verified.verification.a102().vk_sha256,
        verified.manifest.children[0].vk_sha256
    );
    assert_eq!(
        verified.verification.a80().vk_sha256,
        verified.manifest.children[1].vk_sha256
    );
    assert_eq!(
        verified.verification.a102().context_hash,
        verified.manifest.children[0].context_hash
    );
    assert_eq!(
        verified.verification.a80().context_hash,
        verified.manifest.children[1].context_hash
    );
}

/// Native negative: one flipped proof byte, re-identified in the manifest so
/// every host layer passes, must refuse at cryptographic verification. A
/// refusal at any earlier layer does not satisfy this case; the assertion
/// pins the verification error code.
#[test]
#[ignore = "native SRS/keygen/verify: selected resource scope and MORIARTY_SHERLOCK_LINK_* env required"]
fn literal_seven_link_verify_refuses_tampered_proof() {
    let options = options();
    let source = PathBuf::from(required_env(ARTIFACT_ENV));
    let dir = std::env::temp_dir()
        .join(format!("moriarty-link-tampered-proof-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir(&dir).unwrap();
    for name in LINK_FILES {
        fs::copy(source.join(name), dir.join(name)).unwrap();
    }
    let mut proof = fs::read(dir.join("a102.proof")).unwrap();
    proof[0] ^= 1;
    fs::write(dir.join("a102.proof"), &proof).unwrap();
    let text = fs::read_to_string(dir.join("link.json")).unwrap();
    let mut manifest = LinkManifest::from_json(&text).unwrap();
    manifest.children[0].proof_sha256 = sha(&proof);
    fs::write(dir.join("link.json"), json_bytes(&manifest).unwrap()).unwrap();
    let err = verify_literal_seven_link_dir(&dir, &options)
        .err()
        .expect("tampered proof must refuse");
    assert_eq!(err.code, "mori::backend::verification", "{}", err.message);
    let _ = fs::remove_dir_all(&dir);
}
