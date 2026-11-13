// SPDX-License-Identifier: Apache-2.0
// mori-gpu CUDA core: BLS12-381 G1 MSM and scalar-field NTT through sppark templates
// (Apache-2.0, third_party/sppark, see VENDORED.txt), plus the coset and quotient
// kernels of the wrap prover (quotient.cuh). Host types follow blst: Fr is 4 x u64
// Montgomery (R = 2^256), G1 Jacobian is blst_p1 (X, Y, Z in Fp Montgomery, R = 2^384).
#include <cuda.h>
#include <cstdio>
#include <cstring>
#include <cstdlib>
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
#include "poseidon.cuh"
#include "frac.cuh"

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

// ------------------------------------------- Midnight Poseidon over Fr (E4, poseidon.cuh)
// rc: 68 x 3 round constants, then the 3 x 3 MDS row-major, Montgomery (host fr layout).
// mode 0: plain rounds; mode 1: sparse partial rounds (needs the full POS_NCONST table).
extern "C" int mori_pos_set_constants(const fr_t* c, size_t n, int mode)
{
    if (n != POS_NCONST || (mode != 0 && mode != 1)) return fail("pos_set_constants", (int)n);
    (void)select_gpu(0);
    cudaError_t e = cudaMemcpyToSymbol(POS_C, c, n * sizeof(fr_t));
    if (e == cudaSuccess) e = cudaMemcpyToSymbol(POS_MODE, &mode, sizeof(int));
    const char* q = getenv("MORI_POS_SQR");
    int sq = (q && q[0] == '1') ? 1 : 0;
    if (e == cudaSuccess) e = cudaMemcpyToSymbol(POS_SQR, &sq, sizeof(int));
    return e == cudaSuccess ? 0 : fail("pos_set_constants", e);
}

// Threads per block of the Poseidon kernels: MORI_POS_BLOCK (32 to 1024), default 128.
static int pos_block()
{
    static int b = 0;
    if (!b) {
        const char* e = getenv("MORI_POS_BLOCK");
        int v = e ? atoi(e) : 128;
        b = (v >= 32 && v <= 1024 && v % 32 == 0) ? v : 128;
    }
    return b;
}

static int pos_grid(size_t n)
{
    size_t b = (n + pos_block() - 1) / pos_block();
    return (int)(b < 65535 * 4 ? b : 65535 * 4);
}

// Leaf hashes of n rows of width w (row-major, host), in chunks; out on the host.
extern "C" int mori_pos_hash_rows(fr_t* out, const fr_t* vals, size_t n, uint32_t w,
                                  const fr_t* cap)
{
    auto& gpu = select_gpu(0);
    const F8 c = *(const F8*)cap;
    size_t chunk = ((size_t)1 << 30) / (w * sizeof(fr_t));
    if (chunk > n) chunk = n;
    if (chunk == 0) chunk = 1;
    fr_t *dv = nullptr, *dout = nullptr;
    cudaError_t e = cudaMalloc(&dv, chunk * w * sizeof(fr_t));
    if (e == cudaSuccess) e = cudaMalloc(&dout, chunk * sizeof(fr_t));
    for (size_t off = 0; e == cudaSuccess && off < n; off += chunk) {
        size_t m = n - off < chunk ? n - off : chunk;
        e = cudaMemcpyAsync(dv, vals + off * w, m * w * sizeof(fr_t), cudaMemcpyHostToDevice, gpu);
        if (e != cudaSuccess) break;
        mori_pos_rows<<<pos_grid(m), pos_block(), 0, gpu>>>(dout, dv, m, w, c);
        e = cudaGetLastError();
        if (e != cudaSuccess) break;
        e = cudaMemcpyAsync(out + off, dout, m * sizeof(fr_t), cudaMemcpyDeviceToHost, gpu);
        if (e == cudaSuccess) e = cudaStreamSynchronize(gpu);
    }
    if (dv) cudaFree(dv);
    if (dout) cudaFree(dout);
    return e == cudaSuccess ? 0 : fail("pos_hash_rows", e);
}

