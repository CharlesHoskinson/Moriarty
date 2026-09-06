---
id: zkir.instruction-set
type: language
title: ZKIR Instruction Set
status: active
updated_at: 2026-09-03T14:38:22Z
sources:
  - SRC-0006
  - SRC-0038
---

# ZKIR Instruction Set

## Overview and Core Execution Philosophy

The Zero-Knowledge Intermediate Representation (ZKIR) is a domain-specific, straight-line intermediate representation designed to specify computational circuits for the Midnight zero-knowledge proving system (CLM-0400; SRC-0038 docs/zkir-v3-spec.md:L25-35; source fact; unperformed; high; S5). ZKIR represents relation instances executed over the scalar field of the BLS12-381 pairing-friendly elliptic curve, denoted $\mathbb{F}_r$ or `Native` (CLM-0400; SRC-0038 docs/zkir-v3-spec.md:L42-50; source fact; unperformed; high; S5). The language does not provide native control flow constructs such as loops, function calls, jumps, or dynamic branching (CLM-0400; SRC-0038 docs/zkir-v3-spec.md:L78-85; source fact; unperformed; high; S5). Instead, all programs execute as a static, linear sequence of instructions within a single flat scope (CLM-0400; SRC-0006 zkir/src/ir.rs:L35-50; repository observation; inspection; high; S5).

The authoritative textual specification of ZKIR version 3 is documented in `docs/zkir-v3-spec.md` under repository `input-output-hk/arc-zkir` at commit `fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9` (SRC-0038) (CLM-0401; SRC-0038 docs/zkir-v3-spec.md:L1-15; source fact; unperformed; high; S5). The specification defines an instruction set comprising exactly thirty-four instructions (CLM-0401; SRC-0038 docs/zkir-v3-spec.md:L310-375; source fact; unperformed; high; S5). The historical extract from `midnight-ledger` commit `92e8bdd3` (`ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src/ir.rs`) precisely mirrors this thirty-four instruction surface (CLM-0401; SRC-0006 midnight-ledger 92e8bdd3:zkir-v3/src/ir.rs:L360-920; repository observation; inspection; high; S5).

In contrast, the standalone `midnightntwrk/midnight-zkir` repository at branch `zkir-v3` (commit `2ffe2d17bbb736aec36fb300aeaca679a10d2278`, dated 2026-09-02) represents a subsequent evolutionary stage containing forty-one instruction variants (CLM-0402; SRC-0006 zkir/src/ir.rs:L385-910; repository observation; inspection; high; S5). This evolution introduces first-class byte slicing, indexing, concatenation, n-ary boolean gates, constant loading, and SHA-512 hashing, while deprecating legacy bit-decomposition instructions (CLM-0402; SRC-0006 zkir/src/ir.rs:L480-550; repository observation; inspection; high; S5). In accordance with repository research rules, the specification in SRC-0038 is treated as authoritative, and discrepancies with the Rust crate implementation are recorded explicitly as divergences (CLM-0402; SRC-0038 docs/zkir-v3-divergence-review.md:L1-40; source fact; unperformed; high; S5).

Every ZKIR circuit operates under a dual-execution semantics:
1. An off-circuit witness generation phase (known as `preprocess` or witness simulation), which executes prior to proving to populate variable assignments, verify assertions, and compute public input vectors (CLM-0403; SRC-0038 docs/zkir-v3-spec.md:L420-460; source fact; unperformed; high; S5).
2. An in-circuit constraint synthesis phase (known as `synthesize`), which compiles the instruction sequence into a PLONKish arithmetization over Halo2 columns, custom gates, and lookup tables (CLM-0403; SRC-0038 docs/zkir-v3-spec.md:L700-750; source fact; unperformed; high; S5).

Variables in ZKIR inhabit a named register memory mapping identifiers to typed values:

$$\text{memory}: \text{Identifier} \to \text{IrValue}$$

Every variable assignment is immutable; each register identifier must be defined exactly once before being read as an operand in subsequent instructions (CLM-0404; SRC-0038 docs/zkir-v3-spec.md:L120-135; source fact; unperformed; high; S5). Branching is emulated without control flow by evaluating both execution paths and multiplexing the results using conditional selection instructions (`cond_select`) or guarded transcript operations (CLM-0404; SRC-0038 docs/zkir-v3-spec.md:L140-155; source fact; unperformed; high; S5).

```
+----------------------------------------------------------------------------------------------------+
|                                    ZKIR Execution Pipeline                                         |
+----------------------------------------------------------------------------------------------------+
|                                                                                                    |
|    Proof Preimage (Private Inputs, Transcripts)                                                    |
|                         |                                                                          |
|                         v                                                                          |
|         +-------------------------------+                                                          |
|         |  Preprocess (Witness Sim)     | ---> Verifies Assertions & Preconditions                 |
|         |  Off-Circuit Reduction        | ---> Computes Public Input Vector pi                     |
|         +-------------------------------+ ---> Determines pi_skips for Guarded Operations         |
|                         |                                                                          |
|                         v                                                                          |
|         +-------------------------------+                                                          |
|         |  In-Circuit Synthesis         | ---> Halo2 PLONKish Arithmetization                      |
|         |  Constraint Layout            | ---> Custom Gates, Permutations, Lookup Arguments        |
|         +-------------------------------+                                                          |
|                         |                                                                          |
|                         v                                                                          |
|              Halo2 SNARK Prover (KZG)                                                              |
|                                                                                                    |
+----------------------------------------------------------------------------------------------------+
```

## Operand Syntax and Immediate Representation

Instructions in ZKIR receive inputs via operands. An operand in ZKIR version 3 is represented by the algebraic type `Operand`, which accepts either a variable reference or an immediate field element constant (CLM-0405; SRC-0006 zkir/src/ir.rs:L180-245; repository observation; inspection; high; S5):

```rust
pub enum Operand {
    Variable(Identifier),
    Immediate(Fr),
}
```

### Identifier Grammar

Variable references denote named memory cells populated by circuit inputs or instruction outputs. In ZKIR version 3, variable names must begin with the percent symbol (`%`), followed by an alphanumeric string identifier (CLM-0405; SRC-0006 zkir/src/ir.rs:L236-243; repository observation; inspection; high; S5). Variable identifiers without a leading `%` are rejected during deserialization with a parse error:

