mod support;
use mori_core::{Scenario, UInt, Value};
use mori_ledger::{codec, layout::NativeState, local_deployment::admit_local_seed};

#[test]
fn full_keyless_seed_admission_does_not_execute_an_action() {
    let source = "language \"moriarty/1\"; agreement Frozen { state n: UInt=0; state active: Bool=true; action step(delta: UInt) { requires active; next n=n+delta; } action reset() { next n=0; } }";
    let f = support::fixture(
        source,
        "step",
        Scenario {
            state: [
                ("n".into(), Value::UInt(UInt(u128::MAX))),
                ("active".into(), Value::Bool(false)),
            ]
            .into_iter()
            .collect(),
            inputs: [("delta".into(), Value::UInt(UInt(1)))]
                .into_iter()
                .collect(),
            ..Default::default()
        },
    );
    assert!(
        mori_ledger::prepare::prepare_call(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters,
            &f.request
        )
        .is_err()
    );
    let seed = admit_local_seed(
        &f.source,
        &f.program,
        &f.deployment,
        &f.predecessor,
        &f.parameters,
    )
    .unwrap();
    assert_eq!(seed.action_names(), vec!["step", "reset"]);
}

#[test]
fn seed_checks_complete_frame_and_refuses_existing_ir() {
    let mut f = support::payment();
    let mut state: NativeState = codec::decode_native(&f.predecessor).unwrap();
    let address = codec::from_hex(&f.deployment.address_hex).unwrap();
    let mut contract = state.index(address).unwrap();
    let mut operation = (*contract.operations.get(&b"settle".to_vec().into()).unwrap()).clone();
    operation.ir = Some(midnight_storage::arena::Sp::new(
        midnight_onchain_state::state::IrBuf(vec![1]),
    ));
    contract.operations = contract
        .operations
        .insert(b"settle".to_vec().into(), operation);
    state.contract = state.contract.insert(address, contract);
    support::repin(&mut f, &state);
    assert_eq!(
        admit_local_seed(
            &f.source,
            &f.program,
            &f.deployment,
            &f.predecessor,
            &f.parameters
        )
        .err()
        .unwrap()
        .code,
        "unproven-operation"
    );
    let f = support::payment();
    let mut wrong = f.deployment.clone();
    wrong.balance_hash = "00".into();
    assert!(
        admit_local_seed(&f.source, &f.program, &wrong, &f.predecessor, &f.parameters).is_err()
    );
    let mut wrong = f.deployment.clone();
    wrong.parameters_hash = "00".into();
    assert!(
        admit_local_seed(&f.source, &f.program, &wrong, &f.predecessor, &f.parameters).is_err()
    );
}

#[test]
fn legacy_wrappers_preserve_mixed_invalid_refusal_priority() {
    let f = support::payment();
    let mut d = f.deployment.clone();
    d.profile = "bad".into();
    d.source_hash = "bad".into();
    assert_eq!(
        mori_ledger::relation::compile(&f.source, &f.program, &d, "settle")
            .err()
            .unwrap()
            .code,
        "source-identity"
    );
    d.toolchain.rust = "bad".into();
    assert_eq!(
        mori_ledger::layout::admit_source(&f.program, &d)
            .err()
            .unwrap()
            .code,
        "toolchain-or-limits"
    );
    let abi = mori_ledger::layout::admit_source(&f.program, &f.deployment).unwrap();
    let mut request = f.request.clone();
    request.segment = 2;
    let mut d = f.deployment.clone();
    d.predecessor_hash = "bad".into();
    let state = codec::decode_native(&f.predecessor).unwrap();
    assert_eq!(
        mori_ledger::layout::decode_predecessor(&f.program, &abi, &d, state, &request)
            .err()
            .unwrap()
            .code,
        "request-scope"
    );
    let mut d = f.deployment.clone();
    d.gas_hex = "bad".into();
    let mut request = f.request.clone();
    request.selected_spends = vec!["zz".into()];
    let state = codec::decode_native(&f.predecessor).unwrap();
    assert_eq!(
        mori_ledger::layout::decode_predecessor(&f.program, &abi, &d, state, &request)
            .err()
            .unwrap()
            .code,
        "canonical-hex"
    );
}
