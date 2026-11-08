#include <cstdint>
#include <cuda_runtime.h>
// 4-limb (256-bit) schoolbook product low half, 32-bit limbs, the shape of a field-mul inner loop.
__global__ void mul256_lo(const uint32_t* a, const uint32_t* b, uint32_t* out, int n) {
    int i = blockIdx.x * blockDim.x + threadIdx.x; if (i >= n) return;
    const uint32_t* x = a + 8 * i; const uint32_t* y = b + 8 * i; uint32_t r[8] = {0};
    for (int j = 0; j < 8; j++) { uint64_t carry = 0;
        for (int k = 0; j + k < 8; k++) { uint64_t t = (uint64_t)x[j] * y[k] + r[j + k] + carry; r[j + k] = (uint32_t)t; carry = t >> 32; } }
    for (int j = 0; j < 8; j++) out[8 * i + j] = r[j];
}
extern "C" int mori_smoke_mul256(const uint32_t* a, const uint32_t* b, uint32_t* out, int n, int* sm_major, int* sm_minor) {
    cudaDeviceProp p; cudaGetDeviceProperties(&p, 0); *sm_major = p.major; *sm_minor = p.minor;
    // a, b, out are plain host malloc memory: rely on HMM/ATS unified addressing (pageableMemoryAccess=1 on GB10).
    mul256_lo<<<(n + 255) / 256, 256>>>(a, b, out, n);
    return (int)cudaDeviceSynchronize();
}
