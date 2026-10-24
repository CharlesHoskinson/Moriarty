### Reviewer Identity & Execution Attestation

- **Reviewer**: Gemini 4 Argon (High), fresh independent non-author ordinary semantic/model-source reviewer (acting during observed Opus outage).
- **Author Independence**: Did not author the candidate v2 K semantics, observer, converter, or independent expected fixtures/tapes. No prior v1 pass is transferred.
- **Execution Attestation**: No tools were invoked, no commands or tests were executed, no external models were called, and no files were modified for this request.

---

### 1. Exact Readable Coverage

All twelve inline bodies provided in the review packet were read in full:

1. **`source/nominal-product-syntax.k`** (`bytes=4476`, `sha256=e80251e7aee1b7bf68bd86993f7d250cf06e7bbc4398b0f631b367d923310c20`): Complete module `SHERLOCK-NOMINAL-PRODUCT-SYNTAX` declaring sorts and constructors for `NGValue`, `NGValues`, `NGTypes`, `NGExprIds`, `NGCtor`, `NGCtors`, `NGDecls`, `NGPattern`, `NGPatterns`, `NGArms`, `NGExpr`, `NGNodes`, `NGParams`, `NGFunction`, `NGFunctions`, `NGProgram`, `NGPath`, `NGBindings`, `NGMatching`, `NGEnvironment`, `NGExprLookup`, `NGFunctionLookup`, `NGCtorLookup`, 7 `NGFrame` variants, `NGFrames`, 7 `NGControl` variants, 19 `NGEvent` variants, `NGEvents`, and `NGState`.
2. **`source/nominal-product.k`** (`bytes=19167`, `sha256=0d7d4ec32206a478d868194860f717f941bc45997b99c866af1df2b679b43131`): Complete module `SHERLOCK-NOMINAL-PRODUCT` implementing lookup helpers, parameter binding, pattern matching, environment extension, `ngInitial`, `ngStep`, `ngAdvance`, `ngOutside`, `ngTerminal`, `ngEvaluate`, `ngStartConstructor`, `ngFinishAdd`, `ngEnterFunction`, `ngEnterBindings`, `ngSelectArm`, and all frame-return transitions.
3. **`source/normal-trace-observations.k`** (`bytes=1386`, `sha256=f4eaa0b5ad2969f2d0ab1324fc0894aba23cfe5140d23a7a81a58d54a3219f5f`): Complete module `SHERLOCK-NOMINAL-PRODUCT-OBSERVATIONS` defining `NGStates`, `NGTape`, `ngAppendState`, `ngObserve`, `ngObserveLoop`, and `ngObserveNext`.
4. **`source/independent-expected-tapes.k`** (`bytes=62832`, `sha256=375cb881953d54e84cf4f70df94d6428966bf1e918f85beeb995b96fbdd778fb`): Complete module `SHERLOCK-NOMINAL-PRODUCT-GOLDENS` defining prefix helper `xp(NGEvents, Int)`, 142 zero-argument constant rules `x0()..x141()`, 7 full event lists `xe0()..xe6()`, 7 input programs `inP0()..inP6()`, 7 input environments `inE0()..inE6()`, and 7 complete expected tapes `xt0()..xt6()`.
5. **`context/ORACLE-EXPANSION-CONTRACT.md`** (`bytes=2907`, `sha256=04f3be3ee8f5d30d3c8ba9b8715ef4fe0fa6d64010d55ca29aca47ebce6c205a`): Lossless expansion contract from the unchanged 16-member independent JSON freeze (`d4c9aee74c507299413c4009e7a4e8545f7c076b166876a8f32bde3cd47d7826`).
6. **`context/ORACLE-ROUNDTRIP-PROOF.compact.json`** (`bytes=3303`, `sha256=b00715e071e0d6d275f92e1da59096bb6c5930c801d5a69580d59eff1a12444f`): Data-only roundtrip verification record across all 7 cases (`construct-asymmetric`, `construct-repeated`, `ordered-first-arm`, `recursive-constructor-fields`, `structural-sum-asymmetric`, `structural-sum-repeated`, `zero-field-constructor`).
7. **`context/ORACLE-REVIEW-INTERFACE.compact.json`** (`bytes=2370`, `sha256=7c67187612362bd7377af8b172a74733e4d9cbaf522be03594742e5bc16a0179`): Case step counts, observation bounds (`steps + 10`), raw tape SHA-256 digests, and exact `actual`/`expected`/`equal` query strings.
8. **`context/NORMATIVE-REQUIREMENTS.txt`** (`bytes=9036`, `sha256=375bc39b01bb44772d2f81fcc5c706124f2daa024cee513933b6b995d88695d4`): Normative text for `UIREQ-034`, `SCON-016` (including `SCON-016.R02`), `universal-interpreter/interfaces.md`, and `sherlock-machine-circuit-layout/extensions-and-obligations.md`.
9. **`review/PUBLIC-API-CANONICAL.json`** (`bytes=5303`, `sha256=77743ca9ba6ea3cfd03f8f530fa2fa9c07ec0c1c893b2b6d28caa7b7195eb71e`): Canonical declaration-only positional payload orders and conventions for `NGState`, entrypoints, 7 `NGFrame` constructors, 19 `NGEvent` constructors, and 7 `NGControl` constructors.
10. **`context/EXISTING-LAUNCH-CALLABLE-CLOSURE.py.txt`** (`bytes=4228`, `sha256=996aadee88cf0ac4688925789676dcc9e407cbd3d7c15b912f25204b10861242`): Complete source of `utc`, `open_exclusive`, `processes`, `kill_group`, `sweep`, and `launch`, plus imports and required globals (`CHILD_SECONDS = 30`, `SWEEP_SECONDS = 3`, `TOKEN_NAME = 'SHERLOCK_K_ATTEMPT_TOKEN'`).
11. **`review/IMPORT-TIME-FACTS-LOSSLESS-MEMBERSHIP.json`** (`bytes=8252`, `sha256=54bde1f2fea5cafeeeefc189438e41d0f52d3e208467f8e6868b0a17465cf028`): Complete top-level AST membership of `k_engine_attempt_f03_01.py` (`sha256=f1bcf0a0d21b4a7aaedd2f2aae1a1e9e5b62219271eb41b3f71ed97d5a0bc24d`), all imports, all 13 call-bearing top-level assignments inline, function definition line spans, and `if __name__ == '__main__':` guard.
12. **`review/EXACT-OPERATIONS-LOSSLESS-PROJECTION.json`** (`bytes=9922`, `sha256=8082dab19d4db2c03661d96839189dd7be323f9520ff211a2a1bdf285861bff2`): Lossless template projection expanding to all 22 prospective child operations (`00-compile` plus 7 ordered `<case>-kast`, `<case>-reduce`, `<case>-print` triples) with exact `argv`, `environment`, `cwd`, `SHERLOCK_K_ATTEMPT_TOKEN`, and resource limits.

