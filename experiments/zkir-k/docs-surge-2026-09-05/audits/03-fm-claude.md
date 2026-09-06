# Audit of experiments/zkir-k/docs/03-program-model.md (formal methods expert)

Verdict: ACCEPT
(No blocking or major findings. The minor findings below are worth fixing
before publication; F1 and F5 concern text that the published chapter will
carry.)

Checks performed:
- Compared the `Program`, `TypedId`, list sorts, `IrType` (13 constructors), `encodedLen` (13 rules), `Operand`, `Guard`, `Alignment`/`Segments`/`Segment`/`Alignments`/`Atom` declarations with `experiments/zkir-k/semantics/zkir-syntax.k` lines 20-81.
- Compared all 34 rows of the instruction table (K constructor, `symbol(...)`, argument order, `reads`, `writes`) with `zkir-syntax.k` lines 86-119, 135-170 and 173-206, and with the `Instruction` enum field order in `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir.rs` lines 360-920 (serde attribute `#[serde(rename_all = "snake_case", tag = "op")]` at line 358).
- Compared the 13 `encodedLen` values with `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir_types.rs` lines 93-114 and the serde renames at lines 39-88.
- Read `experiments/zkir-k/tools/zkir_kast.py` in full and checked every message template in the rejection table against the `raise ZkirFormatError(...)` sites (lines 84, 97, 100, 102, 105, 108, 114, 120, 127, 133, 139, 145, 151, 163, 177, 181, 184, 196, 201, 253, 274, 325, 333, 336, 340, 342, 345, 348, 352, 356), the CLI exit code (line 402) and the default definition (line 378).
- Ran `immediate`, `operand`, `ir_type`, `alignment`, `guard`, `instruction` and `program` from `zkir_kast.py` on probe inputs: `0x0100` gives 1, `0x0001` gives 256, `-0x01` gives r-1, `0X0a` gives 10, 33 bytes and the 32-byte all-ones value are rejected with the out-of-range message, `foo`/`x`/`-foo` give the operand format message, `Bytes<0>`/`Bytes<01>`/`Bytes<16777217>` are rejected and `Bytes<16777216>` accepted with `--ext`, `Bytes<32>` gives `Bytes32`, `0x0\n` raises an unwrapped `ValueError`, `load_constant` with encoding `[1]` raises an unwrapped `AttributeError`, version parts `True`, `3.0`, `256` give the u8 message and `2.0`/`3.1` give the unhandled-version message.
- Ran the worked example command (`kast /dev/stdin` with the heredoc) from the repository root and rendered the returned KAST as constructor notation; it is exactly the term shown in the chapter. Confirmed the JSON node shapes (`KApply` with `label`, `args`, `arity`, `variable: false`; `KToken` with `token` and `sort`).
- Ran the two `kore` and `check` commands on `experiments/zkir-k/corpus/handmade/native_bytes.zkir` (exit 0, `check` prints `wfOk`) and `check` on `corpus/handmade-negative/divmod_outputs.zkir` (prints `wfError ( "div_mod_power_of_two requires exactly 2 outputs" )`, exit 0).
- Read `ir.rs` lines 155-246 (`Identifier`, `TypedIdentifier`, `Operand` serialize/deserialize), 874-882 (`PublicInput` guard `Option<Operand>`), 918-990 (`SerdeVersion` with `u8` fields, `IrSource::load` version rewrite and error strings), and `Impact { guard: Operand, inputs: Vec<Operand> }`; confirmed no `deny_unknown_fields` anywhere in `ir.rs`.
- Read `repos/_build/ledger-92e8bdd3/transient-crypto/src/curve.rs` lines 346-354 (`Fr::from_le_bytes` returns `None` above 32 bytes, then `from_repr` canonical check) and `repos/_build/ledger-92e8bdd3/base-crypto/src/fab/encoding.rs` lines 183-184, 266-277 and 637-650 (`Alignment`, `AlignmentSegment` tagged `tag`/`value`, `AlignmentAtom::Bytes { length: u32 }`).
- Read `const-hex` 1.19.0 (pinned in `repos/_build/ledger-92e8bdd3/Cargo.lock` line 972) `src/lib.rs` `decode` and `strip_prefix` (lines 523-534, 702-707).
- Read `experiments/zkir-k/tools/diff_test.py` lines 50-60 (`oracle`: 101 is panic, other nonzero exits are `load-error`) and 186-201 (major-version filter, rejection path with `{'inputs': [], 'binding_input': '0'}`, `status == 'load-error'`).
- Grepped `evidence/zkir-k-differential-92e8bdd3-2026-09-05b.txt` (lines 133-135) and `evidence/zkir-k-differential-ext-2ffe2d1-2026-09-05b.txt` (lines 229-231, 402) for the `format` rows.
- Read `experiments/zkir-k/semantics/zkir-ext.k` lines 18-23 (`boolT`, `byteT`, `bytesT(Int)` with `symbol(BytesN)`), `repos/_build/midnight-zkir-2ffe2d1/zkir/src/ir_types.rs` lines 45 and 157-190 (`MAX_BYTES_LEN = 1 << 24`, `from_type_string`), `experiments/zkir-k/semantics/zkir-check.k` (rule `P:Program => wf(P)`), `experiments/zkir-k/semantics/zkir-vm.k` lines 463-466 (`#impact(boolOk(false), Xs) => .K`), and `wiki/zkir-k-semantics-plan.md` line 57 (CLM-0712).
- Checked the working checkout `repos/midnightntwrk/midnight-ledger` (HEAD `a8ab82ba2124c36f92795c683e70bd888bc1d1fb`, `Secp256k1Point => 8` at `zkir-v3/src/ir_types.rs` line 75) against the maintainers' note.
- Style: exactly one `#` title, no em-dash, no placeholder or drafting vocabulary, about 1330 words outside tables and code blocks.

