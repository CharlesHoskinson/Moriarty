#include "msm.cuh"

namespace midnight_plonk {

cudaError_t msm_naive_launch(const G1Affine *points, const Fr *scalars, int count,
                             G1Jacobian *out, cudaStream_t stream) {
  dim3 blocks(1);
  dim3 threads(1);
  msm_naive_kernel<<<blocks, threads, 0, stream>>>(points, scalars, count, out);
  return cudaGetLastError();
}

cudaError_t msm_partial_launch(const G1Affine *points, const Fr *scalars, int count,
                               G1Jacobian *out, cudaStream_t stream) {
  int threads = 256;
  int blocks = (count + threads - 1) / threads;
  msm_partial_kernel<<<blocks, threads, 0, stream>>>(points, scalars, count, out);
  return cudaGetLastError();
}

static __global__ void msm_reduce_kernel(const G1Jacobian *in, G1Jacobian *out, int count) {
  int idx = blockIdx.x * blockDim.x + threadIdx.x;
  int i = idx * 2;
  if (i >= count) {
    return;
  }
  if (i + 1 < count) {
    g1_add(out[idx], in[i], in[i + 1]);
  } else {
    out[idx] = in[i];
  }
}

cudaError_t msm_reduce_launch(const G1Jacobian *in, G1Jacobian *out, int count,
                              cudaStream_t stream) {
  int threads = 256;
  int out_count = (count + 1) / 2;
  int blocks = (out_count + threads - 1) / threads;
  msm_reduce_kernel<<<blocks, threads, 0, stream>>>(in, out, count);
  return cudaGetLastError();
}

}  // namespace midnight_plonk
