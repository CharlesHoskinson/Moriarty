# MCL API
> Source: https://raw.githubusercontent.com/herumi/mcl/master/api.md
> Collected: 2026-10-26
> Published: Unknown

# C/C++ API
## Minimum sample
A sample demonstrating BLS12-381 usage:
### C
[sample/pairing\_c.c](sample/pairing\_c.c)
```bash
cd mcl
make -j4
make bin/pairing\_c.exe && bin/pairing\_c.exe
```
### C++
[sample/pairing.cpp](sample/pairing.cpp)
```bash
cd mcl
make -j4
make bin/pairing.exe && bin/pairing.exe
```
## Header and libraries
### C
To use BLS12-381, include `` and link `libmcl.{a,so}`.
`384\_256` indicates that the maximum bit size of `Fp` is 384, and the size of `Fr` is 256.
### C++
Include `` and link `libmcl.{a,so}`.
## Notation
The elliptic curve equation is `E: y^2 = x^3 + b`.
- `Fp` : a finite field of prime order `p`, over which the curve is defined.
- `Fr` : a finite field of prime order `r`.
- `Fp2` : field extension over Fp of degree 2. Fp[i] / (i^2 + u).
- `Fp6` : field extension over Fp2 of degree 3. Fp2[v] / (v^3 - Xi) where Xi = i + xi\_a.
- `Fp12` : field extension over Fp6 of degree 2. Fp6[w] / (w^2 - v).
- `G1` : the cyclic subgroup of E(Fp).
- `G2` : the cyclic subgroup of E'(Fp^2), where E' is a twist of E.
- `GT` : the cyclic subgroup of Fp12.
- `G1`, `G2`, and `GT` have the order `r`.
The pairing e: G1 x G2 -> GT is the optimal ate pairing.
mcl treats `G1` and `G2` as additive groups and `GT` as a multiplicative group.
- `mclSize` ; `unsigned int` if WebAssembly, otherwise `size\_t`
- `mclInt` ; `int` if WebAssembly, otherwise `int64\_t`
### Curve Parameters
r = |G1| = |G2| = |GT|
curveType | b|u|xi\_a| r and p |
------------|-|-|-|------------------|
BN254 | 2|1|1|r = 0x2523648240000001ba344d8000000007ff9f800000000010a10000000000000d   
 p = 0x2523648240000001ba344d80000000086121000000000013a700000000000013 |
BN\_SNARK1|3|1|9|r = 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001   
 p = 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47|
BLS12\_381 | 4|1|1|r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001   
 p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab |
BLS12\_377|1|5|0|r = 0x12ab655e9a2ca55660b44d1e5c37b00159aa76fed00000010a11800000000001   
 p = 0x1ae3a4617c510eac63b05c06ca1493b1a22d9f300f5138f1ef3622fba094800170b5d44300000008508c00000000001
BN381 | 2|1|1|r = 0x240026400f3d82b2e42de125b00158405b710818ac000007e0042f008e3e00000000001080046200000000000000000d   
 p = 0x240026400f3d82b2e42de125b00158405b710818ac00000840046200950400000000001380052e000000000000000013 |
