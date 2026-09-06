# Program model

## Artifact boundary

A ZKIR v3 artifact is a JavaScript Object Notation (JSON) object. Its shape follows `IrSource::load`, `IrSource`, `TypedIdentifier` and the serde implementations in `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir.rs`, the pinned copy of midnight-ledger `92e8bdd3` to which every Rust reference and line number below refers. The definition's counterpart is [`tools/zkir_kast.py`](../tools/zkir_kast.py), especially `program`, `instruction` and `operand`.

| Required member | JSON shape | Destination in `program(...)` |
|---|---|---|
| `version` | `{"major":3,"minor":0}` | Minor version, integer `0` |
| `inputs` | Array of `{"name":string,"type":string}` | `TypedIds` |
| `outputs` | Array of type strings, in signature order | `IrTypes` |
| `do_communications_commitment` | Boolean | K `Bool` |
| `instructions` | Array of objects with `op` | `Instrs`, preserving order |

`SerdeVersion` uses unsigned eight-bit integers (`u8`) for both components. The preprocessor's `program` rejects booleans, fractional numbers and integers outside 0 to 255 inclusive, then accepts only version 3.0. The major version is checked and discarded, as in the crate's loader, which replaces the version object with its minor number before deserializing `IrSource`.

Unknown object members are ignored, matching serde structs without `deny_unknown_fields`. The required members stay required, and an empty array is not the same as an omitted member. Inputs declare names and types only. Witness values come from the separate preimage, described in [06-configuration-and-run-lifecycle.md](06-configuration-and-run-lifecycle.md).

`Identifier(pub String)` and Python `identifier` accept strings verbatim. Only operand references require `%`: `operand("%x")` constructs `var("%x")`, retaining the prefix. An input name or destination without `%` passes format validation, but its bare spelling cannot be used as a variable operand. Name uniqueness and definition before use are checked by `wf` in `zkir-syntax.k` rather than by the loader; see [10-well-formedness-and-static-checks.md](10-well-formedness-and-static-checks.md).

## Abstract syntax and types

[`zkir-syntax.k`](../semantics/zkir-syntax.k), module `ZKIR-SYNTAX`, declares the program constructor:

```k
syntax Program ::= program(Int, TypedIds, IrTypes, Bool, Instrs) [symbol(program)]
```

`typedId(String, IrType)` represents one input declaration. `TypedIds`, `IrTypes`, `Ids`, `Instrs` and `Operands` are ordered recursive lists, which Python `klist` builds from right to left: it starts at the nullary terminator and conses the JSON elements in reverse, so the first JSON element remains the head, as in `typedIds(typedId("%x", ...), .typedIds)`. The declaration below assigns separate labels to the list constructor and its empty case:

```k
syntax TypedIds ::= List{TypedId, ","}   [symbol(typedIds), terminator-symbol(.typedIds)]
```

The `symbol(...)` attribute supplies the label that pyk's `KApply` uses, so `condSelect` in K rules is the `KApply` labeled `cond_select`, the JSON operation. `terminator-symbol(...)` names the empty lists, such as `.instrs` and `.operands`. Integer, string and boolean leaves are `KToken` values of the corresponding sorts.

K receives no concrete ZKIR source language to parse: JSON parsing and elaboration happen in Python, which keeps JSON object traversal outside the rewrite semantics (decision CLM-0712 in `wiki/zkir-k-semantics-plan.md`). The resulting abstract syntax tree (AST) carries only the data the definition needs, without JSON formatting or ignored metadata.

The 13 base types in the table below are declared in `zkir-syntax.k` (`IrType` and `encodedLen`), with the JSON mapping in Python `TYPE_SYMBOLS`. Lengths count raw native field elements, not bytes, and agree with `IrType::encoded_len` in pinned `ir_types.rs`.

| JSON type | K constructor | pyk symbol | `encodedLen` |
|---|---|---|---:|
| `Scalar<BLS12-381>` | `native()` | `Native` | 1 |
| `Bytes<32>` | `bytes32()` | `Bytes32` | 2 |
| `Point<Jubjub>` | `jubjubPoint()` | `JubjubPoint` | 2 |
| `Scalar<Jubjub>` | `jubjubScalar()` | `JubjubScalar` | 1 |
| `Point<Secp256k1>` | `secp256k1Point()` | `Secp256k1Point` | 5 |
| `Base<Secp256k1>` | `secp256k1Base()` | `Secp256k1Base` | 2 |
| `Scalar<Secp256k1>` | `secp256k1Scalar()` | `Secp256k1Scalar` | 2 |
| `Point<Secp256r1>` | `secp256r1Point()` | `Secp256r1Point` | 5 |
| `Base<Secp256r1>` | `secp256r1Base()` | `Secp256r1Base` | 2 |
| `Scalar<Secp256r1>` | `secp256r1Scalar()` | `Secp256r1Scalar` | 2 |
| `Point<Curve25519>` | `curve25519Point()` | `Curve25519Point` | 4 |
| `Base<Curve25519>` | `curve25519Base()` | `Curve25519Base` | 2 |
| `Scalar<Curve25519>` | `curve25519Scalar()` | `Curve25519Scalar` | 2 |

