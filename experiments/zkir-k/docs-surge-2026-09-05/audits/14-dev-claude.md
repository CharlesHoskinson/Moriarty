# Audit of experiments/zkir-k/docs/14-extending-the-semantics.md (developer)

Verdict: REVISE
(One blocking finding: a receipt total that the tool no longer prints. Seven
major findings, all local rewrites. The structure, the checklists and the
worked example are otherwise faithful to the files.)

Checks performed:
- Read briefs/AUDIT-COMMON.md, briefs/AUDIT-dev.md, briefs/COMMON.md, briefs/14.md, then the chapter in full.
- Read zkir-syntax.k, zkir-values.k, zkir-ops.k, zkir-vm.k, zkir-constraints.k, zkir-ext.k, zkir.k, zkir-check.k, zkir-test.k in full; grepped zkir-field.k (`fmod`, `#nonResidue`, `[owise]`) and zkir-hash.k (`shaK`, `poseidonHash`, `absorbAll`, `permute`, `sbox`).
- Traced the `add` worked example through zkir-syntax.k:110,159,197, zkir-ops.k:35-47, zkir-vm.k:241,250, zkir-constraints.k:273-274,85, tools/zkir_kast.py:306-308, tools/diff_test.py:75, corpus/handmade (gen_handmade.py:46-63,79), tools/unit_values.py:157-227.
- Traced `less_than` (zkir-vm.k:382-394, zkir-syntax.k:304-305, zkir-constraints.k:312-318) and `reconstitute_field` (zkir-syntax.k:154, zkir-vm.k:363-380).
- Confirmed every quoted K fragment against its source line (chapter lines 14-17, 27-29, 78-80, 117-120).
- Confirmed the extension conventions: `[priority(30)]` on `job`/`genJob` (zkir-ext.k:168-169), the specific arms `negV(boolV(_))`, `eqDispatch`, `#chipsOfInstrs` sha512 (zkir-ext.k:172-180,274), `mkBytes`/`asBytesAny` (zkir-ext.k:74-80), ZKIR-TEST imports (zkir-test.k:32-36).
- Confirmed the chip vocabulary (zkir-constraints.k:46-85,164-166,446-448), `isSpecialEmit` (zkir-vm.k:180-183), `#finish`/`#verdicts` (zkir-vm.k:516-547), the `eval` catch-all and `#opName` (zkir-constraints.k:494-496).
- Confirmed the preprocessor claims in tools/zkir_kast.py (TYPE_SYMBOLS:35-49, EXT/EXT_TYPES/EXT_OPS:59-61, `ir_type`:73-87, `immediate`:90-109, `guard`:155-158, `_need`:191-197, `reverse_bytes`:251-255, `program`:330-364).
- Confirmed `Runner.run` stuck/depth-exhausted logic (tools/zkir_run.py:298-316), `compare` (tools/diff_test.py:135-160), `ERROR_CLASSES` (diff_test.py:67-89), `EXPECTED` and `CORPORA` (check_corpus.py:29-55), `OUT` (divergence_tests.py:27), gen_handmade.py `OUT` (line 12).
- Checked the pitfalls against review-2026-09-05/CONSOLIDATED.md items 1, 2, 21, 25, 29, 34 and wiki/k-framework/k-best-practices.md (priority 50/200, `[owise]`, `[concrete]`, `symbol(_)`, `terminator-symbol`).
- Checked the K3/K4 labels against docs/13-known-divergences.md:38,50,130-131 and wiki/zkir/zkir-k-definition.md:58.
- Checked the Rust message forms cited at chapter line 22 (repos/_build/ledger-92e8bdd3/zkir-v3/src/ir_instructions/add.rs:62, ir_types.rs:296) and the decimal/hex example in evidence/zkir-k-divergence-tests-2026-09-05b.txt (f02).
- Ran from the repository root: `unit_values.py` (43/43), `unit_hash.py` (18/18), `check_corpus.py` (63 programs, 63 as expected), `diff_test.py --only native_identity --seed 2026 --attempts 8` (4 comparisons, 4 agree), `diff_test.py --ext --only bool_identity --seed 2026 --attempts 8` (4 comparisons, 4 agree), `divergence_tests.py` (20/20; corpus/divergence/ was backed up first and compared afterwards: byte-identical), `zkir_run.py corpus/handmade/native_bytes.zkir <manifest preimage> --checked` (status ok, `<k>` is `.K`, 34 constraints, 34 verdicts, 31 registers), the same with `--depth 5` (status depth-exhausted), `zkir_kast.py check --ext` on bool_identity.zkir (wfOk), without `--ext` (format error: unknown IR type 'Bool', exit 2), `zkir_kast.py check --ext` on ledger9 test_reverse_bytes_proof.zkir (format error: unknown instruction op 'reverse_bytes', exit 2), `zkir_kast.py check` on test_sha512_proof.zkir (format error, exit 2).
- Confirmed `kompile --help` (K v7.1.337) accepts `--emit-json` and `--output-definition`; did not run kompile.
- Confirmed the six receipt files named at chapter line 161 exist under `evidence/` at the repository root and read their totals.
- Lint: one `#` title, no em-dashes, no TODO/TBD, no mention of drafting or reviewing.

