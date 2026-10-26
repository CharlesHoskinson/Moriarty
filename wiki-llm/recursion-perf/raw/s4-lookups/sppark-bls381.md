# sppark BLS12-381 field types
> Source: https://raw.githubusercontent.com/supranational/sppark/main/ff/bls12-381.hpp
> Collected: 2026-10-26
> Published: Unknown

// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
#ifndef \_\_SPPARK\_FF\_BLS13\_381\_HPP\_\_
#define \_\_SPPARK\_FF\_BLS13\_381\_HPP\_\_
#include 
#if defined(\_\_CUDACC\_\_) || defined(\_\_HIPCC\_\_)
namespace device {
#define TO\_CUDA\_T(limb64) (uint32\_t)(limb64), (uint32\_t)(limb64>>32)
static \_\_device\_\_ \_\_constant\_\_ \_\_align\_\_(16) const uint32\_t BLS12\_381\_P[12] = {
TO\_CUDA\_T(0xb9feffffffffaaab), TO\_CUDA\_T(0x1eabfffeb153ffff),
TO\_CUDA\_T(0x6730d2a0f6b0f624), TO\_CUDA\_T(0x64774b84f38512bf),
TO\_CUDA\_T(0x4b1ba7b6434bacd7), TO\_CUDA\_T(0x1a0111ea397fe69a)
};
static \_\_device\_\_ \_\_constant\_\_ \_\_align\_\_(16) const uint32\_t BLS12\_381\_RR[12] = { /\* (1<<768)%P \*/
TO\_CUDA\_T(0xf4df1f341c341746), TO\_CUDA\_T(0x0a76e6a609d104f1),
TO\_CUDA\_T(0x8de5476c4c95b6d5), TO\_CUDA\_T(0x67eb88a9939d83c0),
TO\_CUDA\_T(0x9a793e85b519952d), TO\_CUDA\_T(0x11988fe592cae3aa)
};
static \_\_device\_\_ \_\_constant\_\_ \_\_align\_\_(16) const uint32\_t BLS12\_381\_one[12] = { /\* (1<<384)%P \*/
TO\_CUDA\_T(0x760900000002fffd), TO\_CUDA\_T(0xebf4000bc40c0002),
TO\_CUDA\_T(0x5f48985753c758ba), TO\_CUDA\_T(0x77ce585370525745),
TO\_CUDA\_T(0x5c071a97a256ec6d), TO\_CUDA\_T(0x15f65ec3fa80e493)
};
static \_\_device\_\_ \_\_constant\_\_ \_\_align\_\_(16) const uint32\_t BLS12\_381\_Px8[12] = { /\* left-aligned value of the modulus \*/
TO\_CUDA\_T(0xcff7fffffffd5558), TO\_CUDA\_T(0xf55ffff58a9ffffd),
TO\_CUDA\_T(0x39869507b587b120), TO\_CUDA\_T(0x23ba5c279c2895fb),
TO\_CUDA\_T(0x58dd3db21a5d66bb), TO\_CUDA\_T(0xd0088f51cbff34d2)
};
static \_\_device\_\_ \_\_constant\_\_ const uint32\_t BLS12\_381\_M0 = 0xfffcfffd;
static \_\_device\_\_ \_\_constant\_\_ \_\_align\_\_(16) const uint32\_t BLS12\_381\_r[8] = {
TO\_CUDA\_T(0xffffffff00000001), TO\_CUDA\_T(0x53bda402fffe5bfe),
TO\_CUDA\_T(0x3339d80809a1d805), TO\_CUDA\_T(0x73eda753299d7d48)
};
static \_\_device\_\_ \_\_constant\_\_ \_\_align\_\_(16) const uint32\_t BLS12\_381\_rRR[8] = { /\* (1<<512)%P \*/
TO\_CUDA\_T(0xc999e990f3f29c6d), TO\_CUDA\_T(0x2b6cedcb87925c23),
TO\_CUDA\_T(0x05d314967254398f), TO\_CUDA\_T(0x0748d9d99f59ff11)
};
static \_\_device\_\_ \_\_constant\_\_ \_\_align\_\_(16) const uint32\_t BLS12\_381\_rone[8] = { /\* (1<<256)%P \*/
TO\_CUDA\_T(0x00000001fffffffe), TO\_CUDA\_T(0x5884b7fa00034802),
TO\_CUDA\_T(0x998c4fefecbc4ff5), TO\_CUDA\_T(0x1824b159acc5056f)
};
static \_\_device\_\_ \_\_constant\_\_ \_\_align\_\_(16) const uint32\_t BLS12\_381\_rx2[8] = { /\* left-aligned value of the modulus \*/
TO\_CUDA\_T(0xfffffffe00000002), TO\_CUDA\_T(0xa77b4805fffcb7fd),
TO\_CUDA\_T(0x6673b0101343b00a), TO\_CUDA\_T(0xe7db4ea6533afa90),
};
static \_\_device\_\_ \_\_constant\_\_ /\*const\*/ uint32\_t BLS12\_381\_m0 = 0xffffffff;
}
# if defined(\_\_CUDA\_ARCH\_\_) || defined(\_\_HIPCC\_\_) // device-side field types
# if defined(\_\_CUDA\_ARCH\_\_)
# include "mont\_t.cuh"
# elif defined(\_\_HIPCC\_\_)
# include "mont\_t.hip"
typedef uint64\_t vec256[4];
# endif
namespace bls12\_381 {
typedef mont\_t<381, device::BLS12\_381\_P, device::BLS12\_381\_M0,
device::BLS12\_381\_RR, device::BLS12\_381\_one,
device::BLS12\_381\_Px8> fp\_mont;
struct fp\_t : public fp\_mont {
using mem\_t = fp\_t;
\_\_device\_\_ \_\_forceinline\_\_ fp\_t() {}
\_\_device\_\_ \_\_forceinline\_\_ fp\_t(const fp\_mont& a) : fp\_mont(a) {}
template constexpr fp\_t(Ts... a) : fp\_mont{a...} {}
};
typedef mont\_t<255, device::BLS12\_381\_r, device::BLS12\_381\_m0,
device::BLS12\_381\_rRR, device::BLS12\_381\_rone,
device::BLS12\_381\_rx2> fr\_mont;
struct fr\_t : public fr\_mont {
using mem\_t = fr\_t;
\_\_device\_\_ \_\_forceinline\_\_ fr\_t() {}
\_\_device\_\_ \_\_forceinline\_\_ fr\_t(const fr\_mont& a) : fr\_mont(a) {}
template constexpr fr\_t(Ts... a) : fr\_mont{a...} {}
# ifdef \_\_HIPCC\_\_
\_\_host\_\_ \_\_forceinline\_\_ fr\_t(vec256 a) : fr\_mont(a) {}
# endif
};
} // namespace bls12\_381
# endif
#endif
#if !defined(\_\_CUDA\_ARCH\_\_) && !defined(\_\_HIPCC\_\_) // host-side field types
# include 
# if defined(\_\_GNUC\_\_) && !defined(\_\_clang\_\_)
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wsubobject-linkage"
# endif
namespace bls12\_381 {
static const vec384 BLS12\_381\_P = {
TO\_LIMB\_T(0xb9feffffffffaaab), TO\_LIMB\_T(0x1eabfffeb153ffff),
TO\_LIMB\_T(0x6730d2a0f6b0f624), TO\_LIMB\_T(0x64774b84f38512bf),
TO\_LIMB\_T(0x4b1ba7b6434bacd7), TO\_LIMB\_T(0x1a0111ea397fe69a)
};
static const vec384 BLS12\_381\_RR = { /\* (1<<768)%P \*/
TO\_LIMB\_T(0xf4df1f341c341746), TO\_LIMB\_T(0x0a76e6a609d104f1),
TO\_LIMB\_T(0x8de5476c4c95b6d5), TO\_LIMB\_T(0x67eb88a9939d83c0),
TO\_LIMB\_T(0x9a793e85b519952d), TO\_LIMB\_T(0x11988fe592cae3aa)
};
static const vec384 BLS12\_381\_ONE = { /\* (1<<384)%P \*/
TO\_LIMB\_T(0x760900000002fffd), TO\_LIMB\_T(0xebf4000bc40c0002),
TO\_LIMB\_T(0x5f48985753c758ba), TO\_LIMB\_T(0x77ce585370525745),
TO\_LIMB\_T(0x5c071a97a256ec6d), TO\_LIMB\_T(0x15f65ec3fa80e493)
};
typedef blst\_384\_t<381, BLS12\_381\_P, 0x89f3fffcfffcfffd,
BLS12\_381\_RR, BLS12\_381\_ONE> fp\_mont;
struct fp\_t : public fp\_mont {
using mem\_t = fp\_t;
inline fp\_t() {}
inline fp\_t(const fp\_mont& a) : fp\_mont(a) {}
template
constexpr fp\_t(Ts... a) : fp\_mont{a...} {}
};
static const vec256 BLS12\_381\_r = {
TO\_LIMB\_T(0xffffffff00000001), TO\_LIMB\_T(0x53bda402fffe5bfe),
TO\_LIMB\_T(0x3339d80809a1d805), TO\_LIMB\_T(0x73eda753299d7d48)
};
static const vec256 BLS12\_381\_rRR = { /\* (1<<512)%r \*/
TO\_LIMB\_T(0xc999e990f3f29c6d), TO\_LIMB\_T(0x2b6cedcb87925c23),
TO\_LIMB\_T(0x05d314967254398f), TO\_LIMB\_T(0x0748d9d99f59ff11)
};
static const vec256 BLS12\_381\_rONE = { /\* (1<<256)%r \*/
TO\_LIMB\_T(0x00000001fffffffe), TO\_LIMB\_T(0x5884b7fa00034802),
TO\_LIMB\_T(0x998c4fefecbc4ff5), TO\_LIMB\_T(0x1824b159acc5056f)
};
typedef blst\_256\_t<255, BLS12\_381\_r, 0xfffffffeffffffff,
BLS12\_381\_rRR, BLS12\_381\_rONE> fr\_mont;
struct fr\_t : public fr\_mont {
using mem\_t = fr\_t;
inline fr\_t() {}
inline fr\_t(const fr\_mont& a) : fr\_mont(a) {}
template
constexpr fr\_t(Ts... a) : fr\_mont{a...} {}
};
} // namespace bls12\_381
# if defined(\_\_GNUC\_\_) && !defined(\_\_clang\_\_)
# pragma GCC diagnostic pop
# endif
#endif
#ifdef FEATURE\_BLS12\_381
using namespace bls12\_381;
#endif
#endif