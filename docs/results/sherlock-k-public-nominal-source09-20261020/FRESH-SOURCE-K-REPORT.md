# Fresh Independent Non-Author Formal Math/Source/API Review — K Lane (`source09`)

## 1. Reviewer Identity, Routing & Execution Attestation
- **Role**: Fresh independent non-author formal math/source/API reviewer (under the standing October 16 Opus quota fallback to Gemini 4 Argon at high effort via AGY).
- **Requested Identity / Effort**: Gemini 4 Argon (High) via AGY (`IdentityrequestedHIGH/backendunattested`: model selection requested at High effort in AGY session; underlying provider backend runtime remains unattested by host cryptographic receipt).
- **Execution Discipline**: Read-only inspection only. Zero model, compiler, `kompile`, `kore-exec`, `rustc`, Cargo, SDK, native, proof, provider, network, or Git commands were executed. No shared workspace files were written.
- **Independence Qualification**: Expected typed return trees in `COMPLETE-SEMANTIC-RETURNS-PREFREEZE.json` were hand-derived from the two `.mori` fixtures by the K lane author prior to authoring the K candidate, without reading Rust nominal test assertions or actual compiler outputs. Same-author pre-code expectations are independent of the Rust producer outputs, not a second independent reviewer vote (`Eachauthornotavote`).

---

