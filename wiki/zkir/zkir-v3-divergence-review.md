---
id: zkir.v3.divergence-review
type: contradiction
title: ZKIR v3 in-circuit versus off-circuit divergences
status: active
updated_at: 2026-09-03T14:37:43Z
sources:
  - SRC-0038
---

# ZKIR v3 in-circuit versus off-circuit divergences

## Context and methodology

During the formalization of ZKIR v3 in `arc-zkir`, the authors conducted an exhaustive comparative audit of the off-circuit operational semantics against the in-circuit constraint synthesis in `midnight-ledger` (CLM-0650; SRC-0038 docs/zkir-v3-divergence-review.md; source fact; not reproduced; high; S4).
The audit is documented in `docs/zkir-v3-divergence-review.md` and tracks `midnight-ledger` commit `92e8bdd3` (crate `midnight-zkir-v3 3.0.0`) and `midnight-circuits` versions 7.2.2 and 7.2.4 (CLM-0650; SRC-0038 docs/zkir-v3-divergence-review.md; source fact; not reproduced; high; S4).

In ZKIR, a single circuit specification file (`ir.rs`) serves two distinct execution engines in `ir_vm.rs` (CLM-0651; SRC-0038 docs/zkir-v3-divergence-review.md §Purpose and scope; source fact; not reproduced; high; S4):
1. Off-circuit preprocessing (`IrSource::preprocess`): a concrete interpreter executing over native and foreign values to generate the prover witness and evaluate conditional control flow.
2. In-circuit synthesis (`IrSource::circuit` / `synth`): a constraint generator lowering instructions to custom gates and lookup arguments in Halo2 via `midnight-circuits`.

A divergence occurs when the language accepted or computed by the off-circuit preprocessor differs from the relation enforced by the in-circuit constraint system (CLM-0651; SRC-0038 docs/zkir-v3-divergence-review.md §Purpose and scope; source fact; not reproduced; high; S4).
The review classifies divergences into four root causes (CLM-0652; SRC-0038 docs/zkir-v3-divergence-review.md §Root-cause classification; source fact; not reproduced; high; S4):
- Class A: Functional bugs where the constraint system accepts executions rejected by preprocessing, or rejects executions accepted by preprocessing, risking false statement acceptance or unprovable honest statements.
- Class B: Undefined behavior where an instruction precondition is documented as undefined behavior off-circuit but enforced via satisfiability in-circuit.
- Class C: Decoder canonicity laxness where off-circuit decoders accept non-canonical representations that the canonical encoder never emits.
- Class D: Surface inconsistencies such as stale documentation comments or unused instruction variants.

## Inventory of review findings

The review documents thirteen numbered findings across the instruction set and type system (CLM-0653; SRC-0038 docs/zkir-v3-divergence-review.md; source fact; not reproduced; high; S4).
Findings 1, 2, and 4 were originally flagged as potential divergences and subsequently retired after being formally classified as producer obligations and undefined behavior in the specification (CLM-0653; SRC-0038 docs/zkir-v3-divergence-review.md; repository observation; not reproduced; high; S4).
Finding 3 was analyzed and confirmed to be intentional circuit design (CLM-0653; SRC-0038 docs/zkir-v3-divergence-review.md §3; source fact; not reproduced; high; S4).
Findings 5 through 13 identify active discrepancies, bug fixes, or missing chip integrations (CLM-0653; SRC-0038 docs/zkir-v3-divergence-review.md; source fact; not reproduced; high; S4).

