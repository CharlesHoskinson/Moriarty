#![allow(dead_code)] // Shared public fixtures are used by several scoped targets.
use midnight_base_crypto::{
    cost_model::{CostDuration, RunningCost},
    hash::HashOutput,
    time::Timestamp,
};
use midnight_coin_structure::{
    coin::{UnshieldedTokenType, UserAddress},
    contract::ContractAddress,
};
use midnight_ledger::structure::{
    IntentHash, LedgerState, SignatureVerifyingKey, Utxo, UtxoMeta, UtxoSpend,
};
use midnight_onchain_state::state::{ContractOperation, ContractState, StateValue};
use midnight_storage::{arena::Sp, db::InMemoryDB};
use mori_core::{Program, Scenario, Statement};
use mori_ledger::{codec, layout, profile::*};
use std::collections::BTreeSet;
#[derive(Clone)]
pub struct Fixture {
    pub source: String,
    pub program: Program,
    pub deployment: Deployment,
    pub request: CallRequest,
    pub predecessor: Vec<u8>,
    pub parameters: Vec<u8>,
}
pub fn repin(f: &mut Fixture, ledger: &LedgerState<InMemoryDB>) {
    let address: ContractAddress = codec::from_hex(&f.deployment.address_hex).unwrap();
    let contract = ledger.index(address).unwrap();
    f.predecessor = codec::encode(ledger).unwrap();
    f.parameters = codec::encode(&*ledger.parameters).unwrap();
    f.deployment.predecessor_hash = codec::hash(&f.predecessor);
    f.deployment.native_state_hash = codec::native_hex(&ledger.state_hash()).unwrap();
    f.deployment.parameters_hash = codec::hash(&f.parameters);
    f.deployment.operations_hash = codec::native_hash(&contract.operations).unwrap();
    f.deployment.maintenance_hash = codec::native_hash(&contract.maintenance_authority).unwrap();
    f.deployment.balance_hash = codec::native_hash(&contract.balance).unwrap();
    f.deployment.abi_hash =
        codec::object_hash(&layout::admit_source(&f.program, &f.deployment).unwrap()).unwrap();
}
pub fn payment() -> Fixture {
    let source = include_str!(
        "../../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori"
    )
    .to_string();
    let scenario:Scenario=serde_json::from_str(include_str!("../../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json")).unwrap();
    fixture(&source, "settle", scenario)
}
// Public SEC1 verifier points only. No signing key, RNG, keygen, wallet or proof.
fn public_key(index: usize) -> SignatureVerifyingKey {
    if index == 0 {
        return SignatureVerifyingKey::default();
    }
    let points = [
        "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9",
        "02e493dbf1c10d80f3581e4904930b1404cc6c13900ee0758474fa94abe8c4cd13",
        "022f8bde4d1a07209355b4a7250a5c5128e88b84bddc619ab7cba8d569b240efe4",
    ];
    let bytes = codec::unhex(points[index - 1]).unwrap();
    let mut slice = bytes.as_slice();
    let key=<midnight_base_crypto::ecdsa::VerifyingKey as midnight_serialize::Deserializable>::deserialize(&mut slice,0).unwrap();
    SignatureVerifyingKey::ECDSA(key)
}
pub fn fixture(source: &str, action: &str, scenario: Scenario) -> Fixture {
    let program = mori_core::parse(source).unwrap();
    let mut names: BTreeSet<String> = scenario
        .balances
        .iter()
        .map(|b| b.account.clone())
        .collect();
    names.insert("alice".into());
    for a in &program.actions {
        for s in &a.statements {
            match s {
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
    }
    for l in &scenario.liabilities {
        names.insert(l.debtor.clone());
        names.insert(l.creditor.clone());
    }
    let roles: Vec<_> = names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let vk = public_key(i);
            let address = UserAddress::from(vk.clone());
            RoleBinding {
                role: name.clone(),
                public_key_hex: codec::native_hex(&vk).unwrap(),
                address_hex: codec::native_hex(&address).unwrap(),
            }
        })
        .collect();
    let assets: Vec<_> = program
        .assets
        .iter()
        .map(|a| AssetBinding {
            identity: a.identity.clone(),
            token_hex: codec::native_hex(&UnshieldedTokenType(layout::asset_digest(&a.identity)))
                .unwrap(),
        })
        .collect();
    let mut duties: std::collections::BTreeMap<String, DutyBinding> = scenario
        .liabilities
        .iter()
        .map(|l| {
            (
                l.name.clone(),
                DutyBinding {
                    name: l.name.clone(),
                    asset: l.asset.clone(),
                    debtor: l.debtor.clone(),
                    creditor: l.creditor.clone(),
                },
            )
        })
        .collect();
    for a in &program.actions {
        for s in &a.statements {
            if let Statement::Liability {
                name,
                asset,
                debtor,
                creditor,
                ..
            } = s
            {
                let id = program
                    .assets
                    .iter()
                    .find(|a| &a.name == asset)
                    .unwrap()
                    .identity
                    .clone();
                duties.insert(
                    name.clone(),
                    DutyBinding {
                        name: name.clone(),
                        asset: id,
                        debtor: debtor.clone(),
                        creditor: creditor.clone(),
                    },
                );
            }
        }
    }
    let state_map = program
        .states
        .iter()
        .map(|s| {
            let v = scenario.state.get(&s.name).unwrap_or(&s.initial);
            (
                layout::name_key("state", &s.name),
                mori_ledger::ops::cell(layout::value(v)),
            )
        })
        .collect();
    let duty_map = scenario
        .liabilities
        .iter()
        .map(|l| {
            let binding = &duties[&l.name];
            (
                layout::name_key("duty", &l.name),
                mori_ledger::ops::cell(layout::duty_value(binding, l.amount.0, &roles).unwrap()),
            )
        })
        .collect();
    let root = StateValue::Array(
        vec![
            StateValue::Map(state_map),
            StateValue::Map(duty_map),
            StateValue::Map(Default::default()),
        ]
        .into(),
    );
    let operations = program
        .actions
        .iter()
        .map(|a| (a.name.as_bytes().into(), ContractOperation::new(None, None)))
        .collect();
    let contract = ContractState::new(root, operations, Default::default());
    let address = ContractAddress(HashOutput([7; 32]));
    let mut ledger = LedgerState::<InMemoryDB>::new("local");
    ledger.contract = ledger.contract.insert(address, contract.clone());
    let mut utxo = (*ledger.utxo).clone();
    let mut selected = Vec::new();
    for (i, b) in scenario
        .balances
        .iter()
        .filter(|b| b.amount.0 > 0)
        .enumerate()
    {
        let rb = roles.iter().find(|r| r.role == b.account).unwrap();
        let vk: SignatureVerifyingKey = codec::from_hex(&rb.public_key_hex).unwrap();
        let token: UnshieldedTokenType = codec::from_hex(
            &assets
                .iter()
                .find(|a| a.identity == b.asset)
                .unwrap()
                .token_hex,
        )
        .unwrap();
        let u = Utxo {
            owner: UserAddress::from(vk.clone()),
            value: b.amount.0,
            type_: token,
            intent_hash: IntentHash(HashOutput([(i + 1) as u8; 32])),
            output_no: 0,
        };
        utxo.utxos = utxo.utxos.insert(
            u.clone(),
            UtxoMeta {
                ctime: Timestamp::from_secs(0),
            },
        );
        if b.account == "alice" {
            selected.push(
                codec::native_hex(&UtxoSpend {
                    owner: vk,
                    value: u.value,
                    type_: u.type_,
                    intent_hash: u.intent_hash,
                    output_no: u.output_no,
                })
                .unwrap(),
            );
        }
    }
    ledger.utxo = Sp::new(utxo);
    ledger.check_night_balance_invariant().unwrap();
    let predecessor = codec::encode(&ledger).unwrap();
    let parameters = codec::encode(&*ledger.parameters).unwrap();
    let gas = RunningCost {
        compute_time: CostDuration::SECOND,
        read_time: CostDuration::SECOND,
        bytes_written: 1_000_000,
        bytes_deleted: 1_000_000,
    };
    let mut deployment = Deployment {
        profile: PROFILE.into(),
        abi: ABI.into(),
        qualification: "local-unproven".into(),
        phase_policy: "guaranteed-only".into(),
        source_hash: codec::hash(source.as_bytes()),
        program_hash: codec::object_hash(&program).unwrap(),
        abi_hash: String::new(),
        templates_hash: layout::templates_hash(),
        toolchain: Toolchain::pinned(),
        limits: Limits::supported(),
        network: "local".into(),
        anchor: "local-unqualified".into(),
        address_hex: codec::native_hex(&address).unwrap(),
        payer: "alice".into(),
        predecessor_hash: codec::hash(&predecessor),
        native_state_hash: codec::native_hex(&ledger.state_hash()).unwrap(),
        parameters_hash: codec::hash(&parameters),
        gas_hex: codec::native_hex(&gas).unwrap(),
        operations_hash: codec::native_hash(&contract.operations).unwrap(),
        maintenance_hash: codec::native_hash(&contract.maintenance_authority).unwrap(),
        balance_hash: codec::native_hash(&contract.balance).unwrap(),
        assets,
        roles,
        duties: duties.into_values().collect(),
        economic_bounds: None,
    };
    deployment.abi_hash =
        codec::object_hash(&layout::admit_source(&program, &deployment).unwrap()).unwrap();
    let request = CallRequest {
        action: action.into(),
        inputs: scenario.inputs,
        selected_spends: selected,
        segment: 1,
        ttl_hex: codec::native_hex(&Timestamp::from_secs(100)).unwrap(),
        nonce: mori_core::UInt(1),
        block_context_hex: codec::native_hex(&midnight_onchain_runtime::context::BlockContext {
            tblock: Timestamp::from_secs(1),
            tblock_err: 0,
            parent_block_hash: HashOutput([0; 32]),
            last_block_time: Timestamp::from_secs(0),
        })
        .unwrap(),
        communication_randomness_hex: codec::native_hex(
            &midnight_transient_crypto::curve::Fr::from(0u64),
        )
        .unwrap(),
        binding_randomness_hex: codec::native_hex(
            &midnight_transient_crypto::commitment::PedersenRandomness::default(),
        )
        .unwrap(),
        economic_bounds: None,
    };
    Fixture {
        source: source.into(),
        program,
        deployment,
        request,
        predecessor,
        parameters,
    }
}

