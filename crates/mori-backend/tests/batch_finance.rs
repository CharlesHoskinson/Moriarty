//! Compiler relation checks for the exact batch/swap source.
//! `ProofPreimage::check` is an ordinary constraint check. It is not a native certificate.
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{KeyLocation, ProofPreimage},
};
use mori_backend::{CircuitArtifact, Target, compile};
use mori_core::*;
use std::path::Path;

fn program() -> Program {
    parse(include_str!(
        "../../../examples/core/batch-finance/batch.mori"
    ))
    .expect("batch source")
}
fn fixture(name: &str) -> Scenario {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/core/batch-finance")
        .join(name);
    serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
}
fn field(value: &Value) -> Fr {
    match value {
        Value::UInt(n) => Fr::from(n.0),
        Value::Bool(bit) => Fr::from(*bit),
    }
}
fn debt(snapshot: &Snapshot, name: &str, presence: bool) -> Fr {
    let entry = snapshot.liabilities.iter().find(|item| item.name == name);
    if presence {
        Fr::from(entry.is_some())
    } else {
        Fr::from(entry.map_or(0, |item| item.amount.0))
    }
}
fn values(circuit: &CircuitArtifact, scenario: &Scenario, execution: &Execution) -> Vec<Fr> {
    circuit
        .public_inputs
        .iter()
        .map(|binding| match binding.role.as_str() {
            "context" => Fr::from(binding.constant.as_ref().unwrap().parse::<u128>().unwrap()),
            "pre_state" => field(&execution.pre.state[&binding.name]),
            "post_state" => field(&execution.post.state[&binding.name]),
            "input" => field(&scenario.inputs[&binding.name]),
            "pre_liability_presence" => debt(&execution.pre, &binding.name, true),
            "pre_liability_amount" => debt(&execution.pre, &binding.name, false),
            "post_liability_presence" => debt(&execution.post, &binding.name, true),
            "post_liability_amount" => debt(&execution.post, &binding.name, false),
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
                        .find(|item| {
                            item.account == account && Some(&item.asset) == binding.asset.as_ref()
                        })
                        .unwrap()
                        .amount
                        .0,
                )
            }
            "effect" => {
                let (step, part) = binding.name.split_once('.').unwrap();
                let step = step.parse::<usize>().unwrap();
                match &execution.steps[step].effects[0] {
                    Effect::Liability {
                        name,
                        before,
                        after,
                        ..
                    } => match part {
                        "before_presence" => debt(
                            if step == 0 {
                                &execution.pre
                            } else {
                                &execution.steps[step - 1].snapshot
                            },
                            name,
                            true,
                        ),
                        "after_presence" => debt(&execution.steps[step].snapshot, name, true),
                        "before_amount" => Fr::from(before.0),
                        "after_amount" => Fr::from(after.0),
                        other => panic!("unexpected liability field {other}"),
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
fn check(circuit: &CircuitArtifact, witness: &[Fr]) -> bool {
    let ir = midnight_zkir_v3::IrSource::load(circuit.zkir.as_bytes()).unwrap();
    ProofPreimage {
        inputs: vec![],
        private_transcript: vec![],
        public_transcript_inputs: witness.to_vec(),
        public_transcript_outputs: witness.to_vec(),
        binding_input: Fr::from(0u64),
        communications_commitment: None,
        key_location: KeyLocation("batch-finance-ordinary-check-unqualified".into()),
    }
    .check(&ir)
    .is_ok()
}
fn circuit<'a>(artifact: &'a mori_backend::Artifact, action: &str) -> &'a CircuitArtifact {
    artifact
        .circuits
        .iter()
        .find(|item| item.action == action)
        .unwrap()
}

#[test]
fn same_domain_source_compiles_and_another_domain_target_refuses() {
    let program = program();
    let artifact = compile(&program, &Target::ledger9("local", "batch-finance-anchor")).unwrap();
    assert_eq!(artifact.circuits.len(), 2);
    assert_eq!(artifact.resources.liability_registry_entries, 2);
    assert!(!artifact.resources.cost_measured);
    assert!(artifact.resources.circuit_k.is_none());
    assert!(
        artifact
            .gates
            .iter()
            .all(|gate| gate.status == "unperformed")
    );
    let json = serde_json::to_value(&artifact).unwrap();
    assert_eq!(json["authorization_mode"], "unsigned");
    assert_eq!(json["target"]["network"], "local");
    for action in ["clear", "swap"] {
        let item = circuit(&artifact, action);
        assert!(
            item.balance_frame
                .iter()
                .all(|frame| frame.asset.network == "local")
        );
        assert!(
            item.liability_frame
                .iter()
                .all(|frame| frame.asset.network == "local")
        );
        assert_eq!(item.liability_frame.len(), 2);
        assert_eq!(item.liability_frame[0].name, "clearDuty");
        assert_eq!(item.liability_frame[0].debtor, "maker");
        assert_eq!(item.liability_frame[0].creditor, "reserve");
        assert_eq!(item.liability_frame[1].name, "swapDuty");
        assert_eq!(item.liability_frame[1].debtor, "left");
        assert_eq!(item.liability_frame[1].asset.reference, "QUOTE");
    }
    assert_eq!(
        circuit(&artifact, "clear")
            .effect_frame
            .iter()
            .map(|effect| effect.kind.as_str())
            .collect::<Vec<_>>(),
        [
            "transfer",
            "transfer",
            "transfer",
            "transfer",
            "fee",
            "fee",
            "transfer",
            "transfer",
            "liability",
            "liability",
            "state",
            "state",
            "state",
            "state",
            "emit",
        ]
    );
    assert_eq!(
        circuit(&artifact, "swap")
            .effect_frame
            .iter()
            .map(|effect| effect.kind.as_str())
            .collect::<Vec<_>>(),
        [
            "transfer",
            "transfer",
            "fee",
            "liability",
            "liability",
            "state",
            "state",
            "state",
            "state",
            "emit",
        ]
    );
    let err = compile(&program, &Target::ledger9("other", "batch-finance-anchor")).unwrap_err();
    assert_eq!(err.code, "mori::backend::target");
}

#[test]
fn honest_clear_and_swap_relations_match_reference_and_reject_tamper() {
    let program = program();
    let artifact = compile(&program, &Target::ledger9("local", "batch-finance-anchor")).unwrap();
    for (action, file) in [("clear", "clear.json"), ("swap", "atomic-swap.json")] {
        let scenario = fixture(file);
        let execution = simulate(&program, action, &scenario).unwrap();
        let item = circuit(&artifact, action);
        let witness = values(item, &scenario, &execution);
        assert!(check(item, &witness), "{action} honest relation");
        let duty_name = item.liability_frame[0].name.clone();
        let tamper_at = [
            item.public_inputs
                .iter()
                .position(|binding| binding.role == "post_balance")
                .unwrap(),
            item.public_inputs
                .iter()
                .position(|binding| binding.role == "post_state" && binding.name == "gross")
                .unwrap(),
            item.public_inputs
                .iter()
                .position(|binding| binding.role == "effect")
                .unwrap(),
            item.public_inputs
                .iter()
                .position(|binding| {
                    binding.role == "post_liability_amount" && binding.name == duty_name
                })
                .unwrap(),
            0,
        ];
        for index in tamper_at {
            let mut changed = witness.clone();
            changed[index] = changed[index] + Fr::from(1u64);
            assert!(!check(item, &changed), "{action} tamper {index}");
        }
    }
}

#[test]
fn stale_and_overflow_public_inputs_fail_the_clear_relation() {
    let program = program();
    let artifact = compile(&program, &Target::ledger9("local", "batch-finance-anchor")).unwrap();
    let item = circuit(&artifact, "clear");
    let scenario = fixture("clear.json");
    let execution = simulate(&program, "clear", &scenario).unwrap();
    let mut stale = values(item, &scenario, &execution);
    let now = item
        .public_inputs
        .iter()
        .position(|binding| binding.role == "input" && binding.name == "now")
        .unwrap();
    stale[now] = Fr::from(49u64);
    assert!(!check(item, &stale), "stale now must fail");

    let overflow = item
        .public_inputs
        .iter()
        .map(|binding| match binding.role.as_str() {
            "context" => Fr::from(binding.constant.as_ref().unwrap().parse::<u128>().unwrap()),
            "input" if binding.name == "aliceQuote" => Fr::from(2u128),
            "input"
                if matches!(
                    binding.name.as_str(),
                    "price" | "alicePrice" | "bobPrice" | "oraclePrice"
                ) =>
            {
                Fr::from(1u128 << 127)
            }
            "input" if matches!(binding.name.as_str(), "bobQuote" | "bobBase") => Fr::from(1u128),
            "input" if matches!(binding.name.as_str(), "observedAt" | "now") => Fr::from(1u128),
            "input" if matches!(binding.name.as_str(), "solverCap" | "grossCap") => {
                Fr::from(u128::MAX)
            }
            "input"
                if matches!(
                    binding.name.as_str(),
                    "sameDomain" | "oracleAuthorized" | "solverAuthorized"
                ) =>
            {
                Fr::from(true)
            }
            "pre_liability_presence" | "post_liability_presence" => Fr::from(false),
            _ => Fr::from(0u64),
        })
        .collect::<Vec<_>>();
    assert!(!check(item, &overflow), "overflow witness must fail");
    let runtime = ClearOverflow::default();
    let err = simulate(&program, "clear", &runtime.scenario(&program)).unwrap_err();
    assert_eq!(err.code, "mori::core::overflow");
}

struct ClearOverflow {
    alice_quote: u128,
    price: u128,
}
impl Default for ClearOverflow {
    fn default() -> Self {
        Self {
            alice_quote: 2,
            price: 1u128 << 127,
        }
    }
}
impl ClearOverflow {
    fn scenario(&self, program: &Program) -> Scenario {
        let base = &program
            .assets
            .iter()
            .find(|item| item.name == "BASE")
            .unwrap()
            .identity;
        let quote = &program
            .assets
            .iter()
            .find(|item| item.name == "QUOTE")
            .unwrap()
            .identity;
        let uint = |n: u128| Value::UInt(UInt(n));
        let balance = |account: &str, asset: &AssetId, amount: u128| Balance {
            account: account.into(),
            asset: asset.clone(),
            amount: UInt(amount),
        };
        use std::collections::BTreeMap;
        Scenario {
            inputs: BTreeMap::from([
                ("aliceQuote".into(), uint(self.alice_quote)),
                ("bobQuote".into(), uint(1)),
                ("aliceBase".into(), uint(0)),
                ("bobBase".into(), uint(1)),
                ("price".into(), uint(self.price)),
                ("alicePrice".into(), uint(self.price)),
                ("bobPrice".into(), uint(self.price)),
                ("oraclePrice".into(), uint(self.price)),
                ("observedAt".into(), uint(1)),
                ("now".into(), uint(1)),
                ("aliceFee".into(), uint(0)),
                ("bobFee".into(), uint(0)),
                ("aliceRebate".into(), uint(0)),
                ("bobRebate".into(), uint(0)),
                ("solverCap".into(), uint(u128::MAX)),
                ("grossCap".into(), uint(u128::MAX)),
                ("minNet".into(), uint(0)),
                ("settledDuty".into(), uint(0)),
                ("sameDomain".into(), Value::Bool(true)),
                ("oracleAuthorized".into(), Value::Bool(true)),
                ("solverAuthorized".into(), Value::Bool(true)),
            ]),
            balances: vec![
                balance("maker", base, 1),
                balance("alice", base, 0),
                balance("bob", base, 0),
                balance("alice", quote, 2),
                balance("bob", quote, 1),
                balance("maker", quote, 0),
                balance("pool", quote, 0),
            ],
            ..Default::default()
        }
    }
}
