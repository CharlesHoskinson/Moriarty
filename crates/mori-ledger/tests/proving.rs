//! Ordinary source checks only: this target never invokes native key/proof APIs.
mod support;
use midnight_ledger::structure::{Intent, ProofPreimageMarker, ProofPreimageVersioned};
use midnight_storage::db::InMemoryDB;
use midnight_transient_crypto::commitment::PedersenRandomness;
use mori_core::{AssetId, Balance, Effect, Liability, Scenario, Snapshot, UInt, Value};
use mori_ledger::{codec, prepare::prepare_native_call};

type RawIntent = Intent<(), ProofPreimageMarker, PedersenRandomness, InMemoryDB>;
const METERED: &str = r#"language "moriarty/1"; agreement MeteredRemittance {
 asset QA = {network:"local", issuer:"next-native-qa", reference:"QA", scale:0};
 state running: UInt = 0; state enabled: Bool = true;
 action settle(amount: UInt) {
 requires enabled and amount >= 3 and amount <= 20;
 let before = running; next running = running + amount; requires running >= before;
 transfer(QA,"alice","bob",amount); transfer(QA,"alice","bob",amount);
 fee(QA,"alice","treasury",2); liability("charge",QA,"alice","bob",amount);
 discharge("charge",3); emit "total"(running); emit "enabled"(enabled);
 ensures running == before + amount;
 }}"#;
