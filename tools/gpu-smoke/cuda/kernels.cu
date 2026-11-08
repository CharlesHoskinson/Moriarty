// SPDX-License-Identifier: Apache-2.0
// GPU phase G0 smoke kernels (task 2.10a, predicate F23). Original code, no borrowed sources.
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <cuda_runtime.h>

// Low 256 bits of the product of two 256-bit numbers in 8 x 32-bit limbs (little endian),
// schoolbook with 64-bit intermediates: the shape of a field-mul inner loop.
__global__ void mul256_lo(const uint32_t* a, const uint32_t* b, uint32_t* out, int n) {
    int i = blockIdx.x * blockDim.x + threadIdx.x;
    if (i >= n) return;
    const uint32_t* x = a + 8 * i;
    const uint32_t* y = b + 8 * i;
    uint32_t r[8] = {0};
    for (int j = 0; j < 8; j++) {
        uint64_t carry = 0;
        for (int k = 0; j + k < 8; k++) {
            uint64_t t = (uint64_t)x[j] * y[k] + r[j + k] + carry;
            r[j + k] = (uint32_t)t;
            carry = t >> 32;
        }
    }
    for (int j = 0; j < 8; j++) out[8 * i + j] = r[j];
}

// Writes one byte per 4 KiB page so every page of the allocation is backed.
__global__ void touch_pages(uint8_t* p, size_t n) {
    size_t i = (size_t)blockIdx.x * blockDim.x + threadIdx.x;
    size_t stride = (size_t)gridDim.x * blockDim.x;
    for (size_t j = i * 4096; j < n; j += stride * 4096) p[j] = 1;
}

extern "C" {

struct GsDeviceInfo {
    char name[256];
    int device_count;
    int major;
    int minor;
    int integrated;
    int pageable_memory_access;
    int uses_host_page_tables;
    int concurrent_managed_access;
    int driver_version;
    int runtime_version;
};

// Returns a cudaError_t value; 0 is success. device_count is 0 when no device is visible.
int gs_device_info(GsDeviceInfo* info) {
    memset(info, 0, sizeof *info);
    cudaError_t e = cudaGetDeviceCount(&info->device_count);
    if (e != cudaSuccess) { info->device_count = 0; return (int)e; }
    if (info->device_count == 0) return 0;
    cudaDeviceProp p;
    e = cudaGetDeviceProperties(&p, 0);
    if (e != cudaSuccess) return (int)e;
    strncpy(info->name, p.name, sizeof info->name - 1);
    info->major = p.major;
    info->minor = p.minor;
    info->integrated = p.integrated;
    info->pageable_memory_access = p.pageableMemoryAccess;
    info->uses_host_page_tables = p.pageableMemoryAccessUsesHostPageTables;
    info->concurrent_managed_access = p.concurrentManagedAccess;
    cudaDriverGetVersion(&info->driver_version);
    cudaRuntimeGetVersion(&info->runtime_version);
    return 0;
}

// a, b, out are plain host memory (Rust Vec): the kernel reads and writes them in place
// through unified addressing (pageableMemoryAccess=1 on this host). No copies.
int gs_mul256(const uint32_t* a, const uint32_t* b, uint32_t* out, int n) {
    mul256_lo<<<(n + 255) / 256, 256>>>(a, b, out, n);
    cudaError_t e = cudaGetLastError();
    if (e != cudaSuccess) return (int)e;
    return (int)cudaDeviceSynchronize();
}

// managed = 0: cudaMalloc; managed = 1: cudaMallocManaged. Touches every page on the GPU.
int gs_alloc_touch(int managed, size_t bytes, void** ptr) {
    *ptr = nullptr;
    cudaError_t e = managed ? cudaMallocManaged(ptr, bytes) : cudaMalloc(ptr, bytes);
    if (e != cudaSuccess) return (int)e;
    touch_pages<<<1024, 256>>>((uint8_t*)*ptr, bytes);
    e = cudaGetLastError();
    if (e != cudaSuccess) return (int)e;
    return (int)cudaDeviceSynchronize();
}

int gs_free(void* ptr) { return (int)cudaFree(ptr); }

const char* gs_error_string(int e) { return cudaGetErrorString((cudaError_t)e); }

}  // extern "C"
