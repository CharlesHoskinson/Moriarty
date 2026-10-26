# sppark Pippenger source
> Source: https://raw.githubusercontent.com/supranational/sppark/main/msm/pippenger.cuh
> Collected: 2026-10-26
> Published: Unknown

// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
#ifndef \_\_SPPARK\_MSM\_PIPPENGER\_CUH\_\_
#define \_\_SPPARK\_MSM\_PIPPENGER\_CUH\_\_
#include 
#include 
#include 
#include 
#include 
#include "sort.cuh"
#include "batch\_addition.cuh"
#ifndef WARP\_SZ
# define WARP\_SZ 32
#endif
#ifdef \_\_GNUC\_\_
# define asm \_\_asm\_\_ \_\_volatile\_\_
#else
# define asm asm volatile
#endif
/\*
\* Break down |scalars| to signed |wbits|-wide digits.
\*/
#ifdef \_\_CUDA\_ARCH\_\_
// Transposed scalar\_t
template
class scalar\_T {
uint32\_t val[sizeof(scalar\_t)/sizeof(uint32\_t)][WARP\_SZ];
public:
\_\_device\_\_ const uint32\_t& operator[](size\_t i) const { return val[i][0]; }
\_\_device\_\_ scalar\_T& operator()(uint32\_t laneid)
{ return \*reinterpret\_cast(&val[0][laneid]); }
\_\_device\_\_ scalar\_T& operator=(const scalar\_t& rhs)
{
for (size\_t i = 0; i < sizeof(scalar\_t)/sizeof(uint32\_t); i++)
val[i][0] = rhs[i];
return \*this;
}
};
template
\_\_device\_\_ \_\_forceinline\_\_
static uint32\_t get\_wval(const scalar\_T& scalar, uint32\_t off,
uint32\_t top\_i = (scalar\_t::nbits + 31) / 32 - 1)
{
uint32\_t i = off / 32;
uint64\_t ret = scalar[i];
if (i < top\_i)
ret |= (uint64\_t)scalar[i+1] << 32;
return ret >> (off%32);
}
\_\_device\_\_ \_\_forceinline\_\_
static uint32\_t booth\_encode(uint32\_t wval, uint32\_t wmask, uint32\_t wbits)
{
uint32\_t sign = (wval >> wbits) & 1;
wval = ((wval + 1) & wmask) >> 1;
return sign ? 0-wval : wval;
}
#endif
template
\_\_launch\_bounds\_\_(1024) \_\_global\_\_
void breakdown(vec2d\_t digits, const scalar\_t scalars[], size\_t len,
uint32\_t nwins, uint32\_t wbits, bool mont = true)
{
assert(len <= (1U<<31) && wbits < 32);
#ifdef \_\_CUDA\_ARCH\_\_
extern \_\_shared\_\_ scalar\_T xchange[];
const uint32\_t tid = threadIdx.x;
const uint32\_t tix = threadIdx.x + blockIdx.x\*blockDim.x;
const uint32\_t top\_i = (scalar\_t::nbits + 31) / 32 - 1;
const uint32\_t wmask = 0xffffffffU >> (31-wbits); // (1U << (wbits+1)) - 1;
auto& scalar = xchange[tid/WARP\_SZ](tid%WARP\_SZ);
#pragma unroll 1
for (uint32\_t i = tix; i < (uint32\_t)len; i += gridDim.x\*blockDim.x) {
auto s = scalars[i];
#if 0
s.from();
if (!mont) s.to();
#else
if (mont) s.from();
#endif
// clear the most significant bit
uint32\_t msb = s.abs();
msb <<= 31;
scalar = s;
#pragma unroll 1
for (uint32\_t bit0 = nwins\*wbits - 1, win = nwins; --win;) {
bit0 -= wbits;
uint32\_t wval = get\_wval(scalar, bit0, top\_i);
wval = booth\_encode(wval, wmask, wbits);
if (wval) wval ^= msb;
digits[win][i] = wval;
}
uint32\_t wval = s[0] << 1;
wval = booth\_encode(wval, wmask, wbits);
if (wval) wval ^= msb;
digits[0][i] = wval;
}
#endif
}
#ifndef LARGE\_L1\_CODE\_CACHE
# if \_\_CUDA\_ARCH\_\_-0 >= 800
# define LARGE\_L1\_CODE\_CACHE 1
# define ACCUMULATE\_NTHREADS 384
# else
# define LARGE\_L1\_CODE\_CACHE 0
# define ACCUMULATE\_NTHREADS (bucket\_t::degree == 1 ? 384 : 256)
# endif
#endif
#ifndef MSM\_NTHREADS
# define MSM\_NTHREADS 256
#endif
#if MSM\_NTHREADS < 32 || (MSM\_NTHREADS & (MSM\_NTHREADS-1)) != 0
# error "bad MSM\_NTHREADS value"
#endif
#ifndef MSM\_NSTREAMS
# define MSM\_NSTREAMS 8
#elif MSM\_NSTREAMS<2
# error "invalid MSM\_NSTREAMS"
#endif
template
\_\_launch\_bounds\_\_(ACCUMULATE\_NTHREADS) \_\_global\_\_
void accumulate(bucket\_h buckets\_[], uint32\_t nwins, uint32\_t wbits,
/\*const\*/ affine\_h points\_[], const vec2d\_t digits,
const vec2d\_t histogram, uint32\_t sid = 0)
{
vec2d\_t buckets{buckets\_, 1U<<--wbits};
const affine\_h\* points = points\_;
static \_\_device\_\_ uint32\_t streams[MSM\_NSTREAMS];
uint32\_t& current = streams[sid % MSM\_NSTREAMS];
uint32\_t laneid;
asm("mov.u32 %0, %laneid;" : "=r"(laneid));
const uint32\_t degree = bucket\_t::degree;
const uint32\_t warp\_sz = WARP\_SZ / degree;
const uint32\_t lane\_id = laneid / degree;
uint32\_t x, y;
#if 1
\_\_shared\_\_ uint32\_t xchg;
if (threadIdx.x == 0)
xchg = atomicAdd(¤t, blockDim.x/degree);
\_\_syncthreads();
x = xchg + threadIdx.x/degree;
#else
x = laneid == 0 ? atomicAdd(¤t, warp\_sz) : 0;
x = \_\_shfl\_sync(0xffffffff, x, 0) + lane\_id;
#endif
while (x < (nwins << wbits)) {
y = x >> wbits;
x &= (1U << wbits) - 1;
const uint32\_t\* h = &histogram[y][x];
uint32\_t idx, len = h[0];
asm("{ .reg.pred %did;"
" shfl.sync.up.b32 %0|%did, %1, %2, 0, 0xffffffff;"
" @!%did mov.b32 %0, 0;"
"}" : "=r"(idx) : "r"(len), "r"(degree));
if (lane\_id == 0 && x != 0)
idx = h[-1];
if ((len -= idx) && !(x == 0 && y == 0)) {
const uint32\_t\* digs\_ptr = &digits[y][idx];
uint32\_t digit = \*digs\_ptr++;
affine\_t p = points[digit & 0x7fffffff];
bucket\_t bucket = p;
bucket.cneg(digit >> 31);
while (--len) {
digit = \*digs\_ptr++;
p = points[digit & 0x7fffffff];
if (sizeof(bucket) <= 128 || LARGE\_L1\_CODE\_CACHE)
bucket.add(p, digit >> 31);
else
bucket.uadd(p, digit >> 31);
}
buckets[y][x] = bucket;
} else {
buckets[y][x].inf();
}
x = laneid == 0 ? atomicAdd(¤t, warp\_sz) : 0;
x = \_\_shfl\_sync(0xffffffff, x, 0) + lane\_id;
}
cooperative\_groups::this\_grid().sync();
if (threadIdx.x + blockIdx.x == 0)
current = 0;
}
template
\_\_launch\_bounds\_\_(256) \_\_global\_\_
void integrate(bucket\_h buckets\_[], uint32\_t nwins, uint32\_t wbits, uint32\_t nbits)
{
const uint32\_t degree = bucket\_t::degree;
uint32\_t Nthrbits = 31 - \_\_clz(blockDim.x / degree);
assert((blockDim.x & (blockDim.x-1)) == 0 && wbits-1 > Nthrbits);
vec2d\_t buckets{buckets\_, 1U<<(wbits-1)};
extern \_\_shared\_\_ uint4 scratch\_[];
auto\* scratch = reinterpret\_cast(scratch\_);
const uint32\_t tid = threadIdx.x / degree;
const uint32\_t bid = blockIdx.x;
auto\* row = &buckets[bid][0];
uint32\_t i = 1U << (wbits-1-Nthrbits);
row += tid \* i;
uint32\_t mask = 0;
if ((bid+1)\*wbits > nbits) {
uint32\_t lsbits = nbits - bid\*wbits;
mask = (1U << (wbits-lsbits)) - 1;
}
bucket\_t res, acc = row[--i];
if (i & mask) {
if (sizeof(res) <= 128) res.inf();
else scratch[tid].inf();
} else {
if (sizeof(res) <= 128) res = acc;
else scratch[tid] = acc;
}
bucket\_t p;
#pragma unroll 1
while (i--) {
p = row[i];
uint32\_t pc = i & mask ? 2 : 0;
#pragma unroll 1
do {
if (sizeof(bucket\_t) <= 128) {
p.add(acc);
if (pc == 1) {
res = p;
} else {
acc = p;
if (pc == 0) p = res;
}
} else {
if (LARGE\_L1\_CODE\_CACHE && degree == 1)
p.add(acc);
else
p.uadd(acc);
if (pc == 1) {
scratch[tid] = p;
} else {
acc = p;
if (pc == 0) p = scratch[tid];
}
}
} while (++pc < 2);
}
\_\_syncthreads();
buckets[bid][2\*tid] = p;
buckets[bid][2\*tid+1] = acc;
}
#undef asm
#ifndef SPPARK\_DONT\_INSTANTIATE\_TEMPLATES
template \_\_global\_\_
void accumulate(bucket\_t::mem\_t buckets\_[],
uint32\_t nwins, uint32\_t wbits,
/\*const\*/ affine\_t::mem\_t points\_[],
const vec2d\_t digits,
const vec2d\_t histogram,
uint32\_t sid);
template \_\_global\_\_
void batch\_addition(bucket\_t::mem\_t buckets[],
const affine\_t::mem\_t points[], size\_t npoints,
const uint32\_t digits[], const uint32\_t& ndigits);
template \_\_global\_\_
void integrate(bucket\_t::mem\_t buckets\_[], uint32\_t nwins,
uint32\_t wbits, uint32\_t nbits);
template \_\_global\_\_
void breakdown(vec2d\_t digits, const scalar\_t scalars[],
size\_t len, uint32\_t nwins, uint32\_t wbits, bool mont);
#endif
#include 
#include 
#include 
#include 
template
class msm\_t {
const gpu\_t& gpu;
size\_t npoints;
uint32\_t wbits, nwins;
bucket\_h \*d\_buckets;
affine\_h \*d\_points;
scalar\_t \*d\_scalars;
vec2d\_t d\_hist;
template using vec\_t = slice\_t;
class result\_t {
bucket\_t ret[MSM\_NTHREADS/bucket\_t::degree][2];
public:
result\_t() {}
inline operator decltype(ret)&() { return ret; }
inline const bucket\_t\* operator[](size\_t i) const { return ret[i]; }
};
constexpr static int lg2(size\_t n)
{ int ret=0; while (n>>=1) ret++; return ret; }
public:
msm\_t(const affine\_t points[], size\_t np,
size\_t ffi\_affine\_sz = sizeof(affine\_t), int device\_id = -1)
: gpu(select\_gpu(device\_id)), d\_points(nullptr), d\_scalars(nullptr)
{
npoints = (np+WARP\_SZ-1) & ((size\_t)0-WARP\_SZ);
wbits = 17;
if (npoints > 192) {
wbits = std::min(lg2(npoints + npoints/2) - 8, 18);
if (wbits < 10)
wbits = 10;
} else if (npoints > 0) {
wbits = 10;
}
nwins = (scalar\_t::bit\_length() - 1) / wbits + 1;
uint32\_t row\_sz = 1U << (wbits-1);
size\_t d\_buckets\_sz = (nwins \* row\_sz)
+ (gpu.sm\_count() \* BATCH\_ADD\_BLOCK\_SIZE / WARP\_SZ);
size\_t d\_blob\_sz = (d\_buckets\_sz \* sizeof(d\_buckets[0]))
+ (nwins \* row\_sz \* sizeof(uint32\_t))
+ (points ? npoints \* sizeof(d\_points[0]) : 0);
d\_buckets = reinterpret\_cast(gpu.Dmalloc(d\_blob\_sz));
d\_hist = vec2d\_t(&d\_buckets[d\_buckets\_sz], row\_sz);
if (points) {
d\_points = reinterpret\_cast(d\_hist[nwins]);
gpu.HtoD(d\_points, points, np, ffi\_affine\_sz);
npoints = np;
} else {
npoints = 0;
}
}
inline msm\_t(vec\_t points, size\_t ffi\_affine\_sz = sizeof(affine\_t),
int device\_id = -1)
: msm\_t(points, points.size(), ffi\_affine\_sz, device\_id) {};
inline msm\_t(int device\_id = -1)
: msm\_t(nullptr, 0, 0, device\_id) {};
~msm\_t()
{
gpu.sync();
if (d\_buckets) gpu.Dfree(d\_buckets);
}
private:
void digits(const scalar\_t d\_scalars[], size\_t len,
vec2d\_t& d\_digits, vec2d\_t&d\_temps, bool mont)
{
// Using larger grid size doesn't make 'sort' run faster, actually
// quite contrary. Arguably because global memory bus gets
// thrashed... Stepping far outside the sweet spot has significant
// impact, 30-40% degradation was observed. It's assumed that all
// GPUs are "balanced" in an approximately the same manner. The
// coefficient was observed to deliver optimal performance on
// Turing and Ampere...
uint32\_t grid\_size = gpu.sm\_count() / 3;
while (grid\_size & (grid\_size - 1))
grid\_size -= (grid\_size & (0 - grid\_size));
breakdown<<<2\*grid\_size, 1024, sizeof(scalar\_t)\*1024, gpu[2]>>>(
d\_digits, d\_scalars, len, nwins, wbits, mont
);
CUDA\_OK(cudaGetLastError());
const size\_t shared\_sz = sizeof(uint32\_t) << DIGIT\_BITS;
#if 0
uint32\_t win;
for (win = 0; win < nwins-1; win++) {
gpu[2].launch\_coop(sort, {grid\_size, SORT\_BLOCKDIM, shared\_sz},
d\_digits, len, win, d\_temps, d\_hist,
wbits-1, wbits-1, 0u);
}
uint32\_t top = scalar\_t::bit\_length() - wbits \* win;
gpu[2].launch\_coop(sort, {grid\_size, SORT\_BLOCKDIM, shared\_sz},
d\_digits, len, win, d\_temps, d\_hist,
wbits-1, top-1, 0u);
#else
// On the other hand a pair of kernels launched in parallel run
// ~50% slower but sort twice as much data...
uint32\_t top = scalar\_t::bit\_length() - wbits \* (nwins-1);
uint32\_t win;
for (win = 0; win < nwins-1; win += 2) {
gpu[2].launch\_coop(sort, {{grid\_size, 2}, SORT\_BLOCKDIM, shared\_sz},
d\_digits, len, win, d\_temps, d\_hist,
wbits-1, wbits-1, win == nwins-2 ? top-1 : wbits-1);
}
if (win < nwins) {
gpu[2].launch\_coop(sort, {{grid\_size, 1}, SORT\_BLOCKDIM, shared\_sz},
d\_digits, len, win, d\_temps, d\_hist,
wbits-1, top-1, 0u);
}
#endif
}
public:
RustError invoke(point\_t& out, const affine\_t\* points\_, size\_t npoints,
const scalar\_t\* scalars, bool mont = true,
size\_t ffi\_affine\_sz = sizeof(affine\_t))
{
assert(this->npoints == 0 || npoints <= this->npoints);
uint32\_t lg\_npoints = lg2(npoints + npoints/2);
size\_t batch = 1 << (std::max(lg\_npoints, wbits) - wbits);
batch >>= 6;
batch = batch ? batch : 1;
uint32\_t stride = (npoints + batch - 1) / batch;
stride = (stride+WARP\_SZ-1) & ((size\_t)0-WARP\_SZ);
std::vector res(nwins);
std::vector ones(gpu.sm\_count() \* BATCH\_ADD\_BLOCK\_SIZE / WARP\_SZ);
out.inf();
point\_t p;
try {
// |scalars| being nullptr means the scalars are pre-loaded to
// |d\_scalars|, otherwise allocate stride.
size\_t temp\_sz = scalars ? sizeof(scalar\_t) : 0;
temp\_sz = stride \* std::max(2\*sizeof(uint2), temp\_sz);
// |points| being nullptr means the points are pre-loaded to
// |d\_points|, otherwise allocate double-stride.
const char\* points = reinterpret\_cast(points\_);
size\_t d\_point\_sz = points ? (batch > 1 ? 2\*stride : stride) : 0;
d\_point\_sz \*= sizeof(affine\_h);
size\_t digits\_sz = nwins \* stride \* sizeof(uint32\_t);
dev\_ptr\_t d\_temp{temp\_sz + digits\_sz + d\_point\_sz, gpu[2]};
vec2d\_t d\_temps{&d\_temp[0], stride};
vec2d\_t d\_digits{&d\_temp[temp\_sz], stride};
scalar\_t\* d\_scalars = scalars ? (scalar\_t\*)&d\_temp[0]
: this->d\_scalars;
affine\_h\* d\_points = points ? (affine\_h\*)&d\_temp[temp\_sz + digits\_sz]
: this->d\_points;
size\_t d\_off = 0; // device offset
size\_t h\_off = 0; // host offset
size\_t num = stride > npoints ? npoints : stride;
event\_t ev;
if (scalars)
gpu[2].HtoD(&d\_scalars[d\_off], &scalars[h\_off], num);
digits(&d\_scalars[0], num, d\_digits, d\_temps, mont);
gpu[2].record(ev);
if (points)
gpu[0].HtoD(&d\_points[d\_off], &points[h\_off],
num, ffi\_affine\_sz);
for (uint32\_t i = 0; i < batch; i++) {
gpu[i&1].wait(ev);
batch\_addition<<>>(
&d\_buckets[nwins << (wbits-1)], &d\_points[d\_off], num,
&d\_digits[0][0], d\_hist[0][0]
);
CUDA\_OK(cudaGetLastError());
gpu[i&1].launch\_coop(accumulate,
{gpu.sm\_count(), 0},
d\_buckets, nwins, wbits, &d\_points[d\_off], d\_digits, d\_hist, i&1
);
gpu[i&1].record(ev);
integrate<<>>(
d\_buckets, nwins, wbits, scalar\_t::bit\_length()
);
CUDA\_OK(cudaGetLastError());
if (i < batch-1) {
h\_off += stride;
num = h\_off + stride <= npoints ? stride : npoints - h\_off;
if (scalars)
gpu[2].HtoD(&d\_scalars[0], &scalars[h\_off], num);
gpu[2].wait(ev);
digits(&d\_scalars[scalars ? 0 : h\_off], num,
d\_digits, d\_temps, mont);
gpu[2].record(ev);
if (points) {
size\_t j = (i + 1) & 1;
d\_off = j ? stride : 0;
gpu[j].HtoD(&d\_points[d\_off], &points[h\_off\*ffi\_affine\_sz],
num, ffi\_affine\_sz);
} else {
d\_off = h\_off;
}
}
if (i > 0) {
collect(p, res, ones);
out.add(p);
}
gpu[i&1].DtoH(ones, d\_buckets + (nwins << (wbits-1)));
gpu[i&1].DtoH(res, d\_buckets, sizeof(bucket\_h)<<(wbits-1));
gpu[i&1].sync();
}
} catch (const cuda\_error& e) {
gpu.sync();
#ifdef TAKE\_RESPONSIBILITY\_FOR\_ERROR\_MESSAGE
return RustError{e.code(), e.what()};
#else
return RustError{e.code()};
#endif
}
collect(p, res, ones);
out.add(p);
return RustError{cudaSuccess};
}
#if 0
RustError invoke(point\_t& out, const affine\_t\* points, size\_t npoints,
gpu\_ptr\_t scalars, bool mont = true,
size\_t ffi\_affine\_sz = sizeof(affine\_t))
{
d\_scalars = scalars;
return invoke(out, points, npoints, nullptr, mont, ffi\_affine\_sz);
}
#else
template
RustError invoke(point\_t& out, affine\_ptr\_t points, size\_t npoints,
scalar\_ptr\_t scalars, bool mont = true,
size\_t ffi\_affine\_sz = sizeof(affine\_t))
{
const auto\* p\_ptr = &points[0];
if (is\_device\_ptr::value) {
d\_points = (decltype(d\_points))p\_ptr;
p\_ptr = nullptr;
}
const auto\* s\_ptr = &scalars[0];
if (is\_device\_ptr::value) {
d\_scalars = const\_cast(s\_ptr);
s\_ptr = nullptr;
}
return invoke(out, p\_ptr, npoints, s\_ptr, mont, ffi\_affine\_sz);
}
#endif
RustError invoke(point\_t& out, vec\_t scalars, bool mont = true)
{ return invoke(out, nullptr, scalars.size(), scalars, mont); }
RustError invoke(point\_t& out, vec\_t points,
const scalar\_t\* scalars, bool mont = true,
size\_t ffi\_affine\_sz = sizeof(affine\_t))
{ return invoke(out, points, points.size(), scalars, mont, ffi\_affine\_sz); }
RustError invoke(point\_t& out, vec\_t points,
vec\_t scalars, bool mont = true,
size\_t ffi\_affine\_sz = sizeof(affine\_t))
{ return invoke(out, points, points.size(), scalars, mont, ffi\_affine\_sz); }
RustError invoke(point\_t& out, const std::vector& points,
const std::vector& scalars, bool mont = true,
size\_t ffi\_affine\_sz = sizeof(affine\_t))
{
return invoke(out, points.data(),
std::min(points.size(), scalars.size()),
scalars.data(), mont, ffi\_affine\_sz);
}
private:
point\_t integrate\_row(const result\_t& row, uint32\_t lsbits)
{
const int NTHRBITS = lg2(MSM\_NTHREADS/bucket\_t::degree);
assert(wbits-1 > NTHRBITS);
size\_t i = MSM\_NTHREADS/bucket\_t::degree - 1;
if (lsbits-1 <= NTHRBITS) {
size\_t mask = (1U << (NTHRBITS-(lsbits-1))) - 1;
bucket\_t res, acc = row[i][1];
if (mask) res.inf();
else res = acc;
while (i--) {
acc.add(row[i][1]);
if ((i & mask) == 0)
res.add(acc);
}
return res;
}
point\_t res = row[i][0];
bucket\_t acc = row[i][1];
while (i--) {
point\_t raise = acc;
for (size\_t j = 0; j < lsbits-1-NTHRBITS; j++)
raise.dbl();
res.add(raise);
res.add(point\_t{row[i][0]});
if (i)
acc.add(row[i][1]);
}
return res;
}
void collect(point\_t& out, const std::vector& res,
const std::vector& ones)
{
struct tile\_t {
uint32\_t x, y, dy;
point\_t p;
tile\_t() {}
};
std::vector grid(nwins);
uint32\_t y = nwins-1, total = 0;
grid[0].x = 0;
grid[0].y = y;
grid[0].dy = scalar\_t::bit\_length() - y\*wbits;
total++;
while (y--) {
grid[total].x = grid[0].x;
grid[total].y = y;
grid[total].dy = wbits;
total++;
}
std::vector> row\_sync(nwins); /\* zeroed \*/
counter\_t counter(0);
channel\_t ch;
auto n\_workers = min((uint32\_t)gpu.ncpus(), total);
while (n\_workers--) {
gpu.spawn([&, this, total, counter]() {
for (size\_t work; (work = counter++) < total;) {
auto item = &grid[work];
auto y = item->y;
item->p = integrate\_row(res[y], item->dy);
if (++row\_sync[y] == 1)
ch.send(y);
}
});
}
point\_t one = sum\_up(ones);
out.inf();
size\_t row = 0, ny = nwins;
while (ny--) {
auto y = ch.recv();
row\_sync[y] = -1U;
while (grid[row].y == y) {
while (row < total && grid[row].y == y)
out.add(grid[row++].p);
if (y == 0)
break;
for (size\_t i = 0; i < wbits; i++)
out.dbl();
if (row\_sync[--y] != -1U)
break;
}
}
out.add(one);
}
};
template static
RustError mult\_pippenger(point\_t \*out, const affine\_t points[], size\_t npoints,
const scalar\_t scalars[], bool mont = true,
size\_t ffi\_affine\_sz = sizeof(affine\_t))
{
try {
msm\_t msm{nullptr, npoints};
return msm.invoke(\*out, slice\_t{points, npoints},
scalars, mont, ffi\_affine\_sz);
} catch (const cuda\_error& e) {
out->inf();
#ifdef TAKE\_RESPONSIBILITY\_FOR\_ERROR\_MESSAGE
return RustError{e.code(), e.what()};
#else
return RustError{e.code()};
#endif
}
}
#endif