#include "msm.cuh"
namespace midnight\_plonk {
cudaError\_t msm\_naive\_launch(const G1Affine \*points, const Fr \*scalars, int count,
G1Jacobian \*out, cudaStream\_t stream) {
dim3 blocks(1);
dim3 threads(1);
msm\_naive\_kernel<<>>(points, scalars, count, out);
return cudaGetLastError();
}
cudaError\_t msm\_partial\_launch(const G1Affine \*points, const Fr \*scalars, int count,
G1Jacobian \*out, cudaStream\_t stream) {
int threads = 256;
int blocks = (count + threads - 1) / threads;
msm\_partial\_kernel<<>>(points, scalars, count, out);
return cudaGetLastError();
}
static \_\_global\_\_ void msm\_reduce\_kernel(const G1Jacobian \*in, G1Jacobian \*out, int count) {
int idx = blockIdx.x \* blockDim.x + threadIdx.x;
int i = idx \* 2;
if (i >= count) {
return;
}
if (i + 1 < count) {
g1\_add(out[idx], in[i], in[i + 1]);
} else {
out[idx] = in[i];
}
}
cudaError\_t msm\_reduce\_launch(const G1Jacobian \*in, G1Jacobian \*out, int count,
cudaStream\_t stream) {
int threads = 256;
int out\_count = (count + 1) / 2;
int blocks = (out\_count + threads - 1) / threads;
msm\_reduce\_kernel<<>>(in, out, count);
return cudaGetLastError();
}
} // namespace midnight\_plonk