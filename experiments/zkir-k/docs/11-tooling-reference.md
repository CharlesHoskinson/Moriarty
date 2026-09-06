# Tooling reference

The Python tools for the executable K definition of ZKIR (Zero-Knowledge Intermediate Representation) v3 live in `experiments/zkir-k/tools/`. Invoke them from the repository root through the `zkir-k` dependency group of `pyproject.toml` (`kframework==7.1.337`, `requires-python = ">=3.13"`):

```
uv run --group zkir-k python experiments/zkir-k/tools/<tool>.py ...
```

They load a precompiled LLVM (Low Level Virtual Machine) definition under `experiments/zkir-k/semantics/*-kompiled/` and never recompile, so rebuild as in `02-getting-started.md` after a `.k` edit. The run lifecycle is in `06-configuration-and-run-lifecycle.md` and oracle comparison in `12-oracles-and-differential-testing.md`.

## Talking to K through pyk

The tools build a pyk `KInner` and hand it to `pyk.ktool.krun.KRun`. `KRun(definition)` reads `compiled.json`, so every `kompile` must pass `--emit-json`. The four directories were built with `--backend llvm --emit-json -O1`, and each `backend.txt` is `llvm`.

| Directory | `mainModule.txt` | `$PGM` sort | Callers |
|---|---|---|---|
| `semantics/zkir-kompiled/` | `ZKIR` | `Job` | `zkir_run.py`, `diff_test.py`, `divergence_tests.py` |
| `semantics/zkir-ext-kompiled/` | `ZKIR-EXT` | `Job` | `zkir_run.py --ext`, `diff_test.py --ext` |
| `semantics/zkir-check-kompiled/` | `ZKIR-CHECK` | `Program` | `zkir_kast.py check` and `kore`, `check_corpus.py` |
| `semantics/zkir-test-kompiled/` | `ZKIR-TEST` | `KItem` | `unit_values.py`, `unit_hash.py` |

`--definition DIR` on `zkir_kast.py`, `zkir_run.py` and `check_corpus.py` replaces the default directory. `--ext` selects the midnight-zkir 2ffe2d1 surface in the preprocessor and in `encode_value` / `type_string`; on `zkir_run.py` and `diff_test.py` it also selects `zkir-ext-kompiled` unless `--definition` is given. `zkir_kast.py check --ext` stays on `zkir-check-kompiled`: `ZKIR-CHECK` (`zkir-check.k`) imports `ZKIR-EXT-SYNTAX`, so `corpus/midnight-zkir-2ffe2d1-tests/bool_via_neg.zkir` prints `wfOk` there.

`KRun.kast_to_kore(term, KSort(...))` produces KORE (K Object Representation). `run_process(kore, depth=...)` writes it to a temporary file and invokes the `krun` command on the definition directory, with `--depth` when given; `krun` runs the compiled `interpreter` and must be on `PATH`. `stdout` is the final configuration as KORE. `KoreParser(stdout).pattern()` then `kore_to_kast` recovers a `KInner`, and `pretty_print` renders `<k>`, gates and well-formedness results. `unit_values.py` and `unit_hash.py` use sort `KItem` and take the pretty text between `<k>` and `~> .K` as the function result.

The `kore` command of `zkir_kast.py` prints `kore.text`; for `corpus/ledger9-92e8bdd3-tests/native_identity.zkir` that is:

```
Lblprogram{}(\dv{SortInt{}}("0"), LbltypedIds{}(LbltypedId{}(\dv{SortString{}}("%v_0"), LblNative{}()), Lbl'Stop'typedIds{}()), LblirTypes{}(LblNative{}(), Lbl'Stop'irTypes{}()), \dv{SortBool{}}("true"), Lblinstrs{}(Lbloutput{}(Lbloperands{}(Lblvar{}(\dv{SortString{}}("%v_0")), Lbl'Stop'operands{}())), Lbl'Stop'instrs{}()))
```

The `kast` command never calls `krun`: it prints `KInner.to_dict()` as JSON (JavaScript Object Notation), a tree of `KApply` and `KToken` nodes.

## Preimage JSON

The function `preimage_term` in `zkir_run.py` builds the `preimage` constructor of `zkir-vm.k` (`ZKIR-VM-SYNTAX`). Every integer goes through `fr`, which requires `0 <= n < r` with `r` the BLS12-381 scalar modulus `52435875175126190479447740508185965837690552500527637822603658699938581184513`. Otherwise the tool exits 2 with `preimage error: <n> is not a canonical field element` on stderr. `fr` calls `int(v)`, so decimal strings and JSON numbers both parse; the harnesses write strings. A non-decimal string such as `"0x7"` is a `ValueError` traceback, exit 1, not a `preimage error`.

