//! BabyBear + BLS12-381 Fr Midnight-Poseidon STARK configuration (Sherlock M7, last layer `L`).
//!
//! Modelled on [`super::baby_bear_bn254_poseidon2`]: BabyBear base field, `EF = BabyBear^4`,
//! digests of one Fr element, 8 BabyBear words packed per Fr element (base 2^31), 5 base-p
//! digits per squeeze. The hash domains are those of PROPOSAL section 3.6:
//!
//! - **Permutation**: Midnight Poseidon over BLS12-381 Fr, width 3, rate 2, 8 full + 60 partial
//!   rounds, x^5, constants of midnight-circuits 7.3.0. The permutation is midnight-circuits'
//!   own CPU permutation (`midnight_circuits::hash::poseidon::permutation_cpu`), called on
//!   `midnight_curves::Fq`, the type that [`Bls12381Fr`] wraps.
//! - **Merkle leaf (u = 1)**: Midnight's fixed-length sponge. The leaf's `w` BabyBear words are
//!   packed 8 per Fr element (the last one holds `w mod 8` words when that is not 0), so
//!   `l = ceil(w / 8)`. State `(0, 0, l + 1 * 2^66)`; each block of 2 elements is added into
//!   `(r_0, r_1)` (a last block of 1 adds only `r_0`), one permutation per block; output `r_0`.
//! - **Merkle node (u = 2)**: the same sponge with `l = 2`: state `(left, right, 2 + 2 * 2^66)`,
//!   one permutation, output `r_0`.
//! - **Transcript (u = 3)**: the stock multi-field overwrite duplex
//!   ([`MultiFieldTranscript`]) with this permutation and the initial state
//!   `(0, 0, 2^64 + 3 * 2^66)`. Every transition is stock.

use std::{
    io::{self, Read, Write},
    sync::Arc,
};

use midnight_circuits::hash::poseidon::{permutation_cpu, round_skips::PreComputedRoundCPU};
use num_bigint::BigUint;
use openvm_stark_backend::{
    codec::{
        decode_extension_field32, decode_prime_field32, encode_extension_field32,
        encode_prime_field32, DecodableConfig, EncodableConfig,
    },
    hasher::MerkleHasher,
    p3_symmetric::{CryptographicPermutation, Permutation},
    prover::{Coordinator, CpuColMajorBackend, ReferenceDevice},
    transcript::multi_field::MultiFieldTranscript,
    FiatShamirTranscript, StarkEngine, StarkProtocolConfig, SystemParams,
};
use p3_baby_bear::BabyBear;
pub use p3_bls12_381_fr::{Bls12381Fr, MidnightFq};
use p3_field::{
    extension::BinomialExtensionField, Field, PrimeCharacteristicRing, PrimeField, PrimeField32,
};

/// Width of the Midnight Poseidon permutation.
pub const WIDTH: usize = 3;
/// Rate in Fr elements.
pub const FR_RATE: usize = 2;
/// Digest width in Fr elements.
pub const DIGEST_WIDTH: usize = 1;
/// BabyBear words packed per Fr element.
pub const WORDS_PER_FR: usize = 8;

/// Use code of the Merkle leaf hash (PROPOSAL 3.6).
pub const U_LEAF: u128 = 1;
/// Use code of the Merkle node hash.
pub const U_NODE: u128 = 2;
/// Use code of the transcript.
pub const U_TRANSCRIPT: u128 = 3;

pub type F = BabyBear;
pub type EF = BinomialExtensionField<BabyBear, 4>;
pub const D_EF: usize = 4;
pub type Digest = [Bls12381Fr; DIGEST_WIDTH];
type SC = BabyBearBls12381PoseidonConfig;

/// `cap(u, l) = l + u * 2^66`, the initial capacity of a fixed-length hash of length `l`.
pub fn cap_fixed(u: u128, l: u128) -> Bls12381Fr {
    assert!(l < (1u128 << 64), "length must be below 2^64");
    Bls12381Fr::from_u128(l + (u << 66))
}

