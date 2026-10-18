### Reviewer Identity & Continuation Attestation

- **Reviewer**: Gemini 4 Argon (High), continuing independent non-author ordinary semantic/model-source review in conversation `31395ab6-a9f6-4c1e-95db-aec02eedb3d9` (original cwd `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-nominal-product-model-v2-review-operation-20261018/provider-ready-01/fresh-provider-cwd`) during observed Opus outage.
- **Execution Attestation**: No tools, shell commands, file operations, delegations, or model/compiler/subject executions were performed. Root-selected unchanged observer immediately records `MODEL-TERMINAL` independently of actual CLI exit.

---

### Overall Scoped Verdict: **ACCEPT**

- **SOURCE & BACKEND-DIAGNOSIS VERDICT**: **ACCEPT (SCOPED TO NIX/HASKELL BACKEND `--depth 0` -> `--depth 1` STRATEGY PROOF AND UNCHANGED V2 MODEL, OBSERVER, CODEC, COMPARATOR, AND INDEPENDENT EXPECTATIONS)**
- **15-OPERATION TAIL ELIGIBILITY VERDICT**: **ACCEPT / ELIGIBLE FOR ROOT SELECTION UNDER UNCHANGED `launch` CALLBACK AND RETAINED PRECONDITIONS (EXECUTION REMAINS UNRUN)**
- **BLOCKERS**: **NONE**

---

### 1. Exact Readable Coverage of Every Inline Body

All seventeen inline bodies provided in this continuation were read and verified in full:

