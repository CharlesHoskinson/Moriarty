#include <assert.h>
#include <chrono>
#include <map>
#include <string>
#include <vector>

#include "common/exception.h"
#include "common/halo2_ffi.h"

#include "kernel/quotient.h"

using Scalar = utils::FFITraitObject;

// Lookup quotient. All inputs are device-resident borrowed pointers;
// every H2D happens in the Rust caller.
extern "C" RustError _halo2_quotient_lookups(
    // device-borrowed
    void* d_values,                          // in/out
    const void* d_table_values,
    const void* d_product_coset,
    const void* d_permuted_input_coset,
    const void* d_permuted_table_coset,
    const void* d_l0,
    const void* d_l_last,
    const void* d_l_active_row,
    const void* d_beta,
    const void* d_gamma,
    const void* d_y,
    uint64_t poly_length,
    cudaStream_t stream)
{
    try {
        const uint32_t tile_size = 64;
        const uint32_t block_num = 1024;
        zkpcuda::quotient::cuda_kernel_quotient_lookups<tile_size><<<block_num, tile_size, 0, stream>>>(
            (scalar_t*)d_values,
            (scalar_t*)d_table_values,
            (scalar_t*)d_product_coset,
            (scalar_t*)d_permuted_input_coset,
            (scalar_t*)d_permuted_table_coset,
            (scalar_t*)d_l0,
            (scalar_t*)d_l_last,
            (scalar_t*)d_l_active_row,
            (scalar_t*)d_beta,
            (scalar_t*)d_gamma,
            (scalar_t*)d_y,
            poly_length);
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    return cudaSuccess;
}
