//! Positive compiled-relation observations from independent financial literals.
use midnight_transient_crypto::{
    curve::Fr,
    proofs::{KeyLocation, ProofPreimage},
};
use mori_backend::{CircuitArtifact, LiabilityIdentity, LiabilityOperation, Target};
use mori_core::{AssetId, Effect, Execution, Program, Scenario, Snapshot, Step, Type, UInt, Value};
use std::collections::BTreeSet;

macro_rules! fixture {
    ($name:literal) => {
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../fixtures/financial-report-contract/",
            $name
        ))
    };
}

fn field(value: &Value) -> Fr {
    match value {
        Value::UInt(amount) => Fr::from(amount.0),
        Value::Bool(value) => Fr::from(*value),
    }
}

fn binding(
    circuit: &CircuitArtifact,
    position: usize,
    role: &str,
    name: &str,
    ty: Type,
    asset: Option<&AssetId>,
) {
    let actual = circuit
        .public_inputs
        .get(position)
        .expect("ABI position must exist");
    assert_eq!(actual.role, role);
    assert_eq!(actual.name, name);
    assert_eq!(actual.ty, ty);
    assert_eq!(actual.asset.as_ref(), asset);
    assert!(
        actual.constant.is_none(),
        "financial slot must not be a context constant"
    );
}

fn put(
    circuit: &CircuitArtifact,
    values: &mut [Option<Fr>],
    position: usize,
    role: &str,
    name: &str,
    value: &Value,
    asset: Option<&AssetId>,
) {
    binding(circuit, position, role, name, value.ty(), asset);
    assert!(
        values[position].replace(field(value)).is_none(),
        "ABI slot assigned twice"
    );
}

fn named(
    circuit: &CircuitArtifact,
    values: &mut [Option<Fr>],
    role: &str,
    name: &str,
    value: &Value,
    asset: Option<&AssetId>,
) {
    let positions: Vec<_> = circuit
        .public_inputs
        .iter()
        .enumerate()
        .filter(|(_, entry)| entry.role == role && entry.name == name)
        .map(|(position, _)| position)
        .collect();
    assert_eq!(positions.len(), 1, "named ABI slot must exist exactly once");
    put(circuit, values, positions[0], role, name, value, asset);
}

fn identity(program: &Program, alias: &str) -> AssetId {
    program
        .assets
        .iter()
        .find(|asset| asset.name == alias)
        .expect("frozen asset alias must exist")
        .identity
        .clone()
}

fn alias<'a>(program: &'a Program, asset: &AssetId) -> &'a str {
    let matches: Vec<_> = program
        .assets
        .iter()
        .filter(|entry| &entry.identity == asset)
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "finite specimens use unique asset aliases"
    );
    &matches[0].name
}

fn duty(snapshot: &Snapshot, name: &str) -> (Value, Value) {
    let entry = snapshot.liabilities.iter().find(|entry| entry.name == name);
    (
        Value::Bool(entry.is_some()),
        Value::UInt(entry.map_or(UInt(0), |entry| entry.amount)),
    )
}

