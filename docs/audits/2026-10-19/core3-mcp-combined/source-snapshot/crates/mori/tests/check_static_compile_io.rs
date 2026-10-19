//! Actual public CLI source-static export/reopen only; no native work.
use serde_json::{Value, json};
use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "mori-static-cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn path(&self, n: &str) -> PathBuf {
        self.0.join(n)
    }
    fn setup(&self, id: &str) {
        let ex = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/core/static-compile");
        for (from, to) in [
            (format!("{id}.mori"), "source.mori"),
            (format!("{id}.bindings.json"), "bindings.json"),
            ("signer-policy.json".into(), "signer-policy.json"),
            ("work-limits.json".into(), "work-limits.json"),
        ] {
            std::fs::copy(ex.join(from), self.path(to)).unwrap();
        }
    }
    fn command(&self, mode: &str, inspect: bool) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--color", "never", "compile-call"])
            .arg(self.path("source.mori"))
            .args(["--all-actions", "--static-bindings"])
            .arg(self.path("bindings.json"))
            .args(["--authorization", mode, "--work-limits"])
            .arg(self.path("work-limits.json"));
        if mode == "signed-v2" {
            c.arg("--signer-policy")
                .arg(self.path("signer-policy.json"));
        }
        c.arg(if inspect { "--inspect-static" } else { "--out" })
            .arg(self.path("out"));
        c
    }
    fn json(&self, n: &str) -> Value {
        serde_json::from_slice(&std::fs::read(self.path(n)).unwrap()).unwrap()
    }
    fn write_json(&self, n: &str, v: &Value) {
        std::fs::write(self.path(n), serde_json::to_vec_pretty(v).unwrap()).unwrap();
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn invoke(mut c: Command) -> Output {
    let argv = c
        .get_args()
        .map(|a| a.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    let out = c.output().unwrap();
    if let Some(root) = std::env::var_os("MORI_STATIC_IO_CAPTURE_DIR") {
        let dir = PathBuf::from(root).join(format!(
            "cli-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("argv.json"),
            serde_json::to_vec_pretty(&argv).unwrap(),
        )
        .unwrap();
        std::fs::write(dir.join("stdout"), &out.stdout).unwrap();
        std::fs::write(dir.join("stderr"), &out.stderr).unwrap();
        std::fs::write(
            dir.join("exit"),
            out.status.code().unwrap_or(-1).to_string(),
        )
        .unwrap();
    }
    out
}
fn success(o: Output) -> String {
    assert!(
        o.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    );
    format!(
        "{}{}",
        String::from_utf8_lossy(&o.stdout),
        String::from_utf8_lossy(&o.stderr)
    )
}
fn refused(o: Output) {
    assert!(!o.status.success(), "unexpected success");
}
#[test]
fn public_all_action_exports_and_separate_process_inspect() {
    for id in ["numeric-state", "financial-duty"] {
        for mode in ["unsigned", "signed-v2"] {
            let f = Files::new();
            f.setup(id);
            assert!(
                success(invoke(f.command(mode, false)))
                    .contains("source-static-unproven-uninstalled")
            );
            let marker = f.json("out/static-compile.json");
            assert_eq!(marker["actions"].as_array().unwrap().len(), 2);
            assert!(marker["bindings"].get("economic_bounds").is_none());
            assert_eq!(std::fs::read_dir(f.path("out")).unwrap().count(), 6);
            assert!(
                success(invoke(f.command(mode, true))).contains("source-static-recompile-admitted")
            );
            if let Some(root) = std::env::var_os("MORI_STATIC_IO_CAPTURE_DIR") {
                let dir = PathBuf::from(root).join(format!("bundle-{id}-{mode}"));
                std::fs::create_dir_all(&dir).unwrap();
                for e in std::fs::read_dir(f.path("out")).unwrap() {
                    let e = e.unwrap();
                    std::fs::copy(e.path(), dir.join(e.file_name())).unwrap();
                }
            }
        }
    }
}
#[test]
fn malformed_flag_shapes_refuse_before_file_reads() {
    let variants: Vec<Vec<&str>> = vec![
        vec![],
        vec!["--all-actions"],
        vec!["--all-actions", "--action", "x"],
        vec![
            "--static-bindings",
            "missing",
            "--action",
            "x",
            "--deployment",
            "missing",
            "--out",
            "missing",
        ],
        vec![
            "--all-actions",
            "--static-bindings",
            "missing",
            "--authorization",
            "signed-v1",
            "--work-limits",
            "missing",
            "--out",
            "missing",
        ],
        vec![
            "--all-actions",
            "--static-bindings",
            "missing",
            "--authorization",
            "signed-v2",
            "--work-limits",
            "missing",
            "--out",
            "missing",
        ],
        vec![
            "--all-actions",
            "--static-bindings",
            "missing",
            "--authorization",
            "unsigned",
            "--signer-policy",
            "missing",
            "--work-limits",
            "missing",
            "--out",
            "missing",
        ],
        vec![
            "--all-actions",
            "--static-bindings",
            "missing",
            "--authorization",
            "unsigned",
            "--work-limits",
            "missing",
            "--out",
            "missing",
            "--inspect-static",
            "missing",
        ],
    ];
    for args in variants {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--color", "never", "compile-call", "missing-source"])
            .args(args);
        let o = invoke(c);
        refused(Output {
            status: o.status,
            stdout: o.stdout.clone(),
            stderr: o.stderr.clone(),
        });
        let text = String::from_utf8_lossy(&o.stderr);
        assert!(
            !text.contains("file-metadata") && !text.contains("Read"),
            "must refuse argument shape first: {text}"
        );
    }
}
#[test]
fn strict_cli_inputs_and_stale_admission_leave_no_output() {
    for which in 0..12 {
        let f = Files::new();
        f.setup("financial-duty");
        match which {
            0 => {
                let mut v = f.json("bindings.json");
                v["economic_bounds"] = Value::Null;
                f.write_json("bindings.json", &v);
            }
            1 => {
                let mut v = f.json("bindings.json");
                v["toolchain"]["degree"] = json!(1);
                f.write_json("bindings.json", &v);
            }
            2 => {
                let raw = std::fs::read_to_string(f.path("bindings.json")).unwrap();
                std::fs::write(
                    f.path("bindings.json"),
                    raw.replacen("{", "{\"abi\":\"invented\",", 1),
                )
                .unwrap();
            }
            3 => {
                let mut v = f.json("bindings.json");
                v["source_hash"] = json!("0".repeat(64));
                f.write_json("bindings.json", &v);
            }
            4 => {
                let mut v = f.json("work-limits.json");
                v["profile"] = json!("moriarty-native-call-work-limits/1");
                f.write_json("work-limits.json", &v);
            }
            5 => {
                let mut v = f.json("work-limits.json");
                v["max_actions"] = json!(1);
                f.write_json("work-limits.json", &v);
            }
            6 => {
                let mut v = f.json("work-limits.json");
                v["max_input_bytes"] = json!(10);
                f.write_json("work-limits.json", &v);
            }
            7 => {
                let mut v = f.json("work-limits.json");
                v["max_leaf_bytes"] = json!(10);
                f.write_json("work-limits.json", &v);
            }
            8 => {
                let mut v = f.json("work-limits.json");
                v["max_artifact_bytes"] = json!(1000);
                f.write_json("work-limits.json", &v);
            }
            9 => std::fs::write(f.path("source.mori"), vec![b' '; 1_048_577]).unwrap(),
            10 => std::fs::write(f.path("bindings.json"), [0xff]).unwrap(),
            _ => {
                let mut v = f.json("signer-policy.json");
                v["public_key_hex"] = json!("00".repeat(64));
                f.write_json("signer-policy.json", &v);
            }
        };
        refused(invoke(f.command("signed-v2", false)));
        assert!(!f.path("out").exists(), "preflight {which}");
    }
}
#[test]
fn existing_directory_file_and_dangling_symlink_preserved() {
    for kind in 0..3 {
        let f = Files::new();
        f.setup("numeric-state");
        let out = f.path("out");
        match kind {
            0 => {
                std::fs::create_dir(&out).unwrap();
                std::fs::write(out.join("keep"), b"old").unwrap();
            }
            1 => std::fs::write(&out, b"old").unwrap(),
            _ => {
                #[cfg(unix)]
                std::os::unix::fs::symlink("missing", &out).unwrap();
                #[cfg(not(unix))]
                continue;
            }
        }
        refused(invoke(f.command("unsigned", false)));
        match kind {
            0 => assert_eq!(std::fs::read(out.join("keep")).unwrap(), b"old"),
            1 => assert_eq!(std::fs::read(&out).unwrap(), b"old"),
            _ => assert_eq!(std::fs::read_link(&out).unwrap(), Path::new("missing")),
        };
    }
}
#[test]
fn concurrent_export_has_one_complete_winner() {
    let f = Files::new();
    f.setup("numeric-state");
    let a = f.command("unsigned", false);
    let b = f.command("unsigned", false);
    let first = std::thread::spawn(move || invoke(a));
    let second = std::thread::spawn(move || invoke(b));
    let successes = [
        first.join().unwrap().status.success(),
        second.join().unwrap().status.success(),
    ];
    assert_eq!(successes.iter().filter(|v| **v).count(), 1);
    success(invoke(f.command("unsigned", true)));
}
#[test]
fn independent_caller_bindings_prevent_marker_selected_authority() {
    for kind in 0..6 {
        let f = Files::new();
        f.setup("numeric-state");
        success(invoke(f.command("signed-v2", false)));
        match kind {
            0 => {
                let mut v = f.json("out/static-compile.json");
                v["authorization"] = json!({"mode":"unsigned"});
                f.write_json("out/static-compile.json", &v);
            }
            1 => {
                let mut v = f.json("out/static-compile.json");
                v["actions"].as_array_mut().unwrap().reverse();
                f.write_json("out/static-compile.json", &v);
            }
            2 => std::fs::write(f.path("out/extra.vk.bin"), "invented").unwrap(),
            3 => std::fs::write(f.path("out/action-000001.ir.bin"), b"fabricated").unwrap(),
            4 => {
                let mut v = f.json("bindings.json");
                v["network"] = json!("alternate-unqualified");
                f.write_json("bindings.json", &v);
            }
            _ => {
                let mut v = f.json("out/static-compile.json");
                v["bindings"]["economic_bounds"] = Value::Null;
                f.write_json("out/static-compile.json", &v);
            }
        }
        refused(invoke(f.command("signed-v2", true)));
    }
}
#[test]
fn public_static_inputs_have_no_native_material_and_legacy_selected_action_still_works() {
    let f = Files::new();
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/core/ledger-call/payment");
    let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
    let source = std::fs::read_to_string(root.join("source.mori")).unwrap();
    let p = mori_core::parse(&source).unwrap();
    let mut descriptor: mori_ledger::profile::Deployment =
        mori_ledger::codec::json(&std::fs::read_to_string(root.join("deployment.json")).unwrap())
            .unwrap();
    let abi = mori_ledger::layout::LedgerAbi {
        profile: mori_ledger::profile::ABI.into(),
        language: p.language.clone(),
        semantics: p.semantics.clone(),
        program_hash: mori_ledger::codec::object_hash(&p).unwrap(),
        states: p
            .states
            .iter()
            .map(|s| mori_ledger::layout::StateField {
                name: s.name.clone(),
                ty: s.ty,
                key_hex: mori_ledger::codec::native_hex(&mori_ledger::layout::name_key(
                    "state", &s.name,
                ))
                .unwrap(),
            })
            .collect(),
        actions: p.actions.clone(),
        assets: descriptor.assets.clone(),
        roles: descriptor.roles.clone(),
        duties: descriptor.duties.clone(),
        templates_hash: mori_ledger::layout::templates_hash(),
    };
    let original_metadata = serde_json::to_value(&descriptor).unwrap();
    descriptor.source_hash = mori_ledger::codec::hash(source.as_bytes());
    descriptor.program_hash = abi.program_hash.clone();
    descriptor.abi_hash = mori_ledger::codec::object_hash(&abi).unwrap();
    descriptor.templates_hash = abi.templates_hash;
    let mut before = original_metadata;
    let mut after = serde_json::to_value(&descriptor).unwrap();
    for field in ["source_hash", "program_hash", "abi_hash", "templates_hash"] {
        before.as_object_mut().unwrap().remove(field);
        after.as_object_mut().unwrap().remove(field);
    }
    assert_eq!(
        before, after,
        "all frame/parameter/key/other public metadata is preserved"
    );

    std::fs::write(
        f.path("legacy-deployment.json"),
        serde_json::to_vec_pretty(&descriptor).unwrap(),
    )
    .unwrap();
    c.args(["--color", "never", "compile-call"])
        .arg(root.join("source.mori"))
        .args(["--action", "settle", "--deployment"])
        .arg(f.path("legacy-deployment.json"))
        .arg("--out")
        .arg(f.path("legacy"));
    success(invoke(c));
    if let Some(capture) = std::env::var_os("MORI_STATIC_IO_CAPTURE_DIR") {
        let dir = PathBuf::from(capture).join("legacy-selected-action");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::copy(
            root.join("deployment.json"),
            dir.join("original-deployment.json"),
        )
        .unwrap();
        std::fs::copy(
            f.path("legacy-deployment.json"),
            dir.join("refreshed-public-deployment.json"),
        )
        .unwrap();
        std::fs::copy(root.join("source.mori"), dir.join("source.mori")).unwrap();
        for leaf in ["relation.zkir.bin", "manifest.json"] {
            std::fs::copy(f.path("legacy").join(leaf), dir.join(leaf)).unwrap();
        }
    }
    let names = std::fs::read_dir(f.path("legacy"))
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        names,
        std::collections::BTreeSet::from(["relation.zkir.bin".into(), "manifest.json".into()])
    );
    let source = include_str!("../src/commands/compile_call.rs");
    let static_source = include_str!("../../mori-ledger/src/static_compile.rs");
    for forbidden in [
        "derive_all(",
        "load_new_parameters(",
        ".k()",
        "ProofPreimage",
        "snapshot_public_keyset(",
        "snapshot_public_profile(",
        "PinnedCallParameters",
        "relation::check(",
        "prove_call(",
        "verify_call(",
    ] {
        assert!(
            !static_source.contains(forbidden),
            "static module {forbidden}"
        );
        assert!(!source.contains(forbidden), "CLI {forbidden}");
    }
}
