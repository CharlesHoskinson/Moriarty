# Can K relate the Core semantics to the ZKIR semantics?

**Verdict.** K has no native mechanism for relating two K definitions. `kprove` takes exactly one `--definition` and proves reachability claims *inside* that definition; the only tool that ever took two definitions, `keq`, was added to K in January 2018 and deleted with the Java backend on 6 April 2023, and nothing replaced it in K 7.1 (the version installed here). K can still carry this work, but only by one of two mechanisms, and a reader should choose between them now: (a) a *product definition*, one K module that imports both semantics under distinct cell names and runs a Core program and its compiled ZKIR program in the same configuration, so that a single ordinary `kprove` claim states that both sides reach related final states. This is exactly the "aggregated language" construction of Ciobâcă, Lucanu, Rusu and Roșu, and it gives per-program translation validation, not a compiler theorem. (b) Export both definitions to Lean 4 with `klean` (shipped in `pyk` since 2025) and prove the compiler theorem in Lean, which is what Runtime Verification itself is now doing to relate KEVM to Nethermind's Lean EVM. If the goal is "every Core program agrees with its compilation", the honest path is (b); if the goal is "these N programs agree", (a) is weeks of work and K alone suffices.

## 1. What K provides for cross-semantics work: nothing native

The K user manual describes three activities for a kompiled definition: parsing, execution, and "theorem proving, i.e., verifying whether a set of claims about a K specification hold" (K User Manual, kframework.org/docs/user_manual, retrieved 2026-09-13). A claim is a rewrite rule `claim LHS => RHS requires P ensures Q` whose variables are the definition's own configuration; the manual's "Example: Verification claims" and the `all-path`/`one-path` attributes make this concrete. The installed tool confirms the scope: `kprove --help` (K v7.1.337) lists one `--definition <path>` and one `--spec-module`; there is no second definition, no equivalence mode, no bisimulation flag. A code search of `runtimeverification/k` and `runtimeverification/haskell-backend` for "equivalence" and "bisimulation" returns only alpha-equivalence of variables and internal unification notes.

The historical exception is decisive evidence rather than a hedge. Commit `7cb8f089` "KEq tool in RV-K (#651)" (Dwight Guth, 2018-01-05) added `bin/keq` and `org.kframework.keq.KEqOptions` with flags `--definition1/-d1`, `--definition2/-d2`, `--spec1`, `--spec2`, `--spec-module1`, `--spec-module2`; `KEq.run` called `commonRewriter.equivalence(rewriter1, rewriter2, spec1, spec2)` and printed `#Top` or `#Bottom`. Commit `bc832ef8` "Delete the java backend! (#3251)" (2023-04-06) removed every one of those files. The Haskell backend that replaced it exposes an `implies` RPC endpoint (haskell-backend `docs/2022-07-18-JSON-RPC-Server-API.md`) that checks one KORE pattern against another, but with the stated constraint that "antecedent and consequent must have the same sort"; two definitions with disjoint configurations do not share a sort.

The theory exists outside the tool. Reachability logic (Roșu, Ștefănescu, Ciobâcă, Moore, "One-Path Reachability Logic", LICS 2013, fsl.cs.illinois.edu/publications/rosu-stefanescu-ciobaca-moore-2013-lics.pdf, Fig. 2 read as an image) is a seven-rule system over one transition system. Ciobâcă, Lucanu, Rusu, Roșu, "A Language-Independent Proof System for Full Program Equivalence" (Formal Aspects of Computing 2016, fsl .../ciobaca-lucanu-rusu-rosu-2016-faoc.pdf) and the ICFEM 2014 "Mutual Program Equivalence" paper give a five-rule system (Axiom, Conseq, Case Analysis, Step, Circularity; Fig. 9, p. 21, read as an image) for programs in *two* languages, obtained by aggregating the two signatures and models into one language whose configurations are pairs. The paper states that its proofs are "not computer-checked" and that a "semi-automated version of the proof system" is future work; it was never shipped in K. Matching logic itself (Roșu, "Matching Logic", LMCS 13(4), 2017, arxiv.org/abs/1705.06312) is the shared foundation, which is why the product construction is sound.

## 2. Precedent at scale

