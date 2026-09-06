---
id: zkir.vm-semantics
type: semantics
title: ZKIR Virtual Machine Semantics
status: active
updated_at: 2026-09-03T14:39:31Z
sources:
  - SRC-0006
  - SRC-0038
---

# ZKIR Virtual Machine Semantics

## Dual-Run Execution Model

The execution architecture of the Zero-Knowledge Intermediate Representation (ZKIR) version 3 is founded upon a strict dual-run operational model (CLM-0450; SRC-0038 docs/zkir-v3-spec.md:L400-430; source fact; unperformed; high; S5). Rather than functioning as a single, unified virtual machine interpreter, ZKIR defines two distinct semantics over the same instruction sequence (CLM-0450; SRC-0038 docs/zkir-v3-spec.md:L435-460; source fact; unperformed; high; S5):
1. **Off-Circuit Witness Generation (`preprocess`):** A deterministic, concrete operational reduction that executes in normal program memory prior to proof generation. It consumes the proof preimage (including private witnesses, public transcripts, and commitment randomness), populates named registers with concrete values, verifies dynamic assertions, records public input skip indices, and computes the public input statement vector $\pi$ (CLM-0450; SRC-0006 zkir/src/ir_vm.rs:L207-250; repository observation; inspection; high; S5).
2. **In-Circuit Constraint Synthesis (`synthesize`):** An algebraic circuit compiler that allocates advice and fixed cells within a Halo2 PLONKish constraint matrix. It binds circuit witness variables to the concrete values computed during preprocess, synthesizes custom polynomial gates, enforces copy permutation constraints across cells, and attaches lookup table arguments for range checks and non-native arithmetic (CLM-0450; SRC-0006 zkir/src/ir_vm.rs:L830-865; repository observation; inspection; high; S5).

```
+----------------------------------------------------------------------------------------------------+
|                                    ZKIR Dual-Run Architecture                                      |
+----------------------------------------------------------------------------------------------------+
|                                                                                                    |
|    ProofPreimage (Inputs, Transcripts, Randomness)                                                 |
|                         |                                                                          |
|                         v                                                                          |
|    +------------------------------------------------------------------------------------------+    |
|    | Off-Circuit Reduction (IrSource::preprocess)                                             |    |
|    |                                                                                          |    |
|    | Inputs:  ProofPreimage, IrSource                                                         |    |
|    | State:   Sigma = <memory, pi, kappa, iota^+_i, iota^+_o, iota^-, omega>                  |    |
|    | Outputs: Preprocessed = { memory, pis, pi_skips, binding_input, comm_comm }              |    |
|    +------------------------------------------------------------------------------------------+    |
|                         |                                                                          |
|                         v Witness Values passed to Relation::circuit                               |
|    +------------------------------------------------------------------------------------------+    |
|    | In-Circuit Synthesis (Relation::circuit)                                                 |    |
|    |                                                                                          |    |
|    | Allocates: Advice columns, Instance columns, Fixed columns, Selectors                     |    |
|    | Enforces:  PLONKish Custom Gates (Degree <= 9), Permutation Equalities, Lookup Tables    |    |
|    | Exports:   Public Input Copy Constraints (std.constrain_as_public_input)                 |    |
|    +------------------------------------------------------------------------------------------+    |
|                         |                                                                          |
|                         v Prover generates KZG polynomial commitments                             |
|    +------------------------------------------------------------------------------------------+    |
|    | Halo2 Verifier Key Checks Proof against Statement Vector pi                              |    |
|    +------------------------------------------------------------------------------------------+    |
|                                                                                                    |
+----------------------------------------------------------------------------------------------------+
```

The fundamental correctness contract of the ZKIR virtual machine requires that for any valid preimage and circuit, the witness values computed during preprocess satisfy all constraints generated during synthesis without inconsistency or underconstrained degrees of freedom (CLM-0451; SRC-0038 docs/zkir-v3-spec.md:L500-525; source fact; unperformed; high; S5).

## Preprocess Formal Machine State

The authoritative operational state of the preprocess virtual machine is formalized as a seven-tuple (CLM-0452; SRC-0038 docs/zkir-v3-spec.md:L465-495; source fact; unperformed; high; S5):

$$\Sigma = \langle \text{memory}, \pi, \kappa, \iota^+_i, \iota^+_o, \iota^-, \omega \rangle$$

