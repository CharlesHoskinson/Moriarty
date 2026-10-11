//! Public reconstruction/reference replay only. No proof or parameter APIs.
use mori_backend::{
    Target,
    authorization::{ExpectedAuthority, SignedIntent},
    replay::{self, ReplayAuthorization as Auth, ReplayError, ReplayRecord},
};
use mori_core::{
    Execution, Scenario, UInt,
    intent::{canonical, hash_bytes},
};
use serde_json::{Value, json};
fn oracle() -> Value {
    serde_json::from_str(include_str!(
        "../../mori-core/tests/fixtures/bound-replay-oracle.json"
    ))
    .unwrap()
}
struct Payment {
    source: String,
    scenario: Scenario,
    target: Target,
    authority: ExpectedAuthority,
    signed: SignedIntent,
}
impl Payment {
    fn new() -> Self {
        Self{
  source:include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori").into(),
  scenario:serde_json::from_str(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json")).unwrap(),
  target:Target::from_json(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json")).unwrap(),
  authority:ExpectedAuthority::from_json(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.authority.json")).unwrap(),
  signed:SignedIntent::from_json(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.signed.json")).unwrap(),
 }
    }
    fn auth(&self) -> Auth<'_> {
        Auth::Signed {
            authority: &self.authority,
            at: UInt(15),
            signature: &self.signed,
        }
    }
    fn record(&self) -> ReplayRecord {
        replay::generate(
            &self.source,
            "settle",
            &self.scenario,
            &self.target,
            self.auth(),
        )
        .unwrap()
    }
    fn replay(&self, r: &ReplayRecord) -> Result<(), ReplayError> {
        replay::replay(
            &self.source,
            "settle",
            &self.scenario,
            &self.target,
            self.auth(),
            r,
        )
    }
}
fn digest(v: &impl serde::Serialize) -> String {
    hash_bytes(&canonical(v).unwrap())
}
fn rehash(r: &mut ReplayRecord) {
    r.body.claim.execution_hash = digest(&r.body.execution);
    r.body.trace_hash = digest(&("moriarty-reference-trace/1", &r.body.execution));
    r.record_hash = digest(&(replay::RECORD_FORMAT, &r.body));
}
fn edited(r: &ReplayRecord, edit: impl FnOnce(&mut Value)) -> ReplayRecord {
    let mut v = serde_json::to_value(r).unwrap();
    edit(&mut v);
    // Shape validation cannot enforce hash equality before semantic comparison.
    ReplayRecord::from_json(&v.to_string()).unwrap()
}
fn difference(c: &Payment, r: &ReplayRecord, step: Option<u16>, field: &str) {
    match c.replay(r).unwrap_err() {
        ReplayError::Divergence(d) => {
            assert_eq!(d.step, step);
            assert_eq!(d.field, field);
        }
        e => panic!("{e:?}"),
    }
}
#[test]
fn actual_pipeline_matches_published_entire_claim_execution_and_hash_frames() {
    let c = Payment::new();
    let r = c.record();
    c.replay(&r).unwrap();
    let mut published: Value = serde_json::from_str(include_str!(
        "../../../deliverables/native-owner-intent-2026-10-11/bundle.json"
    ))
    .unwrap();
    published.as_object_mut().unwrap().remove("proof_hex");
    assert_eq!(
        canonical(&r.body.claim).unwrap(),
        canonical(&published).unwrap()
    );
    assert_eq!(
        serde_json::to_value(&r.body.execution).unwrap(),
        oracle()["known_payment"]["expected_execution"]
    );
    assert_eq!(r.body.claim.circuit_k, 13);
    assert_eq!(r.body.source_step_count, 6);
    assert_eq!(
        r.body.zkir_hash,
        "b009f47c019f756f8d73c1f643433058c79715301249d1d396f42581b7a51eff"
    );
    assert_eq!(
        r.body.authority_hash.as_ref().unwrap(),
        &digest(&c.authority)
    );
    assert_eq!(
        r.body.input_hash,
        digest(&json!([
            "moriarty-reference-input/1",
            r.body.claim.source_hash,
            r.body.claim.program_hash,
            "settle",
            r.body.claim.scenario_hash,
            r.body.claim.target_hash,
            r.body.authority_hash,
            "15"
        ]))
    );
    assert_eq!(
        r.body.trace_hash,
        digest(&("moriarty-reference-trace/1", &r.body.execution))
    );
    assert_eq!(r.record_hash, digest(&(replay::RECORD_FORMAT, &r.body)));
    assert_eq!(canonical(&c.record()).unwrap(), canonical(&r).unwrap());
    let text = mori_core::json::export(&r).unwrap();
    let decoded = ReplayRecord::from_json(&text).unwrap();
    c.replay(&decoded).unwrap();
    for field in [
        "signature_hex",
        "private_transcript",
        "private_inputs",
        "proof_hex",
        "native_verified",
        "signature_valid",
    ] {
        assert!(!text.contains(field), "{field}");
    }
    assert!(!text.contains(&c.signed.signature_hex));
}
#[test]
fn independently_authored_fee_first_program_uses_same_general_pipeline() {
    let o = oracle();
    let c = &o["contrasting"];
    let source = c["source"].as_str().unwrap();
    let scenario: Scenario = serde_json::from_value(c["scenario"].clone()).unwrap();
    let target: Target = serde_json::from_value(c["unsigned_target"].clone()).unwrap();
    let r = replay::generate(source, "collect", &scenario, &target, Auth::Unsigned).unwrap();
    assert_eq!(
        serde_json::to_value(&r.body.execution).unwrap(),
        c["expected_execution"]
    );
    assert_eq!(r.body.claim.source_hash, c["source_sha256"]);
    assert_eq!(r.body.claim.program_hash, c["program_sha256"]);
    assert_eq!(r.body.claim.scenario_hash, c["scenario_sha256"]);
    assert_eq!(r.body.claim.target_hash, c["target_sha256"]);
    assert_eq!(r.body.claim.execution_hash, c["execution_sha256"]);
    assert_eq!(r.body.source_step_count, 8);
    assert!(
        r.body.program_address.is_none()
            && r.body.authority_hash.is_none()
            && r.body.claim.authorization.is_none()
    );
    replay::replay(source, "collect", &scenario, &target, Auth::Unsigned, &r).unwrap();
}
#[test]
fn rehashed_early_and_late_mutations_report_first_semantic_field() {
    let c = Payment::new();
    let r = c.record();
    let paths = [
        ("/body/execution/steps/1/index", json!(9)),
        ("/body/execution/steps/1/statement/amount", json!(0)),
        ("/body/execution/steps/1/value/value", json!("69")),
        (
            "/body/execution/steps/1/snapshot/balances/0/amount",
            json!("31"),
        ),
        ("/body/execution/steps/1/effects/0/amount", json!("69")),
    ];
    for (path, value) in paths {
        let mut bad = edited(&r, |v| {
            *v.pointer_mut(path).unwrap() = value;
            v["body"]["execution"]["steps"][5]["index"] = 99.into();
            v["body"]["claim"]["public_values"][0] = "00".repeat(32).into();
        });
        rehash(&mut bad);
        difference(&c, &bad, Some(1), path);
    }
    // A financially self-consistent alternative transfer updates all later snapshots,
    // aggregate effects and post. Hashing those false bytes still cannot authorize it.
    let mut bad = edited(&r, |v| {
        let e = &mut v["body"]["execution"];
        e["steps"][1]["value"]["value"] = "69".into();
        e["steps"][1]["effects"][0]["amount"] = "69".into();
        e["effects"][0]["amount"] = "69".into();
        for i in 1..6 {
            let balances = e["steps"][i]["snapshot"]["balances"]
                .as_array_mut()
                .unwrap();
            balances[0]["amount"] = if i == 1 { "31" } else { "29" }.into();
            balances[1]["amount"] = "79".into();
        }
        e["post"]["balances"][0]["amount"] = "29".into();
        e["post"]["balances"][1]["amount"] = "79".into();
    });
    rehash(&mut bad);
    difference(&c, &bad, Some(1), "/body/execution/steps/1/value/value");
}
#[test]
fn ordered_steps_effects_finalpost_and_untouched_presentzero_refuse() {
    let c = Payment::new();
    let r = c.record();
    let cases: [(fn(&mut Value), Option<u16>, &str); 8] = [
        (
            |v| {
                v["body"]["execution"]["steps"]
                    .as_array_mut()
                    .unwrap()
                    .pop();
            },
            Some(5),
            "/body/execution/steps/5",
        ),
        (
            |v| {
                let s = v["body"]["execution"]["steps"][5].clone();
                v["body"]["execution"]["steps"]
                    .as_array_mut()
                    .unwrap()
                    .push(s);
            },
            Some(6),
            "/body/execution/steps/6",
        ),
        (
            |v| {
                v["body"]["execution"]["steps"]
                    .as_array_mut()
                    .unwrap()
                    .swap(1, 2)
            },
            Some(1),
            "/body/execution/steps/1/index",
        ),
        (
            |v| {
                v["body"]["execution"]["effects"]
                    .as_array_mut()
                    .unwrap()
                    .swap(0, 1)
            },
            None,
            "/body/execution/effects/0/amount",
        ),
        (
            |v| {
                v["body"]["execution"]["effects"]
                    .as_array_mut()
                    .unwrap()
                    .pop();
            },
            None,
            "/body/execution/effects",
        ),
        (
            |v| v["body"]["execution"]["post"]["state"]["paid"]["value"] = "69".into(),
            None,
            "/body/execution/post/state/paid/value",
        ),
        (
            |v| v["body"]["execution"]["post"]["liabilities"][1]["amount"] = "6".into(),
            None,
            "/body/execution/post/liabilities/1/amount",
        ),
        (
            |v| {
                v["body"]["execution"]["post"]["liabilities"]
                    .as_array_mut()
                    .unwrap()
                    .pop();
            },
            None,
            "/body/execution/post/liabilities",
        ),
    ];
    for (edit, step, path) in cases {
        let mut bad = edited(&r, edit);
        rehash(&mut bad);
        difference(&c, &bad, step, path);
    }
    let bad = edited(&r, |v| {
        v["body"]["claim"]["public_values"][0] = "00".repeat(32).into()
    });
    difference(&c, &bad, None, "/body/claim/public_values");
    for key in [
        "source_step_count",
        "trace_hash",
        "record_hash",
        "zkir_hash",
        "network",
        "anchor",
    ] {
        let bad = edited(&r, |v| match key {
            "record_hash" => v[key] = "00".repeat(32).into(),
            "source_step_count" => v["body"][key] = 5.into(),
            "network" | "anchor" => v["body"][key] = "other".into(),
            _ => v["body"][key] = "00".repeat(32).into(),
        });
        let path = if key == "record_hash" {
            "/record_hash".to_owned()
        } else {
            format!("/body/{key}")
        };
        difference(&c, &bad, None, &path);
    }
}
#[test]
fn separately_trusted_inputs_attachment_mode_and_time_cannot_be_substituted() {
    let c = Payment::new();
    let r = c.record();
    let mut other = Payment::new();
    other.source.push('\n');
    assert!(other.replay(&r).is_err());
    let mut other = Payment::new();
    other.scenario.balances.reverse();
    assert!(other.replay(&r).is_err());
    let mut other = Payment::new();
    other.target.anchor = "other".into();
    assert!(other.replay(&r).is_err());
    let mut other = Payment::new();
    other.authority.nonce = UInt(2);
    assert!(other.replay(&r).is_err());
    let mut other = Payment::new();
    other.signed.signature_hex.replace_range(130..132, "00");
    assert!(other.replay(&r).is_err());
    assert!(replay::replay(&c.source, "observe", &c.scenario, &c.target, c.auth(), &r).is_err());
    assert!(
        replay::replay(
            &c.source,
            "settle",
            &c.scenario,
            &c.target,
            Auth::Signed {
                authority: &c.authority,
                at: UInt(16),
                signature: &c.signed
            },
            &r
        )
        .is_err()
    );
    assert!(
        replay::generate(
            &c.source,
            "settle",
            &c.scenario,
            &c.target,
            Auth::Signed {
                authority: &c.authority,
                at: UInt(21),
                signature: &c.signed
            }
        )
        .is_err()
    );
    assert!(replay::generate(&c.source, "settle", &c.scenario, &c.target, Auth::Unsigned).is_err());
    let alternate: Value = serde_json::from_str(include_str!(
        "../../../deliverables/native-owner-intent-2026-10-11/alternative-attachment.json"
    ))
    .unwrap();
    let mut other = Payment::new();
    other.signed.signature_hex = alternate["signature_hex"].as_str().unwrap().into();
    assert_eq!(canonical(&other.record()).unwrap(), canonical(&r).unwrap());
    other.replay(&r).unwrap();
}
#[test]
fn strict_nullable_presence_encodings_duplicates_and_json_bounds() {
    let c = Payment::new();
    let r = c.record();
    let original = serde_json::to_value(&r).unwrap();
    for path in [
        "/body/authority_hash",
        "/body/program_address",
        "/body/claim/authorization",
        "/body/execution/steps/0/value",
    ] {
        let mut v = original.clone();
        let (parent, key) = path.rsplit_once('/').unwrap();
        v.pointer_mut(parent)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .remove(key);
        assert!(ReplayRecord::from_json(&v.to_string()).is_err(), "{path}");
    }
    for path in [
        "/body/network",
        "/body/execution/pre/state",
        "/body/claim/public_values",
    ] {
        let mut v = original.clone();
        *v.pointer_mut(path).unwrap() = Value::Null;
        assert!(ReplayRecord::from_json(&v.to_string()).is_err());
    }
    let text = original.to_string();
    let duplicate = text.replacen("\"body\":", "\"body\":{},\"\\u0062ody\":", 1);
    assert!(ReplayRecord::from_json(&duplicate).is_err());
    for text in [
        format!("{text} {{}}"),
        " ".repeat(1048577),
        "[".repeat(34) + &"]".repeat(34),
        format!("[{}]", vec!["0"; 65536].join(",")),
    ] {
        assert!(ReplayRecord::from_json(&text).is_err());
    }
    for value in [json!("A".repeat(64)), json!("00"), json!(null)] {
        let mut v = original.clone();
        v["record_hash"] = value;
        assert!(ReplayRecord::from_json(&v.to_string()).is_err());
    }
    let mut v = original.clone();
    v["body"]["extra"] = true.into();
    assert!(ReplayRecord::from_json(&v.to_string()).is_err());
    let mut v = original;
    v["body"]["execution"]["post"]["liabilities"][0]["amount"] = 70.into();
    assert!(ReplayRecord::from_json(&v.to_string()).is_err());
}
#[test]
fn direct_typed_candidates_and_trusted_inputs_have_public_bounds() {
    let c = Payment::new();
    let mut r = c.record();
    r.body.program_name = "x".repeat(1048577);
    assert!(matches!(c.replay(&r), Err(ReplayError::Input(_))));
    let mut r = c.record();
    r.body.program_name = "🦀".repeat(90000);
    assert!(matches!(c.replay(&r), Err(ReplayError::Input(_))));
    let mut r = c.record();
    r.body
        .execution
        .post
        .state
        .extend((0..33000).map(|i| (format!("x{i}"), mori_core::Value::Bool(true))));
    assert!(matches!(c.replay(&r), Err(ReplayError::Input(_))));
    let mut c = Payment::new();
    c.scenario
        .state
        .insert("x".repeat(1048577), mori_core::Value::Bool(true));
    assert!(replay::generate(&c.source, "settle", &c.scenario, &c.target, c.auth()).is_err());
    let mut c = Payment::new();
    c.authority.owner = "x".repeat(1048577);
    assert!(replay::generate(&c.source, "settle", &c.scenario, &c.target, c.auth()).is_err());
    let mut c = Payment::new();
    c.target.anchor = "x".repeat(1048577);
    assert!(replay::generate(&c.source, "settle", &c.scenario, &c.target, c.auth()).is_err());
}
#[test]
fn failures_return_no_execution_and_checked_uint_semantics_remain() {
    let o = oracle();
    let c = &o["contrasting"];
    let scenario: Scenario = serde_json::from_value(c["scenario"].clone()).unwrap();
    let target: Target = serde_json::from_value(c["unsigned_target"].clone()).unwrap();
    let source = c["source"].as_str().unwrap();
    for src in [
        "bad source".to_owned(),
        source.replace("ensures frozen and collected == 40;", "ensures false;"),
        source.replace(
            "amount - 3",
            "amount + 340282366920938463463374607431768211455",
        ),
    ] {
        let err =
            replay::generate(&src, "collect", &scenario, &target, Auth::Unsigned).unwrap_err();
        let value = serde_json::to_value(err).unwrap();
        assert!(value.get("execution").is_none());
        assert!(value.get("effects").is_none());
    }
    let _expected: Execution = serde_json::from_value(c["expected_execution"].clone()).unwrap();
}
#[test]
fn all_trusted_target_and_authority_fields_are_independent_roots() {
    let c = Payment::new();
    let r = c.record();
    let target_edits: [fn(&mut Target); 7] = [
        |t| t.network = "other".into(),
        |t| t.anchor = "other".into(),
        |t| t.max_instructions += 1,
        |t| t.max_artifact_bytes += 1,
        |t| t.signed_policy.as_mut().unwrap().program_address = "other".into(),
        |t| t.signed_policy.as_mut().unwrap().owner = "other".into(),
        |t| t.signed_policy.as_mut().unwrap().beneficiary = "other".into(),
    ];
    for edit in target_edits {
        let mut changed = Payment::new();
        edit(&mut changed.target);
        assert!(changed.replay(&r).is_err());
    }
    let authority_edits: [fn(&mut ExpectedAuthority); 13] = [
        |a| a.network = "other".into(),
        |a| a.anchor = "other".into(),
        |a| a.program_address = "other".into(),
        |a| a.target_hash = "00".repeat(32),
        |a| a.owner = "other".into(),
        |a| a.beneficiary = "other".into(),
        |a| {
            a.public_key_hex="0677506737143823730512c5525e0b30acb75906fa7d53f494ba689b78620133d4cdab997110c2f1e02bb16ebef816c9d0a6025386825581a688b9bfa326360a".into()
        },
        |a| a.predecessor = "other".into(),
        |a| a.nonce = UInt(2),
        |a| a.limits[0].gross_cap = UInt(73),
        |a| a.limits[0].fee_cap = UInt(3),
        |a| a.not_before = UInt(11),
        |a| a.not_after = UInt(19),
    ];
    for edit in authority_edits {
        let mut changed = Payment::new();
        edit(&mut changed.authority);
        assert!(changed.replay(&r).is_err());
    }
    for (path, value) in [
        ("/body/claim/context_hash", json!("00".repeat(32))),
        ("/body/claim/financial_bindings/2/name", json!("forged")),
        ("/body/claim/registry/0/creditor", json!("forged")),
        (
            "/body/claim/authorization/public_key_hex",
            json!(
                "0677506737143823730512c5525e0b30acb75906fa7d53f494ba689b78620133d4cdab997110c2f1e02bb16ebef816c9d0a6025386825581a688b9bfa326360a"
            ),
        ),
    ] {
        let mut bad = edited(&r, |v| *v.pointer_mut(path).unwrap() = value);
        rehash(&mut bad);
        assert!(
            matches!(c.replay(&bad), Err(ReplayError::Divergence(_))),
            "{path}"
        );
    }
    // A malformed authorization descriptor shape is refused before semantic traversal.
    let mut bad = serde_json::to_value(&r).unwrap();
    bad["body"]["claim"]["authorization_bindings"][0]["name"] = "forged".into();
    assert!(ReplayRecord::from_json(&bad.to_string()).is_err());
}
#[test]
fn contrast_untouched_balance_state_and_presentzero_membership_are_compared() {
    let o = oracle();
    let c = &o["contrasting"];
    let source = c["source"].as_str().unwrap();
    let scenario: Scenario = serde_json::from_value(c["scenario"].clone()).unwrap();
    let target: Target = serde_json::from_value(c["unsigned_target"].clone()).unwrap();
    let r = replay::generate(source, "collect", &scenario, &target, Auth::Unsigned).unwrap();
    for (path, value) in [
        ("/body/execution/post/state/frozen/value", json!(false)),
        ("/body/execution/post/balances/2/amount", json!("8")),
        ("/body/execution/post/liabilities/2/amount", json!("6")),
    ] {
        let mut bad = edited(&r, |v| *v.pointer_mut(path).unwrap() = value);
        rehash(&mut bad);
        match replay::replay(source, "collect", &scenario, &target, Auth::Unsigned, &bad)
            .unwrap_err()
        {
            ReplayError::Divergence(d) => assert_eq!(d.field, path),
            e => panic!("{e:?}"),
        }
    }
}
mod shapes {
    use mori_backend::{Target, authorization::ExpectedAuthority};
    include!("support/signed_shapes.rs");
}
#[test]
fn full_original_sixteen_asset_duty_shape_keeps_production_preparation_refusal() {
    let c = Payment::new();
    let (source, scenario, target, authority) =
        shapes::fixture(16, 16, "settle", &c.target, &c.authority);
    let program = mori_core::parse(&source).unwrap();
    let artifact = mori_backend::compile_signed(&program, &target).unwrap();
    let circuit = artifact
        .circuits
        .iter()
        .find(|c| c.action == "settle")
        .unwrap();
    assert_eq!(circuit.liability_frame.len(), 16);
    assert_eq!(circuit.balance_frame.len(), 48);
    let execution = mori_core::simulate(&program, "settle", &scenario).unwrap();
    assert_eq!(execution.effects.len(), 50);
    assert_eq!(execution.post.liabilities.len(), 16);
    let refusal = mori_backend::signed_relation::prepare_intent(
        &source,
        "settle",
        &scenario,
        &target,
        &authority,
        UInt(15),
    )
    .unwrap_err();
    assert_eq!(refusal.code, "mori::backend::limit");
    assert!(refusal.message.contains("JSON exceeds 1 MiB"));
}
