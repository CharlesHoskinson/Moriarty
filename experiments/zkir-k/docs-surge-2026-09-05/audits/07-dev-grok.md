# Audit of experiments/zkir-k/docs/07-instruction-reference.md (developer)

Verdict: REVISE
(ACCEPT: no blocking or major findings. REVISE: fixable findings. REJECT:
the chapter must be redrafted.)

Checks performed: compared the 34 `Instr` productions, `symbol(...)` JSON ops, `reads`/`writes` and ZKIR-WF messages with `zkir-syntax.k` 86-121, 134-166, 293-306; compared type dispatch and every off-circuit error string with `zkir-ops.k` (add/mul/neg/inv/testEq/constrainEq/select/ecMul/coordinates/bytes/hashes/`checkBits`/`asBool`); compared `#exec`, `isSpecialEmit`, `resolve`/`resolveBool`/`resolveNatives`, `#divMod`, `#recon`, `#lt`, `#impact`, `#input`, `#output` with `zkir-vm.k` 168-513; compared `eval(gate(...))`, `#eqSupported`, `#matches`, `#chipFor`, `#chipNamed`, `usedChips`, `outputGate`, `piGate`/`guardGate` with `zkir-constraints.k`; grepped every `"op"` in `experiments/zkir-k/corpus/` against the corpus table (assert 19 uses / 13 ledger programs, constrain_eq 63 / 24, private_input 16 programs, and the per-op file lists); ran `grep -rlE '"op"\s*:\s*"less_than"' experiments/zkir-k/corpus/` from the repository root (7 files); ran the shown `zkir_run.py` command as written (`preimage.json` missing, exit 1) and again after writing `handmade/manifest.json`'s `native_bytes` `test_preimage` to `/tmp/native_bytes_preimage.json` (status `ok`, 34 constraints, `verdicts: 34`, `violations: []`); confirmed nine extension ops and `Bool`/`Byte`/`BytesN` in `zkir-ext.k` 18-33; confirmed `typeName` in `zkir-values.k` 73-86, `chipOfType` names, `ir.rs` `Instruction` field order, `val_t` serde rename `"type"`, and the `idx` closure in `ir_vm.rs` 227-231; compared `f01`/`f10` with `tools/divergence_tests.py` 98-102 and 177-182; grepped the chapter for TODO/TBD/em-dashes/extra `#` titles/drafting language (none); confirmed cross-refs `03`, `04`, `05`, `06`, `08`, `09`, `11`, `12`, `13` exist under `experiments/zkir-k/docs/`.

## Findings
### F1 blocking reconstitute_field / "would hold modulo r"
Claim: the `reconstitute_field` gate has no overflow check, so a combination above `r - 1` that the witness rejects "would hold modulo `r`"; cases `f01` and `f10`.
Evidence: Off-circuit `#recon4` fails with `Reconstituted element overflows field` and does not write the output (`zkir-vm.k:376-377`). The gate then does `#matches(#recOf(...), rdId(O, M), "reconstitute_field output")` (`zkir-constraints.k:350-351`). `#recOf` wraps `modInt #r` (`zkir-constraints.k:353`), but `#matches` on a missing register is `unknown(S)` (`zkir-constraints.k:195`), with `S` = `register ... is not in the witness` (`zkir-constraints.k:146`). `divergence_tests.py:177-182` records `f01_reconstitute_overflow` with expected circuit outcome `'unknown'`, not `holds`. After a witness failure this contradicts the chapter's own rule that an unread register is `unknown`.
Fix: "The gate has no overflow check. After the witness fails, the output register is absent, so the verdict is `unknown` (case `f01_reconstitute_overflow`). If that register were present and held the wrapped sum, `#recOf` would hold modulo `r`." Drop `f10` from this sentence (see F2).

### F2 major reconstitute_field / "divergence cases f01 and f10"
Claim: `f10` is an overflow wrap of the linear combination, grouped with `f01`.
Evidence: `divergence_tests.py:98-102` builds `f10_reconstitute_bits_256` with `'bits': 256` and expects `'wfError'` / `'n/a'` for the gate: "bits >= 256 is rejected statically (Rust off-circuit: Excessive bit count; keygen would underflow)". ZKIR-WF rejects `bits > 248` (`zkir-syntax.k:300-301`). That is not the `r - 1` overflow of `f01`.
Fix: cite only `f01` for overflow. Mention `f10` on the Checks line: `ZKIR-WF` (and the off-circuit `Excessive bit count` arm) reject `bits > 248`; case `f10` is `bits = 256`.

### F3 blocking Corpus programs by instruction / shown `zkir_run.py` command
Claim: after saving `test_preimage` to a file, the fenced command from the repository root is a runnable pair.
Evidence: from the repository root, the command as shown is `uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py experiments/zkir-k/corpus/handmade/native_bytes.zkir preimage.json`. It exits 1 with `FileNotFoundError: [Errno 2] No such file or directory: 'preimage.json'`. The chapter never shows how to extract the object from `handmade/manifest.json`. Writing that object to `/tmp/native_bytes_preimage.json` and passing that path does run (status `ok`).
Fix: show the extract step, then a command whose preimage path exists. For example:

```sh
python3 -c "import json,pathlib; m=json.load(open('experiments/zkir-k/corpus/handmade/manifest.json')); json.dump(next(p['test_preimage'] for p in m['programs'] if p['file']=='native_bytes.zkir'), open('/tmp/native_bytes_preimage.json','w'))"
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py \
  experiments/zkir-k/corpus/handmade/native_bytes.zkir /tmp/native_bytes_preimage.json
```