The constituent components of the state tuple are defined as follows (CLM-0452; SRC-0006 zkir/src/ir_vm.rs:L214-255; repository observation; inspection; high; S5):

1. **Register Memory ($\text{memory}$):**
   $$\text{memory}: \text{Identifier} \to \text{IrValue}$$
   A finite mapping from variable identifiers (strings prefixed with `%`) to typed runtime values. In ZKIR, register memory is strictly immutable; every variable must be assigned exactly once before being read as an operand in subsequent instructions (CLM-0452; SRC-0038 docs/zkir-v3-spec.md:L120-135; source fact; unperformed; high; S5).

2. **Public Input Statement Vector ($\pi$):**
   $$\pi \in \mathbb{F}_r^*$$
   A growing vector of scalar field elements in $\mathbb{F}_r$ that constitutes the public statement checked by the verifier (CLM-0453; SRC-0006 zkir/src/ir_vm.rs:L238-246; repository observation; inspection; high; S5).
   - $\pi[0]$ is initialized to `preimage.binding_input`.
   - If `do_communications_commitment` is enabled, $\pi[1]$ is initialized to `preimage.communications_commitment.0`.
   - Subsequent elements are appended dynamically by active `impact` instructions.

3. **Public Input Skip Vector ($\kappa$):**
   $$\kappa \in (\mathbb{N} \cup \{\bot\})^*$$
   A sequence of skip markers that records whether each `impact` instruction was actively evaluated or bypassed due to a disabled guard (CLM-0453; SRC-0006 zkir/src/ir_vm.rs:L569-586; repository observation; inspection; high; S5).
   - When an `impact` instruction executes with `guard == 1`, $\kappa$ appends $\bot$ (`None`).
   - When an `impact` instruction executes with `guard == 0`, $\kappa$ appends $k$ (`Some(k)`), where $k$ is the number of inputs in the bypassed instruction.

4. **Public Transcript Input Cursor ($\iota^+_i$):**
   $$\iota^+_i \in \mathbb{N}$$
   A non-negative integer cursor indexing into `preimage.public_transcript_inputs`. It tracks the number of field elements consumed by active `impact` statements.

5. **Public Transcript Output Cursor ($\iota^+_o$):**
   $$\iota^+_o \in \mathbb{N}$$
   A non-negative integer cursor indexing into `preimage.public_transcript_outputs`. It tracks elements read by `public_input` instructions.

6. **Private Transcript Cursor ($\iota^-$):**
   $$\iota^- \in \mathbb{N}$$
   A non-negative integer cursor indexing into `preimage.private_transcript`. It tracks private witness elements read by `private_input` instructions.

7. **Circuit Output Vector ($\omega$):**
   $$\omega \in \text{IrValue}^*$$
   A list of typed return values collected by the `output` terminator instruction, type-checked against the circuit's declared `outputs` signature and used to verify the communications commitment (CLM-0454; SRC-0006 zkir/src/ir_vm.rs:L749-768; repository observation; inspection; high; S5).

## Preprocess Operational Reduction Rules

The execution of a ZKIR circuit proceeds instruction-by-instruction from the initial state $\Sigma_0$ to the terminal state $\Sigma_n$. Below are the precise operational transition rules for each instruction category (CLM-0455; SRC-0006 zkir/src/ir_vm.rs:L325-770; repository observation; inspection; high; S5).

### 1. Circuit Initialization

Before processing instructions, the machine populates inputs declared in `circuit.inputs`:
- For each declared input $(id, \text{val\_t})$, the machine reads $w = \text{val\_t.encoded\_len()}$ elements from `preimage.inputs` at cursor position `idx`.
- It evaluates $v = \text{decode\_offcircuit}(\text{preimage.inputs}[\text{idx} .. \text{idx}+w], \text{val\_t})$.
- It updates $\text{memory} \leftarrow \text{memory} \cup \{id \mapsto v\}$ and $\text{idx} \leftarrow \text{idx} + w$.
- It verifies that the entire `preimage.inputs` slice is consumed exactly ($\text{idx} = \text{preimage.inputs.len()}$).
- It initializes $\pi$ with `preimage.binding_input` (and the communications commitment digest if enabled).

