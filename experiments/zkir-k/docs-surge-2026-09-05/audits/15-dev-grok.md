# Audit of experiments/zkir-k/docs/15-design-rationale-and-limits.md (developer)

Verdict: REVISE
(ACCEPT: no blocking or major findings. REVISE: fixable findings. REJECT:
the chapter must be redrafted.)

Checks performed:
- Confirmed the chapter shows no shell commands (one K fragment only); compared that `fadd` quotation with `experiments/zkir-k/semantics/zkir-field.k:46` (verbatim) and the `finv` / `^%Int` / invalid-modulus-returns-0 claims with `zkir-field.k:50-60`.
- Confirmed `encodeValue` / `decodeValue` at `zkir-values.k:134,154`; affine arithmetic header at `zkir-curves.k:1-10`; `jubjubFromXY` uses `X modInt 2` at `zkir-curves.k:148-150`.
- Confirmed `zkir_kast.py` `program` / `load_program` at lines 330 and 367; `syntax Program ::= program(...)` at `zkir-syntax.k:20`; `checkedJob` runs `wf` then `job` at `zkir-vm.k:98-105`; `#put` overwrites at `zkir-vm.k:218`; emission before `#exec` and `#exec` dropped on error/panic at `zkir-vm.k:172-188`.
- Confirmed `eval(gate(add(A, B, O)), ...)` plus `usedChips` / `verdicts` / `#fromCoordsGate` in `zkir-constraints.k` (lines 46, 89, 273, 385-398).
- Confirmed `Runner` constructs pyk `KRun` and reports `stuck` / `depth-exhausted` from a non-empty `<k>` (`tools/zkir_run.py:293-315`); `default_definition` is `semantics/zkir-kompiled` (line 40); read `zkir-kompiled/backend.txt` and `zkir-ext-kompiled/backend.txt` (both `llvm`).
- Confirmed `build_preimage` fills transcripts and the commitment from `runner.run(..., gen=True)` (`diff_test.py:104-134`) and `compare` checks status, `err_class` on failure, and on success `pis`, `pi_skips`, encodings and types (`diff_test.py:137-163`).
- Confirmed `repos/_build/ledger-92e8bdd3/zkir-oracle/src/main.rs:89-113` calls `ir.preprocess` and returns `memory: None` on `Err`; it does not call `Relation::circuit`.
- Located CLM-0704, 0705, 0707, 0709, 0710, 0711, 0712, 0713, 0716, 0720, 0721, 0736, 0740, 0749 in `wiki/zkir-k-semantics-plan.md` and CLM-0746 in `wiki/zkir/zkir-k-definition.md:72`; compared each paraphrase with the plan or definition text.
- Recomputed receipt totals: `evidence/zkir-k-differential-92e8bdd3-2026-09-05b.txt:360-361` is `358` / `46` / `309` / `0 successful K runs with a non-holding gate`; extension receipt line 420-421 is `418` / `50` / `364` / same oracle-2 line. Listed escrow, swap and micro-dao rows; `moriarty-core-swap expire.zkir run2` is `K=ok Rust=ok` at base receipt line 309; the file is `experiments/moriarty-core-swap/output/zkir/expire.zkir`.
- Confirmed Agda names: `record IrSource` (`Syntax.agda:288`), `preprocess` (`Semantics.agda:609`), `synth` (`Circuit.agda:928`), `record Assumptions` (`Assumptions.agda:54`), `fromCoordsJ-coordsJ` (`Assumptions.agda:480-481`), `circuit-faithful` (`CircuitProof.agda:2133`); modules take `Assumptions` as a parameter and there is no concrete instance. Read `evidence/arc-zkir-agda-typecheck-2026-09-05.txt` (`--safe`, exit 0).
- Read `evidence/k-rust-compatibility-2026-09-05.txt` items 1-4. Confirmed no `[concrete]` attribute in `zkir-hash.k`; `poseidonHash` / `absorbAll` / `permute` / `#rounds` / `sbox` at lines 33-67. Compared the Haskell wording with `experiments/zkir-k/review-2026-09-05/reports/fable-R4.md` F9.
- Followed every chapter cross-reference: `05`, `08`, `09`, `10`, `12`, `13`, `14` all exist and match the topic pointed at. `12-oracles-and-differential-testing.md` holds the Agda, k-rust and Haskell replay notes. `zkir.k` is six lines and does not name `92e8bdd3`.
- Grepped the chapter for TODO, TBD, placeholder, em-dash, "this document", extra `#` titles (none). Ran `experiments/zkir-k/docs-surge-2026-09-05/lint_docs.py` (the CLM hits are expected: this chapter brief requires claim identifiers). Counted 1263 words excluding tables, code and the maintainers section.

