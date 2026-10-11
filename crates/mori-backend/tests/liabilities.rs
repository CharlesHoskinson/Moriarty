//! Ordinary compiler and honest-preimage checks. These are not native proofs.
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{KeyLocation, ProofPreimage},
};
use mori_backend::{CircuitArtifact, Target, compile};
use mori_core::*;

const SOURCE: &str = r#"
language "moriarty/1";
agreement DutyBook {
  state audit: UInt = 0;
  asset USD = { network: "local", issuer: "mint-a", reference: "usd", scale: 2 };
  asset NOTE = { network: "local", issuer: "mint-b", reference: "note", scale: 0 };
  action open(delta: UInt) {
    liability("zeta", USD, "buyer", "seller", delta);
    liability("alpha", NOTE, "borrower", "lender", 0);
  }
  action repay(delta: UInt) { discharge("zeta", delta); }
  action audit_only() { next audit = audit + 1; emit "checked"(audit); }
  action sequence() {
    liability("zeta", USD, "buyer", "seller", 4);
    discharge("zeta", 3);
    liability("zeta", USD, "buyer", "seller", 2);
    discharge("zeta", 5);
  }
}
"#;
fn artifact(source: &str) -> mori_backend::Artifact {
    compile(
        &parse(source).unwrap(),
        &Target::ledger9("local", "duty-anchor"),
    )
    .unwrap()
}
fn circuit<'a>(a: &'a mori_backend::Artifact, action: &str) -> &'a CircuitArtifact {
    a.circuits.iter().find(|c| c.action == action).unwrap()
}
fn liability(p: &Program, name: &str, amount: u128) -> Liability {
    let alpha = name == "alpha";
    Liability {
        name: name.into(),
        asset: p.assets[usize::from(alpha)].identity.clone(),
        debtor: if alpha { "borrower" } else { "buyer" }.into(),
        creditor: if alpha { "lender" } else { "seller" }.into(),
        amount: UInt(amount),
    }
}
fn field(v: &Value) -> Fr {
    match v {
        Value::UInt(n) => Fr::from(n.0),
        Value::Bool(b) => Fr::from(*b),
    }
}
fn debt(s: &Snapshot, name: &str, presence: bool) -> Fr {
    let entry = s.liabilities.iter().find(|l| l.name == name);
    if presence {
        Fr::from(entry.is_some())
    } else {
        Fr::from(entry.map_or(0, |l| l.amount.0))
    }
}
fn values(c: &CircuitArtifact, s: &Scenario, e: &Execution) -> Vec<Fr> {
    c.public_inputs
        .iter()
        .map(|b| match b.role.as_str() {
            "context" => Fr::from(b.constant.as_ref().unwrap().parse::<u128>().unwrap()),
            "pre_state" => field(&e.pre.state[&b.name]),
            "post_state" => field(&e.post.state[&b.name]),
            "input" => field(&s.inputs[&b.name]),
            "pre_liability_presence" => debt(&e.pre, &b.name, true),
            "pre_liability_amount" => debt(&e.pre, &b.name, false),
            "post_liability_presence" => debt(&e.post, &b.name, true),
            "post_liability_amount" => debt(&e.post, &b.name, false),
            "pre_balance" | "post_balance" => {
                let (_, account): (String, String) = serde_json::from_str(&b.name).unwrap();
                let snapshot = if b.role == "pre_balance" {
                    &e.pre
                } else {
                    &e.post
                };
                Fr::from(
                    snapshot
                        .balances
                        .iter()
                        .find(|balance| {
                            balance.account == account && Some(&balance.asset) == b.asset.as_ref()
                        })
                        .unwrap()
                        .amount
                        .0,
                )
            }
            "effect" => {
                let (step, part) = b.name.split_once('.').unwrap();
                let step = step.parse::<usize>().unwrap();
                match &e.steps[step].effects[0] {
                    Effect::Liability {
                        name,
                        before,
                        after,
                        ..
                    } => match part {
                        "before_presence" => debt(
                            if step == 0 {
                                &e.pre
                            } else {
                                &e.steps[step - 1].snapshot
                            },
                            name,
                            true,
                        ),
                        "after_presence" => debt(&e.steps[step].snapshot, name, true),
                        "before_amount" => Fr::from(before.0),
                        "after_amount" => Fr::from(after.0),
                        _ => panic!("unexpected liability effect field"),
                    },
                    Effect::State { before, after, .. } => {
                        field(if part == "before" { before } else { after })
                    }
                    Effect::Emit { value, .. } => field(value),
                    Effect::Transfer { amount, .. } | Effect::Fee { amount, .. } => {
                        Fr::from(amount.0)
                    }
                }
            }
            role => panic!("unexpected role {role}"),
        })
        .collect()
}
fn check(c: &CircuitArtifact, v: &[Fr]) -> bool {
    let ir = midnight_zkir_v3::IrSource::load(c.zkir.as_bytes()).unwrap();
    ProofPreimage {
        inputs: vec![],
        private_transcript: vec![],
        public_transcript_inputs: v.to_vec(),
        public_transcript_outputs: v.to_vec(),
        binding_input: Fr::from(0u64),
        communications_commitment: None,
        key_location: KeyLocation("ordinary-liability-check-unqualified".into()),
    }
    .check(&ir)
    .is_ok()
}
fn scenario(p: &Program, initial: Option<u128>, delta: Option<u128>) -> Scenario {
    Scenario {
        liabilities: initial
            .into_iter()
            .map(|n| liability(p, "zeta", n))
            .collect(),
        inputs: delta
            .into_iter()
            .map(|n| ("delta".into(), Value::UInt(UInt(n))))
            .collect(),
        ..Default::default()
    }
}
#[test]
fn every_action_has_sorted_typed_complete_frame_and_explicit_revision() {
    let p = parse(SOURCE).unwrap();
    let a = artifact(SOURCE);
    let json = serde_json::to_value(&a).unwrap();
    assert_eq!(json["format"], "moriarty-zkir-artifact/3");
    assert_eq!(json["abi"], "moriarty-public-abi/3");
    assert_eq!(json["relation"], "moriarty-zkir-relation/3");
    assert_eq!(json["authorization_mode"], "unsigned");
    assert_eq!(json["resources"]["liability_registry_entries"], 2);
    assert_eq!(json["resources"]["max_liability_registry_entries"], 16);
    assert!(
        !json["target"]
            .as_object()
            .unwrap()
            .contains_key("max_liability_registry_entries")
    );
    for c in &a.circuits {
        let frame = serde_json::to_value(c).unwrap()["liability_frame"]
            .as_array()
            .unwrap()
            .clone();
        assert_eq!(frame.len(), 2);
        for (i, (name, asset, debtor, creditor)) in [
            ("alpha", 1, "borrower", "lender"),
            ("zeta", 0, "buyer", "seller"),
        ]
        .into_iter()
        .enumerate()
        {
            let f = &frame[i];
            assert_eq!(f["name"], name);
            assert_eq!(
                f["asset"],
                serde_json::to_value(&p.assets[asset].identity).unwrap()
            );
            assert_eq!(f["debtor"], debtor);
            assert_eq!(f["creditor"], creditor);
            for (pos, role, ty) in [
                (
                    "pre_presence_position",
                    "pre_liability_presence",
                    Type::Bool,
                ),
                ("pre_amount_position", "pre_liability_amount", Type::UInt),
                (
                    "post_presence_position",
                    "post_liability_presence",
                    Type::Bool,
                ),
                ("post_amount_position", "post_liability_amount", Type::UInt),
            ] {
                let b = &c.public_inputs[f[pos].as_u64().unwrap() as usize];
                assert_eq!(b.role, role);
                assert_eq!(b.name, name);
                assert_eq!(b.ty, ty);
                assert_eq!(b.constant, None);
                assert_eq!(b.asset.as_ref(), Some(&p.assets[asset].identity));
            }
        }
        let ir = midnight_zkir_v3::IrSource::load(c.zkir.as_bytes()).unwrap();
        assert_eq!(ir.outputs.len(), 5); // audit plus two (presence, amount) pairs
    }
    let c = circuit(&a, "sequence");
    for (i, eff) in c.effect_frame.iter().enumerate() {
        let j = serde_json::to_value(eff).unwrap();
        assert_eq!(eff.step, i as u32);
        assert_eq!(eff.kind, "liability");
        assert_eq!(
            j["operation"],
            if i % 2 == 0 {
                "create_or_accrue"
            } else {
                "discharge"
            }
        );
        assert_eq!(j["liability"]["name"], "zeta");
        assert_eq!(
            j["liability"]["asset"],
            serde_json::to_value(&p.assets[0].identity).unwrap()
        );
        assert_eq!(j["liability"]["debtor"], "buyer");
        assert_eq!(j["liability"]["creditor"], "seller");
        assert_eq!(eff.from, None);
        assert_eq!(eff.to, None);
        assert_eq!(eff.asset, None);
        assert_eq!(eff.public_positions.len(), 4);
        for (pos, part) in eff.public_positions.iter().zip([
            "before_presence",
            "before_amount",
            "after_presence",
            "after_amount",
        ]) {
            assert_eq!(c.public_inputs[*pos as usize].name, format!("{i}.{part}"));
        }
    }
}
#[test]
fn presence_lifecycle_and_ordered_amounts_match_reference_preimages() {
    let p = parse(SOURCE).unwrap();
    let a = artifact(SOURCE);
    for (action, before, delta) in [
        ("open", None, Some(7)),
        ("open", None, Some(0)),
        ("open", Some(0), Some(0)),
        ("open", Some(5), Some(7)),
        ("open", Some(u128::MAX), Some(0)),
        ("repay", Some(12), Some(7)),
        ("repay", Some(7), Some(7)),
        ("repay", Some(0), Some(0)),
        ("repay", Some(u128::MAX), Some(u128::MAX)),
        ("sequence", Some(10), None),
        ("audit_only", Some(19), None),
    ] {
        let mut s = scenario(&p, before, delta);
        s.liabilities.push(liability(&p, "alpha", 0));
        let e = simulate(&p, action, &s).unwrap();
        let c = circuit(&a, action);
        let v = values(c, &s, &e);
        assert!(check(c, &v), "{action}, {before:?}, {delta:?}");
        // Every effect and closing slot is constrained; untouched duties included.
        for (i, b) in c
            .public_inputs
            .iter()
            .enumerate()
            .filter(|(_, b)| b.role == "effect" || b.role.starts_with("post_liability"))
        {
            let mut changed = v.clone();
            changed[i] = if b.ty == Type::Bool {
                Fr::from(changed[i] == Fr::from(0u64))
            } else {
                changed[i] + Fr::from(1u64)
            };
            assert!(!check(c, &changed), "changed {} {}", b.role, b.name);
        }
    }
    let e = simulate(&p, "sequence", &scenario(&p, Some(10), None)).unwrap();
    assert_eq!(
        e.steps
            .iter()
            .map(|s| s
                .snapshot
                .liabilities
                .iter()
                .find(|l| l.name == "zeta")
                .unwrap()
                .amount
                .0)
            .collect::<Vec<_>>(),
        [14, 11, 13, 8]
    );
}
#[test]
fn absence_boolean_canonical_zero_and_uint_ranges_are_constraints() {
    let p = parse(SOURCE).unwrap();
    let a = artifact(SOURCE);
    let c = circuit(&a, "audit_only");
    let s = scenario(&p, None, None);
    let e = simulate(&p, "audit_only", &s).unwrap();
    let v = values(c, &s, &e);
    assert!(check(c, &v));
    for invalid_presence in [Fr::from(2u64), Fr::from(0u64) - Fr::from(1u64)] {
        let mut changed = v.clone();
        for (i, b) in c
            .public_inputs
            .iter()
            .enumerate()
            .filter(|(_, b)| b.name == "zeta" && b.role.ends_with("liability_presence"))
        {
            let _ = b;
            changed[i] = invalid_presence;
        }
        assert!(!check(c, &changed));
    }
    for amount in [Fr::from(1u64), Fr::from(u128::MAX) + Fr::from(1u64)] {
        // Change both sides together to defeat frame equality; absence/range must still reject.
        let mut changed = v.clone();
        for (i, b) in c
            .public_inputs
            .iter()
            .enumerate()
            .filter(|(_, b)| b.name == "zeta" && b.role.ends_with("liability_amount"))
        {
            let _ = b;
            changed[i] = amount;
        }
        assert!(!check(c, &changed));
    }
    let c = circuit(&a, "repay");
    let mut v: Vec<_> = c
        .public_inputs
        .iter()
        .map(|b| {
            if b.role == "context" {
                Fr::from(b.constant.as_ref().unwrap().parse::<u128>().unwrap())
            } else {
                Fr::from(0u64)
            }
        })
        .collect();
    assert!(!check(c, &v), "absent discharge0 must fail");
    // Keep every debt-related presence zero except fabricate the after-effect and closing presence.
    for (i, b) in c.public_inputs.iter().enumerate() {
        if b.name == "0.after_presence" || b.role == "post_liability_presence" && b.name == "zeta" {
            v[i] = Fr::from(1u64);
        }
    }
    assert!(!check(c, &v));
}
#[test]
fn registry_conflicts_unknown_discharges_and_seventeenth_name_refuse() {
    assert!(
        compile(
            &parse(&SOURCE.replace(
                "discharge(\"zeta\", delta)",
                "discharge(\"missing\", delta)"
            ))
            .unwrap(),
            &Target::ledger9("local", "a")
        )
        .is_err()
    );
    for replacement in [
        "liability(\"zeta\", NOTE, \"buyer\", \"seller\", 2)",
        "liability(\"zeta\", USD, \"other\", \"seller\", 2)",
        "liability(\"zeta\", USD, \"buyer\", \"other\", 2)",
    ] {
        assert!(
            compile(
                &parse(&SOURCE.replace(
                    "liability(\"zeta\", USD, \"buyer\", \"seller\", 2)",
                    replacement
                ))
                .unwrap(),
                &Target::ledger9("local", "a")
            )
            .is_err()
        );
    }
    for count in [0, 1, 16, 17] {
        let mut p = parse(SOURCE).unwrap();
        p.actions.truncate(1);
        p.actions[0].statements.clear();
        for i in 0..count {
            p.actions[0].statements.push(Statement::Liability {
                name: format!("duty{i:02}"),
                asset: "USD".into(),
                debtor: "d".into(),
                creditor: "c".into(),
                amount: 0,
            });
        }
        // Literal expression0 in this program is delta; it is a valid UInt input.
        let result = compile(&p, &Target::ledger9("local", "a"));
        if count == 17 {
            assert_eq!(result.unwrap_err().code, "mori::backend::limit");
        } else {
            let a = result.unwrap();
            let j = serde_json::to_value(&a).unwrap();
            assert_eq!(j["resources"]["liability_registry_entries"], count);
            assert_eq!(
                j["circuits"][0]["liability_frame"]
                    .as_array()
                    .unwrap()
                    .len(),
                count
            );
        }
    }
}
#[test]
fn strict_artifact_revision_and_profile_decoding_refuse_stale_or_unknown_fields() {
    let a = artifact(SOURCE);
    let j = serde_json::to_value(&a).unwrap();
    assert!(serde_json::from_value::<mori_backend::Artifact>(j.clone()).is_ok());
    for (key, bad) in [
        ("format", "moriarty-zkir-artifact/1"),
        ("abi", "moriarty-public-abi/1"),
        ("relation", "moriarty-zkir-relation/1"),
        ("format", "moriarty-zkir-artifact/2"),
        ("abi", "moriarty-public-abi/2"),
        ("relation", "moriarty-zkir-relation/2"),
        ("authorization_mode", "signed"),
    ] {
        let mut changed = j.clone();
        changed[key] = bad.into();
        assert!(serde_json::from_value::<mori_backend::Artifact>(changed).is_err());
    }
    let mut changed = j.clone();
    changed["resources"]["max_liability_registry_entries"] = 15.into();
    assert!(serde_json::from_value::<mori_backend::Artifact>(changed).is_err());
    let mut changed = j.clone();
    changed["resources"]["liability_registry_entries"] = 17.into();
    assert!(serde_json::from_value::<mori_backend::Artifact>(changed).is_err());
    for key in ["profile", "ledger_revision", "zkir_version"] {
        let mut changed = j.clone();
        changed["target"][key] = "stale".into();
        assert!(serde_json::from_value::<mori_backend::Artifact>(changed).is_err());
    }
    for location in [
        "root", "circuit", "frame", "effect", "binding", "resource", "gate", "identity",
    ] {
        let mut changed = j.clone();
        let object = match location {
            "root" => &mut changed,
            "circuit" => &mut changed["circuits"][0],
            "frame" => &mut changed["circuits"][0]["liability_frame"][0],
            "effect" => &mut changed["circuits"][0]["effect_frame"][0],
            "binding" => &mut changed["circuits"][0]["public_inputs"][0],
            "resource" => &mut changed["resources"],
            "gate" => &mut changed["gates"][0],
            "identity" => &mut changed["circuits"][0]["effect_frame"][0]["liability"],
            _ => unreachable!(),
        };
        object["unrecognized"] = true.into();
        assert!(
            serde_json::from_value::<mori_backend::Artifact>(changed).is_err(),
            "{location}"
        );
    }
    let mut changed = j;
    changed["circuits"][0]
        .as_object_mut()
        .unwrap()
        .remove("liability_frame");
    assert!(serde_json::from_value::<mori_backend::Artifact>(changed).is_err());
}

