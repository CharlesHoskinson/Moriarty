# Compiler-admission helper sources: primary index and literal excerpts

Finite source-only packet for the designated owner. No execution/admission/effects/safety/build/resource conclusions. Complete files are separate under full-originals; excerpts below preserve exact recorded source lines.

Requested 22 files; copied 22; source bytes 119,126; missing requested files 0.

Provenance classes:16 AWS files are current-cache-only public reads; five SDK files match frozen Git blobs at 04c9c5d9bcebb8d4427d8589fb54d58a55599c14; ff source matches its prior checksum-verified archive/file pin. No current-cache archive verification is invented.

| Complete original | Bytes | Provenance class |
|---|---:|---|
| [aws-lc-sys-0.45.0/builder/cc_builder/apple_aarch64.rs](full-originals/aws-lc-sys-0.45.0/builder/cc_builder/apple_aarch64.rs) | 9,823 | current public cache only |
| [aws-lc-sys-0.45.0/builder/cc_builder/apple_x86_64.rs](full-originals/aws-lc-sys-0.45.0/builder/cc_builder/apple_x86_64.rs) | 10,194 | current public cache only |
| [aws-lc-sys-0.45.0/builder/cc_builder/linux_aarch64.rs](full-originals/aws-lc-sys-0.45.0/builder/cc_builder/linux_aarch64.rs) | 9,861 | current public cache only |
| [aws-lc-sys-0.45.0/builder/cc_builder/linux_arm.rs](full-originals/aws-lc-sys-0.45.0/builder/cc_builder/linux_arm.rs) | 935 | current public cache only |
| [aws-lc-sys-0.45.0/builder/cc_builder/linux_ppc64le.rs](full-originals/aws-lc-sys-0.45.0/builder/cc_builder/linux_ppc64le.rs) | 394 | current public cache only |
| [aws-lc-sys-0.45.0/builder/cc_builder/linux_x86.rs](full-originals/aws-lc-sys-0.45.0/builder/cc_builder/linux_x86.rs) | 985 | current public cache only |
| [aws-lc-sys-0.45.0/builder/cc_builder/linux_x86_64.rs](full-originals/aws-lc-sys-0.45.0/builder/cc_builder/linux_x86_64.rs) | 10,283 | current public cache only |
| [aws-lc-sys-0.45.0/builder/cc_builder/universal.rs](full-originals/aws-lc-sys-0.45.0/builder/cc_builder/universal.rs) | 8,549 | current public cache only |
| [aws-lc-sys-0.45.0/builder/cc_builder/win_aarch64.rs](full-originals/aws-lc-sys-0.45.0/builder/cc_builder/win_aarch64.rs) | 1,630 | current public cache only |
| [aws-lc-sys-0.45.0/builder/cc_builder/win_x86.rs](full-originals/aws-lc-sys-0.45.0/builder/cc_builder/win_x86.rs) | 985 | current public cache only |
| [aws-lc-sys-0.45.0/builder/cc_builder/win_x86_64.rs](full-originals/aws-lc-sys-0.45.0/builder/cc_builder/win_x86_64.rs) | 2,006 | current public cache only |
| [aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/c11.c](full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/c11.c) | 1,039 | current public cache only |
| [aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/stdalign_check.c](full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/stdalign_check.c) | 624 | current public cache only |
| [aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/builtin_swap_check.c](full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/builtin_swap_check.c) | 594 | current public cache only |
| [aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/linux_random_h.c](full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/linux_random_h.c) | 686 | current public cache only |
| [aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/memcmp_invalid_stripped_check.c](full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/memcmp_invalid_stripped_check.c) | 649 | current public cache only |
| [pinned-sdk/transient-crypto/src/macros.rs](full-originals/pinned-sdk/transient-crypto/src/macros.rs) | 4,323 | frozen Git blob verified |
| [pinned-sdk/serialize/src/lib.rs](full-originals/pinned-sdk/serialize/src/lib.rs) | 1,313 | frozen Git blob verified |
| [pinned-sdk/serialize/src/serializable.rs](full-originals/pinned-sdk/serialize/src/serializable.rs) | 6,334 | frozen Git blob verified |
| [pinned-sdk/serialize/src/deserializable.rs](full-originals/pinned-sdk/serialize/src/deserializable.rs) | 8,850 | frozen Git blob verified |
| [pinned-sdk/serialize/src/util.rs](full-originals/pinned-sdk/serialize/src/util.rs) | 21,548 | frozen Git blob verified |
| [ff-0.13.1/src/lib.rs](full-originals/ff-0.13.1/src/lib.rs) | 17,521 | frozen verified registry archive |