fn source_wide(circuits: &[CircuitArtifact], program: &Program, closing: &Snapshot) {
    assert_eq!(
        circuits
            .iter()
            .map(|c| c.action.as_str())
            .collect::<Vec<_>>(),
        program
            .actions
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
    );
    for (circuit, action) in circuits.iter().zip(&program.actions) {
        assert_eq!(circuit.replay_steps as usize, action.statements.len());
        assert!(circuit.authorization_bindings.is_empty());
        assert!(circuit.private_inputs.is_empty());
        for role in ["pre_state", "post_state"] {
            let entries: Vec<_> = circuit
                .public_inputs
                .iter()
                .filter(|entry| entry.role == role)
                .collect();
            assert_eq!(entries.len(), program.states.len());
            for (entry, state) in entries.iter().zip(&program.states) {
                assert_eq!(entry.name, state.name);
                assert_eq!(entry.ty, state.ty);
                assert!(entry.constant.is_none());
                assert!(entry.asset.is_none());
            }
        }
        let inputs: Vec<_> = circuit
            .public_inputs
            .iter()
            .filter(|entry| entry.role == "input")
            .collect();
        assert_eq!(inputs.len(), action.inputs.len());
        for (entry, input) in inputs.iter().zip(&action.inputs) {
            assert_eq!(entry.name, input.name);
            assert_eq!(entry.ty, input.ty);
            assert!(entry.constant.is_none());
            assert!(entry.asset.is_none());
        }
        assert_eq!(circuit.liability_frame.len(), closing.liabilities.len());
        for (frame, expected) in circuit.liability_frame.iter().zip(&closing.liabilities) {
            assert_eq!(frame.name, expected.name);
            assert_eq!(frame.asset, expected.asset);
            assert_eq!(frame.debtor, expected.debtor);
            assert_eq!(frame.creditor, expected.creditor);
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
                binding(
                    circuit,
                    position as usize,
                    role,
                    &frame.name,
                    ty,
                    Some(&frame.asset),
                );
            }
        }
    }
    let register = &circuits[0];
    assert_eq!(register.action, "register");
    assert!(register.balance_frame.is_empty());
    assert_eq!(
        register.effect_frame.len(),
        program.actions[0].statements.len()
    );
    assert_eq!(
        register.public_inputs.len(),
        2 + 2 * program.states.len()
            + 4 * closing.liabilities.len()
            + 4 * program.actions[0].statements.len()
    );
    for (index, (frame, statement)) in register
        .effect_frame
        .iter()
        .zip(&program.actions[0].statements)
        .enumerate()
    {
        let mori_core::Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = statement
        else {
            panic!("frozen register action contains only declarations")
        };
        let asset = identity(program, asset);
        assert_eq!(frame.step as usize, index);
        assert_eq!(frame.kind, "liability");
        assert!(frame.name.is_none() && frame.asset.is_none());
        assert!(frame.from.is_none() && frame.to.is_none());
        assert_eq!(
            frame.liability,
            Some(LiabilityIdentity {
                name: name.clone(),
                asset: asset.clone(),
                debtor: debtor.clone(),
                creditor: creditor.clone(),
            })
        );
        assert_eq!(frame.operation, Some(LiabilityOperation::CreateOrAccrue));
        assert_eq!(frame.public_positions.len(), 4);
        for (position, (part, ty)) in frame.public_positions.iter().zip([
            ("before_presence", Type::Bool),
            ("before_amount", Type::UInt),
            ("after_presence", Type::Bool),
            ("after_amount", Type::UInt),
        ]) {
            binding(
                register,
                *position as usize,
                "effect",
                &format!("{index}.{part}"),
                ty,
                Some(&asset),
            );
        }
    }
}

struct ObservedEffect<'a> {
    kind: &'static str,
    name: Option<&'a str>,
    asset: Option<&'a AssetId>,
    from: Option<&'a str>,
    to: Option<&'a str>,
    liability: Option<LiabilityIdentity>,
    operation: Option<LiabilityOperation>,
    fields: Vec<(&'static str, Value)>,
}

fn effect<'a>(step: &'a Step, effect: &'a Effect, before: &Snapshot) -> ObservedEffect<'a> {
    let mut result = ObservedEffect {
        kind: "",
        name: None,
        asset: None,
        from: None,
        to: None,
        liability: None,
        operation: None,
        fields: Vec::new(),
    };
    match effect {
        Effect::State {
            name,
            before,
            after,
        } => {
            result.kind = "state";
            result.name = Some(name);
            result.fields = vec![("before", before.clone()), ("after", after.clone())];
        }
        Effect::Emit { name, value } => {
            result.kind = "emit";
            result.name = Some(name);
            result.fields = vec![("value", value.clone())];
        }
        Effect::Transfer {
            asset,
            from,
            to,
            amount,
        }
        | Effect::Fee {
            asset,
            from,
            to,
            amount,
        } => {
            result.kind = if matches!(effect, Effect::Fee { .. }) {
                "fee"
            } else {
                "transfer"
            };
            result.asset = Some(asset);
            result.from = Some(from);
            result.to = Some(to);
            result.fields = vec![("amount", Value::UInt(*amount))];
        }
        Effect::Liability {
            name,
            asset,
            debtor,
            creditor,
            before: amount_before,
            after,
        } => {
            result.kind = "liability";
            result.liability = Some(LiabilityIdentity {
                name: name.clone(),
                asset: asset.clone(),
                debtor: debtor.clone(),
                creditor: creditor.clone(),
            });
            result.operation = Some(match &step.statement {
                mori_core::Statement::Liability { .. } => LiabilityOperation::CreateOrAccrue,
                mori_core::Statement::Discharge { .. } => LiabilityOperation::Discharge,
                _ => panic!("frozen liability effect must name its source operation"),
            });
            result.fields = vec![
                ("before_presence", duty(before, name).0),
                ("before_amount", Value::UInt(*amount_before)),
                ("after_presence", duty(&step.snapshot, name).0),
                ("after_amount", Value::UInt(*after)),
            ];
        }
    }
    result
}