// All digest layers of build_digest_layers_scalar: log2(rpq) strided rounds (query stride
// n / rpq), then halving to the root. out holds the layers after the strided rounds,
// concatenated (n / rpq, n / (2 rpq), ..., 1 entries; 2 n / rpq - 1 in total).
extern "C" int mori_pos_digest_layers(fr_t* out, const fr_t* leaves, size_t n, size_t rpq,
                                      const fr_t* cap)
{
    auto& gpu = select_gpu(0);
    const F8 c = *(const F8*)cap;
    size_t qs = n / rpq;
    fr_t *a = nullptr, *b = nullptr;
    cudaError_t e = cudaMalloc(&a, n * sizeof(fr_t));
    if (e == cudaSuccess) e = cudaMalloc(&b, (n / 2 + 1) * sizeof(fr_t));
    if (e == cudaSuccess) e = cudaMemcpyAsync(a, leaves, n * sizeof(fr_t), cudaMemcpyHostToDevice, gpu);
    size_t len = n;
    for (size_t s = rpq; e == cudaSuccess && s > 1; s >>= 1) {
        mori_pos_layer<<<pos_grid(len / 2), pos_block(), 0, gpu>>>(b, a, len / 2, qs, c);
        e = cudaGetLastError();
        fr_t* t = a; a = b; b = t;
        len /= 2;
    }
    size_t at = 0;
    while (e == cudaSuccess) {
        e = cudaMemcpyAsync(out + at, a, len * sizeof(fr_t), cudaMemcpyDeviceToHost, gpu);
        at += len;
        if (len == 1 || e != cudaSuccess) break;
        mori_pos_layer<<<pos_grid(len / 2), pos_block(), 0, gpu>>>(b, a, len / 2, 1, c);
        e = cudaGetLastError();
        fr_t* t = a; a = b; b = t;
        len /= 2;
    }
    if (e == cudaSuccess) e = cudaStreamSynchronize(gpu);
    if (a) cudaFree(a);
    if (b) cudaFree(b);
    return e == cudaSuccess ? 0 : fail("pos_digest_layers", e);
}

// Fused RS encode and leaf hash (E4): coeffs are w columns of h coefficients (host,
// column-major); each column is zero-padded to n = 2^lg_n and transformed in place with the
// forward NTT at the standard 2^lg_n root (natural order, the same values as
// p3 Radix2DitParallel), then the rows are hashed on the device. Outputs on the host: the
// row-major codeword rm (n x w) and the n leaf hashes.
extern "C" int mori_pos_encode_hash(fr_t* rm, fr_t* digests, const fr_t* coeffs, size_t h,
                                    uint32_t w, uint32_t lg_n, const fr_t* cap)
{
    auto& gpu = select_gpu(0);
    const F8 c = *(const F8*)cap;
    size_t n = (size_t)1 << lg_n;
    if (h > n || w == 0) return fail("pos_encode_hash", (int)h);
    // Low-memory streaming path when the GPU reads and writes pageable host memory
    // (coherent platforms such as this one); MORI_POS_STREAM=0 forces the resident path.
    int pageable = 0;
    cudaDeviceGetAttribute(&pageable, cudaDevAttrPageableMemoryAccess, 0);
    const char* sv = getenv("MORI_POS_STREAM");
    if (pageable && !(sv && sv[0] == '0')) {
        static bool said = false;
        if (!said) { fprintf(stderr, "mori-gpu: fused encode: streaming path (pageable host access)\n"); said = true; }
        fr_t *st = nullptr, *ca = nullptr, *cb = nullptr;
        try {
            CUDA_OK(cudaMalloc(&st, 3 * n * sizeof(fr_t)));
            CUDA_OK(cudaMalloc(&ca, n * sizeof(fr_t)));
            CUDA_OK(cudaMalloc(&cb, n * sizeof(fr_t)));
            mori_pos_init<<<pos_grid(n), pos_block(), 0, gpu>>>(st, n, c);
            CUDA_OK(cudaGetLastError());
            for (uint32_t j = 0; j < w; j += 2) {
                fr_t* bufs[2] = {ca, cb};
                for (uint32_t k = 0; k < 2 && j + k < w; k++) {
                    fr_t* col = bufs[k];
                    CUDA_OK(cudaMemcpyAsync(col, coeffs + (size_t)(j + k) * h, h * sizeof(fr_t),
                                            cudaMemcpyHostToDevice, gpu));
                    if (n > h) CUDA_OK(cudaMemsetAsync(col + h, 0, (n - h) * sizeof(fr_t), gpu));
                    MoriNTT::run(col, lg_n, false, gpu);
                    CUDA_OK(cudaGetLastError());
                }
                mori_pos_absorb<<<pos_grid(n), pos_block(), 0, gpu>>>(st, ca, j + 1 < w ? cb : nullptr,
                                                              n, rm, w, j);
                CUDA_OK(cudaGetLastError());
            }
            mori_pos_take0<<<pos_grid(n), pos_block(), 0, gpu>>>(ca, st, n);
            CUDA_OK(cudaGetLastError());
            CUDA_OK(cudaMemcpyAsync(digests, ca, n * sizeof(fr_t), cudaMemcpyDeviceToHost, gpu));
            CUDA_OK(cudaStreamSynchronize(gpu));
        } catch (const cuda_error& e) {
            if (st) cudaFree(st);
            if (ca) cudaFree(ca);
            if (cb) cudaFree(cb);
            return fail("pos_encode_hash_stream", e.code());
        }
        cudaFree(st); cudaFree(ca); cudaFree(cb);
        return 0;
    }
    fr_t *cols = nullptr, *dout = nullptr, *stage = nullptr;
    size_t m = ((size_t)1 << 28) / (w * sizeof(fr_t));
    if (m > n) m = n;
    try {
        CUDA_OK(cudaMalloc(&cols, n * w * sizeof(fr_t)));
        CUDA_OK(cudaMalloc(&dout, n * sizeof(fr_t)));
        CUDA_OK(cudaMalloc(&stage, m * w * sizeof(fr_t)));
        for (uint32_t j = 0; j < w; j++) {
            fr_t* col = cols + (size_t)j * n;
            CUDA_OK(cudaMemcpyAsync(col, coeffs + (size_t)j * h, h * sizeof(fr_t),
                                    cudaMemcpyHostToDevice, gpu));
            if (n > h) CUDA_OK(cudaMemsetAsync(col + h, 0, (n - h) * sizeof(fr_t), gpu));
            MoriNTT::run(col, lg_n, false, gpu);
            CUDA_OK(cudaGetLastError());
        }
        mori_pos_rows_cm<<<pos_grid(n), pos_block(), 0, gpu>>>(dout, cols, n, w, c);
        CUDA_OK(cudaGetLastError());
        CUDA_OK(cudaMemcpyAsync(digests, dout, n * sizeof(fr_t), cudaMemcpyDeviceToHost, gpu));
        for (size_t r0 = 0; r0 < n; r0 += m) {
            size_t mm = n - r0 < m ? n - r0 : m;
            mori_cm_to_rm<<<pos_grid(mm * w), pos_block(), 0, gpu>>>(stage, cols, n, w, r0, mm);
            CUDA_OK(cudaGetLastError());
            CUDA_OK(cudaMemcpyAsync(rm + r0 * w, stage, mm * w * sizeof(fr_t),
                                    cudaMemcpyDeviceToHost, gpu));
        }
        CUDA_OK(cudaStreamSynchronize(gpu));
    } catch (const cuda_error& e) {
        if (cols) cudaFree(cols);
        if (dout) cudaFree(dout);
        if (stage) cudaFree(stage);
        return fail("pos_encode_hash", e.code());
    }
    cudaFree(cols);
    cudaFree(dout);
    cudaFree(stage);
    return 0;
}

