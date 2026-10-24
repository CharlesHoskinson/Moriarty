#pragma once
#include "curve.cuh"
#include "field.cuh"
namespace midnight\_plonk {
MIDNIGHT\_PLONK\_HD bool fp\_is\_zero\_hd(const Fp &a) {
uint64\_t acc = 0;
for (int i = 0; i < 6; ++i) {
acc |= a.limbs[i];
}
return acc == 0;
}
\_\_device\_\_ \_\_forceinline\_\_ int fr\_get\_bit(const Fr &s, int bit) {
int limb = bit / 64;
int shift = bit % 64;
if (limb < 0 || limb >= 4) {
return 0;
}
return (s.limbs[limb] >> shift) & 1ULL;
}
\_\_device\_\_ \_\_forceinline\_\_ uint32\_t fr\_get\_window(const Fr &s, int offset, int width) {
int limb = offset / 64;
int shift = offset % 64;
uint64\_t lo = limb >= 0 && limb < 4 ? s.limbs[limb] : 0;
uint64\_t hi = (limb + 1) >= 0 && (limb + 1) < 4 ? s.limbs[limb + 1] : 0;
uint64\_t word = 0;
if (shift == 0) {
word = lo;
} else if (shift < 64) {
word = (lo >> shift) | (hi << (64 - shift));
} else {
word = hi;
}
uint32\_t mask = (width >= 32) ? 0xFFFFFFFFU : ((1U << width) - 1U);
return static\_cast(word) & mask;
}
MIDNIGHT\_PLONK\_HD void g1\_add(G1Jacobian &out, const G1Jacobian &p, const G1Jacobian &q) {
if (p.infinity) {
out = q;
return;
}
if (q.infinity) {
out = p;
return;
}
Fp z1z1, z2z2, u1, u2, s1, s2, h, i, j, r, v;
fp\_square(z1z1, p.z);
fp\_square(z2z2, q.z);
fp\_mul(u1, p.x, z2z2);
fp\_mul(u2, q.x, z1z1);
fp\_mul(s1, p.y, q.z);
fp\_mul(s1, s1, z2z2);
fp\_mul(s2, q.y, p.z);
fp\_mul(s2, s2, z1z1);
fp\_sub(h, u2, u1);
fp\_add(i, h, h);
fp\_square(i, i);
fp\_mul(j, h, i);
fp\_sub(r, s2, s1);
fp\_add(r, r, r);
if (fp\_is\_zero\_hd(h)) {
if (fp\_is\_zero\_hd(r)) {
g1\_double(out, p);
} else {
g1\_set\_infinity(out);
}
return;
}
fp\_mul(v, u1, i);
fp\_square(out.x, r);
fp\_sub(out.x, out.x, j);
fp\_sub(out.x, out.x, v);
fp\_sub(out.x, out.x, v);
fp\_sub(out.y, v, out.x);
fp\_mul(out.y, out.y, r);
fp\_mul(s1, s1, j);
fp\_add(s1, s1, s1);
fp\_sub(out.y, out.y, s1);
fp\_add(out.z, p.z, q.z);
fp\_square(out.z, out.z);
fp\_sub(out.z, out.z, z1z1);
fp\_sub(out.z, out.z, z2z2);
fp\_mul(out.z, out.z, h);
out.infinity = 0;
}
\_\_device\_\_ \_\_forceinline\_\_ void g1\_scalar\_mul(G1Jacobian &out, const G1Affine &p,
const Fr &scalar) {
constexpr int window\_bits = 4;
constexpr int table\_size = 1 << window\_bits;
constexpr int scalar\_bits = 256;
constexpr int windows = (scalar\_bits + window\_bits - 1) / window\_bits;
G1Jacobian table[table\_size];
g1\_set\_infinity(table[0]);
g1\_from\_affine(table[1], p);
for (int i = 2; i < table\_size; ++i) {
g1\_add(table[i], table[i - 1], table[1]);
}
G1Jacobian acc;
g1\_set\_infinity(acc);
Fr scalar\_normal;
fr\_from\_mont(scalar\_normal, scalar);
for (int w = windows - 1; w >= 0; --w) {
for (int i = 0; i < window\_bits; ++i) {
g1\_double(acc, acc);
}
uint32\_t bucket = fr\_get\_window(scalar\_normal, w \* window\_bits, window\_bits);
if (bucket != 0) {
g1\_add(acc, acc, table[bucket]);
}
}
out = acc;
}
static \_\_global\_\_ void msm\_naive\_kernel(const G1Affine \*points, const Fr \*scalars,
int count, G1Jacobian \*out) {
if (blockIdx.x != 0 || threadIdx.x != 0) {
return;
}
G1Jacobian acc;
g1\_set\_infinity(acc);
for (int i = 0; i < count; ++i) {
G1Jacobian tmp;
g1\_scalar\_mul(tmp, points[i], scalars[i]);
g1\_add(acc, acc, tmp);
}
out[0] = acc;
}
cudaError\_t msm\_naive\_launch(const G1Affine \*points, const Fr \*scalars, int count,
G1Jacobian \*out, cudaStream\_t stream);
static \_\_global\_\_ void msm\_partial\_kernel(const G1Affine \*points, const Fr \*scalars,
int count, G1Jacobian \*out) {
int idx = blockIdx.x \* blockDim.x + threadIdx.x;
if (idx >= count) {
return;
}
g1\_scalar\_mul(out[idx], points[idx], scalars[idx]);
}
cudaError\_t msm\_partial\_launch(const G1Affine \*points, const Fr \*scalars, int count,
G1Jacobian \*out, cudaStream\_t stream);
cudaError\_t msm\_reduce\_launch(const G1Jacobian \*in, G1Jacobian \*out, int count,
cudaStream\_t stream);
} // namespace midnight\_plonk