fn transcript(
    circuit: &CircuitArtifact,
    program: &Program,
    scenario: &Scenario,
    execution: &Execution,
    outside: &[(&str, &str)],
) -> Vec<Fr> {
    assert_eq!(execution.action, circuit.action);
    assert_eq!(execution.semantics, "moriarty-core/1");
    assert_eq!(
        execution.pre,
        Snapshot {
            state: scenario.state.clone(),
            balances: scenario.balances.clone(),
            liabilities: scenario.liabilities.clone(),
        }
    );
    let action = program
        .actions
        .iter()
        .find(|action| action.name == execution.action)
        .unwrap();
    assert_eq!(execution.steps.len(), action.statements.len());
    for (index, (step, statement)) in execution.steps.iter().zip(&action.statements).enumerate() {
        assert_eq!(step.index as usize, index);
        assert_eq!(&step.statement, statement);
    }
    assert_eq!(
        execution.effects,
        execution
            .steps
            .iter()
            .flat_map(|step| step.effects.iter().cloned())
            .collect::<Vec<_>>()
    );
    assert_eq!(execution.post, execution.steps.last().unwrap().snapshot);
    let mut values = vec![None; circuit.public_inputs.len()];
    // Context values are inherited compiler outputs, not independent financial expectations.
    let contexts: Vec<_> = circuit
        .public_inputs
        .iter()
        .enumerate()
        .filter(|(_, entry)| entry.role == "context")
        .collect();
    assert_eq!(contexts.len(), 2);
    for (index, (position, entry)) in contexts.into_iter().enumerate() {
        assert_eq!(position, index);
        assert_eq!(entry.name, format!("program_target_action.{index}"));
        assert_eq!(entry.ty, Type::UInt);
        assert!(entry.asset.is_none());
        values[position] = Some(Fr::from(
            entry.constant.as_ref().unwrap().parse::<u128>().unwrap(),
        ));
    }
    for (role, snapshot) in [
        ("pre_state", &execution.pre),
        ("post_state", &execution.post),
    ] {
        for state in &program.states {
            named(
                circuit,
                &mut values,
                role,
                &state.name,
                &snapshot.state[&state.name],
                None,
            );
        }
    }
    for input in &action.inputs {
        named(
            circuit,
            &mut values,
            "input",
            &input.name,
            &scenario.inputs[&input.name],
            None,
        );
    }
    let mut touched = BTreeSet::new();
    for item in &execution.effects {
        if let Effect::Transfer {
            asset, from, to, ..
        }
        | Effect::Fee {
            asset, from, to, ..
        } = item
        {
            touched.insert((asset.clone(), from.clone()));
            touched.insert((asset.clone(), to.clone()));
        }
    }
    let represented: BTreeSet<_> = circuit
        .balance_frame
        .iter()
        .map(|frame| (frame.asset.clone(), frame.account.clone()))
        .collect();
    assert_eq!(represented.len(), circuit.balance_frame.len());
    assert_eq!(represented, touched);
    let omitted: BTreeSet<_> = execution
        .pre
        .balances
        .iter()
        .filter(|row| !represented.contains(&(row.asset.clone(), row.account.clone())))
        .map(|row| (row.asset.clone(), row.account.clone()))
        .collect();
    let declared_outside: BTreeSet<_> = outside
        .iter()
        .map(|(asset, account)| (identity(program, asset), (*account).to_owned()))
        .collect();
    assert_eq!(omitted, declared_outside);
    for (asset, account) in &omitted {
        let opening = execution
            .pre
            .balances
            .iter()
            .find(|row| &row.asset == asset && &row.account == account)
            .unwrap();
        let closing = execution
            .post
            .balances
            .iter()
            .find(|row| &row.asset == asset && &row.account == account)
            .unwrap();
        assert_eq!(
            opening, closing,
            "outside-frame preservation is a frozen ledger-adapter premise"
        );
    }
    for frame in &circuit.balance_frame {
        let name = serde_json::to_string(&(alias(program, &frame.asset), &frame.account)).unwrap();
        for (position, role, snapshot) in [
            (frame.pre_position, "pre_balance", &execution.pre),
            (frame.post_position, "post_balance", &execution.post),
        ] {
            let row = snapshot
                .balances
                .iter()
                .find(|row| row.account == frame.account && row.asset == frame.asset)
                .unwrap();
            put(
                circuit,
                &mut values,
                position as usize,
                role,
                &name,
                &Value::UInt(row.amount),
                Some(&frame.asset),
            );
        }
    }
    for frame in &circuit.liability_frame {
        let pre = duty(&execution.pre, &frame.name);
        let post = duty(&execution.post, &frame.name);
        for (position, role, value) in [
            (frame.pre_presence_position, "pre_liability_presence", pre.0),
            (frame.pre_amount_position, "pre_liability_amount", pre.1),
            (
                frame.post_presence_position,
                "post_liability_presence",
                post.0,
            ),
            (frame.post_amount_position, "post_liability_amount", post.1),
        ] {
            put(
                circuit,
                &mut values,
                position as usize,
                role,
                &frame.name,
                &value,
                Some(&frame.asset),
            );
        }
    }
    let expected_effects: Vec<_> = execution
        .steps
        .iter()
        .flat_map(|step| step.effects.iter().map(move |item| (step, item)))
        .collect();
    assert_eq!(circuit.effect_frame.len(), expected_effects.len());
    for (frame, (step, item)) in circuit.effect_frame.iter().zip(expected_effects) {
        let before = if step.index == 0 {
            &execution.pre
        } else {
            &execution.steps[step.index as usize - 1].snapshot
        };
        let expected = effect(step, item, before);
        assert_eq!(frame.step, u32::from(step.index));
        assert_eq!(frame.kind, expected.kind);
        assert_eq!(frame.name.as_deref(), expected.name);
        assert_eq!(frame.asset.as_ref(), expected.asset);
        assert_eq!(frame.from.as_deref(), expected.from);
        assert_eq!(frame.to.as_deref(), expected.to);
        assert_eq!(frame.liability, expected.liability);
        assert_eq!(frame.operation, expected.operation);
        assert_eq!(frame.public_positions.len(), expected.fields.len());
        let slot_asset = expected
            .liability
            .as_ref()
            .map(|duty| &duty.asset)
            .or(expected.asset);
        for (position, (part, value)) in frame.public_positions.iter().zip(&expected.fields) {
            put(
                circuit,
                &mut values,
                *position as usize,
                "effect",
                &format!("{}.{}", step.index, part),
                value,
                slot_asset,
            );
        }
    }
    values
        .into_iter()
        .map(|value| value.expect("every ABI slot must receive an explicit observation"))
        .collect()
}

