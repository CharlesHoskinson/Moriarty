# Audit of experiments/zkir-k/docs/11-tooling-reference.md (developer)

Verdict: REVISE
(Two blocking findings, one wrong number and one command that fails; one major usability finding; the rest are small precision fixes. The chapter is otherwise accurate and every other shown command and output was reproduced.)

Checks performed:
- Read all eleven tool sources in `experiments/zkir-k/tools/` and compared every flag, default, message, exit code and output format named in the chapter against the argparse declarations, `return` codes and `print` calls.
- Ran `--help` on all eleven tools from the repository root (`uv run --group zkir-k python experiments/zkir-k/tools/<tool>.py --help`).
- Ran `zkir_kast.py kore|check|kast` on `native_identity.zkir`, `undefined_variable.zkir`, `bool_via_neg.zkir` (with and without `--ext`), `native_bytes.zkir --ext`, `wrong_version.zkir`; compared the KORE text, the `wfOk` / `wfError ( ... )` lines, the `format error:` messages and the exit codes with the chapter.
- Wrote the chapter's complete preimage JSON to a scratch file and ran `zkir_run.py` on `native_identity.zkir` plain, with `--gen` (commitment `0`, opening `9`), with `--depth 1`, with a preimage lacking `communications_commitment` (plain and `--gen`), with an out-of-range integer, with numeric instead of string integers, with `wrong_version.zkir`, and `--checked` on `undefined_variable.zkir`; compared status, `outputs`, `pis`, `violations`, `needs`, residual `<k>` text and exit codes.
- Ran `diff_test.py --only native_identity --no-perturb --attempts 1`, `diff_test.py --only native_identity`, `diff_test.py --ext --only bool_via_neg --attempts 1`; checked row and footer formats.
- Ran `check_corpus.py` in full (63 programs, 63 as expected, exit 0), `unit_values.py` in full, `unit_hash.py` in full, `divergence_tests.py` in full (20/20, exit 0), `gen_handmade.py`, `extract_test_inputs.py` (all idempotent: `git status` of `corpus/` clean afterwards).
- Ran `gen_constants.py` with the chapter's argument (fails) and with the `src/` subdirectory (output identical to `semantics/zkir-constants.k`).
- Checked the K symbols the chapter names against `zkir-vm.k` (`preimage`, `noComm`, `comm`, `job`, `checkedJob`, `genJob`, `Need` constructors, `skipNone`/`skipSome`, `#seedPi`, `#wfGate`), `zkir-check.k` (`P:Program => wf(P)`, `$PGM:Program`), `zkir-test.k` (`$PGM:KItem`), `zkir-constraints.k` (`gate`, `guardGate`, `bindGate`, `commGate`, `outputGate`, `verdict`).
- Checked `mainModule.txt` and `backend.txt` of the four kompiled directories, the corpus file counts (43, 61, 6, 9, 7, 20, 3, 4), the manifest keys, the 13 `test_preimage` entries, `pyproject.toml` (`kframework==7.1.337`, `requires-python = ">=3.13"`), the oracle binaries and the midnight-circuits 7.2.4 registry path.
- Checked pyk's `KRun.run_process` source for what it executes.
- Checked cross-referenced chapter files exist (02, 03, 06, 12, 13) and that `02-getting-started.md` carries the kompile commands with `--output-definition`.
- Hard rules: one `#` title, no em-dash, no TODO/TBD/placeholder, no mention of drafting.

## Findings

### F1 blocking `## unit_values.py` ("Forty-two checks", "42/42 checks passed")
Claim: `unit_values.py` performs forty-two checks and its footer is `42/42 checks passed`.
Evidence: `experiments/zkir-k/tools/unit_values.py` has 43 top-level `check(` calls (lines 168 to 225; the last, line 225, is `dec bytes32 bad high, strict (2ffe2d1 decode_bytes returns None)`, added in the uncommitted working-tree edit shown by `git diff --stat HEAD`). Running the tool prints `43/43 checks passed`.
Fix: replace "Forty-two checks" with "Forty-three checks" and `42/42 checks passed` with `43/43 checks passed`. Also the check list may mention `decodeStrict` (line 225) and `legendre`, `inSubgroup` among the function names.

### F2 blocking `## gen_constants.py` (the shown command and "Argument 1 is the crate source directory")
Claim: the command
```
uv run --group zkir-k python experiments/zkir-k/tools/gen_constants.py \
  ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4 \
  > experiments/zkir-k/semantics/zkir-constants.k
```
regenerates `zkir-constants.k`.
Evidence: `gen_constants.py` line 25 reads `src / 'hash/poseidon/constants/blstrs.rs'` and line 34 reads `src / 'ecc/hash_to_curve/mtc_params.rs'`; these files live under `midnight-circuits-7.2.4/src/`, not under the crate root. With the chapter's argument the script exits 1 with `FileNotFoundError: ... midnight-circuits-7.2.4/hash/poseidon/constants/blstrs.rs`. As written, the redirect would also have truncated `zkir-constants.k` to an empty file before the failure. With `.../midnight-circuits-7.2.4/src` it exits 0 and the output is byte-identical to the current `semantics/zkir-constants.k` (233 lines).
Fix: append `/src` to the path argument in the command block and change "Argument 1 is the crate source directory" to "Argument 1 is the crate's `src/` directory (the one containing `hash/` and `ecc/`)". Consider advising to write to a temporary file first and move it over `zkir-constants.k` after a successful exit, since the redirect truncates the target before the script runs.

