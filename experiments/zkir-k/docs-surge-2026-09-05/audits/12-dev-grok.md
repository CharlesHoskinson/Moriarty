# Audit of experiments/zkir-k/docs/12-oracles-and-differential-testing.md (developer)

Verdict: REVISE
(ACCEPT: no blocking or major findings. REVISE: fixable findings. REJECT:
the chapter must be redrafted.)

Checks performed:
compared `zkir-oracle` `main.rs` in both `_build` workspaces (diff is crate imports only) and both `Cargo.toml` dependency paths
confirmed HEAD `92e8bdd3a97b61b229e38916e1b180de6f448dd5` and `2ffe2d17bbb736aec36fb300aeaca679a10d2278`; `git show` of those commits has `pub(crate) fn preprocess` and `pub(crate) fn get_type` on `IrValue`; local `_build` copies have both as `pub`; `encode_offcircuit` is `pub` upstream
read `diff_test.py` (oracle adapter, `ERROR_CLASSES`, `build_preimage`, `compare`, corpora, format branch, seed, attempts, four perturbations, summary line) against the protocol section
read `zkir-vm.k` `genJob`/`checkedJob`/`#wfGate`, `Need` constructors, generation-mode `#input`/`#impactCheck`/`#finish`
read `divergence_tests.py` `main` (checked=True, `all_verdicts`, memory compare, `OUT`) and `check_corpus.py` CORPORA/EXPECTED
compared every receipt number in the results tables with `evidence/zkir-k-*-2026-09-05b.txt`, `evidence/arc-zkir-agda-typecheck-2026-09-05.txt`, `evidence/k-rust-compatibility-2026-09-05.txt`
read `fable-R4.md` F9 and `CONSOLIDATED.md` item 34 for the Haskell claims; confirmed `r4-spec2.k` is not in the repository
confirmed Agda `Main.agda` is parameterised by `Assumptions`, k-rust 0.4.0 at `687ccd0`, `krust` binary and `lesson-02-a.k` exist
ran from the repository root: `diff_test.py --only native_identity` (4 agree, 1.03 s) and `--ext --only bool_identity` (4 agree, 1.05 s); `unit_values.py` (43/43, 6.43 s); `unit_hash.py` (18/18, 2.98 s); `check_corpus.py` (63/63, 8.66 s); the chapter's divergence tempfile snippet (20/20, 3.56 s, `corpus/divergence/` mtimes unchanged); `uv --no-cache run --no-sync --group zkir-k python -c 'print("ok")'`
did not run `cargo build`, `nix run .#agda`, `krust kcompile`/`krun`/`kast`, or the full 15/13-minute differential (writes or long-running)
grepped the chapter for TODO/TBD/placeholders/em-dashes/extra `#` titles (none; one `#` title); confirmed cross-refs 02, 08, 10, 11, 13, 15 exist

## Findings
### F1 blocking "without demonstrating successful transaction witnesses for them" (Results and reproduction)
Claim: The receipts give only error-path agreement on generated escrow, swap and precompile inputs, and do not show a successful witness for those corpora.
Evidence: `evidence/zkir-k-differential-92e8bdd3-2026-09-05b.txt:309` is `PASS  moriarty-core-swap  expire.zkir  run2  K=ok Rust=ok regs=28 pis=394 passes=2 20.6s verdicts=571`, then lines 310-311 `wrong-pubin` and `wrong-comm` (both `K=error Rust=error`). The 46 successful-run rows in that receipt are 31 ledger tests, 14 handmade, and this one swap program. Escrow and both receipts' precompiles stay `K=error Rust=error` for all eight attempts.
Fix: Replace the last sentence of that paragraph with: "Escrow artifacts and the six precompiles, and the swap artifacts other than `expire.zkir`, never reach `ok` on generated inputs, so those rows compare error paths only (up to eight attempts). `expire.zkir` reaches `K=ok Rust=ok` on attempt `run2` (28 registers, 394 public inputs, 571 verdicts); its `wrong-pubin` and `wrong-comm` perturbations then agree."

