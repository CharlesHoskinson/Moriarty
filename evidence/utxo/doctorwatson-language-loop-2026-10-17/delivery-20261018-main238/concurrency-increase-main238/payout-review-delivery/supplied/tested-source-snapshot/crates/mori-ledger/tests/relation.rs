mod support;

use midnight_transient_crypto::{
    curve::Fr,
    proofs::{KeyLocation, ProofPreimage},
};
use mori_core::{Scenario, UInt, Value};
use mori_ledger::{codec, prepare::prepare_call, relation};
use std::borrow::Cow;

fn native_preimage(f: &support::Fixture) -> ProofPreimage {
    let b = prepare_call(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
        &f.request,
    )
    .unwrap();
    let c: serde_json::Value = serde_json::from_slice(&b.leaves["call.json"]).unwrap();
    let fields = |name: &str| {
        c[name]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| Fr::from_le_bytes(&codec::unhex(v.as_str().unwrap()).unwrap()).unwrap())
            .collect()
    };
    ProofPreimage {
        inputs: fields("preimage_inputs"),
        private_transcript: vec![],
        public_transcript_inputs: fields("public_transcript_inputs"),
        public_transcript_outputs: fields("public_transcript_outputs"),
        binding_input: codec::from_hex(c["binding_input_hex"].as_str().unwrap()).unwrap(),
        communications_commitment: Some((
            codec::from_hex(c["communication_commitment_hex"].as_str().unwrap()).unwrap(),
            codec::from_hex(c["communication_randomness_hex"].as_str().unwrap()).unwrap(),
        )),
        key_location: KeyLocation(Cow::Owned(f.request.action.clone())),
    }
}

fn state_fixture(
    source: &str,
    inputs: &[(&str, Value)],
    state: &[(&str, Value)],
) -> support::Fixture {
    let s = Scenario {
        inputs: inputs
            .iter()
            .map(|(n, v)| (n.to_string(), v.clone()))
            .collect(),
        state: state
            .iter()
            .map(|(n, v)| (n.to_string(), v.clone()))
            .collect(),
        ..Default::default()
    };
    support::fixture(source, "run", s)
}

#[test]
fn one_artifact_checks_mutable_prefix_and_captured_let_across_native_instances() {
    let source = r#"language "moriarty/1"; agreement Prefix {
      state count: UInt = 0; state enabled: Bool = true;
      action run(amount: UInt) {
        let old = enabled; let before = count; next count = count + amount;
        next enabled = not enabled; requires old; ensures count == before + amount;
      }
    }"#;
    let first = state_fixture(source, &[("amount", Value::UInt(UInt(3)))], &[]);
    let second = state_fixture(
        source,
        &[("amount", Value::UInt(UInt(9)))],
        &[("count", Value::UInt(UInt(4)))],
    );
    let a = relation::compile(&first.source, &first.program, &first.deployment, "run").unwrap();
    let other =
        relation::compile(&second.source, &second.program, &second.deployment, "run").unwrap();
    assert_eq!(a.ir_bytes().unwrap(), other.ir_bytes().unwrap());
    assert_eq!(a.manifest_bytes().unwrap(), other.manifest_bytes().unwrap());
    relation::check(&a, &native_preimage(&first)).unwrap();
    relation::check(&a, &native_preimage(&second)).unwrap();
    for (f, expected) in [(&first, 3), (&second, 13)] {
        let b = prepare_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
        )
        .unwrap();
        assert_eq!(
            b.readback.execution.post.state["count"],
            Value::UInt(UInt(expected))
        );
        assert_eq!(
            b.readback.execution.post.state["enabled"],
            Value::Bool(false)
        );
    }
    let mut false_prestate = native_preimage(&first);
    // Correlate all opening copies and Not(false)=true writes. This must fail
    // the source requires(old), rather than merely a split transcript binding.
    let b = prepare_call(
        &first.source,
        &first.program,
        &first.deployment,
        &first.predecessor,
        &first.parameters,
        &first.request,
    )
    .unwrap();
    let mut ops: Vec<mori_ledger::ops::NativeOp> =
        codec::decode_artifact(&b.leaves["ops.bin"]).unwrap();
    use midnight_onchain_state::state::StateValue;
    use midnight_onchain_vm::ops::Op;
    use midnight_storage::arena::Sp;
    for op in &mut ops {
        match op {
            Op::Popeq { result, .. } if *result == true.into() => *result = false.into(),
            Op::Push {
                value: StateValue::Cell(v),
                ..
            } if **v == false.into() => *v = Sp::new(true.into()),
            _ => {}
        }
    }
    replace_transcript(&mut false_prestate, &ops);
    assert!(relation::check(&a, &false_prestate).is_err());
}

