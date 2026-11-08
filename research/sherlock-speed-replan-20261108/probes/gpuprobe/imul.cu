#include <cstdio>
#include <cstdint>
// Throughput probes: how many 64x64->128 and 32x32->64 multiplies per second does this GPU do?
__global__ void k64(uint64_t*out,uint64_t seed,int iters){uint64_t a=seed^threadIdx.x,b=seed+blockIdx.x,c=1,d=2;
#pragma unroll 4
for(int i=0;i<iters;i++){uint64_t h=__umul64hi(a,b);uint64_t l=a*b;a=h^c;b=l+d;c+=h;d^=l;}out[blockIdx.x*blockDim.x+threadIdx.x]=a+b+c+d;}
__global__ void k32(uint32_t*out,uint32_t seed,int iters){uint32_t a=seed^threadIdx.x,b=seed+blockIdx.x,c=1,d=2;
#pragma unroll 4
for(int i=0;i<iters;i++){uint32_t h=__umulhi(a,b);uint32_t l=a*b;a=h^c;b=l+d;c+=h;d^=l;}out[blockIdx.x*blockDim.x+threadIdx.x]=a+b+c+d;}
// 32-bit mad chain (mul.lo + add), the IMAD pipe
__global__ void kmad(uint32_t*out,uint32_t seed,int iters){uint32_t a=seed^threadIdx.x,b=seed+blockIdx.x,c=1,d=2;
#pragma unroll 4
for(int i=0;i<iters;i++){a=a*b+c;b=b*c+d;c=c*d+a;d=d*a+b;}out[blockIdx.x*blockDim.x+threadIdx.x]=a+b+c+d;}
int main(){int blocks=6144/128*32*4, threads=128; int iters=4096; // 4 mul-pairs per iter
uint64_t*o64;uint32_t*o32;cudaMalloc(&o64,(size_t)blocks*threads*8);cudaMalloc(&o32,(size_t)blocks*threads*4);
cudaEvent_t s,e;cudaEventCreate(&s);cudaEventCreate(&e);float ms;
k64<<<blocks,threads>>>(o64,1,16);k32<<<blocks,threads>>>(o32,1,16);kmad<<<blocks,threads>>>(o32,1,16);cudaDeviceSynchronize();
double nthreads=(double)blocks*threads;
cudaEventRecord(s);k64<<<blocks,threads>>>(o64,7,iters);cudaEventRecord(e);cudaEventSynchronize(e);cudaEventElapsedTime(&ms,s,e);
printf("u64 mulhi+mul pairs: %.1f G pairs/s (%.2f ms)\n",nthreads*iters/ms/1e6,ms);
cudaEventRecord(s);k32<<<blocks,threads>>>(o32,7,iters);cudaEventRecord(e);cudaEventSynchronize(e);cudaEventElapsedTime(&ms,s,e);
printf("u32 mulhi+mul pairs: %.1f G pairs/s (%.2f ms)\n",nthreads*iters/ms/1e6,ms);
cudaEventRecord(s);kmad<<<blocks,threads>>>(o32,7,iters);cudaEventRecord(e);cudaEventSynchronize(e);cudaEventElapsedTime(&ms,s,e);
printf("u32 mad (4 per iter): %.1f G mad/s (%.2f ms)\n",nthreads*iters*4/ms/1e6,ms);
// bandwidth: copy 1 GiB device->device
size_t n=1u<<30;uint8_t*a,*b;cudaMalloc(&a,n);cudaMalloc(&b,n);cudaMemset(a,1,n);cudaDeviceSynchronize();
cudaEventRecord(s);cudaMemcpy(b,a,n,cudaMemcpyDeviceToDevice);cudaEventRecord(e);cudaEventSynchronize(e);cudaEventElapsedTime(&ms,s,e);
printf("D2D memcpy 1GiB: %.1f GB/s (read+write counted: %.1f GB/s)\n",n/ms/1e6,2.0*n/ms/1e6);
return 0;}
