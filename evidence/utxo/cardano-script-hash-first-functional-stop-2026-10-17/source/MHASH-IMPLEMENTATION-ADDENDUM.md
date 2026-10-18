# M-HASH implementation addendum: proposed contract clarifications (PROPOSED, NOT ACCEPTED)

Author: claude-opus-5-5, high effort. This is source authoring only. The author is not a vote, and nothing here was executed, imported, compiled or hashed.

This addendum is separate from the contract. It does not edit, supersede or relabel any frozen document. The following stay immutable and in force as written:
- `MHASH-CONTRACT.md` (14229 B, `e43c21f5…`);
- `MHASH-API.json` (23785 B, `295a64c5…`);
- `test_independent_script_hash.py` (28091 B, `a1c00918…`);
- the superseded initial proposal (7359 B `fedf74ff…` / 14378 B `c8d5bc86…`);
- both design reviews, the root RED decision and the root RED result.

Each item is labelled:
- **[NEW-SEMANTIC]**: a choice the contract leaves open. The candidate implements it.
- **[LABEL]**: documentation or citation only, with no behaviour change.
- **[PROCESS]**: review or envelope requirements before any run.
- **[OPEN]**: deliberately not implemented and not decided.

None of the items is accepted. Every item needs **two fresh, agreeing, independent, non-author Opus 5.5 votes** before any GREEN that relies on it. A separate fresh non-author audit of the whole candidate (`independent_script_hash.py` with this addendum and `IMPLEMENTATION-NOTES.md`) is also required.

The unchanged 19-method test does **not** discriminate any [NEW-SEMANTIC] item (see D7). GREEN therefore cannot validate these items; only source review can.

## D1. C7 guard and containment [PROCESS]: review B F1, option (b); review A F5/F6

Option (b) of review B F1 is chosen. Option (a), which would harden the guard, revise the test and rerun RED, is **not** taken. The test is neither revised nor hardened, and RED is not rerun.

Proposed clarification of C7 and API `test.loader` / `meaningfulnessControls.MC14`:
1. The AST import allowlist and banned-name check in the test (test:88-90, 155-165) is a **lint**, not a sandbox. It does not and cannot prove the absence of I/O or dynamic execution. Attribute chains on allowed modules, `getattr`, `__builtins__` and dunder traversal all bypass it.
2. **MC14 is lint-only.** "Guard refuses `import subprocess`/`open()`" shows that the lint rejects the obvious forms. It is not evidence of purity.
3. Before root executes any implementation (GREEN or mutants), a fresh non-author reviewer must do a **mandatory line-by-line effects review of the whole implementation file**. The candidate is built to make this tractable:
   - Imports are only `hashlib` and `json`. Both are already imported by the test itself.
   - The only attribute accesses are `hashlib.blake2b`, `.hexdigest()`, `bytes.fromhex`, `frozenset.issuperset`, `bytes.startswith`, `bytes.decode`, `json.loads`, `set.add` and `list.append`.
   - It has no dunder access, no `getattr`, no string-to-code path and no module-state writes.
4. **The envelope is the containment.** Any future GREEN or mutant run should use the reviewed RED argv class:
   - `timeout`/`prlimit`/`bwrap --unshare-all`, `--clearenv`, `python3 -I -B`, a read-only packet, and one attempt per authorised run.
   - The read-only input directory must hold **exactly two regular files**: the unchanged test (28091 B, `a1c00918…`) and the reviewed implementation at its reviewed bytes/sha256. Both are 0444, in a 0555 directory, with **no `__pycache__`**, symlinks or other entries.
   - Root checks pins and modes of the input and the packet (20 files) **before and after** the run, and confirms that no new entries exist afterwards.
   - This closes review A F5: `exec_module` can load a matching `.pyc`, and the guarded text could differ from the executed code (TOCTOU). With no cache directory and a read-only bind, there is nothing to substitute.
   - Any unexpected outcome means STOP, with no retry.

## D2. Error precedence [NEW-SEMANTIC]: review B F3

C5 and the API (lines 28-29, 33) do not say which exception wins when one entry is malformed and another is well formed but has a wrong hash. Proposed clarification of C5:

> `check_blueprint` first validates the whole document and **every** validator entry: the type and bound of raw, strict UTF-8, strict JSON with no duplicate keys, the preamble and `plutusVersion`, the shape and count of `validators`, every title (non-empty, unique), every `compiledCode` (C4 admission, including framing), every declared `hash` format, and every `parameters` value. Only after all of these pass are the hashes recomputed and compared. Any malformed input raises `ScriptHashInputError`, whatever its position relative to a mismatching entry. `ScriptHashMismatchError` is raised only when every entry is well formed. Every entry is recomputed before the mismatch is raised. The message reports the first mismatching index and the mismatch count.

