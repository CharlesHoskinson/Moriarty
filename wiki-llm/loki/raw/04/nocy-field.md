#pragma once
#include 
#include 
#define MIDNIGHT\_PLONK\_HD \_\_host\_\_ \_\_device\_\_ \_\_forceinline\_\_
namespace midnight\_plonk {
struct Fr {
uint64\_t limbs[4];
};
struct Fp {
uint64\_t limbs[6];
};
MIDNIGHT\_PLONK\_HD void fr\_from\_bytes(Fr &out, const uint8\_t \*bytes);
MIDNIGHT\_PLONK\_HD void fp\_from\_bytes(Fp &out, const uint8\_t \*bytes);
MIDNIGHT\_PLONK\_HD uint64\_t add\_carry(uint64\_t a, uint64\_t b, uint64\_t \*carry) {
uint64\_t sum = a + b;
uint64\_t c1 = sum < a;
sum += \*carry;
uint64\_t c2 = sum < \*carry;
\*carry = c1 + c2;
return sum;
}
MIDNIGHT\_PLONK\_HD uint64\_t sub\_borrow(uint64\_t a, uint64\_t b, uint64\_t \*borrow) {
uint64\_t diff = a - b;
uint64\_t b1 = a < b;
uint64\_t diff2 = diff - \*borrow;
uint64\_t b2 = diff < \*borrow;
\*borrow = b1 + b2;
return diff2;
}
MIDNIGHT\_PLONK\_HD void mul64(uint64\_t a, uint64\_t b, uint64\_t \*hi, uint64\_t \*lo) {
#if defined(\_\_CUDA\_ARCH\_\_)
\*lo = a \* b;
\*hi = \_\_umul64hi(a, b);
#else
unsigned \_\_int128 prod = static\_cast(a) \* b;
\*lo = static\_cast(prod);
\*hi = static\_cast(prod >> 64);
#endif
}
// Fr modulus and Montgomery constants (little-endian limbs).
\_\_device\_\_ \_\_constant\_\_ uint64\_t FR\_MODULUS\_DEVICE[4] = {
0xffffffff00000001ULL,
0x53bda402fffe5bfeULL,
0x3339d80809a1d805ULL,
0x73eda753299d7d48ULL,
};
\_\_device\_\_ \_\_constant\_\_ uint64\_t FR\_R2\_DEVICE[4] = {
0xc999e990f3f29c6dULL,
0x2b6cedcb87925c23ULL,
0x05d314967254398fULL,
0x0748d9d99f59ff11ULL,
};
\_\_device\_\_ \_\_constant\_\_ uint64\_t FR\_INV\_DEVICE = 0xfffffffeffffffffULL;
static constexpr uint64\_t FR\_MODULUS\_HOST[4] = {
0xffffffff00000001ULL,
0x53bda402fffe5bfeULL,
0x3339d80809a1d805ULL,
0x73eda753299d7d48ULL,
};
static constexpr uint64\_t FR\_R2\_HOST[4] = {
0xc999e990f3f29c6dULL,
0x2b6cedcb87925c23ULL,
0x05d314967254398fULL,
0x0748d9d99f59ff11ULL,
};
static constexpr uint64\_t FR\_INV\_HOST = 0xfffffffeffffffffULL;
\_\_device\_\_ \_\_constant\_\_ uint64\_t FR\_TWO\_256\_DEVICE[4] = {
0x00000001fffffffeULL,
0x5884b7fa00034802ULL,
0x998c4fefecbc4ff5ULL,
0x1824b159acc5056fULL,
};
static constexpr uint64\_t FR\_TWO\_256\_HOST[4] = {
0x00000001fffffffeULL,
0x5884b7fa00034802ULL,
0x998c4fefecbc4ff5ULL,
0x1824b159acc5056fULL,
};
// Fp modulus and Montgomery constants (little-endian limbs).
\_\_device\_\_ \_\_constant\_\_ uint64\_t FP\_MODULUS\_DEVICE[6] = {
0xb9feffffffffaaabULL,
0x1eabfffeb153ffffULL,
0x6730d2a0f6b0f624ULL,
0x64774b84f38512bfULL,
0x4b1ba7b6434bacd7ULL,
0x1a0111ea397fe69aULL,
};
\_\_device\_\_ \_\_constant\_\_ uint64\_t FP\_R2\_DEVICE[6] = {
0xf4df1f341c341746ULL,
0x0a76e6a609d104f1ULL,
0x8de5476c4c95b6d5ULL,
0x67eb88a9939d83c0ULL,
0x9a793e85b519952dULL,
0x11988fe592cae3aaULL,
};
\_\_device\_\_ \_\_constant\_\_ uint64\_t FP\_INV\_DEVICE = 0x89f3fffcfffcfffdULL;
static constexpr uint64\_t FP\_MODULUS\_HOST[6] = {
0xb9feffffffffaaabULL,
0x1eabfffeb153ffffULL,
0x6730d2a0f6b0f624ULL,
0x64774b84f38512bfULL,
0x4b1ba7b6434bacd7ULL,
0x1a0111ea397fe69aULL,
};
static constexpr uint64\_t FP\_R2\_HOST[6] = {
0xf4df1f341c341746ULL,
0x0a76e6a609d104f1ULL,
0x8de5476c4c95b6d5ULL,
0x67eb88a9939d83c0ULL,
0x9a793e85b519952dULL,
0x11988fe592cae3aaULL,
};
static constexpr uint64\_t FP\_INV\_HOST = 0x89f3fffcfffcfffdULL;
MIDNIGHT\_PLONK\_HD void fr\_add(Fr &out, const Fr &a, const Fr &b) {
uint64\_t carry = 0;
for (int i = 0; i < 4; ++i) {
out.limbs[i] = add\_carry(a.limbs[i], b.limbs[i], &carry);
}
// Conditional subtract modulus
uint64\_t borrow = 0;
Fr tmp;
for (int i = 0; i < 4; ++i) {
uint64\_t mod = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
mod = FR\_MODULUS\_DEVICE[i];
#else
mod = FR\_MODULUS\_HOST[i];
#endif
tmp.limbs[i] = sub\_borrow(out.limbs[i], mod, &borrow);
}
if (carry != 0 || borrow == 0) {
out = tmp;
}
}
MIDNIGHT\_PLONK\_HD void fr\_sub(Fr &out, const Fr &a, const Fr &b) {
uint64\_t borrow = 0;
for (int i = 0; i < 4; ++i) {
out.limbs[i] = sub\_borrow(a.limbs[i], b.limbs[i], &borrow);
}
if (borrow != 0) {
uint64\_t carry = 0;
for (int i = 0; i < 4; ++i) {
uint64\_t mod = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
mod = FR\_MODULUS\_DEVICE[i];
#else
mod = FR\_MODULUS\_HOST[i];
#endif
out.limbs[i] = add\_carry(out.limbs[i], mod, &carry);
}
}
}
MIDNIGHT\_PLONK\_HD void fr\_mont\_reduce(uint64\_t t[8], Fr &out) {
for (int i = 0; i < 4; ++i) {
uint64\_t inv = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
inv = FR\_INV\_DEVICE;
#else
inv = FR\_INV\_HOST;
#endif
uint64\_t u = t[i] \* inv;
uint64\_t carry = 0;
uint64\_t carry\_high = 0;
for (int j = 0; j < 4; ++j) {
uint64\_t hi, lo;
uint64\_t mod = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
mod = FR\_MODULUS\_DEVICE[j];
#else
mod = FR\_MODULUS\_HOST[j];
#endif
mul64(u, mod, &hi, &lo);
uint64\_t sum = t[i + j];
uint64\_t c1 = 0;
sum = add\_carry(sum, lo, &c1);
uint64\_t c2 = 0;
sum = add\_carry(sum, carry, &c2);
t[i + j] = sum;
uint64\_t carry\_out = c1 + c2;
if (carry\_high) {
++carry\_out;
}
uint64\_t new\_carry = hi + carry\_out;
uint64\_t new\_carry\_high = new\_carry < hi ? 1 : 0;
carry = new\_carry;
carry\_high = new\_carry\_high;
}
int idx = i + 4;
uint64\_t carry2 = carry\_high;
t[idx] = add\_carry(t[idx], carry, &carry2);
int k = idx + 1;
while (carry2 != 0 && k < 8) {
t[k] = add\_carry(t[k], 0, &carry2);
++k;
}
}
for (int i = 0; i < 4; ++i) {
out.limbs[i] = t[i + 4];
}
uint64\_t borrow = 0;
Fr tmp;
for (int i = 0; i < 4; ++i) {
uint64\_t mod = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
mod = FR\_MODULUS\_DEVICE[i];
#else
mod = FR\_MODULUS\_HOST[i];
#endif
tmp.limbs[i] = sub\_borrow(out.limbs[i], mod, &borrow);
}
if (borrow == 0) {
out = tmp;
}
}
MIDNIGHT\_PLONK\_HD void fr\_mul(Fr &out, const Fr &a, const Fr &b) {
uint64\_t t[8] = {0};
for (int i = 0; i < 4; ++i) {
uint64\_t carry = 0;
uint64\_t carry\_high = 0;
for (int j = 0; j < 4; ++j) {
uint64\_t hi, lo;
mul64(a.limbs[i], b.limbs[j], &hi, &lo);
uint64\_t sum = t[i + j];
uint64\_t c1 = 0;
sum = add\_carry(sum, lo, &c1);
uint64\_t c2 = 0;
sum = add\_carry(sum, carry, &c2);
t[i + j] = sum;
uint64\_t carry\_out = c1 + c2;
if (carry\_high) {
++carry\_out;
}
uint64\_t new\_carry = hi + carry\_out;
uint64\_t new\_carry\_high = new\_carry < hi ? 1 : 0;
carry = new\_carry;
carry\_high = new\_carry\_high;
}
int idx = i + 4;
uint64\_t c = carry\_high;
t[idx] = add\_carry(t[idx], carry, &c);
int k = idx + 1;
while (c != 0 && k < 8) {
t[k] = add\_carry(t[k], 0, &c);
++k;
}
}
fr\_mont\_reduce(t, out);
}
MIDNIGHT\_PLONK\_HD void fr\_from\_bytes(Fr &out, const uint8\_t \*bytes) {
for (int i = 0; i < 4; ++i) {
uint64\_t limb = 0;
for (int j = 0; j < 8; ++j) {
limb |= static\_cast(bytes[i \* 8 + j]) << (8 \* j);
}
out.limbs[i] = limb;
}
Fr r2;
for (int i = 0; i < 4; ++i) {
uint64\_t r2\_limb = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
r2\_limb = FR\_R2\_DEVICE[i];
#else
r2\_limb = FR\_R2\_HOST[i];
#endif
r2.limbs[i] = r2\_limb;
}
Fr tmp = out;
fr\_mul(out, tmp, r2);
}
MIDNIGHT\_PLONK\_HD void fr\_from\_uniform\_bytes(Fr &out, const uint8\_t \*bytes) {
Fr lo;
Fr hi;
fr\_from\_bytes(lo, bytes);
fr\_from\_bytes(hi, bytes + 32);
Fr r2;
for (int i = 0; i < 4; ++i) {
uint64\_t limb = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
limb = FR\_R2\_DEVICE[i];
#else
limb = FR\_R2\_HOST[i];
#endif
r2.limbs[i] = limb;
}
Fr hi\_scaled;
fr\_mul(hi\_scaled, hi, r2);
fr\_add(out, lo, hi\_scaled);
}
MIDNIGHT\_PLONK\_HD void fr\_to\_bytes(uint8\_t \*out\_bytes, const Fr &a) {
Fr one;
one.limbs[0] = 1;
one.limbs[1] = 0;
one.limbs[2] = 0;
one.limbs[3] = 0;
Fr tmp;
fr\_mul(tmp, a, one);
for (int i = 0; i < 4; ++i) {
uint64\_t limb = tmp.limbs[i];
for (int j = 0; j < 8; ++j) {
out\_bytes[i \* 8 + j] = static\_cast(limb >> (8 \* j));
}
}
}
MIDNIGHT\_PLONK\_HD void fr\_from\_mont(Fr &out, const Fr &a) {
Fr one;
one.limbs[0] = 1;
one.limbs[1] = 0;
one.limbs[2] = 0;
one.limbs[3] = 0;
fr\_mul(out, a, one);
}
MIDNIGHT\_PLONK\_HD void fp\_add(Fp &out, const Fp &a, const Fp &b) {
uint64\_t carry = 0;
for (int i = 0; i < 6; ++i) {
out.limbs[i] = add\_carry(a.limbs[i], b.limbs[i], &carry);
}
uint64\_t borrow = 0;
Fp tmp;
for (int i = 0; i < 6; ++i) {
uint64\_t mod = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
mod = FP\_MODULUS\_DEVICE[i];
#else
mod = FP\_MODULUS\_HOST[i];
#endif
tmp.limbs[i] = sub\_borrow(out.limbs[i], mod, &borrow);
}
if (carry != 0 || borrow == 0) {
out = tmp;
}
}
MIDNIGHT\_PLONK\_HD void fp\_sub(Fp &out, const Fp &a, const Fp &b) {
uint64\_t borrow = 0;
for (int i = 0; i < 6; ++i) {
out.limbs[i] = sub\_borrow(a.limbs[i], b.limbs[i], &borrow);
}
if (borrow != 0) {
uint64\_t carry = 0;
for (int i = 0; i < 6; ++i) {
uint64\_t mod = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
mod = FP\_MODULUS\_DEVICE[i];
#else
mod = FP\_MODULUS\_HOST[i];
#endif
out.limbs[i] = add\_carry(out.limbs[i], mod, &carry);
}
}
}
MIDNIGHT\_PLONK\_HD void fp\_mont\_reduce(uint64\_t t[12], Fp &out) {
for (int i = 0; i < 6; ++i) {
uint64\_t inv = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
inv = FP\_INV\_DEVICE;
#else
inv = FP\_INV\_HOST;
#endif
uint64\_t u = t[i] \* inv;
uint64\_t carry = 0;
uint64\_t carry\_high = 0;
for (int j = 0; j < 6; ++j) {
uint64\_t hi, lo;
uint64\_t mod = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
mod = FP\_MODULUS\_DEVICE[j];
#else
mod = FP\_MODULUS\_HOST[j];
#endif
mul64(u, mod, &hi, &lo);
uint64\_t sum = t[i + j];
uint64\_t c1 = 0;
sum = add\_carry(sum, lo, &c1);
uint64\_t c2 = 0;
sum = add\_carry(sum, carry, &c2);
t[i + j] = sum;
uint64\_t carry\_out = c1 + c2;
if (carry\_high) {
++carry\_out;
}
uint64\_t new\_carry = hi + carry\_out;
uint64\_t new\_carry\_high = new\_carry < hi ? 1 : 0;
carry = new\_carry;
carry\_high = new\_carry\_high;
}
int idx = i + 6;
uint64\_t carry2 = carry\_high;
t[idx] = add\_carry(t[idx], carry, &carry2);
int k = idx + 1;
while (carry2 != 0 && k < 12) {
t[k] = add\_carry(t[k], 0, &carry2);
++k;
}
}
for (int i = 0; i < 6; ++i) {
out.limbs[i] = t[i + 6];
}
uint64\_t borrow = 0;
Fp tmp;
for (int i = 0; i < 6; ++i) {
uint64\_t mod = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
mod = FP\_MODULUS\_DEVICE[i];
#else
mod = FP\_MODULUS\_HOST[i];
#endif
tmp.limbs[i] = sub\_borrow(out.limbs[i], mod, &borrow);
}
if (borrow == 0) {
out = tmp;
}
}
MIDNIGHT\_PLONK\_HD void fp\_mul(Fp &out, const Fp &a, const Fp &b) {
uint64\_t t[12] = {0};
for (int i = 0; i < 6; ++i) {
uint64\_t carry = 0;
uint64\_t carry\_high = 0;
for (int j = 0; j < 6; ++j) {
uint64\_t hi, lo;
mul64(a.limbs[i], b.limbs[j], &hi, &lo);
uint64\_t sum = t[i + j];
uint64\_t c1 = 0;
sum = add\_carry(sum, lo, &c1);
uint64\_t c2 = 0;
sum = add\_carry(sum, carry, &c2);
t[i + j] = sum;
uint64\_t carry\_out = c1 + c2;
if (carry\_high) {
++carry\_out;
}
uint64\_t new\_carry = hi + carry\_out;
uint64\_t new\_carry\_high = new\_carry < hi ? 1 : 0;
carry = new\_carry;
carry\_high = new\_carry\_high;
}
int idx = i + 6;
uint64\_t c = carry\_high;
t[idx] = add\_carry(t[idx], carry, &c);
int k = idx + 1;
while (c != 0 && k < 12) {
t[k] = add\_carry(t[k], 0, &c);
++k;
}
}
fp\_mont\_reduce(t, out);
}
MIDNIGHT\_PLONK\_HD void fp\_square(Fp &out, const Fp &a) {
fp\_mul(out, a, a);
}
MIDNIGHT\_PLONK\_HD Fp fp\_one() {
Fp one{};
uint8\_t bytes[48] = {0};
bytes[0] = 1;
fp\_from\_bytes(one, bytes);
return one;
}
MIDNIGHT\_PLONK\_HD void fp\_pow(Fp &out, const Fp &a, const uint64\_t exp[6]) {
Fp result = fp\_one();
Fp base = a;
for (int limb = 5; limb >= 0; --limb) {
for (int bit = 63; bit >= 0; --bit) {
fp\_square(result, result);
if ((exp[limb] >> bit) & 1ULL) {
fp\_mul(result, result, base);
}
}
}
out = result;
}
MIDNIGHT\_PLONK\_HD void fp\_inv(Fp &out, const Fp &a) {
const uint64\_t exp[6] = {
0xb9feffffffffaaa9ULL,
0x1eabfffeb153ffffULL,
0x6730d2a0f6b0f624ULL,
0x64774b84f38512bfULL,
0x4b1ba7b6434bacd7ULL,
0x1a0111ea397fe69aULL,
};
fp\_pow(out, a, exp);
}
MIDNIGHT\_PLONK\_HD void fp\_from\_bytes(Fp &out, const uint8\_t \*bytes) {
for (int i = 0; i < 6; ++i) {
uint64\_t limb = 0;
for (int j = 0; j < 8; ++j) {
limb |= static\_cast(bytes[i \* 8 + j]) << (8 \* j);
}
out.limbs[i] = limb;
}
Fp r2;
for (int i = 0; i < 6; ++i) {
uint64\_t r2\_limb = 0;
#if defined(\_\_CUDA\_ARCH\_\_)
r2\_limb = FP\_R2\_DEVICE[i];
#else
r2\_limb = FP\_R2\_HOST[i];
#endif
r2.limbs[i] = r2\_limb;
}
Fp tmp = out;
fp\_mul(out, tmp, r2);
}
MIDNIGHT\_PLONK\_HD void fp\_to\_bytes(uint8\_t \*out\_bytes, const Fp &a) {
Fp one;
one.limbs[0] = 1;
one.limbs[1] = 0;
one.limbs[2] = 0;
one.limbs[3] = 0;
one.limbs[4] = 0;
one.limbs[5] = 0;
Fp tmp;
fp\_mul(tmp, a, one);
for (int i = 0; i < 6; ++i) {
uint64\_t limb = tmp.limbs[i];
for (int j = 0; j < 8; ++j) {
out\_bytes[i \* 8 + j] = static\_cast(limb >> (8 \* j));
}
}
}
MIDNIGHT\_PLONK\_HD void fr\_square(Fr &out, const Fr &a) {
fr\_mul(out, a, a);
}
MIDNIGHT\_PLONK\_HD void fr\_pow(Fr &out, const Fr &a, const uint64\_t exp[4]) {
Fr result{};
uint8\_t one\_bytes[32] = {0};
one\_bytes[0] = 1;
fr\_from\_bytes(result, one\_bytes);
Fr base = a;
for (int limb = 3; limb >= 0; --limb) {
for (int bit = 63; bit >= 0; --bit) {
fr\_square(result, result);
if ((exp[limb] >> bit) & 1ULL) {
fr\_mul(result, result, base);
}
}
}
out = result;
}
MIDNIGHT\_PLONK\_HD void fr\_inv(Fr &out, const Fr &a) {
const uint64\_t exp[4] = {
0xfffffffeffffffffULL,
0x53bda402fffe5bfeULL,
0x3339d80809a1d805ULL,
0x73eda753299d7d48ULL,
};
fr\_pow(out, a, exp);
}
} // namespace midnight\_plonk