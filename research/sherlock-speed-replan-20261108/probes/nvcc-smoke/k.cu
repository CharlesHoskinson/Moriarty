#include <cstdio>
#include <cstdint>
__global__ void mul(const uint64_t*a,const uint64_t*b,uint64_t*hi,int n){int i=blockIdx.x*blockDim.x+threadIdx.x;if(i<n)hi[i]=__umul64hi(a[i],b[i]);}
int main(){const int n=1<<20;uint64_t *a,*b,*h;cudaMallocManaged(&a,n*8);cudaMallocManaged(&b,n*8);cudaMallocManaged(&h,n*8);
for(int i=0;i<n;i++){a[i]=0xFFFFFFFFFFFFFFFFull-i;b[i]=0x123456789ABCDEFull+i;}
mul<<<(n+255)/256,256>>>(a,b,h,n);cudaError_t e=cudaDeviceSynchronize();
unsigned __int128 r=(unsigned __int128)a[7]*b[7];cudaDeviceProp p;cudaGetDeviceProperties(&p,0);
printf("%s sm_%d%d err=%s match=%d\n",p.name,p.major,p.minor,cudaGetErrorString(e),(uint64_t)(r>>64)==h[7]);}
