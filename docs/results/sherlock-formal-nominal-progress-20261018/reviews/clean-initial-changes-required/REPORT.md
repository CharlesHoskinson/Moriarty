### Metadata, Identity, and Review Status

- **Returned Model Identity**: Gemini 4 Argon (High)
- **Effort / Tier**: High
- **Routing / Authority Context**: Fresh independent non-author review for **CleanBatch01 ONLY** under standing observed Opus quota/outage fallback; one context equals one independent opinion (not multiple votes). Root alone adopts or executes.
- **Working Directory (`cwd`)**: `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/formal-three-profile-fresh-review-20261018/clean-batch01/revision02/fresh-provider-cwd`
- **Conversation ID (`CID`)**: `162f2a2e-167e-4c73-a37f-25e93b112428`
- **Execution / Tool Mode**: Strict no-tools / read-only inline review (`NO TOOLS, execution, filewrites orcodegeneration`); 0 commands run, 0 files written, all 4 proposed kernel check stages remain **`UNRUN`**.

---

### 1. Scope, Read Coverage, and Explicit Gaps

#### Complete Inline Bodies Read (No Clipping or Truncation)
All 18 inline blocks in the prompt were received complete (`BEGIN COMPLETE ...` through `END COMPLETE BODY` and terminal `END-FRESH-FORMAL-CLEAN-BATCH01-SOURCE-REVIEW`):
1. **Product & Normative Contracts**:
   - `docs/MORIARTY-PRODUCT-CONTRACT.md`
   - `openspec/changes/sherlock-completion-convergence/ADOPTION.md`
   - `openspec/changes/sherlock-universal-interpreter-design/interfaces.md`
   - `openspec/changes/sherlock-universal-interpreter-design/clean-stage.md`
   - `context/NORMATIVE-REQUIREMENTS.txt` (`UIREQ-001`, `UIREQ-003`, `UIREQ-004`, `UIREQ-008`, `UIREQ-010`, `UIREQ-034`, `SCON-016`, and `SCON-016.R02`)
2. **Production Rust Comparison Excerpts**:
   - `crates/mori-core/src/sherlock/helper_machine/shared/adt.rs`
   - `source-context/production-frame-control-declarations.lines189-309.rs.txt` (with provenance in `PRODUCTION-EXCERPT-PROVENANCE.json`, SHA256 `911a6b4d59eac6f26b69884430cebe3af81766b960b5d38ec045a694513b05e9` from `shared.rs` bytes `14818..21925`)
3. **CleanBatch01 Candidate & Imported Lean Sources**:
   - `afterimage/clean/fork/Clean/Sherlock/CanonicalTypeId.lean`
   - `afterimage/clean/fork/Clean/Sherlock/ConstructorFields.lean`
   - `afterimage/clean/fork/Clean/Sherlock/ConstructorFieldsNormal.lean`
   - `readset/fenrir/clean/fork/Clean/Sherlock/Semantics.lean`
   - `readset/fenrir/clean/fork/lean-toolchain` (`leanprover/lean4:v4.33.1`)
   - `readset/fenrir/clean/fork/lakefile.lean`
4. **Handoff, Operation Plan & Static Toolchain/Oracle Metadata**:
   - `dedicated-clean-20261018/HANDOFF.md`
   - `dedicated-clean-20261018/PROPOSED-ARGV.json`
   - `NORMAL-TOOL-METADATA-PINS.json`
   - `clean-batch01/revision02/TOOLCHAIN-AND-ORACLE-ADDITIVE-FACTS.json`

#### Explicitly Excluded / Unread / Unrun Scope (No Approval Transfer)
- **CleanBatch02 (`NominalValue` / `Schedule` / `MatchPlan`) and CleanBatch03**: Not selected in this profile; all their sources, votes, and checks remain separate and `UNRUN`.
- **Excluded Production Rust & Design Files**: Full `shared.rs` outside lines 189–309 (`44,525` bytes total), compiler/runtime implementation files, full revision03 `design.md`, `FIRST-COMPILER-BATCH.md`, `security-requirements.md`, and `security-candidate-b.md` (Opus-owned `UISEC` / `A1–A4` / `B001–B009`) are not fresh-read here.
- **External Toolchain & Independent Oracle Files**: The 1,114 `Init`/`Std` module sources/artifacts in `TOOLCHAIN-PINS.json` (`2,888,299` bytes, SHA256 `97b9e4410b585e4670588b5ea27eeda64f653f26592bf80ecea02eea22ea8d43`) and the separate independent normal oracle (`dedicated-formal-independent-normal-oracle-20261018/HANDOFF.json`, `890,319` bytes / 11-file full-state oracle) are not inlined and not evaluated or discharged by the 4 author examples in `ConstructorFieldsNormal.lean`.

