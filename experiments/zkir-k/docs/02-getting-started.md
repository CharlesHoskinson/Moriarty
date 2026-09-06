# Getting started

This chapter takes a clean checkout to a first run of the executable K definition of ZKIR (Zero-Knowledge Intermediate Representation) v3. Run every command from the repository root. The program format is described in `03-program-model.md`, the run lifecycle in `06-configuration-and-run-lifecycle.md` and the tools in `11-tooling-reference.md`.

## Prerequisites

The definition is compiled and executed with K v7.1.337. Install that exact release with kup, the K Framework package installer, which itself needs Nix:

```
kup install k --version v7.1.337
```

Confirm `kompile --version` reports `v7.1.337`.

The Python tools talk to K through pyk, the Python bindings shipped as the PyPI package `kframework`. pyk is not a default dependency. It lives in the `zkir-k` dependency group of `pyproject.toml`, pinned to `kframework==7.1.337`, with `requires-python = ">=3.13"`, and every tool runs through uv with that group. The runner's help text lists the flags used below:

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py --help
```

```
usage: zkir_run.py [-h] [--definition DEFINITION] [--ext] [--checked] [--gen]
                   [--depth DEPTH]
                   file preimage

positional arguments:
  file
  preimage

options:
  -h, --help            show this help message and exit
  --definition DEFINITION
  --ext
  --checked             run the static check first (checkedJob)
  --gen                 generation mode (genJob): record transcript needs and
                        the commitment instead of checking them
  --depth DEPTH         stop after this many rewrite steps (status depth-
                        exhausted if the run is unfinished)
```

`tools/zkir_run.py` and `tools/check_corpus.py` need only K and pyk. `tools/unit_hash.py`, `tools/diff_test.py` and `tools/divergence_tests.py` also call the `zkir-oracle` binary at `~/Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle` (and, with `--ext`, `~/Moriarty/repos/_build/midnight-zkir-2ffe2d1/target/release/zkir-oracle`). A Rust toolchain is required only to rebuild that binary from the harness crates under `repos/_build/`.

Run `experiments/zkir-k/toolchain-check/check_k_toolchain.sh` after a K or pyk upgrade. It kompiles K tutorial lesson 1.2 with the LLVM (Low Level Virtual Machine) and Haskell backends, runs `banana.color` and `blueberry.color`, checks that pyk's version equals the expected K version (`7.1.337`, overridable through `EXPECTED_K_VERSION`), and round-trips a term through pyk (`pyk_roundtrip.py`). The script changes into its own directory, deletes and rebuilds `lesson-02-a-kompiled` and `lesson-02-a-haskell-kompiled` there, and exits non-zero on any mismatch.

## Compiling the four definitions

The source files under `experiments/zkir-k/semantics/` produce four LLVM interpreters. pyk reads `compiled.json` from each directory, so every compile must pass `--emit-json`. Pass `-O1` as the optimisation level; the output directory does not record which level built it.

| Main file | Main module | Output directory | Used by |
|---|---|---|---|
| `zkir.k` | `ZKIR` | `experiments/zkir-k/semantics/zkir-kompiled/` | `zkir_run.py`, `diff_test.py` (base surface) |
| `zkir-ext.k` | `ZKIR-EXT` | `experiments/zkir-k/semantics/zkir-ext-kompiled/` | `zkir_run.py --ext`, `diff_test.py --ext` |
| `zkir-check.k` | `ZKIR-CHECK` | `experiments/zkir-k/semantics/zkir-check-kompiled/` | `zkir_kast.py check`, `check_corpus.py` |
| `zkir-test.k` | `ZKIR-TEST` | `experiments/zkir-k/semantics/zkir-test-kompiled/` | `unit_values.py`, `unit_hash.py` |

`kompile FILE.k` writes `FILE-kompiled/` in the current working directory unless `--output-definition` is given. From the repository root:

```
kompile experiments/zkir-k/semantics/zkir-check.k --backend llvm --emit-json -O1 --output-definition experiments/zkir-k/semantics/zkir-check-kompiled
kompile experiments/zkir-k/semantics/zkir-test.k --backend llvm --emit-json -O1 --output-definition experiments/zkir-k/semantics/zkir-test-kompiled
kompile experiments/zkir-k/semantics/zkir.k --backend llvm --emit-json -O1 --output-definition experiments/zkir-k/semantics/zkir-kompiled
kompile experiments/zkir-k/semantics/zkir-ext.k --backend llvm --emit-json -O1 --output-definition experiments/zkir-k/semantics/zkir-ext-kompiled
```

Each compile takes tens of seconds. The four directories already exist; rebuild them after editing any `*.k` file they close over. `--checked` does not use `zkir-check-kompiled`: it runs `checkedJob` inside the `ZKIR` or `ZKIR-EXT` interpreter, which imports `ZKIR-WF` from `zkir-syntax.k`.

## Running a first program

`corpus/handmade/transient_hash.zkir` is a small base-surface program: three `Scalar<BLS12-381>` inputs, four `transient_hash` instructions and three `hash_to_curve` instructions, no outputs and no communications commitment. Its preimage in `corpus/handmade/manifest.json` is:

```
cat > /tmp/transient_hash.pre.json <<'EOF'
{"inputs": ["5", "123456789", "987654321987654321"]}
EOF
```

Omitted preimage keys take the defaults of `preimage_term` in `tools/zkir_run.py`: `binding_input` 0, no communications commitment, empty transcripts. Integers must be canonical BLS12-381 scalar field elements; otherwise the tool prints `preimage error: ...` and exits 2.

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py \
  experiments/zkir-k/corpus/handmade/transient_hash.zkir \
  /tmp/transient_hash.pre.json
```

