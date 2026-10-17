# R4 test part A (R01-R08) notes

AUTHOR-PROPOSED source fragment by Opus 4.8 HIGH (both Opus 4.8 and Opus 5.5 are
authorized for this security authoring per USER-BOTH-OPUS-RULE.md; the actual
author here is Opus 4.8, honestly recorded, impersonating no other model).
Nothing was run, imported, compiled, hashed, reviewed or adopted. Authority 0.
Read / Write / Edit only; no shell, web, AST execution, import, compile, test,
hash, subprocess, prover, VM, Git or provider dispatch.

## Provenance of expectations
- Prefix literals and formatters (`R4-TEST-PREFIX.py.part`): roster, T1, HIST,
  PINS/RAW, cap constants, K6 vectors, `env_cbor`, `ctx_text`, `b2`, `k6_fields`,
  `dumps`, `canonical_json`, `read_bounded`, `plan`, `refused`, `run_argv`,
  `run_main`, `assert_cli_refusal`, `case_scratch`, `tree`, `fresh_export`.
- R4-CONTRACT.md section 7 (R01-R08) and CB401-CB412; R4-API.json
  (c2-builder-r4-api/2) for stages, caps, the JSON depth/int-char limits and the
  OUTDIR/READ errno sets.
- Historical R3 `test_c2_builder.py` vectors: R04 reuses test_06; R07 reuses
  test_05; R08 reuses test_04. Those are the legacy/host/r2-independent reference
  recipes, not R4 output.
- Pinned, R4-independent modules for a few expectations only: `legacy.extract_compiled`
  (R02 script.hex), `host_codec.statement_digest_hex` and
  `host_codec.statement_public_inputs` (R03 edited-statement consistency). r2 is
  reached only through the R4 plan for the R07 CONSUMER boundary.
- No expected value is derived from `build_k10_inputs_r4.py`; R4 output is only
  compared against independent expectations or asserted to refuse.

## First-call / RED invariant
Every method begins with `r4 = self.r4()`. At RED the module file is absent, so
`load_r4` calls `case.fail(MISSING)` and the method raises exactly one
`AssertionError('missing implementation: build_k10_inputs_r4.py')` before any
other assertion or effect. A present-but-broken module is a unittest ERROR (STOP),
never RED.

## Helper compatibility with the frozen prefix
No incompatibility found; no prefix edit, rebinding or work-around is required.
Two points worth recording for the combined review:
- `refused()` asserts `ValueError`. A RED `self.r4()` raises `AssertionError`
  first, so the MISSING failure still surfaces as the method's single assertion
  failure even where a later line would have used `refused()`.
- `run_argv` passes `argv` unchanged, which R04 depends on to deliver the
  non-list, non-str-element, NUL and lone-surrogate argv vectors with their exact
  types; `run_main` str-coerces and is used only where that is wanted (R05 paths).

## Risks and source-only oracle limits (disclosed)
- R03: that r2 admits the edited export through STATEMENT/CONTEXT/DIGEST/CARDANO
  is asserted on the pinned r2 source and the R3 reference recipe, constructed to
  be consistent via the pinned host helpers; it is not executed here. The fixture
  is an explicit security negative control (genuine proof reused, invalid for the
  edited statement), not a new proof or ledger state.
- R06: whether `O_NONBLOCK/O_NOFOLLOW/O_NOCTTY` are set and whether a raced
  device open-time effect occurs are not observable from allowed instrumentation;
  the fifo case shows the no-follow lstat guard refuses before any open. Caps are
  exercised at SETUP_CAP so writes stay within the 2 MiB fsize envelope; the
  16 MiB B9_CAP boundary is not written with a real file.
- R07: the depth-16 and 20-char-signed vectors assert only that
  `parse_public_json` returns a dict; this gate is the builder's own bounded check,
  not Rust `parse_json`, and the `jsonParseLimits` label stays NOT_CHECKED.
- Escape values are described in metadata by Unicode code point names (REVERSE
  SOLIDUS U+005C, LINE FEED U+000A, NULL U+0000, ARABIC-INDIC DIGIT ONE U+0661,
  lone high surrogate U+D800) to avoid export-variant ambiguity; the source file
  keeps correct Python escaping and builds the backslash-u JSON escape with
  `chr(92)` exactly as the R3 reference did.
- Environmental exceptions not mapped by the contract (e.g. non-C2B OSErrors on
  write, MemoryError/OverflowError propagation) are not probed by these 8 methods;
  they remain combined-source audit obligations and, where allowed instrumentation
  cannot observe them, that gap is labelled here rather than filled with a
  forbidden monkeypatch.

## No security claims
K6 is a genuine local proof only; its context is unauthenticated. K2 is a
single-party TEST ONLY, forgeable setup. These tests assert no Rust acceptance,
curve, Groth16, VK-profile, statement re-encode, ledger, fee, ExUnits or fit
property; all 20 UTxO families, full finance, ACTUS, DeFi, PCD recursion, private
handoff, multiparent and Preprod remain OPEN.

## Files read
Fully read: MANIFEST.json, R4-CONTRACT.md, R4-API.json, R4-TEST-PREFIX.py.part,
R4-TEST-INTERFACE.json, HISTORICAL-R3-test_c2_builder.py.txt,
ROOT-CP08-SOURCE-ADOPTION.json, ROOT-PUBLIC-FIXTURE-PINS.json,
USER-BOTH-OPUS-RULE.md, c2_issue_artifacts_r2.py, host_codec.py,
K6-PUBLIC-PROOF.json.

## Not read (disclosed)
Within the packet I did not read: R4-PREFIX-NOTES.md, build_k10_inputs.py (full
body; only `extract_compiled`/`frozen_positive_cbor` behaviour relied on via the
prefix R00 and R3 reference), K2-PUBLIC-SETUP-RECORD.json (used only through the
prefix SETUP global and pins), HIST-09-positive-script-context.txt (used as the
pinned RAW[HIST] bytes), SOL-RENDERER.py.txt, SOL-INTEGRATION-CONTRACT.md,
K2-PUBLIC-SETUP-RECORD.json, B9-APPLIED-BLUEPRINT.json. No provenance path, and no
private/raw/auth/config/key/PK/SRS/cache/wallet/home/research-root file, was
opened. No `build_k10_inputs_r4.py` exists or was created; prefix, interface and
other parts were not edited.
