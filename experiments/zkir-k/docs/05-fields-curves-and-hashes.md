# Fields, curves and hashes

The definition computes field, curve and hash primitives on K's
arbitrary-precision `Int` with an explicit modulus (plan decision CLM-0709),
in `zkir-field.k` (`ZKIR-FIELD`), `zkir-curves.k` (`ZKIR-CURVES`) and
`zkir-hash.k` (`ZKIR-HASH`), with generated tables in `zkir-constants.k`.
The extension adds SHA-512 in `zkir-ext.k` (`ZKIR-SHA512`) and
`zkir-sha512-constants.k`. Instruction wrappers are in `zkir-vm.k` and
`zkir-ops.k`; value encodings are in `04-values-and-encoding.md`. Constants
follow midnight-curves 0.3.1 and midnight-circuits 7.2.4.

## Prime fields

`ZKIR-FIELD` names eight moduli. Every arithmetic function takes the modulus
as an argument and reduces with `modInt`, the Euclidean remainder in `[0, P)`.

| Symbol | Role | Value | Origin |
|---|---|---|---|
| `#r` | BLS12-381 scalar field; ZKIR native field; Jubjub base field | `52435875175126190479447740508185965837690552500527637822603658699938581184513` | midnight-curves `bls12_381/fq.rs`, `0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001` |
| `#rJ` | Jubjub scalar field (prime-order subgroup order) | `6554484396890773809930967563523245729705921265872317281365359162392183254199` | midnight-curves `jubjub/fr.rs`, `0x0e7db4ea6533afa906673b0101343b00a6682093ccc81082d0970e5ed6f72cb7` |
| `#k256P` | secp256k1 base field | `115792089237316195423570985008687907853269984665640564039457584007908834671663` | `2^256 - 2^32 - 977`, Standards for Efficient Cryptography 2 (SEC 2) |
| `#k256N` | secp256k1 group order | `115792089237316195423570985008687907852837564279074904382605163141518161494337` | SEC 2 |
| `#p256P` | secp256r1 base field | `115792089210356248762697446949407573530086143415290314195533631308867097853951` | Federal Information Processing Standards (FIPS) 186-4 |
| `#p256N` | secp256r1 group order | `115792089210356248762697446949407573529996955224135760342422259061068512044369` | FIPS 186-4 |
| `#c25519P` | Curve25519 base field | `57896044618658097711785492504343953926634992332820282019728792003956564819949` | `2^255 - 19` |
| `#c25519L` | Curve25519 subgroup order | `7237005577332262213973186563042994240857116359379907606001950938285454250989` | `2^252 + 27742317777372353535851937790883648493` |

### Arithmetic

`fmod`, `fadd`, `fsub`, `fmul`, `fneg`, `finv`, `fdiv`, `fpow` and `fsq` are
each defined by rules guarded by `P >Int 1` plus an `[owise]` rule returning
`0`. On `P > 1`: `fmod(A, P)` is `A modInt P`; `fadd`, `fsub` and `fmul`
reduce the integer sum, difference or product; `fneg(A, P)` is
`(0 -Int A) modInt P`; `fsq(A, P)` is `fmul(A, A, P)`. `finv` has an explicit
zero rule and a Fermat rule:

```k
rule finv(A, P) => 0 requires P >Int 1 andBool A modInt P ==Int 0
rule finv(A, P) => (A modInt P) ^%Int (P -Int 2) P requires P >Int 1 andBool A modInt P =/=Int 0
rule finv(_, _) => 0 [owise]
```

`fdiv(A, B, P)` is `fmul(A, finv(B, P), P)`, so division by zero is `0`.
`fpow(A, E, P)` is `(A modInt P) ^%Int E P` and also requires `E >=Int 0`.
The `[owise]` rules keep the functions total for the symbolic backend.

### Legendre symbol and square roots

`legendre(A, P)` is `fpow(A, (P -Int 1) /Int 2, P)`; for prime `P` it is in
`{0, 1, P-1}`. Zero counts as a residue: `isQuadraticNonResidue(A, P)` is
`legendre(A, P) ==Int P -Int 1`, as in midnight-curves
`ct_quadratic_non_residue` (`ff_ext/mod.rs`).

`fsqrt(A, P)` has three rules: `sqrtOk(0)` when `A modInt P ==Int 0`;
`noSqrt()` when the remainder is non-zero and `legendre(A, P) =/=Int 1`;
otherwise Tonelli-Shanks:

