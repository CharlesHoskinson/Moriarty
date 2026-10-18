# Requested public field/scalar source excerpts

Mechanical extraction for owner inspection only. No effects, soundness,
security, proof, authorization or resource conclusion is supplied.

All six complete original files are copied byte for byte under `full-originals/`.
`MANIFEST.json` pins complete sizes/hashes, parent-manifest identities and each
excerpt's original line range, original zero-based half-open byte range, byte
count and hash. `SOURCE-INDEX.md` lists included and omitted ranges. Line labels
below are added presentation; strip `NNNN|` to recover each exact source line.
C files are reproduced in full, including their existing comments and branches.
Rust excerpts omit all other ranges explicitly, including tests where possible.
No required function body is shortened within the listed ranges.

Coverage limit: Fr equality is a derive on the wrapper. The Fr arithmetic
macro invocation/import and signed-conversion negation callsite appear below;
the imported `wrap_field_arith` macro body is outside these six requested files
and is NOT supplied. Likewise the trait implementation/default body reached by
`outer::Scalar::from_u128` is not supplied merely by showing that callsite.
This is the requested finite source packet, not a complete dependency closure.
The complete originals retain all omitted source and tests. No source was run.

## Fr — `transient-crypto/src/curve.rs`

Lines 21–24; original bytes [904,1137); SHA-256 `eecdedd14b5dc54edc936847f43d6c81a0845165a2a6e5fdc234ebfe2d41c396`.

```rust
0021|use crate::macros::{fr_display, wrap_display, wrap_field_arith, wrap_group_arith};
0022|use base_crypto::fab::{Aligned, Alignment, AlignmentAtom, AlignmentSegment};
0023|use fake::{Dummy, Faker};
0024|use ff::{Field, FromUniformBytes, PrimeField};
```

Lines 50–60; original bytes [1992,2442); SHA-256 `a3b9e5bc56050e62df671015f1cea9472de949a34c2c2bcd5613f14ac13402b2`.

```rust
0050|/// The outer, main curve
0051|pub mod outer {
0052|    /// The base prime field, used to represent curve points
0053|    pub type Base = midnight_curves::Fp;
0054|    /// The scalar prime field, used in circuit
0055|    pub type Scalar = midnight_curves::Fq;
0056|    /// The affine representation of a curve point
0057|    pub type Affine = midnight_curves::G1Affine;
0058|    /// The size of the outer curve point in bytes
0059|    pub const POINT_BYTES: usize = Affine::compressed_size();
0060|}
```

Lines 75–161; original bytes [3106,6492); SHA-256 `553d01840db4cd0608d0f46857f6e0532c411241499f6e826790ee437605de5e`.

```rust
0075|// Since field elements are large, often sparse, and very common, we handle them specially
0076|// for Borsh serialization: We begin with one byte indicating how many bytes are
0077|// required to little-endian encode the field element, and then that many bytes
0078|// of little endian encoding.
0079|//
0080|// For this encoding to be unique, it is an error for the last byte to be zero.
0081|// (Zero itself is represented as the zero length)
0082|//
0083|// In pseudocode:
0084|//
0085|// b = ceil(f.log2() / 8)
0086|// little_endian(f as u<b>)
0087|macro_rules! field_serialize {
0088|    ($name:ident, $wrapped:ty, $to_bytes:ident) => {
0089|        impl Serialize for $name {
0090|            fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
0091|                let mut vec = Vec::new();
0092|                <$name as Serializable>::serialize(self, &mut vec).map_err(S::Error::custom)?;
0093|                ser.serialize_bytes(&vec)
0094|            }
0095|        }
0096|
0097|        impl<'de> Deserialize<'de> for $name {
0098|            fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
0099|                let bytes = serde_bytes::ByteBuf::deserialize(de)?;
0100|                <$name as Deserializable>::deserialize(&mut &bytes[..], 0)
0101|                    .map_err(serde::de::Error::custom)
0102|            }
0103|        }
0104|
0105|        impl From<$name> for ScaleBigInt {
0106|            fn from(val: $name) -> ScaleBigInt {
0107|                let mut res = ScaleBigInt::default();
0108|                let repr = val.0.to_repr();
0109|                res.0[..repr.len()].copy_from_slice(&repr);
0110|                res
0111|            }
0112|        }
0113|
0114|        impl TryFrom<ScaleBigInt> for $name {
0115|            type Error = std::io::Error;
0116|            fn try_from(val: ScaleBigInt) -> std::io::Result<$name> {
0117|                if val.0[FR_BYTES..].iter().any(|b| *b != 0) {
0118|                    return Err(std::io::Error::new(
0119|                        std::io::ErrorKind::InvalidData,
0120|                        concat!("out of range for ", stringify!($ty)),
0121|                    ));
0122|                }
0123|                let cont_buf: [u8; FR_BYTES] = val.0[..FR_BYTES]
0124|                    .try_into()
0125|                    .expect("slice of known size must coerce to array");
0126|                Ok($name(
0127|                    <Option<_>>::from(<$wrapped>::from_repr(cont_buf.into())).ok_or_else(|| {
0128|                        io::Error::new(
0129|                            io::ErrorKind::InvalidData,
0130|                            concat!("out of range for ", stringify!($name)),
0131|                        )
0132|                    })?,
0133|                ))
0134|            }
0135|        }
0136|
0137|        impl Serializable for $name {
0138|            fn serialize(&self, writer: &mut impl Write) -> io::Result<()> {
0139|                ScaleBigInt::from(*self).serialize(writer)
0140|            }
0141|
0142|            fn serialized_size(&self) -> usize {
0143|                ScaleBigInt::from(*self).serialized_size()
0144|            }
0145|        }
0146|
0147|        impl Deserializable for $name {
0148|            fn deserialize(reader: &mut impl Read, recursion_depth: u32) -> io::Result<Self> {
0149|                <$name>::try_from(ScaleBigInt::deserialize(reader, recursion_depth)?)
0150|            }
0151|        }
0152|    };
0153|}
0154|
0155|/// An element of our primary prime field.
0156|#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Storable)]
0157|#[storable(base)]
0158|pub struct Fr(pub outer::Scalar);
0159|wrap_field_arith!(Fr);
0160|fr_display!(Fr);
0161|field_serialize!(Fr, outer::Scalar, to_bytes_le);
```

Lines 191–196; original bytes [7140,7468); SHA-256 `4bde0e7fc918372b9c024018d5c1d7e70e7eee77aafaf331f50bae8b39e029bd`.

```rust
0191|/// The number of bits required to represent [Fr].
0192|pub const FR_BITS: usize = <outer::Scalar as PrimeField>::NUM_BITS as usize;
0193|/// The number of bytes required to represent [Fr].
0194|pub const FR_BYTES: usize = FR_BITS.div_ceil(8);
0195|/// The number of bytes which can fit in an [Fr].
0196|pub const FR_BYTES_STORED: usize = FR_BYTES - 1;
```

Lines 240–289; original bytes [8629,9783); SHA-256 `e6daf054261e4cbee62e4def4c35687cb966a538036b33b42db5c53b23d6bb13`.

```rust
0240|macro_rules! derive_via {
0241|    ($base:ty, $via:ty, $($ty:ty),*) => {
0242|        $(
0243|        impl From<$ty> for $base {
0244|            fn from(val: $ty) -> $base {
0245|                (val as $via).into()
0246|            }
0247|        }
0248|        )*
0249|    }
0250|}
0251|
0252|macro_rules! derive_signed {
0253|    ($base:ty, $($ty:ty, $via:ty),*) => {
0254|        $(
0255|        impl From<$ty> for $base {
0256|            fn from(val: $ty) -> $base {
0257|                if val < 0 {
0258|                    -<$base>::from(val.unsigned_abs())
0259|                } else {
0260|                    (val as $via).into()
0261|                }
0262|            }
0263|        }
0264|        )*
0265|    }
0266|}
0267|
0268|impl From<bool> for Fr {
0269|    fn from(val: bool) -> Fr {
0270|        Fr(outer::Scalar::from(u64::from(val)))
0271|    }
0272|}
0273|
0274|derive_via!(Fr, u64, u8, u16, u32);
0275|derive_signed!(Fr, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);
0276|derive_via!(EmbeddedFr, u64, u8, u16, u32);
0277|derive_signed!(EmbeddedFr, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128);
0278|
0279|impl From<u64> for Fr {
0280|    fn from(val: u64) -> Fr {
0281|        Fr(outer::Scalar::from(val))
0282|    }
0283|}
0284|
0285|impl From<u128> for Fr {
0286|    fn from(val: u128) -> Fr {
0287|        Fr(outer::Scalar::from_u128(val))
0288|    }
0289|}
```

Lines 344–360; original bytes [11030,11654); SHA-256 `dd20269ad52fd95184cecda5724c0d68de5ff71856e424c27e736d2392e084ba`.

