//! Public local-unproven preparation consumer; no keys, proofs or ledger application.
use std::process::Command;
#[path = "../../mori-ledger/tests/support/mod.rs"]
mod support;
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};
static SERIAL: AtomicUsize = AtomicUsize::new(0);
struct PublicFiles {
    root: PathBuf,
    action: String,
}
impl PublicFiles {
    fn new(f: &support::Fixture) -> Self {
        let root = std::env::temp_dir().join(format!(
            "mori-ledger-cli-{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&root).unwrap();
        for (name, bytes) in [
            ("source.mori", f.source.as_bytes().to_vec()),
            (
                "deployment.json",
                serde_json::to_vec(&f.deployment).unwrap(),
            ),
            ("request.json", serde_json::to_vec(&f.request).unwrap()),
            ("predecessor.bin", f.predecessor.clone()),
            ("parameters.bin", f.parameters.clone()),
        ] {
            std::fs::write(root.join(name), bytes).unwrap();
        }
        Self {
            root,
            action: f.request.action.clone(),
        }
    }
    fn command(&self, out: &Path) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--color", "never", "prepare-call"]);
        c.arg(self.root.join("source.mori"))
            .arg("--action")
            .arg(&self.action);
        for (flag, file) in [
            ("--deployment", "deployment.json"),
            ("--request", "request.json"),
            ("--predecessor", "predecessor.bin"),
            ("--parameters", "parameters.bin"),
        ] {
            c.arg(flag).arg(self.root.join(file));
        }
        c.arg("--out").arg(out);
        c
    }
}
impl Drop for PublicFiles {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.root).unwrap();
    }
}

#[test]
fn actual_cli_compiles_and_consumes_the_source_static_native_relation() {
    for f in [support::payment(), support::collection()] {
        let files = PublicFiles::new(&f);
        let relation = files.root.join("relation");
        let result = Command::new(env!("CARGO_BIN_EXE_mori"))
            .args(["--color", "never", "compile-call"])
            .arg(files.root.join("source.mori"))
            .arg("--action")
            .arg(&files.action)
            .arg("--deployment")
            .arg(files.root.join("deployment.json"))
            .arg("--out")
            .arg(&relation)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let out = files.root.join("checked");
        let result = files
            .command(&out)
            .arg("--relation")
            .arg(&relation)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let r: serde_json::Value =
            serde_json::from_slice(&std::fs::read(out.join("readback.json")).unwrap()).unwrap();
        assert_eq!(
            r["evidence"]["qualification"],
            "local-relation-checked-unproven"
        );
        assert_eq!(r["evidence"]["proof_verification"], "not-performed");
        assert!(out.join("processed-preimage.bin").exists());
        assert!(out.join("preparation.json").exists());
        let artifact = relation.join("manifest.json");
        let before = std::fs::read(&artifact).unwrap();
        std::fs::write(&artifact, b"untrusted altered manifest").unwrap();
        let refused = files.root.join("refused");
        let result = files
            .command(&refused)
            .arg("--relation")
            .arg(&relation)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(!refused.join("preparation.json").exists());
        std::fs::write(&artifact, before).unwrap();
    }
}

#[test]
fn preparation_command_is_a_public_cli_consumer() {
    let out = Command::new(env!("CARGO_BIN_EXE_mori"))
        .args(["--color", "never", "prepare-call", "--help"])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let help = String::from_utf8(out.stdout).unwrap();
    for flag in [
        "--action",
        "--deployment",
        "--predecessor",
        "--parameters",
        "--request",
        "--out",
    ] {
        assert!(help.contains(flag), "missing public input {flag}");
    }
}

