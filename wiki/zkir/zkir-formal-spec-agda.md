---
id: zkir.formal-spec.agda
type: formal
title: ZKIR formal specification and Agda mechanization (arc-zkir)
status: active
updated_at: 2026-09-03T14:39:26Z
sources:
  - SRC-0025
  - SRC-0006
---

# ZKIR formal specification and Agda mechanization (arc-zkir)

## Provenance, source repository, and pins

The formal specification and machine-checked mechanization of the Zero-Knowledge Intermediate Representation (ZKIR) are developed in the repository `input-output-hk/arc-zkir` (CLM-0600; SRC-0025 README.md; repository observation; not reproduced; high; S4).
The repository contains two distinct mechanizations in Agda, designated `zkir-v2` and `zkir-v3` (CLM-0600; SRC-0025 README.md; source fact; not reproduced; high; S4).
The repository git log records 149 commits on branch `main` ending at commit `fd1c24e1bc73e167dbfdacfd59f2cbeb4ad3a4f9` on 2026-08-26 (CLM-0601; SRC-0025 repos/_extracts/arc-zkir-git-log.txt; repository observation; not reproduced; high; S4).

The primary source of truth for ZKIR behavior is the Rust implementation in the repository `midnightntwrk/midnight-ledger` (CLM-0602; SRC-0025 README.md; source fact; not reproduced; high; S6).
The ZKIR v3 specification in `docs/zkir-v3-spec.md` and the `src/zkir-v3` mechanization pin `midnight-ledger` on branch `ledger-9` at commit `92e8bdd3a97b61b229e38916e1b180de6f448dd5` (short hash `92e8bdd3`), tracking crate `midnight-zkir-v3` version `3.0.0` (CLM-0602; SRC-0025 docs/zkir-v3-spec.md §Source pinning; source fact; not reproduced; high; S4).
The v3 pin at `92e8bdd3` is a bare branch-tip commit rather than a release tag, because no published crate release on crates.io or merged tag on `ledger-9` contained the full type surface at the time of authoring (CLM-0603; SRC-0025 README.md; repository observation; not reproduced; high; S4).
A repository discrepancy exists in the root documentation: `README.md` and `docs/zkir-v3-spec.md` cite commit `92e8bdd3`, whereas `CLAUDE.md` cites commit `04c9c5d9` (`3.0.0-rc.2`, branch `ledger-9`) (CLM-0603; SRC-0025 CLAUDE.md; contradiction; not reproduced; high; S4).
The git log reveals that commit `7d8cf18` on 2026-08-15 re-pinned the v3 specification from `04c9c5d9` to `92e8bdd3`, leaving `CLAUDE.md` with a stale citation (CLM-0603; SRC-0025 repos/_extracts/arc-zkir-git-log.txt; repository observation; not reproduced; high; S4).
The authors verify that the `zkir-v3/` crate tree is byte-identical between commit `04c9c5d9` and commit `92e8bdd3` (CLM-0603; SRC-0025 docs/zkir-v3-spec.md §Source pinning; source fact; not reproduced; high; S4).

For ZKIR v2, `docs/zkir-v2-spec.md` links to `midnight-ledger` branch `ledger-8` under directory `zkir/src` (`midnight-zkir` version `2.1.0`), while noting that `zkir` version `2.2.0` (minor version V2) is present on `ledger-9` at commit `92e8bdd3` (CLM-0604; SRC-0025 docs/zkir-v2-spec.md; source fact; not reproduced; high; S6).
A contradiction exists between repository baselines across the project: the Moriarty wiki pins `midnight-ledger` on branch `ledger-8` (commit `a8ab82ba2124c36f92795c683e70bd888bc1d1fb`), whereas the upstream ZKIR v3 specification and mechanization pin `ledger-9` (commit `92e8bdd3`) (CLM-0604; SRC-0006 wiki/midnight-repositories.md; contradiction; not reproduced; high; S4).
Whether an executable formal semantics should target the `ledger-8` V1 contract baseline or the `ledger-9` V0 v3 pipeline remains an open operational question for the semantics architecture (CLM-0604; SRC-0025 docs/zkir-v3-spec.md §1.3; open question; not reproduced; medium; S2).

The repository root includes project-specific agent guidance in `.claude/agents/agda-protocol-formalizer.md` (CLM-0605; SRC-0025 .claude/agents/agda-protocol-formalizer.md; repository observation; not reproduced; high; S4).
The agent file reveals the authors' formalization discipline: formalize distributed and cryptographic protocols without postulates; structure the entire trust base into a single module parameter record named `Assumptions`; maintain compilation under Agda's `--safe` option; postpone abstraction until a pattern repeats three or more times; and maintain persistent project memory in a gitignored directory `.claude/agent-memory/` (CLM-0605; SRC-0025 .claude/agents/agda-protocol-formalizer.md; repository observation; not reproduced; high; S4).

## Formal model of a ZKIR program

### Abstract syntax and data structures

The formal abstract syntax of ZKIR v3 is mechanized in [`Syntax.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Syntax.agda) and documented in `docs/zkir-v3-spec.md` §5 (CLM-0606; SRC-0025 src/zkir-v3/Syntax.agda; source fact; not reproduced; high; S4).
A ZKIR circuit is represented by the record `IrSource`:

```agda
record IrSource : Set where
  constructor mk-ir-source
  field
    version                       : IrMinorVersion
    inputs                        : List TypedIdentifier
    outputs                       : List IrType
    do-communications-commitment  : Bool
    instructions                  : List Instruction
```

In ZKIR v3, variables are named registers represented by the type `Identifier = String`, corresponding to concrete string names prefixed with `%` (CLM-0606; SRC-0025 src/zkir-v3/Syntax.agda; source fact; not reproduced; high; S4).
Operands are defined by the sum type `Operand`:

```agda
data Operand : Set where
  var : Identifier → Operand
  imm : Fr         → Operand
```

An immediate operand always denotes a value of type `Native`, which is an element of the scalar field `Fr` (CLM-0606; SRC-0025 docs/zkir-v3-spec.md §3.4; source fact; not reproduced; high; S4).
No immediate representation exists for non-native field types or curve points (CLM-0606; SRC-0025 docs/zkir-v3-spec.md §3.4; source fact; not reproduced; high; S4).
A declared input is a pair `TypedIdentifier` combining a name and an `IrType` (CLM-0606; SRC-0025 src/zkir-v3/Syntax.agda; source fact; not reproduced; high; S4).
The minor version `IrMinorVersion` contains only the constructor `V0` (CLM-0606; SRC-0025 src/zkir-v3/Syntax.agda; source fact; not reproduced; high; S4).

ZKIR v3 specifies exactly 13 types in `IrType`, mechanized in [`Types.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Types.agda) and matching `zkir-v3/src/ir_types.rs` (CLM-0607; SRC-0025 src/zkir-v3/Types.agda; source fact; not reproduced; high; S4):
1. `Native`: element of the BLS12-381 scalar field `Fr`, with encoded length 1.
2. `Bytes32`: 32-byte sequence represented as two field elements, with encoded length 2.
3. `JubjubPoint`: point on the embedded twisted Edwards curve Jubjub, encoded as affine coordinates `(x, y)` with encoded length 2.
4. `JubjubScalar`: element of the Jubjub scalar field, with encoded length 1.
5. `Secp256k1Point`: point on the Weierstrass curve Secp256k1, encoded as 2 base field limbs for `x`, 2 base field limbs for `y`, and 1 is-identity flag, with encoded length 5.
6. `Secp256k1Base`: element of the Secp256k1 base field `Fp`, encoded as 2 native limbs, with encoded length 2.
7. `Secp256k1Scalar`: element of the Secp256k1 scalar field `Fq`, encoded as 2 native limbs, with encoded length 2.
8. `Secp256r1Point`: point on the Weierstrass curve Secp256r1, encoded as 5 limbs including the is-identity flag, with encoded length 5.
9. `Secp256r1Base`: element of the Secp256r1 base field `Fp`, encoded as 2 native limbs, with encoded length 2.
10. `Secp256r1Scalar`: element of the Secp256r1 scalar field `Fq`, encoded as 2 native limbs, with encoded length 2.
11. `Curve25519Point`: point on the twisted Edwards curve Curve25519 in prime-order subgroup, encoded as 2 base field limbs for `x` and 2 base field limbs for `y`, with encoded length 4 and no is-identity flag.
12. `Curve25519Base`: element of the Curve25519 base field `Fp` (modulo $2^{255}-19$), encoded as 2 native limbs, with encoded length 2.
13. `Curve25519Scalar`: element of the Curve25519 scalar field, encoded as 2 native limbs, with encoded length 2.

