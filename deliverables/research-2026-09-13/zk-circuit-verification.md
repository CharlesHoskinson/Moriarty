# Proving that a circuit means what the program meant

Prior art for lowering a Core language to ZKIR over the BLS12-381 scalar field, 2026-09-13.

## The finding that matters most

Nobody has a verified compiler from a source language to a circuit IR. What exists is a settled shape for the correctness argument and a settled list of the places lowerings go wrong.

The shape is Coglio, McCarthy and Smith's: a gadget denotes a relation R over the field, a specification S is a relation in a logic, correctness is soundness R ⊆ S plus completeness S ⊆ R, and it composes gadget by gadget. Every serious effort since adopts this framing (CirC's field-blaster verification conditions, CLAP's Agda theorems, Clean's Lean circuits, Zequal's consistency) or approximates it (Picus, Ecne, fuzzers). A ZKIR semantics in K is exactly the object that lets us state R, so the missing piece is per-lowering-rule soundness and completeness obligations discharged structurally, not more tooling.

They must be structural because automated finite-field reasoning breaks on precisely our workload. Bit decompositions, range checks and limb recompositions are the benchmarks Gröbner-basis solvers time out on: Picus fails on `Num2BitsNeg`; the monolithic cvc5 field solver cannot prove a 32-bit `Num2Bits` deterministic in a week; CirC's soundness conditions were checked only at 4-bit width. And 95 of the 99 circuit-layer vulnerabilities in the USENIX SoK are under-constraint, with the recurring instances in exactly the decomposition and comparison gadgets we must generate.

## 1. The under-constrained circuit problem

