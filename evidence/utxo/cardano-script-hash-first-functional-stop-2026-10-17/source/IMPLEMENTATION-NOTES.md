# M-HASH independent script-hash checker: implementation notes (PROPOSED SOURCE)

Author: claude-opus-5-5, high effort, security/crypto source author. This is **not a vote**, approval or adoption. Nothing was executed, imported, compiled, hashed, tested or fetched. Every runtime statement below is a source-level prediction.

Source readiness is false. Before any GREEN, a fresh non-author audit of the whole candidate is required, plus two fresh agreeing independent Opus 5.5 design votes on the candidate and the addendum.

| File | Purpose |
| --- | --- |
| `independent_script_hash.py` | The complete pure-stdlib checker implementing C1-C8 and the frozen API |
| `IMPLEMENTATION-NOTES.md` | This file: source, API, effects, coverage, predictions, actual-RED qualification |
| `MHASH-IMPLEMENTATION-ADDENDUM.md` | Separate proposed clarifications D1-D8. Not accepted. The original contract, API and test are immutable |

No test, fixture, PINS, manifest or packet file was written, edited or copied.

## 1. Actual-RED qualification (author premise, not independent approval)

I read these files directly:
- `ROOT-RED-DECISION.json`, `ROOT-RED-RESULT.json`, `ROOT-RED-MANIFEST-PRECHECK.json`, `ROOT-RED-stdout.txt` (0 B) and `ROOT-RED-stderr.txt` (full transcript);
- `ORIGINAL-AUTHOR-MANIFEST.json`;
- both design reviews and their PROCESS records.

The records show the following:
- **Two votes.** Two fresh Opus 5.5 reviews (seat A `ff1b88d4…` and seat B `091ffc98…`). Their PROCESS records show `claude-opus-5-5` only. Both recorded C1-C8 AGREE and recommended exactly one RED, and every finding was retained.
- **Precheck.** Root checked the original manifest (5602 B, `2f91b6fc…`): strict JSON, unique paths, all 18 test PINS matching, `Ledger-Core-Script.hs` absent, and the test was not imported. I independently compared the manifest body with test PINS (test:30-49). All 18 sizes and sha256 strings are equal, there is no `Ledger-Core-Script.hs` entry, and the one extra entry, `USER-OPUS55-ROUTING.md`, is unused by the test. This was a textual comparison; I did not hash anything.
- **Run conditions.** The argv equals the proposed envelope:
  - timeout 10 s, kill-after 2 s; prlimit cpu 10, as 512 MiB, fsize 2 MiB, core 0;
  - bwrap `--unshare-all`, read-only `/usr`, `/input` and `/packet`, `--clearenv`;
  - `python3 -I -B`;
  - one attempt;
  - the input directory held only the test (0444 in 0555), and the packet held 20 files at 0444.
- **Transcript.** The stderr shows:
  - `Ran 19 tests` and `FAILED (failures=14)`, with no `errors=`;
  - the 5 T00 methods `ok`;
  - 14 FAIL blocks, each ending `AssertionError: M-HASH-RED: independent_script_hash.py is absent; no implementation has been written` (13 through `_impl` → `_impl_source`; `test_static_purity` directly through `_impl_source`, test:311).