| JSON key | Default | K argument of `preimage` |
|---|---|---|
| `inputs` | `[]` | first `List` |
| `binding_input` | `0` | `Int` |
| `communications_commitment` | omitted or `null`, `noComm()` | `comm(commitment, opening)` from `cc[0]` and `cc[1]` |
| `private_transcript` | `[]` | fourth `List` |
| `public_transcript_inputs` | `[]` | fifth `List` |
| `public_transcript_outputs` | `[]` | sixth `List` |

`preimage_term` reads only `cc[0]` and `cc[1]` of the value, so a longer array is accepted and its extra elements are ignored. A string of two or more characters is read as two one-character integers (`"19"` becomes `comm(1, 9)`), and JSON `null` is `noComm()` like an omitted key. The oracle accepts exactly a two-element array, so a longer array is rejected there and accepted here.

A program with `do_communications_commitment` true requires the pair at `#seedPi`, even under `--gen`. Without it the run reports `error` with `Expected communications commitment`. Supply `["0", opening]`, read the `comm` entry of `needs`, then rerun without `--gen`. This complete preimage for `native_identity.zkir` uses the commitment `--gen` recorded with opening `9`:

```json
{
  "inputs": ["7"],
  "binding_input": "42",
  "communications_commitment": [
    "47986959901854722077236245854877419366049068979521798207792096482323592476386",
    "9"
  ],
  "private_transcript": [],
  "public_transcript_inputs": [],
  "public_transcript_outputs": []
}
```

On that file the runner reports `status` `ok`, `outputs` `["Native:7"]`, `pis` `["42", "47986959901854722077236245854877419366049068979521798207792096482323592476386"]`, empty `violations`, and exits 0.

## Corpus directories

Programs live under `experiments/zkir-k/corpus/` except the two Moriarty Compact artifact sets. Each `*.zkir` file is a version-3 JSON artifact; `check_corpus.py` and `diff_test.py` skip a file whose `version.major` is not 3.

| Directory | `*.zkir` files | Manifest | Walked by |
|---|---|---|---|
| `corpus/ledger9-92e8bdd3-tests/` | 43 | `commit` `92e8bdd3`, `repo`, `programs[]` | `check_corpus.py`, `diff_test.py` |
| `corpus/midnight-zkir-2ffe2d1-tests/` | 61 | `commit` `2ffe2d1`, `repo`, `programs[]` | `diff_test.py --ext` |
| `corpus/midnight-zkir-2ffe2d1-precompiles/` | 6 | `commit`, `repo`, `branch` `zkir-v3`, `programs[]` | `check_corpus.py`, `diff_test.py`, `diff_test.py --ext` |
| `corpus/handmade/` | 9 | `note`, `programs[]` | `diff_test.py` (both surfaces); `unit_hash.py` uses `transient_hash.zkir` and `std_hashes.zkir` |
| `corpus/handmade-negative/` | 7 | none | `check_corpus.py` |
| `corpus/divergence/` | 20 | none | written and read by `divergence_tests.py` |
| `experiments/moriarty-compact-escrow/output/zkir/` | 3 | none | `check_corpus.py`, `diff_test.py` |
| `experiments/moriarty-core-swap/output/zkir/` | 4 | none | `check_corpus.py`, `diff_test.py` |

A ledger, midnight-zkir or precompile `programs[]` entry has `file`, `source` and `ops`; the test extracts also carry `test_fn` and `line`, and 13 of the 43 ledger entries carry `test_preimage`. A handmade entry has `file`, optional `ops`, and `test_preimage.inputs`. `diff_test.py` seeds attempt 0 from `test_preimage` when listed.

## `zkir_kast.py`