### 2. Guarded Transcript Input: `public_input`

Given instruction:
```json
{ "op": "public_input", "type": "T", "output": "%id", "guard": <guard> }
```

The reduction rule evaluates:
$$\text{guard\_val} = \begin{cases} \text{true} & \text{if guard is null} \\ \text{resolve\_operand\_bool}(\text{memory}, \text{guard}) & \text{otherwise} \end{cases}$$

1. If $\text{guard\_val} = \text{false}$:
   $$\text{memory}' = \text{memory} \cup \{\%id \mapsto \text{IrValue::default}(T)\}$$
   Cursors $\iota^+_o, \iota^+_i, \iota^-$ remain unchanged (CLM-0456; SRC-0006 zkir/src/ir_vm.rs:L409-412; repository observation; inspection; high; S5).
2. If $\text{guard\_val} = \text{true}$:
   Let $w = T.\text{encoded\_len()}$. The slice is read from the public transcript output stream:
   $$\text{slice} = \text{preimage.public\_transcript\_outputs}[\iota^+_o .. \iota^+_o + w]$$
   $$v = \text{decode\_offcircuit}(\text{slice}, T)$$
   $$\text{memory}' = \text{memory} \cup \{\%id \mapsto v\}, \quad \iota^{+'}_o = \iota^+_o + w$$

### 3. Guarded Private Witness Input: `private_input`

Given instruction:
```json
{ "op": "private_input", "type": "T", "output": "%id", "guard": <guard> }
```

Evaluates identically to `public_input`, except that when $\text{guard\_val} = \text{true}$, data is consumed from `preimage.private_transcript` and cursor $\iota^-$ is advanced by $w$ (CLM-0456; SRC-0006 zkir/src/ir_vm.rs:L423-442; repository observation; inspection; high; S5).

### 4. Guarded Statement Commitment: `impact`

Given instruction:
```json
{ "op": "impact", "guard": <guard>, "inputs": [op_1, \dots, op_k] }
```

Let $\text{guard\_val} = \text{resolve\_operand\_bool}(\text{memory}, \text{guard})$.
1. If $\text{guard\_val} = \text{false}$:
   - The machine appends $k$ zero elements to $\pi$:
     $$\pi' = \pi \mathbin{\Vert} [0, \dots, 0]_k$$
   - It records a skip marker in $\kappa$:
     $$\kappa' = \kappa \mathbin{\Vert} [\text{Some}(k)]$$
   - The transcript cursor $\iota^+_i$ is **not advanced** (CLM-0457; SRC-0006 zkir/src/ir_vm.rs:L571-578; repository observation; inspection; high; S5).
2. If $\text{guard\_val} = \text{true}$:
   - For each input $op_j$, resolve $x_j = \text{resolve\_operand}(\text{memory}, op_j) \in \mathbb{F}_r$.
   - Append resolved elements to $\pi$:
     $$\pi' = \pi \mathbin{\Vert} [x_1, \dots, x_k]$$
   - Record active execution in $\kappa$:
     $$\kappa' = \kappa \mathbin{\Vert} [\text{None}]$$
   - Advance transcript cursor $\iota^+_i \leftarrow \iota^+_i + k$.
   - **Transcript Consistency Assertion:** For each $j \in \{0, \dots, k-1\}$, the machine asserts that the computed public input matches the preimage expectation:
     $$\pi[\text{len}(\pi) - k + j] == \text{preimage.public\_transcript\_inputs}[\iota^+_i - k + j]$$
     If a mismatch occurs, execution aborts with:
     ```text
     "Public transcript input mismatch for input <idx>; expected: <exp>, computed: <act>"
     ```

### 5. Conditional Selection: `cond_select`

Given instruction:
```json
{ "op": "cond_select", "bit": <bit>, "a": <op_a>, "b": <op_b>, "output": "%id" }
```

- Verifies $b_{\text{val}} = \text{resolve\_operand\_bool}(\text{memory}, \text{bit}) \in \{0, 1\}$.
- Resolves $v_a = \text{resolve\_operand}(\text{memory}, op_a)$ and $v_b = \text{resolve\_operand}(\text{memory}, op_b)$.
- Verifies $v_a.\text{get\_type}() == v_b.\text{get\_type}()$.
- If $b_{\text{val}} = 1$, assigns $\%id \mapsto v_a$; if $b_{\text{val}} = 0$, assigns $\%id \mapsto v_b$ (CLM-0458; SRC-0006 zkir/src/ir_instructions/select.rs:L45-54; repository observation; inspection; high; S5).

