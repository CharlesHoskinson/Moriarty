//! The host witness stream (mori-backend) and the IVC node and run decider (mori-recursion) must
//! compute one state digest, or the leaf chain cannot link to the node. Compare canonical bytes.
#![cfg(feature = "sherlock-ledger")]

use ff::PrimeField;
use midnight_transient_crypto::curve::Fr;
use mori_backend::sherlock_c6_child::{StateOutcome, state_digest_offcircuit};
use mori_recursion::ivc::state_digest;

#[test]
fn backend_and_recursion_state_digests_agree() {
    let cases: [(u64, u128, u128); 4] = [
        (0, 0, 0),
        (1, 7, 123_456_789),
        (1, 6, u128::MAX >> 3),
        (2, u64::MAX as u128, 1),
    ];
    for (tag, value, lanes) in cases {
        let backend = state_digest_offcircuit(
            StateOutcome {
                tag: tag.into(),
                value,
            },
            Fr::from(lanes),
        );
        let recursion = state_digest(tag, value, PrimeField::from_u128(lanes));
        assert_eq!(
            backend.0.to_repr().as_ref(),
            recursion.to_repr().as_ref(),
            "tag {tag} value {value} lanes {lanes}"
        );
    }
}
