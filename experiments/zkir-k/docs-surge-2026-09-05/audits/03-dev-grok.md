# Audit of experiments/zkir-k/docs/03-program-model.md (developer)

Verdict: REVISE
(ACCEPT: no blocking or major findings. REVISE: fixable findings. REJECT:
the chapter must be redrafted.)

Checks performed: ran the three commands in "Worked translation and commands" from the repository root (`kast /dev/stdin` heredoc, `kore` and `check` on `experiments/zkir-k/corpus/handmade/native_bytes.zkir`); reconstructed the compact term from the `kast` JSON and compared it with the displayed tree; ran `check` on a program with `%missing` and recorded stdout and exit code; fed `zkir_kast.py kast /dev/stdin` with each rejection-table condition plus `--ext` `reverse_bytes`, non-list `load_constant` encoding, integer `load_constant` encoding, extra JSON member, version minor 255/256, omitted/null `guard`, missing `impact.guard`, alignment example, `Bytes<4>`/`Bytes<32>`/`Bytes<0>`/`Bytes<01>` with `--ext`, and `0x0001`; reproduced `immediate` on `"0x0\n"` (HEX match, `bytes.fromhex` message, CLI exit 1 traceback); compared the instruction table with `zkir-syntax.k` `Instr`/`reads`/`writes` (lines 86-206) and `tools/zkir_kast.py` `instruction`; compared the 13-type/`encodedLen` table with `zkir-syntax.k:31-60` and `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir_types.rs:93-113`; compared `Bytes<n>` text with `zkir_kast.py:73-84` and `zkir-ext.k:18-20`; compared `IrSource::load`/`SerdeVersion`/`Operand::{serialize,deserialize}`/`Instruction` serde with `repos/_build/ledger-92e8bdd3/zkir-v3/src/ir.rs:37-50,188-245,358,922-988`; compared `from_type_string`/`MAX_BYTES_LEN` with `repos/_build/midnight-zkir-2ffe2d1/zkir/src/ir_types.rs:45,159-191`; compared `#impact` skip with `zkir-vm.k:463-466`; compared harness text with `tools/diff_test.py:52-60,165-201` and receipts `evidence/zkir-k-differential-92e8bdd3-2026-09-05b.txt:133-135` and `evidence/zkir-k-differential-ext-2ffe2d1-2026-09-05b.txt:229-231`; compared CLM-0712 with `wiki/zkir-k-semantics-plan.md:54-57`; compared `zkir-check.k:13` with the `check` description; verified maintainer-note HEAD `a8ab82ba2124c36f92795c683e70bd888bc1d1fb` and `encoded_len` in `repos/midnightntwrk/midnight-ledger/zkir-v3/src/ir_types.rs`; grepped the chapter for `TODO`, `TBD`, em-dashes, extra `#` titles, and "this document".

## Findings
### F1 major "Its `HEX` check and byte decoder reject whitespace"
Claim: Python `immediate` rejects whitespace via `HEX` and the byte decoder, and such inputs take the documented `ZkirFormatError` path (`format error: …`, exit 2). The later caveat names only `json.load` errors and malformed extension payloads as escapes.
Evidence: `tools/zkir_kast.py:87` is `HEX = re.compile(r'^[0-9a-fA-F]+$')`. Python `$` matches before a trailing newline. For body `0\n`, `HEX.match` succeeds and `len(hex_str) % 2 == 0`, so line 101 does not raise. Line 103 `bytes.fromhex` ignores whitespace and then raises `ValueError: non-hexadecimal number found in fromhex() arg at position 1`. CLI `kast` on `"b": "0x0\n"` exited 1 with that traceback, not `format error` / exit 2. `bytes.fromhex` does not reject whitespace; it skips it. The catch-all at lines 147-148 does not mention a base-surface immediate. Notes for maintainers already record this case.
Fix: Replace the whitespace sentence with: the `HEX` pattern requires even-length `[0-9a-fA-F]` and rejects internal whitespace; Python `$` still allows a final newline, and `immediate("0x0\n")` then raises `ValueError` from `bytes.fromhex` so the CLI exits 1 without the `format error:` prefix. In the catch-all, list that base-surface case beside `json.load` and the `load_constant` non-string encoding element.