---

### 2. CleanBatch01 Conditional Semantic Source Review

#### Source Verdict: **REVISE**

`CanonicalTypeId.lean` and `ConstructorFieldsNormal.lean` are syntactically and logically sound within their stated scope, and the conditional continuation model in `ConstructorFields.lean` accurately reflects `SCON-016.R02` and `interfaces.md` without overclaiming runtime or circuit integration. **However**, `Clean/Sherlock/ConstructorFields.lean` contains **two concrete Lean 4 frontend/kernel defects** (left undetected because all checks are currently `UNRUN`) that will cause Stage 2 (`ConstructorFields`) to fail compilation in Lean 4.33.1 unless revised.

---

#### Mandatory Fixes for `Clean/Sherlock/ConstructorFields.lean`

1. **Mandatory Fix 1 — Reserved Keyword Collision on Identifier `prefix`**:
   - **Location**: `Clean/Sherlock/ConstructorFields.lean`, `def beginFrom` (lines 60–63) and `theorem runReturned_complete` (lines 96–114).
   - **Literal Quote**:
     ```lean
     def beginFrom (typeId : TypeId) (ctorId : CtorId) (origin : Nat)
         (prefix : List Value) (expressions : List Expr) (saved : Env) : Cursor Expr Value Env :=
       schedule { typeId, ctorId, origin, completedValues := prefix,
         remainingExprs := expressions, savedEnvironment := saved }
     ```
     and:
     ```lean
     theorem runReturned_complete (typeId : TypeId) (ctorId : CtorId) (origin : Nat)
         (expressions : List Expr) (values : List Value) (saved : Env) (prefix : List Value)
         (hLength : expressions.length = values.length) :
         runReturned values (beginFrom typeId ctorId origin prefix expressions saved) =
           some (.build { typeId, ctorId, origin, fields := prefix ++ values,
             savedEnvironment := saved }) := by
       induction expressions generalizing values prefix with
     ```
   - **Defect / Counterexample**: In Lean 4 (`Init.Notation`), `prefix` is a registered command/notation keyword (`prefix:prec ... => ...`) in the global parser token table. Unescaped `prefix` is lexed as a keyword token rather than an `ident`, causing the Lean 4 parser to reject `(prefix : List Value)` with `unexpected token 'prefix'; expected identifier`.
   - **Required Fix**: Rename the parameter/variable `prefix` to a non-keyword identifier such as `fieldPrefix` or `completedPrefix` (or escape as `«prefix»`) in `beginFrom` and `runReturned_complete`.

2. **Mandatory Fix 2 — Non-Definitional `rfl` on `prefix ++ []` in `runReturned_complete` Base Case**:
   - **Location**: `Clean/Sherlock/ConstructorFields.lean`, `theorem runReturned_complete` (lines 102–104).
   - **Literal Quote**:
     ```lean
       induction expressions generalizing values prefix with
       | nil =>
         cases values with
         | nil => rfl
         | cons value rest => simp at hLength
     ```
   - **Defect / Counterexample**: When `expressions = []` and `values = []` with `prefix : List Value` generalized as a free variable:
     - The LHS `runReturned [] (beginFrom typeId ctorId origin prefix [] saved)` reduces definitionally to:
       `some (.build { typeId := typeId, ctorId := ctorId, origin := origin, fields := prefix, savedEnvironment := saved })`.
     - The RHS is:
       `some (.build { typeId := typeId, ctorId := ctorId, origin := origin, fields := prefix ++ [], savedEnvironment := saved })`.
     - Because `List.append` in Lean 4 is defined by structural recursion on its **first** argument (`[] ++ ys = ys` is `rfl`, whereas `xs ++ [] = xs` is the inductive theorem `List.append_nil`), `prefix ++ []` is stuck on the free variable `prefix` and is **not** definitionally equal to `prefix`. Thus `rfl` fails in the kernel/elaborator.
   - **Required Fix**: Replace `| nil => rfl` on line 104 with:
     ```lean
         | nil => simp [runReturned, beginFrom, schedule]
     ```
     (which rewrites `prefix ++ []` to `prefix` via `@[simp] theorem List.append_nil`). Note that in the `cons` case (lines 110–114), `change runReturned values (beginFrom typeId ctorId origin (fieldPrefix ++ [value]) expressions saved) = _` is definitionally valid because `beginFrom` unfolds to `schedule` with `completedValues := fieldPrefix ++ [value]`, matching `returnField` on `schedule`, and `simp only [List.append_assoc, List.singleton_append]` closes `(fieldPrefix ++ [value]) ++ values = fieldPrefix ++ value :: values`.

