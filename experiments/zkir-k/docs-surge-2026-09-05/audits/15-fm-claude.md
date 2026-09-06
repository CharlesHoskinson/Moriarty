# Audit of experiments/zkir-k/docs/15-design-rationale-and-limits.md (formal methods expert)

Verdict: REVISE
(No blocking finding. Two major findings in the threats-to-validity section: one mis-stated mechanism for the oracle's shared blind spots, one sentence that argues against a claim the chapter never states. The rest is minor. Every statement about what the definition establishes was checked against the rules and the recorded results and found neither stronger nor weaker than the evidence supports.)

Checks performed:
- Located every claim identifier the chapter cites in `wiki/zkir-k-semantics-plan.md` (CLM-0704, 0705, 0707, 0709, 0710, 0711, 0712, 0713, 0716, 0720, 0721, 0736, 0740, 0749; lines 24, 26, 49, 53, 57, 61, 65-69, 80, 86, 92) and CLM-0746 in `wiki/zkir/zkir-k-definition.md:72`; compared each paraphrase with the plan text.
- Compared the quoted `fadd` rule with `zkir-field.k:46` (verbatim match) and the `finv`, `^%Int` and `[owise] => 0` statements with `zkir-field.k:50-63`.
- Confirmed `jubjubFromXY` uses `X modInt 2` (`zkir-curves.k:148-150`), `fromXY` requires `onCurve` of the exact pair (`zkir-curves.k:139-141`), and `#fromCoordsGate`/`#fromCoordsPt` report `violated` for an off-curve pair (`zkir-constraints.k:385-398`).
- Confirmed `eval(gate(add(A, B, O)), ...)` checks `#need`, `#chipsForValues` and `#matches(#bin2(...))` (`zkir-constraints.k:273-274`); `usedChips`, `eval`, `verdicts` and the five outcome constructors exist (`zkir-constraints.k:40-46, 89, 95`).
- Confirmed emission before witness half (`zkir-vm.k:172`), `#exec` dropped on error or panic (`zkir-vm.k:174-175`), `<piIdx>` advanced at emission without a status guard (`zkir-vm.k:184-186`), `#put` overwrites (`zkir-vm.k:218`), `checkedJob` runs `wf` first (`zkir-vm.k:98`), `#verdicts` is scheduled unconditionally at the end of `job` (`zkir-vm.k:105, 517`).
- Confirmed `wf` lives in `zkir-syntax.k:250` and `program(...)` is the `Program` constructor (`zkir-syntax.k:20`); `zkir_kast.py` builds `KApply('program', ...)` in `program` and `load_program` (lines 330-374).
- Confirmed `Runner` uses pyk `KRun` and reports `stuck`/`depth-exhausted` from a non-empty `<k>` (`zkir_run.py:24, 293-315`); `compare` returns on status mismatch, compares `err_class` on error/panic, and on success compares `pis`, `pi_skips`, memory encodings and types (`diff_test.py:137-163`); `build_preimage` derives needs and commitment from `runner.run(..., gen=True)` (`diff_test.py:104-134`).
- Confirmed the oracle calls `ir.preprocess` and returns `memory: None` on `Err` (`repos/_build/ledger-92e8bdd3/zkir-oracle/src/main.rs:89-113`); `impl Relation for IrSource` with `fn circuit` exists at `ir_vm.rs:694-705` and is not called by the oracle.
- Read `backend.txt` of all four `*-kompiled` directories: `llvm`.
- Checked the Agda names: `record Assumptions` (`Assumptions.agda:54`), `fromCoordsJ-coordsJ : ∀ {x y p} → fromCoordsJ x y ≡ just p → coordsJ p ≡ (x , y)` (`Assumptions.agda:480-481`), `record IrSource` (`Syntax.agda:288`), `preprocess : IrSource → ProofPreimage → Maybe State` (`Semantics.agda:609`), `synth : IrSource → Circuit` (`Circuit.agda:928`), `circuit-faithful` (`CircuitProof.agda:2133`), `statement-sound-false/true` (`StatementSoundness.agda:5472, 5643`), `statement-unique` with `producer-SA` hypothesis (`StatementUniqueness.agda:837`); arc-zkir README pins `92e8bdd3` (`README.md:4-5`).
- Read `evidence/arc-zkir-agda-typecheck-2026-09-05.txt` (fifteen modules, `--safe`, exit 0) and `evidence/k-rust-compatibility-2026-09-05.txt` (four items; item 2 is `krust kast` of one term, item 3 a `krust krun` killed at ten minutes, item 4 `kcompile` exit 124 after 25 minutes with no output).
- Recomputed the differential totals from the receipts: base 358 rows = 46 `K=ok Rust=ok` + 309 `K=error Rust=error` + 3 `format` rows; extension 418 = 50 + 364 + 4 `format` rows; both end with `oracle 2: 0 successful K runs with a non-holding gate`.
- Listed every escrow, swap and precompile row in both receipts: escrow (3 programs, 24 rows) all `K=error Rust=error`; swap (4 programs, 29 rows) all error except `moriarty-core-swap expire.zkir run2 K=ok Rust=ok regs=28 pis=394` and its two perturbation rows; `midnight-zkir-2ffe2d1-precompiles` (48 rows in each receipt) all error.
- Read `review-2026-09-05/reports/fable-R4.md` F9 (`add` claim proves at 10.6 s with all 17 cells pinned; `transient_hash` claim times out at 420 s; no `[concrete]` attribute in `zkir-hash.k`) and `CONSOLIDATED.md` items 1, 34.
- Verified no `[concrete]` attribute in `zkir-hash.k`, `zkir-field.k`, `zkir-constants.k`.
- Hard rules: one `#` title, no em-dash, no TODO/TBD/draft/review mention outside the maintainers section, 1263 words excluding tables, code and the maintainers section.

## Findings

### F1 major "Oracle agreement has shared blind spots" (section Threats to validity and open work, first paragraph)
Claim: "`diff_test.py`, `build_preimage`, derives transcript needs and commitments from K generation runs, and both implementations follow related source formulas. The inference is that agreement can miss a shared mistake or an untested semantic path."
Evidence: The mechanism named does not produce a shared blind spot. `build_preimage` (`experiments/zkir-k/tools/diff_test.py:104-134`) takes the public transcript inputs and the commitment from K's `genJob`, but the oracle then runs the crate's `preprocess` on that preimage (`repos/_build/ledger-92e8bdd3/zkir-oracle/src/main.rs:89`), which recomputes every public-input value and the commitment itself and rejects a mismatch ("Public transcript input mismatch", "Communications commitment mismatch" rows in the receipt). A K error in a transcript value or in `poseidonHash` would therefore surface as `K=ok Rust=error`, not be hidden. The actual shared blind spots supported by the sources are: (a) the crate is the ground truth (`wiki/zkir-k-semantics-plan.md:67`, "The crate is the ground truth the spec itself defers to"), so a crate defect that the definition reproduces faithfully is invisible to the comparison; (b) the oracle exercises `preprocess` only, never `Relation::circuit` (`main.rs:89`; `ir_vm.rs:694-705`), so no verdict is ever compared with the crate's circuit; (c) `compare` reduces error messages to `err_class` (`diff_test.py:92, 141-145`), which the chapter already states; (d) the generated inputs are small typed values (`build_preimage`, `small=True` at `diff_test.py:214`), so many paths are never reached.
Fix: Replace the first two sentences with: "Oracle agreement has shared blind spots. The crate is the ground truth of the comparison, so a crate defect that the definition reproduces faithfully cannot be detected. The oracle's `main` runs `preprocess` only, so no verdict is ever compared with the crate's `Relation::circuit`. Generated inputs are small typed values, so agreement says nothing about paths they do not reach." Keep the sentence on error classes and on completeness versus soundness.

### F2 major "Thus the blanket claim that generated inputs never succeed for swap is false" (section Threats to validity and open work, third paragraph)
Claim: The chapter refutes "the blanket claim" without stating or citing it.
Evidence: The claim being refuted is `wiki/zkir/zkir-k-definition.md:53` ("The escrow, swap and micro-dao programs never reach a successful witness with generated inputs"), which the chapter names only in `## Notes for maintainers`. A reader of the published chapter sees a rebuttal with no antecedent. The underlying facts are correct: `evidence/zkir-k-differential-92e8bdd3-2026-09-05b.txt` has `moriarty-core-swap expire.zkir run2 K=ok Rust=ok regs=28 pis=394 passes=2` followed by `wrong-pubin` and `wrong-comm` rows; `decide`, `fundAlice`, `fundBob` and the three escrow entry points (`fund`, `refundAfterTimeout`, `release`) record eight `K=error Rust=error` attempts each; the 48 precompile rows in each receipt are all errors.
Fix: Replace "There is an explicit exception: swap `expire.zkir`, `run2`, records `K=ok Rust=ok`. Thus the blanket claim that generated inputs never succeed for swap is false; the evidence still falls short of complete transaction-context coverage." with: "Of the four swap entry points only `expire.zkir` reaches a successful witness (third attempt, `K=ok Rust=ok`, 28 registers, 394 public inputs, agreeing wrong-transcript and wrong-commitment perturbations); `decide`, `fundAlice`, `fundBob` and the three escrow entry points record eight error-path agreements each. One successful application witness does not amount to transaction-context coverage."

### F3 minor "Proving-system implementation | Out of scope" (Fidelity boundary table)
Claim: "No proof generation, verifier execution or general modelling of prover-side chip-hint panics establishes cryptographic soundness here."
Evidence: The sentence reads as if these three things exist in the definition but fail to establish soundness. `wiki/zkir/zkir-k-definition.md:42` states the intended fact: they are not modelled, and a run never establishes that no other witness exists or that a hash gadget is fully constrained.
Fix: "Not modelled: proof generation, verifier execution, and the prover-side panics of chip hints. Nothing in the definition establishes cryptographic soundness or the absence of underconstrained circuits."

### F4 minor "Implementation descriptions below are repository observations; recorded execution results are experiment observations" (introduction, and "The following boundaries are repository observations" in Fidelity boundary)
Claim: Categorises the chapter's statements with the wiki's claim-taxonomy labels.
Evidence: "repository observation" and "executed test" are evidence-type labels of the wiki claim format (`wiki/zkir/zkir-k-definition.md:19, 40`, `wiki/zkir-k-semantics-plan.md:24`). The documentation set never defines them, so the sentence gives the stated audience nothing usable.
Fix: Replace with plain wording: "Statements about the implementation describe the files as they are; statements about results cite the receipt under `evidence/` that records them; the reasons behind decisions are those recorded in the semantics plan." Drop the corresponding sentence at the head of the fidelity table.

### F5 minor "gadget contracts" (Relation to arc-zkir, second paragraph)
Claim: "That record supplies field, curve and hash operations, gadget contracts, and laws such as encoding round trips."
Evidence: `Assumptions.agda:17-19` says "Chips are modeled at the level of their *functional contract*: each field below is the relation a `midnight-zk-stdlib` operation guarantees, not its gate-level lowering"; the arc-zkir README calls them "chip contracts" (`README.md:112`). The chapter uses "chip" for the same notion elsewhere (`usedChips`, "chip requirements").
Fix: "chip contracts (each chip as the functional relation it guarantees, not its gate-level lowering)".

### F6 minor "records successful syntax parsing but a full-definition compilation timeout after 25 minutes" (Threats to validity and open work, fifth paragraph)
Claim: The k-rust receipt records syntax parsing and a compile timeout.
Evidence: `evidence/k-rust-compatibility-2026-09-05.txt` items 2 to 4: item 2 is `krust kast` of the single term `native()` in `ZKIR-SYNTAX` (3.5 s), not a parse of the definition; item 3 is `krust krun` of a one-instruction program that produced no result within the ten-minute tool limit and was killed; item 4 is the `kcompile` killed at 25 minutes with no output. The chapter omits item 3 and overstates item 2.
Fix: "The k-rust receipt records that `krust kast` parses a term of `ZKIR-SYNTAX` in under four seconds, that `krust krun` of a one-instruction program produced no result in ten minutes, and that `krust kcompile` of the full definition was killed after 25 minutes with no output and no diagnostic (CLM-0740)."

### F7 minor "The granularity is deliberate" (Two semantics in one configuration, second paragraph)
Claim: The instruction-level gate granularity is a recorded decision, but no claim identifier is given although the brief asks for them where the plan uses them.
Evidence: The plan records the constraint set as instruction-level entries appended per rule (CLM-0704, `wiki/zkir-k-semantics-plan.md:24`) and the check as "every entry of the `constraints` cell must hold under the `mem` cell" (CLM-0716 item 2, line 67); the statement "Gates are instruction-level relations, not PLONKish gate rows" is CLM-0734 (`wiki/zkir/zkir-k-definition.md:40`).
Fix: "The granularity is deliberate (CLM-0704, CLM-0734)."

### F8 minor "`zkir.k`, `ZKIR`, targets midnight-ledger `92e8bdd3`" (Threats to validity and open work, fourth paragraph)
Claim: The pin is attributed to `zkir.k`.
Evidence: `zkir.k` (six lines) only imports `ZKIR-VM`; the commit is named in the header of `zkir-vm.k:1-4` ("zkir-v3/src/ir_vm.rs at midnight-ledger 92e8bdd3") and in `zkir-ext.k:1-2`. The statement is true of the module but a reader following the file reference finds no pin.
Fix: "`zkir-vm.k`, which the main module `ZKIR` in `zkir.k` imports, names midnight-ledger `92e8bdd3` in its header; `zkir-ext.k`, `ZKIR-EXT`, names midnight-zkir `2ffe2d1`."

### F9 minor "see `14-extending-the-semantics.md`" (last paragraph)
Claim: Cross-reference to chapter 14.
Evidence: `experiments/zkir-k/docs/` holds chapters 01 to 13 and 15; `14-extending-the-semantics.md` is not present at the time of this audit. It is in the common brief's chapter list, so this is a note for the editor, not an error of the chapter.
Fix: Confirm the file exists when the set is assembled; otherwise remove the reference.

## Coverage
- Decisions with claim identifiers (K Int arithmetic, no concrete syntax, instruction-level gates, dual modelling, LLVM backend, oracle strategy): covered; instruction-level gates lack the identifier (F7).
- Table of faithful, abstracted and out-of-scope items with reasons: covered (wording of one row, F3).
- Relation to arc-zkir (shared surface, different purpose, Assumptions record, correspondence by reading): covered; terminology F5.
- Status of symbolic reasoning (Haskell backend experiment, why hashes unfold, uninterpreted hashes not implemented): covered; all numbers and names match `fable-R4.md` F9 and `zkir-hash.k`.
- Threats to validity (oracle shared blind spots, escrow/swap/micro-dao coverage, abstraction limits, version pinning): covered, with the blind-spot mechanism mis-stated (F1) and the swap exception stated as a rebuttal without antecedent (F2); k-rust item understated (F6).
- Open work stated as facts: covered (last paragraph names the concrete Agda oracle, hash abstraction, application witnesses, gadget correspondence).