The runner builds `job(Program, Preimage)` (`zkir-vm.k`, symbol `job`), runs the LLVM interpreter, and prints a JSON object:

```
{
 "status": "ok",
 "memory": {
  "%h0": {
   "variant": "Native",
   "type": "Native",
   "encoded": [
    "0"
   ]
  },
  "%c": {
   "variant": "Native",
   "type": "Native",
   "encoded": [
    "987654321987654321"
   ]
  },
  "%h2": {
   "variant": "Native",
   "type": "Native",
   "encoded": [
    "14402477254716675426448561996916292808062279109581899385137257520097226725002"
   ]
  },
  "%p1": {
   "variant": "JubjubPoint",
   "type": "JubjubPoint",
   "encoded": [
    "5968386414577532069423544848855387078616839520222048692755488202494369665752",
    "11845943366701987068132992202839937613190733374629462571709163139310435919544"
   ]
  },
  "%b": {
   "variant": "Native",
   "type": "Native",
   "encoded": [
    "123456789"
   ]
  },
  "%p3": {
   "variant": "JubjubPoint",
   "type": "JubjubPoint",
   "encoded": [
    "4569577756810611467293458388298139564296031823062755956756675343503875985632",
    "295413728597880211383420139287456294991164181691837837594251154661518621526"
   ]
  },
  "%h1": {
   "variant": "Native",
   "type": "Native",
   "encoded": [
    "47699447692912459726522410466534423256066746656616207079667384395905208673480"
   ]
  },
  "%p0": {
   "variant": "JubjubPoint",
   "type": "JubjubPoint",
   "encoded": [
    "44213293505969394789042601503418778706414741682180306351834794018856757334207",
    "27491819084390928894926537993238951574147548917121491235854409941565096408052"
   ]
  },
  "%h3": {
   "variant": "Native",
   "type": "Native",
   "encoded": [
    "26921307761948379978259856002653291703686817870398067659696385897001549630837"
   ]
  },
  "%a": {
   "variant": "Native",
   "type": "Native",
   "encoded": [
    "5"
   ]
  }
 },
 "pis": [
  "0"
 ],
 "pi_skips": [],
 "cursors": [
  0,
  0,
  0
 ],
 "constraints": 8,
 "k_cell": ".K",
 "needs": [],
 "verdicts": 8,
 "all_verdicts": [
  [
   "holds",
   "",
   "bindGate ( 0 )"
  ],
  [
   "holds",
   "",
   "gate ( transientHash ( var ( \"%a\" ) , var ( \"%b\" ) , var ( \"%c\" ) , .Operands , \"%h3\" ) )"
  ],
  [
   "holds",
   "",
   "gate ( transientHash ( var ( \"%a\" ) , .Operands , \"%h1\" ) )"
  ],
  [
   "holds",
   "",
   "gate ( transientHash ( .Operands , \"%h0\" ) )"
  ],
  [
   "holds",
   "",
   "gate ( transientHash ( var ( \"%a\" ) , var ( \"%b\" ) , .Operands , \"%h2\" ) )"
  ],
  [
   "holds",
   "",
   "gate ( hashToCurve ( var ( \"%a\" ) , var ( \"%b\" ) , var ( \"%c\" ) , .Operands , \"%p3\" ) )"
  ],
  [
   "holds",
   "",
   "gate ( hashToCurve ( var ( \"%a\" ) , .Operands , \"%p1\" ) )"
  ],
  [
   "holds",
   "",
   "gate ( hashToCurve ( .Operands , \"%p0\" ) )"
  ]
 ],
 "violations": [],
 "outputs": []
}
```

