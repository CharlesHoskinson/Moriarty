# M-HASH `test_hex_and_type_inputs` / `mixed-case` — root-cause diagnosis

Author: Opus 4.8 (requested claude-opus-4-8 HIGH; fallback claude-opus-5-5 HIGH — both user-authorized).
Actual acting identity honestly recorded: this diagnosis was produced by claude-opus-4-8.
Role: source diagnosis and tightly-scoped test correction only. Not a vote, adoption, waiver or new
authority. One source attempt (~1200 s budget). No execution, no import, no AST/compile/hash/lint
probe, no retry, native authority 0.

## 1. The exact failing observation (authoritative, not inferred)

`ACTUAL-stderr.txt` and `ACTUAL-RESULT.json` (ordinary authorized GREEN result; attempt 1 of max 1;
exit 1; implementation sha256 `4f6af890…`, 9070 B; test sha256 `a1c00918…`, 28091 B) record one
GREEN run of 19 methods: 18 `ok`, one sub-test FAIL and nothing else.

- Failing method: `T01ScriptHash.test_hex_and_type_inputs`, sub-test `case='mixed-case'`.
- Failure site: `assertInputError` at test line 402 → `self.assertRaises(m.ScriptHashInputError)`
  at test line 184.
- Message: `AssertionError: ScriptHashInputError not raised`.
- `errors: 0`; stdout empty; all 20 packet files and both input files unchanged before/after;
  interpreter unchanged; `allPinsModesListingsUnchanged: true`.

Every other implementation-dependent test (`T01.3` CIP vector, `T01.4` B9, the `uppercase`,
`code-uppercase` and `hash-uppercase` case-rejection fixtures, the whole blueprint suite) passed.
So the primitive, the tag, the framing, the bound logic and lowercase-hex rejection all work.

## 2. Exact input construction and data trace of the failing sub-test

The failing fixture is built at the original test line 386 as:

    CIP57_CODE[:10] + CIP57_CODE[10:20].upper() + CIP57_CODE[20:]

`CIP57_CODE` is a frozen public literal — the CIP-0057 `hello_world` `compiledCode`, re-verified
byte-for-byte against the pinned `CIP-0057.md` by `test_t00_primary_source_anchors` and
`test_t00_cip57_vector_framing`, both of which passed in this same run. Taken in 10-character spans
the literal begins:

    index  0-9   58ad010000
    index 10-19  3232322225   <-- the slice the fixture upper-cases
    index 20-29  3330043232
    …            …

The slice `CIP57_CODE[10:20]` is therefore the string of decimal digits `3232322225`. It contains
no characters in `a-f`. `str.upper()` affects only cased letters, so `"3232322225".upper()` returns
`"3232322225"` unchanged. The three concatenated pieces re-assemble the original literal exactly:

    CIP57_CODE[:10] + "3232322225" + CIP57_CODE[20:]  ==  CIP57_CODE

So the `mixed-case` variant handed to the implementation is **byte-identical to `CIP57_CODE`** — the
canonical, admitted hello_world input. Tracing it through the implementation
(`independent_script_hash.py`, actual run bytes `4f6af890…`):

1. `script_hash("v2", CIP57_CODE)` → `_language_tag("v2")` returns `b"\x02"` (Lang:545).
2. `_admit_compiled_code(CIP57_CODE)`: it is a `str`; length 350 ≤ 65536; even; `_is_lower_hex`
   is true because the whole literal is lowercase hex (`_HEX_DIGITS.issuperset`); decodes to 175
   bytes with header `0x58`, length `0xad = 173`, 24 ≤ 173 ≤ 255, total `2 + 173 = 175` — admitted.
3. `_digest` returns `blake2b(b"\x02" + code, digest_size=28).hexdigest()` = `CIP57_HASH`
   (`5e1e8fa8…0a02`).

The call returns the valid 56-hex identifier and raises nothing. `assertInputError` then fails with
`ScriptHashInputError not raised`. This is exactly the recorded stderr, reproduced by data trace
with no execution.

## 3. Root cause (stated precisely, no assumption)

**The negative fixture is malformed, not the implementation.** The fixture's label and intent
(`mixed-case`) require a hex string that mixes lowercase with uppercase `A-F` so the C4 lowercase-hex
rule rejects it. The chosen slice `[10:20]` of the frozen literal happens to contain only decimal
digits, so `.upper()` is a no-op and the "mixed-case" input degenerates into the *accepted canonical
input*. The fixture thus asserts that the valid hello_world `compiledCode` must be rejected — which
directly contradicts `test_cip57_primary_vector` (passing in the same run), which asserts the same
bytes are accepted and hash to `CIP57_HASH`.

The implementation behaved correctly on both: it accepted the canonical input in `T01.3` and, being
handed that same canonical input again under the `mixed-case` label, accepted it again. No contract,
algorithm, tag, framing, bound, vector or oracle value is implicated. Normative M-HASH behaviour
(C1-C8) is unchanged and correct as observed.

The sibling `uppercase` case on the preceding line (`CIP57_CODE.upper()`) upper-cases the whole
literal, including its real `a/b/c/d/e/f` letters, producing a genuinely non-lowercase string that
the implementation correctly rejected — that case passed. Only `mixed-case` is defective. The
blueprint-side `code-uppercase` and `hash-uppercase` fixtures are likewise well formed and passed.

## 4. Why both prior reviews missed it

