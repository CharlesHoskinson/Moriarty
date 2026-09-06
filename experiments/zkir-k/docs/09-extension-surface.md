# Extension surface

## Version boundary

The extension surface follows midnight-zkir `2ffe2d17bbb736aec36fb300aeaca679a10d2278`; the base surface follows midnight-ledger `92e8bdd3`. Source facts below come from the extension crate's `zkir/src/ir.rs`, `ir_types.rs`, `ir_vm.rs` and `ir_instructions/`, available under `repos/_build/midnight-zkir-2ffe2d1/`.

`semantics/`, `tools/` and `corpus/` paths are relative to `experiments/zkir-k/`; evidence and repository paths are relative to the repository root.

| Surface change | Rust locator | Definition locator |
|---|---|---|
| Adds `and`, `or`, `xor`, `concat`, `slice`, `nth`, `reverse`, `load_constant`, `sha512` | `ir.rs`, `Instruction` | `zkir-ext.k`, `ZKIR-EXT-SYNTAX` |
| Removes `reverse_bytes`; `reverse` handles arbitrary byte-string lengths | `ir.rs`, `Instruction::Reverse` | `tools/zkir_kast.py`, `instruction` rejects `reverse_bytes` with `--ext` |
| Adds `Bool`, `Byte`; replaces `Bytes32` with `Bytes(u32)` | `ir_types.rs`, `IrType` | `zkir-ext.k`, `boolT()`, `byteT()`, `bytesT(Int)` |
| Requires canonical re-encoding after decoding | `ir_instructions/encode.rs`, `decode_offcircuit` | `zkir-values.k`, `decodeStrict` |
| Extends equality to Boolean, byte and byte-string pairs | `ir_instructions/eq.rs`, `test_eq_offcircuit` | `zkir-ext.k`, `eqDispatch` |

These are separate selectable surfaces, not a change to the base entry module. See [07-instruction-reference.md](07-instruction-reference.md) for inherited instructions.

## Modules and representation

All extension modules live in `semantics/zkir-ext.k`.

| Module | Responsibility |
|---|---|
| `ZKIR-EXT-SYNTAX` | Imports `ZKIR-SYNTAX`; adds types, instructions, `encodedLen`, `reads` and `writes` rules |
| `ZKIR-EXT-VALUES` | Imports the extension syntax, `ZKIR-VALUES` and `ZKIR-OPS`; adds values, encodings and byte helpers |
| `ZKIR-SHA512` | Imports `ZKIR-HASH` and `ZKIR-SHA512-CONSTANTS`; implements Secure Hash Algorithm 512 (`sha512Bytes`) |
| `ZKIR-EXT` | Imports `ZKIR-VM`, `ZKIR-EXT-VALUES` and `ZKIR-SHA512`; adds execution and gate evaluation rules |

`tools/zkir_kast.py`, `ir_type`, maps `Bytes<32>` to the base `bytes32()` type. Values retain `bytes32(B)`; other lengths use `bytesT(N)` and `bytesV(B)`. `mkBytes` chooses the representation by length and `asBytesAny` accepts both, so the base 32-byte conversion rules remain usable. The removed instruction keeps its execution and gate rules, inherited from `ZKIR-VM`; only the JavaScript Object Notation (JSON) boundary, `tools/zkir_kast.py`, `instruction`, rejects `reverse_bytes` under `--ext`.

`boolV(B)` encodes as one field element, 0 or 1; `byteV(X)` as one element below 256. `encodeValue(bytesV(B))` calls `#encChunks`, packing successive little-endian chunks of at most 31 bytes. `encodedLen(bytesT(N))` is `(N + 30) / 31`. `ir_type` accepts canonical decimal lengths from 1 through 16777216. See [04-values-and-encoding.md](04-values-and-encoding.md) for the shared encoding model.

The extension enables strict decoding before either raw execution or generation:

```k
rule <k> job(P, Pre) => job(P, Pre) ... </k> <strictDecode> false => true </strictDecode> [priority(30)]
rule <k> genJob(P, Pre) => genJob(P, Pre) ... </k> <strictDecode> false => true </strictDecode> [priority(30)]
```

