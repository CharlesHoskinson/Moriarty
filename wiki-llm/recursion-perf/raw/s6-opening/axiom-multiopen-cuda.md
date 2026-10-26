# Axiom multiopen CUDA kernels
> Source: https://raw.githubusercontent.com/axiom-crypto/halo2-gpu/b2d4bcfd4261cc506b5412fb04fe631f028849d7/halo2_proofs/cuda/src/multiopen.cu
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

#include "kernel/polynomial.h"

using Scalar = utils::FFITraitObject;

// multiopen
class CudaMultiopenCalculationInfo {
public:
    CudaMultiopenCalculationInfo(uint64_t poly_length, uint64_t batch_size)
    {
        this->poly_length = poly_length;
        this->batch_size = batch_size;
        this->poly_size = poly_length * Scalar::ELT_BYTES;
    }

    uint64_t get_required_memory_size()
    {
        uint64_t required_memory_size = 0;
        // in/out
        required_memory_size += poly_size * 2; // poly_in[0] and poly_in[1] for double buffering
        required_memory_size += poly_size; // poly_acc
        required_memory_size += batch_size * Scalar::ELT_BYTES; // eval_point
        required_memory_size += batch_size * Scalar::ELT_BYTES; // eval_result
        required_memory_size += batch_size * Scalar::ELT_BYTES; // challenge point
        // temp
        required_memory_size += (eval_tile_size + eval_block_num) * Scalar::ELT_BYTES; // power_lut
        required_memory_size += eval_block_num * Scalar::ELT_BYTES; // block_res
        return required_memory_size;
    }

    uint64_t poly_length = 0;
    uint64_t batch_size = 0;
    uint64_t poly_size = 0;
    static const uint32_t eval_tile_size = 64;
    static const uint32_t eval_block_num = 256;
};

extern "C" uint64_t _halo2_multiopen_poly_max_len(
    uint64_t poly_length,
    uint64_t batch_size,
    uint64_t free_bytes)
{
    for (uint64_t _length = poly_length; _length > 0; _length = _length >> 1) {
        CudaMultiopenCalculationInfo multiopen_info(_length, batch_size);
        if (multiopen_info.get_required_memory_size() < free_bytes) {
            return _length;
        }
    }
    return 0;
}

// Pure host preflight for `_halo2_multiopen_poly_calculation`. Note: the
// existing `_halo2_multiopen_poly_max_len` returns elements (the largest
// poly_length that fits in free GPU memory) — semantically different and
// kept unchanged. This new entry returns bytes for the ScratchSpan.
extern "C" uint64_t _halo2_multiopen_poly_calculation_workspace_size(
    uint64_t poly_length,
    uint64_t batch_size)
{
    CudaMultiopenCalculationInfo info(poly_length, batch_size);
    return align_up(info.get_required_memory_size(), 32);
}

