# Independent Non-Author Full-Source & Exact 17-Operation Review: Generic Nominal Product `Let` / `If` Successor (`generic-nominal-let-if-five-case-actual-qa-20261018`)

- **Reviewer Identity**: `Gemini 4 Argon (High)` (Independent Non-Author Ordinary Semantic / Model-Source Reviewer during observed Opus outage)
- **Conversation ID**: `31395ab6-a9f6-4c1e-95db-aec02eedb3d9` (Two-Part Manual Inline Continuation — Part 1 ACKed, Part 1 + Part 2 Complete Review)
- **Original Reviewer CWD**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-nominal-product-model-v2-review-operation-20261018/provider-ready-01/fresh-provider-cwd`
- **Target Candidate Root**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/generic-nominal-let-if-five-case-actual-qa-20261018`
- **Reviewed Freeze & Plan Digests**:
  - `SOURCE-FREEZE.json`: `10,582B`, `sha256=bec50e0bffb37aeb0997b754a1dedcebaaf093eafe0c2695b04e25a08b23b460`
  - `PROPOSED-OPERATIONS.json`: `53,084B`, `sha256=f81b7d10ae3e188ac1b095167f7a830c4ce193ad28c754690fcad6fa6e234f1c` (losslessly projected via `review/EXACT-OPERATIONS-LOSSLESS-PROJECTION.json`, `9,177B`, `sha256=dd7638e6135ed0b291f58f1fd264a217253c16d40ccb3027d9ebfb167ab21c19`, with canonical 17-object expansion `sha256=b0693df3cc655bd6af96f2d97ff813a9b9b0225fffcbf02996eba98515031cf0` proven in `EXACT17-EXPANSION-PROOF.json`, `3,099B`, `sha256=55f71364cd9aeb6b97147b7e83e5f1c9dd016ddb34e7d1b0895725442fcf2b98`)
- **Constraint Compliance**: Zero tool calls, zero shell commands, zero filesystem reads or writes, zero subagent delegations, zero model queries, and no subject or provider execution performed.

---

## 1. Full Inline Read Scope & Coverage Verification

Both manual inline parts (Part 1 and Part 2) were received and read in full with **no coverage gaps**:

### 1.1 Part 1 Inline Read Set
1. **Complete `original31` Approved-Source Context (`generic-nominal-let-if-source-successor-20261018/REVIEW-MEMBERSHIP.json`, `5,642B`, `sha256=80808cbb77acc44b9d86107194df760daabfc19342755e4fae5d905d15de2a58`)**:
   - All 31 member files read in full inline, including:
     - **3-File K Source & Observer Closure (`28,332B` total)**:
       - `source/nominal-product-syntax.k` (`5,090B`, `sha256=750f9b820a672865b2a1abc50bef4f417f2c6c02e7b77f661a0431388dacab8c`)
       - `source/nominal-product.k` (`21,856B`, `sha256=29da74d7ab2b89cb23155521b94ffb6880920befcbf9806941ba97826d47b197`)
       - `source/normal-trace-observations.k` (`1,386B`, `sha256=f4eaa0b5ad2969f2d0ab1324fc0894aba23cfe5140d23a7a81a58d54a3219f5f`)
     - **Delta, API & Design Specs**: `SOURCE.diff` (`6,293B`, `f8374ecb...`), `PUBLIC-API.json` (`10,944B`, `ebbdd2ea...`), `EXTENSION-API.json` (`2,329B`, `74898525...`), `DESIGN.md` (`3,117B`, `41af5988...`), `DESIGN-FREEZE.json` (`1,185B`, `ac0c1b3b...`), `PLAN.md` (`1,245B`, `77b8f9a3...`), `PRIMARY-CORRESPONDENCE.md` (`1,810B`, `f16bce5c...`), `SOURCE-FREEZE.json` (`1,102B`, `02f1570d...`), `SOURCE-ONLY-CHECKPLAN.md` (`2,571B`, `a059d4ac...`), `DATA-ONLY-READBACK.json` (`1,728B`, `f4f50973...`), `HANDOFF.json` (`1,626B`, `725d1b60...`).
     - **Normative Context & Safe Common Inputs**: `context/NORMATIVE-REFERENCE-PINS.json` (`1,694B`, `48a21960...`), `context/NORMATIVE-REQUIREMENTS.txt` (`8,378B`, `436db7bf...`), `inputs/SAFE-COMMON-INPUTS.json` (`14,294B`, `af670405...`), `inputs/INPUT-ONLY-PORT.json` (`1,870B`, `6c89fc4f...`), `inputs/NORMALIZED-INPUT-ONLY-PORT.json` (`2,147B`, `db88a946...`), the 4 normalized `.program.json` and `.initial-environment.json` files, and the 4 historical parenthesized-Map `.initial.ng` files (explicitly excluded from selected operations).