fn replace_transcript(p: &mut ProofPreimage, ops: &[mori_ledger::ops::NativeOp]) {
    use midnight_transient_crypto::{fab::AlignedValueExt, repr::FieldRepr};
    p.public_transcript_inputs = ops.iter().flat_map(FieldRepr::field_vec).collect();
    p.public_transcript_outputs.clear();
    for op in ops {
        if let midnight_onchain_vm::ops::Op::Popeq { result, .. } = op {
            result.value_only_field_repr(&mut p.public_transcript_outputs);
        }
    }
}

#[test]
fn checked_multiplication_refuses_the_literal_modular_wrap_counterfeit() {
    let source = r#"language "moriarty/1"; agreement Product {
      state product: UInt = 0; action run(a: UInt, b: UInt) { next product = a * b; }
    }"#;
    let f = state_fixture(
        source,
        &[("a", Value::UInt(UInt(1))), ("b", Value::UInt(UInt(1)))],
        &[],
    );
    let a = relation::compile(&f.source, &f.program, &f.deployment, "run").unwrap();
    relation::check(&a, &native_preimage(&f)).unwrap();
    let mut bad = native_preimage(&f);
    bad.inputs = vec![
        Fr::from(170141183460469231731687303715884105728u128),
        Fr::from(308190375243917312857644309053803048971u128),
    ];
    let rand = bad.communications_commitment.unwrap().1;
    bad.communications_commitment = Some((
        midnight_transient_crypto::hash::transient_commit(&bad.inputs[..], rand),
        rand,
    ));
    // A raw field multiplication maps this overflowing product into UInt128.
    let forged = Fr::from(58830589151200628854506063105544421375u128);
    assert_eq!(bad.inputs[0] * bad.inputs[1], forged);
    let last = bad.public_transcript_inputs.len() - 2; // State write Push payload precedes Ins.
    bad.public_transcript_inputs[last] = forged;
    assert!(relation::check(&a, &bad).is_err());
}

