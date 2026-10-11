use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        let dir = std::env::temp_dir().join(format!(
            "mori-native-proof-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&dir).unwrap();
        std::fs::write(dir.join("source.mori"), "language \"moriarty/1\"; agreement Proof { state count: UInt = 0; action run(amount: UInt) { next count = count + amount; emit \"count\"(count); } }").unwrap();
        std::fs::write(dir.join("scenario.json"), r#"{"state":{},"inputs":{"amount":{"type":"UInt","value":"7"}},"balances":[],"liabilities":[]}"#).unwrap();
        let target = mori_backend::Target::ledger9("local", "native-proof-qa");
        std::fs::write(
            dir.join("target.json"),
            serde_json::to_vec(&target).unwrap(),
        )
        .unwrap();
        Self(dir)
    }
    fn command(&self, operation: &str) -> Command {
        self.command_with_params(operation, &self.0.join("missing-params"))
    }
    fn command_with_params(&self, operation: &str, params: &Path) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_mori"));
        cmd.args(["--color", "never", operation])
            .arg(self.0.join("source.mori"))
            .args(["--action", "run", "--scenario"])
            .arg(self.0.join("scenario.json"))
            .arg("--target")
            .arg(self.0.join("target.json"))
            .arg("--params-dir")
            .arg(params);
        cmd
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
#[test]
fn proof_publication_preserves_existing_destination() {
    let files = Files::new();
    let out = files.0.join("proof.json");
    std::fs::write(&out, b"original user content").unwrap();
    let output = files
        .command("prove")
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("exists"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    assert_eq!(std::fs::read(&out).unwrap(), b"original user content");
}
#[test]
fn verify_rejects_duplicate_bundle_keys_before_key_generation() {
    let files = Files::new();
    let proof = files.0.join("duplicate.json");
    std::fs::write(&proof, r#"{"format":"first","format":"second"}"#).unwrap();
    let output = files
        .command("verify")
        .arg("--proof")
        .arg(&proof)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("duplicate"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    assert_eq!(
        std::fs::read_to_string(&proof).unwrap(),
        r#"{"format":"first","format":"second"}"#
    );
}
#[test]
fn invalid_proof_cap_refuses_without_creating_output() {
    let files = Files::new();
    let out = files.0.join("new-proof.json");
    let output = files
        .command("prove")
        .args(["--max-k", "0", "--out"])
        .arg(&out)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("cap") || stderr.contains("limit"),
        "{stderr}"
    );
    assert!(output.stdout.is_empty());
    assert!(!out.exists());
}
#[cfg(unix)]
#[test]
fn proof_publication_preserves_dangling_symlink() {
    let files = Files::new();
    let out = files.0.join("proof-link.json");
    let destination = files.0.join("uncreated.json");
    std::os::unix::fs::symlink(&destination, &out).unwrap();
    let output = files
        .command("prove")
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("exists"));
    assert!(output.stdout.is_empty());
    assert_eq!(std::fs::read_link(&out).unwrap(), destination);
    assert!(!destination.exists());
}
#[test]
fn missing_srs_does_not_publish_or_change_inputs() {
    let files = Files::new();
    let before = std::fs::read(files.0.join("source.mori")).unwrap();
    let out = files.0.join("new.json");
    let output = files
        .command("prove")
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("SRS"));
    assert!(output.stdout.is_empty());
    assert!(!out.exists());
    assert_eq!(std::fs::read(files.0.join("source.mori")).unwrap(), before);
    assert!(!std::fs::read_dir(&files.0).unwrap().any(|p| {
        p.unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".mori-proof-")
    }));
}
#[test]
#[ignore = "requires explicitly supplied pinned local SRS via MORIARTY_TEST_PARAMS"]
fn genuine_native_source_proof_verifies_and_changed_proof_refuses() {
    let params = PathBuf::from(
        std::env::var_os("MORIARTY_TEST_PARAMS").expect("explicit offline SRS directory required"),
    );
    let files = Files::new();
    let out = files.0.join("proof.json");
    let output = files
        .command_with_params("prove", &params)
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let produced: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        produced["execution"]["post"]["state"]["count"],
        serde_json::json!({"type":"UInt","value":"7"})
    );
    let output = files
        .command_with_params("verify", &params)
        .arg("--proof")
        .arg(&out)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let verified: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(verified["execution"], produced["execution"]);
    let mut bundle: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&out).unwrap()).unwrap();
    let hex = bundle["proof_hex"].as_str().unwrap().to_owned();
    bundle["proof_hex"] = format!("{hex}00").into();
    std::fs::write(&out, serde_json::to_vec(&bundle).unwrap()).unwrap();
    let output = files
        .command_with_params("verify", &params)
        .arg("--proof")
        .arg(&out)
        .output()
        .unwrap();
    assert!(
        !output.status.success(),
        "native verification must reject a suffixed proof"
    );
    assert!(output.stdout.is_empty());
}
#[test]
#[ignore = "requires explicitly supplied pinned local SRS via MORIARTY_TEST_PARAMS"]
fn genuine_native_financial_readback_and_context_controls() {
    let params = PathBuf::from(
        std::env::var_os("MORIARTY_TEST_PARAMS").expect("explicit offline SRS directory required"),
    );
    let files = Files::new();
    std::fs::write(
        files.0.join("source.mori"),
        include_str!("../../../examples/core/native-payment/payment.mori"),
    )
    .unwrap();
    std::fs::write(
        files.0.join("scenario.json"),
        include_str!("../../../examples/core/native-payment/scenario.json"),
    )
    .unwrap();
    let out = files.0.join("financial.json");
    let output = files
        .command_with_params("prove", &params)
        .args(["--max-k", "10"])
        .arg("--out")
        .arg(&out)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let produced: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        produced["execution"]["post"]["state"]["paid"],
        serde_json::json!({"type":"UInt","value":"9"})
    );
    let balances = produced["execution"]["post"]["balances"]
        .as_array()
        .unwrap();
    for (account, amount) in [
        ("alice", "91"),
        ("bob", "17"),
        ("treasury", "5"),
        ("reserve", "50"),
    ] {
        assert_eq!(
            balances.iter().find(|b| b["account"] == account).unwrap()["amount"],
            amount
        );
    }
    let effects = produced["execution"]["effects"].as_array().unwrap();
    assert_eq!(effects.len(), 4);
    for (effect, kind) in effects.iter().zip(["Transfer", "Fee", "State", "Emit"]) {
        assert_eq!(effect["kind"], kind);
    }
    println!(
        "financial_native_k={} proof_bytes={} target_sha256={}",
        produced["circuit_k"],
        produced["proof_bytes"],
        hash_bytes(&std::fs::read(files.0.join("target.json")).unwrap())
    );
    let output = files
        .command_with_params("verify", &params)
        .args(["--max-k", "10"])
        .arg("--proof")
        .arg(&out)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let verified: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(verified["execution"], produced["execution"]);
    let original = std::fs::read(&out).unwrap();
    let scenario_path = files.0.join("scenario.json");
    let good: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&scenario_path).unwrap()).unwrap();
    for modified in [
        {
            let mut s = good.clone();
            s["inputs"]["amount"]["value"] = "8".into();
            s
        },
        {
            let mut s = good.clone();
            s["balances"][3]["amount"] = "51".into();
            s
        },
    ] {
        std::fs::write(&scenario_path, serde_json::to_vec(&modified).unwrap()).unwrap();
        let output = files
            .command_with_params("verify", &params)
            .args(["--max-k", "10"])
            .arg("--proof")
            .arg(&out)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(String::from_utf8_lossy(&output.stderr).contains("metadata differs"));
        assert_eq!(std::fs::read(&out).unwrap(), original);
    }
    let changed: mori_core::Scenario =
        serde_json::from_slice(&std::fs::read(&scenario_path).unwrap()).unwrap();
    let source = std::fs::read_to_string(files.0.join("source.mori")).unwrap();
    let execution =
        mori_core::simulate(&mori_core::parse(&source).unwrap(), "run", &changed).unwrap();
    let hash = |value: &serde_json::Value| -> String {
        hash_bytes(&mori_core::intent::canonical(value).unwrap())
    };
    let mut forged: serde_json::Value = serde_json::from_slice(&original).unwrap();
    forged["scenario_hash"] = hash(&serde_json::to_value(&changed).unwrap()).into();
    forged["execution_hash"] = hash(&serde_json::to_value(&execution).unwrap()).into();
    std::fs::write(&out, serde_json::to_vec(&forged).unwrap()).unwrap();
    let output = files
        .command_with_params("verify", &params)
        .args(["--max-k", "10"])
        .arg("--proof")
        .arg(&out)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("mori::backend::verification"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::write(&scenario_path, serde_json::to_vec(&good).unwrap()).unwrap();
    for mutation in ["suffix", "bitflip"] {
        let mut bad: serde_json::Value = serde_json::from_slice(&original).unwrap();
        let mut hex = bad["proof_hex"].as_str().unwrap().to_owned();
        if mutation == "suffix" {
            hex.push_str("00");
        } else {
            let first = if hex.starts_with("00") { "01" } else { "00" };
            hex.replace_range(..2, first);
        }
        bad["proof_hex"] = hex.into();
        std::fs::write(&out, serde_json::to_vec(&bad).unwrap()).unwrap();
        let output = files
            .command_with_params("verify", &params)
            .args(["--max-k", "10"])
            .arg("--proof")
            .arg(&out)
            .output()
            .unwrap();
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("mori::backend::verification"),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
fn hash_bytes(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