The priority-30 rules precede base initialization. `zkir-vm.k`, `checkedJob`, reaches `job` after `wf`, so successful checked execution also uses strict decoding. Input loading (`#loadInputs`) and transcript decoding (`#decodeSlice`) pass `<strictDecode>` to `decodeStrict`. In `zkir-values.k`, `#canonical` keeps a preceding `decErr`; for `bytes32()` it turns the base decoder's `decPanic` into `decErr("Failed to decode as Bytes32")`, because the 2ffe2d1 `decode_bytes` returns `None` where 92e8bdd3 asserts; it keeps `decPanic` for other types; and it accepts `decOk(V)` only when `encodeValue(V) ==K L`, otherwise `The encoded value of type T is not in canonical form`. A `Bytes<32>` input with low element `2^248` is thus an error under `--ext` and a panic on the base surface.

The extension also adds `negV(boolV(B)) => vOk(boolV(notBool B))`. This is logical negation of a typed Boolean. The inherited `#exec(not(...))` in `zkir-vm.k` continues to use native Boolean conversion.

## Instruction entries

Entries use the Syntax, Off-circuit, Gate, Run-time checks and Corpus structure of chapter 07. Unless stated otherwise, symbols belong to `zkir-ext.k`, and corpus filenames are relative to `corpus/midnight-zkir-2ffe2d1-tests/`.

Every instruction writes its final `String` argument. `reads` returns the input operands, in order; `loadConstant` lists encoding immediates instead. The inherited `zkir-vm.k` sequencing rule emits `gate(I)` and executes `#exec(I)`. Extension `eval` rules use `zkir-constraints.k`, `#matches`, to compare the computed value with the output register: equal values hold, unequal values violate, a computation error is a synthesis error, and an absent output register is unknown. The `and`, `or`, `xor`, `concat`, `slice`, `nth` and `reverse` rules call no `#need`, so an absent input register is also a synthesis error (`and output: variable not found: Identifier("%p")`), where every base gate reports unknown (see [08-constraints-and-verdicts.md](08-constraints-and-verdicts.md)); for these seven gates a synthesis error can mean that an earlier instruction never wrote the register. Only `sha512`, through `#needAll`, keeps the base convention.

`ZKIR-WF` in `zkir-syntax.k` checks reads and writes generically; its `#checkArity` adds no extension-specific checks. Since `reads(loadConstant(_, Es, _))` returns the encoding immediates, `#checkReads` rejects an encoding element outside the field (`immediate out of field range`); byte-string lengths, `slice` and `nth` bounds and Boolean operand types are checked only at run time.

### and

- Syntax: JSON `and`; `andI(Operands, String)` (`inputs`, `output`).
- Off-circuit: `#exec(andI(...))` calls `#boolGate` and `#boolFold`, producing `boolV` conjunction.
- Gate: `eval(gate(andI(...)))` checks the same fold; no named chip.
- Run-time checks: requires nonempty, exclusively `boolV` inputs. `#boolGate` reports `Boolean gate requires at least one input`; `#boolFold` reports `Boolean gate expects Bool inputs, found T`.
- Corpus: `test_bool_gates.zkir`, `test_bool_gate_empty_inputs_fails.zkir`, `test_bool_gate_wrong_result_fails.zkir`.

### or

- Syntax: JSON `or`; `orI(Operands, String)` (`inputs`, `output`).
- Off-circuit: `#exec(orI(...))` calls `#boolGate`; `#boolFold` produces `boolV` disjunction.
- Gate: `eval(gate(orI(...)))` checks disjunction; no named chip.
- Run-time checks: the same nonempty Boolean-input checks and messages as `and`.
- Corpus: `test_bool_gates.zkir`.

### xor

- Syntax: JSON `xor`; `xorI(Operands, String)` (`inputs`, `output`).
- Off-circuit: `#exec(xorI(...))` calls `#boolGate`; `#boolFold` produces `boolV` exclusive-or parity.
- Gate: `eval(gate(xorI(...)))` checks parity; no named chip.
- Run-time checks: the same nonempty Boolean-input checks and messages as `and`.
- Corpus: `test_bool_gates.zkir`.

### concat

- Syntax: JSON `concat`; `concat(Operands, String)` (`inputs`, `output`).
- Off-circuit: `#exec(concat(...))` calls `#concat`, appending `byteV`, `bytesV` and `bytes32` contents; `mkBytes` wraps the result.
- Gate: `eval(gate(concat(...)))` checks concatenation, including its length bound; no named chip.
- Run-time checks: rejects empty results with `Concat requires at least one byte of input`, results above 16777216 bytes with `Concat result length exceeds MAX_BYTES_LEN`, and other types with `Concat expects Byte or Bytes inputs, found T`.
- Corpus: `test_bytes_concat_and_nth.zkir`.

### slice

