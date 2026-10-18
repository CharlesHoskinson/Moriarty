# R4 test part B (R09-R13) - author notes

AUTHOR-PROPOSED source fragment by actual claude-opus-4-8 HIGH (authorized alongside
claude-opus-5-5 by USER-BOTH-OPUS-RULE.md). Nothing was run, imported, compiled,
hashed, reviewed or adopted. Authority 0. This is an independently authored source
fragment only; it makes no testsRun / sourceReady / adoption / "complete 18 tests"
claim. The full combined-source and R00 compatibility review happens after all parts.

## Scope
Exactly five unittest methods in `class TestR09ToR13(R4TestBase)`, appended
byte-for-byte after `R4-TEST-PREFIX.py.part` (part prefix) and the R01-R08 part
(part A), before the R14-R17 part (part C) which owns the only `unittest.main`
footer. This part writes no footer, creates no `build_k10_inputs_r4.py`, and edits
no prefix / interface / other part.

- test_r09: CB407 POLICY pin and extractor guard (mismatch, bytearray, over-cap;
  counter proving `legacy.extract_compiled` is not reached, plus a genuine-plan
  baseline of one call).
- test_r10: CB408 CONSUMER conformance (verbatim C2I-EXPORT/SETUP/PROFILE wrap,
  nonconforming collapse to the fixed literal) and LINK-before-CONSUMER ordering.
- test_r11: CB409 BRIDGE - all 22 disagreements (the 13 R3 test_08 edits plus the
  nine R4-specific ones named in contract R11), each failing closed C2B-BRIDGE.
- test_r12: section 5 `legacy_env_from_host` - independent tree equality, exact
  non-mutation of input, and seven shape/type mutations refusing C2B-BRIDGE.
- test_r13: CB410/CB412 - T5/T6/T7 contexts, the complete manifest literals
  (33 keys, 16/9/10/4 maps, gaps, inputs, cases) and the no-'subset' scan.

## Provenance of expected values
Every expectation is an independent literal or formatter from the prefix, or a value
from the pinned, root-adopted `r2` / `legacy` / `host_codec` modules - never from
`build_k10_inputs_r4.py`. `r2` is a dependency oracle (CP08 scope: R4 test authoring
only), not the code under test. The R11/R12 `good` baseline is a direct
`r2.prepare_c2_issue(fresh_export(), fresh_setup(), K6_EXPIRY, policy_id=...)` call,
matching R3 test_08. The R12 positive oracle is `legacy.env_data(vkDigest,
statementDigest)`, exactly the R3 test_08 comparison. R13 reuses `k6_rows`,
`expected_context`, `expected_case`, `expected_gaps`, `expected_inputs` and
`walk_strings` from the prefix.

## r4()-first and RED
Each method's first statement is `self.r4()` (test_r12 binds `r4 = self.r4()`), so at
RED - with the module absent - `load_r4` raises the single
`missing implementation: build_k10_inputs_r4.py` AssertionError before any other
assertion or effect (including the R11/R12 `r2(...)` baseline and every wrapper).
A present-but-unloadable module or any pin/prerequisite failure is a unittest ERROR =
STOP, which this part neither produces nor masks.

## Helper compatibility
The shared prefix helpers compose cleanly for this span: `self.plan` (with the `b9=`
and `export=` keyword paths), `self.refused`, `self.counted`, `self.faulted` and
`self.consumer_wrapper`. `consumer_wrapper(after)` runs `after(result)` before
returning, so an in-place `result[key] = value` mutation reaches the BRIDGE checks -
the mechanism R11 needs. `counted`/`faulted`/`consumer_wrapper` all restore the
original attribute in `finally` via `wrapped`, which only permits `r2`, `host_codec`
and `legacy`. No incompatibility was found; no prefix byte, global or helper is
edited, rebound or worked around. (Had one been found, it would be stated here and in
the contract, not patched.)

## Escaping
The R10 CR and LF nonconforming vectors are written with `chr(10)` (Unicode code
point LINE FEED) and `chr(13)` (Unicode code point CARRIAGE RETURN) rather than
in-string backslash escapes, so the injected messages are unambiguous. `swap1` (first
hex-digit swap) and `flip1` are the prefix helpers.

## Risks / source-only limits
- R09 over-cap builds a 16777217-byte object; it drives the cap branch but, as a
  source test, cannot observe whether the implementation copies input-proportionally
  before the size check.
- R10 conformance is asserted via injected faults; genuine C2I provenance inside r2
  is r2's own obligation.
- R11/R12 trust `r2` as the oracle; r2's own correctness is separately reviewed.
- R13 asserts the 33 manifest keys but intentionally does not assert values for
  compiledValidatorTitle / scriptHexFile / scriptCompiledBytes / scriptInnerFlatBytes:
  those four are not restated in the R4 API and R13 does not require their values, so
  they are deliberately left to whichever part reads the pinned `build_k10_inputs.py`
  main literals rather than guessed here.
- No proof / curve / Groth16 / VK-profile / statement re-encode / Rust-acceptance /
  ledger / fee / ExUnits property is asserted. The K2 setup is single-party TEST ONLY
  and forgeable; the K6 proof is a captured local proof in an unauthenticated context.

## Normative inputs read (whole)
R4-CONTRACT.md, R4-API.json, R4-TEST-PREFIX.py.part, R4-TEST-INTERFACE.json,
HISTORICAL-R3-test_c2_builder.py.txt, ROOT-CP08-SOURCE-ADOPTION.json,
ROOT-PUBLIC-FIXTURE-PINS.json, USER-BOTH-OPUS-RULE.md, MANIFEST.json. host_codec.py
was read only in part (encoder head) and is not called directly by this part;
build_k10_inputs.py and the K6/K2/B9/HIST/SOL bodies were not fully read because this
span derives its expectations from prefix formatters and the r2/legacy module objects
rather than from those raw bodies.

## Imports
No local stdlib import was added; this part uses only names already imported by the
prefix (`copy`, `json`) and the prefix globals/helpers. (Disclosed per the part
rules; nothing under test is imported to form expected values.)
