//! Actual replay CLI diagnostics only; no native proof or ledger APIs.
use mori_backend::{
    Target,
    replay::{self, ReplayAuthorization},
};
use mori_core::Scenario;
use serde_json::{Value, json};
use std::{
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Files(PathBuf);
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
impl Files {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-challenge-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        let f = Self(path);
        let o: Value = serde_json::from_str(include_str!(
            "../../mori-core/tests/fixtures/bound-replay-oracle.json"
        ))
        .unwrap();
        let c = &o["contrasting"];
        f.write("source.mori", c["source"].as_str().unwrap());
        f.write("scenario.json", c["scenario"].to_string());
        f.write("target.json", c["unsigned_target"].to_string());
        f.write("record.json", f.record().to_string());
        f
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
    fn write(&self, name: &str, bytes: impl AsRef<[u8]>) {
        std::fs::write(self.path(name), bytes).unwrap();
    }
    fn read(&self, name: &str) -> Value {
        serde_json::from_slice(&std::fs::read(self.path(name)).unwrap()).unwrap()
    }
    fn record(&self) -> Value {
        let source = std::fs::read_to_string(self.path("source.mori")).unwrap();
        let scenario: Scenario = serde_json::from_value(self.read("scenario.json")).unwrap();
        let target: Target = serde_json::from_value(self.read("target.json")).unwrap();
        serde_json::to_value(
            replay::generate(
                &source,
                "collect",
                &scenario,
                &target,
                ReplayAuthorization::Unsigned,
            )
            .unwrap(),
        )
        .unwrap()
    }
    fn command(&self) -> Command {
        self.command_action("collect")
    }
    fn command_action(&self, action: &str) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--color", "never", "replay"])
            .arg(self.path("source.mori"))
            .args(["--action", action, "--scenario"])
            .arg(self.path("scenario.json"))
            .arg("--target")
            .arg(self.path("target.json"))
            .arg("--record")
            .arg(self.path("record.json"))
            .arg("--challenge-out")
            .arg(self.path("challenge.json"));
        c
    }
    fn run(&self) -> Output {
        self.command().output().unwrap()
    }
    fn tamper(&self, path: &str, value: Value) {
        let mut r = self.read("record.json");
        *r.pointer_mut(path).unwrap() = value;
        r["record_hash"] = "00".repeat(32).into();
        self.write("record.json", r.to_string());
    }
}
fn report(o: &Output) -> Value {
    serde_json::from_slice(&o.stdout)
        .unwrap_or_else(|_| panic!("{}", String::from_utf8_lossy(&o.stderr)))
}
#[test]
fn actual_cli_exports_early_semantic_difference_and_trusted_roots() {
    let f = Files::new();
    let actual = f.record();
    let mut bad = actual.clone();
    bad["body"]["execution"]["steps"][0]["index"] = 99.into();
    bad["body"]["execution"]["steps"][7]["index"] = 98.into();
    bad["body"]["claim"]["public_values"][0] = "00".repeat(32).into();
    bad["record_hash"] = "00".repeat(32).into();
    f.write("record.json", bad.to_string());
    let bytes_before = std::fs::read(f.path("record.json")).unwrap();
    let out = f.run();
    assert!(!out.status.success());
    assert_eq!(
        report(&out)["error"],
        json!({"kind":"divergence", "step":0,"field":"/body/execution/steps/0/index"})
    );
    let challenge = f.read("challenge.json");
    assert_eq!(challenge["format"], "moriarty-reference-challenge/1");
    assert_eq!(
        challenge["qualification"],
        "local-reference-replay-diagnostic"
    );
    assert_eq!(
        challenge["divergence"],
        json!({"step":0,"field":"/body/execution/steps/0/index"})
    );
    for field in [
        "record_hash",
        "input_hash",
        "trace_hash",
        "zkir_hash",
        "network",
        "anchor",
    ] {
        let expected = if field == "record_hash" {
            &actual[field]
        } else {
            &actual["body"][field]
        };
        assert_eq!(&challenge["trusted"][field], expected, "{field}");
    }
    for field in [
        "source_hash",
        "program_hash",
        "scenario_hash",
        "target_hash",
        "action",
    ] {
        assert_eq!(challenge["trusted"][field], actual["body"]["claim"][field]);
    }
    assert_eq!(std::fs::read(f.path("record.json")).unwrap(), bytes_before);
}
#[test]
fn first_effect_last_and_absent_steps_are_exported_in_order() {
    for (path, value, step) in [
        ("/body/execution/steps/1/effects/0/amount", json!("4"), 1),
        ("/body/execution/steps/7/index", json!(99), 7),
    ] {
        let f = Files::new();
        f.tamper(path, value);
        let out = f.run();
        assert_eq!(report(&out)["error"]["field"], path);
        assert_eq!(f.read("challenge.json")["divergence"]["step"], step);
    }
    let f = Files::new();
    let mut r = f.read("record.json");
    r["body"]["execution"]["steps"]
        .as_array_mut()
        .unwrap()
        .pop();
    f.write("record.json", r.to_string());
    assert_eq!(
        report(&f.run())["error"]["field"],
        "/body/execution/steps/7"
    );
    assert_eq!(f.read("challenge.json")["divergence"]["step"], 7);
}
#[test]
fn equality_and_input_refusals_never_publish_challenges() {
    let f = Files::new();
    let out = f.run();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(report(&out)["status"], "accepted");
    assert!(!f.path("challenge.json").exists());
    for bytes in [
        "{}".into(),
        " ".repeat(1_048_577),
        "{\"body\":1,\"body\":2}".into(),
    ] {
        f.write("record.json", bytes);
        let out = f.run();
        assert!(!out.status.success());
        assert_eq!(report(&out)["error"]["kind"], "input");
        assert!(!f.path("challenge.json").exists());
    }
    f.write("record.json", f.record().to_string());
    f.write("source.mori", "bad source");
    assert_eq!(report(&f.run())["error"]["kind"], "input");
    assert!(!f.path("challenge.json").exists());
}
#[test]
fn cross_source_and_domain_do_not_supply_their_own_trusted_context() {
    let f = Files::new();
    let actual = f.record();
    f.tamper("/body/anchor", json!("attacker-anchor"));
    let out = f.run();
    assert_eq!(report(&out)["error"]["field"], "/body/anchor");
    assert_eq!(
        f.read("challenge.json")["trusted"]["anchor"],
        actual["body"]["anchor"]
    );
    std::fs::remove_file(f.path("challenge.json")).unwrap();
    f.write("record.json", actual.to_string());
    let source = std::fs::read_to_string(f.path("source.mori")).unwrap();
    f.write("source.mori", format!("{source}\n"));
    assert_eq!(report(&f.run())["error"]["kind"], "divergence");
    assert_eq!(
        f.read("challenge.json")["trusted"]["source_hash"],
        f.record()["body"]["claim"]["source_hash"]
    );
}
#[test]
fn publication_refuses_existing_files_directories_symlinks_and_races() {
    let f = Files::new();
    f.tamper("/body/execution/steps/0/index", json!(99));
    f.write("challenge.json", "existing");
    assert!(!f.run().status.success());
    assert_eq!(
        std::fs::read(f.path("challenge.json")).unwrap(),
        b"existing"
    );
    std::fs::remove_file(f.path("challenge.json")).unwrap();
    std::fs::create_dir(f.path("challenge.json")).unwrap();
    assert!(!f.run().status.success());
    std::fs::remove_dir(f.path("challenge.json")).unwrap();
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(f.path("missing"), f.path("challenge.json")).unwrap();
        assert!(!f.run().status.success());
        assert!(!f.path("missing").exists());
        std::fs::remove_file(f.path("challenge.json")).unwrap();
    }
    let a = f
        .command()
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let b = f
        .command()
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let a = a.wait_with_output().unwrap();
    let b = b.wait_with_output().unwrap();
    assert!(!a.status.success());
    assert!(!b.status.success());
    let kinds = [
        report(&a)["error"]["kind"].clone(),
        report(&b)["error"]["kind"].clone(),
    ];
    assert_eq!(kinds.iter().filter(|v| **v == "divergence").count(), 1);
    assert_eq!(kinds.iter().filter(|v| **v == "output").count(), 1);
    assert_eq!(f.read("challenge.json")["divergence"]["step"], 0);
    assert!(!std::fs::read_dir(&f.0).unwrap().any(|e| {
        e.unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".mori-proof")
    }));
}
#[test]
fn oversized_escaped_sidecar_is_refused_before_publication() {
    let f = Files::new();
    let mut t = f.read("target.json");
    t["anchor"] = "🦀".repeat(256).into();
    f.write("target.json", t.to_string());
    f.write("record.json", f.record().to_string());
    f.tamper("/body/execution/steps/0/index", json!(99));
    let out = f.run();
    assert!(!out.status.success());
    assert!(!f.path("challenge.json").exists());
    assert_eq!(report(&out)["error"]["kind"], "output");
}