### F2 blocking "Both workspaces contain a visibility change in `ir_vm.rs`: `pub(crate) fn preprocess` becomes `pub fn preprocess`" (The Rust oracle)
Claim: The only crate function made public for the harness is `preprocess`; `encode_offcircuit` is already public; those plus the `zkir-oracle` workspace member are the local prerequisites.
Evidence: `git show 92e8bdd3:zkir-v3/src/ir_types.rs` has `pub(crate) fn get_type` at line 161 (`IrValue`); the `_build` copy is `pub fn get_type`. The same one-line change is `zkir/src/ir_types.rs:310` at 2ffe2d1. `zkir-oracle/src/main.rs:102` calls `v.get_type()` to fill the `type` field. `git diff` in each `_build` tree is `Cargo.toml` (workspace member), `Cargo.lock`, `ir_vm.rs` (`preprocess`), and `ir_types.rs` (`get_type`). A checkout that applies only the changes the chapter lists does not compile the harness. Must-cover item: which crate functions were made public.
Fix: "Both workspaces carry two visibility changes beyond the upstream commits: in `ir_vm.rs`, `pub(crate) fn preprocess` becomes `pub fn preprocess` (`zkir-v3/src/ir_vm.rs:185`, `zkir/src/ir_vm.rs:210`); in `ir_types.rs`, `IrValue::get_type` becomes `pub` (`zkir-v3/src/ir_types.rs:161`, `zkir/src/ir_types.rs:310`), because the harness prints `format!("{:?}", v.get_type())`. `encode_offcircuit` is already `pub`. Workspace manifests also list `zkir-oracle`."

### F3 major link `../review-2026-09-05/reports/fable-R4.md#f9-minor-proof-readiness-on-the-haskell-backend` (Haskell backend proofs)
Claim: The Haskell experiment is documented by a named reviewer report and a finding-severity anchor.
Evidence: The path and filename name a reviewer (`fable-R4`). `briefs/COMMON.md` hard rule 4 forbids mention of reviewers or audits, and the `CONSOLIDATED.md` entry says not to cite reviewers by name. The same facts without a reviewer name are in `experiments/zkir-k/review-2026-09-05/CONSOLIDATED.md` lines 193-197 (item 34).
Fix: Drop the markdown link and the `fable-R4` / `F9` / `minor` labels. Keep the experimental facts, or point at `experiments/zkir-k/review-2026-09-05/CONSOLIDATED.md` by file name only.

### F4 major "it claims the output register contains `(A +Int 1) modInt #r`" (Haskell backend proofs)
Claim: The completed `add` claim's postcondition is that the output register holds `(A +Int 1) modInt #r` for a canonical native input `A`.
Evidence: The cited report (`fable-R4.md:78`) says only "a claim over one `add` with a symbolic input proves (`r4-spec2.k`, `#Top`, 10.6 s) once every one of the 17 cells is pinned". `r4-spec2.k` is not in the repository; the chapter itself says the claim files live in scratch storage. A developer cannot check this postcondition from the documentation set. I had to guess it from the `add` instruction, which is not a substitute for the missing spec file.
Fix: State only what is in the repository: "A claim over one `add` of a symbolic native input proves (`#Top`, 10.6 s) when every configuration cell is given explicitly; omitting those cells leaves decoding stuck at `#bind`." Do not quote `(A +Int 1) modInt #r` unless the claim module is added to the repository.

### F5 major "42/42 checks" reproduced by `unit_values.py` (Results and reproduction)
Claim: The unit-values receipt is 42/42, and running the shown command reproduces that result.
Evidence: `evidence/zkir-k-unit-values-2026-09-05b.txt` ends `42/42 checks passed` (receipt figure is correctly quoted). Current `experiments/zkir-k/tools/unit_values.py:225` adds `check('dec bytes32 bad high, strict (2ffe2d1 decode_bytes returns None)', ...)`. Ran `uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py`: `43/43 checks passed` in 6.43 s, exit 0. A developer following the reproduction command does not see 42/42.
Fix: After the table, add: "The current `unit_values.py` has 43 checks; the extra check is `dec bytes32 bad high, strict`. A fresh run prints `43/43`." Leave the receipt cell as 42/42, or regenerate the receipt (outside this chapter).

