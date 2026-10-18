//! Ordinary source compilation and public-preimage IR interpretation only.
//! No key generation, parameters, proving, native certificate or ledger access.
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{KeyLocation, ProofPreimage},
};
use mori_backend::{Artifact, CircuitArtifact, Target, compile};
use mori_core::*;

const SOURCE: &str = include_str!("../../../examples/core/escrow-recovery/escrow.mori");
const MANIFEST: &str = include_str!("../../../examples/core/escrow-recovery/mori-tests.json");
fn fixture(action: &str) -> Scenario {
    let text = match action {
        "fund" => include_str!("../../../examples/core/escrow-recovery/fund.json"),
        "partial" => include_str!("../../../examples/core/escrow-recovery/partial.json"),
        "complete" => include_str!("../../../examples/core/escrow-recovery/complete.json"),
        "recover" => include_str!("../../../examples/core/escrow-recovery/recover.json"),
        _ => panic!("unknown fixture"),
    };
    serde_json::from_value(json::parse(text).unwrap()).unwrap()
}
fn artifact(program: &Program) -> Artifact {
    compile(program, &Target::ledger9("local", "finite-escrow-local-qa")).unwrap()
}
fn circuit<'a>(a: &'a Artifact, action: &str) -> &'a CircuitArtifact {
    a.circuits.iter().find(|c| c.action == action).unwrap()
}
fn field(v: &Value) -> Fr {
    match v {
        Value::UInt(n) => Fr::from(n.0),
        Value::Bool(b) => Fr::from(*b),
    }
}
fn duty(snapshot: &Snapshot, name: &str, presence: bool) -> Fr {
    let entry = snapshot.liabilities.iter().find(|l| l.name == name);
    if presence {
        Fr::from(entry.is_some())
    } else {
        Fr::from(entry.map_or(0, |l| l.amount.0))
    }
}
fn public_values(c: &CircuitArtifact, s: &Scenario, e: &Execution) -> Vec<Fr> {
    c.public_inputs
        .iter()
        .map(|b| match b.role.as_str() {
            "context" => Fr::from(b.constant.as_ref().unwrap().parse::<u128>().unwrap()),
            "input" => field(&s.inputs[&b.name]),
            "pre_state" => field(&e.pre.state[&b.name]),
            "post_state" => field(&e.post.state[&b.name]),
            "pre_liability_presence" => duty(&e.pre, &b.name, true),
            "pre_liability_amount" => duty(&e.pre, &b.name, false),
            "post_liability_presence" => duty(&e.post, &b.name, true),
            "post_liability_amount" => duty(&e.post, &b.name, false),
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
                        .find(|row| row.account == account && Some(&row.asset) == b.asset.as_ref())
                        .unwrap()
                        .amount
                        .0,
                )
            }
            "effect" => {
                let (step, part) = b.name.split_once('.').unwrap();
                let step: usize = step.parse().unwrap();
                match &e.steps[step].effects[0] {
                    Effect::Liability {
                        name,
                        before,
                        after,
                        ..
                    } => match part {
                        "before_presence" => duty(
                            if step == 0 {
                                &e.pre
                            } else {
                                &e.steps[step - 1].snapshot
                            },
                            name,
                            true,
                        ),
                        "after_presence" => duty(&e.steps[step].snapshot, name, true),
                        "before_amount" => Fr::from(before.0),
                        "after_amount" => Fr::from(after.0),
                        _ => panic!("unknown duty effect part"),
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
            role => panic!("unknown public role {role}"),
        })
        .collect()
}
fn check(c: &CircuitArtifact, values: &[Fr]) -> bool {
    let ir = midnight_zkir_v3::IrSource::load(c.zkir.as_bytes()).unwrap();
    ProofPreimage {
        inputs: vec![],
        private_transcript: vec![],
        public_transcript_inputs: values.to_vec(),
        public_transcript_outputs: values.to_vec(),
        binding_input: Fr::from(0u64),
        communications_commitment: None,
        key_location: KeyLocation("escrow-ordinary-ir-check-unqualified".into()),
    }
    .check(&ir)
    .is_ok()
}
fn replace(c: &CircuitArtifact, values: &mut [Fr], role: &str, name: &str, amount: u128) {
    let position = c
        .public_inputs
        .iter()
        .position(|b| b.role == role && b.name == name)
        .unwrap();
    values[position] = Fr::from(amount);
}

#[test]
fn source_fixed_two_asset_duty_and_effect_frames_match_independent_expectations() {
    let p = parse(SOURCE).unwrap();
    let a = artifact(&p);
    assert_eq!(a.circuits.len(), 4);
    assert_eq!(a.resources.liability_registry_entries, 2);
    assert!(!a.resources.cost_measured);
    assert!(a.gates.iter().all(|gate| gate.status == "unperformed"));
    let manifest = json::parse(MANIFEST).unwrap();
    for case in manifest["cases"].as_array().unwrap() {
        let action = case["action"].as_str().unwrap();
        let s = fixture(action);
        let e = simulate(&p, action, &s).unwrap();
        let expected_post: Snapshot =
            serde_json::from_value(case["expected"]["post"].clone()).unwrap();
        let expected_effects: Vec<Effect> =
            serde_json::from_value(case["expected"]["effects"].clone()).unwrap();
        assert_eq!(e.post, expected_post);
        assert_eq!(e.effects, expected_effects);
        let c = circuit(&a, action);
        assert_eq!(c.replay_steps as usize, e.steps.len());
        assert_eq!(c.liability_frame.len(), 2);
        for (frame, name, asset, debtor, creditor) in [
            (
                &c.liability_frame[0],
                "delivery",
                &p.assets[1].identity,
                "seller",
                "buyer",
            ),
            (
                &c.liability_frame[1],
                "refund",
                &p.assets[0].identity,
                "escrow",
                "buyer",
            ),
        ] {
            assert_eq!(frame.name, name);
            assert_eq!(&frame.asset, asset);
            assert_eq!(frame.debtor, debtor);
            assert_eq!(frame.creditor, creditor);
            for (position, role, ty) in [
                (
                    frame.pre_presence_position,
                    "pre_liability_presence",
                    Type::Bool,
                ),
                (
                    frame.pre_amount_position,
                    "pre_liability_amount",
                    Type::UInt,
                ),
                (
                    frame.post_presence_position,
                    "post_liability_presence",
                    Type::Bool,
                ),
                (
                    frame.post_amount_position,
                    "post_liability_amount",
                    Type::UInt,
                ),
            ] {
                let binding = &c.public_inputs[position as usize];
                assert_eq!(binding.role, role);
                assert_eq!(binding.name, name);
                assert_eq!(binding.ty, ty);
                assert_eq!(binding.asset.as_ref(), Some(asset));
            }
        }
        assert_eq!(c.effect_frame.len(), e.effects.len());
        for (descriptor, (step, effect)) in c.effect_frame.iter().zip(
            e.steps
                .iter()
                .flat_map(|step| step.effects.iter().map(move |effect| (step.index, effect))),
        ) {
            assert_eq!(descriptor.step, u32::from(step));
            match effect {
                Effect::Transfer {
                    asset, from, to, ..
                }
                | Effect::Fee {
                    asset, from, to, ..
                } => {
                    assert_eq!(descriptor.asset.as_ref(), Some(asset));
                    assert_eq!(descriptor.from.as_ref(), Some(from));
                    assert_eq!(descriptor.to.as_ref(), Some(to));
                    assert!(descriptor.liability.is_none());
                    assert_eq!(
                        descriptor.kind,
                        if matches!(effect, Effect::Fee { .. }) {
                            "fee"
                        } else {
                            "transfer"
                        }
                    );
                }
                Effect::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    ..
                } => {
                    let identity = descriptor.liability.as_ref().unwrap();
                    assert_eq!(identity.name, *name);
                    assert_eq!(identity.asset, *asset);
                    assert_eq!(identity.debtor, *debtor);
                    assert_eq!(identity.creditor, *creditor);
                    assert_eq!(descriptor.kind, "liability");
                    assert_eq!(descriptor.public_positions.len(), 4);
                    assert_eq!(
                        descriptor.operation.unwrap(),
                        if action == "fund" {
                            mori_backend::LiabilityOperation::CreateOrAccrue
                        } else {
                            mori_backend::LiabilityOperation::Discharge
                        }
                    );
                }
                Effect::State { name, .. } => {
                    assert_eq!(descriptor.kind, "state");
                    assert_eq!(descriptor.name.as_ref(), Some(name));
                }
                Effect::Emit { name, .. } => {
                    assert_eq!(descriptor.kind, "emit");
                    assert_eq!(descriptor.name.as_ref(), Some(name));
                }
            }
        }
        assert!(check(c, &public_values(c, &s, &e)), "honest {action}");
    }
}

