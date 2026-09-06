# Design rationale and limits

The definition makes two things executable together for the Zero-Knowledge Intermediate Representation (ZKIR): concrete witness computation and instruction-level constraint checking. Agreement between them is evidence about the tested witness, not a proof that the underlying circuit excludes every invalid witness.

The reasons behind decisions are those recorded in [the semantics plan](../../../wiki/zkir-k-semantics-plan.md). Descriptions of the implementation follow the files as they are, and statements about results cite the receipt under `evidence/` that records them. K filenames refer to `experiments/zkir-k/semantics/`, and tool filenames to `experiments/zkir-k/tools/`.

## Representation and execution choices

### Explicit field arithmetic

`zkir-field.k` represents field elements with arbitrary-precision K `Int`, using explicit modulus arguments in `fadd`, `fmul`, `finv` and related functions. For example:

```k
  rule fadd(A, B, P)  => (A +Int B) modInt P requires P >Int 1
```

The plan rejects fixed-width `MInt` arithmetic because its wraparound does not match the field modulus (CLM-0709). Euclidean `modInt` supplies canonical residues even after subtraction, and `finv` uses modular exponentiation through `^%Int` for nonzero residues. These are mathematical operations on integers; they do not simulate Rust's internal field representation, and their field interpretation assumes the configured prime moduli. Returning zero for invalid modulus arguments in the arithmetic helpers is a totality convention, not field mathematics.

`zkir-curves.k` likewise uses affine point operations, and `zkir-values.k`, `encodeValue` and `decodeValue`, make external encodings explicit. This separates mathematical values from their serialized representation (CLM-0710); it does not establish that every low-level circuit assignment has that representation. See [05-fields-curves-and-hashes.md](05-fields-curves-and-hashes.md).

### Abstract terms at the input boundary

There is no separate concrete ZKIR source-language grammar. `zkir_kast.py`, `program` and `load_program`, translate JavaScript Object Notation (JSON) artifacts into pyk's K abstract syntax tree, using the `Program` constructors of `zkir-syntax.k`; the plan chooses this boundary to avoid rewriting large JSON trees inside K (CLM-0712). K still declares the term syntax needed to construct and execute programs.

The preprocessor is therefore part of the trusted input path, and testing the transition rules does not prove its correspondence with Rust deserialization. Loading and static checking also remain distinct: `zkir-vm.k`, `checkedJob`, invokes `zkir-syntax.k`, `wf`, before execution; raw `job` does not. The `#put` rule overwrites registers as the crate does. Single assignment is a static restriction, needed when interpreting earlier gates against final memory (CLM-0711). See [10-well-formedness-and-static-checks.md](10-well-formedness-and-static-checks.md).

### Two semantics in one configuration

In `zkir-vm.k`, instruction emission appends to `<constraints>` before `#exec` computes the witness. After an error or panic, subsequent `#exec` terms disappear while instruction emission continues, and `<piIdx>` advances on the constraint side independently of witness progress. This realizes the plan's paired semantics and preserves disagreements after witness failure (CLM-0704, CLM-0705, CLM-0720).

The granularity is deliberate (CLM-0704, CLM-0734). `zkir-constraints.k`, `gate(add(A, B, O))`, checks the arithmetic relation and chip requirements against register values; it does not allocate the circuit's polynomial rows. `verdicts` evaluates emitted gates and records outcomes separately from the run status, so successful off-circuit execution can coexist with a violated relation or synthesis failure. See [08-constraints-and-verdicts.md](08-constraints-and-verdicts.md).

### Concrete execution and checking

The LLVM (Low Level Virtual Machine) backend is the concrete execution engine (CLM-0713). The compiled base and extension definitions record `llvm` in `semantics/zkir-kompiled/backend.txt` and `semantics/zkir-ext-kompiled/backend.txt`, and `zkir_run.py`, `Runner`, invokes them through pyk's `KRun`. Its completion check also inspects `<k>`, so residual computation is reported as runner-level `stuck` or `depth-exhausted`, rather than accepted solely because `<status>` remains `ok`.