| Finding ID and Feature | Off-Circuit Behaviour | In-Circuit Behaviour | Consequence | Review Recommendation | Status |
|---|---|---|---|---|---|
| Finding 1: `ReconstituteField` Modulo Wrapping | Checks $modulus < 2^{bits}$ and computes $divisor \cdot 2^{bits} + modulus$, silently reducing modulo $|Fr|$ if the result exceeds the field order (CLM-0654; SRC-0038 docs/zkir-v3-divergence-review.md §1; source fact; not reproduced; high; S4). | Halo2 range check chip enforces the exact integer recomposition equation without modular reduction (CLM-0654; SRC-0038 docs/zkir-v3-divergence-review.md §1; source fact; not reproduced; high; S4). | A prover witness with integer sum exceeding $|Fr|$ succeeds off-circuit but produces an unsatisfiable constraint system in-circuit (CLM-0654; SRC-0038 docs/zkir-v3-divergence-review.md §1; source fact; not reproduced; high; S4). | Classified as undefined behavior in specification §6.6(d) and enforced by producer obligation O4 (`WShape`); verify recomposition arithmetic via Aeneas/Lean (CLM-0654; SRC-0038 docs/zkir-v3-divergence-review.md §1; source fact; not reproduced; high; S4). | Retired from active bugs; specified as undefined behavior (CLM-0654; SRC-0038 docs/zkir-v3-divergence-review.md §1; source fact; not reproduced; high; S4). |
| Finding 2: `Assert` Condition Booleanity | Checks that the condition equals 1, returning an error if $cond \ne 1$ (CLM-0655; SRC-0038 docs/zkir-v3-divergence-review.md §2; source fact; not reproduced; high; S4). | Invokes `assert_not_zero`, which enforces only that $cond \ne 0$ (CLM-0655; SRC-0038 docs/zkir-v3-divergence-review.md §2; source fact; not reproduced; high; S4). | For condition values in $2 \dots |Fr|-1$, off-circuit execution errors while the circuit constraint would be satisfied (CLM-0655; SRC-0038 docs/zkir-v3-divergence-review.md §2; source fact; not reproduced; high; S4). | Classified as undefined behavior in specification §6.6(d) and obligation O4; compiler frontends must guarantee boolean operands (CLM-0655; SRC-0038 docs/zkir-v3-divergence-review.md §2; source fact; not reproduced; high; S4). | Retired from active bugs; specified as undefined behavior (CLM-0655; SRC-0038 docs/zkir-v3-divergence-review.md §2; source fact; not reproduced; high; S4). |
| Finding 3: `PublicInput` / `PrivateInput` Guard Uncoupling | Reads guard as boolean; if guard is false, assigns type default value; if guard is true, decodes next transcript element (CLM-0656; SRC-0038 docs/zkir-v3-divergence-review.md §3; source fact; not reproduced; high; S4). | Allocates a free witness variable constrained only by type; the guard wire takes no part in circuit constraints (CLM-0656; SRC-0038 docs/zkir-v3-divergence-review.md §3; source fact; not reproduced; high; S4). | Multiple distinct in-circuit witnesses satisfy the circuit when the guard is false; transcript inputs are not coupled to the guard inside the circuit (CLM-0656; SRC-0038 docs/zkir-v3-divergence-review.md §3; source fact; not reproduced; high; S4). | Document for auditors and compiler writers that transcript input cells are free in-circuit and guard coupling must be enforced downstream (CLM-0656; SRC-0038 docs/zkir-v3-divergence-review.md §3; source fact; not reproduced; high; S4). | By design; intentional prover slack for private transcript reads (CLM-0656; SRC-0038 docs/zkir-v3-divergence-review.md §3; source fact; not reproduced; high; S4). |
| Finding 4: `LessThan` Bit-Width Bounds | Compares $a < b$ as integers with bounds $a, b < 2^{bits}$ (CLM-0657; SRC-0038 docs/zkir-v3-divergence-review.md §4; source fact; not reproduced; high; S4). | Low-level chip rounds bit width up to an even bound $\max(n + n\%2, 4)$ (CLM-0657; SRC-0038 docs/zkir-v3-divergence-review.md §4; source fact; not reproduced; high; S4). | Minor bound discrepancy at odd bit widths if inputs exceed nominal bit width (CLM-0657; SRC-0038 docs/zkir-v3-divergence-review.md §4; source fact; not reproduced; high; S4). | Align formal specification with chip implementation and document effective evened bound (CLM-0657; SRC-0038 docs/zkir-v3-divergence-review.md §4; source fact; not reproduced; high; S4). | Retired; Agda model aligned with chip semantics (CLM-0657; SRC-0038 docs/zkir-v3-divergence-review.md §4; source fact; not reproduced; high; S4). |
| Finding 5: `Secp256k1` / Foreign Decode Canonicity | Off-circuit `from_public_input` in `midnight-circuits` accepts non-canonical limbs $\ge p_{foreign}$ and silently reduces them modulo the field order; accepts identity flag outside $\{0, 1\}$; short-circuits on identity flag without validating coordinates; panics on limbs $\ge 2^{192}$ (CLM-0658; SRC-0038 docs/zkir-v3-divergence-review.md §5; source fact; not reproduced; high; S4). | In-circuit circuits enforce canonical limb range checks and strict point validity (CLM-0658; SRC-0038 docs/zkir-v3-divergence-review.md §5; source fact; not reproduced; high; S4). | Prover supplying non-canonical limbs in preimages or transcript reads is accepted off-circuit but cannot generate a valid proof; Agda theorems transfer only to canonical data (CLM-0658; SRC-0038 docs/zkir-v3-divergence-review.md §5; source fact; not reproduced; high; S4). | Require canonical limb validation in `from_public_input`; validate identity flag $\in \{0, 1\}$; inspect coordinates regardless of identity flag (CLM-0658; SRC-0038 docs/zkir-v3-divergence-review.md §5; source fact; not reproduced; high; S4). | Panic on limbs $\ge 2^{192}$ fixed in circuits 7.2.4; silent reduction and identity flag laxness remain open (CLM-0658; SRC-0038 docs/zkir-v3-divergence-review.md §5; source fact; not reproduced; high; S4). |
| Finding 6: `CondSelect` Type Asymmetry | Permitted on `Bytes32` and `JubjubScalar` off-circuit (CLM-0659; SRC-0038 docs/zkir-v3-divergence-review.md §6; source fact; not reproduced; high; S4). | In-circuit synthesis in `ir_vm.rs` explicitly lacks arms for `Bytes32` and `JubjubScalar`, failing with `Error::Synthesis` (CLM-0659; SRC-0038 docs/zkir-v3-divergence-review.md §6; source fact; not reproduced; high; S4). | Off-circuit preprocessing accepts programs using `CondSelect` on `Bytes32` or `JubjubScalar`, but circuit synthesis and key generation fail (CLM-0659; SRC-0038 docs/zkir-v3-divergence-review.md §6; source fact; not reproduced; high; S4). | Align off-circuit typing with in-circuit capabilities or implement multiplexer constraints for `Bytes32` and `JubjubScalar` (CLM-0659; SRC-0038 docs/zkir-v3-divergence-review.md §6; source fact; not reproduced; high; S4). | Partially fixed in `midnight-ledger` PR #656 (`Bytes32` rejected off-circuit); `JubjubScalar` status open (CLM-0659; SRC-0038 docs/zkir-v3-divergence-review.md §6; source fact; not reproduced; high; S4). |
| Finding 7: `ConstrainEq` Type Asymmetry | Permitted on `JubjubScalar` off-circuit (CLM-0660; SRC-0038 docs/zkir-v3-divergence-review.md §7; source fact; not reproduced; high; S4). | In-circuit synthesis lacks an arm for `JubjubScalar`, returning `Error::Synthesis` (CLM-0660; SRC-0038 docs/zkir-v3-divergence-review.md §7; source fact; not reproduced; high; S4). | Programs asserting equality of Jubjub scalars run off-circuit but cannot be synthesized into circuits (CLM-0660; SRC-0038 docs/zkir-v3-divergence-review.md §7; source fact; not reproduced; high; S4). | Either add in-circuit scalar equality constraints or reject `JubjubScalar` in off-circuit type checking (CLM-0660; SRC-0038 docs/zkir-v3-divergence-review.md §7; source fact; not reproduced; high; S4). | Partially fixed in PR #656 (`JubjubScalar` rejected off-circuit); in-circuit scalar equality constraint open (CLM-0660; SRC-0038 docs/zkir-v3-divergence-review.md §7; source fact; not reproduced; high; S4). |
| Finding 8: `Bytes32FromLowHigh` Operand Types | Allows `low` and `high` to be foreign base or scalar field elements as long as values are within bounds (CLM-0661; SRC-0038 docs/zkir-v3-divergence-review.md §8; source fact; not reproduced; high; S4). | In-circuit synthesis requires `high` to be strictly of type `Native`, while permitting `low` to be foreign (CLM-0661; SRC-0038 docs/zkir-v3-divergence-review.md §8; source fact; not reproduced; high; S4). | Passing a foreign field element as `high` succeeds off-circuit but aborts in-circuit with `Error::Synthesis` (CLM-0661; SRC-0038 docs/zkir-v3-divergence-review.md §8; source fact; not reproduced; high; S4). | Unify type signatures: restrict both `low` and `high` to `Native` (matching documentation) or expand in-circuit synthesis (CLM-0661; SRC-0038 docs/zkir-v3-divergence-review.md §8; source fact; not reproduced; high; S4). | Fixed in `midnight-ledger` PR #656 and PR #668 by restricting both operands to `Native` (CLM-0661; SRC-0038 docs/zkir-v3-divergence-review.md §8; source fact; not reproduced; high; S4). |
| Finding 9: Stale Rustdoc Comments | Documentation comments in `ir.rs` stated outdated limb counts for `Encode` and omitted foreign types from `TestEq` (CLM-0662; SRC-0038 docs/zkir-v3-divergence-review.md §9; source fact; not reproduced; high; S4). | Implementation correctly dispatches over all foreign types (CLM-0662; SRC-0038 docs/zkir-v3-divergence-review.md §9; source fact; not reproduced; high; S4). | Confusion for auditors and developers relying on rustdoc rather than source code (CLM-0662; SRC-0038 docs/zkir-v3-divergence-review.md §9; source fact; not reproduced; high; S4). | Update doc comments in `ir.rs` to reflect actual type coverage and output counts (CLM-0662; SRC-0038 docs/zkir-v3-divergence-review.md §9; source fact; not reproduced; high; S4). | `Encode` comments corrected in PR #656; `TestEq` comment update open (CLM-0662; SRC-0038 docs/zkir-v3-divergence-review.md §9; source fact; not reproduced; high; S4). |
| Finding 10: `ReconstituteField` Bit-Count Panic | Permits any `bits: u32` as long as `modulus < 2^bits` and no field overflow occurs (CLM-0663; SRC-0038 docs/zkir-v3-divergence-review.md §10; source fact; not reproduced; high; S4). | In-circuit keygen evaluates `FR_BITS as u32 - *bits` (`255 - bits`), panicking on unsigned integer underflow when `bits >= 256` (CLM-0663; SRC-0038 docs/zkir-v3-divergence-review.md §10; source fact; not reproduced; high; S4). | Prover or compiler crashes with an unhandled panic during proving key generation on valid off-circuit inputs with `bits >= 256` (CLM-0663; SRC-0038 docs/zkir-v3-divergence-review.md §10; source fact; not reproduced; high; S4). | Check `if *bits > FR_BITS { return Err(Error::Synthesis(...)); }` before subtraction (CLM-0663; SRC-0038 docs/zkir-v3-divergence-review.md §10; source fact; not reproduced; high; S4). | Fixed in `midnight-ledger` PR #656 (CLM-0663; SRC-0038 docs/zkir-v3-divergence-review.md §10; source fact; not reproduced; high; S4). |
| Finding 11: `Curve25519` Subgroup Panic | Witness generation evaluates coordinates; `midnight-circuits` 7.2.2 `into_subgroup()` panics with `.expect(...)` on an on-curve point with non-zero 8-torsion (CLM-0664; SRC-0038 docs/zkir-v3-divergence-review.md §11; source fact; not reproduced; high; S4). | Circuit constraint system enforces prime-order subgroup membership via cofactor clearing without panicking (CLM-0664; SRC-0038 docs/zkir-v3-divergence-review.md §11; source fact; not reproduced; high; S4). | Unhandled panic during witness generation when encountering an on-curve point outside the prime-order subgroup; `FromCoordinates` hint panics in-circuit (CLM-0664; SRC-0038 docs/zkir-v3-divergence-review.md §11; source fact; not reproduced; high; S4). | Return `Result` from `into_subgroup()` instead of panicking; catch invalid subgroup points gracefully in witness generation (CLM-0664; SRC-0038 docs/zkir-v3-divergence-review.md §11; source fact; not reproduced; high; S4). | Decoder panic fixed in circuits 7.2.4; `FromCoordinates` in-circuit hint panic remains open (CLM-0664; SRC-0038 docs/zkir-v3-divergence-review.md §11; source fact; not reproduced; high; S4). |
| Finding 12: `EcMulGenerator` Parity Gap | Supports `JubjubScalar` and `Secp256k1Scalar` (CLM-0665; SRC-0038 docs/zkir-v3-divergence-review.md §12; source fact; not reproduced; high; S4). | `EcMulGenerator` has no dispatch arms for `Secp256r1` or `Curve25519` (CLM-0665; SRC-0038 docs/zkir-v3-divergence-review.md §12; source fact; not reproduced; high; S4). | Incomplete API coverage across supported elliptic curves; not a divergence between engines since both reject the missing curves (CLM-0665; SRC-0038 docs/zkir-v3-divergence-review.md §12; source fact; not reproduced; high; S4). | Either implement generator multiplication for all curves or document deliberate omission (CLM-0665; SRC-0038 docs/zkir-v3-divergence-review.md §12; source fact; not reproduced; high; S4). | Open design omission; not a behavioral divergence (CLM-0665; SRC-0038 docs/zkir-v3-divergence-review.md §12; source fact; not reproduced; high; S4). |
| Finding 13: Chip Gating Misses `FromBytes32` | Evaluates `FromBytes32` without chip allocation errors (CLM-0666; SRC-0038 docs/zkir-v3-divergence-review.md §13; source fact; not reproduced; high; S4). | `used_chips` scanner neglects `FromBytes32`; circuits using foreign fields only via `FromBytes32` fail to allocate chips, panicking at keygen (CLM-0666; SRC-0038 docs/zkir-v3-divergence-review.md §13; source fact; not reproduced; high; S4). | Circuits converting byte strings to foreign elements crash during key generation (CLM-0666; SRC-0038 docs/zkir-v3-divergence-review.md §13; source fact; not reproduced; high; S4). | Include `Instruction::FromBytes32` in the `used_chips` scanner in `ir_vm.rs` (CLM-0666; SRC-0038 docs/zkir-v3-divergence-review.md §13; source fact; not reproduced; high; S4). | Fixed in `midnight-ledger` PR #656 and PR #667 (CLM-0666; SRC-0038 docs/zkir-v3-divergence-review.md §13; source fact; not reproduced; high; S4). |