2. **Current Candidate Freeze, Alias Proof, and Manifests (`generic-nominal-let-if-five-case-actual-qa-20261018`)**:
   - `SOURCE-FREEZE.json` (`10,582B`, `bec50e0b...`), `BYTE-IDENTICAL-CURRENT-BODY-ALIASES.json` (`3,372B`, `79ec2f5b...`), `DEPENDENCY-PINS.json` (`2,307B`, `86f81932...`), `PREPARATION-READBACK.json` (`1,795B`, `220e6b7e...`), `SOURCE-AND-OPERATION-REVIEW-SCOPE.md` (`1,878B`, `9f214e55...`), `SOURCE-CUT.json` (`3,904B`, `e15edc26...`), and `comparison/COMPARISON-MANIFEST.json` (`8,301B`, `33c86806...`).

### 1.2 Part 2 Inline Read Set
1. **Complete Python Parser, Complete-Wire Adapter, and Two-Pass Comparator Source (`33,814B` total)**:
   - `comparison/complete_k_wire.py` (`13,275B`, `sha256=b475887749e86b77b55645651d7df442546d768684d855b316971803408dcb06`)
   - `comparison/complete_wire_comparison.py` (`6,351B`, `sha256=2f4de37bb23e033146f451274900842ca561b34d0683e2923c8b6f93fc5b10b7`)
   - `comparison/nominal_trace_codec.py` (`14,188B`, `sha256=6ddfc623f8870bffe5477933c24b52458c88e19fcc676127612fa3b4eb928cb6`)
2. **Variant Coverage, Ordinal Policy, and Safe Wire / Binding Schemas**:
   - `context/FULL-VARIANT-FIELD-COVERAGE.json` (`14,784B`, `99ea3e1e...`)
   - `context/ROOT-ORDINAL-POLICY.json` (`399B`, `260bc438...`)
   - `context/SAFE-BINDINGS.json` (`9,136B`, `e7cab45a...`)
   - `context/SAFE-WIRE-SCHEMA.json` (`3,726B`, `f4d9065c...`)
3. **All 5 Plain-Map Observation Queries (`.term`) and All 5 Plain-Map Initial State Literals (`.initial.ng`)**:
   - `inputs/01-let-if-call-true.term` (`696B`, `ec9167d2...`) & `inputs/let-if-call-true.initial.ng` (`680B`, `dcc44236...`)
   - `inputs/02-constructor-let-if-true.term` (`951B`, `91cb83ab...`) & `inputs/constructor-let-if-true.initial.ng` (`935B`, `5d1214ec...`)
   - `inputs/03-let-if-call-false.term` (`697B`, `4c83022a...`) & `inputs/let-if-call-false.initial.ng` (`681B`, `64a30699...`)
   - `inputs/04-constructor-let-if-false.term` (`952B`, `e0444e73...`) & `inputs/constructor-let-if-false.initial.ng` (`936B`, `cfb7ea20...`)
   - `inputs/05-constructor-let-if-nominal-call.term` (`1,112B`, `4d9ee1b5...`) & `inputs/constructor-let-if-nominal-call.initial.ng` (`1,096B`, `b144f079...`)
4. **Complete 17-Operation Plan Projection, Expansion Proof, Static Tool Pins, Predecessor Context, and F03 Launch Closure**:
   - `review/EXACT-OPERATIONS-LOSSLESS-PROJECTION.json` (`9,177B`, `dd7638e6...`)
   - `review-delivery-20261018/EXACT17-EXPANSION-PROOF.json` (`3,099B`, `55f71364...`)
   - `review-delivery-20261018/SELECTED-EXECUTABLE-STATIC-PINS.json` (`1,821B`, `d9736911...`)
   - `review-delivery-20261018/PRIOR-QUALIFIED-CONTEXT.json` (`4,642B`, `4eae1603...`)
   - `context/EXISTING-LAUNCH-CALLABLE-CLOSURE.py.txt` (`4,228B`, `996aadee...`)
   - `context/IMPORT-TIME-AST-REVIEW-PROJECTION.json` (`55,525B`, `9f9e9188...`)