- **Scope.** Both the design and the test-source audits treated the 19-method test as frozen and
  out of scope: `MHASH-CONTRACT.md:3` and `MHASH-API.json` (`pairedUnchangedTest`) state the test is
  "unchanged and is not touched by this revision," and `MHASH-IMPLEMENTATION-ADDENDUM.md:20` records
  that the audits reasoned about what the "unchanged 19-method test" does and does not *discriminate*.
  The reviews audited contract source-grounding (C3 citations) and implementation semantics / coverage
  gaps; they never audited whether each negative fixture's literal value actually realises its label.
- **The defect is invisible at RED.** The frozen `predictedRED` (`MHASH-API.json`: ran 19, pass 5,
  fail 14, errors 0) is produced with the implementation *absent*. `assertInputError` calls `_impl()`
  first, which raises `AssertionError(RED)` before any variant is constructed or hashed. So under RED
  every `T01`/`T02` method fails with the RED message and the `mixed-case` branch never reaches a real
  `script_hash` call. The predicted RED therefore cannot surface the bug.
- **The defect only surfaces at GREEN**, which the frozen `predictedGREEN` asserted as 19/19 pass.
  That prediction assumed each negative fixture was well constructed; nobody computed the concrete
  value of `CIP57_CODE[10:20].upper()` to notice it equals the admitted input. D7 of the addendum even
  enumerates *future* test additions but never re-examines the existing negative literals.
- **Net:** a latent, label-vs-value mismatch inside a fixture that both reviews had declared immutable
  and that RED structurally masks. GREEN is the first place it can appear, and it did.

## 5. Minimal correction basis and effects

Source proves a malformed negative fixture, so a complete revised `test_independent_script_hash.py`
is written (same directory as this report). Changes are confined to `test_hex_and_type_inputs`:

- The `mixed-case` variant is rebuilt from a slice of the **same** frozen literal that actually
  contains hex letters: `CIP57_CODE[40:50]` is `2e646e6400`, whose `.upper()` is `2E646E6400`.
  The variant is `CIP57_CODE[:40] + CIP57_CODE[40:50].upper() + CIP57_CODE[50:]` — a true mixed-case
  hex string of the identical 175 bytes.
- Three independent, implementation-free literal assertions are added so the fixture can never
  silently degenerate again:
  - `assertNotEqual(mixed_case, CIP57_CODE)` — it is genuinely altered, not a digit-only no-op;
  - `assertTrue(any(c in "ABCDEF" for c in mixed_case))` — it really contains uppercase `A-F`;
  - `assertEqual(mixed_case.lower(), CIP57_CODE)` — the only difference from the admitted input is
    letter case, so the rejection is attributable to case alone.

Nothing else is touched. All 19 method names, their coverage, every label, every fixture, every
oracle constant (`CIP57_CODE`, `CIP57_HASH`, the 7 receipts, the pins, the bounds) and the genuine
CIP and B9 originals are byte-preserved. The `assertInputError` expectation on `mixed-case` is kept,
not weakened. No positive implementation call was added: exact-original acceptance is already an
unambiguous control via `test_cip57_primary_vector` (`T01.3`), so the method count stays 19. No
D7/MC work is folded in. No implementation, contract, API, addendum, framing, tag, vector, CBOR
bytes or bound is altered.

### Before / after predictions (for the revised test; not run here)
- **Before (recorded, retained forever):** GREEN run of the frozen test = 19 ran, 18 ok, 1 FAIL
  (`test_hex_and_type_inputs` `case='mixed-case'`, `ScriptHashInputError not raised`), 0 errors,
  exit 1, pins unchanged. This stands as the actual result; it is not falsified to all-pass.
- **After (predicted, needs a fresh authorized run + audit):**
  - new RED (implementation absent): 19 ran, 5 pass (`T00.1-T00.5`), 14 fail with the RED
    `AssertionError`, 0 errors, exit 1 — the three added assertions sit *after* `m = _impl()` inside
    the method, so they are not reached while the implementation is absent; predicted RED is
    unchanged from the frozen `predictedRED`.
  - new GREEN (same reviewed implementation `4f6af890…`): 19 ran, 19 pass, 0 failures, 0 errors,
    exit 0. The `mixed-case` variant is now rejected by `_is_lower_hex` with `ScriptHashInputError`,
    and the three added literal assertions hold.

These are predictions only. No run was performed and none is authorized by this packet.

## 6. Limits and future gates

- Diagnosis only used the frozen literals, the implementation text, the actual stderr/result/launch
  artifacts, the contract, API (both embedded predictions), and the addendum. Nothing was executed,
  imported, compiled, hashed, linted or AST-parsed.
- The revised test is a **new artifact** with new bytes and a new sha256; it is not the frozen
  `a1c00918…` test and does not claim to be. Before it can drive any decision it needs: a fresh full
  source/effects Opus review of the whole changed file, explicit new-RED and new-GREEN decisions, and
  an actual-result audit under the reviewed envelope (one attempt, no retry).
- No semantic waiver, no new retry of the original run, no native authority is granted. The prior
  failed 19-method run is retained permanently and is not re-labelled all-pass.
- M-HASH remains OPEN. All 20 UTxO families, full finance / ACTUS / DeFi obligations, PCD/recursion,
  private handoff, multi-parent composition and real Preprod remain OPEN. Native authority 0.
- Shared-primitive, source-connection and dependency-correspondence limitations in
  `MHASH-CONTRACT.md` / `MHASH-API.json` are unchanged; this correction neither strengthens nor
  weakens them.
