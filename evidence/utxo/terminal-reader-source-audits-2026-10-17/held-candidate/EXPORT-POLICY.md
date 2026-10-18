# Terminal structured_output export — policy for the `--export-terminal-review` flag

Status: **PROPOSAL / SOURCE ONLY.** Nothing here is approved for use. No
implementation, unit run, or real export is authorised. Adoption requires two
fresh, independent, agreeing **actual Opus 5.5** security audits, one ordinary
unit verification, and a root decision — in that order. The author does not
vote.

## 1. The observed bug (source observation, not proof)

In the current reader (`CURRENT-READER.py`, mirrored from
`run_source_task.py`) the collector is:

```
def walk(x,collect=False): ...
...
walk(x,kind=='result')
```

`walk` accepts `collect` but **never uses it**. For every stream line it
recurses into every nested value (`for v in x.values(): walk(v,collect)`) and
into every string that looks like JSON, including fenced ```` ```json ````
blocks (`walk(json.loads(z),collect)`). Any dict whose `reviewToken` matches and
which validates is registered as a terminal-review candidate — regardless of
*where* in the stream it came from. So internal tool-call arguments, quoted
JSON inside assistant text, and the final result text are all eligible to
become review candidates.

This is the plausible root of the B observation: `B-PROCESS.json` records
`completeTerminalReviews: 2` and `AMBIGUOUS-COMPLETE-REVIEWS.json` holds two
whole objects that are identical except at `findings[5].evidence` (B-I6), where
one reads a `0`-style escape and the other a `0`-style escape
(`PUBLIC-DIFFERENCE.json`). Seat A (`SINGLE-PROCESS-A.json`) produced exactly
one object — the working example.

**Limit of this claim.** This is a source-level hypothesis only. I did **not**
inspect any raw capture (`provider.private.jsonl`, stderr, logs). Which private
event generated each of the two B objects — a second genuine `result` event, a
quoted copy in assistant text, or a nested tool argument — is **unproven**. I
neither read the raw stream nor assume its shape.

## 2. The fix

A new opt-in flag `--export-terminal-review`:

- **implies `export_only`** before any dispatch logic, so a provider is never
  launched and there are no provider retries;
- **rejects any non-`claude` provider**;
- reads the already-captured stream **read-only**;
- selects, via the pure function `select_terminal_structured_review`, **exactly
  the typed `structured_output` dict on the single successful `result` event**,
  with **no fallback** to text, partial output, nested tool arguments or
  quoted/embedded JSON. The recursive whole-stream walk is not used on this
  path, so a contradictory-but-valid draft/quoted/nested object can never
  override the final typed object;
- writes **only** two new files, `TERMINAL-EXPORT.json` (an allowlisted
  receipt) and, on EXPORT, `TERMINAL-REVIEW.json` (the whole canonical object),
  via **exclusive create** (`open('x')`). It **never overwrites** and **never
  mutates** `PROCESS.json`, `REVIEW.json`, `AMBIGUOUS-COMPLETE-REVIEWS.json` or
  `WHOLE-SCHEMA-VALID-CANDIDATES.json`. Every old candidate/record is preserved;
- exports **no secrets, no partials, no raw** provider content.

The normal path and the ordinary `--export-only` path are **unchanged**, so no
live job behaviour changes.

## 3. Selection contract (what qualifies, and every STOP)

EXPORT requires **all** of:

1. provider is `claude`;
2. requested model is exactly `claude-opus-5-5`;
3. original process `exitCode == 0` (not `None`, not non-zero);
4. input pins unchanged;
5. the exclusive actual assistant-envelope model is `claude-opus-5-5` (at least
   one assistant frame, and **no** other model such as `claude-opus-4-8`);
6. exactly **one** terminal `result` event;
7. that result is a success (`subtype == 'success'`, not `is_error`);
8. `structured_output` present and a **dict** (a string STOPs);
9. `reviewToken` matches exactly;
10. the object validates against the frozen `OUTPUT-SCHEMA`;
11. the object is byte-identical to one already-retained public whole
    candidate.

Any failure is a STOP with a named reason and **no partial salvage and no text
fallback**. In particular: non-zero/missing exit, wrong requested model, mixed
or wrong actual model, no actual frames, missing/multiple terminal results, a
failed result, absent/string `structured_output`, token or schema mismatch, or
a canonical object not among the retained candidates — each STOPs.

## 4. Authority selection vs unique judgment

This flag performs **authority selection only**: it picks the single
*provider-canonical artifact* — the typed object the provider itself emitted on
its success `result` event — and records that this is what it did. It does
**not**:

- choose the "better" or more convenient of two candidate votes;
- splice, merge, or edit any security-judgment field;
- decide B's security disposition or close B's multiplicity STOP. The receipt
  records `legacyRecursiveWholeObjectCount` (e.g. 2 for B) and asserts the old
  record is preserved; it does not falsify or supersede it.

A valid typed `structured_output` dict proves **provider-artifact identity**
only. It does **not** prove any cryptographic or runtime property, nor the
materiality of the finding it contains. The *unique judgment* — whether the
selected review's content is correct and adoptable — remains a separate
decision that this flag never makes.

The model-identity gate mirrors the `WRONG-MODEL-REJECTION.json` precedent: a
run that requested 5.5 but returned actual 4.8 was rejected and preserved with
its real identity, not relabelled, even though a 5.5 fallback was requested.
Rule 5 enforces exactly that at selection time.

## 5. Why a fresh independent security review is required before use

The flag introduces **new write behaviour** (two new files) and a **new trust
rule** (treat `result.structured_output` as canonical and ignore every other
stream location). Both are new semantics, not a like-for-like refactor. New
policy, code, unit and effects therefore require **two fresh agreeing actual
Opus 5.5 audits plus a root decision** before any unit run or real export, and
then one ordinary unit verification. Only after that may the existing,
already-trusted reader read its *own* B capture in export-only mode; no other
agent opens raw files, and there are no provider retries.

## 6. Unit preparation

`test_terminal_review_selection.py` AST-extracts **only** the pure selector and
exec's that one function node with `json` + `jsonschema`. It never imports or
runs the whole candidate, its top-level job logic, or any private-file logic.
Fixtures are **derived UNIT envelopes** modeled on the public B objects (the
B-I6 `0` vs `0` difference) — not real provider records and not Cardano
ledger data. The future bound run may instead bind the two public code files,
the public B/A whole objects, the real `OUTPUT-SCHEMA` and the public PROCESS
fixtures, and only those; no provider-raw or auth paths.

- Frozen method count: **20**.
- Expected RED: with no selector in the candidate, loading fails and all
  methods error.
- Expected GREEN: all 20 pass against the candidate as written.
- Bounded effects: reads one public code file to AST-extract one function;
  writes nothing; opens no socket; adds no dependency beyond stdlib +
  jsonschema; reads no private capture.
- **Not run now.** The outcomes above are recorded expectations, not observed
  results.

## 7. Named gaps (not read, not assumed)

- The exact provider-envelope contract — whether `structured_output` is emitted
  at most once on the terminal `result`, and whether the second B object arose
  from quoted text / a nested tool argument rather than a second `result` event
  — is **unknown from the public packet**. It lives in the raw
  `provider.private.jsonl`, which must not be read. If confirming the protocol
  priority of "`result.structured_output` over all other locations" requires
  that private envelope format, **that contract is the precise gap**; it is
  named here rather than read or assumed.
- Consequently this proposal does **not** assert which of the two retained B
  objects is the canonical one; the flag computes that at runtime from the
  capture (rule 11), and STOPs if the canonical object is not among the retained
  candidates.
- No native/resource authority is taken (`nativeAuthority: 0`). No project
  tests, SDK, prover, VM, ledger or wallet are touched. M-HASH and the broader
  Moriarty / financial / PCD / Preprod work remain open and untouched.