---

## 2. Ordinary Source Correctness Review

### 2.1 K Syntax, `Let` / `If` Evaluator Extension, and Observer (`28,332B` Closure)

1. **Syntax & Payload Arity Alignment (`source/nominal-product-syntax.k`, `5,090B`, `750f9b82...`)**:
   - Retains `imports DOMAINS-SYNTAX` and `imports MAP` (preventing the missing `Map` sort error observed in v2 Attempt 1).
   - Preserves all 7 `NGControl` constructors (`ngEval`, `ngReturn`, `ngApply`, `ngChoose`, `ngMaterialize`, `ngHalted`, `ngOutsideModel`), all 7 original `NGFrame` constructors, and all 19 original `NGEvent` constructors with unchanged field sorts and positions.
   - Adds `ngLet(Int, Int, Int, Int)` and `ngIf(Int, Int, Int, Int)` to `NGExpr`, 3 new frames to `NGFrame` (total 10), and 5 new events to `NGEvent` (total 24).
   - Every constructor name, arity, child sort sequence, and field name in `nominal-product-syntax.k` matches `PUBLIC-API.json` (`ebbdd2ea...`) and `FULL-VARIANT-FIELD-COVERAGE.json` (`99ea3e1e...`), including the 8-argument `ngArgumentReturned` and 9-argument `ngConstructorFieldReturned` and `ngLetBound`.

2. **Lexical `Let` Evaluation, Shadowing, and Environment Restoration (`source/nominal-product.k`, `21,856B`, `29da74d7...`)**:
   - **Start (`ngEvaluate` on `ngLet(O, X, Bound, Body)`)**: Evaluates `Bound` first in the current environment `Rho`, pushes `ngLetValue(E, O, X, Bound, Body, Rho)` on top of `Fs`, and appends `ngLetStarted(E, O, X, Bound, Body, Rho)`.
   - **Bind (`ngStep` on `ngReturn(V)` with `ngLetValue(E, O, X, Bound, Body, Saved)`)**: Evaluates `Body` under `Saved[X <- V]`, replaces the top frame with `ngLetScope(E, O, X, Saved)` (preserving the pre-`Let` `Saved` environment so shadowed bindings are not lost), and appends `ngLetBound(E, O, X, Bound, ngOccurrence(ngFindExpr(Bound, ngNodes(P))), V, Body, Saved, Saved[X <- V])`. Notice that `bound_occurrence` is looked up from the original `Bound` expression ID in `ngNodes(P)` via `ngOccurrence(ngFindExpr(Bound, ngNodes(P)))`, so even when `Bound` is a composite constructor or call expression (as in Cases 02, 04, 05 where `Bound = 2` is `ngConstruct(1002, 20, 22, ...)`), `bound_occurrence` records `1002` rather than a child leaf occurrence.
   - **Restore (`ngStep` on `ngReturn(V)` with `ngLetScope(E, O, X, Saved)`)**: Pops `ngLetScope`, restores `Saved` as the current environment, returns `ngReturn(V)` to the caller's top frame on `Fs`, and appends `ngLetBodyReturned(E, O, X, V, Saved)`.

3. **Condition-Once `If` Evaluation and Selected-Branch-Only Execution (`source/nominal-product.k`)**:
   - **Start (`ngEvaluate` on `ngIf(O, Condition, Yes, No)`)**: Evaluates `Condition` once in `Rho`, pushes `ngCondition(E, O, Condition, Yes, No, Rho)` onto `Fs`, and appends `ngIfStarted(E, O, Condition, Yes, No, Rho)`.
   - **Branch Choice (`ngChooseBranch`)**:
     - On `ngBool(true)`, pops `ngCondition`, restores `Saved`, transitions to `ngEval(Yes)`, and appends `ngBranchSelected(E, O, Condition, ngOccurrence(ngFindExpr(Condition, ngNodes(P))), true, Yes, Saved)`. `No` is never evaluated.
     - On `ngBool(false)`, pops `ngCondition`, restores `Saved`, transitions to `ngEval(No)`, and appends `ngBranchSelected(E, O, Condition, ngOccurrence(ngFindExpr(Condition, ngNodes(P))), false, No, Saved)`. `Yes` is never evaluated.
     - On `notBool ngIsBool(V)` (`ngIsBool` is total over `ngBool`, `ngUInt`, `ngNominal`), transitions to `ngOutside(..., "condition_type_outside_model")` while retaining `ngReturn(V)` and the `ngCondition` frame.

