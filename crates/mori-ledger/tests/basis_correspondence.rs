use midnight_base_crypto::{
    cost_model::{CostDuration, RunningCost},
    hash::HashOutput,
    time::Timestamp,
};
use midnight_coin_structure::{coin::UserAddress, contract::ContractAddress};
use midnight_ledger::structure::{LedgerState, SignatureVerifyingKey};
use midnight_onchain_state::state::{ContractOperation, ContractState, StateValue};
use midnight_storage::db::InMemoryDB;
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{KeyLocation, ProofPreimage},
};
use mori_core::{Program, Scenario, Statement, UInt, Value};
use mori_ledger::{codec, layout, prepare::prepare_call, profile::*, relation};
use serde_json::Value as Json;
use std::{borrow::Cow, collections::BTreeSet};

#[derive(Clone)]
struct Fixture {
    source: String,
    program: Program,
    deployment: Deployment,
    request: CallRequest,
    predecessor: Vec<u8>,
    parameters: Vec<u8>,
}

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
    let key = <midnight_base_crypto::ecdsa::VerifyingKey as midnight_serialize::Deserializable>::deserialize(&mut slice, 0).unwrap();
    SignatureVerifyingKey::ECDSA(key)
}

fn fixture(source: &str, action: &str, scenario: Scenario) -> Fixture {
    let program = mori_core::parse(source).unwrap();
    let mut names: BTreeSet<String> = scenario
        .balances
        .iter()
        .map(|balance| balance.account.clone())
        .collect();
    names.insert("alice".into());
    for action in &program.actions {
        for statement in &action.statements {
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
    }
    let roles = names
        .iter()
        .enumerate()
        .map(|(index, name)| {
            let key = public_key(index);
            let address = UserAddress::from(key.clone());
            RoleBinding {
                role: name.clone(),
                public_key_hex: codec::native_hex(&key).unwrap(),
                address_hex: codec::native_hex(&address).unwrap(),
            }
        })
        .collect::<Vec<_>>();
    let assets = program
        .assets
        .iter()
        .map(|asset| AssetBinding {
            identity: asset.identity.clone(),
            token_hex: codec::native_hex(&midnight_coin_structure::coin::UnshieldedTokenType(
                layout::asset_digest(&asset.identity),
            ))
            .unwrap(),
        })
        .collect::<Vec<_>>();
    let state_map = program
        .states
        .iter()
        .map(|state| {
            let value = scenario.state.get(&state.name).unwrap_or(&state.initial);
            (
                layout::name_key("state", &state.name),
                mori_ledger::ops::cell(layout::value(value)),
            )
        })
        .collect();
    let root = StateValue::Array(
        vec![
            StateValue::Map(state_map),
            StateValue::Map(Default::default()),
            StateValue::Map(Default::default()),
        ]
        .into(),
    );
    let operations = program
        .actions
        .iter()
        .map(|action| {
            (
                action.name.as_bytes().into(),
                ContractOperation::new(None, None),
            )
        })
        .collect();
    let contract = ContractState::new(root, operations, Default::default());
    let address = ContractAddress(HashOutput([7; 32]));
    let mut ledger = LedgerState::<InMemoryDB>::new("local");
    ledger.contract = ledger.contract.insert(address, contract.clone());
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
        duties: Vec::new(),
        economic_bounds: None,
    };
    deployment.abi_hash =
        codec::object_hash(&layout::admit_source(&program, &deployment).unwrap()).unwrap();
    let request = CallRequest {
        action: action.into(),
        inputs: scenario.inputs,
        selected_spends: Vec::new(),
        segment: 1,
        ttl_hex: codec::native_hex(&Timestamp::from_secs(100)).unwrap(),
        nonce: UInt(1),
        block_context_hex: codec::native_hex(&midnight_onchain_runtime::context::BlockContext {
            tblock: Timestamp::from_secs(1),
            tblock_err: 0,
            parent_block_hash: HashOutput([0; 32]),
            last_block_time: Timestamp::from_secs(0),
        })
        .unwrap(),
        communication_randomness_hex: codec::native_hex(&Fr::from(0u64)).unwrap(),
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

struct Prim {
    id: &'static str,
    uint_inputs: bool,
    result_bool: bool,
    names: &'static [&'static str],
    body: &'static str,
}

const PRIMS: &[Prim] = &[
    Prim {
        id: "add",
        uint_inputs: true,
        result_bool: false,
        names: &["a", "b"],
        body: "next out = a + b;",
    },
    Prim {
        id: "sub",
        uint_inputs: true,
        result_bool: false,
        names: &["a", "b"],
        body: "next out = a - b;",
    },
    Prim {
        id: "mul",
        uint_inputs: true,
        result_bool: false,
        names: &["a", "b"],
        body: "next out = a * b;",
    },
    Prim {
        id: "eq",
        uint_inputs: true,
        result_bool: true,
        names: &["a", "b"],
        body: "next out = a == b;",
    },
    Prim {
        id: "ne",
        uint_inputs: true,
        result_bool: true,
        names: &["a", "b"],
        body: "next out = a not = b;",
    },
    Prim {
        id: "lt",
        uint_inputs: true,
        result_bool: true,
        names: &["a", "b"],
        body: "next out = a < b;",
    },
    Prim {
        id: "gt",
        uint_inputs: true,
        result_bool: true,
        names: &["a", "b"],
        body: "next out = a > b;",
    },
    Prim {
        id: "le",
        uint_inputs: true,
        result_bool: true,
        names: &["a", "b"],
        body: "next out = a <= b;",
    },
    Prim {
        id: "ge",
        uint_inputs: true,
        result_bool: true,
        names: &["a", "b"],
        body: "next out = a >= b;",
    },
    Prim {
        id: "and",
        uint_inputs: false,
        result_bool: true,
        names: &["a", "b"],
        body: "next out = a and b;",
    },
    Prim {
        id: "or",
        uint_inputs: false,
        result_bool: true,
        names: &["a", "b"],
        body: "next out = a or b;",
    },
    Prim {
        id: "not",
        uint_inputs: false,
        result_bool: true,
        names: &["a"],
        body: "next out = not a;",
    },
];

struct Sample {
    result: Fr,
    operands: Vec<Fr>,
    preimage: ProofPreimage,
}

fn source(prim: &Prim) -> String {
    let input_ty = if prim.uint_inputs { "UInt" } else { "Bool" };
    let output_ty = if prim.result_bool { "Bool" } else { "UInt" };
    let initial = if prim.result_bool { "false" } else { "0" };
    let args = prim
        .names
        .iter()
        .map(|name| format!("{name}: {input_ty}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "language \"moriarty/1\"; agreement Basis {{ state out: {output_ty} = {initial}; action run({args}) {{ {} }} }}",
        prim.body
    )
}

fn typed_value(as_uint: bool, text: &str) -> Value {
    if as_uint {
        Value::UInt(UInt(text.parse::<u128>().expect("u128 value")))
    } else if text == "0" {
        Value::Bool(false)
    } else if text == "1" {
        Value::Bool(true)
    } else {
        panic!("bool value {text}")
    }
}

fn value_fr(value: &Value) -> Fr {
    match value {
        Value::UInt(UInt(n)) => Fr::from(*n),
        Value::Bool(bit) => Fr::from(u128::from(*bit)),
    }
}

fn fr_text(text: &str) -> Fr {
    if text == "2^128" {
        return Fr::from(1u128 << 127) * Fr::from(2u128);
    }
    Fr::from(text.parse::<u128>().expect("field decimal"))
}

fn witness(bundle: &mori_ledger::prepare::PreparedBundle, action: &str) -> ProofPreimage {
    let call: Json = serde_json::from_slice(&bundle.leaves["call.json"]).expect("call.json");
    let fields = |name: &str| {
        call[name]
            .as_array()
            .unwrap()
            .iter()
            .map(|item| Fr::from_le_bytes(&codec::unhex(item.as_str().unwrap()).unwrap()).unwrap())
            .collect()
    };
    ProofPreimage {
        inputs: fields("preimage_inputs"),
        private_transcript: vec![],
        public_transcript_inputs: fields("public_transcript_inputs"),
        public_transcript_outputs: fields("public_transcript_outputs"),
        binding_input: codec::from_hex(call["binding_input_hex"].as_str().unwrap()).unwrap(),
        communications_commitment: Some((
            codec::from_hex(call["communication_commitment_hex"].as_str().unwrap()).unwrap(),
            codec::from_hex(call["communication_randomness_hex"].as_str().unwrap()).unwrap(),
        )),
        key_location: KeyLocation(Cow::Owned(action.to_string())),
    }
}

fn bind(base: &Fixture, prim: &Prim, values: &[Value]) -> Fixture {
    let mut fixture = base.clone();
    fixture.request.inputs = prim
        .names
        .iter()
        .zip(values)
        .map(|(name, value)| ((*name).to_string(), value.clone()))
        .collect();
    fixture
}

fn scenario(fixture: &Fixture) -> Scenario {
    Scenario {
        inputs: fixture.request.inputs.clone(),
        ..Default::default()
    }
}

// Result slots are the transcript fields that carry every honest output.
fn result_slots(samples: &[Sample]) -> Result<Vec<usize>, String> {
    let width = samples[0].preimage.public_transcript_inputs.len();
    if samples
        .iter()
        .any(|sample| sample.preimage.public_transcript_inputs.len() != width)
    {
        return Err("public transcript length changed between honest outputs".into());
    }
    for left in 0..samples.len() {
        for right in left + 1..samples.len() {
            if samples[left].result == samples[right].result {
                continue;
            }
            if samples[left].operands.contains(&samples[left].result)
                || samples[right].operands.contains(&samples[right].result)
            {
                continue;
            }
            let slots = (0..width)
                .filter(|&index| {
                    samples[left].preimage.public_transcript_inputs[index] == samples[left].result
                        && samples[right].preimage.public_transcript_inputs[index]
                            == samples[right].result
                })
                .collect::<Vec<_>>();
            if slots.is_empty() {
                continue;
            }
            let tracks = samples.iter().all(|sample| {
                slots
                    .iter()
                    .all(|&index| sample.preimage.public_transcript_inputs[index] == sample.result)
            });
            if tracks {
                return Ok(slots);
            }
        }
    }
    if width >= 2 {
        let index = width - 2;
        if samples
            .iter()
            .all(|sample| sample.preimage.public_transcript_inputs[index] == sample.result)
        {
            return Ok(vec![index]);
        }
    }
    Err(format!(
        "no correlated output slot in public transcript length {width}"
    ))
}

fn forge(donor: &ProofPreimage, inputs: Vec<Fr>, slots: &[usize], output: Fr) -> ProofPreimage {
    let mut hostile = donor.clone();
    hostile.inputs = inputs;
    let randomness = hostile.communications_commitment.unwrap().1;
    hostile.communications_commitment = Some((
        midnight_transient_crypto::hash::transient_commit(&hostile.inputs[..], randomness),
        randomness,
    ));
    for &slot in slots {
        hostile.public_transcript_inputs[slot] = output;
    }
    hostile
}

fn texts<'a>(case: &'a Json, key: &str) -> Vec<&'a str> {
    case[key]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| item.as_str().unwrap())
        .collect()
}

