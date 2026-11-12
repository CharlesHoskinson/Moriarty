// SPDX-License-Identifier: Apache-2.0
// Quotient numerator nu(X) on the extended coset, one thread per point (grid stride).
// It executes the stock midnight-proofs 0.8.3 `Evaluator` (plonk/evaluation.rs): the
// custom-gate graph, the permutation, lookup and trashcan terms, in the stock order of
// the `value = value * y + term` chain. Field arithmetic is exact, so each output
// element equals the CPU value bit for bit. Graph intermediates live in a global
// scratch area, slot-major: scratch[slot * stride + thread], slots reused by liveness
// (computed on the host). Column pointers may be host (unified memory) or device.
#pragma once

struct QSrc {
    uint32_t kind;  // 0 const, 1 interm, 2 fixed, 3 advice, 4 instance, 5 challenge,
                    // 6 beta, 7 gamma, 8 theta, 9 trash, 10 y, 11 previous
    uint32_t a;     // index (const, interm slot, column, challenge)
    int32_t rot;    // rotation (columns)
};

struct QCalc {
    uint32_t op;    // 0 add, 1 sub, 2 mul, 3 square, 4 double, 5 negate, 6 horner, 7 store
    uint32_t target;
    QSrc x, y;      // horner: x = start, y = factor
    uint32_t parts_off, parts_len;
};

struct QGraph {
    const QCalc* calcs;
    uint32_t ncalcs;
    uint32_t result_slot;   // slot of the last calculation (only read if ncalcs > 0)
    const QSrc* parts;
    const fr_t* constants;
};

struct QLookup {
    QGraph graph;
    const fr_t* product;
    const fr_t* permuted_input;
    const fr_t* permuted_table;
};

struct QTrash {
    QGraph graph;
    const fr_t* trash;
    uint32_t selector_col;
};

struct QuotientJob {
    fr_t* out;                 // n points (device or unified)
    uint64_t n;                // extended size
    int32_t rot_scale;
    const fr_t* const* fixed;
    const fr_t* const* advice;
    const fr_t* const* instance;
    const fr_t* challenges;
    // sc: beta, gamma, theta, trash challenge, y, delta_start (beta * g_coset),
    // delta (F::DELTA), extended omega
    const fr_t* sc;
    // gates
    QGraph gates;
    // permutation
    uint32_t nsets, chunk_len, ncols;
    const fr_t* const* perm_products;  // nsets
    const uint32_t* col_kind;          // ncols: 0 advice, 1 fixed, 2 instance
    const uint32_t* col_index;
    const fr_t* const* perm_sigma;     // ncols
    int32_t last_rot;
    const fr_t* l0;
    const fr_t* l_last;
    const fr_t* l_active;
    // lookups, trashcans
    uint32_t nlookups, ntrash;
    const QLookup* lookups;
    const QTrash* trash;
    // scratch
    fr_t* scratch;
    uint64_t stride;           // number of threads in the grid
};

// Loads with 16-byte alignment (host Vec<Fq> from malloc is 16-aligned; fr_t asks 32).
#ifdef __CUDA_ARCH__
__device__ __forceinline__ fr_t ldf(const fr_t* p)
{
    const uint4* q = reinterpret_cast<const uint4*>(p);
    uint4 a = q[0], b = q[1];
    fr_t r;
    r[0] = a.x; r[1] = a.y; r[2] = a.z; r[3] = a.w;
    r[4] = b.x; r[5] = b.y; r[6] = b.z; r[7] = b.w;
    return r;
}
#endif

#ifdef __CUDA_ARCH__
__device__ __forceinline__ void stf(fr_t* p, const fr_t& v)
{
    uint4* q = reinterpret_cast<uint4*>(p);
    q[0] = make_uint4(v[0], v[1], v[2], v[3]);
    q[1] = make_uint4(v[4], v[5], v[6], v[7]);
}
#endif

// A field element passed by value to a kernel (host and device fr_t differ in alignment).
struct F8 { uint32_t v[8]; };
#ifdef __CUDA_ARCH__
__device__ __forceinline__ fr_t f8(const F8& a)
{
    fr_t r;
    for (int i = 0; i < 8; i++) r[i] = a.v[i];
    return r;
}
#endif

__global__ void mori_distribute_zeta(fr_t* a, size_t n, F8 z1_, F8 z2_)
{
#ifdef __CUDA_ARCH__
    const fr_t z1 = f8(z1_), z2 = f8(z2_);
    for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n;
         i += (size_t)gridDim.x * blockDim.x) {
        uint32_t r = i % 3;
        if (r == 1) stf(&a[i], ldf(&a[i]) * z1);
        else if (r == 2) stf(&a[i], ldf(&a[i]) * z2);
    }