#[test]
fn every_closing_effect_state_balance_and_duty_public_value_is_constrained() {
    let p = parse(SOURCE).unwrap();
    let a = artifact(&p);
    for action in ["fund", "partial", "complete", "recover"] {
        let s = fixture(action);
        let e = simulate(&p, action, &s).unwrap();
        let c = circuit(&a, action);
        let values = public_values(c, &s, &e);
        assert!(check(c, &values));
        for (i, b) in c
            .public_inputs
            .iter()
            .enumerate()
            .filter(|(_, b)| b.role == "effect" || b.role.starts_with("post_"))
        {
            let mut changed = values.clone();
            changed[i] = if b.ty == Type::Bool {
                Fr::from(changed[i] == Fr::from(0u64))
            } else {
                changed[i] + Fr::from(1u64)
            };
            assert!(
                !check(c, &changed),
                "{action}: changed {} {}",
                b.role,
                b.name
            );
        }
    }
}

#[test]
fn compiler_refuses_forged_terminal_phase_receipts_recipient_and_clock() {
    let p = parse(SOURCE).unwrap();
    let a = artifact(&p);
    for action in ["partial", "complete", "recover"] {
        let s = fixture(action);
        let e = simulate(&p, action, &s).unwrap();
        let c = circuit(&a, action);
        let values = public_values(c, &s, &e);
        for (role, name, amount) in [
            ("pre_state", "phase", 4),
            ("pre_state", "terminal", 1),
            ("input", "recipient", 2),
            (
                "input",
                if action == "recover" {
                    "unusedDocument"
                } else {
                    "successDocument"
                },
                0,
            ),
            ("input", "now", if action == "recover" { 100 } else { 101 }),
        ] {
            let mut changed = values.clone();
            replace(c, &mut changed, role, name, amount);
            assert!(!check(c, &changed), "{action}: {role} {name}");
        }
    }
}