### 6. Assertions and Dynamic Checking

#### `assert`
- Resolves $b = \text{resolve\_operand\_bool}(\text{memory}, \text{cond})$.
- If $b \ne 1$, preprocess halts with fatal error `"Failed direct assertion"` (CLM-0459; SRC-0006 zkir/src/ir_vm.rs:L393-397; repository observation; inspection; high; S5).

#### `constrain_eq`
- Resolves $v_a$ and $v_b$. Asserts structural equality $v_a = v_b$. If unequal, preprocess aborts (CLM-0459; SRC-0006 zkir/src/ir_instructions/constrain_eq.rs:L45-80; repository observation; inspection; high; S5).

#### `constrain_to_boolean`
- Resolves $x = \text{resolve\_operand}(\text{memory}, \text{val}) \in \mathbb{F}_r$. Asserts $x \in \{0, 1\}$.

#### `constrain_bits`
- Resolves $x \in \mathbb{F}_r$. Asserts that the canonical integer value of $x$ satisfies $x < 2^{\text{bits}}$.

### 7. Terminal Verification and Communications Commitment

Upon executing the `output` terminator instruction:
1. The machine verifies that all declared outputs match runtime types and appends them to $\omega$ (CLM-0460; SRC-0006 zkir/src/ir_vm.rs:L749-768; repository observation; inspection; high; S5).
2. **Transcript Exhaustion Check:** The machine asserts that all transcripts were consumed completely:
   $$\iota^+_i = \text{preimage.public\_transcript\_inputs.len()}$$
   $$\iota^+_o = \text{preimage.public\_transcript\_outputs.len()}$$
   $$\iota^- = \text{preimage.private\_transcript.len()}$$
   If any transcript contains unconsumed trailing elements, preprocess aborts with `"Transcripts not fully consumed"` (CLM-0460; SRC-0006 zkir/src/ir_vm.rs:L772-785; repository observation; inspection; high; S5).
3. **Communications Commitment Verification:** If `do_communications_commitment` is enabled:
   - Constructs input payload: $I = \text{preimage.inputs} \mathbin{\Vert} \text{encode\_offcircuit}(\omega)$.
   - Recomputes commitment digest using deterministic blinding factor $\rho$:
     $$\text{expected\_comm} = \text{transient\_commit}(I, \rho)$$
   - Asserts that $\text{expected\_comm} == \text{preimage.communications\_commitment.0}$ (CLM-0460; SRC-0006 zkir/src/ir_vm.rs:L786-805; repository observation; inspection; high; S5).

## In-Circuit Constraint Synthesis & Halo2 Arithmetization

Circuit synthesis compiles the circuit into a PLONKish constraint system via the `Relation` trait implementation for `IrSource` (CLM-0461; SRC-0006 zkir/src/ir_vm.rs:L818-840; repository observation; inspection; high; S5).

```rust
impl Relation for IrSource {
    type Instance = Vec<outer::Scalar>;
    type Witness = Preprocessed;
    type Error = midnight_proofs::plonk::Error;

    fn circuit(
        &self,
        std: &ZkStdLib,
        layouter: &mut impl Layouter<outer::Scalar>,
        _instance: Value<Self::Instance>,
        witness: Value<Self::Witness>,
    ) -> Result<(), Error>;
}
```

### Layout and Cell Assignment

During synthesis:
1. Circuit inputs declared in `circuit.inputs` are assigned to advice columns via `assign_incircuit(std, layouter, &id.val_t, &[value])` (CLM-0461; SRC-0006 zkir/src/ir_vm.rs:L853-857; repository observation; inspection; high; S5).
2. The initial public inputs $\pi[0]$ (`binding_input`) and $\pi[1]$ (`comm_comm`) are assigned and pushed to the public input column vector (CLM-0461; SRC-0006 zkir/src/ir_vm.rs:L945-959; repository observation; inspection; high; S5).
3. Named register allocations are stored in `memory: HashMap<Identifier, CircuitValue>`.
4. As each instruction synthesizes, cell assignments are verified against the witness values computed during preprocess:
   ```rust
   witness.as_ref().zip(cell.value()).error_if_known_and(|(preproc, v)| {
       if let Some(expected) = preproc.memory.get(&id) && *expected != *v {
           error!("Misalignment between prepare and synthesize runs. This is a bug.");
           return true;
       }
       false
   })?;
   ```