The type system and instruction set are further cross-linked to [zkir-type-system.md](zkir-type-system.md) and [zkir-instruction-set.md](zkir-instruction-set.md).
The off-circuit value domain is `IrValue`, which pairs each `IrType` constructor with its underlying mathematical carrier: `val-native`, `val-bytes32`, `val-jubjub-point`, `val-jubjub-scalar`, `val-secp256k1-point`, `val-secp256k1-base`, `val-secp256k1-scalar`, `val-secp256r1-point`, `val-secp256r1-base`, `val-secp256r1-scalar`, `val-curve25519-point`, `val-curve25519-base`, and `val-curve25519-scalar` (CLM-0607; SRC-0025 src/zkir-v3/Types.agda; source fact; not reproduced; high; S4).

The instruction set consists of 34 variants in `Instruction`:
`encode`, `assert`, `cond-select`, `constrain-bits`, `constrain-eq`, `constrain-to-boolean`, `copy`, `impact`, `ec-mul`, `ec-mul-generator`, `hash-to-curve`, `into-coordinates`, `from-coordinates`, `into-bytes32`, `from-bytes32`, `reverse-bytes`, `bytes32-into-low-high`, `bytes32-from-low-high`, `div-mod-power-of-two`, `reconstitute-field`, `transient-hash`, `persistent-hash`, `keccak256`, `test-eq`, `add`, `mul`, `neg`, `inv`, `not`, `less-than`, `jubjub-scalar-from-native`, `public-input`, `private-input`, and `circuit-output` (CLM-0608; SRC-0025 src/zkir-v3/Syntax.agda; source fact; not reproduced; high; S4).

### Operational semantics (off-circuit witness generation)

The operational semantics, designated *preprocess*, represents witness generation executed by the prover before proving (CLM-0609; SRC-0025 docs/zkir-v3-spec.md §6; source fact; not reproduced; high; S4).
It is mechanized as a deterministic step function in [`Semantics.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Semantics.agda) and discussed in [zkir-vm-semantics.md](zkir-vm-semantics.md).
The prover executes a circuit with a proof preimage `P` of type `ProofPreimage`:

```agda
record ProofPreimage : Set where
  field
    inputs                  : List Fr
    binding-input           : Fr
    comm-commitment         : Maybe (Fr × Fr)
    pub-transcript-inputs   : List Fr
    pub-transcript-outputs  : List Fr
    priv-transcript         : List Fr
```

The interpreter state `State` tracks execution progress (CLM-0609; SRC-0025 src/zkir-v3/Semantics.agda; source fact; not reproduced; high; S4):
- `mem : Identifier ⇀ IrValue`: a partial map from variable identifiers to typed runtime values.
- `pis : List Fr`: the public input vector constructed during execution.
- `pi-skips : List (Maybe ℕ)`: skip markers indicating active (`none`) or skipped (`some n`) groups of public inputs.
- `cur-pub-in : ℕ`: cursor tracking consumed expected public transcript inputs.
- `cur-pub-out : ℕ`: cursor tracking consumed public transcript outputs.
- `cur-priv : ℕ`: cursor tracking consumed private transcript elements.
- `outputs : List IrValue`: accumulator for circuit return values fed into the communications commitment.

Initial state construction `init S P` decodes `P.inputs` into the named registers in `S.inputs` according to each input's `encoded-len` (CLM-0610; SRC-0025 docs/zkir-v3-spec.md §6.2; source fact; not reproduced; high; S4).
It initializes `pis` to `[P.binding-input]`, appends `c` when `do-communications-commitment` is set and `P.comm-commitment = some (c, r)`, sets all cursors to zero, and initializes `outputs` to empty (CLM-0610; SRC-0025 docs/zkir-v3-spec.md §6.2; source fact; not reproduced; high; S4).
Execution proceeds sequentially by `step` across `S.instructions`, yielding a final state `s` via `run P S st0 (instructions S) = just s` (CLM-0610; SRC-0025 src/zkir-v3/Semantics.agda; source fact; not reproduced; high; S4).

A run accepts under two terminal conditions (CLM-0611; SRC-0025 docs/zkir-v3-spec.md §6.5; source fact; not reproduced; high; S4):
1. Terminal Condition 1 (TC1): all three preimage transcript streams are completely consumed (`cur-pub-in = |P.pub-transcript-inputs|`, `cur-pub-out = |P.pub-transcript-outputs|`, and `cur-priv = |P.priv-transcript|`).
2. Terminal Condition 2 (TC2): if `do-communications-commitment` is true, the commitment value `c` in `P.comm-commitment` must equal `transient-commit (P.inputs ++ encode(outputs)) r`.
When both conditions hold, `preprocess S P` evaluates to `just s` (CLM-0611; SRC-0025 src/zkir-v3/Semantics.agda; source fact; not reproduced; high; S4).

### Circuit semantics (in-circuit constraint synthesis)

The circuit semantics formalizes the constraint system synthesized from `IrSource` alone, without knowledge of the preimage (CLM-0612; SRC-0025 docs/zkir-v3-spec.md §7; source fact; not reproduced; high; S4).
Synthesis is mechanized in [`Circuit.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Circuit.agda) as a total function `synth : IrSource → Circuit` (CLM-0612; SRC-0025 src/zkir-v3/Circuit.agda; source fact; not reproduced; high; S4).
A circuit is represented by the record `Circuit`:

```agda
record Circuit : Set where
  field
    constraints : List Constraint
    pi-len      : ℕ
    has-comm    : Bool
```

The datatype `Constraint` represents an enumerable vocabulary of arithmetization primitives corresponding to chip calls in `midnight-zk-stdlib` and `midnight-circuits` (CLM-0612; SRC-0025 src/zkir-v3/Circuit.agda; source fact; not reproduced; high; S4).
These include gate operations (`gate-add`, `gate-mul`, `gate-neg`, `gate-inv`, `gate-copy`, `encode-eq`, `eq`, `boolean`, `non-zero`, `in-range`, `select`, `test-eq`, `is-not`, `lt`, `jubjub-scalar-from-native-c`), curve and hash operations (`ec-mul-c`, `ec-mul-gen-c`, `h2c-c`, `into-coords`, `from-coords`, `into-b32`, `from-b32`, `rev-b32`, `b32-into-lh`, `b32-from-lh`, `div-mod-c`, `reconstitute-c`, `trans-hash-c`, `pers-hash-c`, `keccak-c`), public input assignment (`pi-bind`), and commitment constraints (`comm`) (CLM-0612; SRC-0025 src/zkir-v3/Circuit.agda; source fact; not reproduced; high; S4).

A circuit witness `CircuitWitness` packages:
- `assign : Identifier → Maybe IrValue`: the in-circuit assignment to each variable name.
- `pis : List Fr`: the public input vector seen by the verifier.
- `comm-rand : Maybe Fr`: the private commitment randomness.

The relation `holds : CircuitWitness → Constraint → Set` interprets satisfaction for each primitive constraint atom (CLM-0613; SRC-0025 src/zkir-v3/Circuit.agda; source fact; not reproduced; high; S4).
Whole-circuit satisfaction `satisfies : Circuit → CircuitWitness → Set` requires that every constraint in `c.constraints` holds, `length (w.pis) ≡ c.pi-len`, and `w.comm-rand` matches `c.has-comm` (CLM-0613; SRC-0025 src/zkir-v3/Circuit.agda; source fact; not reproduced; high; S4).
For any run `(P, s)`, the canonical witness is defined by `witness-of P s = mk-witness (State.mem s) (State.pis s) (comm-rand-of (IrSource.do-communications-commitment S) P)` (CLM-0613; SRC-0025 src/zkir-v3/CircuitBridge.agda; source fact; not reproduced; high; S4).

## The cryptographic trust base and the `--safe` discipline

The cryptographic primitives and algebraic axioms are encapsulated in the record `Assumptions` in [`Assumptions.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Assumptions.agda) (CLM-0614; SRC-0025 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).
Every module in `src/zkir-v3` parameterizes over `Assumptions`:

```agda
module zkir-v3.Main (⋯ : _) (open Assumptions ⋯) where
```

The entire formal development typechecks under Agda's `--safe` flag and contains zero `postulate` statements (CLM-0614; SRC-0025 src/zkir-v3/Main.agda; source fact; not reproduced; high; S4).
The `Assumptions` record defines:
- Carrier types: `Fr`, `Alignment`, `JubjubPoint`, `JubjubScalar`, `Secp256k1Point`, `Secp256k1Base`, `Secp256k1Scalar`, `Secp256r1Point`, `Secp256r1Base`, `Secp256r1Scalar`, `Curve25519Point`, `Curve25519Base`, and `Curve25519Scalar` (CLM-0615; SRC-0025 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).
- Field operations on `Fr`: identities `0ᶠ` and `1ᶠ`, addition `_+ᶠ_`, multiplication `_*ᶠ_`, negation `-ᶠ_`, partial inversion `invᶠ`, decidable equality `_≟ᶠ_`, bit length `FR-BITS`, order `FR-ORDER`, and little-endian conversions `to-le-bits` and `from-le-bits` (CLM-0615; SRC-0025 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).
- Gadget functional contracts: point addition, scalar multiplication, group generators, affine coordinate projections, coordinate reconstruction, and byte conversions for Jubjub, Secp256k1, Secp256r1, and Curve25519 (CLM-0615; SRC-0025 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).
- Cryptographic hash functions: `transient-hash-fn`, `persistent-hash-fn`, `keccak-fn`, and `transient-commit` (CLM-0615; SRC-0025 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).

The assumed axioms in `Assumptions` are categorized into three groups:
- Group A (Field Non-Triviality): the single algebraic law `1ᶠ≢0ᶠ : ¬ (1ᶠ ≡ 0ᶠ)` (CLM-0616; SRC-0025 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).
- Group B (Valuation): `valFr x = bits-to-ℕ (to-le-bits x)` is defined directly without assumed axioms; numeric bounds in range checks and decomposition chips are trusted at the chip contract level (CLM-0616; SRC-0025 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).
- Group C (Typed-Encoding Round-Trips): mutual inverse laws between encoders and decoders on valid data (CLM-0616; SRC-0025 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).
The `*-round` laws assert `decode (encode v) ≡ just v` for valid values, while the `*-sound` laws assert `encode (decode limbs) ≡ limbs` on canonical limb representations (CLM-0616; SRC-0025 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).
These round-trip laws cover Jubjub coordinates and scalars, `Bytes32` low-high splits, and the limb representations of Secp256k1, Secp256r1, and Curve25519 (CLM-0616; SRC-0025 src/zkir-v3/Assumptions.agda; source fact; not reproduced; high; S4).

A critical trust note is documented in `Assumptions.agda` regarding foreign curve decoders: the Agda model assumes decoders are canonical partial inverses that reject non-canonical representations (CLM-0617; SRC-0025 src/zkir-v3/Assumptions.agda; repository observation; not reproduced; high; S4).
In the deployed Rust code of `midnight-circuits` (versions 7.2.2 and 7.2.4), `from_public_input` silently reduces non-canonical limb encodings modulo the foreign modulus, so the theorems transfer to the deployed implementation exactly on canonically encoded data (CLM-0617; SRC-0025 src/zkir-v3/Assumptions.agda; repository observation; not reproduced; high; S4).

## Theorem inventory

The mechanization in `src/zkir-v3` proves four central theorem families connecting the operational semantics to the constraint system (CLM-0618; SRC-0025 src/zkir-v3/README.md; source fact; not reproduced; high; S4).
All definitions and theorem signatures are cited verbatim from the Agda source files.

### 1. Circuit faithfulness (Property P5)

Circuit faithfulness establishes that witness generation succeeds if and only if the synthesized circuit accepts the canonical witness (CLM-0619; SRC-0025 docs/zkir-v3-spec.md §8.3; source fact; not reproduced; high; S4).
The program-level theorem is mechanized in [`CircuitProof.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/CircuitProof.agda) (CLM-0619; SRC-0025 src/zkir-v3/CircuitProof.agda; source fact; not reproduced; high; S4):