- Syntax: JSON `slice`; `slice(Operand, Int, Int, String)` (`bytes`, `start`, `len`, `output`).
- Off-circuit: `#exec(slice(...))` calls `#slice` and `#slice2`, selecting `[start, start + len)` through `mkBytes`.
- Gate: `eval(gate(slice(...)))` checks the selected bytes; no named chip.
- Run-time checks: `asBytesAny` rejects non-byte-strings with `cannot convert T to Bytes`; zero length gives `slice length must be at least 1`; an excessive endpoint gives `slice out of bounds`. The preprocessor's `u32` rejects negative indices and lengths.
- Corpus: `test_bytes_slice.zkir`, `test_slice_out_of_bounds_fails.zkir`.

### nth

- Syntax: JSON `nth`; `nth(Operand, Int, String)` (`bytes`, `index`, `output`).
- Off-circuit: `#exec(nth(...))` calls `#nth` and `#nth2`, returning `byteV(B[I])` at a zero-based index.
- Gate: `eval(gate(nth(...)))` checks that byte; no named chip.
- Run-time checks: `asBytesAny` enforces byte-string input; `I >= lengthBytes(B)` gives `nth out of bounds`. The preprocessor's `u32` excludes negative indices.
- Corpus: `test_bytes_concat_and_nth.zkir`, `test_nth_out_of_bounds_fails.zkir`.

### reverse

- Syntax: JSON `reverse`; `reverse(Operand, String)` (`bytes`, `output`).
- Off-circuit: `#exec(reverse(...))` calls `#rev` and `#rev2`; the latter reverses byte order with the K builtin `reverseBytes`, and `mkBytes` wraps it.
- Gate: `eval(gate(reverse(...)))` checks the reversed sequence; no named chip.
- Run-time checks: `asBytesAny` accepts either byte-string representation and rejects other types.
- Corpus: `test_bytes_slice.zkir`, `test_reverse_bytes_proof.zkir`.

### load_constant

- Syntax: JSON `load_constant`; `loadConstant(IrType, Operands, String)` (`type`, `encoding`, `output`).
- Off-circuit: `#exec(loadConstant(...))` calls `decodeStrict(#immInts(Es), T, true)` and writes the decoded constant.
- Gate: `eval(gate(loadConstant(...)))` fixes the output and checks `#chipFor(T, Chips)` from `zkir-constraints.k`.
- Run-time checks: strict decoding enforces the encoding. `tools/zkir_kast.py`, `instruction`, requires hexadecimal immediates. `usedChips` does not discover chips from constant types, so a curve constant can compute successfully while its gate reports a missing chip.
- Corpus: `test_constant_proof.zkir`, `test_constant_bad_encoding_rejected.zkir`.

### sha512

- Syntax: JSON `sha512`; `sha512(Alignment, Operands, String)` (`alignment`, `inputs`, `output`).
- Off-circuit: `#exec(sha512(...))` calls `#sha512V`, converts inputs to natives, uses `zkir-ops.k`, `alignedBytes`, then hashes with `sha512Bytes`, returning 64 bytes in `bytesV`.
- Gate: `eval(gate(sha512(...)))` requires every input register to be present and Native (`#needAll`: absent is unknown, non-Native is a synthesis error), checks the `sha2_512` chip with `#chipNamed`, and compares the same `#sha512V` result. `#chipsOfInstrs` enables that chip.
- Run-time checks: native conversion and alignment decoding errors propagate. The gate goes through `#sha512C`, which decodes the alignment with `alignedBytesCircuit` of `zkir-constraints.k` as the base hash gates do, because the crate's `circuit` shares one byte-hash path (`fab_decode_to_bytes`) for `persistent_hash`, `keccak256` and `sha512`; an alignment with an `option` or `compress` segment is therefore `synthErr` in circuit even when the witness accepts it.
- Corpus: `test_sha512_proof.zkir`.

## Byte-string equality divergence

The extension's `eqDispatch` compares `boolV` pairs, `byteV` pairs and `bytesV` pairs by value. Mixed `bytesV`/`bytes32` pairs dispatch to `eqFalse()`. `zkir-ops.k`, `#testEq`, still produces `native(0)` or `native(1)`, not `boolV`.