## Non-divergent features confirmed consistent

The review investigated several additional mechanisms suspected of harboring divergences and verified that they are enforced identically across both engines (CLM-0667; SRC-0038 docs/zkir-v3-divergence-review.md §Also checked, found enforced; source fact; not reproduced; high; S4):
1. `DivModPowerOfTwo` Canonicity: in earlier ZKIR versions, non-canonical quotient/remainder pairs could satisfy constraints; in v3, `enforce_canonical` is invoked in-circuit, matching off-circuit integer division (CLM-0667; SRC-0038 docs/zkir-v3-divergence-review.md; source fact; not reproduced; high; S4).
2. `Inv` Division-by-Zero: off-circuit execution returns an explicit error on zero; in-circuit synthesis asserts non-zero via inverse constraint $a \cdot a^{-1} = 1$, rendering the circuit unsatisfiable on zero (CLM-0667; SRC-0038 docs/zkir-v3-divergence-review.md; source fact; not reproduced; high; S4).
3. `ConstrainBits` Range Bounds: checked against bit-length constraints; both engines prevent modular overflow (CLM-0667; SRC-0038 docs/zkir-v3-divergence-review.md; source fact; not reproduced; high; S4).
4. FAB Aligner Formatting: alignments used in `PersistentHash` and `Keccak256` parse field representations consistently (CLM-0667; SRC-0038 docs/zkir-v3-divergence-review.md; source fact; not reproduced; high; S4).