No literal serde identifier was located in the four copied serialize files; this bounded search result is recorded in SOURCE-SEARCH-FACTS.json. No implementation or conversion is inferred beyond the copied source.

Root-provided context states cc15.2.0/Linuxx86_64 in compiler-toolchain-metadata. This lane did not query/read/execute that toolchain and draws no build-choice conclusion from it.

## PrimeField trait declaration

[full-originals/ff-0.13.1/src/lib.rs](full-originals/ff-0.13.1/src/lib.rs), source lines 195–195; exact excerpt SHA256 `efd6735658ab63e2463c39d6fab9d7ae5062de5b92a2d5105cae638657532f52`.

```rust
pub trait PrimeField: Field + From<u64> {
```

## PrimeField::from_u128 default source

[full-originals/ff-0.13.1/src/lib.rs](full-originals/ff-0.13.1/src/lib.rs), source lines 254–262; exact excerpt SHA256 `e8c6299d1ae6c93fd15f94a6e52d9333810d0abba51a325b42b15265438e6ea0`.

```rust
    fn from_u128(v: u128) -> Self {
        let lower = v as u64;
        let upper = (v >> 64) as u64;
        let mut tmp = Self::from(upper);
        for _ in 0..64 {
            tmp = tmp.double();
        }
        tmp + Self::from(lower)
    }
```

## Frozen transient-crypto macro definitions

[full-originals/pinned-sdk/transient-crypto/src/macros.rs](full-originals/pinned-sdk/transient-crypto/src/macros.rs), source lines 14–136; exact excerpt SHA256 `d27110c1c4bd275ba159a6f319e2ad57439c862466f17bbf0ef151eed9cd46eb`.