### F3 major `## divergence_tests.py`, `## unit_values.py`, `## unit_hash.py`, `## gen_handmade.py`, `## extract_test_inputs.py` ("No flags")
Claim: each of these tools has "No flags".
Evidence: none of the five has an `argparse` parser or reads `sys.argv`; any argument, including `--help`, is ignored and the tool runs at once. Running `--help` on them executed the full tool: `divergence_tests.py` rewrote `corpus/divergence/*.zkir`, `gen_handmade.py` rewrote the seven handmade programs and `corpus/handmade/manifest.json`, `extract_test_inputs.py` rewrote `corpus/ledger9-92e8bdd3-tests/manifest.json` (all idempotent here, `git status` stayed clean, but a reader with a stale ledger checkout would have a changed manifest). A developer who reads "No flags" and types `--help` to confirm gets a multi-second run with file writes and no usage text.
Fix: in each of the five sections replace "No flags." with "No argument parsing: every argument, including `--help`, is ignored and the tool runs immediately" and, for the three writers, add "(and writes its files)".

### F4 minor `## zkir_run.py`, the `--gen` example block ("prints:")
Claim: the JSON block after "A `--gen` run on `native_identity.zkir` ... prints:" is the tool's output.
Evidence: `main` prints `json.dumps(result, indent=1)` (`zkir_run.py` line 381), which puts every array element and object member on its own line; the real output has `"memory": {\n  "%v_0": {\n   "variant": "Native",\n   "type": "Native",\n   "encoded": [\n    "7"\n   ]\n  }\n }` and `"needs": [\n  [\n   "comm",\n   4798...6386\n  ]\n ]`. The chapter's block condenses these onto single lines. All values match the real run.
Fix: either paste the real `indent=1` layout or change "prints:" to "prints (whitespace condensed here):".

### F5 minor `## Talking to K through pyk` ("`run_process(kore, depth=...)` runs the `interpreter` binary")
Claim: `KRun.run_process` runs the interpreter binary.
Evidence: pyk `ktool/krun.py` `run_process` (line 66 onward) writes the KORE to a temp file and calls `_krun(command=self.command, input_file=..., definition_dir=..., depth=...)`, where `self.command` defaults to `'krun'`. It is the `krun` command-line tool that in turn executes the definition's interpreter. A developer debugging a failure needs `krun` on `PATH`, not only the interpreter.
Fix: "`run_process(kore, depth=...)` invokes the `krun` command on the definition directory (which runs the compiled interpreter) with `--depth` when given."

### F6 minor `## Talking to K through pyk` ("`--ext` ... selects `zkir-ext-kompiled` only when `--definition` is omitted")
Claim: read with the preceding sentence (which names `zkir_kast.py`, `zkir_run.py` and `check_corpus.py`), `--ext` switches the definition to `zkir-ext-kompiled`.
Evidence: only `zkir_run.py` (`Runner.__init__`, line 296) and `diff_test.py` (`Runner(ext=args.ext)`, line 181) do that. `zkir_kast.py check --ext` keeps `zkir-check-kompiled` (`main`, line 407, `default_definition()`); this works because `zkir-check.k` line 10 imports `ZKIR-EXT-SYNTAX`. Verified: `zkir_kast.py check --ext corpus/midnight-zkir-2ffe2d1-tests/bool_via_neg.zkir` prints `wfOk`, exit 0. The chapter never states that `check --ext` is supported, which is the first thing an extension developer asks.
Fix: "`--ext` selects the extension surface in the preprocessor and in `encode_value` / `type_string`. On `zkir_run.py` and `diff_test.py` it also selects `zkir-ext-kompiled` unless `--definition` is given. `zkir_kast.py check --ext` stays on `zkir-check-kompiled`, whose `ZKIR-CHECK` module imports `ZKIR-EXT-SYNTAX` (`zkir-check.k`), so extension programs are checked there: `bool_via_neg.zkir` prints `wfOk`."

