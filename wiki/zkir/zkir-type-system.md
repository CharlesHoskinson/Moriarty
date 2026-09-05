---
id: zkir.type-system
type: language
title: ZKIR Type System
status: active
updated_at: 2026-09-03T14:38:56Z
sources:
  - SRC-0006
  - SRC-0025
---

# ZKIR Type System

## Overview and Architectural Role

The Zero-Knowledge Intermediate Representation (ZKIR) type system governs the representation, manipulation, and verification of mathematical values across two distinct execution domains: off-circuit witness generation (`preprocess`) and in-circuit constraint synthesis (`synthesize`) (CLM-0425; SRC-0025 docs/zkir-v3-spec.md:L40-60; source fact; unperformed; high; S5). In ZKIR, types serve not only as static checking annotations for compiler passes, but also as prescriptive arithmetization specifications that dictate how many native field cells must be allocated in the Halo2 constraint matrix, which specialized cryptographic chips must be synthesized, and which range constraints and canonicity checks must be enforced (CLM-0425; SRC-0025 docs/zkir-v3-spec.md:L65-90; source fact; unperformed; high; S5).

The authoritative textual specification of ZKIR version 3 is established in `docs/zkir-v3-spec.md` under repository `input-output-hk/arc-zkir` at commit `fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9` (SRC-0025) (CLM-0426; SRC-0025 docs/zkir-v3-spec.md:L1-20; source fact; unperformed; high; S5). The specification establishes a thirteen-type surface organized around the scalar field of the BLS12-381 pairing curve and four elliptic curve groups: Jubjub, Secp256k1, Secp256r1 (P-256), and Curve25519 (CLM-0426; SRC-0025 docs/zkir-v3-spec.md:L100-145; source fact; unperformed; high; S5). This exact thirteen-type surface is mirrored in the historical extract from `midnight-ledger` commit `92e8bdd3` (`ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src/ir_types.rs`) (CLM-0426; SRC-0006 midnight-ledger 92e8bdd3:zkir-v3/src/ir_types.rs:L1-115; repository observation; inspection; high; S5).

In contrast, the standalone `midnightntwrk/midnight-zkir` repository at branch `zkir-v3` (commit `2ffe2d17bbb736aec36fb300aeaca679a10d2278`, dated 2026-09-02) provides an extended fifteen-type surface (CLM-0427; SRC-0006 zkir/src/ir_types.rs:L25-75; repository observation; inspection; high; S5). The standalone crate promotes boolean values and single bytes to first-class types (`Bool` and `Byte`) and replaces the fixed-size `Bytes32` type with a dynamically parameterized `Bytes(u32)` type representing arbitrary byte buffers up to $2^{24}$ bytes (CLM-0427; SRC-0006 zkir/src/ir_types.rs:L38-42; repository observation; inspection; high; S5).

```
+----------------------------------------------------------------------------------------------------+
|                                    ZKIR Type Surface Hierarchy                                     |
+----------------------------------------------------------------------------------------------------+
|                                                                                                    |
|    Authoritative Surface (SRC-0025 / 92e8bdd3: 13 Types)                                           |
|    +------------------------------------------------------------------------------------------+    |
|    | Native (Scalar<BLS12-381>)                                                               |    |
|    | Bytes32 (Bytes<32>)                                                                      |    |
|    | Jubjub Group:          JubjubPoint, JubjubScalar                                         |    |
|    | Secp256k1 Triple:      Secp256k1Point, Secp256k1Base, Secp256k1Scalar                    |    |
|    | Secp256r1 Triple:      Secp256r1Point, Secp256r1Base, Secp256r1Scalar                    |    |
|    | Curve25519 Triple:     Curve25519Point, Curve25519Base, Curve25519Scalar                |    |
|    +------------------------------------------------------------------------------------------+    |
|                                 |                                                                  |
|                                 v Generalization in 2ffe2d1 (15 Types)                             |
|    +------------------------------------------------------------------------------------------+    |
|    | Promoted Primitives:   Bool, Byte                                                        |    |
|    | Parameterized Buffers: Bytes(u32) (Generalizes Bytes32, capped at 2^24 bytes)             |    |
|    +------------------------------------------------------------------------------------------+    |
|                                                                                                    |
+----------------------------------------------------------------------------------------------------+
```

## The Authoritative Thirteen-Type Surface

