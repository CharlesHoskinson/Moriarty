# Axiom Kate division scan implementation
> Source: https://raw.githubusercontent.com/axiom-crypto/halo2-gpu/b2d4bcfd4261cc506b5412fb04fe631f028849d7/halo2_proofs/cuda/include/kernel/kate_division.h
> Collected: 2026-10-26
> Published: Unknown

SOURCE — Fetched with scrapling; raw response retrieved again to preserve source-code punctuation.

```
#pragma once

#include "field/alt_bn128.hpp"
#include <cuda.h>
#include <cuda_runtime.h>

typedef fr_t scalar_t;

namespace zkpcuda {
namespace kate_division {

    // Affine-pair Brent-Kung prefix scan for kate division.
    //
    // Computes the inclusive scan of affine functions
    //   f_i(x) = u * x + p[i]
    // under composition (a2, b2) ∘ (a1, b1) = (a2 * a1, a2 * b1 + b2)
    // where ∘ denotes "apply (a1, b1) first, then (a2, b2)".
    //
    // The CPU `kate_division(a, b)` recurrence (arithmetic.rs) is
    //   q[j] = a[j+1] + u * q[j+1]   for j = n-2..0
    // with u = b. Reindexing r[i] = q[n-2-i] and p[i] = a[n-1-i] gives
    //   r[i] = p[i] + u * r[i-1]     for i = 0..n-2  (r[-1] := 0)
    // which is exactly the value f_{n-2} ∘ ... ∘ f_0 applied to 0,
    // i.e. the b-component of the inclusive scan at index i.
    //
    // Per-element pair (during first_round):     (u, p[i]) = (u, a[length-i])
    //   where length = n-1, so a[length-offset] reads a[n-1-offset].
    // On subsequent rounds the pair state is read from (d_state_a, d_state_b).
    //
    // Scaffolding mirrors prefix_scan.h::prefix_scan_block. The only
    // structural differences are: (1) two state arrays for the affine
    // pair, (2) the affine combine op replaces multiplicative *=, (3)
    // identity is (one, zero) instead of one, (4) no external d_prefix.
    template <uint32_t ACC_PER_THREAD, uint32_t SHARED_DATA>
    __global__ void kate_division_scan_block(
        const scalar_t* d_p_input,      // length n; only read on first round
        scalar_t* d_state_a,            // length n-1; in/out
        scalar_t* d_state_b,            // length n-1; in/out
        const scalar_t* d_u,            // single scalar (the root)
        uint64_t length,                // n-1
        uint64_t round_stride)
    {
        scalar_t acc_a = scalar_t::one();
        scalar_t acc_b;
        acc_b.zero();
        scalar_t acc_data_a[ACC_PER_THREAD];
        scalar_t acc_data_b[ACC_PER_THREAD];
        __shared__ scalar_t shared_a[SHARED_DATA];
        __shared__ scalar_t shared_b[SHARED_DATA];

        uint32_t tile_idx = threadIdx.x;
        uint32_t shared_elem_per_block = blockDim.x;
        uint64_t index = (blockIdx.x * blockDim.x + threadIdx.x) * ACC_PER_THREAD;
        bool first_round = (round_stride == 1);

        scalar_t u_local = *d_u;

#pragma unroll
        for (uint32_t i = 0; i < ACC_PER_THREAD; i++) {
            scalar_t pair_a, pair_b;
            uint64_t offset = first_round ? index + i : (index + i + 1) * round_stride - 1;
            bool in_range = (offset < length);
            if (in_range) {
                if (first_round) {
                    pair_a = u_local;
                    pair_b = d_p_input[length - offset];
                } else {
                    pair_a = d_state_a[offset];
                    pair_b = d_state_b[offset];
                }
            } else {
                pair_a = scalar_t::one();
                pair_b.zero();
            }
            scalar_t new_a = pair_a * acc_a;
            scalar_t new_b = pair_a * acc_b + pair_b;
            acc_a = new_a;
            acc_b = new_b;
            acc_data_a[i] = acc_a;
            acc_data_b[i] = acc_b;
        }
        shared_a[tile_idx] = acc_a;
        shared_b[tile_idx] = acc_b;
        __syncthreads();

        // Brent-Kung upsweep
        uint32_t stride = 2;
        uint32_t src_offset = 0;
        uint32_t dst_offset = stride - 1;
        for (uint32_t idx = shared_elem_per_block >> 1; idx > 0; idx = idx >> 1) {
            if (tile_idx < idx) {
                scalar_t dst_a = shared_a[tile_idx * stride + dst_offset];
                scalar_t dst_b = shared_b[tile_idx * stride + dst_offset];
                scalar_t src_a = shared_a[tile_idx * stride + src_offset];
                scalar_t src_b = shared_b[tile_idx * stride + src_offset];
                scalar_t new_a = dst_a * src_a;
                scalar_t new_b = dst_a * src_b + dst_b;
                shared_a[tile_idx * stride + dst_offset] = new_a;
                shared_b[tile_idx * stride + dst_offset] = new_b;
            }
            src_offset = stride - 1;
            stride = stride << 1;
            dst_offset = stride - 1;
            __syncthreads();
        }

        // Brent-Kung downsweep
        for (uint32_t stride2 = shared_elem_per_block; stride2 > 1; stride2 = stride2 >> 1) {
            src_offset = stride2 - 1;
            dst_offset = src_offset + (stride2 >> 1);
            uint32_t thread_in_round = shared_elem_per_block / stride2 - 1;
            if (tile_idx < thread_in_round) {
                scalar_t dst_a = shared_a[tile_idx * stride2 + dst_offset];
                scalar_t dst_b = shared_b[tile_idx * stride2 + dst_offset];
                scalar_t src_a = shared_a[tile_idx * stride2 + src_offset];
                scalar_t src_b = shared_b[tile_idx * stride2 + src_offset];
                scalar_t new_a = dst_a * src_a;
                scalar_t new_b = dst_a * src_b + dst_b;
                shared_a[tile_idx * stride2 + dst_offset] = new_a;
                shared_b[tile_idx * stride2 + dst_offset] = new_b;
            }
            __syncthreads();
        }

        // Combine block-prefix with each thread's local scan, write back.
        scalar_t prefix_a = scalar_t::one();
        scalar_t prefix_b;
        prefix_b.zero();
        if (tile_idx > 0) {
            prefix_a = shared_a[tile_idx - 1];
            prefix_b = shared_b[tile_idx - 1];
        }

#pragma unroll
        for (uint32_t i = 0; i < ACC_PER_THREAD; i++) {
            uint64_t offset = first_round ? index + i : (index + i + 1) * round_stride - 1;
            if (offset < length) {
                scalar_t new_a = acc_data_a[i] * prefix_a;
                scalar_t new_b = acc_data_a[i] * prefix_b + acc_data_b[i];
                d_state_a[offset] = new_a;
                d_state_b[offset] = new_b;
            }
        }
    }

    // Propagate the prefix of the previous round_stride super-block into
    // each basic_level end-of-block within the current super-block.
    // Mirrors prefix_scan_block_downsweep; combine op is the affine pair.
    __global__ static void kate_division_scan_downsweep(
        scalar_t* d_state_a,
        scalar_t* d_state_b,
        uint64_t length,
        uint64_t round_stride,
        uint64_t basic_level)
    {
        uint64_t low_level_round_stride = round_stride / basic_level;
        uint64_t dst_index = (blockIdx.x * blockDim.x + threadIdx.x) * low_level_round_stride - 1;
        uint64_t src_index = (dst_index / round_stride) * round_stride - 1;
        bool is_last_elem = dst_index % round_stride == round_stride - 1;
        uint64_t level_offset = dst_index / round_stride;
        if (dst_index >= length || is_last_elem || level_offset == 0)
            return;

        scalar_t dst_a = d_state_a[dst_index];
        scalar_t dst_b = d_state_b[dst_index];
        scalar_t src_a = d_state_a[src_index];
        scalar_t src_b = d_state_b[src_index];
        d_state_a[dst_index] = dst_a * src_a;
        d_state_b[dst_index] = dst_a * src_b + dst_b;
    }

    // For every non-last position within a basic_level block, prepend the
    // immediately-preceding basic_level block's running prefix.
    // Mirrors prefix_scan_epilogue; combine op is the affine pair.
    __global__ static void kate_division_scan_epilogue(
        scalar_t* d_state_a,
        scalar_t* d_state_b,
        uint64_t length,
        uint64_t basic_level)
    {
        uint64_t dst_index = blockIdx.x * blockDim.x + threadIdx.x;
        uint64_t level_offset = dst_index / basic_level;
        uint64_t src_index = level_offset * basic_level - 1;
        bool is_last_elem_in_level = dst_index % basic_level == basic_level - 1;
        if (level_offset == 0 || dst_index >= length)
            return;
        if (is_last_elem_in_level)
            return;

        scalar_t dst_a = d_state_a[dst_index];
        scalar_t dst_b = d_state_b[dst_index];
        scalar_t src_a = d_state_a[src_index];
        scalar_t src_b = d_state_b[src_index];
        d_state_a[dst_index] = dst_a * src_a;
        d_state_b[dst_index] = dst_a * src_b + dst_b;
    }

    // Write the kate-division output by reversing the scan's b-component:
    //   d_q[j] = d_state_b[length - 1 - j]   for j in [0, length).
    __global__ static void kate_division_write_q(
        scalar_t* d_q,                 // length n-1
        const scalar_t* d_state_b,     // length n-1
        uint64_t length)               // n-1
    {
        uint64_t stride = (uint64_t)gridDim.x * (uint64_t)blockDim.x;
        uint64_t start = (uint64_t)blockIdx.x * (uint64_t)blockDim.x + (uint64_t)threadIdx.x;
        for (uint64_t j = start; j < length; j += stride) {
            d_q[j] = d_state_b[length - 1 - j];
        }
    }

    // Padded variant: write the length-(n-1) quotient at positions
    //   d_q[j] = d_state_b[length - 1 - j]   for j in [0, length)
    // and zero at positions
    //   d_q[j].zero()                        for j in [length, out_len).
    // The quotient and the trailing zeros are written together in this single
    // launch.
    //
    // Precondition: `out_len >= length`. The launcher enforces this; when
    // `length == 0` (n == 1), the kernel writes zeros over the full
    // `out_len` range and `d_state_b` is unread.
    __global__ static void kate_division_write_q_padded(
        scalar_t* d_q,                 // length out_len
        const scalar_t* d_state_b,     // length n-1 (unused if length == 0)
        uint64_t length,               // n-1
        uint64_t out_len)              // >= length
    {
        uint64_t stride = (uint64_t)gridDim.x * (uint64_t)blockDim.x;
        uint64_t start = (uint64_t)blockIdx.x * (uint64_t)blockDim.x + (uint64_t)threadIdx.x;
        for (uint64_t j = start; j < out_len; j += stride) {
            if (j < length) {
                d_q[j] = d_state_b[length - 1 - j];
            } else {
                d_q[j].zero();
            }
        }
    }

} // namespace kate_division
} // namespace zkpcuda

```