## Findings

### F1 minor "Notes for maintainers", first paragraph
Claim: `repos/midnightntwrk/midnight-ledger/zkir-v3/src/ir_types.rs:68` in the current checkout gives `Secp256k1Point` an encoded length of 8.
Evidence: In that checkout (HEAD `a8ab82ba2124c36f92795c683e70bd888bc1d1fb`) `grep -n 'Secp256k1Point => 8' zkir-v3/src/ir_types.rs` reports line 75, not 68. The pinned copy `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir_types.rs` line 100 gives 5, as the note says.
Fix: Replace `ir_types.rs:68` with `ir_types.rs:75`.

### F2 minor "Rejections and loader fidelity", sentence "These checks follow the pinned crate's serde field types, tuple sizes, enum tags, operand decoder and version dispatch."
Claim: The chapter says which crate mechanisms the checks follow but not which individual rejections have a counterpart in `IrSource::load` and which are the preprocessor's own approximation of a generic serde error.
Evidence: `ir.rs` carries the same message content for six of them: "Expected a JSON object" (line 986), "Expected a version entry" (line 962), "Unhandled version: {major}.{minor}" (line 980), "hex immediate must have at least one digit after '0x'" (line 224), "Out of range for field element" (line 232) and "Invalid operand format: '{s}'. Variables must start with '%', immediates must start with '0x'" (line 239). The remaining rows (`missing field`, `expected a sequence`, `must be a pair`, `must be a u32`, `unknown IR type`, `unknown instruction op`, the alignment messages, the version u8 message) correspond to serde-derived errors whose text the crate does not control, and `const_hex::decode` produces the odd-length and non-hex errors. The chapter brief lists "which rejections match" as a must-cover item.
Fix: After the quoted sentence add one sentence naming the six messages above as direct counterparts (with `ir.rs` line numbers) and stating that the other rows stand in for serde-derived and `const_hex` errors that the harness compares only by status.

### F3 minor "Operands, guards and alignments", sentence "Pinned Rust `ir.rs`, `Operand::deserialize`, uses `const_hex::decode` and `Fr::from_le_bytes` for the same byte interpretation and range boundary."
Claim: Read together with the preceding paragraph ("require hexadecimal digits of even length"), a reader infers that the two decoders accept the same immediate spellings.
Evidence: `const_hex::decode` (const-hex 1.19.0, `src/lib.rs` lines 528 and 702-707) strips one further `0x` or `0X` prefix from its input before decoding, so the crate loads `"0x0x01"` as the value 1, while `zkir_kast.py` line 101 rejects it with the odd-length or non-hex message. The byte interpretation and the 32-byte and below-r boundary are the same, as the chapter says; the accepted spellings are not.
Fix: Append to the sentence: "The crate's decoder additionally tolerates one repeated `0x` or `0X` prefix inside the digits (`const_hex::decode` strips it); the preprocessor rejects that spelling."