fn metered(running: u128, amount: u128, charge: Option<u128>) -> support::Fixture {
    let p = mori_core::parse(METERED).unwrap();
    let asset = p.assets[0].identity.clone();
    let s = Scenario {
        inputs: [("amount".into(), Value::UInt(UInt(amount)))]
            .into_iter()
            .collect(),
        state: [("running".into(), Value::UInt(UInt(running)))]
            .into_iter()
            .collect(),
        balances: [("alice", 100), ("bob", 5), ("treasury", 1)]
            .into_iter()
            .map(|(account, n)| Balance {
                account: account.into(),
                asset: asset.clone(),
                amount: UInt(n),
            })
            .collect(),
        liabilities: charge
            .into_iter()
            .map(|n| Liability {
                name: "charge".into(),
                asset: asset.clone(),
                debtor: "alice".into(),
                creditor: "bob".into(),
                amount: UInt(n),
            })
            .collect(),
    };
    support::fixture(METERED, "settle", s)
}
#[test]
fn typed_preparation_owns_raw_intent_and_preserves_literal_metered_outcomes() {
    let mut identities = Vec::new();
    for (f, total, charge, alice, bob) in [
        (metered(4, 7, None), 11, 4, 84, 19),
        (metered(31, 3, Some(9)), 34, 9, 92, 11),
    ] {
        let prepared = prepare_native_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
        )
        .unwrap();
        let b = prepared.bundle();
        let raw: RawIntent = codec::decode_artifact(&b.leaves["raw-intent.bin"]).unwrap();
        assert_eq!(
            codec::encode(&raw).unwrap(),
            codec::encode(prepared.raw_intent()).unwrap()
        );
        let call = raw.calls().next().unwrap();
        let ProofPreimageVersioned::V2(preimage) = &call.proof else {
            panic!("unsupported raw preimage")
        };
        assert_eq!(preimage.binding_input, 0u64.into());
        assert_eq!(codec::encode(call).unwrap(), b.leaves["raw-call.bin"]);
        assert_ne!(b.leaves["raw-call.bin"], b.leaves["processed-call.bin"]);
        assert_eq!(
            b.readback.execution.post.state["running"],
            Value::UInt(UInt(total))
        );
        assert_eq!(
            b.readback.execution.post.state["enabled"],
            Value::Bool(true)
        );
        assert_eq!(b.readback.execution.post.liabilities.len(), 1);
        assert_eq!(
            b.readback.execution.post.liabilities[0].amount,
            UInt(charge)
        );
        for (name, n) in [("alice", alice), ("bob", bob), ("treasury", 3)] {
            assert_eq!(
                b.readback
                    .prospective_balances
                    .iter()
                    .find(|v| v.account == name)
                    .unwrap()
                    .amount,
                UInt(n)
            );
        }
        let asset = AssetId {
            network: "local".into(),
            issuer: "next-native-qa".into(),
            reference: "QA".into(),
            scale: 0,
        };
        let (before, amount, old_charge) = if total == 11 { (4, 7, 0) } else { (31, 3, 9) };
        let liability = |before, after| Effect::Liability {
            name: "charge".into(),
            asset: asset.clone(),
            debtor: "alice".into(),
            creditor: "bob".into(),
            before: UInt(before),
            after: UInt(after),
        };
        assert_eq!(
            b.readback.execution.effects,
            vec![
                Effect::State {
                    name: "running".into(),
                    before: Value::UInt(UInt(before)),
                    after: Value::UInt(UInt(total))
                },
                Effect::Transfer {
                    asset: asset.clone(),
                    from: "alice".into(),
                    to: "bob".into(),
                    amount: UInt(amount)
                },
                Effect::Transfer {
                    asset: asset.clone(),
                    from: "alice".into(),
                    to: "bob".into(),
                    amount: UInt(amount)
                },
                Effect::Fee {
                    asset: asset.clone(),
                    from: "alice".into(),
                    to: "treasury".into(),
                    amount: UInt(2)
                },
                liability(old_charge, old_charge + amount),
                liability(old_charge + amount, charge),
                Effect::Emit {
                    name: "total".into(),
                    value: Value::UInt(UInt(total))
                },
                Effect::Emit {
                    name: "enabled".into(),
                    value: Value::Bool(true)
                },
            ]
        );
        assert_eq!(
            b.readback.execution.post,
            Snapshot {
                state: [
                    ("running".into(), Value::UInt(UInt(total))),
                    ("enabled".into(), Value::Bool(true))
                ]
                .into_iter()
                .collect(),
                balances: [("alice", alice), ("bob", bob), ("treasury", 3)]
                    .into_iter()
                    .map(|(name, n)| Balance {
                        account: name.into(),
                        asset: asset.clone(),
                        amount: UInt(n)
                    })
                    .collect(),
                liabilities: vec![Liability {
                    name: "charge".into(),
                    asset: asset.clone(),
                    debtor: "alice".into(),
                    creditor: "bob".into(),
                    amount: UInt(charge)
                }],
            }
        );
        assert_eq!(b.readback.execution.steps.len(), 12);
        assert_eq!(
            b.readback.preserved_frame.commitments["complete_ledger"],
            f.deployment.predecessor_hash
        );
        assert_eq!(
            b.readback.preserved_frame.commitments["contract_operations"],
            f.deployment.operations_hash
        );
        assert_eq!(
            b.readback.preserved_frame.commitments["contract_maintenance"],
            f.deployment.maintenance_hash
        );
        assert_eq!(
            b.readback.preserved_frame.commitments["contract_balance"],
            f.deployment.balance_hash
        );
        let outputs: Vec<midnight_ledger::structure::Utxo> = b
            .readback
            .prospective_outputs_hex
            .iter()
            .map(|text| codec::from_hex(text).unwrap())
            .collect();
        assert_eq!(outputs.iter().map(|o| o.value).sum::<u128>(), 100);
        for (role, n) in [("bob", 2 * amount), ("treasury", 2), ("alice", alice)] {
            let owner = codec::from_hex(
                &f.deployment
                    .roles
                    .iter()
                    .find(|r| r.role == role)
                    .unwrap()
                    .address_hex,
            )
            .unwrap();
            assert_eq!(
                outputs
                    .iter()
                    .filter(|o| o.owner == owner)
                    .map(|o| o.value)
                    .sum::<u128>(),
                n
            );
        }

        assert_eq!(raw.calls().count(), 1);
        assert!(call.fallible_transcript.is_none());
        assert!(b.readback.requires_reconstruction_after_final_intent);
        assert_eq!(b.readback.evidence.proof_verification, "not-performed");
        let relation = b.readback.relation.as_ref().unwrap();
        assert_eq!(
            relation.public_inputs.len(),
            2 + prepared
                .artifact()
                .manifest
                .groups
                .iter()
                .map(|g| g.field_count)
                .sum::<usize>()
        );
        identities.push((
            prepared.artifact().ir_bytes().unwrap(),
            prepared.artifact().manifest_bytes().unwrap(),
            relation.activity.clone(),
        ));
    }
    assert_eq!(identities[0].0, identities[1].0);
    assert_eq!(identities[0].1, identities[1].1);
    assert_ne!(identities[0].2, identities[1].2);
}
#[test]
fn typed_preparation_refuses_changed_source_and_false_semantics() {
    let mut f = metered(4, 7, None);
    assert!(
        prepare_native_call(
            &(f.source.clone() + " "),
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request
        )
        .is_err()
    );
    f.request
        .inputs
        .insert("amount".into(), Value::UInt(UInt(2)));
    assert!(
        prepare_native_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request
        )
        .is_err()
    );
}

#[test]
fn actual_sdk_adapter_types_are_wired_without_native_execution() {
    fn provider<T: midnight_transient_crypto::proofs::ProvingProvider>() {}
    fn resolver<T: midnight_transient_crypto::proofs::Resolver>() {}
    fn reusable<T: Clone>() {}
    reusable::<mori_ledger::proving::DerivedCallMaterial>();
    provider::<mori_ledger::proving::SourceCallProver>();
    resolver::<mori_ledger::proving::SourceCallResolver>();
    // Merely compile the production verifier signature. Calling it is native work.
    let _verify: fn(
        &mori_ledger::prepare::PreparedNativeCall,
        &mori_ledger::proving::PublicDerivedCallKey,
        &midnight_onchain_state::state::ContractOperation,
        &mori_ledger::proving::ProvenIntent,
    ) -> mori_ledger::Result<mori_ledger::proving::CallVerification> =
        mori_ledger::proving::verify_proven_call;
}