The plan's oracle strategy has separate obligations: compare off-circuit observables with Rust, evaluate the instruction-level relations, and obtain an independent Agda comparison (CLM-0716). `diff_test.py`, `compare`, checks status and error class on failures; on success it compares register types and encodings, public inputs and skips. The oracle's `main` in `repos/_build/ledger-92e8bdd3/zkir-oracle/src/main.rs` calls `preprocess`, not `Relation::circuit`, so verdicts are compared with nothing outside the definition. The differential run checks only that no successful K run has a non-holding gate, and the divergence cases check the outcome of one selected gate each. Constraint checking supplies a separately encoded relation, not a second execution of the Rust circuit.

## Fidelity boundary

| Subject | Treatment | Reason and limit |
|---|---|---|
| Off-circuit instructions and transcripts | Concrete model in `zkir-vm.k`, `#exec`, and `zkir-ops.k` | Follows the pinned `preprocess` behavior, including implemented error and panic cases; correspondence is tested for selected observables and inputs. |
| Value encodings and hashes | Concrete `zkir-values.k`, `encodeValue` and `decodeValue`, and `zkir-hash.k`, `poseidonHash` | Allows byte-level and field-level comparisons (`unit_values.py`, 43 checks against independent Python encoders); functional output agreement does not verify hash gadget internals. |
| In-circuit dispatch and chip availability | Instruction-level abstraction in `zkir-constraints.k`, `eval` and `usedChips` | Models dispatch failures and chip requirements without constructing Halo2 circuits. |
| Auxiliary witnesses, copy wiring and assignment constraints | Abstracted away | Register values replace circuit cells; byte ranges, foreign limbs, on-curve and cofactor assignment constraints are not reproduced as gadget constraints. In particular, canonical K Jubjub scalars do not prove canonical circuit assignment. |
| Proving-system implementation | Out of scope | Not modelled: proof generation, verifier execution, and the prover-side panics of chip hints. Nothing in the definition establishes cryptographic soundness or the absence of underconstrained circuits. |
| Failure-state correspondence | Limited | The oracle exposes no partial memory on returned errors, so K's diagnostic cells are not a verified Rust failure snapshot. |
| Compiler correctness and ledger feasibility | Out of scope | Executing an artifact does not establish correct compilation, transaction validity, resource cost or deployment behavior. |

## Relation to arc-zkir

The shared target is the base surface at midnight-ledger `92e8bdd3` (CLM-0707). In `repos/input-output-hk/arc-zkir/src/zkir-v3/`, `Syntax.agda` defines `IrSource`, `Semantics.agda` defines `preprocess`, and `Circuit.agda` defines `synth`. Reading these establishes the structural correspondence: named typed inputs and instructions, a witness store and transcript state, and a separate constraint interpretation. It does not establish an equivalence theorem between K rewrites and Agda evaluation.

The purposes differ. K computes concrete primitives and exposes implementation divergences; Agda proves properties of a model parameterized by `Assumptions.agda`, `Assumptions`. That record supplies field, curve and hash operations, chip contracts (each chip as the functional relation it guarantees, not its gate-level lowering), and laws such as encoding round trips. `CircuitProof.agda`, `circuit-faithful`, and the statement soundness and uniqueness developments remain conditional on their stated assumptions and on producer or witness-shape hypotheses, and their conclusions do not transfer to K merely because the instruction surface matches.

The [Agda type-check receipt](../../../evidence/arc-zkir-agda-typecheck-2026-09-05.txt) records successful checking under `--safe`. The repository supplies no concrete `Assumptions` instance, so the planned executable Agda oracle has not run (CLM-0736).

One recorded contradiction also limits correspondence by reading. `Assumptions.agda`, `fromCoordsJ-coordsJ`, requires successful Jubjub coordinate construction to preserve both coordinates. `zkir-curves.k`, `jubjubFromXY`, follows the crate's decompression behavior, which uses only the parity of the supplied x-coordinate, while the in-circuit check `zkir-constraints.k`, `#fromCoordsGate`, tests exact coordinates. This is the recorded K1 divergence, detailed in [13-known-divergences.md](13-known-divergences.md).

## Symbolic reasoning