4. **Composition with Named Calls and Pending `ngConstructorFields` (`SCON-016.R02`)**:
   - Because `ngConstructorFields(E, O, T, C, Ts, I, Active, Prefix, Rest, Saved)` remains on `Fs` beneath `ngLetValue`, `ngLetScope`, `ngCondition`, `ngArguments`, and `ngFunctionReturn`, evaluating a `Let`/`If`/call inside a constructor field (Cases 02, 04, and especially Case 05 `05-constructor-let-if-nominal-call.term`) preserves the outer constructor's `Active` field expression ID (`1`), its caller occurrence (`ngOccurrence(ngFindExpr(1, ngNodes(P))) = 1001`), its evaluated prefix, its remaining field list (`[9]`), and its outer saved environment.
   - No expected values, case-specific branches, or shortcuts appear anywhere in `nominal-product-syntax.k`, `nominal-product.k`, or `normal-trace-observations.k`.

### 2.2 Five New Input Queries (`inputs/01..05-*.term` and `*.initial.ng`)

- All 5 `.term` files and their corresponding `.initial.ng` files use **unparenthesized plain `Map` syntax** (`10 |-> ... 20 |-> ...`), avoiding the `kast` `--input program` parse error on outer/entry `Map` parentheses observed in v2 Attempt 3.
- All 5 `.term` files wrap the exact `.initial.ng` term in `ngObserve(..., 128)`.
- Manual step counts for the 5 cases are **14, 24, 9, 25, and 29 advancing steps** respectively (producing **15, 25, 10, 26, and 30 total states** including `initial`), so the observation bound `128` is strictly sufficient for every case to reach `ngHalted` without premature budget truncation, and `ngObserveLoop` stops immediately at `ngTerminal(C)` without terminal stutters.
- Case 05 (`05-constructor-let-if-nominal-call.term`) specifically exercises a named nominal function call (`func 30(param 40: Schedule) -> Schedule`) inside the `true` branch of an `If` inside the body of a shadowing `Let` inside the first field of `Both(field_0, field_1)` while `field_1` (`Pay(5, Done)`) remains pending in `ngConstructorFields`.

### 2.3 Offline Parser, Complete-Wire Adapter (`complete_k_wire.py`), and Two-Pass Comparator (`complete_wire_comparison.py`)

1. **Parser & Registry (`comparison/nominal_trace_codec.py`, `14,188B`, `6ddfc623...`)**:
   - Byte-identical to the audited v2 parser. `codec.configure` dynamically populates `codec.API` from `PUBLIC-API.json` and `codec.REG` from `nominal-product-syntax.k`, capturing all 7 controls, 10 frames, 24 events, and 8 expression constructors (including `ngLet` and `ngIf`).
   - Note: `nominal_trace_codec.py`'s legacy `encode`/`decode` functions (which only listed the 6 v2 expression kinds) are unused by the new pipeline; only `configure` and `parse_printed` (`PrintedTermParser`) are called, and `CompleteKWire` performs all term validation and decoding.

2. **Complete-Wire Adapter (`comparison/complete_k_wire.py`, `13,275B`, `b4758877...`)**:
   - Verifies at initialization that all 41 variants across `NGControl` (7), `NGFrame` (10), and `NGEvent` (24) in `PUBLIC-API.json` match `codec.REG` in sort, exact arity, and non-repeated field names.
   - `decode(term)` enforces exact sort and child-sort checks on every AST node, checks Python scalar types (`type(args[0]) is not scalar_types[sort]`), preserves all `@Map` rows sorted by `name_id` without dictionary overwrite, decodes all 8 `NGExpr` kinds (`Literal`, `Variable`, `Add`, `Construct`, `Call`, `Match`, `Let`, `If`) with separate `id` and `occurrence`, and formats controls, frames, and events as one-key objects `{constructor_name: {field: value, ...}}` matching `SAFE-WIRE-SCHEMA.json`.
   - `complete_states(tape)` fails closed (`ObservationFormatError`) if:
     - the root term is not `ngTape` of sort `NGTape` (e.g., an unreduced `ngObserve(...)` term),
     - `final != states[-1]`,
     - `program` changes in any state,
     - any state prior to the last has a terminal control (`ngHalted` or `ngOutsideModel`), or
     - `events` in step $k+1$ does not equal `previous["events"]` plus exactly one new event (`len(events) != len(prior_events) + 1 or events[:-1] != prior_events`).

