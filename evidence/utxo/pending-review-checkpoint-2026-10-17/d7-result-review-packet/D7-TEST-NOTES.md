# M-HASH D7 addendum tests: author notes (PROPOSED SOURCE, NOT RUN)

Author: claude-opus-5-5, high effort. This is independent test-source authoring only. The author is not a vote.

Nothing in this directory was executed, imported, compiled, linted, hashed or reviewed. Files:
- `test_script_hash_addendum.py`: 14 methods, standalone.
- `D7-TEST-CONTRACT.json`: method names, every expected positive and negative outcome, premises, predictions and the effects proposal, all recorded before any run.
- `D7-TEST-NOTES.md`: this file.

There is no sourceReady flag, acceptance or native grant.

## 1. Why this exists

ADDENDUM D7 and both implementation reviews (A-F5, B-I1) say the frozen 19-method test does not discriminate:
- D2 precedence;
- fresh results;
- BOM and UTF-16/32 handling;
- NaN;
- `parameters: null`;
- exact types;
- nesting and int-limit mapping.

They require a separately reviewed test revision before anyone relies on those behaviours. This file is that revision's source.

It is a **separate** module:
- It does not import, read or run the frozen test, which stays byte-unchanged.
- It does not rewrite the baseline.
- It is not a replacement for the baseline.

## 2. Independence and expectation sources

- **Implementation not read.** I did not open `independent_script_hash.py` or any implementation path, even where cited. Implementation line references in the reviews are their assertions, not my observations.
- **Where expectations come from.** Every expected outcome comes from C4/C5, the API and the proposed D2, D3 and D4. D8 stays open.
- **One expected digest.** The only expected digest is the frozen CIP-57 hello_world literal `5e1e8fa8…0a02`. The test computes no BLAKE2b and never calls the implementation to obtain an expected value. sha256 is used only for the two packet pins.
- **Where positive inputs come from.** Positive inputs are the verbatim CIP slice, the same slicing as baseline T00.3, or the parsed CIP entry re-serialised with only `title`/`parameters` changed. Neither change affects the hash input.
- **Negative controls.** Altered documents are security negative controls, not ledger data. No ledger state, UPLC, proof or hash rule was invented.

## 3. Coverage map (D7 row → method)

| D7 row / requirement | Method(s) | Contrast that discriminates |
| --- | --- | --- |
| D2 precedence | T10.1 | 5 malformation kinds × all 6 orders of (ok, mismatch, malformed) + both 2-entry orders → exactly InputError (40) |
| Mismatch independent of position | T10.2 | hash nibble or code byte flipped at positions 0/1/2 of three identical entries, plus both pair orders → exactly Mismatch (8). This is the CIP-derived form of MC10's order dependence |
| Fresh detached result | T11 | the first result is mutated, cleared and popped; later calls are unaffected, and later calls do not write into earlier results |
| UTF-8 BOM refused | T12.1 | `EF BB BF` + slice → InputError, against the accepted slice |
| UTF-16/32 refusal with and without BOM | T12.2 | 8 encodings → InputError. The `json.loads(bytes)` auto-detection mutant would accept them |
| Strict UTF-8 codec | T12.3 | overlong, encoded surrogate, truncated, >U+10FFFF bytes in the unread description → InputError. `errors="replace"/"ignore"` mutants would accept them |
| NaN/±Infinity refused | T13.1 | 6 positions (top extra, preamble extra, datum-schema `index`) against a finite `0.5` in the same slot |
| `1e999` left as is | T13.2 | accepted, per D3.3. No refusal is invented |
| `parameters: null` (and false/0/""/{}) | T14 | against absent and `[]` → accepted. A "falsy means absent" mutant would accept them |
| D4 exact types | T15.1, T15.2 | str/bytes subclasses with *valid* content (an `isinstance` mutant would return the real hash), tripwire subclasses, `__class__` impostors (`isinstance` true, `type()` false), memoryview, bytearray, bool, None, JSON `true` in str fields |
| Deep nesting → InputError | T16.1 | 100000-deep arrays/objects against a 16-deep positive, under premise P1 |
| Int-digit limit → InputError | T16.2 | 50000-digit datum-schema `index` against `12345`, under premise P2 |

Every implementation method contains a positive control from the same CIP base, so a checker that refuses everything cannot pass.

Exceptions are asserted by **exact** class (`type(e) is …`). Every method also re-checks that both classes are `ValueError` subclasses and that neither is a subclass of the other.

Tripwire hooks raise `TripwireError`, which is not a `ValueError`, and log the call:
- A hook that is consulted surfaces as an error.
- A hook that is consulted and then swallowed surfaces through `log == []`.
- `__len__`, `__repr__` and `__str__` are not tripped, so no check ordering beyond D4 is imposed.

## 4. Interpreter premises (reviewers must accept or reject explicitly)

- **P1.** Stdlib `json.loads(str)` on each 100000-deep control raises exactly `RecursionError` and does not crash.
- **P2.** Stdlib `json.loads(str)` on the 50000-digit control raises exactly `ValueError`, not `JSONDecodeError`. That is the CPython ≥3.11 default `int_max_str_digits = 4300`. `-I` ignores `PYTHONINTMAXSTRDIGITS`.

How the premises are handled:
- **Probe.** Both are probed by calling stdlib `json.loads` on the test's own controls, in T00 and again in T16. No `sys` query is made.
- **Values are fixed.** DEEP=100000 and INT_DIGITS=50000 are fixed. There is no autotuning and no skip.
- **If a premise is unmet.** The test fails with `M-HASH-D7-PREMISE … BLOCKED`. Because T00 runs during the missing-implementation RED, a premise failure makes the RED invalid, so: STOP before any reliance.
- **Portable version is BLOCKED.** An interpreter-independent depth or int-size refusal would need a tool limit that no approved clause defines.