#endif
}

__global__ void mori_scale(fr_t* a, size_t n, F8 c_)
{
#ifdef __CUDA_ARCH__
    const fr_t c = f8(c_);
    for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n;
         i += (size_t)gridDim.x * blockDim.x)
        stf(&a[i], ldf(&a[i]) * c);
#endif
}

__global__ void mori_mul_periodic(fr_t* a, size_t n, const fr_t* t, size_t tn)
{
#ifdef __CUDA_ARCH__
    for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n;
         i += (size_t)gridDim.x * blockDim.x)
        stf(&a[i], ldf(&a[i]) * ldf(&t[i % tn]));
#endif
}

#ifdef __CUDA_ARCH__
__device__ __forceinline__ size_t q_rot(size_t idx, int32_t rot, int32_t rot_scale, uint64_t n)
{
    int64_t r = (int64_t)idx + (int64_t)rot * rot_scale;
    r %= (int64_t)n;
    if (r < 0) r += n;
    return (size_t)r;
}
#endif

#ifdef __CUDA_ARCH__
__device__ __forceinline__ fr_t q_get(const QuotientJob& J, const QGraph& g, const QSrc& s,
                                      size_t idx, size_t tid, const fr_t& prev)
{
    switch (s.kind) {
    case 0: return ldf(&g.constants[s.a]);
    case 1: return ldf(&J.scratch[(size_t)s.a * J.stride + tid]);
    case 2: return ldf(&J.fixed[s.a][q_rot(idx, s.rot, J.rot_scale, J.n)]);
    case 3: return ldf(&J.advice[s.a][q_rot(idx, s.rot, J.rot_scale, J.n)]);
    case 4: return ldf(&J.instance[s.a][q_rot(idx, s.rot, J.rot_scale, J.n)]);
    case 5: return ldf(&J.challenges[s.a]);
    case 6: return ldf(&J.sc[0]);
    case 7: return ldf(&J.sc[1]);
    case 8: return ldf(&J.sc[2]);
    case 9: return ldf(&J.sc[3]);
    case 10: return ldf(&J.sc[4]);
    default: return prev;
    }
}
#endif

#ifdef __CUDA_ARCH__
__device__ fr_t q_eval(const QuotientJob& J, const QGraph& g, size_t idx, size_t tid,
                       const fr_t& prev)
{
    fr_t v;
    for (uint32_t c = 0; c < g.ncalcs; c++) {
        const QCalc& k = g.calcs[c];
        switch (k.op) {
        case 0: v = q_get(J, g, k.x, idx, tid, prev) + q_get(J, g, k.y, idx, tid, prev); break;
        case 1: v = q_get(J, g, k.x, idx, tid, prev) - q_get(J, g, k.y, idx, tid, prev); break;
        case 2: v = q_get(J, g, k.x, idx, tid, prev) * q_get(J, g, k.y, idx, tid, prev); break;
        case 3: v = q_get(J, g, k.x, idx, tid, prev); v = v * v; break;
        case 4: v = q_get(J, g, k.x, idx, tid, prev); v = v + v; break;
        case 5: v = -q_get(J, g, k.x, idx, tid, prev); break;
        case 6: {
            fr_t f = q_get(J, g, k.y, idx, tid, prev);
            v = q_get(J, g, k.x, idx, tid, prev);
            for (uint32_t p = 0; p < k.parts_len; p++)
                v = v * f + q_get(J, g, g.parts[k.parts_off + p], idx, tid, prev);
            break;
        }
        default: v = q_get(J, g, k.x, idx, tid, prev); break;
        }
        stf(&J.scratch[(size_t)k.target * J.stride + tid], v);
    }
    if (g.ncalcs == 0) {
        return fr_t::one(1);
    }
    return ldf(&J.scratch[(size_t)g.result_slot * J.stride + tid]);
}
#endif

#ifdef __CUDA_ARCH__
__device__ __forceinline__ const fr_t* q_col(const QuotientJob& J, uint32_t c)
{
    switch (J.col_kind[c]) {
    case 0: return J.advice[J.col_index[c]];
    case 1: return J.fixed[J.col_index[c]];
    default: return J.instance[J.col_index[c]];
    }
}
#endif

#ifdef __CUDA_ARCH__
__device__ fr_t q_pow(fr_t b, uint64_t e)
{
    fr_t r = fr_t::one();
    while (e) {
        if (e & 1) r = r * b;
        b = b * b;
        e >>= 1;
    }
    return r;
}
#endif

