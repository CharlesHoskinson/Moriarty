# Does totality let us guarantee well-constrainedness by construction?

**Answer: partly, and not for the reason we hoped.** Totality does not remove the wall the post-hoc checkers hit, because that wall is not undecidability. Every checker in the table below already works on a finite polynomial system: a Circom or Halo2 circuit has no loops, no recursion, and static size, exactly what our Core language guarantees. The undecided bucket (17 to 42 percent for Picus and CIVER, 3 to 11 percent for AC4) comes from solving non-linear systems over a 254-bit prime field, chiefly Gröbner-basis blow-up on elliptic-curve and bit-decomposition gadgets. Totality has nothing to say about that. What totality *does* buy is different and real: a finite constructor set with fixed-width types makes the per-constructor proof strategy viable. Two published results (Ozdemir et al., Coglio et al.) prove the composition theorem we want: if each encoding rule or gadget satisfies a local soundness and completeness condition, the whole compiler or circuit is sound. The theorem's hypothesis is where the danger lives. Gadget soundness composes only when every gadget's *precondition* (inputs in range, bits are bits, no field alias) is discharged at every call site; 34 of the 95 under-constrained bugs catalogued in the SNARK SoK are exactly failures of that discharge. So the honest statement is: totality plus a finite constructor set lets us replace per-circuit solving with roughly fifteen fixed proof obligations plus a mechanically-checked interface discipline. It does not make the hard obligations easy; it makes them finite, fixed, and discharged once.

## 1. The post-hoc checkers and why they stall

| Checker | Method | Undecided share | Cause of the undecided bucket |
|---|---|---|---|
| Picus / QED2 (Pailoor et al., PLDI 2023) | Uniqueness constraint propagation, then cvc5 finite-field SMT | 30 percent overall (101 of 163 solved); large circomlib-core circuits 20 percent solved | "Unknown" when no new unique variable is inferred, or resource limit. Failure analysis: BabyAdd requires proving Bernstein-Lange Thm 3.3 (EC addition); Num2BitsNeg gives polynomials of very large degree and coefficients; "the timeout is due to the Groebner basis computation within the SMT solver" |
| CIVER (Isabel, Rodríguez-Núñez, Rubio, IEEE S&P 2024) | Bottom-up modular transformation/deduction rules, Z3 back end | 42 percent unsolved on AC4's benchmark (58.13 percent solved) | Strict per-component timeout; AC4 reports it "skip[s] overly complex circuits". Full paper is paywalled; scope taken from AC4's comparison and the tool README |
| halo2-analyzer / Korrekt (Soureshjani et al., SMT 2023) | Abstract interpretation for unconstrained cells; cvc5 finite-field SMT for uniqueness | No percentage published; run time grows to 4.5 s at 128 bits on a toy | Authors: "the use of an SMT solver likely means that this approach will not scale" |
| Ecne (Wang, 0xPARC, 2022) | Propagation of uniquely-determined variables through R1CS | Not measured | README: "false does not mean that the constraints aren't sound, it just means that Ecne is unable to prove" it |
| AC4 (Yang, Liang, Chen, Li, arXiv 2403.15676v5) | Gauss-Jordan for linear systems, algebraic and binary methods for quadratic | 11 percent not precisely solved (156 of 175); 3 percent unknown after relaxation | "Unknown results may stem from circuit size, computational complexity, or the inherent NP-hardness of solving polynomial equations over finite fields"; Sha256compression exceeds memory |

None of these buckets is caused by loops, recursion, or dynamic indexing: Circom templates are unrolled at compile time and the tools consume flattened R1CS. The wall is algebraic. Our totality reproduces conditions the checkers already enjoy, so by itself it cannot shrink their bucket.

## 2. Correct-by-construction: who does it and at what granularity

**Per compiler pass (CirC, Stanford).** Ozdemir, Kremer, Tinelli, Barrett, "Bounded Verification for Finite-Field-Blasting" (CAV 2023), define compiler correctness as demonstrable completeness plus demonstrable soundness (an efficient inverse from low-level witnesses to high-level witnesses), prove that correctness "is preserved under sequential composition, so proving the correctness of each compiler pass individually suffices" (Theorem 1), and prove that if every encoding rule satisfies its local verification condition, the whole field-blaster is correct (Theorem 3). The rule VCs were checked with cvc5, but only at bit-width 4 and arity 4. Even so, they found four bugs including "a soundness bug that allowed the prover to lie about the results of certain bit-vector comparisons".

**Per gadget, mechanically (Aleo/Kestrel).** Coglio, McCarthy, Smith et al., "Compositional Formal Verification of Zero-Knowledge Circuits" (ePrint 2023/1278) and "Formal Verification of Zero-Knowledge Circuits" (arXiv 2311.08858), verify snarkVM's R1CS gadgets in ACL2 bottom-up: each gadget's constraint set is proved sound and complete against a functional spec, and composite gadgets are proved using the theorems of their sub-gadgets. Their PFCS formalism existentially quantifies auxiliary variables, which is what makes composition through shared wires sound. Scope: the R1CS construction step; correct compilation of Leo to Aleo instructions is stated as future work.