## Findings

### F1 blocking "Replace the receipts under `evidence/` when the totals change: `zkir-k-unit-values-2026-09-05b.txt` (42 of 42)"
Claim: the unit-values layer totals 42 of 42.
Evidence: `uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py` prints `43/43 checks passed` (the 43rd check is `dec bytes32 bad high, strict (2ffe2d1 decode_bytes returns None)`, tools/unit_values.py:225). The receipt evidence/zkir-k-unit-values-2026-09-05b.txt ends with `42/42 checks passed`, so the receipt is behind the tool by one check. The chapter itself says a stale artefact behind a receipt is a failed check; a developer running the layer as instructed sees a different total from the one the chapter gives.
Fix: write "`zkir-k-unit-values-2026-09-05b.txt` (43 of 43; the tool prints 43 checks and the receipt must be regenerated to match)" or, if the editor prefers not to describe the receipt state, give only the tool's current total (43) and drop the parenthetical for this receipt.

### F2 major "the symbol of an `IrType` is the JSON type key" (Conventions, first paragraph)
Claim: `IrType` symbols are the JSON type keys.
Evidence: zkir-syntax.k:31-43 declares `native() [symbol(Native)]`, `bytes32() [symbol(Bytes32)]`, `jubjubPoint() [symbol(JubjubPoint)]`; the JSON strings are `Scalar<BLS12-381>`, `Bytes<32>`, `Point<Jubjub>`, and tools/zkir_kast.py:35-49 `TYPE_SYMBOLS` maps the JSON string to the symbol. The chapter's own type table (line 94) says "serde name to K symbol", contradicting line 7.
Fix: "the symbol of an `IrType` is the Rust `IrType` variant name (`Native`, `Bytes32`, `JubjubPoint`, ...); `TYPE_SYMBOLS` in `tools/zkir_kast.py` maps the JSON type string (`Scalar<BLS12-381>`, `Bytes<32>`, `Point<Jubjub>`, ...) to it."

### F3 major "Unit check | no `add` row in `unit_values.py`; `fadd` / `ecAdd` are already checked" (worked example table)
Claim: `fadd` and `ecAdd` have unit checks.
Evidence: tools/unit_values.py:167-225 calls `finv`, `fsqrt`, `legendre`, `#jubjubD`, `onCurve`, `#jubjubGenerator`, `inSubgroup`, `ecMul`, `jubjubFromXY`, `encodeValue`, `decodeValue`, `decodeStrict`; no check invokes `fadd`, `ecAdd` or `addV`. tools/unit_hash.py checks Poseidon, hash_to_curve, alignedBytes, sha256 and keccak256 only.
Fix: "no unit check exercises `addV`, `fadd` or `ecAdd` (`ecMul` is checked on all four curves); the instruction is covered by the differential run and the handmade programs only."

