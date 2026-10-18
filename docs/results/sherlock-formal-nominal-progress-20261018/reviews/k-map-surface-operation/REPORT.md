### Reviewer Identity & Continuation Attestation

- **Reviewer**: Gemini 4 Argon (High), continuing independent non-author ordinary semantic/model-source review in conversation `31395ab6-a9f6-4c1e-95db-aec02eedb3d9` during observed Opus outage.
- **Execution Attestation**: No tools were called, no commands or tests were executed, no external models were queried, and no files were modified.

---

### 1. Exact Readable Coverage in This Continuation

All twenty inline blocks provided in this continuation were read and verified in full:

1. **`ROOT-CAUSE.md` & `REPORT.md`**: Localization of the `01-construct-asymmetric-kast` parse failure (exit `113` in `1.219s` at line `1`, column `4404` on `(`) following the successful `00-compile` (exit `0` in `4.238s`).
2. **`INPUT-FORMAT-DELTA.json`**: Exact byte sizes, SHA-256 digests, and before/after `Map` surface strings across all 7 `.term` files (removing only 4, 4, 6, 6, 6, 8, and 4 wrapper parentheses respectively).
3. **`context/SELECTED-MAP-SYNTAX.txt` & `context/COMPILED-MAP-AND-INJECTION.txt`**: Builtin `domains.md` (`lines 212–260`, `sha256=eb5b7c3cd...`) and compiled `compiled.txt` / `definition.kore` excerpts showing `Map` productions (`Map Map`, `.Map`, `KItem "|->" KItem` with `priority _|->_ > _Map_ .Map`), `KItem ::= Int` and `KItem ::= NGValue` (`inj{SortNGValue{}, SortKItem{}}`), and the absence of a `Map ::= "(" Map ")"` bracket production in the compiled program grammar.
4. **`context/compile-result.json`, `context/kast-result.json`, & `context/kast-stderr.txt`**: Complete execution receipts for `00-compile` (`returncode: 0`, `elapsed_seconds: 4.238`) and `01-construct-asymmetric-kast` (`returncode: 113`, `elapsed_seconds: 1.219`), plus full `kast` stderr showing the exact input query and failure caret at `,((612 |-> ngUInt(77)))),29)`.
5. **`REUSED-COMPILED-ARTIFACT-PINS.json` & `UNCHANGED-DEPENDENCY-PINS.json`**: Byte sizes and SHA-256 pins for all 15 generated files in `generic-nominal-product-model-v2-actual-tape-comparison-20261018/actual/nominal-kompiled`, the 3 compiled K source files (`25,043` bytes total), `nominal_trace_codec.py` (`sha256=6ddfc623...`), `compare_complete_tapes.py` (`sha256=43ac8f29...`), `PUBLIC-API.json` (`sha256=558f0f72...`), and `EXPECTATION-FREEZE.json` (`sha256=d4c9aee7...`).
6. **`comparison/COMPARISON-MANIFEST.json` & `review/EXACT-OPERATIONS-LOSSLESS-PROJECTION.json`**: Complete rebased offline comparison manifest and lossless 22-operation tail projection (`expanded_count: 22`, `expanded_canonical_sha256=d3a3e77a6afb9cb5d39c3ff85fc7643de992c9f67a9147f7ff51d94e9eed527c`, `full_plan_sha256=1c5040bf711ba4560661e51d3ea69b8f2263f4f1cbad418b169cb3a068851719`).
7. **All 7 Complete Corrected Input Query Files (`COMPLETE CORRECTED SAFE INPUT 01-construct-asymmetric.term` .. `07-zero-field-constructor.term`)**: Previously attested only by metadata in Turn 3, all 7 full `ngObserve(ngInitial(ngProgram(...), <map>), <bound>)` query files are now read inline byte-for-byte.
8. **`EXPLICIT UNCHANGED F03 READBASIS`, `UNCHANGED COMPLETE EXISTING LAUNCH CALLABLE CLOSURE`, & `UNCHANGED COMPLETE COMPACT IMPORT-TIME REPRESENTATION`**: Complete 4,228-byte `launch` callable closure and 15,521-byte statement-hashed import-time AST representation for `k_engine_attempt_f03_01.py`.

---

### 2. SOURCE & INPUT-FORMAT Verdict and Findings

- **SOURCE & INPUT-FORMAT VERDICT**: **PASS (SCOPED TO THE 7 SURFACE `Map` INPUT QUERIES PLUS UNCHANGED V2 MODEL/OBSERVER, CODEC/COMPARATOR, AND INDEPENDENT EXPECTATIONS)**

#### Material Findings on the Input-Format Correction

