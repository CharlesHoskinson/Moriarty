#![cfg(feature = "ledger")]
// Privileged local system-transaction semantics, not user financial settlement.
use midnight_base_crypto::{schnorr::SigningKey, time::Timestamp};
use midnight_coin_structure::coin::UserAddress;
use midnight_ledger::structure::{
    ClaimKind, LedgerState, MAX_SUPPLY, OutputInstructionUnshielded, SystemTransaction,
};
use midnight_serialize::Serializable;
use midnight_storage::db::InMemoryDB;
use rand::{Rng, rngs::OsRng};
fn bytes<T: Serializable>(v: &T) -> Vec<u8> {
    let mut b = Vec::new();
    v.serialize(&mut b).unwrap();
    b
}
#[test]
fn actual_ledger_local_reserve_reward_replay_and_atomic_failure() {
    let initial = LedgerState::<InMemoryDB>::new("local-test");
    let time = Timestamp::from_secs(0);
    let (funded, _) = initial
        .apply_system_tx(&SystemTransaction::DistributeReserve(500000), time)
        .unwrap();
    assert_eq!(initial.reserve_pool, MAX_SUPPLY);
    assert_eq!(funded.reserve_pool, MAX_SUPPLY - 500000);
    assert_eq!(funded.block_reward_pool, 500000);
    let address = UserAddress::from(SigningKey::sample(OsRng).verifying_key());
    let output = OutputInstructionUnshielded {
        amount: 100,
        target_address: address,
        nonce: OsRng.r#gen(),
    };
    let tx = SystemTransaction::DistributeNight(ClaimKind::Reward, vec![output.clone()]);
    let (post, _) = funded.apply_system_tx(&tx, time).unwrap();
    assert_eq!(post.block_reward_pool, 499900);
    assert_eq!(
        post.unclaimed_block_rewards.get(&address).copied(),
        Some(100)
    );
    let before = bytes(&post);
    assert!(post.apply_system_tx(&tx, time).is_err());
    assert_eq!(bytes(&post), before);
    let mut fresh = output.clone();
    fresh.nonce = OsRng.r#gen();
    let mixed = SystemTransaction::DistributeNight(ClaimKind::Reward, vec![fresh, output.clone()]);
    assert!(post.apply_system_tx(&mixed, time).is_err());
    assert_eq!(bytes(&post), before);
    let mut excess = output;
    excess.amount = 500000;
    excess.nonce = OsRng.r#gen();
    assert!(
        post.apply_system_tx(
            &SystemTransaction::DistributeNight(ClaimKind::Reward, vec![excess]),
            time
        )
        .is_err()
    );
    assert_eq!(bytes(&post), before);
    assert!(
        initial
            .apply_system_tx(&SystemTransaction::DistributeReserve(MAX_SUPPLY + 1), time)
            .is_err()
    );
    assert_eq!(initial.reserve_pool, MAX_SUPPLY);
    println!(
        "Official ledger local privileged reserve/reward/replay/atomic-error semantics checked; no user signature authorization, financial S0 caller, Preview or finality claim"
    );
}