```rust
macro_rules! wrap_field_arith {
    ($wrapper_name:ident) => {
        impl std::ops::Add for $wrapper_name {
            type Output = $wrapper_name;

            fn add(self, other: $wrapper_name) -> $wrapper_name {
                $wrapper_name(self.0 + other.0)
            }
        }

        impl std::ops::Sub for $wrapper_name {
            type Output = $wrapper_name;

            fn sub(self, other: $wrapper_name) -> $wrapper_name {
                $wrapper_name(self.0 - other.0)
            }
        }

        impl std::ops::Mul for $wrapper_name {
            type Output = $wrapper_name;

            fn mul(self, other: $wrapper_name) -> $wrapper_name {
                $wrapper_name(self.0 * other.0)
            }
        }

        impl std::ops::Neg for $wrapper_name {
            type Output = $wrapper_name;

            fn neg(self) -> $wrapper_name {
                $wrapper_name(-self.0)
            }
        }

        impl std::ops::Div for $wrapper_name {
            type Output = $wrapper_name;

            fn div(self, rhs: $wrapper_name) -> $wrapper_name {
                self.mul($wrapper_name(rhs.0.invert().unwrap()))
            }
        }
    };
}

macro_rules! wrap_group_arith {
    ($wrapper_name:ident, $scalar_wrapper:ident) => {
        impl std::ops::Add for $wrapper_name {
            type Output = $wrapper_name;

            fn add(self, other: $wrapper_name) -> $wrapper_name {
                $wrapper_name((self.0 + other.0).into())
            }
        }

        impl std::ops::Sub for $wrapper_name {
            type Output = $wrapper_name;

            fn sub(self, other: $wrapper_name) -> $wrapper_name {
                $wrapper_name((self.0 - other.0).into())
            }
        }

        impl std::ops::Mul<$scalar_wrapper> for $wrapper_name {
            type Output = $wrapper_name;

            fn mul(self, other: $scalar_wrapper) -> $wrapper_name {
                $wrapper_name(self.0.mul(other.0).into())
            }
        }

        impl std::ops::Neg for $wrapper_name {
            type Output = $wrapper_name;

            fn neg(self) -> $wrapper_name {
                $wrapper_name(-self.0)
            }
        }
    };
}

macro_rules! fr_display {
    ($wrapper_name:ident) => {
        impl std::fmt::Display for $wrapper_name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                let mut repr = self.as_le_bytes();
                while let Some(0) = repr.last() {
                    repr.pop();
                }
                if repr.is_empty() {
                    write!(formatter, "-")?;
                } else {
                    for byte in repr {
                        write!(formatter, "{byte:02x}")?;
                    }
                }
                Ok(())
            }
        }

        impl std::fmt::Debug for $wrapper_name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Display::fmt(&self, formatter)
            }
        }
    };
}

macro_rules! wrap_display {
    ($wrapper_name:ident) => {
        impl std::fmt::Display for $wrapper_name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.0, formatter)
            }
        }
        impl std::fmt::Debug for $wrapper_name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                std::fmt::Debug::fmt(&self.0, formatter)
            }
        }
    };
}

pub(crate) use {fr_display, wrap_display, wrap_field_arith, wrap_group_arith};
```

## Serializable definition

[full-originals/pinned-sdk/serialize/src/serializable.rs](full-originals/pinned-sdk/serialize/src/serializable.rs), source lines 42–45; exact excerpt SHA256 `b643d3c0f245f13e615668414f2c3b2f349f5340af8e4460f9d75084ac9dffcf`.

```rust
pub trait Serializable {
    fn serialize(&self, writer: &mut impl Write) -> std::io::Result<()>;
    fn serialized_size(&self) -> usize;
}
```

## Deserializable definition

[full-originals/pinned-sdk/serialize/src/deserializable.rs](full-originals/pinned-sdk/serialize/src/deserializable.rs), source lines 134–154; exact excerpt SHA256 `7f4e409868dbf967adb92470d24ce479e1c225ed27e5471cfb895fa4b7258a39`.

```rust
pub trait Deserializable
where
    Self: Sized,
{
    const LIMIT_RECURSION: bool = true;

    fn deserialize(reader: &mut impl Read, recursion_depth: u32) -> std::io::Result<Self>;

    fn check_rec(depth: &mut u32) -> std::io::Result<()> {
        if Self::LIMIT_RECURSION {
            *depth += 1;
            if *depth > RECURSION_LIMIT {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "exceeded recursion depth deserializing",
                ));
            }
        }
        Ok(())
    }
}
```

## Immediate fixed-width conversion macro

[full-originals/pinned-sdk/serialize/src/util.rs](full-originals/pinned-sdk/serialize/src/util.rs), source lines 118–145; exact excerpt SHA256 `4187beeca19ab1cc8412b880a22991ebf6aef454ac1b982f882d01bcf8c5306a`.

```rust
macro_rules! via_le_bytes {
    ($ty:ty, $len:expr_2021) => {
        impl Serializable for $ty {
            fn serialize(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
                writer.write_all(&self.to_le_bytes()[..])
            }
            fn serialized_size(&self) -> usize {
                $len
            }
        }

        impl Deserializable for $ty {
            fn deserialize(reader: &mut impl Read, _recursion_depth: u32) -> std::io::Result<Self> {
                let mut buf = [0u8; $len];
                reader.read_exact(&mut buf[..])?;
                Ok(<$ty>::from_le_bytes(buf))
            }
        }

        impl Tagged for $ty {
            fn tag() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(stringify!($ty))
            }
            fn tag_unique_factor() -> String {
                stringify!($ty).into()
            }
        }
    };
```