`zkir_kast.py` is the preprocessor from a `.zkir` JSON file to the `program` term of `zkir-syntax.k`. It mirrors the serde layer of `ir.rs` at midnight-ledger 92e8bdd3: variables start with `%`, immediates are little-endian hex with an even number of digits and a value below `r`, `guard: null` is `noGuard()`, unknown object members are ignored, and a missing required field is `ZkirFormatError`. The program model is in `03-program-model.md`.

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_kast.py {kore|kast|check} FILE.zkir [--definition DIR] [--ext]
```

| Argument | Effect |
|---|---|
| `kore` | print the `Program` term as KORE text (uses `KRun`) |
| `kast` | print `term.to_dict()` as JSON (no `krun`) |
| `check` | run `ZKIR-CHECK` (`zkir-check.k`: `P:Program => wf(P)`) and print `wf_result` |
| `--definition DIR` | kompiled directory for `kore` and `check`; default `semantics/zkir-check-kompiled` |
| `--ext` | accept the midnight-zkir 2ffe2d1 surface: types `Bool`, `Byte`, `Bytes<n>` with `1 <= n <= 2^24`, and the nine extra instructions; reject `reverse_bytes` |

`check` prints `wfOk` when the pretty term contains `wfOk ( )`, or the `wfError ( ... )` span up to the first closing parenthesis with newlines turned into spaces, or `unexpected: ` plus the first 200 characters of the term when neither appears. `corpus/handmade-negative/undefined_variable.zkir` prints `wfError ( "undefined variable %c" )`, and `native_identity.zkir` prints `wfOk`.

| Exit | When | Example |
|---|---|---|
| 0 | term built, and for `check` `krun` returned 0 | |
| 2 | `ZkirFormatError`; stderr `format error: ...` | `bool_via_neg.zkir` without `--ext`: `format error: unknown IR type 'Bool'`; `native_bytes.zkir` with `--ext`: `format error: unknown instruction op 'reverse_bytes' (removed at midnight-zkir 2ffe2d1; use reverse)`; `wrong_version.zkir`: `format error: unhandled version: 3.1` |
| `krun`'s code | `check` when `run_process` is not 0; stderr is `krun`'s stderr | |

The tool depends on the `zkir-syntax.k` constructors and, for `kore`/`check`, on `zkir-check-kompiled`.

## `zkir_run.py`

`zkir_run.py` runs `job`, `checkedJob` or `genJob` on ZKIR-VM and prints a JSON object:

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py FILE.zkir PREIMAGE.json [--definition DIR] [--ext] [--checked] [--gen] [--depth N]
```

| Flag | Effect |
|---|---|
| `--definition DIR` | kompiled directory; default `zkir-kompiled`, or `zkir-ext-kompiled` with `--ext` |
| `--ext` | extension surface in the preprocessor and in `encode_value` / `type_string` |
| `--checked` | entry point `checkedJob` (`zkir-vm.k`) |
| `--gen` | entry point `genJob`; wins over `--checked` when both are set |
| `--depth N` | passed to `KRun.run_process`; an unfinished configuration is `depth-exhausted`, whether the bound was hit or no rule applied; `stuck` is reported only without `--depth` |

`main` prints `json.dumps(result, indent=1)`, one array element or object member per line, and exits 0 for every reported `status`, including `error`, `panic`, `stuck`, `depth-exhausted` and `krun-failed`, so callers must read `status`. Exit 2 is `format error: ...` (`ZkirFormatError`) or `preimage error: ...` (`PreimageError`), both on stderr. Any other failure is an uncaught Python exception with a traceback on stderr and exit 1: a missing program or preimage file, a `communications_commitment` with fewer than two elements (`IndexError`) or one that is not indexable, such as a JSON number (`TypeError`), or a missing kompiled directory.

### `Runner.run`

`Runner(definition=None, ext=False)` constructs `KRun`. `run(program, preimage, depth=None, gen=False, checked=False)` wraps `program` and `preimage_term(preimage)` as `genJob` if `gen`, else `checkedJob` if `checked`, else `job`. `run_file(path, preimage, gen=False, checked=False)` loads the file through `zkir_kast.load_program` and calls `run` without a depth bound. The CLI calls `run`.

| Reported `status` | Condition |
|---|---|
| `krun-failed` | `run_process` return code not 0; the dict is only `{status, error}` with `error` equal to `stderr[-2000:]` |
| `depth-exhausted` | `<k>` is not an empty `KSequence`, and `depth is not None` |
| `stuck` | `<k>` is not an empty `KSequence`, and `depth is None` |
| `ok` | empty `<k>` and `<status>` is `ok()` |
| `panic` | empty `<k>` and `<status>` is `panic(S)`; `error` is `S` |
| `error` | empty `<k>` and any other `<status>` constructor (`error(S)`); `error` is `S` |