```k
rule fsqrt(A, P) => #ts(A modInt P, P, #twoAdicity(P -Int 1, 0), #oddPart(P -Int 1), #nonResidue(P, 2))
  requires A modInt P =/=Int 0 andBool legendre(A, P) ==Int 1
```

`#twoAdicity(N, S)` recurses on `N /Int 2` while `N > 0` is even and returns
`S` once `N` is odd or `N <= 0`; `#oddPart(N)` strips the same factors.
`#nonResidue(P, Z)` is the smallest quadratic non-residue in `[Z, P)`, or
`0` when `Z >= P`. For `#r`, `P - 1 = 2^32 * Q` and the search from `2`
finds `5`.

`#ts(A, P, S, Q, Z)` starts `#tsLoop(P, M, c, t, R)` with `M = S`,
`c = Z^Q`, `t = A^Q`, `R = A^((Q+1)/2)`. `#tsLoop` returns `noSqrt()` if
`t = 0` and `sqrtOk(R)` if `t = 1`; otherwise `#tsOrder` finds the least `i`
with `t^(2^i) = 1` (stopping at `M`) and `#tsStep` returns `noSqrt()` if
`i >= M`, else continues with `b = c^(2^(M-i-1))`, `M = i`, `c = b^2`,
`t = t b^2`, `R = R b`. `M` strictly decreases, so the loop terminates. Which
root is returned is not specified; Jubjub decompression and the
Shallue-van de Woestijne map fix the parity themselves.

### Bit helpers

`bitAt(A, I)` is little-endian bit `I`; `lowBits(A, N)` masks to `N` bits;
`highBits(A, N)` is `A >>Int N`. All three require `A >= 0` and a
non-negative index or width, and return `0` otherwise (`[owise]`).
`fitsBits(A, N)` is `A >=Int 0 andBool A <Int (1 <<Int N)` and requires only
`N >= 0` (`false` otherwise); a negative `A` is simply not in `[0, 2^N)`.
`isOddInt(A)` is `A modInt 2 ==Int 1`.

## Curves

A point is `pt(X, Y)` or `inf()`. Descriptors are `edwards(p, d, order)` and
`weierstrass(p, a, b, order)`:

- `#jubjub` is `edwards(#r, -(10240/10241) mod #r, #rJ)`.
- `#curve25519` is `edwards(#c25519P, -(121665/121666) mod #c25519P, #c25519L)`.
- `#secp256k1` is `weierstrass(#k256P, 0, 7, #k256N)`.
- `#secp256r1` is `weierstrass(#p256P, #p256P - 3, b, #p256N)` with FIPS 186-4
  `b = 41058363725152142129326129780047268409114441015993725554835256314039467401291`.

Both Edwards curves have `a = -1`, which is not stored: `onCurve` tests
`-x^2 + y^2 = 1 + d x^2 y^2`. Jubjub `d` equals midnight-curves `EDWARDS_D`
(`jubjub/curve.rs`; `#jubjubD` is the same value standalone) and Curve25519
`d` equals `CURVE_D` (`curve25519/curve.rs`). Both are non-squares, so the
unified Edwards addition is complete. The Weierstrass equation is
`y^2 = x^3 + a x + b`. `onCurve` accepts `inf()` for every descriptor and
requires finite coordinates in `[0, P)`. The Edwards identity is `pt(0, 1)`
and Edwards values never use `inf()`; the Weierstrass identity is `inf()`.

### Group law

`ecNeg` sends Edwards `(x, y)` to `(-x, y)`, Weierstrass `(x, y)` to
`(x, -y)`, and fixes `inf()`. Edwards `ecAdd` is the unified formula with
denominators `1 ± d x1 x2 y1 y2`, which never vanish on curve points; its
rules for `inf()` return `inf()` and are unreachable for valid values.
Edwards `ecDouble(C, Q)` is `ecAdd(C, Q, Q)`.

Weierstrass `ecAdd` is the affine chord-and-tangent law (`#wChord`): `inf()`
is a unit, equal points double, and points with equal `x` and different `y`
sum to `inf()`. `ecDouble` of `inf()` or of a point with `y = 0` is `inf()`;
otherwise the slope is `(3 x^2 + a) / (2 y)`. `ecMul(C, Q, K)` is
right-to-left double-and-add over the bits of `K` (`#ecMulAcc`), returning
`identity(C)` when `K <= 0`; callers pass canonical non-negative scalars.

### Generators and subgroups

