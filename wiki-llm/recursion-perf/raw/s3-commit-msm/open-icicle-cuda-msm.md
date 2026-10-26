# Open ICICLE CUDA MSM source
> Source: https://raw.githubusercontent.com/ingonyama-zk/open-icicle/main/icicle/backend/cuda/src/msm/cuda_msm.cuh
> Collected: 2026-10-26
> Published: Unknown

#include "icicle/backend/msm\_backend.h"
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include 
#include "icicle/curves/affine.h"
#include "icicle/curves/projective.h"
#include "icicle/fields/field.h"
#include "gpu-utils/error\_handler.h"
#include "cuda\_mont.cuh"
#include "msm/cuda\_msm\_config.cuh"
#include "error\_translation.h"
namespace msm {
namespace {
#define MAX\_TH 256
#define MAX\_C\_FOR\_PRECOMPUTATION 22
template 
\_\_global\_\_ void
precompute\_points\_kernel(const A\* points, int shift, int prec\_factor, int count, A\* points\_out, bool is\_montgomery)
{
int tid = blockIdx.x \* blockDim.x + threadIdx.x;
if (tid >= count) return;
A point\_a = is\_montgomery ? points[tid].from\_montgomery() : points[tid];
P point = P::from\_affine(point\_a);
points\_out[tid \* prec\_factor] = point.to\_affine();
for (int i = 1; i < prec\_factor; i++) {
for (unsigned j = 0; j < shift; j++)
point = point.dbl();
points\_out[tid \* prec\_factor + i] = point.to\_affine();
}
}
unsigned get\_optimal\_c(int msm\_size)
{
return (unsigned)min(max(ceil(std::log2(msm\_size)) - 4.0, 1.0), 20.0);
} // 20 as the largest value is a temporary fix for the large buckets problem
\_\_global\_\_ void normalize\_kernel(unsigned\* inout, unsigned factor, int n)
{
int tid = blockIdx.x \* blockDim.x + threadIdx.x;
if (tid < n) inout[tid] = (inout[tid] + factor - 1) / factor;
}
// a kernel that writes to bucket\_indices which enables large bucket accumulation to happen afterwards.
// specifically we map thread indices to buckets which said threads will handle in accumulation.
template 
\_\_global\_\_ void initialize\_large\_bucket\_indices(
unsigned\* sorted\_bucket\_sizes\_sum,
unsigned nof\_pts\_per\_thread,
unsigned nof\_large\_buckets,
// log\_nof\_buckets\_to\_compute should be equal to ceil(log(nof\_buckets\_to\_compute))
unsigned log\_nof\_large\_buckets,
unsigned\* bucket\_indices)
{
const int tid = blockIdx.x \* blockDim.x + threadIdx.x;
if (tid >= nof\_large\_buckets) { return; }
unsigned start = (sorted\_bucket\_sizes\_sum[tid] + nof\_pts\_per\_thread - 1) / nof\_pts\_per\_thread + tid;
unsigned end = (sorted\_bucket\_sizes\_sum[tid + 1] + nof\_pts\_per\_thread - 1) / nof\_pts\_per\_thread + tid + 1;
for (unsigned i = start; i < end; i++) {
// this just concatenates two pieces of data - large bucket index and (i - start)
bucket\_indices[i] = tid | ((i - start) << log\_nof\_large\_buckets);
}
}
// this function provides a single step of reduction across buckets sizes of
// which are given by large\_bucket\_sizes pointer
template 
\_\_global\_\_ void sum\_reduction\_variable\_size\_kernel(
P\* v,
unsigned\* bucket\_sizes\_sum,
unsigned\* bucket\_sizes,
unsigned\* large\_bucket\_thread\_indices,
unsigned nof\_threads)
{
const int tid = blockIdx.x \* blockDim.x + threadIdx.x;
if (tid >= nof\_threads) { return; }
unsigned large\_bucket\_tid = large\_bucket\_thread\_indices[tid];
unsigned segment\_ind = tid - bucket\_sizes\_sum[large\_bucket\_tid] - large\_bucket\_tid;
unsigned large\_bucket\_size = bucket\_sizes[large\_bucket\_tid];
if (segment\_ind < (large\_bucket\_size >> 1)) { v[tid] = v[tid] + v[tid + ((large\_bucket\_size + 1) >> 1)]; }
}
template 
\_\_global\_\_ void single\_stage\_multi\_reduction\_kernel(
const P\* v,
P\* v\_r,
unsigned orig\_block\_size,
unsigned block\_size,
unsigned write\_stride,
unsigned buckets\_per\_bm,
unsigned write\_phase,
unsigned step,
unsigned nof\_threads)
{
const int tid = blockIdx.x \* blockDim.x + threadIdx.x;
if (tid >= nof\_threads) return;
// we need shifted tid because we don't want to be reducing into zero buckets, this allows to skip them.
// for write\_phase==1, the read pattern is different so we don't skip over anything.
const int shifted\_tid = write\_phase ? tid : tid + (tid + step) / step;
const int jump = block\_size / 2;
const int block\_id = shifted\_tid / jump;
// here the reason for shifting is the same as for shifted\_tid but we skip over entire blocks which happens
// only for write\_phase=1 because of its read pattern.
const int shifted\_block\_id = write\_phase ? block\_id + (block\_id + step) / step : block\_id;
const int block\_tid = shifted\_tid % jump;
const unsigned read\_ind = orig\_block\_size \* shifted\_block\_id + block\_tid;
const unsigned write\_ind = jump \* shifted\_block\_id + block\_tid;
const unsigned v\_r\_key =
write\_stride ? ((write\_ind / buckets\_per\_bm) \* 2 + write\_phase) \* write\_stride + write\_ind % buckets\_per\_bm
: read\_ind;
v\_r[v\_r\_key] = v[read\_ind] + v[read\_ind + jump];
}
// this kernel performs single scalar multiplication
// each thread multiplies a single scalar and point
template 
\_\_global\_\_ void ssm\_kernel(const S\* scalars, const P\* points, P\* results, unsigned N)
{
unsigned tid = (blockIdx.x \* blockDim.x) + threadIdx.x;
if (tid < N) results[tid] = scalars[tid] \* points[tid];
}
// this kernel sums all the elements in a given vector using multiple threads
template 
\_\_global\_\_ void sum\_reduction\_kernel(P\* v, P\* v\_r)
{
unsigned tid = blockIdx.x \* blockDim.x + threadIdx.x;
// Start at 1/2 block stride and divide by two each iteration
for (unsigned s = blockDim.x / 2; s > 0; s >>= 1) {
// Each thread does work unless it is further than the stride
if (threadIdx.x < s) { v[tid] = v[tid] + v[tid + s]; }
\_\_syncthreads();
}
// Let the thread 0 for this block write the final result
if (threadIdx.x == 0) { v\_r[blockIdx.x] = v[tid]; }
}
// this kernel initializes the buckets with zero points
// each thread initializes a different bucket
template 
\_\_global\_\_ void initialize\_buckets\_kernel(P\* buckets, unsigned N)
{
unsigned tid = (blockIdx.x \* blockDim.x) + threadIdx.x;
if (tid < N) buckets[tid] = P::zero(); // zero point
}
// this kernel splits the scalars into digits of size c
// each thread splits a single scalar into nof\_bms digits
template 
\_\_global\_\_ void split\_scalars\_kernel(
unsigned\* buckets\_indices,
unsigned\* point\_indices,
const S\* scalars,
unsigned nof\_scalars,
unsigned points\_size,
unsigned msm\_size,
unsigned nof\_bms\_per\_msm,
unsigned bm\_bitsize,
unsigned c,
unsigned precompute\_factor,
unsigned nof\_precomputed\_bms\_per\_msm)
{
unsigned tid = (blockIdx.x \* blockDim.x) + threadIdx.x;
if (tid >= nof\_scalars) return;
unsigned bucket\_index;
unsigned current\_index;
unsigned msm\_index = tid / msm\_size;
const S& scalar = scalars[tid];
for (unsigned bm = 0; bm < nof\_bms\_per\_msm; bm++) {
const unsigned precomputed\_index = bm / nof\_precomputed\_bms\_per\_msm;
const unsigned target\_bm = bm % nof\_precomputed\_bms\_per\_msm;
bucket\_index = scalar.get\_scalar\_digit(bm, c);
current\_index = bm \* nof\_scalars + tid;
if (bucket\_index != 0) {
buckets\_indices[current\_index] =
(msm\_index << (c + bm\_bitsize)) | (target\_bm << c) |
bucket\_index; // the bucket module number and the msm number are appended at the msbs
} else {
buckets\_indices[current\_index] = 0; // will be skipped
}
point\_indices[current\_index] =
(tid \* precompute\_factor + precomputed\_index) % points\_size; // the point index is saved for later
}
}
template 
\_\_global\_\_ void
find\_cutoff\_kernel(unsigned\* v, unsigned size, unsigned cutoff, unsigned run\_length, unsigned\* result)
{
unsigned tid = (blockIdx.x \* blockDim.x) + threadIdx.x;
const unsigned nof\_threads = (size + run\_length - 1) / run\_length;
if (tid >= nof\_threads) { return; }
const unsigned start\_index = tid \* run\_length;
for (int i = start\_index; i < min(start\_index + run\_length, size - 1); i++) {
if (v[i] > cutoff && v[i + 1] <= cutoff) {
result[0] = i + 1;
return;
}
}
if (tid == 0 && v[size - 1] > cutoff) { result[0] = size; }
}
// this kernel adds up the points in each bucket
template 
\_\_global\_\_ void accumulate\_buckets\_kernel(
P\* \_\_restrict\_\_ buckets,
unsigned\* \_\_restrict\_\_ bucket\_offsets,
unsigned\* \_\_restrict\_\_ bucket\_sizes,
unsigned\* \_\_restrict\_\_ single\_bucket\_indices,
const unsigned\* \_\_restrict\_\_ point\_indices,
A\* \_\_restrict\_\_ points,
const unsigned nof\_buckets,
const unsigned nof\_buckets\_to\_compute,
const unsigned msm\_idx\_shift,
const unsigned c,
bool init\_buckets)
{
unsigned tid = (blockIdx.x \* blockDim.x) + threadIdx.x;
if (tid >= nof\_buckets\_to\_compute) return;
unsigned msm\_index = single\_bucket\_indices[tid] >> msm\_idx\_shift;
const unsigned single\_bucket\_index = (single\_bucket\_indices[tid] & ((1 << msm\_idx\_shift) - 1));
unsigned bucket\_index = msm\_index \* nof\_buckets + single\_bucket\_index;
const unsigned bucket\_offset = bucket\_offsets[tid];
const unsigned bucket\_size = bucket\_sizes[tid];
P bucket;
if (!init\_buckets) bucket = buckets[bucket\_index];
for (unsigned i = 0; i < bucket\_size;
i++) { // add the relevant points starting from the relevant offset up to the bucket size
unsigned point\_ind = point\_indices[bucket\_offset + i];
A point = points[point\_ind];
bucket = i || !init\_buckets ? (point.is\_zero() ? bucket : bucket + point)
: (point.is\_zero() ? P::zero() : P::from\_affine(point));
}
buckets[bucket\_index] = bucket;
}
template 
\_\_global\_\_ void accumulate\_large\_buckets\_kernel(
P\* \_\_restrict\_\_ buckets,
unsigned\* \_\_restrict\_\_ bucket\_offsets,
unsigned\* \_\_restrict\_\_ bucket\_sizes,
unsigned\* \_\_restrict\_\_ large\_bucket\_thread\_indices,
unsigned\* \_\_restrict\_\_ point\_indices,
A\* \_\_restrict\_\_ points,
const unsigned nof\_buckets\_to\_compute,
const unsigned c,
const int points\_per\_thread,
// log\_nof\_buckets\_to\_compute should be equal to ceil(log(nof\_buckets\_to\_compute))
const unsigned log\_nof\_buckets\_to\_compute,
const unsigned nof\_threads)
{
unsigned tid = (blockIdx.x \* blockDim.x) + threadIdx.x;
if (tid >= nof\_threads) return;
int bucket\_segment\_index = large\_bucket\_thread\_indices[tid] >> log\_nof\_buckets\_to\_compute;
large\_bucket\_thread\_indices[tid] &= ((1 << log\_nof\_buckets\_to\_compute) - 1);
int bucket\_ind = large\_bucket\_thread\_indices[tid];
const unsigned bucket\_offset = bucket\_offsets[bucket\_ind] + bucket\_segment\_index \* points\_per\_thread;
const unsigned bucket\_size = max(0, (int)bucket\_sizes[bucket\_ind] - bucket\_segment\_index \* points\_per\_thread);
P bucket;
unsigned run\_length = min(bucket\_size, points\_per\_thread);
for (unsigned i = 0; i < run\_length;
i++) { // add the relevant points starting from the relevant offset up to the bucket size
unsigned point\_ind = point\_indices[bucket\_offset + i];
A point = points[point\_ind];
bucket =
i ? (point.is\_zero() ? bucket : bucket + point) : (point.is\_zero() ? P::zero() : P::from\_affine(point));
}
buckets[tid] = run\_length ? bucket : P::zero();
}
template 
\_\_global\_\_ void distribute\_large\_buckets\_kernel(
const P\* large\_buckets,
P\* buckets,
const unsigned\* sorted\_bucket\_sizes\_sum,
const unsigned\* single\_bucket\_indices,
const unsigned size,
const unsigned nof\_buckets,
const unsigned msm\_idx\_shift,
bool init\_buckets)
{
unsigned tid = (blockIdx.x \* blockDim.x) + threadIdx.x;
if (tid >= size) { return; }
unsigned msm\_index = single\_bucket\_indices[tid] >> msm\_idx\_shift;
unsigned bucket\_index = msm\_index \* nof\_buckets + (single\_bucket\_indices[tid] & ((1 << msm\_idx\_shift) - 1));
unsigned large\_bucket\_index = sorted\_bucket\_sizes\_sum[tid] + tid;
buckets[bucket\_index] =
init\_buckets ? large\_buckets[large\_bucket\_index] : buckets[bucket\_index] + large\_buckets[large\_bucket\_index];
}
// this kernel sums the entire bucket module
// each thread deals with a single bucket module
template 
\_\_global\_\_ void big\_triangle\_sum\_kernel(const P\* buckets, P\* final\_sums, unsigned nof\_bms, unsigned c)
{
unsigned tid = (blockIdx.x \* blockDim.x) + threadIdx.x;
if (tid >= nof\_bms) return;
unsigned buckets\_in\_bm = (1 << c);
P line\_sum = buckets[(tid + 1) \* buckets\_in\_bm - 1];
final\_sums[tid] = line\_sum;
for (unsigned i = buckets\_in\_bm - 2; i > 0; i--) {
line\_sum = line\_sum + buckets[tid \* buckets\_in\_bm + i]; // using the running sum method
final\_sums[tid] = final\_sums[tid] + line\_sum;
}
}
// this kernel uses single scalar multiplication to multiply each bucket by its index
// each thread deals with a single bucket
template 
\_\_global\_\_ void ssm\_buckets\_kernel(P\* buckets, unsigned\* single\_bucket\_indices, unsigned nof\_buckets, unsigned c)
{
unsigned tid = (blockIdx.x \* blockDim.x) + threadIdx.x;
if (tid >= nof\_buckets) return;
unsigned bucket\_index = single\_bucket\_indices[tid];
S scalar\_bucket\_multiplier;
scalar\_bucket\_multiplier = {
bucket\_index & ((1 << c) - 1), 0, 0, 0, 0, 0, 0, 0}; // the index without the bucket module index
buckets[bucket\_index] = scalar\_bucket\_multiplier \* buckets[bucket\_index];
}
template 
\_\_global\_\_ void last\_pass\_kernel(
const P\* final\_buckets,
P\* final\_sums,
unsigned nof\_sums\_per\_batch,
unsigned batch\_size,
unsigned nof\_bms\_per\_batch,
unsigned orig\_c)
{
unsigned tid = (blockIdx.x \* blockDim.x) + threadIdx.x;
if (tid >= nof\_sums\_per\_batch \* batch\_size) return;
unsigned batch\_index = tid / nof\_sums\_per\_batch;
unsigned batch\_tid = tid % nof\_sums\_per\_batch;
unsigned bm\_index = batch\_tid / orig\_c;
unsigned bm\_tid = batch\_tid % orig\_c;
for (unsigned c = orig\_c; c > 1;) {
c = (c + 1) >> 1;
bm\_index <<= 1;
if (bm\_tid >= c) {
bm\_index++;
bm\_tid -= c;
}
}
final\_sums[tid] = final\_buckets[2 \* (batch\_index \* nof\_bms\_per\_batch + bm\_index) + 1];
}
// this kernel computes the final result using the double and add algorithm
// it is done by a single thread
template 
\_\_global\_\_ void final\_accumulation\_kernel(
const P\* final\_sums, P\* final\_results, unsigned nof\_msms, unsigned nof\_results, unsigned c)
{
unsigned tid = (blockIdx.x \* blockDim.x) + threadIdx.x;
if (tid >= nof\_msms) return;
P final\_result = P::zero();
// Note: in some cases accumulation of bm is implemented such that some bms are known to be empty. Therefore
// skipping them.
for (unsigned i = nof\_results; i > 1; i--) {
final\_result = final\_result + final\_sums[i - 1 + tid \* nof\_results]; // add
for (unsigned j = 0; j < c; j++) // double
{
final\_result = final\_result + final\_result;
}
}
final\_results[tid] = final\_result + final\_sums[tid \* nof\_results];
}
template 
static cudaError\_t from\_montgomery\_on\_device(const E\* d\_input, int n, cudaStream\_t stream, E\* d\_output)
{
auto config = default\_vec\_ops\_config();
config.is\_a\_on\_device = true;
config.is\_result\_on\_device = true;
config.is\_async = stream != nullptr;
config.stream = stream;
return montgomery::ConvertMontgomery(d\_input, n, config, d\_output);
}
template 
static cudaError\_t split\_and\_sort\_scalars(
cudaStream\_t stream,
unsigned nof\_scalars,
unsigned\* d\_sorted\_point\_indices,
const S\* d\_scalars,
unsigned nof\_points,
unsigned single\_msm\_size,
unsigned total\_bms\_per\_msm,
unsigned bm\_bitsize,
unsigned c,
int precompute\_factor,
unsigned nof\_bms\_per\_msm,
unsigned input\_indexes\_count,
unsigned total\_nof\_buckets,
unsigned\* d\_single\_bucket\_indices,
unsigned\* d\_bucket\_sizes,
unsigned\* d\_nof\_buckets\_to\_compute,
unsigned\* d\_bucket\_offsets)
{
unsigned\* bucket\_indices = nullptr;
unsigned\* point\_indices = nullptr;
unsigned\* d\_sorted\_bucket\_indices = nullptr;
CHK\_IF\_RETURN(cudaMallocAsync(&bucket\_indices, sizeof(unsigned) \* input\_indexes\_count, stream));
CHK\_IF\_RETURN(cudaMallocAsync(&point\_indices, sizeof(unsigned) \* input\_indexes\_count, stream));
CHK\_IF\_RETURN(cudaMallocAsync(&d\_sorted\_bucket\_indices, sizeof(unsigned) \* input\_indexes\_count, stream));
// split scalars into digits
unsigned NUM\_THREADS = 1 << 10;
unsigned NUM\_BLOCKS = (nof\_scalars + NUM\_THREADS - 1) / NUM\_THREADS;
split\_scalars\_kernel<<>>(
bucket\_indices, point\_indices, d\_scalars, nof\_scalars, nof\_points, single\_msm\_size, total\_bms\_per\_msm,
bm\_bitsize, c, precompute\_factor, nof\_bms\_per\_msm);
// ------------------------------ Sorting routines for scalars start here ----------------------------------
// sort indices - the indices are sorted from smallest to largest in order to group together the points that
// belong to each bucket
unsigned\* sort\_indices\_temp\_storage{};
size\_t sort\_indices\_temp\_storage\_bytes;
// The second to last parameter is the default value supplied explicitly to allow passing the stream
// See https://nvlabs.github.io/cub/structcub\_1\_1\_device\_radix\_sort.html#a65e82152de448c6373ed9563aaf8af7e for
// more info
CHK\_IF\_RETURN(cub::DeviceRadixSort::SortPairs(
sort\_indices\_temp\_storage, sort\_indices\_temp\_storage\_bytes, bucket\_indices, d\_sorted\_bucket\_indices,
point\_indices, d\_sorted\_point\_indices, input\_indexes\_count, 0, sizeof(unsigned) \* 8, stream));
CHK\_IF\_RETURN(cudaMallocAsync(&sort\_indices\_temp\_storage, sort\_indices\_temp\_storage\_bytes, stream));
// The second to last parameter is the default value supplied explicitly to allow passing the stream
// See https://nvlabs.github.io/cub/structcub\_1\_1\_device\_radix\_sort.html#a65e82152de448c6373ed9563aaf8af7e for
// more info
CHK\_IF\_RETURN(cub::DeviceRadixSort::SortPairs(
sort\_indices\_temp\_storage, sort\_indices\_temp\_storage\_bytes, bucket\_indices, d\_sorted\_bucket\_indices,
point\_indices, d\_sorted\_point\_indices, input\_indexes\_count, 0, sizeof(unsigned) \* 8, stream));
CHK\_IF\_RETURN(cudaFreeAsync(sort\_indices\_temp\_storage, stream));
CHK\_IF\_RETURN(cudaFreeAsync(bucket\_indices, stream));
CHK\_IF\_RETURN(cudaFreeAsync(point\_indices, stream));
// find bucket\_sizes
// +1 here and in other places because there still is zero index corresponding to zero bucket at this point
unsigned\* encode\_temp\_storage{};
size\_t encode\_temp\_storage\_bytes = 0;
CHK\_IF\_RETURN(cub::DeviceRunLengthEncode::Encode(
encode\_temp\_storage, encode\_temp\_storage\_bytes, d\_sorted\_bucket\_indices, d\_single\_bucket\_indices,
d\_bucket\_sizes, d\_nof\_buckets\_to\_compute, input\_indexes\_count, stream));
CHK\_IF\_RETURN(cudaMallocAsync(&encode\_temp\_storage, encode\_temp\_storage\_bytes, stream));
CHK\_IF\_RETURN(cub::DeviceRunLengthEncode::Encode(
encode\_temp\_storage, encode\_temp\_storage\_bytes, d\_sorted\_bucket\_indices, d\_single\_bucket\_indices,
d\_bucket\_sizes, d\_nof\_buckets\_to\_compute, input\_indexes\_count, stream));
CHK\_IF\_RETURN(cudaFreeAsync(encode\_temp\_storage, stream));
CHK\_IF\_RETURN(cudaFreeAsync(d\_sorted\_bucket\_indices, stream));
// get offsets - where does each new bucket begin
unsigned\* offsets\_temp\_storage{};
size\_t offsets\_temp\_storage\_bytes = 0;
CHK\_IF\_RETURN(cub::DeviceScan::ExclusiveSum(
offsets\_temp\_storage, offsets\_temp\_storage\_bytes, d\_bucket\_sizes, d\_bucket\_offsets, total\_nof\_buckets + 1,
stream));
CHK\_IF\_RETURN(cudaMallocAsync(&offsets\_temp\_storage, offsets\_temp\_storage\_bytes, stream));
CHK\_IF\_RETURN(cub::DeviceScan::ExclusiveSum(
offsets\_temp\_storage, offsets\_temp\_storage\_bytes, d\_bucket\_sizes, d\_bucket\_offsets, total\_nof\_buckets + 1,
stream));
CHK\_IF\_RETURN(cudaFreeAsync(offsets\_temp\_storage, stream));
return CHK\_LAST();
}
template 
static cudaError\_t upload\_scalars(
cudaStream\_t stream,
bool are\_scalars\_on\_device,
bool are\_scalars\_montgomery\_form,
unsigned nof\_scalars,
const S\* scalars,
S\*& d\_allocated\_scalars,
const S\*& d\_scalars)
{
if (!are\_scalars\_on\_device) {
// copy scalars to gpu
CHK\_IF\_RETURN(cudaMallocAsync(&d\_allocated\_scalars, sizeof(S) \* nof\_scalars, stream));
CHK\_IF\_RETURN(
cudaMemcpyAsync(d\_allocated\_scalars, scalars, sizeof(S) \* nof\_scalars, cudaMemcpyHostToDevice, stream));
if (are\_scalars\_montgomery\_form) {
CHK\_IF\_RETURN(from\_montgomery\_on\_device(d\_allocated\_scalars, nof\_scalars, stream, d\_allocated\_scalars));
}
d\_scalars = d\_allocated\_scalars;
} else { // already on device
if (are\_scalars\_montgomery\_form) {
CHK\_IF\_RETURN(cudaMallocAsync(&d\_allocated\_scalars, sizeof(S) \* nof\_scalars, stream));
CHK\_IF\_RETURN(from\_montgomery\_on\_device(scalars, nof\_scalars, stream, d\_allocated\_scalars));
d\_scalars = d\_allocated\_scalars;
} else {
d\_scalars = scalars;
}
}
return CHK\_LAST();
}
template 
static cudaError\_t upload\_points(
bool init\_points,
bool are\_points\_on\_device,
bool are\_points\_montgomery\_form,
cudaStream\_t& stream\_points,
const A\* points,
A\*\* points\_processed,
unsigned nof\_points,
A\*& d\_allocated\_points,
const A\*& d\_points)
{
if (init\_points) {
if (!are\_points\_on\_device || are\_points\_montgomery\_form) CHK\_IF\_RETURN(cudaStreamCreate(&stream\_points));
if (!are\_points\_on\_device) {
// copy points to gpu
CHK\_IF\_RETURN(cudaMallocAsync(&d\_allocated\_points, sizeof(A) \* nof\_points, stream\_points));
CHK\_IF\_RETURN(
cudaMemcpyAsync(d\_allocated\_points, points, sizeof(A) \* nof\_points, cudaMemcpyHostToDevice, stream\_points));
if (are\_points\_montgomery\_form) {
CHK\_IF\_RETURN(from\_montgomery\_on\_device(d\_allocated\_points, nof\_points, stream\_points, d\_allocated\_points));
}
d\_points = d\_allocated\_points;
} else { // already on device
if (are\_points\_montgomery\_form) {
CHK\_IF\_RETURN(cudaMallocAsync(&d\_allocated\_points, sizeof(A) \* nof\_points, stream\_points));
CHK\_IF\_RETURN(from\_montgomery\_on\_device(points, nof\_points, stream\_points, d\_allocated\_points));
d\_points = d\_allocated\_points;
} else {
d\_points = points;
}
}
} else {
d\_points = \*points\_processed;
}
return CHK\_LAST();
}
template 
static cudaError\_t allocate\_and\_sort\_buckets\_by\_size(
cudaStream\_t stream,
bool init\_buckets,
P\*& buckets, /\*out\*/
P\*\* buckets\_external,
unsigned total\_nof\_buckets,
unsigned nof\_bms\_in\_batch,
unsigned\* d\_nof\_buckets\_to\_compute,
unsigned\* d\_single\_bucket\_indices,
unsigned& h\_nof\_buckets\_to\_compute, /\*out\*/
unsigned\*& sorted\_single\_bucket\_indices, /\*out\*/
unsigned\*& d\_sorted\_bucket\_sizes, /\*out\*/
unsigned\*& d\_sorted\_bucket\_offsets, /\*out\*/
unsigned\* d\_bucket\_sizes,
unsigned\* d\_bucket\_offsets)
{
if (init\_buckets) {
CHK\_IF\_RETURN(cudaMallocAsync(&buckets, sizeof(P) \* (total\_nof\_buckets + nof\_bms\_in\_batch), stream));
// launch the bucket initialization kernel with maximum threads
unsigned NUM\_THREADS = 1 << 10;
unsigned NUM\_BLOCKS = (total\_nof\_buckets + nof\_bms\_in\_batch + NUM\_THREADS - 1) / NUM\_THREADS;
initialize\_buckets\_kernel<<>>(
buckets, total\_nof\_buckets + nof\_bms\_in\_batch);
} else {
buckets = \*buckets\_external;
}
// removing zero bucket, if it exists
unsigned smallest\_bucket\_index;
CHK\_IF\_RETURN(cudaMemcpyAsync(
&smallest\_bucket\_index, d\_single\_bucket\_indices, sizeof(unsigned), cudaMemcpyDeviceToHost, stream));
// maybe zero bucket is empty after all? in this case zero\_bucket\_offset is set to 0
unsigned zero\_bucket\_offset = (smallest\_bucket\_index == 0) ? 1 : 0;
// sort by bucket sizes
CHK\_IF\_RETURN(cudaMemcpyAsync(
&h\_nof\_buckets\_to\_compute, d\_nof\_buckets\_to\_compute, sizeof(unsigned), cudaMemcpyDeviceToHost, stream));
CHK\_IF\_RETURN(cudaFreeAsync(d\_nof\_buckets\_to\_compute, stream));
h\_nof\_buckets\_to\_compute -= zero\_bucket\_offset;
CHK\_IF\_RETURN(cudaMallocAsync(&d\_sorted\_bucket\_sizes, sizeof(unsigned) \* h\_nof\_buckets\_to\_compute, stream));
CHK\_IF\_RETURN(cudaMallocAsync(&d\_sorted\_bucket\_offsets, sizeof(unsigned) \* h\_nof\_buckets\_to\_compute, stream));
unsigned\* sort\_offsets\_temp\_storage{};
size\_t sort\_offsets\_temp\_storage\_bytes = 0;
CHK\_IF\_RETURN(cub::DeviceRadixSort::SortPairsDescending(
sort\_offsets\_temp\_storage, sort\_offsets\_temp\_storage\_bytes, d\_bucket\_sizes + zero\_bucket\_offset,
d\_sorted\_bucket\_sizes, d\_bucket\_offsets + zero\_bucket\_offset, d\_sorted\_bucket\_offsets, h\_nof\_buckets\_to\_compute,
0, sizeof(unsigned) \* 8, stream));
CHK\_IF\_RETURN(cudaMallocAsync(&sort\_offsets\_temp\_storage, sort\_offsets\_temp\_storage\_bytes, stream));
CHK\_IF\_RETURN(cub::DeviceRadixSort::SortPairsDescending(
sort\_offsets\_temp\_storage, sort\_offsets\_temp\_storage\_bytes, d\_bucket\_sizes + zero\_bucket\_offset,
d\_sorted\_bucket\_sizes, d\_bucket\_offsets + zero\_bucket\_offset, d\_sorted\_bucket\_offsets, h\_nof\_buckets\_to\_compute,
0, sizeof(unsigned) \* 8, stream));
CHK\_IF\_RETURN(cudaFreeAsync(sort\_offsets\_temp\_storage, stream));
CHK\_IF\_RETURN(cudaFreeAsync(d\_bucket\_offsets, stream));
CHK\_IF\_RETURN(
cudaMallocAsync(&sorted\_single\_bucket\_indices, sizeof(unsigned) \* h\_nof\_buckets\_to\_compute, stream));
unsigned\* sort\_single\_temp\_storage{};
size\_t sort\_single\_temp\_storage\_bytes = 0;
CHK\_IF\_RETURN(cub::DeviceRadixSort::SortPairsDescending(
sort\_single\_temp\_storage, sort\_single\_temp\_storage\_bytes, d\_bucket\_sizes + zero\_bucket\_offset,
d\_sorted\_bucket\_sizes, d\_single\_bucket\_indices + zero\_bucket\_offset, sorted\_single\_bucket\_indices,
h\_nof\_buckets\_to\_compute, 0, sizeof(unsigned) \* 8, stream));
CHK\_IF\_RETURN(cudaMallocAsync(&sort\_single\_temp\_storage, sort\_single\_temp\_storage\_bytes, stream));
CHK\_IF\_RETURN(cub::DeviceRadixSort::SortPairsDescending(
sort\_single\_temp\_storage, sort\_single\_temp\_storage\_bytes, d\_bucket\_sizes + zero\_bucket\_offset,
d\_sorted\_bucket\_sizes, d\_single\_bucket\_indices + zero\_bucket\_offset, sorted\_single\_bucket\_indices,
h\_nof\_buckets\_to\_compute, 0, sizeof(unsigned) \* 8, stream));
CHK\_IF\_RETURN(cudaFreeAsync(sort\_single\_temp\_storage, stream));
CHK\_IF\_RETURN(cudaFreeAsync(d\_bucket\_sizes, stream));
CHK\_IF\_RETURN(cudaFreeAsync(d\_single\_bucket\_indices, stream));
return CHK\_LAST();
}
template 
static cudaError\_t compute\_nof\_large\_buckets(
unsigned bucket\_th,
cudaStream\_t stream,
unsigned h\_nof\_buckets\_to\_compute,
unsigned\* d\_sorted\_bucket\_sizes,
unsigned& h\_nof\_large\_buckets /\*out\*/)
{
// how large a bucket must be to qualify as a "large bucket"
unsigned\* nof\_large\_buckets;
CHK\_IF\_RETURN(cudaMallocAsync(&nof\_large\_buckets, sizeof(unsigned), stream));
CHK\_IF\_RETURN(cudaMemsetAsync(nof\_large\_buckets, 0, sizeof(unsigned), stream));
unsigned TOTAL\_THREADS = 129000; // TODO: device dependent
unsigned cutoff\_run\_length = max(2, h\_nof\_buckets\_to\_compute / TOTAL\_THREADS);
unsigned cutoff\_nof\_runs = (h\_nof\_buckets\_to\_compute + cutoff\_run\_length - 1) / cutoff\_run\_length;
unsigned NUM\_THREADS = 1 << 5;
unsigned NUM\_BLOCKS = (cutoff\_nof\_runs + NUM\_THREADS - 1) / NUM\_THREADS;
if (h\_nof\_buckets\_to\_compute > 0 && bucket\_th > 0)
find\_cutoff\_kernel~~<<>>(
d\_sorted\_bucket\_sizes, h\_nof\_buckets\_to\_compute, bucket\_th, cutoff\_run\_length, nof\_large\_buckets);
CHK\_IF\_RETURN(
cudaMemcpyAsync(&h\_nof\_large\_buckets, nof\_large\_buckets, sizeof(unsigned), cudaMemcpyDeviceToHost, stream));
CHK\_IF\_RETURN(cudaFreeAsync(nof\_large\_buckets, stream));
return CHK\_LAST();
}
template 
static cudaError\_t large\_bucket\_accumulation(
cudaStream\_t stream,
unsigned h\_nof\_large\_buckets,
unsigned bucket\_th,
cudaStream\_t& stream\_large\_buckets, /\*out\*/
cudaEvent\_t& event\_large\_buckets\_accumulated, /\*out\*/
unsigned\* d\_sorted\_bucket\_sizes,
unsigned average\_bucket\_size,
unsigned\* d\_sorted\_bucket\_offsets,
unsigned\* d\_sorted\_point\_indices,
const A\* d\_points,
unsigned c,
P\* buckets,
unsigned\* sorted\_single\_bucket\_indices,
unsigned nof\_buckets,
unsigned nof\_bms\_per\_msm,
unsigned bm\_bitsize,
bool init\_buckets)
{
CHK\_IF\_RETURN(cudaStreamCreate(&stream\_large\_buckets));
CHK\_IF\_RETURN(cudaEventCreateWithFlags(&event\_large\_buckets\_accumulated, cudaEventDisableTiming));
unsigned\* d\_sorted\_bucket\_sizes\_sum;
CHK\_IF\_RETURN(cudaMallocAsync(
&d\_sorted\_bucket\_sizes\_sum, sizeof(unsigned) \* (h\_nof\_large\_buckets + 1), stream\_large\_buckets));
CHK\_IF\_RETURN(cudaMemsetAsync(d\_sorted\_bucket\_sizes\_sum, 0, sizeof(unsigned), stream\_large\_buckets));
unsigned\* large\_bucket\_temp\_storage{};
size\_t large\_bucket\_temp\_storage\_bytes = 0;
CHK\_IF\_RETURN(cub::DeviceScan::InclusiveSum(
large\_bucket\_temp\_storage, large\_bucket\_temp\_storage\_bytes, d\_sorted\_bucket\_sizes,
d\_sorted\_bucket\_sizes\_sum + 1, h\_nof\_large\_buckets, stream\_large\_buckets));
CHK\_IF\_RETURN(cudaMallocAsync(&large\_bucket\_temp\_storage, large\_bucket\_temp\_storage\_bytes, stream\_large\_buckets));
CHK\_IF\_RETURN(cub::DeviceScan::InclusiveSum(
large\_bucket\_temp\_storage, large\_bucket\_temp\_storage\_bytes, d\_sorted\_bucket\_sizes,
d\_sorted\_bucket\_sizes\_sum + 1, h\_nof\_large\_buckets, stream\_large\_buckets));
CHK\_IF\_RETURN(cudaFreeAsync(large\_bucket\_temp\_storage, stream\_large\_buckets));
unsigned h\_nof\_pts\_in\_large\_buckets;
CHK\_IF\_RETURN(cudaMemcpyAsync(
&h\_nof\_pts\_in\_large\_buckets, d\_sorted\_bucket\_sizes\_sum + h\_nof\_large\_buckets, sizeof(unsigned),
cudaMemcpyDeviceToHost, stream\_large\_buckets));
unsigned h\_largest\_bucket;
CHK\_IF\_RETURN(cudaMemcpyAsync(
&h\_largest\_bucket, d\_sorted\_bucket\_sizes, sizeof(unsigned), cudaMemcpyDeviceToHost, stream\_large\_buckets));
// the number of threads for large buckets has an extra h\_nof\_large\_buckets term to account for bucket sizes
// unevenly divisible by average\_bucket\_size. there are similar corrections elsewhere when accessing large
// buckets
unsigned large\_buckets\_nof\_threads =
(h\_nof\_pts\_in\_large\_buckets + average\_bucket\_size - 1) / average\_bucket\_size + h\_nof\_large\_buckets;
unsigned log\_nof\_large\_buckets = (unsigned)ceil(std::log2(h\_nof\_large\_buckets));
unsigned\* large\_bucket\_indices;
CHK\_IF\_RETURN(cudaMallocAsync(&large\_bucket\_indices, sizeof(unsigned) \* large\_buckets\_nof\_threads, stream));
unsigned NUM\_THREADS = max(1, min(1 << 8, h\_nof\_large\_buckets));
unsigned NUM\_BLOCKS = (h\_nof\_large\_buckets + NUM\_THREADS - 1) / NUM\_THREADS;
initialize\_large\_bucket\_indices

<<>>(
d\_sorted\_bucket\_sizes\_sum, average\_bucket\_size, h\_nof\_large\_buckets, log\_nof\_large\_buckets,
large\_bucket\_indices);
P\* large\_buckets;
CHK\_IF\_RETURN(cudaMallocAsync(&large\_buckets, sizeof(P) \* large\_buckets\_nof\_threads, stream\_large\_buckets));
NUM\_THREADS = max(1, min(1 << 8, large\_buckets\_nof\_threads));
NUM\_BLOCKS = (large\_buckets\_nof\_threads + NUM\_THREADS - 1) / NUM\_THREADS;
accumulate\_large\_buckets\_kernel<<>>(
large\_buckets, d\_sorted\_bucket\_offsets, d\_sorted\_bucket\_sizes, large\_bucket\_indices, d\_sorted\_point\_indices,
d\_points, h\_nof\_large\_buckets, c, average\_bucket\_size, log\_nof\_large\_buckets, large\_buckets\_nof\_threads);
NUM\_THREADS = max(1, min(MAX\_TH, h\_nof\_large\_buckets));
NUM\_BLOCKS = (h\_nof\_large\_buckets + NUM\_THREADS - 1) / NUM\_THREADS;
// normalization is needed to update buckets sizes and offsets due to reduction that already took place
normalize\_kernel<<>>(
d\_sorted\_bucket\_sizes\_sum, average\_bucket\_size, h\_nof\_large\_buckets);
// reduce
for (int s = h\_largest\_bucket; s > 1; s = ((s + 1) >> 1)) {
NUM\_THREADS = max(1, min(MAX\_TH, h\_nof\_large\_buckets));
NUM\_BLOCKS = (h\_nof\_large\_buckets + NUM\_THREADS - 1) / NUM\_THREADS;
normalize\_kernel<<>>(
d\_sorted\_bucket\_sizes, s == h\_largest\_bucket ? average\_bucket\_size : 2, h\_nof\_large\_buckets);
NUM\_THREADS = max(1, min(MAX\_TH, large\_buckets\_nof\_threads));
NUM\_BLOCKS = (large\_buckets\_nof\_threads + NUM\_THREADS - 1) / NUM\_THREADS;
sum\_reduction\_variable\_size\_kernel<<>>(
large\_buckets, d\_sorted\_bucket\_sizes\_sum, d\_sorted\_bucket\_sizes, large\_bucket\_indices,
large\_buckets\_nof\_threads);
}
CHK\_IF\_RETURN(cudaFreeAsync(large\_bucket\_indices, stream\_large\_buckets));
// distribute
NUM\_THREADS = max(1, min(MAX\_TH, h\_nof\_large\_buckets));
NUM\_BLOCKS = (h\_nof\_large\_buckets + NUM\_THREADS - 1) / NUM\_THREADS;
distribute\_large\_buckets\_kernel<<>>(
large\_buckets, buckets, d\_sorted\_bucket\_sizes\_sum, sorted\_single\_bucket\_indices, h\_nof\_large\_buckets,
nof\_buckets + nof\_bms\_per\_msm, c + bm\_bitsize, init\_buckets);
CHK\_IF\_RETURN(cudaFreeAsync(large\_buckets, stream\_large\_buckets));
CHK\_IF\_RETURN(cudaFreeAsync(d\_sorted\_bucket\_sizes\_sum, stream\_large\_buckets));
CHK\_IF\_RETURN(cudaEventRecord(event\_large\_buckets\_accumulated, stream\_large\_buckets));
return CHK\_LAST();
}
template 
static cudaError\_t bucket\_accumulation(
cudaStream\_t stream,
unsigned h\_nof\_buckets\_to\_compute,
unsigned h\_nof\_large\_buckets,
unsigned bucket\_th,
unsigned\* d\_sorted\_bucket\_sizes,
unsigned average\_bucket\_size,
unsigned\* d\_sorted\_bucket\_offsets,
unsigned\* d\_sorted\_point\_indices,
const A\* d\_points,
unsigned c,
P\* buckets,
unsigned\* sorted\_single\_bucket\_indices,
unsigned nof\_buckets,
unsigned nof\_bms\_per\_msm,
unsigned bm\_bitsize,
bool init\_buckets,
cudaStream\_t& stream\_large\_buckets,
cudaEvent\_t& event\_large\_buckets\_accumulated)
{
// ---------------- This is where handling of large buckets happens (if there are any) -------------
if (h\_nof\_large\_buckets > 0 && bucket\_th > 0) {
auto err = large\_bucket\_accumulation(
stream, h\_nof\_large\_buckets, bucket\_th, stream\_large\_buckets /\*out\*/, event\_large\_buckets\_accumulated /\*out\*/,
d\_sorted\_bucket\_sizes, average\_bucket\_size, d\_sorted\_bucket\_offsets, d\_sorted\_point\_indices, d\_points, c,
buckets, sorted\_single\_bucket\_indices, nof\_buckets, nof\_bms\_per\_msm, bm\_bitsize, init\_buckets);
CHK\_IF\_RETURN(err);
}
// ------------------------- Accumulation of (non-large) buckets ---------------------------------
if (h\_nof\_buckets\_to\_compute > h\_nof\_large\_buckets) {
unsigned NUM\_THREADS = 1 << 8;
unsigned NUM\_BLOCKS = (h\_nof\_buckets\_to\_compute - h\_nof\_large\_buckets + NUM\_THREADS - 1) / NUM\_THREADS;
// launch the accumulation kernel with maximum threads
accumulate\_buckets\_kernel<<>>(
buckets, d\_sorted\_bucket\_offsets + h\_nof\_large\_buckets, d\_sorted\_bucket\_sizes + h\_nof\_large\_buckets,
sorted\_single\_bucket\_indices + h\_nof\_large\_buckets, d\_sorted\_point\_indices, d\_points,
nof\_buckets + nof\_bms\_per\_msm, h\_nof\_buckets\_to\_compute - h\_nof\_large\_buckets, c + bm\_bitsize, c,
init\_buckets);
}
CHK\_IF\_RETURN(cudaFreeAsync(d\_sorted\_point\_indices, stream));
CHK\_IF\_RETURN(cudaFreeAsync(d\_sorted\_bucket\_sizes, stream));
CHK\_IF\_RETURN(cudaFreeAsync(d\_sorted\_bucket\_offsets, stream));
CHK\_IF\_RETURN(cudaFreeAsync(sorted\_single\_bucket\_indices, stream));
if (h\_nof\_large\_buckets > 0 && bucket\_th > 0) {
// all the large buckets need to be accumulated before the final summation
CHK\_IF\_RETURN(cudaStreamWaitEvent(stream, event\_large\_buckets\_accumulated));
}
return CHK\_LAST();
}
template 
static cudaError\_t bucket\_reduction(
cudaStream\_t stream,
unsigned nof\_bms\_per\_msm,
unsigned c,
unsigned nof\_bms\_in\_batch,
P\*& buckets, /\*out\*/
unsigned nof\_buckets,
unsigned batch\_size,
unsigned bitsize,
bool are\_results\_on\_device,
P\* final\_result,
P\* d\_allocated\_final\_result,
bool \_is\_big\_triangle)
{
unsigned nof\_final\_results\_per\_msm =
nof\_bms\_per\_msm; // for big-triangle accumluation this is the number of bucket modules
P\* final\_results;
if (\_is\_big\_triangle || c == 1) {
CHK\_IF\_RETURN(cudaMallocAsync(&final\_results, sizeof(P) \* nof\_bms\_in\_batch, stream));
// launch the bucket module sum kernel - a thread for each bucket module
unsigned NUM\_THREADS = 32;
unsigned NUM\_BLOCKS = (nof\_bms\_in\_batch + NUM\_THREADS - 1) / NUM\_THREADS;
big\_triangle\_sum\_kernel<<>>(buckets, final\_results, nof\_bms\_in\_batch, c);
} else {
// the iterative reduction algorithm works with 2 types of reduction that can run on parallel streams
cudaStream\_t stream\_reduction;
cudaEvent\_t event\_finished\_reduction;
CHK\_IF\_RETURN(cudaStreamCreate(&stream\_reduction));
CHK\_IF\_RETURN(cudaEventCreateWithFlags(&event\_finished\_reduction, cudaEventDisableTiming));
unsigned source\_bits\_count = c;
unsigned source\_windows\_count = nof\_bms\_per\_msm;
unsigned source\_buckets\_count = nof\_buckets + nof\_bms\_per\_msm; // nof buckets per msm including zero buckets
unsigned target\_windows\_count;
P\* source\_buckets = buckets;
buckets = nullptr;
P\* target\_buckets;
P\* temp\_buckets1;
P\* temp\_buckets2;
for (unsigned i = 0;; i++) {
const unsigned target\_bits\_count = (source\_bits\_count + 1) >> 1; // half the bits rounded up
target\_windows\_count = source\_windows\_count << 1; // twice the number of bms
const unsigned target\_buckets\_count = target\_windows\_count << target\_bits\_count; // new\_bms\*2^new\_c
CHK\_IF\_RETURN(cudaMallocAsync(&target\_buckets, sizeof(P) \* target\_buckets\_count \* batch\_size, stream));
CHK\_IF\_RETURN(cudaMallocAsync(
&temp\_buckets1, sizeof(P) \* source\_buckets\_count \* batch\_size,
stream)); // for type1 reduction (strided, bottom window - evens)
CHK\_IF\_RETURN(cudaMallocAsync(
&temp\_buckets2, sizeof(P) \* source\_buckets\_count \* batch\_size,
stream)); // for type2 reduction (serial, top window - odds)
initialize\_buckets\_kernel<<<(target\_buckets\_count \* batch\_size + 255) / 256, 256>>>(
target\_buckets, target\_buckets\_count \* batch\_size); // initialization is needed for the odd c case
for (unsigned j = 0; j < target\_bits\_count; j++) {
const bool is\_first\_iter = (j == 0);
const bool is\_second\_iter = (j == 1);
const bool is\_last\_iter = (j == target\_bits\_count - 1);
const bool is\_odd\_c = source\_bits\_count & 1;
unsigned nof\_threads =
(((source\_windows\_count << target\_bits\_count) - source\_windows\_count) << (target\_bits\_count - 1 - j)) \*
batch\_size; // nof sections to reduce (minus the section that goes to zero buckets) shifted by nof threads
// per section
unsigned NUM\_THREADS = max(1, min(MAX\_TH, nof\_threads));
unsigned NUM\_BLOCKS = (nof\_threads + NUM\_THREADS - 1) / NUM\_THREADS;
if (!is\_odd\_c || !is\_first\_iter) { // skip if c is odd and it's the first iteration
single\_stage\_multi\_reduction\_kernel<<>>(
is\_first\_iter || (is\_second\_iter && is\_odd\_c) ? source\_buckets : temp\_buckets1,
is\_last\_iter ? target\_buckets : temp\_buckets1, 1 << source\_bits\_count,
1 << (source\_bits\_count - j + (is\_odd\_c ? 1 : 0)), is\_last\_iter ? 1 << target\_bits\_count : 0,
1 << target\_bits\_count, 0 /\*=write\_phase\*/, (1 << target\_bits\_count) - 1, nof\_threads);
}
nof\_threads = (((source\_windows\_count << (source\_bits\_count - target\_bits\_count)) - source\_windows\_count)
<< (target\_bits\_count - 1 - j)) \*
batch\_size; // nof sections to reduce (minus the section that goes to zero buckets) shifted by
// nof threads per section
NUM\_THREADS = max(1, min(MAX\_TH, nof\_threads));
NUM\_BLOCKS = (nof\_threads + NUM\_THREADS - 1) / NUM\_THREADS;
single\_stage\_multi\_reduction\_kernel<<>>(
is\_first\_iter ? source\_buckets : temp\_buckets2, is\_last\_iter ? target\_buckets : temp\_buckets2,
1 << target\_bits\_count, 1 << (target\_bits\_count - j), is\_last\_iter ? 1 << target\_bits\_count : 0,
1 << (target\_bits\_count - (is\_odd\_c ? 1 : 0)), 1 /\*=write\_phase\*/,
(1 << (target\_bits\_count - (is\_odd\_c ? 1 : 0))) - 1, nof\_threads);
}
CHK\_IF\_RETURN(cudaEventRecord(event\_finished\_reduction, stream\_reduction));
CHK\_IF\_RETURN(
cudaStreamWaitEvent(stream, event\_finished\_reduction)); // sync streams after every write to target\_buckets
if (target\_bits\_count == 1) {
// Note: the reduction ends up with 'target\_windows\_count' windows per batch element. Some are guaranteed
// to be empty when target\_windows\_count>bitsize. for example consider bitsize=253 and c=2. The reduction
// ends with 254 bms but the most significant one is guaranteed to be zero since the scalars are 253b.
// precomputation and odd c can cause additional empty windows.
nof\_final\_results\_per\_msm = min(c \* nof\_bms\_per\_msm, bitsize);
nof\_bms\_per\_msm = target\_windows\_count;
unsigned total\_nof\_final\_results = nof\_final\_results\_per\_msm \* batch\_size;
CHK\_IF\_RETURN(cudaMallocAsync(&final\_results, sizeof(P) \* total\_nof\_final\_results, stream));
unsigned NUM\_THREADS = 32;
unsigned NUM\_BLOCKS = (total\_nof\_final\_results + NUM\_THREADS - 1) / NUM\_THREADS;
last\_pass\_kernel<<>>(
target\_buckets, final\_results, nof\_final\_results\_per\_msm, batch\_size, nof\_bms\_per\_msm, c);
c = 1;
CHK\_IF\_RETURN(cudaFreeAsync(source\_buckets, stream));
CHK\_IF\_RETURN(cudaFreeAsync(target\_buckets, stream));
CHK\_IF\_RETURN(cudaFreeAsync(temp\_buckets1, stream));
CHK\_IF\_RETURN(cudaFreeAsync(temp\_buckets2, stream));
CHK\_IF\_RETURN(cudaStreamDestroy(stream\_reduction));
break;
}
CHK\_IF\_RETURN(cudaFreeAsync(source\_buckets, stream));
CHK\_IF\_RETURN(cudaFreeAsync(temp\_buckets1, stream));
CHK\_IF\_RETURN(cudaFreeAsync(temp\_buckets2, stream));
source\_buckets = target\_buckets;
target\_buckets = nullptr;
temp\_buckets1 = nullptr;
temp\_buckets2 = nullptr;
source\_bits\_count = target\_bits\_count;
source\_windows\_count = target\_windows\_count;
source\_buckets\_count = target\_buckets\_count;
}
}
// ------- This is the final stage where bucket modules/window sums get added up with appropriate weights
// -------
unsigned NUM\_THREADS = 32;
unsigned NUM\_BLOCKS = (batch\_size + NUM\_THREADS - 1) / NUM\_THREADS;
// launch the double and add kernel, a single thread per batch element
final\_accumulation\_kernel<<>>(
final\_results, are\_results\_on\_device ? final\_result : d\_allocated\_final\_result, batch\_size,
nof\_final\_results\_per\_msm, c);
CHK\_IF\_RETURN(cudaFreeAsync(final\_results, stream));
return CHK\_LAST();
}
// this function computes msm using the bucket method
template 
static cudaError\_t bucket\_method\_msm(
unsigned bitsize,
unsigned c,
const S\* scalars,
const A\* points,
A\*\* points\_processed,
bool return\_processed\_points,
bool init\_points,
unsigned batch\_size, // number of MSMs to compute
unsigned single\_msm\_size, // number of elements per MSM (a.k.a N)
bool are\_points\_shared\_in\_batch, // number of EC points in 'points' array. Must be either (1) single\_msm\_size if
// MSMs are sharing points or (2) single\_msm\_size\*batch\_size otherwise
P\*\* buckets\_external,
bool init\_buckets,
bool return\_buckets,
P\* final\_result,
bool are\_scalars\_on\_device,
bool are\_scalars\_montgomery\_form,
bool are\_points\_on\_device,
bool are\_points\_montgomery\_form,
bool are\_results\_on\_device,
bool \_is\_big\_triangle,
int large\_bucket\_factor,
int precompute\_factor,
bool is\_async,
cudaStream\_t stream)
{
CHK\_INIT\_IF\_RETURN();
const unsigned nof\_scalars = batch\_size \* single\_msm\_size; // assuming scalars not shared between batch elements
const unsigned nof\_points = (are\_points\_shared\_in\_batch ? single\_msm\_size : nof\_scalars) \* precompute\_factor;
const unsigned total\_bms\_per\_msm = (bitsize + c - 1) / c;
const unsigned nof\_bms\_per\_msm = (total\_bms\_per\_msm - 1) / precompute\_factor + 1;
const unsigned input\_indexes\_count = nof\_scalars \* total\_bms\_per\_msm;
const unsigned bm\_bitsize = (unsigned)ceil(std::log2(nof\_bms\_per\_msm));
const unsigned average\_bucket\_size = (single\_msm\_size / (1 << c)) \* precompute\_factor;
const unsigned bucket\_th = large\_bucket\_factor \* average\_bucket\_size;
// minus nof\_bms\_per\_msm because zero bucket is not included in each bucket module
const unsigned nof\_buckets = (nof\_bms\_per\_msm << c) - nof\_bms\_per\_msm;
const unsigned total\_nof\_buckets = nof\_buckets \* batch\_size;
const unsigned nof\_bms\_in\_batch = nof\_bms\_per\_msm \* batch\_size;
unsigned h\_nof\_large\_buckets = 0; // computed later
unsigned\* d\_sorted\_point\_indices = nullptr;
unsigned\* d\_single\_bucket\_indices = nullptr;
unsigned\* d\_bucket\_sizes = nullptr;
unsigned\* d\_nof\_buckets\_to\_compute = nullptr;
unsigned\* d\_bucket\_offsets = nullptr;
unsigned\* d\_sorted\_bucket\_sizes = nullptr;
unsigned\* d\_sorted\_bucket\_offsets = nullptr;
cudaStream\_t stream\_points = nullptr;
A\* d\_allocated\_points = nullptr;
const A\* d\_points = nullptr;
S\* d\_allocated\_scalars = nullptr;
const S\* d\_scalars = nullptr;
cudaEvent\_t event\_points\_uploaded = nullptr;
P\* buckets = nullptr;
unsigned h\_nof\_buckets\_to\_compute = 0;
unsigned\* sorted\_single\_bucket\_indices = nullptr;
cudaStream\_t stream\_large\_buckets = nullptr;
cudaEvent\_t event\_large\_buckets\_accumulated = nullptr;
// (1) Copy scalars to GPU if on host and convert from montgomery if need to
CHK\_IF\_RETURN((upload\_scalars~~(
stream, are\_scalars\_on\_device, are\_scalars\_montgomery\_form, nof\_scalars, scalars, d\_allocated\_scalars,
d\_scalars)));
// (2) upload points to device in parallel to scalar sorting (using another stream)
CHK\_IF\_RETURN((upload\_points(
init\_points, are\_points\_on\_device, are\_points\_montgomery\_form, stream\_points /\*out\*/, points, points\_processed,
nof\_points, d\_allocated\_points /\*out\*/, d\_points /\*out\*/)));
// (3) Split and sort scalars
CHK\_IF\_RETURN(cudaMallocAsync(&d\_sorted\_point\_indices, sizeof(unsigned) \* input\_indexes\_count, stream));
CHK\_IF\_RETURN(cudaMallocAsync(&d\_bucket\_offsets, sizeof(unsigned) \* (total\_nof\_buckets + 1), stream));
CHK\_IF\_RETURN(cudaMallocAsync(&d\_single\_bucket\_indices, sizeof(unsigned) \* (total\_nof\_buckets + 1), stream));
CHK\_IF\_RETURN(cudaMallocAsync(&d\_bucket\_sizes, sizeof(unsigned) \* (total\_nof\_buckets + 1), stream));
CHK\_IF\_RETURN(cudaMallocAsync(&d\_nof\_buckets\_to\_compute, sizeof(unsigned), stream));
CHK\_IF\_RETURN(split\_and\_sort\_scalars(
stream, nof\_scalars, d\_sorted\_point\_indices, d\_scalars, nof\_points, single\_msm\_size, total\_bms\_per\_msm,
bm\_bitsize, c, precompute\_factor, nof\_bms\_per\_msm, input\_indexes\_count, total\_nof\_buckets,
d\_single\_bucket\_indices, d\_bucket\_sizes, d\_nof\_buckets\_to\_compute, d\_bucket\_offsets));
if (return\_processed\_points)
\*points\_processed = d\_allocated\_points; // return pointer to processed points for the next msm in batch
if (stream\_points) {
CHK\_IF\_RETURN(cudaEventCreateWithFlags(&event\_points\_uploaded, cudaEventDisableTiming));
CHK\_IF\_RETURN(cudaEventRecord(event\_points\_uploaded, stream\_points));
}
// (4) Allocate and sort buckets by size
CHK\_IF\_RETURN((allocate\_and\_sort\_buckets\_by\_size

(
stream, init\_buckets, buckets /\*out\*/, buckets\_external, total\_nof\_buckets, nof\_bms\_in\_batch,
d\_nof\_buckets\_to\_compute, d\_single\_bucket\_indices, h\_nof\_buckets\_to\_compute /\*out\*/,
sorted\_single\_bucket\_indices /\*out\*/, d\_sorted\_bucket\_sizes /\*out\*/, d\_sorted\_bucket\_offsets /\*out\*/,
d\_bucket\_sizes, d\_bucket\_offsets)));
// (5) Accumulate buckets
CHK\_IF\_RETURN((compute\_nof\_large\_buckets~~(
bucket\_th, stream, h\_nof\_buckets\_to\_compute, d\_sorted\_bucket\_sizes, h\_nof\_large\_buckets /\*out\*/)));
// by this point, points need to be on device. Therefore a wait event is pushed to the stream
if (stream\_points) { CHK\_IF\_RETURN(cudaStreamWaitEvent(stream, event\_points\_uploaded)); }
CHK\_IF\_RETURN((bucket\_accumulation(
stream, h\_nof\_buckets\_to\_compute, h\_nof\_large\_buckets, bucket\_th, d\_sorted\_bucket\_sizes, average\_bucket\_size,
d\_sorted\_bucket\_offsets, d\_sorted\_point\_indices, d\_points, c, buckets, sorted\_single\_bucket\_indices,
nof\_buckets, nof\_bms\_per\_msm, bm\_bitsize, init\_buckets, stream\_large\_buckets,
event\_large\_buckets\_accumulated)));
if (return\_buckets) {
\*buckets\_external = buckets; // stopping after accumulation when using multiple chunks
if (d\_allocated\_scalars) CHK\_IF\_RETURN(cudaFreeAsync(d\_allocated\_scalars, stream));
if (d\_allocated\_points && !return\_processed\_points) CHK\_IF\_RETURN(cudaFreeAsync(d\_allocated\_points, stream));
return CHK\_LAST();
}
P\* d\_allocated\_final\_result = nullptr;
if (!are\_results\_on\_device)
CHK\_IF\_RETURN(cudaMallocAsync(&d\_allocated\_final\_result, sizeof(P) \* batch\_size, stream));
// (6) Bucket Reduction - sum each bucket module then finally sum all buckets module to final sum
CHK\_IF\_RETURN((bucket\_reduction(
stream, nof\_bms\_per\_msm, c, nof\_bms\_in\_batch, buckets /\*out\*/, nof\_buckets, batch\_size, bitsize,
are\_results\_on\_device, final\_result, d\_allocated\_final\_result, \_is\_big\_triangle)));
if (!are\_results\_on\_device)
CHK\_IF\_RETURN(cudaMemcpyAsync(
final\_result, d\_allocated\_final\_result, sizeof(P) \* batch\_size, cudaMemcpyDeviceToHost, stream));
// (7) cleaunp - free memory and release streams+events (possibly async)
if (d\_allocated\_scalars) CHK\_IF\_RETURN(cudaFreeAsync(d\_allocated\_scalars, stream));
if (d\_allocated\_points && !return\_processed\_points) CHK\_IF\_RETURN(cudaFreeAsync(d\_allocated\_points, stream));
if (d\_allocated\_final\_result) CHK\_IF\_RETURN(cudaFreeAsync(d\_allocated\_final\_result, stream));
CHK\_IF\_RETURN(cudaFreeAsync(buckets, stream));
auto cleanup = [stream\_points, event\_points\_uploaded, stream\_large\_buckets, event\_large\_buckets\_accumulated]() {
// points upload stream and event
if (stream\_points) {
cudaStreamSynchronize(stream\_points);
cudaEventDestroy(event\_points\_uploaded);
cudaStreamDestroy(stream\_points);
}
// large bucket accumulation stream and event
if (stream\_large\_buckets) {
cudaStreamSynchronize(stream\_large\_buckets);
cudaEventDestroy(event\_large\_buckets\_accumulated);
cudaStreamDestroy(stream\_large\_buckets);
}
};
if (!is\_async) {
CHK\_IF\_RETURN(cudaStreamSynchronize(stream));
cleanup();
} else {
// async case: use a detached thread to wait for the stream and release the resources then
if (stream\_points || stream\_large\_buckets) { std::thread(cleanup).detach(); }
}
return CHK\_LAST();
}
} // namespace
template 
static void compute\_required\_memory(
const MSMConfig& config,
int msm\_size,
unsigned c,
int batch\_size,
unsigned bitsize,
unsigned& nof\_bms\_after\_precomputation,
unsigned long& scalars\_mem,
unsigned long& indices\_mem,
unsigned long& points\_mem,
unsigned long& buckets\_mem,
double& reduced\_gpu\_memory)
{
unsigned nof\_bms = (bitsize + c - 1) / c;
nof\_bms\_after\_precomputation = (nof\_bms + config.precompute\_factor - 1) / config.precompute\_factor;
scalars\_mem = sizeof(S) \* msm\_size \* batch\_size;
indices\_mem = 7 \* sizeof(unsigned) \* msm\_size \* batch\_size \*
nof\_bms; // factor 7 as an estimation for the sorting extra memory. can be reduced by
// sorting separately or changing sort algorithm
points\_mem = sizeof(A) \* msm\_size \* config.precompute\_factor \* (config.are\_points\_shared\_in\_batch ? 1 : batch\_size);
buckets\_mem = 4 \* sizeof(P) \* (1 << c) \* batch\_size \*
nof\_bms\_after\_precomputation; // factor 3 for the extra memory in the iterative reduction algorithm.
// +1 for large buckets. can be reduced with some optimizations.
unsigned long total\_memory;
unsigned long gpu\_memory;
cudaMemGetInfo(&gpu\_memory, &total\_memory); // TODO - handle cuda error
if (config.are\_points\_on\_device)
gpu\_memory += points\_mem; // if points are already allocated we don't need to take them into account
if (config.are\_scalars\_on\_device)
gpu\_memory += scalars\_mem; // if scalars are already allocated we don't need to take them into account
reduced\_gpu\_memory =
0.7 \* static\_cast(gpu\_memory); // 0.7 is an arbitrary value in order to have some spare memory
}
template 
static int get\_min\_nof\_chunks(
const MSMConfig& config, int msm\_size, unsigned c, unsigned bitsize, unsigned& fixed\_c, int& fixed\_batch\_size)
{
unsigned long scalars\_mem;
unsigned long indices\_mem;
unsigned long points\_mem;
unsigned long buckets\_mem;
unsigned nof\_bms\_after\_precomputation;
double reduced\_gpu\_memory;
compute\_required\_memory(
config, msm\_size, c, config.batch\_size, bitsize, nof\_bms\_after\_precomputation, scalars\_mem, indices\_mem,
points\_mem, buckets\_mem, reduced\_gpu\_memory);
double lower\_bound;
unsigned min\_nof\_chunks;
if (config.batch\_size > 1) { // batched msm
if (config.are\_points\_shared\_in\_batch) { // shared points batch
if (2 \* points\_mem > reduced\_gpu\_memory) {
min\_nof\_chunks = 0;
} else {
lower\_bound = static\_cast(2 \* (scalars\_mem + indices\_mem) + buckets\_mem) /
static\_cast(reduced\_gpu\_memory - static\_cast(2 \* points\_mem));
min\_nof\_chunks = static\_cast(lower\_bound) + 1;
}
} else { // different points batch
lower\_bound =
static\_cast(2 \* (scalars\_mem + points\_mem + indices\_mem) + buckets\_mem) / reduced\_gpu\_memory;
min\_nof\_chunks = static\_cast(lower\_bound) + 1;
}
if (min\_nof\_chunks > config.batch\_size || min\_nof\_chunks == 0) {
ICICLE\_LOG\_WARNING
<< "msm: msm size (including precomputation) is too large for batching, running sequential msms instead";
fixed\_batch\_size = 1;
compute\_required\_memory(
config, msm\_size, c, fixed\_batch\_size, bitsize, nof\_bms\_after\_precomputation, scalars\_mem, indices\_mem,
points\_mem, buckets\_mem, reduced\_gpu\_memory);
}
}
if (config.batch\_size < 2 || fixed\_batch\_size == 1) { // single msm
if (buckets\_mem > reduced\_gpu\_memory) {
ICICLE\_ASSERT(config.precompute\_factor <= 1)
<< "msm: optimal or given c = " << c
<< " was too large for memory, please use a lower c value for precomputation and try again"; // TODO - handle
// this case
// better
fixed\_c = c;
while (buckets\_mem > reduced\_gpu\_memory) {
--fixed\_c;
compute\_required\_memory(
config, msm\_size, fixed\_c, 1, bitsize, nof\_bms\_after\_precomputation, scalars\_mem, indices\_mem, points\_mem,
buckets\_mem, reduced\_gpu\_memory);
}
ICICLE\_LOG\_WARNING << "msm: optimal or given c = " << c << " was too large for memory, using c = " << fixed\_c
<< " instead";
}
lower\_bound = static\_cast(2 \* (scalars\_mem + points\_mem + indices\_mem)) /
static\_cast(reduced\_gpu\_memory - static\_cast(buckets\_mem));
min\_nof\_chunks = static\_cast(lower\_bound) + 1;
}
return min\_nof\_chunks;
}
template 
static int get\_optimal\_nof\_chunks(
const MSMConfig& config, int msm\_size, unsigned c, unsigned bitsize, unsigned& fixed\_c, int& fixed\_batch\_size)
{
int min\_nof\_chunks = get\_min\_nof\_chunks(config, msm\_size, c, bitsize, fixed\_c, fixed\_batch\_size);
if (min\_nof\_chunks > 1) return min\_nof\_chunks; // if the data is too large and a single chunk can't fit in memory
if (!config.are\_points\_on\_device || !config.are\_scalars\_on\_device)
return 4; // if it does fit in memory we still want to parallelize compute and data transfer
return 1; // if all data is already on device
}
/\*
The following function enables splitting msm computation into chunks.
It only makes sense to use chunks when either the points or the scalars are on host.
They would be on the host when the msm size is too large for the GPU or simply if they haven't been copied.
The chunking method enables parallelization of data transfer and computation - we copy the next chunk whilst the
previous chunk is being computed. This is done using streams. We distinguish between three modes of operation:
1. Single large msm - the msm itself is split into chunks and they are accumulated into the same buckets one after the
other. Finally the buckets are reduced and the results is returned.
2. Large batch of regular msms with different points for each msm - no msm is split, instead the batch is split into
smaller batches, so each chunks is composed of multiple msms.
3. Large batch of regular msms that share the points - same as mode 2 but we only need to copy the points once. Only
the first chunk copies the points (and converts from montgomery form if needed) and returns a pointer to the processed
points which is used by the next chunks. In all cases the first chunk uses the internal copying mechanism of
bucket\_method msm that enables copying the points while processing the scalars. The other chunks are copied
externally.
\*/
template 
static cudaError\_t multi\_chunked\_msm(
const S\* scalars,
const A\* points,
int msm\_size,
int nof\_chunks,
const MSMConfig& config,
P\* results,
unsigned bitsize,
unsigned c,
int batch\_size,
cudaStream\_t stream)
{
const bool \_is\_big\_triangle = is\_big\_triangle(config.ext);
const int \_large\_bucket\_factor = get\_large\_bucket\_factor(config.ext);
bool internal\_are\_scalars\_on\_device = config.are\_scalars\_on\_device;
bool internal\_are\_points\_on\_device = config.are\_points\_on\_device;
int total\_size = batch\_size \* msm\_size;
bool multi\_batch\_mode = batch\_size > 1;
bool batch\_same\_points = multi\_batch\_mode && config.are\_points\_shared\_in\_batch;
bool process\_batch\_same\_points =
batch\_same\_points && (config.are\_points\_montgomery\_form || !config.are\_points\_on\_device);
int scalars\_chunk\_size = (total\_size + nof\_chunks - 1) / nof\_chunks;
if (multi\_batch\_mode) scalars\_chunk\_size = ((batch\_size + nof\_chunks - 1) / nof\_chunks) \* msm\_size;
int points\_chunk\_size = scalars\_chunk\_size \* config.precompute\_factor;
int batch\_chunk\_size = max(1, scalars\_chunk\_size / msm\_size);
A\* points\_d;
A\* points\_processed;
S\* scalars\_d;
P\* buckets\_d;
if ((!config.are\_scalars\_on\_device) && nof\_chunks > 1) {
CHK\_IF\_RETURN(cudaMallocAsync(&scalars\_d, sizeof(S) \* scalars\_chunk\_size \* 2, stream));
}
if (((!config.are\_points\_on\_device)) && nof\_chunks > 1) {
CHK\_IF\_RETURN(cudaMallocAsync(&points\_d, sizeof(A) \* points\_chunk\_size \* 2, stream));
}
int sub\_scalars\_size = scalars\_chunk\_size;
int sub\_points\_size = sub\_scalars\_size \* config.precompute\_factor;
// create streams
cudaStream\_t stream1, stream2;
CHK\_IF\_RETURN(cudaStreamCreate(&stream1));
CHK\_IF\_RETURN(cudaStreamCreate(&stream2));
// create events
cudaEvent\_t finish\_allocations, finish\_compute, finish\_chunked1, finish\_chunked2;
CHK\_IF\_RETURN(cudaEventCreateWithFlags(&finish\_allocations, cudaEventDisableTiming));
CHK\_IF\_RETURN(cudaEventCreateWithFlags(&finish\_compute, cudaEventDisableTiming));
CHK\_IF\_RETURN(cudaEventCreateWithFlags(&finish\_chunked1, cudaEventDisableTiming));
CHK\_IF\_RETURN(cudaEventCreateWithFlags(&finish\_chunked2, cudaEventDisableTiming));
// sync with main stream
CHK\_IF\_RETURN(cudaEventRecord(finish\_allocations, stream));
CHK\_IF\_RETURN(cudaStreamWaitEvent(stream1, finish\_allocations));
CHK\_IF\_RETURN(cudaStreamWaitEvent(stream2, finish\_allocations));
for (int i = 0; i < nof\_chunks; i++) {
const bool is\_last\_iter = total\_size <= scalars\_chunk\_size \* (i + 1);
const int sub\_batch\_size = multi\_batch\_mode ? sub\_scalars\_size / msm\_size : batch\_size;
const int sub\_msm\_size = multi\_batch\_mode ? msm\_size : sub\_scalars\_size;
const bool init\_buckets = (i == 0) || multi\_batch\_mode;
const bool return\_buckets = (!is\_last\_iter) && (!multi\_batch\_mode);
if (i) {
internal\_are\_scalars\_on\_device = true;
internal\_are\_points\_on\_device = true;
}
bool internal\_are\_points\_montgomery\_form =
(batch\_same\_points && i) || config.precompute\_factor > 1
? false
: config.are\_points\_montgomery\_form; // if points were already converted by first chunk or by precomputation
// function then they are no longer in montgomery form
bool return\_points\_poiter = (i == 0) && process\_batch\_same\_points;
bool init\_points = (i == 0) || !process\_batch\_same\_points;
// switch streams
cudaStream\_t compute\_stream = i % 2 ? stream2 : stream1;
cudaStream\_t transfer\_stream = i % 2 ? stream1 : stream2;
if (i) CHK\_IF\_RETURN(cudaStreamWaitEvent(compute\_stream, finish\_compute)); // wait for previous compute
CHK\_IF\_RETURN(bucket\_method\_msm(
bitsize, c,
(internal\_are\_scalars\_on\_device && !config.are\_scalars\_on\_device ? scalars\_d : scalars) +
(config.are\_scalars\_on\_device ? i : i % 2) \*
scalars\_chunk\_size, // use scalars\_d from second round if scalars are on host, o.w. use scalars, which are
// on device
(i == 0 || (config.are\_points\_on\_device && !process\_batch\_same\_points) ? points
: process\_batch\_same\_points ? points\_processed
: points\_d) +
(batch\_same\_points ? 0
: (config.are\_points\_on\_device ? i : i % 2) \*
points\_chunk\_size), // use points\_d from second round if points are on host, use
// points\_processed from second round in same points if the points
// were processed, o.w. use points, which are on device
&points\_processed, return\_points\_poiter, init\_points, sub\_batch\_size, sub\_msm\_size,
config.are\_points\_shared\_in\_batch, &buckets\_d, init\_buckets, return\_buckets,
results + (multi\_batch\_mode ? i \* batch\_chunk\_size : 0), internal\_are\_scalars\_on\_device,
config.are\_scalars\_montgomery\_form, internal\_are\_points\_on\_device, internal\_are\_points\_montgomery\_form,
config.are\_results\_on\_device, \_is\_big\_triangle, \_large\_bucket\_factor, config.precompute\_factor, true,
compute\_stream));
CHK\_IF\_RETURN(cudaEventRecord(finish\_compute, compute\_stream));
if (is\_last\_iter) break;
// copy for next iteration
sub\_scalars\_size = min(total\_size - scalars\_chunk\_size \* (i + 1), scalars\_chunk\_size);
if (sub\_scalars\_size <= 0) break; // if last chunk size is 0 - break;
sub\_points\_size = sub\_scalars\_size \* config.precompute\_factor;
if (!config.are\_scalars\_on\_device)
CHK\_IF\_RETURN(cudaMemcpyAsync(
scalars\_d + ((i + 1) % 2) \* scalars\_chunk\_size, scalars + (i + 1) \* scalars\_chunk\_size,
sizeof(S) \* sub\_scalars\_size, cudaMemcpyHostToDevice, transfer\_stream)); // scalars on host
if (!config.are\_points\_on\_device && !batch\_same\_points)
CHK\_IF\_RETURN(cudaMemcpyAsync(
points\_d + ((i + 1) % 2) \* points\_chunk\_size, points + (i + 1) \* points\_chunk\_size,
sizeof(A) \* sub\_points\_size, cudaMemcpyHostToDevice, transfer\_stream)); // points on host
}
// sync with main stream
CHK\_IF\_RETURN(cudaEventRecord(finish\_chunked1, stream1));
CHK\_IF\_RETURN(cudaEventRecord(finish\_chunked2, stream2));
CHK\_IF\_RETURN(cudaStreamWaitEvent(stream, finish\_chunked1));
CHK\_IF\_RETURN(cudaStreamWaitEvent(stream, finish\_chunked2));
if ((!config.are\_scalars\_on\_device) && nof\_chunks > 1) CHK\_IF\_RETURN(cudaFreeAsync(scalars\_d, stream));
if ((!config.are\_points\_on\_device) && nof\_chunks > 1) CHK\_IF\_RETURN(cudaFreeAsync(points\_d, stream));
if (process\_batch\_same\_points) CHK\_IF\_RETURN(cudaFreeAsync(points\_processed, stream));
auto cleanup = [stream1, stream2, finish\_allocations, finish\_compute, finish\_chunked1, finish\_chunked2]() {
cudaStreamSynchronize(stream1);
cudaStreamSynchronize(stream2);
cudaStreamDestroy(stream1);
cudaStreamDestroy(stream2);
cudaEventDestroy(finish\_allocations);
cudaEventDestroy(finish\_compute);
cudaEventDestroy(finish\_chunked1);
cudaEventDestroy(finish\_chunked2);
};
if (!config.is\_async) {
CHK\_IF\_RETURN(cudaStreamSynchronize(stream));
cleanup();
} else {
std::thread(cleanup).detach();
}
return CHK\_LAST();
}
template 
cudaError\_t msm\_cuda(const S\* scalars, const A\* points, int msm\_size, const MSMConfig& config, P\* results)
{
const int bitsize = (config.bitsize == 0) ? S::NBITS : config.bitsize;
cudaStream\_t stream = reinterpret\_cast(config.stream);
unsigned c = (config.c == 0) ? get\_optimal\_c(msm\_size) : config.c;
if (c > 30) {
ICICLE\_LOG\_WARNING << "msm: c value is much too high - should be lower than 30, reducing to 30";
c = 30;
}
if (config.c == 0 && config.precompute\_factor > 1)
c = min(c, MAX\_C\_FOR\_PRECOMPUTATION); // limit precomputation c so we won't run into bucket memory overflow in msm
// (TODO - find better solution)
const int \_nof\_chunks = get\_nof\_chunks(config.ext);
int nof\_chunks, min\_nof\_chunks;
unsigned fixed\_c = 0;
int fixed\_batch\_size = 0;
if (\_nof\_chunks) {
nof\_chunks = \_nof\_chunks;
min\_nof\_chunks = get\_min\_nof\_chunks(config, msm\_size, c, bitsize, fixed\_c, fixed\_batch\_size);
if (min\_nof\_chunks > nof\_chunks || fixed\_c || fixed\_batch\_size) {
nof\_chunks = min\_nof\_chunks;
ICICLE\_LOG\_WARNING << "msm: using given parameters msm will require too much memory, using these parameters "
"instead: nof\_chunks = "
<< nof\_chunks << ", c = " << (fixed\_c ? fixed\_c : c)
<< ", batch\_size = " << (fixed\_batch\_size ? fixed\_batch\_size : config.batch\_size);
}
} else {
nof\_chunks = get\_optimal\_nof\_chunks(config, msm\_size, c, bitsize, fixed\_c, fixed\_batch\_size);
}
if (fixed\_batch\_size) {
for (int i = 0; i < config.batch\_size; i++) {
CHK\_IF\_RETURN(multi\_chunked\_msm(
scalars + i \* msm\_size,
points + i \* (config.are\_points\_shared\_in\_batch ? 0 : msm\_size) \* config.precompute\_factor, msm\_size,
nof\_chunks, config, results + i, bitsize, fixed\_c ? fixed\_c : c, fixed\_batch\_size,
stream)); // TODO - run on different streams
}
return CHK\_LAST();
}
return CHK\_STICKY(multi\_chunked\_msm(
scalars, points, msm\_size, nof\_chunks, config, results, bitsize, fixed\_c ? fixed\_c : c, config.batch\_size,
stream));
}
template 
static cudaError\_t precompute\_msm\_points\_chunk(
const A\* points,
int points\_size,
int precompute\_factor,
int c,
bool are\_points\_on\_device,
bool are\_points\_montgomery\_form,
cudaStream\_t stream,
A\* output\_points)
{
CHK\_INIT\_IF\_RETURN();
A\* points\_d;
if (!are\_points\_on\_device) {
CHK\_IF\_RETURN(cudaMallocAsync(&points\_d, sizeof(A) \* points\_size, stream));
CHK\_IF\_RETURN(cudaMemcpyAsync(points\_d, points, sizeof(A) \* points\_size, cudaMemcpyHostToDevice, stream));
}
unsigned total\_nof\_bms = (P::SCALAR\_FIELD\_NBITS - 1) / c + 1;
unsigned shift = c \* ((total\_nof\_bms - 1) / precompute\_factor + 1);
unsigned NUM\_THREADS = 1 << 8;
unsigned NUM\_BLOCKS = (points\_size + NUM\_THREADS - 1) / NUM\_THREADS;
precompute\_points\_kernel<<>>(
points, shift, precompute\_factor, points\_size, output\_points, are\_points\_montgomery\_form);
if (!are\_points\_on\_device) CHK\_IF\_RETURN(cudaFreeAsync(points\_d, stream));
return CHK\_LAST();
}
template 
static cudaError\_t chunked\_precompute(
const A\* points,
int msm\_size,
unsigned c,
int points\_batch\_size,
const MSMConfig& config,
A\* points\_precomputed,
int nof\_chunks)
{
cudaStream\_t stream = reinterpret\_cast(config.stream);
A\* points\_d;
A \*points\_precomputed\_d, \*points\_precomputed\_h;
int points\_size = msm\_size \* points\_batch\_size;
bool multi\_batch\_mode = points\_batch\_size > 1;
int chunk\_size = (points\_size + nof\_chunks - 1) / nof\_chunks;
if (multi\_batch\_mode) chunk\_size = ((points\_batch\_size + nof\_chunks - 1) / nof\_chunks) \* msm\_size;
if (config.are\_points\_on\_device) {
points\_precomputed\_d = points\_precomputed;
} else {
CHK\_IF\_RETURN(cudaMallocAsync(&points\_d, sizeof(A) \* chunk\_size \* 2, stream));
CHK\_IF\_RETURN(
cudaMallocAsync(&points\_precomputed\_d, sizeof(A) \* chunk\_size \* 2 \* config.precompute\_factor, stream));
points\_precomputed\_h = points\_precomputed;
}
for (int i = 0; i < nof\_chunks; i++) {
int sub\_msm\_size = min(points\_size - chunk\_size \* i, chunk\_size);
if (sub\_msm\_size <= 0) break;
if (!config.are\_points\_on\_device) {
CHK\_IF\_RETURN(cudaMemcpyAsync(
points\_d + (i % 2) \* chunk\_size, points + i \* chunk\_size, sizeof(A) \* sub\_msm\_size, cudaMemcpyHostToDevice,
stream)); // points are on host
CHK\_IF\_RETURN((precompute\_msm\_points\_chunk(
points\_d + (i % 2) \* chunk\_size, sub\_msm\_size, config.precompute\_factor, c, true,
config.are\_points\_montgomery\_form, stream,
points\_precomputed\_d + (i % 2) \* chunk\_size \* config.precompute\_factor)));
CHK\_IF\_RETURN(cudaMemcpyAsync(
points\_precomputed\_h + i \* chunk\_size \* config.precompute\_factor,
points\_precomputed\_d + (i % 2) \* chunk\_size \* config.precompute\_factor,
sizeof(A) \* sub\_msm\_size \* config.precompute\_factor, cudaMemcpyDeviceToHost, stream));
} else {
CHK\_IF\_RETURN((precompute\_msm\_points\_chunk(
points + i \* chunk\_size, sub\_msm\_size, config.precompute\_factor, c, true, config.are\_points\_montgomery\_form,
stream,
points\_precomputed\_d + i \* chunk\_size \* config.precompute\_factor))); // poinst are on device
}
}
if (!config.are\_points\_on\_device) {
CHK\_IF\_RETURN(cudaFreeAsync(points\_d, stream));
CHK\_IF\_RETURN(cudaFreeAsync(points\_precomputed\_d, stream));
}
return CHK\_LAST();
}
template 
static int get\_precomputation\_nof\_chunks(const MSMConfig& config, int msm\_size, int points\_batch\_size)
{
unsigned long points\_mem = sizeof(A) \* msm\_size \* config.precompute\_factor \* points\_batch\_size;
unsigned long total\_memory;
unsigned long gpu\_memory;
cudaMemGetInfo(&gpu\_memory, &total\_memory); // TODO - handle cuda error
double reduced\_gpu\_memory =
0.7 \* static\_cast(gpu\_memory); // 0.7 is an arbitrary value in order to have some spare memory
double lower\_bound =
static\_cast(4 \* points\_mem) / reduced\_gpu\_memory; // factor is between 2-3, we take 4 for good measure
int nof\_chunks = static\_cast(lower\_bound) + 1;
if (points\_batch\_size > 1 && nof\_chunks > points\_batch\_size) { return 0; }
return nof\_chunks;
}
template 
static cudaError\_t
cuda\_precompute\_msm\_points(const A\* points, int msm\_size, const MSMConfig& config, A\* output\_points)
{
unsigned c = (config.c == 0) ? min(get\_optimal\_c(msm\_size), MAX\_C\_FOR\_PRECOMPUTATION)
: config.c; // limit precomputation c so we won't run into bucket memory overflow in
// msm (TODO - find better solution)
const int bitsize = (config.bitsize == 0) ? S::NBITS : config.bitsize;
int nof\_chunks = get\_precomputation\_nof\_chunks(
config, msm\_size, config.are\_points\_shared\_in\_batch ? 1 : config.batch\_size);
if (nof\_chunks) {
return CHK\_STICKY((chunked\_precompute(
points, msm\_size, c, config.are\_points\_shared\_in\_batch ? 1 : config.batch\_size, config, output\_points,
nof\_chunks)));
}
nof\_chunks = get\_precomputation\_nof\_chunks(config, msm\_size, 1);
for (int i = 0; i < config.batch\_size; i++) {
CHK\_IF\_RETURN((chunked\_precompute(
points + i \* msm\_size, msm\_size, c, 1, config, output\_points + i \* msm\_size \* config.precompute\_factor,
nof\_chunks)));
}
if (!config.is\_async) {
cudaStream\_t stream = reinterpret\_cast(config.stream);
CHK\_IF\_RETURN(cudaStreamSynchronize(stream));
}
return CHK\_LAST();
}
template 
static eIcicleError msm\_cuda\_wrapper(
const Device& device, const S\* scalars, const A\* bases, int msm\_size, const MSMConfig& config, P\* results)
{
auto err = msm\_cuda(scalars, bases, msm\_size, config, results);
return translateCudaError(err);
}
template 
static eIcicleError msm\_precompute\_bases\_cuda\_wrapper(
const Device& device, const A\* input\_bases, int nof\_bases, const MSMConfig& config, A\* output\_bases)
{
auto err = cuda\_precompute\_msm\_points(input\_bases, nof\_bases, config, output\_bases);
return translateCudaError(err);
}
} // namespace msm~~~~~~