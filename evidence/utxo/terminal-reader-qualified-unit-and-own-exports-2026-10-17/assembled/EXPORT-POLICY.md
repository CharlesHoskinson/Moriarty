# Terminal structured_output export — corrected policy for `--export-terminal-review`

Status: **PROPOSAL / SOURCE ONLY. NOTHING HERE IS APPROVED FOR USE.** No
implementation, no unit run, no AST/compile/hash/lint run, and no real export is
authorised now. This is an ordinary defensive correction of an already-public,
opt-in reader candidate and its unit, produced after two fresh independent
source audits (FRESH-SOURCE-AUDIT-A and -B). Adoption still requires, in order:
two fresh, independent, agreeing non-author Opus security audits of this
corrected source/unit/effects, then one ordinary unit verification, then a
separate root adoption decision. The author does not vote. Same-provider fresh
contexts give context independence only, not provider diversity. Different Opus
models do not establish provider diversity.

This delivery carries **no reviewer vote, no ordinary-unit permission, no
export/adoption, and no native authority** (`nativeAuthority: 0`). The existing
trusted reader is unchanged; only proposed source is delivered here. Any claim
about which of B's two whole objects is the "actual old canonical B identity"
stays **UNKNOWN** until a later, separately adopted own-reader export. All
original ambiguous / whole / process / review records remain immutable.

## 0. Honest runtime identity

This correction was authored by **actual Opus 4.8 (`claude-opus-4-8`)**, which
is the model this session reports. `USER-BOTH-OPUS-RULE.md` authorises both
`claude-opus-4-8` and `claude-opus-5-5` at high effort for this security work;
the request naming 5.5 does not make a 4.8 response a routing failure, and a 4.8
response is never relabelled as 5.5. No provider was dispatched; a prior
separate source attempt that reported a classifier stop is neither overridden
nor reproduced here.

## 1. What was corrected, and the four delivered files

1. `run_source_task_candidate.py` — the corrected public candidate: the existing
   legacy reader PLUS (a) the pure selector `select_terminal_structured_review`
   and (b) the additive `--export-terminal-review` branch. The no-flag normal
   path and the ordinary `--export-only` path are **byte-for-byte unchanged**
   from `CURRENT-READER.py` (lines 7-152 of the current reader reappear verbatim
   as the tail of the candidate). The current/trusted reader itself is **not**
   modified.
2. `test_terminal_review_selection.py` — the corrected ordinary unit. It
   AST-extracts **only** the single pure selector FunctionDef and exec's that
   node alone with `json` + `jsonschema`. It never imports/runs the module,
   dispatch, or any private-file logic.
3. `READER-PATCH.diff` — proposal-only unified diff relative to
   `CURRENT-READER.py` (two additive hunks; remainder unchanged). **Not applied.**
4. `EXPORT-POLICY.md` — this document.

## 2. Reconciliation of the two audits (disposition of EVERY finding)

Both audits AGREED on ER01, ER02, ER05, ER06, ER07, ER09, ER10 (A also raised
ER03 as owner-questions; B marked ER03 NEEDS_FIX). Both flagged the same two
substantive defects (empty-assistant unit defect; the 5.5-only model gate vs the
both-Opus rule). Dispositions:

- **A F-ER08-UNIT-EMPTY-ASSISTANT / B F1-UNIT-INDEXERROR (high) — FIXED.**
  `_stream` no longer indexes `assistant_models[0]` unconditionally; the
  DRAFT-carrying nested/quoted frames are emitted **only** when the assistant set
  is non-empty. `test_reject_no_actual_assistant_frames` now reaches the selector
  and asserts `NO_ACTUAL_ASSISTANT_MODEL_FRAMES`.
- **B F2-EMPTY-ASSISTANT-UNREACHABLE (medium) — FIXED.** With the carrier frames
  gone for the empty case, the stream contains no model-bearing assistant frame,
  so the branch is genuinely exercised (the `system` init frame is `type:system`
  and the result `result` text is not an assistant frame, so neither is counted).
- **A F-ER04-MODEL-GATE-RECONCILE / B F3-5P5-ONLY-GATE-VS-BOTH-MODELS (medium) —
  FIXED by parameterisation + honest identity.** The selector takes an explicit
  `allowed_models` set. Its default is the single historical actual-5.5 export
  scope (`claude-opus-5-5`), preserving the `WRONG-MODEL-REJECTION` precedent for
  the exact current B/A subjects. The wrapper passes
  `T.get('terminalReviewAllowedModels', ['claude-opus-5-5'])`, so a future
  4.8-authored subject can be enabled by task config (honouring
  `USER-BOTH-OPUS-RULE.md`) **only** with a fresh model-compatibility review. The
  EXPORT receipt now records the REAL exclusive actual assistant identity
  (`exclusiveActualAssistantModel`), never a relabel. Requested model must be in
  the allowed set; the actual assistant-envelope model must be exactly one model
  AND a member of the allowed set; any mix or out-of-set identity STOPs.