---

### 2. SOURCE Verdict & Material Findings

- **SOURCE VERDICT**: **PASS (SCOPED TO ADMITTED POSITIVE MONOMORPHIC NOMINAL REFERENCE EVALUATOR AND COMPLETE-STATE TEST OBSERVER)**

#### Material Source Findings (Addressing Required Source Questions)

1. **Complete-state observer (`ngObserve`) fidelity, chronological ordering, and stutter-free termination (`source/normal-trace-observations.k`, `source/nominal-product.k`)**:
   - `ngObserve(S, N)` initializes `ngObserveLoop(S, S, ngNoStates(), N)`.
   - Whenever `N >Int 0 andBool notBool ngTerminal(C)`, `ngObserveLoop` calls actual `ngStep(ngState(P, C, Rho, Fs, Es))` on the entire 5-component pending `NGState` without projection or shortcut.
   - `ngObserveNext` appends each returned `Next` state to the tail of `States` via `ngAppendState(States, Next)`, preserving exact chronological microstep order `S_1, ..., S_k`.
   - When `Next` transitions to a terminal control (`ngHalted(V)` or `ngOutsideModel(S)`), `Next` is appended once to `States`; on the immediately following `ngObserveLoop` call, `ngTerminal(C)` holds, so `ngObserveLoop` returns `ngTape(Initial, States, ngState(P, C, Rho, Fs, Es))` immediately even when remaining budget `N >Int 0` (such as the `+10` margin in bounds `29, 29, 22, 71, 78, 78, 13`). No extra `ngStep` is taken and no terminal stutter state is appended.
   - Each equality query `ngObserve(ngInitial(inP_i(), inE_i()), bound_i) ==K xt_i()` compares the initial state, every intermediate state after each event (`States`), and the final state against `xt0()..xt6()`.