On `stuck` / `depth-exhausted`, `error` is the pretty `<k>` term, newlines as spaces, cut to 300 characters, and the remaining fields are still filled. `--depth 1` on `native_identity.zkir` with the complete preimage above yields `depth-exhausted` and a residual `#loadInputs ( ListItem ( 7 ) , typedId ( "%v_0" , native ( ) ) , .TypedIds , 0 ) ~> ...`.

Need labels come from cell `<needs>` (`zkir-vm.k`, sort `Need`):

| K constructor | JSON label | Payload |
|---|---|---|
| `needPubOut(T)` | `pubOut` | IR type string (`Scalar<BLS12-381>`, `Bytes<n>`, ...) |
| `needPriv(T)` | `priv` | IR type string |
| `needPubIn(n)` | `pubIn` | integer |
| `needComm(n)` | `comm` | integer (the recorded commitment) |

Memory records come from `<mem>`, in K map order. Each register is `{variant, type, encoded}`. `variant` and `encoded` come from `encode_value` in `zkir_run.py`, a Python re-encoding of the register value with the same formulas as `encodeValue` in `zkir-values.k`, and `encoded` is a list of decimal strings. `type` comes from `type_string`, the crate's `{:?}` (Debug) rendering of `IrType`, which is what the oracle prints. `unit_values.py` checks `encodeValue` against an inline copy of those formulas, not against `encode_value`. `encode_value` itself is checked only through `diff_test.py` and `divergence_tests.py`, which compare `type` and `encoded` with the oracle on joint `ok` runs. No tool compares `variant` with anything. For `bytes32V` the names differ by surface: variant and type `Bytes32` on the base surface, variant `Bytes` and type `Bytes(32)` on the extension surface.

| JSON key | Source |
|---|---|
| `pis` | decimal strings from `<pi>` |
| `pi_skips` | `null` for `skipNone()`, integer for `skipSome(n)` |
| `cursors` | `[<pubInIdx>, <pubOutIdx>, <privIdx>]` |
| `constraints` | length of `<constraints>` |
| `k_cell` | pretty `<k>` |
| `needs` | list of `[label, payload]` as above |
| `verdicts` | count of `<verdicts>` |
| `all_verdicts` | `[outcome, message, gate]` per `verdict` |
| `violations` | the same triples whose outcome is neither `holds` nor `unconstrained`, gate cut to 120 characters |
| `witness_space` | the `<witnessSpace>` cell: every verdict holds or is unconstrained and every register is well typed |
| `unconstrained` | the registers of `<unconstrainedRegs>`, whose assigning relation is `unconstrained` |
| `outputs` | `"<type>:<enc0>,<enc1>,..."` per `<outputs>` value |
| `observable` | the `<observable>` cell, `obs(status, encoded outputs, public inputs)`, as `{status, error?, outputs, pis}` with decimal strings; `null` while the cell is `noObs()` (16-compilation-target-contract.md) |

A `--gen` run on `native_identity.zkir` with provisional commitment `0` and opening `9` prints the following, with whitespace condensed:

```json
{
 "status": "ok",
 "memory": {
  "%v_0": {"variant": "Native", "type": "Native", "encoded": ["7"]}
 },
 "pis": ["42", "0"],
 "pi_skips": [],
 "cursors": [0, 0, 0],
 "constraints": 3,
 "k_cell": ".K",
 "needs": [["comm", 47986959901854722077236245854877419366049068979521798207792096482323592476386]],
 "verdicts": 3,
 "all_verdicts": [
  ["holds", "", "bindGate ( 0 )"],
  ["violated", "communications commitment differs from poseidon(rand ++ encode(inputs) ++ encode(outputs))", "commGate ( 1 , typedId ( \"%v_0\" , native ( ) ) , .TypedIds , 9 )"],
  ["holds", "", "outputGate ( var ( \"%v_0\" ) , .Operands , native ( ) , .IrTypes )"]
 ],
 "violations": [
  ["violated", "communications commitment differs from poseidon(rand ++ encode(inputs) ++ encode(outputs))", "commGate ( 1 , typedId ( \"%v_0\" , native ( ) ) , .TypedIds , 9 )"]
 ],
 "outputs": ["Native:7"]
}
```

`status` is `ok` because generation records the commitment instead of checking it, but `commGate` is still `violated` against the provisional `0`. The tool depends on `zkir_kast.py` and on `zkir-kompiled` or `zkir-ext-kompiled`.

## `diff_test.py`

`diff_test.py` is the differential harness against the oracle. Only its protocol is described below; the comparison rules and receipts are in `12-oracles-and-differential-testing.md`.

