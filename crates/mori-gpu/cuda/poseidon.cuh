// SPDX-License-Identifier: Apache-2.0
// Midnight Poseidon over BLS12-381 Fr (width 3, rate 2, 8 full and 60 partial rounds, x^5),
// on the sppark fr_t (8 x 32-bit Montgomery limbs, kernel K0). The permutation is the
// unskipped form of midnight-circuits 7.3.0 `permutation_cpu`: add RC[0]; then for each
// round r = 0..67 the S-box (all cells in full rounds, cell 2 in partial rounds), then
// state = MDS * state + RC[r + 1] (0 after the last round). The round-skip form of the CPU
// code is an algebraic rewrite of the same map. Constants come from the host
// (`PoseidonField::ROUND_CONSTANTS` and `MDS` of midnight-circuits), never transcribed.
//
// Leaf hash (stark-sdk `FrMerkleHasher::hash_slice`): state (0, 0, cap), each block of 2
// values added into (s0, s1) (a last block of 1 adds only s0), one permutation per block,
// digest s0. Node hash (`compress`): state (left, right, cap), one permutation, digest s0.

#define POS_ROUNDS 68
#define POS_FULL_HALF 4
#define POS_PARTIAL 60
// Constant table: RC (68 x 3), MDS (9), then the sparse partial-round table (60 x 6:
// a, v0, v1, w0, w1, k), the final 2 x 2 block (4) and the carried constants (2).
#define POS_BASE (POS_ROUNDS * 3 + 9)
#define POS_NCONST (POS_BASE + POS_PARTIAL * 6 + 6)

__constant__ uint32_t POS_C[POS_NCONST * 8];
// 0: plain rounds (the definition); 1: sparse partial rounds (default).
__constant__ int POS_MODE;

#ifdef __CUDA_ARCH__
__device__ __forceinline__ fr_t pos_c(int i)
{
    fr_t r;
    for (int j = 0; j < 8; j++) r[j] = POS_C[i * 8 + j];
    return r;
}

__device__ __forceinline__ fr_t pow5(const fr_t& x)
{
    fr_t x2 = x * x;
    fr_t x4 = x2 * x2;
    return x4 * x;
}

// One plain round r: S-box (all cells when full, cell 2 otherwise), MDS, RC[r + 1].
__device__ __forceinline__ void pos_round(int r, bool full, fr_t& s0, fr_t& s1, fr_t& s2)
{
    const int M = POS_ROUNDS * 3;
    if (full) { s0 = pow5(s0); s1 = pow5(s1); }
    s2 = pow5(s2);
    fr_t n0 = pos_c(M + 0) * s0 + pos_c(M + 1) * s1 + pos_c(M + 2) * s2;
    fr_t n1 = pos_c(M + 3) * s0 + pos_c(M + 4) * s1 + pos_c(M + 5) * s2;
    fr_t n2 = pos_c(M + 6) * s0 + pos_c(M + 7) * s1 + pos_c(M + 8) * s2;
    if (r + 1 < POS_ROUNDS) {
        n0 += pos_c(3 * (r + 1) + 0);
        n1 += pos_c(3 * (r + 1) + 1);
        n2 += pos_c(3 * (r + 1) + 2);
    }
    s0 = n0; s1 = n1; s2 = n2;
}