Implemented in `check_blueprint` as Phase 1 (validate all) and Phase 2 (recompute all, then compare). This is a **new** choice and is not accepted. A "first error in document order wins" mutant passes all 19 unchanged tests (D7).

## D3. Strict UTF-8 and strict JSON details [NEW-SEMANTIC]: review B F4

C5 says "strict UTF-8 JSON in which duplicate object keys are an error". The proposal makes the following explicit, with no implicit repair:
1. **Explicit decoding.** `raw.decode("utf-8", "strict")` produces the text, and `json.loads` receives only that `str`. It never receives bytes, so json's UTF-8/16/32 auto-detection never runs.
   - Invalid UTF-8 is rejected, including overlong forms and encoded surrogates (CPython's strict codec).
   - UTF-16 and UTF-32 documents are rejected. With a BOM, they are not valid UTF-8. Without a BOM, the NUL bytes decode but are not admissible JSON.
2. **A UTF-8 BOM (`EF BB BF`) is rejected** rather than stripped. RFC 8259 lets parsers ignore a BOM; the tool chooses strict refusal.
3. **`NaN`, `Infinity` and `-Infinity` are rejected**: they are not JSON values. This uses `parse_constant`. Non-finite results from finite literals (for example `1e999`) are a json-library behaviour inside fields the tool never reads, and they are left as is.
4. These library-level parse failures map to `ScriptHashInputError`:
   - `JSONDecodeError`;
   - CPython's integer-digit limit (`ValueError`);
   - nesting depth (`RecursionError`).

   No other exception class is mapped.
5. Exceptions are not chained to library exceptions that hold the input (`UnicodeDecodeError.object`, `JSONDecodeError.doc`). The library exception is mapped after its handler exits.

Unchanged by this item:
- duplicate keys are rejected at every nesting level;
- unknown fields are not validated (D8);
- JSON string escapes decode under standard JSON rules (for example `"0"` is `"0"`).

## D4. Exact builtin input types [NEW-SEMANTIC]: per the authoring instruction and contract types

- `script_hash` requires `type(plutus_version) is str` and `type(compiled_code_hex) is str`.
- `check_blueprint` requires `type(raw) is bytes`.
- Parsed JSON containers and fields are checked with `type(x) is dict/list/str`.

As a result, `bytearray`, `memoryview` and `str`/`bytes` subclasses are refused with `ScriptHashInputError`, and `bool` is never accepted where `str` is required. The test covers only `bytes`, `bytearray`, `None`, `int` and `list` for the hex, and `str` for raw.

## D5. C3 labels and citations; C4 refusals are local [LABEL]: review A F3/F4/F8, review B F2

These are proposed label-only corrections, to be recorded in a future contract revision. The current text stays preserved.
1. **CONTRACT:43** (`Therefore [S] no layer is removed or added …`) should read:
   - [S, ledger `d6eec237`] The bytes hashed (`originalBytes`, Lang:205, 261-262) are the same `bs` passed to `PV2/PV3.deserialiseScript` (Lang:546-547, 566-567). The identity "hashed bytes = deserialised bytes" is a ledger source fact.
   - [S at plutus HEAD `3de6f245`] `Common.deserialiseScript` decodes `fromShort sScript` from offset 0 and returns `sScript` unmodified (SS:263, 265-269), and `serialiseUPLC = … serialise . SerialiseViaFlat …` (SS:155-160).
   - [I] `PV2/PV3.deserialiseScript` = `Common.deserialiseScript` (`PlutusLedgerApi.V2/V3` uninspected).
   - [I] Ledger `d6eec237` builds against a plutus-ledger-api matching `3de6f245` (dependency pins uninspected).
   - So "decoded as one CBOR-encoded Flat script" is [S-at-HEAD] plus [I], **not** a ledger-pinned [S]. Add both [I] items to C3's own [I] list.
2. **Citation fix.** CONTRACT:41 and API:57 cite `Lang:117-118`. The correct citations are:
   - Lang:118: `ScriptDecodeError (..)` import (Lang:117 imports `PlutusLedgerLanguage`);
   - Lang:158: `P.CBORDeserialiseError`;
   - Lang:163: `P.RemainderError`.
3. **Every C4 framing refusal is local to the tool.** This covers:
   - short-form headers;
   - non-minimal `58`/`59` headers;
   - indefinite length;
   - `5a`/`5b`;
   - other major types;
   - missing or trailing bytes (v2 and v3);
   - the 32768 B bound;
   - the 1 MiB / 64-validator bounds.

   These are admission refusals. They are never a different hash, and they are neither protocol claims nor changes to the hash rule. A lenient CBOR decoder or the ledger may accept some of these inputs; the ledger accepts V1/V2 trailing bytes (SS:259-261).

## D6. Oracle, MC10 and source-gap qualifications [LABEL]: review B F6, review A F2

1. **MC10 depends on order.** "Check only the first handler per group" is caught by T02.3 only if the mutated handlers (`c2.c2.spend`, `vs4.vs4.else`) are not first in their groups in B9:
   - Review B's partial B9 read confirms that `c2.c2.mint` precedes `c2.c2.spend`.
   - FIXTURE-STRUCTURE lists `vs4.vs4.withdraw` before `vs4.vs4.else`, but it is a root projection. The vs4 order in the opaque B9 body is unverified by this author.

   The candidate has no per-group logic, so this mutant must be inserted deliberately.
2. **Positive oracle.** Every positive vector is Aiken-produced: the CIP hello_world vector and the seven same-run Aiken v1.1.24 receipts.
   - The receipts are comparison values, not independent oracles.
   - The checker uses CPython `hashlib.blake2b(digest_size=28)`. Agreement shows CPython/Aiken BLAKE2b-224 agreement on these bytes under C2/C3 framing. A defect shared by both primitives is undetected.
   - There is no ledger- or chain-produced vector.
3. **Source gaps that remain.** The supplements are repository-HEAD bodies (plutus `3de6f245`, cardano-base `5813d211`). Their correspondence to the dependencies of ledger `d6eec237`, to Aiken v1.1.24, or to any installed binary or lockfile is unproven. The following were not inspected:
   - `Cardano.Crypto.Hash.Class` and the re-export;
   - `Libsodium.C` and libsodium;
   - `SerialiseViaFlat`/cborg;
   - `PlutusLedgerApi.V2/V3` and `Common.Hash`;
   - Aiken source and its lock;
   - the ledger dependency pins.

   No authenticated ledger oracle exists, and libsodium/CPython equivalence is not formally shown.

## D7. Coverage gaps of the immutable test [PROCESS]: review A F8, review B F3/F4

These behaviours are **not covered** by the unchanged 19 tests. The source review must inspect them. Mutants of them survive GREEN.

| Behaviour (candidate) | Surviving mutant under the unchanged test |
| --- | --- |
| D2 precedence | Raise the first error in document order |
| Fresh, detached result dict | Return a cached or shared dict |
| UTF-8 BOM rejection (D3.2) | Strip the BOM, or rely on json's BOM error |
| UTF-16/32 rejection, explicit decode (D3.1) | `json.loads(raw)` with bytes auto-detection |
| `NaN`/`Infinity` rejection (D3.3) | Default `json.loads` constants |
| `parameters: null` rejected | Treat `null` as absent |
| `bytearray`/`memoryview`/subclass refusal (D4) | `isinstance` checks |
| Deep nesting / huge int mapped to InputError | Leak `RecursionError`/`ValueError` |

Proposed **future** test additions, for a separately reviewed test revision only. They are **not written and not run now**:
- a mixed-order malformed-plus-mismatch pair;
- a UTF-8-BOM-prefixed blueprint;
- a UTF-16-LE-BOM blueprint;
- a UTF-16 blueprint without a BOM;
- a `NaN` field;
- `parameters: null`;
- `bytearray` raw input;
- a call that mutates the first result and checks that a second call is unchanged;
- a perturbation of a first handler and a non-first handler separately (MC10).

## D8. Open policy, deliberately not decided [OPEN]

The candidate follows the exact contract and adds **no** limits for the following:
- Titles are compared by exact code-point equality, with no Unicode normalisation. Lone surrogates from JSON `\uD800`-style escapes are accepted. There is no title length limit beyond the 1 MiB document bound.
- Unknown or additional top-level, preamble and validator fields (`$schema`, `definitions`, `datum`, `redeemer`, `description`, `compiler`, `title` of the preamble, and so on) are neither required nor validated. CIP-57 schema validation is out of scope.

Any of these would be a new semantic requirement needing two votes.

## Stop check

No contract gap found while authoring requires an unapproved semantic change beyond D2-D4:
- The C1-C4 hash rule, framing and bounds are implemented exactly.
- The C5 schema is implemented exactly as listed.

If reviewers reject D2, D3 or D4, the affected source is confined to `check_blueprint` Phase 1/2, `_parse_blueprint` and the `type(...) is` checks. The outcomes of the 19 unchanged tests would not change.

Status of open work:
- M-HASH remains OPEN.
- All 20 UTxO families, financial/ACTUS/DeFi obligations, PCD/recursion, private handoff, multi-parent composition and real Preprod remain open.
- There is no four-piece chain integration.
- Native authority is 0.
