### Reviewer Identity & Continuation Attestation

- **Reviewer**: Gemini 4 Argon (High), continuing non-author ordinary semantic/model-source review in conversation `31395ab6-a9f6-4c1e-95db-aec02eedb3d9` during observed Opus outage.
- **Execution Attestation**: No tools were called, no commands or tests were executed, no external models were queried, and no files were modified.

---

### 1. Exact Readable Coverage in This Continuation

All seven new inline blocks were read in full and cross-checked against the previously reviewed v2 model, observer, API schema, and independent oracle expectations in this conversation:

1. **`BOTH ACTUAL COMPILE FAILURE FACTS` & `SECOND COMPILE STDOUT`**:
   - Attempt 1 (`generic-nominal-product-model-v2-review-operation-20261018/actual/00-compile`): exit `113` (`1.023s`), `Could not find sorts: [Map]`.
   - Attempt 2 (`generic-nominal-product-model-v2-map-import-correction-20261018/actual/00-compile`): exit `3` (`6.048s`), `Terminating due to java.lang.OutOfMemoryError: Java heap space` when compiling the 62,832-byte `independent-expected-tapes.k` under `-Xmx512m`.
2. **`PREPARATION/EXACT SOURCE IDENTITY AND COMPLETE DATA COVERAGE`**:
   - Confirms the 3 K source files (`nominal-product-syntax.k` `4490B` `sha256=bf626f8...`, `nominal-product.k` `19167B` `sha256=0d7d4ec...`, `normal-trace-observations.k` `1386B` `sha256=f4eaa0b...`, totaling `25,043` bytes) are byte-identical to the last reviewed corrected source, with `independent-expected-tapes.k` (`62,832` bytes) excluded from the compiler closure.
3. **`COMPLETE PURE TAPE CODEC/PARSER` (`nominal_trace_codec.py`)**:
   - Complete Python source defining `configure`, `T`, `C`, `atom`, `chain`, `arms_index`, `encode`, `unchain`, `decode`, `render`, `LEX`, `PrintedTermParser`, `parse_printed`, and `expanded_expected`.
4. **`COMPLETE OFFLINE COMPARISON SOURCE` (`compare_complete_tapes.py`)**:
   - Complete Python source defining `first_difference` and `main`, reading `COMPARISON-MANIFEST.json`, verifying the 7 independent expected JSON SHA-256 digests, parsing each captured `kprint` stdout, decoding to full canonical state/event dicts, and comparing against `expanded_expected(fixture)`.
5. **`COMPLETE DECLARATION API` (`PUBLIC-API.json`)**:
   - Complete canonical payload order declarations for `NGState`, 7 `NGFrame` variants, 19 `NGEvent` variants, and 7 `NGControl` variants (identical to `review/PUBLIC-API-CANONICAL.json` reviewed in Turn 1).
6. **`COMPLETE OFFLINE INPUT/EXPECTED MANIFEST` (`COMPARISON-MANIFEST.json`)**:
   - Complete manifest binding `api`, `syntax`, `independent_freeze` (`d4c9aee74c507299413c4009e7a4e8545f7c076b166876a8f32bde3cd47d7826`), and all 7 cases with their `expected_json_sha256` pins, `actual_print`, `actual_kore`, `actual_query`, and `observation_bound`.
7. **`LOSSLESS EXACT23OPERATIONS`**:
   - Complete template projection (`expanded_count: 23`, `expanded_canonical_sha256=f2f67d7055772b314dd30ea9a91483e20411b7076d740520b3419b19844e91bc`, `full_plan_sha256=9ed7b5bb02e33b7d5ab3edf972927d84018446febbc86df2ad1a0f6327344ded`) for `@ROOT@ = /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-nominal-product-model-v2-actual-tape-comparison-20261018`.

---

### 2. SOURCE & OFFLINE COMPARISON Verdict and Findings

- **SOURCE & COMPARISON VERDICT**: **PASS (SCOPED TO UNCHANGED V2 MODEL/OBSERVER SOURCE AND COMPLETE-TAPE OFFLINE CODEC/COMPARISON LOGIC)**

#### Material Source & Codec Findings

1. **Unchanged K semantics and complete exclusion of expected data from compilation/reduction**:
   - The 3 K source files (`nominal-product-syntax.k`, `nominal-product.k`, `normal-trace-observations.k`, `25,043` bytes total) are byte-identical to the corrected source approved in Turn 2.
   - Excluding `independent-expected-tapes.k` (`62,832` bytes) removes all 142 factored expected constants and 7 expected tape terms from `kompile`, `kast`, `kore-exec`, and `kprint`. The main and syntax module is now `SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS`, and the 7 queries evaluate `ngObserve(ngInitial(P, E), bound)` of sort `NGTape` using only frozen source `NGProgram` and initial `Map` inputs.
