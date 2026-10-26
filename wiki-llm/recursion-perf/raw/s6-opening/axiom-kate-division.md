# Axiom Kate division CUDA
> Source: https://raw.githubusercontent.com/axiom-crypto/halo2-gpu/b2d4bcfd4261cc506b5412fb04fe631f028849d7/halo2_proofs/cuda/src/polynomial/kate_division.cu
> Collected: 2026-10-26
> Published: Unknown

SOURCE — Fetched with scrapling; raw response retrieved again to preserve source-code punctuation.

```
#include <assert.h>
#include <chrono>
#include <map>
#include <string>
#include <vector>

#include "common/exception.h"
#include "common/halo2_ffi.h"
#include "common/scratch_span.h"

#include "kernel/kate_division.h"

using Scalar = utils::FFITraitObject;

namespace {

// Kernel launch shape mirrors `_halo2_grand_product_device_inputs` in
// `commitment/grand_product.cu`: acc=4 per thread, 256 threads per block,
// 1024 elements per block per scan round.
constexpr uint32_t kAccPerThread = 4;
constexpr uint32_t kTilesPerBlock = 256;
constexpr uint32_t kElementsPerBlock = kAccPerThread * kTilesPerBlock;

} // namespace

// Workspace = two scalar arrays of length (n-1) — the affine-pair scan state.
extern "C" uint64_t _halo2_kate_division_workspace_size(uint64_t n)
{
    if (n <= 1) {
        return 0;
    }
    const uint64_t field_size = Scalar::ELT_BYTES;
    const uint64_t state_bytes = (n - 1) * field_size;
    // Two arrays, each aligned-up to 32 bytes by ScratchSpan::take.
    return align_up(state_bytes, 32) + align_up(state_bytes, 32);
}

// Compute q(X) = (a(X) - a(u)) / (X - u) on the device.
//   d_a:  caller-owned device pointer to a length-n input poly (read-only).
//   d_q:  caller-owned device pointer to a length-(n-1) output poly.
//   d_u:  caller-owned device pointer to a single 32-byte scalar.
// Scratch: workspace block of size `_halo2_kate_division_workspace_size(n)`.
//
// Pipeline:
//   1) affine-pair Brent-Kung scan over (u, p[length - i]) values, where
//      p[length - i] = a[n-1 - i] (reversed indexing into d_a); state lives
//      in two parallel scratch arrays (`d_state_a`, `d_state_b`).
//   2) reverse-write d_q[j] = d_state_b[length - 1 - j].
extern "C" RustError _halo2_kate_division_device(
    const void* d_a,
    void* d_q,
    const void* d_u,
    uint64_t n,
    void* scratch,
    uint64_t scratch_bytes,
    cudaStream_t stream)
{
    if (n < 2) {
        // q has length n-1; length 0 is a no-op.
        return cudaSuccess;
    }
    const uint64_t length = n - 1;
    const uint64_t state_bytes = length * Scalar::ELT_BYTES;

    ScratchSpan span { (uint8_t*)scratch, (size_t)scratch_bytes };
    scalar_t* d_state_a = (scalar_t*)span.take(state_bytes);
    scalar_t* d_state_b = (scalar_t*)span.take(state_bytes);

    try {
        // First round: scan over the full length-(n-1) range, reading p[i]
        // from d_a in reversed index order.
        uint32_t block_num = (length + kElementsPerBlock - 1) / kElementsPerBlock;
        uint64_t round_stride = 1;
        zkpcuda::kate_division::kate_division_scan_block<kAccPerThread, kTilesPerBlock>
            <<<block_num, kTilesPerBlock, 0, stream>>>(
                (const scalar_t*)d_a,
                d_state_a,
                d_state_b,
                (const scalar_t*)d_u,
                length,
                round_stride);

        // Subsequent rounds: scan over the per-block end-of-block pair
        // values, climbing round_stride by element_per_block per pass.
        while (block_num > 1) {
            block_num = (block_num + kElementsPerBlock - 1) / kElementsPerBlock;
            round_stride = round_stride * kElementsPerBlock;
            zkpcuda::kate_division::kate_division_scan_block<kAccPerThread, kTilesPerBlock>
                <<<block_num, kTilesPerBlock, 0, stream>>>(
                    (const scalar_t*)d_a,
                    d_state_a,
                    d_state_b,
                    (const scalar_t*)d_u,
                    length,
                    round_stride);
        }

        // Block downsweep: propagate higher-level prefixes back down.
        while (round_stride > kElementsPerBlock) {
            uint64_t low_level_round_stride = round_stride / kElementsPerBlock;
            uint64_t node_num = (length + low_level_round_stride - 1) / low_level_round_stride;
            uint64_t down_block_num = (node_num + 256 - 1) / 256;
            zkpcuda::kate_division::kate_division_scan_downsweep
                <<<down_block_num, 256, 0, stream>>>(
                    d_state_a,
                    d_state_b,
                    length,
                    round_stride,
                    kElementsPerBlock);
            round_stride = low_level_round_stride;
        }

        // Epilogue: propagate basic-level prefixes into inner positions.
        uint32_t epilog_block_num = (length + 256 - 1) / 256;
        zkpcuda::kate_division::kate_division_scan_epilogue
            <<<epilog_block_num, 256, 0, stream>>>(
                d_state_a,
                d_state_b,
                length,
                kElementsPerBlock);

        // Reverse-write q[j] = d_state_b[length-1 - j].
        const uint32_t write_threads = 256;
        const uint32_t write_blocks = (length + write_threads - 1) / write_threads;
        zkpcuda::kate_division::kate_division_write_q
            <<<write_blocks, write_threads, 0, stream>>>(
                (scalar_t*)d_q,
                d_state_b,
                length);
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    return cudaSuccess;
}

// Padded variant: compute the kate-division quotient on the device AND
// write trailing zeros so the output buffer is length `out_len`
// (`out_len >= n-1`). The padded write kernel emits the length-(n-1)
// quotient at positions [0, n-1) and zeros at [n-1, out_len) in a
// single launch.
//
// Behavior at `out_len == 0`: no-op. At `n < 2` (length == 0): only the
// padded write kernel runs, zeroing the full `out_len` range; no scan.
// Otherwise: identical scan as `_halo2_kate_division_device`, then a
// padded write that combines the quotient reverse-write with the
// trailing zero-pad.
extern "C" RustError _halo2_kate_division_device_padded(
    const void* d_a,
    void* d_q,
    const void* d_u,
    uint64_t n,
    uint64_t out_len,
    void* scratch,
    uint64_t scratch_bytes,
    cudaStream_t stream)
{
    if (out_len == 0) {
        return cudaSuccess;
    }

    if (n < 2) {
        // Length-0 quotient padded to `out_len`: write zeros only.
        try {
            const uint32_t write_threads = 256;
            const uint32_t write_blocks =
                (out_len + write_threads - 1) / write_threads;
            zkpcuda::kate_division::kate_division_write_q_padded
                <<<write_blocks, write_threads, 0, stream>>>(
                    (scalar_t*)d_q,
                    /*d_state_b=*/nullptr,
                    /*length=*/0,
                    out_len);
        } catch (const cuda_error& error) {
            return RustError(error.code(), error.what());
        };
        return cudaSuccess;
    }

    const uint64_t length = n - 1;
    const uint64_t state_bytes = length * Scalar::ELT_BYTES;

    ScratchSpan span { (uint8_t*)scratch, (size_t)scratch_bytes };
    scalar_t* d_state_a = (scalar_t*)span.take(state_bytes);
    scalar_t* d_state_b = (scalar_t*)span.take(state_bytes);

    try {
        // First round.
        uint32_t block_num = (length + kElementsPerBlock - 1) / kElementsPerBlock;
        uint64_t round_stride = 1;
        zkpcuda::kate_division::kate_division_scan_block<kAccPerThread, kTilesPerBlock>
            <<<block_num, kTilesPerBlock, 0, stream>>>(
                (const scalar_t*)d_a,
                d_state_a,
                d_state_b,
                (const scalar_t*)d_u,
                length,
                round_stride);

        while (block_num > 1) {
            block_num = (block_num + kElementsPerBlock - 1) / kElementsPerBlock;
            round_stride = round_stride * kElementsPerBlock;
            zkpcuda::kate_division::kate_division_scan_block<kAccPerThread, kTilesPerBlock>
                <<<block_num, kTilesPerBlock, 0, stream>>>(
                    (const scalar_t*)d_a,
                    d_state_a,
                    d_state_b,
                    (const scalar_t*)d_u,
                    length,
                    round_stride);
        }

        while (round_stride > kElementsPerBlock) {
            uint64_t low_level_round_stride = round_stride / kElementsPerBlock;
            uint64_t node_num = (length + low_level_round_stride - 1) / low_level_round_stride;
            uint64_t down_block_num = (node_num + 256 - 1) / 256;
            zkpcuda::kate_division::kate_division_scan_downsweep
                <<<down_block_num, 256, 0, stream>>>(
                    d_state_a,
                    d_state_b,
                    length,
                    round_stride,
                    kElementsPerBlock);
            round_stride = low_level_round_stride;
        }

        uint32_t epilog_block_num = (length + 256 - 1) / 256;
        zkpcuda::kate_division::kate_division_scan_epilogue
            <<<epilog_block_num, 256, 0, stream>>>(
                d_state_a,
                d_state_b,
                length,
                kElementsPerBlock);

        // Padded reverse-write fused with the trailing zero-pad.
        const uint32_t write_threads = 256;
        const uint32_t write_blocks =
            (out_len + write_threads - 1) / write_threads;
        zkpcuda::kate_division::kate_division_write_q_padded
            <<<write_blocks, write_threads, 0, stream>>>(
                (scalar_t*)d_q,
                d_state_b,
                length,
                out_len);
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    return cudaSuccess;
}

```