---

#### Detailed Evaluation of CleanBatch01 Modules and Conditional Premises

1. **`Clean/Sherlock/CanonicalTypeId.lean` vs. `adt.rs`**:
   - **Quotes**:
     - `CanonicalTypeId.lean`: `def TypeId.ofBuiltin : BuiltinType → TypeId | .uint => ⟨0⟩ | .bool => ⟨1⟩ | .list => ⟨2⟩ | .tree => ⟨3⟩` and `def TypeId.builtin : TypeId → Option BuiltinType | ⟨0⟩ => some .uint ... | ⟨_ + 4⟩ => none`.
     - `adt.rs`: `pub struct TypeIdC(pub usize);` with `pub const UInt: Self = Self(0); pub const Bool: Self = Self(1); pub const List: Self = Self(2); pub const Tree: Self = Self(3);` and `pub struct CtorIdC(pub usize);`.
   - **Assessment**: `builtin_ofBuiltin`, `builtin_nominal`, and `typeId_eq_iff` are valid and establish that builtin sorts occupy indices `0..3` while nominal indices `offset + 4` are preserved without collapsing to a 4-way sort tag. As stated in the module docstring, this is a mathematical `Nat` carrier only; it does not prove `usize` bounds, `Bounds.typeRows`/`Bounds.constructorRows` admission, or circuit field encoding.

2. **`Clean/Sherlock/ConstructorFields.lean` vs. `SCON-016.R02` & `interfaces.md`**:
   - **Quotes**:
     - `SCON-016.R02`: `"GenericNominal construction SHALL have an explicit ConstructorFields semantic continuation before Build/Materialize can be treated as end-to-end construction. Its payload SHALL retain nominal type/constructor identity, source occurrence where supported, saved environment, evaluated field prefix, remaining field expressions or equivalent cursor, and eventual return context."`
     - `interfaces.md`: `ConstructorFields(typeId,ctorId,completedValues,remainingExprs,savedEnvironment)`
     - `ConstructorFields.lean`: `def returnField (cursor : Cursor Expr Value Env) (value : Value) : Option (Cursor Expr Value Env) := match cursor with | .eval _ _ frame => some (schedule { frame with completedValues := frame.completedValues ++ [value] }) | .build _ => none`
   - **Conditional Premise Boundaries**:
     - **Pure Value-Return Relay vs. Child Evaluation**: `runReturned (values : List Value) (cursor : Cursor Expr Value Env)` does **not** evaluate any `expression : Expr`. When `schedule` constructs `.eval expression frame.savedEnvironment { frame with remainingExprs := rest }`, `returnField` discards `expression` and `environment` (`| .eval _ _ frame => ...`) and appends the externally supplied `value` to `frame.completedValues`.
     - **Role of `FieldsEvaluate` in `completed_fields_meaning`**:
       ```lean
       theorem completed_fields_meaning (evaluates : Env → Expr → Value → Prop)
           (typeId : TypeId) (ctorId : CtorId) (origin : Nat) (saved : Env)
           (expressions : List Expr) (values : List Value)
           (h : FieldsEvaluate evaluates saved expressions values) :
           runReturned values (beginFields typeId ctorId origin expressions saved) =
             some (.build { typeId, ctorId, origin, fields := values, savedEnvironment := saved }) := by
         exact runReturned_complete typeId ctorId origin expressions values saved [] h.length_eq
       ```
       `FieldsEvaluate` is an uninterpreted relational premise `(evaluates : Env → Expr → Value → Prop)` used solely to derive `h.length_eq : expressions.length = values.length`. It proves that **given** any child evaluation relation producing a value list of matching length, `ConstructorFields` schedules expressions in strict left-to-right source order, resets the active child environment to `frame.savedEnvironment` at every field boundary (`context_schedule`, `returnField_preserves_context`), composes across technical cuts (`runReturned_append`), and produces `.build` with `fields := values` and `savedEnvironment := saved`.
     - **What Is Not Established**:
       - No coupling to `Semantics.evalExpression` / `Semantics.advance` / `Semantics.step`.
       - No recursive child frame stack (`frames : [Frame]`) push/pop or `ReturnToCaller` restoration across nested calls (`SCON-016.R02` scenarios *Recursive first constructor field* and *Recursive last constructor field* are represented only at the single-frame continuation level, not as a multi-frame machine run).
       - No `Append` helper microsteps (`HelperControlC::Append`), post-evaluation `Build`/`Materialize` arena allocation (`dataNodes`/`dataFieldSlots`), `CtorLayout` arity/field-type checking, B-schedule work charging (`sherlock-helper-microstep/1`), or `MachineImage`/`StepInv` encoding.
     - **Production Rust Boundary**:
       - `shared.rs` (lines 211–214): `// FrameRow: exactly 9 core variants. ConstructorFields is NOT a variant: a reserved ConstructorFields tag at any owner boundary is refused HostViolation{WrongVariant}/C-INV, its payload is never carried here.`
       - `ADOPTION.md`: `"The first actual compiler source task found ProgramView has twelve fields and no constructor/type/match-plan tables."`
       - CleanBatch01 accurately treats `ConstructorFields` as a specification-stage Lean continuation slice and implies no new Rust `FrameRowC` admission, tag, width, guard, or error code.