pub fn collection() -> Fixture {
    let source = r#"language "moriarty/1";
 agreement CounterflowCollection43 {
 asset QA = { network: "local", issuer: "counterflow-qa", reference: "QA", scale: 0 };
 state collected: UInt = 0; state frozen: Bool = true;
 action seed() { liability("addon", QA, "alice", "carol", 0); liability("payment", QA, "alice", "carol", 12); liability("reserve", QA, "alice", "auditor", 5); liability("zero", QA, "alice", "auditor", 0); }
 action collect(amount: UInt) { fee(QA,"alice","treasury",3); transfer(QA,"alice","carol",amount); liability("addon",QA,"alice","carol",4); discharge("payment",7); next collected = collected + amount; emit "collected"(collected); ensures collected == 40; }
 }"#;
    let p = mori_core::parse(source).unwrap();
    let asset = p.assets[0].identity.clone();
    let scenario = Scenario {
        state: Default::default(),
        inputs: [("amount".into(), mori_core::Value::UInt(mori_core::UInt(40)))]
            .into_iter()
            .collect(),
        balances: [
            ("alice", 100),
            ("carol", 7),
            ("spectator", 9),
            ("treasury", 1),
        ]
        .into_iter()
        .map(|(a, n)| mori_core::Balance {
            account: a.into(),
            asset: asset.clone(),
            amount: mori_core::UInt(n),
        })
        .collect(),
        liabilities: [
            ("addon", 0, "carol"),
            ("payment", 12, "carol"),
            ("reserve", 5, "auditor"),
            ("zero", 0, "auditor"),
        ]
        .into_iter()
        .map(|(name, n, creditor)| mori_core::Liability {
            name: name.into(),
            asset: asset.clone(),
            debtor: "alice".into(),
            creditor: creditor.into(),
            amount: mori_core::UInt(n),
        })
        .collect(),
    };
    fixture(source, "collect", scenario)
}
pub fn cash(statements: &str, budget: u128) -> Fixture {
    let source = format!(
        "language \"moriarty/1\"; agreement GeneralCash {{ asset QA = {{network:\"local\", issuer:\"qa\", reference:\"QA\", scale:0}}; state count: UInt = 0; action run() {{ {statements} }} }}"
    );
    let p = mori_core::parse(&source).unwrap();
    let asset = p.assets[0].identity.clone();
    let scenario = Scenario {
        state: Default::default(),
        inputs: Default::default(),
        balances: [("alice", budget), ("bob", 0), ("treasury", 0)]
            .into_iter()
            .map(|(a, n)| mori_core::Balance {
                account: a.into(),
                asset: asset.clone(),
                amount: mori_core::UInt(n),
            })
            .collect(),
        liabilities: vec![],
    };
    fixture(&source, "run", scenario)
}
pub fn multiple_assets() -> Fixture {
    let source = r#"language "moriarty/1"; agreement MultiAssetRoute {
 asset QA = {network:"local",issuer:"qa",reference:"QA",scale:0};
 asset BOND = {network:"local",issuer:"bond",reference:"BOND",scale:6};
 action run(amount: UInt) { transfer(QA,"alice","bob",amount); fee(BOND,"alice","treasury",3); liability("note",BOND,"alice","bob",5); }
 }"#;
    let p = mori_core::parse(source).unwrap();
    let qa = p
        .assets
        .iter()
        .find(|a| a.name == "QA")
        .unwrap()
        .identity
        .clone();
    let bond = p
        .assets
        .iter()
        .find(|a| a.name == "BOND")
        .unwrap()
        .identity
        .clone();
    let scenario = Scenario {
        inputs: [("amount".into(), mori_core::Value::UInt(mori_core::UInt(70)))]
            .into_iter()
            .collect(),
        balances: vec![
            mori_core::Balance {
                account: "alice".into(),
                asset: qa.clone(),
                amount: mori_core::UInt(100),
            },
            mori_core::Balance {
                account: "alice".into(),
                asset: bond,
                amount: mori_core::UInt(20),
            },
            mori_core::Balance {
                account: "bob".into(),
                asset: qa,
                amount: mori_core::UInt(2),
            },
        ],
        ..Default::default()
    };
    fixture(source, "run", scenario)
}