The authoritative specification defines thirteen fundamental types. Each type corresponds to a distinct algebraic or binary structure (CLM-0428; SRC-0025 docs/zkir-v3-spec.md:L105-150; source fact; unperformed; high; S5).

```
+----------------------------------------------------------------------------------------------------+
|                                Authoritative 13-Type Inventory                                     |
+-------------------+----------------------+-------------+-------------+-----------------------------+
| Spec Type Name    | Rust Enum Variant    | Wire Tag    | Encoded Len | Mathematical Sort           |
+-------------------+----------------------+-------------+-------------+-----------------------------+
| Native            | Native               | Native      | 1           | F_r (BLS12-381 scalar field)|
| Bytes32           | Bytes32              | Bytes32     | 2           | 32-byte binary octet string |
| JubjubPoint       | JubjubPoint          | JubjubPoint | 2           | Jubjub prime subgroup point |
| JubjubScalar      | JubjubScalar         | JubjubScalar| 1           | F_s (Jubjub scalar field)   |
| Secp256k1Point    | Secp256k1Point       | Secp256k1Pt | 5           | Secp256k1 curve point       |
| Secp256k1Base     | Secp256k1Base        | Secp256k1B  | 2           | F_p (Secp256k1 base field)  |
| Secp256k1Scalar   | Secp256k1Scalar      | Secp256k1S  | 2           | F_q (Secp256k1 scalar field)|
| Secp256r1Point    | Secp256r1Point       | Secp256r1Pt | 5           | P-256 curve point           |
| Secp256r1Base     | Secp256r1Base        | Secp256r1B  | 2           | F_p (P-256 base field)      |
| Secp256r1Scalar   | Secp256r1Scalar      | Secp256r1S  | 2           | F_q (P-256 scalar field)    |
| Curve25519Point   | Curve25519Point      | Curve25519Pt| 4           | Curve25519 Edwards point    |
| Curve25519Base    | Curve25519Base       | Curve25519B | 2           | F_p (Curve25519 base field) |
| Curve25519Scalar  | Curve25519Scalar     | Curve25519S | 2           | F_l (Curve25519 scalar field|
+-------------------+----------------------+-------------+-------------+-----------------------------+
```

### 1. `Native` (BLS12-381 Scalar Field)

The `Native` type represents an element of the scalar field $\mathbb{F}_r$ of the BLS12-381 elliptic curve construction (CLM-0429; SRC-0025 docs/zkir-v3-spec.md:L160-175; source fact; unperformed; high; S5). It forms the primary computational domain of the Midnight proving system. All PLONKish gate equations, public inputs, and permutation arguments are defined directly over this field (CLM-0429; SRC-0006 zkir/src/ir_instructions/mod.rs:L18-25; repository observation; inspection; high; S5).

The BLS12-381 scalar field modulus is the 255-bit prime integer:

$$r = 52435875175126190479447740508185965837690552500527637822603658699938581184513$$

In hexadecimal notation:

$$r = \text{0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001}$$

The field order satisfies $2^{254} < r < 2^{255}$. In-circuit, each `Native` value occupies exactly one assigned cell in an advice or fixed column. Its encoded length is 1 (`encoded_len() == 1`) (CLM-0429; SRC-0006 zkir/src/ir_types.rs:L85-90; repository observation; inspection; high; S5).

### 2. `Bytes32` (Fixed 32-Byte Array)

The `Bytes32` type represents a contiguous sequence of exactly thirty-two raw bytes ($[u8; 32]$) (CLM-0430; SRC-0025 docs/zkir-v3-spec.md:L180-195; source fact; unperformed; high; S5). It serves as the primary data container for cryptographic digests (such as SHA-256 and Keccak-256 outputs), contract state keys, serialized public keys, and cross-contract payload chunks (CLM-0430; SRC-0006 zkir/src/ir_instructions/into_bytes32.rs:L30-40; repository observation; inspection; high; S5).

Because 32 bytes contain $32 \times 8 = 256$ bits of information, a 32-byte value cannot fit within a single BLS12-381 scalar field element without risk of modular overflow, since $256 > \log_2(r) \approx 254.85$ (CLM-0430; SRC-0025 docs/zkir-v3-spec.md:L200-215; source fact; unperformed; high; S5). Consequently, `Bytes32` values are decomposed across two native field elements for serialization and transcript communication (`encoded_len() == 2`):
- The lower limb encodes the first 31 bytes (248 bits), which strictly satisfies $2^{248} < r$.
- The upper limb encodes the 32nd byte (8 bits), satisfying $2^8 < r$.