`#jubjubGenerator` is `ecMul(#jubjub, G, 8)` where
`G = (44746807950788659978687200207992930935149218647843500701850233404325651525118, 11)`
is `JubjubAffine::generator()` of midnight-curves. `JubjubSubgroup::generator()`
is `JubjubExtended::generator().clear_cofactor()`, three doublings, so the
two agree; `G` itself is not in the prime-order subgroup.
`#curve25519Generator` is `ED25519_BASEPOINT_POINT` of curve25519-dalek 4.1.3
(the version midnight-curves pins), already in the prime-order subgroup. The
secp256k1 and secp256r1 generators are the SEC 2 and FIPS 186-4 base points.

`inSubgroup` on an Edwards curve is `isIdentity(C, ecMul(C, Q, N))` with `N`
the subgroup order. On a Weierstrass curve it is identically `true`, because
those groups have prime order; it does not re-test `onCurve`.

### Constructors

`fromXY(C, X, Y)` returns `ptOk(pt(X, Y))` when `onCurve` holds and
`ptErr("point is not on the curve")` otherwise: the exact test of
`K256Affine::from_xy`, P-256 `affine_from_xy` (SEC 1 decoding) and
`Curve25519Affine::from_xy`.

Jubjub does not use `fromXY`. `jubjubFromXY(X, Y)` follows midnight-circuits
`ecc/curves.rs`, which compresses `(y, low bit of x)` and decompresses
through `JubjubAffine::from_bytes`. Coordinates outside `[0, #r)` give
`ptErr("coordinate out of range")`. Otherwise `#jubjubDecompress` receives
`X modInt 2` and `fsqrt((y^2 - 1) / (1 + d y^2))`:

- `noSqrt()` gives `ptErr("y is not on Jubjub")`;
- a root `U` with `U modInt 2` equal to the sign gives `pt(U, Y)`;
- `U = 0` with the other sign gives `ptErr("invalid Jubjub encoding: -0")`
  (Zcash Improvement Proposal 216);
- otherwise the result is `pt(-U, Y)`.

A wrong `x` with the right parity therefore recovers the true point; the
off-circuit instruction still requires `inSubgroup` (`zkir-ops.k`,
`#ptToValue`). This parity-only witness against the in-circuit exact
coordinates is K1 in `13-known-divergences.md`.

## Poseidon

`poseidonHash` is the fixed-length sponge behind `transient_hash`: width
`t = 3` over `#r`, rate 2, `R_F = 8` full and `R_P = 60` partial rounds. The
round constants and the Maximum Distance Separable (MDS) matrix are the 204
scalars `poseidonRC(0..67, 0..2)` and the 3-by-3 `poseidonMDS` of
`zkir-constants.k`, generated by `tools/gen_constants.py` from
midnight-circuits `hash/poseidon/constants/blstrs.rs`, itself the output of
`generate_parameters_grain.sage` for `(3, 8, 60)` over `#r`.

`sbox(X)` is `X^5`. `permute` adds `poseidonRC(0, *)` to the state and runs
68 layers (`#rounds`): four full, sixty partial, four full. A layer applies
the S-box (to all three cells, or only the last cell in a partial round) and
then `linear`, which multiplies by the MDS matrix and adds
`#rc(R, *) = poseidonRC(R + 1, *)`; for the last layer `#rc` is `0`
(`[owise]`), so the 68 stored triples are consumed exactly once. The crate's
CPU code folds partial rounds three at a time (`NB_SKIPS_CPU = 2`), an
algebraic rewrite of the same round function.

`absorbAll` adds elements in pairs onto registers 0 and 1 and permutes after
each pair; a trailing singleton goes to register 0. `poseidonHash(Xs)`
starts from `ps(0, 0, size(Xs) modInt #r)` and returns register 0. An empty
input runs no permutation and hashes to `0`, as in the crate
(`poseidon_cpu.rs` iterates over an empty queue); `unit_hash.py` checks this
case. `transientCommit(Vs, Rand)` is `poseidonHash(ListItem(Rand) Vs)`, the
opening followed by the values (see `06-configuration-and-run-lifecycle.md`).
`varLenSponge(Xs)`, used by hash-to-curve, starts with capacity `2^64` and
absorbs `Xs` followed by `size(Xs)` as padding, matching
`PoseidonChip::init(None)` in `poseidon_cpu.rs`.

## Hash to curve