2. **Complete retention of all state, frame, environment, and event fields in offline comparison**:
   - `expanded_expected(fixture)` expands each state in `initial_state`, `after_each_event`, and `expected_final_state` to all 5 `NGState` components: full `shared_program` (`declarations`, `expressions` with `id`/`occurrence`/`kind`, `functions`, `entry`), `control`, `environment`, `frames_top_first`, and the exact prefix `expected_events[:events_prefix_length]`.
   - `decode(actual_term, ctx)` decodes the parsed `NGTape` (`initial`, `after_each_event`, `final`) across all 5 `NGState` components, all 7 `NGControl` variants, all 7 `NGFrame` variants, and all 19 `NGEvent` variants using the exact field key orders from `PUBLIC-API.json`.
   - For `source_arms` (in `ngScrutinee`, `ngMatchStarted`, `ngScrutineeReturned`) and `remaining_source_arms` (in `ngChoose`), `decode` explicitly asserts `all(ctx['arms'][v0['source_arm_id']] == v0 for v0 in v)` before mapping to `source_arm_id` lists—verifying that every arm's decoded `pattern` and `body` inside every state/event matches `shared_program` before comparing arm ID order.
   - Every list sort (`NGStates`, `NGFrames`, `NGEvents`, `NGValues`, `NGTypes`, `NGExprIds`, `NGPatterns`, `NGCtors`, `NGFunctions`, `NGPath`, `NGArms`, `NGBindings`, `NGNodes`, `NGParams`, `NGDecls`) is decoded via `unchain(t)` in strict head-to-tail order with no truncation, stutter removal, or value-only projection.
3. **Whitespace, parentheses, and `Map` AC/unit normalization without binding loss**:
   - `PrintedTermParser` strips only whitespace and balanced outer parentheses around typed subterms, while `map_term()` accepts `.Map`, `Int |-> NGValue`, and parenthesized map groups.
   - `map_term()` accumulates all `(key, value)` pairs in a list (`rows.append(...)` / `rows.extend(...)`) rather than a Python `dict`, sorting by integer `name_id` (`tuple(sorted(rows, key=lambda row: row[0].args[0]))`). No duplicate binding can be silently overwritten, and empty token streams raise `ValueError('Expected complete finite environment Map')`.
4. **Fail-closed parsing on unknown or trailing tokens**:
   - `LEX` rejects any unparsed non-whitespace byte with `ValueError('Unparsed printed byte at ...')`.
   - `term(sort)` validates every constructor token against `REG` and enforces exact child sort signatures (`assert tuple(a.sort for a in args) == sorts`). Because neither `ngObserve` nor `ngStep` is in `REG`, any unreduced observer/step term immediately fails parsing.
   - `complete()` enforces `self.at == len(self.tokens)`, rejecting trailing tokens.
   - `compare_complete_tapes.py` verifies each `expected_json` file against its pinned `expected_json_sha256` (identical to the 7 frozen JSON SHA-256 pins in Turn 1), catches any parse/decode/IO error as `comparison_error` with `complete_tape_equal: False`, and exits `0` only if all 7 cases have `complete_tape_equal == True`.

---

### 3. Concrete 23-Operation Eligibility & Limitations (Separate from Source Verdict)

- **OPERATION VERDICT**: **ELIGIBLE FOR ROOT SELECTION UNDER THE SAME QUALIFIED EXISTING `launch` BINDING**

#### Classification of the 23 Prospective Child Operations (`LOSSLESS EXACT23OPERATIONS`)

1. **`00-compile` (1 JVM child)**: Compiles `@ROOT@/source/normal-trace-observations.k` (`25,043` bytes across the 3 source files, excluding the 62,832-byte expected module) with `--backend haskell --no-haskell-binary --main-module SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS --syntax-module SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS` under `-Xms64m -Xmx512m -Xss32m -XX:ActiveProcessorCount=2`.
2. **Phases 2..22 (7 ordered cases × 3 stages `<case>-kast`, `<case>-reduce`, `<case>-print` = 21 children)**:
   - `<case>-kast`: parses `@ROOT@/inputs/<case>.term` with `--module SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS --sort NGTape --input program --output kore`.
   - `<case>-reduce`: runs `.kore-exec-wrapped` with `--module SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS --depth 0 --smt none` to normalize the `NGTape` term into `@ROOT@/actual/<case>-reduce/result.kore`.
   - `<case>-print`: runs `kprint` on `result.kore` (`false false`) to capture the complete printed `NGTape` surface term in `@ROOT@/actual/<case>-print/stdout`.
3. **Phase 23 (`08-offline-complete-tapes`, 1 Python child)**:
   - Runs `/usr/bin/python3 -B @ROOT@/comparison/compare_complete_tapes.py @ROOT@/comparison/COMPARISON-MANIFEST.json` with `cwd = @ROOT@/actual/08-offline-complete-tapes`, `inherit_environment: false`, `SHERLOCK_K_ATTEMPT_TOKEN = be3f59b279b9f6b0e9f48ef0f68e0ea5`, and the same per-child resource/process-group limits.

#### Concrete Operational Limitations & Preconditions