**Per program, with a typed DSL (Coda).** Liu et al., "Certifying Zero-Knowledge Circuits with Refinement Types" (IEEE S&P 2024), give a language whose refinement types are the specification and whose type checker emits Coq lemmas. Type preservation (Theorem 1) is conditional on "well-typed valuations": if inputs violate the input refinements, nothing is guaranteed. Re-implementing 79 circomlib circuits found six previously unknown under-constrained bugs, five of them "missing logical constraints" and one "missing range check".

**Per virtual machine (Cairo).** Avigad et al. (CPP 2022) prove in Lean that the Cairo AIR soundly encodes the machine's step relation, range checks included; per-VM, not per-program, and excluding completeness.

**Noir.** Reilabs' Lampe extracts Noir semantics into Lean for source-level proofs and makes no claim about the emitted ACIR circuit.

## 3. The per-constructor strategy: theorem and counterexamples

The theorem exists twice. Ozdemir's Theorem 3 and Coglio's bottom-up gadget theorems both say: local soundness and completeness of each constructor, plus a composition rule that quantifies internal wires existentially and matches interface types, yields whole-program soundness. Our hypothesis is the published design of CirC's verified field-blaster.

Where it fails to compose, and why our design must care:

1. **Unsafe reuse of a sound sub-circuit.** SoK Table 4 counts 25 "missing input constraints" and 9 "unsafe reuse of circuit" bugs, all under-constrained. The circom-pairing case: BigLessThan is correct, but CoreVerifyPubkeyG1 never constrained its outputs, so inputs above the BLS12-381 prime were accepted. The SoK says it plainly: "the BigLessThan circuit itself is not faulty".
2. **Preconditions as typing assumptions.** Coda's guarantee holds only under the valuation-typing hypothesis. A constructor proved sound for inputs in [0, 2^n) is unsound at a call site that passes an unranged field element. The composition theorem's hypothesis, not its conclusion, is what breaks.
3. **Instance versus witness soundness.** Ozdemir needs different VCs for public-instance and witness variables ("the validity of the encodings of instance variables need not be explicitly enforced"). A constructor proof that assumes a wire is public becomes unsound if the wire is later moved to the witness.
4. **Aliasing across the modulus.** Num2Bits(254) accepts two bit-strings for the same field element because 2^254 exceeds p; circomlib's `_strict` variants exist for this reason (RareSkills). A bit-decomposition constructor is sound only with n strictly below log2 p and an explicit alias check at the boundary.
5. **Bounded verification of the rules.** CirC's rule VCs were checked at width 4; an SMT proof at small width is evidence, not a theorem, at width 64.

The finite constructor set turns these from an open-ended audit into a fixed checklist: each constructor gets a precondition, postcondition, soundness proof and completeness proof, and the compiler, not the programmer, inserts the range and alias constraints that discharge preconditions at every application. That is where Circom's per-template hazard disappears.

## 4. What totality buys, stated precisely

Turner, "Total Functional Programming" (JUCS 2004): a total language trades universality for security; "there are no run-time errors and everything terminates", and equational reasoning is sound without a bottom element; the price is that the language cannot express its own interpreter. Dhall states the operational claim exactly: "You can always type-check an expression in a finite amount of time", evaluation "always succeeds in a finite amount of time", qualified at once by "some short pathological programs that take longer than the heat death of the universe to evaluate". Biere et al. (2003) give the general principle: under a bound k, verification reduces to satisfiability, complete only once k reaches the completeness threshold. The eBPF verifier is that principle in production: bounded loops are accepted by checking "every possible permutation of a loop" against a complexity limit.

The precise statement for us: totality with bounded data turns every semantic question about a Core program into a finite question over a fixed-size polynomial system, moving it from undecidable to NP-hard-but-finite. Circom's compiler already makes that move, which is why the checkers' buckets are non-empty. Totality's gift is structural: a finite grammar means a finite set of proof obligations, and no fixpoint means the composition theorem needs induction over syntax only.

## 5. Residual hazards that totality does not protect against