`hashToCurve(Xs)` runs `varLenSponge`, maps registers 0 and 1 through
`mapToCurve` and adds the two Jubjub points. `mapToCurve(U)` is
Shallue-van de Woestijne on the Weierstrass form of Jubjub (`#svdw`), then
`#wToMont` (`x' = x K - J/3`, `y' = y K`), then `#edFromMont`, then
multiplication by the cofactor 8. `#svdwZ`, `#svdwA`, `#svdwB`, `#montJ` and
`#montK` come from midnight-circuits `ecc/hash_to_curve/mtc_params.rs`;
`#svdwZ = #r - 2` and `#montJ = 40962`. With `g(x) = x^3 + A x + B`, the
derived constants are `c1 = g(Z)`, `c2 = -Z/2`, `c3` the even square root of
`-g(Z)(3 Z^2 + 4 A)` (`#evenRoot`) and `c4 = -4 g(Z) / (3 Z^2 + 4 A)`.
`#svdwPick` takes the first of `x1, x2, x3` whose `g`-image is not a
non-residue; `#svdwSign` takes the root of `g(x)` with the parity of `U`;
`#edFromMont2` uses `finv` as `inv0` and sets `y' = 1` when the inverse is
zero, as `mtc_cpu.rs` does.

## SHA-256, Keccak-256 and SHA-512

`sha256Bytes` is FIPS 180-4 SHA-256: 32-bit words, 64 rounds, 64-byte
blocks, padding `0x80`, zeros, then an 8-byte big-endian bit length. The 64
constants are `#shaKs`; the initial hash begins `1779033703` (`0x6a09e667`).
`persistent_hash` consumes the 32-byte digest as `bytes32` (`zkir-vm.k`,
`#stdHash3`).

`keccak256Bytes` is Keccak-f[1600] with rate 1088 bits (136-byte blocks) and
the pre-NIST padding `0x01 .. 0x80`, not the SHA-3 suffix `0x06`. The state
is a `List` of 25 lanes, 64-bit little-endian words at index `x + 5 y`;
`#keccakRCs` holds the 24 round constants and `#keccakRots` the 25 rotation
offsets. A round is θ, ρ+π, χ, ι (`keccakF`); the squeeze takes the first
four lanes, truncated to 32 bytes.

`sha512Bytes` (`ZKIR-SHA512`, extension only) is FIPS 180-4 SHA-512: 64-bit
words, 80 rounds, 128-byte blocks, padding `0x80`, zeros, then a 16-byte
big-endian bit length, with `#sha512Ks` and `#sha512H0` in
`zkir-sha512-constants.k` (first `K` word `0x428a2f98d728ae22`, first `H`
word `0x6a09e667f3bcc908`). The instruction `sha512` uses the same
`alignedBytes` as the base hashes and stores `bytesV(sha512Bytes(B))`, a
64-byte value rather than `Bytes32` (see `09-extension-surface.md`).

`rotr32` with a shift outside `[0, 32)` returns its argument masked to 32
bits, as does `rotl64` outside `(0, 64)`. `rotr64` has no guard; its call
sites pass shifts between 1 and 61.

## Totality and out-of-domain results

Every function above carries `[function, total]`. The attribute is a
declaration, not a proof: the compiler does not check it, and a call that
matches no rule has no result (the compiled interpreter aborts). The table
gives what the rules return outside the intended domain.

| Function | Intended domain | Off-domain result |
|---|---|---|
| `fadd`, `fsub`, `fmul`, `fmod`, `fneg`, `finv`, `fdiv`, `fsq` | `P > 1` | `0` (`[owise]`) |
| `fpow` | `P > 1` and `E >= 0` | `0` (`[owise]`) |
| `finv` / `fdiv` | non-zero divisor | `0` (explicit rule for `A modInt P ==Int 0`) |
| `legendre` | prime `P` | `0` for `P <= 1` (through `fpow`; no `[owise]` rule needed) |
| `isQuadraticNonResidue` | prime `P` | `0 ==Int P -Int 1`: `false` for `P <= 0`, `true` for `P = 1` |
| `fsqrt` | residue modulo a configured prime | see below |
| `#nonResidue` | a non-residue exists in `[Z, P)` | `0` if `Z >= P` |
| `bitAt`, `lowBits`, `highBits` | `A >= 0`, width or index `>= 0` | `0` |
| `fitsBits` | width `>= 0` | `false` |
| `ecMul` | `K > 0` | `identity(C)` for `K <= 0` |
| `fromXY` | on-curve coordinates in `[0, P)` | `ptErr("point is not on the curve")` |
| `jubjubFromXY` | `y` on Jubjub, ZIP 216 encoding | the three `ptErr` strings above |
| `poseidonRC`, `poseidonMDS`, `shaK`, `keccakRC`, `keccakRot` | indices in range | `0` |
| `#evenRoot(noSqrt())` | `c3` radicand is a residue | `0` |
| `#svdwSign(_, X, noSqrt())` | `g(x)` is a residue | `pt(X, 0)` (unreachable) |

