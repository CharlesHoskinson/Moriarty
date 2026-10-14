//! Strict native-entry source refusals only: no VM, SDK query/check, k, keys or proof.
//!
//! Each descriptor is admitted for a coherent single-payer source variant first.
//! Only its source/program hashes are rebound to the real mixed-debit program.
//! The four public entry points below must return source-profile/one-public-payer
//! before native artifact/predecessor/parameter decoding or preparation can run.
use midnight_base_crypto::{hash::HashOutput, time::Timestamp};
use midnight_coin_structure::coin::{UnshieldedTokenType, UserAddress};
use midnight_ledger::structure::SignatureVerifyingKey;
use midnight_onchain_runtime::context::BlockContext;
use midnight_transient_crypto::{commitment::PedersenRandomness, curve::Fr};
use mori_core::{Program, Scenario, Statement, Type, UInt, Value};
use mori_ledger::{codec, layout, local_deployment, prepare, profile::*, relation};
use std::collections::{BTreeMap, BTreeSet};

const CROSS_ACTION: &str = r#"language "moriarty/1"; agreement CrossActionDebit {
asset A = { network: "local", issuer: "public-fixture", reference: "A", scale: 0 };
state count: UInt = 0;
action fund(amount: UInt) { transfer(A, "alice", "pool", amount); }
action claim(amount: UInt) { transfer(A, "pool", "bob", amount); }
action idle() { next count = count + 1; }
}"#;
const SAME_ACTION_FEE: &str = r#"language "moriarty/1"; agreement SameActionFeeDebit {
asset A = { network: "local", issuer: "public-fixture", reference: "A", scale: 0 };
action settle(amount: UInt) {
  transfer(A, "alice", "pool", amount);
  fee(A, "pool", "treasury", 1);
}
}"#;
const FEE_POOL: &str = include_str!("../../../examples/core/fee-pool/fee-pool.mori");

fn public_roles(program: &Program, payer: &str) -> Vec<RoleBinding> {
    // Published compressed verifier points, without private scalars or signing.
    let points = [
        "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9",
        "02e493dbf1c10d80f3581e4904930b1404cc6c13900ee0758474fa94abe8c4cd13",
    ];
    let mut names = BTreeSet::from([payer.to_owned()]);
    for statement in program.actions.iter().flat_map(|a| &a.statements) {
        match statement {
            Statement::Transfer { from, to, .. } | Statement::Fee { from, to, .. } => {
                names.insert(from.clone());
                names.insert(to.clone());
            }
            Statement::Liability {
                debtor, creditor, ..
            } => {
                names.insert(debtor.clone());
                names.insert(creditor.clone());
            }
            _ => {}
        }
    }
    assert!(names.len() <= points.len());
    names.into_iter().zip(points).map(|(role, point)| {
        let bytes = codec::unhex(point).unwrap();
        let public = <midnight_base_crypto::ecdsa::VerifyingKey as midnight_serialize::Deserializable>::deserialize(&mut bytes.as_slice(), 0).unwrap();
        let public = SignatureVerifyingKey::ECDSA(public);
        RoleBinding {
            role,
            public_key_hex: codec::native_hex(&public).unwrap(),
            address_hex: codec::native_hex(&UserAddress::from(public)).unwrap(),
        }
    }).collect()
}