/// `2^64 + u * 2^66`, the initial capacity of a variable-length sponge with use code `u`.
pub fn cap_var(u: u128) -> Bls12381Fr {
    Bls12381Fr::from_u128((1u128 << 64) + (u << 66))
}

/// Midnight Poseidon permutation over BLS12-381 Fr (midnight-circuits 7.3.0 CPU code). The
/// round-skip tables are built once and shared.
#[derive(Clone)]
pub struct MidnightPoseidonFr {
    pre: Arc<PreComputedRoundCPU<MidnightFq>>,
}

impl std::fmt::Debug for MidnightPoseidonFr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("MidnightPoseidonFr")
    }
}

impl Default for MidnightPoseidonFr {
    fn default() -> Self {
        Self::new()
    }
}

impl MidnightPoseidonFr {
    pub fn new() -> Self {
        Self {
            pre: Arc::new(PreComputedRoundCPU::<MidnightFq>::init()),
        }
    }

    /// The permutation on midnight-curves values.
    #[inline]
    pub fn permute_ff(&self, state: &mut [MidnightFq; WIDTH]) {
        permutation_cpu(&self.pre, &mut state[..]);
    }
}

impl Permutation<[Bls12381Fr; WIDTH]> for MidnightPoseidonFr {
    #[inline]
    fn permute_mut(&self, state: &mut [Bls12381Fr; WIDTH]) {
        let mut s = state.map(Bls12381Fr::to_ff);
        self.permute_ff(&mut s);
        *state = s.map(Bls12381Fr::from_ff);
    }
}

impl CryptographicPermutation<[Bls12381Fr; WIDTH]> for MidnightPoseidonFr {}

/// Base 2^31 packing of up to 8 BabyBear words: `sum_i b_i 2^(31 i)` (as `pack_f_to_sf`).
#[inline]
pub fn pack_words(words: &[F]) -> Bls12381Fr {
    debug_assert!(words.len() <= WORDS_PER_FR);
    let base = Bls12381Fr::from_u64(1u64 << 31);
    words.iter().rev().fold(Bls12381Fr::ZERO, |acc, w| {
        acc * base + Bls12381Fr::from_u32(w.as_canonical_u32())
    })
}

/// Midnight's fixed-length sponge with use code `u`: state `(0, 0, cap(u, l))`, additive
/// absorption in blocks of 2, one permutation per block, output `r_0`. With `u = 0` this is
/// Midnight's native fixed-length `PoseidonChip::hash`.
pub fn fixed_len_hash(perm: &MidnightPoseidonFr, u: u128, inputs: &[Bls12381Fr]) -> Bls12381Fr {
    let mut state = [Bls12381Fr::ZERO, Bls12381Fr::ZERO, cap_fixed(u, inputs.len() as u128)];
    for block in inputs.chunks(FR_RATE) {
        for (s, x) in state.iter_mut().zip(block) {
            *s += *x;
        }
        perm.permute_mut(&mut state);
    }
    state[0]
}

/// Merkle hasher of `L`: leaf hash u = 1 and node hash u = 2 (PROPOSAL 3.6).
#[derive(Clone, Debug, Default)]
pub struct MidnightMerkleHasher {
    perm: MidnightPoseidonFr,
}

impl MidnightMerkleHasher {
    pub fn new(perm: MidnightPoseidonFr) -> Self {
        Self { perm }
    }

    pub fn perm(&self) -> &MidnightPoseidonFr {
        &self.perm
    }
}

impl MerkleHasher for MidnightMerkleHasher {
    type F = F;
    type Digest = Digest;

