# Audit of experiments/zkir-k/docs/02-getting-started.md (developer)

Verdict: REVISE
(ACCEPT: no blocking or major findings. REVISE: fixable findings. REJECT:
the chapter must be redrafted.)

Checks performed:
- Verified every path named in the chapter exists from the repository root (toolchain script, pyproject.toml, four `*-kompiled/` directories and their `compiled.json`, the five corpus programs used, the eight tools named, both `zkir-oracle` binaries under `~/Moriarty/repos/_build/`). Of the cross-referenced chapters only `03-program-model.md` exists at audit time; 06, 08, 09, 10, 11, 12, 13 are not yet in `experiments/zkir-k/docs/`.
- `kompile --version` prints `K version: v7.1.337`; `pyproject.toml` lines 19-21 define group `zkir-k = ["kframework==7.1.337"]`, line 4 `requires-python = ">=3.13"`; `kup install --help` accepts `--version VERSION`; `kompile --help` lists `-O1` and `--output-definition`.
- Ran the first-program command (section "Running a first program") exactly as written, with the preimage written to `/tmp/transient_hash.pre.json`; diffed the real JSON against the chapter's block: byte-identical (memory order, all ten registers, `pis`, `cursors`, 8 constraints, 8 verdicts, gate strings).
- Ran the same command with `--checked`; output byte-identical to the raw run, as the chapter states.
- Ran the `--ext` command on `bool_via_neg.zkir` with the chapter's preimage: output identical to the chapter (commGate holds with commitment 13017667727313041872964242512164587272394045712655975830054323807645461770462 and opening 7, `outputs` `["Bool:0"]`). Ran it without `--ext`: traceback ends `zkir_kast.ZkirFormatError: unknown IR type 'Bool'` raised at `zkir_kast.py` line 84 in `ir_type`. `zkir_kast.py check` on the same file prints `format error: unknown IR type 'Bool'` and exits 2.
- Ran `undefined_variable.zkir` raw and with `--checked`: both outputs identical to the chapter. Ran `zkir_kast.py check` on it: prints `wfError ( "undefined variable %c" )`.
- Ran `reassignment.zkir` raw (status ok, `%b` encoded `2`, `copy` and `add` gates `violated`) and `--checked` (`well-formedness: reassignment of %b`, 0 constraints, empty memory): matches the chapter's prose.
- Ran `f02_assert_non_boolean.zkir` with the chapter's preimage: output identical to the chapter. `divergence_tests.py` line 51 labels the case `'Finding 2'`.
- Ran the four suites: `unit_values.py` ends `42/42 checks passed`; `unit_hash.py` ends `18/18 checks passed`; `check_corpus.py` ends `63 programs, 63 as expected, 0 unexpected, 8.1s`; `divergence_tests.py` ends `20/20 divergence cases behave as expected`. All exit 0.
- Ran `diff_test.py --only transient_hash --no-perturb --attempts 1`: one PASS row for `handmade transient_hash.zkir run0 K=ok Rust=ok regs=10 pis=1 passes=1`, summary `1 comparisons ... 1 agree, 0 disagree`.
- Compared the K fragment in "Running with `--checked`" with `zkir-vm.k` lines 98-103: verbatim. Confirmed `ZKIR-VM` imports `ZKIR-WF` (`zkir-vm.k` line 47), `ZKIR-WF` is in `zkir-syntax.k` (line 242), `ZKIR` imports `ZKIR-VM` (`zkir.k` line 5), `ZKIR-EXT` imports `ZKIR-VM` (`zkir-ext.k` line 161). `job`, `checkedJob`, `genJob` symbols at `zkir-vm.k` lines 40-42.
- Compared the preimage description with `preimage_term` (`zkir_run.py` lines 72-82: defaults `binding_input` 0, `noComm`, empty lists) and `PreimageError` (lines 56-64); confirmed with a non-canonical input (r itself) that the tool raises `PreimageError: ... is not a canonical field element`.
- Compared the output-key explanation with `zkir_run.py` lines 305-335 (`stuck` vs `depth-exhausted`, `cursors` from `<pubInIdx>`, `<pubOutIdx>`, `<privIdx>`, `k_cell`, `needs`) and `zkir-vm.k` lines 66-70, 524-527 (`needComm` only under `<genMode> true`).
- Compared the four-suite descriptions with the tools: `unit_hash.py` lines 1-9 and 27 (oracle path), `check_corpus.py` lines 30-35 (five corpora) and 92 (major version 3 filter), `divergence_tests.py` lines 11, 27, 203, 217 (writes to `corpus/divergence/`, `checked=True`), `diff_test.py` lines 37-38, 40-45, 167-171 (defaults `--seed 2026`, `--attempts 8`), 176-179 (`--ext` corpora), 188 (`--only` substring on the file name), 233-257 (the four perturbation labels).
- Checked the claim that `IrSource::load` accepts reassignment: `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir.rs` lines 955-989 (`load` is serde parsing plus a version check only) and `ir_vm.rs` lines 202, 297-315 (`memory.insert`, which overwrites).
- Checked the Common problems table: outside a project uv prints `warning: `--group zkir-k` has no effect when used outside of a project` then `ModuleNotFoundError: No module named 'pyk'`; the toolchain script's `FAIL:` strings are at `check_k_toolchain.sh` lines 16 and 47.
- Checked the four `compiled.json` timestamps (15:04:14, 15:04:26, 15:04:40, 15:04:56 on 2026-09-05), consistent with "tens of seconds" per compile; all newer than every `*.k` file (15:00:53).
- Checked the chapter for hygiene: one `#` title, no em-dash, no TODO/TBD/placeholder, no mention of drafting or reviewing; about 1270 words outside code blocks and tables.
- Confirmed the three "Notes for maintainers" items against `divergence_tests.py` lines 175-179, the traceback of `zkir_run.py` without `--ext`, and the `Path.home()` oracle paths at `unit_hash.py` line 27, `diff_test.py` lines 37-38, `divergence_tests.py` line 28.