3. **Two-Pass Comparator (`comparison/complete_wire_comparison.py`, `6,351B`, `2f4de37b...`)**:
   - **Pass 1 (Actual-Only Decode & Exclusive Persistence)**: Reads `actual_print` and `actual_kore` for all 5 cases, parses `actual_print`, validates `states = adapter.complete_states(tape)`, derives `observer_cuts = list(range(len(states)))` solely from actual state length (`allObservedStateOrdinals` policy in `ROOT-ORDINAL-POLICY.json`), and writes each `actual_shared_wire` via `open("xb")`. If any of the 5 actual outputs fails to parse, fails continuity validation, or fails exclusive file creation, `compare_manifest` returns exit code `1` with `"all_actual_decoded_before_expected_read": False` and `"expected_documents_opened": 0` — **before opening any golden file**.
   - **Pass 2 (Pinned Golden Verification & Whole-Wire Equality)**: Reads each `expected_wire` path, verifies both `expected_wire_sha256` and `expected_wire_bytes` against `COMPARISON-MANIFEST.json` / `SAFE-BINDINGS.json`, re-verifies that `hashlib.sha256(encoded(wire)).hexdigest() == wire_pin["sha256"]`, and runs `first_difference(wire, expected)` across the entire wire object (enforcing strict `type(actual) is type(expected)`, exact dict keys, exact list lengths/order, and exact values). Returns `0` iff all 5 cases match completely.

---

## 3. Audit of the Exact 17 Prospective Operations (`PROPOSED-OPERATIONS.json` `f81b7d10...`)

1. **Exact 17-Child Structure & Expansion**:
   - `EXACT17-EXPANSION-PROOF.json` (`55f71364...`) confirms that `review/EXACT-OPERATIONS-LOSSLESS-PROJECTION.json` (`dd7638e6...`) expands losslessly to all 17 phase objects of `PROPOSED-OPERATIONS.json` (`53,084B`, `sha256=f81b7d10...`, canonical expanded list `sha256=b0693df3...`):
     - **1 Model-Only Compile (`00-compile`)**: Compiles `@ROOT@/source/normal-trace-observations.k` (`28,332B` 3-file closure, no expected constant module) with `kompile --backend haskell --no-haskell-binary --main-module SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS --syntax-module SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS` into `@ROOT@/actual/nominal-kompiled`.
     - **15 Case Operations (5 ordered `kast` / `reduce` / `print` triples)** for `01-let-if-call-true`, `02-constructor-let-if-true`, `03-let-if-call-false`, `04-constructor-let-if-false`, and `05-constructor-let-if-nominal-call`:
       - `kast`: parses `@ROOT@/inputs/@CASE@.term` as sort `NGTape` (`--input program --output kore`).
       - `reduce`: runs `.kore-exec-wrapped` with `--depth 1 --smt none --log-format standard --disable-log-timestamps` (activating the first `[Begin, Simplify, Rewrite, Simplify]` strategy bundle proven necessary in the v2 `--depth 1` tail).
       - `print`: runs `kprint @ROOT@/actual/nominal-kompiled @ROOT@/actual/@CASE@-reduce/result.kore false false`.
     - **1 Offline Complete-Wire Comparison (`06-offline-complete-wires`)**: Runs `/usr/bin/python3 -B @ROOT@/comparison/complete_wire_comparison.py @ROOT@/comparison/COMPARISON-MANIFEST.json`.
2. **Containment, Resource Bounds, and Static Tool Pins**:
   - Every child uses the unchanged F03 `launch(record, command, env, cwd, token, 536870912)` closure (`EXISTING-LAUNCH-CALLABLE-CLOSURE.py.txt`, `996aadee...`) with `wall_seconds: 30`, `sweep_seconds: 3`, `rlimit_core: 0`, `rlimit_fsize: 536870912` (512 MiB), `umask: 0o077`, isolated per-child `HOME`/`TMPDIR`/`cwd`, `inherit_environment: false`, unique 32-hex `SHERLOCK_K_ATTEMPT_TOKEN`, and JVM flags `-Xms64m -Xmx512m -Xss32m -XX:ActiveProcessorCount=2`.
   - Static executable pins (`SELECTED-EXECUTABLE-STATIC-PINS.json`, `d9736911...`) confirm `java` (`2a065ca5...`), `kprint` (`5d7094f9...`), `.kore-exec-wrapped` (`4ebe3cd5...`), and `/usr/bin/python3.14` (`52e0a13e...`).
   - Note for Root staging: As in prior runs using `open_exclusive(cwd + '/stdout')` and `persist(..., open("xb"))`, each child's `@ROOT@/actual/<stage>` directory (plus `home` and `tmp` subdirectories for JVM/tool stages) must exist and contain no pre-existing `stdout`, `stderr`, or `*.actual-wire.json` files prior to launch.