// Sparse partial rounds (Poseidon paper appendix B, built on the host by stark-sdk
// `gpu::constants`): in coordinates t = (s2, s0, s1) the round matrix of round k is
// M'_k M''_k with M'_k = diag(1, A_k) pushed into the next round and
// M''_k = [[a, v0, v1], [w0, 1, 0], [w1, 0, 1]]; the round constants of t1, t2 are carried
// forward and only k_k is added to t0. After round 60: (t1, t2) = A_60 (t1, t2) + carry.
__device__ __forceinline__ void pos_partial_sparse(fr_t& s0, fr_t& s1, fr_t& s2)
{
    fr_t t0 = s2, t1 = s0, t2 = s1;
    #pragma unroll 1
    for (int k = 0; k < POS_PARTIAL; k++) {
        const int b = POS_BASE + 6 * k;
        t0 = pow5(t0);
        fr_t n0 = pos_c(b + 0) * t0 + pos_c(b + 1) * t1 + pos_c(b + 2) * t2 + pos_c(b + 5);
        t1 += pos_c(b + 3) * t0;
        t2 += pos_c(b + 4) * t0;
        t0 = n0;
    }
    const int f = POS_BASE + 6 * POS_PARTIAL;
    fr_t u1 = pos_c(f + 0) * t1 + pos_c(f + 1) * t2 + pos_c(f + 4);
    fr_t u2 = pos_c(f + 2) * t1 + pos_c(f + 3) * t2 + pos_c(f + 5);
    s2 = t0; s0 = u1; s1 = u2;
}

__device__ __forceinline__ void pos_permute(fr_t& s0, fr_t& s1, fr_t& s2)
{
    s0 += pos_c(0); s1 += pos_c(1); s2 += pos_c(2);
    if (POS_MODE == 1) {
        #pragma unroll 1
        for (int r = 0; r < POS_FULL_HALF; r++) pos_round(r, true, s0, s1, s2);
        pos_partial_sparse(s0, s1, s2);
        #pragma unroll 1
        for (int r = POS_ROUNDS - POS_FULL_HALF; r < POS_ROUNDS; r++) pos_round(r, true, s0, s1, s2);
        return;
    }
    const int M = POS_ROUNDS * 3;
    #pragma unroll 1
    for (int r = 0; r < POS_ROUNDS; r++) {
        bool full = r < POS_FULL_HALF || r >= POS_ROUNDS - POS_FULL_HALF;
        if (full) { s0 = pow5(s0); s1 = pow5(s1); }
        s2 = pow5(s2);
        fr_t n0 = pos_c(M + 0) * s0 + pos_c(M + 1) * s1 + pos_c(M + 2) * s2;
        fr_t n1 = pos_c(M + 3) * s0 + pos_c(M + 4) * s1 + pos_c(M + 5) * s2;
        fr_t n2 = pos_c(M + 6) * s0 + pos_c(M + 7) * s1 + pos_c(M + 8) * s2;
        if (r + 1 < POS_ROUNDS) {
            n0 += pos_c(3 * (r + 1) + 0);
            n1 += pos_c(3 * (r + 1) + 1);
            n2 += pos_c(3 * (r + 1) + 2);
        }
        s0 = n0; s1 = n1; s2 = n2;
    }
}
#endif

// out[i] = leaf hash of row i of the row-major matrix vals (n rows, width w).
__global__ void mori_pos_rows(fr_t* out, const fr_t* vals, size_t n, uint32_t w, F8 cap_)
{
#ifdef __CUDA_ARCH__
    const fr_t cap = f8(cap_);
    for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n;
         i += (size_t)gridDim.x * blockDim.x) {
        const fr_t* row = vals + i * w;
        fr_t s0, s1, s2 = cap;
        s0.zero(); s1.zero();
        for (uint32_t b = 0; b < w; b += 2) {
            s0 += ldf(&row[b]);
            if (b + 1 < w) s1 += ldf(&row[b + 1]);
            pos_permute(s0, s1, s2);
        }
        stf(&out[i], s0);
    }
#endif
}

// One digest layer: out[i] = compress(prev[l], prev[r]) with x = i / qs, y = i % qs,
// l = 2 x qs + y, r = (2 x + 1) qs + y (qs = 1 for an ordinary layer).
__global__ void mori_pos_layer(fr_t* out, const fr_t* prev, size_t n, size_t qs, F8 cap_)
{
#ifdef __CUDA_ARCH__
    const fr_t cap = f8(cap_);
    for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n;
         i += (size_t)gridDim.x * blockDim.x) {
        size_t x = i / qs, y = i % qs;
        fr_t s0 = ldf(&prev[2 * x * qs + y]);
        fr_t s1 = ldf(&prev[(2 * x + 1) * qs + y]);
        fr_t s2 = cap;
        pos_permute(s0, s1, s2);
        stf(&out[i], s0);
    }