fn single_payer_descriptor(
    source: &str,
    program: &Program,
    payer: &str,
    roles: Vec<RoleBinding>,
) -> Deployment {
    assert!(
        program.actions.iter().flat_map(|a| &a.statements).all(|s| {
            match s {
                Statement::Transfer { from, .. } | Statement::Fee { from, .. } => from == payer,
                _ => true,
            }
        }),
        "never build the coherent fixture from the mixed-debit source"
    );
    let mut duties = BTreeMap::new();
    for statement in program.actions.iter().flat_map(|a| &a.statements) {
        if let Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = statement
        {
            duties.insert(
                name.clone(),
                DutyBinding {
                    name: name.clone(),
                    asset: program
                        .assets
                        .iter()
                        .find(|a| &a.name == asset)
                        .unwrap()
                        .identity
                        .clone(),
                    debtor: debtor.clone(),
                    creditor: creditor.clone(),
                },
            );
        }
    }
    // The public source/ABI is coherent. Native frame fields intentionally have
    // no predecessor/parameter material: they must never be decoded by this test.
    let mut descriptor = Deployment {
        profile: PROFILE.into(),
        abi: ABI.into(),
        qualification: "local-unproven".into(),
        phase_policy: "guaranteed-only".into(),
        source_hash: codec::hash(source.as_bytes()),
        program_hash: codec::object_hash(program).unwrap(),
        abi_hash: String::new(),
        templates_hash: layout::templates_hash(),
        toolchain: Toolchain::pinned(),
        limits: Limits::supported(),
        network: "local".into(),
        anchor: "local-unqualified".into(),
        address_hex: String::new(),
        payer: payer.into(),
        predecessor_hash: String::new(),
        native_state_hash: String::new(),
        parameters_hash: String::new(),
        gas_hex: String::new(),
        operations_hash: String::new(),
        maintenance_hash: String::new(),
        balance_hash: String::new(),
        assets: program
            .assets
            .iter()
            .map(|a| AssetBinding {
                identity: a.identity.clone(),
                token_hex: codec::native_hex(&UnshieldedTokenType(layout::asset_digest(
                    &a.identity,
                )))
                .unwrap(),
            })
            .collect(),
        roles,
        duties: duties.into_values().collect(),
        economic_bounds: None,
    };
    let abi = layout::admit_source(program, &descriptor)
        .expect("the single-payer fixture must admit before its hashes are rebound");
    descriptor.abi_hash = codec::object_hash(&abi).unwrap();
    assert_eq!(
        descriptor.abi_hash,
        codec::object_hash(&layout::admit_source(program, &descriptor).unwrap()).unwrap()
    );
    descriptor
}

fn rebind_only_source_identity(
    descriptor: &Deployment,
    mixed: &str,
    program: &Program,
) -> Deployment {
    let mut rebound = descriptor.clone();
    rebound.source_hash = codec::hash(mixed.as_bytes());
    rebound.program_hash = codec::object_hash(program).unwrap();
    let mut original = serde_json::to_value(descriptor).unwrap();
    original["source_hash"] = serde_json::json!(rebound.source_hash);
    original["program_hash"] = serde_json::json!(rebound.program_hash);
    assert_eq!(original, serde_json::to_value(&rebound).unwrap());
    assert_ne!(rebound.source_hash, descriptor.source_hash);
    assert_ne!(rebound.program_hash, descriptor.program_hash);
    // The variant's ABI hash deliberately remains; the strict payer check must
    // take precedence over any later ABI comparison, not fail as a stale hash.
    rebound
}

fn request(
    program: &Program,
    action: &str,
    inputs: Option<BTreeMap<String, Value>>,
) -> CallRequest {
    let selected = program.actions.iter().find(|a| a.name == action).unwrap();
    let inputs = inputs.unwrap_or_else(|| {
        selected
            .inputs
            .iter()
            .map(|i| {
                (
                    i.name.clone(),
                    match i.ty {
                        Type::UInt => Value::UInt(UInt(5)),
                        Type::Bool => Value::Bool(true),
                    },
                )
            })
            .collect()
    });
    assert_eq!(inputs.len(), selected.inputs.len());
    assert!(
        selected
            .inputs
            .iter()
            .all(|i| inputs.get(&i.name).is_some_and(|v| v.ty() == i.ty))
    );
    CallRequest {
        action: action.into(),
        inputs,
        selected_spends: Vec::new(),
        segment: 1,
        ttl_hex: codec::native_hex(&Timestamp::from_secs(100)).unwrap(),
        nonce: UInt(1),
        block_context_hex: codec::native_hex(&BlockContext {
            tblock: Timestamp::from_secs(1),
            tblock_err: 0,
            parent_block_hash: HashOutput([0; 32]),
            last_block_time: Timestamp::from_secs(0),
        })
        .unwrap(),
        communication_randomness_hex: codec::native_hex(&Fr::from(0u64)).unwrap(),
        binding_randomness_hex: codec::native_hex(&PedersenRandomness::default()).unwrap(),
        economic_bounds: None,
    }
}

fn exact_payer_refusal<T>(label: &str, result: mori_ledger::Result<T>) {
    let error = result
        .err()
        .unwrap_or_else(|| panic!("{label}: strict mixed-debit source unexpectedly admitted"));
    assert_eq!(error.layer, "source-profile", "{label}: {error}");
    assert_eq!(error.code, "one-public-payer", "{label}: {error}");
    assert_eq!(
        error.to_string(),
        "mori::ledger::source-profile: one-public-payer"
    );
    println!("{label}: source-profile/one-public-payer; no native material supplied");
}

