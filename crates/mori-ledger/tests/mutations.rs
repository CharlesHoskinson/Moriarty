mod support;
use midnight_ledger::{
    construct::{PreTranscript, partition_transcripts},
    structure::{LedgerParameters, UnshieldedOffer},
};
use midnight_onchain_runtime::{context::QueryContext, transcript::Transcript};
use midnight_onchain_vm::ops::Op;
use midnight_storage::{arena::Sp, db::InMemoryDB};
use mori_ledger::{codec, custody, layout, ops, prepare};

fn admitted(f: &support::Fixture) -> layout::AdmittedPredecessor {
    let abi = layout::admit_source(&f.program, &f.deployment).unwrap();
    layout::decode_predecessor(
        &f.program,
        &abi,
        &f.deployment,
        codec::decode_native(&f.predecessor).unwrap(),
        &f.request,
    )
    .unwrap()
}
fn call(f: &support::Fixture) -> mori_ledger::Result<prepare::PreparedBundle> {
    prepare::prepare_call(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
        &f.request,
    )
}

#[test]
fn transcript_declarations_are_checked_even_when_native_reexecution_accepts() {
    let f = support::payment();
    let pre = admitted(&f);
    let e = mori_core::simulate(&f.program, &f.request.action, &pre.scenario).unwrap();
    let lowered = ops::lower_ops(&f.program, &pre, &e).unwrap();
    let params: LedgerParameters = codec::decode_native(&f.parameters).unwrap();
    let mut c = QueryContext::new(pre.contract.data.clone(), pre.address);
    c.call_context = pre.context.clone();
    let pairs = partition_transcripts(
        &[PreTranscript {
            context: c.clone(),
            program: lowered.program.clone(),
            comm_comm: None,
        }],
        &params,
    )
    .unwrap();
    let t = pairs[0].0.as_ref().unwrap();
    let r = c
        .run_transcript(t, &params.cost_model.runtime_cost_model)
        .unwrap();
    prepare::validate_query(&lowered, &r).unwrap();
    let mut missing_event = lowered.program.clone();
    missing_event.truncate(missing_event.len() - 2);
    let changed = c
        .query(
            &missing_event,
            Some(pre.gas),
            &params.cost_model.runtime_cost_model,
        )
        .unwrap();
    assert_eq!(
        prepare::validate_query(&lowered, &changed)
            .unwrap_err()
            .code,
        "direct-state-effects-events"
    );
    let mut extra_duty = lowered.program.clone();
    let zero = pre.duties["zero"].clone();
    let bad = layout::duty_value(&zero, 1, &f.deployment.roles).unwrap();
    extra_duty.extend(ops::write_cell(1, layout::name_key("duty", "zero"), bad));
    let changed = c
        .query(
            &extra_duty,
            Some(pre.gas),
            &params.cost_model.runtime_cost_model,
        )
        .unwrap();
    assert_eq!(
        prepare::validate_query(&lowered, &changed)
            .unwrap_err()
            .code,
        "direct-state-effects-events"
    );
    prepare::validate_transcript(t, t, &lowered, &r, &pre.gas).unwrap();
    let mut effect = t.clone();
    effect.effects = Default::default();
    assert!(
        c.run_transcript(&effect, &params.cost_model.runtime_cost_model)
            .is_ok()
    );
    assert_eq!(
        prepare::validate_transcript(t, &effect, &lowered, &r, &pre.gas)
            .unwrap_err()
            .code,
        "declared-effects"
    );
    let mut version = t.clone();
    version.version = None;
    assert!(
        c.run_transcript(&version, &params.cost_model.runtime_cost_model)
            .is_ok()
    );
    assert_eq!(
        prepare::validate_transcript(t, &version, &lowered, &r, &pre.gas)
            .unwrap_err()
            .code,
        "transcript-version"
    );
    let mut large = t.clone();
    large.gas = pre.gas;
    assert!(
        c.run_transcript(&large, &params.cost_model.runtime_cost_model)
            .is_ok()
    );
    assert_eq!(
        prepare::validate_transcript(t, &large, &lowered, &r, &pre.gas)
            .unwrap_err()
            .code,
        "partition-gas-identity"
    );
    let mut zero = t.clone();
    zero.gas = Default::default();
    assert!(
        c.run_transcript(&zero, &params.cost_model.runtime_cost_model)
            .is_err()
    );
    let mut program = t.clone();
    program.program = ops::abi_guard().into();
    assert_eq!(
        prepare::validate_transcript(t, &program, &lowered, &r, &pre.gas)
            .unwrap_err()
            .code,
        "transcript-program"
    );
    assert_eq!(
        t.version.as_deref(),
        Some(&Transcript::<InMemoryDB>::VERSION)
    );
}