Two K definitions have been related to each other exactly once in the literature: Kasampalis, Park, Lin, Adve, Roșu, "Language-parametric compiler validation with application to LLVM" (ASPLOS 2021, doi 10.1145/3445814.3446751; abstract retrieved through Semantic Scholar, full text blocked by ACM). Keq, "the first program equivalence checker that is parametric to the input and output language semantics", is based on cut-bisimulation and validated LLVM IR to MachineIR instruction selection for "over 90% of 4732 supported functions" from SPEC 2006. That is per-function translation validation with two K semantics, using the tool that K has since deleted.

The current precedent uses a proof assistant. `runtimeverification/evm-equivalence` (Runtime Verification, active, last push 2026-05-08) proves KEVM equivalent to Nethermind's Lean 4 EvmYul model "on a per-opcode basis": K-generated Lean (`klean ... --rule 'EVM-OPTIMIZATIONS.optimized.add'`), a hand-written `StateMap.lean` from KEVM states to EvmYul states, 309 manual theorems, 266 generated definitions, and 73 generated plus 17 manual axioms; the README marks it WIP and its blueprint has chapters titled "Axioms and sorries" and "Gas subgoal sorries".

Everything else is single-definition verification: KEVM (Hildenbrandt et al., CSF 2018, 2,628 lines of K, 1,025 rules), IELE (Kasampalis et al., FM 2019, 3,122 lines, 1,255 rules), KWasm (`runtimeverification/wasm-semantics`, `./kwasm prove tests/proofs/...`), Kontrol (docs.runtimeverification.com/kontrol: "compositional symbolic execution" of Solidity against Foundry-style specs) and Simbolik (a symbolic Solidity debugger). None relates two definitions. The FM 2019 IELE paper mentions a Solidity-to-IELE compiler but reports no correctness proof for it.

## 3. The K-only route: a product definition and one `kprove` claim

Since no claim can span two kompiled directories, build one. Concretely:

1. Write `core-zkir-product.k` that `imports` the Core main module and the ZKIR main module and declares one configuration wrapping both, with cell names made distinct (both definitions currently use `<k>`; the Core configuration is `<k> $PGM:Packet </k> <out> pending </out>`, the ZKIR one is `<zkir> <k> $PGM:Job </k> <mem/> <pi/> ... <outputs/> <constraints/> <chips/> </zkir>`). Rename the Core side to `<core> <ck/> <out/> </core>` and re-target its rules, or wrap with a `[multiplicity]`-free outer cell and rely on distinct sorts. Ciobâcă et al. §5 lists three aggregation schedules; for two deterministic semantics the simplest is sequential, Core to completion then ZKIR.
2. `kompile core-zkir-product.k --backend haskell`.
3. Write `core-zkir-product-spec.k`:

```
module CORE-ZKIR-SPEC
  imports CORE-ZKIR-PRODUCT
  claim <core> <ck> P:Packet => .K </ck> <out> pending => ?O </out> </core>
        <zkir> <k> J:Job => .K </k> <outputs> .List => ?Z </outputs> ... </zkir>
    requires wellFormed(P) andBool J ==K compile(P)
    ensures  agrees(?O, ?Z)
endmodule
```

4. `kprove core-zkir-product-spec.k --definition core-zkir-product-kompiled --spec-module CORE-ZKIR-SPEC`.

What this does and does not give. `?O`, `?Z` are existentials on the RHS (User Manual, "Verification claims"); with the default all-path type the claim says every execution of the pair reaches related final outputs, which is the Step rule of the equivalence system for terminating programs. `compile` must either be a K function (feasible if the compiler is small and total) or be replaced by the concrete ZKIR job for each test program, which reduces the claim to translation validation. `agrees` is the state-similarity relation the FAoC paper makes a parameter; writing it is most of the intellectual work. Loops in either semantics need `[circularity]` claims or `kprove`'s automatic loop detection, and the manual warns that the Haskell backend "does not attempt to prove claims which right-hand side is `#Bottom`", so non-termination on one side cannot be stated as a claim. A cheaper per-program check is `krun` both sides with `--output kore` and compare with the `implies` endpoint, but that is testing, not proof.

## 4. Proof assistants

