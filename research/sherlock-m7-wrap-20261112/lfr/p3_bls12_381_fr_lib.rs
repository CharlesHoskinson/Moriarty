//! The scalar field `Fr` of the BLS12-381 curve as a Plonky3 0.4.3 field.
//!
//! `r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001` (255 bits).
//!
//! Forked from `p3-bn254` 0.4.3 (trait layout). The arithmetic is not reimplemented: every
//! operation delegates to `midnight_curves::Fq` (the BLS12-381 scalar field of midnight-curves
//! 0.3.1, blst Montgomery backend), which is also the field type of Midnight's Poseidon. The
//! wrapper is `repr(transparent)`, so a `[Bls12381Fr; N]` and a `[Fq; N]` share one layout.
//!
//! Serialization (serde and `RawDataSerializable`) uses the canonical little-endian 32-byte
//! encoding (`ff::PrimeField::to_repr`), and deserialization rejects non-canonical values.
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use ff::{Field as FfField, PrimeField as FfPrimeField};
pub use midnight_curves::Fq as MidnightFq;
use num_bigint::BigUint;
use p3_field::integers::QuotientMap;
use p3_field::{
    Field, InjectiveMonomial, Packable, PrimeCharacteristicRing, PrimeField, RawDataSerializable,
    TwoAdicField,
};
use rand::Rng;
use rand::distr::{Distribution, StandardUniform};
use serde::{Deserialize, Deserializer, Serialize};

/// The BLS12-381 scalar field prime as little-endian u64 limbs.
pub const BLS12_381_FR_PRIME: [u64; 4] = [
    0xffff_ffff_0000_0001,
    0x53bd_a402_fffe_5bfe,
    0x3339_d808_09a1_d805,
    0x73ed_a753_299d_7d48,
];

/// `r - 1` as little-endian u64 limbs.
const PRIME_MINUS_ONE: [u64; 4] = [
    0xffff_ffff_0000_0000,
    0x53bd_a402_fffe_5bfe,
    0x3339_d808_09a1_d805,
    0x73ed_a753_299d_7d48,
];

/// An element of the BLS12-381 scalar field.
#[derive(Copy, Clone, Default, Eq, PartialEq)]
#[repr(transparent)]
#[must_use]
pub struct Bls12381Fr(pub MidnightFq);

impl Bls12381Fr {
    /// Wrap a midnight-curves value.
    #[inline]
    pub const fn from_ff(x: MidnightFq) -> Self {
        Self(x)
    }

    /// The midnight-curves value.
    #[inline]
    pub const fn to_ff(self) -> MidnightFq {
        self.0
    }

    /// From a little-endian u64 integer (any value, reduced mod r). Const.
    #[inline]
    pub const fn from_u64_limbs(limbs: [u64; 4]) -> Self {
        Self(MidnightFq::from_raw(limbs))
    }

    /// From a canonical little-endian 32-byte encoding; `None` if the value is >= r.
    #[inline]
    pub fn from_canonical_bytes(bytes: [u8; 32]) -> Option<Self> {
        Option::from(MidnightFq::from_repr(bytes)).map(Self)
    }

    /// The canonical little-endian 32-byte encoding.
    #[inline]
    pub fn to_canonical_bytes(self) -> [u8; 32] {
        self.0.to_repr()
    }

    /// The canonical value as little-endian u64 limbs.
    #[inline]
    pub fn to_canonical_u64_limbs(self) -> [u64; 4] {
        let b = self.to_canonical_bytes();
        core::array::from_fn(|i| u64::from_le_bytes(b[8 * i..8 * i + 8].try_into().unwrap()))
    }

    /// From a big integer; reduced mod r. `None` if it has more than 4 u64 digits.
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_biguint(value: BigUint) -> Option<Self> {
        let digits = value.to_u64_digits();
        if digits.len() > 4 {
            return None;
        }
        let mut limbs = [0u64; 4];
        limbs[..digits.len()].copy_from_slice(&digits);
        Some(Self::from_u64_limbs(limbs))
    }
}

impl From<MidnightFq> for Bls12381Fr {
    #[inline]
    fn from(x: MidnightFq) -> Self {
        Self(x)
    }
}

impl From<Bls12381Fr> for MidnightFq {
    #[inline]
    fn from(x: Bls12381Fr) -> Self {
        x.0
    }
}