In-circuit, `Bytes32` is represented as an array of thirty-two `AssignedByte<outer::Scalar>` cells, each constrained to the range $[0, 255]$ via lookup tables (CLM-0430; SRC-0006 zkir/src/ir_types.rs:L180-195; repository observation; inspection; high; S5).

### 3. Jubjub Curve Types (`JubjubPoint` and `JubjubScalar`)

Jubjub is a twisted Edwards curve defined over the scalar field $\mathbb{F}_r$ of BLS12-381 (CLM-0431; SRC-0025 docs/zkir-v3-spec.md:L220-250; source fact; unperformed; high; S5). Because the base field of Jubjub is identical to the native circuit field $\mathbb{F}_r$, group operations on Jubjub points are exceptionally efficient and do not require non-native arithmetic emulation (CLM-0431; SRC-0006 zkir/src/ir_instructions/ec_mul.rs:L35-45; repository observation; inspection; high; S5).

The curve equation for Jubjub in twisted Edwards form is:

$$-x^2 + y^2 = 1 + d \cdot x^2 \cdot y^2$$

where $d = -(10240 / 10241) \pmod r$.

The order of the full curve is $8 \cdot s$, where the prime subgroup order $s$ is the 252-bit prime integer:

$$s = 655448439689077380993096756352324572970592126587231728136515949875690759883$$

In hexadecimal notation:

$$s = \text{0x0e7db4ea6533afa906673b0101343b00a6682093ccc81082d0970e5ed6f72cb7}$$

- `JubjubPoint`: Represents a point on the prime-order subgroup of Jubjub (CLM-0431; SRC-0006 zkir/src/ir_types.rs:L45-50; repository observation; inspection; high; S5). In affine coordinates $(x, y) \in \mathbb{F}_r^2$, both coordinates are native field elements. The identity element is $(0, 1)$. Its encoded length is 2 (`encoded_len() == 2`), serializing $x$ followed by $y$.
- `JubjubScalar`: Represents a scalar in $\mathbb{F}_s$ (CLM-0431; SRC-0006 zkir/src/ir_types.rs:L51-55; repository observation; inspection; high; S5). Its encoded length is 1 (`encoded_len() == 1`). Because $s < r$, any canonical integer in $\mathbb{F}_s$ fits within a single native field element.

### 4. Non-Native Curve Triples: Secp256k1, Secp256r1, and Curve25519

Midnight circuits support three external elliptic curves to enable signature verification, address derivation, and interoperability with Bitcoin, Ethereum, TLS, and modern cryptographic protocols (CLM-0432; SRC-0025 docs/zkir-v3-spec.md:L255-285; source fact; unperformed; high; S5). Because these curves are defined over prime fields distinct from BLS12-381, arithmetic operations on their coordinates and scalars require non-native field emulation via multi-limb representations and range-check lookup tables (CLM-0432; SRC-0006 zkir/src/ir_vm.rs:L1480-1510; repository observation; inspection; high; S5).

Each curve is represented by a consistent triple of types: a Point type, a Base field type, and a Scalar field type (CLM-0432; SRC-0025 docs/zkir-v3-spec.md:L115-135; source fact; unperformed; high; S5).

#### (a) Secp256k1 (Koblitz Curve, Bitcoin / Ethereum)
Secp256k1 is a short Weierstrass curve defined by the equation:

$$y^2 = x^3 + 7$$

over the 256-bit prime base field $\mathbb{F}_p$:

$$p_{\text{secp256k1}} = 2^{256} - 2^{32} - 977 = \text{0xfffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f}$$

Its prime group order $q_{\text{secp256k1}}$ is:

$$q_{\text{secp256k1}} = \text{0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141}$$

- `Secp256k1Base`: Represents an element of $\mathbb{F}_p$. Encodes to 2 native elements (`encoded_len() == 2`) by splitting the 256-bit integer into a 248-bit lower limb and an 8-bit upper limb (CLM-0432; SRC-0006 zkir/src/ir_instructions/encode.rs:L35-50; repository observation; inspection; high; S5).
- `Secp256k1Scalar`: Represents an element of $\mathbb{F}_q$. Encodes to 2 native elements (`encoded_len() == 2`) using the same 248-bit / 8-bit split.
- `Secp256k1Point`: Represents an affine point $(x, y) \in \mathbb{F}_p^2$ or the point at infinity $\mathcal{O}$ (CLM-0433; SRC-0025 docs/zkir-v3-spec.md:L290-305; source fact; unperformed; high; S5). Encodes to 5 native elements (`encoded_len() == 5`):
  1. $x$ lower limb (248 bits)
  2. $x$ upper limb (8 bits)
  3. $y$ lower limb (248 bits)
  4. $y$ upper limb (8 bits)
  5. $\text{is\_identity}$ boolean flag ($1$ if the point is $\mathcal{O}$, and $0$ otherwise)