## Findings
### F1 major "Oracle agreement has shared blind spots" (Threats to validity and open work)
Claim: "`diff_test.py`, `build_preimage`, derives transcript needs and commitments from K generation runs, and both implementations follow related source formulas. The inference is that agreement can miss a shared mistake or an untested semantic path."
Evidence: The named mechanism does not hide a K mistake in those values. `build_preimage` (`experiments/zkir-k/tools/diff_test.py:118-129`) does take `public_transcript_inputs` and the commitment from `genJob`, but the oracle then runs the crate's `preprocess` on that preimage (`repos/_build/ledger-92e8bdd3/zkir-oracle/src/main.rs:89`) and rejects a mismatch (`Public transcript input mismatch`, `Communications commitment mismatch` rows in the receipts). A wrong K transcript value or a wrong `poseidonHash` would surface as `K=ok Rust=error`, not as agreement. A developer debugging "why isn't this comparison independent?" would therefore try to stop feeding `genJob` outputs into the oracle, which would not address the real shared blind spots: the crate is the ground truth of the comparison (`wiki/zkir-k-semantics-plan.md:67`); the oracle never runs `Relation::circuit`; `compare` collapses errors to `err_class` (`diff_test.py:141-145`); generated inputs are `small_encoded` (`diff_test.py:105,214`).
Fix: Replace the first two sentences with: "Oracle agreement has shared blind spots. The crate is the ground truth of the comparison, so a crate defect that the definition reproduces faithfully cannot be detected. The oracle's `main` runs `preprocess` only, so no verdict is ever compared with the crate's `Relation::circuit`. Generated inputs are small typed values, so agreement says nothing about paths they do not reach." Keep the sentence on error classes and on completeness versus soundness.

### F2 major "Thus the blanket claim that generated inputs never succeed for swap is false" (Threats to validity and open work)
Claim: The chapter refutes a "blanket claim" and names `expire.zkir` `run2` as the exception.
Evidence: The claim being refuted is `wiki/zkir/zkir-k-definition.md:53`, which this chapter names only in `## Notes for maintainers`. That section is removed before publication, so a reader of the published chapter sees a rebuttal with no antecedent. The underlying row is true: `evidence/zkir-k-differential-92e8bdd3-2026-09-05b.txt:309` is `moriarty-core-swap expire.zkir run2 K=ok Rust=ok regs=28 pis=394`, followed by agreeing `wrong-pubin` and `wrong-comm` rows; `decide`, `fundAlice`, `fundBob` and the three escrow entry points are error-path agreements; every `midnight-zkir-2ffe2d1-precompiles` row in both receipts is `K=error Rust=error`. A developer who wants to replay the one successful application witness is not given the path `experiments/moriarty-core-swap/output/zkir/expire.zkir`.
Fix: Drop the rebuttal. Write: "Of the four swap entry points only `expire.zkir` (`experiments/moriarty-core-swap/output/zkir/expire.zkir`) reaches a successful witness (third attempt, `K=ok Rust=ok`, 28 registers, 394 public inputs, agreeing wrong-transcript and wrong-commitment perturbations); `decide`, `fundAlice`, `fundBob` and the three escrow entry points record eight error-path agreements each. One successful application witness does not amount to transaction-context coverage."

### F3 minor "records successful syntax parsing but a full-definition compilation timeout after 25 minutes" (Threats to validity and open work)
Claim: The k-rust receipt records syntax parsing and a compile timeout.
Evidence: `evidence/k-rust-compatibility-2026-09-05.txt` items 2-4: item 2 is `krust kast` of the single term `native()` in `ZKIR-SYNTAX` (3.5 s), not a parse of the definition; item 3 is `krust krun` of a one-instruction program, killed after ten minutes with no result; item 4 is `kcompile` killed at 25 minutes with no output. A developer deciding whether k-rust can execute even a tiny program needs item 3. `12-oracles-and-differential-testing.md` (section k-rust) already lists the commands.
Fix: "The k-rust receipt records that `krust kast` parses a term of `ZKIR-SYNTAX` in under four seconds, that `krust krun` of a one-instruction program produced no result in ten minutes, and that `krust kcompile` of the full definition was killed after 25 minutes with no output and no diagnostic (CLM-0740). See `12-oracles-and-differential-testing.md` for the commands."

