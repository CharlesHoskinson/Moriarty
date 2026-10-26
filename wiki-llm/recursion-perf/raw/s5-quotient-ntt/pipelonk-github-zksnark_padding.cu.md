# pipelonk PLONK/utils/zkp/cuda/zksnark_padding.cu (ae branch)

> Source: https://raw.githubusercontent.com/zlyber/pipelonk/ae/PLONK/utils/zkp/cuda/zksnark_padding.cu
> Collected: 2026-10-26
> Published: Unknown

#include "zksnark.cuh"
namespace cuda{
// void checkCudaError(cudaError\_t err, const char\* msg) {
// if (err != cudaSuccess) {
// std::cerr << "CUDA error: " << msg << " - " << cudaGetErrorString(err) << std::endl;
// exit(EXIT\_FAILURE);
// }
// }
template 
\_\_global\_\_ void padding(T\* input, T\* output, int64\_t N, int64\_t pad\_len) {
int64\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if(tid < N){
output[tid] = input[tid];
}
if(tid >= N - pad\_len)
{
output[tid].zero();
}
}
\_\_global\_\_ void kernel(uint64\_t\* input, uint64\_t\* output, uint64\_t N) {
int64\_t idx = blockIdx.x \* blockDim.x + threadIdx.x;
if (idx < N) {
output[idx \* 4] = input[idx];
}
}
template
static void padding\_template(T\* input, T\* output, int64\_t N, int64\_t pad\_len) {
assert((N + pad\_len) > 0 && (N + pad\_len) <= ::std::numeric\_limits::max());
int64\_t grid = ((N + pad\_len) + block\_work\_size() - 1) / block\_work\_size();
padding<<>>(input, output, N, pad\_len);
CUDA\_CHECK(cudaGetLastError());
}
void make\_tensor\_template(uint64\_t\* input, uint64\_t\* output, uint64\_t N, uint64\_t pad\_len) {
assert(N > 0 && N <= ::std::numeric\_limits::max());
int64\_t grid = (pad\_len + block\_work\_size() - 1) / block\_work\_size();
kernel<<>>(input, output, N);
CUDA\_CHECK(cudaGetLastError());
}
SyncedMemory padding\_cuda(SyncedMemory input, uint64\_t pad\_len){
int64\_t N = input.size()/(fr\_LIMBS \* sizeof(uint64\_t));
SyncedMemory output((N + pad\_len) \* fr\_LIMBS \* sizeof(uint64\_t));
void\* out\_gpu = output.mutable\_gpu\_data();
void\* in\_gpu = input.mutable\_gpu\_data();
padding\_template(static\_cast(in\_gpu), static\_cast(out\_gpu), N, pad\_len);
return output;
}
SyncedMemory make\_tensor(SyncedMemory input, uint64\_t pad\_len){
uint64\_t N = input.size()/sizeof(uint64\_t);
SyncedMemory output(pad\_len \* fr\_LIMBS \* sizeof(uint64\_t));
void\* out\_gpu = output.mutable\_gpu\_data();
void\* in\_gpu = input.mutable\_gpu\_data();
caffe\_gpu\_memset(output.size(), 0, out\_gpu);
make\_tensor\_template(static\_cast(in\_gpu), static\_cast(out\_gpu), N, pad\_len);
return output;
}
}