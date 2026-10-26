# gnark-cuda MSM adapter source
> Source: https://raw.githubusercontent.com/polytope-labs/gnark-cuda/main/src/api.cu
> Collected: 2026-10-26
> Published: Unknown

// =============================================================================
// gnark-cuda api.cu — the icicle-delegation layer + api-resident bespoke kernels.
//
// CUDA port of gnark-hip/src/api.hip. This file exports the `gpu\_\*` C-ABI
// (see include/gnark\_cuda.h) that the gnark-crypto seam binds to. Generic
// primitives (MSM/NTT/vec) delegate to a self-built open-icicle CUDA backend;
// the small bespoke PLONK kernels are ported in-place from the HIP original.
//
// CORRECTNESS STATUS (see STATUS.md):
// \* Device mgmt / memory / streams / bitreverse / MSM / bespoke kernels /
// wrappers -> faithful ports, expected correct (gated by M2/M3 KATs).
// \* NTT family + fused FFT (gpu\_ntt\*, gpu\_fft\_scale\_fft\*, gpu\_plonk\_restore\_
// batch) -> BEST-EFFORT mapping of gnark-hip's explicit DIT/DIF/bit-reverse/
// 1-over-n semantics onto icicle's Ordering + always-scaled inverse. This is
// the #1 fragile area (plan §4/§6.7/§8.5). Every branch tagged [FRAGILE]
// MUST pass the M4 per-mode round-trip tests on real sm\_120 hardware before
// being trusted. Do not assume correctness from the fact that it compiles.
// =============================================================================
#include "gnark\_cuda.h"
#include "plonk.h"
#include "ratio.h"
#include "fr.h"
#include 
#include 
#include "icicle/runtime.h"
#include "icicle/device.h"
#include "icicle/errors.h"
#include "icicle/ntt.h"
#include "icicle/msm.h"
#include "icicle/vec\_ops.h"
#include "icicle/api/bls12\_381.h"
using namespace icicle;
using bls12\_381::scalar\_t;
using bls12\_381::affine\_t;
using bls12\_381::projective\_t;
// Internal launchers defined in vecops.cu. (vec\_from\_mont/vec\_to\_mont are part
// of the public C-ABI declared in gnark\_cuda.h and defined in vecops.cu, so we
// do not redeclare them here.)
extern "C" cudaError\_t vec\_mul(Fr\* d\_r, const Fr\* d\_a, const Fr\* d\_b, uint32\_t n, cudaStream\_t stream);
extern "C" cudaError\_t vec\_denominators(Fr\* d\_r, const Fr\* d\_twiddles, const Fr\* d\_coset, uint32\_t n, cudaStream\_t stream);
static inline int ok(eIcicleError e) { return e == eIcicleError::SUCCESS ? 0 : -1; }
// Multiply each element by the field element n (= 2^log\_n). Used to undo
// icicle's mandatory 1/n on inverse NTT for the no-scale variants. n < 2^32.
\_\_global\_\_ void ntt\_scale\_by\_n\_kernel(Fr\* data, uint32\_t n) {
uint32\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if (tid >= n) return;
Fr n\_can = {{(uint64\_t)n, 0, 0, 0}};
Fr n\_mont; fr::to\_mont(n\_mont, n\_can);
Fr tmp; fr::mul(tmp, data[tid], n\_mont);
data[tid] = tmp;
}
// Batch element-wise multiply: polys[p][i] \*= scale[i] for all p != skip\_idx.
\_\_global\_\_ void batch\_vec\_mul\_kernel(Fr\*\* polys, const Fr\* scale,
uint32\_t n, uint32\_t count, int skip\_idx) {
uint64\_t tid = (uint64\_t)blockIdx.x \* blockDim.x + threadIdx.x;
uint64\_t total = (uint64\_t)count \* n;
if (tid >= total) return;
uint32\_t poly = tid / n;
if ((int)poly == skip\_idx) return;
uint32\_t idx = tid % n;
Fr\* data = polys[poly];
Fr tmp;
fr::mul(tmp, data[idx], scale[idx]);
data[idx] = tmp;
}
extern "C" {
// --- Device management -------------------------------------------------------
int gpu\_init(int device\_id) {
// Load the self-built open-icicle CUDA backend from ICICLE\_BACKEND\_INSTALL\_DIR.
icicle\_load\_backend\_from\_env\_or\_default();
Device dev{"CUDA", device\_id};
return ok(icicle\_set\_device(dev));
}
int gpu\_device\_count() {
int count = 0;
icicle\_get\_device\_count(count);
return count;
}
// icicle's active device is per-thread. Go cgo calls run on arbitrary OS
// threads, so every goroutine doing GPU work must (re)select CUDA or icicle
// silently dispatches to the CPU backend and dereferences device pointers as
// host memory. Cheap; call at the start of each GPU entry point.
int gpu\_set\_device(int device\_id) {
Device dev{"CUDA", device\_id};
return ok(icicle\_set\_device(dev));
}
void gpu\_sync() {
icicle\_device\_synchronize();
}
// --- Memory management -------------------------------------------------------
void\* gpu\_malloc(size\_t size) {
void\* ptr = nullptr;
return icicle\_malloc(&ptr, size) == eIcicleError::SUCCESS ? ptr : nullptr;
}
void gpu\_free(void\* ptr) {
if (ptr) icicle\_free(ptr);
}
int gpu\_memcpy\_h2d(void\* dst, const void\* src, size\_t size) {
return ok(icicle\_copy\_to\_device(dst, src, size));
}
int gpu\_memcpy\_d2h(void\* dst, const void\* src, size\_t size) {
return ok(icicle\_copy\_to\_host(dst, src, size));
}
int gpu\_memcpy\_d2d(void\* dst, const void\* src, size\_t size) {
return ok(icicle\_copy(dst, src, size));
}
int gpu\_memcpy\_h2d\_on\_stream(void\* dst, const void\* src, size\_t size, void\* stream) {
return ok(icicle\_copy\_to\_device\_async(dst, src, size, (icicleStreamHandle)stream));
}
int gpu\_memcpy\_d2h\_on\_stream(void\* dst, const void\* src, size\_t size, void\* stream) {
return ok(icicle\_copy\_to\_host\_async(dst, src, size, (icicleStreamHandle)stream));
}
// --- Streams -----------------------------------------------------------------
void\* gpu\_stream\_create() {
icicleStreamHandle s = nullptr;
return icicle\_create\_stream(&s) == eIcicleError::SUCCESS ? (void\*)s : nullptr;
}
void gpu\_stream\_sync(void\* stream) {
icicle\_stream\_synchronize((icicleStreamHandle)stream);
}
void gpu\_stream\_destroy(void\* stream) {
if (stream) icicle\_destroy\_stream((icicleStreamHandle)stream);
}
// --- NTT (delegated to icicle) ----------------------------------------------
//
// gnark-hip exposed NTTs in terms of explicit DIT/DIF + bit-reverse + a
// "noscale" flag that suppresses the 1/n on inverse. icicle instead takes an
// `Ordering` (kNN/kNR/kRN) and ALWAYS applies 1/n on inverse. The mapping below
// reproduces the gnark-hip surface; the [FRAGILE] tags mark every place whose
// ordering/scaling equivalence must be confirmed by the M4 round-trip tests.
//
// Reference (gnark-hip api.hip):
// gpu\_ntt = ntt\_execute (natural in/out, inv scales 1/n)
// gpu\_ntt\_dif = ntt\_execute then bitreverse (natural in, bit-rev out)
// gpu\_ntt\_dit = ntt\_execute\_nobr (bit-rev in, natural out)
// gpu\_ntt\_dit\_nosc = ntt\_execute\_nobr\_noscale (bit-rev in, natural out, no 1/n)
// gpu\_ntt\_dif\_nosc = BR -> nobr\_noscale -> BR
// gpu\_ntt\_coset = ntt\_coset\_execute (coset\_gen, natural in/out)
// gpu\_ntt\_coset\_dit = ntt\_coset\_execute\_nobr
static NTTConfig make\_ntt\_cfg(Ordering ord, icicleStreamHandle s) {
NTTConfig cfg = default\_ntt\_config();
cfg.ordering = ord;
cfg.are\_inputs\_on\_device = true;
cfg.are\_outputs\_on\_device = true;
cfg.is\_async = (s != nullptr);
cfg.stream = s;
return cfg;
}
// In-place icicle NTT (input aliased as output). dir: kForward / kInverse.
static int icicle\_ntt\_inplace(Fr\* d, uint32\_t log\_n, NTTDir dir, Ordering ord,
icicleStreamHandle s) {
NTTConfig cfg = make\_ntt\_cfg(ord, s);
scalar\_t\* p = reinterpret\_cast(d);
return ok(bls12\_381\_ntt(p, (int)(1u << log\_n), dir, &cfg, p));
}
static uint32\_t g\_ntt\_domain\_log = 0; // size the domain is currently built for
int gpu\_ntt\_init(uint32\_t log\_n) {
if (g\_ntt\_domain\_log >= log\_n) return 0; // existing (larger) domain covers it
if (g\_ntt\_domain\_log != 0) bls12\_381\_ntt\_release\_domain();
// Build the domain from GNARK's 2-adic root of unity so icicle's NTT basis
// matches gnark's FFT. omega\_{2^32} (canonical, from gnark-crypto fr) raised
// to 2^(32-log\_n) gives the primitive 2^log\_n-th root. icicle's internal
// scalar form is canonical (it strips Montgomery on input), so we pass the
// canonical root; gnark's Montgomery \*data\* then passes R through linearly.
// Build for the LARGEST size requested; smaller NTTs use derived sub-roots.
static const uint64\_t OMEGA32\_CANON[4] = {
0x3829971f439f0d2bULL, 0xb63683508c2280b9ULL,
0xd09b681922c813b4ULL, 0x16a2a19edfe81f20ULL};
scalar\_t root;
memcpy(&root, OMEGA32\_CANON, sizeof(root));
for (uint32\_t i = 0; i < 32u - log\_n; i++) root = root \* root;
NTTInitDomainConfig cfg = default\_ntt\_init\_domain\_config();
if (bls12\_381\_ntt\_init\_domain(&root, &cfg) != eIcicleError::SUCCESS) return -1;
g\_ntt\_domain\_log = log\_n;
return 0;
}
void gpu\_ntt\_cleanup() {
g\_ntt\_domain\_log = 0;
bls12\_381\_ntt\_release\_domain();
}
int gpu\_ntt(void\* d, uint32\_t log\_n, int direction, void\* stream) {
// [FRAGILE] natural-order both ends.
return icicle\_ntt\_inplace((Fr\*)d, log\_n,
direction == 0 ? NTTDir::kForward : NTTDir::kInverse,
Ordering::kNN, (icicleStreamHandle)stream);
}
int gpu\_ntt\_dif(void\* d, uint32\_t log\_n, int direction, void\* stream) {
// [FRAGILE] natural in, bit-reversed out -> Ordering::kNR.
return icicle\_ntt\_inplace((Fr\*)d, log\_n,
direction == 0 ? NTTDir::kForward : NTTDir::kInverse,
Ordering::kNR, (icicleStreamHandle)stream);
}
int gpu\_ntt\_dit(void\* d, uint32\_t log\_n, int direction, void\* stream) {
// [FRAGILE] bit-reversed in, natural out -> Ordering::kRN.
return icicle\_ntt\_inplace((Fr\*)d, log\_n,
direction == 0 ? NTTDir::kForward : NTTDir::kInverse,
Ordering::kRN, (icicleStreamHandle)stream);
}
// The "\_noscale" inverse variants suppress the 1/n that icicle ALWAYS applies on
// inverse. We undo it by multiplying the result by n (montgomery data x canonical
// n via the kernel's montgomery mul). Forward direction needs no compensation.
int gpu\_ntt\_dit\_noscale(void\* d, uint32\_t log\_n, int direction, void\* stream) {
NTTDir dir = direction == 0 ? NTTDir::kForward : NTTDir::kInverse;
if (icicle\_ntt\_inplace((Fr\*)d, log\_n, dir, Ordering::kRN, (icicleStreamHandle)stream) != 0) return -1;
if (direction != 0) {
uint32\_t n = 1u << log\_n;
ntt\_scale\_by\_n\_kernel<<<(n + 255) / 256, 256, 0, (cudaStream\_t)stream>>>((Fr\*)d, n);
if (cudaGetLastError() != cudaSuccess) return -1;
}
return 0;
}
int gpu\_ntt\_dif\_noscale(void\* d, uint32\_t log\_n, int direction, void\* stream) {
NTTDir dir = direction == 0 ? NTTDir::kForward : NTTDir::kInverse;
if (icicle\_ntt\_inplace((Fr\*)d, log\_n, dir, Ordering::kNR, (icicleStreamHandle)stream) != 0) return -1;
if (direction != 0) {
uint32\_t n = 1u << log\_n;
ntt\_scale\_by\_n\_kernel<<<(n + 255) / 256, 256, 0, (cudaStream\_t)stream>>>((Fr\*)d, n);
if (cudaGetLastError() != cudaSuccess) return -1;
}
return 0;
}
// --- Coset NTT: coset-shift multiply + NTT (matches gnark domain coset FFT) ---
// gnark's coset FFT shifts by g = FrMultiplicativeGen = 7. Forward coset:
// data[i]\*=g^i then FFT. Inverse coset: IFFT (incl. 1/n) then data[i]\*=g^{-i}.
// g^{±i} is computed per-thread (square-and-multiply on the index) to avoid a
// 2^25-element (1 GB) coset-powers table. Mirrors gnark-hip's ntt\_coset\_execute.
\_\_device\_\_ Fr g\_coset\_gen[1]; // 7 in Montgomery form
\_\_device\_\_ Fr g\_coset\_gen\_inv[1]; // 7^{-1} in Montgomery form
\_\_global\_\_ void coset\_gen\_setup\_kernel() {
Fr seven; seven.limbs[0] = 7; seven.limbs[1] = 0; seven.limbs[2] = 0; seven.limbs[3] = 0;
fr::to\_mont(g\_coset\_gen[0], seven);
fr::inv(g\_coset\_gen\_inv[0], g\_coset\_gen[0]);
}
static bool g\_coset\_gen\_ready = false;
static int ensure\_coset\_gen(cudaStream\_t s) {
if (g\_coset\_gen\_ready) return 0;
coset\_gen\_setup\_kernel<<<1, 1, 0, s>>>();
if (cudaStreamSynchronize(s) != cudaSuccess) return -1;
g\_coset\_gen\_ready = true;
return 0;
}
// Two-level coset-powers table: l1[j]=g^j (j>logB] \* l1[i & (B-1)] — one lookup-multiply per element. Replaces
// a per-thread square-and-multiply (which warp-diverges + dominated divideByZH
// at 33M elements). The two small tables are built once per (inverse,logB).
static Fr\* d\_coset\_l1 = nullptr;
static Fr\* d\_coset\_l2 = nullptr;
static const uint32\_t COSET\_LMAX = 1u << 14; // per-level table capacity
\_\_global\_\_ void coset\_table\_build\_kernel(Fr\* l1, Fr\* l2, uint32\_t B, uint32\_t nL2, int inverse) {
Fr g = inverse ? g\_coset\_gen\_inv[0] : g\_coset\_gen[0];
fr::set\_one(l1[0]);
for (uint32\_t j = 1; j < B; j++) fr::mul(l1[j], l1[j - 1], g);
Fr gB; fr::mul(gB, l1[B - 1], g); // g^B
fr::set\_one(l2[0]);
for (uint32\_t k = 1; k < nL2; k++) fr::mul(l2[k], l2[k - 1], gB);
}
\_\_global\_\_ void coset\_mul2\_kernel(Fr\* data, const Fr\* l1, const Fr\* l2, uint32\_t n, uint32\_t logB) {
uint32\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if (tid >= n) return;
uint32\_t lo = tid & ((1u << logB) - 1);
uint32\_t hi = tid >> logB;
Fr p; fr::mul(p, l2[hi], l1[lo]);
fr::mul(data[tid], data[tid], p);
}
// Multiply data[i] by g^{±i} (inverse selects g^{-1}) via the two-level table.
static int coset\_scale\_fast(Fr\* d, uint32\_t log\_n, int inverse, cudaStream\_t s) {
if (d\_coset\_l1 == nullptr) {
if (cudaMalloc(&d\_coset\_l1, COSET\_LMAX \* sizeof(Fr)) != cudaSuccess) return -1;
if (cudaMalloc(&d\_coset\_l2, COSET\_LMAX \* sizeof(Fr)) != cudaSuccess) return -1;
}
uint32\_t logB = log\_n / 2;
if (logB > 13) logB = 13;
uint32\_t B = 1u << logB;
uint32\_t nL2 = (1u << log\_n) >> logB;
if (nL2 == 0) nL2 = 1;
if (B > COSET\_LMAX || nL2 > COSET\_LMAX) return -1;
coset\_table\_build\_kernel<<<1, 1, 0, s>>>(d\_coset\_l1, d\_coset\_l2, B, nL2, inverse);
uint32\_t n = 1u << log\_n;
uint32\_t block = 256, grid = (n + block - 1) / block;
coset\_mul2\_kernel<<>>(d, d\_coset\_l1, d\_coset\_l2, n, logB);
return 0;
}
// Coset NTT in the kNN (natural-in/natural-out) ordering.
int gpu\_ntt\_coset(void\* d, uint32\_t log\_n, int direction, void\* stream) {
cudaStream\_t s = (cudaStream\_t)stream;
if (ensure\_coset\_gen(s) != 0) return -1;
if (direction == 1) { // inverse: IFFT (+1/n) then \* g^{-i}
if (gpu\_ntt(d, log\_n, 1, stream) != 0) return -1;
if (coset\_scale\_fast((Fr\*)d, log\_n, 1, s) != 0) return -1;
} else { // forward: \* g^i then FFT
if (coset\_scale\_fast((Fr\*)d, log\_n, 0, s) != 0) return -1;
if (gpu\_ntt(d, log\_n, 0, stream) != 0) return -1;
}
return 0;
}
// Coset NTT in DIT ordering (bit-reversed input -> natural output for inverse).
// This is the path gnark's a.ToCanonical(bigDomain) takes for a LagrangeCoset /
// BitReverse polynomial (divideByZH): DIT inverse FFT then \* g^{-i} in natural order.
int gpu\_ntt\_coset\_dit(void\* d, uint32\_t log\_n, int direction, void\* stream) {
cudaStream\_t s = (cudaStream\_t)stream;
if (ensure\_coset\_gen(s) != 0) return -1;
if (direction == 1) { // inverse: DIT IFFT (bit-rev in -> natural out, +1/n) then \* g^{-i}
if (gpu\_ntt\_dit(d, log\_n, 1, stream) != 0) return -1;
if (coset\_scale\_fast((Fr\*)d, log\_n, 1, s) != 0) return -1;
} else { // forward: \* g^i then DIT FFT
if (coset\_scale\_fast((Fr\*)d, log\_n, 0, s) != 0) return -1;
if (gpu\_ntt\_dit(d, log\_n, 0, stream) != 0) return -1;
}
return 0;
}
int gpu\_bitreverse(void\* d, uint32\_t log\_n, void\* stream) {
VecOpsConfig cfg = default\_vec\_ops\_config();
cfg.is\_a\_on\_device = true;
cfg.is\_result\_on\_device = true;
cfg.is\_async = (stream != nullptr);
cfg.stream = (icicleStreamHandle)stream;
scalar\_t\* p = reinterpret\_cast(d);
return ok(bls12\_381\_bit\_reverse(p, (uint64\_t)(1u << log\_n), &cfg, p));
}
// --- Fused FFT: IFFT -> multiply -> FFT (device-resident) --------------------
// [FRAGILE] Mirrors gnark-hip's gpu\_fft\_scale\_fft using the DIT/DIF mapping
// above. Inherits all the NTT-mapping caveats; M4 round-trips the whole fused
// op against a CPU IFFT->scale->FFT reference for every (ifft\_dir,fft\_dir).
int gpu\_fft\_scale\_fft(void\* d\_data, void\* d\_scale, uint32\_t log\_n,
int ifft\_dir, int fft\_dir, void\* stream) {
icicleStreamHandle s = (icicleStreamHandle)stream;
uint32\_t n = 1u << log\_n;
// Phase 1: inverse FFT (DIT=kRN bit-rev-in / DIF=kNR natural-in).
if ((ifft\_dir == 1 ? gpu\_ntt\_dif(d\_data, log\_n, 1, stream)
: gpu\_ntt\_dit(d\_data, log\_n, 1, stream)) != 0) return -1;
// Phase 2: element-wise scale on device.
if (vec\_mul((Fr\*)d\_data, (Fr\*)d\_data, (Fr\*)d\_scale, n, (cudaStream\_t)s) != cudaSuccess) return -1;
// Phase 3: forward FFT.
if ((fft\_dir == 1 ? gpu\_ntt\_dif(d\_data, log\_n, 0, stream)
: gpu\_ntt\_dit(d\_data, log\_n, 0, stream)) != 0) return -1;
return 0;
}
// d\_polys is a DEVICE array of `count` Fr\* pointers. Loop the per-poly fused FFT
// (correctness-first; batched NTT optimization deferred). Copy the pointer array
// to host to iterate.
int gpu\_fft\_scale\_fft\_batch(void\* d\_polys, uint32\_t count, int skip\_idx,
void\* d\_scale, uint32\_t log\_n,
int ifft\_dir, int fft\_dir, void\* stream) {
if (count == 0) return 0;
void\*\* hostPtrs = (void\*\*)malloc(count \* sizeof(void\*));
if (!hostPtrs) return -1;
if (icicle\_copy\_to\_host(hostPtrs, d\_polys, count \* sizeof(void\*)) != eIcicleError::SUCCESS) { free(hostPtrs); return -1; }
int rc = 0;
for (uint32\_t i = 0; i < count; i++) {
if ((int)i == skip\_idx) continue;
if (gpu\_fft\_scale\_fft(hostPtrs[i], d\_scale, log\_n, ifft\_dir, fft\_dir, stream) != 0) { rc = -1; break; }
}
free(hostPtrs);
return rc;
}
// --- MSM (delegated to icicle) ----------------------------------------------
// [VERIFY @ M3] Result coordinate convention: icicle projective\_t vs gnark
// G1Jac. If icicle's projective is homogeneous (x=X/Z) rather than Jacobian
// (x=X/Z^2) the host result must be converted. Asserted in the M3 MSM KAT.
// gnark BLS12-381 fp Montgomery 1 (= R mod p), from gnark-crypto fp Element.SetOne().
static const uint64\_t FP\_ONE\_MONT[6] = {
0x760900000002fffdULL, 0xebf4000bc40c0002ULL, 0x5f48985753c758baULL,
0x77ce585370525745ULL, 0x5c071a97a256ec6dULL, 0x15f65ec3fa80e493ULL};
// gpu\_msm expects CANONICAL (non-Montgomery) device points + scalars. The caller
// (gnark-crypto gpu.go) converts Montgomery->canonical and caches the canonical
// points, because (a) icicle's in-MSM Montgomery path is buggy when the MSM is
// chunked under memory pressure (see ICICLE\_MSM\_MONTGOMERY\_CHUNKING\_BUG.md), and
// (b) doing the conversion outside lets the reused SRS points be cached.
int gpu\_msm(const void\* d\_points, const void\* d\_scalars, uint32\_t n,
void\* h\_result, uint32\_t window\_size, void\* stream) {
MSMConfig cfg = default\_msm\_config();
cfg.stream = (icicleStreamHandle)stream;
cfg.are\_scalars\_on\_device = true;
cfg.are\_points\_on\_device = true;
cfg.are\_results\_on\_device = false; // write result to host
cfg.are\_scalars\_montgomery\_form = false; // inputs are already canonical
cfg.are\_points\_montgomery\_form = false;
if (window\_size > 0) cfg.c = (int)window\_size;
cfg.is\_async = false;
projective\_t proj; // icicle homogeneous projective (canonical)
const bool dbg = getenv("GPU\_MSM\_DEBUG") != nullptr;
if (dbg) fprintf(stderr, "[gpu\_msm] n=%u calling bls12\_381\_msm\n", n);
if (bls12\_381\_msm(reinterpret\_cast(d\_scalars),
reinterpret\_cast(d\_points),
(int)n, &cfg, &proj) != eIcicleError::SUCCESS)
return -1;
if (dbg) fprintf(stderr, "[gpu\_msm] msm ok, marshaling\n");
// Marshal -> gnark G1Jac {X,Y,Z fp Montgomery}, 3\*48 bytes. icicle projective
// is HOMOGENEOUS (affine = X/Z), gnark wants Jacobian (affine = X/Z^2), so we
// route via affine. icicle output is canonical -> convert into Montgomery.
uint64\_t\* out = reinterpret\_cast(h\_result); // X[6] Y[6] Z[6]
const uint64\_t\* pz = reinterpret\_cast(&proj) + 12; // proj.z
bool is\_inf = true;
for (int i = 0; i < 6; i++) if (pz[i] != 0) { is\_inf = false; break; }
if (is\_inf) { // gnark infinity: X=1, Y=1, Z=0
for (int i = 0; i < 6; i++) { out[i] = FP\_ONE\_MONT[i]; out[6+i] = FP\_ONE\_MONT[i]; out[12+i] = 0; }
return 0;
}
if (dbg) fprintf(stderr, "[gpu\_msm] not inf, to\_affine\n");
affine\_t aff; bls12\_381\_to\_affine(&proj, &aff); // canonical affine
if (dbg) fprintf(stderr, "[gpu\_msm] to\_affine ok, convert\_montgomery\n");
affine\_t affM; VecOpsConfig vc = default\_vec\_ops\_config();
bls12\_381\_affine\_convert\_montgomery(&aff, 1, /\*is\_into=\*/true, &vc, &affM);
if (dbg) fprintf(stderr, "[gpu\_msm] marshal done\n");
const uint64\_t\* axy = reinterpret\_cast(&affM); // x[6] y[6]
for (int i = 0; i < 6; i++) { out[i] = axy[i]; out[6+i] = axy[6+i]; out[12+i] = FP\_ONE\_MONT[i]; }
return 0; // {x\_m, y\_m, 1\_m} = Jacobian of affine
}
// Size of the icicle homogeneous projective result, so the Go caller can size the
// host landing buffer for gpu\_msm\_async / gpu\_msm\_marshal.
size\_t gpu\_msm\_proj\_bytes(void) { return sizeof(projective\_t); }
// gpu\_msm\_async runs the MSM ASYNCHRONOUSLY on `stream`, writing the raw icicle
// projective result into the host buffer h\_proj (>= gpu\_msm\_proj\_bytes). It does
// NOT sync and does NOT marshal, so several MSMs issued on distinct streams
// overlap on the GPU. The caller syncs each stream, then calls gpu\_msm\_marshal.
// h\_proj and the scalar buffer must stay alive until the sync.
int gpu\_msm\_async(const void\* d\_points, const void\* d\_scalars, uint32\_t n,
void\* h\_proj, uint32\_t window\_size, void\* stream) {
MSMConfig cfg = default\_msm\_config();
cfg.stream = (icicleStreamHandle)stream;
cfg.are\_scalars\_on\_device = true;
cfg.are\_points\_on\_device = true;
cfg.are\_results\_on\_device = false; // result D2H'd to h\_proj (async)
cfg.are\_scalars\_montgomery\_form = false;
cfg.are\_points\_montgomery\_form = false;
if (window\_size > 0) cfg.c = (int)window\_size;
cfg.is\_async = true;
if (bls12\_381\_msm(reinterpret\_cast(d\_scalars),
reinterpret\_cast(d\_points),
(int)n, &cfg, reinterpret\_cast(h\_proj)) != eIcicleError::SUCCESS)
return -1;
return 0;
}
// gpu\_msm\_marshal converts a settled icicle projective (from gpu\_msm\_async, after
// the stream is synced) into a gnark G1Jac {X,Y,Z fp Montgomery}. Pure host work.
int gpu\_msm\_marshal(const void\* h\_proj, void\* h\_result) {
const projective\_t\* proj = reinterpret\_cast(h\_proj);
uint64\_t\* out = reinterpret\_cast(h\_result);
const uint64\_t\* pz = reinterpret\_cast(proj) + 12;
bool is\_inf = true;
for (int i = 0; i < 6; i++) if (pz[i] != 0) { is\_inf = false; break; }
if (is\_inf) {
for (int i = 0; i < 6; i++) { out[i] = FP\_ONE\_MONT[i]; out[6+i] = FP\_ONE\_MONT[i]; out[12+i] = 0; }
return 0;
}
affine\_t aff; bls12\_381\_to\_affine(const\_cast(proj), &aff);
affine\_t affM; VecOpsConfig vc = default\_vec\_ops\_config();
bls12\_381\_affine\_convert\_montgomery(&aff, 1, /\*is\_into=\*/true, &vc, &affM);
const uint64\_t\* axy = reinterpret\_cast(&affM);
for (int i = 0; i < 6; i++) { out[i] = axy[i]; out[6+i] = axy[6+i]; out[12+i] = FP\_ONE\_MONT[i]; }
return 0;
}
// --- Bespoke PLONK kernels (api-resident; ported from HIP) -------------------
\_\_global\_\_ void plonk\_scatter\_result\_kernel(const Fr\* src, Fr\* dst, uint32\_t n, uint32\_t rho,
uint32\_t iter, uint32\_t shift\_bits) {
uint32\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if (tid >= n) return;
uint64\_t out\_idx = ((uint64\_t)rho \* tid) + iter;
out\_idx = \_\_brevll(out\_idx) >> shift\_bits;
dst[out\_idx] = src[tid];
}
\_\_global\_\_ void plonk\_divide\_zh\_kernel(Fr\* data, const Fr\* inv\_rho, uint32\_t n, uint32\_t rho,
uint32\_t shift\_bits) {
uint32\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if (tid >= n) return;
uint64\_t i\_rev = \_\_brevll((uint64\_t)tid) >> shift\_bits;
Fr tmp;
fr::mul(tmp, data[tid], inv\_rho[i\_rev % rho]);
data[tid] = tmp;
}
\_\_global\_\_ void plonk\_fold\_quotient\_kernel(const Fr\* h1, const Fr\* h2, const Fr\* h3,
Fr\* out, const Fr\* zeta\_n\_plus\_2, uint32\_t n) {
uint32\_t tid = blockIdx.x \* blockDim.x + threadIdx.x;
if (tid >= n) return;
const Fr z = \*zeta\_n\_plus\_2;
Fr t;
fr::mul(t, h3[tid], z);
fr::add(t, t, h2[tid]);
fr::mul(t, t, z);
fr::add(out[tid], t, h1[tid]);
}
int gpu\_plonk\_evaluate(void\*\* d\_polys, const void\* d\_twiddles0,
const void\* d\_bp, const void\* d\_challenges,
const void\* d\_precomp\_denoms,
uint32\_t n, uint32\_t npolys, uint32\_t nbBsbGates,
void\* d\_result, void\* stream) {
return plonk\_evaluate\_constraints(
(Fr\*\*)d\_polys, (const Fr\*)d\_twiddles0, (const Fr\*)d\_bp,
(const Fr\*)d\_challenges, (const Fr\*)d\_precomp\_denoms,
n, npolys, nbBsbGates, (Fr\*)d\_result, (cudaStream\_t)stream) == cudaSuccess ? 0 : -1;
}
int gpu\_plonk\_scatter\_result(const void\* d\_src, void\* d\_dst, uint32\_t n, uint32\_t rho,
uint32\_t iter, uint32\_t shift\_bits, void\* stream) {
uint32\_t block = 256, grid = (n + block - 1) / block;
plonk\_scatter\_result\_kernel<<>>(
(const Fr\*)d\_src, (Fr\*)d\_dst, n, rho, iter, shift\_bits);
return cudaGetLastError() == cudaSuccess ? 0 : -1;
}
int gpu\_plonk\_divide\_zh(void\* d\_data, const void\* d\_inv\_rho, uint32\_t n, uint32\_t rho,
uint32\_t shift\_bits, void\* stream) {
uint32\_t block = 256, grid = (n + block - 1) / block;
plonk\_divide\_zh\_kernel<<>>(
(Fr\*)d\_data, (const Fr\*)d\_inv\_rho, n, rho, shift\_bits);
return cudaGetLastError() == cudaSuccess ? 0 : -1;
}
int gpu\_plonk\_fold\_quotient(const void\* d\_h1, const void\* d\_h2, const void\* d\_h3,
void\* d\_out, const void\* d\_zeta\_n\_plus\_2, uint32\_t n, void\* stream) {
uint32\_t block = 256, grid = (n + block - 1) / block;
plonk\_fold\_quotient\_kernel<<>>(
(const Fr\*)d\_h1, (const Fr\*)d\_h2, (const Fr\*)d\_h3, (Fr\*)d\_out,
(const Fr\*)d\_zeta\_n\_plus\_2, n);
return cudaGetLastError() == cudaSuccess ? 0 : -1;
}
int gpu\_ratio\_copy\_terms(const void\* d\_l, const void\* d\_r, const void\* d\_o,
const void\* d\_s1, const void\* d\_s2, const void\* d\_s3,
const void\* d\_twiddles0, const void\* d\_challenges,
uint32\_t n, void\* d\_out\_num, void\* d\_out\_den, void\* stream) {
return ratio\_copy\_terms(
(const Fr\*)d\_l, (const Fr\*)d\_r, (const Fr\*)d\_o,
(const Fr\*)d\_s1, (const Fr\*)d\_s2, (const Fr\*)d\_s3,
(const Fr\*)d\_twiddles0, (const Fr\*)d\_challenges,
n, (Fr\*)d\_out\_num, (Fr\*)d\_out\_den, (cudaStream\_t)stream) == cudaSuccess ? 0 : -1;
}
int gpu\_ratio\_prefix\_scan(void\* d\_data, uint32\_t n, void\* stream) {
return ratio\_prefix\_scan((Fr\*)d\_data, n, (cudaStream\_t)stream) == cudaSuccess ? 0 : -1;
}
int gpu\_ratio\_apply\_inverse(void\* d\_coeffs, const void\* d\_den, uint32\_t n, void\* stream) {
return ratio\_apply\_inverse((Fr\*)d\_coeffs, (const Fr\*)d\_den, n, (cudaStream\_t)stream) == cudaSuccess ? 0 : -1;
}
int gpu\_vec\_mul(void\* d\_r, const void\* d\_a, const void\* d\_b, uint32\_t n, void\* stream) {
return vec\_mul((Fr\*)d\_r, (const Fr\*)d\_a, (const Fr\*)d\_b, n, (cudaStream\_t)stream) == cudaSuccess ? 0 : -1;
}
int gpu\_vec\_denominators(void\* d\_r, const void\* d\_twiddles, const void\* d\_coset, uint32\_t n, void\* stream) {
return vec\_denominators((Fr\*)d\_r, (const Fr\*)d\_twiddles, (const Fr\*)d\_coset, n, (cudaStream\_t)stream) == cudaSuccess ? 0 : -1;
}
// Convert affine G1 points Montgomery->canonical on device (d\_dst may alias d\_src).
// Used by the seam to convert + cache reused MSM bases once.
int gpu\_affine\_from\_mont(void\* d\_dst, const void\* d\_src, uint32\_t n, void\* stream) {
VecOpsConfig vc = default\_vec\_ops\_config();
vc.is\_a\_on\_device = true; vc.is\_result\_on\_device = true;
vc.stream = (icicleStreamHandle)stream; vc.is\_async = (stream != nullptr);
return ok(bls12\_381\_affine\_convert\_montgomery(
reinterpret\_cast(d\_src), (size\_t)n, /\*is\_into=\*/false, &vc,
reinterpret\_cast(d\_dst)));
}
// --- Batch restore (PLONK post-rho) ------------------------------------------
// Lagrange Regular -> Canonical Regular (inverse FFT with 1/n) then multiply by
// the coset-inverse scale\_powers, per poly. Loop over the device pointer array.
int gpu\_plonk\_restore\_batch(void\*\* d\_polys, uint32\_t count, int skip\_idx,
const void\* d\_scale\_powers, uint32\_t log\_n, void\* stream) {
if (count == 0) return 0;
uint32\_t n = 1u << log\_n;
void\*\* hostPtrs = (void\*\*)malloc(count \* sizeof(void\*));
if (!hostPtrs) return -1;
if (icicle\_copy\_to\_host(hostPtrs, d\_polys, count \* sizeof(void\*)) != eIcicleError::SUCCESS) { free(hostPtrs); return -1; }
int rc = 0;
for (uint32\_t i = 0; i < count; i++) {
if ((int)i == skip\_idx) continue;
// inverse FFT (natural in/out + 1/n) == ntt\_execute inverse
if (gpu\_ntt(hostPtrs[i], log\_n, /\*inverse=\*/1, stream) != 0) { rc = -1; break; }
if (vec\_mul((Fr\*)hostPtrs[i], (Fr\*)hostPtrs[i], (const Fr\*)d\_scale\_powers, n, (cudaStream\_t)stream) != cudaSuccess) { rc = -1; break; }
}
free(hostPtrs);
return rc;
}
// --- Polynomial evaluation at a point (chunked Horner) -----------------------
int gpu\_poly\_eval(const void\* d\_coeffs, uint32\_t n, const void\* d\_point,
void\* h\_result, void\* stream) {
cudaStream\_t s = (cudaStream\_t)stream;
const uint32\_t NUM\_CHUNKS = EVAL\_CHUNKS;
uint32\_t chunk\_size = (n + NUM\_CHUNKS - 1) / NUM\_CHUNKS;
uint32\_t actual\_chunks = (n + chunk\_size - 1) / chunk\_size;
Fr\* d\_partials = nullptr; Fr\* d\_point\_power = nullptr; Fr\* d\_result = nullptr;
cudaError\_t err;
if ((err = cudaMalloc(&d\_partials, NUM\_CHUNKS \* sizeof(Fr))) != cudaSuccess) goto cleanup;
if ((err = cudaMalloc(&d\_point\_power, sizeof(Fr))) != cudaSuccess) goto cleanup;
if ((err = cudaMalloc(&d\_result, sizeof(Fr))) != cudaSuccess) goto cleanup;
poly\_eval\_phase1\_kernel<<<1, NUM\_CHUNKS, 0, s>>>(
(const Fr\*)d\_coeffs, (const Fr\*)d\_point, d\_partials, d\_point\_power, n, chunk\_size);
if ((err = cudaGetLastError()) != cudaSuccess) goto cleanup;
poly\_eval\_phase2\_kernel<<<1, 1, 0, s>>>(d\_partials, d\_point\_power, d\_result, actual\_chunks);
if ((err = cudaGetLastError()) != cudaSuccess) goto cleanup;
if ((err = cudaStreamSynchronize(s)) != cudaSuccess) goto cleanup;
err = cudaMemcpy(h\_result, d\_result, sizeof(Fr), cudaMemcpyDeviceToHost);
cleanup:
if (d\_partials) cudaFree(d\_partials);
if (d\_point\_power) cudaFree(d\_point\_power);
if (d\_result) cudaFree(d\_result);
if (err != cudaSuccess) {
fprintf(stderr, "[gpu\_poly\_eval] failed: %s\n", cudaGetErrorString(err));
return -1;
}
return 0;
}
// Async, no-alloc, no-sync variant of gpu\_poly\_eval: the caller supplies scratch
// (d\_partials of EVAL\_CHUNKS Fr, d\_point\_power of 1 Fr) and a device d\_result, and
// syncs the stream once after issuing a whole batch. d\_partials/d\_point\_power may be
// reused across the batch since the kernels serialize on the stream.
int gpu\_poly\_eval\_async(const void\* d\_coeffs, uint32\_t n, const void\* d\_point,
void\* d\_partials, void\* d\_point\_power, void\* d\_result, void\* stream) {
cudaStream\_t s = (cudaStream\_t)stream;
const uint32\_t NUM\_CHUNKS = EVAL\_CHUNKS;
uint32\_t chunk\_size = (n + NUM\_CHUNKS - 1) / NUM\_CHUNKS;
uint32\_t actual\_chunks = (n + chunk\_size - 1) / chunk\_size;
poly\_eval\_phase1\_kernel<<<1, NUM\_CHUNKS, 0, s>>>(
(const Fr\*)d\_coeffs, (const Fr\*)d\_point, (Fr\*)d\_partials, (Fr\*)d\_point\_power, n, chunk\_size);
poly\_eval\_phase2\_kernel<<<1, 1, 0, s>>>(
(Fr\*)d\_partials, (Fr\*)d\_point\_power, (Fr\*)d\_result, actual\_chunks);
return cudaGetLastError() == cudaSuccess ? 0 : -1;
}
// --- Linearized polynomial ---------------------------------------------------
int gpu\_plonk\_linearized\_poly(
const void\* d\_blindedZ, const void\* d\_s3, const void\* d\_ql, const void\* d\_qr,
const void\* d\_qm, const void\* d\_qo, const void\* d\_qk, const void\* d\_hFolded,
const void\* d\_scalars,
uint32\_t n, uint32\_t n\_blindedZ, uint32\_t n\_hFolded,
void\* d\_result, void\* stream)
{
uint32\_t block = 256, grid = (n\_blindedZ + block - 1) / block;
plonk\_linearized\_poly\_kernel<<>>(
(const Fr\*)d\_blindedZ, (const Fr\*)d\_s3, (const Fr\*)d\_ql, (const Fr\*)d\_qr,
(const Fr\*)d\_qm, (const Fr\*)d\_qo, (const Fr\*)d\_qk, (const Fr\*)d\_hFolded,
(const Fr\*)d\_scalars, n, n\_blindedZ, n\_hFolded, (Fr\*)d\_result);
cudaError\_t err = cudaGetLastError();
if (err != cudaSuccess) {
fprintf(stderr, "[gpu\_plonk\_linearized\_poly] launch failed: %s\n", cudaGetErrorString(err));
return -1;
}
return 0;
}
} // extern "C"