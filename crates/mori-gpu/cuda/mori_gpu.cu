// SPDX-License-Identifier: Apache-2.0
// mori-gpu CUDA core: BLS12-381 G1 MSM and scalar-field NTT through sppark templates
// (Apache-2.0, third_party/sppark, see VENDORED.txt), plus the coset and quotient
// kernels of the wrap prover (quotient.cuh). Host types follow blst: Fr is 4 x u64
// Montgomery (R = 2^256), G1 Jacobian is blst_p1 (X, Y, Z in Fp Montgomery, R = 2^384).
#include <cuda.h>
#include <cstdio>
#include <cstring>
#include <mutex>
#include <map>

#include <ff/bls12-381.hpp>
#include <ec/jacobian_t.hpp>
#include <ec/xyzz_t.hpp>

typedef jacobian_t<fp_t> point_t;
typedef xyzz_t<fp_t> bucket_t;
typedef bucket_t::affine_t affine_t;
typedef fr_t scalar_t;

#include <util/all_gpus.cpp>
#include <msm/pippenger.cuh>
#include <ntt/ntt.cuh>

#include "quotient.cuh"

#ifndef __CUDA_ARCH__

static thread_local char g_err[256];

static int fail(const char* what, int code)
{
    snprintf(g_err, sizeof g_err, "%s: code %d", what, code);
    return code ? code : -1;
}

extern "C" const char* mori_gpu_last_error() { return g_err; }

// Device count with cooperative launch (what sppark needs); 0 when no usable device.
extern "C" int mori_gpu_device_count(char* name, size_t name_len, int* sm_count,
                                     int* major, int* minor)
{
    int n = 0;
    if (cudaGetDeviceCount(&n) != cudaSuccess || n == 0)
        return 0;
    cudaDeviceProp prop;
    if (cudaGetDeviceProperties(&prop, 0) != cudaSuccess)
        return 0;
    if (name && name_len) {
        strncpy(name, prop.name, name_len - 1);
        name[name_len - 1] = 0;
    }
    if (sm_count) *sm_count = prop.multiProcessorCount;
    if (major) *major = prop.major;
    if (minor) *minor = prop.minor;
    try {
        (void)select_gpu(0);
    } catch (const cuda_error& e) {
        return 0;
    }
    return n;
}

extern "C" int mori_gpu_mem_info(size_t* free_b, size_t* total_b)
{
    return cudaMemGetInfo(free_b, total_b) == cudaSuccess ? 0 : -1;
}

// ---------------------------------------------------------------- MSM
// One-shot MSM: affine bases on the host (copied per call), Montgomery scalars.
extern "C" int mori_msm_g1(point_t* out, const affine_t* points, size_t npoints,
                           const scalar_t* scalars)
{
    RustError r = mult_pippenger<bucket_t>(out, points, npoints, scalars, true);
    if (r.code != cudaSuccess)
        return fail("msm", r.code);
    return 0;
}

// Cached base table: the affine bases are uploaded once (SRS g or g_lagrange).
typedef msm_t<bucket_t, point_t, affine_t, scalar_t> msm_ctx_t;

extern "C" void* mori_msm_ctx_new(const affine_t* points, size_t npoints)
{
    try {
        return new msm_ctx_t(points, npoints);
    } catch (const cuda_error& e) {
        fail("msm_ctx_new", e.code());
        return nullptr;
    }
}

extern "C" void mori_msm_ctx_free(void* ctx)
{
    delete reinterpret_cast<msm_ctx_t*>(ctx);
}

extern "C" int mori_msm_ctx_run(void* ctx, point_t* out, const scalar_t* scalars,
                                size_t nscalars)
{
    auto* m = reinterpret_cast<msm_ctx_t*>(ctx);
    try {
        RustError r = m->invoke(*out, (const affine_t*)nullptr, nscalars, scalars, true);
        if (r.code != cudaSuccess)
            return fail("msm_ctx_run", r.code);
    } catch (const cuda_error& e) {
        return fail("msm_ctx_run", e.code());
    }
    return 0;
}

// ---------------------------------------------------------------- NTT
// sppark keeps NTT_internal protected; this subclass runs it on a device buffer.
struct MoriNTT : public NTT {
    static void run(fr_t* d, uint32_t lg, bool inverse, stream_t& s)
    {
        NTT_internal(d, lg, InputOutputOrder::NN,
                     inverse ? Direction::inverse : Direction::forward,
                     Type::standard, s);
    }
};

// Host-to-host transform in natural order. Forward: no scaling (best_fft with the
// standard 2^lg root). Inverse: omega^-1 and the 1/n scaling (EvaluationDomain::ifft).
extern "C" int mori_ntt(fr_t* inout, uint32_t lg, int inverse)
{
    RustError r = NTT::Base(select_gpu(0), inout, lg, NTT::InputOutputOrder::NN,
                            inverse ? NTT::Direction::inverse : NTT::Direction::forward,
                            NTT::Type::standard);
    if (r.code != cudaSuccess)
        return fail("ntt", r.code);
    return 0;
}

// ------------------------------------------------- device buffers (resident polys)
extern "C" void* mori_dev_alloc(size_t bytes)
{
    void* p = nullptr;
    if (cudaMalloc(&p, bytes) != cudaSuccess) {
        fail("cudaMalloc", (int)bytes);
        return nullptr;
    }
    return p;
}