1. **Unobserved `kprint` surface format compatibility**:
   - The data-only codec roundtrip (`PURE_FORMAT_AND_COMPLETE_DATA_ROUNDTRIP_ONLY_NOT_SUBJECT_TEST`) tested `nominal_trace_codec.py` against rendered literals and normalized `Map` AC/unit variations, not actual `kprint` output. Whether `kprint` emits surface syntax accepted by `PrintedTermParser` (specifically for `Map` bindings and `String` literals if any `ngOutsideModel` occurred) is unobserved until execution; any format discrepancy fails closed (`complete_tape_equal: False`, exit `1`).
2. **Co-location and materialization preconditions at `@ROOT@`**:
   - `@ROOT@/comparison/compare_complete_tapes.py`, `@ROOT@/comparison/nominal_trace_codec.py` (imported as `import nominal_trace_codec as codec`), and `@ROOT@/comparison/COMPARISON-MANIFEST.json` must be materialized in `@ROOT@/comparison/`.
   - `@ROOT@/inputs/01-construct-asymmetric.term` through `07-zero-field-constructor.term` must contain the 7 input-only `NGTape` queries (`ngObserve(ngInitial(...), bound)` without `==K xt_i()`).
   - For each of the 23 stages, `@ROOT@/actual/<stage>`, `@ROOT@/actual/<stage>/home`, and `@ROOT@/actual/<stage>/tmp` must exist with no pre-existing `stdout`/`stderr` files before `module.launch(...)` is called.
3. **Retained per-child resource and fail-stop constraints**:
   - **Enforced per child by `launch()`**: `30s` wall clock (`CHILD_SECONDS = 30`), `3s` `/proc` pgid+token sweep (`SWEEP_SECONDS = 3`), `start_new_session=True` with `SIGKILL` to process group on timeout and exit, `RLIMIT_CORE = 0`, `RLIMIT_FSIZE = 536870912` (512 MiB per file), `umask = 0o077`, and JVM flags `-Xms64m -Xmx512m -Xss32m -XX:ActiveProcessorCount=2` on the 8 JVM children (`00-compile` and 7 `<case>-kast`).
   - **Lack of aggregate caps**: No aggregate CPU, RSS/virtual memory, or multi-file disk hard cap is enforced on native/Python children (`.kore-exec-wrapped`, `kprint`, `/usr/bin/python3`); the arithmetic worst-case child+sweep wall-clock envelope across 23 children is `23 * (30 + 3) = 759s` excluding Root overhead.
   - **Caller fail-stop inspection**: Root must inspect `record` after each `launch()` call (`outcome == 'exited'`, `returncode == 0`, `not timed_out`, `not containment_breach`) to stop on first failure, and verify exit `0` plus `"all_cases_equal": true` from `08-offline-complete-tapes`.

---

### 4. Readable-Coverage Limits & Retained OPEN Claims

- **Readable-Coverage Limits**:
  - The 7 new `NGTape` query files `@ROOT@/inputs/01-construct-asymmetric.term` .. `07-zero-field-constructor.term` and the 7 external `expected/*.json` files are attested via `PREPARATION/EXACT SOURCE IDENTITY AND COMPLETE DATA COVERAGE` (`actual_query_input_only_initial_matches_independent_source_and_environment: true`) and the unchanged `expected_json_sha256` pins in `COMPARISON-MANIFEST.json` (which match the lossless `independent-expected-tapes.k` reviewed in Turn 1), rather than separate raw inline file dumps.
- **Claims That Remain OPEN**:
  1. **Actual K compilation, reduction, `kprint` formatting, and offline comparison results**: All 23 operations are unrun; whether `00-compile` succeeds within the 512 MiB JVM heap after removing `independent-expected-tapes.k`, whether `kprint` output parses cleanly in `PrintedTermParser`, and whether all 7 actual tapes match the independent JSON expectations remain OPEN until executed.
  2. **Canonical declaration elaboration, `CtorLayout`, `MatchPlan`, and `workMap` compilation (`UIREQ-034`, `SCON-016`, `interfaces.md`)**: Pre-assigned integer IDs and direct source-arm interpretation in `nominal-product.k` do not implement `TypeDecl`/`CtorLayout` derivation, `MatchPlan` decision-tree compilation (`Switch`, `Bind`, `Select`, `NoMatch`), or `workMap` cursor tracking.
  3. **Mutually recursive multi-type declaration groups and bounded parameterized specialization**: Only the single self-recursive nominal type `Schedule` (`type_id = 20`, `recursive_group = 9`) is exercised in the 7 cases; multi-type mutually recursive groups and bounded parameterized specialization remain OPEN.
  4. **Negative admission, binder-linearity enforcement, and mid-prefix technical cut resumption (`SCON-016`, `SCON-016.R02`)**: All 7 cases are positive normal executions (`ngHalted`) on closed, binder-linear inputs; `ngOutsideModel` rejection paths, non-linear binder handling, `NoMatch -> Rejected(error9)` mapping, and mid-prefix cut (`N < steps`) export/import resumption remain OPEN.
  5. **A39 / B helper clock, fuel/work charges, circuit/machine layout correspondence, native PCD proofs, financial semantics, and Preview/stagenet acceptance**: All cross-layer, resource-accounting, native proof, and product acceptance gates remain OPEN.