#### (b) Secp256r1 (NIST P-256 / prime256v1)
Secp256r1 is a short Weierstrass curve defined by the equation:

$$y^2 = x^3 - 3x + b$$

over the 256-bit prime base field $\mathbb{F}_p$:

$$p_{\text{secp256r1}} = 2^{256} - 2^{224} + 2^{192} + 2^{96} - 1 = \text{0xffffffff00000001000000000000000000000000ffffffffffffffffffffffff}$$

Its prime group order $q_{\text{secp256r1}}$ is:

$$q_{\text{secp256r1}} = \text{0xffffffff00000000ffffffffffffffffbce6faada7179e84f3b9cac2fc632551}$$

- `Secp256r1Base`: Encodes to 2 native elements (`encoded_len() == 2`).
- `Secp256r1Scalar`: Encodes to 2 native elements (`encoded_len() == 2`).
- `Secp256r1Point`: Encodes to 5 native elements (`encoded_len() == 5`), structured identically to `Secp256k1Point` with limbs for $x$, limbs for $y$, and an $\text{is\_identity}$ flag (CLM-0433; SRC-0025 docs/zkir-v3-spec.md:L295-305; source fact; unperformed; high; S5).

#### (c) Curve25519 (Twisted Edwards / Ed25519)
Curve25519 is a twisted Edwards curve birationally equivalent to Montgomery curve $y^2 = x^3 + 486662x^2 + x$ (CLM-0434; SRC-0025 docs/zkir-v3-spec.md:L306-320; source fact; unperformed; high; S5). In Edwards form, its equation is:

$$-x^2 + y^2 = 1 - \frac{121665}{121666} x^2 y^2$$

over the 255-bit prime base field:

$$p_{\text{curve25519}} = 2^{255} - 19 = \text{0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffed}$$

Its prime subgroup order is:

$$\ell = 2^{252} + 27742317777372353535851937790883648493 = \text{0x1000000000000000000000000000000014def9dea2f79cd65812631a5cf5d3ed}$$

- `Curve25519Base`: Encodes to 2 native elements (`encoded_len() == 2`).
- `Curve25519Scalar`: Encodes to 2 native elements (`encoded_len() == 2`).
- `Curve25519Point`: Represents a point in the prime-order subgroup of Curve25519. In twisted Edwards coordinates, the identity element is the regular affine point $(0, 1)$. Because the identity possesses well-defined affine coordinates, Curve25519 does not require an identity flag (CLM-0434; SRC-0006 zkir/src/ir_instructions/into_coordinates.rs:L68-73; repository observation; inspection; high; S5). Consequently, `Curve25519Point` encodes to exactly 4 native elements (`encoded_len() == 4`), comprising two limbs for $x$ and two limbs for $y$.

## Weierstrass vs Edwards Identity Representation

A major asymmetry exists in the ZKIR type system between short Weierstrass curves (`Secp256k1`, `Secp256r1`) and twisted Edwards curves (`Jubjub`, `Curve25519`) (CLM-0435; SRC-0025 docs/zkir-v3-spec.md:L290-325; source fact; unperformed; high; S5).

In short Weierstrass curves, the group identity $\mathcal{O}$ is the point at infinity in projective space, which cannot be represented by any finite affine coordinate pair $(x, y) \in \mathbb{F}_p^2$ (CLM-0435; SRC-0006 zkir/src/ir_instructions/into_coordinates.rs:L50-64; repository observation; inspection; high; S5). Therefore:
1. The point types `Secp256k1Point` and `Secp256r1Point` require five native field elements during serialization: $(x_{\text{low}}, x_{\text{high}}, y_{\text{low}}, y_{\text{high}}, \text{is\_identity})$. When $\text{is\_identity} = 1$, the coordinate limbs are ignored or set to zero.
2. The coordinate extraction instruction `into_coordinates` cannot evaluate on $\mathcal{O}$. Off-circuit, attempting to extract coordinates from $\mathcal{O}$ fails with:
   ```text
   "Cannot extract coordinates of the Secp256k1 identity"
   ```
   In-circuit, `into_coordinates` enforces $\text{is\_identity} = 0$ via `curve.assert_non_zero(layouter, p)`, rendering the circuit unsatisfiable if evaluated on the identity (CLM-0435; SRC-0006 zkir/src/ir_instructions/into_coordinates.rs:L110-124; repository observation; inspection; high; S5).