#[test]
fn wrong_new_claim_and_wrong_prefix_read_are_actual_native_controls() {
    let f = support::cash(
        "transfer(QA,\"alice\",\"bob\",10); transfer(QA,\"alice\",\"bob\",15); next count = count + 1; requires count > 0;",
        100,
    );
    let pre = admitted(&f);
    let e = mori_core::simulate(&f.program, &f.request.action, &pre.scenario).unwrap();
    let mut l = ops::lower_ops(&f.program, &pre, &e).unwrap();
    let p: LedgerParameters = codec::decode_native(&f.parameters).unwrap();
    let mut c = QueryContext::new(pre.contract.data.clone(), pre.address);
    c.call_context = pre.context.clone();
    let good = c
        .query(&l.program, Some(pre.gas), &p.cost_model.runtime_cost_model)
        .unwrap();
    assert_eq!(good.context.effects, l.expected_effects);
    assert_eq!(
        l.claim_bindings[1].previous_aggregate,
        Some(mori_core::UInt(10))
    );
    assert_eq!(l.claim_bindings[1].next_aggregate, mori_core::UInt(25));
    let index = l.claim_bindings[1].new_value.op as usize;
    let old = l.program[index].clone();
    l.program[index] = ops::push(26u128.into());
    let wrong = c
        .query(&l.program, Some(pre.gas), &p.cost_model.runtime_cost_model)
        .unwrap();
    assert_ne!(wrong.context.effects, l.expected_effects);
    assert_eq!(
        prepare::validate_query(&l, &wrong).unwrap_err().code,
        "direct-state-effects-events"
    );
    l.program[index] = old;
    let b = l
        .bindings
        .iter()
        .rev()
        .find(|b| {
            b.origin == "source-expression-prefix-state"
                && matches!(l.program[b.op as usize], Op::Popeq { .. })
        })
        .unwrap();
    l.program[b.op as usize] = Op::Popeq {
        cached: false,
        result: 0u128.into(),
    };
    assert!(
        c.query(&l.program, Some(pre.gas), &p.cost_model.runtime_cost_model)
            .is_err()
    );
    assert!(
        l.bindings
            .iter()
            .filter(|b| b.origin == "abi-root-array3")
            .all(|b| b.dynamic_field_count == 0)
    );
    use midnight_transient_crypto::fab::AlignedValueExt;
    use midnight_transient_crypto::repr::FieldRepr;
    let wire = |span: &ops::FieldBinding| {
        l.program[span.op as usize].field_vec()[span.offset..span.offset + span.count].to_vec()
    };
    let value = |v: midnight_base_crypto::fab::AlignedValue| {
        let mut out = Vec::new();
        v.value_only_field_repr(&mut out);
        out
    };
    // Restoring the altered source read does not change claim scalar spans.
    for binding in &l.claim_bindings {
        assert_eq!(
            wire(&binding.membership_result),
            value(binding.expected_membership.into())
        );
        assert_eq!(
            wire(&binding.new_value),
            value(binding.next_aggregate.0.into())
        );
        if let (Some(old), Some(span)) = (binding.previous_aggregate, &binding.previous_result) {
            assert_eq!(wire(span), value(old.0.into()));
        }
        let key: midnight_base_crypto::fab::AlignedValue =
            codec::from_hex(&binding.native_key_hex).unwrap();
        for span in &binding.key_values {
            assert_eq!(wire(span), value(key.clone()));
        }
        if let Some(span) = &binding.previous_lookup_key {
            assert_eq!(wire(span), value(key));
        }
    }
}