- **A F-ER05-BYTE-IDENTICAL-WORDING / B F6-BYTE-IDENTICAL-WORDING (info) —
  FIXED (wording).** The membership cross-check is **canonical-sorted-key-JSON
  value identity** (`json.dumps(..., sort_keys=True)`), NOT raw-byte identity.
  The receipt now carries `identityMatchRule:
  "canonical-sorted-key-JSON-value-identity"`, the docstring says so, and this
  policy says so. The two public B whole objects parse to distinct values at
  `findings[5].evidence` and are therefore never conflated (see section 5).
- **A F-ER06-WRITE-ORDER (low) — FIXED.** On EXPORT the whole-object
  `TERMINAL-REVIEW.json` is written **before** the receipt `TERMINAL-EXPORT.json`
  (both via exclusive-create). A failure between the two can therefore never
  leave an export-claiming receipt without its review. On STOP only the receipt
  is written.
- **A F-ER03-OWNER-QUESTIONS / B F4-NO-LIVE-PID-RECHECK (low) — FIXED
  (liveness).** The terminal branch now performs the same live-PID/status
  recheck as the ordinary `--export-only` path, WITHOUT mutating `PROCESS.json`:
  a still-live original process refuses re-export; a missing one degrades the
  effective exit to `None`, which the selector STOPs on
  (`NON_ZERO_OR_MISSING_TERMINAL_EXIT`).
- **A F-ER03 / B F5-SILENT-MALFORMED-LINE (low) — FIXED (parse accounting +
  fail-closed).** The branch counts malformed capture lines
  (`capture_parse_errors`), records the count in the receipt
  (`captureParseErrors`), and passes it to the selector, which STOPs
  (`CAPTURE_PARSE_ERRORS`) if any line was malformed. Confirmed direction: a
  dropped line can only reduce the events seen, so it can only cause a STOP,
  never a wrong EXPORT.
- **A F-RETAINED-NONE-MEMBERSHIP-SKIP (low) — FIXED (fail-closed default).**
  `require_retained_membership` defaults to `True`; a valid typed object with no
  retained whole-candidate record STOPs (`RETAINED_SET_ABSENT`). For the current
  B scope the `AMBIGUOUS-COMPLETE-REVIEWS.json` record exists, so membership runs
  normally. The flag remains parameterised so the absent-set path is unit-tested.
- **B F7-NO-RECORD-HASH-PINS (info) — RECONCILED (both in-artifact pin AND root
  duty).** The receipt now records `preservedRecordSha256Before` (sha256 of each
  existing preserved record at read time). The branch exits before any
  existing-record write, so after == before. Root remains responsible for the
  external before/after verification of all preserved records and of the schema
  and candidate source; this policy states that duty rather than silently
  omitting it.
- **B F8-AUTHOR-IDENTITY-MISMATCH (info) — PRESERVED HONESTLY, NOT ADOPTED.**
  The historical author metadata self-reports session identity Opus 5.5 while its
  own process record shows actual `claude-opus-4-8` (30 assistant frames). That
  discrepancy is preserved as-is; the author claim is **never** adopted or
  relabelled, and neither author metadata object is a vote. This corrected
  source makes no claim about the author's self-report beyond recording the
  discrepancy.
- **B F9-DERIVED-FIXTURES (info) — ADDRESSED.** In addition to the derived UNIT
  fixtures, the corrected unit now also binds the **real** public packet
  fixtures (the two B whole objects, the real frozen B OUTPUT-SCHEMA, and the
  single A review) read-only when present, enumerating both B alternatives
  without a vote (section 4, methods 25-29).

## 3. Selection contract (what qualifies, and every STOP)

EXPORT requires **all** of, in order:

1. provider is `claude` (else `NON_CLAUDE_PROVIDER`);
2. requested model is in `allowed_models` (else `REQUESTED_MODEL_NOT_ALLOWED`);
3. original process `exit_code == 0` — not `None`, not non-zero (else
   `NON_ZERO_OR_MISSING_TERMINAL_EXIT`);
4. input pins unchanged (else `INPUT_PINS_CHANGED`);
5. zero malformed capture lines (else `CAPTURE_PARSE_ERRORS`);
6. exactly one actual assistant-envelope model, and it is in `allowed_models`
   (else `NO_ACTUAL_ASSISTANT_MODEL_FRAMES` / `MIXED_OR_WRONG_ACTUAL_ASSISTANT_MODEL`);
