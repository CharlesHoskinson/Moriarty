# pipelonk miss_ops.cu (ae branch)

> Source: https://raw.githubusercontent.com/zlyber/pipelonk/ae/miss_ops.cu
> Collected: 2026-10-26
> Published: Unknown

#include 
#include 
#include 
#include 
#include "caffe/interface.hpp"
#include "PLONK/utils/function.cuh"
void read\_file(const char\* filename, void\* data){
// 打开文件
std::ifstream file(filename, std::ios::binary);
if (!file) {
std::cerr << "Error opening file: " << filename << std::endl;
return;
}
file.seekg(0, std::ios::end);
size\_t fileSize = file.tellg();
file.seekg(0, std::ios::beg);
file.read(reinterpret\_cast(data), fileSize);
if (!file) {
std::cerr << "Error reading file: " << filename << std::endl;
}
file.close();
}
SyncedMemory one() {
SyncedMemory one(4 \* sizeof(uint64\_t));
void\* one\_ = one.mutable\_cpu\_data();
uint64\_t fr\_one[4] = {8589934590UL, 6378425256633387010UL, 11064306276430008309UL, 1739710354780652911UL};
memcpy(one\_,fr\_one,one.size());
return one;
}
SyncedMemory naive\_gp(int lg\_N, SyncedMemory input){
uint64\_t N = 1<(input.mutable\_cpu\_data());
cpu::BLS12\_381\_Fr\_G1\* output\_ = reinterpret\_cast(output.mutable\_cpu\_data());
SyncedMemory state = one();
memcpy((void\*)output\_, state.mutable\_cpu\_data(), state.size());
cpu::BLS12\_381\_Fr\_G1\* state\_ = reinterpret\_cast(state.mutable\_cpu\_data());
for (size\_t i = 0; i < N - 1; ++i) {
state\_[0] \*= input\_[i];
memcpy(output\_ + i, state.mutable\_cpu\_data(), state.size());
}
return output;
}
SyncedMemory naive\_div(int lg\_N, SyncedMemory input, SyncedMemory divisor){
size\_t N = 1<(quotient.mutable\_cpu\_data());
memcpy(remainder.mutable\_cpu\_data(), input.mutable\_cpu\_data(), input.size());
for (size\_t i = 0; i < q\_len; ++i) {
memcpy(quotient\_ + i, zero, 4\*sizeof(uint64\_t));
}
cpu::BLS12\_381\_Fr\_G1\* divisor\_ = reinterpret\_cast(divisor.mutable\_cpu\_data());
cpu::BLS12\_381\_Fr\_G1 divisor\_leading = divisor\_[1];
cpu::BLS12\_381\_Fr\_G1 divisor\_leading\_inv = 1/divisor\_leading;
cpu::BLS12\_381\_Fr\_G1\* remainder\_ = reinterpret\_cast(remainder.mutable\_cpu\_data());
//divisor\_len = 2
while (remainder\_len >= 2){
cpu::BLS12\_381\_Fr\_G1 remainder\_leading = remainder\_[remainder\_len - 1];
cpu::BLS12\_381\_Fr\_G1 cur\_q\_coeff = remainder\_leading \* divisor\_leading\_inv;
size\_t cur\_q\_degree = remainder\_len - 2;
quotient\_[cur\_q\_degree] = cur\_q\_coeff;
for (size\_t i = 0; i < 2; ++i){
cpu::BLS12\_381\_Fr\_G1 div\_coeff = divisor\_[i];
cpu::BLS12\_381\_Fr\_G1 temp = cur\_q\_coeff \* div\_coeff;
remainder\_[cur\_q\_degree + i] -= temp;
}
while (remainder\_[remainder\_len - 1].is\_zero()){
remainder\_len --;
}
}
return quotient;
}
int main() {
int lg\_N = 22;
uint64\_t N = 1<(end - start).count();
std::cout << "[naive\_gp] execution time: " << duration\_ms << " ms" << std::endl;
}
for (int rep = 0; rep < 3; ++rep) {
read\_file(file\_path, inp\_);
void\* d\_inp\_ = input.mutable\_gpu\_data();
std::cout << "[ours] run " << (rep + 1) << "/3" << std::endl;
cudaEvent\_t start, stop;
cudaEventCreate(&start);
cudaEventCreate(&stop);
cudaEventRecord(start, 0);
SyncedMemory out = accumulate\_mul\_poly(input);
cudaEventRecord(stop, 0);
cudaEventSynchronize(stop);
float milliseconds = 0;
cudaEventElapsedTime(&milliseconds, start, stop);
std::cout << "[ours\_gp] execution time: " << milliseconds << " ms" << std::endl;
cudaEventDestroy(start);
cudaEventDestroy(stop);
}
// bench poly evaluation
SyncedMemory point(4\*sizeof(uint64\_t)); //random choose
uint64\_t pointt[4] = {123456789UL, 987654321UL, 1122334455UL, 6677889900UL};
memcpy(point.mutable\_cpu\_data(), pointt, point.size());
std::cout << "test poly evaluation" << std::endl;
for (int rep = 0; rep < 3; ++rep) {
read\_file(file\_path, inp\_);
void\* d\_inp\_ = input.mutable\_gpu\_data();
void\* d\_point\_ = point.mutable\_gpu\_data();
std::cout << "[ours] run " << (rep + 1) << "/3" << std::endl;
cudaEvent\_t start, stop;
cudaEventCreate(&start);
cudaEventCreate(&stop);
cudaEventRecord(start, 0);
SyncedMemory out = evaluate(input, point);
cudaEventRecord(stop, 0);
cudaEventSynchronize(stop);
float milliseconds = 0;
cudaEventElapsedTime(&milliseconds, start, stop);
std::cout << "[ours\_eval] execution time: " << milliseconds << " ms" << std::endl;
cudaEventDestroy(start);
cudaEventDestroy(stop);
}
// bench poly division
std::cout << "test poly division" << std::endl;
SyncedMemory divisor(8\*sizeof(uint64\_t)); //x-z, z can be random
uint64\_t divisorr[8] = {123456789123456789UL, 987654321987654321UL, 1122334455667788UL, 1122334455667788UL, 8589934590UL, 6378425256633387010UL, 11064306276430008309UL, 1739710354780652911UL};
memcpy(divisor.mutable\_cpu\_data(), divisorr, divisor.size());
for (int rep = 0; rep < 3; ++rep) {
inp\_ = input.mutable\_cpu\_data();
read\_file(file\_path, inp\_);
std::cout << "[naive] run " << (rep + 1) << "/3" << std::endl;
auto start = std::chrono::high\_resolution\_clock::now();
SyncedMemory out = naive\_div(lg\_N, input, divisor);
auto end = std::chrono::high\_resolution\_clock::now();
auto duration\_ms =
std::chrono::duration\_cast(end - start).count();
std::cout << "[naive\_div] execution time: " << duration\_ms << " ms" << std::endl;
}
SyncedMemory divisor\_gpu(4\*sizeof(uint64\_t));
uint64\_t divisorr\_gpu[4] = {123456789123456789UL, 987654321987654321UL, 1122334455667788UL, 1122334455667788UL};
memcpy(divisor\_gpu.mutable\_cpu\_data(), divisorr\_gpu, divisor\_gpu.size());
void\* d\_divisor\_ = divisor\_gpu.mutable\_gpu\_data();
for (int rep = 0; rep < 3; ++rep) {
read\_file(file\_path, inp\_);
void\* d\_inp\_ = input.mutable\_gpu\_data();
std::cout << "[ours] run " << (rep + 1) << "/3" << std::endl;
cudaEvent\_t start, stop;
cudaEventCreate(&start);
cudaEventCreate(&stop);
cudaEventRecord(start, 0);
SyncedMemory out = poly\_div\_poly(input, divisor\_gpu);
cudaEventRecord(stop, 0);
cudaEventSynchronize(stop);
float milliseconds = 0;
cudaEventElapsedTime(&milliseconds, start, stop);
std::cout << "[ours\_div] execution time: " << milliseconds << " ms" << std::endl;
cudaEventDestroy(start);
cudaEventDestroy(stop);
}
return 0;
}