With `--ext`, Python `ir_type` also accepts `Bool`, `Byte` and `Bytes<n>`, following `from_type_string` in `repos/_build/midnight-zkir-2ffe2d1/zkir/src/ir_types.rs` of midnight-zkir `2ffe2d1`. The length uses canonical ASCII decimal digits without leading zeros and satisfies `1 <= n <= 2^24`. `Bytes<32>` retains `Bytes32`; other lengths use `BytesN(n)`, whose K constructor is `bytesT(n)` in `zkir-ext.k`. See [09-extension-surface.md](09-extension-surface.md) for extension instructions and [04-values-and-encoding.md](04-values-and-encoding.md) for value representations.

## Operands, guards and alignments

`Operand` has `var(String)` and `imm(Int)` constructors in `zkir-syntax.k`. Python `immediate` accepts `0x` or `0X`, optionally preceded by `-`. Its `HEX` pattern requires hexadecimal digits of even length and rejects internal whitespace. Byte order is little-endian, so `0x0100` denotes 1 and `0x0001` denotes 256. The decoded byte sequence must fit within 32 bytes, and its unsigned value must be below the BLS12-381 scalar modulus `r`, named `#blsScalarModulus` in K. Oversized positive values are rejected, not reduced. Negation applies only after validation and produces `(-value) mod r`.

One whitespace form escapes the `HEX` check. Python's `$` anchor matches before a final newline, so `immediate("0x0\n")` passes the pattern and `bytes.fromhex` then raises a `ValueError` that `main` does not catch. The command-line interface (CLI) exits 1 with a traceback rather than a `format error:` line, and the value is still rejected.

`Operand::deserialize` in pinned `ir.rs` uses `const_hex::decode` and `Fr::from_le_bytes`, so the byte interpretation and the range boundary are the same. The accepted spellings differ in one case. `const_hex::decode` (const-hex 1.19.0, `strip_prefix`) removes one further `0x` or `0X` prefix after the deserializer's own, so the crate loads `"0x0x01"` as the value 1 while the preprocessor rejects it with the odd-length or non-hex message. `Operand::serialize` emits little-endian hex after trimming high zero bytes, retaining at least one byte, so loading preserves the field value, not the spelling.

For `public_input` and `private_input`, Python `guard` maps an omitted or null `guard` member to `noGuard()` and any other to `guard(Operand)`, whereas `impact` requires a direct operand named `guard`. The two forms are distinct in the `Guard` and `Instr` declarations of `zkir-syntax.k`. Guard evaluation is described in [07-instruction-reference.md](07-instruction-reference.md).

Python `alignment` builds `alignment(Segments)`, where each `Segment` is `atom(Atom)` or `option(Alignments)`. Atoms are `fieldAtom()`, `bytesAtom(length)` and `compressAtom()`, and the byte length passes the unsigned 32-bit integer (`u32`) check. An option carries an array of alternative alignments, recursively. For example:

```json
[{"tag":"atom","value":{"tag":"bytes","length":32}}]
```

This becomes `alignment(segments(atom(bytesAtom(32)), .segments))` in symbol notation. Loading checks the tagged structure only. Whether a chosen hash operation can consume it is covered in [05-fields-curves-and-hashes.md](05-fields-curves-and-hashes.md).

## Instruction constructors

The 34 rows below follow `Instr`, `reads` and `writes` in `zkir-syntax.k`, and Python `instruction`. Argument names are JSON keys. Indexed names unpack JSON pairs. The unindexed `inputs`, `outputs` and `vals` denote whole lists. Each operation string is also the pyk symbol. Reads list operands, including immediates; writes list register identifiers, not transcript or return-value effects. `none` means an empty list, and an optional guard contributes a read only when present.