impl Serialize for Bls12381Fr {
    #[inline]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&self.to_canonical_bytes())
    }
}

impl<'de> Deserialize<'de> for Bls12381Fr {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let bytes: Vec<u8> = Deserialize::deserialize(d)?;
        let arr: [u8; 32] = bytes
            .as_slice()
            .try_into()
            .map_err(|_| serde::de::Error::custom("BLS12-381 Fr needs 32 bytes"))?;
        Self::from_canonical_bytes(arr)
            .ok_or_else(|| serde::de::Error::custom("non-canonical BLS12-381 Fr element"))
    }
}

impl Packable for Bls12381Fr {}

impl Hash for Bls12381Fr {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Ord for Bls12381Fr {
    #[inline]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // midnight-curves orders by the canonical big-endian bytes.
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Bls12381Fr {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Bls12381Fr {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.as_canonical_biguint(), f)
    }
}

impl Debug for Bls12381Fr {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.as_canonical_biguint(), f)
    }
}

impl PrimeCharacteristicRing for Bls12381Fr {
    type PrimeSubfield = Self;

    const ZERO: Self = Self(<MidnightFq as FfField>::ZERO);
    const ONE: Self = Self(<MidnightFq as FfField>::ONE);
    const TWO: Self = Self::from_u64_limbs([2, 0, 0, 0]);
    const NEG_ONE: Self = Self::from_u64_limbs(PRIME_MINUS_ONE);

    #[inline]
    fn from_prime_subfield(f: Self::PrimeSubfield) -> Self {
        f
    }

    #[inline]
    fn halve(&self) -> Self {
        Self(self.0 * <MidnightFq as FfPrimeField>::TWO_INV)
    }

    #[inline]
    fn square(&self) -> Self {
        Self(FfField::square(&self.0))
    }

    #[inline]
    fn double(&self) -> Self {
        Self(FfField::double(&self.0))
    }
}

/// `gcd(r - 1, 5) = 1`, the S-box degree of Midnight's Poseidon.
impl InjectiveMonomial<5> for Bls12381Fr {}

impl RawDataSerializable for Bls12381Fr {
    const NUM_BYTES: usize = 32;

    #[allow(refining_impl_trait)]
    #[inline]
    fn into_bytes(self) -> [u8; 32] {
        self.to_canonical_bytes()
    }

    #[inline]
    fn into_u32_stream(input: impl IntoIterator<Item = Self>) -> impl IntoIterator<Item = u32> {
        input.into_iter().flat_map(|x| {
            x.to_canonical_u64_limbs()
                .into_iter()
                .flat_map(|d| [d as u32, (d >> 32) as u32])
        })
    }

    #[inline]
    fn into_u64_stream(input: impl IntoIterator<Item = Self>) -> impl IntoIterator<Item = u64> {
        input.into_iter().flat_map(|x| x.to_canonical_u64_limbs())
    }

    #[inline]
    fn into_parallel_byte_streams<const N: usize>(
        input: impl IntoIterator<Item = [Self; N]>,
    ) -> impl IntoIterator<Item = [u8; N]> {
        input.into_iter().flat_map(|vector| {
            let bytes = vector.map(|e| e.to_canonical_bytes());
            (0..Self::NUM_BYTES).map(move |i| core::array::from_fn(|j| bytes[j][i]))
        })
    }

    #[inline]
    fn into_parallel_u32_streams<const N: usize>(
        input: impl IntoIterator<Item = [Self; N]>,
    ) -> impl IntoIterator<Item = [u32; N]> {
        input.into_iter().flat_map(|vector| {
            let u32s: [[u32; 8]; N] = vector.map(|e| {
                let l = e.to_canonical_u64_limbs();
                core::array::from_fn(|k| (l[k / 2] >> (32 * (k % 2))) as u32)
            });
            (0..8).map(move |i| core::array::from_fn(|j| u32s[j][i]))
        })
    }

    #[inline]
    fn into_parallel_u64_streams<const N: usize>(
        input: impl IntoIterator<Item = [Self; N]>,
    ) -> impl IntoIterator<Item = [u64; N]> {
        input.into_iter().flat_map(|vector| {
            let u64s = vector.map(|e| e.to_canonical_u64_limbs());
            (0..4).map(move |i| core::array::from_fn(|j| u64s[j][i]))
        })
    }
}

impl Field for Bls12381Fr {
    type Packing = Self;