    #[inline]
    fn hash_slice(&self, vals: &[F]) -> Digest {
        let l = vals.len().div_ceil(WORDS_PER_FR);
        let mut state = [Bls12381Fr::ZERO, Bls12381Fr::ZERO, cap_fixed(U_LEAF, l as u128)];
        for block in vals.chunks(WORDS_PER_FR * FR_RATE) {
            for (s, words) in state.iter_mut().zip(block.chunks(WORDS_PER_FR)) {
                *s += pack_words(words);
            }
            self.perm.permute_mut(&mut state);
        }
        [state[0]]
    }

    #[inline]
    fn compress(&self, left: Digest, right: Digest) -> Digest {
        // Fixed-length sponge, l = 2, u = 2: (0 + left, 0 + right, 2 + 2 * 2^66), one permutation.
        let mut state = [left[0], right[0], cap_fixed(U_NODE, 2)];
        self.perm.permute_mut(&mut state);
        [state[0]]
    }
}

#[derive(Clone, Debug)]
pub struct BabyBearBls12381PoseidonConfig {
    params: SystemParams,
    hasher: MidnightMerkleHasher,
}

impl BabyBearBls12381PoseidonConfig {
    pub fn new(params: SystemParams, perm: MidnightPoseidonFr) -> Self {
        Self {
            params,
            hasher: MidnightMerkleHasher::new(perm),
        }
    }

    pub fn default_from_params(params: SystemParams) -> Self {
        Self::new(params, MidnightPoseidonFr::new())
    }
}

impl StarkProtocolConfig for BabyBearBls12381PoseidonConfig {
    type F = F;
    type EF = EF;
    type Digest = Digest;
    type Hasher = MidnightMerkleHasher;

    fn params(&self) -> &SystemParams {
        &self.params
    }

    fn hasher(&self) -> &Self::Hasher {
        &self.hasher
    }
}

impl EncodableConfig for BabyBearBls12381PoseidonConfig {
    fn encode_base_field<W: Write>(val: &F, writer: &mut W) -> io::Result<()> {
        encode_prime_field32(val, writer)
    }

    fn encode_extension_field<W: Write>(val: &EF, writer: &mut W) -> io::Result<()> {
        encode_extension_field32::<Self::F, _, _>(val, writer)
    }

    fn encode_digest<W: Write>(digest: &Self::Digest, writer: &mut W) -> io::Result<()> {
        for val in digest {
            // 32 big-endian canonical bytes, as the BN254 template.
            let mut buf = val.to_canonical_bytes();
            buf.reverse();
            writer.write_all(&buf)?;
        }
        Ok(())
    }
}

impl DecodableConfig for BabyBearBls12381PoseidonConfig {
    fn decode_base_field<R: Read>(reader: &mut R) -> io::Result<F> {
        decode_prime_field32(reader)
    }

    fn decode_extension_field<R: Read>(reader: &mut R) -> io::Result<EF> {
        decode_extension_field32::<F, _, _>(reader)
    }

    fn decode_digest<R: Read>(reader: &mut R) -> io::Result<Digest> {
        let mut result = Digest::default();
        for val in &mut result {
            let mut buf = [0u8; 32];
            reader.read_exact(&mut buf)?;
            if BigUint::from_bytes_be(&buf) >= Bls12381Fr::order() {
                return Err(io::Error::other("non-canonical BLS12-381 Fr element >= modulus"));
            }
            buf.reverse();
            *val = Bls12381Fr::from_canonical_bytes(buf)
                .ok_or_else(|| io::Error::other("invalid BLS12-381 Fr element"))?;
        }
        Ok(result)
    }
}

/// The stock multi-field transcript over Fr with Midnight's permutation.
pub type InnerTranscript = MultiFieldTranscript<F, Bls12381Fr, MidnightPoseidonFr, WIDTH, FR_RATE>;

/// Transcript of `L` (u = 3): the stock multi-field overwrite duplex with initial state
/// `(0, 0, 2^64 + 3 * 2^66)`.
#[derive(Clone, Debug)]
pub struct Transcript(pub InnerTranscript);