## Immediate SCALE conversion macro, invocations, ScaleBigInt definition/default and Serializable/Deserializable implementations

[full-originals/pinned-sdk/serialize/src/util.rs](full-originals/pinned-sdk/serialize/src/util.rs), source lines 148–312; exact excerpt SHA256 `4e632eba4880109c0c80e632561d4ac60882fb4d06eb013c5e4a8cfa44469f77`.

```rust
macro_rules! via_scale {
    ($ty:ty, $n:expr_2021) => {
        impl Serializable for $ty {
            fn serialize(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
                ScaleBigInt::from(*self).serialize(writer)
            }
            fn serialized_size(&self) -> usize {
                ScaleBigInt::from(*self).serialized_size()
            }
        }

        impl Deserializable for $ty {
            fn deserialize(reader: &mut impl Read, recursion_depth: u32) -> std::io::Result<Self> {
                <$ty>::try_from(ScaleBigInt::deserialize(reader, recursion_depth)?)
            }
        }

        impl From<$ty> for ScaleBigInt {
            fn from(val: $ty) -> ScaleBigInt {
                let mut res = ScaleBigInt([0u8; SCALE_MAX_BYTES]);
                let le_bytes = val.to_le_bytes();
                res.0[..$n].copy_from_slice(&le_bytes[..]);
                res
            }
        }

        impl TryFrom<ScaleBigInt> for $ty {
            type Error = std::io::Error;
            fn try_from(val: ScaleBigInt) -> std::io::Result<$ty> {
                if val.0[$n..].iter().any(|b| *b != 0) {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        concat!("out of range for ", stringify!($ty)),
                    ));
                }
                Ok(<$ty>::from_le_bytes(
                    val.0[..$n]
                        .try_into()
                        .expect("slice of known size must coerce to array"),
                ))
            }
        }

        impl Tagged for $ty {
            fn tag() -> std::borrow::Cow<'static, str> {
                std::borrow::Cow::Borrowed(stringify!($ty))
            }
            fn tag_unique_factor() -> String {
                stringify!($ty).into()
            }
        }
    };
}

via_le_bytes!(u8, 1);
via_le_bytes!(u16, 2);
via_le_bytes!(i8, 1);
via_le_bytes!(i16, 2);
via_le_bytes!(i32, 4);
via_le_bytes!(i64, 8);
via_le_bytes!(i128, 16);
via_scale!(u32, 4);
via_scale!(u64, 8);
via_scale!(u128, 16);

const SCALE_MAX_BYTES: usize = 67;
pub struct ScaleBigInt(pub [u8; SCALE_MAX_BYTES]);

impl Default for ScaleBigInt {
    fn default() -> Self {
        ScaleBigInt([0u8; SCALE_MAX_BYTES])
    }
}

const SCALE_ONE_BYTE_MARKER: u8 = 0b00;
const SCALE_TWO_BYTE_MARKER: u8 = 0b01;
const SCALE_FOUR_BYTE_MARKER: u8 = 0b10;
const SCALE_N_BYTE_MARKER: u8 = 0b11;

impl Serializable for ScaleBigInt {
    fn serialize(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        let top2bits = |b| (b & 0b1100_0000) >> 6;
        let bot6bits = |b| (b & 0b0011_1111) << 2;
        match self.serialized_size() {
            1 => writer.write_all(&[bot6bits(self.0[0]) | SCALE_ONE_BYTE_MARKER]),
            2 => {
                let b0 = bot6bits(self.0[0]) | SCALE_TWO_BYTE_MARKER;
                let b1 = top2bits(self.0[0]) | bot6bits(self.0[1]);
                writer.write_all(&[b0, b1])
            }
            4 => {
                let b0 = bot6bits(self.0[0]) | SCALE_FOUR_BYTE_MARKER;
                let b1 = top2bits(self.0[0]) | bot6bits(self.0[1]);
                let b2 = top2bits(self.0[1]) | bot6bits(self.0[2]);
                let b3 = top2bits(self.0[2]) | bot6bits(self.0[3]);
                writer.write_all(&[b0, b1, b2, b3])
            }
            n => {
                writer.write_all(&[(n as u8 - 5) << 2 | SCALE_N_BYTE_MARKER])?;
                writer.write_all(&self.0[..n - 1])
            }
        }
    }
    fn serialized_size(&self) -> usize {
        let trailing_zeros = self.0.iter().rev().take_while(|x| **x == 0).count();
        let occupied = SCALE_MAX_BYTES - trailing_zeros;
        let can_squeeze = self.0[occupied.saturating_sub(1)] < 64;
        match (occupied, can_squeeze) {
            (0, _) | (1, true) => 1,
            (1, false) | (2, true) => 2,
            (2, false) | (3, _) | (4, true) => 4,
            (n, _) => n + 1,
        }
    }
}

impl Deserializable for ScaleBigInt {
    fn deserialize(reader: &mut impl Read, recursion_depth: u32) -> std::io::Result<Self> {
        let first = u8::deserialize(reader, recursion_depth)?;
        let mut res = ScaleBigInt([0u8; SCALE_MAX_BYTES]);
        let top6bits = |b| (b & 0b1111_1100) >> 2;
        let bot2bits = |b| (b & 0b0000_0011) << 6;
        match first & 0b11 {
            SCALE_ONE_BYTE_MARKER => res.0[0] = top6bits(first),
            SCALE_TWO_BYTE_MARKER => {
                let second = u8::deserialize(reader, recursion_depth)?;
                if second == 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "non-canonical scale encoding",
                    ));
                }
                res.0[0] = top6bits(first) | bot2bits(second);
                res.0[1] = top6bits(second);
            }
            SCALE_FOUR_BYTE_MARKER => {
                let second = u8::deserialize(reader, recursion_depth)?;
                let third = u8::deserialize(reader, recursion_depth)?;
                let fourth = u8::deserialize(reader, recursion_depth)?;
                if third == 0 && fourth == 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "non-canonical scale encoding",
                    ));
                }
                res.0[0] = top6bits(first) | bot2bits(second);
                res.0[1] = top6bits(second) | bot2bits(third);
                res.0[2] = top6bits(third) | bot2bits(fourth);
                res.0[3] = top6bits(fourth);
            }
            SCALE_N_BYTE_MARKER => {
                let n = top6bits(first) as usize + 4;
                reader.read_exact(&mut res.0[..n])?;
                if res.0[n - 1] == 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "non-canonical scale encoding",
                    ));
                }
            }
            _ => unreachable!(),
        }
        Ok(res)
    }
}
```