5. Public inputs generated by `impact` instructions are constrained to the public input instance column using `std.constrain_as_public_input(layouter, cell)` (CLM-0461; SRC-0006 zkir/src/ir_vm.rs:L1456-1459; repository observation; inspection; high; S5).

### Chip Gating via `used_chips()`

To minimize proving time and circuit degree, Midnight dynamically disables unused cryptographic chips in the constraint system (CLM-0462; SRC-0006 zkir/src/ir_vm.rs:L1461-1512; repository observation; inspection; high; S5). The method `used_chips(&self) -> ZkStdLibArch` inspects the circuit's declared input types and instruction sequence:
- `jubjub`: Activated if `JubjubPoint` or `JubjubScalar` appears in inputs or I/O instructions, or if `HashToCurve` is present.
- `poseidon`: Activated if `do_communications_commitment` is true, or if `TransientHash` or `HashToCurve` is present.
- `sha2_256`: Activated if `PersistentHash` is present.
- `sha2_512`: Activated if `Sha512` is present.
- `keccak_256`: Activated if `Keccak256` is present.
- `secp256k1`: Activated if `Secp256k1Point`, `Secp256k1Base`, or `Secp256k1Scalar` appears.
- `p256`: Activated if `Secp256r1Point`, `Secp256r1Base`, or `Secp256r1Scalar` appears.
- `curve25519`: Activated if `Curve25519Point`, `Curve25519Base`, or `Curve25519Scalar` appears.

## Divergence Analysis: In-Circuit vs Off-Circuit Semantics

The authoritative divergence review (`docs/zkir-v3-divergence-review.md` in SRC-0038) provides an exhaustive analysis of thirteen discrepancies between off-circuit preprocess evaluation and in-circuit synthesis (CLM-0463; SRC-0038 docs/zkir-v3-divergence-review.md:L1-896; source fact; unperformed; high; S5). Below is the disposition of the critical divergences.

### Finding 1: Chip Gating Soundness and Unconstrained Operations
When a specialized chip is disabled by `used_chips()`, any operations associated with that chip are omitted from the circuit (CLM-0463; SRC-0038 docs/zkir-v3-divergence-review.md:L45-95; source fact; unperformed; high; S5). In earlier crate versions, `used_chips()` inspected only `inputs`, `public_input`, and `private_input` to detect type usage. An instruction operating on non-native types constructed dynamically (such as via constants or coordinate synthesis) would fail to activate the chip, leaving the operation completely unconstrained in the proof system (CLM-0463; SRC-0038 docs/zkir-v3-divergence-review.md:L70-90; source fact; unperformed; high; S5).

### Finding 2: Canonicity Verification in Decode
In `decode_offcircuit`, raw field elements are converted to high-level types. If the decoding logic does not verify that the input limbs represent canonical field integers strictly less than the target field modulus, a malicious prover can provide non-canonical representations that satisfy circuit equality but violate unique representation (CLM-0464; SRC-0038 docs/zkir-v3-divergence-review.md:L120-170; source fact; unperformed; high; S5). This divergence was resolved in commit `2ffe2d1` by integrating PR #679, which asserts that re-encoding a decoded value yields the identical input vector.

### Finding 3: `CondSelect` and `ConstrainEq` Type Incompleteness
The authoritative specification defines `cond_select` and `constrain_eq` across all thirteen types (CLM-0465; SRC-0038 docs/zkir-v3-divergence-review.md:L180-230; source fact; unperformed; high; S5). However, early implementations of `select_incircuit` and `constrain_eq_incircuit` lacked arms for foreign curve points and base/scalar fields. In `2ffe2d1`, both instructions provide complete type support across all fifteen types, routing selection and equality through the respective chip standard library interfaces.

