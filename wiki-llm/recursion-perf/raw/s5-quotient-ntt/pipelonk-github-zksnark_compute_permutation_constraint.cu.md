# pipelonk PLONK/utils/zkp/cuda/zksnark_compute_permutation_constraint.cu (ae branch)

> Source: https://raw.githubusercontent.com/zlyber/pipelonk/ae/PLONK/utils/zkp/cuda/zksnark_compute_permutation_constraint.cu
> Collected: 2026-10-26
> Published: Unknown

#include "zksnark.cuh"
namespace cuda{
template 
\_\_global\_\_ void compute\_quotient\_identity\_range\_check\_i(const int64\_t N, const T\* x,
const T\* w\_l, const T\* w\_r, const T\* w\_o, const T\* w\_4,
const T\* z, const T\* alpha, const T\* beta, const T\* gamma,
const T\* k1, const T\* k2, const T\* k3, T\* result)
{
int64\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if(tid < N){
result[tid] = (x[tid] \* beta[0] + w\_l[tid] + gamma[0]) \* (x[tid] \* beta[0] \* k1[0] + w\_r[tid] + gamma[0]) \* \
(x[tid] \* beta[0] \* k2[0] + w\_o[tid] + gamma[0]) \* (x[tid] \* beta[0] \* k3[0] + w\_4[tid] + gamma[0]) \* z[tid] \* alpha[0];
}
}
template 
\_\_global\_\_ void compute\_quotient\_copy\_range\_check\_i(const int64\_t N,
const T\* left\_sigma, const T\* right\_sigma,
const T\* out\_sigma, const T\* fourth\_sigma, const T\* mod,
const T\* w\_l, const T\* w\_r, const T\* w\_o, const T\* w\_4,
const T\* z\_next, const T\* alpha, const T\* beta, const T\* gamma,
T\* result)
{
int64\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if(tid < N){
result[tid] = result[tid] + \
mod[tid] - \
(left\_sigma[tid] \* beta[0] + w\_l[tid] + gamma[0]) \* \
(right\_sigma[tid] \* beta[0] + w\_r[tid] + gamma[0]) \* \
(out\_sigma[tid] \* beta[0] + w\_o[tid] + gamma[0]) \* \
(fourth\_sigma[tid] \* beta[0] + w\_4[tid] + gamma[0]) \* \
z\_next[tid] \* alpha[0];
}
}
template 
\_\_global\_\_ void compute\_quotient\_term\_check\_one\_i(const int64\_t N,
const T\* z, const T\* l1, const T\* one, T\* result)
{
int64\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if(tid < N){
result[tid] = result[tid] + \
(z[tid] - one[0]) \* l1[tid];
}
}
void compute\_quotient\_identity\_range\_check\_i(SyncedMemory x,
SyncedMemory w\_l, SyncedMemory w\_r, SyncedMemory w\_o, SyncedMemory w\_4,
SyncedMemory z, SyncedMemory alpha, SyncedMemory beta, SyncedMemory gamma,
SyncedMemory k1, SyncedMemory k2, SyncedMemory k3, SyncedMemory result, cudaStream\_t stream)
{
int64\_t N = x.size()/(fr\_LIMBS \* sizeof(uint64\_t));
void\* x\_ = x.mutable\_gpu\_data\_async(stream);
void\* w\_l\_ = w\_l.mutable\_gpu\_data\_async(stream);
void\* w\_r\_ = w\_r.mutable\_gpu\_data\_async(stream);
void\* w\_o\_ = w\_o.mutable\_gpu\_data\_async(stream);
void\* w\_4\_ = w\_4.mutable\_gpu\_data\_async(stream);
void\* z\_ = z.mutable\_gpu\_data\_async(stream);
void\* alpha\_ = alpha.mutable\_gpu\_data\_async(stream);
void\* beta\_ = beta.mutable\_gpu\_data\_async(stream);
void\* gamma\_ = gamma.mutable\_gpu\_data\_async(stream);
void\* k1\_ = k1.mutable\_gpu\_data\_async(stream);
void\* k2\_ = k2.mutable\_gpu\_data\_async(stream);
void\* k3\_ = k3.mutable\_gpu\_data\_async(stream);
void\* res\_ = result.mutable\_gpu\_data\_async(stream);
int64\_t grid = (N + 128 - 1) / 128;
compute\_quotient\_identity\_range\_check\_i<<>>(N,
reinterpret\_cast(x\_),
reinterpret\_cast(w\_l\_), reinterpret\_cast(w\_r\_), reinterpret\_cast(w\_o\_), reinterpret\_cast(w\_4\_),
reinterpret\_cast(z\_),
reinterpret\_cast(alpha\_), reinterpret\_cast(beta\_), reinterpret\_cast(gamma\_),
reinterpret\_cast(k1\_), reinterpret\_cast(k2\_), reinterpret\_cast(k3\_),
reinterpret\_cast(res\_));
}
void compute\_quotient\_copy\_range\_check\_i(SyncedMemory left\_sigma, SyncedMemory right\_sigma,
SyncedMemory out\_sigma, SyncedMemory fourth\_sigma, SyncedMemory mod,
SyncedMemory w\_l, SyncedMemory w\_r, SyncedMemory w\_o, SyncedMemory w\_4,
SyncedMemory z\_next, SyncedMemory alpha, SyncedMemory beta, SyncedMemory gamma,
SyncedMemory result, cudaStream\_t stream)
{
int64\_t N = left\_sigma.size()/(fr\_LIMBS \* sizeof(uint64\_t));
void\* left\_ = left\_sigma.mutable\_gpu\_data\_async(stream);
void\* right\_ = right\_sigma.mutable\_gpu\_data\_async(stream);
void\* out\_ = out\_sigma.mutable\_gpu\_data\_async(stream);
void\* fourth\_ = fourth\_sigma.mutable\_gpu\_data\_async(stream);
void\* w\_l\_ = w\_l.mutable\_gpu\_data\_async(stream);
void\* w\_r\_ = w\_r.mutable\_gpu\_data\_async(stream);
void\* w\_o\_ = w\_o.mutable\_gpu\_data\_async(stream);
void\* w\_4\_ = w\_4.mutable\_gpu\_data\_async(stream);
void\* z\_next\_ = z\_next.mutable\_gpu\_data\_async(stream);
void\* alpha\_ = alpha.mutable\_gpu\_data\_async(stream);
void\* beta\_ = beta.mutable\_gpu\_data\_async(stream);
void\* gamma\_ = gamma.mutable\_gpu\_data\_async(stream);
void\* mod\_ = mod.mutable\_gpu\_data\_async(stream);
void\* res\_ = result.mutable\_gpu\_data\_async(stream);
int64\_t grid = (N + 256 - 1) / 256;
compute\_quotient\_copy\_range\_check\_i<<>>(N,
reinterpret\_cast(left\_), reinterpret\_cast(right\_), reinterpret\_cast(out\_), reinterpret\_cast(fourth\_),
reinterpret\_cast(mod\_),
reinterpret\_cast(w\_l\_), reinterpret\_cast(w\_r\_), reinterpret\_cast(w\_o\_), reinterpret\_cast(w\_4\_),
reinterpret\_cast(z\_next\_),
reinterpret\_cast(alpha\_), reinterpret\_cast(beta\_), reinterpret\_cast(gamma\_),
reinterpret\_cast(res\_));
}
void compute\_quotient\_term\_check\_one\_i(SyncedMemory z, SyncedMemory l1, SyncedMemory one, SyncedMemory result, cudaStream\_t stream)
{
int64\_t N = z.size()/(fr\_LIMBS \* sizeof(uint64\_t));
void\* z\_ = z.mutable\_gpu\_data\_async(stream);
void\* one\_ = one.mutable\_gpu\_data\_async(stream);
void\* l1\_ = l1.mutable\_gpu\_data\_async(stream);
void\* res\_ = result.mutable\_gpu\_data\_async(stream);
int64\_t grid = (N + 256 - 1) / 256;
compute\_quotient\_term\_check\_one\_i<<>>(N, reinterpret\_cast(z\_), reinterpret\_cast(l1\_), reinterpret\_cast(one\_), reinterpret\_cast(res\_));
}
}