2. **Strict separation between expected data and actual semantics (`source/nominal-product.k`, `source/normal-trace-observations.k`, `source/independent-expected-tapes.k`)**:
   - Neither `source/nominal-product-syntax.k`, `source/nominal-product.k`, nor `source/normal-trace-observations.k` imports `SHERLOCK-NOMINAL-PRODUCT-GOLDENS` or contains any fixture constants, case branches, or expected shortcuts.
   - `source/independent-expected-tapes.k` defines only `xp(NGEvents, Int)` (pure list-prefix extraction over `NGEvents`), acyclic 0-ary constant constructors `x0()..x141()`, event lists `xe0()..xe6()`, inputs `inP0()..inP6()` / `inE0()..inE6()`, and expected tapes `xt0()..xt6()`. It adds zero rewrite rules for `ngStep`, `ngObserve`, `ngAdvance`, or any evaluator helper.
   - `inP0()..inP6()` and `inE0()..inE6()` supply only the source `NGProgram` and initial `Map` (`612 |-> ngUInt(77)` plus case inputs `610`, `611`) to `ngInitial`; `xt0()..xt6()` appear exclusively on the RHS of `==K`.

3. **Normal enclosing evaluator correctness across the 7 required semantic dimensions (`source/nominal-product.k`, `source/independent-expected-tapes.k`)**:
   - **Selected declaration domain**: `x4()` declares nominal type `20` (`Schedule`) in recursive group `9` with constructors `21` (`Done`, order `0`, types `[]`), `22` (`Pay`, order `1`, types `[0 (UInt128), 20 (Schedule)]`), and `23` (`Both`, order `2`, types `[20, 20]`). `ngFinishAdd` enforces `UInt128` bounds (`0 <= L, R` and `L +Int R < 2^128 = 340282366920938463463374607431768211456`).
   - **Source occurrence identity**: Every `NGExpr` carries its own source occurrence integer `O` as its first positional field, distinct from expression table node ID `E` in `ngNode(E, Expr, Rest)`. In `xt1` (`construct-repeated`) and `xt5` (`structural-sum-repeated`), equal-looking `Pay(4, Done)` subtrees at nodes `2101..2103` (occurrences `21010..21030`) and `2104..2106` (occurrences `21040..21060`) remain distinct throughout all frames and events while evaluating to the same extensional value `x1()`.
   - **Full `ngConstructorFields` continuation and caller tails (`SCON-016.R02`)**:
     - Zero-field constructors (`ngCtor(T, C, _, ngNoTypes())` with `ngNoExprs()`, tested in `xt6` at `2600` and at leaves `2003, 2006, 2103, 2106, 1210`) transition from `ngEval(E)` directly to `ngMaterialize(E, O, T, C, ngNoValues())` and then `ngReturn(ngNominal(T, C, ngNoValues()))` in 2 microsteps (`3` total with `ngFinished` in `xt6`), without pushing `ngConstructorFields` or inventing field-evaluation steps.
     - Non-empty constructors push `ngConstructorFields(E, O, T, C, Ts, 0, X, ngNoValues(), Rest, Rho)` onto the caller's frame stack `Fs`. Fields evaluate strictly left-to-right.
     - When a field expression `Active` performs a nested recursive call (such as first field `2501 = copy(610)` or last field `2503 = copy(611)` in `xt3`, or recursive second field `1222 = copy(602)` inside `1220`), `ngConstructorFields` preserves the constructor's `E, O, T, C, Ts`, field cursor `I`, original caller field expression ID `Active`, already evaluated `Prefix`, `Rest`, `Saved` environment, and underlying caller frame stack `Fs`.
     - Upon field return, `ngConstructorFieldReturned` computes `completed_expr_occurrence` via `ngOccurrence(ngFindExpr(Active, ngNodes(P)))`, preserving the caller field expression's occurrence (`25010`, `25030`, `12220`) rather than the callee's last leaf expression (`12100`). Construction occurs only after all fields return, via the explicit `ngMaterialize(E, O, T, C, Vs)` control state.
   - **Named recursive call environment restoration**: `ngBindParameters` constructs callee environments starting from `.Map` (`ngBoundEnvironment(.Map)`), binding only formal parameters after checking arity, parameter uniqueness, and `ngTypeOf(V) ==Int T`. `ngEnterBindings` saves the caller's environment `Rho` in `ngFunctionReturn(E, O, F, T, Rho)`. On return, `ngStep` verifies `ngTypeOf(V) ==Int T` and restores `Saved`, preventing any callee bindings (or ambient caller variable `612 |-> ngUInt(77)`) from leaking across call boundaries.
   - **Declaration membership and dynamic type checks**: `ngStartConstructor` checks `ngFindCtor(T, C, ngDeclarations(P))` and `ngTypeCount(Ts) ==Int ngExprCount(Xs)`; field returns check `ngTypeOf(V) ==Int ngTypeAt(Ts, I)`; function entries and returns check parameter and return type IDs.
   - **Ordered first-arm selection**: `ngChoose` tests arms in source order one arm per microstep, emitting `ngArmMiss` on `ngMiss()` and immediately selecting the first matching arm on `ngMatched(Bs)` (`ngArmSelected`), pushing `ngArmScope(E, O, A, Saved)` and restoring `Saved` on `ngArmBodyReturned`. In `xt2` (`ordered-first-arm`), both arm `7100` (`Both(Pay(605, _), _)`) and arm `7101` (`Both(_, 604)`) match `x5()`, and arm `7100` is selected on the first `ngChoose` step, returning `ngUInt(4)` in 12 steps.
   - **Recursive sums and copy**: Verified across `xt3` (`copy` across both fields of `Both`, 61 steps -> `x5()`), `xt4` (`sum` on asymmetric `Both(Pay(4, Done), Pay(5, Done))`, 68 steps -> `ngUInt(9)`), and `xt5` (`sum` on repeated `Both(Pay(4, Done), Pay(4, Done))`, 68 steps -> `ngUInt(8)`).

