//! I0: the native permutation and hashes against midnight's own sponge, the hex codec.
use ff::Field;
use midnight_circuits::{hash::poseidon::PoseidonChip, instructions::hash::HashCPU};
use rand::{SeedableRng, rngs::StdRng};
use sherlock_wrap::native::*;

#[test]
fn i0_u0_sponge_equals_midnight_hash() {
    // u = 0 fixed-length sponge with this crate's permutation equals PoseidonChip::hash.
    let mut rng = StdRng::seed_from_u64(5);
    for len in 0..9usize {
        let v: Vec<F> = (0..len).map(|_| F::random(&mut rng)).collect();
        let mut s = [F::ZERO, F::ZERO, cap_fixed(0, len as u128)];
        for b in v.chunks(RATE) {
            for (x, y) in s.iter_mut().zip(b) {
                *x += y;
            }
            permute(&mut s);
        }
        assert_eq!(s[0], <PoseidonChip<F> as HashCPU<F, F>>::hash(&v), "len {len}");
    }
}

#[test]
fn i0_hex_and_low_bits() {
    let x = F::from(0x1234_5678_9abc_def0u64);
    assert_eq!(f_from_hex(&f_to_hex(&x)), Some(x));
    assert_eq!(low_u64(&x), 0x1234_5678_9abc_def0);
    assert!(f_to_hex(&x).ends_with("123456789abcdef0"));
    assert_eq!(f_from_hex(&"ff".repeat(32)), None);
}
