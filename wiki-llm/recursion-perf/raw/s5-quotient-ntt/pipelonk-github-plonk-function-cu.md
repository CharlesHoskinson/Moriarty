# pipelonk PLONK/utils/function.cu (ae branch)

> Source: https://raw.githubusercontent.com/zlyber/pipelonk/ae/PLONK/utils/function.cu
> Collected: 2026-10-26
> Published: Unknown

#include "function.cuh"
#include 
#include 
SyncedMemory to\_mont(SyncedMemory input) {
if(input.head() == SyncedMemory::HEAD\_AT\_CPU){
return cpu::to\_mont\_cpu(input);
}
else{
return cuda::to\_mont\_cuda(input);
}
}
SyncedMemory to\_base(SyncedMemory input, cudaStream\_t stream) {
if(input.head() == SyncedMemory::HEAD\_AT\_CPU){
return cpu::to\_base\_cpu(input);
}
else{
return cuda::to\_base\_cuda(input, stream);
}
}
SyncedMemory neg\_mod(SyncedMemory input) {
if(input.head() == SyncedMemory::HEAD\_AT\_CPU){
return cpu::neg\_mod\_cpu(input);
}
else{
return cuda::neg\_mod\_cuda(input);
}
}
SyncedMemory inv\_mod(SyncedMemory input) {
if(input.head() == SyncedMemory::HEAD\_AT\_CPU){
return cpu::inv\_mod\_cpu(input);
}
else{
return cuda::inv\_mod\_cuda(input);
}
}
SyncedMemory add\_mod(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream) {
if(input1.head() == SyncedMemory::HEAD\_AT\_CPU){
return cpu::add\_mod\_cpu(input1, input2);
}
else{
return cuda::add\_mod\_cuda(input1, input2, stream);
}
}
void add\_mod\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream) {
if(input1.head() == SyncedMemory::HEAD\_AT\_CPU){
cpu::add\_mod\_cpu\_(input1, input2);
}
else{
cuda::add\_mod\_cuda\_(input1, input2, stream);
}
}
SyncedMemory sub\_mod(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream) {
if(input1.head() == SyncedMemory::HEAD\_AT\_CPU){
return cpu::sub\_mod\_cpu(input1, input2);
}
else{
return cuda::sub\_mod\_cuda(input1, input2, stream);
}
}
void sub\_mod\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream) {
if(input1.head() == SyncedMemory::HEAD\_AT\_CPU){
cpu::sub\_mod\_cpu\_(input1, input2);
}
else{
cuda::sub\_mod\_cuda\_(input1, input2, stream);
}
}
SyncedMemory mul\_mod(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream) {
if(input1.head() == SyncedMemory::HEAD\_AT\_CPU){
return cpu::mul\_mod\_cpu(input1, input2);
}
else{
return cuda::mul\_mod\_cuda(input1, input2, stream);
}
}
void mul\_mod\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream) {
if(input1.head() == SyncedMemory::HEAD\_AT\_CPU){
cpu::mul\_mod\_cpu\_(input1, input2);
}
else{
cuda::mul\_mod\_cuda\_(input1, input2, stream);
}
}
SyncedMemory div\_mod(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream) {
if(input1.head() == SyncedMemory::HEAD\_AT\_CPU){
return cpu::div\_mod\_cpu(input1, input2);
}
else{
return cuda::div\_mod\_cuda(input1, input2, stream);
}
}
void div\_mod\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream) {
if(input1.head() == SyncedMemory::HEAD\_AT\_CPU){
cpu::div\_mod\_cpu\_(input1, input2);
}
else{
cuda::div\_mod\_cuda\_(input1, input2, stream);
}
}
SyncedMemory exp\_mod(SyncedMemory input, int exp) {
if(input.head() == SyncedMemory::HEAD\_AT\_CPU){
return cpu::exp\_mod\_cpu(input, exp);
}
else{
return cuda::exp\_mod\_cuda(input, exp);
}
}
SyncedMemory add\_mod\_scalar(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream){
return cuda::add\_mod\_scalar\_cuda(input1, input2, stream);
}
void add\_mod\_scalar\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream){
cuda::add\_mod\_scalar\_cuda\_(input1, input2, stream);
}
SyncedMemory sub\_mod\_scalar(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream){
return cuda::sub\_mod\_scalar\_cuda(input1, input2, stream);
}
void sub\_mod\_scalar\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream){
cuda::sub\_mod\_scalar\_cuda\_(input1, input2, stream);
}
SyncedMemory mul\_mod\_scalar(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream){
return cuda::mul\_mod\_scalar\_cuda(input1, input2, stream);
}
void mul\_mod\_scalar\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream){
cuda::mul\_mod\_scalar\_cuda\_(input1, input2, stream);
}
SyncedMemory div\_mod\_scalar(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream){
return cuda::div\_mod\_scalar\_cuda(input1, input2, stream);
}
void div\_mod\_scalar\_(SyncedMemory input1, SyncedMemory input2, cudaStream\_t stream){
cuda::div\_mod\_scalar\_cuda\_(input1, input2, stream);
}
SyncedMemory gen\_sequence(uint64\_t N, SyncedMemory x, cudaStream\_t stream){
return cuda::poly\_eval\_cuda(x, N, stream);
}
void gen\_sequence\_(uint64\_t N, SyncedMemory x, SyncedMemory y, cudaStream\_t stream){
cuda::poly\_eval\_cuda\_(x, y, N, stream);
}
SyncedMemory repeat\_to\_poly(SyncedMemory x, uint64\_t N, cudaStream\_t stream){
return cuda::repeat\_to\_poly\_cuda(x, N, stream);
}
void repeat\_to\_poly\_(SyncedMemory x, SyncedMemory y, uint64\_t N, cudaStream\_t stream){
cuda::repeat\_to\_poly\_cuda\_(x, y, N, stream);
}
SyncedMemory evaluate(SyncedMemory poly, SyncedMemory x){
if (poly.size() == 0){
SyncedMemory result(4 \* sizeof(uint64\_t));
void\* res\_gpu = result.mutable\_gpu\_data();
cudaMemset(res\_gpu, 0, 4 \* sizeof(uint64\_t));
return result;
}
else{
SyncedMemory y = cuda::poly\_eval\_cuda(x, poly.size()/ (cuda::fr\_LIMBS\*sizeof(uint64\_t)));
return cuda::poly\_reduce\_cuda(y, poly);
}
}
SyncedMemory poly\_div\_poly(SyncedMemory divid, SyncedMemory c){
return cuda::poly\_div\_cuda(divid,c);
}
SyncedMemory pad\_poly(SyncedMemory x, uint64\_t N, cudaStream\_t stream){
if(x.head() == SyncedMemory::HEAD\_AT\_CPU){
return cpu::pad\_poly\_cpu(x, N);
}
else{
return cuda::pad\_poly\_cuda(x, N, stream);
}
}
void pad\_poly\_(SyncedMemory x, SyncedMemory out, uint64\_t N, cudaStream\_t stream){
if(x.head() == SyncedMemory::HEAD\_AT\_CPU)
cpu::pad\_poly\_cpu\_(x, out, N);
else
cuda::pad\_poly\_cuda\_(x, out, N, stream);
}
SyncedMemory repeat\_zero(uint64\_t N){
SyncedMemory out(N \* cpu::fr\_LIMBS \* sizeof(uint64\_t));
void\* out\_ = out.mutable\_cpu\_data();
memset(out\_, 0, out.size());
return out;
}
SyncedMemory cat(SyncedMemory a, SyncedMemory b){
SyncedMemory res(a.size() + b.size());
if(a.head() == SyncedMemory::HEAD\_AT\_CPU){
void\* res\_ = res.mutable\_cpu\_data();
void\* a\_ = a.mutable\_cpu\_data();
void\* b\_ = b.mutable\_cpu\_data();
memcpy(res\_, a\_, a.size());
memcpy(res\_ + a.size(), b\_, b.size());
}
else{
void\* res\_ = res.mutable\_gpu\_data();
void\* a\_ = a.mutable\_gpu\_data();
void\* b\_ = b.mutable\_gpu\_data();
caffe\_gpu\_memcpy(a.size(), a\_, res\_);
caffe\_gpu\_memcpy(b.size(), b\_, res\_ + a.size());
}
return res;
}
SyncedMemory slice(SyncedMemory a, uint64\_t len, bool forward){
if(a.head() == SyncedMemory::HEAD\_AT\_CPU){
SyncedMemory res(cpu::fr\_LIMBS \* len \* sizeof(uint64\_t));
void\* res\_ = res.mutable\_cpu\_data();
void\* a\_ = a.mutable\_cpu\_data();
if(forward){
memcpy(res\_, a\_, res.size());
}
else{
memcpy(res\_, a\_ + a.size()-res.size(), res.size());
}
return res;
}
else{
SyncedMemory res(cuda::fr\_LIMBS \* len \* sizeof(uint64\_t));
void\* res\_ = res.mutable\_gpu\_data();
void\* a\_ = a.mutable\_gpu\_data();
if(forward){
caffe\_gpu\_memcpy(res.size(), a\_, res\_);
}
else{
caffe\_gpu\_memcpy(res.size(), a\_ + a.size()-res.size(), res\_);
}
return res;
}
}
SyncedMemory accumulate\_mul\_poly(SyncedMemory product, cudaStream\_t stream){
return cuda::accumulate\_mul\_poly\_cuda(product, stream);
}
void accumulate\_mul\_poly\_(SyncedMemory product, SyncedMemory output, cudaStream\_t stream){
cuda::accumulate\_mul\_poly\_cuda\_(product, output, stream);
}
SyncedMemory make\_tensor(SyncedMemory input, uint64\_t pad\_len){
return cuda::make\_tensor(input, pad\_len);
}
void lookup\_ratio\_step1\_(SyncedMemory h\_1, SyncedMemory h\_2, SyncedMemory h\_1\_next,
SyncedMemory one, SyncedMemory delta, SyncedMemory epsilon, SyncedMemory result, cudaStream\_t stream){
cuda::lookup\_ratio\_step1\_cuda\_(h\_1, h\_2, h\_1\_next, one, delta, epsilon, result, stream);
}
void lookup\_ratio\_step2\_(SyncedMemory f, SyncedMemory t, SyncedMemory t\_next,
SyncedMemory one, SyncedMemory delta, SyncedMemory epsilon, SyncedMemory result, cudaStream\_t stream){
cuda::lookup\_ratio\_step2\_cuda\_(f, t, t\_next, one, delta, epsilon, result, stream);
}
Ntt::Ntt(int domain\_size, cudaStream\_t stream): Params(cuda::params\_zkp\_cuda(domain\_size, false, stream)) {}
SyncedMemory Ntt::forward(SyncedMemory input, cudaStream\_t stream) {
return cuda::ntt\_zkp\_cuda(input, Params, false, stream);
}
void Ntt::forward\_(SyncedMemory input, SyncedMemory output, cudaStream\_t stream) {
cuda::ntt\_zkp\_cuda(input, output, Params, false, stream);
}
void Ntt::forward1(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream) {
cuda::ntt\_zkp\_step1\_cuda(input, Params, lg\_domain\_size, false, stage, chunk\_id, stream);
}
void Ntt::forward1\_kcolumn(SyncedMemory input, int lg\_domain\_size, int i\_size, int k, int\* stage, cudaStream\_t stream) {
cuda::ntt\_kcolumn\_step1\_cuda(input, Params, lg\_domain\_size, i\_size, k, false, stage, stream);
}
void Ntt::forward1\_kcolumn\_raw(uint64\_t\* input, int lg\_domain\_size, int64\_t numel, int i\_size, int k, int\* stage, cudaStream\_t stream) {
cuda::ntt\_kcolumn\_step1\_cuda\_raw(input, Params, numel, lg\_domain\_size, i\_size, k, false, stage, stream);
}
void Ntt::forward2(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream) {
cuda::ntt\_zkp\_step2\_cuda(input, Params, lg\_domain\_size, false, stage, chunk\_id, stream);
}
void Ntt::forward2\_kcolumn(SyncedMemory input, int lg\_domain\_size, int j\_size, int k, int\* stage, cudaStream\_t stream) {
cuda::ntt\_kcolumn\_step2\_cuda(input, Params, lg\_domain\_size, j\_size, k, false, stage, stream);
}
void Ntt::forward3(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream) {
cuda::ntt\_zkp\_step3\_cuda(input, Params, lg\_domain\_size, false, stage, chunk\_id, stream);
}
void Ntt::forward1\_internal(SyncedMemory input, SyncedMemory output, int\* stage, cudaStream\_t stream) {
cuda::ntt\_zkp\_step1\_internal\_cuda(input, output, Params, false, stage, stream);
}
void Ntt::forward2\_internal(SyncedMemory inout, int\* stage, cudaStream\_t stream) {
cuda::ntt\_zkp\_step2\_internal\_cuda(inout, Params, false, stage, stream);
}
void Ntt::forward3\_internal(SyncedMemory inout, int\* stage, cudaStream\_t stream) {
cuda::ntt\_zkp\_step3\_internal\_cuda(inout, Params, false, stage, stream);
}
Intt::Intt(int domain\_size, cudaStream\_t stream): Params(cuda::params\_zkp\_cuda(domain\_size, true, stream)) {}
SyncedMemory Intt::forward(SyncedMemory input, cudaStream\_t stream) {
return cuda::ntt\_zkp\_cuda(input, Params, true, stream);
}
void Intt::forward\_(SyncedMemory input, SyncedMemory output, cudaStream\_t stream) {
cuda::ntt\_zkp\_cuda(input, output, Params, true, stream);
}
void Intt::\_forward\_(SyncedMemory input, cudaStream\_t stream) {
cuda::ntt\_zkp\_cuda\_(input, Params, true, stream);
}
void Intt::forward1(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream) {
cuda::ntt\_zkp\_step1\_cuda(input, Params, lg\_domain\_size, true, stage, chunk\_id, stream);
}
void Intt::forward2(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream) {
cuda::ntt\_zkp\_step2\_cuda(input, Params, lg\_domain\_size, true, stage, chunk\_id, stream);
}
void Intt::forward3(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream) {
cuda::ntt\_zkp\_step3\_cuda(input, Params, lg\_domain\_size, true, stage, chunk\_id, stream);
}
Ntt\_coset::Ntt\_coset(int domain\_size, int coset\_size)
:lg\_domain\_size(coset\_size), Params(cuda::params\_zkp\_cuda(domain\_size, false)) {}
void Ntt\_coset::init(SyncedMemory input, int lg\_domain\_size, int chunk\_id, bool is\_intt, cudaStream\_t stream) {
cuda::ntt\_zkp\_coset\_init\_cuda(input, Params, lg\_domain\_size, is\_intt, chunk\_id, stream);
}
void Ntt\_coset::init\_and\_forward(SyncedMemory input, SyncedMemory output, int lg\_domain\_size, int chunk\_id, int lambda, int stage, bool is\_intt, cudaStream\_t stream) {
cuda::ntt\_zkp\_coset\_init\_and\_step1\_cuda(input, output, Params, lg\_domain\_size, chunk\_id, lambda, stage, is\_intt, stream);
}
void Ntt\_coset::forward1(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream) {
cuda::ntt\_zkp\_coset\_step1\_cuda(input, Params, lg\_domain\_size, false, stage, chunk\_id, stream);
}
void Ntt\_coset::forward2(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream) {
cuda::ntt\_zkp\_coset\_step2\_cuda(input, Params, lg\_domain\_size, false, stage, chunk\_id, stream);
}
void Ntt\_coset::forward3(SyncedMemory input, SyncedMemory output, int lg\_domain\_size, int stage, int chunk\_id, int lambda, cudaStream\_t stream) {
cuda::ntt\_zkp\_coset\_step3\_cuda(input, output, Params, lg\_domain\_size, false, stage, chunk\_id, lambda, stream);
}
Intt\_coset::Intt\_coset(int domain\_size, int coset\_size, cudaStream\_t stream)
:lg\_domain\_size(coset\_size), Params(cuda::params\_zkp\_cuda(domain\_size, true, stream)) {}
void Intt\_coset::init(SyncedMemory input, int lg\_domain\_size, int chunk\_id, bool is\_intt, cudaStream\_t stream) {
cuda::ntt\_zkp\_coset\_init\_cuda(input, Params, lg\_domain\_size, is\_intt, chunk\_id, stream);
}
void Intt\_coset::forward1(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream) {
cuda::ntt\_zkp\_coset\_step1\_cuda(input, Params, lg\_domain\_size, true, stage, chunk\_id, stream);
}
void Intt\_coset::forward2(SyncedMemory input, int lg\_domain\_size, int stage, int chunk\_id, cudaStream\_t stream) {
cuda::ntt\_zkp\_coset\_step2\_cuda(input, Params, lg\_domain\_size, true, stage, chunk\_id, stream);
}
void Intt\_coset::forward3(SyncedMemory input, SyncedMemory output, int lg\_domain\_size, int stage, int chunk\_id, int lambda, cudaStream\_t stream) {
cuda::ntt\_zkp\_coset\_step3\_cuda(input, output, Params, lg\_domain\_size, true, stage, chunk\_id, lambda, stream);
}
SyncedMemory multi\_scalar\_mult(SyncedMemory points, SyncedMemory scalars, cudaStream\_t stream){
int64\_t point\_num = points.size()/(sizeof(uint64\_t) \* cpu::fq\_LIMBS);
int64\_t scalar\_num = scalars.size()/(sizeof(uint64\_t) \* cpu::fr\_LIMBS);
int64\_t msm\_size = std::min(point\_num, scalar\_num);
int device;
cudaError\_t err = cudaGetDevice(&device);
cudaDeviceProp deviceProp;
err = cudaGetDeviceProperties(&deviceProp, device);
int smcount = deviceProp.multiProcessorCount;
SyncedMemory step1\_res = cuda::msm\_zkp\_cuda(points, scalars, smcount, stream);
void\* step1\_cpu\_ = step1\_res.mutable\_cpu\_data\_async(stream);
SyncedMemory step2\_res = cpu::msm\_collect\_cpu(step1\_res, msm\_size);
return step2\_res;
}
void writeToFile(const std::string& filename, uint64\_t\* array, uint64\_t size) {
std::ofstream outfile(filename);
if (!outfile) {
std::cerr << "Error opening file for writing: " << filename << std::endl;
return;
}
// 写入数组到文本文件，每个元素占一行
for (uint64\_t i = 0; i < size; ++i) {
outfile << array[i] << std::endl;
}
if (!outfile) {
std::cerr << "Error writing to file: " << filename << std::endl;
}
outfile.close();
}
void rotate\_indices(uint32\_t& idx0, uint32\_t& idx1, int stage, int iterations) {
// rotate "iterations" bits in indices
uint32\_t mask = ((uint32\_t)1 << (stage + iterations)) - ((uint32\_t)1 << stage);
// Rotate idx0
uint32\_t rotw = idx0 & mask;
rotw = (rotw >> 1) | (rotw << (iterations - 1));
idx0 = (idx0 & ~mask) | (rotw & mask);
// Rotate idx1
rotw = idx1 & mask;
rotw = (rotw >> 1) | (rotw << (iterations - 1));
idx1 = (idx1 & ~mask) | (rotw & mask);
}
void calculate\_indices(uint32\_t tid, int stage1, int iterations1, int stage2, int iterations2, uint32\_t\* map) {
const uint32\_t out\_ntt\_size1 = (1 << (stage1 + iterations1 - 1));
const uint32\_t inp\_ntt\_size1 = (uint32\_t)1 << stage1;
const uint32\_t thread\_ntt\_pos1 = (tid & (out\_ntt\_size1 - 1)) >> (iterations1 - 1);
uint32\_t idx1\_0 = tid & ~(out\_ntt\_size1 - 1);
idx1\_0 += (tid << stage1) & (out\_ntt\_size1 - 1);
idx1\_0 = idx1\_0 \* 2 + thread\_ntt\_pos1;
uint32\_t idx1\_1 = idx1\_0 + inp\_ntt\_size1;
const uint32\_t out\_ntt\_size2 = (1 << (stage2 + iterations2 - 1));
const uint32\_t inp\_ntt\_size2 = (uint32\_t)1 << stage2;
const uint32\_t thread\_ntt\_pos2 = (tid & (out\_ntt\_size2 - 1)) >> (iterations2 - 1);
uint32\_t idx2\_0 = tid & ~(out\_ntt\_size2 - 1);
idx2\_0 += (tid << stage2) & (out\_ntt\_size2 - 1);
idx2\_0 = idx2\_0 \* 2 + thread\_ntt\_pos2;
uint32\_t idx2\_1 = idx2\_0 + inp\_ntt\_size2;
map[idx1\_0] = idx2\_0;
map[idx1\_1] = idx2\_1;
}
// Function to reverse bits for an unsigned integer type
uint32\_t bit\_rev\_32(uint32\_t i, uint32\_t nbits) {
uint32\_t rev = 0;
for (uint32\_t j = 0; j < nbits; ++j) {
rev = (rev << 1) | (i & 1);
i >>= 1;
}
return rev;
}
// Overload for long long integer (64-bit)
uint64\_t bit\_rev\_64(uint64\_t i, uint32\_t nbits) {
uint64\_t rev = 0;
for (uint32\_t j = 0; j < nbits; ++j) {
rev = (rev << 1) | (i & 1);
i >>= 1;
}
return rev;
}
uint32\_t bit\_rev(uint32\_t i, uint32\_t nbits) {
if (sizeof(i) == 4 || nbits <= 32) {
return bit\_rev\_32((i), nbits);
} else {
return bit\_rev\_64((i), nbits);
}
}
void calculate\_map1(uint32\_t\* map, int lg\_N) {
uint64\_t N = 1 << lg\_N;
for(int i = 0; i< N; i++){
uint32\_t r = bit\_rev(i, lg\_N);
map[r] = i;
}
}