#endif
}

// Leaf hashes of n rows held column-major on the device (column j at cols + j * n).
__global__ void mori_pos_rows_cm(fr_t* out, const fr_t* cols, size_t n, uint32_t w, F8 cap_)
{
#ifdef __CUDA_ARCH__
    const fr_t cap = f8(cap_);
    for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n;
         i += (size_t)gridDim.x * blockDim.x) {
        fr_t s0, s1, s2 = cap;
        s0.zero(); s1.zero();
        for (uint32_t b = 0; b < w; b += 2) {
            s0 += ldf(&cols[b * n + i]);
            if (b + 1 < w) s1 += ldf(&cols[(b + 1) * n + i]);
            pos_permute(s0, s1, s2);
        }
        stf(&out[i], s0);
    }
#endif
}

// Row-major copy of rows [r0, r0 + m) of a column-major device matrix into stage (m x w).
__global__ void mori_cm_to_rm(fr_t* stage, const fr_t* cols, size_t n, uint32_t w, size_t r0,
                              size_t m)
{
#ifdef __CUDA_ARCH__
    for (size_t t = blockIdx.x * (size_t)blockDim.x + threadIdx.x; t < m * w;
         t += (size_t)gridDim.x * blockDim.x) {
        size_t r = t % m, j = t / m;
        stf(&stage[r * w + j], ldf(&cols[j * n + r0 + r]));
    }
#endif
}

// Streaming form of the fused encode (low device memory): the sponge states of all n rows
// stay on the device; each call absorbs one column pair (a, b; b null for an odd last
// column), permutes, and writes the two codeword columns straight into the host row-major
// matrix rm (n x w, at column j) through the GPU's access to pageable host memory.
__global__ void mori_pos_init(fr_t* st, size_t n, F8 cap_)
{
#ifdef __CUDA_ARCH__
    const fr_t cap = f8(cap_);
    for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n;
         i += (size_t)gridDim.x * blockDim.x) {
        fr_t z; z.zero();
        stf(&st[3 * i], z); stf(&st[3 * i + 1], z); stf(&st[3 * i + 2], cap);
    }
#endif
}

#ifdef __CUDA_ARCH__
// Store to host memory with 32-bit words: a Rust Vec<Fq> is only 8-byte aligned, and the
// vector store of stf needs 16 (cudaErrorMisalignedAddress otherwise, MEASURED).
__device__ __forceinline__ void st_host(fr_t* p, const fr_t& v)
{
    uint32_t* q = (uint32_t*)p;
    for (int k = 0; k < 8; k++) q[k] = v[k];
}
#endif

__global__ void mori_pos_absorb(fr_t* st, const fr_t* a, const fr_t* b, size_t n, fr_t* rm,
                                uint32_t w, uint32_t j)
{
#ifdef __CUDA_ARCH__
    for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n;
         i += (size_t)gridDim.x * blockDim.x) {
        fr_t s0 = ldf(&st[3 * i]), s1 = ldf(&st[3 * i + 1]), s2 = ldf(&st[3 * i + 2]);
        fr_t x = ldf(&a[i]);
        s0 += x;
        st_host(&rm[i * w + j], x);
        if (b) {
            fr_t y = ldf(&b[i]);
            s1 += y;
            st_host(&rm[i * w + j + 1], y);
        }
        pos_permute(s0, s1, s2);
        stf(&st[3 * i], s0); stf(&st[3 * i + 1], s1); stf(&st[3 * i + 2], s2);
    }
#endif
}

__global__ void mori_pos_take0(fr_t* out, const fr_t* st, size_t n)
{
#ifdef __CUDA_ARCH__
    for (size_t i = blockIdx.x * (size_t)blockDim.x + threadIdx.x; i < n;
         i += (size_t)gridDim.x * blockDim.x)
        stf(&out[i], ldf(&st[3 * i]));
#endif
}