| K constructor | JSON `op` | K argument order from JSON | `reads` | `writes` |
|---|---|---|---|---|
| `encode` | `encode` | input, outputs | input | outputs |
| `assert` | `assert` | cond | cond | none |
| `condSelect` | `cond_select` | bit, a, b, output | bit, a, b | output |
| `constrainBits` | `constrain_bits` | val, bits | val | none |
| `constrainEq` | `constrain_eq` | a, b | a, b | none |
| `constrainToBoolean` | `constrain_to_boolean` | val | val | none |
| `copy` | `copy` | val, output | val | output |
| `impact` | `impact` | guard, inputs | guard, inputs | none |
| `ecMul` | `ec_mul` | a, scalar, output | a, scalar | output |
| `ecMulGenerator` | `ec_mul_generator` | scalar, output | scalar | output |
| `hashToCurve` | `hash_to_curve` | inputs, output | inputs | output |
| `intoCoordinates` | `into_coordinates` | point, outputs[0], outputs[1] | point | outputs[0], outputs[1] |
| `fromCoordinates` | `from_coordinates` | inputs[0], inputs[1], output | inputs[0], inputs[1] | output |
| `intoBytes32` | `into_bytes32` | input, output | input | output |
| `fromBytes32` | `from_bytes32` | bytes, type, output | bytes | output |
| `reverseBytes` | `reverse_bytes` | bytes, output | bytes | output |
| `bytes32IntoLowHigh` | `bytes32_into_low_high` | bytes, outputs[0], outputs[1] | bytes | outputs[0], outputs[1] |
| `bytes32FromLowHigh` | `bytes32_from_low_high` | inputs[0], inputs[1], output | inputs[0], inputs[1] | output |
| `divModPowerOfTwo` | `div_mod_power_of_two` | val, bits, outputs | val | outputs |
| `reconstituteField` | `reconstitute_field` | divisor, modulus, bits, output | modulus, divisor | output |
| `transientHash` | `transient_hash` | inputs, output | inputs | output |
| `persistentHash` | `persistent_hash` | alignment, inputs, output | inputs | output |
| `keccak256` | `keccak256` | alignment, inputs, output | inputs | output |
| `testEq` | `test_eq` | a, b, output | a, b | output |
| `add` | `add` | a, b, output | a, b | output |
| `mul` | `mul` | a, b, output | a, b | output |
| `neg` | `neg` | a, output | a | output |
| `inv` | `inv` | a, output | a | output |
| `not` | `not` | a, output | a | output |
| `lessThan` | `less_than` | a, b, bits, output | a, b | output |
| `jubjubScalarFromNative` | `jubjub_scalar_from_native` | native, output | native | output |
| `publicInput` | `public_input` | guard, type, output | optional guard | output |
| `privateInput` | `private_input` | guard, type, output | optional guard | output |
| `output` | `output` | vals | vals | none |

`reads` is a structural inventory, not a guarantee that execution resolves every listed operand: a guarded-off `impact` skips its inputs in `zkir-vm.k`, `#impact`. `reconstituteField` reads modulus before divisor despite the constructor order. Python `pair` enforces exactly two entries for coordinates and low/high conversions, while `div_mod_power_of_two.outputs` remains a list at loading time. The arity of that list is checked by `#checkArity` in module `ZKIR-WF` (`zkir-syntax.k`), and therefore by the `check` command and by `checkedJob` (see [10-well-formedness-and-static-checks.md](10-well-formedness-and-static-checks.md)).

## Rejections and loader fidelity

Python `main` catches `ZkirFormatError`, prints `format error: ` followed by the error's message to standard error, and exits with code 2. The templates in the table are quoted from the preprocessor's builders, and interpolated values use Python representations where shown.

| Builder and rejected condition | Message or message template |
|---|---|
| `program`: non-object root | `expected a JSON object` |
| `program`: absent or malformed version object | `expected a version entry` |
| `program`: invalid version component | `version.{part}: expected u8, got {v!r}` |
| `program`: unsupported version | `unhandled version: {major}.{minor}` |
| `program`: missing required member | `missing field {key!r}` |
| `program`: non-array collection | `{key}: expected a sequence` |
| `program`: malformed input declaration | `input must have name and type, got {entry!r}` |
| `program`: non-boolean commitment flag | `do_communications_commitment must be a boolean` |
| `ir_type`: unsupported type or byte length spelling | `unknown IR type {name!r}` |
| `identifier` / `operand`: non-string | `identifier must be a string, got {text!r}` / `operand must be a string, got {text!r}` |
| `operand`: bare `foo` | `invalid operand format: 'foo'. Variables must start with '%', immediates must start with '0x'` |
| `immediate`: empty hex body | `hex immediate must have at least one digit after '0x'` |
| `immediate`: odd length or invalid character | `invalid hex immediate {text!r}: odd length or non-hex character` |
| `immediate`: excessive length or value | `immediate {text!r} out of range for field element` |
| `operands` / `identifiers`: non-array | `expected a list of operands, got {items!r}` / `expected a list of identifiers, got {items!r}` |
| `pair`: wrong shape or length | `{what} must be a pair, got {items!r}` |
| `u32`: boolean, non-integer or value outside 0 to 2^32 - 1 inclusive | `{what} must be a u32, got {value!r}` |
| `instruction`: missing operation discriminator | `instruction must be an object with "op", got {ins!r}` |
| `_need`: missing instruction members | `{op}: missing field(s) {missing}` |
| `instruction`: unknown operation | `unknown instruction op {op!r}` |
| `alignment`: malformed list, tag or option | `alignment must be a list of segments, got {value!r}`; `unknown alignment atom {atom!r}`; `unknown alignment segment {seg!r}`; `alignment option must carry a list, got {alts!r}` |

