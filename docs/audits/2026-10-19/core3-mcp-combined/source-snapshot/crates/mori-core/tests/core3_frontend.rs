//! Owner-authored `/3` frontend regression source. UNRUN: expectations are
//! hand-derived from the `/3` contract, not from a subject evaluator run.
//! No invented hash or golden bytes are asserted.
use mori_core::stage::{self, StageEvent, StageOutcome};
use mori_core::{Balance, Liability, Scenario, UInt, Value};
use std::collections::BTreeMap;

const NON_AWAITING: &str = r#"language "moriarty/3"; agreement P {
asset A = { network: "local", issuer: "i", reference: "A", scale: 0 };
duty "loan" = { asset: A, debtor: "alice", creditor: "bob" };
action pay() { discharge("loan", 3); }
}"#;
const AWAITING: &str = r#"language "moriarty/3"; agreement P {
asset A = { network: "local", issuer: "i", reference: "A", scale: 0 };
duty "loan" = { asset: A, debtor: "alice", creditor: "bob" };
action pay() { await ready(go: Bool) when go; discharge("loan", 3); }
}"#;
const AWAITING2: &str = r#"language "moriarty/2"; agreement P {
asset A = { network: "local", issuer: "i", reference: "A", scale: 0 };
action pay() { await ready(go: Bool) when go; discharge("loan", 3); }
}"#;

fn scenario(p: &stage::StagedProgram, amount: u128) -> Scenario {
    Scenario {
        state: BTreeMap::new(),
        inputs: BTreeMap::new(),
        balances: vec![],
        liabilities: vec![Liability {
            name: "loan".into(),
            asset: p.program().assets[0].identity.clone(),
            debtor: "alice".into(),
            creditor: "bob".into(),
            amount: UInt(amount),
        }],
    }
}

#[test]
fn core3_non_awaiting_discharge_only_completes_and_present_zero_stays_present() {
    let sp = stage::load_v3(NON_AWAITING).unwrap();
    let out = stage::start(&sp, "pay", scenario(&sp, 3), None).unwrap();
    let StageOutcome::Complete { segment, residual, .. } = out else { panic!("complete") };
    assert_eq!(segment.execution.post.liabilities.len(), 1);
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(0));
    assert!(residual.is_empty());
    let out = stage::start(&sp, "pay", scenario(&sp, 5), None).unwrap();
    let StageOutcome::Complete { segment, .. } = out else { panic!("complete") };
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(2));
}

#[test]
fn core3_awaiting_discharge_only_waits_resumes_and_false_guard_is_unchanged() {
    let sp = stage::load_v3(AWAITING).unwrap();
    let out = stage::start(&sp, "pay", scenario(&sp, 5), None).unwrap();
    let StageOutcome::Waiting { continuation, .. } = out else { panic!("waiting") };
    let event = |go: bool| StageEvent {
        format: stage::STAGE_EVENT_FORMAT.into(),
        action: "pay".into(),
        label: "ready".into(),
        values: vec![stage::NamedValue { name: "go".into(), value: Value::Bool(go) }],
    };
    let out = stage::resume(&sp, continuation.clone(), event(false), Some("pay"), None).unwrap();
    let StageOutcome::Waiting { advanced, continuation: again, .. } = out else { panic!("waiting") };
    assert!(!advanced);
    assert_eq!(again, continuation);
    let out = stage::resume(&sp, continuation, event(true), Some("pay"), None).unwrap();
    let StageOutcome::Complete { segment, .. } = out else { panic!("complete") };
    assert_eq!(segment.execution.post.liabilities[0].amount, UInt(2));
    // The resumed Await Step snapshot is the pre-guard boundary.
    assert_eq!(segment.execution.steps[0].snapshot.liabilities[0].amount, UInt(5));
}

#[test]
fn core2_awaiting_discharge_only_is_refused_and_core3_identity_mismatch_is_refused() {
    // `/2` has no source duty declaration, so R7 refuses the same boundary.
    assert!(stage::load(AWAITING2).is_err() || {
        let sp = stage::load(AWAITING2).unwrap();
        stage::start(&sp, "pay", scenario(&sp, 5), None).is_err()
    });
    let sp = stage::load_v3(NON_AWAITING).unwrap();
    let mut bad = scenario(&sp, 5);
    bad.liabilities[0].debtor = "mallory".into();
    assert!(stage::start(&sp, "pay", bad, None).is_err());
    let mut bad = scenario(&sp, 5);
    bad.liabilities[0].asset.scale = 1;
    assert!(stage::start(&sp, "pay", bad, None).is_err());
}

#[test]
fn core3_duty_conflicts_duplicates_and_legacy_wires_refuse() {
    let conflict = "language \"moriarty/3\"; agreement P { asset A = { network: \"local\", issuer: \"i\", reference: \"A\", scale: 0 }; duty \"loan\" = { asset: A, debtor: \"alice\", creditor: \"bob\" }; action pay() { liability(\"loan\", A, \"alice\", \"carol\", 1); } }";
    assert_eq!(stage::load_v3(conflict).unwrap_err().code, "mori::core::sequence_duty_declaration");
    let duplicate = "language \"moriarty/3\"; agreement P { asset A = { network: \"local\", issuer: \"i\", reference: \"A\", scale: 0 }; duty \"loan\" = { asset: A, debtor: \"alice\", creditor: \"bob\" }; duty \"loan\" = { asset: A, debtor: \"alice\", creditor: \"bob\" }; action pay() { discharge(\"loan\", 1); } }";
    assert_eq!(stage::load_v3(duplicate).unwrap_err().code, "mori::core::sequence_duty_declaration");
    // Legacy routing and profiles refuse `/3`; extended routing names it.
    assert!(mori_core::language(NON_AWAITING).is_err());
    assert_eq!(mori_core::language_v3(NON_AWAITING).unwrap(), "moriarty/3");
    assert!(mori_core::parse(NON_AWAITING).is_err());
    assert!(stage::load(NON_AWAITING).is_err());
    // `/1` serialization has no duties key; frozen v1 decoding refuses one.
    let p1 = mori_core::parse("language \"moriarty/1\"; agreement P { action a() { emit \"x\"(1); } }").unwrap();
    let value = serde_json::to_value(&p1).unwrap();
    assert!(value.get("duties").is_none());
    let mut forged = value.clone();
    forged["duties"] = serde_json::json!([]);
    // An empty array is skipped on serialize but the shared decode is via v1,
    // whose Program has no duties field and denies unknown fields.
    assert!(serde_json::from_value::<mori_core::Program>(forged).is_err());
    let _ = Balance { account: "a".into(), asset: p1.assets.first().map(|a| a.identity.clone()).unwrap_or(mori_core::AssetId { network: "n".into(), issuer: "i".into(), reference: "r".into(), scale: 0 }), amount: UInt(0) };
}