#[test]
fn source_claims_cover_zero_first_repeated_and_wide_amounts() {
    for (body, budget, expected) in [
        (
            "transfer(QA,\"alice\",\"bob\",0); transfer(QA,\"alice\",\"bob\",10); fee(QA,\"alice\",\"bob\",2);",
            100,
            12,
        ),
        (
            "transfer(QA,\"alice\",\"alice\",10); transfer(QA,\"alice\",\"alice\",2);",
            100,
            12,
        ),
        (
            "transfer(QA,\"alice\",\"bob\",18446744073709551616); transfer(QA,\"alice\",\"bob\",18446744073709551616);",
            1u128 << 65,
            1u128 << 65,
        ),
    ] {
        let f = support::cash(body, budget);
        let a = relation::compile(&f.source, &f.program, &f.deployment, &f.request.action).unwrap();
        let good = native_preimage(&f);
        let activity = relation::check(&a, &good).unwrap();
        assert!(activity.iter().any(Option::is_some)); // The durable skeleton includes omitted alternatives.
        let b = prepare_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
        )
        .unwrap();
        assert_eq!(
            b.readback
                .effects
                .claimed_unshielded_spends
                .iter()
                .map(|e| *e.1)
                .collect::<Vec<_>>(),
            vec![expected]
        );
        // Correlate every copy of native token/recipient key field limbs. The
        // trusted source/static binding remains fixed, so each must refuse.
        use midnight_coin_structure::coin::{
            PublicAddress, TokenType, UnshieldedTokenType, UserAddress,
        };
        use midnight_onchain_runtime::context::ClaimedUnshieldedSpendsKey;
        use midnight_transient_crypto::fab::AlignedValueExt;
        let token: UnshieldedTokenType =
            codec::from_hex(&f.deployment.assets[0].token_hex).unwrap();
        let role = if body.contains("\"bob\"") {
            "bob"
        } else {
            "alice"
        };
        let address: UserAddress = codec::from_hex(
            &f.deployment
                .roles
                .iter()
                .find(|r| r.role == role)
                .unwrap()
                .address_hex,
        )
        .unwrap();
        let key = ClaimedUnshieldedSpendsKey::from_inner(
            TokenType::Unshielded(token),
            PublicAddress::User(address),
        );
        let mut key_fields = vec![];
        midnight_base_crypto::fab::AlignedValue::from(key).value_only_field_repr(&mut key_fields);
        let mut controls = 0;
        for original in key_fields
            .into_iter()
            .filter(|f| f.as_le_bytes()[2..].iter().any(|b| *b != 0))
        {
            assert!(good.public_transcript_inputs.contains(&original));
            let mut bad = good.clone();
            for v in bad
                .public_transcript_inputs
                .iter_mut()
                .chain(bad.public_transcript_outputs.iter_mut())
            {
                if *v == original {
                    *v = original + Fr::from(1);
                }
            }
            assert!(relation::check(&a, &bad).is_err());
            controls += 1;
        }
        assert!(controls >= 2);
        // Change every published new/old total carrying the final amount; the
        // source sum still has its original operands and must reject.
        let mut bad = good.clone();
        let mut program: Vec<mori_ledger::ops::NativeOp> =
            codec::decode_artifact(&b.leaves["ops.bin"]).unwrap();
        for op in &mut program {
            match op {
                midnight_onchain_vm::ops::Op::Popeq { result, .. }
                    if *result == expected.into() =>
                {
                    *result = (expected + 1).into()
                }
                midnight_onchain_vm::ops::Op::Push {
                    value: midnight_onchain_state::state::StateValue::Cell(v),
                    ..
                } if **v == expected.into() => {
                    *v = midnight_storage::arena::Sp::new((expected + 1).into())
                }
                _ => {}
            }
        }
        replace_transcript(&mut bad, &program);
        assert!(relation::check(&a, &bad).is_err());
    }
}

#[test]
fn typed_event_hash_is_source_computed_for_all_normalization_boundaries() {
    let source = r#"language "moriarty/1"; agreement Event {
      action settle(value: UInt) { emit "paid"(value); }
    }"#;
    let fixture = |value| {
        support::fixture(
            source,
            "settle",
            Scenario {
                inputs: [("value".into(), Value::UInt(UInt(value)))]
                    .into_iter()
                    .collect(),
                ..Default::default()
            },
        )
    };
    let first = fixture(0);
    let a = relation::compile(&first.source, &first.program, &first.deployment, "settle").unwrap();
    for value in [
        0,
        1,
        31,
        32,
        70,
        255,
        256,
        65535,
        65536,
        1u128 << 64,
        u128::MAX,
    ] {
        let f = fixture(value);
        let other = relation::compile(&f.source, &f.program, &f.deployment, "settle").unwrap();
        assert_eq!(a.ir_bytes().unwrap(), other.ir_bytes().unwrap());
        assert_eq!(a.manifest_bytes().unwrap(), other.manifest_bytes().unwrap());
        let good = native_preimage(&f);
        let activity = relation::check(&a, &good).unwrap();
        assert_eq!(activity.len(), 37);
        assert_eq!(activity.iter().filter(|x| x.is_none()).count(), 5);
        assert_eq!(activity.iter().flatten().sum::<usize>(), 240);
        assert_eq!(good.public_transcript_inputs.len(), 21);
        if value == 70 {
            assert_eq!(
                good.public_transcript_inputs[19],
                Fr::from_le_bytes(
                    &codec::unhex(
                        "34a5480b183a7cce420af3bfaa0a6287532ed340476d86a9d90d828b8c5da811"
                    )
                    .unwrap()
                )
                .unwrap()
            );
        }
        let mut bad = good.clone();
        bad.public_transcript_inputs[19] = bad.public_transcript_inputs[19] + Fr::from(1);
        assert!(relation::check(&a, &bad).is_err());
    }
    let bool_source = r#"language "moriarty/1"; agreement BooleanEvent { action run(value: Bool) { emit "flag"(value); } }"#;
    for v in [false, true] {
        let f = state_fixture(bool_source, &[("value", Value::Bool(v))], &[]);
        let a = relation::compile(&f.source, &f.program, &f.deployment, "run").unwrap();
        let activity = relation::check(&a, &native_preimage(&f)).unwrap();
        assert_eq!(activity, vec![None; 5]);
    }
}