### F2 minor "each application has `node: \"KApply\"`, a `KLabel`, `args`, `arity` and `variable: false`"
Claim: those names are the fields of the `kast` JSON object a developer will parse.
Evidence: the heredoc `kast` command printed `"node": "KApply", "label": {"node": "KLabel", "name": "program", "params": []}, "args": [...], "arity": 5, "variable": false`. Token leaves were `"node": "KToken", "token": "0", "sort": {"node": "KSort", "name": "Int", "params": []}`. There is no top-level key `KLabel` or `KSort`. String tokens include K quotes, e.g. `"token": "\"%x\""`.
Fix: Name the JSON keys: `label` (object with `node: "KLabel"`, `name`, `params`) and `sort` (object with `node: "KSort"`, `name`). Note that a String token's `token` field includes the escaped quotes.

### F3 minor "A completed static rejection prints `wfError(...)` but still exits zero"
Claim: the `check` CLI prints `wfError(...)`.
Evidence: `check` on a program whose `output` reads `%missing` printed `wfError ( "undefined variable %missing" )` and exited 0. `tools/zkir_kast.py:384-387` copies a substring from the pretty-printer starting at `wfError (`. Successful `check` of `native_bytes.zkir` printed `wfOk` (no parentheses), as the chapter says.
Fix: Write that a completed static rejection prints `wfError ( "…" )` with spaces around the parentheses, and still exits 0. Keep `wfOk` as the success spelling.

### F4 minor Notes for maintainers, `ir_types.rs:68`
Claim: `repos/midnightntwrk/midnight-ledger/zkir-v3/src/ir_types.rs:68` gives `Secp256k1Point` encoded length 8.
Evidence: that file at `HEAD` `a8ab82ba2124c36f92795c683e70bd888bc1d1fb` has `pub fn encoded_len` at line 68 and `IrType::Secp256k1Point => 8` at line 75. The substance (current checkout 8 versus `92e8bdd3` 5) is true (`repos/_build/ledger-92e8bdd3/zkir-v3/src/ir_types.rs:100`).
Fix: Cite `ir_types.rs:75` for the `Secp256k1Point => 8` arm, or say `encoded_len` starting at line 68.

### F5 minor "Python `klist` constructs them from right to left"
Claim: list construction is right-to-left, without saying the resulting term keeps JSON order.
Evidence: `tools/zkir_kast.py:66-70` starts from the terminator and conses `reversed(items)`, so the first JSON element is the list head. The worked example correctly has `add` then `output`. A reader who only has the "right to left" sentence could reverse `instructions` when building a term by hand.
Fix: After that sentence, add that the walk is from the last JSON element so the first JSON element remains the head (`typedIds(typedId("%x", …), .typedIds)`, not the reverse).

## Coverage
- ZKIR v3 artifact JSON shape (version, inputs, outputs, do_communications_commitment, instructions), identifiers, immediates, guards, alignments; `ir.rs` serde and `zkir_kast.py`: covered (F1 is the HEX/whitespace mechanism).
- K abstract syntax in `zkir-syntax.k`: Program, TypedId(s), IrType (13 types with encodedLen), Operand, Guard, Alignment/Segment/Atom, Instr (34-row table: K constructor, JSON op, argument order, reads/writes); `symbol(_)` / `terminator-symbol`; why there is no concrete syntax: covered.
- `zkir_kast.py` / pyk: HEX (even length, little-endian, value below r), `%`-prefixed variables, unknown members ignored, u8 version parts, `Bytes<n>` in the extension, rejection messages, `kore`/`kast`/`check`: covered (F1 qualifies HEX whitespace; F2/F3 qualify `kast`/`check` output spelling).
- Fidelity to `IrSource::load` (which rejections match) and how `diff_test.py` verifies rejections: covered.
- Worked example: small JSON program and the exact term from `kast`: covered (compact rendering matches the JSON spine; F2 is about the JSON field names around it).