### F6 minor "The repository's `repos` symlink supplies that layout here" (The Rust oracle)
Claim: The tools find the oracle binaries through the repository `repos` symlink.
Evidence: `diff_test.py:37-38`, `unit_hash.py:27` and `divergence_tests.py:28` use `Path.home() / 'Moriarty/repos/_build/...'`. The symlink `repos -> ../../repos` is not consulted. It only makes the chapter's `cargo build --manifest-path repos/_build/...` commands resolve. A developer who repoints the symlink and rebuilds there still has the tools look at `~/Moriarty/repos/_build/`.
Fix: Keep the `~/Moriarty/repos/_build/` sentence. Replace the symlink sentence with: "The tools do not follow the repository `repos` symlink. They require the binaries at `~/Moriarty/repos/_build/<workspace>/target/release/zkir-oracle`. The symlink is what makes the `cargo build --manifest-path repos/_build/...` commands above work from the repository root."

### F7 minor "`extract_test_inputs.py` supplies these fixtures from Rust tests" (Differential protocol, step 3)
Claim: Literal `test_preimage` values in the manifests come from `extract_test_inputs.py`.
Evidence: `extract_test_inputs.py:17` writes only `corpus/ledger9-92e8bdd3-tests/manifest.json`. Extension seeding reads `corpus/midnight-zkir-2ffe2d1-tests/manifest.json`, which also has `test_preimage` entries (for example `bool_identity.zkir`) that this tool does not write. A developer regenerating fixtures for `--ext` with that command would not update the extension manifest.
Fix: "For the ledger tests, `extract_test_inputs.py` copies literal `ProofPreimage` fields from the Rust tests into `corpus/ledger9-92e8bdd3-tests/manifest.json`. The extension corpus has a separate manifest of the same shape; that file is not produced by this script."

### F8 minor "Rows print `PASS` or `FAIL`, corpus, filename, run label, and details" (Raw runs, checked runs, and comparisons)
Claim: The run labels a developer will see are the unperturbed `runN` rows, the four perturbation labels, and `format`.
Evidence: `diff_test.py:215-218` also emits `gen<N>` with summary `krun failed` when `build_preimage` raises `RuntimeError` from a `krun-failed` generation pass; that row counts as a disagreement and stops further attempts for that program. Neither 2026-09-05b receipt contains such a row, but a failed reproduction can.
Fix: Add: "If a generation pass returns `krun-failed`, the row label is `gen<N>`, the summary is `krun failed`, the comparison counts as a disagreement, and no further attempts run for that program."

### F9 minor "repository observations" / "experiment observations" / "Source observation:" (introduction and Agda)
Claim: Statements are labelled with evidence-class tags.
Evidence: `briefs/COMMON.md` hard rule 4 forbids status tags and drafting apparatus. The labels do not change which command to type. Every later claim already names a file or receipt.
Fix: Delete the second introductory paragraph except "All commands start at the repository root; paths beginning with `tools/` or `semantics/` in prose are relative to `experiments/zkir-k/`." Delete "Source observation:" and start at "`Main.agda` takes ...".

## Coverage
- The oracle (`zkir-oracle` under `repos/_build/`, functions made public, printed fields, exit 101, how to build, two versions): partly. Printed JSON fields, exit 101, build commands, two workspaces and the `main.rs` import-only diff match; `IrValue::get_type` is omitted (F2).
- The differential protocol of `diff_test.py` step by step (literal seeds, generation passes, consistent preimage, raw versus checked, comparisons via `ERROR_CLASSES`, four perturbations, format rejection, summary line): covered. Steps match `diff_test.py`; `gen<N>` is missing (F8); extension fixtures are not produced by `extract_test_inputs.py` (F7).
- Results as of the receipts (358/358 and 418/418 with successful-run / error-run breakdowns; divergence, unit and corpus receipts): covered for the quoted receipt numbers (358=46+309+3, 418=50+364+4, 0 disagree, oracle-2 zero, 42/42, 18/18, 63/63, 20/20). The escrow/swap/precompile interpretation is false for `expire.zkir` (F1). Live `unit_values.py` is 43/43 (F5).
- Other oracles (arc-zkir Agda type-checks and is not executable; k-rust does not compile the definition; Haskell one proof done, hash unfolding problem): covered. Haskell paragraph cites a reviewer report (F3) and an unverifiable postcondition (F4).
- How to reproduce each receipt (commands) and how long each takes: covered. Shown Python commands run from the repository root; focused `--only` runs produced four agreeing comparisons in about one second as claimed; unit-hash ~3 s, divergence snippet ~4 s, corpus receipt 27.6 s (live 8.5 s, which the chapter already says not to treat as identical). Agda has no recorded duration. Haskell has no portable command, which the chapter states. Did not re-run cargo, nix, or `krust kcompile`.
