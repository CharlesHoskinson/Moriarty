#include 
#include 
#include "caffe/interface.hpp"
#include "PLONK/utils/function.cuh"
#include 
#include 
#include 
#include 
#include 
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
template 
void transpose\_block(fr\* input, fr\* output, size\_t start\_row, size\_t end\_row, size\_t rows, size\_t cols) {
for (size\_t i = start\_row; i < end\_row; ++i) {
for (size\_t j = 0; j < cols; ++j) {
output[j \* rows + i] = input[i \* cols + j];
}
}
}
template 
void transpose\_block\_chunk(fr\* input, fr\* output, size\_t start\_row, size\_t end\_row, size\_t rows, size\_t cols, size\_t idx) {
for (size\_t i = start\_row; i < end\_row; ++i) {
for (size\_t j = 0; j < cols; ++j) {
output[idx+ j \* rows + i] = input[i \* cols + j];
}
}
}
template 
void transpose\_matrix(fr\* input, fr\* output, size\_t rows, size\_t cols, size\_t num\_threads) {
size\_t rows\_per\_thread = rows / num\_threads;
std::vector threads;
for (size\_t i = 0; i < num\_threads; ++i) {
size\_t start\_row = i \* rows\_per\_thread;
size\_t end\_row = (i == num\_threads - 1) ? rows : (i + 1) \* rows\_per\_thread;
threads.push\_back(std::thread(transpose\_block, input, output, start\_row, end\_row, rows, cols));
}
for (auto& t : threads) {
t.join();
}
}
void naive\_ntt(SyncedMemory input, int lg\_N, int lg\_i, int lg\_j, int lg\_k, int num\_threads, Ntt ntt)
{
uint32\_t N = 1<(in\_), reinterpret\_cast(in\_), I, J, num\_threads);
cudaEventRecord(stop, 0);
cudaEventSynchronize(stop);
float milliseconds = 0;
cudaEventElapsedTime(&milliseconds, start, stop);
std::cout << "naive-ntt execution time: " << milliseconds << " ms" << std::endl;
cudaEventDestroy(start);
cudaEventDestroy(stop);
}
void our\_ntt(SyncedMemory input, int lg\_N, int lg\_i, int lg\_j, int lg\_k, int step1\_thread, int step2\_thread, Ntt ntt)
{
uint32\_t N = 1<(inp\_), reinterpret\_cast(in\_), step1\_map, step1\_thread, lg\_N, 0);
for(int i = 0; i < J\*2/k;i+=2){
int stage = 0;
cudaMemcpyAsync(d\_in\_, inp\_ + d\_in.size()\*i, d\_in.size(), cudaMemcpyHostToDevice, stream1);
cudaEventRecord(even1, stream1);
cudaStreamWaitEvent(stream2, even1);
ntt.forward1(d\_in, lg\_N, 0, i, stream1);
cudaMemcpyAsync(d\_in\_2\_, inp\_ + d\_in.size()\*(i+1), d\_in\_2.size(), cudaMemcpyHostToDevice, stream2);
cudaEventRecord(even2, stream2);
ntt.forward1(d\_in\_2, lg\_N, 0, i+1, stream2);
cudaStreamWaitEvent(stream1, even2);
cudaMemcpyAsync(step1\_out\_ + d\_in.size()\*i, d\_in\_, d\_in.size(), cudaMemcpyDeviceToHost, stream1);
cudaMemcpyAsync(step1\_out\_ + d\_in\_2.size()\*(i+1), d\_in\_2\_, d\_in\_2.size(), cudaMemcpyDeviceToHost, stream2);
bit\_rev\_step2\_parallel(reinterpret\_cast(step2\_in\_), reinterpret\_cast(step1\_out\_ + i\*d\_in.size()), step2\_map, lg\_i+lg\_k-1, step2\_thread, i\*I\*k/2);
}
cudaDeviceSynchronize();
for(int i = 0; i < J\*2/k;i+=2){
bit\_rev\_step2\_parallel(reinterpret\_cast(step2\_in\_), reinterpret\_cast(step1\_out\_+ (i+1)\*d\_in.size()), step2\_map, lg\_i+lg\_k -1, step2\_thread, (i+1)\*I\*k/2);
cudaMemcpyAsync(d\_in\_, step2\_in\_ + d\_in.size()\*i, d\_in.size(), cudaMemcpyHostToDevice, stream1);
cudaEventRecord(even1, stream1);
cudaStreamWaitEvent(stream2, even1);
ntt.forward2(d\_in, lg\_N, 8, i, stream1);
cudaMemcpyAsync(d\_in\_2\_, step2\_in\_ + d\_in.size()\*(i+1), d\_in\_2.size(), cudaMemcpyHostToDevice, stream2);
cudaEventRecord(even2, stream2);
ntt.forward2(d\_in\_2, lg\_N, 8, i+1, stream2);
cudaStreamWaitEvent(stream1, even2);
cudaMemcpyAsync(step2\_out\_ + d\_in.size()\*i, d\_in\_, d\_in.size(), cudaMemcpyDeviceToHost, stream1);
cudaEventRecord(even3, stream1);
cudaStreamWaitEvent(stream2, even3);
cudaMemcpyAsync(step2\_out\_ + d\_in\_2.size()\*(i+1), d\_in\_2\_, d\_in\_2.size(), cudaMemcpyDeviceToHost, stream2);
}
cudaEventRecord(stop, 0);
cudaEventSynchronize(stop);
float milliseconds = 0;
cudaEventElapsedTime(&milliseconds, start, stop);
std::cout << "out-ntt execution time: " << milliseconds<< " ms" << std::endl;
cudaEventDestroy(start);
cudaEventDestroy(stop);
cudaEventDestroy(even1);
cudaEventDestroy(even2);
cudaEventDestroy(even3);
cudaStreamDestroy(stream1);
cudaStreamDestroy(stream2);
cudaStreamDestroy(stream3);
delete step1\_map;
delete step2\_map;
}
int main() {
int lg\_N = 22;
int lg\_i = 12;
int lg\_j = 10;
uint64\_t N = 1<