#[test]
fn typed_checked_consumer_recompiles_public_artifact_and_preserves_legacy_refusal() {
    let f = metered(4, 7, None);
    let mut artifact =
        mori_ledger::relation::compile(&f.source, &f.program, &f.deployment, "settle").unwrap();
    artifact.manifest.action = "other".into();
    assert!(
        mori_ledger::prepare::prepare_native_call_checked(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
            &artifact
        )
        .is_err()
    );
    let mut changed = f.clone();
    let mut state: mori_ledger::layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let address = codec::from_hex(&f.deployment.address_hex).unwrap();
    let mut contract = state.index(address).unwrap();
    contract.operations = contract.operations.insert(
        b"settle".to_vec().into(),
        midnight_onchain_state::state::ContractOperation::new(
            None,
            Some(midnight_storage::arena::Sp::new(
                midnight_onchain_state::state::IrBuf(vec![]),
            )),
        ),
    );
    state.contract = state.contract.insert(address, contract);
    support::repin(&mut changed, &state);
    assert_eq!(
        prepare_native_call(
            &changed.source,
            &changed.program,
            &changed.deployment,
            &changed.predecessor,
            &changed.parameters,
            &changed.request
        )
        .err()
        .unwrap()
        .code,
        "unproven-operation"
    );
}

#[test]
fn typed_toggle_counter_prepares_different_event_branches_on_one_source_ir() {
    const SOURCE: &str = r#"language "moriarty/1"; agreement ToggleCounter {
 state count:UInt=0; state enabled:Bool=true;
 action run(amount:UInt) { let before=count; next count=count+amount;
 requires count>=before; next enabled=not enabled;
 emit "count"(count); emit "enabled"(enabled); ensures count==before+amount; }}"#;
    let mut identity = None;
    for (before, amount, after) in [
        (0, 0, 0),
        (31, 1, 32),
        (32, 223, 255),
        (255, 1, 256),
        (256, 0, 256),
        (u128::MAX, 0, u128::MAX),
    ] {
        let scenario = Scenario {
            state: [
                ("count".into(), Value::UInt(UInt(before))),
                ("enabled".into(), Value::Bool(true)),
            ]
            .into_iter()
            .collect(),
            inputs: [("amount".into(), Value::UInt(UInt(amount)))]
                .into_iter()
                .collect(),
            ..Default::default()
        };
        let f = support::fixture(SOURCE, "run", scenario);
        let prepared = prepare_native_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
        )
        .unwrap();
        let b = prepared.bundle();
        assert_eq!(
            b.readback.execution.post.state["count"],
            Value::UInt(UInt(after))
        );
        assert_eq!(
            b.readback.execution.post.state["enabled"],
            Value::Bool(false)
        );
        assert!(b.readback.execution.post.balances.is_empty());
        assert!(b.readback.execution.post.liabilities.is_empty());
        assert!(b.readback.prospective_outputs_hex.is_empty());
        assert!(b.readback.native_events_hex.len() == 2);
        let bytes = prepared.artifact().ir_bytes().unwrap();
        if let Some(expected) = &identity {
            assert_eq!(&bytes, expected);
        } else {
            identity = Some(bytes);
        }
    }
    let f = support::fixture(
        SOURCE,
        "run",
        Scenario {
            state: [
                ("count".into(), Value::UInt(UInt(u128::MAX))),
                ("enabled".into(), Value::Bool(true)),
            ]
            .into_iter()
            .collect(),
            inputs: [("amount".into(), Value::UInt(UInt(1)))]
                .into_iter()
                .collect(),
            ..Default::default()
        },
    );
    assert!(
        prepare_native_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request
        )
        .is_err()
    );
}
#[test]
fn typed_zero_transfers_fees_and_duties_keep_zero_semantics() {
    let f = support::cash(
        "transfer(QA,\"alice\",\"bob\",0); fee(QA,\"alice\",\"treasury\",0); liability(\"note\",QA,\"alice\",\"bob\",0); discharge(\"note\",0);",
        100,
    );
    let prepared = prepare_native_call(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
        &f.request,
    )
    .unwrap();
    let b = prepared.bundle();
    assert_eq!(b.readback.execution.effects.len(), 4);
    assert_eq!(b.readback.execution.post.liabilities.len(), 1);
    assert_eq!(b.readback.execution.post.liabilities[0].amount, UInt(0));
    assert!(b.readback.effects.claimed_unshielded_spends.is_empty());
    let outputs: Vec<midnight_ledger::structure::Utxo> = b
        .readback
        .prospective_outputs_hex
        .iter()
        .map(|v| codec::from_hex(v).unwrap())
        .collect();
    assert_eq!(outputs.len(), 1);
    assert_eq!(outputs[0].value, 100);
    // Actual reference semantics reject discharge-zero when the duty is absent.
    let source =
        r#"language "moriarty/1"; agreement MissingDuty { action run() { discharge("note",0); }}"#;
    let program = mori_core::parse(source).unwrap();
    assert_eq!(
        mori_core::simulate(&program, "run", &Scenario::default())
            .unwrap_err()
            .code,
        "mori::core::liability"
    );
}