### Finding 4: In-Circuit vs Off-Circuit Equality (`test_eq`)
Off-circuit, `test_eq` performs structural equality across operands. In-circuit, `test_eq` allocates an auxiliary witness $w$ and enforces $(a - b) \cdot (1 - \text{out}) = 0$ and $(a - b) \cdot w = \text{out}$ (CLM-0466; SRC-0038 docs/zkir-v3-divergence-review.md:L240-290; source fact; unperformed; high; S5). This gadget is formally sound over prime fields. However, `test_eq` is currently unsupported in-circuit for foreign curve points and foreign base fields, restricting equality predicates on foreign curves to static `constrain_eq`.

### Finding 5: `assert` Semantics Divergence
Off-circuit, `assert` enforces that operand `cond` is a boolean value equal to 1 (CLM-0467; SRC-0038 docs/zkir-v3-divergence-review.md:L350-380; source fact; unperformed; high; S5). In-circuit, `assert` compiles to `std.assert_non_zero(layouter, cond)`. If `cond` is not explicitly constrained to $\{0, 1\}$ by a preceding `constrain_to_boolean`, an unconstrained field element $x \ge 2$ will satisfy the in-circuit constraint ($x \ne 0$) while causing off-circuit preprocess to abort.

### Finding 6: Range Constraints and Bit Bounds
In `constrain_bits`, preprocess checks $x < 2^{\text{bits}}$ over the integers. In-circuit, synthesis invokes `std.assigned_to_le_bits` (CLM-0468; SRC-0038 docs/zkir-v3-divergence-review.md:L390-430; source fact; unperformed; high; S5). If $\text{bits} \ge 255$, synthesis behaves inconsistently across standard library versions, requiring the compiler to reject bit bounds exceeding 248.

### Finding 7: Identity Extraction in Weierstrass Curves
Off-circuit, extracting coordinates from the Secp256k1 identity point throws a fatal exception. In-circuit, `into_coordinates` enforces $\text{is\_identity} = 0$ via `curve.assert_non_zero` (CLM-0469; SRC-0038 docs/zkir-v3-divergence-review.md:L450-480; source fact; unperformed; high; S5). This creates an unprovable circuit condition rather than a graceful branch, requiring high-level compilers to guard coordinate extraction with boolean branch checks.

## Formal Properties of ZKIR (P1 through P10)

The authoritative specification defines ten formal verification properties that any conformant ZKIR implementation must satisfy (CLM-0470; SRC-0038 docs/zkir-v3-spec.md:L820-910; source fact; unperformed; high; S5):

```
+----------------------------------------------------------------------------------------------------+
|                                    Formal Verification Properties                                  |
+----+------------------------------+----------------------------------------------------------------+
| ID | Property Name                | Formal Definition                                              |
+----+------------------------------+----------------------------------------------------------------+
| P1 | Preprocess Determinism       | Sigma_0 x IR -> Sigma_n is a deterministic partial function    |
| P2 | Synthesis Determinism        | Layout, columns, and selectors depend solely on IR structure   |
| P3 | Witness Completeness         | Valid preprocess execution implies satisfiable circuit witness |
| P4 | Constraint Soundness         | Satisfiable circuit witness implies preprocess equivalence     |
| P5 | Transcript Exhaustion        | Preprocess terminates iff all transcript cursors are consumed  |
| P6 | Public Input Correspondence  | Public inputs in pi match circuit copy constraints exactly     |
| P7 | Commitment Soundness         | comm_comm binds inputs and outputs under Poseidon commit       |
| P8 | Subgroup Confinement         | Every decoded point belongs to prime-order curve subgroup      |
| P9 | Non-Native Canonicity        | Multi-limb foreign field elements decode to canonical integers |
| P10| Guard Bypassing Invariance   | Guarded-off impact appends zero without advancing transcript   |
+----+------------------------------+----------------------------------------------------------------+
```