// ------------------------------------------- Fractional sumcheck over Fr (E4, frac.cuh)
static int frac_grid(size_t n)
{
    size_t b = (n + FRAC_BLOCK - 1) / FRAC_BLOCK;
    return (int)(b < 2048 ? (b ? b : 1) : 2048);
}

// Internal nodes of the segment tree: tree (host, 2^(N+1) fractions, interleaved p, q) holds
// the leaves at [2^N, 2^(N+1)); nodes [1, 2^N) are written back.
extern "C" int mori_frac_tree(fr_t* tree, uint32_t lg)
{
    auto& gpu = select_gpu(0);
    const size_t n = (size_t)1 << lg;
    fr_t* T = nullptr;
    try {
        CUDA_OK(cudaMalloc(&T, 4 * n * sizeof(fr_t)));
        CUDA_OK(cudaMemcpyAsync(T + 2 * n, tree + 2 * n, 2 * n * sizeof(fr_t),
                                cudaMemcpyHostToDevice, gpu));
        for (size_t lo = n / 2; lo >= 1; lo /= 2) {
            mori_frac_tree_layer<<<frac_grid(lo), FRAC_BLOCK, 0, gpu>>>(T, lo);
            CUDA_OK(cudaGetLastError());
        }
        CUDA_OK(cudaMemcpyAsync(tree + 2, T + 2, (2 * n - 2) * sizeof(fr_t),
                                cudaMemcpyDeviceToHost, gpu));
        CUDA_OK(cudaStreamSynchronize(gpu));
    } catch (const cuda_error& e) {
        if (T) cudaFree(T);
        return fail("frac_tree", e.code());
    }
    cudaFree(T);
    return 0;
}

// One GKR layer sumcheck. seg: the 2 es fractions of the layer (interleaved), eq: es values.
struct FracLayer {
    fr_t *seg, *w[2], *part, *out;
    FracCols c;
    size_t m;      // current number of points
    int cur;       // buffer holding the current columns (-1: seg and eq)
};