3. **`Clean/Sherlock/ConstructorFieldsNormal.lean` vs. `Semantics.lean` and Independent Oracle**:
   - **Quotes**:
     - `ConstructorFieldsNormal.lean`: `def initial : Cursor Expr Value Env := beginFields ⟨9⟩ ⟨12⟩ 37 [firstExpression, secondExpression] saved` and `theorem two_fields_build_in_source_order : runReturned [.uint 7, .bool false] initial = some (.build { typeId := ⟨9⟩, ctorId := ⟨12⟩, origin := 37, fields := [.uint 7, .bool false], savedEnvironment := saved }) := rfl`
   - **Assessment**: The 4 theorems (`first_return_retains_pending_second`, `two_fields_build_in_source_order`, `nullary_build`, `zero_returns_preserve_cursor`) are valid `rfl` reductions when instantiated with `Semantics.Expr`, `Semantics.Value`, and `Semantics.Env`. However, they supply `[.uint 7, .bool false]` directly to `runReturned` rather than evaluating `firstExpression` (`.var 10`) or `secondExpression` (`.literal (.bool false)`) through `Semantics.step`, and `Semantics.Value` remains the 4-variant first-order type (`uint | bool | list | tree`). These 4 author examples do **not** substitute for the separate 11-file (`890,319`-byte) independent normal oracle cohort recorded in `TOOLCHAIN-AND-ORACLE-ADDITIVE-FACTS.json`.

---

### 3. Exact Finite Operation Classification (`PROPOSED-ARGV.json`)

#### Shared Envelope and Preconditions Across All Four Stages
- **Resource & Isolation Envelope**: `systemd-run --user --scope --quiet -p MemoryMax=4G -p MemorySwapMax=0 -p TasksMax=256`, `timeout --kill-after=10s 200s`, `bwrap --unshare-net --ro-bind / /` with writable `--bind` mounts restricted strictly to `.../proposed-local-check/artifacts` and `.../proposed-local-check/scratch`, `ulimit -f 16384` (16 MiB file size limit), direct `/home/charl/.elan/toolchains/leanprover--lean4---v4.33.1/bin/lean` frontend invocation. No `lake`, no network fetch, no native prover, no circuit execution.
- **Single Artifact Root**: `LEAN_PATH` is set uniformly to `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-clean-20261018/proposed-local-check/artifacts`, resolving the cross-directory import between `afterimage/clean/fork` and `readset/fenrir/clean/fork` into a single fresh `.olean` DAG (`Clean/Sherlock/*.olean`).
- **Static Pin Caveat**: Tool hashes in `NORMAL-TOOL-METADATA-PINS.json` and the 1,114-module static `Init`+`Std` inventory in `TOOLCHAIN-AND-ORACLE-ADDITIVE-FACTS.json` are file metadata only, not proof of host `systemd --user`/`bwrap` availability or dynamic linker behavior. Root must create `proposed-local-check/artifacts/Clean/Sherlock` and `proposed-local-check/scratch`, verify file hashes and destination absence, and capture the inherited execution environment and full stdout/stderr/exit status.