#[test]
fn recovery_cannot_erase_delivery_or_replenish_gross_budget_in_compiled_frame() {
    let p = parse(SOURCE).unwrap();
    let a = artifact(&p);
    let s = fixture("recover");
    let e = simulate(&p, "recover", &s).unwrap();
    let c = circuit(&a, "recover");
    let values = public_values(c, &s, &e);
    assert!(check(c, &values));
    // Change both the recovery duty effect and closing duty, so the check must
    // preserve the unfulfilled duty rather than merely compare two outputs.
    let mut erased = values.clone();
    replace(c, &mut erased, "post_liability_amount", "delivery", 0);
    let effect = c
        .effect_frame
        .iter()
        .find(|d| d.liability.as_ref().is_some_and(|l| l.name == "delivery"))
        .unwrap();
    erased[effect.public_positions[3] as usize] = Fr::from(0u64);
    assert!(!check(c, &erased));
    let mut allowance_restored = values.clone();
    replace(c, &mut allowance_restored, "post_state", "grossPayment", 7);
    assert!(!check(c, &allowance_restored));
}

#[test]
fn coherent_excess_delivery_frame_refuses_completion() {
    let p = parse(SOURCE).unwrap();
    let a = artifact(&p);
    let s = fixture("complete");
    let e = simulate(&p, "complete", &s).unwrap();
    let c = circuit(&a, "complete");
    let mut values = public_values(c, &s, &e);
    replace(c, &mut values, "pre_liability_amount", "delivery", 8);
    replace(c, &mut values, "post_liability_amount", "delivery", 1);
    let effect = c
        .effect_frame
        .iter()
        .find(|d| d.liability.as_ref().is_some_and(|l| l.name == "delivery"))
        .unwrap();
    values[effect.public_positions[1] as usize] = Fr::from(8u64);
    values[effect.public_positions[3] as usize] = Fr::from(1u64);
    assert!(
        !check(c, &values),
        "coherent excess debt must violate the escrow guard"
    );
}

#[test]
fn duty_reads_follow_statement_wires_and_preserve_captured_values() {
    let p = parse(
        r#"language "moriarty/1"; agreement DutyReads {
      asset A = { network: "local", issuer: "i", reference: "A", scale: 0 };
      state x: UInt = 0;
      action read() {
        requires not liability_present("d") and liability_amount("d") == 0;
        liability("d", A, "debtor", "creditor", 7);
        let saved = liability_amount("d");
        discharge("d", 3);
        next x = liability_amount("d");
        ensures x == 4 and saved == 7 and liability_amount("d") == 4;
        discharge("d", liability_amount("d"));
        ensures liability_present("d") and liability_amount("d") == 0;
      }
    }"#,
    )
    .unwrap();
    let a = artifact(&p);
    let s = Scenario::default();
    let e = simulate(&p, "read", &s).unwrap();
    let c = circuit(&a, "read");
    assert!(check(c, &public_values(c, &s, &e)));
    let mut tombstone = public_values(c, &s, &e);
    replace(c, &mut tombstone, "pre_liability_presence", "d", 1);
    let first = c
        .effect_frame
        .iter()
        .find(|d| d.liability.is_some())
        .unwrap();
    tombstone[first.public_positions[0] as usize] = Fr::from(1u64);
    assert!(!check(c, &tombstone));
}