In twisted Edwards curves, the neutral element is the affine point $(0, 1)$ on Jubjub and Curve25519 (CLM-0435; SRC-0006 zkir/src/ir_instructions/into_coordinates.rs:L68-73; repository observation; inspection; high; S5). Both coordinates exist as regular field elements. Hence:
1. `Curve25519Point` requires only four native field elements $(x_{\text{low}}, x_{\text{high}}, y_{\text{low}}, y_{\text{high}})$.
2. `JubjubPoint` requires only two native field elements $(x, y)$.
3. Neither curve requires an identity flag, and `into_coordinates` operates unconditionally on all valid subgroup points, including the identity.

```
+----------------------------------------------------------------------------------------------------+
|                               Curve Identity & Limb Allocation Table                               |
+------------------+-------------------+-----------------+-------------+-----------------------------+
| Curve Family     | Model Form        | Identity Affine | Limbs (x, y)| Identity Flag Present?      |
+------------------+-------------------+-----------------+-------------+-----------------------------+
| Jubjub           | Twisted Edwards   | (0, 1)          | (1, 1)      | No (Total: 2 elements)      |
| Curve25519       | Twisted Edwards   | (0, 1)          | (2, 2)      | No (Total: 4 elements)      |
| Secp256k1        | Short Weierstrass | Point at Inf    | (2, 2)      | Yes (Total: 5 elements)     |
| Secp256r1        | Short Weierstrass | Point at Inf    | (2, 2)      | Yes (Total: 5 elements)     |
+------------------+-------------------+-----------------+-------------+-----------------------------+
```

## Repository Type System Extensions (commit `2ffe2d1`)

In commit `2ffe2d1`, the standalone `midnight-zkir` crate introduced several structural revisions to `ir_types.rs` that extend the type system beyond the thirteen-type surface of SRC-0025 (CLM-0436; SRC-0006 zkir/src/ir_types.rs:L25-75; repository observation; inspection; high; S5).

```rust
pub enum IrType {
    Native,
    Bool,
    Byte,
    Bytes(u32),
    JubjubPoint,
    JubjubScalar,
    Secp256k1Point,
    Secp256k1Base,
    Secp256k1Scalar,
    Secp256r1Point,
    Secp256r1Base,
    Secp256r1Scalar,
    Curve25519Point,
    Curve25519Base,
    Curve25519Scalar,
}
```

### 1. First-Class `Bool`
In the authoritative v3 specification, boolean values are uncommitted abstractions typically represented as `Native` values constrained to $\{0, 1\}$ via `constrain_to_boolean` (CLM-0436; SRC-0025 docs/zkir-v3-spec.md:L110; source fact; unperformed; high; S5). In `2ffe2d1`, `Bool` is an explicit, first-class type:
- Runtime value variant: `IrValue::Bool(bool)`.
- Circuit representation: `CircuitValue::Bool(AssignedBit<F>)`.
- Encoded length: 1 (`encoded_len() == 1`), serializing to native $0$ or $1$.
- Supports logical negation via `neg` and n-ary boolean operations via `and`, `or`, and `xor`.

### 2. First-Class `Byte`
In `2ffe2d1`, single octet values are represented by `Byte`:
- Runtime value variant: `IrValue::Byte(u8)`.
- Circuit representation: `CircuitValue::Byte(AssignedByte<F>)`.
- Encoded length: 1 (`encoded_len() == 1`), serializing to a native field element in $[0, 255]$.
- Supports addition modulo 256 via `add` and conditional selection via `cond_select`.