```
uv run --group zkir-k python experiments/zkir-k/tools/diff_test.py [--only SUBSTR] [--seed N] [--no-perturb] [--attempts N] [--ext]
```

| Flag | Default | Effect |
|---|---|---|
| `--only SUBSTR` | `''` | keep files whose names contain the substring |
| `--seed N` | `2026` | per-file RNG seed `f'{seed}:{path.name}'` |
| `--no-perturb` | off | skip the perturbation variants after a joint success |
| `--attempts N` | `8` | generated preimages per program; stop after the first joint `ok` |
| `--ext` | off | `zkir-ext-kompiled`, the 2ffe2d1 oracle, and the extension corpora |

Without `--ext` the corpora are `ledger9-92e8bdd3-tests`, `midnight-zkir-2ffe2d1-precompiles`, `handmade`, plus the escrow and swap artifacts; with `--ext` they are `midnight-zkir-2ffe2d1-tests`, `midnight-zkir-2ffe2d1-precompiles` and `handmade`. Oracle binaries are `~/Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle` and `~/Moriarty/repos/_build/midnight-zkir-2ffe2d1/target/release/zkir-oracle`, and a missing binary is `FileNotFoundError`. Oracle exit 101 is `panic`; any other non-zero exit is `load-error`.

| Step | Action |
|---|---|
| 1 | load the program; a `ZkirFormatError` is compared with crate `load-error` |
| 2 | draw typed raw inputs (`zkir_values.small_encoded`); seed attempt 0 from `test_preimage` |
| 3 | run `genJob` until `pubOut` and `priv` needs are empty (at most 8 passes) |
| 4 | run `job` and the oracle on the same preimage |
| 5 | after a joint `ok`, unless `--no-perturb`, compare up to four variants: `perturbed-raw` and `perturbed-typed` when `inputs` is non-empty, `wrong-pubin` when `public_transcript_inputs` is non-empty, `wrong-comm` when a commitment is present |

`compare` requires matching `status`. On `error`/`panic` it also requires matching `err_class` (`ERROR_CLASSES`, 21 named patterns, else `other:` plus a prefix). For `ok` it requires matching `pis`, `pi_skips`, the same set of register names (a name on one side only is `missing in K` / `missing in Rust`), and every register's `type` and `encoded`; `variant`, `cursors`, `constraints` and the verdicts are not compared. Each row is `PASS` or `FAIL`, corpus, file, label (`run0`, `perturbed-raw`, `format`, ...) and a summary. The footer first counts rows whose statuses agree (`ok`/`ok`, then `error`/`error` or `panic`/`panic`); that count is status agreement only and includes a joint `ok` row with differing registers. Then come the totals that agree and disagree under `compare`, then `oracle 2: N successful K runs with a non-holding gate`. Exit status is 0 iff every comparison agrees, else 1. The harness depends on `zkir_kast.py`, `zkir_run.Runner`, `zkir_values.py` and the oracle.

## `divergence_tests.py`

`divergence_tests.py` runs twenty executable cases for the splits in `13-known-divergences.md`. It parses no arguments: every argument, including `--help`, is ignored, and the tool runs immediately and writes its files.

```
uv run --group zkir-k python experiments/zkir-k/tools/divergence_tests.py
```

Each `CASES` tuple is `(name, finding, program, raw_inputs, expected_K, expected_Rust, gate_sub, expected_outcome, note)`. The script writes `corpus/divergence/<name>.zkir` and builds the preimage `{inputs: raw as strings, binding_input: "42"}` in memory; the oracle receives it through a temporary file that is deleted on close. When a case needs a `communications_commitment`, the script fills it from a `genJob` `comm` need with opening `7`. It then runs `Runner.run(..., checked=True)` and the 92e8bdd3 oracle.

If K's error starts with `well-formedness`, the reported K status is `wfError` and the outcome is `n/a`. Otherwise the target gate is selected from `all_verdicts` as follows:

```python
norm = lambda t: t.replace('_', '').replace(' ', '').lower()
targets = [v for v in k['all_verdicts']
           if norm(v[2]).startswith('gate(' + norm(gate_sub) + '(')
           or norm(v[2]).startswith(norm(gate_sub) + '(')]
```