## Named C source: c11

[full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/c11.c](full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/c11.c), source lines 1–24; exact excerpt SHA256 `766c186a0e2d00a441d03618ae997ee87f3b8a4ac2e0e2b0e9ca503b3db040d4`.

```c
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Simple program that should also be able to compiler, udeful to test different
// compiler flags

#include <stdlib.h>
#include <stdatomic.h>

// Some platforms define ATOMIC_LONG_LOCK_FREE as an expression like:
//  (__atomic_always_lock_free (sizeof (atomic_long), (void *) 0) ? 2 :
//     (__atomic_is_lock_free (sizeof (atomic_long), (void *) 0) ? 1 : 0))
// which relies on __atomic_is_lock_free which is a runtime function and not a
// compile time constant. This means ATOMIC_LONG_LOCK_FREE can not be used in a
// preprocessor check which AWS-LC needs to do to swap implementations of
// CRYPTO_refcount_inc at compile time. If that is the case the following #if
// won't compile, the following message is helpful to debug any future issues.
#if ATOMIC_LONG_LOCK_FREE < 0
#error "Should not get here, the above line should be false or invalid"
#endif

int main(int argc, char **argv) {
    return EXIT_SUCCESS;
}
```

## Named C source: stdalign_check

[full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/stdalign_check.c](full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/stdalign_check.c), source lines 1–24; exact excerpt SHA256 `991b0a38b488aae5d0b48a7438235320322c0ad22341bba187505914436a2dc6`.