#[test]
fn exact_custody_rejects_split_change_merged_self_and_lost_multiplicity() {
    for (source, values) in [
        (
            "transfer(QA,\"alice\",\"bob\",10); transfer(QA,\"alice\",\"bob\",15); fee(QA,\"alice\",\"bob\",2);",
            vec![27, 36, 37],
        ),
        (
            "transfer(QA,\"alice\",\"alice\",10); transfer(QA,\"alice\",\"bob\",20); fee(QA,\"alice\",\"alice\",2);",
            vec![12, 20, 34, 34],
        ),
    ] {
        let f = support::cash(source, 100);
        let pre = admitted(&f);
        let e = mori_core::simulate(&f.program, "run", &pre.scenario).unwrap();
        let o = custody::prepare_offer(&pre, &e).unwrap();
        custody::validate_offer(&pre, &e, &o.offer).unwrap();
        let payer = pre.roles["alice"].address;
        let bob = pre.roles["bob"].address;
        let token = pre.assets[&f.program.assets[0].identity];
        let mut outputs: Vec<_> = values
            .into_iter()
            .map(|value| midnight_ledger::structure::UtxoOutput {
                owner: if value == 27 || value == 20 {
                    bob
                } else {
                    payer
                },
                type_: token,
                value,
            })
            .collect();
        outputs.sort();
        assert_eq!(outputs.iter().map(|o| o.value).sum::<u128>(), 100);
        let candidate = UnshieldedOffer {
            inputs: o.offer.inputs.clone(),
            outputs: outputs.into(),
            signatures: Default::default(),
        };
        assert!(custody::validate_offer(&pre, &e, &candidate).is_err());
    }
    let f = support::cash("transfer(QA,\"alice\",\"alice\",50);", 100);
    let pre = admitted(&f);
    let e = mori_core::simulate(&f.program, "run", &pre.scenario).unwrap();
    let mut o = custody::prepare_offer(&pre, &e).unwrap().offer;
    let outputs = Vec::from(&o.outputs);
    assert_eq!(outputs[0], outputs[1]);
    o.outputs = vec![outputs[0].clone()].into();
    assert!(custody::validate_offer(&pre, &e, &o).is_err());
}

#[test]
fn identity_scope_spend_and_native_balance_refusals_are_public() {
    let base = support::payment();
    let mut f = base.clone();
    f.deployment.source_hash = "00".repeat(32);
    assert_eq!(call(&f).err().unwrap().code, "raw-source");
    let mut f = base.clone();
    f.deployment.parameters_hash = "00".repeat(32);
    assert_eq!(call(&f).err().unwrap().code, "parameters");
    let mut f = base.clone();
    f.deployment.predecessor_hash = "00".repeat(32);
    assert_eq!(call(&f).err().unwrap().code, "predecessor");
    let mut f = base.clone();
    f.request.segment = 0;
    assert_eq!(call(&f).err().unwrap().code, "request-scope");
    let mut f = base.clone();
    f.request
        .selected_spends
        .push(f.request.selected_spends[0].clone());
    assert_eq!(call(&f).err().unwrap().code, "selected-utxo");
    let mut f = base.clone();
    f.request.selected_spends.clear();
    assert_eq!(call(&f).err().unwrap().code, "insufficient-selected-inputs");
    let mut f = base.clone();
    f.request
        .inputs
        .insert("unexpected".into(), mori_core::Value::Bool(true));
    assert_eq!(call(&f).err().unwrap().code, "core-refusal");
    let mut f = base.clone();
    f.request.action = "unknown".into();
    assert_eq!(call(&f).err().unwrap().code, "selected-action");
    let mut f = base.clone();
    f.deployment.roles[1].public_key_hex = f.deployment.roles[0].public_key_hex.clone();
    f.deployment.roles[1].address_hex = f.deployment.roles[0].address_hex.clone();
    assert_eq!(call(&f).err().unwrap().code, "role-address");
    let f = support::cash("transfer(QA,\"alice\",\"bob\",0);", 1u128 << 127);
    assert_eq!(call(&f).err().unwrap().layer, "native-balance");
    let mut f = base.clone();
    f.deployment.toolchain.rust = "other".into();
    assert_eq!(call(&f).err().unwrap().code, "toolchain-or-limits");
    let mut f = base.clone();
    f.deployment.qualification = "deployed".into();
    assert_eq!(call(&f).err().unwrap().code, "local-profile");
    let mut f = base.clone();
    f.deployment.assets[0].identity.scale += 1;
    assert_eq!(call(&f).err().unwrap().code, "source-asset");
    let mut f = base.clone();
    f.deployment.assets.push(f.deployment.assets[0].clone());
    assert_eq!(call(&f).err().unwrap().code, "asset-mapping");
    let mut f = base.clone();
    f.deployment.duties[0].creditor = "treasury".into();
    assert_eq!(call(&f).err().unwrap().code, "source-duty-identity");
}