7. exactly one terminal `result` event (else `NO_TERMINAL_RESULT_EVENT` /
   `MULTIPLE_TERMINAL_RESULT_EVENTS`);
8. that result is a success (`subtype == 'success'`, not `is_error`) (else
   `FAILED_OR_NON_SUCCESS_TERMINAL_RESULT`);
9. `structured_output` present and a dict (else `STRUCTURED_OUTPUT_ABSENT` /
   `STRUCTURED_OUTPUT_NOT_DICT`);
10. `reviewToken` matches exactly (else `REVIEW_TOKEN_MISMATCH`);
11. the object validates against the frozen OUTPUT-SCHEMA (else `SCHEMA_INVALID`);
12. retained set present when required (else `RETAINED_SET_ABSENT`) and the object
    is canonical-sorted-key-JSON value-identical to one retained whole candidate
    (else `CANONICAL_OBJECT_NOT_IN_RETAINED`).

There is **no** fallback to assistant text, partial output, nested tool
arguments, or quoted/embedded JSON; the recursive whole-stream walk is never used
on this path. This is **authority selection** of the single provider-canonical
typed artifact only — not a convenient-result selection, not a relabel, not a
vote between candidates, and not a splice/merge/edit of any judgment field. A
valid typed object proves provider-artifact identity only; it proves no
cryptographic/runtime property and no finding materiality, and it does **not**
decide or close B's multiplicity STOP. Grounding: the official structured-outputs
doc (OFFICIAL-STRUCTURED-OUTPUTS.md:57, :381, :390) states the validated data
lives on the success `result` message, that a `success` result with no
`structured_output` must be treated as a failure, and that a model fallback can
retract a completed output mid-stream.

## 4. Frozen unit: exact method names, count, and expected outcomes

**FROZEN METHOD COUNT: 29** (24 always-run derived/pure + 5 real-fixture
skip-if-absent). **No unit, AST, compile, hash, or lint run NOW.** All GREEN
assertions below are **predicted only**, source-level, not observed.

Always-run (class `TerminalReviewSelection`, predicted GREEN/pass):
1. `test_derived_fixture_has_prefix_stream_multiplicity`
2. `test_exports_single_canonical_structured_output`
3. `test_ignores_quoted_and_nested_draft`
4. `test_contradictory_valid_nested_cannot_override_typed_object`
5. `test_reject_non_claude_provider`
6. `test_reject_requested_model_not_allowed`
7. `test_reject_non_zero_exit`
8. `test_reject_missing_exit`
9. `test_reject_pins_changed`
10. `test_reject_capture_parse_errors`
11. `test_reject_no_actual_assistant_frames`
12. `test_reject_mixed_actual_48`
13. `test_reject_no_terminal_result_event`
14. `test_reject_multiple_terminal_result_events`
15. `test_reject_failed_terminal_result`
16. `test_reject_structured_output_absent`
17. `test_reject_structured_output_string`
18. `test_reject_review_token_mismatch`
19. `test_reject_schema_invalid`
20. `test_reject_retained_set_absent`
21. `test_reject_canonical_not_in_retained`
22. `test_both_opus_authorized_actual_48_honest_identity`
23. `test_export_receipt_bounded_fields`
24. `test_membership_is_canonical_serialization_not_raw_byte`

Real-fixture, class `TerminalReviewSelection` (predicted GREEN/pass when the
correction packet is present at the pinned path; otherwise a defined SKIP, never
a failure):
25. `test_real_b_objects_validate_against_real_schema`
26. `test_real_b_objects_differ_only_at_b_i6_binding`
27. `test_real_b_selects_whichever_object_is_on_structured_output`
28. `test_real_b_third_variant_not_in_retained_stops`

Real-fixture, class `SeatAGrounding` (same SKIP-if-absent rule):
29. `test_real_a_is_single_object_distinct_token`

EXPECTED RED (documented, not run): with no selector present in the candidate,
`_load_selector` raises `AssertionError` and every method errors.

**Do not assume which B object is canonical.** Method 27 enumerates BOTH public B
whole objects: for each, a DERIVED UNIT stream carries that object on the single
success `result.structured_output`, and the selector exports exactly that object
by typed selection + membership — no historical vote. Method 28 shows a third,
schema-valid, token-matching variant STOPs on `CANONICAL_OBJECT_NOT_IN_RETAINED`.
All constructed stream frames are DERIVED UNIT frames, explicitly NOT provider
events and NOT Cardano ledger data.

## 5. The B-I6 escape binding (byte vs canonical, in ASCII)