```rust
0344|impl Fr {
0345|    /// Interpret a little-endiang byte-string as an [Fr].
0346|    pub fn from_le_bytes(bytes: &[u8]) -> Option<Self> {
0347|        let mut repr = [0u8; FR_BYTES];
0348|        if bytes.len() <= repr.len() {
0349|            repr[..bytes.len()].copy_from_slice(bytes)
0350|        } else {
0351|            return None;
0352|        }
0353|        outer::Scalar::from_repr(repr).map(Fr).into()
0354|    }
0355|
0356|    /// Initialize an [Fr] from arbitrary 64 bytes (little-endian)
0357|    /// ensuring the result falls into the space by taking modulo.
0358|    pub fn from_uniform_bytes(bytes: &[u8; 64]) -> Self {
0359|        Fr(outer::Scalar::from_uniform_bytes(bytes))
0360|    }
```

Lines 370–373; original bytes [11909,11998); SHA-256 `2b9b2275e78bd36cfc5d09dbcfae55f471c3a2e86775675ba9210113d49a5a21`.

```rust
0370|    pub fn as_le_bytes(&self) -> Vec<u8> {
0371|        self.0.to_bytes_le().to_vec()
0372|    }
0373|}
```

Omitted original line ranges: 1–20, 25–49, 61–74, 162–190, 197–239, 290–343, 361–369, 374–700.

## Fq — `midnight-curves-0.3.1/src/bls12_381/fq.rs`

Lines 1–29; original bytes [0,867); SHA-256 `c3c48b49b04344b1cfa7ffbcf12889ee2e27ac5d4251f63cd425ab885dbeb9b3`.