#### Per-Stage Classification Table

| Stage | Unit Name | Source Input (`-R` Root) | `.olean` Output in `LEAN_PATH` | `requiresPassed` | Current Status | Classification & Conditions |
|---|---|---|---|---|---|---|
| **1. `CanonicalTypeId`** | `sherlock-dedicated-clean-canonicaltypeid-20261018` | `.../afterimage/clean/fork/Clean/Sherlock/CanonicalTypeId.lean` (`-R .../afterimage/clean/fork`) | `.../artifacts/Clean/Sherlock/CanonicalTypeId.olean` | `[]` | `UNRUN` | **ELIGIBLE (Conditional on Root pre-flight directory/hash checks)**. Imports only `Std` via `LEAN_SYSROOT`; no syntax or proof defects found. |
| **2. `ConstructorFields`** | `sherlock-dedicated-clean-constructorfields-20261018` | `.../afterimage/clean/fork/Clean/Sherlock/ConstructorFields.lean` (`-R .../afterimage/clean/fork`) | `.../artifacts/Clean/Sherlock/ConstructorFields.olean` | `["CanonicalTypeId"]` | `UNRUN` | **ELIGIBLE AS ORDINARY CHECK / WILL FAIL KERNEL CHECK UNLESS REVISED**. Envelope and DAG (`CanonicalTypeId.olean`) are valid, but unmodified source will fail on the `prefix` keyword collision (lines 61, 97) and `rfl` on `prefix ++ []` (line 104). Eligible either to observe that exact failure or to run after applying Mandatory Fixes 1 & 2. |
| **3. `Semantics`** | `sherlock-dedicated-clean-semantics-20261018` | `.../readset/fenrir/clean/fork/Clean/Sherlock/Semantics.lean` (`-R .../readset/fenrir/clean/fork`) | `.../artifacts/Clean/Sherlock/Semantics.olean` | `["CanonicalTypeId", "ConstructorFields"]` | `UNRUN` | **ELIGIBLE (Conditional on Stage 1–2 completion under sequential stop-on-first-failure rule)**. Compiles frozen existing `Semantics.lean` (imports only `Std`) into the shared `LEAN_PATH`. Blocked if Stage 2 fails and halts the sequence. |
| **4. `ConstructorFieldsNormal`** | `sherlock-dedicated-clean-constructorfieldsnormal-20261018` | `.../afterimage/clean/fork/Clean/Sherlock/ConstructorFieldsNormal.lean` (`-R .../afterimage/clean/fork`) | `.../artifacts/Clean/Sherlock/ConstructorFieldsNormal.olean` | `["CanonicalTypeId", "ConstructorFields", "Semantics"]` | `UNRUN` | **ELIGIBLE (Conditional on Stages 1–3 passing after `ConstructorFields.lean` fixes)**. Imports `Clean.Sherlock.ConstructorFields` and `Clean.Sherlock.Semantics` from the shared `LEAN_PATH`; its 4 concrete `rfl` theorems are well-typed once `ConstructorFields.olean` and `Semantics.olean` exist. |

#### Non-Equivalence to Product / Target Acceptance
Even after `ConstructorFields.lean` is revised and all 4 stages pass the Lean 4.33.1 kernel check, that result establishes **only** finite ordinary Lean typechecking and theorem kernel verification for the CleanBatch01 conditional continuation slice. It does **not** establish CleanBatch02/03 coverage, independent normal oracle cohort agreement, Candidate B helper-metered correspondence, `MachineImage`/`StepInv` soundness (Opus `A1–A4`/`B001–B009`), production Rust `FrameRowC` integration, Clean circuit constraints, ZKIRv3 compilation, or Midnight native/financial acceptance.