#[test]
fn admitted_long_core_event_is_refused_before_native_silent_drop() {
    let name = "x".repeat(1000);
    let f = support::cash(&format!("emit \"{name}\"(count);"), 0);
    assert_eq!(call(&f).err().unwrap().code, "event-emitted-byte-limit");
    let pre = admitted(&f);
    let p: LedgerParameters = codec::decode_native(&f.parameters).unwrap();
    let mut c = QueryContext::new(pre.contract.data.clone(), pre.address);
    c.call_context = pre.context.clone();
    let payload = ops::cell(midnight_base_crypto::fab::AlignedValue::from(vec![
        1u8;
        1500
    ]));
    let wrapper = midnight_onchain_state::state::StateValue::Array(
        vec![
            ops::cell(1u32.into()),
            ops::cell((midnight_onchain_vm::ops::LogEventType::Misc as u8).into()),
            payload,
        ]
        .into(),
    );
    let mut program = ops::abi_guard();
    program.extend([
        Op::Push {
            storage: false,
            value: wrapper,
        },
        Op::Log,
    ]);
    let r = c
        .query(&program, Some(pre.gas), &p.cost_model.runtime_cost_model)
        .unwrap();
    assert!(r.events.is_empty());
    // Full native charged state is retained, including query accounting.
    let b = call(&support::cash("transfer(QA,\"alice\",\"bob\",0);", 0)).unwrap();
    let state: midnight_onchain_state::state::ChargedState<InMemoryDB> =
        codec::decode_artifact(&b.leaves["successor-data.bin"]).unwrap();
    assert_eq!(
        codec::native_hex(&state).unwrap(),
        b.readback.charged_successor_hex
    );
    assert_eq!(
        codec::native_hex(&*state.get()).unwrap(),
        b.readback.semantic_successor_hex
    );
    let _: Sp<midnight_onchain_state::state::StateValue<InMemoryDB>> = state.get();
}

