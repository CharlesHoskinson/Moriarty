# pipelonk PLONK/utils/zkp/cuda/zksnark_compute_query_table.cu (ae branch)

> Source: https://raw.githubusercontent.com/zlyber/pipelonk/ae/PLONK/utils/zkp/cuda/zksnark_compute_query_table.cu
> Collected: 2026-10-26
> Published: Unknown

#include "zksnark.cuh"
namespace cuda{
template 
\_\_global\_\_ void compute\_query\_table\_poly(T\* q\_lookup, T\* w\_l, T\* w\_r, T\* w\_o, T\* w\_4,
T\* t, T\* f\_0, T\* f\_1, T\* f\_2, T\* f\_3, int64\_t N) {
int64\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if(tid < N)
{
if(q\_lookup[tid].is\_zero()){
f\_0[tid] = \*t;
f\_1[tid].zero();
f\_2[tid].zero();
f\_3[tid].zero();
}
else{
f\_0[tid] = w\_l[tid];
f\_1[tid] = w\_r[tid];
f\_2[tid] = w\_o[tid];
f\_3[tid] = w\_4[tid];
}
}
}
template 
\_\_global\_\_ void compress\_poly\_cuda(T\* f, T\* f\_0, T\* challenge, int64\_t N) {
int64\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if(tid < N)
{
f[tid] = f[tid] \* challenge[0] + f\_0[tid];
}
}
template 
\_\_global\_\_ void compress\_poly\_cuda\_2(T\* f, T\* out, T\* challenge, int64\_t N) {
int64\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if(tid < N)
{
out[tid] = f[tid] + f[tid + N] \* challenge[0] \
+ f[tid + 2\*N] \* challenge[0] \* challenge[0] \
+ f[tid + 3\*N] \* challenge[0] \* challenge[0] \* challenge[0];
}
}
template 
static void compute\_query\_table\_poly\_template(T\* padded\_q\_lookup, T\* w\_l\_scalar,
T\* w\_r\_scalar, T\* w\_o\_scalar, T\* w\_4\_scalar,
T\* t\_poly, T\* f, int64\_t numel, cudaStream\_t stream) {
int64\_t N = numel / num\_uint64(padded\_q\_lookup);
auto f\_1 = f + N;
auto f\_2 = f + 2\*N;
auto f\_3 = f + 3\*N;
assert(N > 0 && N <= ::std::numeric\_limits::max());
int64\_t grid = (N + block\_work\_size() - 1) / block\_work\_size();
compute\_query\_table\_poly<<>>(padded\_q\_lookup, w\_l\_scalar, w\_r\_scalar, w\_o\_scalar, w\_4\_scalar,
t\_poly, f, f\_1, f\_2, f\_3, N);
CUDA\_CHECK(cudaGetLastError());
}
template 
static void compress\_poly\_template(T\* f,
T\* f\_0, T\* f\_1, T\* f\_2,
T\* challenge, int64\_t numel, cudaStream\_t stream) {
int64\_t N = numel/num\_uint64(f);
assert(N > 0 && N <= ::std::numeric\_limits::max());
int64\_t grid = (N + block\_work\_size() - 1) / block\_work\_size();
compress\_poly\_cuda<<>>(f, f\_2, challenge, N);
CUDA\_CHECK(cudaGetLastError());
compress\_poly\_cuda<<>>(f, f\_1, challenge, N);
CUDA\_CHECK(cudaGetLastError());
compress\_poly\_cuda<<>>(f, f\_0, challenge, N);
CUDA\_CHECK(cudaGetLastError());
}
template 
static void compress\_poly\_template\_2(T\* f, T\* out, T\* challenge, int64\_t numel, cudaStream\_t stream) {
int64\_t N = numel / num\_uint64(f);
assert(N > 0 && N <= ::std::numeric\_limits::max());
int64\_t grid = (N + block\_work\_size() - 1) / block\_work\_size();
compress\_poly\_cuda\_2<<>>(f, out, challenge, N);
CUDA\_CHECK(cudaGetLastError());
}
void compute\_query\_table\_cuda(SyncedMemory output, SyncedMemory padded\_q\_lookup,
SyncedMemory w\_l\_scalar, SyncedMemory w\_r\_scalar, SyncedMemory w\_o\_scalar, SyncedMemory w\_4\_scalar,
SyncedMemory t\_poly, cudaStream\_t stream){
int64\_t N = padded\_q\_lookup.size() \* 4/(fr\_LIMBS \* sizeof(uint64\_t));
void\* out\_gpu = output.mutable\_gpu\_data\_async(stream);
void\* lookup\_gpu = padded\_q\_lookup.mutable\_gpu\_data\_async(stream);
void\* w\_l\_gpu = w\_l\_scalar.mutable\_gpu\_data\_async(stream);
void\* w\_r\_gpu = w\_r\_scalar.mutable\_gpu\_data\_async(stream);
void\* w\_o\_gpu = w\_o\_scalar.mutable\_gpu\_data\_async(stream);
void\* w\_4\_gpu = w\_4\_scalar.mutable\_gpu\_data\_async(stream);
void\* t\_gpu = t\_poly.mutable\_gpu\_data\_async(stream);
int64\_t numel = padded\_q\_lookup.size() / sizeof(uint64\_t);
compute\_query\_table\_poly\_template(reinterpret\_cast(lookup\_gpu), reinterpret\_cast(w\_l\_gpu),
reinterpret\_cast(w\_r\_gpu), reinterpret\_cast(w\_o\_gpu), reinterpret\_cast(w\_4\_gpu),
reinterpret\_cast(t\_gpu), reinterpret\_cast(out\_gpu), numel, stream);
}
void compress\_cuda(SyncedMemory output, SyncedMemory f\_0, SyncedMemory f\_1,
SyncedMemory f\_2, SyncedMemory f\_3, SyncedMemory challenge, cudaStream\_t stream){
void\* out\_gpu = output.mutable\_gpu\_data\_async(stream);
void\* f\_3\_gpu = f\_3.mutable\_gpu\_data\_async(stream);
void\* f\_2\_gpu = f\_2.mutable\_gpu\_data\_async(stream);
void\* f\_1\_gpu = f\_1.mutable\_gpu\_data\_async(stream);
void\* f\_0\_gpu = f\_0.mutable\_gpu\_data\_async(stream);
void\* c\_gpu = challenge.mutable\_gpu\_data\_async(stream);
cudaMemcpyAsync(
out\_gpu,
f\_3\_gpu,
f\_3.size(),
cudaMemcpyDeviceToDevice,
stream);
int64\_t numel = f\_3.size() / sizeof(uint64\_t);
compress\_poly\_template(reinterpret\_cast(out\_gpu), reinterpret\_cast(f\_0\_gpu),
reinterpret\_cast(f\_1\_gpu), reinterpret\_cast(f\_2\_gpu), reinterpret\_cast(c\_gpu), numel, stream);
}
void compress\_cuda\_2(SyncedMemory concatenated\_f, SyncedMemory output, SyncedMemory challenge, int64\_t numel, cudaStream\_t stream){
void\* out\_gpu = output.mutable\_gpu\_data\_async(stream);
void\* f\_gpu = concatenated\_f.mutable\_gpu\_data\_async(stream);
void\* c\_gpu = challenge.mutable\_gpu\_data\_async(stream);
CUDA\_CHECK(cudaGetLastError());
compress\_poly\_template\_2(reinterpret\_cast(f\_gpu),
reinterpret\_cast(out\_gpu),
reinterpret\_cast(c\_gpu), numel,
stream);
}
}//namespace::cuda