1. **`PREPARATION-READBACK.json`**: Confirms all 7 prior `--depth 0` roots remained unreduced, 0 compile phases, 0 `kast` phases, 15 physical children with `backend_depth_argument: 1`, `first_case: "zero-field-constructor"`, unchanged model/codec/parser/comparator, and `corrected_backend_execution: "UNRUN"`.
2. **`PROPOSED-OPERATIONS.json`** (`sha256=d33af5cddf0adf3a1f65916148d6f7fb226ba76ddda787a25f7c60d57b47ddad`): Complete 15-phase operation plan (`07-zero-field-constructor-{reduce,print}`, then `01..06-{reduce,print}`, then `08-offline-complete-tapes`) with full `argv`, `environment`, `cwd`, `SHERLOCK_K_ATTEMPT_TOKEN`, and `limits`.
3. **`REPORT.md` & `ROOT-CAUSE.md`**: Complete diagnosis of the `--depth 0` strategy skip and specification of the 15-child `--depth 1` tail starting with `07-zero-field-constructor`.
4. **`REUSE-PINS.json`**: Byte sizes and SHA-256 pins for the succeeded `00-compile/RESULT.json`, all 15 compiled files in `nominal-kompiled`, all 7 succeeded `<case>-kast/RESULT.json` receipts, all 7 non-empty `<case>-kast/stdout` Kore pattern files (`38,999`–`41,238` bytes), all 7 `.term` inputs, the 3 K source files (`25,043` bytes total), `compare_complete_tapes.py` (`sha256=43ac8f29...`), `nominal_trace_codec.py` (`sha256=6ddfc623...`), and `EXPECTATION-FREEZE.json` (`sha256=d4c9aee7...`).
5. **`comparison/COMPARISON-MANIFEST.json`**: Rebased manifest pointing `actual_print` and `actual_kore` to `generic-nominal-product-model-v2-initial-simplification-tail-20261018/actual/...`, `actual_query` to the succeeded `map-surface-tail-correction` `.term` files, and `expected_json` to the 7 unchanged independent JSON files and SHA-256 digests.
6. **`context/ACTUAL-COMPARISON-REPORT.json`, `context/ACTUAL-REDUCE-RESULT.json`, `context/ACTUAL-REDUCE-STDERR.txt`, & `context/ALL7-ACTUAL-UNREDUCED-ROOTS.json`**: Complete failure evidence from the prior `--depth 0` run showing exit `0` on `kore-exec` with stderr `WarnDepthLimitExceeded: The depth limit 0 was exceeded`, unreduced root `LblngObserve'LParUndsCommUndsRParUnds'SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS'Unds'NGTape'Unds'NGState'Unds'Int{}` across all 7 cases, and the offline comparator's fail-closed rejection (`all_cases_equal: false`, `ValueError: Unknown constructor/sort ('ngObserve', 'NGTape')`).
7. **`context/ACTUAL-COMPILED-FUNCTION-AND-OBSERVER-EQUATION.txt`**: Excerpts from `nominal-kompiled/definition.kore` (`sha256=5ebae208...`, lines `451`, `483`, `10334–10354`) proving `ngInitial` and `ngObserve` are compiled `[function{}()]` symbols and the `ngObserve(S, N) => ngObserveLoop(S, S, ngNoStates(), N)` equational axiom is present in `definition.kore`.
8. **`context/SELECTED-BACKEND-DEPTH-AND-SIMPLIFIER-SOURCE.txt` & `context/SELECTED-BACKEND-SOURCE-BINDING.json`**: Derivation chain from `.kore-exec-wrapped` (`sha256=4ebe3cd5...`) -> `vvck3q3b4qjxjymnw7yflanmmbndkp8q-kore-exec.drv` -> `94lzp7h52y5w36yz7k68sq0q8x4lx9sd-kore-0.1.155.drv` -> `/nix/store/pmjs0mz53bz7p8jzzqx4nba1pfwyhf39-kore` and exact Haskell source excerpts from `app/exec/Main.hs` (`721–762`), `src/Kore/Exec.hs` (`281–363`), `src/Data/Limit.hs` (`54–79`), `src/Kore/Rewrite.hs` (`207–232`), and `src/Kore/Rewrite/Function/Evaluator.hs` (`185–218`).
9. **`review/EXACT-OPERATIONS-LOSSLESS-PROJECTION.json` & `HANDOFF.json`**: Lossless 15-phase projection (`expanded_canonical_sha256=a7c2b5665b8fe214545c071e7afb5a5c6c28409e3427b26b547fb2dbb8fd5ce3`, `full_plan_sha256=d33af5cddf0adf3a1f65916148d6f7fb226ba76ddda787a25f7c60d57b47ddad`) and handoff metadata.
10. **`EXISTING-LAUNCH-CALLABLE-CLOSURE.py.txt` & `IMPORT-TIME-AST-COMPACT.json`**: Complete unchanged 4,228-byte F03 `launch` callable closure (`sha256=996aadee...`) and 15,521-byte compact import-time AST representation (`sha256=f7f19f06...`) at their original `generic-nominal-product-model-v2-review-operation-20261018/context/` locations.

---

### 2. Source Proof & Changed-Approach Assessment

1. **The Haskell backend source closure conclusively proves why `--depth 0` skipped all `[function]` evaluation**:
   - In `src/Kore/Exec.hs` (`lines 358–363`), `execStrategy` is constructed as `Limit.takeWithin depthLimit $ [Begin, Simplify, Rewrite, Simplify] : repeat [Begin, Rewrite, Simplify]`.
   - In `src/Data/Limit.hs` (`lines 54–79`), `takeWithin limit bs` zips `bs` with `enumFromLimit limit (toEnum 1) = takeWhile (<= u) [1, 2, ...]`.
   - When `depthLimit` is `Limit 0` (`--depth 0`), `1 <= 0` is `False`, so `enumFromLimit (Limit 0) 1` evaluates to `[]` and `execStrategy` is the **empty strategy list `[]`**.
   - Consequently, `GraphTraversal.graphTraversal` in `Exec.hs` (`lines 312–332`) performs zero strategy transitions—skipping even the initial `Simplify` pass (`Pattern.simplifyTopConfiguration` in `src/Kore/Rewrite.hs` `lines 218–228`)—emits `WarnDepthLimitExceeded` for `0`, and outputs the unreduced input pattern `ngObserve(ngInitial(...), N)`.