fn observe(
    source: &str,
    expected_program: &str,
    anchor: &str,
    cases: &[(&str, &str)],
    outside: &[(&str, &str)],
) -> Vec<Execution> {
    let program = mori_core::parse(source).expect("exact financial source must parse");
    let frozen: mori_core::v1::Program = serde_json::from_str(expected_program).unwrap();
    let expected_program: Program = frozen.into();
    assert_eq!(program, expected_program);
    let target = Target::ledger9("local", anchor);
    let artifact = mori_backend::compile(&program, &target).expect("whole source must compile");
    assert_eq!(artifact.format, "moriarty-zkir-artifact/3");
    assert_eq!(artifact.abi, "moriarty-public-abi/3");
    assert_eq!(artifact.relation, "moriarty-zkir-relation/3");
    assert_eq!(artifact.authorization_mode, "unsigned");
    assert_eq!(artifact.semantics, expected_program.semantics);
    let mut observations = Vec::new();
    for (scenario, execution) in cases {
        let scenario: Scenario = serde_json::from_str(scenario).unwrap();
        let frozen: mori_core::v1::Execution = serde_json::from_str(execution).unwrap();
        let expected: Execution = frozen.into();
        source_wide(&artifact.circuits, &expected_program, &expected.post);
        let circuit = artifact
            .circuits
            .iter()
            .find(|circuit| circuit.action == expected.action)
            .unwrap();
        let values = transcript(circuit, &expected_program, &scenario, &expected, outside);
        let ir = midnight_zkir_v3::IrSource::load(circuit.zkir.as_bytes()).unwrap();
        let _ = ProofPreimage {
            inputs: vec![],
            private_transcript: vec![],
            public_transcript_inputs: values.clone(),
            public_transcript_outputs: values,
            binding_input: Fr::from(0u64),
            communications_commitment: None,
            key_location: KeyLocation("ordinary-financial-observation-unqualified".into()),
        }
        .check(&ir)
        .expect("independently specified positive observation must satisfy emitted relation");
        observations.push(expected);
    }
    observations
}

#[test]
fn invoice_relation_accepts_both_independent_linked_observations() {
    let observations = observe(
        fixture!("installment-invoice.mori"),
        fixture!("installment-invoice.program.expected.json"),
        "financial-invoice-compiler-test",
        &[
            (
                fixture!("invoice-first.scenario.json"),
                fixture!("invoice-first.execution.expected.json"),
            ),
            (
                fixture!("invoice-final.scenario.json"),
                fixture!("invoice-final.execution.expected.json"),
            ),
        ],
        &[("CASH", "observer"), ("CASH", "reserve")],
    );
    assert_eq!(observations[0].post, observations[1].pre);
}

#[test]
fn reserve_relation_accepts_independent_multiasset_observation() {
    observe(
        fixture!("conditional-reserve.mori"),
        fixture!("conditional-reserve.program.expected.json"),
        "financial-reserve-compiler-test",
        &[(
            fixture!("reserve-approved.scenario.json"),
            fixture!("reserve-approved.execution.expected.json"),
        )],
        &[("BOND", "observer"), ("CASH", "observer")],
    );
}