#[test]
fn source_wide_layout_frame_and_full_selected_spend_fields_are_checked() {
    let f = support::cash("transfer(QA,\"alice\",\"bob\",10);", 100);
    let other = f.source.replace(
        "action run()",
        "action other() { transfer(QA,\"bob\",\"alice\",1); } action run()",
    );
    let p = mori_core::parse(&other).unwrap();
    assert_eq!(
        layout::admit_source(&p, &f.deployment).unwrap_err().code,
        "one-public-payer"
    );
    for field in 0..5 {
        let mut f = f.clone();
        let mut spend: midnight_ledger::structure::UtxoSpend =
            codec::from_hex(&f.request.selected_spends[0]).unwrap();
        match field {
            0 => spend.value += 1,
            1 => spend.output_no += 1,
            2 => {
                spend.intent_hash = midnight_ledger::structure::IntentHash(
                    midnight_base_crypto::hash::HashOutput([99; 32]),
                )
            }
            3 => spend.owner = codec::from_hex(&f.deployment.roles[1].public_key_hex).unwrap(),
            _ => {
                spend.type_ = midnight_coin_structure::coin::UnshieldedTokenType(
                    midnight_base_crypto::hash::HashOutput([99; 32]),
                )
            }
        }
        f.request.selected_spends[0] = codec::native_hex(&spend).unwrap();
        assert_eq!(call(&f).err().unwrap().code, "selected-utxo");
    }
    let mut changed = f.clone();
    let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let address = codec::from_hex(&f.deployment.address_hex).unwrap();
    let mut contract = ledger.index(address).unwrap();
    contract.operations = Default::default();
    ledger.contract = ledger.contract.insert(address, contract);
    support::repin(&mut changed, &ledger);
    assert_eq!(call(&changed).err().unwrap().code, "operation-set");
    for ir in [false, true] {
        let mut changed = f.clone();
        let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
        let mut contract = ledger.index(address).unwrap();
        let operation = midnight_onchain_state::state::ContractOperation::new(
            None,
            if ir {
                Some(Sp::new(midnight_onchain_state::state::IrBuf(vec![])))
            } else {
                None
            },
        );
        contract.operations = contract.operations.insert(
            if ir {
                b"run".to_vec()
            } else {
                b"extra".to_vec()
            }
            .into(),
            operation,
        );
        ledger.contract = ledger.contract.insert(address, contract);
        support::repin(&mut changed, &ledger);
        assert_eq!(
            call(&changed).err().unwrap().code,
            if ir {
                "unproven-operation"
            } else {
                "operation-set"
            }
        );
    }
    let mut changed = f.clone();
    let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let mut contract = ledger.index(address).unwrap();
    contract.data = midnight_onchain_state::state::ChargedState::new(
        midnight_onchain_state::state::StateValue::Array(
            vec![midnight_onchain_state::state::StateValue::Null; 2].into(),
        ),
    );
    ledger.contract = ledger.contract.insert(address, contract);
    support::repin(&mut changed, &ledger);
    assert_eq!(call(&changed).err().unwrap().code, "abi-root");
    let mut changed = f.clone();
    let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let mut contract = ledger.index(address).unwrap();
    let root = contract.data.get();
    let midnight_onchain_state::state::StateValue::Array(array) = &*root else {
        panic!()
    };
    let states = layout::map(array.get(0).unwrap())
        .unwrap()
        .insert(layout::name_key("state", "count"), ops::cell(0u64.into()));
    contract.data = midnight_onchain_state::state::ChargedState::new(
        midnight_onchain_state::state::StateValue::Array(
            array
                .insert(0, midnight_onchain_state::state::StateValue::Map(states))
                .unwrap(),
        ),
    );
    ledger.contract = ledger.contract.insert(address, contract);
    support::repin(&mut changed, &ledger);
    assert!(call(&changed).is_err());
    let mut changed = f.clone();
    let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    ledger.reserve_pool += 1;
    support::repin(&mut changed, &ledger);
    assert_eq!(call(&changed).err().unwrap().code, "night-supply");
}