`fsqrt` has no `[owise]` rule. Its guards evaluate `A modInt P`, which K
defines for every non-zero divisor (the remainder lies in `[0, |P|)`), so
the three rules give, for every modulus:

- `P = 0`: no rule applies; the function is undefined.
- `P = 1`: `sqrtOk(0)`, because every `A modInt 1` is `0`.
- `P < 0`: `sqrtOk(0)` when `P` divides `A`, otherwise `noSqrt()`, because
  `legendre` is `0`.
- `P > 1`: a rule always applies and the loop terminates, but the answer is
  correct only for prime `P` (`fsqrt(4, 15)` is `noSqrt()`).

No caller passes a modulus other than the eight primes.

The hash interfaces also restrict the shape of their collections.
`absorbAll` matches `ListItem(A)` with `A` used as an `Int`, so
`poseidonHash`, `transientCommit`, `varLenSponge` and `hashToCurve` are
defined on lists of `Int` only; any other element sort matches no rule.
`keccakF` reads lanes with `lane(S, X, Y) => {S[...]}:>Int`, so it needs
exactly 25 `Int` lanes (`keccakF(.List)` has no result), and `#shaRound` and
`#sha512Round` need an eight-word working list. The byte-level entry points
`sha256Bytes`, `keccak256Bytes` and `sha512Bytes` build these collections
themselves and are total on `Bytes`.

## How the primitives are checked

`tools/unit_values.py` evaluates terms through `ZKIR-TEST` (`zkir-test.k`)
against an independent Python implementation. Of its 43 checks, 23 are field
and curve cases: `finv`, `fsqrt` of 4, non-residuosity of both Edwards `d`
constants, `legendre(3, #r)`, `#jubjubD`, each of the four generators on its
curve, `#jubjubGenerator = 8 G`, `8 G` in and `G` outside the Jubjub
subgroup, the Curve25519 generator in its subgroup, a scalar multiple on
each curve, `n G = inf()` on both Weierstrass groups, and the three
`jubjubFromXY` parity cases; the other 20 are encodings
(`04-values-and-encoding.md`). `evidence/zkir-k-unit-values-2026-09-05c.txt`
records `43/43 checks passed`.

`tools/unit_hash.py` compares K with the crate's `preprocess` through the
oracle on `corpus/handmade/{transient_hash,std_hashes}.zkir`. Its 18 checks
are Poseidon of length 0 to 3; `hashToCurve` of length 0, 1 and 3; SHA-256
and Keccak-256 of 1-byte and 32-byte aligned preimages against the oracle
(the 1-byte SHA-256 also against `hashlib`, and the oracle's own 32-byte
SHA-256 register against `hashlib`); SHA-256 of a field-plus-`bytes(6)`
preimage; Keccak-256 of a 232-byte (`bytes(200)` plus field) preimage; and
three `alignedBytes` layout checks. `evidence/zkir-k-unit-hash-2026-09-05c.txt`
records `18/18 checks passed`. SHA-512 is absent because `ZKIR-TEST` does not
import `ZKIR-SHA512`; `corpus/midnight-zkir-2ffe2d1-tests/test_sha512_proof.zkir`
exercises it in the extension differential harness
(`12-oracles-and-differential-testing.md`).

Both tools need the compiled `semantics/zkir-test-kompiled/`; `unit_hash.py`
also needs the oracle binary at
`~/Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle`. A
mismatch prints a `FAIL` line with the expected and actual terms and exits
with status 1 (see `11-tooling-reference.md`). From the repository root:

```bash
uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py
uv run --group zkir-k python experiments/zkir-k/tools/unit_hash.py
```

The Rust references are midnight-curves 0.3.1 (`jubjub/`, `bls12_381/`,
`curve25519/`, `k256/`, `p256/`) and midnight-circuits 7.2.4
(`hash/poseidon/`, `ecc/hash_to_curve/`, `ecc/curves.rs`) in the cargo
registry; SHA-256, SHA-512 and Keccak-256 follow FIPS 180-4 and
Keccak-f[1600] directly.
