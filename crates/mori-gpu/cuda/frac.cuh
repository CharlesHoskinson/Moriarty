// SPDX-License-Identifier: Apache-2.0
// Fractional sumcheck (LogUp GKR) of the L_Fr prover over BLS12-381 Fr (E4), the GPU form of
// stark-backend `fractional_sumcheck` with EF = F = Fr. A fraction is the pair (p, q), stored
// interleaved (Frac<EF> is repr(C)). Field arithmetic is exact, so every sum and product gives
// the same value as the CPU code in any order: the proof bytes do not change.

#define FRAC_BLOCK 256

// Segment tree, node k = (T[2k], T[2k + 1]) for k in [1, 2^(N+1)). One layer: the nodes
// k in [lo, 2 lo) from their children 2k, 2k + 1 (Frac add: p = pa qb + qa pb, q = qa qb).
__global__ void mori_frac_tree_layer(fr_t* T, size_t lo)
{
#ifdef __CUDA_ARCH__
    for (size_t t = blockIdx.x * (size_t)blockDim.x + threadIdx.x; t < lo;
         t += (size_t)gridDim.x * blockDim.x) {
        size_t k = lo + t, a = 2 * k, b = 2 * k + 1;
        fr_t pa = ldf(&T[2 * a]), qa = ldf(&T[2 * a + 1]);
        fr_t pb = ldf(&T[2 * b]), qb = ldf(&T[2 * b + 1]);
        stf(&T[2 * k], pa * qb + qa * pb);
        stf(&T[2 * k + 1], qa * qb);
    }
#endif
}

// The five MLE columns of one GKR layer sumcheck: eq, p_j0, q_j0, p_j1, q_j1. Element x of
// column j is c[j][x * s[j]].
struct FracCols {
    const fr_t* c[5];
    uint32_t s[5];
};

#ifdef __CUDA_ARCH__
__device__ __forceinline__ void frac_block_sum(fr_t (*sh)[FRAC_BLOCK], fr_t* acc, fr_t* out)
{
    const int t = threadIdx.x;
    for (int j = 0; j < 3; j++) sh[j][t] = acc[j];
    __syncthreads();
    for (int h = FRAC_BLOCK / 2; h > 0; h >>= 1) {
        if (t < h)
            for (int j = 0; j < 3; j++) sh[j][t] = sh[j][t] + sh[j][t + h];
        __syncthreads();
    }
    if (t == 0)
        for (int j = 0; j < 3; j++) out[j] = sh[j][0];
}
#endif

// Round polynomial at X = 1, 2, 3 (stark-backend `sumcheck_round_poly_evals` with d = 3):
// sum over y < half of eq * (p0 q1 + p1 q0 + lambda q0 q1), every column at
// t(X) = t(2y) + (t(2y + 1) - t(2y)) X. One partial sum of 3 values per block.
__global__ void __launch_bounds__(FRAC_BLOCK) mori_frac_eval(fr_t* part, FracCols c,
                                                             size_t half, F8 lam_)
{
#ifdef __CUDA_ARCH__
    __shared__ fr_t sh[3][FRAC_BLOCK];
    const fr_t lam = f8(lam_);
    fr_t acc[3];
    for (int j = 0; j < 3; j++) acc[j].zero();
    for (size_t y = blockIdx.x * (size_t)blockDim.x + threadIdx.x; y < half;
         y += (size_t)gridDim.x * blockDim.x) {
        fr_t v[5], d[5];
        for (int j = 0; j < 5; j++) {
            fr_t a = ldf(&c.c[j][(2 * y) * c.s[j]]);
            v[j] = ldf(&c.c[j][(2 * y + 1) * c.s[j]]);
            d[j] = v[j] - a;
        }
        for (int x = 0; x < 3; x++) {
            if (x)
                for (int j = 0; j < 5; j++) v[j] += d[j];
            fr_t q01 = v[2] * v[4];
            acc[x] += v[0] * (v[1] * v[4] + v[3] * v[2] + lam * q01);
        }
    }
    frac_block_sum(sh, acc, &part[3 * blockIdx.x]);
#endif
}

// out[0..3] = sum of the nb partial sums (one block).
__global__ void __launch_bounds__(FRAC_BLOCK) mori_frac_sum(fr_t* out, const fr_t* part,
                                                            size_t nb)
{
#ifdef __CUDA_ARCH__
    __shared__ fr_t sh[3][FRAC_BLOCK];
    fr_t acc[3];
    for (int j = 0; j < 3; j++) acc[j].zero();
    for (size_t b = threadIdx.x; b < nb; b += blockDim.x)
        for (int j = 0; j < 3; j++) acc[j] += ldf(&part[3 * b + j]);
    frac_block_sum(sh, acc, out);
#endif
}

// Fold at r (stark-backend `fold_mle_evals`): dst column j (contiguous, half entries) gets
// t(2y) + (t(2y + 1) - t(2y)) r.
__global__ void mori_frac_fold(fr_t* dst, FracCols c, size_t half, F8 r_)
{
#ifdef __CUDA_ARCH__
    const fr_t r = f8(r_);
    for (size_t y = blockIdx.x * (size_t)blockDim.x + threadIdx.x; y < half;
         y += (size_t)gridDim.x * blockDim.x)
        for (int j = 0; j < 5; j++) {
            fr_t a = ldf(&c.c[j][(2 * y) * c.s[j]]);
            fr_t b = ldf(&c.c[j][(2 * y + 1) * c.s[j]]);
            stf(&dst[j * half + y], a + (b - a) * r);
        }
#endif
}
