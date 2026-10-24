/\*
\* Copyright Supranational LLC
\* Licensed under the Apache License, Version 2.0, see LICENSE for details.
\* SPDX-License-Identifier: Apache-2.0
\*/
#ifndef \_\_BLST\_H\_\_
#define \_\_BLST\_H\_\_
#ifdef \_\_SIZE\_TYPE\_\_
typedef \_\_SIZE\_TYPE\_\_ size\_t;
#else
#include 
#endif
#if defined(\_\_UINT8\_TYPE\_\_) && defined(\_\_UINT32\_TYPE\_\_) \
&& defined(\_\_UINT64\_TYPE\_\_)
typedef \_\_UINT8\_TYPE\_\_ uint8\_t;
typedef \_\_UINT32\_TYPE\_\_ uint32\_t;
typedef \_\_UINT64\_TYPE\_\_ uint64\_t;
#else
#include 
#endif
#ifdef \_\_cplusplus
extern "C" {
#elif !defined(\_\_STDC\_VERSION\_\_) || \_\_STDC\_VERSION\_\_<202311
# if defined(\_\_BLST\_CGO\_\_)
typedef \_Bool bool; /\* it's assumed that cgo calls modern enough compiler \*/
# elif defined(\_\_BLST\_RUST\_BINDGEN\_\_) || defined(\_\_BLST\_ZIG\_\_)
# define bool \_Bool
# elif defined(\_\_STDC\_VERSION\_\_) && \_\_STDC\_VERSION\_\_>=199901
# include 
# elif !defined(bool)
# define bool int
# define \_\_blst\_h\_bool\_\_
# endif
#endif
#ifdef SWIG
# define DEFNULL =NULL
#elif defined \_\_cplusplus
# define DEFNULL =0
#else
# define DEFNULL
#endif
typedef enum {
BLST\_SUCCESS = 0,
BLST\_BAD\_ENCODING,
BLST\_POINT\_NOT\_ON\_CURVE,
BLST\_POINT\_NOT\_IN\_GROUP,
BLST\_AGGR\_TYPE\_MISMATCH,
BLST\_VERIFY\_FAIL,
BLST\_PK\_IS\_INFINITY,
BLST\_BAD\_SCALAR,
} BLST\_ERROR;
typedef uint8\_t byte;
typedef uint64\_t limb\_t;
typedef struct { byte b[256/8]; } blst\_scalar;
typedef struct { limb\_t l[256/8/sizeof(limb\_t)]; } blst\_fr;
typedef struct { limb\_t l[384/8/sizeof(limb\_t)]; } blst\_fp;
/\* 0 is "real" part, 1 is "imaginary" \*/
typedef struct { blst\_fp fp[2]; } blst\_fp2;
typedef struct { blst\_fp2 fp2[3]; } blst\_fp6;
typedef struct { blst\_fp6 fp6[2]; } blst\_fp12;
void blst\_scalar\_from\_uint32(blst\_scalar \*out, const uint32\_t a[8]);
void blst\_uint32\_from\_scalar(uint32\_t out[8], const blst\_scalar \*a);
void blst\_scalar\_from\_uint64(blst\_scalar \*out, const uint64\_t a[4]);
void blst\_uint64\_from\_scalar(uint64\_t out[4], const blst\_scalar \*a);
void blst\_scalar\_from\_bendian(blst\_scalar \*out, const byte a[32]);
void blst\_bendian\_from\_scalar(byte out[32], const blst\_scalar \*a);
void blst\_scalar\_from\_lendian(blst\_scalar \*out, const byte a[32]);
void blst\_lendian\_from\_scalar(byte out[32], const blst\_scalar \*a);
bool blst\_scalar\_fr\_check(const blst\_scalar \*a);
bool blst\_sk\_check(const blst\_scalar \*a);
bool blst\_sk\_add\_n\_check(blst\_scalar \*out, const blst\_scalar \*a,
const blst\_scalar \*b);
bool blst\_sk\_sub\_n\_check(blst\_scalar \*out, const blst\_scalar \*a,
const blst\_scalar \*b);
bool blst\_sk\_mul\_n\_check(blst\_scalar \*out, const blst\_scalar \*a,
const blst\_scalar \*b);
void blst\_sk\_inverse(blst\_scalar \*out, const blst\_scalar \*a);
bool blst\_scalar\_from\_le\_bytes(blst\_scalar \*out, const byte \*in, size\_t len);
bool blst\_scalar\_from\_be\_bytes(blst\_scalar \*out, const byte \*in, size\_t len);
#ifndef SWIG
/\*
\* BLS12-381-specific Fr operations.
\*/
void blst\_fr\_add(blst\_fr \*ret, const blst\_fr \*a, const blst\_fr \*b);
void blst\_fr\_sub(blst\_fr \*ret, const blst\_fr \*a, const blst\_fr \*b);
void blst\_fr\_mul\_by\_3(blst\_fr \*ret, const blst\_fr \*a);
void blst\_fr\_lshift(blst\_fr \*ret, const blst\_fr \*a, size\_t count);
void blst\_fr\_rshift(blst\_fr \*ret, const blst\_fr \*a, size\_t count);
void blst\_fr\_mul(blst\_fr \*ret, const blst\_fr \*a, const blst\_fr \*b);
void blst\_fr\_sqr(blst\_fr \*ret, const blst\_fr \*a);
void blst\_fr\_cneg(blst\_fr \*ret, const blst\_fr \*a, bool flag);
void blst\_fr\_eucl\_inverse(blst\_fr \*ret, const blst\_fr \*a);
void blst\_fr\_inverse(blst\_fr \*ret, const blst\_fr \*a);
void blst\_fr\_from\_uint64(blst\_fr \*ret, const uint64\_t a[4]);
void blst\_uint64\_from\_fr(uint64\_t ret[4], const blst\_fr \*a);
void blst\_fr\_from\_scalar(blst\_fr \*ret, const blst\_scalar \*a);
void blst\_scalar\_from\_fr(blst\_scalar \*ret, const blst\_fr \*a);
/\*
\* BLS12-381-specific Fp operations.
\*/
void blst\_fp\_add(blst\_fp \*ret, const blst\_fp \*a, const blst\_fp \*b);
void blst\_fp\_sub(blst\_fp \*ret, const blst\_fp \*a, const blst\_fp \*b);
void blst\_fp\_mul\_by\_3(blst\_fp \*ret, const blst\_fp \*a);
void blst\_fp\_mul\_by\_8(blst\_fp \*ret, const blst\_fp \*a);
void blst\_fp\_lshift(blst\_fp \*ret, const blst\_fp \*a, size\_t count);
void blst\_fp\_mul(blst\_fp \*ret, const blst\_fp \*a, const blst\_fp \*b);
void blst\_fp\_sqr(blst\_fp \*ret, const blst\_fp \*a);
void blst\_fp\_cneg(blst\_fp \*ret, const blst\_fp \*a, bool flag);
void blst\_fp\_eucl\_inverse(blst\_fp \*ret, const blst\_fp \*a);
void blst\_fp\_inverse(blst\_fp \*ret, const blst\_fp \*a);
bool blst\_fp\_sqrt(blst\_fp \*ret, const blst\_fp \*a);
void blst\_fp\_from\_uint32(blst\_fp \*ret, const uint32\_t a[12]);
void blst\_uint32\_from\_fp(uint32\_t ret[12], const blst\_fp \*a);
void blst\_fp\_from\_uint64(blst\_fp \*ret, const uint64\_t a[6]);
void blst\_uint64\_from\_fp(uint64\_t ret[6], const blst\_fp \*a);
void blst\_fp\_from\_bendian(blst\_fp \*ret, const byte a[48]);
void blst\_bendian\_from\_fp(byte ret[48], const blst\_fp \*a);
void blst\_fp\_from\_lendian(blst\_fp \*ret, const byte a[48]);
void blst\_lendian\_from\_fp(byte ret[48], const blst\_fp \*a);
/\*
\* BLS12-381-specific Fp2 operations.
\*/
void blst\_fp2\_add(blst\_fp2 \*ret, const blst\_fp2 \*a, const blst\_fp2 \*b);
void blst\_fp2\_sub(blst\_fp2 \*ret, const blst\_fp2 \*a, const blst\_fp2 \*b);
void blst\_fp2\_mul\_by\_3(blst\_fp2 \*ret, const blst\_fp2 \*a);
void blst\_fp2\_mul\_by\_8(blst\_fp2 \*ret, const blst\_fp2 \*a);
void blst\_fp2\_lshift(blst\_fp2 \*ret, const blst\_fp2 \*a, size\_t count);
void blst\_fp2\_mul(blst\_fp2 \*ret, const blst\_fp2 \*a, const blst\_fp2 \*b);
void blst\_fp2\_sqr(blst\_fp2 \*ret, const blst\_fp2 \*a);
void blst\_fp2\_cneg(blst\_fp2 \*ret, const blst\_fp2 \*a, bool flag);
void blst\_fp2\_eucl\_inverse(blst\_fp2 \*ret, const blst\_fp2 \*a);
void blst\_fp2\_inverse(blst\_fp2 \*ret, const blst\_fp2 \*a);
bool blst\_fp2\_sqrt(blst\_fp2 \*ret, const blst\_fp2 \*a);
/\*
\* BLS12-381-specific Fp12 operations.
\*/
void blst\_fp12\_sqr(blst\_fp12 \*ret, const blst\_fp12 \*a);
void blst\_fp12\_cyclotomic\_sqr(blst\_fp12 \*ret, const blst\_fp12 \*a);
void blst\_fp12\_mul(blst\_fp12 \*ret, const blst\_fp12 \*a, const blst\_fp12 \*b);
void blst\_fp12\_mul\_by\_xy00z0(blst\_fp12 \*ret, const blst\_fp12 \*a,
const blst\_fp6 \*xy00z0);
void blst\_fp12\_conjugate(blst\_fp12 \*a);
void blst\_fp12\_inverse(blst\_fp12 \*ret, const blst\_fp12 \*a);
/\* caveat lector! |n| has to be non-zero and not more than 3! \*/
void blst\_fp12\_frobenius\_map(blst\_fp12 \*ret, const blst\_fp12 \*a, size\_t n);
bool blst\_fp12\_is\_equal(const blst\_fp12 \*a, const blst\_fp12 \*b);
bool blst\_fp12\_is\_one(const blst\_fp12 \*a);
bool blst\_fp12\_in\_group(const blst\_fp12 \*a);
const blst\_fp12 \*blst\_fp12\_one(void);
#endif // SWIG
/\*
\* BLS12-381-specific point operations.
\*/
typedef struct { blst\_fp x, y, z; } blst\_p1;
typedef struct { blst\_fp x, y; } blst\_p1\_affine;
void blst\_p1\_add(blst\_p1 \*out, const blst\_p1 \*a, const blst\_p1 \*b);
void blst\_p1\_add\_or\_double(blst\_p1 \*out, const blst\_p1 \*a, const blst\_p1 \*b);
void blst\_p1\_add\_affine(blst\_p1 \*out, const blst\_p1 \*a,
const blst\_p1\_affine \*b);
void blst\_p1\_add\_or\_double\_affine(blst\_p1 \*out, const blst\_p1 \*a,
const blst\_p1\_affine \*b);
void blst\_p1\_double(blst\_p1 \*out, const blst\_p1 \*a);
void blst\_p1\_mult(blst\_p1 \*out, const blst\_p1 \*p, const byte \*scalar,
size\_t nbits);
void blst\_p1\_cneg(blst\_p1 \*p, bool cbit);
void blst\_p1\_to\_affine(blst\_p1\_affine \*out, const blst\_p1 \*in);
void blst\_p1\_from\_affine(blst\_p1 \*out, const blst\_p1\_affine \*in);
bool blst\_p1\_on\_curve(const blst\_p1 \*p);
bool blst\_p1\_in\_g1(const blst\_p1 \*p);
bool blst\_p1\_is\_equal(const blst\_p1 \*a, const blst\_p1 \*b);
bool blst\_p1\_is\_inf(const blst\_p1 \*a);
const blst\_p1 \*blst\_p1\_generator(void);
bool blst\_p1\_affine\_on\_curve(const blst\_p1\_affine \*p);
bool blst\_p1\_affine\_in\_g1(const blst\_p1\_affine \*p);
bool blst\_p1\_affine\_is\_equal(const blst\_p1\_affine \*a, const blst\_p1\_affine \*b);
bool blst\_p1\_affine\_is\_inf(const blst\_p1\_affine \*a);
const blst\_p1\_affine \*blst\_p1\_affine\_generator(void);
typedef struct { blst\_fp2 x, y, z; } blst\_p2;
typedef struct { blst\_fp2 x, y; } blst\_p2\_affine;
void blst\_p2\_add(blst\_p2 \*out, const blst\_p2 \*a, const blst\_p2 \*b);
void blst\_p2\_add\_or\_double(blst\_p2 \*out, const blst\_p2 \*a, const blst\_p2 \*b);
void blst\_p2\_add\_affine(blst\_p2 \*out, const blst\_p2 \*a,
const blst\_p2\_affine \*b);
void blst\_p2\_add\_or\_double\_affine(blst\_p2 \*out, const blst\_p2 \*a,
const blst\_p2\_affine \*b);
void blst\_p2\_double(blst\_p2 \*out, const blst\_p2 \*a);
void blst\_p2\_mult(blst\_p2 \*out, const blst\_p2 \*p, const byte \*scalar,
size\_t nbits);
void blst\_p2\_cneg(blst\_p2 \*p, bool cbit);
void blst\_p2\_to\_affine(blst\_p2\_affine \*out, const blst\_p2 \*in);
void blst\_p2\_from\_affine(blst\_p2 \*out, const blst\_p2\_affine \*in);
bool blst\_p2\_on\_curve(const blst\_p2 \*p);
bool blst\_p2\_in\_g2(const blst\_p2 \*p);
bool blst\_p2\_is\_equal(const blst\_p2 \*a, const blst\_p2 \*b);
bool blst\_p2\_is\_inf(const blst\_p2 \*a);
const blst\_p2 \*blst\_p2\_generator(void);
bool blst\_p2\_affine\_on\_curve(const blst\_p2\_affine \*p);
bool blst\_p2\_affine\_in\_g2(const blst\_p2\_affine \*p);
bool blst\_p2\_affine\_is\_equal(const blst\_p2\_affine \*a, const blst\_p2\_affine \*b);
bool blst\_p2\_affine\_is\_inf(const blst\_p2\_affine \*a);
const blst\_p2\_affine \*blst\_p2\_affine\_generator(void);
/\*
\* Multi-scalar multiplications and other multi-point operations.
\*/
void blst\_p1s\_to\_affine(blst\_p1\_affine dst[], const blst\_p1 \*const points[],
size\_t npoints);
void blst\_p1s\_add(blst\_p1 \*ret, const blst\_p1\_affine \*const points[],
size\_t npoints);
size\_t blst\_p1s\_mult\_wbits\_precompute\_sizeof(size\_t wbits, size\_t npoints);
void blst\_p1s\_mult\_wbits\_precompute(blst\_p1\_affine table[], size\_t wbits,
const blst\_p1\_affine \*const points[],
size\_t npoints);
size\_t blst\_p1s\_mult\_wbits\_scratch\_sizeof(size\_t npoints);
void blst\_p1s\_mult\_wbits(blst\_p1 \*ret, const blst\_p1\_affine table[],
size\_t wbits, size\_t npoints,
const byte \*const scalars[], size\_t nbits,
limb\_t \*scratch);
size\_t blst\_p1s\_mult\_pippenger\_scratch\_sizeof(size\_t npoints);
void blst\_p1s\_mult\_pippenger(blst\_p1 \*ret, const blst\_p1\_affine \*const points[],
size\_t npoints, const byte \*const scalars[],
size\_t nbits, limb\_t \*scratch);
void blst\_p1s\_tile\_pippenger(blst\_p1 \*ret, const blst\_p1\_affine \*const points[],
size\_t npoints, const byte \*const scalars[],
size\_t nbits, limb\_t \*scratch,
size\_t bit0, size\_t window);
void blst\_p2s\_to\_affine(blst\_p2\_affine dst[], const blst\_p2 \*const points[],
size\_t npoints);
void blst\_p2s\_add(blst\_p2 \*ret, const blst\_p2\_affine \*const points[],
size\_t npoints);
size\_t blst\_p2s\_mult\_wbits\_precompute\_sizeof(size\_t wbits, size\_t npoints);
void blst\_p2s\_mult\_wbits\_precompute(blst\_p2\_affine table[], size\_t wbits,
const blst\_p2\_affine \*const points[],
size\_t npoints);
size\_t blst\_p2s\_mult\_wbits\_scratch\_sizeof(size\_t npoints);
void blst\_p2s\_mult\_wbits(blst\_p2 \*ret, const blst\_p2\_affine table[],
size\_t wbits, size\_t npoints,
const byte \*const scalars[], size\_t nbits,
limb\_t \*scratch);
size\_t blst\_p2s\_mult\_pippenger\_scratch\_sizeof(size\_t npoints);
void blst\_p2s\_mult\_pippenger(blst\_p2 \*ret, const blst\_p2\_affine \*const points[],
size\_t npoints, const byte \*const scalars[],
size\_t nbits, limb\_t \*scratch);
void blst\_p2s\_tile\_pippenger(blst\_p2 \*ret, const blst\_p2\_affine \*const points[],
size\_t npoints, const byte \*const scalars[],
size\_t nbits, limb\_t \*scratch,
size\_t bit0, size\_t window);
/\*
\* Hash-to-curve operations.
\*/
#ifndef SWIG
void blst\_map\_to\_g1(blst\_p1 \*out, const blst\_fp \*u, const blst\_fp \*v DEFNULL);
void blst\_map\_to\_g2(blst\_p2 \*out, const blst\_fp2 \*u, const blst\_fp2 \*v DEFNULL);
#endif
void blst\_encode\_to\_g1(blst\_p1 \*out,
const byte \*msg, size\_t msg\_len,
const byte \*DST DEFNULL, size\_t DST\_len DEFNULL,
const byte \*aug DEFNULL, size\_t aug\_len DEFNULL);
void blst\_hash\_to\_g1(blst\_p1 \*out,
const byte \*msg, size\_t msg\_len,
const byte \*DST DEFNULL, size\_t DST\_len DEFNULL,
const byte \*aug DEFNULL, size\_t aug\_len DEFNULL);
void blst\_encode\_to\_g2(blst\_p2 \*out,
const byte \*msg, size\_t msg\_len,
const byte \*DST DEFNULL, size\_t DST\_len DEFNULL,
const byte \*aug DEFNULL, size\_t aug\_len DEFNULL);
void blst\_hash\_to\_g2(blst\_p2 \*out,
const byte \*msg, size\_t msg\_len,
const byte \*DST DEFNULL, size\_t DST\_len DEFNULL,
const byte \*aug DEFNULL, size\_t aug\_len DEFNULL);
/\*
\* Zcash-compatible serialization/deserialization.
\*/
void blst\_p1\_serialize(byte out[96], const blst\_p1 \*in);
void blst\_p1\_compress(byte out[48], const blst\_p1 \*in);
void blst\_p1\_affine\_serialize(byte out[96], const blst\_p1\_affine \*in);
void blst\_p1\_affine\_compress(byte out[48], const blst\_p1\_affine \*in);
BLST\_ERROR blst\_p1\_uncompress(blst\_p1\_affine \*out, const byte in[48]);
BLST\_ERROR blst\_p1\_deserialize(blst\_p1\_affine \*out, const byte in[96]);
void blst\_p2\_serialize(byte out[192], const blst\_p2 \*in);
void blst\_p2\_compress(byte out[96], const blst\_p2 \*in);
void blst\_p2\_affine\_serialize(byte out[192], const blst\_p2\_affine \*in);
void blst\_p2\_affine\_compress(byte out[96], const blst\_p2\_affine \*in);
BLST\_ERROR blst\_p2\_uncompress(blst\_p2\_affine \*out, const byte in[96]);
BLST\_ERROR blst\_p2\_deserialize(blst\_p2\_affine \*out, const byte in[192]);
/\*
\* Specification defines two variants, 'minimal-signature-size' and
\* 'minimal-pubkey-size'. To unify appearance we choose to distinguish
\* them by suffix referring to the public key type, more specifically
\* \_pk\_in\_g1 corresponds to 'minimal-pubkey-size' and \_pk\_in\_g2 - to
\* 'minimal-signature-size'. It might appear a bit counterintuitive
\* in sign call, but no matter how you twist it, something is bound to
\* turn a little odd.
\*/
/\*
\* Secret-key operations.
\*/
void blst\_keygen(blst\_scalar \*out\_SK, const byte \*IKM, size\_t IKM\_len,
const byte \*info DEFNULL, size\_t info\_len DEFNULL);
void blst\_sk\_to\_pk\_in\_g1(blst\_p1 \*out\_pk, const blst\_scalar \*SK);
void blst\_sign\_pk\_in\_g1(blst\_p2 \*out\_sig, const blst\_p2 \*hash,
const blst\_scalar \*SK);
void blst\_sk\_to\_pk\_in\_g2(blst\_p2 \*out\_pk, const blst\_scalar \*SK);
void blst\_sign\_pk\_in\_g2(blst\_p1 \*out\_sig, const blst\_p1 \*hash,
const blst\_scalar \*SK);
/\*
\* Pairing interface.
\*/
#ifndef SWIG
void blst\_miller\_loop(blst\_fp12 \*ret, const blst\_p2\_affine \*Q,
const blst\_p1\_affine \*P);
void blst\_miller\_loop\_n(blst\_fp12 \*ret, const blst\_p2\_affine \*const Qs[],
const blst\_p1\_affine \*const Ps[],
size\_t n);
void blst\_final\_exp(blst\_fp12 \*ret, const blst\_fp12 \*f);
void blst\_precompute\_lines(blst\_fp6 Qlines[68], const blst\_p2\_affine \*Q);
void blst\_miller\_loop\_lines(blst\_fp12 \*ret, const blst\_fp6 Qlines[68],
const blst\_p1\_affine \*P);
bool blst\_fp12\_finalverify(const blst\_fp12 \*gt1, const blst\_fp12 \*gt2);
#endif
#ifdef \_\_BLST\_CGO\_\_
typedef limb\_t blst\_pairing;
#elif defined(\_\_BLST\_RUST\_BINDGEN\_\_)
typedef struct {} blst\_pairing;
#else
typedef struct blst\_opaque blst\_pairing;
#endif
size\_t blst\_pairing\_sizeof(void);
void blst\_pairing\_init(blst\_pairing \*new\_ctx, bool hash\_or\_encode,
const byte \*DST DEFNULL, size\_t DST\_len DEFNULL);
const byte \*blst\_pairing\_get\_dst(const blst\_pairing \*ctx);
void blst\_pairing\_commit(blst\_pairing \*ctx);
BLST\_ERROR blst\_pairing\_aggregate\_pk\_in\_g2(blst\_pairing \*ctx,
const blst\_p2\_affine \*PK,
const blst\_p1\_affine \*signature,
const byte \*msg, size\_t msg\_len,
const byte \*aug DEFNULL,
size\_t aug\_len DEFNULL);
BLST\_ERROR blst\_pairing\_chk\_n\_aggr\_pk\_in\_g2(blst\_pairing \*ctx,
const blst\_p2\_affine \*PK,
bool pk\_grpchk,
const blst\_p1\_affine \*signature,
bool sig\_grpchk,
const byte \*msg, size\_t msg\_len,
const byte \*aug DEFNULL,
size\_t aug\_len DEFNULL);
BLST\_ERROR blst\_pairing\_mul\_n\_aggregate\_pk\_in\_g2(blst\_pairing \*ctx,
const blst\_p2\_affine \*PK,
const blst\_p1\_affine \*sig,
const byte \*scalar,
size\_t nbits,
const byte \*msg,
size\_t msg\_len,
const byte \*aug DEFNULL,
size\_t aug\_len DEFNULL);
BLST\_ERROR blst\_pairing\_chk\_n\_mul\_n\_aggr\_pk\_in\_g2(blst\_pairing \*ctx,
const blst\_p2\_affine \*PK,
bool pk\_grpchk,
const blst\_p1\_affine \*sig,
bool sig\_grpchk,
const byte \*scalar,
size\_t nbits,
const byte \*msg,
size\_t msg\_len,
const byte \*aug DEFNULL,
size\_t aug\_len DEFNULL);
BLST\_ERROR blst\_pairing\_aggregate\_pk\_in\_g1(blst\_pairing \*ctx,
const blst\_p1\_affine \*PK,
const blst\_p2\_affine \*signature,
const byte \*msg, size\_t msg\_len,
const byte \*aug DEFNULL,
size\_t aug\_len DEFNULL);
BLST\_ERROR blst\_pairing\_chk\_n\_aggr\_pk\_in\_g1(blst\_pairing \*ctx,
const blst\_p1\_affine \*PK,
bool pk\_grpchk,
const blst\_p2\_affine \*signature,
bool sig\_grpchk,
const byte \*msg, size\_t msg\_len,
const byte \*aug DEFNULL,
size\_t aug\_len DEFNULL);
BLST\_ERROR blst\_pairing\_mul\_n\_aggregate\_pk\_in\_g1(blst\_pairing \*ctx,
const blst\_p1\_affine \*PK,
const blst\_p2\_affine \*sig,
const byte \*scalar,
size\_t nbits,
const byte \*msg,
size\_t msg\_len,
const byte \*aug DEFNULL,
size\_t aug\_len DEFNULL);
BLST\_ERROR blst\_pairing\_chk\_n\_mul\_n\_aggr\_pk\_in\_g1(blst\_pairing \*ctx,
const blst\_p1\_affine \*PK,
bool pk\_grpchk,
const blst\_p2\_affine \*sig,
bool sig\_grpchk,
const byte \*scalar,
size\_t nbits,
const byte \*msg,
size\_t msg\_len,
const byte \*aug DEFNULL,
size\_t aug\_len DEFNULL);
BLST\_ERROR blst\_pairing\_merge(blst\_pairing \*ctx, const blst\_pairing \*ctx1);
bool blst\_pairing\_finalverify(const blst\_pairing \*ctx,
const blst\_fp12 \*gtsig DEFNULL);
/\*
\* Customarily applications aggregate signatures separately.
\* In which case application would have to pass NULLs for |signature|
\* to blst\_pairing\_aggregate calls and pass aggregated signature
\* collected with these calls to blst\_pairing\_finalverify. Inputs are
\* Zcash-compatible "straight-from-wire" byte vectors, compressed or
\* not.
\*/
BLST\_ERROR blst\_aggregate\_in\_g1(blst\_p1 \*out, const blst\_p1 \*in,
const byte \*zwire);
BLST\_ERROR blst\_aggregate\_in\_g2(blst\_p2 \*out, const blst\_p2 \*in,
const byte \*zwire);
void blst\_aggregated\_in\_g1(blst\_fp12 \*out, const blst\_p1\_affine \*signature);
void blst\_aggregated\_in\_g2(blst\_fp12 \*out, const blst\_p2\_affine \*signature);
/\*
\* "One-shot" CoreVerify entry points.
\*/
BLST\_ERROR blst\_core\_verify\_pk\_in\_g1(const blst\_p1\_affine \*pk,
const blst\_p2\_affine \*signature,
bool hash\_or\_encode,
const byte \*msg, size\_t msg\_len,
const byte \*DST DEFNULL,
size\_t DST\_len DEFNULL,
const byte \*aug DEFNULL,
size\_t aug\_len DEFNULL);
BLST\_ERROR blst\_core\_verify\_pk\_in\_g2(const blst\_p2\_affine \*pk,
const blst\_p1\_affine \*signature,
bool hash\_or\_encode,
const byte \*msg, size\_t msg\_len,
const byte \*DST DEFNULL,
size\_t DST\_len DEFNULL,
const byte \*aug DEFNULL,
size\_t aug\_len DEFNULL);
extern const blst\_p1\_affine BLS12\_381\_G1;
extern const blst\_p1\_affine BLS12\_381\_NEG\_G1;
extern const blst\_p2\_affine BLS12\_381\_G2;
extern const blst\_p2\_affine BLS12\_381\_NEG\_G2;
#include "blst\_aux.h"
#ifdef \_\_cplusplus
}
#elif defined(\_\_blst\_h\_bool\_\_)
# undef \_\_blst\_h\_bool\_\_
# undef bool
#endif
#endif