## Findings

### F1 major "the commitment is the value a `genJob` run records as `needComm`"
Claim: section "Running an extension program with `--ext`" says the reader can obtain the communications commitment from a `genJob` run.
Evidence: `experiments/zkir-k/tools/zkir_run.py` lines 360-364 define only `file`, `preimage`, `--definition`, `--ext`, `--checked`; the `gen` switch exists only as a Python parameter of `Runner.run` (line 298, used at line 301). The only callers with `gen=True` are `diff_test.py` line 120 and `divergence_tests.py` line 211. A developer following this chapter who wants to run `bool_via_neg.zkir` on an input other than `1` has no command to produce a consistent commitment and must guess.
Fix: either state plainly that `zkir_run.py` offers no generation flag and give the one-liner that does it from Python (`Runner(...).run(program, preimage, gen=True)['needs']`, naming the module functions actually used), or point to the tool that exposes it for the reader (`diff_test.py --ext --only bool_via_neg --no-perturb --attempts 1` prints a generated preimage row) and to `06-configuration-and-run-lifecycle.md` for generation mode, and remove the implication that the reader can run `genJob` with the commands shown.

### F2 minor "`-O1` is the LLVM optimisation level of the interpreters already in the tree"
Claim: the four compiled directories were built with `-O1`.
Evidence: no file under `experiments/zkir-k/semantics/zkir-kompiled/` (or the other three) records the optimisation level, and no receipt, wiki page or script in the repository states the kompile command used (the only `-O1` mentions outside this chapter are the generic K manual pages `wiki/k-framework/k-user-manual.md` line 439 and `wiki/k-framework/k-best-practices.md` line 165). The claim cannot be checked by a reader.
Fix: replace with "`-O1` is the optimisation level to use; the LLVM backend does not record it in the output directory" or drop the sentence and keep `-O1` in the commands.