1. **P1 (Preprocess Determinism):** For any valid initial state $\Sigma_0$ and circuit $C$, preprocess execution either halts in a unique final state $\Sigma_n$ or aborts with a distinguished error.
2. **P2 (Synthesis Determinism):** Circuit layout, allocated columns, custom gates, and gate offsets are strictly deterministic functions of the circuit AST and do not depend on witness data.
3. **P3 (Witness Completeness):** If `preprocess(preimage, circuit)` succeeds producing witness $W$, then $W$ satisfies all PLONKish constraints generated by `Relation::circuit`.
4. **P4 (Constraint Soundness):** If an adversary produces a proof accepted by the verifier key, there exists a valid preimage whose preprocess execution matches the public input vector $\pi$.
5. **P5 (Transcript Exhaustion):** Preprocess terminates successfully if and only if all elements of `public_transcript_inputs`, `public_transcript_outputs`, and `private_transcript` are fully consumed.
6. **P6 (Public Input Correspondence):** Every element of $\pi$ computed during preprocess corresponds one-to-one with an assigned cell constrained via `constrain_as_public_input`.
7. **P7 (Commitment Soundness):** The communications commitment $\pi[1]$ cryptographically binds all declared inputs and returned outputs under the Poseidon commitment scheme.
8. **P8 (Subgroup Confinement):** Decoded points of types `JubjubPoint`, `Secp256k1Point`, `Secp256r1Point`, and `Curve25519Point` are strictly confined to their prime-order subgroups.
9. **P9 (Non-Native Canonicity):** Decomposed limb representations of non-native field elements decode to integers strictly smaller than the prime modulus of the target field.
10. **P10 (Guard Bypassing Invariance):** When an `impact` instruction executes with $\text{guard} = 0$, it pushes exactly $k$ zeros to $\pi$ and records $\text{Some}(k)$ in $\kappa$, leaving transcript cursor $\iota^+_i$ unchanged.

## Formal VM Semantics in the K Framework

The formalization of the ZKIR virtual machine in the K Framework models the preprocess machine state $\Sigma$ as a top-level configuration:

```k
module ZKIR-CONFIGURATION
  imports ZKIR-TYPES
  imports MAP
  imports LIST
  imports INT
  imports STRING

  configuration
    <zkir>
      <k> $PGM:Instructions </k>
      <registers> .Map </registers>
      <publicInputs> .List </publicInputs>
      <piSkips> .List </piSkips>
      <transcriptCursors>
        <publicInCursor> 0 </publicInCursor>
        <publicOutCursor> 0 </publicOutCursor>
        <privateCursor> 0 </privateCursor>
      </transcriptCursors>
      <transcripts>
        <publicIn> .List </publicIn>
        <publicOut> .List </publicOut>
        <private> .List </private>
      </transcripts>
      <commCommitment>
        <enabled> false </enabled>
        <commDigest> 0 </commDigest>
        <commRand> 0 </commRand>
      </commCommitment>
      <outputs> .List </outputs>
    </zkir>
endmodule
```

### Rewrite Rule Examples in K

#### Guarded Impact Execution in K
```k
module ZKIR-IMPACT-RULES
  imports ZKIR-CONFIGURATION

  // Active Impact: Guard == 1
  rule <k> impact(1, Inputs:List) => . ... </k>
       <publicInputs> ... .List => Inputs </publicInputs>
       <piSkips> ... .List => ListItem(bot) </piSkips>
       <publicInCursor> C => C +Int size(Inputs) </publicInCursor>
       <publicIn> Transcript:List </publicIn>
    requires range(Transcript, C, size(Inputs)) ==K Inputs

  // Bypassed Impact: Guard == 0
  rule <k> impact(0, Inputs:List) => . ... </k>
       <publicInputs> ... .List => zeros(size(Inputs)) </publicInputs>
       <piSkips> ... .List => ListItem(some(size(Inputs))) </piSkips>
       <publicInCursor> C </publicInCursor>
endmodule
```

#### Multiplicative Inverse in K
```k
module ZKIR-ARITH-RULES
  imports ZKIR-CONFIGURATION

  rule <k> inv(NativeVal(X), OutId:Id) => . ... </k>
       <registers> Regs => Regs[OutId <- NativeVal(invMod(X, BLS_R))] </registers>
    requires X =/=Int 0 andBool notBool (OutId in_keys(Regs))

  rule <k> inv(NativeVal(0), _) => ExecutionError("cannot invert zero of type Native") ... </k>
endmodule
```

This K semantics provides an executable, mathematically rigorous formal foundation for validating compiler correctness, generating test oracles, and proving cross-layer equivalence with Midnight contracts.

For further architectural context, refer to [ZKIR Instruction Set](zkir-instruction-set.md), [ZKIR Type System](zkir-type-system.md), and [Compact to ZKIR Pipeline](compact-to-zkir-pipeline.md).
