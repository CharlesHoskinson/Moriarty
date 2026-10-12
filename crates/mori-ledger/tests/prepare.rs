mod support;
use mori_ledger::prepare::prepare_call;
fn native_key(hex: &str) -> midnight_base_crypto::fab::AlignedValue {
    let bytes = mori_ledger::codec::unhex(hex).unwrap();
    let array: [u8; 32] = bytes.try_into().unwrap();
    midnight_base_crypto::hash::HashOutput(array).into()
}
fn actual_native(
    b: &mori_ledger::prepare::PreparedBundle,
    f: &support::Fixture,
    states: &[(&str, mori_core::Value)],
    duties: &[(&str, u128)],
    claims: &[(&str, u128)],
    event: &str,
    event_value: u128,
) {
    use midnight_onchain_state::state::StateValue;
    let root: StateValue<midnight_storage::db::InMemoryDB> =
        mori_ledger::codec::from_hex(&b.readback.semantic_successor_hex).unwrap();
    let StateValue::Array(ref a) = root else {
        panic!()
    };
    let keys = [
        (
            "paid",
            "f0f316e7b9fb9479b794e51c824dadf00366732b50801cee71135b2947dd47dc",
        ),
        (
            "collected",
            "3607beda935241b72bd2c81149eb82715887818d6b0f02bd509b7fe30589e702",
        ),
        (
            "frozen",
            "017ae2d1224ad5b78f86440a9aca00d6038e48fa33b2db240570de0aac902894",
        ),
        (
            "payment",
            "d77277ec7533cbbe9407285bea7eb827e11fdec2d0018973f5d4252a95e44adc",
        ),
        (
            "reserve",
            "3e1d4f3b5f9b4c84b9c58613ba98e6df6dc805b497c903a00fb84c7176303139",
        ),
        (
            "zero",
            "7b27b6906055e0e00fe79324630f0e1064b602377812a373720829ff47a9c4d6",
        ),
        (
            "addon",
            "3a066511d120d6bf06d10ac61bf7bf43705ba325eee94687c55be411ea37bde9",
        ),
    ];
    let key = |name: &str| native_key(keys.iter().find(|k| k.0 == name).unwrap().1);
    let StateValue::Map(sm) = a.get(0).unwrap() else {
        panic!()
    };
    let StateValue::Map(dm) = a.get(1).unwrap() else {
        panic!()
    };
    for (name, value) in states {
        let cell = sm.get(&key(name)).unwrap();
        let StateValue::Cell(v) = &*cell else {
            panic!()
        };
        let expected: midnight_base_crypto::fab::AlignedValue = match value {
            mori_core::Value::UInt(n) => n.0.into(),
            mori_core::Value::Bool(v) => (*v).into(),
        };
        assert_eq!(**v, expected);
    }
    for (name, n) in duties {
        let cell = dm.get(&key(name)).unwrap();
        let StateValue::Cell(v) = &*cell else {
            panic!()
        };
        assert_eq!(u128::try_from(&v.value[3..4]).unwrap(), *n);
    }
    let actual: midnight_onchain_runtime::context::Effects<midnight_storage::db::InMemoryDB> =
        mori_ledger::codec::from_hex(&b.readback.effects_hex).unwrap();
    let mut expected = midnight_onchain_runtime::context::Effects::default();
    for (role, n) in claims {
        let role = f.deployment.roles.iter().find(|r| r.role == *role).unwrap();
        let address = mori_ledger::codec::from_hex(&role.address_hex).unwrap();
        let token = mori_ledger::codec::from_hex(&f.deployment.assets[0].token_hex).unwrap();
        let k = midnight_onchain_runtime::context::ClaimedUnshieldedSpendsKey::from_inner(
            midnight_coin_structure::coin::TokenType::Unshielded(token),
            midnight_coin_structure::coin::PublicAddress::User(address),
        );
        expected.claimed_unshielded_spends = expected.claimed_unshielded_spends.insert(k, *n);
    }
    assert_eq!(actual, expected); // Includes every one of the nine native Effects components.
    let observed: midnight_onchain_vm::ops::VersionedLogItem<midnight_storage::db::InMemoryDB> =
        mori_ledger::codec::from_hex(&b.readback.native_events_hex[0]).unwrap();
    assert_eq!(observed.version, 1);
    assert_eq!(
        observed.event_type,
        midnight_onchain_vm::ops::LogEventType::Misc
    );
    let encoded =
        mori_ledger::codec::encode(&midnight_base_crypto::fab::AlignedValue::from(event_value))
            .unwrap();
    let domain = b"moriarty-ledger-public-abi/1:event";
    let parts = [
        event.as_bytes(),
        f.request.action.as_bytes(),
        encoded.as_slice(),
    ];
    let mut bytes = Vec::new();
    bytes.extend((domain.len() as u32).to_le_bytes());
    bytes.extend(domain);
    bytes.extend(3u32.to_le_bytes());
    for part in parts {
        bytes.extend((part.len() as u32).to_le_bytes());
        bytes.extend(part);
    }
    let StateValue::Cell(ref v) = observed.data else {
        panic!()
    };
    assert_eq!(**v, midnight_base_crypto::fab::AlignedValue::from(bytes));
    assert_eq!(b.readback.native_events_hex.len(), 1);
    if event == "paid" {
        let golden: serde_json::Value =
            serde_json::from_str(include_str!("fixtures/role-event-vectors.json")).unwrap();
        assert_eq!(
            mori_ledger::codec::hex(&encoded),
            golden["event"]["value_native_hex"]
        );
        assert_eq!(
            mori_ledger::codec::hex(&v.value.0[0].0),
            golden["event"]["framed_hex"]
        );
        assert_eq!(
            mori_ledger::codec::hash(&v.value.0[0].0),
            golden["event"]["sha256"]
        );
        for role in golden["roles"].as_array().unwrap() {
            let actual = f
                .deployment
                .roles
                .iter()
                .find(|r| r.role == role["role"].as_str().unwrap())
                .unwrap();
            assert_eq!(actual.public_key_hex, role["public_key_hex"]);
            assert_eq!(actual.address_hex, role["address_hex"]);
            assert_eq!(
                mori_ledger::codec::hex(&mori_ledger::layout::role_digest(actual).unwrap().0),
                role["sha256"]
            );
        }
    }
}