### F4 minor "Worked translation and commands", sentence "A completed static rejection prints `wfError(...)` but still exits zero"
Claim: The rejection is printed as `wfError(...)`.
Evidence: `zkir_kast.py` lines 384-387 return the pretty-printed slice, which K spaces as `wfError ( "..." )`; the actual output for `corpus/handmade-negative/divmod_outputs.zkir` is `wfError ( "div_mod_power_of_two requires exactly 2 outputs" )`. A reader matching output on `wfError(` will not match.
Fix: Write "prints `wfError ( "..." )` (the pretty-printed form, with spaces around the parentheses)" and give the example output above.

### F5 minor "Operands, guards and alignments", sentence "Its `HEX` check and byte decoder reject whitespace"
Claim: Whitespace is rejected.
Evidence: `zkir_kast.py` line 87 uses `$`, which matches before a trailing newline, so `immediate("0x0\n")` passes the `HEX` check (length 2, even) and `bytes.fromhex` at line 103 raises `ValueError: non-hexadecimal number found in fromhex() arg at position 1`, which `main` does not catch (line 400 catches only `ZkirFormatError`). The value is still rejected, but not through the `format error:` path and not with exit code 2. The chapter records this only in "Notes for maintainers", which is removed before publication.
Fix: In the main text write "reject whitespace, except that a trailing newline after an odd number of digits escapes the `HEX` check and is rejected by `bytes.fromhex` with an uncaught `ValueError` rather than a `format error:` line" (or, if the tool is fixed first, leave the sentence as it is).

### F6 minor "Artifact boundary", "integers outside `0..255`" and "Rejections and loader fidelity", "value outside `0..2^32-1`"
Claim: Ranges written with `..`.
Evidence: `zkir_kast.py` line 339 accepts `0 <= v <= 255` and line 150 accepts `0 <= value < 2**32`; both are inclusive of the upper value written. In Rust notation, which the chapter quotes elsewhere (`SerdeVersion`, `u8`), `0..255` excludes 255.
Fix: Write "outside 0 to 255 inclusive" and "outside 0 to 2^32 - 1 inclusive".

### F7 minor "Instruction constructors", sentence "Its arity is checked later."
Claim: The `div_mod_power_of_two` output count is checked later, without saying where.
Evidence: `zkir-syntax.k` lines 296-297, rule `#checkArity(divModPowerOfTwo(_, _, Os), _) => wfError("div_mod_power_of_two requires exactly 2 outputs")` in module `ZKIR-WF`, applied by `wf` and therefore by `checkedJob` and the `check` command.
Fix: Write "Its arity is checked by `#checkArity` in module `ZKIR-WF` (`zkir-syntax.k`), so by the `check` command and by `checkedJob`; see 10-well-formedness-and-static-checks.md."

## Coverage
- The ZKIR v3 artifact (JSON shape, identifiers, immediates, guards, alignments; `ir.rs` serde and `zkir_kast.py`): covered.
- The K abstract syntax in `zkir-syntax.k` (Program, TypedId(s), IrType with `encodedLen`, Operand, Guard, Alignment/Segment/Atom, 34-instruction table with constructor, op, argument order, reads, writes; `symbol(_)`, `terminator-symbol`, why no concrete syntax): covered; all 34 rows and 13 lengths verified.
- How `zkir_kast.py` builds the term with pyk (HEX handling, `%` variables, unknown members ignored, u8 version parts, `Bytes<n>` under `--ext`, rejections and messages, `kore`/`kast`/`check`): covered; see F5 for the one whitespace edge.
- Fidelity to `IrSource::load` (which rejections match, how the harness verifies rejections): partly; the harness mechanism and receipts are covered, the per-rejection correspondence is given only in aggregate (F2, F3).
- Worked example (small JSON program and the exact term via `kast`): covered; the term is exactly what the command produces.
