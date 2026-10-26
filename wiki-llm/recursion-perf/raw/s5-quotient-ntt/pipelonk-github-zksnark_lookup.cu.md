# pipelonk PLONK/utils/zkp/cuda/zksnark_lookup.cu (ae branch)

> Source: https://raw.githubusercontent.com/zlyber/pipelonk/ae/PLONK/utils/zkp/cuda/zksnark_lookup.cu
> Collected: 2026-10-26
> Published: Unknown

#include "zksnark.cuh"
namespace cuda{
template 
\_\_global\_\_ void lookup\_ratio\_step1(const int64\_t N, const T\* h\_1, const T\* h\_2, const T\* h\_1\_next,
const T\* one, const T\* delta, const T\* epsilon, T\* result){
int64\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if(tid < N){
result[tid] = (((h\_2[tid] \* delta[0]) + ((delta[0] + one[0]) \* epsilon[0] + h\_1[tid])) \* ((delta[0] + one[0]) \* epsilon[0] + h\_2[tid] + h\_1\_next[tid] \* delta[0]));
result[tid] = result[tid].reciprocal();
}
}
template 
\_\_global\_\_ void lookup\_ratio\_step2(const int64\_t N, const T\* f, const T\* t, const T\* t\_next,
const T\* one, const T\* delta, const T\* epsilon, T\* result){
int64\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if(tid < N){
result[tid] = result[tid] \*
(((delta[0] + one[0]) \* epsilon[0] + t[tid]) + delta[0] \* t\_next[tid]) \*
(delta[0] + one[0]) \* (epsilon[0] + f[tid]);
}
}
void lookup\_ratio\_step1\_cuda\_(SyncedMemory h\_1, SyncedMemory h\_2, SyncedMemory h\_1\_next,
SyncedMemory one, SyncedMemory delta, SyncedMemory epsilon, SyncedMemory result, cudaStream\_t stream){
int64\_t N = h\_1.size()/(fr\_LIMBS \* sizeof(uint64\_t));
void\* h\_1\_ = h\_1.mutable\_gpu\_data\_async(stream);
void\* h\_2\_ = h\_2.mutable\_gpu\_data\_async(stream);
void\* h\_1\_next\_ = h\_1\_next.mutable\_gpu\_data\_async(stream);
void\* one\_ = one.mutable\_gpu\_data\_async(stream);
void\* delta\_ = delta.mutable\_gpu\_data\_async(stream);
void\* epsilon\_ = epsilon.mutable\_gpu\_data\_async(stream);
void\* result\_ = result.mutable\_gpu\_data\_async(stream);
int64\_t grid = (N + 128 - 1) / 128;
lookup\_ratio\_step1<<>>(N,
reinterpret\_cast(h\_1\_), reinterpret\_cast(h\_2\_), reinterpret\_cast(h\_1\_next\_),
reinterpret\_cast(one\_), reinterpret\_cast(delta\_), reinterpret\_cast(epsilon\_),
reinterpret\_cast(result\_));
}
void lookup\_ratio\_step2\_cuda\_(SyncedMemory f, SyncedMemory t, SyncedMemory t\_next,
SyncedMemory one, SyncedMemory delta, SyncedMemory epsilon, SyncedMemory result, cudaStream\_t stream){
int64\_t N = f.size()/(fr\_LIMBS \* sizeof(uint64\_t));
void\* f\_ = f.mutable\_gpu\_data\_async(stream);
void\* t\_ = t.mutable\_gpu\_data\_async(stream);
void\* t\_next\_ = t\_next.mutable\_gpu\_data\_async(stream);
void\* one\_ = one.mutable\_gpu\_data\_async(stream);
void\* delta\_ = delta.mutable\_gpu\_data\_async(stream);
void\* epsilon\_ = epsilon.mutable\_gpu\_data\_async(stream);
void\* result\_ = result.mutable\_gpu\_data\_async(stream);
int64\_t grid = (N + block\_work\_size() - 1) / block\_work\_size();
lookup\_ratio\_step2<<>>(N,
reinterpret\_cast(f\_), reinterpret\_cast(t\_), reinterpret\_cast(t\_next\_),
reinterpret\_cast(one\_), reinterpret\_cast(delta\_), reinterpret\_cast(epsilon\_),
reinterpret\_cast(result\_));
}
}