4. **Exact positional payload alignment for all 19 events, 7 frames, and 7 controls (`source/nominal-product-syntax.k`, `source/nominal-product.k`, `review/PUBLIC-API-CANONICAL.json`, `source/independent-expected-tapes.k`)**:
   - All 7 `NGFrame` constructors (`ngAddLeft`, `ngAddRight`, `ngArguments`, `ngFunctionReturn`, `ngConstructorFields`, `ngScrutinee`, `ngArmScope`), all 7 `NGControl` constructors (`ngEval`, `ngReturn`, `ngApply`, `ngChoose`, `ngMaterialize`, `ngHalted`, `ngOutsideModel`), and all 19 `NGEvent` constructors (`ngLiteralEvaluated`, `ngVariableRead`, `ngAddStarted`, `ngAddRightRequested`, `ngAdded`, `ngConstructorStarted`, `ngConstructorFieldReturned`, `ngConstructorBuilt`, `ngCallStarted`, `ngArgumentReturned`, `ngFunctionEntered`, `ngFunctionReturned`, `ngMatchStarted`, `ngScrutineeReturned`, `ngArmMiss`, `ngArmSelected`, `ngArmBodyReturned`, `ngFinished`, `ngOutsideEvent`) match `PUBLIC-API-CANONICAL.json` and `independent-expected-tapes.k` position-for-position with zero omissions.
   - Specifically, `remaining_expr_ids_after` in `ngConstructorFieldReturned` and `ngArgumentReturned` follows the canonical `return_event_remainder_convention` (the tail-only remainder after selecting the next active expression `X`, or `ngNoExprs()` on the last field/argument).

