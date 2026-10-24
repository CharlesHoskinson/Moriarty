// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
#pragma once
// Device-side BN254 fr\_t / fp\_t and Montgomery constants now live in
// cuda-common. This file is a shim: it pulls the canonical device path
// from cuda-common and retains the host-side blst-based field types in
// place so NVCC's host-compile pass over kernel headers — which see
// `typedef fr\_t scalar\_t;` outside \_\_CUDA\_ARCH\_\_ — still finds concrete
// types. Replacing the host blst path is a follow-up.
#include "ff/alt\_bn128.cuh"
#ifndef \_\_CUDA\_ARCH\_\_ // host-side field types
#include "../blst/blst\_t.hpp"
static const vec256 ALT\_BN128\_P = {
TO\_LIMB\_T(0x3c208c16d87cfd47), TO\_LIMB\_T(0x97816a916871ca8d),
TO\_LIMB\_T(0xb85045b68181585d), TO\_LIMB\_T(0x30644e72e131a029)
};
static const vec256 ALT\_BN128\_RR = { /\* (1<<512)%P \*/
TO\_LIMB\_T(0xf32cfc5b538afa89), TO\_LIMB\_T(0xb5e71911d44501fb),
TO\_LIMB\_T(0x47ab1eff0a417ff6), TO\_LIMB\_T(0x06d89f71cab8351f),
};
static const vec256 ALT\_BN128\_ONE = { /\* (1<<256)%P \*/
TO\_LIMB\_T(0xd35d438dc58f0d9d), TO\_LIMB\_T(0x0a78eb28f5c70b3d),
TO\_LIMB\_T(0x666ea36f7879462c), TO\_LIMB\_T(0x0e0a77c19a07df2f)
};
typedef blst\_256\_t fp\_t;
static const vec256 ALT\_BN128\_r = {
TO\_LIMB\_T(0x43e1f593f0000001), TO\_LIMB\_T(0x2833e84879b97091),
TO\_LIMB\_T(0xb85045b68181585d), TO\_LIMB\_T(0x30644e72e131a029)
};
static const vec256 ALT\_BN128\_rRR = { /\* (1<<512)%r \*/
TO\_LIMB\_T(0x1bb8e645ae216da7), TO\_LIMB\_T(0x53fe3ab1e35c59e3),
TO\_LIMB\_T(0x8c49833d53bb8085), TO\_LIMB\_T(0x0216d0b17f4e44a5)
};
static const vec256 ALT\_BN128\_rONE = { /\* (1<<256)%r \*/
TO\_LIMB\_T(0xac96341c4ffffffb), TO\_LIMB\_T(0x36fc76959f60cd29),
TO\_LIMB\_T(0x666ea36f7879462e), TO\_LIMB\_T(0x0e0a77c19a07df2f)
};
typedef blst\_256\_t fr\_t;
#endif