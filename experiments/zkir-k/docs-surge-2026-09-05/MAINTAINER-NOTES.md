# Notes for maintainers collected from the chapter drafts

(kept out of the published chapters)

## From 04-values-and-encoding
- wiki/zkir/zkir-type-system.md lines 136-142 and 184 describe foreign field elements as a 248-bit lower limb and an 8-bit upper limb; the K layout and midnight-circuits 7.2.4 use 64-bit limbs (51-bit for the Curve25519 scalar), three (four) limbs, after subtracting one; the two elements carry 192 + 64 bits (204 + 51 bits). The wiki page is wrong.
- repos/midnightntwrk/midnight-ledger working tree is at a8ab82ba, not 92e8bdd3 (its ir_types.rs has seven variants). Pinned sources are under repos/_build/ledger-92e8bdd3/.
- diff_test.py ERROR_CLASSES: no class matches the K message "assertion failed: Bytes32 low element ..." nor the Rust panic text, so a both-panic comparison would compare two fallback classes; none occurs in the current receipts.
- zkir-vm.k line 454: the vPanic branch of #decodeSlice is unreachable (the #input rules already require enough transcript); reachable panics are the #panicNow rules at lines 433 and 443.
- zkir-vm.k line 130: message prints the bare identifier where preprocess prints Identifier("%x"); same error class.

## From 02-getting-started
- `tools/divergence_tests.py` case `k02_transcript_too_short_panics` (the note string on the `CASES.append` around line 175) still says "K reports an error" while the expected statuses on the same tuple are `'panic', 'panic'` and a live run prints `K=panic`. The note is stale relative to the expected fields.
- `tools/zkir_run.py` does not catch `ZkirFormatError`. Loading `bool_via_neg.zkir` without `--ext` ends in a traceback whose last line is `zkir_kast.ZkirFormatError: unknown IR type 'Bool'`. `zkir_kast.py` itself prints `format error: …` and exits 2.
- `unit_hash.py`, `diff_test.py` and `divergence_tests.py` resolve the oracle as `Path.home() / 'Moriarty/repos/_build/ledger-92e8bdd3/target/release/zkir-oracle'` (and the 2ffe2d1 path for `--ext`). A checkout that is not under `~/Moriarty` will not find the binary even when `repos/_build/` exists in the worktree.

## From 03-program-model
`repos/midnightntwrk/midnight-ledger/zkir-v3/src/ir_types.rs:68` in the current checkout gives `Secp256k1Point` an encoded length of 8. The expected base revision `92e8bdd3` gives 5, matching `zkir-syntax.k`, `encodedLen`. The checkout's `HEAD` is `a8ab82ba2124c36f92795c683e70bd888bc1d1fb`; source comparisons here use `git show 92e8bdd3:...` to read the intended base content.

`tools/zkir_kast.py:101` permits a final newline through the regular expression when the preceding digit count is odd. For `immediate("0x0\n")`, line 103 then raises `ValueError: non-hexadecimal number found in fromhex() arg at position 1`, rather than the expected `ZkirFormatError`. The value is rejected, but the CLI does not wrap the error.

`tools/zkir_kast.py:275`, the `load_constant` encoding comprehension, passes each element directly to `immediate`. With extension mode enabled, an encoding array containing integer `1` raises `AttributeError: 'int' object has no attribute 'startswith'`, rather than a format diagnostic for the non-string element.

## From 05-fields-curves-and-hashes
`fsqrt` and `legendre` in `zkir-field.k` are marked `[function, total]` but
unlike `fadd` they have no `[owise]` rule. `fsqrt` matches first on
`A modInt P ==Int 0`. For `P <= 0` that remainder is not defined in `INT`,
so totality holds for the eight configured primes and not for a zero or
negative modulus. Expected: the same `P > 1` / `[owise] =>` convention as
the arithmetic operations. Observed: no such clause; the functions are
never called with `P <= 0`.

