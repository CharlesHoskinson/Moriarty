# pipelonk PLONK/utils/function.cuh (ae branch)

> Source: https://raw.githubusercontent.com/zlyber/pipelonk/ae/PLONK/utils/function.cuh
> Collected: 2026-10-26
> Published: Unknown

#pragma once
#include "../../caffe/interface.hpp"
#include "mont/cuda/curve\_def.cuh"
#include "mont/cpu/mont\_arithmetic.h"
#include "mont/cuda/mont\_arithmetic.cuh"
#include "zkp/cpu/msmcollect.hpp"
#include "zkp/cuda/zksnark.cuh"
#include 
#include 
#include 
SyncedMemory to\_mont(SyncedMemory input);
SyncedMemory to\_base(SyncedMemory input, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory neg\_mod(SyncedMemory input);
SyncedMemory inv\_mod(SyncedMemory input);
SyncedMemory add\_mod(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
void add\_mod\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory sub\_mod(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
void sub\_mod\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory mul\_mod(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
void mul\_mod\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory div\_mod(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
void div\_mod\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory exp\_mod(SyncedMemory input, int exp);
SyncedMemory add\_mod\_scalar(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
void add\_mod\_scalar\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory sub\_mod\_scalar(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
void sub\_mod\_scalar\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory mul\_mod\_scalar(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
void mul\_mod\_scalar\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory div\_mod\_scalar(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
void div\_mod\_scalar\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory gen\_sequence(uint64\_t N, SyncedMemory x, cudaStream\_t stream = (cudaStream\_t)0);
void gen\_sequence\_(uint64\_t N, SyncedMemory x, SyncedMemory y, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory repeat\_to\_poly(SyncedMemory x, uint64\_t N, cudaStream\_t stream = (cudaStream\_t)0);
void repeat\_to\_poly\_(SyncedMemory x, SyncedMemory y, uint64\_t N, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory evaluate(SyncedMemory poly, SyncedMemory x);
SyncedMemory poly\_div\_poly(SyncedMemory divid, SyncedMemory c);
SyncedMemory pad\_poly(SyncedMemory x, uint64\_t N, cudaStream\_t stream = (cudaStream\_t)0);
void pad\_poly\_(SyncedMemory x, SyncedMemory out, uint64\_t N, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory repeat\_zero(uint64\_t N);
SyncedMemory cat(SyncedMemory a, SyncedMemory b);
SyncedMemory slice(SyncedMemory a, uint64\_t len, bool forward);
std::vector split\_tx\_poly(uint64\_t n, SyncedMemory t\_poly);
SyncedMemory accumulate\_mul\_poly(SyncedMemory product, cudaStream\_t stream = (cudaStream\_t)0);
void accumulate\_mul\_poly\_(SyncedMemory product, SyncedMemory output, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory make\_tensor(SyncedMemory input, uint64\_t pad\_len);
void lookup\_ratio\_step1\_(SyncedMemory h\_1, SyncedMemory h\_2, SyncedMemory h\_1\_next,
SyncedMemory one, SyncedMemory delta, SyncedMemory epsilon, SyncedMemory result, cudaStream\_t stream = (cudaStream\_t)0);
void lookup\_ratio\_step2\_(SyncedMemory f, SyncedMemory t, SyncedMemory t\_next,
SyncedMemory one, SyncedMemory delta, SyncedMemory epsilon, SyncedMemory result, cudaStream\_t stream = (cudaStream\_t)0);
void calculate\_indices(uint32\_t tid, int stage1, int iterations1, int stage2, int iterations2, uint32\_t\* map);
class Ntt{
public:
SyncedMemory Params;
Ntt(int domain\_size, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory forward(SyncedMemory input, cudaStream\_t stream = (cudaStream\_t)0);
void forward\_(SyncedMemory input, SyncedMemory output, cudaStream\_t stream = (cudaStream\_t)0);
void forward1(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void forward2(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void forward3(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void forward1\_kcolumn(SyncedMemory input, int lg\_domain\_size, int i\_size, int k, int\* stage, cudaStream\_t stream = (cudaStream\_t)0);
void forward1\_kcolumn\_raw(uint64\_t\* input, int lg\_domain\_size, int64\_t numel, int i\_size, int k, int\* stage, cudaStream\_t stream = (cudaStream\_t)0);
void forward2\_kcolumn(SyncedMemory input, int lg\_domain\_size, int i\_size, int k, int\* stage, cudaStream\_t stream = (cudaStream\_t)0);
void forward1\_internal(SyncedMemory input, SyncedMemory output, int\* stage, cudaStream\_t stream = (cudaStream\_t)0);
void forward2\_internal(SyncedMemory inout, int\* stage, cudaStream\_t stream = (cudaStream\_t)0);
void forward3\_internal(SyncedMemory inout, int\* stage, cudaStream\_t stream = (cudaStream\_t)0);
};
class Intt {
public:
SyncedMemory Params;
Intt(int domain\_size, cudaStream\_t stream = (cudaStream\_t)0);
SyncedMemory forward(SyncedMemory input, cudaStream\_t stream = (cudaStream\_t)0);
void forward\_(SyncedMemory input, SyncedMemory output, cudaStream\_t stream = (cudaStream\_t)0);
void \_forward\_(SyncedMemory input, cudaStream\_t stream = (cudaStream\_t)0);
void forward1(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void forward2(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void forward3(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
};
class Ntt\_coset {
public:
SyncedMemory Params;
int lg\_domain\_size;
Ntt\_coset(int domain\_size, int coset\_size);
void init(SyncedMemory input, int lg\_domain\_size, int chunk\_id, bool is\_intt = false, cudaStream\_t stream = (cudaStream\_t)0);
void init\_and\_forward(SyncedMemory input, SyncedMemory output, int lg\_domain\_size, int chunk\_id, int lambda, int stage, bool is\_intt, cudaStream\_t stream = (cudaStream\_t)0);
void forward1(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void forward2(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void forward3(SyncedMemory input, SyncedMemory output, int lg\_domain\_size, int stage, int chunk\_id, int lambda, cudaStream\_t stream = (cudaStream\_t)0);
};
class Intt\_coset {
public:
SyncedMemory Params;
int lg\_domain\_size;
Intt\_coset(int domain\_size, int coset\_size, cudaStream\_t stream = (cudaStream\_t)0);
void init(SyncedMemory input, int lg\_domain\_size, int chunk\_id, bool is\_intt = true, cudaStream\_t stream = (cudaStream\_t)0);
void forward1(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void forward2(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream = (cudaStream\_t)0);
void forward3(SyncedMemory input, SyncedMemory output, int lg\_domain\_size, int stage, int chunk\_id, int lambda, cudaStream\_t stream = (cudaStream\_t)0);
};
SyncedMemory multi\_scalar\_mult(SyncedMemory points, SyncedMemory scalars, cudaStream\_t stream = (cudaStream\_t)0);
bool gt\_zkp(SyncedMemory a, SyncedMemory b);
void compress(SyncedMemory output, SyncedMemory t\_0, SyncedMemory t\_1, SyncedMemory t\_2, SyncedMemory t\_3,
SyncedMemory challenge, cudaStream\_t = (cudaStream\_t)0);
void compute\_query\_table(std::vector f\_chunks, std::vector global\_buffer, std::vector q\_lookup,
std::vector w\_l\_scalar, std::vector w\_r\_scalar, std::vector w\_o\_scalar,
std::vector w\_4\_scalar, std::vector t\_poly, SyncedMemory challenge, int chunk\_num,
cudaStream\_t stream1 = (cudaStream\_t)0, cudaStream\_t stream2 = (cudaStream\_t)0);
void writeToFile(const std::string& filename, uint64\_t\* array, uint64\_t size);
uint32\_t bit\_rev(uint32\_t i, uint32\_t nbits);
void calculate\_map1(uint32\_t\* map, int lg\_N);
template 
void bit\_rev\_step1\_worker(
fr\* d\_out,
fr\* d\_in,
uint32\_t\* map,
uint64\_t idx,
uint64\_t start\_idx,
uint64\_t end\_idx) {
for (uint64\_t i = start\_idx; i < end\_idx; ++i) {
d\_out[map[idx + i]] = d\_in[i];
}
}
template 
void pad\_transpose\_worker(
fr\* d\_out,
const fr\* d\_in,
uint64\_t start\_idx,
uint64\_t end\_idx) {
for (uint64\_t i = start\_idx; i < end\_idx; ++i) {
d\_out[i << 3] = d\_in[i];
}
}
template 
void bit\_rev\_step2\_worker(
fr\* d\_out,
fr\* d\_in,
uint32\_t\* map,
uint64\_t idx,
uint64\_t start\_idx,
uint64\_t end\_idx) {
for (uint64\_t i = start\_idx; i < end\_idx; ++i) {
d\_out[map[idx+i]] = d\_in[i];
}
}
template 
void bit\_rev\_step2\_worker\_chunk(
std::vector d\_out,
fr\* d\_in,
uint32\_t\* map,
uint64\_t idx,
uint64\_t start\_idx,
uint64\_t end\_idx,
uint64\_t bound) {
for (uint64\_t i = start\_idx; i < end\_idx; ++i) {
uint32\_t index = map[idx+i];
int target\_out = index/bound;
fr\* out\_ = reinterpret\_cast(d\_out[target\_out].mutable\_cpu\_data());
out\_[index%bound] = d\_in[i];
}
}
// Multithreaded version of bit\_rev\_permutation
template 
void bit\_rev\_step1\_parallel(
fr\* d\_out,
fr\* d\_in,
uint32\_t\* map,
int lg\_N,
int num\_threads,
uint64\_t idx) {
uint64\_t N = 1 << lg\_N;
uint32\_t chunk\_size = N / num\_threads;
std::vector threads;
for (uint32\_t t = 0; t < num\_threads; ++t) {
uint64\_t start\_idx = t \* chunk\_size;
uint64\_t end\_idx = (t == num\_threads - 1) ? N : start\_idx + chunk\_size;
threads.push\_back(std::thread(bit\_rev\_step1\_worker, d\_out, d\_in, map, idx, start\_idx, end\_idx));
}
for (auto& t : threads) {
t.join();
}
}
template 
void bit\_rev\_step2\_parallel(
fr\* d\_out,
fr\* d\_in,
uint32\_t\* map,
int lg\_N,
int num\_threads,
uint64\_t idx) {
uint64\_t N = 1 << lg\_N;
uint32\_t chunk\_size = N / num\_threads;
std::vector threads;
for (uint32\_t t = 0; t < num\_threads; ++t) {
uint64\_t start\_idx = t \* chunk\_size;
uint64\_t end\_idx = (t == num\_threads - 1) ? N : start\_idx + chunk\_size;
threads.push\_back(std::thread(bit\_rev\_step2\_worker, d\_out, d\_in, map, idx, start\_idx, end\_idx));
}
for (auto& t : threads) {
t.join();
}
}
template 
void bit\_rev\_step2(
fr\* d\_out,
fr\* d\_in,
uint32\_t\* map,
int lg\_N,
uint64\_t idx) {
uint64\_t N = 1 << lg\_N;
bit\_rev\_step2\_worker(d\_out, d\_in, map, idx, 0, N-1);
}