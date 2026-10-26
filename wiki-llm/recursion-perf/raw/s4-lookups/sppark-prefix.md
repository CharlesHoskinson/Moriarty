# sppark polynomial prefix operation
> Source: https://raw.githubusercontent.com/supranational/sppark/main/polynomial/prefix_op.cuh
> Collected: 2026-10-26
> Published: Unknown

// Copyright Supranational LLC
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0
#if !defined(\_\_SPPARK\_POLYNOMIAL\_PREFIX\_OP\_CUH\_\_) && \
(defined(\_\_CUDACC\_\_) || defined(\_\_HIPCC\_\_))
#define \_\_SPPARK\_POLYNOMIAL\_PREFIX\_OP\_CUH\_\_
#include 
#ifdef \_\_HIPCC\_\_
# include 
#else
# include 
#endif
#include 
template 
class Add {
public:
using T = fr\_t;
static const int CHUNK = 192 / sizeof(fr\_t);
static\_assert(CHUNK != 0, "field size is too large");
\_\_device\_\_ \_\_host\_\_ \_\_forceinline\_\_
fr\_t operator()(const fr\_t& a, const fr\_t& b) const
{ return a + b; }
\_\_device\_\_ \_\_host\_\_ \_\_forceinline\_\_
fr\_t identity() const
{ fr\_t ret; ret.zero(); return ret; }
};
template 
class Multiply {
public:
using T = fr\_t;
static const int CHUNK = 128 / sizeof(fr\_t);
static\_assert(CHUNK != 0, "field size is too large");
\_\_device\_\_ \_\_host\_\_ \_\_forceinline\_\_
fr\_t operator()(const fr\_t& a, const fr\_t& b) const
{ return a \* b; }
\_\_device\_\_ \_\_host\_\_ \_\_forceinline\_\_
fr\_t identity() const
{ return fr\_t::one(); }
};
template 
\_\_global\_\_ \_\_launch\_bounds\_\_(sizeof(fr\_t)<=16 ? 1024 : 512)
void d\_prefix\_op(OutPtr out, InPtr inp, size\_t len)
{
struct warp {
\_\_device\_\_ \_\_forceinline\_\_
static fr\_t& prefix\_op(fr\_t& x\_lane, uint32\_t limit = WARP\_SZ)
{
const uint32\_t laneid = threadIdx.x % WARP\_SZ;
const Operation op;
\_\_builtin\_assume(limit > 1);
#pragma unroll 1
for (uint32\_t offset = 1; offset < limit; offset <<= 1) {
fr\_t temp = shfl\_up(x\_lane, offset);
temp = op(temp, x\_lane);
x\_lane = fr\_t::csel(x\_lane, temp, laneid < offset);
}
return x\_lane;
}
/\*
\* Compiler apparently doesn't fuse loops, at least not these ones,
\* it has to be done "manually."
\*/
\_\_device\_\_ \_\_forceinline\_\_
static void prefix\_op(fr\_t chunk[CHUNK])
{
const uint32\_t laneid = threadIdx.x % WARP\_SZ;
const Operation op;
#pragma unroll 1
for (uint32\_t offset = 1; offset < WARP\_SZ; offset <<= 1) {
#pragma unroll
for (int i = 0; i < CHUNK; i++) {
fr\_t temp = shfl\_up(chunk[i], offset);
temp = op(temp, chunk[i]);
chunk[i] = fr\_t::csel(chunk[i], temp, laneid < offset);
}
}
}
#ifdef \_\_HIPCC\_\_
static \_\_device\_\_ \_\_noinline\_\_ void noop() { asm(""); }
#else
static \_\_device\_\_ \_\_forceinline\_\_ void noop() {}
#endif
};
#if 0
assert(blockDim.x%WARP\_SZ == 0 && gridDim.x <= blockDim.x);
#endif
const uint32\_t tid = threadIdx.x + blockDim.x \* blockIdx.x;
const uint32\_t warpid = threadIdx.x / WARP\_SZ;
const uint32\_t laneid = threadIdx.x % WARP\_SZ;
const uint32\_t nwarps = blockDim.x / WARP\_SZ;
static \_\_shared\_\_ fr\_t xchg[1024/WARP\_SZ]; // 1024 is maximum for blockDim
const uint32\_t chunk\_size = blockDim.x \* CHUNK;
const uint32\_t blob\_size = gridDim.x \* chunk\_size;
constexpr bool coalesce = CHUNK/sizeof(fr\_t) > 1;
const uint32\_t lane\_off = (tid / WARP\_SZ) \* WARP\_SZ \* CHUNK +
(coalesce ? laneid : laneid\*CHUNK);
const Operation op;
const fr\_t identity = op.identity();
fr\_t grid\_carry = identity;
fr\_t chunk[CHUNK];
const bool do\_prefetch = true;
fr\_t prefetch;
if (do\_prefetch) {
#ifdef \_\_CUDA\_ARCH\_\_
prefetch = identity;
if (lane\_off < len)
prefetch = inp[lane\_off];
#else
prefetch = inp[lane\_off 0);
#pragma unroll 1
for (size\_t blob = 0; blob < len; blob += blob\_size) {
size\_t lane\_idx = blob + lane\_off;
int top = CHUNK;
if (do\_prefetch) {
if (lane\_idx >= len)
top = 0;
chunk[0] = prefetch;
}
#pragma unroll
for (int i = do\_prefetch; i < CHUNK; i++) {
size\_t idx = lane\_idx + (coalesce ? WARP\_SZ\*i : i);
if (top == CHUNK && idx >= len)
top = i;
#ifdef \_\_CUDA\_ARCH\_\_
chunk[i] = identity;
if (i < top)
chunk[i] = inp[idx];
#else
chunk[i] = inp[i 0)
warp\_carry = xchg[laneid - 1];
warp\_carry = warp::prefix\_op(warp\_carry, nwarps);
warp\_carry = shfl\_idx(warp\_carry, warpid);
if (coalesce) {
chunk[CHUNK - 1] = op(chunk[CHUNK - 1], warp\_carry);
} else {
fr\_t lane\_carry = shfl\_up(chunk[CHUNK-1], 1);
lane\_carry = fr\_t::csel(identity, lane\_carry, laneid == 0);
chunk[CHUNK - 1] = op(chunk[CHUNK - 1], warp\_carry);
warp\_carry = op(warp\_carry, lane\_carry);
}
fr\_t grid\_carry\_in = grid\_carry;
if (gridDim.x == 1) {
if (threadIdx.x == blockDim.x-1)
xchg[1024/WARP\_SZ - 1] = chunk[CHUNK - 1];
\_\_syncthreads();
grid\_carry = xchg[1024/WARP\_SZ - 1];
grid\_carry = op(grid\_carry, grid\_carry\_in);
} else {
bool tail\_sync = !do\_prefetch ||
(len - blob) <= (2\*blob\_size - chunk\_size);
uint32\_t bias = tail\_sync ? 0 : blob\_size;
size\_t grid\_xchg = blob + (blockIdx.x\*chunk\_size + bias);
if (threadIdx.x == blockDim.x-1 && grid\_xchg < len)
out[grid\_xchg] = chunk[CHUNK - 1];
cooperative\_groups::this\_grid().sync();
\_\_syncthreads();
uint32\_t rank = gridDim.x <= WARP\_SZ ? laneid : threadIdx.x;
fr\_t block\_carry = identity;
grid\_xchg = blob + (rank\*chunk\_size + bias);
if (rank < gridDim.x && grid\_xchg < len)
block\_carry = out[grid\_xchg];
block\_carry = warp::prefix\_op(block\_carry, min(WARP\_SZ, gridDim.x));
if (gridDim.x <= WARP\_SZ) {
grid\_carry = op(grid\_carry, shfl\_idx(block\_carry, gridDim.x-1));
if (blockIdx.x > 0)
block\_carry = shfl\_idx(block\_carry, blockIdx.x - 1);
else
block\_carry = identity;
} else {
const uint32\_t nwarps = (gridDim.x + WARP\_SZ - 1) / WARP\_SZ;
// The last warp saves a value only when gridDim.x is
// divisible by WARP\_SZ. However, note that this value is
// later disregarded, hence there is no need to handle all
// divisibility cases, comparison to gridDim.x suffices.
if (laneid == WARP\_SZ - 1 && threadIdx.x < gridDim.x)
xchg[warpid] = block\_carry;
\_\_syncthreads();
fr\_t inner\_carry = identity;
if (laneid < nwarps && laneid > 0)
inner\_carry = xchg[laneid - 1];
inner\_carry = warp::prefix\_op(inner\_carry, nwarps);
inner\_carry = shfl\_idx(inner\_carry, warpid);
\_\_syncthreads();
if (threadIdx.x == blockIdx.x-1 || threadIdx.x == gridDim.x-1) {
block\_carry = op(block\_carry, inner\_carry);
xchg[threadIdx.x == gridDim.x-1] = block\_carry;
}
\_\_syncthreads();
if (blockIdx.x > 0)
block\_carry = xchg[0];
else
block\_carry = identity;
grid\_carry = xchg[1];
grid\_carry = op(grid\_carry, grid\_carry\_in);
}
if (tail\_sync) {
cooperative\_groups::this\_grid().sync();
\_\_syncthreads();
}
grid\_carry\_in = op(grid\_carry\_in, block\_carry);
}
warp\_carry = op(warp\_carry, grid\_carry\_in);
#pragma unroll
for (int i = 0; i < CHUNK-1; i++)
chunk[i] = op(chunk[i], warp\_carry);
chunk[CHUNK - 1] = op(chunk[CHUNK - 1], grid\_carry\_in);
#pragma unroll
for(int i = 0; i < CHUNK; i++) {
if (i < top)
out[lane\_idx + (coalesce ? WARP\_SZ\*i : i)] = chunk[i];
}
}
}
template 
void prefix\_op(OutPtr d\_out, InPtr d\_inp, size\_t len, const stream\_t& s,
int gridDim = 0)
{
if (gridDim <= 0)
gridDim = s.sm\_count();
int blockDim = sizeof(fr\_t)<=16 ? 1024 : 512;
int chunkSize = blockDim \* Operation::CHUNK;
size\_t blocks = (len + chunkSize - 1)/chunkSize;
# define \_\_PREFIX\_OP\_\_(N) d\_prefix\_op
if (blocks <= (unsigned)gridDim) {
if ((Operation::CHUNK/4)\*4 == Operation::CHUNK &&
blocks\*4 <= (unsigned)gridDim) {
// +70-90% improvement depending on field and GPU
chunkSize = blockDim \* Operation::CHUNK/4;
gridDim = (int)((len + chunkSize - 1)/chunkSize);
s.launch\_coop(\_\_PREFIX\_OP\_\_(Operation::CHUNK/4),
{gridDim, blockDim},
d\_out, d\_inp, len);
return;
} else if ((Operation::CHUNK/2)\*2 == Operation::CHUNK &&
blocks\*2 <= (unsigned)gridDim) {
// +20-40% improvement depending on field and GPU
chunkSize = blockDim \* Operation::CHUNK/2;
gridDim = (int)((len + chunkSize - 1)/chunkSize);
s.launch\_coop(\_\_PREFIX\_OP\_\_(Operation::CHUNK/2),
{gridDim, blockDim},
d\_out, d\_inp, len);
return;
} else if ((Operation::CHUNK\*3/4)\*4 == Operation::CHUNK\*3 &&
blocks\*4 <= (unsigned)gridDim\*3) {
// +10-20% improvement depending on field and GPU
chunkSize = blockDim \* Operation::CHUNK\*3/4;
gridDim = (int)((len + chunkSize - 1)/chunkSize);
s.launch\_coop(\_\_PREFIX\_OP\_\_(Operation::CHUNK\*3/4),
{gridDim, blockDim},
d\_out, d\_inp, len);
return;
}
gridDim = (int)blocks;
} else if ((Operation::CHUNK\*3/4)\*4 == Operation::CHUNK\*3) {
chunkSize = blockDim \* (Operation::CHUNK\*3/4);
blocks = (len + chunkSize - 1)/chunkSize;
if (blocks <= (unsigned)gridDim\*2) {
// +10-20% improvement depending on field and GPU
s.launch\_coop(\_\_PREFIX\_OP\_\_(Operation::CHUNK\*3/4),
{gridDim, blockDim},
d\_out, d\_inp, len);
return;
}
}
s.launch\_coop(\_\_PREFIX\_OP\_\_(Operation::CHUNK),
{gridDim, blockDim},
d\_out, d\_inp, len);
# undef \_\_PREFIX\_OP\_\_
}
template 
void prefix\_op(fr\_t\* d\_inout, size\_t len, const stream\_t& s, int gridDim = 0)
{
prefix\_op(d\_inout, reinterpret\_cast(d\_inout), len,
s, gridDim);
}
#endif