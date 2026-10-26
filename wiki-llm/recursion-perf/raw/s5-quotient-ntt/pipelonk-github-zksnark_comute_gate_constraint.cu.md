# pipelonk PLONK/utils/zkp/cuda/zksnark_comute_gate_constraint.cu (ae branch)

> Source: https://raw.githubusercontent.com/zlyber/pipelonk/ae/PLONK/utils/zkp/cuda/zksnark_comute_gate_constraint.cu
> Collected: 2026-10-26
> Published: Unknown

#include "zksnark.cuh"
namespace cuda{
template 
\_\_global\_\_ void allmerge\_kernel(const int64\_t N,
const T\*\_\_restrict\_\_ a\_val, const T\*\_\_restrict\_\_ b\_val, const T\*\_\_restrict\_\_ q\_m, const T\*\_\_restrict\_\_ q\_l,
const T\*\_\_restrict\_\_ q\_r, const T\*\_\_restrict\_\_ c\_val, const T\*\_\_restrict\_\_ q\_o, const T\*\_\_restrict\_\_ d\_val,
const T\*\_\_restrict\_\_ q\_4, const T\*\_\_restrict\_\_ q\_hl, const T\*\_\_restrict\_\_ q\_hr, const T\*\_\_restrict\_\_ q\_h4,
const T\*\_\_restrict\_\_ q\_c, const T\*\_\_restrict\_\_ q\_arith, const T\*\_\_restrict\_\_ d\_next\_eval, const T\*\_\_restrict\_\_ range,
const T\*\_\_restrict\_\_ a\_next\_eval, const T\*\_\_restrict\_\_ b\_next\_eval, const T\*\_\_restrict\_\_ logic, const T\*\_\_restrict\_\_ fixed\_group\_add,
const T\*\_\_restrict\_\_ variable\_group\_add, const T\*\_\_restrict\_\_ pi\_eval\_8n,
T\*\_\_restrict\_\_ gate\_contributions,
const T\*\_\_restrict\_\_ four, const T\*\_\_restrict\_\_ one, const T\*\_\_restrict\_\_ two, const T\*\_\_restrict\_\_ three,
const T\*\_\_restrict\_\_ kappa\_range, const T\*\_\_restrict\_\_ kappa\_sq\_range, const T\*\_\_restrict\_\_ kappa\_cu\_range, const T\*\_\_restrict\_\_ range\_challenge,
const T\*\_\_restrict\_\_ P\_D, const T\*\_\_restrict\_\_ nine, const T\*\_\_restrict\_\_ kappa\_cu\_logic,
const T\*\_\_restrict\_\_ eighteen, const T\*\_\_restrict\_\_ eightyone, const T\*\_\_restrict\_\_ eightythree, const T\*\_\_restrict\_\_ kappa\_qu\_logic,
const T\*\_\_restrict\_\_ logic\_challenge, const T\*\_\_restrict\_\_ P\_A, const T\*\_\_restrict\_\_ kappa\_fb, const T\*\_\_restrict\_\_ kappa\_sq\_fb,
const T\*\_\_restrict\_\_ kappa\_cu\_fb, const T\*\_\_restrict\_\_ fb\_challenge, const T\*\_\_restrict\_\_ kappa\_vb, const T\*\_\_restrict\_\_ kappa\_sq\_vb,
const T\*\_\_restrict\_\_ vb\_challenge, int64\_t SBOX\_ALPHA)
{
int64\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if(tid < N)
{
gate\_contributions[tid] = (((((((c\_val[tid] - (q\_c[tid] \* (d\_val[tid] - (d\_val[tid] - d\_next\_eval[tid])))) \* kappa\_fb[0]) + ((((b\_next\_eval[tid] - ((b\_val[tid] \* (a\_val[tid] \* (c\_val[tid] \* b\_next\_eval[tid]))) \* P\_D[0])) - ((b\_val[tid] \* (((q\_r[tid] \* one[0]) \* ((d\_val[tid] - (d\_val[tid] - d\_next\_eval[tid])) ^ 2)) \* one[0])) - (a\_val[tid] \* ((q\_l[tid] \* (d\_val[tid] - (d\_val[tid] - d\_next\_eval[tid]))) \* P\_A[0])))) \* kappa\_cu\_fb[0]) + ((((d\_val[tid] - (d\_val[tid] - d\_next\_eval[tid])) \* one[0]) \* ((d\_val[tid] - (d\_val[tid] - d\_next\_eval[tid])) \* ((d\_val[tid] - (d\_val[tid] - d\_next\_eval[tid])) \* one[0]))) + (((a\_next\_eval[tid] + ((b\_val[tid] \* (a\_val[tid] \* (c\_val[tid] \* a\_next\_eval[tid]))) \* P\_D[0])) - ((b\_val[tid] \* (q\_l[tid] \* (d\_val[tid] - (d\_val[tid] - d\_next\_eval[tid])))) + (a\_val[tid] \* (((q\_r[tid] \* one[0]) \* ((d\_val[tid] - (d\_val[tid] - d\_next\_eval[tid])) ^ 2)) \* one[0])))) \* kappa\_sq\_fb[0])))) \* fb\_challenge[0]) \* fixed\_group\_add[tid]) + ((((((((((d\_next\_eval[tid] - (d\_val[tid] \* four[0])) + (((a\_val[tid] \* four[0]) - a\_next\_eval[tid]) + ((b\_val[tid] \* four[0]) - b\_next\_eval[tid]))) \* three[0]) - ((c\_val[tid] \* ((((((a\_val[tid] \* four[0]) - a\_next\_eval[tid]) + ((b\_val[tid] \* four[0]) - b\_next\_eval[tid])) \* eightyone[0]) - ((c\_val[tid] \* (((c\_val[tid] \* four[0]) - ((((a\_val[tid] \* four[0]) - a\_next\_eval[tid]) + ((b\_val[tid] \* four[0]) - b\_next\_eval[tid])) \* eighteen[0])) \* eightyone[0])) + (((((a\_val[tid] \* four[0]) - a\_next\_eval[tid]) ^ 2) + (((a\_val[tid] \* four[0]) - a\_next\_eval[tid]) ^ 2)) \* eighteen[0]))) \* eightythree[0])) \* two[0])) + (q\_c[tid] \* (((d\_next\_eval[tid] - (d\_val[tid] \* four[0])) \* nine[0]) - ((((a\_val[tid] \* four[0]) - a\_next\_eval[tid]) + ((b\_val[tid] \* four[0]) - b\_next\_eval[tid])) \* three[0])))) \* kappa\_qu\_logic[0]) + (((c\_val[tid] - (((a\_val[tid] \* four[0]) - a\_next\_eval[tid]) \* ((b\_val[tid] \* four[0]) - b\_next\_eval[tid]))) \* kappa\_cu\_logic[0]) + (((d\_next\_eval[tid] - (d\_val[tid] \* four[0])) \* (((d\_next\_eval[tid] - (d\_val[tid] \* four[0])) \* three[0]) \* (((d\_next\_eval[tid] - (d\_val[tid] \* four[0])) \* one[0]) \* ((d\_next\_eval[tid] - (d\_val[tid] \* four[0])) \* two[0])))) + ((((a\_val[tid] \* four[0]) - a\_next\_eval[tid]) \* ((((a\_val[tid] \* four[0]) - a\_next\_eval[tid]) \* three[0]) \* ((((a\_val[tid] \* four[0]) - a\_next\_eval[tid]) \* one[0]) \* (((a\_val[tid] \* four[0]) - a\_next\_eval[tid]) \* two[0])))) + (((b\_val[tid] \* four[0]) - b\_next\_eval[tid]) \* ((((b\_val[tid] \* four[0]) - b\_next\_eval[tid]) \* three[0]) \* ((((b\_val[tid] \* four[0]) - b\_next\_eval[tid]) \* one[0]) \* (((b\_val[tid] \* four[0]) - b\_next\_eval[tid]) \* two[0])))))))) \* logic\_challenge[0]) \* logic[tid]) + ((((((((a\_val[tid] \* four[0]) - d\_next\_eval[tid]) \* ((((a\_val[tid] \* four[0]) - d\_next\_eval[tid]) \* three[0]) \* ((((a\_val[tid] \* four[0]) - d\_next\_eval[tid]) \* one[0]) \* (((a\_val[tid] \* four[0]) - d\_next\_eval[tid]) \* two[0])))) \* kappa\_cu\_range[0]) + ((((a\_val[tid] - (b\_val[tid] \* four[0])) \* (((a\_val[tid] - (b\_val[tid] \* four[0])) \* three[0]) \* (((a\_val[tid] - (b\_val[tid] \* four[0])) \* one[0]) \* ((a\_val[tid] - (b\_val[tid] \* four[0])) \* two[0])))) \* kappa\_sq\_range[0]) + (((c\_val[tid] - (d\_val[tid] \* four[0])) \* (((c\_val[tid] - (d\_val[tid] \* four[0])) \* three[0]) \* (((c\_val[tid] - (d\_val[tid] \* four[0])) \* one[0]) \* ((c\_val[tid] - (d\_val[tid] \* four[0])) \* two[0])))) + (((b\_val[tid] - (c\_val[tid] \* four[0])) \* (((b\_val[tid] - (c\_val[tid] \* four[0])) \* three[0]) \* (((b\_val[tid] - (c\_val[tid] \* four[0])) \* one[0]) \* ((b\_val[tid] - (c\_val[tid] \* four[0])) \* two[0])))) \* kappa\_range[0])))) \* range\_challenge[0]) \* range[tid]) + ((((((d\_val[tid] ^ SBOX\_ALPHA) \* q\_h4[tid]) + (((b\_val[tid] ^ SBOX\_ALPHA) \* q\_hr[tid]) + (((a\_val[tid] ^ SBOX\_ALPHA) \* q\_hl[tid]) + ((d\_val[tid] \* q\_4[tid]) + ((c\_val[tid] \* q\_o[tid]) + ((b\_val[tid] \* q\_r[tid]) + (((a\_val[tid] \* b\_val[tid]) \* q\_m[tid]) + (a\_val[tid] \* q\_l[tid])))))))) + q\_c[tid]) \* q\_arith[tid]) + pi\_eval\_8n[tid])))) + (((((((b\_val[tid] \* d\_val[tid]) - ((a\_val[tid] \* c\_val[tid]) \* P\_A[0])) - (b\_next\_eval[tid] - ((b\_val[tid] \* c\_val[tid]) \* (d\_next\_eval[tid] \* (b\_next\_eval[tid] \* P\_D[0]))))) \* kappa\_sq\_vb[0]) + ((d\_next\_eval[tid] - (a\_val[tid] \* d\_val[tid])) + (((d\_next\_eval[tid] + (b\_val[tid] \* c\_val[tid])) - (a\_next\_eval[tid] + ((b\_val[tid] \* c\_val[tid]) \* (d\_next\_eval[tid] \* (a\_next\_eval[tid] \* P\_D[0]))))) \* kappa\_vb[0]))) \* vb\_challenge[0]) \* variable\_group\_add[tid]));
}
}
SyncedMemory compute\_gate\_constraint\_allmerge\_cuda(SyncedMemory a\_val, SyncedMemory b\_val, SyncedMemory c\_val, SyncedMemory d\_val,
SyncedMemory a\_next\_eval, SyncedMemory b\_next\_eval, SyncedMemory d\_next\_eval,
SyncedMemory q\_m, SyncedMemory q\_l, SyncedMemory q\_r, SyncedMemory q\_o,
SyncedMemory q\_4, SyncedMemory q\_c, SyncedMemory q\_hl, SyncedMemory q\_hr, SyncedMemory q\_h4, SyncedMemory q\_arith,
SyncedMemory range, SyncedMemory logic, SyncedMemory fixed\_group\_add, SyncedMemory variable\_group\_add, SyncedMemory pi\_eval\_8n,
SyncedMemory four, SyncedMemory one, SyncedMemory two, SyncedMemory three,
SyncedMemory kappa\_range, SyncedMemory kappa\_sq\_range, SyncedMemory kappa\_cu\_range, SyncedMemory range\_challenge,
SyncedMemory P\_D, SyncedMemory nine, SyncedMemory kappa\_cu\_logic,
SyncedMemory eighteen, SyncedMemory eightyone, SyncedMemory eightythree, SyncedMemory kappa\_qu\_logic,
SyncedMemory logic\_challenge, SyncedMemory P\_A, SyncedMemory kappa\_fb, SyncedMemory kappa\_sq\_fb,
SyncedMemory kappa\_cu\_fb, SyncedMemory fb\_challenge, SyncedMemory kappa\_vb, SyncedMemory kappa\_sq\_vb,
SyncedMemory vb\_challenge, int64\_t SBOX\_ALPHA, cudaStream\_t stream)
{
int64\_t N = a\_val.size()/(fr\_LIMBS \* sizeof(uint64\_t));
SyncedMemory gate\_contributions(a\_val.size());
void\* a\_val\_ = a\_val.mutable\_gpu\_data\_async(stream);
void\* b\_val\_ = b\_val.mutable\_gpu\_data\_async(stream);
void\* q\_m\_ = q\_m.mutable\_gpu\_data\_async(stream);
void\* q\_l\_ = q\_l.mutable\_gpu\_data\_async(stream);
void\* q\_r\_ = q\_r.mutable\_gpu\_data\_async(stream);
void\* c\_val\_ = c\_val.mutable\_gpu\_data\_async(stream);
void\* q\_o\_ = q\_o.mutable\_gpu\_data\_async(stream);
void\* d\_val\_ = d\_val.mutable\_gpu\_data\_async(stream);
void\* q\_4\_ = q\_4.mutable\_gpu\_data\_async(stream);
void\* q\_hl\_ = q\_hl.mutable\_gpu\_data\_async(stream);
void\* q\_hr\_ = q\_hr.mutable\_gpu\_data\_async(stream);
void\* q\_h4\_ = q\_h4.mutable\_gpu\_data\_async(stream);
void\* q\_c\_ = q\_c.mutable\_gpu\_data\_async(stream);
void\* q\_arith\_ = q\_arith.mutable\_gpu\_data\_async(stream);
void\* d\_next\_eval\_ = d\_next\_eval.mutable\_gpu\_data\_async(stream);
void\* range\_ = range.mutable\_gpu\_data\_async(stream);
void\* a\_next\_eval\_ = a\_next\_eval.mutable\_gpu\_data\_async(stream);
void\* b\_next\_eval\_ = b\_next\_eval.mutable\_gpu\_data\_async(stream);
void\* logic\_ = logic.mutable\_gpu\_data\_async(stream);
void\* fixed\_group\_add\_ = fixed\_group\_add.mutable\_gpu\_data\_async(stream);
void\* variable\_group\_add\_ = variable\_group\_add.mutable\_gpu\_data\_async(stream);
void\* pi\_eval\_8n\_ = pi\_eval\_8n.mutable\_gpu\_data\_async(stream);
void\* gate\_contributions\_ = gate\_contributions.mutable\_gpu\_data\_async(stream);
void\* four\_ = four.mutable\_gpu\_data\_async(stream);
void\* one\_ = one.mutable\_gpu\_data\_async(stream);
void\* two\_ = two.mutable\_gpu\_data\_async(stream);
void\* three\_ = three.mutable\_gpu\_data\_async(stream);
void\* kappa\_range\_ = kappa\_range.mutable\_gpu\_data\_async(stream);
void\* kappa\_sq\_range\_ = kappa\_sq\_range.mutable\_gpu\_data\_async(stream);
void\* kappa\_cu\_range\_ = kappa\_cu\_range.mutable\_gpu\_data\_async(stream);
void\* range\_challenge\_ = range\_challenge.mutable\_gpu\_data\_async(stream);
void\* P\_D\_ = P\_D.mutable\_gpu\_data\_async(stream);
void\* nine\_ = nine.mutable\_gpu\_data\_async(stream);
void\* kappa\_cu\_logic\_ = kappa\_cu\_logic.mutable\_gpu\_data\_async(stream);
void\* eighteen\_ = eighteen.mutable\_gpu\_data\_async(stream);
void\* eightyone\_ = eightyone.mutable\_gpu\_data\_async(stream);
void\* eightythree\_ = eightythree.mutable\_gpu\_data\_async(stream);
void\* kappa\_qu\_logic\_ = kappa\_qu\_logic.mutable\_gpu\_data\_async(stream);
void\* logic\_challenge\_ = logic\_challenge.mutable\_gpu\_data\_async(stream);
void\* P\_A\_ = P\_A.mutable\_gpu\_data\_async(stream);
void\* kappa\_fb\_ = kappa\_fb.mutable\_gpu\_data\_async(stream);
void\* kappa\_sq\_fb\_ = kappa\_sq\_fb.mutable\_gpu\_data\_async(stream);
void\* kappa\_cu\_fb\_ = kappa\_cu\_fb.mutable\_gpu\_data\_async(stream);
void\* fb\_challenge\_ = fb\_challenge.mutable\_gpu\_data\_async(stream);
void\* kappa\_vb\_ = kappa\_vb.mutable\_gpu\_data\_async(stream);
void\* kappa\_sq\_vb\_ = kappa\_sq\_vb.mutable\_gpu\_data\_async(stream);
void\* vb\_challenge\_ = vb\_challenge.mutable\_gpu\_data\_async(stream);
int64\_t grid = (N + 128 - 1) / 128;
allmerge\_kernel<<>>(N,
reinterpret\_cast(a\_val\_), reinterpret\_cast(b\_val\_), reinterpret\_cast(q\_m\_), reinterpret\_cast(q\_l\_),
reinterpret\_cast(q\_r\_), reinterpret\_cast(c\_val\_), reinterpret\_cast(q\_o\_), reinterpret\_cast(d\_val\_),
reinterpret\_cast(q\_4\_), reinterpret\_cast(q\_hl\_), reinterpret\_cast(q\_hr\_), reinterpret\_cast(q\_h4\_),
reinterpret\_cast(q\_c\_), reinterpret\_cast(q\_arith\_), reinterpret\_cast(d\_next\_eval\_), reinterpret\_cast(range\_),
reinterpret\_cast(a\_next\_eval\_), reinterpret\_cast(b\_next\_eval\_), reinterpret\_cast(logic\_), reinterpret\_cast(fixed\_group\_add\_),
reinterpret\_cast(variable\_group\_add\_), reinterpret\_cast(pi\_eval\_8n\_), reinterpret\_cast(gate\_contributions\_), reinterpret\_cast(four\_), reinterpret\_cast(one\_), reinterpret\_cast(two\_), reinterpret\_cast(three\_),
reinterpret\_cast(kappa\_range\_), reinterpret\_cast(kappa\_sq\_range\_), reinterpret\_cast(kappa\_cu\_range\_), reinterpret\_cast(range\_challenge\_),
reinterpret\_cast(P\_D\_), reinterpret\_cast(nine\_), reinterpret\_cast(kappa\_cu\_logic\_),
reinterpret\_cast(eighteen\_), reinterpret\_cast(eightyone\_), reinterpret\_cast(eightythree\_), reinterpret\_cast(kappa\_qu\_logic\_),
reinterpret\_cast(logic\_challenge\_), reinterpret\_cast(P\_A\_), reinterpret\_cast(kappa\_fb\_), reinterpret\_cast(kappa\_sq\_fb\_),
reinterpret\_cast(kappa\_cu\_fb\_), reinterpret\_cast(fb\_challenge\_), reinterpret\_cast(kappa\_vb\_), reinterpret\_cast(kappa\_sq\_vb\_),
reinterpret\_cast(vb\_challenge\_), SBOX\_ALPHA);
return gate\_contributions;
}
}//namespace::cuda