```text
"Invalid operand format: '<name>'. Variables must start with '%', immediates must start with '0x'"
```

In the concrete grammar, register identifiers conform to the regular expression:

$$\%[a-zA-Z_][a-zA-Z0-9_\.]*$$

Examples of valid identifiers include `%v_0`, `%t.1`, `%apk.6`, `%sk.3`, and `%modulus`.

### Immediate Hexadecimal Grammar

Immediate operands encode constant scalar field elements in $\mathbb{F}_r$. In the JSON wire format, immediate values are serialized as hexadecimal string literals prefixed by `0x` or `-0x` (CLM-0405; SRC-0006 zkir/src/ir.rs:L192-234; repository observation; inspection; high; S5). Immediates are encoded in little-endian byte order (CLM-0405; SRC-0006 zkir/src/ir.rs:L193-198; repository observation; inspection; high; S5). Trailing zero bytes in the 32-byte representation are stripped to produce the shortest canonical hexadecimal string, with the requirement that at least one hexadecimal digit follows the `0x` prefix (CLM-0405; SRC-0006 zkir/src/ir.rs:L194-197; repository observation; inspection; high; S5).

A leading negative sign indicates field negation: `-0x01` decodes to $-1 \pmod r = r - 1$, where $r$ is the BLS12-381 scalar field modulus:

$$r = 52435875175126190479447740508185965837690552500527637822603658699938581184513$$

Immediate operands can only represent scalar field elements of type `Native` (CLM-0405; SRC-0006 zkir/src/ir_vm.rs:L263-264; repository observation; inspection; high; S5). Non-native constant values (such as points or foreign field elements) cannot be expressed directly as immediate operands; in ZKIR version 3, they are loaded via `load_constant` or constructed dynamically from native field coordinates (CLM-0405; SRC-0006 zkir/src/ir_instructions/assign_constant.rs:L30-80; repository observation; inspection; high; S5).

Examples of immediate values:
- `"0x00"`: Represents the additive identity $0 \in \mathbb{F}_r$.
- `"0x01"`: Represents the multiplicative identity $1 \in \mathbb{F}_r$.
- `"0x2a"`: Represents integer 42 ($0\text{x}2a$).
- `"-0x02"`: Represents $r - 2 \pmod r$.
- `"0x0001"`: Invalid serialization if not stripped; the canonical form is `"0x00"` or `"0x01"`.

```
+----------------------------------------------------------------------------------------------------+
|                                    ZKIR Operand Format Matrix                                      |
+----------------------------------------------------------------------------------------------------+
| Operand Variant     | Prefix   | Encoding Scheme         | Target Sort                             |
+---------------------+----------+-------------------------+-----------------------------------------+
| Variable Identifier | %        | Alphanumeric string     | Arbitrary supported IrType              |
| Immediate Constant  | 0x       | Hex Little-Endian Bytes | Native only (Scalar<BLS12-381>)         |
| Negated Immediate   | -0x      | Negated Hex LE Bytes    | Native only (r - x mod r)               |
+----------------------------------------------------------------------------------------------------+
```

## Complete Catalog of Authoritative Instructions (34 Instructions)

The authoritative specification defines thirty-four instructions grouped into eleven functional categories (CLM-0406; SRC-0038 docs/zkir-v3-spec.md:L310-375; source fact; unperformed; high; S5). Below is the comprehensive technical definition of each instruction, including JSON wire format schema, typing constraints, off-circuit reduction rules, and in-circuit synthesis semantics.

```
+----------------------------------------------------------------------------------------------------+
|                                    ZKIR v3 Instruction Families                                    |
+----------------------------------------------------------------------------------------------------+
| Arithmetic & Fields   | add, mul, neg, inv, jubjub_scalar_from_native                              |
| Comparison & Tests    | test_eq, less_than                                                         |
| Assertions & Checks   | assert, constrain_eq, constrain_to_boolean, constrain_bits                 |
| Memory & Selection    | copy, cond_select                                                          |
| Bit Decomposition     | div_mod_power_of_two, reconstitute_field                                   |
| Boolean Logic         | not                                                                        |
| Elliptic Curves       | ec_mul, ec_mul_generator, hash_to_curve, into_coordinates, from_coordinates |
| Byte Operations       | into_bytes32, from_bytes32, reverse_bytes, bytes32_into_low_high,          |
|                       | bytes32_from_low_high                                                      |
| Hash Functions        | transient_hash, persistent_hash, keccak256                                 |
| Encoding              | encode                                                                     |
| System & Transcripts  | public_input, private_input, impact, output                                |
+----------------------------------------------------------------------------------------------------+
```

### 1. Arithmetic Instructions

#### `add`
- **JSON Syntax:**
  ```json
  { "op": "add", "a": "%v_0", "b": "0x05", "output": "%v_1" }
  ```
- **Signature:** $(T, T) \to T$, where $T \in \{\text{Native}, \text{JubjubPoint}, \text{Secp256k1Point}, \text{Secp256k1Base}, \text{Secp256k1Scalar}, \text{Secp256r1Point}, \text{Secp256r1Base}, \text{Secp256r1Scalar}, \text{Curve25519Point}, \text{Curve25519Base}, \text{Curve25519Scalar}\}$ (CLM-0407; SRC-0006 zkir/src/ir_instructions/add.rs:L30-58; repository observation; inspection; high; S5). In repository commit `2ffe2d1`, `add` additionally supports `Byte` addition modulo 256.
- **Preprocess Semantics:** Evaluates $a + b$ in the algebraic structure of type $T$. When operating on elliptic curve points, performs group addition. When operating on field elements, performs addition modulo the respective field modulus. Errors if operand types mismatch or type $T$ is unsupported.
- **Circuit Synthesis:** Allocates constraint gates in the corresponding arithmetic chip: `std.add` for `Native`, complete curve addition for points, or non-native field addition chips for foreign base/scalar types (CLM-0407; SRC-0006 zkir/src/ir_instructions/add.rs:L75-125; repository observation; inspection; high; S5).