`status` is `ok`: the off-circuit witness succeeded. `memory` maps each register to the variant label of `encode_value`, the crate's `IrType` display name of `type_string` (both in `tools/zkir_run.py`), and the off-circuit encoding. The two names agree on the base surface, where `bytes32V` has variant and type `Bytes32`. Every byte string on the extension surface, including `bytes32V` at length 32, has variant `Bytes` and type `Bytes(n)`. Map order is the K `<mem>` walk, not instruction order.

`pis` is the public-input vector, here only the binding input, which defaulted to `0`. `pi_skips` is empty because the program has no `impact`, and the `cursors` (`<pubInIdx>`, `<pubOutIdx>` and `<privIdx>`) are all `0` because no transcript was consumed. `k_cell` is `.K`, so the configuration finished. A residual term gives status `stuck` without a depth limit and `depth-exhausted` with one, with the term in `error`; the second label records that a limit was given, not that it was reached.

`constraints` and `verdicts` are both 8: `bindGate(0)` plus one gate per instruction. Every outcome is `holds`, so `violations` is empty. `needs` is filled only by `--gen`. Outcome names are in `08-constraints-and-verdicts.md`.

## Running with `--checked`

The same command with `--checked` prints the same JSON object. `--checked` switches the entry point from `job` to `checkedJob` (`zkir-vm.k`):

```k
rule <k> checkedJob(P, Pre) => #wfGate(wf(P)) ~> job(P, Pre) ... </k>
syntax KItem ::= #wfGate(WfResult)
rule <k> #wfGate(wfOk()) => .K ... </k>
rule <k> #wfGate(wfError(S)) => .K ... </k> <status> ok() => error("well-formedness: " +String S) </status>
rule <k> job(_, _) => .K ... </k> <status> error(_) </status>
```

On a well-formed program `wf` returns `wfOk()` and execution continues as `job`. On a well-formedness error the status becomes `error("well-formedness: " + S)` and the `job` rule for a failed status discards the program: no inputs are loaded, no gates are emitted. `job` models `IrSource::preprocess` on any program `IrSource::load` accepts, including one that overwrites a register, so `checkedJob` accepts strictly fewer programs than the crate. `corpus/handmade-negative/` below shows the difference.

## Running an extension program with `--ext`

`--ext` selects the `ZKIR-EXT` interpreter (`zkir-ext-kompiled`) and tells `zkir_kast.py` to accept the midnight-zkir 2ffe2d1 surface: types `Bool`, `Byte` and `Bytes<n>`, plus the instructions in `09-extension-surface.md`. `corpus/midnight-zkir-2ffe2d1-tests/bool_via_neg.zkir` declares a `Bool` input `%b`, negates it, and outputs the result. It sets `do_communications_commitment` to true, so the preimage must carry `communications_commitment` as `[commitment, opening]`, with a commitment that matches the inputs and outputs.