### F4 major Corpus programs by instruction / "the runner prints ... the verdicts"
Claim: `zkir_run.py` prints the status, the final memory with encodings, and the verdicts.
Evidence: the tool prints one JSON object (`zkir_run.py:381`). After the successful `native_bytes` run, `verdicts` is the integer `34` (the count, `zkir_run.py:348`) and the list is `all_verdicts` (triples `[outcome, message, gate]`, `zkir_run.py:336-349`). Non-holds are also under `violations`. A developer who reads the `verdicts` key does not see the per-gate outcomes the chapter just taught them to interpret.
Fix: "The runner prints JSON: `status`, `memory` (each register's `variant` / `type` / `encoded`), `verdicts` (count), `all_verdicts` (per-gate outcome, message, pretty-printed constraint), and `violations`."

### F5 major Summary / `alignedBytes` file
Claim: the summary table attributes `alignedBytes`, `sha256Bytes` to `zkir-hash.k`, and `alignedBytes`, `keccak256Bytes` to `zkir-hash.k`.
Evidence: `alignedBytes` is in `zkir-ops.k:236-237`. `sha256Bytes` and `keccak256Bytes` are in `zkir-hash.k:161` and `231`. The table footnote says a function without a file is in `zkir-ops.k`, so putting `zkir-hash.k` after `alignedBytes` sends the reader to the wrong file.
Fix: `alignedBytes` (`zkir-ops.k`), `sha256Bytes` (`zkir-hash.k`) on the `persistent_hash` row; `alignedBytes` (`zkir-ops.k`), `keccak256Bytes` (`zkir-hash.k`) on the `keccak256` row.

### F6 major cond_select / "or the JubjubScalar text"
Claim: `#eqSupported` gives `synthErr("Unsupported cond_select: Bytes32 ? Bytes32")` or the same shape for `JubjubScalar`.
Evidence: the Bytes32 pair is exact (`zkir-constraints.k:244`). A `JubjubScalar` pair uses the generic rule at `zkir-constraints.k:243`: `Unsupported cond_select: JubjubScalar == JubjubScalar` (`==`, not `?`). Mixed types also use `==` (`zkir-constraints.k:242`). Off-circuit `selectV` still uses `?` (`zkir-ops.k:136`). A developer grepping the in-circuit `JubjubScalar` message for `?` will not find it.
Fix: quote `synthErr("Unsupported cond_select: Bytes32 ? Bytes32")` and `synthErr("Unsupported cond_select: JubjubScalar == JubjubScalar")`.

### F7 major Machinery shared by every entry / `#matches` and quoted hash `synthErr` strings
Claim: `#matches(expected, register, what)` gives `synthErr` when the expected value cannot be computed, and the `persistent_hash` / `keccak256` gates are `synthErr("synthesis: in-circuit decoding of alignment options is not yet implemented")` and `synthErr("synthesis: Cannot decode compressed value from field elements")`.
Evidence: `#matches(vErr(S), _, What)` is `synthErr(What + ": " + S)` (`zkir-constraints.k:193`). For those hashes, `What` is `"persistent_hash output"` / `"keccak256 output"` (`zkir-constraints.k:444-445`) and `S` comes from `alignedBytesCircuit` (`zkir-constraints.k:474-475`). The verdict message is therefore `persistent_hash output: synthesis: in-circuit decoding of alignment options is not yet implemented` (and the compress analogue). The same prefix applies to every unsupported arithmetic/bytes `#matches` (`"add output: Unsupported addition: ..."` and so on).
Fix: in the shared machinery, say `#matches` yields `synthErr(what + ": " + S)` when the expected value is `vErr(S)`, and `violated(what)` when the values differ. In the hash entries, quote the prefixed strings, or say the inner `alignedBytesCircuit` text is the suffix of that `synthErr`.

### F8 major Machinery shared by every entry / missing hash-chip message
Claim: `#chipsForValues` and `#chipFor` give `synthErr("chip not initialised for T")` when a type's chip is absent; hash entries then say the gate needs the `poseidon` / `sha2_256` / `keccak_256` / `jubjub` chip without a second message.
Evidence: type chips use `#chipFor` / `"chip not initialised for " + typeName(T)` (`zkir-constraints.k:165-166`). Hash gates use `#chipNamed` (`zkir-constraints.k:442-445`, 446-448), which is `synthErr("chip not initialised: " + N)` with a colon and the chip name (`poseidon`, not `Poseidon` / `Native`). A developer debugging a missing Poseidon chip who searches for `chip not initialised for poseidon` will miss the real string.
Fix: after the `#chipFor` sentence, add: hash gates call `#chipNamed`, which fails with `synthErr("chip not initialised: poseidon")` (and the same form for `jubjub`, `sha2_256`, `keccak_256`).

## Coverage
- One entry per base-surface instruction (34, in the order of `zkir-syntax.k`): JSON op, K constructor and arguments, off-circuit semantics (the `zkir-ops.k` function and its type dispatch, including every error message), the gate it emits and its in-circuit relation in `zkir-constraints.k` (one sentence, with a pointer to 08-constraints-and-verdicts.md for the outcome model), run-time checks in `zkir-vm.k` (arity, bit counts), and the corpus programs that exercise it: covered (summary table follows `zkir-syntax.k` 86-119; body grouped by family; corpus in the end table; 08 pointer in "How to read an entry")
- Group the entries by family (encoding, assertions, control, curve, bytes, arithmetic, hashing, comparison, transcript) but keep the uniform entry structure: covered
- Include a summary table at the top: op, reads, writes, chip needed, off-circuit function, gate: covered (F5 on two hash rows)
- The extension instructions belong to 09-extension-surface.md; only point there: covered
