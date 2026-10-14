//! Inspect's public CLI only: no proofs, SDK execution or private parameters.
use mori_backend::{Artifact, Target, authorization::SignedPolicy};
use serde_json::{Value, json};
use std::{
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT: AtomicUsize = AtomicUsize::new(0);
const SOURCE: &str = include_str!("../../../examples/core/enforcement-report/source.mori");
const COUNTER: &str = "language \"moriarty/1\"; agreement IndependentCounter { state n: UInt = 7; state ready: Bool = true; action increment(delta: UInt) { requires ready; next n = n + delta; ensures n >= delta; emit \"counter\"(n); } }";
struct Files(PathBuf);
impl Files {
    fn new(source: &str, target: &Target) -> Self {
        let p = std::env::temp_dir().join(format!(
            "mori-inspect-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        let f = Self(p);
        f.write("source.mori", source);
        f.write("target.json", serde_json::to_vec_pretty(target).unwrap());
        f
    }
    fn write(&self, name: &str, data: impl AsRef<[u8]>) {
        std::fs::write(self.0.join(name), data).unwrap();
    }
    fn inspect(&self, target: bool) -> Output {
        let mut c = Command::new(env!("CARGO_BIN_EXE_mori"));
        c.args(["--color", "never", "inspect"])
            .arg(self.0.join("source.mori"));
        if target {
            c.arg("--target").arg(self.0.join("target.json"));
        }
        c.output().unwrap()
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn report(o: &Output) -> Value {
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert!(o.stderr.is_empty());
    assert!(o.stdout.is_ascii());
    serde_json::from_slice(&o.stdout).unwrap()
}
fn refuse(o: &Output) {
    assert!(!o.status.success());
    assert!(
        o.stdout.is_empty(),
        "a refusal must not publish report success"
    );
}
fn compile(source: &str, target: &Target) -> Artifact {
    let p = mori_core::parse(source).unwrap();
    if target.signed_policy.is_some() {
        mori_backend::compile_signed(&p, target).unwrap()
    } else {
        mori_backend::compile(&p, target).unwrap()
    }
}
#[test]
fn basic_inspection_preserves_source_metadata_without_target_qualification() {
    let f = Files::new(SOURCE, &Target::ledger9("local", "unqualified"));
    let v = report(&f.inspect(false));
    assert_eq!(v["agreement"], "DeliveryAccounting");
    assert_eq!(v["qualification"], "source-only-unqualified");
    assert_eq!(v["enforcement_report"], Value::Null);
    assert!(
        v["open_premises"]
            .as_array()
            .unwrap()
            .iter()
            .any(|s| s.as_str().unwrap().contains("target"))
    );
}
#[test]
fn contrasting_unseen_sources_expose_every_public_field_statement_and_duty() {
    let target = Target::ledger9("local", "inspection-unseen");
    for source in [SOURCE, COUNTER] {
        let f = Files::new(source, &target);
        let first = f.inspect(true);
        let v = report(&first);
        assert_eq!(first.stdout, f.inspect(true).stdout);
        let a = compile(source, &target);
        let e = &v["enforcement_report"];
        assert_eq!(e["identity"]["program_hash"], a.program_hash);
        assert_eq!(e["target"], serde_json::to_value(&target).unwrap());
        assert_eq!(e["resources"], serde_json::to_value(&a.resources).unwrap());
        assert_eq!(e["gates"], serde_json::to_value(&a.gates).unwrap());
        let p = mori_core::parse(source).unwrap();
        assert_eq!(
            v["expressions"],
            serde_json::to_value(&p.expressions).unwrap()
        );
        let actions = e["actions"].as_array().unwrap();
        assert_eq!(actions.len(), a.circuits.len());
        for ((r, c), s) in actions.iter().zip(&a.circuits).zip(&p.actions) {
            assert_eq!(r["action"], c.action);
            assert_eq!(r["context_hash"], c.context_hash);
            assert_eq!(
                r["effect_frame"],
                serde_json::to_value(&c.effect_frame).unwrap()
            );
            assert_eq!(
                r["balance_frame"],
                serde_json::to_value(&c.balance_frame).unwrap()
            );
            assert_eq!(
                r["liability_frame"],
                serde_json::to_value(&c.liability_frame).unwrap()
            );
            let fields = r["fields"].as_array().unwrap();
            assert_eq!(fields.len(), c.public_inputs.len());
            for (position, (field, binding)) in fields.iter().zip(&c.public_inputs).enumerate() {
                assert_eq!(field["position"], position);
                assert_eq!(field["binding"], serde_json::to_value(binding).unwrap());
                assert_eq!(field["constraint"]["status"], "encoded-not-verified");
                assert_eq!(field["reference"]["status"], "available-not-executed");
                assert!(
                    !field["constraint"]["predicates"]
                        .as_array()
                        .unwrap()
                        .is_empty()
                );
                assert!(field["source"].is_string());
                assert!(field["state"]["status"].is_string());
                assert!(field["unknown"].is_array());
            }
            let expression_constraints = r["expression_constraints"].as_array().unwrap();
            for row in expression_constraints {
                let id = row["expression"].as_u64().unwrap() as usize;
                assert_eq!(
                    row["source"],
                    serde_json::to_value(&p.expressions[id]).unwrap()
                );
                assert_eq!(row["constraint"]["status"], "encoded-not-verified");
            }
            if c.action == "deliver" {
                assert!(
                    expression_constraints
                        .iter()
                        .any(|row| row["source"]["op"] == "Mul")
                );
            }
            let steps = r["statements"].as_array().unwrap();
            assert_eq!(steps.len(), s.statements.len());
            for (i, (step, statement)) in steps.iter().zip(&s.statements).enumerate() {
                assert_eq!(step["step"], i);
                assert_eq!(step["source"], serde_json::to_value(statement).unwrap());
                assert!(
                    !step["constraint"]["predicates"]
                        .as_array()
                        .unwrap()
                        .is_empty()
                );
            }
            // The program-wide duty registry must survive the unrelated close action.
            if source == SOURCE && c.action == "close" {
                assert_eq!(r["liability_frame"][0]["name"], "remaining-delivery");
                assert!(
                    r["fields"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|f| f["binding"]["role"] == "post_liability_amount")
                );
            }
        }
    }
}
#[test]
fn signed_report_keeps_all_authorization_fields_and_only_public_private_descriptors() {
    let mut t = Target::ledger9("local", "signed-inspection");
    t.signed_policy = Some(SignedPolicy {
        owner: "sender".into(),
        beneficiary: "recipient".into(),
        program_address: "local/DeliveryAccounting".into(),
    });
    let f = Files::new(SOURCE, &t);
    let v = report(&f.inspect(true));
    let a = compile(SOURCE, &t);
    assert_eq!(
        v["enforcement_report"]["identity"]["authorization_mode"],
        t.authorization_mode()
    );
    for (r, c) in v["enforcement_report"]["actions"]
        .as_array()
        .unwrap()
        .iter()
        .zip(&a.circuits)
    {
        let auth = r["authorization_fields"].as_array().unwrap();
        assert_eq!(auth.len(), 19); // 11 + four caps for each of two assets.
        for (field, binding) in auth.iter().zip(&c.authorization_bindings) {
            assert_eq!(field["binding"], serde_json::to_value(binding).unwrap());
            assert_eq!(field["position"], binding.position);
            assert_eq!(field["constraint"]["status"], "encoded-not-verified");
        }
        for n in [
            "public_key.x",
            "nonce",
            "at",
            "predecessor.lo",
            "limits.0.gross_cap",
            "limits.1.liability_cap",
        ] {
            assert!(auth.iter().any(|f| f["binding"]["name"] == n));
        }
        assert_eq!(
            r["private_input_descriptors"],
            serde_json::to_value(&c.private_inputs).unwrap()
        );
        assert_eq!(
            r["authorization_constraints"]["status"],
            "encoded-not-verified"
        );
    }
    assert_eq!(
        v["enforcement_report"]["target"]["signed_policy"]["owner"],
        "sender"
    );
    assert!(!String::from_utf8_lossy(&f.inspect(true).stdout).contains("signature_hex"));
    let u = report(&Files::new(SOURCE, &Target::ledger9("local", "unsigned")).inspect(true));
    for r in u["enforcement_report"]["actions"].as_array().unwrap() {
        assert_eq!(r["authorization_fields"], json!([]));
        assert_eq!(r["authorization_constraints"]["status"], "absent-unsigned");
    }
}
#[test]
fn unknown_authenticated_and_native_predicates_never_become_enforced() {
    let f = Files::new(SOURCE, &Target::ledger9("local", "unproven"));
    let v = report(&f.inspect(true));
    let e = &v["enforcement_report"];
    assert_eq!(v["qualification"], "local-compiled-unqualified");
    assert_eq!(e["resources"]["cost_measured"], false);
    assert_eq!(e["resources"]["circuit_k"], Value::Null);
    let unknown = e["unknown"].as_array().unwrap();
    for id in [
        "ledger-authentication",
        "history-replay-revocation",
        "clock-and-external-inputs",
        "native-proof-soundness",
        "compiler-correspondence",
        "fraud-proof-and-replay-cost",
        "owner-key-authority",
        "sponsor-authority-and-settlement",
        "live-deployment-and-finality",
        "host-commitment-reconstruction",
    ] {
        let row = unknown
            .iter()
            .find(|r| r["id"] == id)
            .unwrap_or_else(|| panic!("missing {id}"));
        assert_eq!(row["status"], "unproven");
    }
    for g in e["gates"].as_array().unwrap() {
        assert_eq!(g["status"], "unperformed");
    }
    // Initial state is a source declaration, never a claimed authenticated head.
    let pre = e["actions"][0]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["binding"]["role"] == "pre_state")
        .unwrap();
    assert_eq!(pre["state"]["status"], "unauthenticated");
    assert!(
        !pre["constraint"]["predicates"]
            .as_array()
            .unwrap()
            .iter()
            .any(|p| p.as_str().unwrap().contains("initial value"))
    );
}
#[test]
fn target_missing_mismatched_or_malformed_refuses_without_partial_report() {
    let base = Target::ledger9("local", "refusal");
    for edit in 0..7 {
        let mut t = base.clone();
        match edit {
            0 => t.profile = "unknown".into(),
            1 => t.ledger_revision = "stale".into(),
            2 => t.zkir_version = "stale".into(),
            3 => t.network = "foreign".into(),
            4 => t.anchor.clear(),
            5 => t.max_instructions = 1,
            _ => t.max_artifact_bytes = 1,
        }
        refuse(&Files::new(SOURCE, &t).inspect(true));
    }
    let f = Files::new(SOURCE, &base);
    std::fs::remove_file(f.0.join("target.json")).unwrap();
    refuse(&f.inspect(true));
    let raw = serde_json::to_string(&base).unwrap();
    for bad in [
        raw.replacen('{', "{\"unknown\":1,", 1),
        raw.replacen('{', "{\"signed_policy\":null,", 1),
        raw.replacen(
            '{',
            "{\"network\":\"local\",\"\\u006eetwork\":\"local\",",
            1,
        ),
        "{}".into(),
        "{".into(),
        "x".repeat(1_048_577),
    ] {
        f.write("target.json", bad);
        refuse(&f.inspect(true));
    }
    f.write("target.json", raw);
    for bad in [
        "not a program".into(),
        SOURCE.replace("quantity * price", "quantity / price"),
        "x".repeat(1_048_577),
    ] {
        f.write("source.mori", bad);
        refuse(&f.inspect(true));
    }
}

#[test]
fn public_report_escapes_hostile_labels_and_refuses_oversized_output_atomically() {
    let t = Target::ledger9("local", "public-report");
    let hostile = SOURCE.replace("\"treasury\"", "\"treasury\\u202eé\"");
    let f = Files::new(&hostile, &t);
    let out = f.inspect(true);
    let v = report(&out);
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(!text.contains('\u{202e}'));
    assert!(text.contains("\\u202e"));
    assert!(
        v["enforcement_report"]["actions"][1]["balance_frame"]
            .as_array()
            .unwrap()
            .iter()
            .any(|b| b["account"] == "treasury\u{202e}é")
    );
    let large = format!(
        "language \"moriarty/1\"; agreement Wide {{ {} {} }}",
        (0..32)
            .map(|i| format!("state s{i}: UInt = 0;"))
            .collect::<String>(),
        (0..128)
            .map(|i| format!("action a{i}() {{ next s0 = s0 + 1; }}"))
            .collect::<String>()
    );
    // Compiler admission and the report's own bounded public exporter differ.
    compile(&large, &t);
    let out = Files::new(&large, &t).inspect(true);
    refuse(&out);
    assert!(String::from_utf8_lossy(&out.stderr).contains("JSON"));
}

#[test]
fn inspection_describes_literal_duty_reads_and_keeps_authentication_open() {
    let source = r#"language "moriarty/1"; agreement Reads {
        asset A = { network: "local", issuer: "i", reference: "A", scale: 0 };
        action declare() { liability("d", A, "debtor", "creditor", 0); }
        action read() { requires not liability_present("d"); requires liability_amount("d") == 0; }
    }"#;
    let f = Files::new(source, &Target::ledger9("local", "read-inspection"));
    let first = f.inspect(true);
    let v = report(&first);
    assert_eq!(first.stdout, f.inspect(true).stdout);
    let action = &v["enforcement_report"]["actions"][1];
    let rows = action["expression_constraints"].as_array().unwrap();
    for kind in ["LiabilityPresent", "LiabilityAmount"] {
        let row = rows
            .iter()
            .find(|row| row["source"]["kind"] == kind)
            .unwrap();
        assert_eq!(row["source"]["name"], "d");
        assert!(row["constraint"].to_string().contains("current duty"));
        assert!(
            row["constraint"]
                .to_string()
                .contains("source-fixed identity")
        );
    }
    assert!(v.to_string().contains("ledger-authentication"));
}