fn assert_public_entries_refuse(
    source: &str,
    program: &Program,
    descriptor: &Deployment,
    request: &CallRequest,
) {
    // Proven order: compile calls layout::admit_source before compile_bindings;
    // admit invokes compile before artifact identity; prepare_native_call invokes
    // compile before any checked/native preparation. Empty slices cannot satisfy
    // a later native decode, so a decode error is a test failure, not a success.
    exact_payer_refusal(
        &format!("{}/compile", request.action),
        relation::compile(source, program, descriptor, &request.action),
    );
    exact_payer_refusal(
        &format!("{}/admit", request.action),
        relation::admit(source, program, descriptor, &request.action, &[], &[]),
    );
    exact_payer_refusal(
        &format!("{}/prepare_native_call", request.action),
        prepare::prepare_native_call(source, program, descriptor, &[], &[], request),
    );
}

#[test]
fn cross_action_mixed_debits_refuse_even_when_selected_action_uses_the_payer() {
    let mixed = mori_core::parse(CROSS_ACTION).unwrap();
    let safe = CROSS_ACTION.replace("transfer(A, \"pool\"", "transfer(A, \"alice\"");
    let supported = mori_core::parse(&safe).unwrap();
    let descriptor = single_payer_descriptor(
        &safe,
        &supported,
        "alice",
        public_roles(&supported, "alice"),
    );
    let rebound = rebind_only_source_identity(&descriptor, CROSS_ACTION, &mixed);
    for action in ["fund", "claim", "idle"] {
        assert_public_entries_refuse(
            CROSS_ACTION,
            &mixed,
            &rebound,
            &request(&mixed, action, None),
        );
    }
    // admit_local_seed has no selected action and invokes admit_source before
    // either predecessor or parameter decoding, so one call covers that entry.
    exact_payer_refusal(
        "cross-action/admit_local_seed",
        local_deployment::admit_local_seed(CROSS_ACTION, &mixed, &rebound, &[], &[]),
    );
}

#[test]
fn same_action_transfer_and_fee_from_different_owners_refuse() {
    let mixed = mori_core::parse(SAME_ACTION_FEE).unwrap();
    let safe = SAME_ACTION_FEE.replace("fee(A, \"pool\"", "fee(A, \"alice\"");
    let supported = mori_core::parse(&safe).unwrap();
    let descriptor = single_payer_descriptor(
        &safe,
        &supported,
        "alice",
        public_roles(&supported, "alice"),
    );
    let rebound = rebind_only_source_identity(&descriptor, SAME_ACTION_FEE, &mixed);
    assert_public_entries_refuse(
        SAME_ACTION_FEE,
        &mixed,
        &rebound,
        &request(&mixed, "settle", None),
    );
    exact_payer_refusal(
        "same-action-fee/admit_local_seed",
        local_deployment::admit_local_seed(SAME_ACTION_FEE, &mixed, &rebound, &[], &[]),
    );
}

#[test]
fn all_actual_fee_pool_actions_retain_strict_native_source_admission() {
    let mixed = mori_core::parse(FEE_POOL).unwrap();
    let safe = FEE_POOL.replace(
        "transfer(FEES, \"funding-source\"",
        "transfer(FEES, \"fee-pool\"",
    );
    let supported = mori_core::parse(&safe).unwrap();
    let public: StaticBindings = serde_json::from_str(include_str!(
        "../../../examples/core/fee-pool/static-bindings.json"
    ))
    .unwrap();
    let descriptor = single_payer_descriptor(&safe, &supported, "fee-pool", public.roles);
    let rebound = rebind_only_source_identity(&descriptor, FEE_POOL, &mixed);
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/core/fee-pool");
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("cases/expected.json")).unwrap()).unwrap();
    assert_eq!(mixed.actions.len(), 23);
    for action in &mixed.actions {
        // Accepted independent local cases supply type-correct action inputs;
        // they do not supply a native predecessor, funds or authorization here.
        let case = manifest["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["action"] == action.name && case["expected"]["status"] == "accepted")
            .unwrap();
        let scenario: Scenario = serde_json::from_slice(
            &std::fs::read(root.join(case["scenario"].as_str().unwrap())).unwrap(),
        )
        .unwrap();
        assert_public_entries_refuse(
            FEE_POOL,
            &mixed,
            &rebound,
            &request(&mixed, &action.name, Some(scenario.inputs)),
        );
    }
    exact_payer_refusal(
        "fee-pool/admit_local_seed",
        local_deployment::admit_local_seed(FEE_POOL, &mixed, &rebound, &[], &[]),
    );
}