`gate_sub` is the ZKIR op name as written in the JSON (`less_than`, `public_input`, `from_coordinates`, ...) or `commGate` / `guardGate`. `norm` drops underscores, spaces and case so that the op name matches the K production name in the pretty gate text; `less_than` matches `gate ( lessThan ( ... ) )`. The first match's outcome constructor is the observed outcome, and no match is `no-such-gate`. `assert` matches `gate ( assert ( ... ) )`, whereas `commGate` and `guardGate` match constructors not wrapped in `gate(...)`. A case passes when K status, Rust status and outcome equal the tuple and K is not `stuck`. When both sides are `ok`, `{type, encoded}` of every register must match, and a difference prints `MEMORY DIFFERS` and fails.

Each case prints `PASS` or `FAIL`, name, finding, `K=`, `Rust=`, `gate=<sub>:<outcome>`, the note, and a detail line. The footer is `N/20 divergence cases behave as expected`, where `N` is 20 minus the failure count; a memory difference on a joint `ok` counts as a second failure of the same case. Exit status is 0 iff every case matches, else 1. The script depends on `zkir_kast.py`, `zkir_run.Runner`, `zkir_values.py`, the 92e8bdd3 oracle, and write access to `corpus/divergence/`.

## `unit_values.py`

`unit_values.py` runs forty-three checks of `ZKIR-FIELD`, `ZKIR-CURVES` and `ZKIR-VALUES` through `zkir-test-kompiled`. It parses no arguments: every argument, including `--help`, is ignored, and the tool runs immediately. Each check builds a function term (`finv`, `fsqrt`, `legendre`, `onCurve`, `inSubgroup`, `ecMul`, `jubjubFromXY`, `encodeValue`, `decodeValue`, `decodeStrict`, ...), runs it as a `KItem`, and compares the pretty result with an expected value. That value is either an independent Python formula in the same file (field inverse, Legendre symbol, curve arithmetic, encodings) or a literal (curve membership, subgroup membership, `inf ( )`, `noSqrt ( )`, the `decErr` / `decPanic` messages). `fsqrt 4` accepts either square root.

```
uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py
```

Lines are `PASS <name>` or `FAIL <name>` plus expected/actual, and the footer is `43/43 checks passed`, as in the receipt `evidence/zkir-k-unit-values-2026-09-05c.txt`. Exit status is 0 if every check matches, else 1, and a failed `krun` raises `AssertionError`. The tool depends on `zkir-test-kompiled` only and uses no oracle.

## `unit_hash.py`

`unit_hash.py` runs eighteen known-answer checks of `ZKIR-HASH` against the 92e8bdd3 oracle and `hashlib.sha256`. It parses no arguments: every argument, including `--help`, is ignored, and the tool runs immediately. The tool runs `corpus/handmade/transient_hash.zkir` and `std_hashes.zkir` through the oracle only, on inputs `5`, `123456789`, `987654321987654321`, to obtain expected register values; the programs never go through `ZKIR-VM`. Seventeen checks evaluate a K term in `ZKIR-TEST` (`poseidonHash`, `hashToCurve`, `encodeValue(bytes32V(sha256Bytes(...)))`, `encodeValue(bytes32V(keccak256Bytes(...)))`, `alignedBytes`). One, `sha256 bytes(32) alignment vs hashlib`, runs no K term and compares an oracle register with `hashlib` alone. Hash checks compare with the oracle's registers (`%h0` to `%h3`, `%p0`, `%p1`, `%p3`, `%s1`, `%k1`, `%s32`, `%k32`, `%sf`, `%k200`) and, for one-byte sha256, with `hashlib`. The three `alignedBytes` checks compare with byte strings built in the file from the same inputs (`pre32`, `pref`, `pre200`).

```
uv run --group zkir-k python experiments/zkir-k/tools/unit_hash.py
```

Output has the same `PASS`/`FAIL` lines and an `18/18 checks passed` footer, and the exit status is 0 if every check matches, else 1. A non-zero oracle exit is `CalledProcessError`, because the oracle is invoked with `subprocess.run(..., check=True)`. The preimage is written with `NamedTemporaryFile(..., delete=False)` and left in the process temp directory. Dependencies are `zkir-test-kompiled`, the two handmade programs, and the 92e8bdd3 oracle.

## `check_corpus.py`

`check_corpus.py` checks the well-formedness of every version-3 program in its corpora against an expectation table.

```
uv run --group zkir-k python experiments/zkir-k/tools/check_corpus.py [--definition DIR]
```