Generation mode produces that value. Give a provisional commitment `0` with the opening you intend to use (here `7`) and pass `--gen`, which runs `genJob`. The rule for `genJob` in `zkir-vm.k` sets `<genMode>` to true. Leaving the pair out fails with `Expected communications commitment` even in generation mode (`zkir-vm.k`, rule for `#seedPi`).

```
cat > /tmp/bool_via_neg.gen.json <<'EOF'
{"inputs": ["1"], "binding_input": "0", "communications_commitment": ["0", "7"]}
EOF

uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py \
  experiments/zkir-k/corpus/midnight-zkir-2ffe2d1-tests/bool_via_neg.zkir \
  /tmp/bool_via_neg.gen.json --ext --gen
```

The run finishes with `status` `ok`, and `needs` records the commitment that the `#finish` rule computes from the inputs, outputs and opening:

```
 "needs": [
  [
   "comm",
   13017667727313041872964242512164587272394045712655975830054323807645461770462
  ]
 ],
```

In the same output the `commGate` verdict is `violated`, because the gates are still evaluated against the provisional value. Put the recorded value into the preimage and run without `--gen`:

```
cat > /tmp/bool_via_neg.pre.json <<'EOF'
{"inputs": ["1"], "binding_input": "0", "communications_commitment": ["13017667727313041872964242512164587272394045712655975830054323807645461770462", "7"]}
EOF

uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py \
  experiments/zkir-k/corpus/midnight-zkir-2ffe2d1-tests/bool_via_neg.zkir \
  /tmp/bool_via_neg.pre.json --ext
```

```
{
 "status": "ok",
 "memory": {
  "%b": {
   "variant": "Bool",
   "type": "Bool",
   "encoded": [
    "1"
   ]
  },
  "%nb": {
   "variant": "Bool",
   "type": "Bool",
   "encoded": [
    "0"
   ]
  }
 },
 "pis": [
  "0",
  "13017667727313041872964242512164587272394045712655975830054323807645461770462"
 ],
 "pi_skips": [],
 "cursors": [
  0,
  0,
  0
 ],
 "constraints": 4,
 "k_cell": ".K",
 "needs": [],
 "verdicts": 4,
 "all_verdicts": [
  [
   "holds",
   "",
   "bindGate ( 0 )"
  ],
  [
   "holds",
   "",
   "commGate ( 1 , typedId ( \"%b\" , boolT ( ) ) , .TypedIds , 7 )"
  ],
  [
   "holds",
   "",
   "gate ( neg ( var ( \"%b\" ) , \"%nb\" ) )"
  ],
  [
   "holds",
   "",
   "outputGate ( var ( \"%nb\" ) , .Operands , boolT ( ) , .IrTypes )"
  ]
 ],
 "violations": [],
 "outputs": [
  "Bool:0"
 ]
}
```

Public input 0 is the binding input; public input 1 is the commitment. `%nb` is the negation of `%b`, reported in `outputs` as `Bool:0`. Without `--ext` the preprocessor rejects the program: `format error: unknown IR type 'Bool'` on stderr, exit 2 (`zkir_kast.py`, function `ir_type`).

## Reading a failure

### A static failure: `undefined_variable`

`corpus/handmade-negative/undefined_variable.zkir` adds `%a` to undeclared `%c`. The raw entry point fails at run time:

```
cat > /tmp/undefined.pre.json <<'EOF'
{"inputs": ["1"]}
EOF

uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py \
  experiments/zkir-k/corpus/handmade-negative/undefined_variable.zkir \
  /tmp/undefined.pre.json
```

