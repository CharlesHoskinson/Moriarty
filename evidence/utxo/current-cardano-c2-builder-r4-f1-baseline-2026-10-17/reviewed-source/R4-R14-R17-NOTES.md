# R4 test part C (R14-R17) notes

Author: actual Opus 4.8 HIGH (both Opus 4.8 and Opus 5.5 are authorized for this
security work per USER-BOTH-OPUS-RULE.md). Requested model was Opus 4.8 HIGH; this
is an honest Opus 4.8 response, not relabelled. Authority 0. Nothing here was run,
imported, compiled, hashed, reviewed or adopted; no RED or GREEN was observed; no
test executed; no implementation, source-ready or 18-test claim is made.

## What this part is

`R4-R14-R17.py.part` is an independent source fragment: exactly the four unittest
methods `test_r14_*` through `test_r17_*` in `class TestR14ToR17(R4TestBase)`, plus
the single module footer `if __name__ == "__main__": unittest.main(verbosity=2)`.
It appends byte for byte after the frozen prefix, part A (R01-R08) and part B
(R09-R13), in prefix/A/B/C order. It does not create `build_k10_inputs_r4.py`, never
edits the prefix, interface or other parts, and does not write a `build_k10_inputs`
anything.

## Provenance of expectations

Every expected value is a frozen contract/API literal, a prefix formatter/literal,
or a pinned-fixture fact, never output of the builder under test:

- R14 roster and modes: `ROSTER`/`CASES` from the prefix, `tree()`, and the mode
  literals 0o644 (files) and 0o755 (dirs) from CB404/CB411 and R4-API openFlags.
- R15 stages and precedence: `C2B-ARGV/OUTDIR/READ` from `REFUSAL_STAGES` and the
  CB403/CB404/CB405 order and the section 3 stage table.
- R16 mappings: EACCES on a 0o555 parent to `C2B-WRITE` (first mkdir) and a mid-run
  input-copy byte change to `C2B-REREAD` with cleanup, both from CB411.
- R17 host APIs: the seven from `CONSUMER_HOST_APIS` (API dependency.consumerHostApis)
  and the four from `BUILDER_HOST_APIS` (CB409/CB410 equality checks). The r2 and
  host_codec bodies were read only to confirm the consumer's call sites
  (parse_statement_header in STATEMENT; statement_digest_hex and
  statement_public_inputs in DIGEST; build_env_data, encode_data, build_c2_mint_data,
  build_value_data in BUILD); that reading is independent of R4 and seeds no builder
  expectation.

HISTORICAL-R3 `test_10` is the pattern source for the CLI success and refusal shapes;
its v1 consumer and `/1` manifest literals are intentionally dropped.

## RED shape

Each method's first statement is `self.r4()`. With `build_k10_inputs_r4.py` absent,
`load_r4` reaches `case.fail(MISSING)` and raises one `AssertionError` with the text
`missing implementation: build_k10_inputs_r4.py`; no later assertion or filesystem
effect runs. So at RED each method fails with exactly that one AssertionError and
nothing else. A present-but-unloadable module is the prefix loader's RuntimeError
STOP (unittest ERROR), never RED.

## Helper / prefix compatibility

No incompatibility found. The base-class and module helpers cover every R14-R17
need as composed: `run_main`/`run_argv`, `assert_cli_success`, `assert_cli_refusal`,
`case_scratch`, `copy_inputs`, `assert_inputs_unchanged`, `consumer_wrapper`,
`counted`, `tree`, `require_unprivileged`, `fresh_export`/`fresh_setup`, and the
frozen literals. No prefix global is rebound and no helper is redefined. No local
stdlib import is added; the part uses only prefix module-level names (contextlib,
os, stat, unittest) and builtins, and imports no code under test to form expected
values. If a later full-source review finds an incompatibility, it must be recorded
rather than patched around in the prefix.

## Risks and source-only limits

- Umask: there is no read-only umask API in the stdlib, so R14 observes it with a
  set/restore pair (`umask = os.umask(0o022); os.umask(umask)`), net unchanged, before
  `main` runs under the restored real umask. The transient process-global touch is a
  disclosed source effect for full-source review; it is not a patch of os and changes
  no R4 behaviour.
- R16 must run unprivileged; uid 0 is STOP (not bypassed) via `require_unprivileged()`.
  It chmods its own scratch subdir to 0o555 and restores 0o700 in a finally so the
  scratch tree stays cleanable; the 0o555 window is confined to one case_scratch dir.
- R17 asserts host-call presence by the run-minus-baseline difference (the consumer
  reaches all seven; the builder adds exactly its four) rather than exact per-API
  totals, because totals depend on R4 internals that must not seed expectations, and
  to avoid any blanket all-host or no-alias inference.
- Source-only gaps (also in the contract): no durability/fsync claim; internal
  cleanup order and the builder's internally parsed dicts are not source-observable;
  whole-filesystem non-touch beyond the pinned inputs and the scratch tree is a
  source-audit obligation that allowed instrumentation cannot fully observe;
  contract-unmapped environmental exceptions propagate per the API and are never
  manufactured by a forbidden monkeypatch.

## Unread / not consulted

Read in full: R4-CONTRACT.md, R4-API.json, R4-TEST-PREFIX.py.part,
R4-TEST-INTERFACE.json, HISTORICAL-R3-test_c2_builder.py.txt,
ROOT-CP08-SOURCE-ADOPTION.json, ROOT-PUBLIC-FIXTURE-PINS.json, USER-BOTH-OPUS-RULE.md,
MANIFEST.json. Read in part for the R17 counter derivation: c2_issue_artifacts_r2.py
(header and stage body) and host_codec.py (header). Not needed and not consulted for
this part: build_k10_inputs.py body, K6-PUBLIC-PROOF.json, K2-PUBLIC-SETUP-RECORD.json,
HIST-09-positive-script-context.txt, SOL-RENDERER.py.txt,
SOL-INTEGRATION-CONTRACT.md, R4-PREFIX-NOTES.md bodies (their relevant facts reach
this part through frozen prefix literals and the API). No provenance path, and no
private/raw/auth/config/key/PK/SRS/cache/wallet/home/research-root location, was
opened. The four manifest fields not fully restated in the API
(compiledValidatorTitle, scriptHexFile, scriptCompiledBytes, scriptInnerFlatBytes)
are owned by the manifest/R13 part, not by R14-R17; this part states no literal for
them and does not guess.
