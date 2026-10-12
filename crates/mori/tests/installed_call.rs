use std::process::Command;
#[test]
fn installed_commands_expose_required_explicit_inputs() {
    for name in ["keygen-call", "prove-call", "verify-call"] {
        let result = Command::new(env!("CARGO_BIN_EXE_mori"))
            .args([name, "--help"])
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        let text = String::from_utf8(result.stdout).unwrap();
        assert!(text.contains("--params-dir"));
        assert!(text.contains("--work-limits"));
        let missing = Command::new(env!("CARGO_BIN_EXE_mori"))
            .arg(name)
            .output()
            .unwrap();
        assert!(!missing.status.success());
    }
}

#[path = "../../mori-ledger/tests/support/mod.rs"]
mod support;
use std::{
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
fn root() -> PathBuf {
    let p = std::env::temp_dir().join(format!(
        "mori-installed-cli-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir(&p).unwrap();
    p
}
fn write_limits(root: &std::path::Path, max_input: u64) {
    let limits = mori_ledger::profile::NativeWorkLimits {
        profile: "moriarty-native-call-work-limits/1".into(),
        max_degree: 19,
        max_actions: 4,
        max_srs_bytes: 1024,
        max_prover_key_bytes: 1024,
        max_verifier_key_bytes: 1024,
        max_input_bytes: max_input,
        max_leaf_bytes: 1_048_576,
        max_artifact_bytes: 4_194_304,
        max_operations: 10000,
        max_public_inputs: 10000,
    };
    std::fs::write(
        root.join("limits.json"),
        serde_json::to_vec(&limits).unwrap(),
    )
    .unwrap();
}
fn keygen(root: &std::path::Path, action: &str) -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
    c.arg("keygen-call")
        .arg(root.join("source.mori"))
        .arg("--seed-deployment")
        .arg(root.join("deployment.json"))
        .arg("--seed-state")
        .arg(root.join("state.bin"))
        .arg("--parameters")
        .arg(root.join("parameters.bin"))
        .args(["--action", action])
        .arg("--params-dir")
        .arg(root.join("no-srs"))
        .arg("--work-limits")
        .arg(root.join("limits.json"))
        .arg("--out")
        .arg(root.join("out"));
    c
}
fn write_fixture(root: &std::path::Path, f: &support::Fixture) {
    std::fs::write(root.join("source.mori"), &f.source).unwrap();
    std::fs::write(
        root.join("deployment.json"),
        serde_json::to_vec(&f.deployment).unwrap(),
    )
    .unwrap();
    std::fs::write(root.join("state.bin"), &f.predecessor).unwrap();
    std::fs::write(root.join("parameters.bin"), &f.parameters).unwrap();
}
#[test]
fn real_keygen_command_refuses_installed_seed_and_unknown_action_before_native_work() {
    let root = root();
    write_limits(&root, 1_048_576);
    let mut f = support::payment();
    let mut state: mori_ledger::layout::NativeState =
        mori_ledger::codec::decode_native(&f.predecessor).unwrap();
    let address = mori_ledger::codec::from_hex(&f.deployment.address_hex).unwrap();
    let mut contract = state.index(address).unwrap();
    let name = b"settle".to_vec().into();
    let mut operation = (*contract.operations.get(&name).unwrap()).clone();
    operation.ir = Some(midnight_storage::arena::Sp::new(
        midnight_onchain_state::state::IrBuf(vec![1]),
    ));
    contract.operations = contract.operations.insert(name, operation);
    state.contract = state.contract.insert(address, contract);
    support::repin(&mut f, &state);
    write_fixture(&root, &f);
    let result = keygen(&root, "settle").output().unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("unproven-operation"));
    assert!(!root.join("out").exists());
    write_fixture(&root, &support::payment());
    let result = keygen(&root, "unknown").arg("--quiet").output().unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("selected-action"));
    assert!(result.stdout.is_empty());
    assert!(!root.join("out/public-keyset.json").exists());
    assert!(!root.join("out").exists());
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn explicit_input_limit_refuses_source_and_descriptor_before_native_work() {
    let root = root();
    let mut f = support::payment();
    write_limits(&root, 1024);
    f.source.push_str(&" ".repeat(1025));
    f.deployment.source_hash = mori_ledger::codec::hash(f.source.as_bytes());
    write_fixture(&root, &f);
    let result = keygen(&root, "settle").output().unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("file-kind-or-size"));
    assert!(!root.join("out").exists());
    let f = support::payment();
    write_fixture(&root, &f);
    // The source fits this bound; its complete descriptor exceeds it.
    assert!(f.source.len() <= 1024);
    assert!(
        std::fs::metadata(root.join("deployment.json"))
            .unwrap()
            .len()
            > 1024
    );
    let result = keygen(&root, "settle").output().unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("file-kind-or-size"));
    assert!(!root.join("out").exists());
    std::fs::remove_dir_all(root).unwrap();
}
#[cfg(unix)]
#[test]
fn work_limit_fifo_is_refused_without_waiting_for_a_writer() {
    let root = root();
    assert!(
        Command::new("mkfifo")
            .arg(root.join("limits.json"))
            .status()
            .unwrap()
            .success()
    );
    let mut child = keygen(&root, "settle")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let start = std::time::Instant::now();
    loop {
        if child.try_wait().unwrap().is_some() {
            break;
        }
        if start.elapsed() > std::time::Duration::from_secs(3) {
            child.kill().unwrap();
            let _ = child.wait();
            panic!("FIFO work limits blocked");
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let result = child.wait_with_output().unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("file-kind-or-size"));
    assert!(!root.join("out").exists());
    std::fs::remove_dir_all(root).unwrap();
}