5. **Converter factoring and roundtrip coverage limits (`context/ORACLE-EXPANSION-CONTRACT.md`, `context/ORACLE-ROUNDTRIP-PROOF.compact.json`)**:
   - The 142 constant definitions `x0()..x141()` form a finite, acyclic substitution DAG, and every `xp(xe_i(), k)` call in `xt0()..xt6()` uses a valid prefix index `0 <= k <= len(xe_i())`.
   - **Explicit limitation**: `ORACLE-ROUNDTRIP-PROOF.compact.json` (`DATA_ONLY_ROUNDTRIP_NOT_K_EXECUTION_OR_ACCEPTANCE`) verifies lossless AST/JSON equivalence using only the Python data serializer/parser. It does not establish K frontend parsing, typechecking, `kompile`, `kore-exec` reduction, or `kprint` execution.

---

### 3. OPERATION Eligibility & Concrete Limitations (Separate from Source Verdict)

- **OPERATION VERDICT**: **ELIGIBLE FOR ROOT SELECTION SUBJECT TO EXPLICIT CALLER PRECONDITIONS AND RESOURCE LIMITATIONS**

#### Classification of the 22 Prospective Child Operations (`review/EXACT-OPERATIONS-LOSSLESS-PROJECTION.json`)

- **Phase 1 (`00-compile`)**: 1 JVM `kompile` child (`--backend haskell --no-haskell-binary --main-module SHERLOCK-NOMINAL-PRODUCT-GOLDENS --syntax-module SHERLOCK-NOMINAL-PRODUCT-GOLDENS`) compiling `@ROOT@/source/independent-expected-tapes.k` (which transitively requires `normal-trace-observations.k`, `nominal-product.k`, `nominal-product-syntax.k`, and `domains.md`) into `@ROOT@/actual/nominal-kompiled`.
- **Phases 2..22 (7 ordered cases × 3 stages `<case>-kast`, `<case>-reduce`, `<case>-print`)**:
  - `01-construct-asymmetric`, `02-construct-repeated`, `03-structural-sum-asymmetric`, `04-ordered-first-arm`, `05-structural-sum-repeated`, `06-recursive-constructor-fields`, `07-zero-field-constructor`.
  - Each `<case>-kast` parses `@ROOT@/inputs/<case>.term` as sort `Bool` into `@ROOT@/actual/<case>-kast/stdout` (Kore format).
  - Each `<case>-reduce` runs `.kore-exec-wrapped` with `--depth 0 --smt none` (pure equational normalization of the `Bool` equality query) into `@ROOT@/actual/<case>-reduce/result.kore`.
  - Each `<case>-print` runs `kprint` on `result.kore` into `@ROOT@/actual/<case>-print/stdout` for comparison against `@ROOT@/expected/<case>.expected` (`true\n`).
- Every child has a unique 32-hex `SHERLOCK_K_ATTEMPT_TOKEN`, `inherit_environment: false`, `stdin: DEVNULL`, and an isolated per-stage working directory `@ROOT@/actual/<stage>` with stage-local `HOME` and `TMPDIR`.

#### Assessment of Root Callback Binding `module.launch(record, phase["argv"], phase["environment"], phase["cwd"], phase["environment"]["SHERLOCK_K_ATTEMPT_TOKEN"], 536870912)`