## Consequences for a K semantics

The divergences and design asymmetries documented above have immediate structural implications for the formal specification of ZKIR in the K Framework (CLM-0668; SRC-0038 docs/zkir-v3-divergence-review.md; inference; not reproduced; high; S4).
This section is cross-linked to [../zkir-k-semantics-plan.md](../zkir-k-semantics-plan.md), [../k-framework/k-user-manual.md](../k-framework/k-user-manual.md), and [zkir-vm-semantics.md](zkir-vm-semantics.md).

### 1. Primary behaviour in K definition

Inference: an executable formal semantics in K must treat the off-circuit operational semantics (`preprocess`) as its primary transition system for program execution, while explicitly formalizing the constraint generation relation (`synth`) as a verification condition (CLM-0669; SRC-0038 docs/zkir-v3-divergence-review.md; inference; not reproduced; high; S4).
Operational execution represents the actual computation performed by provers, testing tools, and contract simulators (CLM-0669; SRC-0038 docs/zkir-v3-spec.md §6; inference; not reproduced; high; S4).
However, treating off-circuit preprocessing alone as the semantics would obscure zero-knowledge soundness, because the blockchain verifier checks only constraint satisfiability (CLM-0669; SRC-0038 docs/zkir-v3-spec.md §7; inference; not reproduced; high; S4).
Therefore, the K definition must formalize both behaviors and specify the exact predicate under which they coincide (CLM-0669; SRC-0038 src/zkir-v3/CircuitProof.agda; inference; not reproduced; high; S4).