#[test]
fn duties_accrue_and_discharge_under_present_and_absent_native_openings() {
    let source = r#"language "moriarty/1"; agreement Duty {
      asset QA = { network:"local", issuer:"issuer", reference:"QA", scale:0 };
      action run(amount: UInt) {
        liability("note",QA,"alice","bob",amount); discharge("note",1);
        liability("note",QA,"alice","bob",0);
      }
    }"#;
    let first = state_fixture(source, &[("amount", Value::UInt(UInt(2)))], &[]);
    let mut scenario = Scenario::default();
    scenario
        .inputs
        .insert("amount".into(), Value::UInt(UInt(3)));
    scenario.liabilities.push(mori_core::Liability {
        name: "note".into(),
        asset: first.program.assets[0].identity.clone(),
        debtor: "alice".into(),
        creditor: "bob".into(),
        amount: UInt(5),
    });
    let second = support::fixture(source, "run", scenario);
    let a = relation::compile(&first.source, &first.program, &first.deployment, "run").unwrap();
    let other =
        relation::compile(&second.source, &second.program, &second.deployment, "run").unwrap();
    assert_eq!(a.ir_bytes().unwrap(), other.ir_bytes().unwrap());
    assert_eq!(a.manifest_bytes().unwrap(), other.manifest_bytes().unwrap());
    relation::check(&a, &native_preimage(&first)).unwrap();
    relation::check(&a, &native_preimage(&second)).unwrap();
    let mut identity_forgery = native_preimage(&second);
    let original = identity_forgery.public_transcript_outputs[2];
    assert_ne!(original, Fr::from(5));
    let changed = original + Fr::from(1);
    for value in identity_forgery
        .public_transcript_inputs
        .iter_mut()
        .chain(identity_forgery.public_transcript_outputs.iter_mut())
    {
        if *value == original {
            *value = changed;
        }
    }
    assert!(relation::check(&a, &identity_forgery).is_err());

    for (f, total) in [(&first, 1u128), (&second, 7u128)] {
        let b = prepare_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
        )
        .unwrap();
        assert_eq!(b.readback.execution.post.liabilities[0].amount, UInt(total));
    }
}

#[test]
fn actual_processed_call_matches_literal_event_noop_and_public_input_counts() {
    let source = r#"language "moriarty/1"; agreement ProcessedEvent { action run(value: UInt) { emit "value"(value); } }"#;
    for (value, noops, ops) in [(0, 1, 6), (256, 2, 7), (u128::MAX, 1, 6)] {
        let f = state_fixture(source, &[("value", Value::UInt(UInt(value)))], &[]);
        let a = relation::compile(&f.source, &f.program, &f.deployment, "run").unwrap();
        let b = mori_ledger::prepare::prepare_call_checked(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
            &a,
        )
        .unwrap();
        let r = b.readback.relation.as_ref().unwrap();
        assert_eq!(
            (
                r.raw_op_count,
                r.processed_op_count,
                r.noop_count,
                r.inactive_fields
            ),
            (5, ops, noops, 240)
        );
        assert_eq!(r.public_inputs.len(), 263);
        assert_eq!(
            b.readback.evidence.qualification,
            "local-relation-checked-unproven"
        );
        assert_eq!(b.readback.evidence.proof_verification, "not-performed");
        assert!(b.readback.requires_reconstruction_after_final_intent);
        assert!(r.processed_call_use.starts_with("inspection-only"));
        type Call = midnight_ledger::structure::ContractCall<
            midnight_ledger::structure::ProofPreimageMarker,
            midnight_storage::db::InMemoryDB,
        >;
        let processed: Call = codec::decode_artifact(&b.leaves["processed-call.bin"]).unwrap();
        let raw: Call = codec::decode_artifact(&b.leaves["raw-call.bin"]).unwrap();
        let bound: ProofPreimage =
            codec::decode_artifact(&b.leaves["processed-preimage.bin"]).unwrap();
        let midnight_ledger::structure::ProofPreimageVersioned::V2(embedded) = processed.proof
        else {
            panic!("unexpected preimage version")
        };
        assert_eq!(embedded.binding_input, bound.binding_input);
        assert_eq!(
            embedded.public_transcript_inputs,
            bound.public_transcript_inputs
        );
        let midnight_ledger::structure::ProofPreimageVersioned::V2(raw_preimage) = raw.proof else {
            panic!("unexpected preimage version")
        };
        assert_eq!(raw_preimage.binding_input, Fr::from(0));
        assert_ne!(embedded.binding_input, raw_preimage.binding_input);
    }
    let source = r#"language "moriarty/1"; agreement AdjacentEvents { action run(a: UInt,b: UInt) { emit "first"(a); emit "second"(b); } }"#;
    let f = state_fixture(
        source,
        &[
            ("a", Value::UInt(UInt(0))),
            ("b", Value::UInt(UInt(u128::MAX))),
        ],
        &[],
    );
    let a = relation::compile(&f.source, &f.program, &f.deployment, "run").unwrap();
    let b = mori_ledger::prepare::prepare_call_checked(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
        &f.request,
        &a,
    )
    .unwrap();
    let r = b.readback.relation.as_ref().unwrap();
    assert_eq!(
        (
            r.raw_op_count,
            r.processed_op_count,
            r.noop_count,
            r.inactive_fields
        ),
        (7, 8, 1, 480)
    );
    assert_eq!(r.public_inputs.len(), 518);
}