#[test]
fn independent_payment_executes_complete_native_preparation() {
    let f = support::payment();
    let bundle = prepare_call(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
        &f.request,
    )
    .unwrap();
    let r = &bundle.readback;
    assert_eq!(
        r.execution.post.state["paid"],
        mori_core::Value::UInt(mori_core::UInt(70))
    );
    assert_eq!(
        r.prospective_balances
            .iter()
            .map(|b| (b.account.as_str(), b.amount.0))
            .collect::<Vec<_>>(),
        vec![("alice", 28), ("auditor", 0), ("bob", 80), ("treasury", 2)]
    );
    let duties = &r.execution.post.liabilities;
    assert_eq!(
        duties
            .iter()
            .map(|d| (d.name.as_str(), d.amount.0))
            .collect::<Vec<_>>(),
        vec![("payment", 30), ("reserve", 5), ("zero", 0)]
    );
    assert!(r.effects.claimed_unshielded_spends.size() == 2);
    assert!(bundle.leaves.contains_key("guaranteed-transcript.bin"));
    assert!(bundle.leaves.contains_key("offer.bin"));
    assert_eq!(r.evidence.ledger_apply, "not-performed");
    assert_eq!(
        r.preserved_frame.scope,
        "unchanged-supplied-ledger-not-an-applied-successor"
    );
    assert_eq!(
        r.preserved_frame.commitments["complete_ledger"],
        f.deployment.predecessor_hash
    );
    assert_eq!(
        r.preserved_frame.commitments["contract_operations"],
        f.deployment.operations_hash
    );
    assert_eq!(
        r.preserved_frame.commitments["contract_maintenance"],
        f.deployment.maintenance_hash
    );
    assert_eq!(
        r.preserved_frame.commitments["contract_balance"],
        f.deployment.balance_hash
    );
    actual_native(
        &bundle,
        &f,
        &[("paid", mori_core::Value::UInt(mori_core::UInt(70)))],
        &[("payment", 30), ("reserve", 5), ("zero", 0)],
        &[("bob", 70), ("treasury", 2)],
        "paid",
        70,
    );
}