## C Structures
### `mclBnFp`
This is a struct for `Fp`. The value is stored in Montgomery representation.
### `mclBnFr`
This is a struct for `Fr`. The value is stored in Montgomery representation.
### `mclBnFp2`
This is a struct for `Fp2` with a member `mclBnFp d[2]`.
An element `x` of `Fp2` is represented as `x = d[0] + d[1] i` where `i^2 = -1`.
### `mclBnG1`
This is a struct for `G1` with three members `x`, `y`, `z` of type `mclBnFp`.
An element `P` of `G1` is represented as `P = [x:y:z]` in Jacobian coordinates.
### `mclBnG2`
This is a struct for `G2` with three members `x`, `y`, `z` of type `mclBnFp2`.
An element `Q` of `G2` is represented as `Q = [x:y:z]` in Jacobian coordinates.
### `mclBnGT`
This is a struct for `GT` with a member `mclBnFp d[12]`.
## C++ Structures
The namespace is `mcl::bn`. `` also provides the namespace `mcl::bls12`, which imports `mcl::bn`.
### `Fp`
This is a class for `Fp`.
### `Fr`
This is a class for `Fr`.
### `Fp2`
This is a struct for `Fp2` with members `a` and `b` of type `Fp`.
An element `x` of `Fp2` is represented as `x = a + b i` where `i^2 = -1`.
### `Fp6`
This is a struct for `Fp6` with members `a`, `b`, and `c` of type `Fp2`.
An element `x` of `Fp6` is represented as `x = a + b v + c v^2` where `v^3 = xi := 1 + i`.
### `Fp12`
This is a struct for `Fp12` with members `a` and `b` of type `Fp6`.
An element `x` of `Fp12` is represented as `x = a + b w` where `w^2 = v`.
### `G1`
This is a struct for `G1` with three members `x`, `y`, `z` of type `Fp`.
An element `P` of `G1` is represented as `P = [x:y:z]` in Jacobian coordinates.
### `G2`
This is a struct for `G2` with three members `x`, `y`, `z` of type `Fp2`.
An element `Q` of `G2` is represented as `Q = [x:y:z]` in Jacobian coordinates.
### `GT`
`GT` is an alias for `Fp12`.
However, it represents the set `{ x in Fp12 | x^r = 1}`.
### Parameter Settings and Supported Curves
By default, `MCL\_FP\_BIT=384` and `MCL\_FR\_BIT=256` are set.
If you want to change these values, use commands such as `make MCL\_FP\_BIT=256 MCL\_FR\_BIT=256`.
You must set the same macro and value to compile your application program.
MCL\_FP\_BIT|MCL\_FR\_BIT|sizeof Fr|sizeof Fp|supported curves
-|-|-|-|-
256|256|32|32|MCL\_BN254, MCL\_BN\_SNARK1
384|256|48|32|+MCL\_BLS12\_381, MCL\_BLS12\_377
384|384|48|48|+MCL\_BN381\_1
## Thread safety
All functions except initialization and global setting changes are thread-safe.
## Initialization
### C
Initialize the mcl library. Call this function first before calling other functions.
```c
int mclBn\_init(int curve, int compiledTimeVar);
```
- `curve` ; specify the curve type
- `compiledTimeVar` ; set `MCLBN\_COMPILED\_TIME\_VAR`, a macro used to ensure that
the values are consistent between library build time and usage.
- return 0 on success.
- This is not thread safe.
### C++
```cpp
void initPairing(bool \*pb, const mcl::CurveParam& cp = mcl::BN254);
void initPairing(const mcl::CurveParam& cp = mcl::BN254); // throws an exception on error
```
The curve parameters are defined in `mcl/curve\_type.hpp`.
- mcl::BN254
- mcl::BN\_SNARK1
- mcl::BLS12\_381
- mcl::BLS12\_377
- mcl::BN381\_1
## Global setting
```c
int mclBn\_setMapToMode(int mode);
```
- `mode = MCL\_MAP\_TO\_MODE\_ORIGINAL` : the old hash-to-curve (for backward compatibility)
- `mode = MCL\_MAP\_TO\_MODE\_HASH\_TO\_CURVE` : the hash-to-curve defined in [RFC 9380 (Hashing to Elliptic Curves)](https://www.rfc-editor.org/rfc/rfc9380)
### Control to verify that a point of the elliptic curve has the order `r`.
This function affects `setStr()` and `deserialize()` for G1/G2.
### C
```c
void mclBn\_verifyOrderG1(int doVerify);
void mclBn\_verifyOrderG2(int doVerify);
```
### C++
```cpp
verifyOrderG1(bool doVerify);
verifyOrderG2(bool doVerify);
```
- verify if `doVerify` is 1, otherwise do not verify.
- The default values set by initialization are as follows: for BLS12 curves, both G1 and G2 are verified (a fast subgroup check is used). For BN curves, G1 is not verified (the cofactor is 1, so the check is unnecessary) and G2 is verified.
- Set `doVerify = 1` if protection against subgroup attacks is necessary.
- This is not thread-safe.
### Set DST for hash functions
### C
```c
int mclBnG1\_setDst(const char \*dst, mclSize dstSize);
int mclBnG2\_setDst(const char \*dst, mclSize dstSize);
```
return 0 if success else -1
### C++
```cpp
bool setDstG1(const char \*dst, size\_t dstSize);
bool setDstG2(const char \*dst, size\_t dstSize);
```
return true if success else false
- set `dst[0, dstSize)` to the DST for hash functions
- The default value:
- G1 : `BLS\_SIG\_BLS12381G1\_XMD:SHA-256\_SSWU\_RO\_POP\_`
- G2 : `BLS\_SIG\_BLS12381G2\_XMD:SHA-256\_SSWU\_RO\_POP\_`
## Setter / Getter
### Clear
Set `x` to zero.
```cpp
void mclBnFr\_clear(mclBnFr \*x);
void mclBnFp\_clear(mclBnFp \*x);
void mclBnFp2\_clear(mclBnFp2 \*x);
void mclBnG1\_clear(mclBnG1 \*x);
void mclBnG2\_clear(mclBnG2 \*x);
void mclBnGT\_clear(mclBnGT \*x);
```
C++
```c
T::clear();
```
### Set an integer `x` to `y`.
```c
void mclBnFp\_setInt(mclBnFp \*y, mclInt x);
void mclBnFr\_setInt(mclBnFr \*y, mclInt x);
void mclBnGT\_setInt(mclBnGT \*y, mclInt x);
```
C++
```cpp
T x = ;
```
### Set `bufSize` bytes from `buf` to `x` with masking according to the following method.
```c
int mclBnFp\_setLittleEndian(mclBnFp \*x, const void \*buf, mclSize bufSize);
int mclBnFr\_setLittleEndian(mclBnFr \*x, const void \*buf, mclSize bufSize);
```
C++
```cpp
T::setArrayMask(const uint8\_t \*buf, size\_t n);
```
1. set x = buf[0..bufSize-1] as little endian
2. x &= (1 << bitLen(r)) - 1
3. if (x >= r), then x &= (1 << (bitLen(r) - 1)) - 1
- always returns 0
### Set `bufSize` bytes from `buf` modulo `p` or `r` to `x`.
```c
int mclBnFp\_setLittleEndianMod(mclBnFp \*x, const void \*buf, mclSize bufSize);
int mclBnFr\_setLittleEndianMod(mclBnFr \*x, const void \*buf, mclSize bufSize);
int mclBnFp\_setBigEndianMod(mclBnFp \*x, const void \*buf, mclSize bufSize);
int mclBnFr\_setBigEndianMod(mclBnFr \*x, const void \*buf, mclSize bufSize);
```
C++
```cpp
void T::setLittleEndianMod(const uint8\_t \*buf, size\_t bufSize);
void T::setBigEndianMod(const uint8\_t \*buf, size\_t bufSize);
```
- return 0 if bufSize <= 64 (MCL\_MAX\_BUF\_BYTE\_SIZE), otherwise -1
### Get little-endian byte sequence in `buf` corresponding to `x`
```c
mclSize mclBnFr\_getLittleEndian(void \*buf, mclSize maxBufSize, const mclBnFr \*x);
mclSize mclBnFp\_getLittleEndian(void \*buf, mclSize maxBufSize, const mclBnFp \*x);
```
C++
```cpp
size\_t T::getLittleEndian(uint8\_t \*buf, size\_t maxBufSize) const
```
- write `x` to `buf` in little-endian format
- return the written size on success, otherwise 0
- NOTE: if `x` is zero, then `buf[0] = 0` and the function returns 1.
### Serialization
### Serialize
```c
mclSize mclBnFr\_serialize(void \*buf, mclSize maxBufSize, const mclBnFr \*x);
mclSize mclBnG1\_serialize(void \*buf, mclSize maxBufSize, const mclBnG1 \*x);
mclSize mclBnG2\_serialize(void \*buf, mclSize maxBufSize, const mclBnG2 \*x);
mclSize mclBnGT\_serialize(void \*buf, mclSize maxBufSize, const mclBnGT \*x);
mclSize mclBnFp\_serialize(void \*buf, mclSize maxBufSize, const mclBnFp \*x);
mclSize mclBnFp2\_serialize(void \*buf, mclSize maxBufSize, const mclBnFp2 \*x);
```
C++
```cpp
mclSize T::serialize(void \*buf, mclSize maxBufSize) const;
```
- serialize `x` into `buf[0..maxBufSize-1]`
- return written byte size on success, otherwise 0
### Serialization Format
- `Fp` (resp. `Fr`) ; a little-endian byte sequence with a fixed size
- the size is the return value of `mclBn\_getFpByteSize()` (resp. `mclBn\_getFrByteSize()`).
- `G1` ; compressed with a fixed size
- the size is equal to `mclBn\_getG1ByteSize()` (=`mclBn\_getFpByteSize()`).
- `G2` ; compressed with a fixed size
- the size is equal to `mclBn\_getG2ByteSize()`.
A pseudo-code to serialize `P` of `G1` (resp. `G2`):
```python
size = mclBn\_getG1ByteSize() # resp. mclBn\_getG2ByteSize()
if P is zero:
return [0] \* size
else:
P = P.normalize()
s = P.x.serialize()
# x in Fp2 is odd <=> x.a is odd
if Fp is fullBit:
s = [P.y is odd ? 3 : 2] + s # prepend one byte
elif P.y is odd: # resp. P.y.d[0] is odd
s[byte-length(s) - 1] |= 0x80
return s
```
### Ethereum serialization mode for BLS12-381
```c
void mclBn\_setETHserialization(int ETHserialization);
```
- Set `ETHserialization = 1` for Ethereum compatibility (default 0).
- See [FAQ](api.md#faq).
### Deserialize
```c
mclSize mclBnFr\_deserialize(mclBnFr \*x, const void \*buf, mclSize bufSize);
mclSize mclBnG1\_deserialize(mclBnG1 \*x, const void \*buf, mclSize bufSize);
mclSize mclBnG2\_deserialize(mclBnG2 \*x, const void \*buf, mclSize bufSize);
mclSize mclBnGT\_deserialize(mclBnGT \*x, const void \*buf, mclSize bufSize);
mclSize mclBnFp\_deserialize(mclBnFp \*x, const void \*buf, mclSize bufSize);
mclSize mclBnFp2\_deserialize(mclBnFp2 \*x, const void \*buf, mclSize bufSize);
```
C++
```cpp
mclSize T::deserialize(const void \*buf, mclSize bufSize);
```
- deserialize `x` from `buf[0..bufSize-1]`
- return read size on success, otherwise 0
- mclBnG1\_deserialize and mclBnG2\_deserialize check whether the point has the correct order of G1/G2.
- mclBnGT\_deserialize does not check it. Call mclBnGT\_isValid if necessary.
## String conversion
### Get string
```c
mclSize mclBnFr\_getStr(char \*buf, mclSize maxBufSize, const mclBnFr \*x, int ioMode);
mclSize mclBnG1\_getStr(char \*buf, mclSize maxBufSize, const mclBnG1 \*x, int ioMode);
mclSize mclBnG2\_getStr(char \*buf, mclSize maxBufSize, const mclBnG2 \*x, int ioMode);
mclSize mclBnGT\_getStr(char \*buf, mclSize maxBufSize, const mclBnGT \*x, int ioMode);
mclSize mclBnFp\_getStr(char \*buf, mclSize maxBufSize, const mclBnFp \*x, int ioMode);
```
C++
```cpp
size\_t T::getStr(char \*buf, size\_t maxBufSize, int iMode = 0) const
```
- write `x` to `buf` according to `ioMode`
- `ioMode`
- 2 ; binary number
- 10 ; decimal number
- 16 ; hexadecimal number
- `MCLBN\_IO\_EC\_PROJ` ; output in Jacobian coordinates
- return `strlen(buf)` on success, otherwise 0.
The meaning of the output of `G1`:
- `0` ; infinity
- `1 ` ; affine coordinate
- `4 ` ; Jacobian coordinates
- the element `` of `G2` outputs `d[0] d[1]`.
### Set string
```c
int mclBnFr\_setStr(mclBnFr \*x, const char \*buf, mclSize bufSize, int ioMode);
int mclBnG1\_setStr(mclBnG1 \*x, const char \*buf, mclSize bufSize, int ioMode);
int mclBnG2\_setStr(mclBnG2 \*x, const char \*buf, mclSize bufSize, int ioMode);
int mclBnGT\_setStr(mclBnGT \*x, const char \*buf, mclSize bufSize, int ioMode);
int mclBnFp\_setStr(mclBnFp \*x, const char \*buf, mclSize bufSize, int ioMode);
```
C++
```cpp
void T::setStr(bool \*pb, const char \*str, int iMode = 0)
void T::setStr(const char \*str, int iMode = 0)
```
- set `buf[0..bufSize-1]` to `x` according to `ioMode`
- mask and truncate the value if it is greater than (r or p).
- See [masking](api.md#set-bufsize-bytes-from-buf-to-x-with-masking-according-to-the-following-method)
- A `bufSize` that is too large is rejected. The maximum length depends on compile options, but it is at least the bit length of the type of `x`.
- Setting a string for G1/G2 fails if the point is not on the elliptic curve.
- By default, it also checks that the point has the correct order of G1/G2.
- You can disable this check with `mclBn\_verifyOrderG1(0)` / `mclBn\_verifyOrderG2(0)`.
- return 0 if success else -1
- \*pb = result of setStr or throw exception if error (C++)
- mclBnG1\_setStr and mclBnG2\_setStr check whether the point has the correct order of G1/G2.
- mclBnGT\_setStr does not check it. Call mclBnGT\_isValid if necessary.
If you want to use the same BLS12-381 generator as [zkcrypto](https://www.ietf.org/archive/id/draft-irtf-cfrg-pairing-friendly-curves-11.html#section-4.2.1) then,
```c
mclBnG1 P;
const char \*g1Str = "1 0x17f1d3a73197d7942695638c4fa9ac0fc3688c4f9774b905a14e3a3f171bac586c55e83ff97a1aeffb3af00adb22c6bb 0x08b3f481e3aaa0f1a09e30ed741d8ae4fcf5e095d5d00af600db18cb2c04b3edd03cc744a2888ae40caa232946c5e7e1";
mclBnG1\_setStr(&P, g1Str, strlen(g1Str), 16);
mclBnG2 Q;
const char \*g2Str = "1 0x24aa2b2f08f0a91260805272dc51051c6e47ad4fa403b02b4510b647ae3d1770bac0326a805bbefd48056c8c121bdb8 0x13e02b6052719f607dacd3a088274f65596bd0d09920b61ab5da61bbdc7f5049334cf11213945d57e5ac7d055d042b7e 0x0ce5d527727d6e118cc9cdc6da2e351aadfd9baa8cbdd3a76d429a695160d12c923ac9cc3baca289e193548608b82801 0x0606c4a02ea734cc32acd2b02bc28b99cb3e287e85a763af267492ab572e99ab3f370d275cec1da1aaa9075ff05f79be";
mclBnG2\_setStr(&Q, g2Str, strlen(g2Str), 16);
```
## Set a random value
Set `x` by a cryptographically secure pseudo-random number generator.
```c
int mclBnFr\_setByCSPRNG(mclBnFr \*x);
int mclBnFp\_setByCSPRNG(mclBnFp \*x);
```
C++
```cpp
void T::setByCSPRNG()
```
### Change random generator function
```c
void mclBn\_setRandFunc(
void \*self,
unsigned int (\*readFunc)(void \*self, void \*buf, unsigned int bufSize)
);
```
- `self` ; user-defined pointer
- `readFunc` ; user-defined function, which writes random `bufSize` bytes to `buf` and returns `bufSize` if success else returns 0.
- `readFunc` must be thread-safe.
- Set the default random function if `self == 0` and `readFunc == 0`.
- This is not thread-safe.
## Arithmetic operations
### neg / inv / sqr / add / sub / mul / div of `Fr`, `Fp`, `Fp2`, `GT`.
```c
void mclBnFr\_neg(mclBnFr \*y, const mclBnFr \*x);
void mclBnFr\_inv(mclBnFr \*y, const mclBnFr \*x);
// y[i] = 1/x[i] if x[i] != 0 else 0
// faster than normalizing each one individually
void mclBnFr\_invVec(mclBnFr \*y, const mclBnFr \*x, mclSize n);
void mclBnFr\_sqr(mclBnFr \*y, const mclBnFr \*x);
void mclBnFr\_add(mclBnFr \*z, const mclBnFr \*x, const mclBnFr \*y);
void mclBnFr\_sub(mclBnFr \*z, const mclBnFr \*x, const mclBnFr \*y);
void mclBnFr\_mul(mclBnFr \*z, const mclBnFr \*x, const mclBnFr \*y);
void mclBnFr\_div(mclBnFr \*z, const mclBnFr \*x, const mclBnFr \*y);
// z = x \* y for a small integer y (faster than mclBnFr\_mul with the converted y)
void mclBnFr\_mulUnit(mclBnFr \*z, const mclBnFr \*x, uint32\_t y);
void mclBnFp\_neg(mclBnFp \*y, const mclBnFp \*x);
void mclBnFp\_inv(mclBnFp \*y, const mclBnFp \*x);
void mclBnFp\_invVec(mclBnFp \*y, const mclBnFp \*x, mclSize n);
void mclBnFp\_sqr(mclBnFp \*y, const mclBnFp \*x);
void mclBnFp\_add(mclBnFp \*z, const mclBnFp \*x, const mclBnFp \*y);
void mclBnFp\_sub(mclBnFp \*z, const mclBnFp \*x, const mclBnFp \*y);
void mclBnFp\_mul(mclBnFp \*z, const mclBnFp \*x, const mclBnFp \*y);
void mclBnFp\_div(mclBnFp \*z, const mclBnFp \*x, const mclBnFp \*y);
void mclBnFp\_mulUnit(mclBnFp \*z, const mclBnFp \*x, uint32\_t y);
void mclBnFp2\_neg(mclBnFp2 \*y, const mclBnFp2 \*x);
void mclBnFp2\_inv(mclBnFp2 \*y, const mclBnFp2 \*x);
void mclBnFp2\_sqr(mclBnFp2 \*y, const mclBnFp2 \*x);
void mclBnFp2\_add(mclBnFp2 \*z, const mclBnFp2 \*x, const mclBnFp2 \*y);
void mclBnFp2\_sub(mclBnFp2 \*z, const mclBnFp2 \*x, const mclBnFp2 \*y);
void mclBnFp2\_mul(mclBnFp2 \*z, const mclBnFp2 \*x, const mclBnFp2 \*y);
void mclBnFp2\_div(mclBnFp2 \*z, const mclBnFp2 \*x, const mclBnFp2 \*y);
void mclBnGT\_inv(mclBnGT \*y, const mclBnGT \*x); // y = a - bw for x = a + bw where Fp12 = Fp6[w]
void mclBnGT\_sqr(mclBnGT \*y, const mclBnGT \*x);
void mclBnGT\_mul(mclBnGT \*z, const mclBnGT \*x, const mclBnGT \*y);
void mclBnGT\_div(mclBnGT \*z, const mclBnGT \*x, const mclBnGT \*y);
```
- use `mclBnGT\_invGeneric` for an element in Fp12 - GT.
- NOTE: The result of the following functions is not an element of GT because GT is a multiplicative group.
```c
void mclBnGT\_neg(mclBnGT \*y, const mclBnGT \*x);
void mclBnGT\_add(mclBnGT \*z, const mclBnGT \*x, const mclBnGT \*y);
void mclBnGT\_sub(mclBnGT \*z, const mclBnGT \*x, const mclBnGT \*y);
```
C++
- `+`, `-`, `\*`, `/`
```cpp
T::add(T& z, const T& x, const T& y);
T::sub(T& z, const T& x, const T& y);
T::mul(T& z, const T& x, const T& y);
T::div(T& z, const T& x, const T& y);
T::neg(T& y, const T& x);
T::inv(T& y, const T& x);
// y[i] = 1/x[i] if x[i] != 0 else 0
mcl::invVec(T y[], const T x[], size\_t n);
```
### pow of `Fr`, `Fp`
- `pow(z, x, y)` means `z = x^y`.
- `powArray(z, x, y, ySize)` means z = x^y where y is a little endian `ySize`-byte sequence.
```c
// z = x^y
void mclBnFp\_pow(mclBnFp \*z, const mclBnFp \*x, const mclBnFp \*y);
void mclBnFr\_pow(mclBnFr \*z, const mclBnFr \*x, const mclBnFr \*y);
// return 0 if ySize <= mclBn\_getFrByteSize() else -1
int mclBnFr\_powArray(mclBnFr \*z, const mclBnFr \*x, const uint8\_t \*y, mclSize ySize);
// return 0 if ySize <= mclBn\_getFpByteSize() else -1
int mclBnFp\_powArray(mclBnFp \*z, const mclBnFp \*x, const uint8\_t \*y, mclSize ySize);
```
### Square root of `x`.
```c
int mclBnFr\_squareRoot(mclBnFr \*y, const mclBnFr \*x);
int mclBnFp\_squareRoot(mclBnFp \*y, const mclBnFp \*x);
int mclBnFp2\_squareRoot(mclBnFp2 \*y, const mclBnFp2 \*x);
```
C++
```cpp
bool T::squareRoot(T& y, const T& x);
```
- `y` is one of the square roots of `x` if it exists.
- return 0 if success else -1
### add / sub / dbl / neg for `G1` and `G2`.
```c
void mclBnG1\_neg(mclBnG1 \*y, const mclBnG1 \*x);
void mclBnG1\_dbl(mclBnG1 \*y, const mclBnG1 \*x);
void mclBnG1\_add(mclBnG1 \*z, const mclBnG1 \*x, const mclBnG1 \*y);
void mclBnG1\_sub(mclBnG1 \*z, const mclBnG1 \*x, const mclBnG1 \*y);
void mclBnG2\_neg(mclBnG2 \*y, const mclBnG2 \*x);
void mclBnG2\_dbl(mclBnG2 \*y, const mclBnG2 \*x);
void mclBnG2\_add(mclBnG2 \*z, const mclBnG2 \*x, const mclBnG2 \*y);
void mclBnG2\_sub(mclBnG2 \*z, const mclBnG2 \*x, const mclBnG2 \*y);
```
C++
- `+`, `-`
- T::add(T& z, const T& x, const T& y);
- T::sub(T& z, const T& x, const T& y);
- T::neg(T& y, const T& x);
### Convert a point from Jacobian/projective coordinates to affine.
```c
void mclBnG1\_normalize(mclBnG1 \*y, const mclBnG1 \*x);
void mclBnG2\_normalize(mclBnG2 \*y, const mclBnG2 \*x);
// y[i] = normalize(x[i]) for i = 0, 1, ..., n-1
// faster than normalizing each one individually
void mclBnG1\_normalizeVec(mclBnG1 \*y, const mclBnG1 \*x, mclSize n);
void mclBnG2\_normalizeVec(mclBnG2 \*y, const mclBnG2 \*x, mclSize n);
```
C++
```cpp
T::normalize(T& y, const T& x);
T::normalizeVec(T \*y, const T \*x, size\_t n);
```
- convert `[x:y:z]` to `[x:y:1]` if `z != 0` else `[\*:\*:0]`
### scalar multiplication
```c
void mclBnG1\_mul(mclBnG1 \*z, const mclBnG1 \*x, const mclBnFr \*y);
void mclBnG2\_mul(mclBnG2 \*z, const mclBnG2 \*x, const mclBnFr \*y);
void mclBnGT\_pow(mclBnGT \*z, const mclBnGT \*x, const mclBnFr \*y);
```
C++
```cpp
T::mul(T& z, const T& x, const Fr& y);
```
- z = x \* y for G1 / G2
- z = pow(x, y) for GT
- use `mclBnGT\_powGeneric` for an element in Fp12 - GT.
### multi-scalar multiplication
```c
void mclBnG1\_mulVec(mclBnG1 \*z, mclBnG1 \*x, const mclBnFr \*y, mclSize n);
void mclBnG2\_mulVec(mclBnG2 \*z, mclBnG2 \*x, const mclBnFr \*y, mclSize n);
void mclBnGT\_powVec(mclBnGT \*z, const mclBnGT \*x, const mclBnFr \*y, mclSize n);
```
C++
```cpp
T::mulVec(T& z, T\* x, const Fr \*y, size\_t n);
```
- z = sum\_{i=0}^{n-1} mul(x[i], y[i]) for G1 / G2.
- z = prod\_{i=0}^{n-1} pow(x[i], y[i]) for GT.
- `x[]` is not const because the points may be normalized (the values they represent do not change).
### multi-threaded multi-scalar multiplication
```c
void mclBnG1\_mulVecMT(mclBnG1 \*z, mclBnG1 \*x, const mclBnFr \*y, mclSize n, mclSize cpuN);
void mclBnG2\_mulVecMT(mclBnG2 \*z, mclBnG2 \*x, const mclBnFr \*y, mclSize n, mclSize cpuN);
```
- multi-threaded version of `mulVec` with `cpuN` threads
- enabled if the library is built with `MCL\_USE\_OMP=1`
### scalar multiplication of each point
```c
void mclBnG1\_mulEach(mclBnG1 \*x, const mclBnFr \*y, mclSize n);
```
C++
```cpp
G1::mulEach(G1 \*xVec, const Fr \*yVec, size\_t n);
```
- xVec[i] \*= yVec[i]
- `G1::mulVec` and `G1::mulEach` for BLS12-381 use AVX-512 IFMA if possible
## hash-to-curve function
### Set hash of `buf[0..bufSize-1]` to `x`
```c
int mclBnFr\_setHashOf(mclBnFr \*x, const void \*buf, mclSize bufSize);
int mclBnFp\_setHashOf(mclBnFp \*x, const void \*buf, mclSize bufSize);
```
C++
```cpp
T::setHashOf(const void \*msg, size\_t msgSize);
```
- always returns 0
- uses SHA-256 if the bit size of the field is not greater than 256, otherwise SHA-512
- sets the value in the same way as `setLittleEndian`.
- This function is for backward compatibility only. DON'T use it. Instead, apply `setLittleEndianMod` to the hashed value.
### map `x` to G1 / G2.
```c
int mclBnFp\_mapToG1(mclBnG1 \*y, const mclBnFp \*x);
int mclBnFp2\_mapToG2(mclBnG2 \*y, const mclBnFp2 \*x);
```
C++
```cpp
void mapToG1(G1& P, const Fp& x);
void mapToG2(G2& P, const Fp2& x);
```
- See `struct MapTo` in `src/map\_impl.hpp` for details of the algorithm.
- return 0 if success else -1
If you want to use the MapTo function defined in [RFC 9380 (Hashing to Elliptic Curves)](https://www.rfc-editor.org/rfc/rfc9380), then use `mclBn\_setMapToMode`.
```c
mclBn\_setMapToMode(MCL\_MAP\_TO\_MODE\_HASH\_TO\_CURVE);
```
And if you want to change DST (domain separation tag), then use `setDst` functions.
```c
int mclBnG1\_setDst(const char \*dst, mclSize dstSize);
int mclBnG2\_setDst(const char \*dst, mclSize dstSize);
```
### hash and map to G1 / G2.
```c
int mclBnG1\_hashAndMapTo(mclBnG1 \*x, const void \*buf, mclSize bufSize);
int mclBnG2\_hashAndMapTo(mclBnG2 \*x, const void \*buf, mclSize bufSize);
```
C++
```cpp
void hashAndMapToG1(G1& P, const void \*buf, size\_t bufSize);
void hashAndMapToG2(G2& P, const void \*buf, size\_t bufSize);
```
- Combines the `setHashOf` and `mapTo` functions
## Pairing operations
The pairing function `e(P, Q)` consists of two parts:
- `MillerLoop(P, Q)`
- `finalExp(x)`
`finalExp` satisfies the following properties:
- `e(P, Q) = finalExp(MillerLoop(P, Q))`
- `e(P1, Q1) e(P2, Q2) = finalExp(MillerLoop(P1, Q1) MillerLoop(P2, Q2))`
### pairing
```c
void mclBn\_pairing(mclBnGT \*z, const mclBnG1 \*x, const mclBnG2 \*y);
```
C++
```cpp
void pairing(GT& z, const G1& x, const G2& y);
```
### millerLoop
```c
void mclBn\_millerLoop(mclBnGT \*z, const mclBnG1 \*x, const mclBnG2 \*y);
```
C++
```cpp
void millerLoop(GT& z, const G1& x, const G2& y);
```
### finalExp
```c
void mclBn\_finalExp(mclBnGT \*y, const mclBnGT \*x);
```
C++
```cpp
void finalExp(GT& y, const GT& x);
```
## Variants of MillerLoop
### multi pairing
```c
void mclBn\_millerLoopVec(mclBnGT \*z, const mclBnG1 \*x, const mclBnG2 \*y, mclSize n);
```
C++
```cpp
void millerLoopVec(GT& z, const G1 \*x, const G2 \*y, size\_t n);
```
- This function is for multi-pairing
- computes prod\_{i=0}^{n-1} MillerLoop(x[i], y[i])
- prod\_{i=0}^{n-1} e(x[i], y[i]) = finalExp(prod\_{i=0}^{n-1} MillerLoop(x[i], y[i]))
```c
void mclBn\_millerLoopVecMT(mclBnGT \*z, const mclBnG1 \*x, const mclBnG2 \*y, mclSize n, mclSize cpuN);
```
- multi-threaded version of `mclBn\_millerLoopVec` with `cpuN` threads
- enabled if the library is built with `MCL\_USE\_OMP=1`
### pairing for a fixed point of G2
```c
int mclBn\_getUint64NumToPrecompute(void);
void mclBn\_precomputeG2(uint64\_t \*Qbuf, const mclBnG2 \*Q);
void mclBn\_precomputedMillerLoop(mclBnGT \*f, const mclBnG1 \*P, const uint64\_t \*Qbuf);
```
These functions compute the same value as `pairing(P, Q)` as follows:
```c
uint64\_t \*Qbuf = (uint64\_t\*)malloc(mclBn\_getUint64NumToPrecompute() \* sizeof(uint64\_t));
mclBn\_precomputeG2(Qbuf, Q); // precomputing of Q
mclBn\_precomputedMillerLoop(f, P, Qbuf); // pairing of any P of G1 and the fixed Q
free(Qbuf);
```
```c
void mclBn\_precomputedMillerLoop2(
mclBnGT \*f,
const mclBnG1 \*P1, const uint64\_t \*Q1buf,
const mclBnG1 \*P2, const uint64\_t \*Q2buf
);
```
- compute `MillerLoop(P1, Q1buf) \* MillerLoop(P2, Q2buf)`
```c
void mclBn\_precomputedMillerLoop2mixed(
mclBnGT \*f,
const mclBnG1 \*P1, const mclBnG2 \*Q1,
const mclBnG1 \*P2, const uint64\_t \*Q2buf
);
```
- compute `MillerLoop(P1, Q1) \* MillerLoop(P2, Q2buf)`
## Check value
### Check validity
```c
int mclBnFr\_isValid(const mclBnFr \*x);
int mclBnFp\_isValid(const mclBnFp \*x);
int mclBnG1\_isValid(const mclBnG1 \*x);
int mclBnG2\_isValid(const mclBnG2 \*x);
```
C++
```cpp
bool T::isValid() const;
```
- return 1 if true else 0
### Check the order of a point
```c
int mclBnG1\_isValidOrder(const mclBnG1 \*x);
int mclBnG2\_isValidOrder(const mclBnG2 \*x);
```
C++
```cpp
bool T::isValidOrder() const;
```
- Check whether the order of `x` is valid or not
- return 1 if true else 0
- This function always performs the check regardless of the settings of `mclBn\_verifyOrderG1` and `mclBn\_verifyOrderG2`.
### Is equal / zero / one / isOdd
```c
int mclBnFr\_isEqual(const mclBnFr \*x, const mclBnFr \*y);
int mclBnFr\_isZero(const mclBnFr \*x);
int mclBnFr\_isOne(const mclBnFr \*x);
int mclBnFr\_isOdd(const mclBnFr \*x);
int mclBnFp\_isEqual(const mclBnFp \*x, const mclBnFp \*y);
int mclBnFp\_isZero(const mclBnFp \*x);
int mclBnFp\_isOne(const mclBnFp \*x);
int mclBnFp\_isOdd(const mclBnFp \*x);
int mclBnFp2\_isEqual(const mclBnFp2 \*x, const mclBnFp2 \*y);
int mclBnFp2\_isZero(const mclBnFp2 \*x);
int mclBnFp2\_isOne(const mclBnFp2 \*x);
int mclBnG1\_isEqual(const mclBnG1 \*x, const mclBnG1 \*y);
int mclBnG1\_isZero(const mclBnG1 \*x);
int mclBnG2\_isEqual(const mclBnG2 \*x, const mclBnG2 \*y);
int mclBnG2\_isZero(const mclBnG2 \*x);
int mclBnGT\_isEqual(const mclBnGT \*x, const mclBnGT \*y);
int mclBnGT\_isZero(const mclBnGT \*x);
int mclBnGT\_isOne(const mclBnGT \*x);
```
C++
```cpp
bool T::operator==(const T& rhs) const;
bool T::isZero() const;
bool T::isOne() const;
```
- return 1 (true) if true else 0 (false)
### isNegative
```c
int mclBnFr\_isNegative(const mclBnFr \*x);
int mclBnFp\_isNegative(const mclBnFp \*x);
```
return 1 if x >= half where half = (r + 1) / 2 (resp. (p + 1) / 2).
### Compare
```c
int mclBnFr\_cmp(const mclBnFr \*x, const mclBnFr \*y);
int mclBnFp\_cmp(const mclBnFp \*x, const mclBnFp \*y);
```
- compare `x` and `y` as unsigned integers
- return -1 if x < y, 0 if x == y, 1 if x > y
- NOTE: two Montgomery conversions may be required
## Lagrange interpolation
```c
int mclBn\_FrLagrangeInterpolation(mclBnFr \*out, const mclBnFr \*xVec, const mclBnFr \*yVec, mclSize k);
int mclBn\_G1LagrangeInterpolation(mclBnG1 \*out, const mclBnFr \*xVec, const mclBnG1 \*yVec, mclSize k);
int mclBn\_G2LagrangeInterpolation(mclBnG2 \*out, const mclBnFr \*xVec, const mclBnG2 \*yVec, mclSize k);
```
- Lagrange interpolation
- recover out = y(0) from {(xVec[i], yVec[i])} for {i=0..k-1}
- return 0 if success else -1
- requires that xVec[i] != 0 and xVec[i] != xVec[j] for i != j
```c
int mclBn\_FrEvaluatePolynomial(mclBnFr \*out, const mclBnFr \*cVec, mclSize cSize, const mclBnFr \*x);
int mclBn\_G1EvaluatePolynomial(mclBnG1 \*out, const mclBnG1 \*cVec, mclSize cSize, const mclBnFr \*x);
int mclBn\_G2EvaluatePolynomial(mclBnG2 \*out, const mclBnG2 \*cVec, mclSize cSize, const mclBnFr \*x);
```
- Evaluate polynomial
- out = f(x) = c[0] + c[1] \* x + ... + c[cSize - 1] \* x^{cSize - 1}
- return 0 if success else -1
- requires cSize >= 1
## FAQ
### Why is the value set by Fp::setStr different?
The value set by Fp::setStr is masked and truncated if it is greater than p (resp. r).
See [Set string](api.md#set-string)
### What parameters of configuration are for Ethereum?
mcl supports various modes of the hash-to-curve function, serialize/deserialize, and getStr/setStr
for historical reasons and backward compatibility.
If using BLS12-381 and Ethereum compatibility mode, set
```cpp
// C++
Fp::setETHserialization(true);
Fr::setETHserialization(true);
bn::setMapToMode(MCL\_MAP\_TO\_MODE\_HASH\_TO\_CURVE);
```
or
```c
// C
mclBn\_setETHserialization(1);
mclBn\_setMapToMode(MCL\_MAP\_TO\_MODE\_HASH\_TO\_CURVE);
```
and use
```cpp
// C++
void Fp::setBigEndianMod(const uint8\_t \*x, size\_t bufSize);
size\_t T::serialize(void \*buf, size\_t maxBufSize) const
size\_t T::deserialize(const void \*buf, size\_t bufSize);
```
or
```
// C
int mclBnFp\_setBigEndianMod(mclBnFp \*x, const void \*buf, mclSize bufSize);
mclSize mclBnFp\_serialize(void \*buf, mclSize maxBufSize, const mclBnFp \*x);
mclSize mclBnFp\_deserialize(mclBnFp \*x, const void \*buf, mclSize bufSize);
```
Serialization of Fp/Fr
- Fp
- 48 bytes data in big-endian format
- Fr
- 32 bytes data in big-endian format
- G1
- zero : `[0xc0 : (47 bytes zero)]`
- (x, y) : `d = [48 bytes x]`, then `d[0] |= 0x80`, and `d[0] |= 0x20` if `y >= (p+1)/2`.
- G2
- zero : `[0xc0 : (95 bytes zero)]`
- (x, y) : `d = [96 bytes x]`, then `d[0] |= 0x80`, and `d[0] |= 0x20` if `b >= (p+1)/2` where `y = a + bi`.
See [Point Serialization Procedure](https://www.ietf.org/archive/id/draft-irtf-cfrg-pairing-friendly-curves-11.html#name-point-serialization-procedu) for details.