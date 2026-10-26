# pipelonk PLONK/utils/zkp/cuda/zksnark.cuh (ae branch)

> Source: https://raw.githubusercontent.com/zlyber/pipelonk/ae/PLONK/utils/zkp/cuda/zksnark.cuh
> Collected: 2026-10-26
> Published: Unknown

#pragma once
#include 
#include 
#include 
#include 
#include 
#include "../../../../caffe/interface.hpp"
#include "../../mont/cuda/curve\_def.cuh"
#include 
using namespace caffe;
namespace cuda
{
void compress\_cuda(SyncedMemory output, SyncedMemory f\_0, SyncedMemory f\_1,
SyncedMemory f\_2, SyncedMemory f\_3, SyncedMemory challenge, cudaStream\_t stream = (cudaStream\_t)0);
void compress\_cuda\_2(SyncedMemory concatenated\_f, SyncedMemory output, SyncedMemory challenge, int64\_t numel, cudaStream\_t stream = (cudaStream\_t)0);
void compute\_query\_table\_cuda(SyncedMemory output, SyncedMemory padded\_q\_lookup,
SyncedMemory w\_l\_scalar, SyncedMemory w\_r\_scalar, SyncedMemory w\_o\_scalar, SyncedMemory w\_4\_scalar,
SyncedMemory t\_poly, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory msm\_zkp\_cuda(
SyncedMemory points,
SyncedMemory scalars,
int64\_t smcount,
cudaStream\_t stream = (cudaStream\_t)0);
void msm\_zkp\_cuda\_(
SyncedMemory points,
SyncedMemory scalars,
SyncedMemory workspace,
SyncedMemory out,
int64\_t smcount, cudaStream\_t stream = (cudaStream\_t)0);
void msm\_zkp\_chunk\_cuda\_(
SyncedMemory points,
SyncedMemory scalars,
SyncedMemory workspace,
SyncedMemory out,
int64\_t smcount,
int chunk\_id,
uint64\_t npoints,
cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory params\_zkp\_cuda(int64\_t domain\_size, bool is\_intt, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory ntt\_zkp\_cuda(SyncedMemory input, SyncedMemory params, bool is\_intt, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_zkp\_bitrev\_cuda\_(SyncedMemory inout, int lg\_domain\_size, bool is\_intt, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_zkp\_cuda(SyncedMemory input, SyncedMemory output, SyncedMemory params, bool is\_intt, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_zkp\_cuda\_(SyncedMemory inout, SyncedMemory params, bool is\_intt, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_zkp\_step1\_cuda(SyncedMemory inout, SyncedMemory params, int lg\_domain\_size, bool is\_intt, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_kcolumn\_step1\_cuda(SyncedMemory input, SyncedMemory params, int lg\_domain\_size, int i\_size, int k, bool is\_intt, int\* stage, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_kcolumn\_step1\_cuda\_raw(
uint64\_t\* input,
SyncedMemory params,
int64\_t numel,
int lg\_domain\_size,
int i\_size,
int k,
bool is\_intt,
int\* stage,
cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_zkp\_step2\_cuda(SyncedMemory inout, SyncedMemory params, int lg\_domain\_size, bool is\_intt, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_kcolumn\_step2\_cuda(SyncedMemory input, SyncedMemory params, int lg\_domain\_size, int j\_size, int k, bool is\_intt, int\* stage, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_zkp\_step3\_cuda(SyncedMemory inout, SyncedMemory params, int lg\_domain\_size, bool is\_intt, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_zkp\_step1\_internal\_cuda(SyncedMemory input, SyncedMemory output, SyncedMemory params, bool is\_intt, int\* stage, cudaStream\_t stream);
void ntt\_zkp\_step2\_internal\_cuda(SyncedMemory inout, SyncedMemory params, bool is\_intt, int\* stage, cudaStream\_t stream);
void ntt\_zkp\_step3\_internal\_cuda(SyncedMemory inout, SyncedMemory params, bool is\_intt, int\* stage, cudaStream\_t stream);
void ntt\_zkp\_coset\_init\_cuda(SyncedMemory inout, SyncedMemory params, int lg\_domain\_size, bool is\_intt, int stage, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_zkp\_coset\_init\_and\_step1\_cuda(SyncedMemory in, SyncedMemory out, SyncedMemory params, int lg\_domain\_size, int chunk\_id, int lambda, int stage, bool is\_intt, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_zkp\_coset\_step1\_cuda(SyncedMemory input, SyncedMemory params, int lg\_domain\_size, bool is\_intt, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_zkp\_coset\_step2\_cuda(SyncedMemory input, SyncedMemory params, int lg\_domain\_size, bool is\_intt, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void ntt\_zkp\_coset\_step3\_cuda(SyncedMemory input, SyncedMemory output, SyncedMemory params, int lg\_domain\_size, bool is\_intt, int stage, int chunk\_id, int lambda, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory make\_tensor(SyncedMemory input, uint64\_t pad\_len);
void lookup\_ratio\_step1\_cuda\_(SyncedMemory h\_1, SyncedMemory h\_2, SyncedMemory h\_1\_next,
SyncedMemory one, SyncedMemory delta, SyncedMemory epsilon, SyncedMemory result, cudaStream\_t stream = (cudaStream\_t)0);
void lookup\_ratio\_step2\_cuda\_(SyncedMemory f, SyncedMemory t, SyncedMemory t\_next,
SyncedMemory one, SyncedMemory delta, SyncedMemory epsilon, SyncedMemory result, cudaStream\_t stream = (cudaStream\_t)0);
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
SyncedMemory vb\_challenge, int64\_t SBOX\_ALPHA, cudaStream\_t stream = (cudaStream\_t)0);
void compute\_quotient\_identity\_range\_check\_i(SyncedMemory x,
SyncedMemory w\_l, SyncedMemory w\_r, SyncedMemory w\_o, SyncedMemory w\_4,
SyncedMemory z, SyncedMemory alpha, SyncedMemory beta, SyncedMemory gamma,
SyncedMemory k1, SyncedMemory k2, SyncedMemory k3, SyncedMemory result, cudaStream\_t stream = (cudaStream\_t)0);
void compute\_quotient\_copy\_range\_check\_i(SyncedMemory left\_sigma, SyncedMemory right\_sigma,
SyncedMemory out\_sigma, SyncedMemory fourth\_sigma, SyncedMemory mod,
SyncedMemory w\_l, SyncedMemory w\_r, SyncedMemory w\_o, SyncedMemory w\_4,
SyncedMemory z\_next, SyncedMemory alpha, SyncedMemory beta, SyncedMemory gamma,
SyncedMemory result, cudaStream\_t stream = (cudaStream\_t)0);
void compute\_quotient\_term\_check\_one\_i(SyncedMemory z, SyncedMemory l1, SyncedMemory one, SyncedMemory result, cudaStream\_t stream = (cudaStream\_t)0);
void compute\_quotient\_i(SyncedMemory compress\_tuple,
SyncedMemory f, SyncedMemory table, SyncedMemory table\_next, SyncedMemory h1, SyncedMemory h1\_next, SyncedMemory h2,
SyncedMemory z2, SyncedMemory z2\_next,
SyncedMemory l1,
SyncedMemory q\_lookup, SyncedMemory mod,
SyncedMemory delta, SyncedMemory epsilon, SyncedMemory zeta, SyncedMemory one,
SyncedMemory lookup\_seq, SyncedMemory lookup\_seq\_sq, SyncedMemory lookup\_seq\_cu,
SyncedMemory result, cudaStream\_t stream = (cudaStream\_t)0);
} // namespace cuda