### 2. Dual representation in K configuration

Inference: to capture both witness evaluation and constraint generation without state collision, the K configuration should be partitioned into dedicated operational and constraint cells (CLM-0670; SRC-0038 src/zkir-v3/Circuit.agda; inference; not reproduced; high; S4):

```k
configuration
  <zkir>
    <k> $PGM:Instructions </k>
    <mode> "PREPROCESS" </mode> // Execution mode: PREPROCESS, SYNTH, or COMBINED

    // Operational witness state (mirrors Agda State)
    <witnessState>
      <memory> .Map </memory>            // Map(Id, IrValue)
      <publicInputs> .List </publicInputs> // PIS accumulator
      <piSkips> .List </piSkips>          // Skip markers
      <cursorIn> 0 </cursorIn>            // Expected public transcript cursor
      <cursorOut> 0 </cursorOut>          // Consumed public transcript cursor
      <cursorPriv> 0 </cursorPriv>        // Private transcript cursor
      <outputs> .List </outputs>          // Return values accumulator
    </witnessState>

    // Constraint synthesis state (mirrors Agda Circuit)
    <constraintState>
      <constraints> .List </constraints>  // Accumulated Constraint AST
      <piLength> 0 </piLength>            // Structural length of public inputs
      <hasComm> false </hasComm>          // Communications commitment flag
      <activeChips> .Set </activeChips>   // Gated chips (used_chips set)
    </constraintState>

    // Static metadata and obligations
    <metadata>
      <irVersion> "V0" </irVersion>
      <producerObligations>
        <isSingleAssignment> true </isSingleAssignment>
        <isWellTyped> true </isWellTyped>
        <isWellFormedBounds> true </isWellFormedBounds>
      </producerObligations>
    </metadata>
  </zkir>
```