### F7 minor `## Corpus directories`, table row `corpus/midnight-zkir-2ffe2d1-precompiles/`
Claim: the precompiles manifest has `commit`, `repo`, `branch` `zkir-v3`.
Evidence: `corpus/midnight-zkir-2ffe2d1-precompiles/manifest.json` also has `programs[]` with six entries, each `{file, source, ops}` (for example `micro-dao__advance.zkir`, source `zkir-precompiles/micro-dao/advance.zkir`). The following paragraph ("A ledger or midnight-zkir `programs[]` entry has `file`, `source`, and (for the test extracts) `test_fn`, `line` and `ops`") implies `ops` is only on test extracts; precompile entries carry `ops` but no `test_fn` or `line`.
Fix: manifest cell: "`commit`, `repo`, `branch` `zkir-v3`, `programs[]`"; paragraph: "A ledger or midnight-zkir `programs[]` entry has `file`, `source` and `ops`, and for the test extracts also `test_fn` and `line`."

### F8 minor `## gen_constants.py` and `## Running the suite after a semantics change` ("rekompile every definition that imports `ZKIR-CONSTANTS`")
Claim: the reader must work out which definitions import `ZKIR-CONSTANTS`.
Evidence: `ZKIR-CONSTANTS` is imported directly by `zkir-hash.k` and `zkir-test.k`; through `ZKIR-HASH` it reaches `ZKIR` (`zkir-kompiled`) and `ZKIR-EXT` (`zkir-ext-kompiled`), and `ZKIR-TEST` (`zkir-test-kompiled`). `zkir-check-kompiled/definition.kore` contains no `poseidonRC` (0 matches), so `ZKIR-CHECK` does not need a rebuild.
Fix: "rekompile `zkir-kompiled`, `zkir-ext-kompiled` and `zkir-test-kompiled` (`ZKIR-CONSTANTS` is imported by `zkir-hash.k` and `zkir-test.k`; `ZKIR-CHECK` does not include it)."

### F9 minor `## zkir_run.py` ("Exit 2 is only `format error: ...` or `preimage error: ...`")
Claim: the exit-code story is 0 for any reported status, 2 for the two error prefixes.
Evidence: `main` (lines 358 to 382) opens the preimage file and constructs `Runner` before loading the program; a missing or unreadable preimage file, a missing program file, a `communications_commitment` that is not a two-element array, or a missing kompiled directory all surface as an uncaught Python exception (traceback on stderr, exit 1). The main text does not tell the reader that anything else is a traceback.
Fix: add after the exit-2 sentence: "Any other failure (missing program or preimage file, a `communications_commitment` that is not a two-element array, a missing kompiled directory) is an uncaught Python exception: traceback on stderr, exit 1."

### F10 minor `## diff_test.py`, flag table row `--no-perturb` ("skip the four perturbation variants")
Claim: four perturbation variants follow a joint success.
Evidence: `diff_test.py` lines 234 to 257 build `perturbed-raw` and `perturbed-typed` only when `pre['inputs']` is non-empty, `wrong-pubin` only when `public_transcript_inputs` is non-empty, `wrong-comm` only when a commitment is present. `diff_test.py --only native_identity` produced three variants (`perturbed-raw`, `perturbed-typed`, `wrong-comm`), not four.
Fix: "skip the up-to-four perturbation variants (`perturbed-raw`, `perturbed-typed`, `wrong-pubin`, `wrong-comm`) after a joint success; each is built only when the preimage has the field it perturbs".

### F11 minor `## unit_hash.py` ("It runs `corpus/handmade/transient_hash.zkir` and `std_hashes.zkir` on inputs ...")
Claim: the tool runs the two programs.
Evidence: `unit_hash.py` lines 300 to 304 run them only through the oracle (`subprocess.run([ORACLE, program, preimage])`) to obtain the expected register values; the K side evaluates function terms (`poseidonHash`, `hashToCurve`, `sha256Bytes`, `keccak256Bytes`, `alignedBytes`) on `zkir-test-kompiled`, never the programs. A reader may assume the programs go through `ZKIR-VM`.
Fix: "It runs the two programs through the oracle on inputs ..., then evaluates `poseidonHash`, ... in `ZKIR-TEST` and compares with the oracle's registers and with hashlib."

## Coverage
- One section per tool (purpose, invocation, flags, inputs, outputs, exit codes, dependency): covered for all eleven tools; `zkir_run.py` `run()` API (labels, depth, status derivation, memory records) covered. Numbers wrong in `unit_values.py` (F1) and the `gen_constants.py` command fails (F2).
- Preimage JSON format with a complete example: covered; the example is accepted by `zkir_run.py` and produces the stated `status`, `outputs`, `pis`, empty `violations`, exit 0; the `--gen` and `--depth 1` runs match (layout aside, F4).
- How pyk is used (KRun/krun, definition directories, kore vs kast, `--emit-json`): covered; one imprecision on what `run_process` executes (F5) and on `--ext` versus definition selection (F6).
- Corpus directories and their manifests: covered; precompiles `programs[]` omitted (F7).
- Running everything in the right order after changing the semantics: covered; the set of definitions to rekompile after `gen_constants.py` is left to the reader (F8).