2. **Why `--depth 1` is the minimal, exact backend correction**:
   - With `--depth 1` (`Limit 1`), `enumFromLimit (Limit 1) 1` is `[1]`, selecting the single strategy bundle `[Begin, Simplify, Rewrite, Simplify]`.
   - On `Start patt`, `Begin` preserves `Start patt` (`Rewrite.hs` `line 213`), and the initial `Simplify` calls `transitionSimplify Start patt` -> `Pattern.simplifyTopConfiguration` (`Rewrite.hs` `lines 218–219, 227–232`), which evaluates the compiled `[function]` equations (`ngInitial`, `ngObserve`, `ngObserveLoop`, `ngStep`, etc., verified present in `definition.kore`).
   - `--depth 1` is strictly a backend strategy-bundle selector for `kore-exec`; it does not alter any nominal K rule, observer step budget (`13, 29, 29, 78, 22, 78, 71`), A39/B clock/fuel semantics, or per-child resource caps.
3. **Fail-closed offline comparator confirmation**:
   - The prior `--depth 0` run empirically confirmed that `compare_complete_tapes.py` and `PrintedTermParser` fail closed (`exit 1`, `all_cases_equal: false`) when `kore-exec` exits `0` with an unreduced `ngObserve(...)` root, proving that child exit `0` alone cannot produce a false pass.

---

### 3. Exact 15-Operation Eligibility, Artifact Reuse, and Retained Qualifications

1. **Exact 15-child structure (`PROPOSED-OPERATIONS.json` / `EXACT-OPERATIONS-LOSSLESS-PROJECTION.json`)**:
   - **Zero compile or `kast` phases (`compile: null`, `kast: null`)**: Reuses in place the 15 SHA-256-pinned compiled files in `generic-nominal-product-model-v2-actual-tape-comparison-20261018/actual/nominal-kompiled` and the 7 SHA-256-pinned succeeded `<case>-kast/stdout` Kore files in `generic-nominal-product-model-v2-map-surface-tail-correction-20261018/actual/<case>-kast/stdout`. No failed or passing `reduce`/`print` prefix is replayed.
   - **Phases 1–2 (Diagnostic first pair: `07-zero-field-constructor-reduce` and `07-zero-field-constructor-print`)**: Evaluates the smallest 3-step nullary constructor query (`bound = 13`) with `--depth 1` and prints the resulting surface term. Root inspects the complete printed root before dependent continuation; any retained `ngObserve`, `#Bottom`, partial tape, timeout, or parse error is a failed observation.
   - **Phases 3–14 (Remaining 6 `<case>-reduce` / `<case>-print` pairs)**: Runs `--depth 1` reduction and `kprint` surface capture for `01-construct-asymmetric`, `02-construct-repeated`, `03-structural-sum-asymmetric`, `04-ordered-first-arm`, `05-structural-sum-repeated`, and `06-recursive-constructor-fields` into fresh directories under `generic-nominal-product-model-v2-initial-simplification-tail-20261018/actual/`, preserving all prior `--depth 0` outputs immutably.
   - **Phase 15 (`08-offline-complete-tapes`)**: Runs `/usr/bin/python3 -B .../compare_complete_tapes.py .../COMPARISON-MANIFEST.json` to verify all 7 printed `NGTape` terms against the 7 SHA-256-pinned independent JSON expectations across all 5 `NGState` components (`program`, `control`, current `environment`, ordered `frames_top_first` with `saved_environment`, and `events` prefixes) in `initial`, `after_each_event`, and `final`.
