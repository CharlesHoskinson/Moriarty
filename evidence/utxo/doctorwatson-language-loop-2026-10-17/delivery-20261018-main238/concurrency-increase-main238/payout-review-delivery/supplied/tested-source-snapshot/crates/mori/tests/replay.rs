//! Actual CLI consumers; local public reconstruction only, no proof APIs.
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
        let p = std::env::temp_dir().join(format!(
            "mori-replay-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn path(&self, n: &str) -> PathBuf {
        self.0.join(n)
    }
    fn write(&self, n: &str, bytes: impl AsRef<[u8]>) {
        std::fs::write(self.path(n), bytes).unwrap();
    }
    fn payment() -> Self {
        let f = Self::new();
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
        f
    }
    fn command(&self, verb: &str, action: &str, signed: bool) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_mori"));
        cmd.args(["--color", "never", verb])
            .arg(self.path("source.mori"))
            .args(["--action", action, "--scenario"])
            .arg(self.path("scenario.json"));
        if verb == "replay" {
            cmd.arg("--record").arg(self.path("record.json"));
        }
        if signed {
            cmd.args(["--profile", "moriarty-jubjub-owner-intent/1", "--authority"])
                .arg(self.path("authority.json"))
                .args(["--at", "15", "--signature"])
                .arg(self.path("signed.json"));
        }
        cmd
    }
    fn trace(&self, action: &str, signed: bool) -> Output {
        self.command("simulate", action, signed)
            .args(["--trace", "--target"])
            .arg(self.path("target.json"))
            .output()
            .unwrap()
    }
    fn replay(&self, action: &str, signed: bool) -> Output {
        self.command("replay", action, signed)
            .arg("--target")
            .arg(self.path("target.json"))
            .output()
            .unwrap()
    }
}
fn decoded(o: Output) -> Value {
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    serde_json::from_slice(&o.stdout).unwrap()
}
fn digest(v: &Value) -> String {
    mori_core::intent::hash_bytes(&mori_core::intent::canonical(v).unwrap())
}
fn rehash(v: &mut Value) {
    v["body"]["claim"]["execution_hash"] = digest(&v["body"]["execution"]).into();
    v["body"]["trace_hash"] = digest(&json!([
        "moriarty-reference-trace/1",
        v["body"]["execution"]
    ]))
    .into();
    v["record_hash"] = digest(&json!(["moriarty-reference-replay/1", v["body"]])).into();
}
fn rejection(o: Output) {
    assert!(!o.status.success());
    if !o.stdout.is_empty() {
        let v: Value = serde_json::from_slice(&o.stdout).unwrap();
        assert_eq!(v["status"], "rejected");
        assert!(
            v.get("execution").is_none() && v.get("effects").is_none() && v.get("body").is_none()
        );
    }
}
#[test]
fn signed_trace_stdout_is_entire_record_and_real_replay_checks_it() {
    let f = Files::payment();
    let trace = f.trace("settle", true);
    assert!(
        trace.status.success(),
        "{}",
        String::from_utf8_lossy(&trace.stderr)
    );
    let record =
        mori_backend::replay::ReplayRecord::from_json(std::str::from_utf8(&trace.stdout).unwrap())
            .unwrap();
    assert_eq!(record.body.claim.circuit_k, 13);
    assert_eq!(record.body.source_step_count, 6);
    f.write("record.json", &trace.stdout);
    let result = decoded(f.replay("settle", true));
    assert_eq!(
        result,
        json!({"status":"accepted","qualification":"local-reference-replay","record_hash":record.record_hash})
    );
    f.write(
        "signed.json",
        include_str!(
            "../../../deliverables/native-owner-intent-2026-10-11/alternative-attachment.json"
        ),
    );
    assert_eq!(
        decoded(f.trace("settle", true)),
        serde_json::from_slice::<Value>(&trace.stdout).unwrap()
    );
    decoded(f.replay("settle", true));
}
#[test]
fn unsigned_contrast_and_ordinary_simulate_use_real_general_pipeline() {
    let o: Value = serde_json::from_str(include_str!(
        "../../mori-core/tests/fixtures/bound-replay-oracle.json"
    ))
    .unwrap();
    let c = &o["contrasting"];
    let f = Files::new();
    f.write("source.mori", c["source"].as_str().unwrap());
    f.write("scenario.json", c["scenario"].to_string());
    f.write("target.json", c["unsigned_target"].to_string());
    let record = decoded(f.trace("collect", false));
    assert_eq!(record["body"]["execution"], c["expected_execution"]);
    f.write("record.json", record.to_string());
    decoded(f.replay("collect", false));
    let ordinary = decoded(f.command("simulate", "collect", false).output().unwrap());
    assert_eq!(
        ordinary,
        json!({"status":"accepted","qualification":"local-reference-execution","execution":c["expected_execution"]})
    );
    for (path, value) in [
        ("/body/execution/post/state/frozen/value", json!(false)),
        ("/body/execution/post/balances/2/amount", json!("8")),
    ] {
        let mut bad = record.clone();
        *bad.pointer_mut(path).unwrap() = value;
        rehash(&mut bad);
        f.write("record.json", bad.to_string());
        let out = f.replay("collect", false);
        assert!(!out.status.success());
        let v: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(v["error"]["field"], path);
        assert_eq!(v["error"]["step"], Value::Null);
    }
}
#[test]
fn rehashed_earlier_field_survives_later_hash_public_value_and_step_changes() {
    let f = Files::payment();
    let mut r = decoded(f.trace("settle", true));
    r["body"]["execution"]["steps"][1]["effects"][0]["amount"] = "69".into();
    r["body"]["execution"]["steps"][5]["index"] = 99.into();
    r["body"]["claim"]["public_values"][0] = "00".repeat(32).into();
    rehash(&mut r);
    f.write("record.json", r.to_string());
    let out = f.replay("settle", true);
    assert!(!out.status.success());
    let result: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(
        result,
        json!({"status":"rejected","qualification":"local-reference-replay","error":{"kind":"divergence","step":1,"field":"/body/execution/steps/1/effects/0/amount"}})
    );
    let mut r = decoded(f.trace("settle", true));
    r["body"]["execution"]["post"]["liabilities"]
        .as_array_mut()
        .unwrap()
        .pop();
    rehash(&mut r);
    f.write("record.json", r.to_string());
    let out = f.replay("settle", true);
    assert!(!out.status.success());
    let result: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(result["error"]["field"], "/body/execution/post/liabilities");
    assert_eq!(result["error"]["step"], Value::Null);
}
#[test]
fn cli_groups_bounds_and_trusted_reconstruction_fail_without_proposed_effects() {
    let f = Files::payment();
    let record = decoded(f.trace("settle", true));
    f.write("record.json", record.to_string());
    rejection(f.trace("settle", false));
    rejection(f.replay("settle", false));
    for args in [
        vec!["--trace"],
        vec!["--target"],
        vec!["--profile", "moriarty-jubjub-owner-intent/1"],
    ] {
        let mut cmd = f.command("simulate", "settle", false);
        cmd.args(&args);
        if args == vec!["--target"] {
            cmd.arg(f.path("target.json"));
        }
        rejection(cmd.output().unwrap());
    }
    f.write("record.json", " ".repeat(1048577));
    rejection(f.replay("settle", true));
    f.write("record.json", record.to_string());
    let mut signed:Value=serde_json::from_str(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.signed.json")).unwrap();
    signed["signature_hex"] = "00".repeat(96).into();
    f.write("signed.json", signed.to_string());
    rejection(f.replay("settle", true));
    rejection(f.trace("settle", true));
    f.write("source.mori", "bad source");
    rejection(f.trace("settle", true));
    // Unsigned source refusals exercise a late postcondition after financial effects
    // and checked UInt overflow through the actual trace command.
    let o: Value = serde_json::from_str(include_str!(
        "../../mori-core/tests/fixtures/bound-replay-oracle.json"
    ))
    .unwrap();
    let c = &o["contrasting"];
    f.write("scenario.json", c["scenario"].to_string());
    f.write("target.json", c["unsigned_target"].to_string());
    for source in [
        c["source"]
            .as_str()
            .unwrap()
            .replace("ensures frozen and collected == 40;", "ensures false;"),
        c["source"].as_str().unwrap().replace(
            "amount - 3",
            "amount + 340282366920938463463374607431768211455",
        ),
    ] {
        f.write("source.mori", source);
        rejection(f.trace("collect", false));
    }
}
#[test]
fn cli_refuses_wrong_target_scenario_authority_and_logical_time() {
    let f = Files::payment();
    let record = decoded(f.trace("settle", true));
    f.write("record.json", record.to_string());
    for (name, path, value) in [
        ("target.json", "/anchor", json!("other")),
        (
            "target.json",
            "/signed_policy/program_address",
            json!("other"),
        ),
        ("authority.json", "/predecessor", json!("other")),
        ("scenario.json", "/balances/0/amount", json!("99")),
    ] {
        let original = std::fs::read_to_string(f.path(name)).unwrap();
        let mut changed: Value = serde_json::from_str(&original).unwrap();
        *changed.pointer_mut(path).unwrap() = value;
        f.write(name, changed.to_string());
        rejection(f.replay("settle", true));
        f.write(name, original);
    }
    // Build a separate complete caller group with at16; the record still binds at15.
    let mut cmd = f.command("replay", "settle", false);
    cmd.arg("--target")
        .arg(f.path("target.json"))
        .args(["--profile", "moriarty-jubjub-owner-intent/1", "--authority"])
        .arg(f.path("authority.json"))
        .args(["--at", "16", "--signature"])
        .arg(f.path("signed.json"));
    rejection(cmd.output().unwrap());
}