#[test]
fn malformed_vectors_fail_closed_and_legitimate_context_changes_remain_valid() {
    let f = state_fixture(
        r#"language "moriarty/1"; agreement Identity { state n: UInt=0; action run(value: UInt) { next n=value; } }"#,
        &[("value", Value::UInt(UInt(1)))],
        &[],
    );
    let a = relation::compile(&f.source, &f.program, &f.deployment, "run").unwrap();
    let good = native_preimage(&f);
    for kind in 0..6 {
        let mut bad = good.clone();
        match kind {
            0 => bad.inputs.clear(),
            1 => bad.private_transcript.push(Fr::from(1)),
            2 => {
                bad.public_transcript_outputs.clear();
            }
            3 => {
                bad.public_transcript_inputs.pop();
            }
            4 => bad.public_transcript_outputs.push(Fr::from(1)),
            _ => bad.key_location = KeyLocation(Cow::Borrowed("other")),
        }
        assert!(relation::check(&a, &bad).is_err(), "malformed case {kind}");
    }
    let mut alternate = good.clone();
    alternate.binding_input = Fr::from(123);
    relation::check(&a, &alternate).unwrap(); // Arbitrary first instance field, not cryptographic verification.
    let rand = Fr::from(7);
    alternate.communications_commitment = Some((
        midnight_transient_crypto::hash::transient_commit(&alternate.inputs[..], rand),
        rand,
    ));
    relation::check(&a, &alternate).unwrap();
    alternate.communications_commitment.as_mut().unwrap().1 = Fr::from(8);
    assert!(relation::check(&a, &alternate).is_err());
    let activity = relation::check(&a, &good).unwrap();
    let b = prepare_call(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
        &f.request,
    )
    .unwrap();
    let ops: Vec<mori_ledger::ops::NativeOp> =
        codec::decode_artifact(&b.leaves["ops.bin"]).unwrap();
    let params: midnight_ledger::structure::LedgerParameters =
        codec::decode_native(&f.parameters).unwrap();
    let mut missing = activity.clone();
    missing.pop();
    assert!(relation::process(&a, &ops, &missing, &params.cost_model.runtime_cost_model).is_err());
    let mut forged = activity;
    forged[0] = Some(2);
    assert!(relation::process(&a, &ops, &forged, &params.cost_model.runtime_cost_model).is_err());
}