## 5. D8 and other deliberate non-assertions

- **D8 is not decided by this test.** There are no title limits, no Unicode normalisation, no lone-surrogate-escape policy (raw invalid UTF-8 bytes are D3.1 and are tested; `\uD800` escapes are not), and no closed schema.
- **Disclosed reliance.** Nine controls use one extra member, `x-d7-unread`. Their positive controls (0.5, 1e999, shallow nesting) assume an extra unread member is accepted, as already sanctioned for `1e999`. If D8 is ever closed, those positives fail visibly; they cannot mask it. The int-digit control and the datum-index NaN/-Infinity controls use the existing CIP datum-schema `index` slot.
- **Escaped `u0030` example.** The D3 escaped-`u0030` example is not tested while the Seat B label ambiguity awaits the security owner.
- **Not asserted:**
  - D2 message text (first index and count);
  - the internal order of recomputation;
  - D3.5 non-chaining (not a D7 row; a candidate for a later revision);
  - D4 exact types of parsed containers (not observable through the public API without patching json).
- **MC10 on B9 is not covered.** B9 is not in this packet.

## 6. Loader

- **Lazy.** The first statement of each implementation method is `self.api()`.
- **Absent implementation.** An absent file raises `AssertionError("M-HASH-D7-RED: independent_script_hash.py is absent; no implementation is present")`. That is a unittest failure, never an `ImportError`. A symlink is refused.
- **Present implementation.** The loader:
  1. reads the file once, at most 1 MiB;
  2. decodes it as strict UTF-8;
  3. runs the baseline allowlist and banned-name **lint**;
  4. `exec`s `compile()` of exactly that linted AST into a fresh `types.ModuleType`.
- **What that rules out.** There is no second read, no SourceFileLoader and no `.pyc` read, which closes A-F5 inside the test as defence in depth. There is no `sys.path` or `sys.modules` change.
- **Not a sandbox.** The lint is not a sandbox (D1). Containment is the envelope plus the mandatory line-by-line checker review.

## 7. Predictions (not results)

- **D7-RED** (only this test in `/input`):
  - Expected: 14 ran, T00 ok, 13 failures each with the exact RED message, 0 errors, exit 1.
  - Any T00 failure or any error means STOP.
- **D7-GREEN** (this unmodified test plus the reviewed checker 9070 B `4f6af890…`, as reported):
  - Expected: 14 ran, 14 pass, exit 0.
  - This is conditional on: the checker matching D2–D4 as the reviews report; P1 and P2 holding; and CPython/Aiken BLAKE2b-224 agreement on the CIP vector (the shared primitive).
  - A GREEN would show conformance to *proposed* D2–D4, not their adoption.
  - A failure means STOP. Do not weaken the test or edit the checker without review.

## 8. Future effects proposal (review before any execution)

Everything here is a proposal. Full details are in `D7-TEST-CONTRACT.json` under `futureEffectsProposal`.

- **Stages.** Two stages, D7-RED then D7-GREEN, each needing its own approval and allowed one attempt.
- **Approvals before any execution:**
  - root full-effects inspection;
  - two fresh, independent security design/effects reviews;
  - a root decision.
- **Input directory.**
  - RED: `/input` holds exactly this test.
  - GREEN: `/input` holds exactly this test plus the reviewed checker.
  - In both, files are 0444 in a 0555 directory, with no `__pycache__`, symlinks or extras.
  - Root pins this test's bytes and sha256 at freeze. The author computed none.
- **Packet.** `/packet` holds exactly `CIP-0057.md` and `SOURCES.json`.
- **Root checks.** Root records before/after pins, modes and listings, and STOPs on any difference.
- **Envelope.** The existing class, unchanged:
  - `timeout --kill-after=2s 10s`;
  - `prlimit` cpu 10 / as 512 MiB / fsize 2 MiB / core 0;
  - `bwrap --unshare-all`;
  - read-only `/usr`, `/input` and `/packet`;
  - `--clearenv` with a minimal environment;
  - `python3 -I -B`.
- **What does not change.**
  - The baseline 19-method GREEN is not repeated.
  - Baseline, checker and fixture PINS are unchanged.
  - Meaningfulness mutants are selected by independent auditors under a separately reviewed envelope. None is automatic.
  - No runner, scheduler, helper or checker is supplied.

## 9. Read coverage (honest)

I read the packet `MANIFEST.json` and all ten listed bodies, each in full in one read:
- `MHASH-CONTRACT.md`
- `MHASH-API.json`
- `MHASH-IMPLEMENTATION-ADDENDUM.md`
- `FROZEN-19-METHOD-TEST-REFERENCE.py`
- `CIP-0057.md`
- `SOURCES.json`
- `DESIGN-REVIEW-A.json`
- `DESIGN-REVIEW-B.json`
- `IMPLEMENTATION-REVIEW-A.json`
- `IMPLEMENTATION-REVIEW-B-WHOLE-OBJECTS.json` (both whole objects)

Limits on that reading:
- **Not verified against the manifest.** I did not compute sizes or sha256 for these files; the manifest figures are taken as given.
- **Not read:** every `originalPublicSource` path, the checker, B9, the ledger and supplement sources, the envelopes, and anything outside the packet.
- **Line endings.** The LF line-ending premise for slicing the CIP example comes from the baseline T00.3 pass that the reviews report. I did not observe it.

## 10. Status

- M-HASH remains OPEN.
- The historical reviews are not relabelled.
- The Seat B whole-object ambiguity stays pending.
- No dual approval exists.
- All 20 UTxO families, general financial/ACTUS/DeFi, PCD/recursion, private handoff, multi-parent composition and real Preprod remain open.
- Native authority is 0.