1. **Import-time safety (`review/IMPORT-TIME-FACTS-LOSSLESS-MEMBERSHIP.json`)**:
   - Importing `k_engine_attempt_f03_01.py` as a non-`__main__` module executes only stdlib imports, literal/container bindings, and 13 pure in-memory expressions (`dict`, `':'.join`, `re.compile`, `tuple`, `range`, `list`).
   - `if __name__ == '__main__': sys.exit(main())` is false under module import; none of `main`, `execute`, `run_stages`, `child_stage`, `initializer_stage`, `preflight`, `postflight`, `persist`, or `append_ledger` are invoked.
2. **Per-child lifecycle & cleanup (`context/EXISTING-LAUNCH-CALLABLE-CLOSURE.py.txt`)**:
   - **30s wall clock**: Enforced via `CHILD_SECONDS = 30` and `proc.wait(timeout=max(0.0, started + CHILD_SECONDS - time.monotonic()))`, followed by `kill_group(proc.pid)` (`SIGKILL`) and `proc.wait()` on timeout.
   - **Process-group cleanup**: `start_new_session=True` places each child in its own session and process group (`pgid == proc.pid`). The `finally` block sends `SIGKILL` to `proc.pid`'s process group both if `proc.returncode is None` and unconditionally after exit.
   - **3s token sweep**: `sweep(proc.pid, token)` polls `/proc` for up to `SWEEP_SECONDS = 3` (`0.1s` interval), sending `SIGKILL` to any non-zombie process matching `pgid == proc.pid` or carrying `SHERLOCK_K_ATTEMPT_TOKEN=<token>` in `/proc/<pid>/environ`, and sets `record['containment_breach']` if any process survives or escaped the process group.
   - **512 MiB `RLIMIT_FSIZE` per file, `RLIMIT_CORE = 0`, `umask = 0o077`**: Enforced in `preexec_fn=limit_child` via `resource.setrlimit(resource.RLIMIT_CORE, (0, 0))`, `resource.setrlimit(resource.RLIMIT_FSIZE, (536870912, 536870912))`, and `os.umask(0o077)`. `stdout` and `stderr` are opened via `open_exclusive` (`O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW`, mode `0o600`) and closed in `finally`.
   - **JVM heap/stack/processor flags**: Passed explicitly in `argv` for `00-compile` and all 7 `<case>-kast` phases (`-Xms64m -Xmx512m -Xss32m -XX:ActiveProcessorCount=2 -XX:+ExitOnOutOfMemoryError -XX:-UsePerfData -XX:+DisableAttachMechanism`).

#### Concrete Operational Limitations & Narrowly-Owned Preconditions

1. **Lack of aggregate CPU / RSS / disk hard caps**:
   - `limit_child()` sets only per-file `RLIMIT_FSIZE` (512 MiB) and `RLIMIT_CORE` (0). There are no OS/cgroup hard caps on RSS/virtual memory (`RLIMIT_AS`/`RLIMIT_RSS`), CPU cores/time (`RLIMIT_CPU`), process count (`RLIMIT_NPROC`), or aggregate multi-file disk consumption across the 22 directories. Non-JVM native children (`.kore-exec-wrapped`, `kprint`, and tools invoked on `PATH` by `kompile`) run with default native memory/CPU limits bounded only by the 30s per-child wall timeout (worst-case child+sweep envelope `22 * 33s = 726s` excluding Root overhead).
2. **Pre-existing directory and clean `stdout`/`stderr` precondition for raw `launch()`**:
   - In `k_engine_attempt_f03_01.py`, directory creation (`os.makedirs`) was previously handled outside `launch()` (in `child_stage` / setup), whereas `launch()` immediately calls `open_exclusive(cwd + '/stdout')` and `open_exclusive(cwd + '/stderr')`. Before invoking `module.launch(...)` for each phase, the caller (Root) must ensure `@ROOT@/actual/<stage>`, `@ROOT@/actual/<stage>/home`, and `@ROOT@/actual/<stage>/tmp` exist and do not contain pre-existing `stdout` or `stderr` files.