#[test]
fn native_intermediate_i128_overflow_and_all_balance_keys_are_checked() {
    let mut f = support::cash(
        "transfer(QA,\"alice\",\"alice\",85070591730234615865843651857942052864);",
        1u128 << 127,
    );
    let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let (original, meta) = ledger
        .utxo
        .utxos
        .iter()
        .next()
        .map(|r| ((*r.0).clone(), (*r.1).clone()))
        .unwrap();
    let mut utxo = (*ledger.utxo).clone();
    utxo.utxos = Default::default();
    f.request.selected_spends.clear();
    let key: midnight_ledger::structure::SignatureVerifyingKey = codec::from_hex(
        &f.deployment
            .roles
            .iter()
            .find(|r| r.role == "alice")
            .unwrap()
            .public_key_hex,
    )
    .unwrap();
    for i in 0..2 {
        let mut u = original.clone();
        u.value = 1u128 << 126;
        u.output_no = i;
        utxo.utxos = utxo.utxos.insert(u.clone(), meta.clone());
        f.request.selected_spends.push(
            codec::native_hex(&midnight_ledger::structure::UtxoSpend {
                owner: key.clone(),
                value: u.value,
                type_: u.type_,
                intent_hash: u.intent_hash,
                output_no: u.output_no,
            })
            .unwrap(),
        );
    }
    ledger.utxo = Sp::new(utxo);
    support::repin(&mut f, &ledger);
    assert_eq!(call(&f).err().unwrap().layer, "native-balance");
    let token = midnight_coin_structure::coin::TokenType::Unshielded(original.type_);
    let zero = [((token, 0u16), 0i128)].into_iter().collect();
    prepare::validate_balance(&zero).unwrap();
    let mut nonzero = zero;
    nonzero.insert(
        (
            midnight_coin_structure::coin::TokenType::Unshielded(
                midnight_coin_structure::coin::UnshieldedTokenType(
                    midnight_base_crypto::hash::HashOutput([98; 32]),
                ),
            ),
            7,
        ),
        1,
    );
    assert_eq!(
        prepare::validate_balance(&nonzero).unwrap_err().code,
        "nonzero-delta"
    );
}

#[test]
fn native_extensions_foreign_duties_unknown_utxos_and_accounting_are_preserved() {
    use midnight_base_crypto::hash::HashOutput;
    use midnight_coin_structure::coin::UnshieldedTokenType;
    use midnight_onchain_state::state::{ChargedState, StateValue};
    let mut f = support::cash("next count = count + 1;", 100);
    let mut ledger: layout::NativeState = codec::decode_native(&f.predecessor).unwrap();
    let address = codec::from_hex(&f.deployment.address_hex).unwrap();
    let mut contract = ledger.index(address).unwrap();
    let foreign = mori_core::AssetId {
        network: "local".into(),
        issuer: "foreign".into(),
        reference: "other".into(),
        scale: 6,
    };
    let token = UnshieldedTokenType(HashOutput([44; 32]));
    f.deployment
        .assets
        .push(mori_ledger::profile::AssetBinding {
            identity: foreign.clone(),
            token_hex: codec::native_hex(&token).unwrap(),
        });
    let duty = mori_ledger::profile::DutyBinding {
        name: "foreign-duty".into(),
        asset: foreign,
        debtor: "alice".into(),
        creditor: "bob".into(),
    };
    let duty_key = layout::name_key("duty", &duty.name);
    let duty_value = ops::cell(layout::duty_value(&duty, 0, &f.deployment.roles).unwrap());
    f.deployment.duties.push(duty);
    let root = contract.data.get();
    let StateValue::Array(array) = &*root else {
        panic!()
    };
    let duties = layout::map(array.get(1).unwrap())
        .unwrap()
        .insert(duty_key.clone(), duty_value.clone());
    let ext_key = HashOutput([55; 32]).into();
    let ext_value = ops::cell(9u128.into());
    let extensions = layout::map(array.get(2).unwrap())
        .unwrap()
        .insert(ext_key, ext_value);
    let updated = array
        .insert(1, StateValue::Map(duties))
        .unwrap()
        .insert(2, StateValue::Map(extensions.clone()))
        .unwrap();
    contract.data = ChargedState::new(StateValue::Array(updated));
    ledger.contract = ledger.contract.insert(address, contract);
    let mut utxo = (*ledger.utxo).clone();
    let unknown = midnight_ledger::structure::Utxo {
        owner: midnight_coin_structure::coin::UserAddress(HashOutput([43; 32])),
        type_: token,
        value: 77,
        intent_hash: midnight_ledger::structure::IntentHash(HashOutput([66; 32])),
        output_no: 3,
    };
    let meta = midnight_ledger::structure::UtxoMeta {
        ctime: midnight_base_crypto::time::Timestamp::from_secs(19),
    };
    utxo.utxos = utxo.utxos.insert(unknown.clone(), meta.clone());
    ledger.utxo = Sp::new(utxo);
    support::repin(&mut f, &ledger);
    let original = f.predecessor.clone();
    let b = call(&f).unwrap();
    assert_eq!(f.predecessor, original);
    assert!(b.readback.execution.post.liabilities.is_empty());
    assert!(
        b.readback
            .retained_utxos_hex
            .contains(&codec::native_hex(&(unknown, meta)).unwrap())
    );
    let successor: ChargedState<InMemoryDB> =
        codec::decode_artifact(&b.leaves["successor-data.bin"]).unwrap();
    let successor_root = successor.get();
    let StateValue::Array(array) = &*successor_root else {
        panic!()
    };
    assert_eq!(array.get(2), Some(&StateValue::Map(extensions)));
    assert_eq!(
        layout::map(array.get(1).unwrap())
            .unwrap()
            .get(&duty_key)
            .as_deref(),
        Some(&duty_value)
    );
    let mut next = ledger.index(address).unwrap();
    next.data = successor;
    ledger.contract = ledger.contract.insert(address, next);
    support::repin(&mut f, &ledger);
    let second = call(&f).unwrap();
    assert_eq!(
        second.readback.execution.pre.state["count"],
        mori_core::Value::UInt(mori_core::UInt(1))
    );
    assert_eq!(
        second.readback.execution.post.state["count"],
        mori_core::Value::UInt(mori_core::UInt(2))
    );
    let same = call(&f).unwrap();
    assert_eq!(second.marker, same.marker);
    assert_eq!(second.leaves, same.leaves);
    f.request.nonce = mori_core::UInt(2);
    let nonce = call(&f).unwrap();
    assert_ne!(nonce.marker, same.marker);
    assert_eq!(nonce.leaves, same.leaves);
}
#[test]
fn heavy_source_and_late_semantic_failure_have_no_prepared_bundle() {
    let f = support::cash(&"next count = count + 1;".repeat(30), 0);
    assert_eq!(call(&f).err().unwrap().layer, "unsupported-phase");
    for statements in [
        "transfer(QA,\"alice\",\"bob\",10); ensures false;",
        "next count = count - 1;",
        "requires false; transfer(QA,\"alice\",\"bob\",10);",
    ] {
        let f = support::cash(statements, 100);
        assert_eq!(call(&f).err().unwrap().layer, "semantic");
    }
}