#[test]
fn correlated_late_ensures_failure_and_discarded_eager_overflow_reject() {
    let source = r#"language "moriarty/1"; agreement Late { state n: UInt=0; action run(value: UInt) { next n=n+value; ensures n<2; } }"#;
    let f = state_fixture(source, &[("value", Value::UInt(UInt(1)))], &[]);
    let a = relation::compile(&f.source, &f.program, &f.deployment, "run").unwrap();
    let mut bad = native_preimage(&f);
    let b = prepare_call(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
        &f.request,
    )
    .unwrap();
    let mut ops: Vec<mori_ledger::ops::NativeOp> =
        codec::decode_artifact(&b.leaves["ops.bin"]).unwrap();
    for op in &mut ops {
        match op {
            midnight_onchain_vm::ops::Op::Popeq { result, .. } if *result == 1u128.into() => {
                *result = 2u128.into()
            }
            midnight_onchain_vm::ops::Op::Push {
                value: midnight_onchain_state::state::StateValue::Cell(v),
                ..
            } if **v == 1u128.into() => *v = midnight_storage::arena::Sp::new(2u128.into()),
            _ => {}
        }
    }
    bad.inputs[0] = Fr::from(2);
    let rand = bad.communications_commitment.unwrap().1;
    bad.communications_commitment = Some((
        midnight_transient_crypto::hash::transient_commit(&bad.inputs[..], rand),
        rand,
    ));
    replace_transcript(&mut bad, &ops);
    assert!(relation::check(&a, &bad).is_err());
    let source = r#"language "moriarty/1"; agreement Eager { action run(a: UInt,b: UInt) { let discarded=a*b; requires false or true; } }"#;
    let f = state_fixture(
        source,
        &[("a", Value::UInt(UInt(1))), ("b", Value::UInt(UInt(1)))],
        &[],
    );
    let a = relation::compile(&f.source, &f.program, &f.deployment, "run").unwrap();
    let mut bad = native_preimage(&f);
    bad.inputs = vec![Fr::from(1u128 << 127), Fr::from(2)];
    let rand = bad.communications_commitment.unwrap().1;
    bad.communications_commitment = Some((
        midnight_transient_crypto::hash::transient_commit(&bad.inputs[..], rand),
        rand,
    ));
    assert!(relation::check(&a, &bad).is_err());
}

#[test]
fn event_emission_domain_preserves_shorter_instances_and_refuses_oversized_branches() {
    use midnight_base_crypto::fab::AlignedValue;
    use midnight_onchain_state::state::StateValue;
    use midnight_onchain_vm::ops::{LogEventType, Op};
    use midnight_storage::arena::Sp;
    let admitted = |name: &str, value: Value| {
        let bytes = mori_ledger::ops::event_payload(name, "run", &value).unwrap();
        Sp::new(mori_ledger::ops::cell(AlignedValue::from(bytes)))
            .serialize_to_node_list_bounded(midnight_onchain_vm::vm::MAX_LOG_EMITTED)
            .is_some()
    };
    // Find the pinned SDK node-list boundary, rather than assuming raw bytes
    // have the same size as their native serialized storage representation.
    let n = (0..1100)
        .find(|n| {
            admitted(&"x".repeat(*n), Value::UInt(UInt(0)))
                && !admitted(&"x".repeat(*n), Value::UInt(UInt(u128::MAX)))
        })
        .unwrap();
    let name = "x".repeat(n);
    let source = format!(
        "language \"moriarty/1\"; agreement Size {{ action run(value: UInt) {{ emit \"{name}\"(value); }} }}"
    );
    let f = state_fixture(&source, &[("value", Value::UInt(UInt(0)))], &[]);
    let a = relation::compile(&f.source, &f.program, &f.deployment, "run").unwrap();
    let mut bad = native_preimage(&f);
    relation::check(&a, &bad).unwrap();
    let b = prepare_call(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
        &f.request,
    )
    .unwrap();
    let mut ops: Vec<mori_ledger::ops::NativeOp> =
        codec::decode_artifact(&b.leaves["ops.bin"]).unwrap();
    let payload =
        mori_ledger::ops::event_payload(&name, "run", &Value::UInt(UInt(u128::MAX))).unwrap();
    for op in &mut ops {
        if let Op::Push {
            value: StateValue::Array(v),
            ..
        } = op
            && v.len() == 3
        {
            *op = Op::Push {
                storage: false,
                value: StateValue::Array(
                    vec![
                        mori_ledger::ops::cell(1u32.into()),
                        mori_ledger::ops::cell((LogEventType::Misc as u8).into()),
                        mori_ledger::ops::cell(AlignedValue::from(payload.clone())),
                    ]
                    .into(),
                ),
            };
        }
    }
    bad.inputs[0] = Fr::from(u128::MAX);
    let rand = bad.communications_commitment.unwrap().1;
    bad.communications_commitment = Some((
        midnight_transient_crypto::hash::transient_commit(&bad.inputs[..], rand),
        rand,
    ));
    replace_transcript(&mut bad, &ops);
    assert!(relation::check(&a, &bad).is_err());
    let invalid = state_fixture(&source, &[("value", Value::UInt(UInt(u128::MAX)))], &[]);
    assert!(
        prepare_call(
            &invalid.source,
            &invalid.program,
            &invalid.deployment,
            &invalid.predecessor,
            &invalid.parameters,
            &invalid.request
        )
        .is_err()
    );
    let n = (0..1100)
        .find(|n| !admitted(&"x".repeat(*n), Value::Bool(false)))
        .unwrap();
    for (len, valid) in [(n - 1, true), (n, false)] {
        let name = "x".repeat(len);
        let source = format!(
            "language \"moriarty/1\"; agreement BoolSize {{ action run(value: Bool) {{ emit \"{name}\"(value); }} }}"
        );
        let f = state_fixture(&source, &[("value", Value::Bool(false))], &[]);
        let a = relation::compile(&f.source, &f.program, &f.deployment, "run");
        assert_eq!(a.is_ok(), valid);
        if let Ok(a) = a {
            relation::check(&a, &native_preimage(&f)).unwrap();
        }
    }
}