```rust
0001|//! An implementation of the BLS12-381 scalar field Fq,
0002|//! where `q =
0003|//! 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001`
0004|
0005|use core::{
0006|    borrow::Borrow,
0007|    cmp,
0008|    convert::TryInto,
0009|    fmt,
0010|    iter::{Product, Sum},
0011|    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
0012|};
0013|
0014|use blst::*;
0015|use byte_slice_cast::AsByteSlice;
0016|use ff::{Field, FieldBits, PrimeField, PrimeFieldBits, WithSmallOrderMulGroup};
0017|use rand_core::RngCore;
0018|use subtle::{Choice, ConditionallySelectable, ConstantTimeEq, CtOption};
0019|
0020|use crate::serde_traits::SerdeObject;
0021|
0022|/// Represents an element of the scalar field Fq of the BLS12-381 elliptic
0023|/// curve construction.
0024|///
0025|/// The inner representation `blst_fr` is stored in Montgomery form as
0026|/// little-endian `u64` limbs.
0027|#[derive(Default, Clone, Copy)]
0028|#[repr(transparent)]
0029|pub struct Fq(pub(crate) blst_fr);
```

Lines 42–48; original bytes [1143,1338); SHA-256 `bb6bcda486a712babb8f0afed359a7743004db1ee0d54bd9c2afcbbae200ce46`.

```rust
0042|// Little-endian non-Montgomery form not reduced mod p.
0043|const MODULUS: [u64; 4] = [
0044|    0xffff_ffff_0000_0001,
0045|    0x53bd_a402_fffe_5bfe,
0046|    0x3339_d808_09a1_d805,
0047|    0x73ed_a753_299d_7d48,
0048|];
```

Lines 173–189; original bytes [4653,5055); SHA-256 `26530c51a59d435baa37f018115254be9e1c8fd25ee397c194385b4970ae1fc1`.

```rust
0173|impl PartialEq for Fq {
0174|    #[inline]
0175|    fn eq(&self, other: &Self) -> bool {
0176|        self.0.l == other.0.l
0177|    }
0178|}
0179|
0180|impl Eq for Fq {}
0181|
0182|impl ConstantTimeEq for Fq {
0183|    fn ct_eq(&self, other: &Self) -> Choice {
0184|        self.0.l[0].ct_eq(&other.0.l[0])
0185|            & self.0.l[1].ct_eq(&other.0.l[1])
0186|            & self.0.l[2].ct_eq(&other.0.l[2])
0187|            & self.0.l[3].ct_eq(&other.0.l[3])
0188|    }
0189|}
```

Lines 216–222; original bytes [5695,5888); SHA-256 `f09c93aeabf256019b0d9c1383bc8ac40f6c49b0dcbbb160f3e9b3c14ed3ffb7`.

```rust
0216|impl From<u64> for Fq {
0217|    fn from(val: u64) -> Fq {
0218|        let mut repr = [0u8; 32];
0219|        repr[..8].copy_from_slice(&val.to_le_bytes());
0220|        Fq::from_bytes_le(&repr).unwrap()
0221|    }
0222|}
```

Lines 263–281; original bytes [6734,7028); SHA-256 `be243e22cc9a26ac0bc9cb4c0f23c4bd50a733795fa061c33cfda15c3663fa61`.

```rust
0263|impl Neg for &Fq {
0264|    type Output = Fq;
0265|
0266|    #[inline]
0267|    fn neg(self) -> Fq {
0268|        let mut neg = *self;
0269|        unsafe { blst_fr_cneg(&mut neg.0, &self.0, true) };
0270|        neg
0271|    }
0272|}
0273|
0274|impl Neg for Fq {
0275|    type Output = Fq;
0276|
0277|    #[inline]
0278|    fn neg(self) -> Fq {
0279|        -&self
0280|    }
0281|}
```

Lines 439–465; original bytes [10445,11125); SHA-256 `2a78d97887e230ea5c1ecebcce72dce7c36729dfd658a6c174e3856fde1fd7d4`.

```rust
0439|fn is_valid(a: &[u64]) -> bool {
0440|    debug_assert_eq!(a.len(), 4);
0441|    for (a, b) in a.iter().zip(MODULUS.iter()).rev() {
0442|        if a > b {
0443|            return false;
0444|        } else if a < b {
0445|            return true;
0446|        }
0447|    }
0448|
0449|    false
0450|}
0451|
0452|#[inline]
0453|fn u64s_from_bytes(bytes: &[u8; 32]) -> [u64; 4] {
0454|    [
0455|        u64::from_le_bytes(bytes[0..8].try_into().unwrap()),
0456|        u64::from_le_bytes(bytes[8..16].try_into().unwrap()),
0457|        u64::from_le_bytes(bytes[16..24].try_into().unwrap()),
0458|        u64::from_le_bytes(bytes[24..32].try_into().unwrap()),
0459|    ]
0460|}
0461|
0462|impl PrimeField for Fq {
0463|    // Little-endian non-Montgomery form bigint mod p.
0464|    type Repr = [u8; 32];
0465|
```

Lines 501–525; original bytes [12014,12835); SHA-256 `fa3e97d46ff893be429e0d9478339237a8fdbc2fcf34f06a1fe5bb6b509633a4`.

```rust
0501|    /// Constant representing the modulus
0502|    const MODULUS: &'static str =
0503|        "0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001";
0504|
0505|    /// Converts a little-endian non-Montgomery form `repr` into a Montgomery
0506|    /// form `Fq`.
0507|    fn from_repr(repr: Self::Repr) -> CtOption<Self> {
0508|        Self::from_bytes_le(&repr)
0509|    }
0510|
0511|    fn from_repr_vartime(repr: Self::Repr) -> Option<Self> {
0512|        let bytes_u64 = u64s_from_bytes(&repr);
0513|
0514|        if !is_valid(&bytes_u64) {
0515|            return None;
0516|        }
0517|        let mut out = blst_fr::default();
0518|        unsafe { blst_fr_from_uint64(&mut out, bytes_u64.as_ptr()) };
0519|        Some(Fq(out))
0520|    }
0521|
0522|    /// Converts a Montgomery form `Fq` into little-endian non-Montgomery from.
0523|    fn to_repr(&self) -> Self::Repr {
0524|        self.to_bytes_le()
0525|    }
```

Lines 736–749; original bytes [19864,20395); SHA-256 `319a7d017f573bbc27fd2dd9f8373dd5ebd1bdc011e6a91263542b2d7fe4cc4f`.

```rust
0736|impl Fq {
0737|    /// Attempts to convert a little-endian byte representation of
0738|    /// a scalar into a `Fq`, failing if the input is not canonical.
0739|    pub fn from_bytes_le(bytes: &[u8; 32]) -> CtOption<Fq> {
0740|        let is_some =
0741|            Choice::from(unsafe { blst_scalar_fr_check(&blst_scalar { b: *bytes }) as u8 });
0742|
0743|        let mut out = blst_fr::default();
0744|        let bytes_u64 = u64s_from_bytes(bytes);
0745|
0746|        unsafe { blst_fr_from_uint64(&mut out, bytes_u64.as_ptr()) };
0747|
0748|        CtOption::new(Fq(out), is_some)
0749|    }
```

Lines 759–766; original bytes [20705,21017); SHA-256 `192fcb17cc41d35a03f5958a1bd4ef59924d4aad03a3753e408764e2ba2d9ce3`.

```rust
0759|    /// Converts an element of `Fq` into a byte representation in
0760|    /// little-endian byte order.
0761|    #[inline]
0762|    pub fn to_bytes_le(&self) -> [u8; 32] {
0763|        let mut out = [0u64; 4];
0764|        unsafe { blst_uint64_from_fr(out.as_mut_ptr(), &self.0) };
0765|        out.as_byte_slice().try_into().unwrap()
0766|    }
```

Omitted original line ranges: 30–41, 49–172, 190–215, 223–262, 282–438, 466–500, 526–735, 750–758, 767–2106.

## exports.c — `blst-0.3.17/blst/src/exports.c`

Lines 1–620; original bytes [0,16631); SHA-256 `aa64e527fe4e3741863f7242232441e1e05d669ecc2da6096ac10e9666681b3f`.

```c
0001|/*
0002| * Copyright Supranational LLC
0003| * Licensed under the Apache License, Version 2.0, see LICENSE for details.
0004| * SPDX-License-Identifier: Apache-2.0
0005| */
0006|/*
0007| * Why this file? Overall goal is to ensure that all internal calls
0008| * remain internal after linking application. This is to both
0009| *
0010| * a) minimize possibility of external name conflicts (since all
0011| *    non-blst-prefixed and [assembly subroutines] remain static);
0012| * b) preclude possibility of unintentional internal reference
0013| *    overload in shared library context (one can achieve same
0014| *    effect with -Bsymbolic, but we don't want to rely on end-user
0015| *    to remember to use it);
0016| */
0017|
0018|#include "fields.h"
0019|#include "bytes.h"
0020|
0021|/*
0022| * BLS12-381-specific Fr shortcuts to assembly.
0023| */
0024|void blst_fr_add(vec256 ret, const vec256 a, const vec256 b)
0025|{   add_mod_256(ret, a, b, BLS12_381_r);   }
0026|
0027|void blst_fr_sub(vec256 ret, const vec256 a, const vec256 b)
0028|{   sub_mod_256(ret, a, b, BLS12_381_r);   }
0029|
0030|void blst_fr_mul_by_3(vec256 ret, const vec256 a)
0031|{   mul_by_3_mod_256(ret, a, BLS12_381_r);   }
0032|
0033|void blst_fr_lshift(vec256 ret, const vec256 a, size_t count)
0034|{   lshift_mod_256(ret, a, count, BLS12_381_r);   }
0035|
0036|void blst_fr_rshift(vec256 ret, const vec256 a, size_t count)
0037|{   rshift_mod_256(ret, a, count, BLS12_381_r);   }
0038|
0039|void blst_fr_mul(vec256 ret, const vec256 a, const vec256 b)
0040|{   mul_mont_sparse_256(ret, a, b, BLS12_381_r, r0);   }
0041|
0042|void blst_fr_ct_bfly(vec256 x0, vec256 x1, const vec256 twiddle)
0043|{
0044|    vec256 x2;
0045|
0046|    mul_mont_sparse_256(x2, x1, twiddle, BLS12_381_r, r0);
0047|    sub_mod_256(x1, x0, x2, BLS12_381_r);
0048|    add_mod_256(x0, x0, x2, BLS12_381_r);
0049|}
0050|
0051|void blst_fr_gs_bfly(vec256 x0, vec256 x1, const vec256 twiddle)
0052|{
0053|    vec256 x2;
0054|
0055|    sub_mod_256(x2, x0, x1, BLS12_381_r);
0056|    add_mod_256(x0, x0, x1, BLS12_381_r);
0057|    mul_mont_sparse_256(x1, x2, twiddle, BLS12_381_r, r0);
0058|}
0059|
0060|void blst_fr_sqr(vec256 ret, const vec256 a)
0061|{   sqr_mont_sparse_256(ret, a, BLS12_381_r, r0);   }
0062|
0063|void blst_fr_cneg(vec256 ret, const vec256 a, int flag)
0064|{   cneg_mod_256(ret, a, is_zero(flag) ^ 1, BLS12_381_r);   }
0065|
0066|void blst_fr_to(vec256 ret, const vec256 a)
0067|{   mul_mont_sparse_256(ret, BLS12_381_rRR, a, BLS12_381_r, r0);   }
0068|
0069|void blst_fr_from(vec256 ret, const vec256 a)
0070|{   from_mont_256(ret, a, BLS12_381_r, r0);   }
0071|
0072|void blst_fr_from_scalar(vec256 ret, const pow256 a)
0073|{
0074|    const union {
0075|        long one;
0076|        char little;
0077|    } is_endian = { 1 };
0078|
0079|    if ((uptr_t)ret == (uptr_t)a && is_endian.little) {
0080|        mul_mont_sparse_256(ret, (const limb_t *)a, BLS12_381_rRR,
0081|                                                    BLS12_381_r, r0);
0082|    } else {
0083|        vec256 out;
0084|        limbs_from_le_bytes(out, a, 32);
0085|        mul_mont_sparse_256(ret, BLS12_381_rRR, out, BLS12_381_r, r0);
0086|        vec_zero(out, sizeof(out));
0087|    }
0088|}
0089|
0090|void blst_scalar_from_fr(pow256 ret, const vec256 a)
0091|{
0092|    const union {
0093|        long one;
0094|        char little;
0095|    } is_endian = { 1 };
0096|
0097|    if ((uptr_t)ret == (uptr_t)a && is_endian.little) {
0098|        from_mont_256((limb_t *)ret, a, BLS12_381_r, r0);
0099|    } else {
0100|        vec256 out;
0101|        from_mont_256(out, a, BLS12_381_r, r0);
0102|        le_bytes_from_limbs(ret, out, 32);
0103|        vec_zero(out, sizeof(out));
0104|    }
0105|}
0106|
0107|int blst_scalar_fr_check(const pow256 a)
0108|{   return (int)(check_mod_256(a, BLS12_381_r) |
0109|                 bytes_are_zero(a, sizeof(pow256)));
0110|}
0111|
0112|int blst_sk_check(const pow256 a)
0113|{   return (int)check_mod_256(a, BLS12_381_r);   }
0114|
0115|int blst_sk_add_n_check(pow256 ret, const pow256 a, const pow256 b)
0116|{   return (int)add_n_check_mod_256(ret, a, b, BLS12_381_r);   }
0117|
0118|int blst_sk_sub_n_check(pow256 ret, const pow256 a, const pow256 b)
0119|{   return (int)sub_n_check_mod_256(ret, a, b, BLS12_381_r);   }
0120|
0121|int blst_sk_mul_n_check(pow256 ret, const pow256 a, const pow256 b)
0122|{
0123|    vec256 t[2];
0124|    const union {
0125|        long one;
0126|        char little;
0127|    } is_endian = { 1 };
0128|    bool_t is_zero;
0129|
0130|    if (((size_t)a|(size_t)b)%sizeof(limb_t) != 0 || !is_endian.little) {
0131|        limbs_from_le_bytes(t[0], a, sizeof(pow256));
0132|        limbs_from_le_bytes(t[1], b, sizeof(pow256));
0133|        a = (const byte *)t[0];
0134|        b = (const byte *)t[1];
0135|    }
0136|    mul_mont_sparse_256(t[0], BLS12_381_rRR, (const limb_t *)a, BLS12_381_r, r0);
0137|    mul_mont_sparse_256(t[0], t[0], (const limb_t *)b, BLS12_381_r, r0);
0138|    le_bytes_from_limbs(ret, t[0], sizeof(pow256));
0139|    is_zero = vec_is_zero(t[0], sizeof(vec256));
0140|    vec_zero(t, sizeof(t));
0141|
0142|    return (int)(is_zero^1);
0143|}
0144|
0145|void blst_sk_inverse(pow256 ret, const pow256 a)
0146|{
0147|    const union {
0148|        long one;
0149|        char little;
0150|    } is_endian = { 1 };
0151|
0152|    if (((size_t)a|(size_t)ret)%sizeof(limb_t) == 0 && is_endian.little) {
0153|        limb_t *out = (limb_t *)ret;
0154|        mul_mont_sparse_256(out, BLS12_381_rRR, (const limb_t *)a,
0155|                                 BLS12_381_r, r0);
0156|        reciprocal_fr(out, out);
0157|        from_mont_256(out, out, BLS12_381_r, r0);
0158|    } else {
0159|        vec256 out;
0160|        limbs_from_le_bytes(out, a, 32);
0161|        mul_mont_sparse_256(out, BLS12_381_rRR, out, BLS12_381_r, r0);
0162|        reciprocal_fr(out, out);
0163|        from_mont_256(out, out, BLS12_381_r, r0);
0164|        le_bytes_from_limbs(ret, out, 32);
0165|        vec_zero(out, sizeof(out));
0166|    }
0167|}
0168|
0169|/*
0170| * BLS12-381-specific Fp shortcuts to assembly.
0171| */
0172|void blst_fp_add(vec384 ret, const vec384 a, const vec384 b)
0173|{   add_fp(ret, a, b);   }
0174|
0175|void blst_fp_sub(vec384 ret, const vec384 a, const vec384 b)
0176|{   sub_fp(ret, a, b);   }
0177|
0178|void blst_fp_mul_by_3(vec384 ret, const vec384 a)
0179|{   mul_by_3_fp(ret, a);   }
0180|
0181|void blst_fp_mul_by_8(vec384 ret, const vec384 a)
0182|{   mul_by_8_fp(ret, a);   }
0183|
0184|void blst_fp_lshift(vec384 ret, const vec384 a, size_t count)
0185|{   lshift_fp(ret, a, count);   }
0186|
0187|void blst_fp_mul(vec384 ret, const vec384 a, const vec384 b)
0188|{   mul_fp(ret, a, b);   }
0189|
0190|void blst_fp_sqr(vec384 ret, const vec384 a)
0191|{   sqr_fp(ret, a);   }
0192|
0193|void blst_fp_cneg(vec384 ret, const vec384 a, int flag)
0194|{   cneg_fp(ret, a, is_zero(flag) ^ 1);   }
0195|
0196|void blst_fp_to(vec384 ret, const vec384 a)
0197|{   mul_fp(ret, a, BLS12_381_RR);   }
0198|
0199|void blst_fp_from(vec384 ret, const vec384 a)
0200|{   from_fp(ret, a);   }
0201|
0202|/*
0203| * Fp serialization/deserialization.
0204| */
0205|void blst_fp_from_uint32(vec384 ret, const unsigned int a[12])
0206|{
0207|    if (sizeof(limb_t) == 8) {
0208|        int i;
0209|        for (i = 0; i < 6; i++)
0210|            ret[i] = a[2*i] | ((limb_t)a[2*i+1] << (32 & (8*sizeof(limb_t)-1)));
0211|        a = (const unsigned int *)ret;
0212|    }
0213|    mul_fp(ret, (const limb_t *)a, BLS12_381_RR);
0214|}
0215|
0216|void blst_uint32_from_fp(unsigned int ret[12], const vec384 a)
0217|{
0218|    if (sizeof(limb_t) == 4) {
0219|        from_fp((limb_t *)ret, a);
0220|    } else {
0221|        vec384 out;
0222|        int i;
0223|
0224|        from_fp(out, a);
0225|        for (i = 0; i < 6; i++) {
0226|            limb_t limb = out[i];
0227|            ret[2*i]   = (unsigned int)limb;
0228|            ret[2*i+1] = (unsigned int)(limb >> (32 & (8*sizeof(limb_t)-1)));
0229|        }
0230|    }
0231|}
0232|
0233|void blst_fp_from_uint64(vec384 ret, const unsigned long long a[6])
0234|{
0235|    const union {
0236|        long one;
0237|        char little;
0238|    } is_endian = { 1 };
0239|
0240|    if (sizeof(limb_t) == 4 && !is_endian.little) {
0241|        int i;
0242|        for (i = 0; i < 6; i++) {
0243|            unsigned long long limb = a[i];
0244|            ret[2*i]   = (limb_t)limb;
0245|            ret[2*i+1] = (limb_t)(limb >> 32);
0246|        }
0247|        a = (const unsigned long long *)ret;
0248|    }
0249|    mul_fp(ret, (const limb_t *)a, BLS12_381_RR);
0250|}
0251|
0252|void blst_uint64_from_fp(unsigned long long ret[6], const vec384 a)
0253|{
0254|    const union {
0255|        long one;
0256|        char little;
0257|    } is_endian = { 1 };
0258|
0259|    if (sizeof(limb_t) == 8 || is_endian.little) {
0260|        from_fp((limb_t *)ret, a);
0261|    } else {
0262|        vec384 out;
0263|        int i;
0264|
0265|        from_fp(out, a);
0266|        for (i = 0; i < 6; i++)
0267|            ret[i] = out[2*i] | ((unsigned long long)out[2*i+1] << 32);
0268|    }
0269|}
0270|
0271|void blst_fp_from_bendian(vec384 ret, const unsigned char a[48])
0272|{
0273|    vec384 out;
0274|
0275|    limbs_from_be_bytes(out, a, sizeof(vec384));
0276|    mul_fp(ret, out, BLS12_381_RR);
0277|}
0278|
0279|void blst_bendian_from_fp(unsigned char ret[48], const vec384 a)
0280|{
0281|    vec384 out;
0282|
0283|    from_fp(out, a);
0284|    be_bytes_from_limbs(ret, out, sizeof(vec384));
0285|}
0286|
0287|void blst_fp_from_lendian(vec384 ret, const unsigned char a[48])
0288|{
0289|    vec384 out;
0290|
0291|    limbs_from_le_bytes(out, a, sizeof(vec384));
0292|    mul_fp(ret, out, BLS12_381_RR);
0293|}
0294|
0295|void blst_lendian_from_fp(unsigned char ret[48], const vec384 a)
0296|{
0297|    vec384 out;
0298|
0299|    from_fp(out, a);
0300|    le_bytes_from_limbs(ret, out, sizeof(vec384));
0301|}
0302|
0303|/*
0304| * BLS12-381-specific Fp2 shortcuts to assembly.
0305| */
0306|void blst_fp2_add(vec384x ret, const vec384x a, const vec384x b)
0307|{   add_fp2(ret, a, b);   }
0308|
0309|void blst_fp2_sub(vec384x ret, const vec384x a, const vec384x b)
0310|{   sub_fp2(ret, a, b);   }
0311|
0312|void blst_fp2_mul_by_3(vec384x ret, const vec384x a)
0313|{   mul_by_3_fp2(ret, a);   }
0314|
0315|void blst_fp2_mul_by_8(vec384x ret, const vec384x a)
0316|{   mul_by_8_fp2(ret, a);   }
0317|
0318|void blst_fp2_lshift(vec384x ret, const vec384x a, size_t count)
0319|{   lshift_fp2(ret, a, count);    }
0320|
0321|void blst_fp2_mul(vec384x ret, const vec384x a, const vec384x b)
0322|{   mul_fp2(ret, a, b);   }
0323|
0324|void blst_fp2_sqr(vec384x ret, const vec384x a)
0325|{   sqr_fp2(ret, a);   }
0326|
0327|void blst_fp2_cneg(vec384x ret, const vec384x a, int flag)
0328|{   cneg_fp2(ret, a, is_zero(flag) ^ 1);   }
0329|
0330|/*
0331| * Scalar serialization/deserialization.
0332| */
0333|void blst_scalar_from_uint32(pow256 ret, const unsigned int a[8])
0334|{
0335|    const union {
0336|        long one;
0337|        char little;
0338|    } is_endian = { 1 };
0339|    size_t i;
0340|
0341|    if ((uptr_t)ret==(uptr_t)a && is_endian.little)
0342|        return;
0343|
0344|    for(i = 0; i < 8; i++) {
0345|        unsigned int w = a[i];
0346|        *ret++ = (byte)w;
0347|        *ret++ = (byte)(w >> 8);
0348|        *ret++ = (byte)(w >> 16);
0349|        *ret++ = (byte)(w >> 24);
0350|    }
0351|}
0352|
0353|void blst_uint32_from_scalar(unsigned int ret[8], const pow256 a)
0354|{
0355|    const union {
0356|        long one;
0357|        char little;
0358|    } is_endian = { 1 };
0359|    size_t i;
0360|
0361|    if ((uptr_t)ret==(uptr_t)a && is_endian.little)
0362|        return;
0363|
0364|    for(i = 0; i < 8; i++) {
0365|        unsigned int w = (unsigned int)(*a++);
0366|        w |= (unsigned int)(*a++) << 8;
0367|        w |= (unsigned int)(*a++) << 16;
0368|        w |= (unsigned int)(*a++) << 24;
0369|        ret[i] = w;
0370|    }
0371|}
0372|
0373|void blst_scalar_from_uint64(pow256 ret, const unsigned long long a[4])
0374|{
0375|    const union {
0376|        long one;
0377|        char little;
0378|    } is_endian = { 1 };
0379|    size_t i;
0380|
0381|    if ((uptr_t)ret==(uptr_t)a && is_endian.little)
0382|        return;
0383|
0384|    for(i = 0; i < 4; i++) {
0385|        unsigned long long w = a[i];
0386|        *ret++ = (byte)w;
0387|        *ret++ = (byte)(w >> 8);
0388|        *ret++ = (byte)(w >> 16);
0389|        *ret++ = (byte)(w >> 24);
0390|        *ret++ = (byte)(w >> 32);
0391|        *ret++ = (byte)(w >> 40);
0392|        *ret++ = (byte)(w >> 48);
0393|        *ret++ = (byte)(w >> 56);
0394|    }
0395|}
0396|
0397|void blst_uint64_from_scalar(unsigned long long ret[4], const pow256 a)
0398|{
0399|    const union {
0400|        long one;
0401|        char little;
0402|    } is_endian = { 1 };
0403|    size_t i;
0404|
0405|    if ((uptr_t)ret==(uptr_t)a && is_endian.little)
0406|        return;
0407|
0408|    for(i = 0; i < 4; i++) {
0409|        unsigned long long w = (unsigned long long)(*a++);
0410|        w |= (unsigned long long)(*a++) << 8;
0411|        w |= (unsigned long long)(*a++) << 16;
0412|        w |= (unsigned long long)(*a++) << 24;
0413|        w |= (unsigned long long)(*a++) << 32;
0414|        w |= (unsigned long long)(*a++) << 40;
0415|        w |= (unsigned long long)(*a++) << 48;
0416|        w |= (unsigned long long)(*a++) << 56;
0417|        ret[i] = w;
0418|    }
0419|}
0420|
0421|void blst_scalar_from_bendian(pow256 ret, const unsigned char a[32])
0422|{
0423|    vec256 out;
0424|    limbs_from_be_bytes(out, a, sizeof(out));
0425|    le_bytes_from_limbs(ret, out, sizeof(out));
0426|    vec_zero(out, sizeof(out));
0427|}
0428|
0429|void blst_bendian_from_scalar(unsigned char ret[32], const pow256 a)
0430|{
0431|    vec256 out;
0432|    limbs_from_le_bytes(out, a, sizeof(out));
0433|    be_bytes_from_limbs(ret, out, sizeof(out));
0434|    vec_zero(out, sizeof(out));
0435|}
0436|
0437|void blst_scalar_from_lendian(pow256 ret, const unsigned char a[32])
0438|{
0439|    size_t i;
0440|
0441|    if ((uptr_t)ret==(uptr_t)a)
0442|        return;
0443|
0444|    for (i = 0; i < 32; i++)
0445|        ret[i] = a[i];
0446|}
0447|
0448|void blst_lendian_from_scalar(unsigned char ret[32], const pow256 a)
0449|{
0450|    size_t i;
0451|
0452|    if ((uptr_t)ret==(uptr_t)a)
0453|        return;
0454|
0455|    for (i = 0; i < 32; i++)
0456|        ret[i] = a[i];
0457|}
0458|
0459|void blst_fr_from_uint64(vec256 ret, const unsigned long long a[4])
0460|{
0461|    const union {
0462|        long one;
0463|        char little;
0464|    } is_endian = { 1 };
0465|
0466|    if (sizeof(limb_t) == 4 && !is_endian.little) {
0467|        int i;
0468|        for (i = 0; i < 4; i++) {
0469|            unsigned long long limb = a[i];
0470|            ret[2*i]   = (limb_t)limb;
0471|            ret[2*i+1] = (limb_t)(limb >> 32);
0472|        }
0473|        a = (const unsigned long long *)ret;
0474|    }
0475|    mul_mont_sparse_256(ret, BLS12_381_rRR, (const limb_t *)a, BLS12_381_r, r0);
0476|}
0477|
0478|void blst_uint64_from_fr(unsigned long long ret[4], const vec256 a)
0479|{
0480|    const union {
0481|        long one;
0482|        char little;
0483|    } is_endian = { 1 };
0484|
0485|    if (sizeof(limb_t) == 8 || is_endian.little) {
0486|        from_mont_256((limb_t *)ret, a, BLS12_381_r, r0);
0487|    } else {
0488|        vec256 out;
0489|        int i;
0490|
0491|        from_mont_256(out, a, BLS12_381_r, r0);
0492|        for (i = 0; i < 4; i++)
0493|            ret[i] = out[2*i] | ((unsigned long long)out[2*i+1] << 32);
0494|        vec_zero(out, sizeof(out));
0495|    }
0496|}
0497|
0498|int blst_scalar_from_le_bytes(pow256 out, const unsigned char *bytes, size_t n)
0499|{
0500|    size_t rem = n ? ((n - 1) % 32 + 1) : 0;
0501|    struct { vec256 out, digit; } t;
0502|    limb_t ret;
0503|
0504|    vec_zero(t.out, sizeof(t.out));
0505|
0506|    n -= rem;
0507|    limbs_from_le_bytes(t.out, bytes += n, rem);
0508|    mul_mont_sparse_256(t.out, BLS12_381_rRR, t.out, BLS12_381_r, r0);
0509|
0510|    while (n) {
0511|        limbs_from_le_bytes(t.digit, bytes -= 32, 32);
0512|        add_mod_256(t.out, t.out, t.digit, BLS12_381_r);
0513|        mul_mont_sparse_256(t.out, BLS12_381_rRR, t.out, BLS12_381_r, r0);
0514|        n -= 32;
0515|    }
0516|
0517|    from_mont_256(t.out, t.out, BLS12_381_r, r0);
0518|
0519|    ret = vec_is_zero(t.out, sizeof(t.out));
0520|    le_bytes_from_limbs(out, t.out, 32);
0521|    vec_zero(&t, sizeof(t));
0522|
0523|    return (int)(ret^1);
0524|}
0525|
0526|int blst_scalar_from_be_bytes(pow256 out, const unsigned char *bytes, size_t n)
0527|{
0528|    size_t rem = n ? ((n - 1) % 32 + 1) : 0;
0529|    struct { vec256 out, digit; } t;
0530|    limb_t ret;
0531|
0532|    vec_zero(t.out, sizeof(t.out));
0533|
0534|    limbs_from_be_bytes(t.out, bytes, rem);
0535|    mul_mont_sparse_256(t.out, BLS12_381_rRR, t.out, BLS12_381_r, r0);
0536|
0537|    while (n -= rem) {
0538|        limbs_from_be_bytes(t.digit, bytes += rem, 32);
0539|        add_mod_256(t.out, t.out, t.digit, BLS12_381_r);
0540|        mul_mont_sparse_256(t.out, BLS12_381_rRR, t.out, BLS12_381_r, r0);
0541|        rem = 32;
0542|    }
0543|
0544|    from_mont_256(t.out, t.out, BLS12_381_r, r0);
0545|
0546|    ret = vec_is_zero(t.out, sizeof(t.out));
0547|    le_bytes_from_limbs(out, t.out, 32);
0548|    vec_zero(&t, sizeof(t));
0549|
0550|    return (int)(ret^1);
0551|}
0552|
0553|void blst_fp_from_le_bytes(vec384 out, const unsigned char *bytes, size_t n)
0554|{
0555|    size_t rem = n ? ((n - 1) % 48 + 1) : 0;
0556|    vec384 digit;
0557|
0558|    vec_zero(out, sizeof(vec384));
0559|
0560|    n -= rem;
0561|    limbs_from_le_bytes(out, bytes += n, rem);
0562|    mul_mont_384(out, BLS12_381_RR, out, BLS12_381_P, p0);
0563|
0564|    while (n) {
0565|        limbs_from_le_bytes(digit, bytes -= 48, 48);
0566|        add_mod_384(out, out, digit, BLS12_381_P);
0567|        mul_mont_384(out, BLS12_381_RR, out, BLS12_381_P, p0);
0568|        n -= 48;
0569|    }
0570|}
0571|
0572|void blst_fp_from_be_bytes(vec384 out, const unsigned char *bytes, size_t n)
0573|{
0574|    size_t rem = n ? ((n - 1) % 48 + 1) : 0;
0575|    vec384 digit;
0576|
0577|    vec_zero(out, sizeof(vec384));
0578|
0579|    limbs_from_be_bytes(out, bytes, rem);
0580|    mul_mont_384(out, BLS12_381_RR, out, BLS12_381_P, p0);
0581|
0582|    while (n -= rem) {
0583|        limbs_from_be_bytes(digit, bytes += rem, 48);
0584|        add_mod_384(out, out, digit, BLS12_381_P);
0585|        mul_mont_384(out, BLS12_381_RR, out, BLS12_381_P, p0);
0586|        rem = 48;
0587|    }
0588|}
0589|
0590|/*
0591| * Single-short SHA-256 hash function.
0592| */
0593|#include "sha256.h"
0594|
0595|void blst_sha256(unsigned char md[32], const void *msg, size_t len)
0596|{
0597|    SHA256_CTX ctx;
0598|
0599|    sha256_init(&ctx);
0600|    sha256_update(&ctx, msg, len);
0601|    sha256_final(md, &ctx);
0602|}
0603|
0604|/*
0605| * Test facilitator.
0606| */
0607|void blst_scalar_from_hexascii(pow256 ret, const char *hex)
0608|{   bytes_from_hexascii(ret, sizeof(pow256), hex);   }
0609|
0610|void blst_fr_from_hexascii(vec256 ret, const char *hex)
0611|{
0612|    limbs_from_hexascii(ret, sizeof(vec256), hex);
0613|    mul_mont_sparse_256(ret, BLS12_381_rRR, ret, BLS12_381_r, r0);
0614|}
0615|
0616|void blst_fp_from_hexascii(vec384 ret, const char *hex)
0617|{
0618|    limbs_from_hexascii(ret, sizeof(vec384), hex);
0619|    mul_fp(ret, ret, BLS12_381_RR);
0620|}
```

Omitted original line ranges: none; complete file reproduced.

## vect.h — `blst-0.3.17/blst/src/vect.h`

Lines 1–429; original bytes [0,13749); SHA-256 `7aaeb0246108beabf3d315f435b0af08a62c6fce6c725ef70b060f9b70fe70e6`.

```c
0001|/*
0002| * Copyright Supranational LLC
0003| * Licensed under the Apache License, Version 2.0, see LICENSE for details.
0004| * SPDX-License-Identifier: Apache-2.0
0005| */
0006|#ifndef __BLS12_381_ASM_VECT_H__
0007|#define __BLS12_381_ASM_VECT_H__
0008|
0009|#include <stddef.h>
0010|
0011|#if defined(__x86_64__) || defined(__aarch64__)
0012|/* These are available even in ILP32 flavours, but even then they are
0013| * capable of performing 64-bit operations as efficiently as in *P64. */
0014|typedef unsigned long long limb_t;
0015|# define LIMB_T_BITS    64
0016|
0017|#elif defined(_WIN64)   /* Win64 is P64 */
0018|typedef unsigned __int64 limb_t;
0019|# define LIMB_T_BITS    64
0020|
0021|#elif defined(__BLST_NO_ASM__) || defined(__wasm64__)
0022|typedef unsigned int limb_t;
0023|# define LIMB_T_BITS    32
0024|# ifndef __BLST_NO_ASM__
0025|#  define __BLST_NO_ASM__
0026|# endif
0027|
0028|#else                   /* 32 bits on 32-bit platforms, 64 - on 64-bit */
0029|typedef unsigned long limb_t;
0030|#  ifdef _LP64
0031|#   define LIMB_T_BITS   64
0032|#  else
0033|#   define LIMB_T_BITS   32
0034|#   define __BLST_NO_ASM__
0035|#  endif
0036|#endif
0037|
0038|/*
0039| * Why isn't LIMB_T_BITS defined as 8*sizeof(limb_t)? Because pre-processor
0040| * knows nothing about sizeof(anything)...
0041| */
0042|#if LIMB_T_BITS == 64
0043|# define TO_LIMB_T(limb64)     limb64
0044|#else
0045|# define TO_LIMB_T(limb64)     (limb_t)limb64,(limb_t)(limb64>>32)
0046|#endif
0047|
0048|#define NLIMBS(bits)   (bits/LIMB_T_BITS)
0049|
0050|typedef limb_t vec256[NLIMBS(256)];
0051|typedef limb_t vec512[NLIMBS(512)];
0052|typedef limb_t vec384[NLIMBS(384)];
0053|typedef limb_t vec768[NLIMBS(768)];
0054|typedef vec384 vec384x[2];      /* 0 is "real" part, 1 is "imaginary" */
0055|
0056|typedef unsigned char byte;
0057|#define TO_BYTES(limb64)    (byte)limb64,(byte)(limb64>>8),\
0058|                            (byte)(limb64>>16),(byte)(limb64>>24),\
0059|                            (byte)(limb64>>32),(byte)(limb64>>40),\
0060|                            (byte)(limb64>>48),(byte)(limb64>>56)
0061|typedef byte pow256[256/8];
0062|
0063|/*
0064| * Internal Boolean type, Boolean by value, hence safe to cast to or
0065| * reinterpret as 'bool'.
0066| */
0067|typedef limb_t bool_t;
0068|
0069|/*
0070| * Assembly subroutines...
0071| */
0072|#if defined(__ADX__) /* e.g. -march=broadwell */ && !defined(__BLST_PORTABLE__)\
0073|                                                 && !defined(__BLST_NO_ASM__)
0074|# define mul_mont_sparse_256 mulx_mont_sparse_256
0075|# define sqr_mont_sparse_256 sqrx_mont_sparse_256
0076|# define from_mont_256 fromx_mont_256
0077|# define redc_mont_256 redcx_mont_256
0078|# define mul_mont_384 mulx_mont_384
0079|# define sqr_mont_384 sqrx_mont_384
0080|# define sqr_n_mul_mont_384 sqrx_n_mul_mont_384
0081|# define sqr_n_mul_mont_383 sqrx_n_mul_mont_383
0082|# define mul_384 mulx_384
0083|# define sqr_384 sqrx_384
0084|# define redc_mont_384 redcx_mont_384
0085|# define from_mont_384 fromx_mont_384
0086|# define sgn0_pty_mont_384 sgn0x_pty_mont_384
0087|# define sgn0_pty_mont_384x sgn0x_pty_mont_384x
0088|# define ct_inverse_mod_384 ctx_inverse_mod_384
0089|#endif
0090|
0091|void mul_mont_sparse_256(vec256 ret, const vec256 a, const vec256 b,
0092|                         const vec256 p, limb_t n0);
0093|void sqr_mont_sparse_256(vec256 ret, const vec256 a, const vec256 p, limb_t n0);
0094|void redc_mont_256(vec256 ret, const vec512 a, const vec256 p, limb_t n0);
0095|void from_mont_256(vec256 ret, const vec256 a, const vec256 p, limb_t n0);
0096|
0097|void add_mod_256(vec256 ret, const vec256 a, const vec256 b, const vec256 p);
0098|void sub_mod_256(vec256 ret, const vec256 a, const vec256 b, const vec256 p);
0099|void mul_by_3_mod_256(vec256 ret, const vec256 a, const vec256 p);
0100|void cneg_mod_256(vec256 ret, const vec256 a, bool_t flag, const vec256 p);
0101|void lshift_mod_256(vec256 ret, const vec256 a, size_t count, const vec256 p);
0102|void rshift_mod_256(vec256 ret, const vec256 a, size_t count, const vec256 p);
0103|bool_t eucl_inverse_mod_256(vec256 ret, const vec256 a, const vec256 p,
0104|                            const vec256 one);
0105|limb_t check_mod_256(const pow256 a, const vec256 p);
0106|limb_t add_n_check_mod_256(pow256 ret, const pow256 a, const pow256 b,
0107|                                       const vec256 p);
0108|limb_t sub_n_check_mod_256(pow256 ret, const pow256 a, const pow256 b,
0109|                                       const vec256 p);
0110|
0111|void vec_prefetch(const void *ptr, size_t len);
0112|
0113|void mul_mont_384(vec384 ret, const vec384 a, const vec384 b,
0114|                  const vec384 p, limb_t n0);
0115|void sqr_mont_384(vec384 ret, const vec384 a, const vec384 p, limb_t n0);
0116|void sqr_n_mul_mont_384(vec384 ret, const vec384 a, size_t count,
0117|                        const vec384 p, limb_t n0, const vec384 b);
0118|void sqr_n_mul_mont_383(vec384 ret, const vec384 a, size_t count,
0119|                        const vec384 p, limb_t n0, const vec384 b);
0120|
0121|void mul_384(vec768 ret, const vec384 a, const vec384 b);
0122|void sqr_384(vec768 ret, const vec384 a);
0123|void redc_mont_384(vec384 ret, const vec768 a, const vec384 p, limb_t n0);
0124|void from_mont_384(vec384 ret, const vec384 a, const vec384 p, limb_t n0);
0125|limb_t sgn0_pty_mont_384(const vec384 a, const vec384 p, limb_t n0);
0126|limb_t sgn0_pty_mont_384x(const vec384x a, const vec384 p, limb_t n0);
0127|limb_t sgn0_pty_mod_384(const vec384 a, const vec384 p);
0128|limb_t sgn0_pty_mod_384x(const vec384x a, const vec384 p);
0129|
0130|void add_mod_384(vec384 ret, const vec384 a, const vec384 b, const vec384 p);
0131|void sub_mod_384(vec384 ret, const vec384 a, const vec384 b, const vec384 p);
0132|void mul_by_8_mod_384(vec384 ret, const vec384 a, const vec384 p);
0133|void mul_by_3_mod_384(vec384 ret, const vec384 a, const vec384 p);
0134|void cneg_mod_384(vec384 ret, const vec384 a, bool_t flag, const vec384 p);
0135|void lshift_mod_384(vec384 ret, const vec384 a, size_t count, const vec384 p);
0136|void rshift_mod_384(vec384 ret, const vec384 a, size_t count, const vec384 p);
0137|void div_by_2_mod_384(vec384 ret, const vec384 a, const vec384 p);
0138|void ct_inverse_mod_384(vec768 ret, const vec384 inp, const vec384 mod,
0139|                                                      const vec384 modx);
0140|void ct_inverse_mod_256(vec512 ret, const vec256 inp, const vec256 mod,
0141|                                                      const vec256 modx);
0142|bool_t ct_is_square_mod_384(const vec384 inp, const vec384 mod);
0143|
0144|#if defined(__ADX__) /* e.g. -march=broadwell */ && !defined(__BLST_PORTABLE__)
0145|# define mul_mont_384x mulx_mont_384x
0146|# define sqr_mont_384x sqrx_mont_384x
0147|# define mul_382x mulx_382x
0148|# define sqr_382x sqrx_382x
0149|#endif
0150|
0151|void mul_mont_384x(vec384x ret, const vec384x a, const vec384x b,
0152|                   const vec384 p, limb_t n0);
0153|void sqr_mont_384x(vec384x ret, const vec384x a, const vec384 p, limb_t n0);
0154|void mul_382x(vec768 ret[2], const vec384x a, const vec384x b, const vec384 p);
0155|void sqr_382x(vec768 ret[2], const vec384x a, const vec384 p);
0156|
0157|void add_mod_384x(vec384x ret, const vec384x a, const vec384x b,
0158|                  const vec384 p);
0159|void sub_mod_384x(vec384x ret, const vec384x a, const vec384x b,
0160|                  const vec384 p);
0161|void mul_by_8_mod_384x(vec384x ret, const vec384x a, const vec384 p);
0162|void mul_by_3_mod_384x(vec384x ret, const vec384x a, const vec384 p);
0163|void mul_by_1_plus_i_mod_384x(vec384x ret, const vec384x a, const vec384 p);
0164|void add_mod_384x384(vec768 ret, const vec768 a, const vec768 b,
0165|                     const vec384 p);
0166|void sub_mod_384x384(vec768 ret, const vec768 a, const vec768 b,
0167|                     const vec384 p);
0168|
0169|/*
0170| * C subroutines
0171| */
0172|static void exp_mont_384(vec384 out, const vec384 inp, const byte *pow,
0173|                         size_t pow_bits, const vec384 p, limb_t n0);
0174|static void exp_mont_384x(vec384x out, const vec384x inp, const byte *pow,
0175|                          size_t pow_bits, const vec384 p, limb_t n0);
0176|static void div_by_zz(limb_t val[]);
0177|static void div_by_z(limb_t val[]);
0178|
0179|#ifdef __UINTPTR_TYPE__
0180|typedef __UINTPTR_TYPE__ uptr_t;
0181|#else
0182|typedef const void *uptr_t;
0183|#endif
0184|
0185|#if !defined(restrict)
0186|# if !defined(__STDC_VERSION__) || __STDC_VERSION__<199901
0187|#  if defined(__GNUC__) && __GNUC__>=2
0188|#   define restrict __restrict__
0189|#  elif defined(_MSC_VER)
0190|#   define restrict __restrict
0191|#  else
0192|#   define restrict
0193|#  endif
0194|# endif
0195|#endif
0196|
0197|#if !defined(inline) && !defined(__cplusplus)
0198|# if !defined(__STDC_VERSION__) || __STDC_VERSION__<199901
0199|#  if defined(__GNUC__) && __GNUC__>=2
0200|#   define inline __inline__
0201|#  elif defined(_MSC_VER)
0202|#   define inline __inline
0203|#  else
0204|#   define inline
0205|#  endif
0206|# endif
0207|#endif
0208|
0209|#if defined(__GNUC__) || defined(__clang__)
0210|# define launder(var) __asm__ __volatile__("" : "+r"(var))
0211|#else
0212|# define launder(var)
0213|#endif
0214|
0215|static inline bool_t is_bit_set(const byte *v, size_t i)
0216|{
0217|    bool_t ret = (v[i/8] >> (i%8)) & 1;
0218|    launder(ret);
0219|    return ret;
0220|}
0221|
0222|static inline bool_t byte_is_zero(unsigned char c)
0223|{
0224|    limb_t ret = ((limb_t)(c) - 1) >> (LIMB_T_BITS - 1);
0225|    launder(ret);
0226|    return ret;
0227|}
0228|
0229|static inline bool_t bytes_are_zero(const unsigned char *a, size_t num)
0230|{
0231|    unsigned char acc;
0232|    size_t i;
0233|
0234|    for (acc = 0, i = 0; i < num; i++)
0235|        acc |= a[i];
0236|
0237|    return byte_is_zero(acc);
0238|}
0239|
0240|static inline void vec_cswap(void *restrict a, void *restrict b, size_t num,
0241|                             bool_t cbit)
0242|{
0243|    limb_t ai, *ap = (limb_t *)a;
0244|    limb_t bi, *bp = (limb_t *)b;
0245|    limb_t xorm, mask;
0246|    size_t i;
0247|
0248|    launder(cbit);
0249|    mask = (limb_t)0 - cbit;
0250|
0251|    num /= sizeof(limb_t);
0252|
0253|    for (i = 0; i < num; i++) {
0254|        xorm = ((ai = ap[i]) ^ (bi = bp[i])) & mask;
0255|        ap[i] = ai ^ xorm;
0256|        bp[i] = bi ^ xorm;
0257|    }
0258|}
0259|
0260|/* ret = bit ? a : b */
0261|void vec_select_32(void *ret, const void *a, const void *b, bool_t sel_a);
0262|void vec_select_48(void *ret, const void *a, const void *b, bool_t sel_a);
0263|void vec_select_96(void *ret, const void *a, const void *b, bool_t sel_a);
0264|void vec_select_144(void *ret, const void *a, const void *b, bool_t sel_a);
0265|void vec_select_192(void *ret, const void *a, const void *b, bool_t sel_a);
0266|void vec_select_288(void *ret, const void *a, const void *b, bool_t sel_a);
0267|static inline void vec_select(void *ret, const void *a, const void *b,
0268|                              size_t num, bool_t sel_a)
0269|{
0270|    launder(sel_a);
0271|#ifndef __BLST_NO_ASM__
0272|    if (num == 32)          vec_select_32(ret, a, b, sel_a);
0273|    else if (num == 48)     vec_select_48(ret, a, b, sel_a);
0274|    else if (num == 96)     vec_select_96(ret, a, b, sel_a);
0275|    else if (num == 144)    vec_select_144(ret, a, b, sel_a);
0276|    else if (num == 192)    vec_select_192(ret, a, b, sel_a);
0277|    else if (num == 288)    vec_select_288(ret, a, b, sel_a);
0278|#else
0279|    if (0) ;
0280|#endif
0281|    else {
0282|        limb_t bi;
0283|        volatile limb_t *rp = (limb_t *)ret;
0284|        const limb_t *ap = (const limb_t *)a;
0285|        const limb_t *bp = (const limb_t *)b;
0286|        limb_t xorm, mask = (limb_t)0 - sel_a;
0287|        size_t i;
0288|
0289|        num /= sizeof(limb_t);
0290|
0291|        for (i = 0; i < num; i++) {
0292|            xorm = (ap[i] ^ (bi = bp[i])) & mask;
0293|            rp[i] = bi ^ xorm;
0294|        }
0295|    }
0296|}
0297|
0298|static inline bool_t is_zero(limb_t l)
0299|{
0300|    limb_t ret = (~l & (l - 1)) >> (LIMB_T_BITS - 1);
0301|    launder(ret);
0302|    return ret;
0303|}
0304|
0305|static inline bool_t vec_is_zero(const void *a, size_t num)
0306|{
0307|    const limb_t *ap = (const limb_t *)a;
0308|    limb_t acc;
0309|    size_t i;
0310|
0311|#ifndef __BLST_NO_ASM__
0312|    bool_t vec_is_zero_16x(const void *a, size_t num);
0313|    if ((num & 15) == 0)
0314|        return vec_is_zero_16x(a, num);
0315|#endif
0316|
0317|    num /= sizeof(limb_t);
0318|
0319|    for (acc = 0, i = 0; i < num; i++)
0320|        acc |= ap[i];
0321|
0322|    return is_zero(acc);
0323|}
0324|
0325|static inline bool_t vec_is_equal(const void *a, const void *b, size_t num)
0326|{
0327|    const limb_t *ap = (const limb_t *)a;
0328|    const limb_t *bp = (const limb_t *)b;
0329|    limb_t acc;
0330|    size_t i;
0331|
0332|#ifndef __BLST_NO_ASM__
0333|    bool_t vec_is_equal_16x(const void *a, const void *b, size_t num);
0334|    if ((num & 15) == 0)
0335|        return vec_is_equal_16x(a, b, num);
0336|#endif
0337|
0338|    num /= sizeof(limb_t);
0339|
0340|    for (acc = 0, i = 0; i < num; i++)
0341|        acc |= ap[i] ^ bp[i];
0342|
0343|    return is_zero(acc);
0344|}
0345|
0346|static inline void cneg_mod_384x(vec384x ret, const vec384x a, bool_t flag,
0347|                                 const vec384 p)
0348|{
0349|    cneg_mod_384(ret[0], a[0], flag, p);
0350|    cneg_mod_384(ret[1], a[1], flag, p);
0351|}
0352|
0353|static inline void vec_copy(void *restrict ret, const void *a, size_t num)
0354|{
0355|    limb_t *rp = (limb_t *)ret;
0356|    const limb_t *ap = (const limb_t *)a;
0357|    size_t i;
0358|
0359|    num /= sizeof(limb_t);
0360|
0361|    for (i = 0; i < num; i++)
0362|        rp[i] = ap[i];
0363|}
0364|
0365|static inline void vec_zero(void *ret, size_t num)
0366|{
0367|    volatile limb_t *rp = (volatile limb_t *)ret;
0368|    size_t i;
0369|
0370|    num /= sizeof(limb_t);
0371|
0372|    for (i = 0; i < num; i++)
0373|        rp[i] = 0;
0374|
0375|#if defined(__GNUC__) || defined(__clang__)
0376|    __asm__ __volatile__("" : : "r"(ret) : "memory");
0377|#endif
0378|}
0379|
0380|static inline void vec_czero(void *ret, size_t num, bool_t cbit)
0381|{
0382|    limb_t *rp = (limb_t *)ret;
0383|    size_t i;
0384|    limb_t mask;
0385|
0386|    launder(cbit);
0387|    mask = (limb_t)0 - (cbit^1);
0388|
0389|    num /= sizeof(limb_t);
0390|
0391|    for (i = 0; i < num; i++)
0392|        rp[i] &= mask;
0393|}
0394|
0395|/*
0396| * Some compilers get arguably overzealous(*) when passing pointer to
0397| * multi-dimensional array [such as vec384x] as 'const' argument.
0398| * General direction seems to be to legitimize such constification,
0399| * so it's argued that suppressing the warning is appropriate.
0400| *
0401| * (*)  http://www.open-std.org/jtc1/sc22/wg14/www/docs/n1923.htm
0402| */
0403|#if defined(__INTEL_COMPILER)
0404|# pragma warning(disable:167)
0405|# pragma warning(disable:556)
0406|#elif defined(__GNUC__) && !defined(__clang__) && (__STDC_VERSION__-0) < 202311
0407|# pragma GCC diagnostic ignored "-Wpedantic"
0408|#elif defined(_MSC_VER)
0409|# pragma warning(disable: 4127 4189)
0410|#endif
0411|
0412|#if !defined(__wasm__) && __STDC_HOSTED__-0 != 0
0413|# include <stdlib.h>
0414|#endif
0415|
0416|#if defined(__GNUC__)
0417|# ifndef alloca
0418|#  define alloca(s) __builtin_alloca(s)
0419|# endif
0420|#elif defined(__sun)
0421|# include <alloca.h>
0422|#elif defined(_WIN32)
0423|# include <malloc.h>
0424|# ifndef alloca
0425|#  define alloca(s) _alloca(s)
0426|# endif
0427|#endif
0428|
0429|#endif /* __BLS12_381_ASM_VECT_H__ */
```

Omitted original line ranges: none; complete file reproduced.

## bytes.h — `blst-0.3.17/blst/src/bytes.h`

Lines 1–152; original bytes [0,3862); SHA-256 `ab19306003f4033ed522238580836bc662334df2a9ed56e2f8bfa822e79e1ae9`.

```c
0001|/*
0002| * Copyright Supranational LLC
0003| * Licensed under the Apache License, Version 2.0, see LICENSE for details.
0004| * SPDX-License-Identifier: Apache-2.0
0005| */
0006|#ifndef __BLS12_381_ASM_BYTES_H__
0007|#define __BLS12_381_ASM_BYTES_H__
0008|
0009|static inline void bytes_zero(unsigned char *a, size_t num)
0010|{
0011|    size_t i;
0012|
0013|    for (i = 0; i < num; i++)
0014|        a[i] = 0;
0015|}
0016|
0017|static inline void limbs_from_be_bytes(limb_t *restrict ret,
0018|                                       const unsigned char *in, size_t n)
0019|{
0020|    limb_t limb = 0;
0021|
0022|    while(n--) {
0023|        limb <<= 8;
0024|        limb |= *in++;
0025|        /*
0026|         * 'if (n % sizeof(limb_t) == 0)' is omitted because it's cheaper
0027|         * to perform redundant stores than to pay penalty for
0028|         * mispredicted branch. Besides, some compilers unroll the
0029|         * loop and remove redundant stores to 'restrict'-ed storage...
0030|         */
0031|        ret[n / sizeof(limb_t)] = limb;
0032|    }
0033|}
0034|
0035|static inline void be_bytes_from_limbs(unsigned char *out, const limb_t *in,
0036|                                       size_t n)
0037|{
0038|    limb_t limb;
0039|
0040|    while(n--) {
0041|        limb = in[n / sizeof(limb_t)];
0042|        *out++ = (unsigned char)(limb >> (8 * (n % sizeof(limb_t))));
0043|    }
0044|}
0045|
0046|static inline void limbs_from_le_bytes(limb_t *restrict ret,
0047|                                       const unsigned char *in, size_t n)
0048|{
0049|    limb_t limb = 0;
0050|
0051|    while(n--) {
0052|        limb <<= 8;
0053|        limb |= in[n];
0054|        /*
0055|         * 'if (n % sizeof(limb_t) == 0)' is omitted because it's cheaper
0056|         * to perform redundant stores than to pay penalty for
0057|         * mispredicted branch. Besides, some compilers unroll the
0058|         * loop and remove redundant stores to 'restrict'-ed storage...
0059|         */
0060|        ret[n / sizeof(limb_t)] = limb;
0061|    }
0062|}
0063|
0064|static inline void le_bytes_from_limbs(unsigned char *out, const limb_t *in,
0065|                                       size_t n)
0066|{
0067|    const union {
0068|        long one;
0069|        char little;
0070|    } is_endian = { 1 };
0071|    limb_t limb;
0072|    size_t i, j, r;
0073|
0074|    if ((uptr_t)out == (uptr_t)in && is_endian.little)
0075|        return;
0076|
0077|    r = n % sizeof(limb_t);
0078|    n /= sizeof(limb_t);
0079|
0080|    for(i = 0; i < n; i++) {
0081|        for (limb = in[i], j = 0; j < sizeof(limb_t); j++, limb >>= 8)
0082|            *out++ = (unsigned char)limb;
0083|    }
0084|    if (r) {
0085|        for (limb = in[i], j = 0; j < r; j++, limb >>= 8)
0086|            *out++ = (unsigned char)limb;
0087|    }
0088|}
0089|
0090|static inline char hex_from_nibble(unsigned char nibble)
0091|{
0092|    int mask = (9 - (nibble &= 0xf)) >> 31;
0093|    return (char)(nibble + ((('a'-10) & mask) | ('0' & ~mask)));
0094|}
0095|
0096|static unsigned char nibble_from_hex(char c)
0097|{
0098|    int mask, ret;
0099|
0100|    mask = (('a'-c-1) & (c-1-'f')) >> 31;
0101|    ret  = (10 + c - 'a') & mask;
0102|    mask = (('A'-c-1) & (c-1-'F')) >> 31;
0103|    ret |= (10 + c - 'A') & mask;
0104|    mask = (('0'-c-1) & (c-1-'9')) >> 31;
0105|    ret |= (c - '0') & mask;
0106|    mask = ((ret-1) & ~mask) >> 31;
0107|    ret |= 16 & mask;
0108|
0109|    return (unsigned char)ret;
0110|}
0111|
0112|static void bytes_from_hexascii(unsigned char *ret, size_t sz, const char *hex)
0113|{
0114|    size_t len;
0115|    unsigned char b = 0;
0116|
0117|    if (hex[0]=='0' && (hex[1]=='x' || hex[1]=='X'))
0118|        hex += 2;
0119|
0120|    for (len = 0; len<2*sz && nibble_from_hex(hex[len])<16; len++) ;
0121|
0122|    bytes_zero(ret, sz);
0123|
0124|    while(len--) {
0125|        b <<= 4;
0126|        b |= nibble_from_hex(*hex++);
0127|        if (len % 2 == 0)
0128|            ret[len / 2] = b;
0129|    }
0130|}
0131|
0132|static void limbs_from_hexascii(limb_t *ret, size_t sz, const char *hex)
0133|{
0134|    size_t len;
0135|    limb_t limb = 0;
0136|
0137|    if (hex[0]=='0' && (hex[1]=='x' || hex[1]=='X'))
0138|        hex += 2;
0139|
0140|    for (len = 0; len<2*sz && nibble_from_hex(hex[len])<16; len++) ;
0141|
0142|    vec_zero(ret, sz);
0143|
0144|    while(len--) {
0145|        limb <<= 4;
0146|        limb |= nibble_from_hex(*hex++);
0147|        if (len % (2*sizeof(limb_t)) == 0)
0148|            ret[len / (2*sizeof(limb_t))] = limb;
0149|    }
0150|}
0151|
0152|#endif
```

Omitted original line ranges: none; complete file reproduced.

## consts.h — `blst-0.3.17/blst/src/consts.h`

Lines 1–30; original bytes [0,1152); SHA-256 `e983c6c80045adc46379ff1fd088dbf85e20c4946eb98191835eed91f7598727`.

```c
0001|/*
0002| * Copyright Supranational LLC
0003| * Licensed under the Apache License, Version 2.0, see LICENSE for details.
0004| * SPDX-License-Identifier: Apache-2.0
0005| */
0006|#ifndef __BLS12_381_ASM_CONST_H__
0007|#define __BLS12_381_ASM_CONST_H__
0008|#include "vect.h"
0009|
0010|extern const vec384 BLS12_381_P;
0011|extern const limb_t BLS12_381_p0;
0012|static const limb_t p0 = (limb_t)0x89f3fffcfffcfffd;  /* -1/P */
0013|typedef union { vec384 p12[12]; vec384x p2; vec384 p; } radix384;
0014|extern const radix384 BLS12_381_Rx; /* (1<<384)%P, "radix", one-in-Montgomery */
0015|extern const vec384 BLS12_381_RR;   /* (1<<768)%P, "radix"^2, to-Montgomery   */
0016|
0017|#define ONE_MONT_P TO_LIMB_T(0x760900000002fffd), \
0018|                   TO_LIMB_T(0xebf4000bc40c0002), \
0019|                   TO_LIMB_T(0x5f48985753c758ba), \
0020|                   TO_LIMB_T(0x77ce585370525745), \
0021|                   TO_LIMB_T(0x5c071a97a256ec6d), \
0022|                   TO_LIMB_T(0x15f65ec3fa80e493)
0023|
0024|#define ZERO_384 (BLS12_381_Rx.p2[1])
0025|
0026|extern const vec256 BLS12_381_r;    /* order */
0027|static const limb_t r0 = (limb_t)0xfffffffeffffffff;  /* -1/r */
0028|extern const vec256 BLS12_381_rRR;  /* (1<<512)%r, "radix"^2, to-Montgomery   */
0029|
0030|#endif
```

Omitted original line ranges: none; complete file reproduced.

