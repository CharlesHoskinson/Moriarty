## Verdict: APPROVE_WITH_SCOPE

Top finding: the exact RED, GREEN, and FOCUSED commands below are eligible for ordinary pure-Python execution, within the limits stated here. No substantive source or test correction is required before execution. This approval does not infer any test pass, authorize reliance, W landing, publication, or a new engine, and does not replace the required final actual-result audit.

No tools were used in this audit. No source or test was executed, imported, compiled, or hashed by this Muse context.

### Exact source identities, as supplied

These are the header/operator-supplied values. This no-tool audit did not independently recompute them.

- [k_capture_observations.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_capture_observations.py): 41183 bytes, SHA-256 `5df990ed9c8b54ca1e090e80883f483f2e745138a740cdbcbbfc80fd00ce77d1`
- [test_k_capture_observations.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/test_k_capture_observations.py): 88920 bytes, SHA-256 `15a4d0bf2a187bfdd549b665f2fc1e3df86fe3aced9e0bc6e42605a25dea2ad6`
- [k_observations.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_observations.py): 7686 bytes, SHA-256 `540128fdb7f693a7b1f415898fd74bd6c1918df50a7a48ce602471b6e85f9b07`
- [k_terms.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_terms.py): 5666 bytes, SHA-256 `c79a852cbda9259c736d36da29132486d10903007fd3adc755e7d0585b8366ca`
- [RED k_capture_observations.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/RED/models/sherlock/qa/k_capture_observations.py): 21483 bytes, SHA-256 `7f99f2955b92f183c7e4033fc14c46e0f2216dc2e9c8d370dad868d95a91ccbc`
- [TEST-MATERIALIZATION.json](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/TEST-MATERIALIZATION.json): 1094 bytes, SHA-256 `ed9e6355d9de8b87c6109e1487b64c8adb4031175e89c8874d8fb881eae2f021`

The frozen GREEN module therefore remains the stated 41183-byte module. Nothing in this approval authorizes editing it or the byte-exact test file, including its two preserved author no-op expressions.

### Transport coverage and read limitations

The final inline tail marker, `K-SOURCE05-CANONICAL-TESTS-END-20261016`, is present after [TEST-MATERIALIZATION.json](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/TEST-MATERIALIZATION.json). On the face of the supplied payload, the inline review copy is not truncated.

The following were **not** independently verified by this no-tool Muse context:

- The two original visible-message chunks, their stated byte lengths and hashes, or the zero-separator concatenation.
- The 94-character prefix, 318-character suffix, or exact Python-fence extraction boundaries.
- The reported AST pass.
- The on-disk byte counts or hashes of any supplied file.
- The existence or contents of the separate frozen one-case bundle, receipt tree, compiled-span artifact, or fixture-transport JSON.
- The asserted directory contents for the FOCUSED discovery command.

That limitation is material. It is not converted into a blocker for this exact pre-execution eligibility review because the request expressly allocates original-chunk verification to the separate Gemini security-owner review and reserves complete verification for the final actual-result audit. The operator must nevertheless verify the on-disk hashes before execution through the separately authorized process. If any on-disk size or hash differs from the values above, this approval no longer applies to that tree.

### Approved exact commands

Approve only these commands, in the stated directories, with logs and exit statuses retained separately:

1. RED, from `R/adapter-actual-source-05/RED/models/sherlock/qa`:

   `/usr/bin/python3 -I -B test_k_capture_observations.py ActualN01CaptureTests.test_actual_n01_capture_is_comparison_ready_without_authentication`

2. GREEN, from `R/adapter-actual-source-05/models/sherlock/qa`:

   `/usr/bin/python3 -I -B test_k_capture_observations.py`

3. FOCUSED, from the same GREEN directory:

   `/usr/bin/python3 -I -B -m unittest discover -p 'test_k_*.py'`

No other test selection, directory, interpreter flag set, engine command, subprocess, network operation, SDK/native/circuit/proof/key operation, write, or delegation is approved by this review.

### Pure-Python eligibility

The candidate and test imports are limited to ordinary standard-library facilities: `base64`, `hashlib`, `importlib.util`, `json`, `pathlib`, `re`, `time`, and `unittest`. On the supplied source, the candidate performs no K execution, Kore decoding/evaluation, subprocess launch, network access, proof work, or target/native correspondence.

The tests do read sibling source bytes for the fixed-source assertions and decode embedded base64 fixtures. That remains local pure-Python test activity. The `/usr/bin/python3 -I -B` form is appropriate: isolated mode and no bytecode writing.

FOCUSED is approved only on the supplied representation that the GREEN qa directory contains the new capture test plus the unchanged `test_k_observations.py` and `test_k_terms.py`, totaling the stated 18 shared tests. Discovery must not be broadened.

### RED interpretation limit

The selected RED test is not the source-hash-pin test. It calls the new `decode_capture` API and new capture schema against the exact source04 snapshot. A RED result is therefore meaningful as a non-pass for the new actual-capture contract.