```agda
circuit-faithful : ∀ {S P s st0}
  → producer-WT S
  → init S P ≡ just st0
  → BwdWalk P S st0 (IrSource.instructions S) s
  → Consumed P s
  → (preprocess S P ≡ just s) ⇔ (satisfies (synth S) (witness-of P s))
```

Plain-language meaning: for a well-typed source `S` that satisfies single-assignment and typing obligations (`producer-WT S`), given that initial state evaluation succeeds (`init S P ≡ just st0`), that `s` has the step consumption shape of a backward walk (`BwdWalk`), and that all preimage transcripts are fully consumed (`Consumed P s`), running witness generation succeeds with final state `s` if and only if the synthesized constraint system `synth S` is satisfied by the canonical witness `witness-of P s` (CLM-0619; SRC-0025 src/zkir-v3/CircuitProof.agda; source fact; not reproduced; high; S4).

Faithfulness is factored into two directional theorems:

Forward faithfulness (`forward-sa` in `CircuitProof.agda`):

```agda
forward-sa : ∀ {S P s st0}
  → producer-SA S
  → init S P ≡ just st0
  → preprocess S P ≡ just s
  → satisfies (synth S) (witness-of P s)
```

Plain-language meaning: if the source satisfies static single assignment (`producer-SA S`), initial state evaluation succeeds, and witness generation succeeds, then the synthesized circuit is guaranteed to be satisfied by the generated canonical witness (CLM-0620; SRC-0025 src/zkir-v3/CircuitProof.agda; source fact; not reproduced; high; S4).
This proves completeness of constraint synthesis: every genuine witness generation run produces an accepting circuit assignment (CLM-0620; SRC-0025 docs/zkir-v3-spec.md §8.3; source fact; not reproduced; high; S4).

Backward faithfulness (`backward` in `CircuitProof.agda`):

```agda
backward : ∀ {S P s st0}
  → producer-WT S
  → init S P ≡ just st0
  → Consumed P s
  → BwdWalk P S st0 (IrSource.instructions S) s
  → satisfies (synth S) (witness-of P s)
  → run-shaped S P s
```

Plain-language meaning: if a circuit witness satisfies the synthesized constraints, the source is statically well-typed, transcripts are consumed, and the backward execution walk holds, then there exists an operational execution trace (`run-shaped`) explaining the witness (CLM-0621; SRC-0025 src/zkir-v3/CircuitProof.agda; source fact; not reproduced; high; S4).
The backward spine `BwdWalk` can be projected directly from an operational run using `preprocess→BwdWalk`, eliminating manual spine construction for callers holding an honest run (CLM-0621; SRC-0025 src/zkir-v3/CircuitProof.agda; source fact; not reproduced; high; S4).

### 2. Statement soundness

Statement soundness establishes that every satisfying circuit witness corresponds to a genuine operational execution agreeing on public inputs (CLM-0622; SRC-0025 docs/zkir-v3-spec.md §8.4; source fact; not reproduced; high; S4).
The result is mechanized in [`StatementSoundness.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/StatementSoundness.agda) (CLM-0622; SRC-0025 src/zkir-v3/StatementSoundness.agda; source fact; not reproduced; high; S4).
The realizer record `SubRealizer` packages the extracted witness data:

```agda
CommWF : Bool → Maybe (Fr × Fr) → Set
CommWF b cc = b ≡ false → cc ≡ nothing

record SubRealizer (S : IrSource) (w : CircuitWitness) : Set where
  constructor mk-subrealizer
  field
    P          : ProofPreimage
    s          : State
    shaped     : run-shaped S P s
    preproc-ok : preprocess S P ≡ just s
    pis-agree  : CircuitWitness.pis (witness-of P s) ≡ CircuitWitness.pis w
    mem-agree  : State.mem s ⊑ᵂ w
    rand-agree : IrSource.do-communications-commitment S ≡ true
      → CircuitWitness.comm-rand (witness-of P s)
        ≡ CircuitWitness.comm-rand w
    comm-wf    : CommWF (IrSource.do-communications-commitment S)
                        (ProofPreimage.comm-commitment P)
```

The main statement soundness theorem is:

```agda
statement-sound : ∀ {S w}
  → producer-WT S → satisfies (synth S) w → WShape S w
  → SubRealizer S w
```

Plain-language meaning: for any satisfying witness `w` conforming to the witness-shape predicate `WShape S w` on a statically well-typed source `S`, there exists an extracted preimage `P` and run state `s` such that `preprocess S P ≡ just s`, the public input vector of the extracted run matches `w.pis` exactly, the final memory of `s` is a sub-assignment `⊑ᵂ` of `w` agreeing on the entire run domain, the commitment randomness matches under the commitment flag, and vestigial commitments are excluded when the flag is disabled (CLM-0622; SRC-0025 src/zkir-v3/StatementSoundness.agda; source fact; not reproduced; high; S4).

Two companion theorems complete the soundness theory:

Extractor completeness (`extractor-complete` in `StatementSoundness.agda`):

```agda
extractor-complete : ∀ {S w}
  → producer-SA S
  → (r : SubRealizer S w)
  → satisfies (synth S)
      (witness-of (SubRealizer.P r) (SubRealizer.s r))
```

Plain-language meaning: the extracted preimage produced by `statement-sound` actually proves; the canonical witness generated from `(P, s)` satisfies the synthesized circuit (CLM-0623; SRC-0025 src/zkir-v3/StatementSoundness.agda; source fact; not reproduced; high; S4).

Non-vacuity of `WShape` (`preprocess→WShape` in `StatementSoundness.agda`):

```agda
preprocess→WShape : ∀ {S P s st0}
  → producer-SA S
  → init S P ≡ just st0
  → preprocess S P ≡ just s
  → WShape S (witness-of P s)
```

Plain-language meaning: every canonical witness produced by an honest preprocess run satisfies the `WShape` predicate, proving that `statement-sound` is non-vacuous and applies to all honestly provable executions (CLM-0623; SRC-0025 src/zkir-v3/StatementSoundness.agda; source fact; not reproduced; high; S4).

### 3. Extraction uniqueness

Extraction uniqueness proves that the extracted execution explaining a satisfying witness is mathematically unique (CLM-0624; SRC-0025 docs/zkir-v3-spec.md §8.4; source fact; not reproduced; high; S4).
The theorems are mechanized in [`StatementUniqueness.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/StatementUniqueness.agda) (CLM-0624; SRC-0025 src/zkir-v3/StatementUniqueness.agda; source fact; not reproduced; high; S4):