Inference: operational rules rewrite `<memory>` and transcript cursors, while synthesis rules rewrite `<constraints>` and `<activeChips>` (CLM-0671; SRC-0038 src/zkir-v3/Circuit.agda; inference; not reproduced; high; S4).
In a `COMBINED` verification mode, each instruction simultaneously checks that the operational value satisfies the constraint emitted into `<constraints>`, directly realizing the circuit faithfulness equivalence theorem P5 (`circuit-faithful`) inside the rewrite engine (CLM-0671; SRC-0038 src/zkir-v3/CircuitProof.agda; inference; not reproduced; high; S4).

### 3. Edge cases for the K semantics test suite

Inference: the thirteen findings from the divergence review dictate mandatory negative and positive test cases that the K semantics test suite must cover (CLM-0672; SRC-0038 docs/zkir-v3-divergence-review.md; inference; not reproduced; high; S4):

1. `Assert` Condition Divergence (Finding 2):
The test suite must verify that an operand value of $cond = 2$ fails in `PREPROCESS` mode (or halts with an undefined-behavior label), while in `SYNTH` mode it emits constraint `non-zero(cond)` (CLM-0673; SRC-0038 docs/zkir-v3-divergence-review.md §2; inference; not reproduced; high; S4).
The K semantics should verify that obligation O4 rejects non-boolean assert conditions prior to proof generation (CLM-0673; SRC-0038 docs/zkir-v3-spec.md §8.2; inference; not reproduced; high; S4).

2. `ReconstituteField` Modulo Wrapping and Underflow Panic (Findings 1 and 10):
The test suite must supply operands where $divisor \cdot 2^{bits} + modulus \ge |Fr|$:
- In pure integer semantics without range bounds, verify that the integer recomposition equation does not reduce modulo $|Fr|$ (CLM-0674; SRC-0038 docs/zkir-v3-divergence-review.md §1; inference; not reproduced; high; S4).
- Verify that inputs with $bits \ge 256$ (e.g. $bits = 256$ and $bits = 257$) are rejected by static well-formedness checks (`producer-WF2`) rather than causing arithmetic underflow panics in $255 - bits$ (CLM-0674; SRC-0038 docs/zkir-v3-divergence-review.md §10; inference; not reproduced; high; S4).

