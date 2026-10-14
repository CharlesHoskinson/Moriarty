//! Public fixed k256 reference vectors; no signing or native execution.
use mori_core::{intent::*, *};
use serde_json::{Value as Json, json};
fn fixtures() -> Json {
    serde_json::from_str(include_str!(
        "../../mori/tests/fixtures/metered-intents/public-vectors.json"
    ))
    .unwrap()
}
struct Fixture {
    source: String,
    program: Program,
    scenario: Scenario,
    authority: Authorization,
    signed: SignedIntent,
    oracle: Json,
    prepared: PreparedIntent,
}
fn fixture(name: &str, scheme: &str) -> Fixture {
    let all = fixtures();
    let v = &all[name];
    let sig = &v["vectors"][scheme];
    let source = v["source"].as_str().unwrap().to_owned();
    Fixture {
        program: parse(&source).unwrap(),
        source,
        scenario: serde_json::from_value(v["scenario"].clone()).unwrap(),
        authority: serde_json::from_value(sig["authority"].clone()).unwrap(),
        signed: serde_json::from_value(sig["signed"].clone()).unwrap(),
        oracle: v["oracle"].clone(),
        prepared: serde_json::from_value(sig["prepared"].clone()).unwrap(),
    }
}
fn run(f: &Fixture, cap: u128, at: u128) -> Result<MeteredExecution, MeteredExecutionError> {
    verify_and_simulate_metered(
        &f.program,
        &hash_bytes(f.source.as_bytes()),
        "run",
        &f.scenario,
        &f.authority,
        &f.signed,
        UInt(at),
        UInt(cap),
    )
}
fn report(c: &CostReport, cap: u128, used: u128) {
    assert_eq!(
        serde_json::to_value(c).unwrap(),
        json!({"profile":"moriarty-reference-logical-cost/1","cap":cap.to_string(),"used":used.to_string()})
    );
}
fn refusal(f: &Fixture, cap: u128, used: u128, code: &str, step: Option<u16>) {
    let before = (f.scenario.clone(), f.authority.clone());
    let e = run(f, cap, 15).unwrap_err();
    report(&e.cost, cap, used);
    assert_eq!(e.error.code, format!("mori::core::{code}"));
    assert_eq!(e.error.step, step);
    let j = serde_json::to_value(e).unwrap();
    for field in ["execution", "post", "effects", "signature_valid"] {
        assert!(j.get(field).is_none());
    }
    assert_eq!(before, (f.scenario.clone(), f.authority.clone()));
}
#[test]
fn exact_caps_and_fixed_signing_bytes_preserve_complete_legacy_execution() {
    for scheme in ["schnorr", "ecdsa"] {
        for (name, total, step) in [
            ("counter", 16, 4),
            ("payment", 4, 1),
            ("two_asset_duty", 18, 7),
        ] {
            let f = fixture(name, scheme);
            assert_eq!(
                prepare_intent(
                    &f.program,
                    &hash_bytes(f.source.as_bytes()),
                    "run",
                    &f.scenario,
                    &f.authority
                )
                .unwrap(),
                f.prepared
            );
            assert_eq!(
                hex(&signing_message(&f.signed.intent).unwrap()),
                f.prepared.signing_message_hex
            );
            let ordinary = verify_and_simulate(
                &f.program,
                &hash_bytes(f.source.as_bytes()),
                "run",
                &f.scenario,
                &f.authority,
                &f.signed,
                UInt(15),
            )
            .unwrap();
            for cap in [total, u128::MAX] {
                let m = run(&f, cap, 15).unwrap();
                report(&m.cost, cap, total);
                assert_eq!(m.execution, ordinary);
                assert_eq!(
                    serde_json::to_vec(&m.execution).unwrap(),
                    serde_json::to_vec(&ordinary).unwrap()
                );
            }
            let post = serde_json::to_value(&ordinary.post).unwrap();
            for (oracle, field) in [
                ("expected_state", "state"),
                ("expected_balances", "balances"),
                ("expected_liabilities", "liabilities"),
            ] {
                if let Some(expected) = f.oracle.get(oracle) {
                    assert_eq!(&post[field], expected);
                }
            }
            // Independent effect order and amounts, in addition to complete legacy equality.
            let amounts = ordinary
                .effects
                .iter()
                .filter_map(|effect| match effect {
                    Effect::Transfer { amount, .. } | Effect::Fee { amount, .. } => Some(amount.0),
                    _ => None,
                })
                .collect::<Vec<_>>();
            match name {
                "payment" => {
                    assert_eq!(amounts, vec![7, 2]);
                    assert!(matches!(
                        ordinary.effects.as_slice(),
                        [Effect::Transfer { .. }, Effect::Fee { .. }]
                    ));
                }
                "two_asset_duty" => {
                    assert_eq!(amounts, vec![7, 2, 3, 5, 1]);
                    assert!(matches!(
                        ordinary.effects.as_slice(),
                        [
                            Effect::Transfer { .. },
                            Effect::Fee { .. },
                            Effect::Transfer { .. },
                            Effect::Transfer { .. },
                            Effect::Fee { .. },
                            Effect::Liability { .. },
                            Effect::Liability { .. }
                        ]
                    ));
                }
                _ => assert!(matches!(
                    ordinary.effects.as_slice(),
                    [Effect::State { .. }, Effect::Emit { .. }]
                )),
            }
            refusal(&f, 0, 0, "meter", Some(0));
            refusal(&f, total - 1, total - 1, "meter", Some(step));
            report(&run(&f, total, 15).unwrap().cost, total, total);
        }
    }
}
#[test]
fn payment_exact_cost_is_four() {
    for scheme in ["schnorr", "ecdsa"] {
        let f = fixture("payment", scheme);
        report(&run(&f, 4, 15).unwrap().cost, 4, 4);
    }
}
#[test]
fn payment_cap_three_refuses_second_statement() {
    for scheme in ["schnorr", "ecdsa"] {
        refusal(&fixture("payment", scheme), 3, 3, "meter", Some(1));
    }
}
#[test]
fn late_authority_checks_keep_full_cost_and_hide_all_execution() {
    for scheme in ["schnorr", "ecdsa"] {
        for name in [
            "payment_gross_cap",
            "payment_fee_cap",
            "payment_net_floor",
            "payment_missing_limit",
            "two_asset_duty_gross_cap",
            "two_asset_duty_fee_cap",
            "two_asset_duty_net_floor",
            "two_asset_duty_liability_cap",
            "two_asset_duty_other_fee",
            "two_asset_duty_missing_limit",
        ] {
            let f = fixture(name, scheme);
            let total = if name.starts_with("payment") { 4 } else { 18 };
            refusal(&f, total, total, "authority", None);
            refusal(
                &f,
                total - 1,
                total - 1,
                "meter",
                Some(if total == 4 { 1 } else { 7 }),
            );
        }
    }
}
#[test]
fn authentication_bindings_and_time_precede_zero_budget() {
    for scheme in ["schnorr", "ecdsa"] {
        let f = fixture("counter", scheme);
        for at in [10, 20] {
            report(&run(&f, 16, at).unwrap().cost, 16, 16);
        }
        for at in [9, 21] {
            let e = run(&f, 0, at).unwrap_err();
            report(&e.cost, 0, 0);
            assert_eq!(e.error.message, "intent is outside validity interval");
        }
        for pointer in [
            "/source_hash",
            "/program_hash",
            "/scenario_hash",
            "/action",
            "/assets",
            "/profile",
            "/semantics",
            "/authority/anchor",
            "/authority/network",
            "/authority/program_address",
            "/authority/target_hash",
            "/authority/nonce",
            "/authority/predecessor",
            "/authority/signer",
            "/authority/public_key_hex",
            "/authority/scheme",
            "/authority/not_before",
            "/authority/not_after",
            "/authority/limits",
        ] {
            let mut changed = fixture("counter", scheme);
            let mut signed = serde_json::to_value(&changed.signed).unwrap();
            let slot = signed["intent"].pointer_mut(pointer).unwrap();
            *slot = match pointer {
                "/assets" => json!([{"network":"local","issuer":"i","reference":"A","scale":0}]),
                "/authority/limits" => {
                    fixtures()["payment"]["vectors"][scheme]["authority"]["limits"].clone()
                }
                "/authority/scheme" => json!(if scheme == "schnorr" {
                    "ecdsa_secp256k1_sha256"
                } else {
                    "schnorr_bip340"
                }),
                _ => json!("changed"),
            };
            if matches!(
                pointer,
                "/authority/nonce" | "/authority/not_before" | "/authority/not_after"
            ) {
                *signed["intent"].pointer_mut(pointer).unwrap() = json!("999");
            }
            changed.signed = serde_json::from_value(signed).unwrap();
            refusal(&changed, 0, 0, "intent", None);
        }
        let mut fresh = fixture("fresh_counter", scheme);
        fresh.signed = f.signed.clone();
        refusal(&fresh, 16, 0, "intent", None);
        report(
            &run(&fixture("fresh_counter", scheme), 16, 15).unwrap().cost,
            16,
            16,
        );
        for mode in [0, 1, 2, 3] {
            let mut bad = fixture("counter", scheme);
            bad.signed.signature_hex = match mode {
                0 => "00".repeat(64),
                1 => bad.signed.signature_hex.to_uppercase(),
                2 => bad.signed.signature_hex[..126].into(),
                _ => format!(
                    "{}{}",
                    if &bad.signed.signature_hex[..1] == "0" {
                        "1"
                    } else {
                        "0"
                    },
                    &bad.signed.signature_hex[1..]
                ),
            };
            refusal(&bad, 0, 0, "signature", None);
        }
        let mut changed_scenario = fixture("counter", scheme);
        changed_scenario
            .scenario
            .inputs
            .insert("amount".into(), Value::UInt(UInt(8)));
        refusal(&changed_scenario, 0, 0, "intent", None);
        let invalid_hash = verify_and_simulate_metered(
            &f.program,
            "INVALID",
            "run",
            &f.scenario,
            &f.authority,
            &f.signed,
            UInt(15),
            UInt(0),
        )
        .unwrap_err();
        report(&invalid_hash.cost, 0, 0);
        assert_eq!(invalid_hash.error.code, "mori::core::intent");
        let mut bad = fixture("counter", scheme);
        bad.authority.public_key_hex = "00".into();
        refusal(&bad, 0, 0, "signature", None);
        let mut bad = fixture("counter", scheme);
        bad.authority.anchor.clear();
        refusal(&bad, 0, 0, "intent", None);
        let mut bad = fixture("counter", scheme);
        bad.program.expressions.push(Expr::Not { value: u16::MAX });
        refusal(&bad, 0, 0, "expression", None);
        let e = verify_and_simulate_metered(
            &f.program,
            &hash_bytes(f.source.as_bytes()),
            "missing",
            &f.scenario,
            &f.authority,
            &f.signed,
            UInt(15),
            UInt(0),
        )
        .unwrap_err();
        report(&e.cost, 0, 0);
        assert_eq!(e.error.message, "unknown action");
    }
    let mut f = fixture("counter", "ecdsa");
    f.signed.signature_hex = fixtures()["counter"]["vectors"]["ecdsa"]["high_s_signature_hex"]
        .as_str()
        .unwrap()
        .into();
    refusal(&f, 0, 0, "signature", None);
    assert_eq!(
        run(&f, 0, 15).unwrap_err().error.message,
        "noncanonical high-S ECDSA signature"
    );
}
#[test]
fn validly_signed_admission_and_runtime_failures_keep_original_precedence() {
    for scheme in ["schnorr", "ecdsa"] {
        for (name, code) in [
            ("wrong_input", "input"),
            ("missing_input", "input"),
            ("duplicate_balance", "balance"),
            ("missing_balance", "balance"),
            ("wrong_asset", "asset"),
        ] {
            refusal(&fixture(name, scheme), 0, 0, code, None);
        }
        refusal(
            &fixture("insufficient_fee", scheme),
            4,
            4,
            "balance",
            Some(1),
        );
        refusal(&fixture("insufficient_fee", scheme), 3, 3, "meter", Some(1));
        refusal(&fixture("requires", scheme), 16, 8, "requires", Some(1));
        refusal(&fixture("requires", scheme), 7, 7, "meter", Some(1));
    }
}