fn exercise(prim: &Prim, cases: &[&Json], failures: &mut Vec<String>) {
    let source = source(prim);
    let base = fixture(
        &source,
        "run",
        Scenario {
            inputs: prim
                .names
                .iter()
                .map(|name| {
                    (
                        (*name).to_string(),
                        if prim.uint_inputs {
                            Value::UInt(UInt(0))
                        } else {
                            Value::Bool(false)
                        },
                    )
                })
                .collect(),
            ..Default::default()
        },
    );
    let artifact = match relation::compile(&base.source, &base.program, &base.deployment, "run") {
        Ok(artifact) => artifact,
        Err(error) => {
            failures.push(format!("{}: compile {error}", prim.id));
            return;
        }
    };
    if artifact.manifest.qualification != "ordinary-ir-unproven" {
        failures.push(format!(
            "{}: qualification {} claims more than an ordinary unchecked relation",
            prim.id, artifact.manifest.qualification
        ));
    }
    if !artifact
        .manifest
        .open_predicates
        .iter()
        .any(|item| item == "cryptographic proof and ledger settlement")
    {
        failures.push(format!(
            "{}: open predicates omit the unperformed native proof",
            prim.id
        ));
    }
    let mut samples = Vec::new();
    for case in cases.iter().copied().filter(|case| case["defined"] == true) {
        let id = case["id"].as_str().unwrap();
        let operands = texts(case, "operands");
        if operands.len() != prim.names.len() {
            failures.push(format!("{id}: operand count"));
            continue;
        }
        let values = operands
            .iter()
            .map(|text| typed_value(prim.uint_inputs, text))
            .collect::<Vec<_>>();
        let expected = typed_value(!prim.result_bool, case["expected"].as_str().unwrap());
        let fixture = bind(&base, prim, &values);
        match mori_core::simulate(&fixture.program, "run", &scenario(&fixture)) {
            Ok(execution) if execution.post.state["out"] == expected => {}
            Ok(execution) => failures.push(format!(
                "{id}: runtime output {:?} expected {expected:?}",
                execution.post.state["out"]
            )),
            Err(error) => failures.push(format!("{id}: runtime refused defined output {error}")),
        }
        let bundle = match prepare_call(
            &fixture.source,
            &fixture.program,
            &fixture.deployment,
            &fixture.predecessor,
            &fixture.parameters,
            &fixture.request,
        ) {
            Ok(bundle) => bundle,
            Err(error) => {
                failures.push(format!("{id}: prepare refused defined output {error}"));
                continue;
            }
        };
        if bundle.readback.execution.post.state["out"] != expected {
            failures.push(format!(
                "{id}: prepare output {:?} expected {expected:?}",
                bundle.readback.execution.post.state["out"]
            ));
        }
        let preimage = witness(&bundle, "run");
        let operand_fields = values.iter().map(value_fr).collect::<Vec<_>>();
        if preimage.inputs != operand_fields {
            failures.push(format!("{id}: proof inputs differ from the action operands"));
        }
        if let Err(error) = relation::check(&artifact, &preimage) {
            failures.push(format!("{id}: relation refused honest output {error}"));
        }
        samples.push((
            id.to_string(),
            case["flip_result"].as_bool() == Some(true),
            Sample {
                result: value_fr(&expected),
                operands: operand_fields,
                preimage,
            },
        ));
    }
    if samples.len() < 2 {
        failures.push(format!("{}: fewer than two honest outputs", prim.id));
        return;
    }
    let slots = match result_slots(
        &samples
            .iter()
            .map(|(_, _, sample)| Sample {
                result: sample.result,
                operands: sample.operands.clone(),
                preimage: sample.preimage.clone(),
            })
            .collect::<Vec<_>>(),
    ) {
        Ok(slots) => slots,
        Err(error) => {
            failures.push(format!("{}: {error}", prim.id));
            return;
        }
    };
    for (id, flip, sample) in &samples {
        if !flip {
            continue;
        }
        let flipped = if sample.result == Fr::from(0u128) {
            Fr::from(1u128)
        } else {
            Fr::from(0u128)
        };
        let hostile = forge(&sample.preimage, sample.operands.clone(), &slots, flipped);
        if relation::check(&artifact, &hostile).is_ok() {
            failures.push(format!("{id}: relation accepted a flipped Boolean output"));
        }
    }
    let donor = &samples[0].2.preimage;
    for case in cases.iter().copied().filter(|case| case["defined"] == false) {
        let id = case["id"].as_str().unwrap();
        let operands = texts(case, "operands");
        if operands.len() != prim.names.len() {
            failures.push(format!("{id}: operand count"));
            continue;
        }
        let control = case["control"].as_str();
        if control.is_none() {
            let values = operands
                .iter()
                .map(|text| typed_value(prim.uint_inputs, text))
                .collect::<Vec<_>>();
            let fixture = bind(&base, prim, &values);
            let wanted = case["core_code"].as_str().unwrap();
            match mori_core::simulate(&fixture.program, "run", &scenario(&fixture)) {
                Err(error) if error.code == wanted => {}
                Err(error) => failures.push(format!("{id}: runtime {error} wanted {wanted}")),
                Ok(_) => failures.push(format!("{id}: runtime accepted an undefined output")),
            }
            match prepare_call(
                &fixture.source,
                &fixture.program,
                &fixture.deployment,
                &fixture.predecessor,
                &fixture.parameters,
                &fixture.request,
            ) {
                Err(error) if error.layer == "semantic" && error.code == "core-refusal" => {}
                Err(error) => failures.push(format!("{id}: prepare refusal {error}")),
                Ok(_) => failures.push(format!("{id}: prepare accepted an undefined output")),
            }
            let inputs = values.iter().map(value_fr).collect::<Vec<_>>();
            let wrap = fr_text(case["wrap_mod_2_128"].as_str().unwrap());
            let wrapped = forge(donor, inputs.clone(), &slots, wrap);
            if relation::check(&artifact, &wrapped).is_ok() {
                failures.push(format!(
                    "{id}: relation accepted the correlated 2^128 wrap"
                ));
            }
            if let Some(image) = case["field_image"].as_str() {
                let image = fr_text(image);
                if inputs.len() != 2 || inputs[0] * inputs[1] != image {
                    failures.push(format!(
                        "{id}: field image is not the scalar product of the operands"
                    ));
                }
                if image == wrap {
                    failures.push(format!(
                        "{id}: scalar-field image is the same residue as the 2^128 wrap"
                    ));
                }
                let field_witness = forge(donor, inputs, &slots, image);
                if relation::check(&artifact, &field_witness).is_ok() {
                    failures.push(format!(
                        "{id}: relation accepted the scalar-field modular image"
                    ));
                }
            }
            continue;
        }
        let inputs = operands.iter().map(|text| fr_text(text)).collect::<Vec<_>>();
        let claimed = Fr::from(0u128);
        let hostile = forge(donor, inputs, &slots, claimed);
        if relation::check(&artifact, &hostile).is_ok() {
            failures.push(format!(
                "{id}: relation accepted an input outside the primitive domain"
            ));
        }
    }
}

