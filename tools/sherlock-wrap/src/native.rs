//! Native (off-circuit) pieces: the Midnight Poseidon permutation over BLS12-381 Fr, the
//! `L_Fr` capacities (PROPOSAL 3.6 rules, `L_Fr` codes u = 5, 6, 7 of E2 I1), and a mirror
//! of the stock overwrite duplex transcript (`stark-backend` `DuplexSponge<Fr, P, 3, 2>`).

use std::sync::OnceLock;

use ff::{Field, PrimeField};
use midnight_circuits::hash::poseidon::{permutation_cpu, round_skips::PreComputedRoundCPU};

pub type F = midnight_curves::Fq;

pub const WIDTH: usize = 3;
pub const RATE: usize = 2;

/// `L_Fr` use codes (E2 I1, `FrDomains::l_fr()`).
pub const U_LFR_LEAF: u128 = 5;
pub const U_LFR_NODE: u128 = 6;
pub const U_LFR_TRANSCRIPT: u128 = 7;

/// `l + u * 2^66`, the capacity of a fixed-length sponge of `l` inputs with use code `u`.
pub fn cap_fixed(u: u128, l: u128) -> F {
    assert!(l < (1u128 << 64));
    F::from_u128(l + (u << 66))
}

/// `2^64 + u * 2^66`, the initial capacity of a variable-length sponge with use code `u`.
pub fn cap_var(u: u128) -> F {
    F::from_u128((1u128 << 64) + (u << 66))
}

fn pre() -> &'static PreComputedRoundCPU<F> {
    static P: OnceLock<PreComputedRoundCPU<F>> = OnceLock::new();
    P.get_or_init(PreComputedRoundCPU::init)
}

/// The Midnight Poseidon permutation (midnight-circuits 7.3.0 `permutation_cpu`).
pub fn permute(state: &mut [F; WIDTH]) {
    permutation_cpu(pre(), &mut state[..]);
}

/// Leaf hash at use code `u` (stock `FrMerkleHasher::hash_slice`).
pub fn hash_slice(u: u128, vals: &[F]) -> F {
    let mut s = [F::ZERO, F::ZERO, cap_fixed(u, vals.len() as u128)];
    for block in vals.chunks(RATE) {
        for (x, v) in s.iter_mut().zip(block) {
            *x += v;
        }
        permute(&mut s);
    }
    s[0]
}

/// Node hash at use code `u` (stock `FrMerkleHasher::compress`).
pub fn compress(u: u128, left: F, right: F) -> F {
    let mut s = [left, right, cap_fixed(u, 2)];
    permute(&mut s);
    s[0]
}

/// Mirror of the stock overwrite duplex (`stark-backend/src/transcript/duplex_sponge.rs`).
#[derive(Clone, Debug)]
pub struct NativeTranscript {
    pub state: [F; WIDTH],
    pub absorb_idx: usize,
    pub sample_idx: usize,
}

impl NativeTranscript {
    pub fn new(u: u128) -> Self {
        Self { state: [F::ZERO, F::ZERO, cap_var(u)], absorb_idx: 0, sample_idx: 0 }
    }

    pub fn observe(&mut self, x: F) {
        self.state[self.absorb_idx] = x;
        self.absorb_idx += 1;
        if self.absorb_idx == RATE {
            permute(&mut self.state);
            self.absorb_idx = 0;
            self.sample_idx = RATE;
        }
    }

    pub fn sample(&mut self) -> F {
        if self.absorb_idx != 0 || self.sample_idx == 0 {
            permute(&mut self.state);
            self.absorb_idx = 0;
            self.sample_idx = RATE;
        }
        self.sample_idx -= 1;
        self.state[self.sample_idx]
    }

    /// Low `bits` bits of the canonical representative of the next sample.
    pub fn sample_bits(&mut self, bits: usize) -> u64 {
        assert!(bits < 32);
        low_u64(&self.sample()) & ((1u64 << bits) - 1)
    }

    pub fn check_witness(&mut self, bits: usize, w: F) -> bool {
        if bits == 0 {
            return true;
        }
        self.observe(w);
        self.sample_bits(bits) == 0
    }
}

/// Low 64 bits of the canonical representative.
pub fn low_u64(x: &F) -> u64 {
    let r = x.to_repr();
    u64::from_le_bytes(r.as_ref()[..8].try_into().unwrap())
}

/// 64-digit big-endian hex (the `wrapin` and stark-backend codec order).
pub fn f_to_hex(x: &F) -> String {
    let mut b = x.to_repr().as_ref().to_vec();
    b.reverse();
    hex::encode(b)
}

pub fn f_from_hex(s: &str) -> Option<F> {
    let mut b = hex::decode(s.trim_start_matches("0x")).ok()?;
    if b.len() != 32 {
        return None;
    }
    b.reverse();
    let mut repr = <F as PrimeField>::Repr::default();
    repr.as_mut().copy_from_slice(&b);
    Option::from(F::from_repr(repr))
}