**QED2 / Picus.** Pailoor, Chen, Wang, Rodríguez, Van Geffen, Morton, Chu, Gu, Feng, Dillig, *Automated Detection of Under-Constrained Circuits in Zero-Knowledge Proofs*, PLDI 2023 (https://eprint.iacr.org/2023/512; https://github.com/Veridise/Picus). Hazard defined as non-uniqueness: ∃ i, o1, o2. P[i,o1] ∧ P[i,o2] ∧ o1 ≠ o2. Sound propagation rules (proved in Appendix A) plus SMT over cvc5's field theory; outcomes verified, refuted with two concrete witnesses, or unknown, and the method is stated to be incomplete. On 163 circomlib circuits (mean 4,396 constraints, 10-minute timeout) it solved 70 percent: 101 verified, 13 refuted from 8 templates, 49 unknown, all unknowns being Gröbner timeouts on `BabyAdd` and `Num2BitsNeg`. It proves output uniqueness only, not which function is computed and not completeness. Refutations carry witnesses, so no false positives in the linting sense.

**Ecne.** Wang, 0xPARC, 2022 (https://github.com/franklynwang/EcneProject; rules at https://hackmd.io/@ONwIGWrPRcutB_-IRIqcUQ/HkENkNtec). Julia propagation rules over R1CS with trusted sub-circuits; sound, incomplete, needs `--O0`, and its README notes that optimised production constraints then rest on the Circom optimiser being correct. Subsumed by Picus's propagation engine.

**ZKAP.** Wen, Stephens, Chen, Ferles, Pailoor, Charbonnet, Dillig, Feng, USENIX Security 2024 (https://www.usenix.org/system/files/usenixsecurity24-wen_1.pdf). Pattern detectors over a circuit dependence graph; 16.4 percent false positives, 98.6 percent recall on 258 Circom circuits; includes a `RangeProofs` pattern.

**Zequal.** Stephens, Pailoor, Dillig, CAV 2025 (https://eprint.iacr.org/2025/916). Defines the property a compiler actually needs: constraints accept exactly the witnesses the witness generator produces. Verified 306 of 464 Circom templates (66 percent) with Z3.

**Mutation and fuzzing.** circom-mutator (https://github.com/aviggiano/circom-mutator) rewrites source with regexes; its README warns of "numerous false positives". Circomspect (https://github.com/trailofbits/circomspect) is a linter; the Picus paper notes none of its 8 bugs match any Circomspect pattern. zkFuzz (Takahashi, Kim, Jana, Yang, arXiv 2504.11961) and AC4 (Yang, Liang, Chen, Li, arXiv 2403.15676, a computer-algebra checker claiming 29 percent more solved than Picus) are post-hoc bug finders.

## 2. Finite-field reasoning

Two distinct lines. **Ozdemir, Kremer, Tinelli, Barrett, *Satisfiability Modulo Finite Fields*, CAV 2023** (https://eprint.iacr.org/2023/091): a cvc5 theory solver for prime fields; Gröbner-basis unsat test made complete by model construction, UNSAT cores from an instrumented engine. Benchmarks are translation-validation conditions for ZoKrates and CirC on Boolean formulas at 255 bits (2 to 12 inputs, up to 128 terms); runtime is nearly independent of field size, whereas bit-vector encodings collapse by 40-bit fields. **Hader, Kaufmann, Kovács, *SMT Solving over Finite Field Arithmetic*, arXiv 2305.00028**: MCSat-style search with zero-decomposition explanations via subresultant chains, for F_q including extension fields, prototyped in Python over Sage. Ozdemir et al. report it scales poorly with field size and "fails even when b = 1" at 2^255. The groups co-authored the SMT-LIB theory of finite fields (Hader, Ozdemir, arXiv 2407.21169), so inputs are interchangeable.

**Ozdemir, Wahby, Brown, Barrett, *Bounded Verification for Finite-Field-Blasting*, CAV 2023** (https://eprint.iacr.org/2023/778). The closest prior art to our task. Compiler correctness is defined as preserving soundness and completeness of the proof system and shown to compose sequentially, so per-pass proofs suffice. Field-blasting is a set of encoding rules with per-rule soundness and completeness conditions, discharged by cvc5 at bit-width 4 and arity 4; about 850 lines of rules verified, the 150-line calculus and 160-line flattening trusted. Four bugs found in CirC's blaster, including a soundness bug in every bit-vector comparison (Appendix G, Fig. 18): bits Δ_i were witnessed and Δ = Σ 2^i Δ_i was *tested* rather than enforced, so a prover could equivocate on x ≥ y.

**Ozdemir, Pailoor, Bassa, Ferles, Barrett, Dillig, *Split Gröbner Bases*, CAV 2024** (https://eprint.iacr.org/2024/572). Bitsums with bit constraints are the failure mode; 98 percent of Circom repositories use them. BitSplit solved 969 benchmarks to the monolithic solver's 475 and ffsat's 67, with gains on determinism queries. **Pertseva, Robert, Barrett, Parker, arXiv 2605.15163 (2026)** add a Lean tactic for field-to-bitvector translation, solving 19 percent more arithmetization benchmarks than SMT and naming conversions and inequalities as the residual weak spots.

Decidability is trivial over a fixed finite field; degree and width are the problem. In a pipeline this is usable as bounded per-rule conditions at small widths or as a BitSplit-backed determinism check, not as the primary argument for 128-bit arithmetic.

## 3. Verified compilation into circuits

**Aleo / Kestrel (ACL2).** *Leo* (Chin, Wu, Chu, Coglio, McCarthy, Smith, 2021, https://eprint.iacr.org/2021/651): per-compilation ACL2-checked proofs, but for canonicalization and type inference only. *Formal Verification of Zero-Knowledge Circuits* (Coglio, McCarthy, Smith, ACL2 Workshop 2023, https://arxiv.org/abs/2311.08858): the R ⊆ S / S ⊆ R framework, an R1CS model, Axe lifting of circuits with thousands of constraints, and PFCS, a hierarchical formalism introduced because flat R1CS exposes internal variables and blocks compositional completeness proofs. *Compositional Formal Verification* (Coglio et al., 2023, https://eprint.iacr.org/2023/1278): snarkVM's Boolean, field and most integer gadgets verified; compilation of Aleo instructions to R1CS is stated as future work. Two bugs: `Field::to_bits_le` accepted the bits of p as a decomposition of 0, and `Field::sqrt` accepted both roots; both fixes need a bitwise less-than, "adding significant cost".

**CirC** (Ozdemir, Brown, Wahby, S&P 2022, https://eprint.iacr.org/2020/1586; https://github.com/circify/circ): SMT-LIB-style term IR lowered to R1CS; only the field-blaster is verified.

**Cairo** (Avigad, Goldberg, Levit, Seginer, Titelman, CPP 2022, https://arxiv.org/abs/2109.14534): Lean 3 proof that every solution of the production AIR is an execution of the Cairo machine. The one complete IR-soundness proof, but for a fixed CPU, not a source-to-circuit compiler.

**CLAP** (Stronati, Firsov, Locascio, Livshits, arXiv 2405.12115; Nethermind Lean rewrite, https://www.nethermind.io/blog/clap-correctly-compiling-the-leanest-possible-circuits): circuits are Lean programs; the emitted constraint system is proved sound, and constraints plus witness generator complete, relative to that program, modularly per gate. Funded by Aptos to replace the 1.4 M-constraint Keyless Circom circuits; currently "tested, but not yet fully verified".

**Noir.** Lampe (Reilabs, https://github.com/reilabs/lampe) models Noir semantics in Lean for program proofs; no verified Noir-to-ACIR compiler was found.

**zkSecurity.** Clean (https://github.com/Verified-zkEVM/clean), a Lean 4 eDSL targeting AIR, PLONK and R1CS with per-circuit proofs and a witness-generation IR. Hirai's comparison (https://blog.zksecurity.xyz/posts/formal-verification-arithmetic-circuits/) found ACL2 the only framework where both test circuits verified without intervention. Nethermind's CertiPlonk (https://www.nethermind.io/blog/from-certiplonk-to-zkvms-shared-methodologies-for-verifying-risc-v-conformance) extracts Plonky3 AIRs into Lean and proves RISC-V conformance for OpenVM and Brevis Pico.

## 4. Circuit IRs and their semantics

R1CS has an ACL2 satisfaction semantics (`kestrel/crypto/r1cs`), used for lifted proofs. PFCS (ACL2) is hierarchical and represents R1CS, CCS, Plonkish and AIR; it is the nearest analogue to a ZKIR semantics in K, and its use was compositional gadget theorems. AIR has the Cairo Lean semantics and CertiPlonk's Plonky3 extraction. Plonkish has CLAP (Agda, then Lean) and Clean (Lean 4). CirC-IR is SMT-LIB-based and used to generate translation-validation and per-rule conditions. LLZK (Veridise, https://github.com/Veridise/llzk-lib) is an MLIR dialect with no formal semantics found.

## 5. Range checks and fixed-width arithmetic

circomlib `Num2Bits(n)` (https://raw.githubusercontent.com/iden3/circomlib/master/circuits/bitify.circom) costs n bit constraints plus one recomposition; `Num2Bits_strict` adds `AliasCheck`, a comparison of the 254 bits against p−1, because a full-width recomposition is otherwise not injective. `LessThan(n)` decomposes in0 + 2^n − in1 into n+1 bits and reads the top bit; its `assert(n <= 252)` constrains the parameter, not the inputs. Coglio's n-bit adder (Fig. 3) needs p to have at least n+2 bits. Halo2 (https://zcash.github.io/halo2/design/gadgets/decomposition.html) uses running-sum decomposition into K-bit lookup windows, with a "strict mode" that zeroes the final sum to make it a range constraint. CLAP's optimiser removes duplicate range checks as a common-subexpression pass.

For us: 128-bit values have 127 bits of headroom in a 255-bit field, so single-limb addition and comparison cannot wrap and an n+1-constraint decomposition is a valid check. The two-limb 256-bit case is where the failures cluster: hi·2^128 + lo is not injective unless each limb is independently range-checked, and any full-width comparison or subtraction is exactly the CirC and snarkVM failure pattern.

## Tool table

| Tool | Verifies | Input | Maturity | Pipeline fit |
|---|---|---|---|---|
| Picus / QED2 | Output uniqueness; no functional spec | Circom, R1CS, gnark | Maintained; 70 % solved on circomlib | Post hoc; times out on bitsums |
| Ecne | Uniqueness by propagation, trusted sub-circuits | R1CS (`--O0`) | Dormant | Post hoc |
| ZKAP | Pattern vulnerabilities on dependence graph | Circom | Research artifact | Post hoc lint |
| Zequal | Constraint / witness-generator consistency | Circom templates | CAV 2025 artifact | Post hoc; the right property |
| Circomspect | Syntactic lints | Circom | Maintained | Post hoc lint |
| circom-mutator, zkFuzz, AC4 | Mutation / fuzz / CAS bug finding | Circom | Early | Post hoc, high FP |
| cvc5 field theory + BitSplit | Bounded QF_FF conditions | SMT-LIB FFA | In cvc5 | In pipeline, bounded per rule |
| Hader et al. ffsat | QF_FF incl. extension fields | Polynomial systems | Prototype; Yices2 port | Not at 255 bits |
| CirC field-blaster | Per-rule soundness/completeness, b = 4 | CirC-IR | Upstreamed | In pipeline, bounded |
| ACL2 R1CS / PFCS | Gadget R ⊆ S and S ⊆ R at full width | R1CS, PFCS | Public books; snarkVM proofs private | Per gadget, proof assistant |
| Cairo Lean | AIR soundness for fixed machine | Cairo AIR | Production | Not a compiler proof |
| CLAP (Agda / Lean) | Lowering sound and complete w.r.t. circuit program | Rust eDSL / Lean | WIP; Keyless unverified | Correct by construction |
| Clean | Per-circuit soundness/completeness | Lean 4 eDSL | Active | Correct by construction |
| Lampe | Noir program properties | Noir | Active | Not compiler verification |
| CertiPlonk | AIR extraction, machine conformance | Plonky3 | Engagement-based | Post hoc, proof assistant |

## Soundness hazards the lowerer must avoid

1. **Witnessed bits tested, not enforced.** Emitting bit-constrained Δ_i and checking Δ = Σ 2^i Δ_i as an output signal lets a prover choose a wrong decomposition; enforce the decomposition and derive the comparison from it (Ozdemir et al. 2023, Appendix G, Fig. 18).
2. **Field-to-bits without a "< p" check.** A field-width decomposition of 0 also matches the bits of p; any decomposition reaching the field's bit length, including our 256-bit two-limb values, needs an alias check (Coglio et al. 2023 B.1.1; circomlib `Num2Bits_strict`).
3. **Comparators with unbounded inputs.** `LessThan(n)` is meaningful only if both inputs are already range-checked to n bits (zk-bug-tracker #1 Dark Forest; circomlib `comparators.circom`).
4. **Non-deterministic gadgets for source-level functions.** sqrt, inverse and division have multiple field solutions and must be pinned (Coglio et al. 2023 B.1.2; Picus uniqueness definition).
5. **Division-by-zero mismatch.** z·y = x for a source defining x/0 = 0 is unsatisfiable (completeness bug); an unconstrained quotient for y = 0 is a soundness bug; match the Core error semantics explicitly (Ozdemir et al. 2023 Appendix G; Coglio's error value E).
6. **Assigned but not constrained.** Never emit a witness hint without the constraint pinning it; constraints must accept exactly what the witness generator produces (SoK Table 4, 14 cases; Zequal; zk-bug-tracker #14 MiMC).
7. **Optimiser deleting the only check.** Duplicate-range-check and dead-signal elimination must prove implication; unused public inputs optimised out is a known exploit (CLAP Section VI; zk-bug-tracker Common Vulnerabilities #5; Ecne README).
8. **External values not reduced modulo p.** Public inputs and limbs from the ledger side must be range-checked at the boundary (zk-bug-tracker #4 Semaphore, #9 ZkDrops).
9. **Field-specific gadgets moved across fields.** circomlib `Sign` was BN254-only (SoK R3); parametrise every width by the ZKIR field, since BN254 and BLS12-381 bit budgets differ.
10. **Wrap-around in fixed-width arithmetic.** An n-bit adder needs p with at least n+2 bits (Coglio 2023 Fig. 3); 128-bit is safe, 256-bit must be limb-wise with range-checked carries (zk-bug-tracker #15 PSE/Scroll `MulAddWords`).
11. **Relying on a post-hoc solver for bitsum-heavy circuits.** It times out (Picus `Num2BitsNeg`; Split-GB: 32-bit `Num2Bits` unsolved in a week). Discharge the obligation once per lowering rule at full width in the K semantics; use SMT as a bounded cross-check.

## Not retrieved

The ACM page for the Picus paper (ePrint copy used); cvc5's finite-field documentation (404; the SMT-LIB FFA paper used instead); Noir's integer and range-constraint documentation (404); Midnight's ZKIR page (404; the Compact reference was retrieved and confirms zkir output and `Field` as the native prime field); the leo-acl2 README (empty); the Circom TDSC 2022 semantics paper; the Yices2 MCSat finite-field paper (arXiv 2402.17927); the 2025 journal version of the field-blasting paper; the first CertiPlonk post; LLZK documentation beyond its README. The halo2 book lost its formulas in extraction, so lookup range-check costs are stated qualitatively.