1. **Exact alignment with compiled `MAP` program syntax without changing model semantics or recompiling**:
   - As demonstrated by `context/kast-stderr.txt`, `context/SELECTED-MAP-SYNTAX.txt`, and `context/COMPILED-MAP-AND-INJECTION.txt`, `SHERLOCK-NOMINAL-PRODUCT-SYNTAX` imports `DOMAINS-SYNTAX` and `MAP` (without `BASIC-K`'s general bracket rules). In the generated `--input program` grammar (`syntaxDefinition.kore` / `compiled.txt`), `Map` has productions `KItem "|->" KItem`, `Map Map`, and `".Map"` with `priority _|->_ > _Map_ .Map`, plus subsort injections `KItem ::= Int` and `KItem ::= NGValue`, but no `"(" Map ")"` bracket production.
   - Consequently, `kast` rejected the outer/per-binding parentheses `((612 |-> ngUInt(77)))` at the first `(` token following `,`.
   - Removing only the outer and per-binding parentheses around the `Map` argument in `ngInitial(ngProgram(...), <Map>)` yields valid `Map` surface syntax (`612 |-> ngUInt(77)` for single-binding cases `01, 02, 07`; space-juxtaposed `610 |-> ... 612 |-> ngUInt(77)` for `03, 04, 05`; and `610 |-> ... 611 |-> ... 612 |-> ngUInt(77)` for `06`), where `|->` binds tighter than `Map Map` juxtaposition.
2. **Byte-for-byte preservation of all `NGProgram` ASTs, environment bindings, entry IDs, and observer bounds**:
   - Direct inspection of the 7 inline `.term` files confirms that every declaration (`x4()`), every expression node (`1000..2600` and occurrence `10000..26000` in `x0()`), every function row (`50, 51, 52` in `x3()`), every case entry ID (`2000, 2100, 2200, 2300, 2400, 2500, 2600`), every initial environment binding key/value/order (`610`, `611`, `612`), and every finite observation bound (`29, 29, 78, 22, 78, 71, 13`) matches the frozen independent case inputs from Turn 1.
   - No expected state, event, or result appears anywhere in the 7 `.term` files.
3. **Compatibility with unchanged `nominal_trace_codec.py` and `compare_complete_tapes.py`**:
   - In `nominal_trace_codec.py` (`sha256=6ddfc623...`), `PrintedTermParser.map_term()` loops over `.Map`, `Int |-> NGValue`, and optional `(...)` groups until the closing delimiter, sorting bindings into `@Map` by `name_id` without dictionary overwrite. Thus `map_term()` handles both unparenthesized `k1 |-> v1 k2 |-> v2` and parenthesized `Map` representations if emitted by `kprint`.
   - `comparison/COMPARISON-MANIFEST.json` changes only `actual_print`, `actual_kore`, and `actual_query` paths to `@ROOT@`, retaining the exact same 7 `expected_json` paths and `expected_json_sha256` digests.

---

### 3. Concrete 22-Operation Tail Eligibility & Limitations (Separate from Source Verdict)

- **OPERATION VERDICT**: **ELIGIBLE FOR ROOT SELECTION UNDER THE SAME QUALIFIED EXISTING `launch` BINDING**

#### Classification of the 22 Prospective Tail Operations (`review/EXACT-OPERATIONS-LOSSLESS-PROJECTION.json`)

1. **In-place reuse of the succeeded `00-compile` artifact directory (`compile: null`)**:
   - Because `00-compile` already succeeded (`returncode: 0`, `4.238s`) on the byte-identical 25,043-byte 3-file K source closure (`nominal-product-syntax.k`, `nominal-product.k`, `normal-trace-observations.k`), and only the `.term` query files changed, no recompilation is needed or queued.
   - All 21 K operations reference `@COMPILED@ = /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-nominal-product-model-v2-actual-tape-comparison-20261018/actual/nominal-kompiled` (pinned across all 15 generated files in `REUSED-COMPILED-ARTIFACT-PINS.json`).
   - No failed or passing prefix is replayed: `01-construct-asymmetric-kast` failed in the prior run and all 21 subsequent phases were unrun.
2. **Phases 1..21 (7 ordered cases × 3 stages `<case>-kast`, `<case>-reduce`, `<case>-print`)**:
   - **Concrete discriminator (Phase 1, `01-construct-asymmetric-kast`)**: Runs `kast` with `--definition @COMPILED@ --module SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS --sort NGTape --input program --output kore` on `@ROOT@/inputs/01-construct-asymmetric.term`. Under serial fail-stop, it immediately tests whether removing the `Map` wrapper parentheses resolves the column-4404 parse failure before any further child runs.
   - All outputs are written to fresh directories under `@ROOT@/actual/<stage>`, preserving the prior failed `01-construct-asymmetric-kast` receipt at `@BASIS@/actual/01-construct-asymmetric-kast`.
   - All 22 phases use fresh, unique 32-hex `SHERLOCK_K_ATTEMPT_TOKEN` strings.
3. **Phase 22 (`08-offline-complete-tapes`)**:
   - Runs `/usr/bin/python3 -B @BASIS@/comparison/compare_complete_tapes.py @ROOT@/comparison/COMPARISON-MANIFEST.json` in `@ROOT@/actual/08-offline-complete-tapes`. Since Python sets `sys.path[0]` to `@BASIS@/comparison`, `import nominal_trace_codec as codec` resolves directly to the unchanged `@BASIS@/comparison/nominal_trace_codec.py`.

#### Retained Operational Limitations & Narrowly-Owned Preconditions

1. **Unchanged `launch` callback & caller preconditions**:
   - Root invokes only `module.launch(record, phase["argv"], phase["environment"], phase["cwd"], phase["environment"]["SHERLOCK_K_ATTEMPT_TOKEN"], 536870912)` from the unchanged `k_engine_attempt_f03_01.py` closure (`sha256=996aadee...`).
   - Before each `launch(...)` call, Root must ensure `@ROOT@/actual/<stage>`, `@ROOT@/actual/<stage>/home`, and `@ROOT@/actual/<stage>/tmp` exist and contain no pre-existing `stdout`/`stderr` files, and that the 15 pinned files in `@COMPILED@` remain intact.
   - After each `launch(...)` call, Root must inspect `record` (`outcome == 'exited'`, `returncode == 0`, `not timed_out`, `not containment_breach`) to enforce serial fail-stop, and check exit `0` / `"all_cases_equal": true` on `08-offline-complete-tapes`.
2. **Per-child resource limits vs. lack of aggregate hard caps**:
   - **Enforced per child**: `30s` wall clock (`CHILD_SECONDS = 30`), `3s` `/proc` pgid+token sweep (`SWEEP_SECONDS = 3`), `start_new_session=True` with `SIGKILL` to process group on timeout and exit, `RLIMIT_CORE = 0`, `RLIMIT_FSIZE = 536870912` (512 MiB per file), `umask = 0o077`, and JVM flags `-Xms64m -Xmx512m -Xss32m -XX:ActiveProcessorCount=2` on the 7 `<case>-kast` JVM children.
   - **Lack of aggregate caps**: No aggregate CPU, RSS/virtual memory, or multi-file disk hard cap is enforced on native/Python children (`.kore-exec-wrapped`, `kprint`, `/usr/bin/python3`); the arithmetic worst-case child+sweep wall-clock envelope across the 22 tail children is `22 * (30 + 3) = 726s` excluding Root overhead.

---

### 4. Readable-Coverage Limits & Retained OPEN Claims

- **Readable-Coverage Limits**:
  - All 7 corrected `.term` query files are now read in full inline (closing the prior turn's `.term` inline gap).
  - The 15 binary/text compiled artifacts in `@COMPILED@` and the 7 external `expected/*.json` files are attested by their SHA-256 pins (`REUSED-COMPILED-ARTIFACT-PINS.json` and `COMPARISON-MANIFEST.json`, with the expected contents already read losslessly via `independent-expected-tapes.k` in Turn 1).
- **Claims That Remain OPEN**:
  1. **Actual `kast` parsing, `kore-exec` reduction, `kprint` surface formatting, and offline comparison results**: All 22 tail operations are unrun; whether `01-construct-asymmetric-kast` and the remaining 20 K stages succeed, whether `kprint` output parses cleanly in `PrintedTermParser`, and whether all 7 actual tapes equal the independent JSON expectations remain OPEN until executed.
  2. **Canonical declaration elaboration, `CtorLayout`, `MatchPlan`, and `workMap` compilation (`UIREQ-034`, `SCON-016`, `interfaces.md`)**: Pre-assigned integer IDs and direct source-arm interpretation in `nominal-product.k` do not implement `TypeDecl`/`CtorLayout` derivation, `MatchPlan` decision-tree compilation (`Switch`, `Bind`, `Select`, `NoMatch`), or `workMap` cursor tracking.
  3. **Mutually recursive multi-type declaration groups and bounded parameterized specialization**: Only the single self-recursive nominal type `Schedule` (`type_id = 20`, `recursive_group = 9`) is exercised in the 7 cases; multi-type mutually recursive groups and bounded parameterized specialization remain OPEN.
  4. **Negative admission, binder-linearity enforcement, and mid-prefix technical cut resumption (`SCON-016`, `SCON-016.R02`)**: All 7 cases are positive normal executions (`ngHalted`) on closed, binder-linear inputs; `ngOutsideModel` rejection paths, non-linear binder handling, `NoMatch -> Rejected(error9)` mapping, and mid-prefix cut (`N < steps`) export/import resumption remain OPEN.
  5. **A39 / B helper clock, fuel/work charges, circuit/machine layout correspondence, native PCD proofs, financial semantics, and Preview/stagenet acceptance**: All cross-layer, resource-accounting, native proof, and product acceptance gates remain OPEN.