---

## 4. Opaque Expected Binding Limitation & Unsupported / `OPEN` Scope

### 4.1 Opaque Expected Golden Limitation (No Oracle Derivation Certification Here)
- The 5 expected K wire documents (`let-if-call-true.K.json` `a56cce13...`, `constructor-let-if-true.K.json` `910d7f08...`, `let-if-call-false.K.json` `1adb1c0e...`, `constructor-let-if-false.K.json` `73d6ca3a...`, and `constructor-let-if-nominal-call.K.json` `28f77f4b...`) and `SAFE-COMBINED-CALL-SUPPLEMENT.json` (`5aa131de...`) are **opaque to the source author and not inline in this packet**.
- **Consequence**: While their paths, byte sizes, SHA-256 digests, schema (`SAFE-WIRE-SCHEMA.json`), and `allObservedStateOrdinals` policy (`ROOT-ORDINAL-POLICY.json`) are fully bound and verified for fail-closed comparison in `complete_wire_comparison.py`, **this source/operation review does not certify the internal semantic derivation or populated state cells of those 5 opaque golden files**. Root and the independent oracle owner (`compiler_knowledge_delta`) bind and audit the populated golden bodies separately.

### 4.2 Explicit Unsupported / `OPEN` Scope
1. **Unrun Execution**: All 17 operations in `generic-nominal-let-if-five-case-actual-qa-20261018` are currently **`UNRUN`**. Source and operation approval authorizes Root to execute the 17-child plan; it is not an observed execution or acceptance result.
2. **Input-Supplied Identities vs. Canonical Elaboration (`OPEN`)**: `TypeId`, `CtorId`, `ExprId`, `occurrence`, and variable/function integer IDs are explicit model inputs; canonical `TypeDecl`/`CtorLayout`/`MatchPlan`/`workMap` elaboration from primary Rust tables remains **`OPEN`**.
3. **Single-Type Fixture & 1-Parameter Non-Recursive Function in This 5-Case Batch (`OPEN`)**: These 5 cases test `Let`/`If`/call/constructor-field composition over `Schedule` (`type_id=20`) and a 1-parameter function (`func 30`). Mutually recursive multi-type declarations, multi-argument ($N>1$) call vectors with `Let`/`If`, and parameterized declarations remain **`OPEN`**.
4. **Positive Admitted Domain vs. Negative / Admission / Binder-Linearity Checks (`OPEN`)**: The K model is a positive admitted-domain reference (`OutsideModel` is reference tooling only, not product error code policy). Duplicate binder rejection, static type/admission checking, and guard/capacity/release policies remain **`OPEN`**.
5. **Observation Ordinals vs. Mid-Prefix Cut Resumption & A39 / B Clock (`OPEN`)**: `observerCuts = list(range(len(actual_states)))` records observation ordinals over full runs to `ngHalted` (`budget=128`); separate mid-prefix cut/export/import resumption execution, A39/B helper-clock correspondence, Quint/Clean cross-profile execution, native proofs, PCD, Preview settlement, and full `UIREQ-034`/`SCON-016` EARS acceptance remain **`OPEN`**.

---

## 5. Scoped Verdicts & Mandatory Fixes

- **Ordinary Complete Source Correctness Verdict (`SOURCE-FREEZE.json` `bec50e0b...` — 3-file K model/syntax/observer `28,332B`, 5 plain-Map `.term`/`.initial.ng` inputs, and `comparison/{nominal_trace_codec,complete_k_wire,complete_wire_comparison}.py`)**: **`APPROVE`** (Qualified by Section 4.1 opaque expected golden body non-inspection and Section 4.2 `OPEN` scope boundaries).
- **Exact 17-Operation Plan Eligibility Verdict (`PROPOSED-OPERATIONS.json` `f81b7d10...` / canonical 17-child expansion `b0693df3...`)**: **`APPROVE`** (Eligible for Root selection and serial fail-stop execution under the unchanged per-child 30s/3s/512MiB/process-group+token caps).
- **Mandatory Fixes Required Before Root Execution**: **`NONE`**.