### F4 minor "Proving-system implementation | Out of scope" (Fidelity boundary table)
Claim: "No proof generation, verifier execution or general modelling of prover-side chip-hint panics establishes cryptographic soundness here."
Evidence: The sentence reads as if those three things exist in the definition but fail to prove soundness. They are not modelled (`wiki/zkir/zkir-k-definition.md:42`).
Fix: "Not modelled: proof generation, verifier execution, and the prover-side panics of chip hints. Nothing in the definition establishes cryptographic soundness or the absence of underconstrained circuits."

### F5 minor "Implementation descriptions below are repository observations" (introduction)
Claim: Categorises statements as "repository observations" and "experiment observations".
Evidence: Those labels are the wiki claim-taxonomy (`wiki/zkir-k-semantics-plan.md:16`, `wiki/zkir/zkir-k-definition.md:19`). The documentation set never defines them. A reader of the stated audience cannot use the sentence.
Fix: "Statements about the implementation describe the files as they are; statements about results cite the receipt under `evidence/` that records them; the reasons behind decisions are those recorded in the semantics plan." Drop the matching sentence above the fidelity table.

### F6 minor "gadget contracts" (Relation to arc-zkir)
Claim: "That record supplies field, curve and hash operations, gadget contracts, and laws such as encoding round trips."
Evidence: `Assumptions.agda:17-19` calls them chip functional contracts. The chapter already uses "chip" for the same idea (`usedChips`, "chip requirements").
Fix: "chip contracts (each chip as the functional relation it guarantees, not its gate-level lowering)".

### F7 minor "`zkir.k`, `ZKIR`, targets midnight-ledger `92e8bdd3`" (Threats to validity and open work)
Claim: The pin is in `zkir.k`.
Evidence: `experiments/zkir-k/semantics/zkir.k` is six lines and only imports `ZKIR-VM`. The commit is in the header of `zkir-vm.k:1-4` and `zkir-ext.k:1-2`. Opening the cited file does not show the pin.
Fix: "`zkir-vm.k`, which the main module `ZKIR` in `zkir.k` imports, names midnight-ledger `92e8bdd3` in its header; `zkir-ext.k`, `ZKIR-EXT`, names midnight-zkir `2ffe2d1`."

### F8 minor "The compiled base and extension definitions identify their backend as `llvm`" (Concrete execution and checking)
Claim: The compiled definitions identify the backend as `llvm`, with no path.
Evidence: The identifier is the one-line file `experiments/zkir-k/semantics/zkir-kompiled/backend.txt` (and `zkir-ext-kompiled/backend.txt`), each containing `llvm`. That path is not named; it had to be guessed.
Fix: "The compiled base and extension definitions record `llvm` in `semantics/zkir-kompiled/backend.txt` and `semantics/zkir-ext-kompiled/backend.txt`."

### F9 minor Symbolic-reasoning and k-rust sections omit the chapter that has the replay notes
Claim: Haskell status is taken from the definition summary; k-rust status is taken from the receipt; the closing paragraph points at `12-oracles-and-differential-testing.md` only for "comparison details".
Evidence: `12-oracles-and-differential-testing.md` sections "k-rust" and "Haskell backend proofs" are where a developer finds the commands, the F9 report path, and the fact that the Haskell claim files live in scratch storage. Chapter 15 answers "what does it mean" but not "what do I type" / "what if it fails" for those two limits.
Fix: At the end of the Symbolic reasoning section, and in the k-rust paragraph, add "see `12-oracles-and-differential-testing.md`".

## Coverage
- Decisions with claim identifiers (K Int arithmetic, no concrete syntax, instruction-level gates, dual modelling, LLVM backend, oracle strategy): covered.
- Table of faithful, abstracted and out-of-scope items with reasons: covered (wording of one row, F4).
- Relation to arc-zkir (shared surface, different purpose, Assumptions record, correspondence by reading): covered (terminology F6).
- Status of symbolic reasoning (Haskell backend experiment, why hashes unfold, uninterpreted hashes not implemented): covered; names and the seven-minute bound match `fable-R4.md` F9; replay pointer missing (F9).
- Threats to validity (oracle shared blind spots, escrow/swap/micro-dao coverage, abstraction limits, version pinning): covered, with the blind-spot mechanism mis-stated (F1) and the swap exception stated as a rebuttal without antecedent (F2); k-rust item understated (F3).
- Open work stated as facts: covered (last paragraph names the concrete Agda oracle, hash abstraction, application witnesses, gadget correspondence).
