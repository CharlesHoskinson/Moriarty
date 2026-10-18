# Continuation Review — QuintBatch01 Revision 02 (`sherlock-quint-nominal-construction/1`)

- **Reviewer / Conversation Identity**: Gemini 4 Argon (High), continuing non-author review in conversation `8a6a2b55-186d-4a62-a8ae-142ea13ba7e7` (`fresh-provider-cwd`; one continuing opinion, not a second independent profile vote).
- **Execution / Tool Posture**: Zero tools invoked, zero files written, zero commands executed (`ALL REVISION 02 COMMANDS UNRUN`).
- **Corrected Source Verdict (`revision02-identifiers`)**: **APPROVE** (scoped finite Quint companion model source for `SCON-016.R02`; actual Quint frontend typecheck, test, and reachability execution by Root remains required before claiming frontend or runtime pass).
- **Selected Finite Ordinary Operations Verdict (3 Commands in `revision02-identifiers/VALIDATION-PROPOSAL.json`)**: **APPROVE** (eligible for Root's existing ordinary bounded execution procedure in the new `revision02-identifiers/afterimages` working directory, preserving the failed `actual-batch01` receipts untouched).

---

## 1. Explicit Withdrawal and Correction of Prior Built-in Collision Claim

In my initial review (`formal-three-profile-fresh-review-20261018/quint-batch01/REPORT.md`), I cited `SOURCE-STATIC-AUDIT.json` (`"builtin_declaration_collisions": []`) and stated under `new-nominal-typecheck` that *"no builtin collisions exist"*.

**That claim was wrong and is explicitly withdrawn.**

- **Root's actual failure receipt** ([stderr](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/actual-batch01/new-nominal-typecheck/stderr), [RESULT.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/actual-batch01/new-nominal-typecheck/RESULT.json), `returncode: 1`, `0.767043836s`) proved that the original `nominalConstruction.qnt` had two `QNT101` built-in name collisions inside `build(s: Snapshot, c: Construction)`:
  1. Line 212: `val fields = s.program.layouts.get(key).fields` (`Error [QNT101]: Built-in name 'fields' is redefined in module 'nominalConstruction'`)
  2. Line 218: `(tokens, field) => tokens.concat(field))` (`Error [QNT101]: Built-in name 'field' is redefined in module 'nominalConstruction'`)
- **Root cause of the missed collision**: The Quint frontend's name collector (`names/collector.js`) checks all module, local `val`, parameter, lambda, and pattern binders against [`exports.builtinNames`](file:///home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/names/base.js#L71-L177) in `names/base.js`, which includes internal record operator names `'fields'` (line 107) and `'field'` (line 161). The v01 static audit inspected only top-level declarations against an incomplete list and ignored local `val` and lambda binders.

---

## 2. Declared Continuation Read Scope and Scanner Limitations

### Newly Inspected Complete Inline Artifacts
1. [nominalConstruction.qnt (revision02)](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/revision02-identifiers/afterimages/models/sherlock/quint/nominal-construction/nominalConstruction.qnt) (`sha256: 884a9dd0c68f65a7e66f7f4f91d9d5bf224f956978d709fc7ca52201e1a20945`, 16,655 B)
2. [nominalConstructionInputs.qnt (revision02)](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/revision02-identifiers/afterimages/models/sherlock/quint/nominal-construction/nominalConstructionInputs.qnt) (`sha256: eb2e83ca84b24a9a3071410ebf4f15ee6253a3d009b7a0b95585df01c7164590`, 4,072 B, unchanged)
3. [nominalConstructionTest.qnt (revision02)](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/revision02-identifiers/afterimages/models/sherlock/quint/nominal-construction/nominalConstructionTest.qnt) (`sha256: 7a1181847503c51c042d08345543b904e8335d2d1e4eebd8425aec6e3ce58a1b`, 5,070 B, unchanged)
4. [candidate.patch](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/revision02-identifiers/candidate.patch), [CORRECTION.md](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/revision02-identifiers/CORRECTION.md), [CANDIDATE-FREEZE.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/revision02-identifiers/CANDIDATE-FREEZE.json), and [VALIDATION-PROPOSAL.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/revision02-identifiers/VALIDATION-PROPOSAL.json)
5. Preserved failure receipts: [actual-batch01/RESULT.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/actual-batch01/RESULT.json), [ROOT-SELECTION.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/actual-batch01/ROOT-SELECTION.json), [new-nominal-typecheck/RESULT.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/actual-batch01/new-nominal-typecheck/RESULT.json), [stdout](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/actual-batch01/new-nominal-typecheck/stdout), [stderr](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/actual-batch01/new-nominal-typecheck/stderr)
6. [names/base.js](file:///home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/names/base.js), [audit_identifiers_text.py](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/revision02-identifiers/audit_identifiers_text.py), [IDENTIFIER-AUDIT-FACTORED-READBACK.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/formal-batch01-correction-continuation-review-20261018/quint-batch01-revision02/IDENTIFIER-AUDIT-FACTORED-READBACK.json), [ACTUAL-PRIOR-COLLISION-CLAIM.txt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/formal-batch01-correction-continuation-review-20261018/quint-batch01-revision02/ACTUAL-PRIOR-COLLISION-CLAIM.txt), and [MECHANICAL-CORRECTION-READBACK.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/formal-batch01-correction-continuation-review-20261018/quint-batch01-revision02/MECHANICAL-CORRECTION-READBACK.json)

### Concrete Limitations of `audit_identifiers_text.py`
`audit_identifiers_text.py` is a regex-only helper, not a parser or type/effect checker:
- Its lambda regex `r'\(([^()]*)\)\s*=>'` matches only parenthesized parameter lists `(a, b) =>` and misses unparenthesized single-argument lambdas (`name => ...` at `nominalConstruction.qnt:257` and `i => checkedStep` at `nominalConstructionTest.qnt:18,32,40`), module header names (`module nominalConstruction`), and import qualifier aliases (`import ... as I` at `nominalConstructionTest.qnt:3`).
- Those unparenthesized binders (`name`, `i`, `I`) and module names were manually checked against [`exports.builtinNames`](file:///home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/names/base.js#L71-L177) and are collision-free.

---

## 3. Full Re-Audit of Revision 02 Source, Frontend Name/Type/Effect Rules, and Semantics

### 3.1 Exact Diff in `build` (`nominalConstruction.qnt` lines 208–224)
In [nominalConstruction.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/revision02-identifiers/afterimages/models/sherlock/quint/nominal-construction/nominalConstruction.qnt#L208-L224), [`build`](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/revision02-identifiers/afterimages/models/sherlock/quint/nominal-construction/nominalConstruction.qnt#L208) now reads:
```quint
  pure def build(s: Snapshot, c: Construction): Snapshot = {
    val key = (c.typeId, c.ctorId)
    if (not(s.program.layouts.keys().contains(key))) stopDomain(s)
    else {
      val fieldTypes = s.program.layouts.get(key).fields
      if (c.completed.length() != fieldTypes.length() or c.remaining.length() != 0) stopDomain(s)
      else if (not(range(0, fieldTypes.length()).foldl(true, (ok, index) =>
        ok and rootType(c.completed.nth(index)) == fieldTypes.nth(index)))) stopDomain(s)
      else {
        val value = c.completed.foldl(List(ConstructorToken({typeId: c.typeId, ctorId: c.ctorId, arity: fieldTypes.length()})),
          (tokens, fieldValue) => tokens.concat(fieldValue))
        {...s, control: Return(value), environment: c.saved, source: c.source, event: ConstructorBuild,
          constructorNodes: s.constructorNodes + 1, constructorFieldSlots: s.constructorFieldSlots + fieldTypes.length(),
          builds: s.builds.append({occurrence: c.occurrence, typeId: c.typeId, ctorId: c.ctorId, fields: c.completed, value: value})}
      }
    }
  }
```
- Local binder `fields` is renamed to `fieldTypes` (line 212) and all 4 of its value references (`fieldTypes.length()` on lines 213, 214, 217, 220 and `fieldTypes.nth(index)` on line 215) are consistently updated.
- Lambda binder `field` is renamed to `fieldValue` (line 218) and its body reference (`tokens.concat(fieldValue)`) is consistently updated.
- Record field labels and projections (`fields: List[int]` in `Layout`, `Constructor`, `BuildObservation`, `fieldsOf`'s `{cursor: 0, fields: List()}`, `.fields` projections) remain valid in Quint because record labels are lowered to string constants inside `Rec(...)` / `field(..., 'fields')` rather than lexical scope definitions, as confirmed by `actual-batch01/new-nominal-typecheck/stderr` (which parsed lines 8, 16, 41, 66, 88, 91, 93, 129 without error and flagged only the value binders at lines 212 and 218).

### 3.2 Full Check for Remaining Frontend / Type / Effect / Polymorphism Errors
Because `actual-batch01` aborted at the `parse` stage (`QNT101`), the subsequent Quint `typecheck` passes (type inference, unresolved-polymorphism check `QNT205`, effect checking, and mode checking) have not yet been observed at runtime. A full manual inspection of all 3 `.qnt` files shows no remaining static defects:
1. **No remaining `exports.builtinNames` collisions**: Every `module`, `type`, sum-variant constructor, `var`, `val`, `pure val`, `pure def`, `action`, `run`, function parameter, local `val`, parenthesized and unparenthesized lambda parameter (`acc`, `token`, `ignored`, `seen`, `name`, `env`, `index`, `ok`, `tokens`, `fieldValue`, `n`, `b`, `frame`, `i`, `ids`, `state`, `states`, `reached`, `s`, `saved`), and `match` payload binder (`c`, `n`, `t`, `v`, `name`, `e`, `f`, `saved`, `id`, `value`, `_`) across `nominalConstruction.qnt`, `nominalConstructionInputs.qnt`, and `nominalConstructionTest.qnt` is disjoint from the 103 entries of [`exports.builtinNames`](file:///home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/names/base.js#L71-L177).
2. **All empty collections (`List()`, `Map()`, `Set()`) have resolved monomorphic types**: Every empty `List()`, `Map()`, and `Set()` across all 3 files is unified with an explicit function return type (`tokenFields`: `List[int]`), record field type (`Construction.completed`: `List[Value]`, `Layout.fields`: `List[int]`, `Snapshot` fields, `Program.functions`: `int -> Function`), parameter type (`bindNames` `saved: Environment`), or typed equality/fold accumulator (`Set(name)`: `Set[int]`, `tokens.slice(...)`: `List[Token]`, `b.occurrence`: `int`, `runPrefix(...)`: `Snapshot`).
3. **Pure vs. State vs. Action Effect Discipline**:
   - All helper transitions in `nominalConstruction.qnt` are `pure def` over explicit `s: Snapshot` and `p: Program` arguments and never access `var machine`.
   - `initProgram`, `tick`, `terminalStutter`, and `step` in `nominalConstruction.qnt` and `init`, `checkedStep` in `nominalConstructionTest.qnt` are `action` definitions assigning `machine'`.
   - `accountingInv`, `valueInv`, `allInv`, and the 4 witness values (`recursiveFirstFieldWitness`, `recursiveLastFieldWitness`, `orderedArmWitness`, `completedWitness`) are state-reading `val` definitions (`Read[machine]`) suitable for `--invariant` and `--witnesses`.
   - All 8 `run ...Test` definitions in `nominalConstructionTest.qnt` evaluate `bool` action/pure expressions compatible with `quint test`.

### 3.3 Retained Semantic Scope and Non-Equivalence Qualifications
All semantic findings and scope boundaries from the initial review remain unchanged because revision 02 alters no transition formula, input table, test assertion, or fuel rule:
- `ConstructorFields(Construction)` (`nominalConstruction.qnt:19-20`), `construction` (`115-117`), `returnField` (`142-148`), and `build` (`208-224`) preserve nominal `(typeId, ctorId)`, `occurrence`, `source`, `completed` field values, `remaining` field expression IDs, and `saved` environment through recursive first-field (`Schedule(make(2), caller9)`) and last-field (`Link(2, make(1))`) calls, allocating only at post-field `Build(c)`.
- `expected-normal.json` is same-author pre-code material, **not** an independent oracle; the 11-file (890,319 B) independent normal oracle (`dedicated-formal-independent-normal-oracle-20261018/HANDOFF.json`) remains unbound; `allPrefixResumeTest` (`nominalConstructionTest.qnt:75-78`) is an internal pure-value `Snapshot` self-consistency check.
- Profile-local rules (`MAX_SCALAR = 255`, `FRAME_LIMIT = 32`, `TERMINAL_PUBLICATION_CHARGE = 1`, `helperWork = 0`, coarse `OutsideDomain` in `stopDomain`, flat `ConstructorPattern`, partial `source` restoration on non-constructor/non-match frames) do **not** establish Candidate B schema/error-code/helper correspondence, `UIREQ-034`/`SCON-016` closure, production `mori-core` (`FrameRowC`/`ProgramView`) admission, or native ZKIRv3/Midnight proof acceptance.

---

## 4. Separate Classification of the 3 Proposed Validation Commands (`revision02-identifiers/VALIDATION-PROPOSAL.json`)

All 3 commands in [revision02-identifiers/VALIDATION-PROPOSAL.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/revision02-identifiers/VALIDATION-PROPOSAL.json) are **UNRUN** and are classified **separately from product/native closure**:

| # | Command Name | Verdict | Concrete Target / Option Verification & Operational Conditions |
|---|---|---|---|
| 1 | `new-nominal-typecheck` | **APPROVE** | Runs pinned `node` (`--max-old-space-size=512`) and `quint/dist/src/cli.js typecheck models/sherlock/quint/nominal-construction/nominalConstructionTest.qnt` in `cwd` `.../dedicated-quint-20261018/revision02-identifiers/afterimages` with `timeout_seconds: 60`. Targets the corrected `nominalConstruction.qnt` (`sha256: 884a9dd0...`) where `QNT101` binders `fields`/`field` are renamed to `fieldTypes`/`fieldValue`. Actual frontend execution is required to confirm parser + type/effect pass. |
| 2 | `new-nominal-directed-tests` | **APPROVE** | Runs `quint/dist/src/cli.js test models/sherlock/quint/nominal-construction/nominalConstructionTest.qnt --main nominalConstructionTest --match Test$ --backend typescript --max-samples 1 --seed 1` in `.../revision02-identifiers/afterimages` with `--max-old-space-size=512` and `timeout_seconds: 120`. Options match `testCmd` in `cli.js`; `Test$` selects the 8 deterministic `run ...Test` definitions. |
| 3 | `new-nominal-directed-reachability` | **APPROVE** | Runs `quint/dist/src/cli.js run models/sherlock/quint/nominal-construction/nominalConstructionTest.qnt --main nominalConstructionTest --invariant allInv --witnesses recursiveFirstFieldWitness recursiveLastFieldWitness orderedArmWitness completedWitness --backend typescript --max-samples 1 --max-steps 192 --seed 1` in `.../revision02-identifiers/afterimages` with `--max-old-space-size=512` and `timeout_seconds: 60`. Options match `runCmd` in `cli.js` (`--witnesses` array of 4 exported `val` names, `--invariant allInv`, default `init`/`step`). |

### Mandatory Operational Conditions for Root
1. **Preserve prior failure receipts**: Root must write outputs for this `revision02-identifiers` run to a new result path and must **not** overwrite `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/dedicated-quint-20261018/actual-batch01/`.
2. **Existing bounded wrapper enforcement**: `timeout_seconds` (`60`, `120`, `60`) and `limits.network: false` are JSON proposal metadata fields, not Quint CLI flags; Root must enforce the `cwd` (`.../revision02-identifiers/afterimages`), wall-clock timeouts, and stdout/stderr/returncode capture via Root's existing ordinary bounded execution procedure without inventing a new runner or claiming host sandbox attestation.
3. **Honest result scope**: Passing these 3 commands establishes finite Quint frontend typecheck, 8 author-fixture unit tests, and 192-step deterministic invariant/witness reachability for `sherlock-quint-nominal-construction/1` only—not independent whole-prefix oracle comparison, cross-model K/Clean/Candidate B equivalence, `UIREQ-034`/`SCON-016` completion, or native ZKIRv3/Midnight proof acceptance.

