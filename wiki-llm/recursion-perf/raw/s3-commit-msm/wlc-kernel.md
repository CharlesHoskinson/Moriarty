# WLC BLS12-381 Pippenger implementation
> Source: https://raw.githubusercontent.com/dunkirkturbo/wlc_msm/master/381_xyzz_bal/sppark/msm/pippenger.cuh
> Collected: 2026-10-26
> Published: Unknown

// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
#include 
#ifndef WARP\_SZ
# define WARP\_SZ 32
#endif
#ifndef NTHREADS
# define NTHREADS 64
#endif
#if NTHREADS < 32 || (NTHREADS & (NTHREADS-1)) != 0
# error "bad NTHREADS value"
#endif
constexpr static int log2(int n)
{ int ret=0; while (n>>=1) ret++; return ret; }
static const int NTHRBITS = log2(NTHREADS);
#ifndef NBITS
# define NBITS 253
#endif
#ifndef WBITS
# define WBITS 16
#endif
#define NWINS 16 // ((NBITS+WBITS-1)/WBITS) // ceil(NBITS/WBITS)
#ifndef LARGE\_L1\_CODE\_CACHE
# define LARGE\_L1\_CODE\_CACHE 0
#endif
\_\_global\_\_
void pre\_compute(affine\_t\* pre\_points, size\_t npoints);
\_\_global\_\_
void process\_scalar\_1(uint16\_t\* scalar, uint32\_t\* scalar\_tuple,
uint32\_t\* d\_scalar\_map, uint32\_t\* point\_idx, size\_t npoints);
\_\_global\_\_
void process\_scalar\_2(uint32\_t\* scalar\_tuple\_out,
uint16\_t\* bucket\_idx, size\_t npoints);
\_\_global\_\_
void bucket\_inf(bucket\_t \*buckets);
// v1.1
\_\_global\_\_
void bucket\_acc(uint32\_t\* scalar\_tuple\_out, uint16\_t\* bucket\_idx, uint32\_t\* point\_idx\_out,
affine\_t\* pre\_points, bucket\_t \*buckets\_pre,
uint16\_t\* bucket\_idx\_pre\_vector, uint16\_t\* bucket\_idx\_pre\_used,
uint32\_t\* bucket\_idx\_pre\_offset, size\_t npoints);
\_\_global\_\_
void bucket\_acc\_2(bucket\_t \*buckets\_pre, uint16\_t\* bucket\_idx\_pre\_vector, uint16\_t\* bucket\_idx\_pre\_used,
uint32\_t\* bucket\_idx\_pre\_offset, bucket\_t \*buckets, uint32\_t upper\_tnum, size\_t npoints);
\_\_global\_\_
void bucket\_agg\_1(bucket\_t \*buckets);
\_\_global\_\_
void bucket\_agg\_2(bucket\_t \*buckets);
\_\_global\_\_
void recursive\_sum(bucket\_t \*buckets, bucket\_t \*res);
#ifdef \_\_CUDA\_ARCH\_\_
#include 
static \_\_shared\_\_ bucket\_t bucket\_acc\_smem[NTHREADS \* 2];
// Transposed scalar\_t
class scalar\_T {
uint32\_t val[sizeof(scalar\_t)/sizeof(uint32\_t)][WARP\_SZ];
public:
\_\_device\_\_ uint32\_t& operator[](size\_t i) { return val[i][0]; }
\_\_device\_\_ const uint32\_t& operator[](size\_t i) const { return val[i][0]; }
\_\_device\_\_ scalar\_T& operator=(const scalar\_t& rhs)
{
for (size\_t i = 0; i < sizeof(scalar\_t)/sizeof(uint32\_t); i++)
val[i][0] = rhs[i];
return \*this;
}
};
class scalars\_T {
scalar\_T\* ptr;
public:
\_\_device\_\_ scalars\_T(void\* rhs) { ptr = (scalar\_T\*)rhs; }
\_\_device\_\_ scalar\_T& operator[](size\_t i)
{ return \*(scalar\_T\*)&(&ptr[i/WARP\_SZ][0])[i%WARP\_SZ]; }
\_\_device\_\_ const scalar\_T& operator[](size\_t i) const
{ return \*(const scalar\_T\*)&(&ptr[i/WARP\_SZ][0])[i%WARP\_SZ]; }
};
constexpr static \_\_device\_\_ int dlog2(int n)
{ int ret=0; while (n>>=1) ret++; return ret; }
#if WBITS==16
template
static \_\_device\_\_ int get\_wval(const scalar\_t& d, uint32\_t off, uint32\_t bits)
{
uint32\_t ret = d[off/32];
return (ret >> (off%32)) & ((1<
static \_\_device\_\_ int get\_wval(const scalar\_t& d, uint32\_t off, uint32\_t bits)
{
uint32\_t top = off + bits - 1;
uint64\_t ret = ((uint64\_t)d[top/32] << 32) | d[off/32];
return (int)(ret >> (off%32)) & ((1<= 32)
return false;
return ((scalar >> bitno) & 0x1);
}
template
static \_\_device\_\_ void mul(bucket\_t& res, const bucket\_t& base, uint32\_t scalar)
{
res.inf();
bool found\_one = false;
uint32\_t mb = max\_bits(scalar);
for (int32\_t i = mb - 1; i >= 0; --i)
{
if (found\_one)
{
res.add(res);
}
if (test\_bit(scalar, i))
{
found\_one = true;
res.add(base);
}
}
}
\_\_global\_\_
void pre\_compute(affine\_t\* pre\_points, size\_t npoints) {
const uint32\_t tnum = blockDim.x \* gridDim.x;
const uint32\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
bucket\_t Pi\_xyzz;
for (uint32\_t i = tid; i < npoints; i += tnum) {
affine\_t\* Pi = pre\_points + i;
Pi\_xyzz = \*Pi;
for (int j = 1; j < 7; j++) {
Pi = Pi + npoints;
Pi\_xyzz.dbl();
Pi\_xyzz.xyzz\_to\_affine(\*Pi);
}
}
}
\_\_global\_\_
void process\_scalar\_1(uint16\_t\* scalar, uint32\_t\* scalar\_tuple,
uint32\_t\* d\_scalar\_map, uint32\_t\* point\_idx, size\_t npoints) {
const uint32\_t tnum = blockDim.x \* gridDim.x;
const uint32\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
for (int i = tid; i < npoints; i += tnum) {
uint16\_t\* cur\_scalar\_ptr = scalar + (i << 4);
uint32\_t cur\_scalar = (uint32\_t)(\*cur\_scalar\_ptr); // uint32\_t instead of uint16\_t, specifically for 0x10000
scalar\_tuple[i] = d\_scalar\_map[cur\_scalar];
point\_idx[i] = i;
for (int j = i + npoints; j < NWINS \* npoints; j += npoints) {
cur\_scalar\_ptr += 1;
cur\_scalar = (uint32\_t)(\*(cur\_scalar\_ptr));
cur\_scalar += (scalar\_tuple[j - npoints] & 1);
scalar\_tuple[j] = d\_scalar\_map[cur\_scalar];
point\_idx[j] = i;
}
}
}
\_\_global\_\_
void process\_scalar\_2(uint32\_t\* scalar\_tuple\_out,
uint16\_t\* bucket\_idx, size\_t npoints) {
const uint32\_t tnum = blockDim.x \* gridDim.y;
const uint32\_t tid = blockIdx.y \* blockDim.x + threadIdx.x;
const uint32\_t bid = blockIdx.x;
uint16\_t\* bucket\_idx\_ptr = bucket\_idx + npoints \* bid;
uint32\_t\* scalar\_tuple\_out\_ptr = scalar\_tuple\_out + npoints \* bid;
for (uint32\_t i = tid; i < npoints; i += tnum) {
bucket\_idx\_ptr[i] = scalar\_tuple\_out\_ptr[i] >> 16;
}
}
// total\_bucket\_num = NWINS \* (1 << (WBITS - 2))
\_\_global\_\_
void bucket\_inf(bucket\_t \*buckets) {
const uint32\_t tnum = blockDim.x \* gridDim.y;
const uint32\_t tid = blockIdx.y \* blockDim.x + threadIdx.x;
const uint32\_t bid = blockIdx.x;
const uint32\_t bucket\_num = 1 << (WBITS - 2);
bucket\_t\* buckets\_ptr = buckets + bucket\_num \* bid;
for (uint32\_t i = tid; i < bucket\_num; i += tnum) {
buckets\_ptr[i].inf();
}
}
// v1.1
\_\_global\_\_
void bucket\_acc(uint32\_t\* scalar\_tuple\_out, uint16\_t\* bucket\_idx, uint32\_t\* point\_idx\_out,
affine\_t\* pre\_points, bucket\_t \*buckets\_pre,
uint16\_t\* bucket\_idx\_pre\_vector, uint16\_t\* bucket\_idx\_pre\_used,
uint32\_t\* bucket\_idx\_pre\_offset, size\_t npoints) {
const uint32\_t tnum = blockDim.x \* gridDim.y;
const uint32\_t tid\_inner = threadIdx.x;
const uint32\_t tid = blockIdx.y \* blockDim.x + tid\_inner;
const uint32\_t bid = blockIdx.x;
const uint32\_t buffer\_len = tnum + (1 << (WBITS - 2));
uint32\_t\* scalar\_tuple\_out\_ptr = scalar\_tuple\_out + npoints \* bid;
uint16\_t\* bucket\_idx\_ptr = bucket\_idx + npoints \* bid;
uint32\_t\* point\_idx\_out\_ptr = point\_idx\_out + npoints \* bid;
bucket\_t\* buckets\_pre\_ptr = buckets\_pre + buffer\_len \* bid;
uint16\_t\* bucket\_idx\_pre\_vector\_ptr = bucket\_idx\_pre\_vector + buffer\_len \* bid;
uint16\_t\* bucket\_idx\_pre\_used\_ptr = bucket\_idx\_pre\_used + tnum \* bid;
uint32\_t\* bucket\_idx\_pre\_offset\_ptr = bucket\_idx\_pre\_offset + tnum \* bid;
const uint32\_t step\_len = (npoints + tnum - 1) / tnum;
uint32\_t s = step\_len \* tid;
uint32\_t e = s + step\_len;
if (s >= npoints) {
bucket\_idx\_pre\_used\_ptr[tid] = 0;
return;
}
if (e >= npoints) e = npoints;
uint16\_t pre\_bucket\_idx = 0x8000; // not exist
bucket\_acc\_smem[tid\_inner \* 2 + 1].inf();
uint32\_t offset = tid + ((bucket\_idx\_ptr[s] + 1) >> 1);
bucket\_idx\_pre\_offset\_ptr[tid] = offset;
uint32\_t unique\_num = 0;
// process [s, e)
for (uint32\_t i = s; i < e; i++) {
uint32\_t power\_of\_2 = (scalar\_tuple\_out\_ptr[i] >> 8) & 0x0f;
uint32\_t dbl\_time = (scalar\_tuple\_out\_ptr[i] >> 12) & 0x0f;
uint16\_t cur\_bucket\_idx = bucket\_idx\_ptr[i];
if (cur\_bucket\_idx != pre\_bucket\_idx && (unique\_num++)) {
buckets\_pre\_ptr[offset + unique\_num - 2] = bucket\_acc\_smem[tid\_inner \* 2 + 1];
bucket\_idx\_pre\_vector\_ptr[offset + unique\_num - 2] = (pre\_bucket\_idx + 1) >> 1;
bucket\_acc\_smem[tid\_inner \* 2 + 1].inf();
}
pre\_bucket\_idx = cur\_bucket\_idx;
bucket\_acc\_smem[tid\_inner \* 2] = pre\_points[point\_idx\_out\_ptr[i] + power\_of\_2 \* npoints];
for (uint32\_t j = 0; j < dbl\_time; j++) {
bucket\_acc\_smem[tid\_inner \* 2].dbl();
}
if (scalar\_tuple\_out\_ptr[i] & 0x01) {
bucket\_acc\_smem[tid\_inner \* 2].neg(true);
}
bucket\_acc\_smem[tid\_inner \* 2 + 1].add(bucket\_acc\_smem[tid\_inner \* 2]);
}
buckets\_pre\_ptr[offset + unique\_num - 1] = bucket\_acc\_smem[tid\_inner \* 2 + 1];
bucket\_idx\_pre\_vector\_ptr[offset + unique\_num - 1] = (pre\_bucket\_idx + 1) >> 1;
bucket\_idx\_pre\_used\_ptr[tid] = unique\_num;
}
// v1.1 (2^{14} THREADS)
\_\_global\_\_
void bucket\_acc\_2(bucket\_t \*buckets\_pre, uint16\_t\* bucket\_idx\_pre\_vector, uint16\_t\* bucket\_idx\_pre\_used,
uint32\_t\* bucket\_idx\_pre\_offset, bucket\_t \*buckets, uint32\_t upper\_tnum, size\_t npoints) {
const uint32\_t tid\_inner = threadIdx.x;
const uint32\_t tid = blockIdx.y \* blockDim.x + tid\_inner;
const uint32\_t bid = blockIdx.x;
const uint32\_t buffer\_len = upper\_tnum + (1 << (WBITS - 2));
bucket\_t\* buckets\_pre\_ptr = buckets\_pre + buffer\_len \* bid;
uint16\_t\* bucket\_idx\_pre\_vector\_ptr = bucket\_idx\_pre\_vector + buffer\_len \* bid;
uint16\_t\* bucket\_idx\_pre\_used\_ptr = bucket\_idx\_pre\_used + upper\_tnum \* bid;
uint32\_t\* bucket\_idx\_pre\_offset\_ptr = bucket\_idx\_pre\_offset + upper\_tnum \* bid;
bucket\_t\* buckets\_ptr = buckets + (1 << (WBITS - 2)) \* bid;
int left = 0, right = upper\_tnum - 1;
bool not\_inf = false;
uint32\_t start\_pos = 0;
while (left <= right) {
int mid = left + ((right - left) >> 1);
uint16\_t vector\_used = bucket\_idx\_pre\_used\_ptr[mid];
if (!vector\_used) {
right = mid - 1;
} else {
uint32\_t vector\_ptr = bucket\_idx\_pre\_offset\_ptr[mid];
uint16\_t min\_idx = bucket\_idx\_pre\_vector\_ptr[vector\_ptr];
uint16\_t max\_idx = bucket\_idx\_pre\_vector\_ptr[vector\_ptr + vector\_used - 1];
if (min\_idx == (tid + 1)) {
start\_pos = mid;
not\_inf = true;
right = mid - 1;
} else if (min\_idx > (tid + 1)) {
right = mid - 1;
} else if (max\_idx < (tid + 1)) {
left = mid + 1;
} else {
for (uint32\_t i = vector\_ptr + 1; i < vector\_ptr + vector\_used; i++) {
if (bucket\_idx\_pre\_vector\_ptr[i] == (tid + 1)) {
start\_pos = mid;
not\_inf = true;
break;
}
}
break;
}
}
}
bucket\_acc\_smem[tid\_inner].inf();
while (not\_inf && start\_pos < upper\_tnum) {
not\_inf = false;
uint16\_t vector\_used = bucket\_idx\_pre\_used\_ptr[start\_pos];
uint32\_t vector\_ptr = bucket\_idx\_pre\_offset\_ptr[start\_pos];
for (uint32\_t i = vector\_ptr; i < vector\_ptr + vector\_used; i++) {
if (bucket\_idx\_pre\_vector\_ptr[i] == (tid + 1)) {
not\_inf = true;
bucket\_acc\_smem[tid\_inner].add(buckets\_pre\_ptr[i]);
break;
}
}
start\_pos++;
}
buckets\_ptr[tid] = bucket\_acc\_smem[tid\_inner]; // can omit kerner `bucket\_inf`
}
\_\_global\_\_
void bucket\_agg\_1(bucket\_t \*buckets) {
const uint32\_t tnum = blockDim.x \* gridDim.y;
const uint32\_t tid = blockIdx.y \* blockDim.x + threadIdx.x;
const uint32\_t bid = blockIdx.x;
bucket\_t\* buckets\_ptr = buckets + (1 << (WBITS - 2)) \* bid;
for (uint32\_t j = tid; j < (1 << (WBITS - 5)); j += tnum) {
uint32\_t s = j << 3;
bucket\_t\* Bi = buckets\_ptr + 0x3fff - s;
for (int i = 1; i < 8; i++) {
(Bi - i)->add(\*(Bi - i + 1));
}
}
}
\_\_global\_\_
void bucket\_agg\_2(bucket\_t \*buckets) {
const uint32\_t tnum = blockDim.x \* gridDim.y;
const uint32\_t tid = blockIdx.y \* blockDim.x + threadIdx.x;
const uint32\_t bid = blockIdx.x;
bucket\_t\* buckets\_ptr = buckets + (1 << (WBITS - 2)) \* bid;
for (uint32\_t i = 3; i < 14; i++) {
for (uint32\_t k = tid; k < (1 << (WBITS - 3)); k += tnum) {
uint32\_t baseline = ((1 + (k >> i)) << (i + 1)) - (1 << i);
uint32\_t offset = k & ((1 << i) - 1);
bucket\_t\* Bi = buckets\_ptr + 0x3fff - (baseline - 1);
bucket\_t\* Bj = Bi - (offset + 1); // B + 0x3fff - (baseline + offset)
Bj->add(\*Bi);
}
cooperative\_groups::this\_grid().sync();
}
}
\_\_global\_\_
void recursive\_sum(bucket\_t \*buckets, bucket\_t \*res) {
const uint32\_t tnum = blockDim.x \* gridDim.y;
const uint32\_t tid = blockIdx.y \* blockDim.x + threadIdx.x;
const uint32\_t bid = blockIdx.x;
bucket\_t\* buckets\_ptr = buckets + (1 << (WBITS - 2)) \* bid;
if (tid == 0) {
res[bid] = \*buckets\_ptr;
}
// cooperative\_groups::this\_grid().sync();
for (uint32\_t j = 1 << (WBITS - 3); j > NTHREADS; j >>= 1) {
for (uint32\_t i = tid; i < j; i += tnum) {
buckets\_ptr[i].add(buckets\_ptr[i + j]);
}
cooperative\_groups::this\_grid().sync();
}
for (uint32\_t j = NTHREADS; j > WARP\_SZ; j >>= 1) {
if (tid < j) {
buckets\_ptr[tid].add(buckets\_ptr[tid + j]);
}
cooperative\_groups::this\_thread\_block().sync();
}
if (tid < WARP\_SZ) {
buckets\_ptr[tid].add(buckets\_ptr[tid + 32]);
buckets\_ptr[tid].add(buckets\_ptr[tid + 16]);
buckets\_ptr[tid].add(buckets\_ptr[tid + 8]);
buckets\_ptr[tid].add(buckets\_ptr[tid + 4]);
buckets\_ptr[tid].add(buckets\_ptr[tid + 2]);
buckets\_ptr[tid].add(buckets\_ptr[tid + 1]);
}
if (tid == 0) {
buckets\_ptr->dbl();
res[bid].neg(true);
res[bid].add(\*buckets\_ptr);
}
/\*cooperative\_groups::this\_grid().sync();
if (tid == 0 && bid == 0) {
bucket\_t check\_res;
check\_res.inf();
for (int i = 15; i > -1; i--) {
for (int j = 0; j < 16; j++) {
check\_res.add(check\_res);
}
check\_res.add(res[i]);
}
printf("\ncheck\_2:\n");
check\_res.xyzz\_print();
}\*/
}
#else
#include 
#include 
using namespace std;
#include 
#include 
#include 
#include 
template
inline void launch\_coop(void(\*f)(Types...),
dim3 gridDim, dim3 blockDim, cudaStream\_t stream,
Types... args)
{
void\* va\_args[sizeof...(args)] = { &args... };
CUDA\_OK(cudaLaunchCooperativeKernel((const void\*)f, gridDim, blockDim,
va\_args, 0, stream));
}
class stream\_t {
cudaStream\_t stream;
public:
stream\_t(int device) {
CUDA\_OK(cudaSetDevice(device));
cudaStreamCreateWithFlags(&stream, cudaStreamNonBlocking);
}
~stream\_t() { cudaStreamDestroy(stream); }
inline operator decltype(stream)() { return stream; }
};
template class result\_t\_faster {
bucket\_t ret[NWINS];
public:
result\_t\_faster() {}
inline operator decltype(ret)&() { return ret; }
};
template
class device\_ptr\_list\_t {
vector d\_ptrs;
public:
device\_ptr\_list\_t() {}
~device\_ptr\_list\_t() {
for(T \*ptr: d\_ptrs) {
cudaFree(ptr);
}
}
size\_t allocate(size\_t bytes) {
T \*d\_ptr;
CUDA\_OK(cudaMalloc(&d\_ptr, bytes));
d\_ptrs.push\_back(d\_ptr);
return d\_ptrs.size() - 1;
}
size\_t size() {
return d\_ptrs.size();
}
T\* operator[](size\_t i) {
if (i > d\_ptrs.size() - 1) {
CUDA\_OK(cudaErrorInvalidDevicePointer);
}
return d\_ptrs[i];
}
};
// Pippenger MSM class
template
class pippenger\_t {
public:
typedef vector,
host\_pinned\_allocator\_t>> result\_container\_t\_faster;
private:
size\_t sm\_count;
bool init\_done = false;
device\_ptr\_list\_t d\_base\_ptrs;
device\_ptr\_list\_t d\_scalar\_ptrs;
device\_ptr\_list\_t d\_bucket\_ptrs;
device\_ptr\_list\_t d\_bucket\_pre\_ptrs; // v1.1
device\_ptr\_list\_t d\_bucket\_idx\_pre\_ptrs; // v1.1
device\_ptr\_list\_t d\_bucket\_idx\_pre2\_ptrs; // v1.2
device\_ptr\_list\_t d\_res\_ptrs;
// GPU device number
int device;
// TODO: Move to device class eventually
thread\_pool\_t \*da\_pool = nullptr;
public:
// Default stream for operations
stream\_t default\_stream;
device\_ptr\_list\_t d\_scalar\_map;
device\_ptr\_list\_t d\_scalar\_tuple\_ptrs;
device\_ptr\_list\_t d\_point\_idx\_ptrs;
device\_ptr\_list\_t d\_bucket\_idx\_ptrs;
device\_ptr\_list\_t d\_cub\_ptrs;
// Parameters for an MSM operation
class MSMConfig {
friend pippenger\_t;
public:
size\_t npoints;
size\_t N;
size\_t n;
};
pippenger\_t() : default\_stream(0) {
device = 0;
}
pippenger\_t(int \_device, thread\_pool\_t \*pool = nullptr)
: default\_stream(\_device) {
da\_pool = pool;
device = \_device;
}
// Initialize instance. Throws cuda\_error on error.
void init() {
if (!init\_done) {
CUDA\_OK(cudaSetDevice(device));
cudaDeviceProp prop;
if (cudaGetDeviceProperties(&prop, 0) != cudaSuccess || prop.major < 7)
CUDA\_OK(cudaErrorInvalidDevice);
sm\_count = prop.multiProcessorCount;
if (da\_pool == nullptr) {
da\_pool = new thread\_pool\_t();
}
init\_done = true;
}
}
int get\_device() {
return device;
}
// Initialize parameters for a specific size MSM. Throws cuda\_error on error.
MSMConfig init\_msm\_faster(size\_t npoints) {
init();
MSMConfig config;
config.npoints = npoints;
config.n = (npoints+WARP\_SZ-1) & ((size\_t)0-WARP\_SZ);
config.N = (sm\_count\*256) / (NTHREADS\*NWINS);
size\_t delta = ((npoints+(config.N)-1)/(config.N)+WARP\_SZ-1) & (0U-WARP\_SZ);
config.N = (npoints+delta-1) / delta;
// if(config.N % 2 == 1) config.N -= 1;
return config;
}
size\_t get\_size\_bases(MSMConfig& config) {
return config.n \* sizeof(affine\_t);
}
size\_t get\_size\_scalars(MSMConfig& config) {
return config.n \* sizeof(scalar\_t);
}
size\_t get\_size\_buckets() {
return sizeof(bucket\_t) \* NWINS \* (1 << (WBITS - 2));
}
size\_t get\_size\_buckets\_pre(MSMConfig& config) { // v1.1
return sizeof(bucket\_t) \* NWINS \* (config.N \* NTHREADS + (1 << (WBITS - 2)));
}
size\_t get\_size\_bucket\_idx\_pre\_vector(MSMConfig& config) { // v1.1
return sizeof(uint16\_t) \* NWINS \* (config.N \* NTHREADS + (1 << (WBITS - 2)));
}
size\_t get\_size\_bucket\_idx\_pre\_used(MSMConfig& config) { // v1.1
return sizeof(uint16\_t) \* config.N \* NTHREADS \* NWINS;
}
size\_t get\_size\_bucket\_idx\_pre\_offset(MSMConfig& config) { // v1.2
return sizeof(uint32\_t) \* config.N \* NTHREADS \* NWINS;
}
size\_t get\_size\_res() {
return sizeof(bucket\_t) \* NWINS;
}
size\_t get\_size\_scalar\_map() {
return ((1 << 16) + 1) \* sizeof(uint32\_t);
}
size\_t get\_size\_scalar\_tuple(MSMConfig& config) {
return config.n \* sizeof(uint32\_t) \* NWINS;
}
size\_t get\_size\_point\_idx(MSMConfig& config) {
return config.n \* sizeof(uint32\_t) \* NWINS;
}
size\_t get\_size\_bucket\_idx(MSMConfig& config) {
return config.n \* sizeof(uint16\_t) \* NWINS;
}
size\_t get\_size\_cub\_sort\_faster(MSMConfig& config){
uint32\_t \*d\_scalar\_tuple = nullptr;
uint32\_t \*d\_scalar\_tuple\_out = nullptr;
uint32\_t \*d\_point\_idx = nullptr;
uint32\_t \*d\_point\_idx\_out = nullptr;
void \*d\_temp = NULL;
size\_t temp\_size = 0;
cub::DeviceRadixSort::SortPairs(d\_temp, temp\_size,
d\_scalar\_tuple, d\_scalar\_tuple\_out,
d\_point\_idx, d\_point\_idx\_out, config.n, 0, 31);
return temp\_size;
}
result\_container\_t\_faster get\_result\_container\_faster() {
result\_container\_t\_faster res(1);
return res;
}
// Allocate storage for bases on device. Throws cuda\_error on error.
// Returns index of the allocated base storage.
size\_t allocate\_d\_bases(MSMConfig& config) {
return d\_base\_ptrs.allocate(7 \* get\_size\_bases(config));
}
size\_t allocate\_d\_scalars(MSMConfig& config) {
return d\_scalar\_ptrs.allocate(get\_size\_scalars(config));
}
size\_t allocate\_d\_buckets() {
return d\_bucket\_ptrs.allocate(get\_size\_buckets());
}
size\_t allocate\_d\_buckets\_pre(MSMConfig& config) { // v1.1
return d\_bucket\_pre\_ptrs.allocate(get\_size\_buckets\_pre(config));
}
size\_t allocate\_d\_bucket\_idx\_pre\_vector(MSMConfig& config) { // v1.1
return d\_bucket\_idx\_pre\_ptrs.allocate(get\_size\_bucket\_idx\_pre\_vector(config));
}
size\_t allocate\_d\_bucket\_idx\_pre\_used(MSMConfig& config) { // v1.1
return d\_bucket\_idx\_pre\_ptrs.allocate(get\_size\_bucket\_idx\_pre\_used(config));
}
size\_t allocate\_d\_bucket\_idx\_pre\_offset(MSMConfig& config) { // v1.2
return d\_bucket\_idx\_pre2\_ptrs.allocate(get\_size\_bucket\_idx\_pre\_offset(config));
}
size\_t allocate\_d\_res() {
return d\_res\_ptrs.allocate(get\_size\_res());
}
size\_t allocate\_d\_scalar\_map() {
return d\_scalar\_map.allocate(get\_size\_scalar\_map());
}
size\_t allocate\_d\_scalar\_tuple(MSMConfig& config) {
return d\_scalar\_tuple\_ptrs.allocate(get\_size\_scalar\_tuple(config));
}
size\_t allocate\_d\_scalar\_tuple\_out(MSMConfig& config) {
return d\_scalar\_tuple\_ptrs.allocate(get\_size\_scalar\_tuple(config));
}
size\_t allocate\_d\_point\_idx(MSMConfig& config) {
return d\_point\_idx\_ptrs.allocate(get\_size\_point\_idx(config));
// return d\_point\_idx\_ptrs.allocate(config.n \* sizeof(uint32\_t));
}
size\_t allocate\_d\_point\_idx\_out(MSMConfig& config) {
return d\_point\_idx\_ptrs.allocate(get\_size\_point\_idx(config));
}
size\_t allocate\_d\_bucket\_idx(MSMConfig& config) {
return d\_bucket\_idx\_ptrs.allocate(get\_size\_bucket\_idx(config));
}
size\_t allocate\_d\_cub\_sort\_faster(MSMConfig& config) {
return d\_cub\_ptrs.allocate(get\_size\_cub\_sort\_faster(config));
}
// Transfer bases to device. Throws cuda\_error on error.
void transfer\_bases\_to\_device(MSMConfig& config, size\_t d\_bases\_idx, const affine\_t points[],
size\_t ffi\_affine\_sz = sizeof(affine\_t),
cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
affine\_t \*d\_points = d\_base\_ptrs[d\_bases\_idx];
CUDA\_OK(cudaSetDevice(device));
if (ffi\_affine\_sz != sizeof(\*d\_points))
CUDA\_OK(cudaMemcpy2DAsync(d\_points, sizeof(\*d\_points),
points, ffi\_affine\_sz,
ffi\_affine\_sz, config.npoints,
cudaMemcpyHostToDevice, stream));
else
CUDA\_OK(cudaMemcpyAsync(d\_points, points, config.npoints\*sizeof(\*d\_points),
cudaMemcpyHostToDevice, stream));
}
// Transfer scalars to device. Throws cuda\_error on error.
void transfer\_scalars\_to\_device(MSMConfig& config,
size\_t d\_scalars\_idx, const scalar\_t scalars[],
cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
scalar\_t \*d\_scalars = d\_scalar\_ptrs[d\_scalars\_idx];
CUDA\_OK(cudaSetDevice(device));
CUDA\_OK(cudaMemcpyAsync(d\_scalars, scalars, config.npoints\*sizeof(\*d\_scalars),
cudaMemcpyHostToDevice, stream));
}
void transfer\_res\_to\_host\_faster(result\_container\_t\_faster &res, size\_t d\_res\_idx,
cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
bucket\_t \*d\_res = d\_res\_ptrs[d\_res\_idx];
CUDA\_OK(cudaSetDevice(device));
CUDA\_OK(cudaMemcpyAsync(res[0], d\_res, sizeof(res[0]),
cudaMemcpyDeviceToHost, stream));
}
void transfer\_scalar\_map\_to\_device(size\_t d\_scalar\_map\_idx, const uint32\_t scalar\_map[],
cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
uint32\_t \*d\_smap = d\_scalar\_map[d\_scalar\_map\_idx];
CUDA\_OK(cudaSetDevice(device));
CUDA\_OK(cudaMemcpyAsync(d\_smap, scalar\_map, ((1 << 16) + 1)\*sizeof(uint32\_t),
cudaMemcpyHostToDevice, stream));
}
void synchronize\_stream() {
CUDA\_OK(cudaSetDevice(device));
CUDA\_OK(cudaStreamSynchronize(default\_stream));
}
void launch\_kernel\_init(MSMConfig& config,
size\_t d\_points\_sn, cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
affine\_t \*d\_points = d\_base\_ptrs[d\_points\_sn];
CUDA\_OK(cudaSetDevice(device));
launch\_coop(pre\_compute, NWINS \* config.N, NTHREADS, stream,
d\_points, config.npoints);
}
void launch\_process\_scalar\_1(MSMConfig& config,
size\_t d\_scalars\_sn, size\_t d\_scalar\_tuples\_sn,
size\_t d\_scalar\_map\_sn, size\_t d\_point\_idx\_sn,
cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
uint16\_t\* d\_scalars = (uint16\_t\*)d\_scalar\_ptrs[d\_scalars\_sn];
uint32\_t\* d\_scalar\_tuple = d\_scalar\_tuple\_ptrs[d\_scalar\_tuples\_sn];
uint32\_t\* d\_smap = d\_scalar\_map[d\_scalar\_map\_sn];
uint32\_t\* d\_point\_idx = d\_point\_idx\_ptrs[d\_point\_idx\_sn];
CUDA\_OK(cudaSetDevice(device));
launch\_coop(process\_scalar\_1, NWINS \* config.N, NTHREADS, stream,
d\_scalars, d\_scalar\_tuple, d\_smap, d\_point\_idx, config.npoints);
}
void launch\_process\_scalar\_2(MSMConfig& config,
size\_t d\_scalar\_tuples\_out\_sn, size\_t d\_bucket\_idx\_sn,
cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
uint32\_t\* d\_scalar\_tuple\_out = d\_scalar\_tuple\_ptrs[d\_scalar\_tuples\_out\_sn];
uint16\_t\* d\_bucket\_idx = d\_bucket\_idx\_ptrs[d\_bucket\_idx\_sn];
CUDA\_OK(cudaSetDevice(device));
launch\_coop(process\_scalar\_2, dim3(NWINS, config.N), NTHREADS, stream,
d\_scalar\_tuple\_out, d\_bucket\_idx, config.npoints);
}
void launch\_bucket\_inf(MSMConfig& config, size\_t d\_buckets\_sn, cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
bucket\_t\* d\_buckets = d\_bucket\_ptrs[d\_buckets\_sn];
CUDA\_OK(cudaSetDevice(device));
launch\_coop(bucket\_inf, dim3(NWINS, config.N), NTHREADS, stream, d\_buckets);
}
void launch\_bucket\_acc(MSMConfig& config,
size\_t d\_scalar\_tuples\_out\_sn, size\_t d\_bucket\_idx\_sn,
size\_t d\_point\_idx\_out\_sn, size\_t d\_points\_sn, size\_t d\_buckets\_sn,
size\_t d\_buckets\_pre\_sn, size\_t d\_bucket\_idx\_pre\_vector\_sn,
size\_t d\_bucket\_idx\_pre\_used\_sn, size\_t d\_bucket\_idx\_pre\_offset\_sn,
cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
uint32\_t\* d\_scalar\_tuple\_out = d\_scalar\_tuple\_ptrs[d\_scalar\_tuples\_out\_sn];
uint16\_t\* d\_bucket\_idx = d\_bucket\_idx\_ptrs[d\_bucket\_idx\_sn];
uint32\_t\* d\_point\_idx\_out = d\_point\_idx\_ptrs[d\_point\_idx\_out\_sn];
affine\_t\* d\_points = d\_base\_ptrs[d\_points\_sn];
bucket\_t\* d\_buckets = d\_bucket\_ptrs[d\_buckets\_sn];
bucket\_t\* d\_buckets\_pre = d\_bucket\_pre\_ptrs[d\_buckets\_pre\_sn];
uint16\_t\* d\_bucket\_idx\_pre\_vector = d\_bucket\_idx\_pre\_ptrs[d\_bucket\_idx\_pre\_vector\_sn];
uint16\_t\* d\_bucket\_idx\_pre\_used = d\_bucket\_idx\_pre\_ptrs[d\_bucket\_idx\_pre\_used\_sn];
uint32\_t\* d\_bucket\_idx\_pre\_offset = d\_bucket\_idx\_pre2\_ptrs[d\_bucket\_idx\_pre\_offset\_sn];
CUDA\_OK(cudaSetDevice(device));
launch\_coop(bucket\_acc, dim3(NWINS, config.N), NTHREADS, stream,
d\_scalar\_tuple\_out, d\_bucket\_idx, d\_point\_idx\_out,
d\_points, d\_buckets\_pre,
d\_bucket\_idx\_pre\_vector, d\_bucket\_idx\_pre\_used,
d\_bucket\_idx\_pre\_offset, config.npoints);
bucket\_acc\_2<<>>(
d\_buckets\_pre, d\_bucket\_idx\_pre\_vector, d\_bucket\_idx\_pre\_used,
d\_bucket\_idx\_pre\_offset, d\_buckets, (uint32\_t)(config.N \* NTHREADS), config.npoints
);
// launch\_coop(bucket\_acc\_2, dim3(NWINS, (1 << (WBITS - 2)) / NTHREADS), NTHREADS, stream,
// d\_buckets\_pre, d\_bucket\_idx\_pre\_vector, d\_bucket\_idx\_pre\_used,
// d\_bucket\_idx\_pre\_offset, d\_buckets, (uint32\_t)(config.N \* NTHREADS), config.npoints);
}
void launch\_bucket\_agg\_1(MSMConfig& config, size\_t d\_buckets\_sn, cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
bucket\_t\* d\_buckets = d\_bucket\_ptrs[d\_buckets\_sn];
CUDA\_OK(cudaSetDevice(device));
// bucket\_agg\_1<<>>(d\_buckets);
launch\_coop(bucket\_agg\_1, dim3(NWINS, config.N), NTHREADS, stream, d\_buckets);
}
void launch\_bucket\_agg\_2(MSMConfig& config, size\_t d\_buckets\_sn, cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
bucket\_t\* d\_buckets = d\_bucket\_ptrs[d\_buckets\_sn];
CUDA\_OK(cudaSetDevice(device));
launch\_coop(bucket\_agg\_2, dim3(NWINS, config.N), NTHREADS, stream, d\_buckets);
}
void launch\_recursive\_sum(MSMConfig& config, size\_t d\_buckets\_sn, size\_t d\_res\_sn, cudaStream\_t s = nullptr) {
cudaStream\_t stream = (s == nullptr) ? default\_stream : s;
bucket\_t\* d\_buckets = d\_bucket\_ptrs[d\_buckets\_sn];
bucket\_t\* d\_res = d\_res\_ptrs[d\_res\_sn];
CUDA\_OK(cudaSetDevice(device));
launch\_coop(recursive\_sum, dim3(NWINS, config.N), NTHREADS, stream, d\_buckets, d\_res);
}
// Perform final accumulation on CPU.
void accumulate\_faster(point\_t &out, result\_container\_t\_faster &res) {
out.inf();
for(int32\_t k = NWINS - 1; k >= 0; k--)
{
for (int32\_t i = 0; i < WBITS; i++)
{
out.dbl();
}
point\_t p = (res[0])[k];
out.add(p);
}
}
};
#endif