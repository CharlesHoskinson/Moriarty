use midnight_base_crypto::{
    cost_model::{CostDuration, RunningCost},
    fab::AlignedValue,
    hash::HashOutput,
};
use midnight_coin_structure::{
    coin::{PublicAddress, TokenType, UnshieldedTokenType, UserAddress},
    contract::ContractAddress,
};
use midnight_ledger::structure::INITIAL_PARAMETERS;
use midnight_onchain_runtime::context::{CallContext, ClaimedUnshieldedSpendsKey, QueryContext};
use midnight_onchain_state::state::{ChargedState, StateValue};
use midnight_onchain_vm::{ops::Op, result_mode::ResultModeVerify};
use mori_ledger::ops::{abi_guard, claim_update};
fn context() -> QueryContext<midnight_storage::db::InMemoryDB> {
    let root = StateValue::Array(
        vec![
            StateValue::Map(Default::default()),
            StateValue::Map(Default::default()),
            StateValue::Map(Default::default()),
        ]
        .into(),
    );
    let address = ContractAddress(HashOutput([7; 32]));
    let mut c = QueryContext::new(ChargedState::new(root), address);
    c.call_context = CallContext {
        own_address: address,
        tblock: Default::default(),
        tblock_err: 0,
        parent_block_hash: HashOutput([0; 32]),
        caller: None,
        balance: Default::default(),
        com_indices: Default::default(),
        last_block_time: Default::default(),
    };
    c
}
fn bound() -> RunningCost {
    RunningCost {
        read_time: CostDuration::SECOND,
        compute_time: CostDuration::SECOND,
        bytes_written: 1_000_000,
        bytes_deleted: 1_000_000,
    }
}
fn key() -> ClaimedUnshieldedSpendsKey {
    ClaimedUnshieldedSpendsKey::from_inner(
        TokenType::Unshielded(UnshieldedTokenType(HashOutput([9; 32]))),
        PublicAddress::User(UserAddress(HashOutput([8; 32]))),
    )
}
#[test]
fn real_vm_read_set_groups_uint128_above_u64() {
    let c = context();
    let mut ops = abi_guard();
    let n = 1u128 << 64;
    ops.extend(claim_update(key(), None, n).unwrap());
    ops.extend(claim_update(key(), Some(n), n).unwrap());
    let r = c
        .query(
            &ops,
            Some(bound()),
            &INITIAL_PARAMETERS.cost_model.runtime_cost_model,
        )
        .unwrap();
    assert_eq!(
        r.context
            .effects
            .claimed_unshielded_spends
            .get(&key())
            .map(|n| *n),
        Some(1u128 << 65)
    );
    assert_eq!(*r.context.state.get(), *c.state.get());
    assert!(r.events.is_empty());
}
#[test]
fn membership_and_prior_value_are_actual_vm_reads() {
    let c = context();
    let mut ops = abi_guard();
    ops.extend(claim_update(key(), Some(4), 3).unwrap());
    assert!(
        c.query(
            &ops,
            Some(bound()),
            &INITIAL_PARAMETERS.cost_model.runtime_cost_model
        )
        .is_err()
    );
    let mut ops = abi_guard();
    ops.extend(claim_update(key(), None, 4).unwrap());
    ops.extend(claim_update(key(), Some(5), 3).unwrap());
    assert!(
        c.query(
            &ops,
            Some(bound()),
            &INITIAL_PARAMETERS.cost_model.runtime_cost_model
        )
        .is_err()
    );
}
#[test]
fn abi_guard_is_nonempty_and_wrong_result_refuses() {
    let c = context();
    let ops = abi_guard();
    let r = c
        .query(
            &ops,
            Some(bound()),
            &INITIAL_PARAMETERS.cost_model.runtime_cost_model,
        )
        .unwrap();
    assert_eq!(r.context.effects, c.effects);
    assert_eq!(*r.context.state.get(), *c.state.get());
    let mut wrong = ops;
    wrong[2] = Op::<ResultModeVerify>::Popeq {
        cached: false,
        result: AlignedValue::from(4u64),
    };
    assert!(
        c.query(
            &wrong,
            Some(bound()),
            &INITIAL_PARAMETERS.cost_model.runtime_cost_model
        )
        .is_err()
    );
}