#[test]
fn exact_unsigned_economic_bounds_count_transfer_and_fee_separately() {
    let mut f = support::payment();
    let limit = mori_core::intent::AssetLimit {
        asset: f.program.assets[0].identity.clone(),
        beneficiary: "bob".into(),
        gross_cap: mori_core::UInt(72),
        fee_cap: mori_core::UInt(2),
        net_floor: mori_core::UInt(68),
        liability_cap: mori_core::UInt(35),
    };
    let bounds = mori_ledger::profile::Bounds {
        owner: "alice".into(),
        limits: vec![limit],
    };
    f.deployment.economic_bounds = Some(bounds.clone());
    f.request.economic_bounds = Some(bounds);
    assert!(call(&f).is_ok());
    for field in 0..4 {
        let mut f = f.clone();
        let limits = &mut f.deployment.economic_bounds.as_mut().unwrap().limits[0];
        match field {
            0 => limits.gross_cap = mori_core::UInt(71),
            1 => limits.fee_cap = mori_core::UInt(1),
            2 => limits.net_floor = mori_core::UInt(69),
            _ => limits.liability_cap = mori_core::UInt(34),
        };
        f.request.economic_bounds = f.deployment.economic_bounds.clone();
        assert_eq!(call(&f).err().unwrap().code, "economic-bounds");
    }
    let mut f = f.clone();
    f.request.economic_bounds = None;
    assert_eq!(call(&f).err().unwrap().code, "request-scope");
}