#[test]
fn signed_cli_validates_independent_authority_and_never_exports_attachment() {
    let f = Files::new();
    for (name, bytes) in [
        (
            "source.mori",
            include_str!(
                "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori"
            ),
        ),
        (
            "scenario.json",
            include_str!(
                "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json"
            ),
        ),
        (
            "target.json",
            include_str!(
                "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json"
            ),
        ),
        (
            "authority.json",
            include_str!(
                "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.authority.json"
            ),
        ),
        (
            "signed.json",
            include_str!(
                "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.signed.json"
            ),
        ),
    ] {
        f.write(name, bytes);
    }
    let source = std::fs::read_to_string(f.path("source.mori")).unwrap();
    let scenario = serde_json::from_value(f.read("scenario.json")).unwrap();
    let target = serde_json::from_value(f.read("target.json")).unwrap();
    let authority = mori_backend::authorization::ExpectedAuthority::from_json(
        &f.read("authority.json").to_string(),
    )
    .unwrap();
    let signature =
        mori_backend::authorization::SignedIntent::from_json(&f.read("signed.json").to_string())
            .unwrap();
    let actual = replay::generate(
        &source,
        "settle",
        &scenario,
        &target,
        ReplayAuthorization::Signed {
            authority: &authority,
            at: mori_core::UInt(15),
            signature: &signature,
        },
    )
    .unwrap();
    f.write("record.json", serde_json::to_string(&actual).unwrap());
    f.tamper("/body/execution/steps/1/effects/0/amount", json!("69"));
    let signed_command = || {
        let mut c = f.command_action("settle");
        c.args(["--profile", "moriarty-jubjub-owner-intent/1", "--authority"])
            .arg(f.path("authority.json"))
            .args(["--at", "15", "--signature"])
            .arg(f.path("signed.json"));
        c
    };
    let out = signed_command().output().unwrap();
    assert_eq!(report(&out)["error"]["kind"], "divergence");
    let bytes = std::fs::read_to_string(f.path("challenge.json")).unwrap();
    assert!(!bytes.contains(&signature.signature_hex));
    assert!(!bytes.contains(&signature.signature_hex[..128]));
    assert!(!bytes.contains(&signature.signature_hex[128..]));
    for label in [
        "signature_hex",
        "message_hex",
        "proof_hex",
        "private_transcript",
        "private_inputs",
        "public_key_hex",
    ] {
        assert!(!bytes.contains(label), "{label}");
    }
    assert_eq!(
        f.read("challenge.json")["trusted"]["authority_hash"],
        serde_json::to_value(&actual.body.authority_hash).unwrap()
    );
    std::fs::remove_file(f.path("challenge.json")).unwrap();
    let mut a = f.read("authority.json");
    a["predecessor"] = "other-predecessor".into();
    f.write("authority.json", a.to_string());
    assert_eq!(
        report(&signed_command().output().unwrap())["error"]["kind"],
        "input"
    );
    assert!(!f.path("challenge.json").exists());
}