`--ext` rejects `reverse_bytes` with `unknown instruction op 'reverse_bytes' (removed at midnight-zkir 2ffe2d1; use reverse)`. The `load_constant` branch of the same mode rejects a non-array encoding with `load_constant encoding must be a list of hex immediates`.

These checks follow the pinned crate's serde field types, tuple sizes, enum tags, operand decoder and version dispatch. Six rows have a counterpart in `ir.rs` with the same message content: `Expected a JSON object` (line 986), `Expected a version entry` (line 963), `Unhandled version: {major}.{minor}` (line 980), the empty hex body (line 223), `Out of range for field element` (line 232) and the bare-variable format message (line 239). The remaining rows stand in for serde-derived errors and for the odd-length and non-hex errors of `const_hex::decode`, whose text the crate does not control. Parser equivalence is not established, because some failures escape the `ZkirFormatError` handler: `json.load` errors, the `0x0\n` immediate above, and a non-string element of an extension `load_constant` encoding (an `AttributeError` from `immediate`).

In `tools/diff_test.py`, `main` sends each preprocessor rejection to the oracle with empty inputs and binding input zero, requiring `status == 'load-error'`, the status its `oracle` helper assigns to nonzero process exits other than panic exit 101. The harness filters to major-version-3 corpus programs and compares rejection status, not message text. Receipts `evidence/zkir-k-differential-92e8bdd3-2026-09-05c.txt` and `evidence/zkir-k-differential-ext-2ffe2d1-2026-09-05c.txt` record agreement for malformed identifiers and odd-length hex. That agreement is finite corpus evidence (see [12-oracles-and-differential-testing.md](12-oracles-and-differential-testing.md)).

## Worked translation and commands

From the repository root, with the artifact on standard input:

```bash
uv run --group zkir-k python experiments/zkir-k/tools/zkir_kast.py kast /dev/stdin <<'JSON'
{
  "version": {"major": 3, "minor": 0},
  "inputs": [{"name": "%x", "type": "Scalar<BLS12-381>"}],
  "outputs": ["Scalar<BLS12-381>"],
  "do_communications_commitment": false,
  "instructions": [
    {"op": "add", "a": "%x", "b": "0x0100", "output": "%y"},
    {"op": "output", "vals": ["%y"]}
  ]
}
JSON
```

`kast` prints the K AST (KAST) JSON of `term.to_dict()` on one line. In constructor notation with typed token leaves, the tree it returns is:

```k
program(0:Int,
  typedIds(typedId("%x":String, Native()), .typedIds()),
  irTypes(Native(), .irTypes()), false:Bool,
  instrs(add(var("%x":String), imm(1:Int), "%y":String),
    instrs(output(operands(var("%y":String), .operands())), .instrs())))
```

The rendering is not input syntax. In the emitted JSON, each application has `node: "KApply"`, a `label` object (`node: "KLabel"`, `name`, `params`), `args`, `arity` and `variable: false`; each leaf has `node: "KToken"`, a `token` string and a `sort` object (`node: "KSort"`, `name`, `params`). A `String` token carries its K quotes inside `token`, as `"\"%y\""`. The `imm` node of the example, verbatim from the output, shows the immediate already reduced to its value:

```json
{"node": "KApply", "label": {"node": "KLabel", "name": "imm", "params": []}, "args": [{"node": "KToken", "token": "1", "sort": {"node": "KSort", "name": "Int", "params": []}}], "arity": 1, "variable": false}
```

`kore` converts the same `Program` to K's core representation (KORE), and `check` executes `zkir-check.k`, whose rule replaces `P:Program` with `wf(P)`. Both default to `zkir-check-kompiled`, while `kast` needs no compiled definition. These commands use existing corpus artifacts:

```bash
uv run --group zkir-k python experiments/zkir-k/tools/zkir_kast.py kore experiments/zkir-k/corpus/handmade/native_bytes.zkir
uv run --group zkir-k python experiments/zkir-k/tools/zkir_kast.py check experiments/zkir-k/corpus/handmade/native_bytes.zkir
uv run --group zkir-k python experiments/zkir-k/tools/zkir_kast.py check experiments/zkir-k/corpus/handmade-negative/divmod_outputs.zkir
```

The second prints `wfOk`. The third prints the pretty-printed form, with spaces around the parentheses, and still exits zero, so callers must inspect the result:

```
wfError ( "div_mod_power_of_two requires exactly 2 outputs" )
```

`--definition` overrides the compiled directory, and `--ext` changes the accepted JSON vocabulary without changing that directory. See [11-tooling-reference.md](11-tooling-reference.md) for integration details.