1. **Field overflow and alias.** Values above p wrap; bit-encodings can alias. SoK root cause R7 (8 bugs); circomlib AliasCheck rationale (RareSkills).
2. **Preconditions not discharged at call sites.** SoK R2 and R3 (34 bugs); Coda's valuation-typing hypothesis; the BigLessThan reuse case.
3. **Instance/witness aliasing and public-input hygiene.** Ozdemir's distinct VCs for instance and witness variables; SoK V4 "Passing Unchecked Data" (10 integration bugs).
4. **Non-deterministic or untrusted witness generation.** Midnight's Compact docs: witness results "should be treated as untrusted input"; SoK V9 witness-generation errors and R1 "assigned but not constrained" (14 bugs); ZKAP's nondeterministic-signal pattern.
5. **Source-to-constraint semantic gap.** SoK R4 "wrong translation of logic into constraints" is the single largest root cause (34 bugs); Lampe verifies Noir source, not ACIR; Coglio's gadget extraction is "trusted".
6. **Constructor proofs that are bounded, not general.** CirC rule VCs at width 4 found real bugs but do not cover width 64.
7. **Hard constructors stay hard.** If any of our fifteen primitives is a hash or curve operation, its one-time proof is the BabyAdd obligation Picus could not discharge automatically; it must be done in a proof assistant, as Coda and Kestrel do.
8. **Completeness bugs.** Over-constraint is invisible to soundness-only checkers (Coglio: "a gadget that has no satisfying assignment is trivially sound"); our obligations must include completeness.
9. **The work meter and 128-element bound are complexity bounds, not correctness bounds.** Dhall's heat-death caveat; eBPF's complexity limit.

## Sources

- Yang, Liang, Chen, Li. "AC4: Algebraic Computation Checker for Circuit Constraints in Zero-Knowledge Proofs." 2025. https://arxiv.org/pdf/2403.15676v5
- Pailoor, Chen, Wang, Rodríguez, Van Gaffen, Morton, Chu, Gu, Feng, Dillig. "Automated Detection of Under-Constrained Circuits in Zero-Knowledge Proofs." PLDI 2023. https://eprint.iacr.org/2023/512.pdf
- Isabel, Rodríguez-Núñez, Rubio. "Scalable Verification of Zero-Knowledge Protocols." IEEE S&P 2024. https://ieeexplore.ieee.org/document/10646792/ (paywalled; not retrieved in full) and https://github.com/costa-group/circom_civer
- Soureshjani, Hall-Andersen, Jahanara, Kam, Gorzny, Ahmadvand. "Automated Analysis of Halo2 Circuits." 2023. https://eprint.iacr.org/2023/1051.pdf
- Wang. "Ecne: An engine for verifying the soundness of R1CS constraints." 2022. https://github.com/franklynwang/EcneProject
- Ozdemir, Kremer, Tinelli, Barrett. "Bounded Verification for Finite-Field-Blasting." CAV 2023. https://eprint.iacr.org/2023/778.pdf
- Coglio, McCarthy, Smith, Chin, Gaddamadugu, Dellepere. "Compositional Formal Verification of Zero-Knowledge Circuits." 2023. https://eprint.iacr.org/2023/1278.pdf
- Coglio, McCarthy, Smith. "Formal Verification of Zero-Knowledge Circuits." 2023. https://arxiv.org/pdf/2311.08858
- Liu, Kretz, Liu, Tan, Wang, Sun, Pearson, Miltner, Dillig, Feng. "Certifying Zero-Knowledge Circuits with Refinement Types." IEEE S&P 2024. https://eprint.iacr.org/2023/547.pdf
- Avigad, Goldberg, Levit, Seginer, Titelman. "A verified algebraic representation of Cairo program execution." CPP 2022. https://arxiv.org/pdf/2109.14534
- Reilabs. "Lampe: Extracting the semantics of Noir to Lean." 2024. https://github.com/reilabs/lampe
- Chaliasos, Ernstberger, Theodore, Wong, Jahanara, Livshits. "SoK: What don't we know? Understanding Security Vulnerabilities in SNARKs." USENIX Security 2024. https://arxiv.org/pdf/2402.15293
- Wen, Stephens, Chen, Ferles, Pailoor, Charbonnet, Dillig, Feng. "Practical Security Analysis of Zero-Knowledge Proof Circuits." USENIX Security 2024. https://www.usenix.org/system/files/usenixsecurity24-wen_1.pdf
- RareSkills. "AliasCheck and Num2Bits_strict in Circomlib." https://rareskills.io/post/circom-aliascheck
- Turner. "Total Functional Programming." JUCS 10(7), 2004. https://www.jucs.org/jucs_10_7/total_functional_programming/jucs_10_07_0751_0768_turner.pdf
- Dhall. "Safety Guarantees." https://docs.dhall-lang.org/discussions/Safety-guarantees.html
- Biere, Cimatti, Clarke, Strichman, Zhu. "Bounded Model Checking." Advances in Computers 58, 2003. https://fmv.jku.at/papers/BiereCimattiClarkeStrichmanZhu-Advances-58-2003-preprint.pdf
- eBPF Docs. "Loops." https://docs.ebpf.io/linux/concepts/loops/
- Midnight. "Compact reference." https://docs.midnight.network/develop/reference/compact/lang-ref
