# cuPQC device NTT constraints
> Source: https://docs.nvidia.com/cuda/cupqc/libraries/cupqc_ntt/cupqc_ntt_feature.html
> Collected: 2026-10-26
> Published: Unknown

cuPQC-NTT: Number Theoretic Transform — cuPQC

[Skip to main content](#main-content)

Back to top


`Ctrl`+`K`

[![cuPQC - Home](../../_static/nvidia-logo-horiz-rgb-blk-for-screen.svg)![cuPQC - Home](../../_static/nvidia-logo-horiz-rgb-wht-for-screen.svg)

cuPQC](../../index.html)

Search
`Ctrl`+`K`

* System Settings
* Light
* Dark

Search
`Ctrl`+`K`

[![cuPQC - Home](../../_static/nvidia-logo-horiz-rgb-blk-for-screen.svg)![cuPQC - Home](../../_static/nvidia-logo-horiz-rgb-wht-for-screen.svg)

cuPQC](../../index.html)

* System Settings
* Light
* Dark

Table of Contents

Introduction

* [Home](../../index.html)
* [Getting Started](../../introduction/getting_started.html)
* [System Requirements](../../introduction/requirements.html)

Libraries

* [cuPQC-BigInt](../cupqc_bigint/index.html)
  + [Features](../cupqc_bigint/cupqc_bigint_feature.html)
  + [Usage Guide](../cupqc_bigint/cupqc_bigint_usage.html)
  + [API Reference](../cupqc_bigint/cupqc_bigint_api.html)
    - [Operators](../cupqc_bigint/api/operators.html)
    - [Types](../cupqc_bigint/api/types.html)
    - [Device Functions](../cupqc_bigint/api/device.html)
* [cuPQC-NTT](index.html)
  + [Features](#)
  + [Usage Guide](cupqc_ntt_usage.html)
  + [API Reference](cupqc_ntt_api.html)
    - [Operators](api/operators.html)
    - [Device Functions](api/device.html)
    - [Host Functions](api/host.html)
    - [Prime Field Constants](api/prime_fields.html)
    - [Types](api/types.html)
* [cuPQC-Hash](../cupqc_hash/index.html)
  + [Features](../cupqc_hash/cupqc_hash_feature.html)
  + [Usage Guide](../cupqc_hash/cupqc_hash_usage.html)
  + [API Reference](../cupqc_hash/cupqc_hash_api.html)
    - [Hash Functions](../cupqc_hash/api/hash_index.html)
      * [Operators](../cupqc_hash/api/hash/operators.html)
      * [Traits](../cupqc_hash/api/hash/traits.html)
      * [Device Functions](../cupqc_hash/api/hash/device.html)
    - [Merkle Trees](../cupqc_hash/api/merkle_index.html)
      * [Operators](../cupqc_hash/api/merkle/operators.html)
      * [Objects](../cupqc_hash/api/merkle/objects.html)
      * [Device Functions](../cupqc_hash/api/merkle/device.html)
* [cuPQC-PK](../cupqc_pk/index.html)
  + [Features](../cupqc_pk/cupqc_pk_feature.html)
  + [Usage Guide](../cupqc_pk/cupqc_pk_usage.html)
  + [API Reference](../cupqc_pk/cupqc_pk_api.html)
    - [Operators](../cupqc_pk/api/operators.html)
    - [Traits](../cupqc_pk/api/traits.html)
    - [Device Functions](../cupqc_pk/api/device.html)
    - [Host Functions](../cupqc_pk/api/host.html)

Additional

* [Release Notes](../../additional/release_notes.html)
* [Security Notes](../../additional/security_notes.html)
* [Software License Agreement](../../additional/license.html)

* [cuPQC-NTT](index.html)
* cuPQC-NTT: Number Theoretic Transform

cuPQC-NTT: Number Theoretic Transform[#](#cupqc-ntt-number-theoretic-transform "Link to this heading")
======================================================================================================

The cuPQC-NTT library provides GPU-optimized implementations of the Number Theoretic Transform
(NTT) designed for direct integration into CUDA kernels. The NTT is the modular-arithmetic
analogue of the Fast Fourier Transform and is a core building block of lattice-based
cryptography, zero-knowledge proof systems, and modular ring arithmetic over finite fields.
As a device-side library, cuPQC-NTT integrates directly into your CUDA kernels, enabling you
to fuse NTTs with other GPU computations for maximum performance.
Transforms run in the Montgomery domain; convert at load/store boundaries and convert
twiddle tables once before reuse.

The notation in the panel below is used throughout the rest of this page for transform lengths and two-pass tiling.

Notation for `N`, `M`, `K`, and `d`

Transform length `N` is a power-of-two, with N =`2``d` and `d` = log2(`N`).

Sub-transform size `M` (a power-of-two) is set with `SubSize<M>`; `K = N/M`.

NTT Transform Modes[#](#ntt-transform-modes "Link to this heading")
-------------------------------------------------------------------

The cuPQC-NTT library provides two execution modes:

* **Standard NTT:** A complete `N`-point transform is performed in a single kernel launch per length-`N` buffer, with the full working set held in shared memory. This mode is ideal for transform lengths that fit within available shared memory.
* **Staged NTT:** For large `N`, per-block shared memory cannot hold the full workspace for an `N`-point transform in a single pass, so a staged NTT is required. With `SubSize<M>` and **two** kernel passes: the first stage runs `M` parallel `K`-point contiguous NTTs (`K = N/M`); the second runs `K` parallel `M`-point strided NTTs. Each stage limits the per-block working set to `max(K, M)` elements, enabling transforms that would not fit in a single block’s shared memory.

Supported Configurations[#](#supported-configurations "Link to this heading")
-----------------------------------------------------------------------------

The table below summarizes the supported **Execution Modes**, **Transform Type**, **Prime Fields**, and **Precomputed Roots**.

| Aspect | Summary |
| --- | --- |
| **Execution modes** | **Standard:** One-kernel transform of length-`N`.  **Staged:** Two-kernel transforms of `(N, SubSize<M>)` pairs. |
| **Transform type** | **Cyclic:** Length-`N` transforms for arithmetic in the cyclic polynomial ring (modulo `xN - 1`). |
| **Prime field** | **Custom:** User-selected prime `p` < `2``62`.  **Built-in:** `BabyBear` / `KoalaBear`. |
| **Precomputed roots** | `BabyBear` / `KoalaBear`: `*_primitive_root_S` (`10` ≤ `S` ≤ `24`). |

See [Prime Field Constants](api/prime_fields.html) for built-in prime values and compile-time root symbols.

Both **Standard NTT** and **Staged NTT** support `uint16_t`, `uint32_t`, and `uint64_t` as precisions for any prime modulus.
The next two subsections tabulate supported transform-size limits for **Standard NTT** and **Staged NTT**.

### Standard NTT Limits[#](#standard-ntt-limits "Link to this heading")

The table lists **Standard NTT** (single-kernel) limits on transform length `N` and exponent `d`. Coefficients and the modulus use the same type width (for example both `uint32_t`) for each prime and coefficient-width row.

| Prime | Width (precision) | Transform Size (N =`2``d`) |
| --- | --- | --- |
| **Custom** `p` | `uint16_t` | `2 ≤ d ≤ 14` |
| `uint32_t` | `2 ≤ d ≤ 13` |
| `uint64_t` | `2 ≤ d ≤ 13` |

### Staged NTT Limits[#](#staged-ntt-limits "Link to this heading")

The table lists **Staged NTT** (two-kernel) limits, pairing each supported transform length `N` with allowed `SubSize<M>` values.

| Transform Size (N =`2``d`) | Allowed Sub-transform Size (`SubSize<M>`) |
| --- | --- |
| `14 ≤ d ≤ 21` | `256` · `512` · `1024` · `2048` · `4096` · `8192` |
| `d = 22` | `512` · `1024` · `2048` · `4096` · `8192` |
| `d = 23` | `1024` · `2048` · `4096` · `8192` |
| `d = 24` | `2048` · `4096` · `8192` |

Key Features[#](#key-features "Link to this heading")
-----------------------------------------------------

* **High Performance:** GPU-optimized NTT kernels exploit shared memory and warp-level
  parallelism for maximum throughput.
* **Montgomery Domain:** Device transforms operate on Montgomery-domain data, with
  conversion at load/store (`load_to_mont` / `store_from_mont` and staged counterparts);
  twiddle tables are converted once before reuse.
* **Cyclic Transforms:** Implements the cyclic NTT for rings modulo `xN - 1`.
* **Flexible Field Support:** Supports user-selected primes up to **62-bit** and built-in primes `BabyBear` / `KoalaBear`.
* **Staged Transforms:** The `SubSize<M>` operator enables memory-efficient transforms at
  large `N` by decomposing them into two kernel passes.
* **Reusable Twiddle Tables:** Twiddle factors are computed once and reused across an entire
  batch, minimizing startup overhead.

[previous

cuPQC-NTT](index.html "previous page")
[next

cuPQC-NTT Usage](cupqc_ntt_usage.html "next page")

On this page

* [NTT Transform Modes](#ntt-transform-modes)
* [Supported Configurations](#supported-configurations)
  + [Standard NTT Limits](#standard-ntt-limits)
  + [Staged NTT Limits](#staged-ntt-limits)
* [Key Features](#key-features)

[![NVIDIA](../../_static/nvidia-logo-horiz-rgb-1c-blk-for-screen.svg)![NVIDIA](../../_static/nvidia-logo-horiz-rgb-1c-wht-for-screen.svg)](https://www.nvidia.com)

[Privacy Policy](https://www.nvidia.com/en-us/about-nvidia/privacy-policy/)
|
[Your Privacy Choices](https://www.nvidia.com/en-us/about-nvidia/privacy-center/)
|
[Terms of Service](https://www.nvidia.com/en-us/about-nvidia/terms-of-service/)
|
[Accessibility](https://www.nvidia.com/en-us/about-nvidia/accessibility/)
|
[Corporate Policies](https://www.nvidia.com/en-us/about-nvidia/company-policies/)
|
[Product Security](https://www.nvidia.com/en-us/product-security/)
|
[Contact](https://www.nvidia.com/en-us/contact/)

Copyright © 2024-2026, NVIDIA Corporation & Affiliates. All rights reserved.

![](https://docs.nvidia.com/akam/13/pixel_4b3bf5?a=dD1kNGRiZjMxMDJkZTExMDAwZGY5OTgwOGJjMGMwOTNmMTI0Mjk5MDAxJmpzPW9mZg==)