```
{
 "status": "error",
 "error": "variable not found: Identifier(\"%c\")",
 "memory": {
  "%a": {
   "variant": "Native",
   "type": "Native",
   "encoded": [
    "1"
   ]
  }
 },
 "pis": [
  "0"
 ],
 "pi_skips": [],
 "cursors": [
  0,
  0,
  0
 ],
 "constraints": 2,
 "k_cell": ".K",
 "needs": [],
 "verdicts": 2,
 "all_verdicts": [
  [
   "holds",
   "",
   "bindGate ( 0 )"
  ],
  [
   "unknown",
   "register %c is not in the witness",
   "gate ( add ( var ( \"%a\" ) , var ( \"%c\" ) , \"%d\" ) )"
  ]
 ],
 "violations": [
  [
   "unknown",
   "register %c is not in the witness",
   "gate ( add ( var ( \"%a\" ) , var ( \"%c\" ) , \"%d\" ) )"
  ]
 ],
 "outputs": []
}
```

`%a` was loaded. The `add` gate was still emitted, because constraint emission does not wait for the witness; its verdict is `unknown` because `%c` is absent. `--checked` stops before that:

```
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py \
  experiments/zkir-k/corpus/handmade-negative/undefined_variable.zkir \
  /tmp/undefined.pre.json --checked
```

```
{
 "status": "error",
 "error": "well-formedness: undefined variable %c",
 "memory": {},
 "pis": [],
 "pi_skips": [],
 "cursors": [
  0,
  0,
  0
 ],
 "constraints": 0,
 "k_cell": ".K",
 "needs": [],
 "verdicts": 0,
 "all_verdicts": [],
 "violations": [],
 "outputs": []
}
```

`zkir_kast.py check` on the same file runs `ZKIR-CHECK` and prints `wfError ( "undefined variable %c" )`. `reassignment.zkir` is the opposite case: `job` returns `ok` with `%b` overwritten to `2` and the `copy` and `add` gates `violated` against the final memory; `--checked` reports `well-formedness: reassignment of %b` and emits nothing.

### An off-circuit / in-circuit split: `f02_assert_non_boolean`

`corpus/divergence/f02_assert_non_boolean.zkir` asserts a Native register whose raw value is `2`:

```
cat > /tmp/f02.pre.json <<'EOF'
{"inputs": ["2"], "binding_input": "42"}
EOF

uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py \
  experiments/zkir-k/corpus/divergence/f02_assert_non_boolean.zkir \
  /tmp/f02.pre.json
```

```
{
 "status": "error",
 "error": "Expected boolean, found: 2",
 "memory": {
  "%c": {
   "variant": "Native",
   "type": "Native",
   "encoded": [
    "2"
   ]
  }
 },
 "pis": [
  "42"
 ],
 "pi_skips": [],
 "cursors": [
  0,
  0,
  0
 ],
 "constraints": 2,
 "k_cell": ".K",
 "needs": [],
 "verdicts": 2,
 "all_verdicts": [
  [
   "holds",
   "",
   "bindGate ( 0 )"
  ],
  [
   "holds",
   "",
   "gate ( assert ( var ( \"%c\" ) ) )"
  ]
 ],
 "violations": [],
 "outputs": []
}
```

Off-circuit, the rule for `#exec(assert(C))` in `zkir-vm.k` requires the condition to pass `asBool` of `zkir-ops.k`, that is, to equal `1`, so the run is `error`. In-circuit, the rule for `eval(gate(assert(C)), ...)` in `zkir-constraints.k` requires only that `C` be non-zero, so the verdict is `holds` and `violations` stays empty. That split is Finding 2 in `tools/divergence_tests.py`; the whole corpus is in `13-known-divergences.md`.

## Running the check suites

All four suites exit 0 on success. The quoted last lines are from live runs.

```
uv run --group zkir-k python experiments/zkir-k/tools/unit_values.py
```

`unit_values.py` checks the field, curve and encoding functions of `ZKIR-TEST` against an independent Python implementation. It ends with `43/43 checks passed`.

```
uv run --group zkir-k python experiments/zkir-k/tools/unit_hash.py
```