#[test]
fn checked_u128_basis_correspondence_corpus() {
    let corpus: Json =
        serde_json::from_str(include_str!("fixtures/basis-vectors.json")).expect("basis vectors");
    assert_eq!(corpus["format"], "moriarty-checked-u128-basis-vectors/1");
    assert_eq!(
        corpus["qualification"],
        "ordinary-source/reference/relation-check-only-no-native-certification"
    );
    assert_eq!(corpus["certificate_status"], "unqualified");
    assert_eq!(corpus["width_bits"], 128);
    assert_eq!(corpus["limb_bits"], 64);
    let cases = corpus["cases"].as_array().expect("cases");
    for class in [
        "limb-boundary",
        "modular-wrap",
        "overflow",
        "underflow",
        "definedness",
        "defined",
    ] {
        assert!(
            cases.iter().any(|case| case["class"] == class),
            "missing class {class}"
        );
    }
    for prim in PRIMS {
        assert!(
            cases.iter().any(|case| case["primitive"] == prim.id),
            "missing primitive {}",
            prim.id
        );
    }
    let mut failures = Vec::new();
    for prim in PRIMS {
        let owned = cases
            .iter()
            .filter(|case| case["primitive"] == prim.id)
            .collect::<Vec<_>>();
        exercise(prim, &owned, &mut failures);
    }
    assert!(
        failures.is_empty(),
        "basis correspondence failures:\n{}",
        failures.join("\n")
    );
}