#[test]
fn contrasting_fee_first_collection_has_full_state_duties_and_native_claims() {
    let f = support::collection();
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
        b.readback.execution.post.state["frozen"],
        mori_core::Value::Bool(true)
    );
    assert_eq!(
        b.readback.execution.post.state["collected"],
        mori_core::Value::UInt(mori_core::UInt(40))
    );
    assert_eq!(
        b.readback
            .prospective_balances
            .iter()
            .map(|b| (b.account.as_str(), b.amount.0))
            .collect::<Vec<_>>(),
        vec![
            ("alice", 57),
            ("auditor", 0),
            ("carol", 47),
            ("spectator", 9),
            ("treasury", 4)
        ]
    );
    assert_eq!(
        b.readback
            .execution
            .post
            .liabilities
            .iter()
            .map(|d| (d.name.as_str(), d.amount.0))
            .collect::<Vec<_>>(),
        vec![("addon", 4), ("payment", 5), ("reserve", 5), ("zero", 0)]
    );
    assert!(matches!(
        b.readback.execution.effects[0],
        mori_core::Effect::Fee { .. }
    ));
    assert!(matches!(
        b.readback.execution.effects[1],
        mori_core::Effect::Transfer { .. }
    ));
    actual_native(
        &b,
        &f,
        &[
            ("collected", mori_core::Value::UInt(mori_core::UInt(40))),
            ("frozen", mori_core::Value::Bool(true)),
        ],
        &[("addon", 4), ("payment", 5), ("reserve", 5), ("zero", 0)],
        &[("carol", 40), ("treasury", 3)],
        "collected",
        40,
    );
}
#[test]
fn ordinary_self_zero_grouped_and_uint128_sources_use_the_same_actual_caller() {
    for (source, budget, outputs) in [
        (
            "transfer(QA,\"alice\",\"alice\",10); transfer(QA,\"alice\",\"bob\",20); fee(QA,\"alice\",\"alice\",2);",
            100,
            vec![12, 20, 68],
        ),
        ("transfer(QA,\"alice\",\"alice\",50);", 100, vec![50, 50]),
        (
            "transfer(QA,\"alice\",\"bob\",10); transfer(QA,\"alice\",\"bob\",15); fee(QA,\"alice\",\"bob\",2);",
            100,
            vec![27, 73],
        ),
        (
            "transfer(QA,\"alice\",\"bob\",0); fee(QA,\"alice\",\"treasury\",0);",
            100,
            vec![100],
        ),
        ("transfer(QA,\"alice\",\"bob\",0);", 0, vec![]),
        ("", 0, vec![]),
        ("", 100, vec![100]),
        (
            "transfer(QA,\"alice\",\"bob\",18446744073709551616); transfer(QA,\"alice\",\"bob\",18446744073709551616);",
            1u128 << 65,
            vec![1u128 << 65],
        ),
        (
            "transfer(QA,\"alice\",\"bob\",9223372036854775808); transfer(QA,\"alice\",\"bob\",9223372036854775808);",
            1u128 << 65,
            vec![1u128 << 64, 1u128 << 64],
        ),
    ] {
        let f = support::cash(source, budget);
        let b = prepare_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request,
        )
        .unwrap();
        let offer: midnight_ledger::structure::UnshieldedOffer<
            (),
            midnight_storage::db::InMemoryDB,
        > = mori_ledger::codec::decode_artifact(&b.leaves["offer.bin"]).unwrap();
        assert_eq!(
            offer.outputs.iter().map(|o| o.value).collect::<Vec<_>>(),
            outputs
        );
        assert!(b.readback.native_events_hex.is_empty());
        assert!(b.readback.execution.post.liabilities.is_empty());
    }
}
#[test]
fn expression_state_reads_follow_actual_prefix_after_next() {
    let f = support::cash(
        "next count = count + 1; requires count > 0; transfer(QA,\"alice\",\"bob\",count); emit \"count\"(count);",
        100,
    );
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
        mori_core::Value::UInt(mori_core::UInt(1))
    );
    assert_eq!(b.readback.prospective_balances[0].amount.0, 99);
    assert_eq!(b.readback.native_events_hex.len(), 1);
    let abi: serde_json::Value = serde_json::from_slice(&b.leaves["abi.json"]).unwrap();
    assert!(
        abi["op_bindings"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|r| r["origin"] == "source-expression-prefix-state")
            .count()
            >= 9
    );
}
#[test]
fn actual_multiasset_custody_and_typed_bool_let_follow_general_source() {
    let f = support::multiple_assets();
    let b = prepare_call(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
        &f.request,
    )
    .unwrap();
    let observed: Vec<_> = b
        .readback
        .prospective_balances
        .iter()
        .map(|b| (b.account.as_str(), b.asset.reference.as_str(), b.amount.0))
        .collect();
    assert_eq!(
        observed,
        vec![
            ("alice", "BOND", 17),
            ("bob", "BOND", 0),
            ("treasury", "BOND", 3),
            ("alice", "QA", 30),
            ("bob", "QA", 72),
            ("treasury", "QA", 0)
        ]
    );
    assert_eq!(b.readback.execution.post.liabilities[0].amount.0, 5);
    let source = r#"language "moriarty/1"; agreement BooleanBook { state enabled: Bool = true; action run() { let allowed = enabled; requires allowed; next enabled = not enabled; emit "disabled"(enabled); } }"#;
    let f = support::fixture(source, "run", mori_core::Scenario::default());
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
        b.readback.execution.post.state["enabled"],
        mori_core::Value::Bool(false)
    );
    assert_eq!(b.readback.native_events_hex.len(), 1);
}
