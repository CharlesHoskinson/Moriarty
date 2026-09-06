# Audit of experiments/zkir-k/docs/11-tooling-reference.md (formal methods expert)

Verdict: REVISE
(No blocking findings. Three major findings, all statements about what a tool checks or accepts that a careful reader would take the wrong way: the preimage shapes `zkir_run.py` accepts, which tool validates the Python memory re-encoding, and what the eighteen `unit_hash.py` checks compare. The rest are precision fixes. Every command and output in the chapter that I re-ran reproduced exactly. The developer audit's findings were already applied and are not repeated.)

Checks performed:
- Read all eleven tool sources in `experiments/zkir-k/tools/` in full and compared every statement of the chapter about what a tool builds, runs, compares, prints and exits with against the source, line by line.
- Compared the `Runner.run` status derivation (chapter table) with `zkir_run.py` lines 298 to 352, the need labels with lines 272 to 283 and `zkir-vm.k` lines 91 to 94, the JSON keys with lines 325 to 351, and `preimage_term` / `fr` with lines 61 to 82.
- Compared the preimage table with `zkir-vm.k` lines 35 to 42 (`preimage(List, Int, CommOpt, List, List, List)`, `noComm`, `comm`, `job`, `checkedJob`, `genJob`), the `#seedPi` rules (lines 143 to 159), the `#wfGate` message prefix `well-formedness: ` (line 101), and the `#finish` rules in generation mode (lines 525 to 530).
- Checked the status constructors `ok()`, `error(String)`, `panic(String)` (`zkir-vm.k` line 54), `skipNone` / `skipSome` (line 55), the gate constructors `gate`, `guardGate`, `bindGate`, `commGate`, `outputGate` and the outcomes (`zkir-constraints.k` lines 34 to 43), and the `commGate` evaluation against `Pi[I]` (lines 101 to 107).
- Checked `zkir-check.k` (`$PGM:Program`, `imports ZKIR-EXT-SYNTAX`, `P:Program => wf(P)`), `zkir-test.k` (`$PGM:KItem`), `mainModule.txt` and `backend.txt` of the four kompiled directories, and the `native()` production with `symbol(Native)` (`zkir-syntax.k` line 31) that explains `LblNative{}()` in KORE against `native ( )` in pretty text.
- Read pyk 7.1.337 sources: `KRun.__init__` (command defaults to `krun`), `KRun.run_process` (temporary file, `_krun` with `output=KORE`, `parser='cat'`, `depth`), `_build_arg_list` (`--depth` only when `depth is not None`), `KPrint.definition` (`compiled.json`), `KPrint.kast_to_kore`.
- Ran `zkir_kast.py kore` on `native_identity.zkir` (output identical to the chapter's block, exit 0).
- Wrote the chapter's complete preimage to a scratch file and ran `zkir_run.py` on `native_identity.zkir` plain, with `--gen` on `["0", "9"]`, and with `--depth 1`; status, `pis`, `constraints`, `needs`, all three gate texts, `violations`, `outputs` and the residual `<k>` match the chapter.
- Ran `zkir_run.py` with a three-element `communications_commitment`, with `null`, with the string `"19"`, and with the input string `"0x7"`, to test the chapter's exit-code statements.
- Counted `check(` calls in `unit_values.py` (43) and `unit_hash.py` (18), the `ERROR_CLASSES` entries in `diff_test.py` (21), the `CASES` entries in `divergence_tests.py` (20), the programs of `gen_handmade.py` (7), and the corpus files (43, 61, 6, 9, 7, 20, 3 + 4).
- Read `repos/_build/ledger-92e8bdd3/zkir-oracle/src/main.rs` for the oracle's exit paths, its `communications_commitment: Option<(String, String)>`, and its `type` field (`format!("{:?}", v.get_type())`).
- Checked the tail of `evidence/zkir-k-unit-values-2026-09-05b.txt` (`42/42 checks passed`, last check `dec native out of field`) and the divergence receipt (`gate=less_than:...` rows).
- Hard rules: one `#` title, no em-dash, no placeholder, no mention of drafting.

## Findings

### F1 major `## zkir_run.py` ("a `communications_commitment` that is not a two-element array ... is an uncaught Python exception: traceback on stderr, exit 1") and `## Preimage JSON` table row `communications_commitment` ("omitted, `noComm()`")
Claim: any `communications_commitment` that is not a two-element array makes `zkir_run.py` exit 1 with a traceback; `noComm()` is produced only when the key is omitted.
Evidence: `zkir_run.py` lines 73 to 74: `cc = p.get('communications_commitment')`, `noComm()` if `cc is None`, else `comm(fr(cc[0]), fr(cc[1]))`. Only the first two elements are read and any indexable value is accepted. Runs on `native_identity.zkir`: with `["4798...6386", "9", "999"]` the result is `status` `ok`, exit 0 (the third element is ignored); with the string `"19"` the term is `comm(1, 9)` and the result is `status` `error`, `Communications commitment mismatch`, exit 0; with `null` the term is `noComm()` and the result is `status` `error`, `Expected communications commitment`, exit 0. Only an array of fewer than two elements (`IndexError`) or a non-indexable value such as a number (`TypeError`) is a traceback. The oracle (`zkir-oracle/src/main.rs` line 24, `Option<(String, String)>`) rejects the three-element array, so on this shape the K tool and the oracle disagree before any semantics run.
Fix: in the exit-code paragraph replace "a `communications_commitment` that is not a two-element array" with "a `communications_commitment` with fewer than two elements or that is not indexable (a JSON number)". Add after the table: "`preimage_term` reads only `cc[0]` and `cc[1]`: a longer array is accepted with its extra elements ignored, a string of two or more characters is read as two one-character integers, and JSON `null` is `noComm()` like an omitted key. The oracle accepts exactly a two-element array." In the table row, change "omitted, `noComm()`" to "omitted or `null`, `noComm()`".

### F2 major `## zkir_run.py`, memory records ("`variant` and `encoded` from `encode_value` in `zkir_run.py` (matching `encodeValue` in `zkir-values.k`, checked by `unit_values.py`)")
Claim: `unit_values.py` checks that the Python `encode_value` of `zkir_run.py` matches the K function `encodeValue`.
Evidence: `unit_values.py` imports only pyk (lines 15 to 18); it never imports or calls `zkir_run.encode_value`. Its encoding checks (lines 200 to 221) compare K's `encodeValue` with the file's own copy of `enc_foreign` (lines 88 to 98) for seven values (`secp256k1BaseV` twice, `curve25519ScalarV`, `secp256k1PointV` twice, `curve25519PointV`, `bytes32V`); no check covers `jubjubPointV`, `jubjubScalarV`, the secp256r1 constructors, `curve25519BaseV`, `boolV`, `byteV` or `bytesV`. The Python `encode_value` used for memory records is exercised only by `diff_test.py` (`compare`, lines 152 to 161, `encoded` and `type` against the oracle's registers on joint `ok` runs) and by `divergence_tests.py` (lines 238 to 243, `type` and `encoded`). No tool compares `variant` with anything.
Fix: "`variant` and `encoded` from `encode_value` in `zkir_run.py`, a Python re-encoding of the register value with the same formulas as `encodeValue` in `zkir-values.k` (`unit_values.py` checks `encodeValue` against an inline copy of those formulas; `encode_value` itself is checked only through `diff_test.py` and `divergence_tests.py`, which compare `type` and `encoded` with the oracle on joint `ok` runs; `variant` is compared nowhere)".

### F3 major `## unit_hash.py` ("Eighteen known-answer checks of `ZKIR-HASH` ... the K side evaluates ... and compares with those registers and with hashlib")
Claim: all eighteen checks evaluate `ZKIR-HASH` functions, and the comparisons are against oracle registers and hashlib.
Evidence: `unit_hash.py` line 116, `sha256 bytes(32) alignment vs hashlib`, compares the oracle's `%s32` register with `hashlib.sha256(pre32)` and runs no K term; it checks the Python model of the alignment layer, not the definition. The three `alignedBytes` checks (lines 118, 124, 133) compare K's `alignedBytes` with byte strings constructed in the file (`pre32`, `pref`, `pre200`), not with a register or with hashlib. Seventeen of the eighteen checks run a K term.
Fix: "Eighteen known-answer checks: seventeen evaluate a K term in `ZKIR-TEST` (`poseidonHash`, `hashToCurve`, `encodeValue(bytes32V(sha256Bytes(...)))`, `encodeValue(bytes32V(keccak256Bytes(...)))`, `alignedBytes`) and one (`sha256 bytes(32) alignment vs hashlib`) compares an oracle register with `hashlib` alone. The hash checks compare with the oracle's registers (`%h0` to `%h3`, `%p0`, `%p1`, `%p3`, `%s1`, `%k1`, `%s32`, `%k32`, `%sf`, `%k200`) and, for one-byte sha256, with `hashlib`; the three `alignedBytes` checks compare with byte strings built in the file from the same inputs."

### F4 minor `## unit_values.py` ("compares the pretty result with an independent Python formula in the same file")
Claim: every check's expectation is computed by a Python formula.
Evidence: sixteen checks have literal expectations (`'true'`, `'false'`, `'inf ( )'`, `'noSqrt ( )'`, and the `decErr` / `decPanic` message strings at lines 216, 218, 223, 224, 225); `fsqrt 4` (line 169) first runs K and accepts either square root (`sqrtOk ( 2 )` or `sqrtOk ( r - 2 )`).
Fix: "compares the pretty result with an expected value: a Python formula in the same file (field inverse, Legendre symbol, curve arithmetic, encodings) or a literal (curve membership, subgroup membership, `inf ( )`, `noSqrt ( )`, the `decErr` / `decPanic` messages); `fsqrt 4` accepts either root."

### F5 minor `## diff_test.py` ("The footer counts successful-run and error-run agreements")
Claim: the two footer counts are counts of agreements.
Evidence: `diff_test.py` lines 269 to 270 count rows whose summary starts with `K=ok Rust=ok`, `K=error Rust=error` or `K=panic Rust=panic`, regardless of `diffs`; a row with both statuses `ok` and differing registers is counted as a "successful-run agreement" and as a disagreement. Only `{len(rows) - failures} agree, {failures} disagree` counts full agreement.
Fix: "The footer counts rows whose statuses agree (`ok`/`ok`; `error`/`error` or `panic`/`panic`), which is status agreement only, then the totals that agree and disagree under `compare`, then `oracle 2: ...`."

### F6 minor `## diff_test.py` ("on `ok`, matching `pis`, `pi_skips`, and every register's `type` and `encoded`")
Claim: the register comparison is per register.
Evidence: `compare` (lines 153 to 157) iterates over the union of register names and reports `missing in K` / `missing in Rust`; `variant`, `cursors`, `constraints` and the verdicts are not compared.
Fix: "on `ok`, matching `pis`, `pi_skips`, the same set of register names, and every register's `type` and `encoded` (`variant`, `cursors`, `constraints` and the verdicts are not compared)".

### F7 minor `## divergence_tests.py` ("The script writes `corpus/divergence/<name>.zkir` and a preimage `{inputs: ..., binding_input: "42"}`" and "the footer is `N/20 divergence cases behave as expected`")
Claim: a preimage file is written next to the program; `N` is the number of passing cases.
Evidence: the preimage is a dict (line 204) passed to `Runner.run` directly and to the oracle through a `NamedTemporaryFile` that is deleted on close (lines 186 to 189); only the program is written under `corpus/divergence/`. `failures` is incremented once for a status or outcome mismatch (line 236) and again for `MEMORY DIFFERS` (line 242), so `N = 20 - failures` can be lower than the number of cases printed `PASS`.
Fix: "writes `corpus/divergence/<name>.zkir` and builds the preimage `{inputs: raw as strings, binding_input: "42"}` in memory (the oracle receives it through a deleted temporary file)"; and "the footer is `N/20 divergence cases behave as expected`, where a memory difference on a joint `ok` counts as a second failure of the same case".

### F8 minor `## divergence_tests.py` (gate selection: "`assert` matches `gate ( assert ( ... ) )`")
Claim: implicit; the chapter never says what `gate_sub` is or why `norm` strips underscores.
Evidence: `gate_sub` is the ZKIR op name (`less_than`, `public_input`, `from_coordinates`, ...) or `commGate` / `guardGate`; K prints the production name (`lessThan`, `publicInput`; `zkir-syntax.k` lines 115, 117), so `norm` (line 224) removes underscores and case to make `less_than` match `gate ( lessThan ( ... ) )`. The receipt rows read `gate=less_than:unknown`.
Fix: add "`gate_sub` is the ZKIR op name as in the JSON (`less_than`), or `commGate` / `guardGate`; `norm` drops underscores, spaces and case so that it matches the K production name in the pretty gate text (`gate ( lessThan ( ... ) )`)."

### F9 minor `## zkir_run.py`, memory records ("`type` from `type_string` (the crate's `IrType` display)")
Claim: `type` is the Display rendering of `IrType`.
Evidence: `zkir_run.py` line 202 docstring: "The Rust `{:?}` of `IrType`"; `zkir-oracle/src/main.rs` line 102: `format!("{:?}", v.get_type())`. In Rust, `Display` and `Debug` are different traits.
Fix: "the crate's `{:?}` (Debug) rendering of `IrType`".

### F10 minor `## Preimage JSON` ("Decimal strings and JSON numbers both parse")
Claim: implicit that other strings are `preimage error`.
Evidence: `fr` (line 62) calls `int(v)`; a non-decimal string such as `"0x7"` raises `ValueError` before `PreimageError` can be raised. Run: `Traceback ... ValueError: invalid literal for int() with base 10: '0x7'`, exit 1.
Fix: add "a non-decimal string (for example hex) is a `ValueError` traceback, exit 1, not a `preimage error`".

### F11 minor `## zkir_run.py`, flag `--depth N` ("an unfinished configuration is `depth-exhausted`")
Claim: implicit that `depth-exhausted` means the bound was reached.
Evidence: `zkir_run.py` line 315 selects `depth-exhausted` whenever `<k>` is non-empty and a depth was given; a run that is stuck before the bound (no rule applies) is also reported `depth-exhausted`, and `stuck` is reported only without `--depth`. The status table states the condition correctly.
Fix: in the flag row: "an unfinished configuration is `depth-exhausted`, whether the bound was hit or no rule applied; `stuck` is reported only without `--depth`".

### F12 minor `## zkir_kast.py` ("`check` prints `wfOk` when the pretty term contains `wfOk ( )`, or the `wfError ( ... )` span")
Claim: two outcomes.
Evidence: `wf_result` (`zkir_kast.py` lines 381 to 388) has a third outcome, `unexpected: ` plus the first 200 characters of the pretty term, when neither pattern is found; the `wfError` span ends at the first `)` after `wfError (`.
Fix: "or the `wfError ( ... )` span up to the first closing parenthesis, or `unexpected: ` plus 200 characters of the term when neither appears".

## Coverage
- One section per tool with purpose, invocation, flags, inputs, outputs, exit codes and dependency; `run()` API (labels, depth, status derivation, memory records): covered; imprecise on accepted preimage shapes (F1), on what validates the memory re-encoding (F2), and on the `type` rendering (F9).
- The preimage JSON format precisely, with a complete example: covered; the example reproduces (`ok`, `outputs` `["Native:7"]`, `pis` `["42", "4798...6386"]`, exit 0); shape and parse edge cases imprecise (F1, F10).
- How pyk is used (KRun/krun, definition directories, kore vs kast, `--emit-json`): covered and confirmed against the pyk 7.1.337 sources.
- The corpus directories and their manifests: covered; counts confirmed.
- Running everything in the right order after changing the semantics: covered.
- `diff_test.py` protocol summary: covered; footer semantics and comparison scope imprecise (F5, F6).
- `divergence_tests.py` case structure, exact gate selection, memory comparison: covered; preimage wording, failure counting and `gate_sub` meaning imprecise (F7, F8).
- `unit_values.py`, `unit_hash.py`, `check_corpus.py`: covered; what `unit_hash.py` compares is overstated (F3), `unit_values.py` expectations overstated (F4); `check_corpus.py` matches the source line by line.