#[test]
fn coherent_duty_shortfall_excess_and_absence_refuse_all_escrow_continuations() {
    let p = parse(SOURCE).unwrap();
    let a = artifact(&p);
    for action in ["partial", "complete", "recover"] {
        let s = fixture(action);
        let e = simulate(&p, action, &s).unwrap();
        let c = circuit(&a, action);
        for name in ["delivery", "refund"] {
            let pre = e
                .pre
                .liabilities
                .iter()
                .find(|l| l.name == name)
                .unwrap()
                .amount
                .0;
            let post = e
                .post
                .liabilities
                .iter()
                .find(|l| l.name == name)
                .unwrap()
                .amount
                .0;
            let effect = c
                .effect_frame
                .iter()
                .find(|d| d.liability.as_ref().is_some_and(|l| l.name == name))
                .unwrap();
            for delta in [-1i128, 1] {
                // Underflow already refuses at discharge when the closing amount is zero.
                if delta < 0 && post == 0 {
                    continue;
                }
                let mut v = public_values(c, &s, &e);
                let before = (pre as i128 + delta) as u128;
                let after = (post as i128 + delta) as u128;
                replace(c, &mut v, "pre_liability_amount", name, before);
                replace(c, &mut v, "post_liability_amount", name, after);
                v[effect.public_positions[1] as usize] = Fr::from(before);
                v[effect.public_positions[3] as usize] = Fr::from(after);
                assert!(
                    !check(c, &v),
                    "{action}/{name}/{delta}: coherent mismatch accepted"
                );
            }
        }
    }
}

#[test]
fn escrow_read_costs_reuse_the_existing_public_abi() {
    let unguarded = SOURCE
        .lines()
        .filter(|line| !line.contains("liability_present(") && !line.contains("liability_amount("))
        .collect::<Vec<_>>()
        .join("\n");
    let before = artifact(&parse(&unguarded).unwrap());
    let after = artifact(&parse(SOURCE).unwrap());
    assert_eq!(
        before.resources.public_values,
        after.resources.public_values
    );
    assert_eq!(
        before.resources.liability_registry_entries,
        after.resources.liability_registry_entries
    );
    assert!(after.resources.instructions > before.resources.instructions);
    assert!(!after.resources.cost_measured);
    println!(
        "escrow resources before={} after={}",
        serde_json::to_string(&before.resources).unwrap(),
        serde_json::to_string(&after.resources).unwrap()
    );
}

#[test]
fn compiled_reads_distinguish_missing_zero_and_exact_uint128_amounts() {
    let p = parse(
        r#"language "moriarty/1"; agreement ExactRead {
      asset A = { network: "local", issuer: "i", reference: "A", scale: 0 };
      action declare() { liability("d", A, "debtor", "creditor", 0); }
      action read(expected: UInt, present: Bool) {
        requires liability_present("d") == present;
        requires liability_amount("d") == expected;
      }
    }"#,
    )
    .unwrap();
    let a = artifact(&p);
    let c = circuit(&a, "read");
    for (present, n) in [
        (false, 0),
        (true, 0),
        (true, 1),
        (true, 7),
        (true, u128::MAX),
    ] {
        let s = Scenario {
            inputs: [
                ("present".into(), Value::Bool(present)),
                ("expected".into(), Value::UInt(UInt(n))),
            ]
            .into(),
            liabilities: if present {
                vec![Liability {
                    name: "d".into(),
                    asset: p.assets[0].identity.clone(),
                    debtor: "debtor".into(),
                    creditor: "creditor".into(),
                    amount: UInt(n),
                }]
            } else {
                vec![]
            },
            ..Scenario::default()
        };
        let e = simulate(&p, "read", &s).unwrap();
        let values = public_values(c, &s, &e);
        assert!(check(c, &values));
        let mut forged = values.clone();
        replace(c, &mut forged, "input", "present", u128::from(!present));
        assert!(!check(c, &forged));
    }
}