extern "C" void mori_dev_free(void* p) { if (p) cudaFree(p); }

extern "C" int mori_htod(void* dst, const void* src, size_t bytes)
{
    auto& gpu = select_gpu(0);
    cudaError_t e = cudaMemcpyAsync(dst, src, bytes, cudaMemcpyHostToDevice, gpu);
    if (e == cudaSuccess) e = cudaStreamSynchronize(gpu);
    return e == cudaSuccess ? 0 : fail("htod", e);
}

extern "C" int mori_dtoh(void* dst, const void* src, size_t bytes)
{
    auto& gpu = select_gpu(0);
    cudaError_t e = cudaMemcpyAsync(dst, src, bytes, cudaMemcpyDeviceToHost, gpu);
    if (e == cudaSuccess) e = cudaStreamSynchronize(gpu);
    return e == cudaSuccess ? 0 : fail("dtoh", e);
}

extern "C" int mori_dev_sync()
{
    auto& gpu = select_gpu(0);
    cudaError_t e = cudaStreamSynchronize(gpu);
    if (e == cudaSuccess) e = cudaGetLastError();
    return e == cudaSuccess ? 0 : fail("sync", e);
}

// NTT in place on a device buffer (no copies).
extern "C" int mori_ntt_dev(fr_t* d, uint32_t lg, int inverse)
{
    auto& gpu = select_gpu(0);
    try {
        MoriNTT::run(d, lg, inverse != 0, gpu);
        CUDA_OK(cudaGetLastError());
    } catch (const cuda_error& e) {
        return fail("ntt_dev", e.code());
    }
    return 0;
}

// coeff_to_extended on device: dst (2^ext_k) = FFT_ext(zeta-distribute(src (2^k)) || 0).
// src and dst may be host (unified) or device pointers; dst must be a device buffer.
extern "C" int mori_coeff_to_extended_dev(fr_t* d_dst, const fr_t* src, uint32_t k,
                                          uint32_t ext_k, const fr_t* zeta2)
{
    auto& gpu = select_gpu(0);
    size_t n = (size_t)1 << k, en = (size_t)1 << ext_k;
    try {
        CUDA_OK(cudaMemcpyAsync(d_dst, src, n * sizeof(fr_t), cudaMemcpyDefault, gpu));
        CUDA_OK(cudaMemsetAsync(d_dst + n, 0, (en - n) * sizeof(fr_t), gpu));
        mori_distribute_zeta<<<(n + 255) / 256, 256, 0, gpu>>>(d_dst, n, ((const F8*)zeta2)[0], ((const F8*)zeta2)[1]);
        CUDA_OK(cudaGetLastError());
        MoriNTT::run(d_dst, ext_k, false, gpu);
        CUDA_OK(cudaGetLastError());
    } catch (const cuda_error& e) {
        return fail("coeff_to_extended_dev", e.code());
    }
    return 0;
}

// Elementwise helpers on device buffers.
extern "C" int mori_distribute_zeta_dev(fr_t* d, size_t n, const fr_t* zeta2)
{
    auto& gpu = select_gpu(0);
    mori_distribute_zeta<<<(n + 255) / 256, 256, 0, gpu>>>(d, n, ((const F8*)zeta2)[0], ((const F8*)zeta2)[1]);
    cudaError_t e = cudaGetLastError();
    return e == cudaSuccess ? 0 : fail("distribute_zeta", e);
}

extern "C" int mori_scale_dev(fr_t* d, size_t n, const fr_t* c)
{
    auto& gpu = select_gpu(0);
    mori_scale<<<(n + 255) / 256, 256, 0, gpu>>>(d, n, *(const F8*)c);
    cudaError_t e = cudaGetLastError();
    return e == cudaSuccess ? 0 : fail("scale", e);
}

// h[i] *= t[i % tn] (divide_by_vanishing_poly with precomputed inverse evaluations)
extern "C" int mori_mul_periodic_dev(fr_t* d, size_t n, const fr_t* t, size_t tn)
{
    auto& gpu = select_gpu(0);
    fr_t* dt = nullptr;
    cudaError_t e = cudaMallocAsync(&dt, tn * sizeof(fr_t), gpu);
    if (e == cudaSuccess) e = cudaMemcpyAsync(dt, t, tn * sizeof(fr_t), cudaMemcpyDefault, gpu);
    if (e != cudaSuccess) return fail("mul_periodic", e);
    mori_mul_periodic<<<(n + 255) / 256, 256, 0, gpu>>>(d, n, dt, tn);
    e = cudaGetLastError();
    cudaFreeAsync(dt, gpu);
    return e == cudaSuccess ? 0 : fail("mul_periodic", e);
}

// Quotient numerator: see quotient.cuh for the job layout.
extern "C" int mori_quotient_run(const QuotientJob* job)
{
    auto& gpu = select_gpu(0);
    try {
        mori_quotient_launch(*job, gpu);
        CUDA_OK(cudaGetLastError());
    } catch (const cuda_error& e) {
        return fail("quotient", e.code());
    }
    return 0;
}

#endif