__global__ void mori_quotient_kernel(const QuotientJob J)
{
#ifdef __CUDA_ARCH__
    const size_t tid = blockIdx.x * (size_t)blockDim.x + threadIdx.x;
    const fr_t one = fr_t::one();
    const fr_t beta = ldf(&J.sc[0]), gamma = ldf(&J.sc[1]), y = ldf(&J.sc[4]);
    const fr_t delta_start = ldf(&J.sc[5]), delta = ldf(&J.sc[6]), ext_omega = ldf(&J.sc[7]);
    for (size_t idx = tid; idx < J.n; idx += J.stride) {
        fr_t value = fr_t::one(1);
        // Custom gates: Horner(previous, parts, y), previous = 0.
        value = q_eval(J, J.gates, idx, tid, value);

        // Permutation.
        if (J.nsets) {
            const size_t r_next = q_rot(idx, 1, J.rot_scale, J.n);
            const size_t r_last = q_rot(idx, J.last_rot, J.rot_scale, J.n);
            const fr_t l0 = ldf(&J.l0[idx]), l_last = ldf(&J.l_last[idx]), l_act = ldf(&J.l_active[idx]);
            const fr_t* first = J.perm_products[0];
            const fr_t* last = J.perm_products[J.nsets - 1];
            value = value * y + ((one - ldf(&first[idx])) * l0);
            fr_t zl = ldf(&last[idx]);
            value = value * y + ((zl * zl - zl) * l_last);
            for (uint32_t s = 1; s < J.nsets; s++)
                value = value * y + ((ldf(&J.perm_products[s][idx]) - ldf(&J.perm_products[s - 1][r_last])) * l0);
            fr_t current_delta = delta_start * q_pow(ext_omega, idx);
            for (uint32_t s = 0; s < J.nsets; s++) {
                const fr_t* z = J.perm_products[s];
                uint32_t c0 = s * J.chunk_len;
                uint32_t c1 = min(c0 + J.chunk_len, J.ncols);
                fr_t left = ldf(&z[r_next]);
                for (uint32_t c = c0; c < c1; c++)
                    left = left * (ldf(&q_col(J, c)[idx]) + beta * ldf(&J.perm_sigma[c][idx]) + gamma);
                fr_t right = ldf(&z[idx]);
                for (uint32_t c = c0; c < c1; c++) {
                    right = right * (ldf(&q_col(J, c)[idx]) + current_delta + gamma);
                    current_delta = current_delta * delta;
                }
                value = value * y + ((left - right) * l_act);
            }
        }

        // Lookups.
        for (uint32_t n = 0; n < J.nlookups; n++) {
            const QLookup& L = J.lookups[n];
            const fr_t zero = fr_t::one(1);
            fr_t table_value = q_eval(J, L.graph, idx, tid, zero);
            const size_t r_next = q_rot(idx, 1, J.rot_scale, J.n);
            const size_t r_prev = q_rot(idx, -1, J.rot_scale, J.n);
            const fr_t l0 = ldf(&J.l0[idx]), l_last = ldf(&J.l_last[idx]), l_act = ldf(&J.l_active[idx]);
            fr_t pz = ldf(&L.product[idx]), pin = ldf(&L.permuted_input[idx]), ptab = ldf(&L.permuted_table[idx]);
            fr_t a_minus_s = pin - ptab;
            value = value * y + ((one - pz) * l0);
            value = value * y + ((pz * pz - pz) * l_last);
            value = value * y + ((ldf(&L.product[r_next]) * (pin + beta) * (ptab + gamma)
                                  - pz * table_value) * l_act);
            value = value * y + (a_minus_s * l0);
            value = value * y + (a_minus_s * (pin - ldf(&L.permuted_input[r_prev])) * l_act);
        }

        // Trashcans.
        for (uint32_t n = 0; n < J.ntrash; n++) {
            const QTrash& T = J.trash[n];
            const fr_t zero = fr_t::one(1);
            fr_t comp = q_eval(J, T.graph, idx, tid, zero);
            fr_t q = ldf(&J.fixed[T.selector_col][idx]);
            value = value * y + (comp - (one - q) * ldf(&T.trash[idx]));
        }

        stf(&J.out[idx], value);
    }
#endif
}

#ifndef __CUDA_ARCH__
static void mori_quotient_launch(const QuotientJob& J, stream_t& s)
{
    // stride = grid size; the host sized the scratch for it.
    uint32_t block = 256;
    uint32_t grid = (uint32_t)(J.stride / block);
    mori_quotient_kernel<<<grid, block, 0, s>>>(J);
}
#endif