## From 07-instruction-reference
- `zkir-constraints.k`, lines 258 and 259: `#selBit` has two rules both marked `[owise]`, `#selBit(_, vOk(A), vOk(B))` and `#selBit(_, _, _)`. The first is reached only when the bit is neither 0 nor 1, which `#boolean` has already turned into `violated` through `#and`, so the result is not observable; the double `owise` is still an overlap the K compiler resolves by rule order.
- `zkir-constraints.k`, lines 494 to 496: `#opName` has a single `[owise]` rule returning `"instruction"`, so the `unsupported` outcome never names the instruction it was raised for.
- `zkir-constraints.k`, line 8, header comment: it names `piFixed(i, _)` as the constraint that pins the binding input and the commitment; the constructors are `bindGate(Int)` and `commGate(Int, TypedIds, Int)`.
- `zkir-ops.k`, line 133 and line 342: `constrainEqV` reports `Equality constraint failed` without the crate's `: {a:?} != {b:?}` suffix, and `checkBits` prints the integer with `Int2String` where the crate prints the `Fr` debug form. Both are class-equal for the differential harness; noted only because the entries state that the texts follow the crate closely rather than exactly.

## From 08-constraints-and-verdicts
- `zkir-constraints.k` header still names `piFixed(i, _)` as the constructor that pins the binding input and the commitment. The constructors in the syntax are `bindGate` and `commGate`. Expected: the header to match the syntax. Observed: stale `piFixed` in the comment at the top of the file.
- `#opName` has only the `[owise]` rule `rule #opName(_) => "instruction" [owise]`. An `unsupported` verdict therefore always says `"this instruction: instruction"` and never names the unmatched constructor.
- `#unsat` is defined as `rule #unsat(O, _) => O` and is never applied.

## From the chapter 03 fix pass
- Immediate spelling `0x0x01`: the crate's `const_hex::decode` (1.19.x, `strip_prefix` twice) accepts a repeated `0x` prefix and the pinned oracle loads such a program; `zkir_kast.py` rejects it (exit 2). Format-only divergence; candidate row for wiki/contradictions.md.
- Trailing newline in an immediate (`0x0\n`) passes the HEX pattern and raises an uncaught ValueError in `bytes.fromhex` (exit 1); a non-string `load_constant` encoding element raises AttributeError under `--ext`. Both should become ZkirFormatError.

## From 09-extension-surface
- `zkir-values.k:165`, inherited by the extension, produces `decPanic` for malformed two-element `Bytes<32>` encodings; `#canonical` preserves it. The expected extension behavior is an error: Rust `ir_instructions/encode.rs:144`, `decode_bytes`, returns `None` for nonzero high bytes, and `decode_offcircuit` converts that to an error. This is a source-level mismatch.
- `zkir-ext.k:274`, the `sha512` gate, reuses `#sha512V`, which calls `alignedBytes` at line 257. The expected in-circuit restriction is rejection of alignment options: Rust `ir_vm.rs:1059` calls `fab_decode_to_bytes`, whose option arm at line 140 rejects them. The definition's corresponding restriction exists in `zkir-constraints.k`, `alignedBytesCircuit`, but this gate does not call it. This is a source-level mismatch.

## From 10-well-formedness-and-static-checks
- `tools/zkir_kast.py`, `default_definition` (line 377) and the `check` branch of `main`: `check --ext FILE` on a program that uses an extension type or instruction ends with an uncaught `KeyError: 'Bool'` from pyk's `kast_to_kore` (exit 1), because the default definition is `zkir-check-kompiled`, which does not import ZKIR-EXT-SYNTAX. `--definition semantics/zkir-ext-kompiled` does not help: that definition's `<k>` cell expects a `Job`, so the `Program` term is left unrewritten and `wf_result` prints `unexpected: <generatedTop> ...`. Expected a clear error for `check --ext` or an extension check module; observed the traceback. The chapter documents `zkir_run.py --ext --checked` as the working route.
- `tools/check_corpus.py`, docstring (line 1) and `CORPORA`: the docstring says "every ZKIR v3 program in the corpus", but `corpus/handmade`, `corpus/midnight-zkir-2ffe2d1-tests` and `corpus/divergence` are not in `CORPORA` (the extension tests could not be checked by this script anyway, see the previous note). The chapter lists the five directories that are checked.