`unit_hash.py` runs known-answer checks for Poseidon, hash-to-curve, SHA-256 and Keccak-256 against the oracle (and hashlib for SHA-256), so it needs the 92e8bdd3 oracle. It ends with `18/18 checks passed`.

```
uv run --group zkir-k python experiments/zkir-k/tools/check_corpus.py
```

`check_corpus.py` runs `ZKIR-CHECK` on every version-3 program in the ledger tests, the six micro-dao precompiles, the Moriarty escrow and swap artifacts, and `corpus/handmade-negative/`. It ends with `66 programs, 66 as expected, 0 unexpected`, followed by the elapsed time, which varies.

```
uv run --group zkir-k python experiments/zkir-k/tools/divergence_tests.py
```

`divergence_tests.py` rewrites the thirty-one programs under `corpus/divergence/`, runs each through `checkedJob`, the oracle and the circuit oracle, and checks one named gate. It needs the 92e8bdd3 `zkir-oracle` and `zkir-circuit-oracle` binaries and the 2ffe2d1 pair for its one extension case, and ends with `31/31 divergence cases behave as expected`.

Do not start with the full differential suite. `tools/diff_test.py` walks every version-3 program in its corpora, generates preimages, compares K with the oracle, then perturbs successes. `--only SUBSTR` keeps only files whose names contain that substring:

```
uv run --group zkir-k python experiments/zkir-k/tools/diff_test.py --only transient_hash --no-perturb --attempts 1
```

```
PASS  handmade                           transient_hash.zkir                                  run0      K=ok Rust=ok regs=10 pis=1 passes=1 0.5s verdicts=8

1 comparisons: 1 successful-run agreements, 0 error-run agreements (status and error class), 1 agree, 0 disagree, 0s
oracle 2: 0 successful K runs with a non-holding gate
```

`--seed` defaults to `2026`. `--attempts` (default 8) bounds the generated preimages per program, and the harness stops after the first run both K and the oracle accept. `--ext` switches interpreter, oracle and corpora to the 2ffe2d1 surface. `--no-perturb` skips the raw, typed, wrong-public-input and wrong-commitment variants that follow a success. Comparison rules are in `12-oracles-and-differential-testing.md`.

## Common problems

| Symptom | Cause | Fix |
|---|---|---|
| `warning: --group zkir-k has no effect when used outside of a project`, then `No module named 'pyk'` | the working directory is not the repository root (uv walks parents for `pyproject.toml`) | `cd` to the repository root before `uv run` |
| `No module named 'pyk'` inside the repository | on a fresh `.venv` the `zkir-k` group was not requested; `kframework` is not a default dependency (once installed by an earlier run with the group, it stays importable without it) | always pass `--group zkir-k` |
| Tools run, but behaviour does not match a `.k` edit | each tool loads its `*-kompiled/` directory and does not recompile | re-run the matching `kompile` command, including `--emit-json` |
| `kompile` succeeded but the tools still use the old interpreter | `kompile FILE.k` wrote `./FILE-kompiled` in the current directory, not under `experiments/zkir-k/semantics/` | pass the `--output-definition` path from the table above |
| `check_k_toolchain.sh` prints `FAIL: expected K 7.1.337` or `FAIL: pyk … != K 7.1.337` | `kompile --version` or `pyk.__version__` is not 7.1.337 | reinstall K with `kup install k --version v7.1.337`; keep `kframework==7.1.337` in the uv group |
| `format error: unknown IR type 'Bool'` (or another extension type), exit 2 | the preprocessor is on the base surface | pass `--ext` |
| `Expected communications commitment` with `--gen` | generation mode still reads the opening from the preimage | supply `communications_commitment` as `["0", opening]` |
| `FileNotFoundError` for `zkir-oracle` from `unit_hash.py`, `diff_test.py` or `divergence_tests.py` | those scripts hardcode `~/Moriarty/repos/_build/…/zkir-oracle` | rebuild the harness, or run `unit_values.py` and `check_corpus.py`, which do not call the oracle |