#[test]
fn independent_mixed_program_keeps_token_state_event_and_duty_effects_separate() {
    let source = r#"
    language "moriarty/1";
    agreement EscrowReceipt {
        state delivered: UInt = 0;
        asset TOKEN = { network: "local", issuer: "independent-issuer", reference: "credit", scale: 3 };
        action deliver(units: UInt) {
            let owed = units * 2;
            liability("invoice", TOKEN, "customer", "provider", owed);
            transfer(TOKEN, "escrow", "provider", units);
            fee(TOKEN, "escrow", "sponsor", 1);
            next delivered = delivered + units;
            discharge("invoice", units);
            emit "delivery"(delivered);
        }
        action zero_cycle() {
            liability("invoice", TOKEN, "customer", "provider", 0);
            discharge("invoice", 0);
            liability("invoice", TOKEN, "customer", "provider", 2);
            discharge("invoice", 2);
        }
    }
    "#;
    let p = parse(source).unwrap();
    let a = artifact(source);
    let s = Scenario {
        inputs: [("units".into(), Value::UInt(UInt(3)))].into(),
        balances: [("escrow", 20), ("provider", 0), ("sponsor", 0)]
            .into_iter()
            .map(|(account, amount)| Balance {
                account: account.into(),
                asset: p.assets[0].identity.clone(),
                amount: UInt(amount),
            })
            .collect(),
        ..Default::default()
    };
    let e = simulate(&p, "deliver", &s).unwrap();
    let c = circuit(&a, "deliver");
    assert!(check(c, &values(c, &s, &e)));
    assert_eq!(
        c.effect_frame
            .iter()
            .map(|e| e.kind.as_str())
            .collect::<Vec<_>>(),
        ["liability", "transfer", "fee", "state", "liability", "emit"]
    );
    assert_eq!(
        c.effect_frame.iter().map(|e| e.step).collect::<Vec<_>>(),
        [1, 2, 3, 4, 5, 6]
    );
    assert_eq!(e.post.liabilities[0].amount.0, 3);
    let s = Scenario::default();
    let e = simulate(&p, "zero_cycle", &s).unwrap();
    let c = circuit(&a, "zero_cycle");
    assert!(check(c, &values(c, &s, &e)));
    assert_eq!(e.post.liabilities.len(), 1);
    assert_eq!(e.post.liabilities[0].amount.0, 0);
    assert_eq!(
        e.steps
            .iter()
            .map(|s| s.snapshot.liabilities[0].amount.0)
            .collect::<Vec<_>>(),
        [0, 0, 2, 0]
    );
}
#[test]
fn intermediate_overflow_and_underflow_cannot_be_hidden_by_final_totals() {
    for overflow in [true, false] {
        let operations = if overflow {
            r#"liability("zeta", USD, "buyer", "seller", 1); discharge("zeta", 1);"#
        } else {
            r#"discharge("zeta", 1); liability("zeta", USD, "buyer", "seller", 1);"#
        };
        let source = format!(
            r#"language "moriarty/1"; agreement InvalidSequence {{
            asset USD = {{ network: "local", issuer: "mint-a", reference: "usd", scale: 2 }};
            action test() {{ {operations} }}
        }}"#
        );
        let p = parse(&source).unwrap();
        let a = artifact(&source);
        let c = circuit(&a, "test");
        let before = if overflow { u128::MAX } else { 0 };
        let s = scenario(&p, Some(before), None);
        assert!(simulate(&p, "test", &s).is_err());
        let intermediate = if overflow {
            Fr::from(before) + Fr::from(1u64)
        } else {
            Fr::from(0u64) - Fr::from(1u64)
        };
        let v = c
            .public_inputs
            .iter()
            .map(|b| match b.role.as_str() {
                "context" => Fr::from(b.constant.as_ref().unwrap().parse::<u128>().unwrap()),
                "pre_liability_presence" | "post_liability_presence" => Fr::from(1u64),
                "pre_liability_amount" | "post_liability_amount" => Fr::from(before),
                "effect" if b.name.ends_with("presence") => Fr::from(1u64),
                "effect" if b.name == "0.after_amount" || b.name == "1.before_amount" => {
                    intermediate
                }
                "effect" => Fr::from(before),
                other => panic!("{other}"),
            })
            .collect::<Vec<_>>();
        assert!(
            !check(c, &v),
            "sequential {}flow",
            if overflow { "over" } else { "under" }
        );
    }
    let p = parse(SOURCE).unwrap();
    let a = artifact(SOURCE);
    for (action, before, delta) in [("open", u128::MAX, 1), ("repay", 3, 4)] {
        let s = scenario(&p, Some(before), Some(delta));
        assert!(simulate(&p, action, &s).is_err());
        let c = circuit(&a, action);
        let v = c
            .public_inputs
            .iter()
            .map(|b| {
                if b.role == "context" {
                    Fr::from(b.constant.as_ref().unwrap().parse::<u128>().unwrap())
                } else if b.role == "input" {
                    Fr::from(delta)
                } else if (b.role == "pre_liability_presence" && b.name == "zeta")
                    || b.name == "0.before_presence"
                    || b.name == "0.after_presence"
                    || (b.role == "post_liability_presence" && b.name == "zeta")
                    || b.name == "1.after_presence"
                    || (b.role == "post_liability_presence" && b.name == "alpha")
                        && action == "open"
                {
                    Fr::from(1u64)
                } else if b.role == "pre_liability_amount" && b.name == "zeta"
                    || b.name == "0.before_amount"
                {
                    Fr::from(before)
                } else {
                    Fr::from(0u64)
                }
            })
            .collect::<Vec<_>>();
        assert!(!check(c, &v), "{action} arithmetic refusal");
    }
}
#[test]
fn context_commits_revision_profile_and_sorted_exact_registry() {
    let p = parse(SOURCE).unwrap();
    let a = artifact(SOURCE);
    let c = circuit(&a, "repay");
    let frame = serde_json::to_value(c).unwrap()["liability_frame"].as_array().unwrap().iter().map(|f| {
        serde_json::json!({"name":f["name"],"asset":f["asset"],"debtor":f["debtor"],"creditor":f["creditor"]})
    }).collect::<Vec<_>>();
    use sha2::{Digest, Sha256};
    // Independent v3 construction: the 18-element context array with the
    // constant-free financial projection of the staged layout.
    let json = serde_json::to_value(c).unwrap();
    let projection: Vec<_> = json["public_inputs"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, b)| serde_json::json!({"position":i,"role":b["role"],"name":b["name"],"ty":b["ty"],"asset":b["asset"]}))
        .collect();
    let expected = Sha256::digest(
        mori_core::intent::canonical(&serde_json::json!([
            "moriarty-financial-context/3",
            "unsigned",
            "moriarty-zkir-artifact/3",
            "moriarty-public-abi/3",
            "moriarty-zkir-relation/3",
            "moriarty-native-transition-binding/3",
            "moriarty-native-transition-proof/3",
            16,
            16,
            frame,
            &p,
            &a.target,
            "repay",
            projection,
            json["effect_frame"],
            json["balance_frame"],
            json["liability_frame"],
            json["replay_steps"]
        ]))
        .unwrap(),
    );
    assert_eq!(c.context_hash, mori_core::intent::hex(&expected));
    for (b, bytes) in c
        .public_inputs
        .iter()
        .take(2)
        .zip(expected.chunks_exact(16))
    {
        assert_eq!(
            b.constant.as_deref(),
            Some(
                u128::from_le_bytes(bytes.try_into().unwrap())
                    .to_string()
                    .as_str()
            )
        );
    }
    let modified = artifact(&SOURCE.replace("\"buyer\"", "\"buyer-2\""));
    assert_ne!(
        c.public_inputs[0].constant,
        circuit(&modified, "repay").public_inputs[0].constant
    );
    assert_ne!(
        c.public_inputs[1].constant,
        circuit(&modified, "repay").public_inputs[1].constant
    );
}