extern "C" void* mori_frac_layer_begin(const fr_t* seg, const fr_t* eq, size_t es)
{
    auto& gpu = select_gpu(0);
    FracLayer* L = new FracLayer();
    try {
        CUDA_OK(cudaMalloc(&L->seg, 5 * es * sizeof(fr_t)));
        CUDA_OK(cudaMalloc(&L->w[0], 5 * (es / 2 + 1) * sizeof(fr_t)));
        CUDA_OK(cudaMalloc(&L->w[1], 5 * (es / 4 + 1) * sizeof(fr_t)));
        CUDA_OK(cudaMalloc(&L->part, 3 * 2048 * sizeof(fr_t)));
        CUDA_OK(cudaMalloc(&L->out, 3 * sizeof(fr_t)));
        fr_t* eqd = L->seg + 4 * es;
        CUDA_OK(cudaMemcpyAsync(L->seg, seg, 4 * es * sizeof(fr_t), cudaMemcpyHostToDevice, gpu));
        CUDA_OK(cudaMemcpyAsync(eqd, eq, es * sizeof(fr_t), cudaMemcpyHostToDevice, gpu));
        // Column x of p_j0 is seg[2x].p: fr index 4x; q_j0 4x + 1; p_j1 4x + 2; q_j1 4x + 3.
        L->c.c[0] = eqd; L->c.s[0] = 1;
        for (int j = 0; j < 4; j++) { L->c.c[1 + j] = L->seg + j; L->c.s[1 + j] = 4; }
        L->m = es;
        L->cur = -1;
        CUDA_OK(cudaStreamSynchronize(gpu));
    } catch (const cuda_error& e) {
        cudaFree(L->seg); cudaFree(L->w[0]); cudaFree(L->w[1]); cudaFree(L->part);
        cudaFree(L->out);
        delete L;
        fail("frac_layer_begin", e.code());
        return nullptr;
    }
    return L;
}

extern "C" int mori_frac_layer_round(void* ctx, const fr_t* lambda, fr_t* out3)
{
    auto& gpu = select_gpu(0);
    FracLayer* L = (FracLayer*)ctx;
    const size_t half = L->m / 2;
    const int nb = frac_grid(half);
    try {
        mori_frac_eval<<<nb, FRAC_BLOCK, 0, gpu>>>(L->part, L->c, half, *(const F8*)lambda);
        CUDA_OK(cudaGetLastError());
        mori_frac_sum<<<1, FRAC_BLOCK, 0, gpu>>>(L->out, L->part, nb);
        CUDA_OK(cudaGetLastError());
        CUDA_OK(cudaMemcpyAsync(out3, L->out, 3 * sizeof(fr_t), cudaMemcpyDeviceToHost, gpu));
        CUDA_OK(cudaStreamSynchronize(gpu));
    } catch (const cuda_error& e) {
        return fail("frac_layer_round", e.code());
    }
    return 0;
}

extern "C" int mori_frac_layer_fold(void* ctx, const fr_t* r)
{
    auto& gpu = select_gpu(0);
    FracLayer* L = (FracLayer*)ctx;
    const size_t half = L->m / 2;
    // Ping-pong: w[0] holds sizes es/2, es/8, ...; w[1] holds es/4, es/16, ...
    const int nxt = L->cur == 0 ? 1 : 0;
    fr_t* dst = L->w[nxt];
    try {
        mori_frac_fold<<<frac_grid(half), FRAC_BLOCK, 0, gpu>>>(dst, L->c, half, *(const F8*)r);
        CUDA_OK(cudaGetLastError());
    } catch (const cuda_error& e) {
        return fail("frac_layer_fold", e.code());
    }
    for (int j = 0; j < 5; j++) { L->c.c[j] = dst + j * half; L->c.s[j] = 1; }
    L->m = half;
    L->cur = nxt;
    return 0;
}

// claims4 = (p_j0, q_j0, p_j1, q_j1) at the folded point (m = 1); frees the layer.
extern "C" int mori_frac_layer_end(void* ctx, fr_t* claims4)
{
    auto& gpu = select_gpu(0);
    FracLayer* L = (FracLayer*)ctx;
    cudaError_t e = cudaSuccess;
    if (claims4) {
        for (int j = 0; j < 4 && e == cudaSuccess; j++)
            e = cudaMemcpyAsync(claims4 + j, L->c.c[1 + j], sizeof(fr_t), cudaMemcpyDeviceToHost,
                                gpu);
        if (e == cudaSuccess) e = cudaStreamSynchronize(gpu);
    }
    cudaFree(L->seg); cudaFree(L->w[0]); cudaFree(L->w[1]); cudaFree(L->part); cudaFree(L->out);
    delete L;
    return e == cudaSuccess ? 0 : fail("frac_layer_end", e);
}

#endif