### F3 minor "Ends with `63 programs, 63 as expected, 0 unexpected, 8.6s`"
Claim: the last line of `check_corpus.py` is quoted as a fixed string.
Evidence: my run printed `63 programs, 63 as expected, 0 unexpected, 8.1s`; the receipt `evidence/zkir-k-milestone2-corpus-check-2026-09-05b.txt` ends with `27.6s`. The trailing figure is wall-clock time (`check_corpus.py` line 101 area, `time.time()` difference).
Fix: quote the stable part only, e.g. "Ends with `63 programs, 63 as expected, 0 unexpected` followed by the elapsed time, which varies."

### F4 minor Common problems row "`No module named 'pyk'` inside the repository"
Claim: omitting `--group zkir-k` inside the repository yields `No module named 'pyk'`.
Evidence: after any earlier run with the group, `uv run python -c 'import pyk; print(pyk.__version__)'` from the repository root prints `7.1.337` with no `--group`, because `uv run` does not remove already-installed packages from `.venv` by default. The symptom occurs only on a fresh environment (the clean-checkout case the chapter targets).
Fix: qualify the cause as "on a fresh `.venv` the `zkir-k` group was not requested" so a reader who cannot reproduce the symptom is not misled; keep the fix "always pass `--group zkir-k`".

### F5 minor "kompiles K tutorial lesson 1.2 ... and checks that pyk's version equals `7.1.337`"
Claim: the toolchain script does the listed steps and deletes and rebuilds `lesson-02-a-kompiled`.
Evidence: `experiments/zkir-k/toolchain-check/check_k_toolchain.sh` line 9 also deletes `lesson-02-a-haskell-kompiled`; lines 44-47 also run a pyk KAST round trip through `pyk_roundtrip.py`; line 8 lets `EXPECTED_K_VERSION` override the expected version; line 7 changes directory into `toolchain-check/`, so the script works from any working directory.
Fix: add "and `lesson-02-a-haskell-kompiled`" and "then round-trips a term through pyk (`pyk_roundtrip.py`)"; optionally mention `EXPECTED_K_VERSION`.

### F6 minor Cross-references to chapters that do not yet exist
Claim: the chapter sends the reader to `06-...`, `08-...`, `09-...`, `10-...`, `11-...`, `12-...`, `13-...`.
Evidence: at audit time `experiments/zkir-k/docs/` contains only `01-overview.md` to `05-fields-curves-and-hashes.md`. The file names match the list in the common brief, so this is not a chapter defect; it is a check the editor must repeat when the set is complete, in particular that `06-configuration-and-run-lifecycle.md` covers `genJob` and `<needs>` (its brief line 13 says it does) and that `08-constraints-and-verdicts.md` lists the outcome names (its brief line 11).
Fix: none in the chapter; verify once the target chapters exist.

## Coverage
- Prerequisites (K v7.1.337 via kup/Nix, uv group `zkir-k`, Rust only for rebuilding the oracle; toolchain script and pyproject): covered (F5 is a small omission in the script description).
- Compiling the four definitions (exact commands, duration order of magnitude, output directories; no kompile run): covered.
- Running a first program from `corpus/handmade/` with its manifest preimage, real output pasted and explained (status, memory, pis, skips, verdicts): covered; output verified identical.
- Running with `--checked` and what changes; running an extension program with `--ext`: covered; outputs verified identical (F1 on how the commitment is obtained).
- Reading a failure: one `handmade-negative/` and one `divergence/` program with explanation: covered; outputs verified identical.
- Running the check suites with real summary lines, and `diff_test.py --only`: covered; all four summary lines reproduced (F3 on the timing suffix).
- Common problems and fixes (working directory, missing group, stale kompiled directory, K version): covered (F4 on the missing-group symptom).