    /// 7, the multiplicative generator of midnight-curves (`ff::PrimeField::MULTIPLICATIVE_GENERATOR`).
    const GENERATOR: Self = Self::from_u64_limbs([7, 0, 0, 0]);

    #[inline]
    fn is_zero(&self) -> bool {
        bool::from(FfField::is_zero(&self.0))
    }

    #[inline]
    fn try_inverse(&self) -> Option<Self> {
        Option::from(FfField::invert(&self.0)).map(Self)
    }

    #[inline]
    fn order() -> BigUint {
        BigUint::from_slice(&[
            BLS12_381_FR_PRIME[0] as u32,
            (BLS12_381_FR_PRIME[0] >> 32) as u32,
            BLS12_381_FR_PRIME[1] as u32,
            (BLS12_381_FR_PRIME[1] >> 32) as u32,
            BLS12_381_FR_PRIME[2] as u32,
            (BLS12_381_FR_PRIME[2] >> 32) as u32,
            BLS12_381_FR_PRIME[3] as u32,
            (BLS12_381_FR_PRIME[3] >> 32) as u32,
        ])
    }
}

macro_rules! quotient_map_unsigned {
    ($($t:ty),*) => {$(
        impl QuotientMap<$t> for Bls12381Fr {
            #[inline]
            fn from_int(int: $t) -> Self {
                let v = int as u128;
                Self::from_u64_limbs([v as u64, (v >> 64) as u64, 0, 0])
            }
            #[inline]
            fn from_canonical_checked(int: $t) -> Option<Self> {
                Some(Self::from_int(int))
            }
            #[inline]
            unsafe fn from_canonical_unchecked(int: $t) -> Self {
                Self::from_int(int)
            }
        }
    )*};
}

macro_rules! quotient_map_signed {
    ($($t:ty),*) => {$(
        impl QuotientMap<$t> for Bls12381Fr {
            #[inline]
            fn from_int(int: $t) -> Self {
                let v = int as i128;
                if v >= 0 {
                    <Self as QuotientMap<u128>>::from_int(v as u128)
                } else {
                    -<Self as QuotientMap<u128>>::from_int(v.unsigned_abs())
                }
            }
            #[inline]
            fn from_canonical_checked(int: $t) -> Option<Self> {
                Some(Self::from_int(int))
            }
            #[inline]
            unsafe fn from_canonical_unchecked(int: $t) -> Self {
                Self::from_int(int)
            }
        }
    )*};
}

quotient_map_unsigned!(u8, u16, u32, u64, u128);
quotient_map_signed!(i8, i16, i32, i64, i128);

impl PrimeField for Bls12381Fr {
    #[inline]
    fn as_canonical_biguint(&self) -> BigUint {
        BigUint::from_bytes_le(&self.to_canonical_bytes())
    }
}

impl Add for Bls12381Fr {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub for Bls12381Fr {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Neg for Bls12381Fr {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self(-self.0)
    }
}

impl Mul for Bls12381Fr {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Div for Bls12381Fr {
    type Output = Self;
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline]
    fn div(self, rhs: Self) -> Self {
        self * rhs.inverse()
    }
}

impl AddAssign for Bls12381Fr {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Bls12381Fr {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Bls12381Fr {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Bls12381Fr {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl Sum for Bls12381Fr {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl Product for Bls12381Fr {
    #[inline]
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl Distribution<Bls12381Fr> for StandardUniform {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Bls12381Fr {
        // Rejection sampling on 255-bit integers.
        loop {
            let mut bytes: [u8; 32] = rng.random();
            bytes[31] &= 0x7f;
            if let Some(x) = Bls12381Fr::from_canonical_bytes(bytes) {
                return x;
            }
        }
    }
}

impl TwoAdicField for Bls12381Fr {
    const TWO_ADICITY: usize = 32;

    #[inline]
    fn two_adic_generator(bits: usize) -> Self {
        assert!(bits <= Self::TWO_ADICITY);
        let mut omega = Self(<MidnightFq as FfPrimeField>::ROOT_OF_UNITY);
        for _ in bits..Self::TWO_ADICITY {
            omega = omega.square();
        }
        omega
    }
}

#[cfg(test)]
mod tests {
    // The cross-checks against the ff type live in the stark-bench-ovm test
    // `tests/m7_bls_fr.rs`, which runs inside the openvm workspace lockfile.
}