- **RESULT.** Exit 1, stdout 0 B, stderr 11275 B (equal to the packet copy's manifest size), unchanged before/after pins and modes, and the implementation still absent.

**Qualification.** These are root-observed records and a transcript. I did not run, hash or re-verify any pin; sha equality is root's claim. RED shows only that the implementation was absent while T00 provenance held. It is **not** functional hash validation. No prerequisite violation is visible, so I take "actual RED acceptable" as the **author premise** for writing source. This is not an independent approval.

## 2. Source and API (exactly the frozen API)

**Imports.** `hashlib` and `json` only. `re`, `typing` and `__future__` are allowed but not needed: hex is checked against a `frozenset`, and `dict[str, str]` needs Python ≥ 3.9. The RED traceback carets indicate 3.11+, but that is an inference.

**Constants.**

| Name | Value | Role |
| --- | --- | --- |
| `MAX_COMPILED_CODE_BYTES` | 32768 | Local tool bound |
| `MAX_BLUEPRINT_BYTES` | 1048576 | Local tool bound |
| `MAX_VALIDATORS` | 64 | Local tool bound |
| `ADMITTED_PLUTUS_VERSIONS` | `("v2", "v3")` | Immutable tuple; the test compares it as a set |
| `_DIGEST_SIZE` | 28 | Private |
| `_HEX_DIGITS` | — | Private frozenset |
| `_UTF8_BOM` | — | Private bytes |

All module-level objects are immutable.

**Exceptions.** `ScriptHashInputError(ValueError)` and `ScriptHashMismatchError(ValueError)`. They are siblings, and neither subclasses the other.

**`script_hash(plutus_version: str, compiled_code_hex: str) -> str`** runs in this order:
1. `_language_tag`: `type is str`, then exact `"v2"` → `b"\x02"` (Lang:545) or exact `"v3"` → `b"\x03"` (Lang:565). Anything else is InputError. The CBOR `encodeEnum` index is never used (Lang:273-278, 329-330).
2. `_admit_compiled_code` (C4):
   - `type is str`;
   - **length-first**: `len > 65536` is rejected before any other work, and this is the single 32768 B bound, because the text is even-length hex;
   - an odd length is rejected;
   - the text must be all lowercase ASCII `[0-9a-f]`;
   - `bytes.fromhex`;
   - the header must be `0x58 L` with L ≥ 24, or `0x59 L16` with L ≥ 256;
   - the total must equal `header + L` exactly.

   Everything else is InputError: empty input, lone or truncated headers, short form, other majors, indefinite length, `5a`/`5b`, non-minimal headers, and trailing or missing bytes. The payload is never inspected.
3. `_digest`: `hashlib.blake2b(tag + code, digest_size=28).hexdigest()`. It is unkeyed, with no salt or person, over the bytes as given: no strip, no extra wrap, no `[enum, bytes]` envelope (C3; Lang:205, 227-229). The result is 56 lowercase hex characters.

**`check_blueprint(raw: bytes) -> dict[str, str]`** runs in this order:
1. `_parse_blueprint`:
   - `type is bytes`;
   - `len ≤ 1048576`, checked before decoding;
   - UTF-8 BOM rejected;
   - explicit `raw.decode("utf-8", "strict")`;
   - `json.loads(text)` with a duplicate-key `object_pairs_hook` (any depth) and a `parse_constant` that rejects `NaN`/`Infinity`/`-Infinity`;
   - `JSONDecodeError`/`ValueError`/`RecursionError` → InputError.
2. Schema (C5):
   - top-level `dict`;
   - `preamble` required `dict`;
   - `preamble.plutusVersion` required, then `_language_tag`;
   - `validators` required `list` with 1..64 entries.
3. **Phase 1** validates every entry before any comparison:
   - the entry is a `dict`;
   - `title`: required, `str`, non-empty, unique by exact equality;
   - `compiledCode`: required, through `_admit_compiled_code`;
   - `hash`: required, `str`, exactly 56 lowercase hex characters;
   - `parameters`: absent or exactly `[]` (CIP:92). Non-empty, `null` and non-list values are all InputError.
4. **Phase 2** recomputes **every** entry with the preamble tag. There is no group shortcut and no use of the declared hash. Every result is built into a fresh dict. If any entry differs, the call raises `ScriptHashMismatchError` naming the first index, its declared and recomputed values, and the count. Otherwise it returns the fresh `{title: recomputed}` in document order.

Unknown fields are ignored, which adds no invented requirements (ADDENDUM D8). The implementation contains no fixture literal, receipt, CIP vector or Aiken/receipt oracle, and no hard-coded outcome.

## 3. Effects review (line by line, whole file)

- **Top level.** The module docstring, two stdlib imports, seven constant assignments, two class definitions and ten function definitions. Nothing executes at import beyond creating these objects. That includes `frozenset(...)` and a bytes literal.
- **Imports.**
  - `hashlib` loads CPython's `_hashlib` (OpenSSL) and `_blake2` extension modules from read-only `/usr`. OpenSSL may try to read its own configuration.
  - `json` loads `_json`.
  - The test already imports both modules (test:17, 19), so the implementation adds no module, native library or file access beyond what the RED envelope already incurred.
  - There is no project SDK, prover, VM, wallet, key, cache or native project code.
- **No effects.** There is:
  - no `open`, file, `os`, `sys`, network, subprocess, environment, clock, random, `eval`/`exec`/`compile`/`__import__`, `getattr` or dunder access;
  - no writes to module or global state, and no caching between calls.

  The `re` module and its internal pattern cache are not used.
- **Attribute accesses (exhaustive).** `hashlib.blake2b`, `.hexdigest`, `bytes.fromhex`, `frozenset.issuperset`, `bytes.startswith`, `bytes.decode`, `json.loads`, `set.add`, `list.append`.
- **Builtins referenced.** `type`, `str`, `bytes`, `len`, `frozenset`, `set`, `enumerate`, `list`, `dict`, `ValueError`, `UnicodeDecodeError`, `RecursionError`. None is in the test's `BANNED_NAMES`. I chose no local name that collides with `input`, `vars`, `compile` and so on.
- **Inputs.** Inputs are never written; `str`/`bytes` are immutable, and parsed JSON is a local object.
- **Results.** Results are new `str` objects and a new `dict`. The parsed document is not referenced after return.
- **Exception chaining.** Library exceptions are mapped after their handler exits, so raised errors do not chain to `UnicodeDecodeError.object` or `JSONDecodeError.doc`.
- **Residual: traceback frames.** As in any Python code, a caller holding a raised exception keeps its traceback frames, and those frames reference locals (including the input) until the exception is dropped.
- **Cost bounds (estimate).**
  - `script_hash`: O(n) on ≤ 65536 characters, so ≤ 32 KiB is hashed.
  - `check_blueprint`: ≤ 1 MiB parsed; ≤ 64 × 32 KiB = 2 MiB decoded and hashed; peak memory is a few MiB beyond json's parse tree.
  - Deep nesting ends in a `RecursionError`, which is mapped to InputError.
  - Comparisons are not constant-time, which is irrelevant for public data.
- **Guard status.** The test's AST guard is lint-only and cannot prove the above (ADDENDUM D1). This section is the author's effects statement. The mandatory line-by-line review is by a fresh non-author.

## 4. Unchanged test: match and GREEN prediction (not executed)

I read the full unchanged test (566 lines). Every implementation method is predicted to pass. This rests on one unverifiable premise: CPython `blake2b(digest_size=28)` over `tag || bytes` must equal the frozen Aiken/CIP literals. If T01.3 fails, the stop rule applies: do not refit tag, boundary or width, and request the first `missingPrimarySymbols` item.

| Test | Prediction and basis |
| --- | --- |
| T00.1-T00.5 | Pass as in RED. They do not touch the implementation |
| T01.1 api_surface | Pass: callables, sibling `ValueError` subclasses, constants, and `set(("v2","v3"))` |
| T01.2 static_purity | Pass: imports `hashlib`/`json` only and uses no banned `Name`. Lint only |
| T01.3 cip57 | Pass **iff** the CPython/Aiken primitives agree: `58ad`+173 B, L=173, 175 = 2+173; tag 0x02; deterministic |
| T01.4 b9 16 handlers | Same premise. All seven codes are `59 L16` with L+3 = appliedBytes ≤ 13018 |
| T01.5 tag sensitivity | v3 on CIP and v2 on B9 differ, barring a BLAKE2b collision. All 13 bad versions fail the type or exact-equality check |
| T01.6 boundary | A double wrap is accepted: `58af` for CIP, `59 0ad2` for c2. Stripped payloads, `82 01/02` envelopes, `78`/`79`, `5f`, `5a` and `5900ad` are all refused |
| T01.7 length | `58ad…` minus 1, `58ad`, `58`, `5900`, +`00`, a concatenation and `58ae`/`58ac` are all refused by the exact-length or header checks |
| T01.8 hex/type | Upper and mixed case, spaces and `\n` (odd length or non-hex), `0x`, fullwidth, `zz` and empty are refused, as are `bytes`, `bytearray`, `None`, `int` and `list` (type) |
| T01.9 bounds | Exactly 32768 B (65536 characters) is accepted. 32769 B, `"00"*500000` (length-first), short form, `5817`, `5900ff` and `40` are refused |
| T01.10 byte flip | Payload indices n-1, n/2 and 4 keep the framing, so the hash differs |
| T02.1 b9 blueprint | Same premise as T01.4. 16 titles map to 7 values (see the B9 residuals below) |
| T02.2 CIP and bounds | The CIP slice gives `{"hello_world": …}` (its `$schema`, `datum` and other fields are ignored). `parameters: []`, exactly 1 MiB (trailing whitespace) and 64 validators are accepted |
| T02.3 substitution | Five B9 mutations plus CIP-as-v3 are all well formed and raise only MismatchError. The consistent c2/vi swap returns the vi receipt for the c2 titles |
| T02.4 malformed | All 32 cases raise InputError, not Mismatch. Both duplicate-key cases are caught by the hook; neither first-wins nor last-wins applies |

**Opaque B9 residuals.** I did not read the B9 body. The basis is the metadata in FIXTURE-STRUCTURE.json and SIZES.json, plus the RED T00.4 pass. That pass establishes: strict UTF-8 JSON with no duplicate keys or BOM, v3, 16 unique titles, identical code/hash/`parameters` absent or `[]` per group, exact `59 L16` framing, and hashes equal to the receipts. T00 does **not** establish:
- lowercase hex in every B9 `compiledCode` (`bytes.fromhex` accepts uppercase; FIXTURE shows lowercase prefixes and Aiken emits lowercase);
- absence of `NaN`/`Infinity` tokens anywhere in B9.

If either assumption failed, T02.1 and T02.3 would fail. That would be an honest refusal, not a refit.

## 5. Meaningfulness controls against this source (predictions; root-run only after review)

| Control | Mutation of this source | Predicted to kill |
| --- | --- | --- |
| MC1 | `_language_tag` returns 0x01/0x02 | T01.3, T01.4, T02.1, T02.2 |
| MC2 | `_digest` drops the tag | T01.3, T01.4, T02.1, T02.2 |
| MC3 | `_digest` hashes `code[header:]` | T01.3, T01.4, T02.1 |
| MC4 | `_digest` adds a CBOR wrap or envelope | T01.3, T01.4, T02.1 |
| MC5 | Digest size 32, 32 truncated, or sha3/sha224 | T01.3, T01.4, T02.1 |
| MC6 | Drop `len(code) != header + length` | T01.7 |
| MC7 | Drop all framing | T01.6, T01.7, T01.9 |
| MC8 | Drop `_is_lower_hex` | T01.8 (`bytes.fromhex` accepts upper case) |
| MC9 | Change the single bound to `>=` or `+2` | T01.9 |
| MC10 | Return the declared hash | T02.3 |
| MC11 | Drop `object_pairs_hook` | T02.4 |
| MC12 | Ignore the preamble, default v3 | T02.2, T02.3 |
| MC13 | Swap the exception classes | T02.3, T02.4 |
| MC14 | Add `import subprocess` | All implementation tests, **lint-only** (D1) |
| MC15 | Admit v1/v4 | T01.5 |

For MC10, a first-handler-per-group variant needs inserted grouping code, and it is killed only if the mutated handlers are not first in B9 (D6.1). I removed redundant checks (a second byte-length bound and an explicit empty check) so that MC9 and similar mutants act on live code. The D7 mutants survive the unchanged test.

## 6. Disposition of review findings 1-5 (details in ADDENDUM)

1. **C7, review B F1, option (b) chosen.**
   - The AST guard and MC14 are lint-only, not a sandbox, and cannot prove the absence of I/O.
   - A mandatory line-by-line effects review of the whole implementation is required.
   - Any future run uses an exact read-only input directory of two files with no `__pycache__`, with before/after pins and modes. The envelope is the containment.
   - The test is not revised or hardened, and RED is not rerun (D1).
2. **Error precedence: validate all, then compare.** This is implemented and is a **new proposed clarification** that needs two votes, not an accepted rule (D2). The explicit strict UTF-8 decode (no bytes auto-detection; BOM, UTF-16 and UTF-32 rejected; no repair) is also implemented and proposed (D3). Exact types are covered in D4. The surrogate and title policy is the exact contract only, and no limits were added (D8).
3. **Gaps in the immutable test.** The test does not cover the fresh result, BOM/UTF-16/32, mixed errors, `parameters: null`, `NaN`, exact types or nesting (D7). These must be inspected in source review. Future tests are proposed only; none are written or run.
4. **C3 labels.**
   - Byte identity is a ledger [S] (Lang:205, 261-262, 546-547, 566-567).
   - The CBOR/Flat decode through PV2/PV3 → Common, and the ledger dependency on plutus HEAD, are [S-at-HEAD] plus [I].
   - The citations are corrected to Lang:118/158/163.
   - All C4 framing refusals are local tool refusals, never protocol or hash changes.
   - These are proposed label clarifications, and the old documents are preserved (D5).
5. **MC10 order and the oracle.**
   - MC10 depends on order (D6.1).
   - The positive oracle is Aiken-produced, and the receipts are same-run comparison values.
   - The CPython/Aiken primitive is shared (D6.2).
   - The gaps between plutus/cardano-base HEAD and the dependency pins, Aiken, the installed binary and the ledger remain, and there is no authenticated ledger oracle (D6.3).

## 7. Non-claims and open scope

This checker only recomputes identifiers from given bytes. It makes **no** claim about any of the following:
- UPLC/Flat validity or behaviour, or parameter-application behaviour;
- semantic equivalence of scripts;
- proofs, signatures, VK or the matrix;
- VM or CEK execution, or ExUnits;
- fees, transaction size or script-size fitness;
- deployability, registration, on-chain presence or ledger acceptance;
- that any installed binary, lockfile or ledger build uses the supplemental commits.

The bounds and the v2/v3 scope are tool choices. The four-piece chain is not integrated: no `hash_chain4.py`, receipts or C2r2 suite is used or called.

M-HASH remains **OPEN**. So do:
- all 20 UTxO families;
- general financial, ACTUS and DeFi obligations;
- PCD/recursion;
- private handoff and multi-parent composition;
- X1/PX04/M-EU, FX1, FV-01, FV-02;
- real Preprod.

Native authority is **0**. Builder reliance is not permitted.

## 8. Read coverage and limitations

**Read in full (raw bodies, exact packet paths):**
- `MANIFEST.json`, `SEGMENT-INDEX.json`;
- `MHASH-CONTRACT.md`, `MHASH-API.json`, `test_independent_script_hash.py`;
- `Ledger-Plutus-Language.hs`, `Ledger-Hashes.hs`, `Ledger-Keys-Internal.hs`, `CIP-0057.md`;
- `SUPPLEMENT-Plutus-SerialisedScript.hs`, `SUPPLEMENT-Plutus-Common.hs`, `SUPPLEMENT-Cardano-Base-Blake2b.hs`, `SUPPLEMENT-SOURCES.json`, `SOURCES.json`;
- `FIXTURE-STRUCTURE.json`, `SIZES.json`, all seven `hash-*.txt`;
- `FOUR-PIECE-README.md`, `ACTUAL-ADOPTION.json`, `EXISTING-TOOL-CHAIN-CHECK.py` (as text), `USER-OPUS55-ROUTING.md`;
- `PROPOSED-ORDINARY-RED-ENVELOPE.json`;
- `DESIGN-REVIEW-A/B.json` and their `-PROCESS.json`;
- all five `ROOT-RED-*` files, `ORIGINAL-AUTHOR-MANIFEST.json`.

**Not read:**
- `READ-SEGMENT-01..08.txt`. I used the raw bodies instead; the index says the segments carry the same 26 non-opaque bodies.
- The **B9 body**: opaque runtime fixture, metadata only.
- Anything outside the packet: provenance paths are data only, and no private captures, home, credentials, keys, PK/SRS, cache, broad research root or web.

**Not verified:** I computed no sha256 on any file. Fidelity rests on the root pins.

Tools used: Read and Write only. I did no execution, import, compile, hash, shell, rerun or retrieval.

## 9. Next steps (none granted)

1. A fresh non-author audit of the whole candidate, including the D1 line-by-line effects review.
2. Two fresh agreeing independent Opus 5.5 votes on the candidate and on D1-D8. The author does not vote.
3. Root pins the implementation and prepares the D1 two-file envelope.
4. One root GREEN, compared with §4.
5. MC1-MC15.
6. A separate adoption decision.

A T01.3 failure triggers the stop rule.