```c
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Old versions of GCC don't support __has_include which could be used to check
// for stdalign.h, try to compile this instead.

#include <stdalign.h>
#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    alignas(8) uint8_t test[16];
    size_t alignment = alignof(uint8_t);

    test[0] = 0;

    // Try to eliminate dead store optimisation and similar
    if (alignment == 1000 && test[0] != 0) {
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
```

## Named C source: builtin_swap_check

[full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/builtin_swap_check.c](full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/builtin_swap_check.c), source lines 1–22; exact excerpt SHA256 `220b1a32fe0d914c3588e2103e993cfe7d7cc7f5513f6700133268ccdc1c81b7`.

```c
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// Builtin swap was added in GCC 4.8 https://gcc.gnu.org/gcc-4.8/changes.html.
// Recent versions of Clang pretend to be 4.2 but they do support the builtin
// swap functions.

#include <stdint.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    uint16_t test16 = 0;
    test16 = __builtin_bswap16(test16);

    uint32_t test32 = 0;
    test32 = __builtin_bswap32(test32);

    uint64_t test64 = 0;
    test64 = __builtin_bswap64(test64);

    return EXIT_SUCCESS;
}
```

## Named C source: linux_random_h

[full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/linux_random_h.c](full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/linux_random_h.c), source lines 1–20; exact excerpt SHA256 `558933a1d501a199bfd3df329035dae7937390083b3625e91cdcedcc94bc5dcd`.

```c
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

// This file checks if <linux/random.h> can be included. Currently, we assume
// that the compiler error is caused by '__u32' not being defined.
// Theory:
// crypto/fipsmodule/rand/urandom.c includes the <linux/random.h> Linux header.
// Some old Linux OS does not define '__u32' causing the following error:
// /usr/include/linux/random.h:38:2: error: unknown type name '__u32'
// __u32 buf[0];
// ^
#if defined(DEFINE_U32)
typedef unsigned int __u32;
#endif
#include <linux/random.h>
#include <stdlib.h>

int main(int argc, char **argv) {
    return EXIT_SUCCESS;
}
```

## Named C source: memcmp_invalid_stripped_check

[full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/memcmp_invalid_stripped_check.c](full-originals/aws-lc-sys-0.45.0/aws-lc/tests/compiler_features_tests/memcmp_invalid_stripped_check.c), source lines 1–16; exact excerpt SHA256 `2c0d8a2c0e2a5a0f42aedd848908441fb5f8a2f61e96f12d8c8e3fa09fa45488`.

```c
// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR ISC

#include <stdio.h>
#include <string.h>

int main(int argc, char **argv) {
  // A bug of 'memcmp' is reported in gcc (9.2, 9.3, 10.1). See https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189
  // AWS-LC warns the build when detecting the unexpected 'memcmp' behavior.
  // Below test case is equivalent to https://gcc.gnu.org/bugzilla/show_bug.cgi?id=95189#c7
  char a[] = "\0abc";
  int res = memcmp(a, "\0\0\0\0", 4);
  printf("memcmp result %d\n", res);
  // If the 'memcmp' bug exists, below will return 1.
  return (res == 0);
}
```

Omissions: only the named16 AWS sources and the named/narrow six frozen source files are copied. Quoted C/assembly/platform references in the AWS lists, further SDK/macro/concrete-field implementations, Tagged/derive implementations, generated artifacts and tool paths are not followed. Full files may contain additional declarations/tests; their presence does not select or execute them. No fetch, recursive source walk or expansion is performed.