### 3. Parameterized `Bytes(u32)`
The fixed `Bytes32` type of the specification is generalized in `2ffe2d1` to dynamically sized byte arrays `Bytes(u32)` (CLM-0437; SRC-0006 zkir/src/ir_types.rs:L38-42; repository observation; inspection; high; S5).
- Runtime value variant: `IrValue::Bytes(Vec<u8>)`.
- Maximum length bound: `MAX_BYTES_LEN = 1 << 24 = 16_777_216` bytes (CLM-0437; SRC-0006 zkir/src/ir_types.rs:L38-40; repository observation; inspection; high; S5). Any attempt to construct or concatenate bytes exceeding this bound raises a fatal error.
- Encoded length calculation:
  $$\text{encoded\_len}(\text{Bytes}(n)) = \lceil n / 31 \rceil + [n = 0]$$
  Specifically, bytes are packed into 31-byte chunks, each chunk occupying one native field element ($31 \times 8 = 248 < 254$). A stray final chunk of $k \le 31$ bytes occupies an additional native element. An empty buffer `Bytes(0)` encodes to 1 field element ($0$). For $n = 32$, this yields $\lceil 32 / 31 \rceil = 2$ field elements, matching `Bytes32`.

## Value Encoding, Decoding, and Canonicity Verification

ZKIR defines a canonical mapping between high-level typed values and sequences of native field elements ($\mathbb{F}_r$) for transcript I/O, public inputs, and communications commitments (CLM-0438; SRC-0006 zkir/src/ir_instructions/encode.rs:L28-100; repository observation; inspection; high; S5).

### Encoding Algorithm (`encode_offcircuit`)

The function `encode_offcircuit(val: &IrValue) -> Vec<IrValue>` evaluates as follows (CLM-0438; SRC-0006 zkir/src/ir_instructions/encode.rs:L30-85; repository observation; inspection; high; S5):
1. `Native(x)` $\to [x]$.
2. `Bool(b)` $\to [b \text{ as } Fr]$.
3. `Byte(b)` $\to [b \text{ as } Fr]$.
4. `Bytes(bs)` $\to$ Splits `bs` into chunks of 31 bytes. Each 31-byte chunk is parsed as a little-endian integer and emitted as a `Native` field element.
5. `JubjubPoint(p)` $\to [x, y]$, where $(x, y)$ are the affine coordinates.
6. `JubjubScalar(s)` $\to [s \text{ as } Fr]$, embedding the 252-bit integer into $\mathbb{F}_r$.
7. Foreign Base and Scalar elements $\to [x_{\text{low}}, x_{\text{high}}]$, where $x_{\text{low}} = x \pmod{2^{248}}$ and $x_{\text{high}} = \lfloor x / 2^{248} \rfloor$.
8. Weierstrass Points $\to [x_{\text{low}}, x_{\text{high}}, y_{\text{low}}, y_{\text{high}}, \text{is\_identity}]$.
9. Curve25519 Points $\to [x_{\text{low}}, x_{\text{high}}, y_{\text{low}}, y_{\text{high}}]$.

### Decoding Algorithm (`decode_offcircuit`)

The function `decode_offcircuit(raw: &[Fr], target_type: &IrType) -> Result<IrValue>` inverts the encoding mapping (CLM-0439; SRC-0006 zkir/src/ir_instructions/encode.rs:L180-280; repository observation; inspection; high; S5). It verifies that:
- Coordinates satisfy curve equations.
- Decoded points belong to the prime-order subgroup.
- Field elements reconstructed from limbs do not exceed the target field modulus.

### Upstream PR #679 Fix: Canonicity Roundtrip Assertion

A major vulnerability discovered in earlier ZKIR versions was that non-canonical field encodings (such as limb values that reconstruct integers $\ge$ the target modulus or non-canonical byte representations) could be accepted during `decode_offcircuit` without triggering an error (CLM-0440; SRC-0025 docs/zkir-v3-divergence-review.md:L120-170; source fact; unperformed; high; S5).

In `midnight-zkir` commit `2ffe2d1`, upstream PR #679 was merged into `decode_offcircuit` (CLM-0440; SRC-0006 zkir/src/ir_instructions/encode.rs:L285-305; repository observation; inspection; high; S5). The decoding routine now performs an explicit roundtrip verification:

```rust
let value = decode_inner(raw, target_type)?;
let re_encoded = encode_offcircuit(&value);
let re_encoded_fr: Vec<Fr> = re_encoded
    .into_iter()
    .map(|v| v.try_into())
    .collect::<Result<_, _>>()?;

if raw != re_encoded_fr {
    bail!(
        "Canonical decoding failed for {:?}: input field elements do not match re-encoded canonical form",
        target_type
    );
}
```