## From 11-tooling-reference
- `zkir_run.py` module docstring lists only `ok`/`error` and `{variant, encoded}` in memory. `Runner.run` also returns `panic`, `stuck`, `depth-exhausted`, `krun-failed`, `type` on each memory record, `needs`, `verdicts`, `all_verdicts`, `violations`, `outputs` and `k_cell`.
- `preimage_term` does not check that `communications_commitment` is a two-element sequence. A present non-pair raises `IndexError` / `TypeError` instead of exit 2.
- `unit_hash.py` line 73: `NamedTemporaryFile(..., delete=False)` leaves the preimage JSON in the process temp directory.
- `divergence_tests.py` note on `k02_transcript_too_short_panics` says "K reports an error"; the expected K status in the same tuple is `panic`.
- `gen_constants.py` has no `argparse`; a missing directory argument is `IndexError`.
- `zkir_values.ENCODED_LEN` has no `Bool` / `Byte` / `Bytes<n>` keys. `diff_test.py` falls back to `len(random_encoded(...))` for those types when it builds a typed perturbation.

## Orchestrator actions
- zkir-check.k now imports ZKIR-EXT-SYNTAX so `zkir_kast.py check --ext` works (re-kompiled zkir-check).
- zkir_run.py: added --gen, --depth, format/preimage error exits.
- K6 (found by the chapter 04 fix pass, verified against the 2ffe2d1 oracle): a non-canonical Bytes<32> raw element (low element >= 2^248 or high element >= 256) is an assert panic in 92e8bdd3 (`encode.rs` assert_eq!, modelled as decPanic) but an ordinary error in 2ffe2d1 (`decode_bytes` returns None: "Failed to decode ... as Bytes(32)"). Fixed in zkir-values.k: `#canonical(decPanic(_), _, bytes32()) => decErr("Failed to decode as Bytes32")` (strict decoding is the 2ffe2d1 path). Oracle check: K error "Failed to decode as Bytes32" vs oracle2 error "Failed to decode [...] as Bytes(32)" on test_bytes32_proof.zkir with inputs [1,1,0,1,0,2^248,0]; canonical input: both panic (transcript too short). unit_values.py now 43/43 (new check "dec bytes32 bad high, strict"). To do at commit time: wiki/contradictions.md row K6, definition page, receipts (unit 43/43, rerun both differential suites after the rebuild).

## From 12-oracles-and-differential-testing
`tools/divergence_tests.py:235` checks expected statuses and the selected gate outcome but never checks the Rust panic location or error class. For `k02_transcript_too_short_panics`, a regression intended to establish the transcript-slice panic site needs that additional predicate; the current test only prints the diagnostic.

## From 13-known-divergences
- `experiments/zkir-k/semantics/zkir-ext.k` lines 184 to 185, `#eqSupported(vOk(bytesV(_)), vOk(bytes32(_)), "test_eq")` and its mirror: they overlap with the generic rule at `zkir-constraints.k` line 242 (`requires notBool sameType(A, B)`). Both give `synthErr` with different messages; the run above produced `Unsupported test_eq: Bytes32 == Bytes`, not `byte strings of different length`. Expected one message; saw the generic one chosen.
- `experiments/zkir-k/semantics/zkir-vm.k` lines 450 to 452, comment above `#decodeSlice`: it says the short-transcript read "here is an error", while the rules set `panic(...)`. Expected the comment to say panic.
- `experiments/zkir-k/tools/divergence_tests.py` line 175, case `k02_transcript_too_short_panics`: the note ends "K reports an error", while the expected K status is `panic`. Expected the note to say panic.

## From 15-design-rationale-and-limits
`wiki/zkir/zkir-k-definition.md:53` says escrow, swap and micro-dao programs never reach a successful generated witness. This predicts no successful swap row, but `evidence/zkir-k-differential-92e8bdd3-2026-09-05b.txt` records `moriarty-core-swap expire.zkir run2` with `K=ok Rust=ok`. The coverage statement needs the swap exception.
- Orchestrator: removed the two overlapping #eqSupported bytesV/bytes32 test_eq rules from zkir-ext.k (generic different-types rule applies); fixed the stale #decodeSlice comment in zkir-vm.k and the k02 note in divergence_tests.py; rebuilt zkir and zkir-ext.
- From the chapter 15 audit: wiki/zkir/zkir-k-definition.md (CLM-0728 paragraph) says the escrow, swap and micro-dao programs never reach a successful witness with generated inputs; the base receipt shows `swap expire.zkir run2` as a successful-run agreement. Correct the sentence to "with one exception (swap expire.zkir)" at commit time.
- Orchestrator: sha512 gate now decodes its alignment with alignedBytesCircuit (crate shares fab_decode_to_bytes with persistent_hash/keccak256 in circuit); new helpers #sha512C/#sha512C2 in zkir-ext.k; rebuilt zkir-ext.