The [definition summary](../../../wiki/zkir/zkir-k-definition.md) records a Haskell backend experiment: an `add` claim with symbolic input succeeds when the surrounding configuration is pinned, whereas a symbolic `transient_hash` claim does not finish within seven minutes (CLM-0746). This is a single recorded experiment whose claim files are not in the repository; it is not a general proof result for the current definition.

The implementation explains the obstacle. `zkir-hash.k`, `poseidonHash`, unfolds through `absorbAll`, `permute` and `#rounds`, and `sbox` expands into field multiplication, which exposes `modInt`. These defining rules have no restriction to concrete arguments, so symbolic execution expands cryptographic arithmetic instead of retaining an opaque hash application.

The plan specifies uninterpreted hashes and field simplification lemmas for proofs (CLM-0721); the uninterpreted-hash treatment is not implemented. Concrete hash evaluation and a successful arithmetic claim do not establish practical symbolic reasoning for cryptographic programs. See [12-oracles-and-differential-testing.md](12-oracles-and-differential-testing.md) for the commands.

## Threats to validity and open work

Oracle agreement has shared blind spots. The crate is the ground truth of the comparison (CLM-0716), so a crate defect that the definition reproduces cannot be detected. The oracle's `main` runs `preprocess` only, so no verdict is ever compared with the crate's `Relation::circuit`. Generated inputs are small typed values (`diff_test.py`, `build_preimage`, with `small_encoded` from `zkir_values.py`); agreement says nothing about paths they do not reach. Error-class agreement also hides differences within a class, and successful relation checks establish satisfaction only for those witnesses, not universal completeness or soundness.

The [base receipt](../../../evidence/zkir-k-differential-92e8bdd3-2026-09-05c.txt) records 358 agreeing comparisons, including 46 successful-run agreements, and the [extension receipt](../../../evidence/zkir-k-differential-ext-2ffe2d1-2026-09-05c.txt) records 418 agreeing comparisons, including 50 successful-run agreements (CLM-0749). Neither records a successful K run with a non-holding gate. The totals include failure and format-rejection cases and are not counts of complete successful witnesses.

Application coverage is incomplete. Among the Moriarty programs, exactly one comparison reaches a successful witness: swap `expire.zkir` (`experiments/moriarty-core-swap/output/zkir/expire.zkir`), third attempt, `K=ok Rust=ok`, 28 registers, 394 public inputs, with agreeing wrong-transcript and wrong-commitment perturbations. The other swap entry points `decide`, `fundAlice` and `fundBob`, and the escrow entry points `fund`, `refundAfterTimeout` and `release`, record eight error-path agreements each; the six micro-dao precompiles record only error-path agreements in both receipts, 48 rows each. One successful application witness does not amount to transaction-context coverage.

Version pinning bounds every result. `zkir-vm.k`, which the main module `ZKIR` in `zkir.k` imports, names midnight-ledger `92e8bdd3` in its header, and `zkir-ext.k`, `ZKIR-EXT`, names midnight-zkir `2ffe2d1`. The extension changes decoding and instruction behavior, as [09-extension-surface.md](09-extension-surface.md) explains. The K6 divergence shows the difference: a non-canonical `Bytes<32>` element is an assertion panic at `92e8bdd3` and a decode error at `2ffe2d1`, and the definition models both (see [13-known-divergences.md](13-known-divergences.md)). Neither pin establishes behavior at later commits.

Alternative-engine execution also remains unestablished. The [k-rust receipt](../../../evidence/k-rust-compatibility-2026-09-05.txt) records that `krust kast` parses a term of `ZKIR-SYNTAX` in under four seconds, that `krust krun` of a one-instruction program produced no result in ten minutes, and that `krust kcompile` of the full definition was killed after 25 minutes with no output and no diagnostic (CLM-0740). The receipt does not distinguish an unsupported construct from a performance limitation. See [12-oracles-and-differential-testing.md](12-oracles-and-differential-testing.md) for the commands.

Open work consists of missing evidence or implementation: a concrete Agda oracle, symbolic hash abstraction, complete successful application witnesses beyond the single swap case, and a correspondence argument connecting instruction-level relations to circuit gadgets. The repository's existing checks do not discharge these obligations. For extension procedures, see [14-extending-the-semantics.md](14-extending-the-semantics.md).
