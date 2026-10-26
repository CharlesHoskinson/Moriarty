//! Feature-gated Sherlock recursive parent on the pinned stock Midnight crates.
//!
//! The parent proves both child verifier relations and carries their collapsed KZG
//! accumulator. Terminal validity still requires the decider and its pairing check.
#![cfg(feature = "sherlock-recursion")]

pub mod child;
pub mod decider;
pub mod flat;
pub mod inner;
pub mod ivc;
pub mod pairing;
pub mod parent;
pub mod reference;
pub mod run_decider;
pub mod standin;
pub mod template;
pub mod tower;
pub mod util;

fn bls_arch(nr_cols: u8) -> midnight_zk_stdlib::ZkStdLibArch {
    midnight_zk_stdlib::ZkStdLibArch {
        bls12_381: true,
        poseidon: true,
        nr_pow2range_cols: nr_cols,
        ..midnight_zk_stdlib::ZkStdLibArch::default()
    }
}
