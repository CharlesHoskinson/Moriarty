//! Actual verify-intent production consumer with public fixed reference signatures.
use serde_json::{Value, json};
use std::{
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};
static NEXT: AtomicUsize = AtomicUsize::new(0);
fn fixtures() -> Value {
    serde_json::from_str(include_str!("fixtures/metered-intents/public-vectors.json")).unwrap()
}
struct Files {
    path: PathBuf,
    vector: Value,
    name: String,
}
impl Files {
    fn new(name: &str, scheme: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "mori-metered-intent-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).unwrap();
        let vector = fixtures()[name].clone();
        let f = Self {
            path,
            vector,
            name: format!("{name}-{scheme}"),
        };
        f.write("source.mori", f.vector["source"].as_str().unwrap());
        f.json("scenario.json", &f.vector["scenario"]);
        f.json("authority.json", &f.vector["vectors"][scheme]["authority"]);
        f.json("signed.json", &f.vector["vectors"][scheme]["signed"]);
        f
    }
    fn write(&self, name: &str, bytes: &str) {
        std::fs::write(self.path.join(name), bytes).unwrap();
    }
    fn json(&self, name: &str, value: &Value) {
        self.write(name, &value.to_string());
    }
    fn command(&self) -> Command {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--color", "never", "verify-intent"])
            .arg(self.path.join("source.mori"))
            .args(["--action", "run", "--scenario"])
            .arg(self.path.join("scenario.json"))
            .arg("--authority")
            .arg(self.path.join("authority.json"))
            .arg("--signature")
            .arg(self.path.join("signed.json"))
            .args(["--at", "15"]);
        c
    }
    fn invoke(&self, c: &mut Command) -> Output {
        let o = c.output().unwrap();
        if let Ok(root) = std::env::var("MORI_METERED_INTENT_EVIDENCE") {
            let p = PathBuf::from(root).join(format!(
                "{}-{}",
                self.name,
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(&p).unwrap();
            std::fs::write(p.join("stdout.txt"), &o.stdout).unwrap();
            std::fs::write(p.join("stderr.txt"), &o.stderr).unwrap();
            std::fs::write(p.join("result.json"),json!({"exit":o.status.code(),"program":c.get_program().to_string_lossy(),"argv":c.get_args().map(|a|a.to_string_lossy().to_string()).collect::<Vec<_>>()}).to_string()).unwrap();
        }
        o
    }
    fn run(&self, cap: Option<&str>) -> Output {
        let mut c = self.command();
        if let Some(cap) = cap {
            c.arg(format!("--logical-cap={cap}"));
        }
        self.invoke(&mut c)
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}
fn decoded(o: &Output) -> Value {
    assert!(
        o.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&o.stderr)
    );
    serde_json::from_slice(&o.stdout).unwrap()
}
fn cost(j: &Value, cap: &str, used: &str) {
    assert_eq!(j["profile"], "moriarty-reference-logical-cost/1");
    assert_eq!(j["logical"], json!({"cap":cap,"used":used}));
}
fn refused(f: &Files, cap: &str, used: &str, code: &str, step: Option<u16>) {
    let o = f.run(Some(cap));
    assert!(
        !o.status.success(),
        "cap {cap} accepted: {}",
        String::from_utf8_lossy(&o.stdout)
    );
    let j = decoded(&o);
    cost(&j, cap, used);
    assert_eq!(j["status"], "rejected");
    assert_eq!(j["error"]["code"], format!("mori::core::{code}"));
    assert_eq!(j["error"]["step"], json!(step));
    for key in ["execution", "effects", "post", "signature_valid"] {
        assert!(j.get(key).is_none());
    }
}
#[test]
fn payment_cap_three_refuses_in_actual_cli() {
    for scheme in ["schnorr", "ecdsa"] {
        refused(&Files::new("payment", scheme), "3", "3", "meter", Some(1));
        refused(&Files::new("payment", scheme), "1", "1", "meter", Some(0));
    }
}
#[test]
fn exact_caps_preserve_legacy_stdout_execution_and_fixed_intent_bytes() {
    for scheme in ["schnorr", "ecdsa"] {
        for (name, total, step) in [
            ("counter", "16", 4),
            ("payment", "4", 1),
            ("two_asset_duty", "18", 7),
        ] {
            let f = Files::new(name, scheme);
            let ordinary = f.run(None);
            assert!(ordinary.status.success());
            let expected = decoded(&ordinary);
            assert_eq!(expected["signature_valid"], true);
            assert_eq!(
                expected["qualification"],
                "local-signature-and-reference-execution"
            );
            assert_eq!(expected.as_object().unwrap().len(), 4);
            let p = mori_core::parse(f.vector["source"].as_str().unwrap()).unwrap();
            let s = serde_json::from_value(f.vector["scenario"].clone()).unwrap();
            let a =
                serde_json::from_value(f.vector["vectors"][scheme]["authority"].clone()).unwrap();
            let signed =
                serde_json::from_value(f.vector["vectors"][scheme]["signed"].clone()).unwrap();
            let execution = mori_core::intent::verify_and_simulate(
                &p,
                &mori_core::intent::hash_bytes(f.vector["source"].as_str().unwrap().as_bytes()),
                "run",
                &s,
                &a,
                &signed,
                mori_core::UInt(15),
            )
            .unwrap();
            let legacy = json!({"status":"accepted","signature_valid":true,"qualification":"local-signature-and-reference-execution","execution":execution});
            assert_eq!(
                ordinary.stdout,
                format!("{}\n", mori_core::json::export(&legacy).unwrap()).into_bytes()
            );
            for cap in [total, "340282366920938463463374607431768211455"] {
                let o = f.run(Some(cap));
                assert!(o.status.success());
                let j = decoded(&o);
                cost(&j, cap, total);
                assert_eq!(j["execution"], expected["execution"]);
                assert_eq!(j["qualification"], expected["qualification"]);
                assert_eq!(j["signature_valid"], true);
            }
            let mut review = f.command();
            review.args(["--logical-cap", total, "--review"]);
            let output = f.invoke(&mut review);
            assert!(output.status.success());
            let text = String::from_utf8(output.stdout).unwrap();
            assert!(text.starts_with(
                "Signature and local authority constraints verified at logical time 15.\n"
            ));
            let reviewed: Value = serde_json::from_str(&text[text.find('{').unwrap()..]).unwrap();
            cost(&reviewed, total, total);
            refused(&f, "0", "0", "meter", Some(0));
            let below = (total.parse::<u128>().unwrap() - 1).to_string();
            refused(&f, &below, &below, "meter", Some(step));
            assert_eq!(f.run(None).stdout, ordinary.stdout);
        }
    }
}
#[test]
fn late_authority_and_runtime_refusals_retain_used_cost_without_success_review() {
    for scheme in ["schnorr", "ecdsa"] {
        for (name, total, used, code, step) in [
            ("payment_gross_cap", "4", "4", "authority", None),
            ("payment_fee_cap", "4", "4", "authority", None),
            ("payment_net_floor", "4", "4", "authority", None),
            ("payment_missing_limit", "4", "4", "authority", None),
            ("two_asset_duty_gross_cap", "18", "18", "authority", None),
            ("two_asset_duty_fee_cap", "18", "18", "authority", None),
            ("two_asset_duty_net_floor", "18", "18", "authority", None),
            (
                "two_asset_duty_liability_cap",
                "18",
                "18",
                "authority",
                None,
            ),
            ("two_asset_duty_other_fee", "18", "18", "authority", None),
            (
                "two_asset_duty_missing_limit",
                "18",
                "18",
                "authority",
                None,
            ),
            ("insufficient_fee", "4", "4", "balance", Some(1)),
            ("requires", "16", "8", "requires", Some(1)),
            ("wrong_input", "0", "0", "input", None),
            ("missing_input", "0", "0", "input", None),
            ("duplicate_balance", "0", "0", "balance", None),
            ("missing_balance", "0", "0", "balance", None),
            ("wrong_asset", "0", "0", "asset", None),
        ] {
            let f = Files::new(name, scheme);
            refused(&f, total, used, code, step);
            let ordinary = f.run(None);
            let old = decoded(&ordinary);
            assert!(!ordinary.status.success());
            assert_eq!(
                ordinary.stdout,
                format!("{}\n", mori_core::json::export(&old).unwrap()).into_bytes()
            );
            assert_eq!(old["error"], decoded(&f.run(Some(total)))["error"]);
            let mut c = f.command();
            c.args(["--logical-cap", total, "--review"]);
            let o = f.invoke(&mut c);
            assert!(!o.status.success());
            assert!(!String::from_utf8_lossy(&o.stdout).contains("constraints verified"));
            decoded(&o);
        }
    }
    refused(
        &Files::new("insufficient_fee", "schnorr"),
        "3",
        "3",
        "meter",
        Some(1),
    );
    refused(&Files::new("requires", "ecdsa"), "7", "7", "meter", Some(1));
    refused(
        &Files::new("two_asset_duty_gross_cap", "ecdsa"),
        "17",
        "17",
        "meter",
        Some(7),
    );
}
#[test]
fn signed_root_signature_and_validity_failures_precede_meter() {
    for scheme in ["schnorr", "ecdsa"] {
        for field in [
            "source_hash",
            "program_hash",
            "scenario_hash",
            "action",
            "profile",
            "semantics",
        ] {
            let f = Files::new("counter", scheme);
            let mut signed = f.vector["vectors"][scheme]["signed"].clone();
            signed["intent"][field] = json!("changed");
            f.json("signed.json", &signed);
            refused(&f, "0", "0", "intent", None);
        }
        for field in [
            "network",
            "anchor",
            "program_address",
            "target_hash",
            "nonce",
            "predecessor",
            "public_key_hex",
        ] {
            let f = Files::new("counter", scheme);
            let mut authority = f.vector["vectors"][scheme]["authority"].clone();
            authority[field] = json!(if field == "nonce" { "8" } else { "changed" });
            f.json("authority.json", &authority);
            refused(
                &f,
                "0",
                "0",
                if field == "public_key_hex" {
                    "signature"
                } else {
                    "intent"
                },
                None,
            );
        }
        for sig in [
            "00".repeat(64),
            fixture_signature(scheme).to_uppercase(),
            fixture_signature(scheme)[..126].into(),
        ] {
            let f = Files::new("counter", scheme);
            let mut signed = f.vector["vectors"][scheme]["signed"].clone();
            signed["signature_hex"] = json!(sig);
            f.json("signed.json", &signed);
            refused(&f, "0", "0", "signature", None);
        }
        for at in ["9", "10", "20", "21"] {
            let f = Files::new("counter", scheme);
            let c = f.command();
            let args = c.get_args().map(|x| x.to_owned()).collect::<Vec<_>>();
            let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
            c.args(&args[..args.len() - 2]).args([
                "--at",
                at,
                "--logical-cap",
                if at == "10" || at == "20" { "16" } else { "0" },
            ]);
            let o = f.invoke(&mut c);
            let j = decoded(&o);
            assert_eq!(o.status.success(), at == "10" || at == "20");
            cost(
                &j,
                if o.status.success() { "16" } else { "0" },
                if o.status.success() { "16" } else { "0" },
            );
        }
        let f = Files::new("counter", scheme);
        f.write(
            "source.mori",
            fixtures()["fresh_counter"]["source"].as_str().unwrap(),
        );
        refused(&f, "16", "0", "intent", None);
        assert!(
            Files::new("fresh_counter", scheme)
                .run(Some("16"))
                .status
                .success()
        );
    }
    let f = Files::new("counter", "ecdsa");
    let mut signed = f.vector["vectors"]["ecdsa"]["signed"].clone();
    signed["signature_hex"] = f.vector["vectors"]["ecdsa"]["high_s_signature_hex"].clone();
    f.json("signed.json", &signed);
    refused(&f, "0", "0", "signature", None);
}
fn fixture_signature(scheme: &str) -> String {
    fixtures()["counter"]["vectors"][scheme]["signed"]["signature_hex"]
        .as_str()
        .unwrap()
        .into()
}
#[test]
fn canonical_caps_and_native_route_conflicts_refuse_before_file_io() {
    let f = Files::new("payment", "schnorr");
    std::fs::remove_dir_all(&f.path).unwrap();
    for cap in [
        "",
        "-1",
        "+1",
        "00",
        "01",
        "1_0",
        "1.0",
        " 1",
        "1 ",
        "１２",
        "340282366920938463463374607431768211456",
    ] {
        let o = f.run(Some(cap));
        assert!(!o.status.success());
        assert!(o.stdout.is_empty());
        let e = String::from_utf8_lossy(&o.stderr);
        assert!(e.contains("--logical-cap"), "{e}");
        assert!(!e.contains("No such file"), "{e}");
    }
    for flags in [
        vec!["--profile", "moriarty-jubjub-owner-intent/1"],
        vec!["--target", "missing-target.json"],
        vec![
            "--profile",
            "moriarty-jubjub-owner-intent/1",
            "--target",
            "missing-target.json",
        ],
    ] {
        let mut c = f.command();
        c.args(["--logical-cap", "4"]).args(flags);
        let o = f.invoke(&mut c);
        assert!(!o.status.success());
        assert!(o.stdout.is_empty());
        let e = String::from_utf8_lossy(&o.stderr);
        assert!(e.contains("--logical-cap"), "{e}");
        assert!(!e.contains("No such file"), "{e}");
    }
}