For two byte strings of unequal length the witness result is false, whatever their representation, while the gate is a synthesis error from the inherited unequal-type rule of `zkir-constraints.k`, `#eqSupported`; `zkir-ext.k` adds no `test_eq` rule of its own. `typeOf(bytesV(B))` is `bytesT(lengthBytes(B))` and `typeName` is `Bytes` for every length, so two `bytesV` values of different lengths give `Unsupported test_eq: Bytes == Bytes`, and a `bytes32` against a `bytesV` gives `Unsupported test_eq: Bytes32 == Bytes` (or the mirror image). Equal-length `bytesV` pairs, and `bytes32` pairs, remain eligible. This is the contradiction recorded under extension `test_eq` in `wiki/contradictions.md`: Rust's `test_eq_offcircuit` compares vectors, whereas `test_eq_incircuit` requires equal lengths. Successful witness computation therefore does not establish circuit constructibility.

For `cond_select`, the one extension rule of `#eqSupported` rejects `bytesV` pairs with `Unsupported cond_select: Bytes ? Bytes`; the inherited rule rejects `bytes32` pairs. See [13-known-divergences.md](13-known-divergences.md).

## Running and testing

Run these Bash commands from the repository root after installing the environment of [02-getting-started.md](02-getting-started.md); `--offline --no-sync` uses the installed environment, and the two variables keep scratch writes outside the source tree.

```bash
export UV_CACHE_DIR="${TMPDIR:-/tmp}/zkir-docs-uv" PYTHONDONTWRITEBYTECODE=1
uv run --offline --no-sync --group zkir-k python experiments/zkir-k/tools/zkir_kast.py check \
  experiments/zkir-k/corpus/midnight-zkir-2ffe2d1-tests/test_bool_gates.zkir --ext

uv run --offline --no-sync --group zkir-k python experiments/zkir-k/tools/zkir_kast.py kore \
  experiments/zkir-k/corpus/midnight-zkir-2ffe2d1-tests/test_bool_gates.zkir --ext

uv run --offline --no-sync --group zkir-k python experiments/zkir-k/tools/zkir_run.py \
  experiments/zkir-k/corpus/midnight-zkir-2ffe2d1-tests/test_bool_gates.zkir \
  <(printf '%s' '{"inputs":[1,0],"binding_input":0,"private_transcript":[],"public_transcript_inputs":[],"public_transcript_outputs":[]}') \
  --ext --checked

uv run --offline --no-sync --group zkir-k python experiments/zkir-k/tools/diff_test.py \
  --ext --only test_bool_gates --no-perturb
```

`zkir_kast.py --ext` enables parsing of the extension surface. Its default definition, `semantics/zkir-check-kompiled`, imports `ZKIR-EXT-SYNTAX`, so `check --ext` prints `wfOk` here and `kore --ext` prints one K intermediate representation (KORE) term with no `--definition` option; `zkir-ext-kompiled` also serves `kore`, the base `zkir-kompiled` fails on the `Bool` symbol, and `check` needs the `ZKIR-CHECK` configuration.

`zkir_run.py`, `Runner`, selects `zkir-ext-kompiled` itself with `--ext`; `--gen` selects `genJob`. It prints one JSON object and exits 0 even when `status` is `error` or `panic`; `all_verdicts` lists `[outcome, message, gate]` triples and `violations` is empty on a holding run. `[1,0]` is the honest preimage for `test_bool_gates.zkir`, whose `constrain_eq` instructions hold only for `%t` 1 and `%f` 0; the run prints `"status": "ok"`, 17 holding verdicts and `"violations": []`.

`diff_test.py --ext` selects `ORACLE_EXT`, the second Rust oracle at `~/Moriarty/repos/_build/midnight-zkir-2ffe2d1/target/release/zkir-oracle`, and these corpora:

| Directory under `corpus/` | Programs |
|---|---:|
| `midnight-zkir-2ffe2d1-tests/` | 61 |
| `midnight-zkir-2ffe2d1-precompiles/` | 6 |
| `handmade/` | 9 |

It prints one `PASS` or `FAIL` line per comparison, then a summary. `--no-perturb` still draws random preimages, so the command above reports an error-run agreement (`K=error Rust=error`, `Equality constraint failed`) before a successful-run agreement: `2 comparisons: 1 successful-run agreements, 1 error-run agreements`.

The receipt `evidence/zkir-k-differential-ext-2ffe2d1-2026-09-05c.txt` records 418 agreements, zero disagreements: 50 successful-run comparisons, 364 error-run comparisons and four parser/load rejections, one being `handmade/native_bytes.zkir` for its `reverse_bytes`. It reports zero successful K runs with a non-holding gate. These are bounded corpus results; see [12-oracles-and-differential-testing.md](12-oracles-and-differential-testing.md) for the comparison predicates and perturbations.