impl Transcript {
    /// The initial sponge state `(0, 0, 2^64 + 3 * 2^66)`.
    pub fn initial_state() -> [Bls12381Fr; WIDTH] {
        [Bls12381Fr::ZERO, Bls12381Fr::ZERO, cap_var(U_TRANSCRIPT)]
    }

    pub fn new(perm: MidnightPoseidonFr) -> Self {
        Self(InnerTranscript::new_with_sponge_state(perm, Self::initial_state()))
    }

    pub fn inner(&self) -> &InnerTranscript {
        &self.0
    }
}

impl From<MidnightPoseidonFr> for Transcript {
    fn from(perm: MidnightPoseidonFr) -> Self {
        Self::new(perm)
    }
}

impl FiatShamirTranscript<SC> for Transcript {
    #[inline]
    fn observe(&mut self, value: F) {
        self.0.observe(value);
    }

    #[inline]
    fn sample(&mut self) -> F {
        self.0.sample()
    }

    #[inline]
    fn observe_commit(&mut self, digest: Digest) {
        FiatShamirTranscript::<SC>::observe_commit(&mut self.0, digest);
    }
}

pub fn default_transcript() -> Transcript {
    Transcript::new(MidnightPoseidonFr::new())
}

/// Reference (column-major) engine.
pub struct BabyBearBls12381PoseidonRefEngine {
    device: ReferenceDevice<SC>,
}

impl StarkEngine for BabyBearBls12381PoseidonRefEngine {
    type SC = SC;
    type PB = CpuColMajorBackend<SC>;
    type PD = ReferenceDevice<SC>;
    type TS = Transcript;

    fn new(params: SystemParams) -> Self {
        Self {
            device: ReferenceDevice::new(BabyBearBls12381PoseidonConfig::default_from_params(
                params,
            )),
        }
    }

    fn config(&self) -> &SC {
        self.device.config()
    }

    fn device(&self) -> &Self::PD {
        &self.device
    }

    fn initial_transcript(&self) -> Self::TS {
        Transcript::new(self.config().hasher().perm().clone())
    }

    fn prover_from_transcript(
        &self,
        transcript: Transcript,
    ) -> Coordinator<Self::SC, Self::PB, Self::PD, Self::TS> {
        Coordinator::new(CpuColMajorBackend::new(), self.device.clone(), transcript)
    }
}

#[cfg(feature = "cpu-backend")]
mod cpu_engine {
    use openvm_cpu_backend::{CpuBackend, CpuDevice};

    use super::*;

    /// Row-major CPU engine for BabyBear + BLS12-381 Fr Midnight Poseidon.
    pub struct BabyBearBls12381PoseidonCpuEngine {
        device: CpuDevice<SC>,
    }

    impl StarkEngine for BabyBearBls12381PoseidonCpuEngine {
        type SC = SC;
        type PB = CpuBackend<SC>;
        type PD = CpuDevice<SC>;
        type TS = Transcript;

        fn new(params: SystemParams) -> Self {
            Self {
                device: CpuDevice::new(BabyBearBls12381PoseidonConfig::default_from_params(
                    params,
                )),
            }
        }

        fn config(&self) -> &SC {
            self.device.config()
        }

        fn device(&self) -> &Self::PD {
            &self.device
        }

        fn initial_transcript(&self) -> Self::TS {
            Transcript::new(self.config().hasher().perm().clone())
        }

        fn prover_from_transcript(
            &self,
            transcript: Transcript,
        ) -> Coordinator<Self::SC, Self::PB, Self::PD, Self::TS> {
            Coordinator::new(CpuBackend::new(), self.device.clone(), transcript)
        }
    }
}

#[cfg(feature = "cpu-backend")]
pub use cpu_engine::BabyBearBls12381PoseidonCpuEngine;

// Keep the unused-import lint quiet when only some items are used.
#[allow(dead_code)]
fn _assert_traits() {
    fn is_prime_field<T: PrimeField + Field>() {}
    is_prime_field::<Bls12381Fr>();
}
