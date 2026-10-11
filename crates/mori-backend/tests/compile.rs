use mori_backend::{Target, compile};
use mori_core::*;

fn increment() -> Program {
    Program {
        language: LANGUAGE.into(),
        semantics: SEMANTICS.into(),
        name: "Counter".into(),
        states: vec![StateDecl {
            name: "count".into(),
            ty: Type::UInt,
            initial: Value::UInt(UInt(0)),
        }],
        assets: vec![],
        expressions: vec![
            Expr::Name {
                name: "count".into(),
            },
            Expr::Name {
                name: "amount".into(),
            },
            Expr::Binary {
                op: BinaryOp::Add,
                lhs: 0,
                rhs: 1,
            },
        ],
        actions: vec![Action {
            name: "increment".into(),
            inputs: vec![InputDecl {
                name: "amount".into(),
                ty: Type::UInt,
            }],
            statements: vec![Statement::Next {
                state: "count".into(),
                value: 2,
            }],
        }],
    }
}
#[test]
fn emits_real_upstream_ir_with_public_pre_and_post_state() {
    let artifact = compile(&increment(), &Target::ledger9("local", "test-anchor")).unwrap();
    assert_eq!(artifact.circuits.len(), 1);
    let ir = midnight_zkir_v3::IrSource::load(artifact.circuits[0].zkir.as_bytes()).unwrap();
    assert!(ir.instructions.len() > 4);
    assert!(
        artifact.circuits[0]
            .public_inputs
            .iter()
            .any(|p| p.role == "pre_state")
    );
    assert!(
        artifact.circuits[0]
            .public_inputs
            .iter()
            .any(|p| p.role == "post_state")
    );
}
#[test]
fn rejects_target_revision_substitution() {
    let mut target = Target::ledger9("local", "test-anchor");
    target.ledger_revision = "unreviewed".into();
    assert!(compile(&increment(), &target).is_err());
}

use midnight_transient_crypto::{
    curve::Fr,
    proofs::{KeyLocation, ProofPreimage},
};
use mori_backend::CircuitArtifact;
use std::collections::BTreeMap;