This ensures that any input vector containing non-canonical or unnormalized field elements is rejected during witness generation, guaranteeing bijectivity between canonical memory values and their wire representations.

## Subtyping, Coercions, and Type Safety

The ZKIR type system enforces strict type isolation:
1. **No Implicit Coercions:** There are no implicit promotions or subtyping coercions. An operand of type `Bool` cannot be passed to an instruction expecting `Native`, nor can `Native` be used where `JubjubScalar` is expected (CLM-0441; SRC-0025 docs/zkir-v3-spec.md:L130-140; source fact; unperformed; high; S5).
2. **Explicit Conversion Instructions:** All cross-type movements require explicit instructions:
   - `jubjub_scalar_from_native`: Explicit modular reduction from $\mathbb{F}_r$ to $\mathbb{F}_s$.
   - `into_bytes32` and `from_bytes32`: Explicit decomposition and reconstitution between field elements and byte buffers.
   - `into_coordinates` and `from_coordinates`: Explicit conversion between group points and affine field coordinate pairs.
   - `encode`: Explicit conversion from any high-level value to a vector of native field elements.
3. **Mismatched Operands:** Instructions taking multiple operands (such as `add`, `mul`, `constrain_eq`, and `cond_select`) require that both operands share the exact same type. Passing mismatched types results in an immediate preprocessing error:
   ```text
   "Unsupported cond_select: <TypeA> ? <TypeB>"
   ```

## Implications for Formal Semantics in the K Framework

Modeling the ZKIR type system in the K Framework involves defining concrete sorts and semantic rules:

```k
module ZKIR-TYPES
  imports INT
  imports BOOL
  imports BYTES

  // Type Identifiers
  syntax IrType ::= "Native"
                  | "Bytes32"
                  | "JubjubPoint"   | "JubjubScalar"
                  | "Secp256k1Point" | "Secp256k1Base" | "Secp256k1Scalar"
                  | "Secp256r1Point" | "Secp256r1Base" | "Secp256r1Scalar"
                  | "Curve25519Point"| "Curve25519Base"| "Curve25519Scalar"
                  | "Bool" | "Byte" | BytesType(Int)

  // Runtime Values
  syntax IrValue ::= NativeVal(Int)
                   | BoolVal(Bool)
                   | ByteVal(Int)
                   | BytesVal(Bytes)
                   | PointVal(x: Int, y: Int, isIdentity: Bool)
                   | ScalarVal(Int)

  // Subgroup and Modulus Checks
  syntax Bool ::= isValidNative(Int) [function]
  rule isValidNative(X) => X >=Int 0 andBool X <Int 52435875175126190479447740508185965837690552500527637822603658699938581184513

  syntax Int ::= encodedLen(IrType) [function]
  rule encodedLen(Native) => 1
  rule encodedLen(Bytes32) => 2
  rule encodedLen(JubjubPoint) => 2
  rule encodedLen(JubjubScalar) => 1
  rule encodedLen(Secp256k1Point) => 5
  rule encodedLen(Secp256k1Base) => 2
  rule encodedLen(Secp256k1Scalar) => 2
  rule encodedLen(Secp256r1Point) => 5
  rule encodedLen(Secp256r1Base) => 2
  rule encodedLen(Secp256r1Scalar) => 2
  rule encodedLen(Curve25519Point) => 4
  rule encodedLen(Curve25519Base) => 2
  rule encodedLen(Curve25519Scalar) => 2
  rule encodedLen(Bool) => 1
  rule encodedLen(Byte) => 1
endmodule
```

Key considerations for K Framework integration:
1. **Mathematical Builtin Mapping:** Large primes ($r, s, p, q, \ell$) exceed standard machine integer ranges and must be represented using K's arbitrary-precision integer builtin `Int` (CLM-0442; SRC-0006 k-rust/crates/k-rust-backend/src/builtins.rs; repository observation; inspection; high; S4). Refer to [K Builtins](../k-framework/k-builtins.md).
2. **Canonicity Invariant:** In K rules, every field element must be normalized modulo its respective prime modulus at every rewrite step to preserve canonicity.
3. **Edwards vs Weierstrass Point Sorts:** Point constructors in K must account for the identity flag on Weierstrass curves while allowing direct affine coordinate manipulation on Jubjub and Curve25519.

For detailed execution and constraint semantics, refer to [ZKIR Instruction Set](zkir-instruction-set.md) and [ZKIR Virtual Machine Semantics](zkir-vm-semantics.md).