3. `LessThan` Bit Bounds (Finding 4):
The test suite must include odd bit-width comparisons (e.g. $bits = 5, 7, 9$) with operands in the interval $[2^n, 2^{n+1}-1]$ (CLM-0675; SRC-0038 docs/zkir-v3-divergence-review.md §4; inference; not reproduced; high; S4).
The test suite should verify that inputs exceeding the nominal bit width but falling within the padded even bound are caught by the specification's precondition (CLM-0675; SRC-0038 docs/zkir-v3-divergence-review.md §4; inference; not reproduced; high; S4).

4. `CondSelect` and `ConstrainEq` Type Rejection (Findings 6 and 7):
The test suite must attempt `CondSelect` on `Bytes32` and `JubjubScalar`, and `ConstrainEq` on `JubjubScalar` (CLM-0676; SRC-0038 docs/zkir-v3-divergence-review.md §6; inference; not reproduced; high; S4).
For the baseline specification (`ledger-9` commit `92e8bdd3`), static typing rules in K must reject these combinations before synthesis to prevent compiler aborts (CLM-0676; SRC-0038 docs/zkir-v3-divergence-review.md §7; inference; not reproduced; high; S4).

5. `Bytes32FromLowHigh` Operand Types (Finding 8):
The test suite must supply non-native field elements for `low` and `high` (CLM-0677; SRC-0038 docs/zkir-v3-divergence-review.md §8; inference; not reproduced; high; S4).
The semantics must enforce that both operands are strictly of type `Native`, ensuring that non-native `high` operands are rejected during static type validation (CLM-0677; SRC-0038 docs/zkir-v3-divergence-review.md §8; inference; not reproduced; high; S4).

6. Non-Canonical Foreign Limb Decoding (Finding 5):
The test suite must supply limb encodings representing integers $\ge p_{foreign}$ in public input preimages and transcript streams (CLM-0678; SRC-0038 docs/zkir-v3-divergence-review.md §5; inference; not reproduced; high; S4).
The K decoder rules must enforce canonical rejection matching the Group C `secp*-sound` axioms in `Assumptions.agda`, documenting the divergence from `midnight-circuits` 7.2.2's silent modular reduction (CLM-0678; SRC-0038 src/zkir-v3/Assumptions.agda; inference; not reproduced; high; S4).

7. Curve25519 Subgroup Torsion Points (Finding 11):
The test suite must provide coordinate operands that lie on the Curve25519 curve but have a non-zero 8-torsion component (CLM-0679; SRC-0038 docs/zkir-v3-divergence-review.md §11; inference; not reproduced; high; S4).
The K operational semantics must gracefully reject the point with an error result rather than an unhandled panic, verifying cofactor clearing constraints in-circuit (CLM-0679; SRC-0038 docs/zkir-v3-divergence-review.md §11; inference; not reproduced; high; S4).

8. `PublicInput` / `PrivateInput` Inactive Guard Slack (Finding 3):
The test suite must execute a program where `PublicInput` or `PrivateInput` has a false guard, supplying arbitrary non-default witness values in the circuit witness cell (CLM-0680; SRC-0038 docs/zkir-v3-divergence-review.md §3; inference; not reproduced; high; S4).
The K semantics must verify that in-circuit constraint evaluation succeeds regardless of the inactive cell's witness value, validating the prover slack property (CLM-0680; SRC-0038 docs/zkir-v3-divergence-review.md §3; inference; not reproduced; high; S4).

9. Chip Gating via `FromBytes32` (Finding 13):
The test suite must construct a minimal ZKIR program that utilizes foreign field types exclusively via `FromBytes32`, with no other foreign arithmetic instructions (CLM-0681; SRC-0038 docs/zkir-v3-divergence-review.md §13; inference; not reproduced; high; S4).
The K semantics synthesis pass must verify that the relevant foreign field chip is recorded in `<activeChips>` to prevent key generation faults (CLM-0681; SRC-0038 docs/zkir-v3-divergence-review.md §13; inference; not reproduced; high; S4).