#[test]
fn checked_arithmetic_boundaries_and_typed_inputs_match_core_and_refuse_correlated_wraps() {
    for (operator, a, b, result) in [
        ("+", u128::MAX - 1, 1, u128::MAX),
        ("-", 0, 0, 0),
        (
            "*",
            (1u128 << 64) - 1,
            (1u128 << 64) - 1,
            ((1u128 << 64) - 1) * ((1u128 << 64) - 1),
        ),
        ("*", 1u128 << 64, 1u128 << 63, 1u128 << 127),
    ] {
        let source = format!(
            "language \"moriarty/1\"; agreement Arithmetic {{ state result: UInt=0; action run(a: UInt,b: UInt) {{ next result=a {operator} b; }} }}"
        );
        let f = state_fixture(
            &source,
            &[("a", Value::UInt(UInt(a))), ("b", Value::UInt(UInt(b)))],
            &[],
        );
        let artifact = relation::compile(&f.source, &f.program, &f.deployment, "run").unwrap();
        relation::check(&artifact, &native_preimage(&f)).unwrap();
        let bundle = prepare_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
        )
        .unwrap();
        assert_eq!(
            bundle.readback.execution.post.state["result"],
            Value::UInt(UInt(result))
        );
        if operator == "+" || operator == "-" {
            let mut bad = native_preimage(&f);
            bad.inputs = if operator == "+" {
                vec![Fr::from(u128::MAX), Fr::from(1)]
            } else {
                vec![Fr::from(0), Fr::from(1)]
            };
            let rand = bad.communications_commitment.unwrap().1;
            bad.communications_commitment = Some((
                midnight_transient_crypto::hash::transient_commit(&bad.inputs[..], rand),
                rand,
            ));
            let forged = if operator == "+" {
                Fr::from(0)
            } else {
                Fr::from(u128::MAX)
            };
            let last = bad.public_transcript_inputs.len() - 2;
            bad.public_transcript_inputs[last] = forged;
            assert!(relation::check(&artifact, &bad).is_err());
        }
    }
    let source = r#"language "moriarty/1"; agreement Comparisons { action run(a: UInt,b: UInt,flag: Bool) {
        requires a not = b and a<b and b>a and a<=b and b>=a and (not flag or flag==false);
    } }"#;
    let f = state_fixture(
        source,
        &[
            ("a", Value::UInt(UInt(1))),
            ("b", Value::UInt(UInt(2))),
            ("flag", Value::Bool(false)),
        ],
        &[],
    );
    let a = relation::compile(&f.source, &f.program, &f.deployment, "run").unwrap();
    let mut bad = native_preimage(&f);
    relation::check(&a, &bad).unwrap();
    bad.inputs[2] = Fr::from(2);
    let rand = bad.communications_commitment.unwrap().1;
    bad.communications_commitment = Some((
        midnight_transient_crypto::hash::transient_commit(&bad.inputs[..], rand),
        rand,
    ));
    assert!(relation::check(&a, &bad).is_err());
}