### F4 major "their gates still demand one through `#chipFor`, which is the recorded K3 / K4 class of synthesis error" (Adding a chip)
Claim: the `from_bytes32` and `jubjub_scalar_from_native` chip cases are K3 and K4.
Evidence: docs/13-known-divergences.md:38 "K3: a Bytes32 raw element outside canonical form panics" (case `k03`, a panic, not a synthesis error); docs/13-known-divergences.md:50,131 "K4: the Jubjub chip is not enabled for two instructions" (cases `k04`, `k01b`: `jubjub_scalar_from_native` and native `from_coordinates`). The `from_bytes32` case is finding 13 of the earlier review (`f13_chip_gating_from_bytes32`), as the chapter's own chip table (line 112) lists. wiki/zkir/zkir-k-definition.md:58 uses the same numbering.
Fix: "which is the class of synthesis error recorded as finding 13 (`from_bytes32`) and K4 (`jubjub_scalar_from_native`, native `from_coordinates`) in 13-known-divergences.md."

### F5 major "A missing runtime arm (for example `div_mod_power_of_two` with one output) looks like a successful empty memory if the runner is not consulted." (Pitfalls, stuck configurations)
Claim: read as present tense, `div_mod_power_of_two` with one output has no runtime arm.
Evidence: zkir-vm.k:348-349 `rule <k> #exec(divModPowerOfTwo(_, _, Os)) => #fail("DivModPowerOfTwo requires exactly 2 outputs") ... requires lenIds(Os) =/=Int 2`, added for CONSOLIDATED.md item 1 (which describes exactly this case as the defect that was fixed). With `--checked` the program never reaches the VM (zkir-syntax.k:296-297). "Empty memory" is also inexact: the registers written before the stuck step remain in `<mem>`.
Fix: state it as history: "Before the runner inspected `<k>`, `div_mod_power_of_two` with one output had no `#exec` rule; the run stopped with `#exec(...)` in `<k>` and `<status>` still `ok()`, and was reported as a success. The rule at zkir-vm.k `#exec(divModPowerOfTwo(_, _, Os))` with `lenIds(Os) =/=Int 2` now fails it, and `Runner.run` reports any residual `<k>` as `stuck`."

### F6 major "`#chipsOfInstrs((_ ; Is))` otherwise skips the instruction, so a new hash that needs a chip is silently ungated until its `#chipsOfInstrs` arm exists." (Pitfalls, catch-all rules)
Claim: a missing `#chipsOfInstrs` arm leaves the new gate unchecked, silently.
Evidence: the direction is reversed. A gate that demands a chip through `#chipNamed` (zkir-constraints.k:446-448) or `#chipFor` (164-166) returns `synthErr("chip not initialised: ...")` when `<chips>` lacks the name; a missing `#chipsOfInstrs` arm therefore makes every run of the instruction report a synthesis error, which is loud, not silent. The differential harness prints such runs on its `oracle 2:` line (tools/diff_test.py:271) but does not count them as failures (`failures` is incremented only from `compare`, lines 200, 244).
Fix: "so a new hash whose gate demands a chip reports `synthErr("chip not initialised: <name>")` on every run until its `#chipsOfInstrs` arm exists; `diff_test.py` lists such runs on its `oracle 2` line without failing, so read that line."

### F7 major "a new program belongs in the corpus check and the differential harness" together with "`zkir-k-milestone2-corpus-check-2026-09-05b.txt` (63 programs as expected)" (Re-running the check layers)
Claim: a program added per the checklist (corpus/handmade/) is covered by check_corpus.py.
Evidence: tools/check_corpus.py:29-35 `CORPORA` scans `corpus/ledger9-92e8bdd3-tests` (43), `corpus/midnight-zkir-2ffe2d1-precompiles` (6), `corpus/handmade-negative` (7) and `experiments/moriarty-compact-escrow/output/zkir` plus `experiments/moriarty-core-swap/output/zkir` (7 together), which is the 63. It does not scan `corpus/handmade/`, `corpus/midnight-zkir-2ffe2d1-tests/` or `corpus/divergence/`. tools/diff_test.py:40-46 does scan `corpus/handmade`. A developer following the checklist will not find the new handmade program in the corpus-check output and cannot tell from the chapter why the total stays at 63.
Fix: name the scanned directories and say: "a new handmade positive program is covered by `diff_test.py` (its `CORPORA` includes `corpus/handmade/`) and by `check_corpus.py` only if it is placed in a directory that tool scans or `CORPORA` there is extended; a new handmade negative goes in `corpus/handmade-negative/` with an `EXPECTED` entry. The 63 of the corpus-check receipt are 43 + 6 + 7 crate and handmade-negative programs plus 7 compiled outputs of the two `experiments/moriarty-*` projects."