3. **On-disk input/expected file materialization & post-call fail-stop checks**:
   - Because `launch()` only populates `record` (`outcome`, `returncode`, `timed_out`, `containment_breach`) and does not itself check `returncode == 0`, verify `result.kore` non-emptiness, or compare `<case>-print/stdout` against `@ROOT@/expected/<case>.expected`, Root's caller loop (or existing helper `stage_problems` at lines `1146..1171` of `k_engine_attempt_f03_01.py`) must check those conditions after each phase to enforce serial fail-stop and verify `@ROOT@/inputs/<case>.term` and `@ROOT@/expected/<case>.expected` before/after execution.

---

### 4. Scope or Unreadable-Input Gaps

- **No unreadable inline inputs**: All 12 inline blocks are intact and completely readable.
- **Factored external artifacts (not inline raw bytes, no disagreement with factoring contract)**:
  - The raw 3.7 MB unfactored K tapes and 16-member original JSON freeze (`d4c9aee74c507299413c4009e7a4e8545f7c076b166876a8f32bde3cd47d7826`) are represented via the losslessly factored `source/independent-expected-tapes.k` and `ORACLE-ROUNDTRIP-PROOF.compact.json`.
  - Unused top-level literal metadata assignments in `k_engine_attempt_f03_01.py` are factored into externally pinned `context/IMPORT-TIME-AST.json` (`sha256=11a38a47e51d205aaa76467eab7f5f8f0768f40df0d5ddbaac30440982292a6e`); all imports, call-bearing top-level assignments, and the complete callable closure of `launch` are verified inline.
  - The physical files `@ROOT@/inputs/<case>.term` and `@ROOT@/expected/<case>.expected` referenced by `<case>-kast` and `<case>-print` are specified via `ORACLE-REVIEW-INTERFACE.compact.json` (`ngObserve(ngInitial(inP0(),inE0()),29) ==K xt0()`, etc.) and `EXACT-OPERATIONS-LOSSLESS-PROJECTION.json` (`exact literal true LF`), rather than separate inline file blocks.

---

### 5. Claims That Remain OPEN

1. **Actual K compilation and reduction results**: All 22 prospective operations (`00-compile` and the 7 `kast`/`reduce`/`print` triples) are unrun in this review; actual K parser, typechecker, compiler, Haskell backend execution, and `true\n` output confirmation remain open until executed.
2. **Canonical declaration elaboration, `CtorLayout`, `MatchPlan`, and `workMap` compilation (`UIREQ-034`, `SCON-016`, `interfaces.md`)**: Nominal/constructor/occurrence IDs in this model are pre-assigned integer inputs, not allocator or elaborator outputs. `nominal-product.k` interprets source arms directly rather than compiling pattern matrices into `MatchPlan` nodes (`Switch`, `Bind`, `Select`, `NoMatch`), computing `CtorLayout` widths/tags/slotKinds, or tracking a `workMap` cursor.
3. **Mutually recursive multi-type declaration groups and bounded parameterized specialization**: While `NGDecls` syntax supports multiple declarations with a `recursive_group` ID, the 7 fixtures exercise only a single self-recursive nominal type (`Schedule`, type `20`, group `9`). Mutually recursive multi-type groups and bounded parameterized specialization remain open.
4. **Negative admission, binder-linearity enforcement, and mid-prefix cut resumption (`SCON-016`, `SCON-016.R02`)**: All 7 cases are positive normal executions (`ngHalted`) on closed, binder-linear inputs. None of the 11 `ngOutsideModel` rejection paths, non-linear binder handling (avoiding fabricated `error8` on Cons/Branch vs generic admission), `NoMatch -> Rejected(error9)` mapping, or mid-prefix technical cut (`N < steps`) export/import resumption scenarios are exercised by the 7 positive queries.
5. **A39 / B helper clock, fuel/work accounting, circuit layout, native PCD proofs, financial semantics, and Preview/stagenet acceptance**: `ngStep`/`ngAdvance`/`ngObserve` steps are uncharged reference microsteps; all cross-layer correspondence, resource/charge accounting, native proof, and product acceptance gates remain OPEN.