fn fr(value: &Value) -> Fr {
    match value {
        Value::UInt(n) => Fr::from(n.0),
        Value::Bool(b) => Fr::from(*b),
    }
}
fn uint(n: u128) -> Value {
    Value::UInt(UInt(n))
}
fn public_values(circuit: &CircuitArtifact, scenario: &Scenario, execution: &Execution) -> Vec<Fr> {
    circuit
        .public_inputs
        .iter()
        .map(|binding| match binding.role.as_str() {
            "context" => Fr::from(binding.constant.as_ref().unwrap().parse::<u128>().unwrap()),
            "pre_state" => fr(&execution.pre.state[&binding.name]),
            "post_state" => fr(&execution.post.state[&binding.name]),
            "input" => fr(&scenario.inputs[&binding.name]),
            "pre_balance" | "post_balance" => {
                let (_, account): (String, String) = serde_json::from_str(&binding.name).unwrap();
                let snapshot = if binding.role == "pre_balance" {
                    &execution.pre
                } else {
                    &execution.post
                };
                Fr::from(
                    snapshot
                        .balances
                        .iter()
                        .find(|b| b.account == account && Some(&b.asset) == binding.asset.as_ref())
                        .unwrap()
                        .amount
                        .0,
                )
            }
            "effect" => {
                let (step, field) = binding.name.split_once('.').unwrap();
                let step = step.parse::<usize>().unwrap();
                match &execution.steps[step].effects[0] {
                    Effect::State { before, after, .. } => {
                        fr(if field == "before" { before } else { after })
                    }
                    Effect::Emit { value, .. } => fr(value),
                    Effect::Transfer { amount, .. } | Effect::Fee { amount, .. } => {
                        Fr::from(amount.0)
                    }
                    _ => panic!("unsupported test effect"),
                }
            }
            role => panic!("unexpected ABI role {role}"),
        })
        .collect()
}
fn check(
    circuit: &CircuitArtifact,
    values: &[Fr],
) -> Result<Vec<Option<usize>>, midnight_transient_crypto::proofs::ProvingError> {
    let ir = midnight_zkir_v3::IrSource::load(circuit.zkir.as_bytes()).unwrap();
    let preimage = ProofPreimage {
        inputs: vec![],
        private_transcript: vec![],
        public_transcript_inputs: values.to_vec(),
        public_transcript_outputs: values.to_vec(),
        binding_input: Fr::from(0u64),
        communications_commitment: None,
        key_location: KeyLocation("local-test-unqualified".into()),
    };
    preimage.check(&ir)
}
#[test]
fn authored_counter_relations_match_reference_and_refuse_changed_state() {
    let source = include_str!("programs/counter.mori");
    let program = parse(source).unwrap();
    let artifact = compile(&program, &Target::ledger9("local", "counter-anchor")).unwrap();
    assert_eq!(artifact.circuits.len(), 2);
    insta::with_settings!({ description => format!("Code:\n\n{source}"), omit_expression => true }, {
        insta::assert_debug_snapshot!("authored_counter_abi", artifact.circuits.iter().map(|c| (&c.action, c.public_inputs.iter().map(|p|(&p.role,&p.name)).collect::<Vec<_>>())).collect::<Vec<_>>());
    });
    for (action, amount) in [("increment", 7), ("reset", 0)] {
        let circuit = artifact
            .circuits
            .iter()
            .find(|c| c.action == action)
            .unwrap();
        let mut scenario = Scenario::default();
        scenario.state.insert("count".into(), uint(5));
        scenario.state.insert("enabled".into(), Value::Bool(true));
        if action == "increment" {
            scenario.inputs.insert("amount".into(), uint(amount));
        }
        let execution = simulate(&program, action, &scenario).unwrap();
        let values = public_values(circuit, &scenario, &execution);
        assert!(check(circuit, &values).is_ok());
        let mut changed = values.clone();
        let post = circuit
            .public_inputs
            .iter()
            .position(|p| p.role == "post_state" && p.name == "count")
            .unwrap();
        changed[post] = changed[post] + Fr::from(1u64);
        assert!(
            check(circuit, &changed).is_err(),
            "post state tamper must reject"
        );
        let mut changed = values.clone();
        changed[0] = changed[0] + Fr::from(1u64);
        assert!(
            check(circuit, &changed).is_err(),
            "context tamper must reject"
        );
        if action == "increment" {
            let effect = circuit
                .public_inputs
                .iter()
                .position(|p| p.role == "effect")
                .unwrap();
            let mut changed = values.clone();
            changed[effect] = changed[effect] + Fr::from(1u64);
            assert!(
                check(circuit, &changed).is_err(),
                "ordered effect tamper must reject"
            );
        }
    }
}
#[test]
fn authored_payment_constrains_complete_balance_and_fee_effects() {
    let source = include_str!("programs/payment.mori");
    let program = parse(source).unwrap();
    let artifact = compile(&program, &Target::ledger9("local", "payment-anchor")).unwrap();
    let circuit = &artifact.circuits[0];
    insta::with_settings!({ description => format!("Code:\n\n{source}"), omit_expression => true }, {
        insta::assert_debug_snapshot!("authored_payment_effects", &circuit.effect_frame);
    });
    let asset = program.assets[0].identity.clone();
    let scenario = Scenario {
        inputs: BTreeMap::from([
            ("amount".into(), uint(10)),
            ("accepted".into(), Value::Bool(true)),
        ]),
        balances: vec![
            Balance {
                account: "payer".into(),
                asset: asset.clone(),
                amount: UInt(40),
            },
            Balance {
                account: "recipient".into(),
                asset: asset.clone(),
                amount: UInt(3),
            },
            Balance {
                account: "sponsor".into(),
                asset,
                amount: UInt(0),
            },
        ],
        ..Default::default()
    };
    let execution = simulate(&program, "pay", &scenario).unwrap();
    let values = public_values(circuit, &scenario, &execution);
    assert!(check(circuit, &values).is_ok());
    assert_eq!(circuit.balance_frame.len(), 3);
    assert_eq!(
        circuit
            .effect_frame
            .iter()
            .map(|e| e.kind.as_str())
            .collect::<Vec<_>>(),
        ["transfer", "fee", "state", "emit"]
    );
    for frame in &circuit.balance_frame {
        let mut changed = values.clone();
        changed[frame.post_position as usize] =
            changed[frame.post_position as usize] + Fr::from(1u64);
        assert!(check(circuit, &changed).is_err());
    }
    let mut false_accept = values.clone();
    let input = circuit
        .public_inputs
        .iter()
        .position(|p| p.role == "input" && p.name == "accepted")
        .unwrap();
    false_accept[input] = Fr::from(false);
    assert!(check(circuit, &false_accept).is_err());
    let mut broke = values.clone();
    let payer = circuit
        .balance_frame
        .iter()
        .find(|b| b.account == "payer")
        .unwrap();
    broke[payer.pre_position as usize] = Fr::from(1u64);
    assert!(check(circuit, &broke).is_err());
}
#[test]
fn exact_integer_multiplication_matches_reference_at_limb_boundaries() {
    let mut program = increment();
    program.name = "Multiplication".into();
    program.expressions[2] = Expr::Binary {
        op: BinaryOp::Mul,
        lhs: 0,
        rhs: 1,
    };
    let circuit = compile(&program, &Target::ledger9("local", "mul-anchor"))
        .unwrap()
        .circuits
        .remove(0);
    for (a, b) in [
        (0, u128::MAX),
        (u128::MAX, 1),
        (u64::MAX as u128, u64::MAX as u128),
        (1u128 << 64, (1u128 << 64) - 1),
        (17, 19),
    ] {
        let scenario = Scenario {
            state: BTreeMap::from([("count".into(), uint(a))]),
            inputs: BTreeMap::from([("amount".into(), uint(b))]),
            ..Default::default()
        };
        let execution = simulate(&program, "increment", &scenario).unwrap();
        let values = public_values(&circuit, &scenario, &execution);
        assert!(check(&circuit, &values).is_ok(), "{a} * {b}");
    }
    for (a, b) in [
        (u128::MAX, 2),
        (1u128 << 64, 1u128 << 64),
        (u128::MAX, u128::MAX),
    ] {
        let scenario = Scenario {
            state: BTreeMap::from([("count".into(), uint(a))]),
            inputs: BTreeMap::from([("amount".into(), uint(b))]),
            ..Default::default()
        };
        assert!(simulate(&program, "increment", &scenario).is_err());
        let values = circuit
            .public_inputs
            .iter()
            .map(|p| match p.role.as_str() {
                "context" => Fr::from(p.constant.as_ref().unwrap().parse::<u128>().unwrap()),
                "pre_state" => Fr::from(a),
                "input" => Fr::from(b),
                "effect" if p.name.ends_with("before") => Fr::from(a),
                _ => Fr::from(0u64),
            })
            .collect::<Vec<_>>();
        assert!(
            check(&circuit, &values).is_err(),
            "overflow {a} * {b} must reject"
        );
    }
}
#[test]
fn exact_add_subtract_reject_overflow_underflow_and_wide_inputs() {
    for (op, a, b) in [(BinaryOp::Add, u128::MAX, 1), (BinaryOp::Sub, 0, 1)] {
        let mut program = increment();
        program.expressions[2] = Expr::Binary { op, lhs: 0, rhs: 1 };
        let circuit = compile(&program, &Target::ledger9("local", "arithmetic-anchor"))
            .unwrap()
            .circuits
            .remove(0);
        let mut values = circuit
            .public_inputs
            .iter()
            .map(|p| match p.role.as_str() {
                "context" => Fr::from(p.constant.as_ref().unwrap().parse::<u128>().unwrap()),
                "pre_state" => Fr::from(a),
                "input" => Fr::from(b),
                "effect" if p.name.ends_with("before") => Fr::from(a),
                _ => Fr::from(0u64),
            })
            .collect::<Vec<_>>();
        assert!(check(&circuit, &values).is_err());
        let pre = circuit
            .public_inputs
            .iter()
            .position(|p| p.role == "pre_state")
            .unwrap();
        values[pre] = Fr::from(u128::MAX) + Fr::from(1u64);
        assert!(check(&circuit, &values).is_err());
    }
}
#[test]
fn target_limits_invalid_ir_and_unresolved_discharge_refuse() {
    let program = increment();
    let mut target = Target::ledger9("local", "anchor");
    target.max_instructions = 3;
    assert_eq!(
        compile(&program, &target).unwrap_err().code,
        "mori::backend::limit"
    );
    target = Target::ledger9("local", "anchor");
    target.max_artifact_bytes = 3;
    assert_eq!(
        compile(&program, &target).unwrap_err().code,
        "mori::backend::limit"
    );
    target = Target::ledger9("local", "");
    assert_eq!(
        compile(&program, &target).unwrap_err().code,
        "mori::backend::target"
    );
    target = Target::ledger9("local", "anchor");
    let mut invalid = program.clone();
    invalid.expressions[2] = Expr::Binary {
        op: BinaryOp::Add,
        lhs: 2,
        rhs: 1,
    };
    assert_eq!(
        compile(&invalid, &target).unwrap_err().code,
        "mori::backend::program"
    );
    let mut unsupported = program.clone();
    unsupported.actions[0]
        .statements
        .push(Statement::Discharge {
            name: "loan".into(),
            amount: 1,
        });
    assert_eq!(
        compile(&unsupported, &target).unwrap_err().code,
        "mori::backend::unsupported"
    );
    let first = compile(&program, &target).unwrap();
    target.anchor = "different-anchor".into();
    let second = compile(&program, &target).unwrap();
    assert_ne!(first.circuits[0].zkir, second.circuits[0].zkir);
    assert_eq!(
        first.resources.artifact_bytes as usize,
        serde_json::to_vec_pretty(&first).unwrap().len()
    );
    assert!(!first.resources.cost_measured);
    assert!(first.resources.circuit_k.is_none());
    assert!(first.gates.iter().all(|g| g.status == "unperformed"));
}
#[test]
fn compiler_identity_matches_canonical_owner_intent_identity() {
    let program = increment();
    let artifact = compile(&program, &Target::ledger9("local", "anchor")).unwrap();
    assert_eq!(
        artifact.program_hash,
        mori_core::intent::hash_bytes(&mori_core::intent::canonical(&program).unwrap())
    );
}
proptest::proptest! {
    #![proptest_config(proptest::test_runner::Config::with_cases(64))]
    #[test]
    fn uint128_multiplication_relation_agrees_with_checked_arithmetic(a in proptest::prelude::any::<u128>(), b in proptest::prelude::any::<u128>()) {
        let mut program=increment();program.expressions[2]=Expr::Binary{op:BinaryOp::Mul,lhs:0,rhs:1};
        let circuit=compile(&program,&Target::ledger9("local","property-anchor")).unwrap().circuits.remove(0);
        let result=a.checked_mul(b);
        let values=circuit.public_inputs.iter().map(|p|match p.role.as_str(){"context"=>Fr::from(p.constant.as_ref().unwrap().parse::<u128>().unwrap()),"pre_state"=>Fr::from(a),"input"=>Fr::from(b),"effect" if p.name.ends_with("before")=>Fr::from(a),_=>Fr::from(result.unwrap_or(0))}).collect::<Vec<_>>();
        proptest::prop_assert_eq!(check(&circuit,&values).is_ok(),result.is_some());
    }
    #[test]
    fn uint64_limb_products_are_accepted_exactly(a in proptest::prelude::any::<u64>(), b in proptest::prelude::any::<u64>()) {
        let mut program=increment();program.expressions[2]=Expr::Binary{op:BinaryOp::Mul,lhs:0,rhs:1};
        let circuit=compile(&program,&Target::ledger9("local","property-anchor")).unwrap().circuits.remove(0);
        let result=u128::from(a)*u128::from(b);
        let values=circuit.public_inputs.iter().map(|p|match p.role.as_str(){"context"=>Fr::from(p.constant.as_ref().unwrap().parse::<u128>().unwrap()),"pre_state"=>Fr::from(a),"input"=>Fr::from(b),"effect" if p.name.ends_with("before")=>Fr::from(a),_=>Fr::from(result)}).collect::<Vec<_>>();
        proptest::prop_assert!(check(&circuit,&values).is_ok());
    }
    #[test]
    fn malformed_flat_handles_never_panic(lhs in proptest::prelude::any::<u16>(), rhs in proptest::prelude::any::<u16>()) {
        let mut program=increment();program.expressions[2]=Expr::Binary{op:BinaryOp::Add,lhs,rhs};
        let result=compile(&program,&Target::ledger9("local","property-anchor"));
        if lhs>=2 || rhs>=2 { proptest::prop_assert!(result.is_err()); }
    }
}
#[test]
fn cross_network_asset_target_is_refused() {
    let mut program = increment();
    program.assets.push(AssetDecl {
        name: "USD".into(),
        identity: AssetId {
            network: "foreign-domain".into(),
            issuer: "issuer".into(),
            reference: "usd".into(),
            scale: 2,
        },
    });
    assert_eq!(
        compile(&program, &Target::ledger9("local", "anchor"))
            .unwrap_err()
            .code,
        "mori::backend::target"
    );
}
#[test]
fn artifact_limit_counts_abi_and_envelope_not_only_zkir_text() {
    let program = increment();
    let mut target = Target::ledger9("local", "anchor");
    let unlimited = compile(&program, &target).unwrap();
    target.max_artifact_bytes = unlimited.resources.zkir_bytes + 500;
    assert_eq!(
        compile(&program, &target).unwrap_err().code,
        "mori::backend::limit"
    );
}