```agda
statement-unique : ∀ {S w} → producer-SA S → WInputs w (IrSource.inputs S)
  → (r r′ : SubRealizer S w)
  → (SubRealizer.P r ≡ SubRealizer.P r′)
  × (SubRealizer.s r ≡ SubRealizer.s r′)
```

Combined exactly-one packaging (`statement-sound-unique` in `StatementUniqueness.agda`):

```agda
statement-sound-unique : ∀ {S w}
  → producer-WT S → satisfies (synth S) w → WShape S w
  → Σ (SubRealizer S w) (λ r → ∀ (r′ : SubRealizer S w)
      → (SubRealizer.P r ≡ SubRealizer.P r′)
      × (SubRealizer.s r ≡ SubRealizer.s r′))
```

Plain-language meaning: for any satisfying witness of proper shape, there exists exactly one realizer; any two sub-realizers extracting from the same witness have identical preimages and identical execution states (CLM-0624; SRC-0025 src/zkir-v3/StatementUniqueness.agda; source fact; not reproduced; high; S4).
Consequently, the extraction process is a deterministic mathematical function of the witness (CLM-0624; SRC-0025 docs/zkir-v3-spec.md §8.4; source fact; not reproduced; high; S4).

## Module map of `src/zkir-v3`

The directory `src/zkir-v3/` consists of 16 files, comprising 15 Agda modules and one README documentation file (CLM-0625; SRC-0025 src/zkir-v3/README.md; repository observation; not reproduced; high; S4).
The dependency architecture forms a strict directed acyclic graph rooted in `Assumptions.agda`:

```
Assumptions → Types → Encoding → Syntax → Semantics → SemanticsProperties
  → Circuit → CircuitBridge → CircuitFaithfulness → CircuitBackward
  → Obligations → CircuitProof → StatementSoundness
  → StatementUniqueness → Main
```