## 2. Inspected Candidate & Pin Inventory
- **Handoff**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/k-next-public-nominal-simulate-source09/HANDOFF.json` (`3160` bytes, SHA256 `3f8e68bec46da7a01475265f381a46fc09d7abda772f55bd54e604001d414d25`)
- **Full Review Prompt**: `fresh-source-API-review-preparation01/PROMPT.txt` (`89367` bytes, SHA256 `946ecfb1ddcf82fb4f9dfed1b69e89d20b0ceee02cdefef5621bfd2fcb229f32`) — read in full.
- **Unchanged Base Semantic Modules (Byte-Exact Preimages)**:
  1. `source/nominal-product-syntax.k` (`5090` bytes, SHA256 `750f9b820a672865b2a1abc50bef4f417f2c6c02e7b77f661a0431388dacab8c`)
  2. `source/nominal-product.k` (`21856` bytes, SHA256 `29da74d7ab2b89cb23155521b94ffb6880920befcbf9806941ba97826d47b197`)
- **New Additive Candidate Modules & Laws (UNRUN)**:
  3. `source/public-nominal-source09.k` (`3530` bytes, SHA256 `6e961f2c8efc205fc83e16d0e6448753266ffd6c6ebf18b960e99d080cca08c1`)
  4. `source/public-nominal-source09-inputs.k` (`4767` bytes, SHA256 `8ffeb118238b75f8ae30cbda08c88bd3226c7e49bff4db67189822a1a388f8a2`)
  5. `tests/public-nominal-source09-laws.k` (`2700` bytes, SHA256 `d4122d9f942fe3c86bfe9bff3ee580ae39e9a51001c52dae83a9685cda6f1f89`)
- **Allowed Source Fixtures**:
  - `map-queue.mori` (`638` bytes, SHA256 `b8b30eb7b66280065092d4e8590c8c798b640e64b3c748153a997107f8c8010d`)
  - `mirror-arbor.mori` (`735` bytes, SHA256 `504fd3f85c354b778f4f5c4d49cd75155c21a62d087c24530f9a2fa51e93bfa9`)

---

## 3. Independent Math, Source & API Verification

### 3.1 Direct Real Boolean `Not` Extension (`SHERLOCK-PUBLIC-NOMINAL-SOURCE09`)
- **Syntax & Frame/Event Carriers**:
  - Adds `syntax NGExpr ::= ngNot(Int, Int)` (occurrence `O`, operand expression ID `X`), `syntax NGFrame ::= ngNotArgument(Int, Int, Map)` (expression ID `E`, occurrence `O`, saved environment `Saved`), and `syntax NGEvent ::= ngNotStarted(Int, Int, Int, Map) | ngNotReturned(Int, Int, NGValue, NGValue, Map)`.
- **Occurrence Lookup Completeness**:
  - Adds `rule ngOccurrence(ngFoundExpr(ngNot(O,_))) => O`.
  - **Critical API Interaction**: In `nominal-product.k` (lines 314–315 and 322–323), when a constructor field expression `Active` returns a value `V` to `ngConstructorFields`, the transition emits `ngConstructorFieldReturned(..., ngOccurrence(ngFindExpr(Active, ngNodes(P))), ...)`. In `npMirrorArborProgram()`, constructor node `3` (`Tip(not(value))`) has field expression `4 = ngNot(2004, 5)`. Because `ngOccurrence` is defined on `ngNot(O, _)`, returning from `ngNot` into `ngConstructorFields` resolves `ngOccurrence` to `2004` without getting stuck.
- **Evaluation & Return Rules**:
  - `ngEvaluate(ngFoundExpr(ngNot(O,X)),E,P,Rho,Fs,Es)` transitions to `ngState(P, ngEval(X), Rho, ngFrame(ngNotArgument(E,O,Rho), Fs), ngAppendEvent(Es, ngNotStarted(E,O,X,Rho)))`. It evaluates `X` once in `Rho`, saves `Rho` in `ngNotArgument`, preserves `P` and `Fs`, and appends `ngNotStarted`.
  - On `ngReturn(ngBool(B))` with `ngFrame(ngNotArgument(E,O,Saved), Fs)` on top, `ngStep` transitions to `ngState(P, ngReturn(ngBool(notBool B)), Saved, Fs, ngAppendEvent(Es, ngNotReturned(E,O,ngBool(B),ngBool(notBool B),Saved)))`. The saved environment `Saved` is restored, replacing any transient environment; `P` and remaining frames `Fs` are preserved.
  - On `ngReturn(V)` with `notBool ngIsBool(V)`, `ngStep` transitions via `ngOutside` to `ngOutsideModel("not_operand_type_outside_model")`, matching the base model's partiality convention without claiming a Core compiler/runtime refusal policy.

### 3.2 Lossless Typed Full-State Observer (`NPObservation` / `NPValue`)
- `NPObservation ::= npObserved(NGState, NPValue)` pairs the **entire final `NGState`** (`NGProgram`, `ngHalted(V)`, final `Map` environment, `NGFrames`, and complete `NGEvents` trace) with the recursively projected `NPValue`.
- `npProjectPositive` recursively resolves concrete local nominal `typeId` and `(typeId, ctorId)` through `NPTypeRows` and `NPCtorRows` into `npNominal(NPType, CtorName, NPFields)`, preserving:
  - Full recursive type arguments (`NPType ::= npUIntType() | npBoolType() | npNominalType(String, NPTypes)`).
  - Exact declaration order of constructor field names (`NPFields`), with no lexicographical sorting (`z_item` before `a_tail`, `z_items` before `a_marker`, `z_left` before `a_right`, `z_tree` before `a_code`).

### 3.3 Independent Derivation & Verification of the Two Source Programs and Four Laws
1. **`npMapQueueProgram()` & `npLawMapQueue()`**:
   - Local declared IDs: `0 = UInt`, `1 = Bool`, `4 = Queue<UInt>`, `5 = Envelope<Queue<UInt>, Bool>`; constructors `(4,0) = Empty()`, `(4,1) = Push(0, 4)`, `(5,2) = Wrap(4, 1)`; functions `10 = bump` (param `31 : 4 -> 4`, body `1`), `20 = pack` (param `31 : 4 -> 5`, body `10`); entry `20`.
   - Input expression `21` constructs `Push(3, Push(8, Empty()))`.
   - `pack` (node `10`) constructs `Wrap(bump(xs), true)` in declaration field order.
   - `bump` (node `1`) matches `xs` (node `2 = ngVariable(1002, 31)`):
     - Arm 0 matches `(4, 0)` (`Empty`) and constructs `Empty()` (node `3`).
     - Arm 1 matches `(4, 1)` (`Push`) binding `32 = item`, `33 = tail`, and constructs `Push(item + 2, bump(tail))` (node `4` with fields `5 = ngAdd(1005, 7, 8)` and `6 = ngCall(1006, 10, ngExprId(9, ngNoExprs()))`).
   - Additions `3 + 2 = 5` and `8 + 2 = 10` satisfy the `ngFinishAdd` normal domain guard (`0 <= L, R` and `L + R < 2^128 = 340282366920938463463374607431768211456`).
   - Halts in ~73 microsteps (well within the `512` observer cut; `ngAdvance` stutters cleanly on `ngHalted`) with projected value:
     `Envelope<Queue<UInt>, Bool>::Wrap(z_items: Queue<UInt>::Push(z_item: 5, a_tail: Queue<UInt>::Push(z_item: 10, a_tail: Queue<UInt>::Empty())), a_marker: true)`.
     Matches `npLawMapQueue()` and `COMPLETE-SEMANTIC-RETURNS-PREFREEZE.json` exactly.

2. **`npMirrorArborProgram()` & `npLawMirrorArbor()`**:
   - Local declared IDs: `4 = Arbor<Bool>`, `5 = Stamp<Arbor<Bool>, UInt>`; constructors `(4,0) = Tip(1)`, `(4,1) = Fork(4, 4)`, `(5,2) = Seal(4, 0)`; functions `10 = reflect` (param `31 : 4 -> 4`, body `1`), `20 = stamp` (param `31 : 4 -> 5`, body `11`); entry `20`.
   - Input expression `21` constructs `Fork(Tip(true), Fork(Tip(false), Tip(true)))`.
   - `stamp` (node `11`) constructs `Seal(reflect(t), 41)`.
   - `reflect` (node `1`) matches `t`:
     - Arm 0 matches `(4, 0)` (`Tip(value=32)`) and constructs `Tip(not(value))` (node `3` -> field `4 = ngNot(2004, 5)` -> `5 = ngVariable(2005, 32)`).
     - Arm 1 matches `(4, 1)` (`Fork(left=33, right=34)`) and constructs `Fork(reflect(right), reflect(left))` (node `6` -> field 0 is `7 = reflect(var 34)`, field 1 is `9 = reflect(var 33)`).
   - Evaluates `reflect(right)` first, producing `Fork(Tip(false), Tip(true))`, then `reflect(left)` producing `Tip(false)`, yielding outer `Fork(z_left: Fork(z_left: Tip(z_value: false), a_right: Tip(z_value: true)), a_right: Tip(z_value: false))`, wrapped in `Stamp<Arbor<Bool>, UInt>::Seal(z_tree: ..., a_code: 41)`.
   - Halts in ~112 microsteps (well within `512`). Matches `npLawMirrorArbor()` and `COMPLETE-SEMANTIC-RETURNS-PREFREEZE.json` exactly.

3. **`npLawHeldOutNotFalse()` & `npLawNotCompleteSavedState()`**:
   - `npLawHeldOutNotFalse()` evaluates `npHeldOutNotProgram()` (`not(false)`) from `ngInitial` through `npRunReference(..., 512)` to `npBool(true)`.
   - `npLawNotCompleteSavedState()` directly tests `ngStep` on a state where the live environment `(99 |-> ngBool(true))` differs from the `ngNotArgument` saved environment `(99 |-> ngUInt(8))` and a lower frame `ngArmScope(3, 1003, 0, .Map)` and prior event `ngLiteralEvaluated(18, 1018, ngBool(false))` are present, verifying exact restoration of `Saved`, frame pop, and event append.

---

## 4. Prospective Operation Envelope & Feasibility Assessment
- **Proposed First Check (`PROPOSED-ONE-EXISTING-F03-COMPILE-DATA.json`)**:
  - Single phase `00-public-nominal-source09-compile`: invokes `org.kframework.main.Main -kompile .../tests/public-nominal-source09-laws.k --backend haskell --no-haskell-binary --main-module SHERLOCK-PUBLIC-NOMINAL-SOURCE09-LAWS --syntax-module SHERLOCK-PUBLIC-NOMINAL-SOURCE09-LAWS`.
  - Reuses the unchanged `F03.launch` wrapper (`k_engine_attempt_f03_01.py`, SHA256 `f1bcf0a0d21b4a7aaedd2f2aae1a1e9e5b62219271eb41b3f71ed97d5a0bc24d`) and existing resource caps (`wall_seconds = 30`, `sweep_seconds = 3`, `rlimit_core = 0`, `rlimit_fsize = 536870912`, `umask = 0o077`, JVM heap `512 MiB`, stack `32 MiB`, `ActiveProcessorCount = 2`).
  - Zero prefix rebuilds, zero law runs in phase 00, and zero new runners/guards.
  - **Follow-on after Compile PASS**: Inventory all actual kompiled output files, qualify generated Kore symbol labels from `definition.kore`, and only then select the separate finite 4-law evaluation run.

---

## 5. Verdict, Mandatory Fixes & Open Boundaries
- **Source/API Verdict**: **APPROVE** (adequacy of scoped K `source09` source contribution and eligibility of the single `00-public-nominal-source09-compile` operation).
- **Mandatory Source Fixes**: **None**.
- **Explicit Open Premises & Non-Claims Preserved**:
  - All candidate K modules and the 4 laws are currently **UNRUN**.
  - Local numeric IDs (`0, 1, 4, 5`, ctor IDs `0, 1, 2`, function IDs `10, 20`, binder IDs `31..34`, occurrences `1001..2028`) are a manual task-local projection, **not** canonical Moriarty compiler `TypeIdC`/AST IDs.
  - Uncharged K reference microsteps and the `512` observer cut do not establish owner-B work/fuel clocks, complete cross-layer event-prefix equality, compiler admission, handwritten/Clean circuit equivalence, native Halo2 proofs, or EARS requirement closure.