`--definition` defaults to `zkir_kast.default_definition()` (`zkir-check-kompiled`). The corpora are `ledger9-92e8bdd3-tests`, `midnight-zkir-2ffe2d1-precompiles`, `handmade-negative`, and the escrow and swap artifacts (63 files). It does not walk `handmade/`, `midnight-zkir-2ffe2d1-tests/` or `divergence/`, and it does not pass `--ext`.

For each file it calls `zkir_kast.load_program` (base surface) and `ZKIR-CHECK`. A `ZkirFormatError` is recorded as `format: <message>` (not the CLI's `format error:` prefix). The default expectation is `wfOk`; overrides in `EXPECTED`:

| File | Expected |
|---|---|
| `output_arity_mismatch.zkir` | `wfError` containing `signature declares 1 return values but instruction has 2` |
| `output_operand_type_mismatch.zkir` | `wfOk` (the mismatch is dynamic) |
| `test_invalid_operand_no_percent_prefix.zkir` | `format` |
| `test_invalid_operand_malformed_identifier.zkir` | `format` |
| `test_invalid_operand_odd_length_hex.zkir` | `format` |
| `reassignment.zkir` | `wfError` containing `reassignment of %b` |
| `undefined_variable.zkir` | `wfError` containing `undefined variable %c` |
| `duplicate_input.zkir` | `wfError` containing `duplicate input %a` |
| `excessive_bits.zkir` | `wfError` containing `constrain_bits: excessive bit bound` |
| `immediate_out_of_range.zkir` | `format` |
| `divmod_outputs.zkir` | `wfError` containing `div_mod_power_of_two requires exactly 2 outputs` |
| `wrong_version.zkir` | `format` |

`format` matches any actual string that starts with `format`, and `wfError:<sub>` matches an actual `wfError` that contains `<sub>`. Each row is `PASS` or `FAIL`, corpus, file, instruction count, and the actual outcome; the footer is `N programs, M as expected, F unexpected, T s`. Exit status is 0 iff every program matches, else 1. The tool depends on `zkir_kast.py` and `zkir-check-kompiled`.

## `gen_handmade.py`

`gen_handmade.py` writes the positive handmade programs so that each value-level instruction has a successful differential run on every type it supports. It parses no arguments: every argument, including `--help`, is ignored, and the tool runs immediately and writes its files. The run is idempotent: it overwrites the same files with the same content.

```
uv run --group zkir-k python experiments/zkir-k/tools/gen_handmade.py
```

It writes seven artifacts (`curve_jubjub.zkir`, `curve_secp256k1.zkir`, `curve_secp256r1.zkir`, `curve_curve25519.zkir`, `native_bytes.zkir`, `transcripts.zkir`, `transcripts_guard_off.zkir`) and `corpus/handmade/manifest.json`. `transient_hash.zkir` and `std_hashes.zkir` are listed in that manifest with `test_preimage.inputs` `["5", "123456789", "987654321987654321"]` but are not generated. Stdout is `wrote 7 programs`, and a failure is an uncaught exception. The tool depends on `zkir_values.py` and write access to `corpus/handmade/`.

## `gen_constants.py`

`gen_constants.py` regenerates `semantics/zkir-constants.k` from midnight-circuits 7.2.4. The single positional argument is the crate's `src/` directory, the one containing `hash/` and `ecc/`. Argument parsing goes no further than `sys.argv[1]`: a missing argument is `IndexError`, and the crate root instead of `src/` is `FileNotFoundError`. Stdout is the module. A shell redirect truncates its target before the script runs, so write to a temporary file and move it over the K file after a successful exit:

```
uv run --group zkir-k python experiments/zkir-k/tools/gen_constants.py \
  ~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src \
  > /tmp/zkir-constants.k \
  && mv /tmp/zkir-constants.k experiments/zkir-k/semantics/zkir-constants.k
```

It reads `hash/poseidon/constants/blstrs.rs` (`ROUND_CONSTANTS`, `MDS`) and `ecc/hash_to_curve/mtc_params.rs` (`SVDW_Z`, `A`, `B`, `MONT_J`, `MONT_K`), where `Fq::from_raw([a,b,c,d])` is four little-endian 64-bit limbs. It asserts 68 rounds times 3 positions (204 constants) and a 3-by-3 MDS matrix, then emits `poseidonRC`, `poseidonMDS`, and `#svdwZ`, `#svdwA`, `#svdwB`, `#montJ`, `#montK`. The script does not import pyk, and on the current registry copy its output is byte-identical to the checked-in file (233 lines). After a regenerate, rekompile `zkir-kompiled`, `zkir-ext-kompiled` and `zkir-test-kompiled`: `ZKIR-CONSTANTS` is imported by `zkir-hash.k` and `zkir-test.k`, and `ZKIR-CHECK` does not include it.

## `extract_test_inputs.py`

`extract_test_inputs.py` attaches the crate's own literal `ProofPreimage` fields to `corpus/ledger9-92e8bdd3-tests/manifest.json`. It parses no arguments: every argument, including `--help`, is ignored, and the tool runs immediately and writes its file. For each manifest entry it `git show`s `92e8bdd3:<source>` from `~/Moriarty/repos/midnightntwrk/midnight-ledger` and copies integer-literal `inputs`, transcript vectors and `binding_input` from the test body (including `assert_check_err_contains` / `assert_typed_output_roundtrip`); computed vectors are dropped. Entries with a literal `inputs` vector receive `test_preimage`; others lose that key.

```
uv run --group zkir-k python experiments/zkir-k/tools/extract_test_inputs.py
```

Stdout is `N programs with literal test inputs:` followed by the file names; the current manifest has 13 such entries. A missing ledger checkout fails `git show` (`CalledProcessError`). The tool depends on that checkout at 92e8bdd3 and on write access to the ledger9 manifest.

## `zkir_values.py`

`zkir_values.py` is the Python reference for field constants, curve arithmetic and public-input encodings, used through `import zkir_values` rather than as a CLI. `diff_test.py`, `divergence_tests.py` and `gen_handmade.py` import it. `unit_values.py` duplicates the same formulas inline, and `zkir_run.encode_value` reimplements `enc_foreign` for memory records.

| Export | Role |
|---|---|
| `R`, `RJ`, `K256P`, `K256N`, `P256P`, `P256N`, `C25519P`, `C25519L` | field moduli |
| `ed_add` / `ed_mul`, `w_add` / `w_mul` | Edwards and Weierstrass arithmetic |
| `JUBJUB_G`, `C25519_G`, `K256_G`, `P256_G` | generators; `JUBJUB_G` is `8 * JUBJUB_G_FULL` |
| `enc_foreign`, `enc_wpoint` | public-input encodings |
| `ENCODED_LEN` | raw width of the 13 base-surface types |
| `random_encoded(ir_type, rng)` | a valid encoding of a random value |
| `small_encoded` | like `random_encoded`, but native values from `{0,1,1,1,2,7,255}` |

`Bool`, `Byte` and `Bytes<n>` are handled in `random_encoded` before the `match` and are not keys of `ENCODED_LEN`; `diff_test.py` uses `ENCODED_LEN.get(type) or len(random_encoded(...))` when it perturbs a typed input.

## Running the suite after a semantics change

After editing a `.k` file, rebuild every compiled definition that imports the changed module, with `--backend llvm --emit-json -O1` and `--output-definition` pointing at the existing directory (commands in `02-getting-started.md`). Then run, in this order:

| Step | Command | What it exercises |
|---|---|---|
| 1 | `gen_constants.py`, then rekompile `zkir-kompiled`, `zkir-ext-kompiled` and `zkir-test-kompiled` | only if Poseidon or Shallue-van de Woestijne constants changed |
| 2 | `unit_values.py` | `ZKIR-TEST`; no oracle |
| 3 | `unit_hash.py` | hashes and alignments; needs the 92e8bdd3 oracle |
| 4 | `zkir_kast.py check` on one well-formed file and one `handmade-negative/` file, then `zkir_run.py` on `native_identity.zkir` or `transient_hash.zkir` | preprocessor and `ZKIR-VM` still load |
| 5 | `check_corpus.py` | `ZKIR-CHECK` on the 63-file set, including the seven negatives |
| 6 | `divergence_tests.py` | rewrites `corpus/divergence/` and checks one named gate per case; needs the 92e8bdd3 oracle |
| 7 | `diff_test.py --only <touched-file> --no-perturb --attempts 1`, then the full base-surface run, then `diff_test.py --ext` | start the full runs only after steps 2 to 5 pass |

`--checked` does not use `zkir-check-kompiled`. It evaluates `wf` inside `ZKIR` / `ZKIR-EXT`, so a well-formedness-only edit still needs those two interpreters rebuilt. An extension-only edit needs `zkir-ext-kompiled` plus `diff_test.py --ext`, and `check_corpus.py` never sees `Bool` / `Byte` / `Bytes<n>` programs.