| Module | Contents and Primary Definitions | Lines / Bytes |
|---|---|---|
| [`Assumptions.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Assumptions.agda) | Cryptographic trust base record: carrier types (`Fr`, `Alignment`, curves), field arithmetic, bit decomposition, Jubjub/foreign curve contracts, hash functions, non-triviality `1ᶠ≢0ᶠ`, Group C round-trips. | 557 lines / 28.5 KB |
| [`Types.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Types.agda) | Definitions of `IrType` (13 constructors), `IrValue` (13 constructors), `encoded-len`, `typeof`, decidable type equality `_≟T_`. | 387 lines / 17.9 KB |
| [`Encoding.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Encoding.agda) | Wire format transformations `encode : IrValue → List Fr` and `decode : IrType → List Fr → Maybe IrValue`. | 96 lines / 4.0 KB |
| [`Syntax.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Syntax.agda) | Abstract syntax: `Identifier`, `Operand` (`var`, `imm`), `TypedIdentifier`, `IrMinorVersion` (`V0`), the 34 `Instruction` variants, and record `IrSource`. | 296 lines / 8.2 KB |
| [`Semantics.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Semantics.agda) | Operational semantics interpreter: `ProofPreimage`, `State`, `step`, `run`, `init`, `preprocess`. | 577 lines / 27.5 KB |
| [`SemanticsProperties.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/SemanticsProperties.agda) | Structural properties of operational semantics: store ordering `_⊑_`, domain growth `step-dom`, memory extension `run-extends`, run inversion `run-inv`, and `preprocess-walk-consumed`. | 772 lines / 40.1 KB |
| [`Circuit.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Circuit.agda) | Constraint vocabulary `Constraint`, witness model `CircuitWitness`, satisfaction relations `holds` and `satisfies`, and synthesis function `synth`. | 939 lines / 40.5 KB |
| [`CircuitBridge.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/CircuitBridge.agda) | Bridge definitions: canonical witness constructor `witness-of`, constraint monotonicity `holds-mono`, constraint lowering `holds-lower`, and constraint extractor `csOf`. | 1,173 lines / 55.2 KB |
| [`CircuitFaithfulness.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/CircuitFaithfulness.agda) | Forward faithfulness: per-instruction forward lemmas `*-fwd` and program-level induction `forward`. | 2,828 lines / 125.9 KB |
| [`CircuitBackward.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/CircuitBackward.agda) | Backward step reconstruction: 43 per-instruction inversion lemmas `*-bwd` reconstructing operational transitions from constraint satisfaction. | 2,130 lines / 106.9 KB |
| [`Obligations.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Obligations.agda) | Static producer checks: single assignment `producer-SA`/`producer-SA?`, value typing `producer-WT`/`producer-WT?`, bit bounds `producer-WF2`/`producer-WF2?`, and transfer theorem `preprocessʳ-agree`. | 2,499 lines / 129.2 KB |
| [`CircuitProof.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/CircuitProof.agda) | Program-level theorem assembly: `BwdWalk`, `bwd-go`, run-spine projection `preprocess→BwdWalk`, `backward`, `forward-sa`, and headline theorem `circuit-faithful`. | 2,151 lines / 112.2 KB |
| [`StatementSoundness.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/StatementSoundness.agda) | Statement soundness: witness shape predicate `WShape`/`WShape?`, preimage construction `build`, record `SubRealizer`, `statement-sound`, `extractor-complete`, and `preprocess→WShape`. | 6,144 lines / 318.6 KB |
| [`StatementUniqueness.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/StatementUniqueness.agda) | Extraction uniqueness: transcript pinning lemmas, `statement-unique`, and exactly-one packaging `statement-sound-unique`. | 915 lines / 47.6 KB |
| [`Main.agda`](file:///home/charl/Moriarty/repos/input-output-hk/arc-zkir/src/zkir-v3/Main.agda) | Aggregation module importing all files above, verifying whole-development compilation under `--safe`. | 27 lines / 1.0 KB |

## Evolution from ZKIR v2 to ZKIR v3

The transition from ZKIR v2 to ZKIR v3 represents a major architectural overhaul documented in `docs/zkir-v3-spec.md` Appendix C (CLM-0626; SRC-0025 docs/zkir-v3-spec.md Appendix C; source fact; not reproduced; high; S4):

| Dimension | ZKIR v2 | ZKIR v3 |
|---|---|---|
| Value Domain | Unityped raw native field elements (`Fr`). | Strongly typed domain (`IrType` and `IrValue`) covering 13 distinct types. |
| Register Memory | Positional integer indexing (`Index = ℕ`) into an append-only register file `List Fr`. | Named value store mapping string identifiers `Identifier ⇀ IrValue`. |
| Supported Curves | Jubjub only (native twisted Edwards curve). | Jubjub plus three foreign curves: Secp256k1, Secp256r1, and Curve25519. |
| Byte Types | No byte type; byte operations simulated over field elements. | First-class `Bytes32` type with byte-level manipulation operations. |
| Operands | Positional wire index `Index` plus dedicated `load_imm` instruction. | Sum type `Operand = Variable Identifier \| Immediate Fr` without `load_imm`. |
| Public Inputs | `DeclarePubInput` instruction paired with non-contiguous `PiSkip` markers. | Fused `Impact` instruction declaring guarded public input blocks. |
| Outputs | Single implicit return value via `Output(var)` instruction. | Explicit positional signature `outputs: List IrType` and `Output` terminator. |
| Instruction Count | 26 primitive instructions. | 34 primitive instructions. |
| Encoding Layer | Implicit; every value was already a single field element. | First-class, security-critical encoding layer (`encode`/`decode`) crossing all boundaries. |
| Commitment Scope | Commitment taken only over the output vector. | Commitment taken over `inputs ++ encode(outputs)`. |
| Minor Versions | Minor versions `V0` and `V1` on `ledger-8` (and `V2` on `ledger-9`). | Minor version `V0` only; major version gated at deserialization. |

## The superseded types-proposal (0021-ZKIR-redesign.md)

Under `types-proposal/0021-ZKIR-redesign.md`, an earlier redesign proposal for ZKIR v3 was drafted (lifecycle status S0) (CLM-0627; SRC-0025 types-proposal/0021-ZKIR-redesign.md; source fact; not reproduced; high; S0).
The proposal advocated an ambitious Static Single Assignment (SSA) redesign featuring nested block control flow, explicit $\phi$ (phi) merge functions after branches, semiring memory shapes ($\mu := \mathbb{0} \mid \mathbb{1} \mid \mu_1 \oplus \mu_2 \mid \mu_1 \otimes \mu_2 \mid \langle T \rangle$), and qualified wire polymorphism mimicking Hindley-Milner type inference with trait constraints (CLM-0627; SRC-0025 types-proposal/0021-ZKIR-redesign.md; source fact; not reproduced; high; S0).
In that design, conditional execution would evaluate both branches off-chain and merge states using explicit join nodes (CLM-0627; SRC-0025 types-proposal/0021-ZKIR-redesign.md; source fact; not reproduced; high; S0).

The proposal was entirely superseded by the implementation adopted in the `midnight-ledger` `zkir-v3` crate (CLM-0628; SRC-0025 README.md; source fact; not reproduced; high; S4).
The implemented ZKIR v3 preserved straight-line instruction sequences without nested branching or phi nodes, implemented conditional selection through `CondSelect` and guarded `Impact` instructions, and avoided qualified polymorphism in favor of static type-directed dispatch over a closed 13-type set (CLM-0628; SRC-0025 docs/zkir-v3-spec.md §1.3; source fact; not reproduced; high; S4).
The accompanying sketch `types-proposal/ZKIR.agda` remains an unverified historical artifact (CLM-0628; SRC-0025 README.md; repository observation; not reproduced; high; S0).

## The Charon/Aeneas/Lean toolchain infrastructure

The repository contains a `lean/` directory providing build infrastructure for extracting real Rust source code to Lean 4 (CLM-0629; SRC-0025 lean/README.md; source fact; not reproduced; high; S4).
The extraction pipeline uses Charon to translate Rust MIR into Low-Level Borrow Calculus (LLBC), followed by Aeneas to translate LLBC into purely functional Lean 4 models (CLM-0629; SRC-0025 lean/README.md; source fact; not reproduced; high; S4).
Aeneas is pinned to bare commit `daa85d7e89400fa978be83fedbc7e475a83f0889` (`origin/main` as of 2026-08-15), and Charon is pinned transitively via Aeneas's `charon-pin` file at version `0.1.232` (CLM-0629; SRC-0025 lean/README.md; source fact; not reproduced; high; S4).

The toolchain is designed to extract small standalone Rust crates and prove functional properties about gadget algorithms in Lean 4 (such as the soundness proof for the `ReconstituteField` arithmetic) (CLM-0630; SRC-0025 lean/README.md; source fact; not reproduced; high; S4).
The toolchain flake in `lean/flake.nix` provides dependencies for building `charon` and `aeneas` binaries and running `lake` against Aeneas's Mathlib oleans (CLM-0630; SRC-0025 lean/README.md; source fact; not reproduced; high; S4).

## Development timeline and biweekly reports

The development progress of `arc-zkir` is documented in five biweekly reports and the git commit log (CLM-0631; SRC-0025 repos/_extracts/arc-zkir-git-log.txt; repository observation; not reproduced; high; S4):
- 2026-04-30: Initial repository commit introducing the early types proposal and Agda sketch (commit `70c45f4`) (CLM-0631; SRC-0025 repos/_extracts/arc-zkir-git-log.txt; repository observation; not reproduced; high; S0).
- 2026-05-19 to 2026-05-29: Setup of CI, ZKIR v2 syntax, semantics, and initial faithfulness and soundness proofs (CLM-0631; SRC-0025 repos/_extracts/arc-zkir-git-log.txt; repository observation; not reproduced; high; S4).
- 2026-06-01 to 2026-06-19 (`biweekly-reports/2026-06-19.md`): ZKIR v2 specification written; central theorems P5 (circuit faithfulness), statement soundness, and extraction uniqueness proven; constraint language refactored into explicit constructors with decidable satisfaction; consultation with Denis Firsov and Cas van der Rest (CLM-0632; SRC-0025 biweekly-reports/2026-06-19.md; source fact; not reproduced; high; S4).
- 2026-06-20 to 2026-07-04 (`biweekly-reports/2026-07-04.md`): ZKIR v2 closed out with memory length internalized into `satisfies` and external review merged (PR #42); ZKIR v3 started directly from `midnight-ledger` crate `zkir-v3` pinned at `d34a6723` (`3.0.0-rc.1`); initial 9,500-line Agda development created for the Native + Jubjub + Bytes32 fragment (33 instructions); forward faithfulness proven and backward faithfulness proven in given-spine form (CLM-0633; SRC-0025 biweekly-reports/2026-07-04.md; source fact; not reproduced; high; S4).
- 2026-07-05 to 2026-07-17 (`biweekly-reports/2026-07-17.md`): ZKIR v3 re-pinned to `b17df9d1` (`3.0.0-rc.2`), adding `reverse_bytes` and retyping hashes to reach 34 instructions; Milestone B completed reaching full v2 parity with `circuit-faithful`, `statement-sound`, and `statement-sound-unique` proven; presentation on 15 July to Shielded team members Miguel Ambrona, Kevin Millikin, and Darren Oliveiro-Priestnall; divergence review memo drafted (CLM-0634; SRC-0025 biweekly-reports/2026-07-17.md; source fact; not reproduced; high; S4).
- 2026-07-18 to 2026-07-31 (`biweekly-reports/2026-07-31.md`): Milestone C completed adding Secp256k1 (seven types total, 14 opcodes gained dispatch arms); pin moved to `a1d1611c`; strengthening pass added run-spine projection `preprocess→BwdWalk`, `WShape` non-vacuity `preprocess→WShape`, and `extractor-complete`; four PRs opened to contribute specs and Agda to `midnight-ledger` (#656, #657 against `ledger-8`, #667, #668 against `ledger-9`); v3 merged to main (PR #44) (CLM-0635; SRC-0025 biweekly-reports/2026-07-31.md; source fact; not reproduced; high; S4).
- 2026-08-01 to 2026-08-14 (`biweekly-reports/2026-08-14.md`): Milestone D completed adding Secp256r1 (PR #50) and Curve25519 (PR #59), achieving the full 13-type surface across all headline theorems; conformance sweeps executed; v3 spec and divergence review re-pinned to `92e8bdd3` (`midnight-zkir-v3 3.0.0`) (CLM-0636; SRC-0025 biweekly-reports/2026-08-14.md; source fact; not reproduced; high; S4).
- 2026-08-15 to 2026-08-26: Lean toolchain flake added (commit `5cdd548`); ECFamily dispatch refactor landed; divergence review root-cause classification updated; foreign curve validity gates audited (commit `fd1c24e`) (CLM-0636; SRC-0025 repos/_extracts/arc-zkir-git-log.txt; repository observation; not reproduced; high; S4).

## Conceptual comparison with Matter Labs CLAP

The relationship between the ZKIR mechanization and Matter Labs' CLAP eDSL (Stronati et al., arXiv:2405.12115v1) is documented in `docs/clap-comparison.md` (CLM-0637; SRC-0025 docs/clap-comparison.md; source fact; not reproduced; high; S4).
Both formalisms prove compiler faithfulness connecting a high-level circuit description to a lower-level constraint system, but they differ fundamentally in structure and scope (CLM-0637; SRC-0025 docs/clap-comparison.md; source fact; not reproduced; high; S4):

1. Terminology Inversion:
In CLAP, `circuit` is the source high-level program and `cs` is the target constraint system (CLM-0637; SRC-0025 docs/clap-comparison.md §⚠️ Terminology inversion; source fact; not reproduced; high; S4).
In ZKIR, `IrSource` evaluated under preprocess semantics is the source, while the target constraint system is the type named `Circuit` produced by the synthesis function `circuit` (in v2) or `synth` (in v3) (CLM-0637; SRC-0025 docs/clap-comparison.md §⚠️ Terminology inversion; source fact; not reproduced; high; S4).

2. Scope and Trust Base:
CLAP's mechanized Agda core fixes `Field = ℤ mod n` and verifies only three gates (`const`, `add`, `eq0`) over an additive constraint language (CLM-0638; SRC-0025 docs/clap-comparison.md §Scope caveat; source fact; not reproduced; high; S4).
ZKIR mechanizes an abstract trust base `Assumptions` over `Fr` and foreign fields, supporting 34 instructions including field inversion, lookup range checks, elliptic curve point arithmetic, and cryptographic hash functions (CLM-0638; SRC-0025 docs/clap-comparison.md; source fact; not reproduced; high; S4).

3. Closed versus Open Instruction Set:
CLAP uses an open, typeclass-style `Gate` abstraction to allow third-party gate extensions (CLM-0639; SRC-0025 docs/clap-comparison.md §Syntax; source fact; not reproduced; high; S4).
ZKIR defines a closed instruction set fixed by the specification and Rust implementation, ensuring exhaustive case splitting and direct alignment with concrete compiler lowerings (CLM-0639; SRC-0025 docs/clap-comparison.md §Takeaways; source fact; not reproduced; high; S4).

4. Witness Uniqueness versus Equivalence:
CLAP proves witness satisfaction only up to an I/O equivalence relation `≈_c` to tolerate non-determinism introduced by oracle gates such as inverse-or-zero (CLM-0640; SRC-0025 docs/clap-comparison.md §Correctness properties; source fact; not reproduced; high; S4).
ZKIR models chips and gadget operations as deterministic canonical functions, allowing the mechanization to prove exact extraction uniqueness (`statement-sound-unique`) where the extractor is a genuine function of the witness (CLM-0640; SRC-0025 docs/clap-comparison.md §Takeaways; source fact; not reproduced; high; S4).

## Comparison of Rust IR definitions: ledger-9 extract versus midnight-zkir

A direct code comparison was conducted between the `zkir-v3` crate extracted from `midnight-ledger` commit `92e8bdd3` (`/home/charl/Moriarty/repos/_extracts/ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src/`) and the standalone crate repository `midnightntwrk/midnight-zkir` at commit `2ffe2d17bbb736aec36fb300aeaca679a10d2278` (`/home/charl/Moriarty/repos/midnightntwrk/midnight-zkir/zkir/src/`) (CLM-0641; SRC-0006 repos/_extracts/ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src/ir.rs; repository observation; not reproduced; high; S4).
The comparison reveals material differences in the `Instruction` and `IrType` enum definitions (CLM-0641; SRC-0006 repos/midnightntwrk/midnight-zkir/zkir/src/ir.rs; repository observation; not reproduced; high; S4):

### 1. Instruction enum differences

In `ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src/ir.rs` (lines 360 to 919), `pub enum Instruction` contains exactly 34 variants:
`Encode`, `Assert`, `CondSelect`, `ConstrainBits`, `ConstrainEq`, `ConstrainToBoolean`, `Copy`, `Impact`, `EcMul`, `EcMulGenerator`, `HashToCurve`, `IntoCoordinates`, `FromCoordinates`, `IntoBytes32`, `FromBytes32`, `ReverseBytes`, `Bytes32IntoLowHigh`, `Bytes32FromLowHigh`, `DivModPowerOfTwo`, `ReconstituteField`, `TransientHash`, `PersistentHash`, `Keccak256`, `TestEq`, `Add`, `Mul`, `Neg`, `Inv`, `Not`, `LessThan`, `JubjubScalarFromNative`, `PublicInput`, `PrivateInput`, and `Output` (CLM-0642; SRC-0006 repos/_extracts/ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src/ir.rs:360-919; source fact; not reproduced; high; S4).

In `midnightntwrk/midnight-zkir/zkir/src/ir.rs` (lines 390 to 1082), `pub enum Instruction` contains 42 variants, eight more than the pinned 92e8bdd3 tree, and one variant is renamed (CLM-0642; SRC-0006 repos/midnightntwrk/midnight-zkir/zkir/src/ir.rs:390-1082; repository observation; reproduced by variant count over both files on 2026-09-03; high; S5).
The differences relative to 92e8bdd3 are (CLM-0642; SRC-0006 repos/midnightntwrk/midnight-zkir/zkir/src/ir.rs; repository observation; reproduced; high; S5):
- `ReverseBytes` is renamed `Reverse`.
- `Slice`, `Nth`, `Concat` and `LoadConstant` are added among the byte and value manipulation instructions.
- `Sha512` is added beside `Keccak256` among the hash instructions.
- `And`, `Or` and `Xor` are added after `Not` as boolean gates over non-empty lists of `Bool` values (lines 968 to 1003).
Any K semantics that targets the arc-zkir specification therefore covers the 34-instruction surface at 92e8bdd3, and the eight additional instructions of the standalone repository are outside the mechanized specification until arc-zkir is updated (CLM-0642; SRC-0025 README.md; inference; not reproduced; high; S4).

### 2. IrType enum differences

In `ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src/ir_types.rs` (lines 36 to 89), `pub enum IrType` defines 13 variants:
`Native`, `Bytes32`, `JubjubPoint`, `JubjubScalar`, `Secp256k1Point`, `Secp256k1Base`, `Secp256k1Scalar`, `Secp256r1Point`, `Secp256r1Base`, `Secp256r1Scalar`, `Curve25519Point`, `Curve25519Base`, and `Curve25519Scalar` (CLM-0643; SRC-0006 repos/_extracts/ledger9-92e8bdd3-zkir-v3-src/zkir-v3/src/ir_types.rs:36-89; source fact; not reproduced; high; S4).

In `midnightntwrk/midnight-zkir/zkir/src/ir_types.rs` (lines 56 to 106), `pub enum IrType` carries attribute `#[tag = "ir-type[v1]"]` and defines 15 variants (CLM-0643; SRC-0006 repos/midnightntwrk/midnight-zkir/zkir/src/ir_types.rs:56-106; source fact; not reproduced; high; S4):
- It adds variant `Bool` (line 62).
- It adds variant `Byte` (line 65).
- It replaces `Bytes32` with a parameterized byte array type `Bytes(u32)` (line 72), constrained by `pub const MAX_BYTES_LEN: u32 = 1 << 24` (line 45).
- It retains the 11 curve and field types (`Native`, `JubjubPoint`, `JubjubScalar`, and the Secp256k1, Secp256r1, and Curve25519 triples).

Inference: the standalone `midnight-zkir` repository at commit `2ffe2d17` represents a later iteration incorporating the `Bool`/`Byte`/`Bytes(n)` pull request chain (`midnight-ledger` PRs #648 to #650), whereas the `arc-zkir` mechanization and specification track the `ledger-9` commit `92e8bdd3` baseline with 34 instructions and 13 types (CLM-0644; SRC-0025 docs/zkir-v3-divergence-review.md; inference; not reproduced; high; S4).

## Stated limitations and verification perimeter

The specification and mechanization authors document several structural limitations in `docs/zkir-v3-spec.md` §9 and `src/zkir-v3/README.md` (CLM-0645; SRC-0025 docs/zkir-v3-spec.md §9; source fact; not reproduced; high; S4):

1. Temporary Instructions:
`Bytes32IntoLowHigh` and `Bytes32FromLowHigh` are explicitly documented in the Rust implementation as a temporary bridge for Compact, intended for removal once Compact natively supports arbitrary-length byte strings without field decomposition (CLM-0645; SRC-0025 docs/zkir-v3-spec.md §9; source fact; not reproduced; high; S4).

2. Unimplemented In-Circuit FAB Segments:
In the field-aligned binary (FAB) alignment decoder used by `PersistentHash` and `Keccak256`, `AlignmentSegment::Option` is unimplemented in-circuit and causes a synthesis error (CLM-0645; SRC-0025 docs/zkir-v3-spec.md §9; source fact; not reproduced; high; S4).

3. Jubjub Scalar Bit-Count Workaround:
`midnight-curves` version `0.2.0` incorrectly reports Jubjub scalar `NUM_BITS` as 255 instead of 252; in-circuit encoding compensates by asserting that the spurious high limb is zero (CLM-0646; SRC-0025 docs/zkir-v3-spec.md §9; source fact; not reproduced; high; S4).

4. Curve Identity Encoding Asymmetries:
On Weierstrass curves (Secp256k1 and Secp256r1), the group identity has no affine coordinates; the encoding represents points with an explicit fifth element boolean identity flag, and `IntoCoordinates` enforces non-identity (CLM-0646; SRC-0025 docs/zkir-v3-spec.md §9; source fact; not reproduced; high; S4).
On Curve25519 (a twisted Edwards curve), the identity has genuine affine coordinates $(0, 1)$; its encoding requires only 4 field elements without an identity flag, and `IntoCoordinates` is a total function (CLM-0646; SRC-0025 docs/zkir-v3-spec.md §9; source fact; not reproduced; high; S4).

5. Verification Perimeter and Exclusions:
The mechanization operates at the circuit layer and explicitly excludes two aspects (CLM-0647; SRC-0025 docs/zkir-v3-spec.md §9.1; source fact; not reproduced; high; S4):
- Contract-level witness invariants: Compact contracts execute off-chain in TypeScript to generate witness values; invariants assumed by contract logic but not enforced by in-circuit constraints are outside the ZKIR verification boundary.
- Chip polynomial correctness: the Halo2 custom gates, permutation arguments, and lookup tables in `midnight-circuits` are trusted to implement their functional contracts; verification of the underlying polynomial identities remains an open gap for the proving backend.

## Architectural implications for K semantics

Inference: the Agda mechanization establishes the foundational proof architecture for an executable formal semantics of ZKIR in the K Framework, cross-linked to [../zkir-k-semantics-plan.md](../zkir-k-semantics-plan.md) and [../k-framework/k-user-manual.md](../k-framework/k-user-manual.md) (CLM-0648; SRC-0025 src/zkir-v3/CircuitProof.agda; inference; not reproduced; high; S4).
First, the K semantics configuration must support dual execution modes or state components mirroring the `preprocess` witness store and the synthesized constraint system (CLM-0648; SRC-0025 src/zkir-v3/Semantics.agda; inference; not reproduced; high; S4).
Second, the K semantics can adopt the closed 13-type set and 34-instruction signature from commit `92e8bdd3` as a verified target baseline, while isolating the `And`/`Or`/`Xor` extensions as an incremental layer (CLM-0648; SRC-0025 src/zkir-v3/Syntax.agda; inference; not reproduced; high; S4).
Third, because Agda proved P5 equivalence under explicit producer obligations (`producer-SA`, `producer-WT`, `producer-WF2`), the K definition should incorporate checkable well-formedness rules to guard symbolic execution and test suite conformance (CLM-0648; SRC-0025 src/zkir-v3/Obligations.agda; inference; not reproduced; high; S4).
Fourth, the implementation divergences identified in `docs/zkir-v3-divergence-review.md` define mandatory negative test cases that any executable formal semantics must precisely classify (CLM-0648; SRC-0025 docs/zkir-v3-divergence-review.md; inference; not reproduced; high; S4).
