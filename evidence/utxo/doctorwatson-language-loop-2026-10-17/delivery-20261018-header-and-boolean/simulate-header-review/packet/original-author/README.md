# Ordinary simulate profile diagnostics — source-only candidate

Normal public `simulate` discards a failed `mori_core::language` result and falls through to the Core/1 parser. Consequently an unknown or malformed header is described as requiring only Core/1 even though the command also dispatches Core/2. The candidate propagates the existing producer error before dispatch, using the existing CLI operation renderer. No lexer, header acceptance or profile semantics change.

The predicate is: three frozen unknown/malformed header sources retain the complete existing library Error JSON (including byte offsets), and actual public simulate returns their frozen complete human diagnostic streams; a valid empty Core/2 action still returns a complete report with the entire expected empty Execution. This tests ordinary CLI routing and diagnostics. It supplies no compiler Artifact, Target, financial transition, security or target acceptance result.

Base is exact `60523062e1b93a3f901b497da7f2cbc727970a20` (tree `509935958d6914544036444ec5317f0a55e3c9b2`); relevant production blobs equal `5f7c2a28efb69d36e171d85ffd39bf59f8e68c68`. `SOURCE-PINS-BEFORE.json` records actual base files, including the absent new test. `SOURCE-PINS-AFTER.json` records hypothetical unapplied candidate bytes, not an after commit. `current-source/` and `SOURCE-PINS.json` retain 104 full source bodies and exact Git/blob/SHA pins. `before/` and `candidate/` retain full changed source. Source-only pins do not assert runtime results.

## Reproduce and integrate

Apply `RED-test-only.patch` first. Run the RED command in COMMANDS.md against unchanged production. It is one aggregate Rust test: anticipated RED is 0 passed / 1 failed, stopping at the first human-stream assertion. All three error processes, complete library JSON errors, and the valid control process are captured before that assertion. Control assertions are not reached on that RED failure; root may separately compare captured control JSON with the frozen complete Execution projection.

Then apply `production-only.patch` and run GREEN with a fresh capture root. Anticipated GREEN is one aggregate test passing all three error cases and the control. These are proposed outcomes, not observed results. Combined `candidate.patch` is an alternative full source patch; do not apply it after the individual patches.

The test accepts optional `MORI_SIMULATE_HEADER_OBSERVATIONS`: an absolute, nonexistent directory under an existing owned parent. It creates four subdirectories and retains each exact input, complete stdout bytes, complete stderr bytes and executable/args/exit JSON before assertions. Error cases also retain complete public-library Error JSON. A fresh path prevents overwriting prior observations. Without the variable, successful temporary output is cleaned, while failure output remains.

## Expectations and limits

`EXPECTED-FREEZE.sha256` and all its members were frozen before any product execution. Author source exposure is explicit in EXPECTED-CASE-SPEC.json; this is ordinary source-derived corroboration, not a blind financial oracle. Error JSON comes from the existing language producer specification/source, with full offset checks; public simulate has a human stream rather than a new JSON-error flag. Human rendering premises and pinned dependency source are retained separately. No execution was used to derive expected bytes. If actual rendering differs, preserve the freeze and failure before diagnosis.

The valid control uses a source with no state/accounts/financial statements. CORE-STAGED-TESTS.md:198 expressly allows a normally complete empty action and its complete empty segment. The expected Execution compares semantics, action, full pre/post snapshots, steps and effects. Other complete wrapper/report bytes are captured but work, hashes, continuation and qualification values are unasserted; this is not a whole report/Program/Artifact oracle.

`PRIOR-APPLICABILITY-DIAGNOSIS.json` explains that the older three-path Boolean proposal added new files, rather than calling missing APIs. It is retained without adaptation. `PRESERVED-27-CONSTRUCTOR-SCOPE.json` preserves all 27 constructor obligations and the finite prior corpus gaps; none closes through this diagnostic fix. U0–U7, M1–M17, mandatory original exits, unsupported grammar rules, full compiler/Target/Artifact, native and ledger obligations remain open. Existing reserve four-test results and immutable financial fixtures are untouched.
