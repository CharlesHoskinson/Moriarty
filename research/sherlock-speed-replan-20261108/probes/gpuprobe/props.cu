#include <cstdio>
#include <cstdint>
#include <cstring>
#include <cstdlib>
#include <unistd.h>
static long meminfo(const char*key){FILE*f=fopen("/proc/meminfo","r");char line[256];long v=0;while(fgets(line,sizeof line,f)){if(!strncmp(line,key,strlen(key))){sscanf(line+strlen(key),"%ld",&v);break;}}fclose(f);return v;}
static long rss_kb(){FILE*f=fopen("/proc/self/status","r");char line[256];long v=0;while(fgets(line,sizeof line,f)){if(!strncmp(line,"VmRSS:",6)){sscanf(line+6,"%ld",&v);break;}}fclose(f);return v;}
__global__ void touch(uint8_t*p,size_t n){size_t i=(size_t)blockIdx.x*blockDim.x+threadIdx.x;size_t stride=(size_t)gridDim.x*blockDim.x;for(size_t j=i*4096;j<n;j+=stride*4096)p[j]=1;}
int main(){cudaDeviceProp p;cudaGetDeviceProperties(&p,0);
printf("name=%s sm=%d.%d SMs=%d busw=%d L2=%d MB regsPerSM=%d shmemPerSM=%zu KB totalGlobal=%zu MB\n",p.name,p.major,p.minor,p.multiProcessorCount,p.memoryBusWidth,p.l2CacheSize>>20,p.regsPerMultiprocessor,p.sharedMemPerMultiprocessor>>10,p.totalGlobalMem>>20);
printf("unifiedAddressing=%d managedMemory=%d concurrentManagedAccess=%d pageableMemoryAccess=%d pageableUsesHostPageTables=%d hostNativeAtomic=%d integrated=%d canMapHostMemory=%d directManagedMemAccessFromHost=%d\n",p.unifiedAddressing,p.managedMemory,p.concurrentManagedAccess,p.pageableMemoryAccess,p.pageableMemoryAccessUsesHostPageTables,p.hostNativeAtomicSupported,p.integrated,p.canMapHostMemory,p.directManagedMemAccessFromHost);
size_t n=(size_t)1<<30; // 1 GiB
printf("before: MemAvailable=%ld MB rss=%ld MB\n",meminfo("MemAvailable:")>>10,rss_kb()>>10);
uint8_t*m;cudaError_t e=cudaMallocManaged(&m,n);printf("managed alloc: %s\n",cudaGetErrorString(e));
touch<<<1024,256>>>(m,n);cudaDeviceSynchronize();
printf("after gpu-touch managed 1GiB: MemAvailable=%ld MB rss=%ld MB\n",meminfo("MemAvailable:")>>10,rss_kb()>>10);
for(size_t j=0;j<n;j+=4096)m[j]=2;
printf("after cpu-touch managed 1GiB: MemAvailable=%ld MB rss=%ld MB\n",meminfo("MemAvailable:")>>10,rss_kb()>>10);
uint8_t*d;e=cudaMalloc(&d,n);printf("cudaMalloc 1GiB: %s\n",cudaGetErrorString(e));touch<<<1024,256>>>(d,n);cudaDeviceSynchronize();
printf("after cudaMalloc+touch 1GiB: MemAvailable=%ld MB rss=%ld MB\n",meminfo("MemAvailable:")>>10,rss_kb()>>10);
// pageable host memory accessed directly from GPU (HMM / ATS)?
uint8_t*h=(uint8_t*)malloc(1<<24);memset(h,0,1<<24);touch<<<64,256>>>(h,1<<24);e=cudaDeviceSynchronize();printf("gpu touch of plain malloc 16MiB: %s\n",cudaGetErrorString(e));
size_t fr,tot;cudaMemGetInfo(&fr,&tot);printf("cudaMemGetInfo free=%zu MB total=%zu MB\n",fr>>20,tot>>20);
cudaFree(m);cudaFree(d);return 0;}