2. **Unchanged callback closure, resource limits, and caller preconditions**:
   - **Callback**: Root invokes only `module.launch(record, phase["argv"], phase["environment"], phase["cwd"], phase["environment"]["SHERLOCK_K_ATTEMPT_TOKEN"], 536870912)` from the unchanged F03 `launch` closure (`sha256=996aadee...`). No JVM children are queued in these 15 phases (7 Haskell `kore-exec` children, 7 LLVM `kprint` children, 1 Python comparator child).
   - **Enforced per child**: `30s` wall clock (`CHILD_SECONDS = 30`), `3s` `/proc` pgid+token sweep (`SWEEP_SECONDS = 3`) with `15` fresh unique `SHERLOCK_K_ATTEMPT_TOKEN` values, `start_new_session=True` with `SIGKILL` to process group on timeout and after exit, `RLIMIT_CORE = 0`, `RLIMIT_FSIZE = 536870912` (512 MiB per file), and `umask = 0o077`.
   - **Retained resource limitation**: No aggregate CPU, RSS/virtual memory, or multi-file disk hard cap is enforced on the native `kore-exec`, `kprint`, or Python children; the arithmetic worst-case child+sweep wall-clock envelope across the 15 children is `15 * (30 + 3) = 495s` excluding Root overhead.
   - **Retained caller preconditions**: Before each `launch(...)` call, Root must ensure `@ROOT@/actual/<stage>`, `home`, and `tmp` exist with no pre-existing `stdout`/`stderr` files and that all reused `@COMPILED@` and `@PARSED@` pins remain intact; after `07-zero-field-constructor-print`, Root must inspect the printed root before continuing, enforce serial fail-stop on every child, and require `returncode == 0` plus `"all_cases_equal": true` from `08-offline-complete-tapes` (since `kore-exec` exit `0` alone does not establish reduction to a terminal `ngTape`).

---

### 4. Readable-Coverage Limits & Retained OPEN Claims

- **Readable-Coverage Limits**:
  - All 17 current packet bodies are read in full inline; the reused compiled binary/text artifacts (`nominal-kompiled/*`), reused `kast` outputs (`<case>-kast/stdout`), and external independent `expected/*.json` files are attested by their unchanged SHA-256 pins in `REUSE-PINS.json` and `COMPARISON-MANIFEST.json` (with the complete model/observer K sources, 7 `.term` inputs, codec/comparator sources, and lossless expected tapes already read inline in Turns 1–4 of this conversation).
- **Claims That Remain OPEN**:
  1. **Actual `--depth 1` reduction, `kprint` surface formatting, and complete-tape equality**: All 15 proposed operations are **UNRUN**; whether `--depth 1` simplifies `ngObserve(ngInitial(...), N)` all the way to a concrete `ngTape(...)` within the 30s per-child wall limit, whether `kprint` output parses cleanly in `PrintedTermParser`, and whether all 7 actual tapes match the independent JSON expectations remain OPEN until executed.
  2. **Canonical declaration elaboration, `CtorLayout`, `MatchPlan`, and `workMap` compilation (`UIREQ-034`, `SCON-016`, `interfaces.md`)**: Pre-assigned integer IDs and direct source-arm interpretation in `nominal-product.k` do not implement `TypeDecl`/`CtorLayout` derivation, `MatchPlan` decision-tree compilation (`Switch`, `Bind`, `Select`, `NoMatch`), or `workMap` cursor tracking.
  3. **Mutually recursive multi-type declaration groups and bounded parameterized specialization**: Only the single self-recursive nominal type `Schedule` (`type_id = 20`, `recursive_group = 9`) is exercised in the 7 cases; multi-type mutually recursive groups and bounded parameterized specialization remain OPEN.
  4. **Negative admission, binder-linearity enforcement, and mid-prefix technical cut resumption (`SCON-016`, `SCON-016.R02`)**: All 7 cases are positive normal executions (`ngHalted`) on closed, binder-linear inputs; `ngOutsideModel` rejection paths, non-linear binder handling, `NoMatch -> Rejected(error9)` mapping, and mid-prefix cut (`N < steps`) export/import resumption remain OPEN.
  5. **A39 / B helper clock, fuel/work charges, circuit/machine layout correspondence, native PCD proofs, financial semantics, and Preview/stagenet acceptance**: All cross-layer, resource-accounting, native proof, and product acceptance gates remain OPEN.