Do not pre-label the outcome as an assertion failure. Against source04 it may present as a unittest `ERROR`, for example from the older `decode_capture` signature lacking `rule_spans`/`run_input`, or from rejection of the new capture keys if execution reaches schema validation. Either a `FAIL` or such an `ERROR` is a non-pass; a pass would invalidate the RED setup and must be investigated rather than treated as success. The final audit must judge the retained RED log literally, not this prediction.

### Substantive review findings

The GREEN module and 39-method suite are coherent on static review:

- Actual fixtures are base64 literals paired with transcribed sizes and SHA-256 values in `RECORDED`; the suite checks decoded fixture length and hash against that table and checks duplicate 06/07 Kore and 11/12 pretty outputs.
- The observed rewrite layout is closed: one `DebugAppliedRewriteRules` span, `WarnDepthLimitExceeded` naming the declared depth, and final `InfoExecDepth` 1. The terminal layout separately requires the explicit `No rules were applied.` variant and `InfoExecDepth` 0.
- Full-span mapping uses complete source path plus start/end line and column equality against the supplied inventory. Start line alone is not treated as identity; the unique id comes only from the matching inventory row.
- Raw log bytes are retained as base64 with size and hash before ASCII decoding. Log size, line count, ASCII, control-character, termination, blank-line, unclaimed-text, header severity, and entry-kind guards are present.
- The module distinguishes engine output Kore hashes from kprint stdout configuration hashes. `_link` records them separately, and the tests assert they are not conflated.
- Receipt key sets are exact. Engine and print receipts separately check exit status, empty counterpart streams, output/stdout hash links, distinct pattern/output paths, admitted option tokenization, exactly depth 1, standard untimestamped selected logging, and the kprint path chain.
- Startup is kept outside the post-startup comparison. Its before state remains logged Kore only, with no typed projection. The logged input-Kore pairing check is expressly not applied as a typed startup comparison.
- Only one `<k>` item of the exact form `X ~> .K` is reduced to `X`. Original configurations, decoded/reduced configurations, raw captures, and the unit-tail note are retained separately. Other K-sequence forms raise `ValueError`.
- The nine printed main.k cells are admitted; raw Kore’s tenth `generatedCounter` label is retained only as raw evidence and is not projected.
- Configuration size, nesting, printable-ASCII, and unknown-cell checks occur in candidate entry points before the shared configuration/term decoders are reached. Log, Kore, term, captures, argv, argument-length, and rule-span inventory bounds are also present.
- Malformed-flood coverage includes oversize and many-line logs, whitespace floods with a time assertion, deep or unbalanced configurations, hostile unit tails, unknown cells, receipt omissions and mutations, mixed-source bundles, and run-input mismatches.
- Hostile mutations are visibly synthetic copies of actual fixtures through `replaced()`, `relogged()`, and `reprinted()`. They are not represented as K runs.
- Actual fixture assertions are observations read from the fixture bytes and recorded metadata. They do not use an independent expected outcome subset, oracle, comparison result, or K-match data.

The two preserved no-op expressions do not require correction under the byte-exact instruction. In `test_receipt_consistency_blocks_edge_terminal_and_startup`, the conditional expression guarded by `False` does not invoke its nested helper, but the following explicit `run(rows)` assertions exercise the edge case. In `test_run_input_guards`, `None if False else 7` simply evaluates to the intended non-string test input. Both should remain untouched.

### Residual gaps

These remain open and must stay explicit in any result report:

- Receipts, argv, paths, statuses, artifact pairing, producer identity, and bundle `source_sha256` are consistency-checked declarations only. Nothing is authenticated.
- `ADAPTER_SOURCES` is hashed at import but, as the module itself states, is not bound to the loaded code.
- The supplied rule-span inventory and its unique ids are not independently authenticated against a compiled artifact by the candidate.
- The actual format is admitted from one frozen N01 fuel-1 capture. Other layouts, cells, sequences, and cases remain gaps.
- The startup pre-state and depth-0 initializer are not typed or compared; `generatedCounter` is not projected.
- Direct calls to the unchanged shared decoders in [k_observations.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_observations.py) or [k_terms.py](/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-parallel-20261016/k-actual-01/adapter-actual-source-05/models/sherlock/qa/k_terms.py) would not themselves inherit every candidate wrapper bound. The approved commands exercise the candidate’s guarded entry points and the existing shared tests; they do not expand that boundary.
- Fixture provenance remains dependent on the external receipt and transport records. Runtime fixture-hash assertions prove internal consistency with the transcribed `RECORDED` table, not independent K provenance.

### Final-review conditions

Before execution, the operator must confirm the on-disk files match the stated sizes and hashes and coordinate the single ordinary slot so the jobs do not overlap. Each command’s cwd, stdout, stderr, exit status, and timing should be retained without editing the sources between RED, GREEN, and FOCUSED.

The full final actual-result audit remains required. It must inspect the retained logs, distinguish RED `FAIL` from `ERROR`, verify that GREEN and FOCUSED ran the intended files, and separately address original-chunk/materialization provenance. This pre-execution approval must not be cited as a test result or as reliance authority.