### F8 major "`divergence_tests.py` rewrites `corpus/divergence/`; redirect `OUT` to scratch if those files must stay untouched" (Re-running the check layers)
Claim: `OUT` can be redirected.
Evidence: tools/divergence_tests.py:27 `OUT = HERE.parent / 'corpus' / 'divergence'` is a module constant; the tool takes no arguments and reads no environment variable (main at lines 197-250). Redirecting means editing the tool, which the chapter does not say. Running the tool as shipped regenerated all 20 files byte-identical to the checked-in ones (verified with a backup and `diff -r`), so the rewrite is harmless while `CASES` is unchanged.
Fix: "`divergence_tests.py` regenerates `corpus/divergence/` from its `CASES` table (byte-identical while the table is unchanged); there is no flag to write elsewhere, so edit the `OUT` constant at the top of the tool if the corpus must stay untouched."

### F9 minor "`reads(reconstituteField(D, Mo, _, _))` is `Mo, D, .Operands`" (Conventions, third paragraph)
Claim: quoted rule text.
Evidence: zkir-syntax.k:154 reads `rule reads(reconstituteField(D, M, _, _)) => M, D, .Operands`; `Mo` is the variable of the VM rule (zkir-vm.k:363), not of `reads`.
Fix: quote verbatim: "`reads(reconstituteField(D, M, _, _)) => M, D, .Operands`".

### F10 minor "`wf` arity | no `#checkArity` arm; `wf` only checks names, immediate range and single assignment" (worked example table)
Claim: `wf` checks only those three things.
Evidence: zkir-syntax.k:251-252 also rejects a non-zero minor version, and `#checkArity` (293-306) checks `output`, `div_mod_power_of_two`, `reconstitute_field`, `constrain_bits`, `less_than`. True for `add`, false as a statement about `wf`.
Fix: "for `add`, `wf` checks only the defined names, the immediate range and single assignment (zkir-syntax.k, `#checkReads`, `#wfWrites`)".

### F11 minor "Replace the receipts under `evidence/`" (Re-running the check layers)
Claim: location of the receipts.
Evidence: the chapter's opening paragraph fixes `semantics/`, `tools/` and `corpus/` as relative to `experiments/zkir-k/`; `evidence/` is at the repository root (`evidence/zkir-k-*-2026-09-05b.txt`), and the reader has to guess which base applies.
Fix: "under `evidence/` at the repository root".

### F12 minor "The extension adds arms for `eval`, `#chipsOfInstrs`, `negV`, `eqDispatch`, `encodedLen`, `typeOf`, `encodeValue` and `decodeValue` on top of the base modules." (Pitfalls, duplicate definitions)
Claim: reads as the full list.
Evidence: zkir-ext.k also adds arms for `reads`, `writes` (35-52), `defaultValue`, `typeName` (66-71) and `#eqSupported` (184).
Fix: add the missing names or write "adds arms for ... among others".

## Coverage
- Conventions of the definition (symbol attributes, total/owise, requires guards, priority(30), sequential chains and why, error-string fidelity, witness/gate split): covered (F2 corrects the type-symbol statement; F9 a quotation).
- Checklist for adding an instruction, illustrated with an existing instruction traced through every file: covered; all thirteen rows present and `add` traced through syntax, reads/writes, wf, ops, VM, gate, chip, preprocessor, error class, handmade, unit, divergence (F3 corrects the unit-check row; F10 the wf row).
- Checklist for adding a type: covered.
- Checklist for adding a chip: covered (F4 corrects the K3/K4 label).
- Pitfalls (duplicate definitions, owise overlap, parse ambiguities, catch-all rules, stuck configurations, symbolic unfolding): covered; each has a source in CONSOLIDATED.md items 1, 21, 24, 25, 29, 34 or the K best-practices page (F5 and F6 correct two of them; F12 completes a list).
- How to re-run the check layers and the receipts to update: covered (F1 corrects a total; F7 names what the corpus check scans; F8 corrects the `OUT` claim; F11 the path).