`PUBLIC-DIFFERENCE.json` records that the two public B whole objects differ only
at `findings[5].evidence` (B-I6): one rendering carries the single character
U+0030 (the digit zero), the other carries the literal six-character escape token
(reverse solidus, letter u, 0, 0, 3, 0 — i.e. the escape for code point U+0030,
written here in ASCII to avoid a literal that the existing reader could collect
twice). These two strings parse to **distinct JSON values**, so their
canonical-sorted-key serialisations differ and membership never conflates them.
This is a label/rendering nit in the underlying addendum (B-I6, severity info,
"label-only"); it has no behaviour impact and is corrected only in a future
addendum revision, not here.

## 6. Bounded effects / time / memory / output envelope (frozen, for the future run)

- **Unit effects:** reads one public code file (the candidate) to AST-extract one
  function; when present, reads three public packet JSON fixtures
  (`B-WHOLE-OBJECTS.json`, `B-OUTPUT-SCHEMA.json`, `SINGLE-REVIEW-A.json`)
  read-only. Writes nothing, opens no socket, spawns no process, imports no
  dependency beyond stdlib + `jsonschema`, and reads no private capture, stream,
  stderr, log, auth, config, key, wallet, PK, SRS, or cache.
- **Time/memory:** well under 1 second and a few MiB; no network, no sleep, no
  unbounded recursion (the naive-walk helper bounds its own JSON re-parsing).
- **Output envelope:** the only runtime product is the unittest result stream;
  the unit writes no files. Expected (predicted) terminal line: 24 always-run
  methods OK, plus 5 real-fixture methods OK-or-skipped, with 0 failures and 0
  errors. Any deviation (a failure, an error, an unexpected extra line, a timeout)
  is a STOP with no retry and a re-review — never a loosened expectation.
- **Candidate `--export-terminal-review` effects (future, not now):** reads
  `TASK.json`, `OUTPUT-SCHEMA.json`, `PROCESS.json`, `provider.private.jsonl`, and
  the retained-candidate record read-only; performs a read-only liveness probe
  (`os.kill(pid, 0)`); writes at most the two new files via exclusive-create
  (review before receipt); never dispatches a provider; never mutates any
  existing record. Receipt fields are bounded/allowlisted (section 3 + the
  wrapper additions `id`, `exportMode`, `decidedAt`, `captureParseErrors`,
  `legacyRecursiveWholeObjectCount`, `legacyReaderRecordPreserved`,
  `preservedRecordSha256Before`, `nativeAuthority: 0`, `sourceGreenExecuted: false`).
  No secrets, partials, or raw provider content are ever exported.

## 7. Named gaps (not read, not assumed)

- The exact provider-envelope contract — whether `structured_output` is emitted
  at most once on the terminal `result`, and which private event produced B's
  second whole object (a second genuine `result`, quoted text, or a nested tool
  argument) — lives only in the raw `provider.private.jsonl`, which must not be
  read. If confirming the protocol priority of
  "`result.structured_output` over all other locations" requires that private
  envelope format, **that contract is the precise gap**; it is named, not read or
  assumed.
- **Wrapper-only behaviours (source-only gap for the unit):** the live-PID probe
  and the malformed-line counting live in the `--export-terminal-review` branch,
  not in the pure selector, so the unit cannot exercise them directly without
  importing the module or reading private files (both forbidden). Their
  **outcomes** are covered at the selector level: effective exit `None`
  (`NON_ZERO_OR_MISSING_TERMINAL_EXIT`) and `capture_parse_errors > 0`
  (`CAPTURE_PARSE_ERRORS`). The residual liveness race (a process that dies
  between the probe and the read) is a named source-only gap, not a test claim.
- This proposal does **not** assert which of the two retained B objects is
  canonical; the flag computes that at runtime from the capture (rule 12) and
  STOPs if the canonical object is not among the retained candidates.
- No native/resource authority is taken (`nativeAuthority: 0`). No project tests,
  SDK, prover, VM, ledger, or wallet are touched. M-HASH and the broader Moriarty
  / UTxO / finance / ACTUS / DeFi / PCD / Preprod work remain open and untouched.

## 8. Why a fresh independent security review is still required before use

The flag introduces **new write behaviour** (two new files) and a **new trust
rule** (treat `result.structured_output` as canonical and ignore every other
stream location), now plus a parameterised model gate, a fail-closed parse/
liveness/retained-set posture, and in-artifact record pins. These are new
semantics, not a like-for-like refactor. They therefore require **two fresh
agreeing non-author Opus audits (4.8 or 5.5) plus a root decision** before any
unit run or real export, then **one** ordinary unit verification, then a separate
root adoption decision. Only after that may the existing, already-trusted reader
read its *own* B capture in export-only mode; no other agent opens raw files, and
there are no provider retries. `realExportReady` is **false** regardless of any
unit outcome, because an actual observed unit result and a separate root adoption
decision must still follow.