`pyk.klean` (k repository, `pyk/src/pyk/klean/README.md`, 17 commits, latest 2026-06-17) generates a Lake project (`Sorts.lean`, `Func.lean`, `Rewrite.lean`) from a kompiled definition; the README warns that the rewrite relation is "an over-approximation, as it does not take rule priorities into consideration" and that functions become axioms. Open issues #4552 "A shallow embedding of K definitions into Lean 4", #4725 "Implement K prelude in Lean 4" and #4743 show the backend is unfinished. This is nonetheless the only route to a universally quantified compiler theorem: export both definitions, write the state map, prove by induction on Core programs.

Coq: Bereczky, Chen, Horpácsi, Peña, Tušil, "Mechanizing Matching Logic in Coq" (FROM 2022, arXiv:2201.05716) and the library `harp-project/AML-Formalization` (Coq 8.20, active 2026-02) mechanize the logic, not K definitions; there is no K-to-Coq exporter. Proof certificates: Chen, Lin, Trinh, Roșu (CAV 2021) and Lin, Chen, Trinh, Wang, Roșu (OOPSLA 2023) make `kprove` runs checkable in Metamath against a 240-line formalization (105 s to generate a 37 MB certificate for sum-to-n), which would let route (a) discharge K's "550,000 lines of unverified code" from the trust base. Chen and Roșu's 2026 basic matching logic paper has a Lean 4 mechanization (`eveil-labs/matching-logic-lean`), unrelated to K definitions. ACL2 and Isabelle: nothing found.

## 5. Cost

Ștefănescu, Park, Yuwen, Li, Roșu, "Semantics-Based Program Verifiers for All Languages" (OOPSLA 2016), Table 2: semantics development 40 months (C, 17,791 LOC), 20 months (Java), 4 months (JavaScript); language-specific verifier effort 4 to 7 days each; the K verification infrastructure itself "about 2.5 man-years". Dasgupta et al. (PLDI 2019) report 8 man-months for x86-64. Keq validated thousands of functions automatically once built, but building it was a PhD-scale project and the tool is gone. evm-equivalence has consumed more than a year of a dedicated team and remains per-opcode and WIP. For this project: route (a) for a fixed corpus of Core programs is 3 to 6 weeks (product module, similarity relation, lemmas for the ZKIR field arithmetic); route (b) for the universal theorem is 6 to 18 months and depends on `klean` maturing.

## Sources retrieved

- K User Manual, K Team, kframework.org/docs/user_manual (2026-09-13); `kprove --help`, K v7.1.337, local install.
- runtimeverification/k commits 7cb8f089 (2018) and bc832ef8 (2023); `KEqOptions.java`, `KEq.java` at parent 42cad946; `pyk/src/pyk/klean/README.md`; issues #4552, #4725, #4743.
- runtimeverification/haskell-backend, `docs/2022-07-18-JSON-RPC-Server-API.md`.
- Roșu, Ștefănescu, Ciobâcă, Moore, One-Path Reachability Logic, LICS 2013 (fsl PDF).
- Ciobâcă, Lucanu, Rusu, Roșu, FAoC 2016 and ICFEM 2014 (fsl PDFs).
- Roșu, Matching Logic, LMCS 2017, arXiv:1705.06312.
- Ștefănescu et al., OOPSLA 2016; Hildenbrandt et al., KEVM, CSF 2018; Kasampalis et al., IELE, FM 2019; Dasgupta et al., PLDI 2019; Bogdănaș and Roșu, K-Java, POPL 2015 (fsl PDFs).
- Kasampalis et al., ASPLOS 2021, doi 10.1145/3445814.3446751 (abstract via Semantic Scholar).
- Chen, Lin, Trinh, Roșu, CAV 2021; Lin et al., OOPSLA 2023 (fsl PDFs).
- runtimeverification/evm-equivalence and its blueprint site; runtimeverification/evm-semantics, wasm-semantics, iele-semantics, kontrol READMEs; docs.runtimeverification.com/kontrol and /simbolik; runtimeverification/k-vs-coq-language-frameworks and the "K vs. Coq" blog part 1.
- Bereczky et al., arXiv:2201.05716; harp-project/AML-Formalization; eveil-labs/matching-logic-lean; kframework/matching-logic-prover; Formal-Systems-Laboratory/matching-logic-mm0.

Not retrieved: the ASPLOS 2021 full text (ACM returned 403), Kasampalis's PhD thesis, the arXiv API (HTTP 503 during this session; the arXiv web search was used instead), and the old fsl.cs.illinois.edu per-paper pages (site simplified; PDFs fetched from the publications index).