#[test]
fn actual_cli_publishes_all_exact_native_leaves_and_last_marker() {
    for (name, f) in [
        ("payment70", support::payment()),
        ("fee-first43", support::collection()),
        (
            "self12-bob20",
            support::cash(
                "transfer(QA,\"alice\",\"alice\",10); transfer(QA,\"alice\",\"bob\",20); fee(QA,\"alice\",\"alice\",2);",
                100,
            ),
        ),
        (
            "zero-change100",
            support::cash(
                "transfer(QA,\"alice\",\"bob\",0); fee(QA,\"alice\",\"treasury\",0);",
                100,
            ),
        ),
        ("genuine-empty", support::cash("", 0)),
        (
            "self-equal",
            support::cash("transfer(QA,\"alice\",\"alice\",50);", 100),
        ),
        (
            "zero-empty",
            support::cash("transfer(QA,\"alice\",\"bob\",0);", 0),
        ),
        (
            "wide-u128",
            support::cash(
                "transfer(QA,\"alice\",\"bob\",18446744073709551616); transfer(QA,\"alice\",\"bob\",18446744073709551616);",
                1u128 << 65,
            ),
        ),
    ] {
        let files = PublicFiles::new(&f);
        let out = files.root.join("prepared");
        let result = files.command(&out).output().unwrap();
        assert!(
            result.status.success(),
            "{name}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        let expected = mori_ledger::prepare::prepare_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
        )
        .unwrap();
        for (leaf, bytes) in &expected.leaves {
            assert_eq!(
                &std::fs::read(out.join(leaf)).unwrap(),
                bytes,
                "{name}: {leaf}"
            );
        }
        assert_eq!(
            std::fs::read(out.join("preparation.json")).unwrap(),
            expected.marker
        );
        let transcript: midnight_onchain_runtime::transcript::Transcript<
            midnight_storage::db::InMemoryDB,
        > = mori_ledger::codec::decode_artifact(
            &std::fs::read(out.join("guaranteed-transcript.bin")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            mori_ledger::codec::native_hex(&transcript.effects).unwrap(),
            expected.readback.effects_hex
        );
        let marker: serde_json::Value = serde_json::from_slice(&expected.marker).unwrap();
        for (leaf, bytes) in &expected.leaves {
            assert_eq!(
                marker["leaves"][leaf]["sha256"],
                mori_ledger::codec::hash(bytes)
            );
        }
        if let Some(capture) = std::env::var_os("MORI_LEDGER_PUBLIC_CAPTURE") {
            let target = PathBuf::from(capture).join(name);
            std::fs::create_dir(&target).unwrap();
            for entry in std::fs::read_dir(&files.root).unwrap() {
                let entry = entry.unwrap();
                if entry.file_type().unwrap().is_file() {
                    std::fs::copy(entry.path(), target.join(entry.file_name())).unwrap();
                }
            }
            std::fs::create_dir(target.join("prepared")).unwrap();
            for entry in std::fs::read_dir(&out).unwrap() {
                let entry = entry.unwrap();
                std::fs::copy(
                    entry.path(),
                    target.join("prepared").join(entry.file_name()),
                )
                .unwrap();
            }
            std::fs::write(target.join("stdout.txt"), result.stdout).unwrap();
            std::fs::write(target.join("stderr.txt"), result.stderr).unwrap();
        }
    }
}

#[test]
fn actual_cli_failures_do_not_publish_success_or_overwrite_existing_paths() {
    let mut f = support::payment();
    f.request.action = "seed".into();
    let mut files = PublicFiles::new(&f);
    files.action = "settle".into();
    let out = files.root.join("mismatch");
    let result = files.command(&out).output().unwrap();
    assert!(!result.status.success());
    assert!(
        String::from_utf8_lossy(&result.stderr)
            .contains("selected action differs from public request")
    );
    assert!(!out.exists());
    let files = PublicFiles::new(&support::payment());
    let out = files.root.join("existing");
    std::fs::create_dir(&out).unwrap();
    std::fs::write(out.join("offer.bin"), b"existing public bytes").unwrap();
    let result = files.command(&out).output().unwrap();
    assert!(!result.status.success());
    assert_eq!(
        std::fs::read(out.join("offer.bin")).unwrap(),
        b"existing public bytes"
    );
    assert!(!out.join("preparation.json").exists());
    let input = files.root.join("predecessor.bin");
    let bytes = std::fs::read(&input).unwrap();
    let result = files.command(&input).output().unwrap();
    assert!(!result.status.success());
    assert_eq!(std::fs::read(&input).unwrap(), bytes);
    let mut request: serde_json::Value =
        serde_json::from_slice(&std::fs::read(files.root.join("request.json")).unwrap()).unwrap();
    request["private_witness"] = serde_json::json!([]);
    std::fs::write(
        files.root.join("request.json"),
        serde_json::to_vec(&request).unwrap(),
    )
    .unwrap();
    let refused = files.root.join("private-refused");
    let result = files.command(&refused).output().unwrap();
    assert!(!result.status.success());
    assert!(!refused.exists());
}

#[test]
fn two_actual_cli_publishers_have_one_directory_winner() {
    let files = PublicFiles::new(&support::payment());
    let out = files.root.join("raced");
    let first = files
        .command(&out)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let second = files
        .command(&out)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();
    let statuses = [
        first.wait_with_output().unwrap().status.success(),
        second.wait_with_output().unwrap().status.success(),
    ];
    assert_eq!(statuses.into_iter().filter(|n| *n).count(), 1);
    let expected = mori_ledger::prepare::prepare_call(
        &support::payment().source,
        &support::payment().program,
        &support::payment().deployment,
        &support::payment().predecessor,
        &support::payment().parameters,
        &support::payment().request,
    )
    .unwrap();
    assert_eq!(
        std::fs::read(out.join("preparation.json")).unwrap(),
        expected.marker
    );
}

#[test]
fn actual_cli_bounded_source_json_and_native_refusals_leave_no_marker() {
    for kind in 0..9 {
        let files = PublicFiles::new(&support::payment());
        let out = files.root.join("refused");
        match kind {
            0 => std::fs::write(files.root.join("source.mori"), [0xff, 0xfe]).unwrap(),
            1 => std::fs::write(
                files.root.join("source.mori"),
                vec![b' '; mori_ledger::codec::INPUT_BYTES + 1],
            )
            .unwrap(),
            2 => std::fs::write(
                files.root.join("request.json"),
                br#"{"action":"settle","action":"settle"}"#,
            )
            .unwrap(),
            3 => std::fs::write(files.root.join("predecessor.bin"), b"wrong-native-tag").unwrap(),
            4 => {
                let path = files.root.join("predecessor.bin");
                let mut bytes = std::fs::read(&path).unwrap();
                bytes.pop();
                std::fs::write(path, bytes).unwrap();
            }
            5 => {
                let path = files.root.join("predecessor.bin");
                let mut bytes = std::fs::read(&path).unwrap();
                bytes.push(0);
                std::fs::write(path, bytes).unwrap();
            }
            6 => std::fs::write(
                files.root.join("parameters.bin"),
                vec![0u8; mori_ledger::codec::INPUT_BYTES + 1],
            )
            .unwrap(),
            8 => std::fs::write(files.root.join("source.mori"), vec![b' '; 65537]).unwrap(),
            _ => {
                let mut f = support::payment();
                f.request
                    .inputs
                    .insert("amount".into(), mori_core::Value::UInt(mori_core::UInt(69)));
                std::fs::write(
                    files.root.join("request.json"),
                    serde_json::to_vec(&f.request).unwrap(),
                )
                .unwrap();
            }
        }
        let result = files.command(&out).output().unwrap();
        assert!(!result.status.success(), "kind {kind}");
        assert!(!out.exists());
        assert!(!String::from_utf8_lossy(&result.stderr).contains("Prepared"));
    }
    let mut f = support::payment();
    f.source.push_str(&" ".repeat(65536 - f.source.len()));
    f.deployment.source_hash = mori_ledger::codec::hash(f.source.as_bytes());
    let files = PublicFiles::new(&f);
    let out = files.root.join("exact-source-limit");
    let result = files.command(&out).output().unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(out.join("preparation.json").exists());
}