#### `mul`
- **JSON Syntax:**
  ```json
  { "op": "mul", "a": "%v_0", "b": "%v_1", "output": "%v_2" }
  ```
- **Signature:** $(T, T) \to T$, where $T \in \{\text{Native}, \text{Secp256k1Base}, \text{Secp256k1Scalar}, \text{Secp256r1Base}, \text{Secp256r1Scalar}, \text{Curve25519Base}, \text{Curve25519Scalar}\}$ (CLM-0408; SRC-0006 zkir/src/ir_instructions/mul.rs:L25-58; repository observation; inspection; high; S5). Point multiplication by a scalar is handled by `ec_mul`, not `mul`. Point-by-point multiplication is unsupported.
- **Preprocess Semantics:** Evaluates $a \cdot b$ in the underlying prime field.
- **Circuit Synthesis:** Allocates field multiplication constraints in the target field chip (`std.mul` for `Native`, using custom gates with degree 2) (CLM-0408; SRC-0006 zkir/src/ir_instructions/mul.rs:L73-118; repository observation; inspection; high; S5).

#### `neg`
- **JSON Syntax:**
  ```json
  { "op": "neg", "a": "%v_0", "output": "%v_1" }
  ```
- **Signature:** $T \to T$, supported on all eleven curve point, base field, scalar field, and native types, plus `Bool` in `2ffe2d1` (CLM-0409; SRC-0006 zkir/src/ir_instructions/neg.rs:L28-70; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Evaluates additive inverse $-a$. For elliptic curve points, computes point negation $(x, -y)$. For `Bool`, evaluates logical negation $\neg a$.
- **Circuit Synthesis:** Allocates additive negation constraints in the corresponding chip (CLM-0409; SRC-0006 zkir/src/ir_instructions/neg.rs:L90-154; repository observation; inspection; high; S5).

#### `inv`
- **JSON Syntax:**
  ```json
  { "op": "inv", "a": "%v_0", "output": "%v_1" }
  ```
- **Signature:** $T \to T$, where $T$ is any field type (`Native`, or foreign base/scalar types) (CLM-0410; SRC-0006 zkir/src/ir_instructions/inv.rs:L25-39; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Computes multiplicative inverse $a^{-1}$ using Fermat's Little Theorem or the extended Euclidean algorithm. If $a = 0$, execution aborts with error: `"cannot invert zero of type <T>"` (CLM-0410; SRC-0006 zkir/src/ir_instructions/inv.rs:L41-76; repository observation; inspection; high; S5).
- **Circuit Synthesis:** Allocates an unconstrained witness cell for the inverse and constrains $a \cdot a^{-1} = 1$ in the active field chip. If $a = 0$, the constraint system is unsatisfiable (CLM-0410; SRC-0006 zkir/src/ir_instructions/inv.rs:L93-136; repository observation; inspection; high; S5).

#### `jubjub_scalar_from_native`
- **JSON Syntax:**
  ```json
  { "op": "jubjub_scalar_from_native", "native": "%v_0", "output": "%s_0" }
  ```
- **Signature:** $\text{Native} \to \text{JubjubScalar}$ (CLM-0411; SRC-0038 docs/zkir-v3-spec.md:L345; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Converts a BLS12-381 scalar $x \in \mathbb{F}_r$ into a Jubjub scalar $s \in \mathbb{F}_s$ by interpreting $x$ as an integer modulo the Jubjub scalar field modulus $s$:
  $$s = 655448439689077380993096756352324572970592126587231728136515949875690759883$$
  Notice that because $r > s$, this modular reduction can map distinct native elements to identical Jubjub scalars (CLM-0411; SRC-0006 zkir/src/ir_instructions/encode.rs:L18-27; repository observation; inspection; high; S5).
- **Circuit Synthesis:** Decomposes the native field element into little-endian bytes, converts them into a BigUint representation, and initializes an assigned Jubjub scalar cell (CLM-0411; SRC-0006 zkir/src/ir_vm.rs:L1135-1142; repository observation; inspection; high; S5).

### 2. Comparison and Predicate Instructions

#### `test_eq`
- **JSON Syntax:**
  ```json
  { "op": "test_eq", "a": "%v_0", "b": "%v_1", "output": "%b_0" }
  ```
- **Signature:** $(T, T) \to \text{Native}$ (or $\text{Bool}$ in extended types) (CLM-0412; SRC-0006 zkir/src/ir_instructions/eq.rs:L25-45; repository observation; inspection; high; S5). Supported on `Native`, `Bool`, `Byte`, and `JubjubPoint`.
- **Preprocess Semantics:** Evaluates structural equality: returns $1$ if $a = b$, and $0$ otherwise.
- **Circuit Synthesis:** Employs an arithmetic equality test gadget. For `Native`, allocates an auxiliary witness $w$ satisfying:
  $$(a - b) \cdot (1 - \text{out}) = 0$$
  $$(a - b) \cdot w = \text{out}$$
  If $a = b$, then $\text{out} = 1$ is forced. If $a \ne b$, then $w = (a - b)^{-1}$ and $\text{out} = 0$ is forced (CLM-0412; SRC-0006 zkir/src/ir_instructions/eq.rs:L85-125; repository observation; inspection; high; S5).

#### `less_than`
- **JSON Syntax:**
  ```json
  { "op": "less_than", "a": "%v_0", "b": "%v_1", "bits": 32, "output": "%b_0" }
  ```
- **Signature:** $(\text{Native}, \text{Native}) \to \text{Native}$ (CLM-0413; SRC-0038 docs/zkir-v3-spec.md:L350-355; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Truncates operands $a$ and $b$ to `bits` bits and evaluates $a < b$ over the integers, returning $1$ if true and $0$ if false. Fails during preprocess if either operand exceeds $2^{\text{bits}} - 1$.
- **Circuit Synthesis:** Invokes `std.lower_than(layouter, a, b, bound)` with a bit width adjusted to $\max(\text{bits} + \text{bits} \pmod 2, 4)$ to satisfy standard library parity constraints (CLM-0413; SRC-0006 zkir/src/ir_vm.rs:L1124-1134; repository observation; inspection; high; S5).

### 3. Assertion and Constraint Instructions

#### `assert`
- **JSON Syntax:**
  ```json
  { "op": "assert", "cond": "%b_0" }
  ```
- **Signature:** $(\text{Native}) \to ()$ (CLM-0414; SRC-0038 docs/zkir-v3-spec.md:L320; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Evaluates operand `cond` as a boolean (requiring $0$ or $1$). Aborts preprocess execution with `"Failed direct assertion"` if $\text{cond} \ne 1$ (CLM-0414; SRC-0006 zkir/src/ir_vm.rs:L393-397; repository observation; inspection; high; S5).
- **Circuit Synthesis:** Constrains `cond` to be non-zero using `std.assert_non_zero(layouter, cond)`. Notice that in-circuit, this asserts $\text{cond} \ne 0$, which diverges from boolean checking if `cond` was not previously constrained to boolean (CLM-0414; SRC-0038 docs/zkir-v3-divergence-review.md:L350-380; source fact; unperformed; high; S5).

#### `constrain_eq`
- **JSON Syntax:**
  ```json
  { "op": "constrain_eq", "a": "%v_0", "b": "%v_1" }
  ```
- **Signature:** $(T, T) \to ()$, supported on all types (CLM-0415; SRC-0006 zkir/src/ir_instructions/constrain_eq.rs:L20-45; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Asserts $a = b$; raises a fatal error during preprocess if the values differ (CLM-0415; SRC-0006 zkir/src/ir_instructions/constrain_eq.rs:L48-80; repository observation; inspection; high; S5).
- **Circuit Synthesis:** Enforces equality constraints between the assigned cells of $a$ and $b$ across all constituent field elements via Halo2 copy constraints (CLM-0415; SRC-0006 zkir/src/ir_instructions/constrain_eq.rs:L85-140; repository observation; inspection; high; S5).

#### `constrain_to_boolean`
- **JSON Syntax:**
  ```json
  { "op": "constrain_to_boolean", "val": "%v_0" }
  ```
- **Signature:** $(\text{Native}) \to ()$ (CLM-0416; SRC-0038 docs/zkir-v3-spec.md:L325; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Asserts that `val` is either $0$ or $1$. Errors if $\text{val} \notin \{0, 1\}$.
- **Circuit Synthesis:** Enforces the quadratic boolean constraint $x \cdot (1 - x) = 0$ via `std.convert(layouter, x)` to `AssignedBit` (CLM-0416; SRC-0006 zkir/src/ir_vm.rs:L1003-1008; repository observation; inspection; high; S5).

#### `constrain_bits`
- **JSON Syntax:**
  ```json
  { "op": "constrain_bits", "val": "%v_0", "bits": 64 }
  ```
- **Signature:** $(\text{Native}) \to ()$ (CLM-0417; SRC-0038 docs/zkir-v3-spec.md:L328; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Verifies that the canonical integer representation of `val` is strictly less than $2^{\text{bits}}$. Fails if `bits >= FR_BITS` (255) or higher bits are non-zero.
- **Circuit Synthesis:** Decomposes `val` into `bits` assigned bits using range-check lookup tables (CLM-0417; SRC-0006 zkir/src/ir_vm.rs:L988-997; repository observation; inspection; high; S5).

### 4. Memory and Multiplexing Instructions

#### `copy`
- **JSON Syntax:**
  ```json
  { "op": "copy", "val": "%v_0", "output": "%v_1" }
  ```
- **Signature:** $T \to T$ (CLM-0418; SRC-0038 docs/zkir-v3-spec.md:L330; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Binds register `%id` to a cloned copy of `val`.
- **Circuit Synthesis:** Copies cell assignment without introducing new arithmetic constraints (CLM-0418; SRC-0006 zkir/src/ir_vm.rs:L1009-1012; repository observation; inspection; high; S5).

#### `cond_select`
- **JSON Syntax:**
  ```json
  { "op": "cond_select", "bit": "%b_0", "a": "%v_0", "b": "%v_1", "output": "%v_2" }
  ```
- **Signature:** $(\text{Native}, T, T) \to T$ (CLM-0419; SRC-0006 zkir/src/ir_instructions/select.rs:L25-54; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Verifies `bit` is boolean ($0$ or $1$). If `bit == 1`, returns $a$; if `bit == 0`, returns $b$. Mismatched operand types are rejected.
- **Circuit Synthesis:** Multiplexes each limb of $a$ and $b$ using the constraint:
  $$\text{out} = b + \text{bit} \cdot (a - b)$$
  This degree-2 constraint ensures $\text{out} = a$ when $\text{bit} = 1$, and $\text{out} = b$ when $\text{bit} = 0$ (CLM-0419; SRC-0006 zkir/src/ir_instructions/select.rs:L77-127; repository observation; inspection; high; S5).

### 5. Field Decomposition and Bit Manipulation Instructions

#### `div_mod_power_of_two`
- **JSON Syntax:**
  ```json
  { "op": "div_mod_power_of_two", "val": "%v_0", "bits": 8, "outputs": ["%div", "%mod"] }
  ```
- **Signature:** $(\text{Native}) \to (\text{Native}, \text{Native})$ (CLM-0420; SRC-0038 docs/zkir-v3-spec.md:L332; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Splits integer $x = q \cdot 2^{\text{bits}} + r$. Outputs `%div` as quotient $q$ and `%mod` as remainder $r$ with $r < 2^{\text{bits}}$. Fails if `bits > 248`.
- **Circuit Synthesis:** Decomposes `val` into bits, reconstructing quotient and remainder via `assigned_from_le_bits` (CLM-0420; SRC-0006 zkir/src/ir_vm.rs:L1170-1189; repository observation; inspection; high; S5). Deprecated in `2ffe2d1`.

#### `reconstitute_field`
- **JSON Syntax:**
  ```json
  { "op": "reconstitute_field", "divisor": "%div", "modulus": "%mod", "bits": 8, "output": "%v_0" }
  ```
- **Signature:** $(\text{Native}, \text{Native}) \to \text{Native}$ (CLM-0421; SRC-0038 docs/zkir-v3-spec.md:L335; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Computes $x = \text{divisor} \cdot 2^{\text{bits}} + \text{modulus}$. Asserts that the reconstructed integer does not overflow the field modulus $r$.
- **Circuit Synthesis:** Asserts $\text{modulus} < 2^{\text{bits}}$ and $\text{divisor} < 2^{255 - \text{bits}}$, then synthesizes the linear combination $2^{\text{bits}} \cdot \text{divisor} + \text{modulus}$ (CLM-0421; SRC-0006 zkir/src/ir_vm.rs:L1190-1222; repository observation; inspection; high; S5). Deprecated in `2ffe2d1`.

### 6. Boolean Logic Instructions

#### `not`
- **JSON Syntax:**
  ```json
  { "op": "not", "a": "%b_0", "output": "%b_1" }
  ```
- **Signature:** $\text{Native} \to \text{Native}$ (or $\text{Bool} \to \text{Bool}$) (CLM-0422; SRC-0038 docs/zkir-v3-spec.md:L340; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Verifies operand $a \in \{0, 1\}$ and returns $1 - a$.
- **Circuit Synthesis:** Converts operand to `AssignedBit` and computes negation via `std.not(layouter, bit)` (CLM-0422; SRC-0006 zkir/src/ir_vm.rs:L1101-1108; repository observation; inspection; high; S5).

### 7. Elliptic Curve Cryptography Instructions

#### `ec_mul`
- **JSON Syntax:**
  ```json
  { "op": "ec_mul", "a": "%p_0", "scalar": "%s_0", "output": "%p_1" }
  ```
- **Signature:** $(P, S) \to P$, where $(P, S) \in \{(\text{JubjubPoint}, \text{JubjubScalar}), (\text{Secp256k1Point}, \text{Secp256k1Scalar}), (\text{Secp256r1Point}, \text{Secp256r1Scalar}), (\text{Curve25519Point}, \text{Curve25519Scalar})\}$ (CLM-0423; SRC-0006 zkir/src/ir_instructions/ec_mul.rs:L25-60; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Performs elliptic curve scalar multiplication $[s]P$.
- **Circuit Synthesis:** Synthesizes variable-base scalar multiplication in the selected curve ECC chip (CLM-0423; SRC-0006 zkir/src/ir_instructions/ec_mul.rs:L80-140; repository observation; inspection; high; S5).

#### `ec_mul_generator`
- **JSON Syntax:**
  ```json
  { "op": "ec_mul_generator", "scalar": "%s_0", "output": "%p_0" }
  ```
- **Signature:** $S \to P$ for `JubjubScalar` and `Secp256k1Scalar` (CLM-0423; SRC-0006 zkir/src/ir_vm.rs:L621-630; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Multiplies the canonical group generator $G$ by scalar $s$: $[s]G$.
- **Circuit Synthesis:** Invokes fixed-base scalar multiplication algorithms optimized for known generator bases (CLM-0423; SRC-0006 zkir/src/ir_vm.rs:L1229-1248; repository observation; inspection; high; S5).

#### `hash_to_curve`
- **JSON Syntax:**
  ```json
  { "op": "hash_to_curve", "inputs": ["%v_0", "%v_1"], "output": "%p_0" }
  ```
- **Signature:** $(\text{Native}, \dots) \to \text{JubjubPoint}$ (CLM-0423; SRC-0038 docs/zkir-v3-spec.md:L360; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Hashes a vector of native field elements to a point on the Jubjub curve using the Poseidon hash and the Shallue-van de Woestijne encoding (CLM-0423; SRC-0006 zkir/src/ir_vm.rs:L606-614; repository observation; inspection; high; S5).
- **Circuit Synthesis:** Allocates Poseidon hashing constraints feeding directly into the in-circuit SWU curve mapper (CLM-0423; SRC-0006 zkir/src/ir_vm.rs:L1249-1262; repository observation; inspection; high; S5).

#### `into_coordinates`
- **JSON Syntax:**
  ```json
  { "op": "into_coordinates", "point": "%p_0", "outputs": ["%x", "%y"] }
  ```
- **Signature:** $P \to (B, B)$, extracting affine coordinates $(x, y)$ (CLM-0423; SRC-0006 zkir/src/ir_instructions/into_coordinates.rs:L30-79; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Decomposes an affine point into its coordinate pair $(x, y)$. For Weierstrass curves (`Secp256k1Point`, `Secp256r1Point`), fails if the point is the point at infinity (identity).
- **Circuit Synthesis:** Constrains Weierstrass points to be non-zero via `curve.assert_non_zero(layouter, p)` and extracts assigned base field cells (CLM-0423; SRC-0006 zkir/src/ir_instructions/into_coordinates.rs:L95-139; repository observation; inspection; high; S5).

#### `from_coordinates`
- **JSON Syntax:**
  ```json
  { "op": "from_coordinates", "inputs": ["%x", "%y"], "output": "%p_0" }
  ```
- **Signature:** $(B, B) \to P$, constructing point from coordinates (CLM-0423; SRC-0006 zkir/src/ir_instructions/from_coordinates.rs:L25-77; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Verifies that $(x, y)$ satisfies the curve equation and lies in the prime-order subgroup; fails if invalid.
- **Circuit Synthesis:** Enforces the curve equation constraints and subgroup membership checks in-circuit (CLM-0423; SRC-0006 zkir/src/ir_instructions/from_coordinates.rs:L92-126; repository observation; inspection; high; S5).

### 8. Byte and Serialization Instructions

#### `into_bytes32`
- **JSON Syntax:**
  ```json
  { "op": "into_bytes32", "input": "%v_0", "output": "%bytes_0" }
  ```
- **Signature:** $T \to \text{Bytes32}$, supported on `Native` and all foreign base/scalar types (CLM-0424; SRC-0006 zkir/src/ir_instructions/into_bytes32.rs:L24-62; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Converts a field element into its 32-byte little-endian canonical representation.
- **Circuit Synthesis:** Decomposes the assigned field element into thirty-two assigned byte cells using `assigned_to_le_bytes(layouter, val, Some(32))` (CLM-0424; SRC-0006 zkir/src/ir_instructions/into_bytes32.rs:L80-132; repository observation; inspection; high; S5).

#### `from_bytes32`
- **JSON Syntax:**
  ```json
  { "op": "from_bytes32", "type": "Scalar<BLS12-381>", "bytes": "%bytes_0", "output": "%v_0" }
  ```
- **Signature:** $(\text{Bytes32}) \to T$ (CLM-0424; SRC-0006 zkir/src/ir_instructions/from_bytes32.rs:L25-80; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Decodes thirty-two little-endian bytes into a field element of type $T$. Fails if the integer exceeds the modulus of field $T$.
- **Circuit Synthesis:** Reconstitutes the field element from thirty-two assigned bytes and enforces canonicity range bounds (CLM-0424; SRC-0006 zkir/src/ir_instructions/from_bytes32.rs:L95-150; repository observation; inspection; high; S5).

#### `reverse_bytes`
- **JSON Syntax:**
  ```json
  { "op": "reverse_bytes", "bytes": "%bytes_0", "output": "%bytes_1" }
  ```
- **Signature:** $\text{Bytes32} \to \text{Bytes32}$ (CLM-0424; SRC-0038 docs/zkir-v3-spec.md:L365; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Reverses the order of thirty-two bytes (converting between little-endian and big-endian representations).
- **Circuit Synthesis:** Permutes assigned byte cell references in reverse order without adding gate constraints (CLM-0424; SRC-0006 zkir/src/ir_vm.rs:L1290-1295; repository observation; inspection; high; S5).

#### `bytes32_into_low_high`
- **JSON Syntax:**
  ```json
  { "op": "bytes32_into_low_high", "bytes": "%bytes_0", "outputs": ["%low", "%high"] }
  ```
- **Signature:** $\text{Bytes32} \to (\text{Native}, \text{Native})$ (CLM-0424; SRC-0038 docs/zkir-v3-spec.md:L370; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Splits a 32-byte array into `%low` (the first 31 bytes, fitting in $\mathbb{F}_r$ since $31 \times 8 = 248 < 254$) and `%high` (the 32nd byte as a single native field element $< 256$) (CLM-0424; SRC-0006 zkir/src/ir_vm.rs:L683-691; repository observation; inspection; high; S5).
- **Circuit Synthesis:** Assigns byte 31 to zero for `%low` and converts byte 31 to native for `%high` (CLM-0424; SRC-0006 zkir/src/ir_vm.rs:L1323-1331; repository observation; inspection; high; S5).

#### `bytes32_from_low_high`
- **JSON Syntax:**
  ```json
  { "op": "bytes32_from_low_high", "inputs": ["%low", "%high"], "output": "%bytes_0" }
  ```
- **Signature:** $(\text{Native}, \text{Native}) \to \text{Bytes32}$ (CLM-0424; SRC-0038 docs/zkir-v3-spec.md:L372; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Verifies that `%low` fits in 31 bytes (byte 31 is zero) and `%high` fits in 1 byte ($< 256$), assembling the 32-byte array (CLM-0424; SRC-0006 zkir/src/ir_vm.rs:L692-705; repository observation; inspection; high; S5).
- **Circuit Synthesis:** Constrains byte 31 of `%low` to equal zero, converts `%high` to byte, and concatenates them (CLM-0424; SRC-0006 zkir/src/ir_vm.rs:L1332-1342; repository observation; inspection; high; S5).

### 9. Cryptographic Hash Instructions

#### `transient_hash`
- **JSON Syntax:**
  ```json
  { "op": "transient_hash", "inputs": ["%v_0", "%v_1"], "output": "%v_2" }
  ```
- **Signature:** $(\text{Native}, \dots) \to \text{Native}$ (CLM-0400; SRC-0038 docs/zkir-v3-spec.md:L362; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Evaluates the unpadded Poseidon permutation over native field elements (CLM-0400; SRC-0006 zkir/src/ir_vm.rs:L524-533; repository observation; inspection; high; S5).
- **Circuit Synthesis:** Allocates Poseidon hash chip gates in-circuit (CLM-0400; SRC-0006 zkir/src/ir_vm.rs:L1027-1036; repository observation; inspection; high; S5).

#### `persistent_hash`
- **JSON Syntax:**
  ```json
  {
    "op": "persistent_hash",
    "alignment": [
      { "tag": "atom", "value": { "length": 32, "tag": "bytes" } }
    ],
    "inputs": ["%v_0"],
    "output": "%bytes_0"
  }
  ```
- **Signature:** $(\text{Native}, \dots) \to \text{Bytes32}$ (CLM-0400; SRC-0038 docs/zkir-v3-spec.md:L364; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Decodes field elements into byte buffers using the declared alignment descriptor and evaluates standard SHA-256 (CLM-0400; SRC-0006 zkir/src/ir_vm.rs:L534-568; repository observation; inspection; high; S5).
- **Circuit Synthesis:** Invokes `fab_decode_to_bytes` in-circuit and routes bytes through the `std.sha2_256` chip (CLM-0400; SRC-0006 zkir/src/ir_vm.rs:L1037-1070; repository observation; inspection; high; S5).

#### `keccak256`
- **JSON Syntax:**
  ```json
  {
    "op": "keccak256",
    "alignment": [
      { "tag": "atom", "value": { "length": 32, "tag": "bytes" } }
    ],
    "inputs": ["%v_0"],
    "output": "%bytes_0"
  }
  ```
- **Signature:** $(\text{Native}, \dots) \to \text{Bytes32}$ (CLM-0400; SRC-0038 docs/zkir-v3-spec.md:L364; source fact; unperformed; high; S5).
- **Preprocess Semantics:** Decodes input field elements according to alignment and hashes via Keccak-256.
- **Circuit Synthesis:** Decodes bytes via `fab_decode_to_bytes` and evaluates constraints using `std.keccak_256` chip.

### 10. Serialization and Wire Format Instructions

#### `encode`
- **JSON Syntax:**
  ```json
  { "op": "encode", "input": "%p_0", "outputs": ["%x", "%y"] }
  ```
- **Signature:** $T \to (\text{Native}, \dots, \text{Native})$ (CLM-0401; SRC-0006 zkir/src/ir_instructions/encode.rs:L28-100; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Encodes value of type $T$ into its canonical sequence of native field elements. The number of output registers must exactly equal `T.encoded_len()`.
- **Circuit Synthesis:** Decomposes the in-circuit representation of $T$ into assigned native field elements using `encode_incircuit` (CLM-0401; SRC-0006 zkir/src/ir_instructions/encode.rs:L105-200; repository observation; inspection; high; S5).

### 11. System Interface and Transcript Instructions

#### `public_input`
- **JSON Syntax:**
  ```json
  { "op": "public_input", "type": "Scalar<BLS12-381>", "output": "%t.1", "guard": null }
  ```
- **Signature:** Reads a value from the public transcript output stream (CLM-0403; SRC-0006 zkir/src/ir_vm.rs:L404-422; repository observation; inspection; high; S5).
- **Preprocess Semantics:** If `guard` is present and resolves to $0$, initializes `%id` to the default value of type `val_t` without advancing transcript cursor $\iota^+_o$. If unguarded or `guard == 1`, consumes $w = \text{val\_t.encoded\_len()}$ elements from `preimage.public_transcript_outputs` and decodes them.
- **Circuit Synthesis:** Allocates unconstrained witness variables initialized from the preprocess witness, enforcing type validity constraints (CLM-0403; SRC-0006 zkir/src/ir_vm.rs:L1143-1169; repository observation; inspection; high; S5).

#### `private_input`
- **JSON Syntax:**
  ```json
  { "op": "private_input", "type": "Scalar<BLS12-381>", "output": "%sk.3", "guard": null }
  ```
- **Signature:** Reads a value from the private transcript stream (CLM-0403; SRC-0006 zkir/src/ir_vm.rs:L423-442; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Identical to `public_input`, but consumes elements from `preimage.private_transcript` (cursor $\iota^-$).
- **Circuit Synthesis:** Assigns private witness variables in the circuit layout.

#### `impact`
- **JSON Syntax:**
  ```json
  { "op": "impact", "guard": "0x01", "inputs": ["0x50", "0x01", "%t.1"] }
  ```
- **Signature:** $(\text{Native}, \text{Native}, \dots) \to ()$ (CLM-0404; SRC-0006 zkir/src/ir_vm.rs:L569-605; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Resolves boolean `guard`. If `guard == 1`, appends each input to the public input vector $\pi$, advances transcript cursor $\iota^+_i$, records `None` in skip vector $\kappa$, and asserts that the values match `preimage.public_transcript_inputs`. If `guard == 0`, appends $n$ zeros to $\pi$ (where $n = \text{inputs.len()}$), leaves transcript cursor $\iota^+_i$ unchanged, and records `Some(n)` in $\kappa$.
- **Circuit Synthesis:** For each input, synthesizes $\text{guarded\_x} = \text{std.select}(\text{layouter}, \text{guard}, x, 0)$ and pushes `guarded_x` to the circuit's public input column instances (CLM-0404; SRC-0006 zkir/src/ir_vm.rs:L1013-1026; repository observation; inspection; high; S5).

#### `output`
- **JSON Syntax:**
  ```json
  { "op": "output", "vals": ["%v_0", "%v_1"] }
  ```
- **Signature:** Circuit terminator asserting final typed outputs (CLM-0405; SRC-0006 zkir/src/ir_vm.rs:L749-768; repository observation; inspection; high; S5).
- **Preprocess Semantics:** Type-checks each operand against the circuit's declared `outputs` signature and collects return values for communications commitment validation.
- **Circuit Synthesis:** Type-checks assigned variables and binds them to the communications commitment input preimage (CLM-0405; SRC-0006 zkir/src/ir_vm.rs:L1400-1419; repository observation; inspection; high; S5).

## Extended Instructions in Repository Evolution (commit `2ffe2d1`)

Repository `2ffe2d1` expands the instruction set from 34 to 42 variants: `reverse_bytes` is renamed `reverse`, and `slice`, `nth`, `concat`, `load_constant`, `sha512`, `and`, `or` and `xor` are added, while `div_mod_power_of_two` and `reconstitute_field` remain present but deprecated (CLM-0402; SRC-0006 zkir/src/ir.rs:L385-910; repository observation; inspection; high; S5):

```
+----------------------------------------------------------------------------------------------------+
|                         Repository Extensions in 2ffe2d1 (1 renamed, 8 new ops)                     |
+----------------------------------------------------------------------------------------------------+
| Op Name         | JSON Wire Name    | Signature                          | Functional Purpose      |
+-----------------+-------------------+------------------------------------+-------------------------+
| reverse         | reverse           | Bytes(n) -> Bytes(n)               | General byte reversal   |
| slice           | slice             | (Bytes(n), u32, u32) -> Bytes(m)   | Sub-byte array slice    |
| nth             | nth               | (Bytes(n), u32) -> Byte            | Single byte indexing    |
| concat          | concat            | [Byte | Bytes(n), ...] -> Bytes(m) | Byte concatenation      |
| load_constant   | load_constant     | [Fr, ...] -> T                     | Constant value loader   |
| sha512          | sha512            | (Alignment, [Native, ...]) -> Bytes| SHA-512 cryptographic   |
| and, or, xor    | and, or, xor      | [Bool, ...] -> Bool                | N-ary boolean gates     |
+----------------------------------------------------------------------------------------------------+
```

### `reverse`
- **JSON Syntax:** `{"op": "reverse", "bytes": "%b_0", "output": "%b_1"}`
- **Signature:** $\text{Bytes}(n) \to \text{Bytes}(n)$ for any $n \ge 1$ (CLM-0402; SRC-0006 zkir/src/ir.rs:L680-690; repository observation; inspection; high; S5).
- **Semantics:** Reverses the vector of $n$ bytes. Replaces `reverse_bytes`.

### `slice`
- **JSON Syntax:** `{"op": "slice", "bytes": "%b_0", "start": 4, "len": 16, "output": "%b_1"}`
- **Signature:** $(\text{Bytes}(n), \text{u32}, \text{u32}) \to \text{Bytes}(\text{len})$ (CLM-0402; SRC-0006 zkir/src/ir_vm.rs:L664-682; repository observation; inspection; high; S5).
- **Semantics:** Requires $\text{len} \ge 1$ and $\text{start} + \text{len} \le n$. Extracts sub-array `bytes[start .. start + len]`.

### `nth`
- **JSON Syntax:** `{"op": "nth", "bytes": "%b_0", "index": 0, "output": "%byte_0"}`
- **Signature:** $(\text{Bytes}(n), \text{u32}) \to \text{Byte}$ (CLM-0402; SRC-0006 zkir/src/ir_vm.rs:L706-717; repository observation; inspection; high; S5).
- **Semantics:** Requires $\text{index} < n$. Extracts the single byte at position `index`.

### `concat`
- **JSON Syntax:** `{"op": "concat", "inputs": ["%b_0", "%byte_1"], "output": "%b_2"}`
- **Signature:** $(\text{Byte} \cup \text{Bytes}(n), \dots) \to \text{Bytes}(m)$ (CLM-0402; SRC-0006 zkir/src/ir_vm.rs:L718-740; repository observation; inspection; high; S5).
- **Semantics:** Flattens operands into a contiguous byte array. Requires total output length $1 \le m \le 2^{24}$.

### `load_constant`
- **JSON Syntax:**
  ```json
  { "op": "load_constant", "type": "Point<Jubjub>", "encoding": ["0x2a", "0x01"], "output": "%p_0" }
  ```
- **Signature:** $[Fr, \dots] \to T$ (CLM-0402; SRC-0006 zkir/src/ir_vm.rs:L741-748; repository observation; inspection; high; S5).
- **Semantics:** Decodes hex immediate field elements via `decode_offcircuit(encoding, val_t)` and assigns fixed cells in-circuit via `assign_constant_incircuit`.

### `sha512`
- **JSON Syntax:**
  ```json
  { "op": "sha512", "alignment": [...], "inputs": ["%v_0"], "output": "%b_0" }
  ```
- **Signature:** $(\text{Native}, \dots) \to \text{Bytes}(64)$ (CLM-0402; SRC-0006 zkir/src/ir_vm.rs:L544-568; repository observation; inspection; high; S5).
- **Semantics:** Evaluates standard SHA-512 over aligned bytes, outputting a 64-byte array.

### `and`, `or`, `xor`
- **JSON Syntax:**
  ```json
  { "op": "and", "inputs": ["%b_0", "%b_1", "%b_2"], "output": "%b_3" }
  ```
- **Signature:** $(\text{Bool}, \dots) \to \text{Bool}$ (CLM-0402; SRC-0006 zkir/src/ir_vm.rs:L368-382; repository observation; inspection; high; S5).
- **Semantics:** Requires at least one `Bool` input. Evaluates n-ary conjunction, disjunction, or exclusive disjunction.

## Type Compatibility Matrix Across Instructions

The table below summarizes type support across the primary instruction families in authoritative ZKIR v3 (CLM-0406; SRC-0038 docs/zkir-v3-spec.md:Appendix B; source fact; unperformed; high; S5):

| Type Name | `add` | `mul` | `neg` | `inv` | `test_eq` | `cond_select` | `into_coords` | `into_bytes32` | `encode` |
|:---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| `Native` | Yes | Yes | Yes | Yes | Yes | Yes | No | Yes | Yes (1) |
| `Bytes32` | No | No | No | No | No | No | No | No | Yes (2) |
| `JubjubPoint` | Yes | No | Yes | No | Yes | Yes | Yes | No | Yes (2) |
| `JubjubScalar` | No | No | No | No | No | No | No | No | Yes (1) |
| `Secp256k1Point` | Yes | No | Yes | No | No | Yes | Yes | No | Yes (5) |
| `Secp256k1Base` | Yes | Yes | Yes | Yes | No | Yes | No | Yes | Yes (2) |
| `Secp256k1Scalar` | Yes | Yes | Yes | Yes | No | Yes | No | Yes | Yes (2) |
| `Secp256r1Point` | Yes | No | Yes | No | No | Yes | Yes | No | Yes (5) |
| `Secp256r1Base` | Yes | Yes | Yes | Yes | No | Yes | No | Yes | Yes (2) |
| `Secp256r1Scalar` | Yes | Yes | Yes | Yes | No | Yes | No | Yes | Yes (2) |
| `Curve25519Point` | Yes | No | Yes | No | No | Yes | Yes | No | Yes (4) |
| `Curve25519Base` | Yes | Yes | Yes | Yes | No | Yes | No | Yes | Yes (2) |
| `Curve25519Scalar` | Yes | Yes | Yes | Yes | No | Yes | No | Yes | Yes (2) |

Notice the critical asymmetries identified in the divergence review (SRC-0038) (CLM-0406; SRC-0038 docs/zkir-v3-divergence-review.md:L450-520; source fact; unperformed; high; S5):
- `test_eq` is supported on `Native` and `JubjubPoint`, but rejected on foreign curve points and foreign base/scalar field elements.
- `into_bytes32` operates on field elements, but is rejected on curve points.
- `into_coordinates` operates on curve points, but is rejected on field elements.
- Weierstrass points encode to 5 native field elements due to the identity flag, whereas Curve25519 points encode to 4 field elements.

## Implications for Formal Semantics in K

The formal specification of the ZKIR instruction set in the K Framework requires several structural design decisions:
1. **Instruction Grammar Sorts:** Each instruction must be declared as a production of sort `Instruction`, belonging to the cell `<instructions>` in the K configuration.
2. **Immutable SSA Register Store:** The register store $\text{memory}$ is modeled as a K map `Map{Identifier, IrValue}`. The K rewrite rules must verify that an identifier does not exist in `<registers>` prior to assignment, enforcing static single assignment.
3. **Operand Resolution:** A helper production `resolveOperand(Operand)` must rewrite either to looking up the identifier in `<registers>` or wrapping a hex immediate into `Native(Int)`.
4. **Failure Propagation:** Instructions whose preconditions fail (such as `inv(0)`, division with bit bound overflow, or asserted conditions evaluating to $0$) rewrite to a distinguished error configuration sort `ExecutionError(String)`, enabling precise formal testing of negative conformance predicates.

For further architectural context and semantics execution rules, refer to [ZKIR Type System](zkir-type-system.md), [ZKIR Virtual Machine Semantics](zkir-vm-semantics.md), and [ZKIR Formal Specification Agda](zkir-formal-spec-agda.md).
