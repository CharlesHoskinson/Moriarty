#include <cstdint>

#include "common/exception.h"
#include "common/halo2_ffi.h"
#include "kernel/permutation_quotient.h"

using Scalar = utils::FFITraitObject;

namespace {

constexpr uint32_t kTileSize = 64;
constexpr uint32_t kBlockNum = 1024;

} // namespace

// `d_values` / `d_l0` / `d_l_last` / `d_l_active_row` alias the device
// buffers `QuotientLookupsGpu` owns for the lookup quotient; this entry
// point folds the permutation argument into the same running quotient
// accumulator. `d_perm_prod_cosets` / `d_perm_cosets` / `d_column_values`
// are all device-resident arrays of device pointers built on the Rust
// side (one entry per polynomial). The `h_*` scalar params are
// dereferenced host-side and passed as kernel-arg scalars by value.
extern "C" RustError _halo2_quotient_permutation(
    // device-borrowed vectors
    void* d_values,
    const void* d_l0,
    const void* d_l_last,
    const void* d_l_active_row,
    // device-borrowed staging built on the Rust side
    const void* d_perm_prod_cosets,
    const void* d_perm_cosets,
    const void* d_column_values,
    // host-borrowed scalar params
    const void* h_beta,
    const void* h_gamma,
    const void* h_y,
    const void* h_delta,
    const void* h_delta_start,
    const void* h_current_extended_omega,
    const void* h_omega,
    // metadata
    uint64_t n_sets,
    uint64_t chunk_len,
    uint64_t n_perm_cols,
    int32_t last_rotation,
    int32_t rot_scale,
    int32_t isize_,
    uint64_t poly_length,
    cudaStream_t stream)
{
    if (n_sets == 0) {
        return cudaSuccess;
    }

    try {
        const scalar_t beta_scalar = *(const scalar_t*)h_beta;
        const scalar_t gamma_scalar = *(const scalar_t*)h_gamma;
        const scalar_t y_scalar = *(const scalar_t*)h_y;
        const scalar_t delta_scalar = *(const scalar_t*)h_delta;
        const scalar_t delta_start_scalar = *(const scalar_t*)h_delta_start;
        const scalar_t current_extended_omega_scalar = *(const scalar_t*)h_current_extended_omega;
        const scalar_t omega_scalar = *(const scalar_t*)h_omega;

        zkpcuda::quotient::cuda_kernel_permutation_quotient
            <<<kBlockNum, kTileSize, 0, stream>>>(
                (scalar_t*)d_values,
                (const scalar_t* const*)d_perm_prod_cosets,
                (const scalar_t* const*)d_perm_cosets,
                (const scalar_t* const*)d_column_values,
                (const scalar_t*)d_l0,
                (const scalar_t*)d_l_last,
                (const scalar_t*)d_l_active_row,
                beta_scalar,
                gamma_scalar,
                y_scalar,
                delta_scalar,
                delta_start_scalar,
                current_extended_omega_scalar,
                omega_scalar,
                n_sets,
                chunk_len,
                n_perm_cols,
                last_rotation,
                rot_scale,
                isize_,
                poly_length);
        CUDA_OK(cudaGetLastError());
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    }

    return cudaSuccess;
}