// calculation:
//     poly_acc = poly_acc * point_v + poly;
//     eval_res = poly.eval(point_e);
extern "C" RustError _halo2_multiopen_poly_calculation(
    const Scalar* poly_in_many, // input:  vec<poly>
    const Scalar* poly_acc, // output: init with 0
    uint64_t poly_offset, // offset of poly_acc in poly_in
    uint64_t poly_length,
    uint64_t batch_size,
    const Scalar* challenge_point, // input:  vec<scalar>
    const Scalar* eval_point, // input:  vec<scalar>
    const Scalar* eval_result, // output: vec<scalar>
    void* scratch,
    uint64_t scratch_bytes,
    cudaStream_t stream)
{
    ScratchSpan span { (uint8_t*)scratch, (size_t)scratch_bytes };
    // input/output
    uint64_t* d_poly_in[2] = { nullptr }; // vec<poly>, double buffer
    uint64_t* d_poly_acc = nullptr; // vec<poly>
    uint64_t* d_challenge_point = nullptr; // vec<scalar>
    uint64_t* d_eval_point = nullptr; // vec<scalar>
    uint64_t* d_eval_result = nullptr; // vec<scalar>
    // temp buffer
    uint64_t* d_eval_pow_lut = nullptr;
    uint64_t* d_eval_block_res = nullptr;

    CudaMultiopenCalculationInfo multiopen_info(poly_length, batch_size);
    const uint32_t poly_size = multiopen_info.poly_size;
    const uint32_t eval_tile_size = CudaMultiopenCalculationInfo::eval_tile_size;
    const uint32_t eval_block_num = CudaMultiopenCalculationInfo::eval_block_num;
    uint64_t required_memory_size = multiopen_info.get_required_memory_size();
    uint64_t* d_gpu_mem = (uint64_t*)span.take(required_memory_size);

    // init
    try {
        // in/out
        d_poly_in[0] = d_gpu_mem;
        d_poly_in[1] = (uint64_t*)((char*)(d_poly_in[0]) + poly_size);
        d_poly_acc = (uint64_t*)((char*)(d_poly_in[1]) + poly_size);
        d_eval_point = (uint64_t*)((char*)d_poly_acc + poly_size);
        d_eval_result = (uint64_t*)((char*)d_eval_point + batch_size * Scalar::ELT_BYTES);
        d_challenge_point = (uint64_t*)((char*)d_eval_result + batch_size * Scalar::ELT_BYTES);
        // temp
        d_eval_pow_lut = (uint64_t*)((char*)d_challenge_point + batch_size * Scalar::ELT_BYTES);
        d_eval_block_res = (uint64_t*)((char*)d_eval_pow_lut + (eval_tile_size + eval_block_num) * Scalar::ELT_BYTES);
        CUDA_OK(cudaMemsetAsync(d_poly_acc, 0, poly_size, stream)); // init, set 0
        CUDA_OK(cudaMemcpyAsync(d_eval_point, eval_point->ptr, batch_size * Scalar::ELT_BYTES, cudaMemcpyHostToDevice, stream));
        CUDA_OK(cudaMemcpyAsync(d_challenge_point, challenge_point->ptr, batch_size * Scalar::ELT_BYTES, cudaMemcpyHostToDevice, stream));
        uint64_t* h_poly_in_ptr = poly_in_many[0].ptr + Scalar::ELT_LIMBS * poly_offset;
        CUDA_OK(cudaMemcpyAsync(d_poly_in[0], h_poly_in_ptr, poly_size, cudaMemcpyHostToDevice, stream));
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    for (uint32_t i = 0; i < batch_size; ++i) {
        try {
            // poly eval
            uint64_t* d_eval_point_ = (uint64_t*)((char*)d_eval_point + i * Scalar::ELT_BYTES);
            uint64_t* d_eval_result_ = (uint64_t*)((char*)d_eval_result + i * Scalar::ELT_BYTES);
            zkpcuda::polynomial::power_of_scalar_init<eval_tile_size><<<1, 1, 0, stream>>>(
                (const scalar_t*)d_eval_point_,
                (scalar_t*)d_eval_pow_lut);
            zkpcuda::polynomial::power_of_scalar_block<eval_tile_size><<<1, 1, 0, stream>>>(
                (scalar_t*)d_eval_pow_lut,
                eval_block_num);
            zkpcuda::polynomial::eval_polynomial_batch<eval_tile_size><<<eval_block_num, eval_tile_size, 0, stream>>>(
                (const scalar_t*)d_poly_in[i & 1],
                (const scalar_t*)d_eval_pow_lut,
                (scalar_t*)d_eval_block_res,
                poly_length);
            zkpcuda::polynomial::eval_polynomial_epilogue<<<1, 1, 0, stream>>>(
                (scalar_t*)d_eval_result_,
                (const scalar_t*)d_eval_block_res,
                eval_block_num);
            // poly multiply_add
            uint64_t* d_challenge_point_ = (uint64_t*)((char*)d_challenge_point + i * Scalar::ELT_BYTES);
            zkpcuda::polynomial::poly_multiply_add<<<512, 64 * 1, 0, stream>>>(
                (scalar_t*)d_poly_acc,
                (const scalar_t*)d_poly_in[i & 1],
                (const scalar_t*)d_challenge_point_,
                poly_length);
            if (i + 1 < batch_size) {
                uint64_t* h_poly_in_ptr = poly_in_many[i + 1].ptr + Scalar::ELT_LIMBS * poly_offset;
                CUDA_OK(cudaMemcpyAsync(d_poly_in[(i + 1) & 1], h_poly_in_ptr, poly_size, cudaMemcpyHostToDevice, stream));
            }
        } catch (const cuda_error& error) {
            return RustError(error.code(), error.what());
        };
    }

    // copy to host
    try {
        uint64_t* h_poly_acc_ptr = poly_acc->ptr + Scalar::ELT_LIMBS * poly_offset;
        CUDA_OK(cudaMemcpyAsync(h_poly_acc_ptr, d_poly_acc, poly_size, cudaMemcpyDeviceToHost, stream));
        CUDA_OK(cudaMemcpyAsync(eval_result->ptr, d_eval_result, batch_size * Scalar::ELT_BYTES, cudaMemcpyDeviceToHost, stream));
        CUDA_OK(cudaStreamSynchronize(stream));
    } catch (const cuda_error& error) {
        return RustError(error.code(), error.what());
    };

    return cudaSuccess;
}

```
