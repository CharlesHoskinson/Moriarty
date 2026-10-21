# Independent Ordinary Quint Model/API Review — `source11` Revision 03 (`review03`)

- **Reviewer**: Gemini 4 Argon (High) via AGY (authorized Opus-outage fallback)
- **Role**: Fresh independent non-author ordinary Quint model/API review
- **Candidate**: `quint-next-public-nominal-identity-source11/revision03-ordered-list-folds` (11 `.qnt` files, `80586` bytes total, frozen in `SOURCE-FREEZE.json` sha256 `59364975a8c77049c1f037449d0d8f31d4a0986ff48de55e031e39f1d92f2995`)
- **Source Verdict**: `APPROVED_SOURCE_ONLY` (`mandatoryCorrections: []`)
- **Operation Eligibility (`02-fiveExactNominalIdentityTests`)**: `ELIGIBLE_AFTER_ROOT_PATH_AND_PIN_REBIND` (source-only approval; tests unrun, no EARS/native/refinement/five-test-pass claim)

---

## 1. Exact 11-File Membership and R3 Frontend Readback

All 11 frozen Quint modules under `revision03-ordered-list-folds/afterimages/` were inspected directly and match `SOURCE-FREEZE.json`:

1. `c6PositiveContextTypes.qnt` (`2978` B, `601ef02d...`)
2. `c6PositiveHostTypes.qnt` (`1897` B, `c17c3edf...`)
3. `c6PositiveMetaTypes.qnt` (`2803` B, `3d93ef1a...`)
4. `c6PositiveProgramTypes.qnt` (`4395` B, `99457551...`)
5. `c6PositiveStateTypes.qnt` (`11346` B, `1336bbc0...`)
6. `nominalPublicSimulation.qnt` (`19394` B, `40459df5...`)
7. `q11NominalIdentity.qnt` (`12420` B, `9d098cab...`) — sole file modified in R3
8. `q11NominalIdentityExpected.qnt` (`6484` B, `ced3884f...`)
9. `q11NominalIdentityInputs.qnt` (`13482` B, `b6b9ba40...`)
10. `q11NominalIdentityTest.qnt` (`4160` B, `d99eb15d...`)
11. `q11NominalIdentityTypes.qnt` (`1227` B, `9733a0aa...`)

`ROOT-R3-ONE-FRONTEND-STRICT-READBACK.json` (`d599ee7d...`) records that `01-newNominalIdentityFrontend` (`quint typecheck .../q11NominalIdentityTest.qnt`) exited `0` with `0`-byte stdout/stderr and full `4686` pre/post pin match (`CURRENT-4686-PIN-MANIFEST.json`, `39d5de90...`). No frontend replay is needed or proposed.

---

## 2. Installed Quint API Verification & Eight Ordered `foldl` + `append` Substitutions

Inspection of the installed Quint distribution (`/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/dist/src/builtin.qnt`, `types/builtinSignatures.js`, `effects/builtinSignatures.js`, `runtime/impl/builtins.js`, `runtime/impl/evaluator.js`) confirms:

- `map` (`builtin.qnt:175`, `types/builtinSignatures.js:51`, `runtime/impl/builtins.js:584-588`) has signature `(Set[a], (a) => b) => Set[b]`, which rejected `List` receivers in R2 (`QNT000: Couldn't unify set and list` at `q11NominalIdentity.qnt:101:5`).
- `range` (`(int, int) => List[int]`, `builtins.js:393-399`), `slice` (`(List[a], int, int) => List[a]`, `builtins.js:431-444`), `foldl` (`(List[a], b, (b, a) => b) => b`, `builtins.js:511-513, 752-770`), and `append` (`(List[a], a) => List[a]`, `builtins.js:448-450`) operate on ordered `Immutable.List` values left-to-right and preserve element multiplicity and order.

The eight R3 substitutions in `q11NominalIdentity.qnt` (`EXACT-LIST-REPAIR-INVERSE.json`) are exact:

1. `q11FieldBindings` (`q11NominalIdentity.qnt:14-18`): `range(0,c.field_len).foldl(List(),(mappedRows,i) => ...)` preserves field count, `ordinal: i`, and `rowId = c.field_start + i`.
2. `q11Bindings.types` (`q11NominalIdentity.qnt:30`): `range(0,v.types.length()).foldl(List(),(mappedRows,i)=>mappedRows.append(q11TypeBinding(v,i)))` preserves runtime `type_id` order `0..v.types.length()-1`.
3. `q11Bindings.constructors` (`q11NominalIdentity.qnt:31`): `range(0,v.constructors.length()).foldl(List(),(mappedRows,i)=>mappedRows.append(q11CtorBinding(v,i)))` preserves runtime `ctor_id` order `0..v.constructors.length()-1`.
4. `q11ConstChildren` (`q11NominalIdentity.qnt:101`): `q11Slice(v.constant_fields,s).foldl(List(),(mappedRows,i)=>mappedRows.append(C6Const(i)))` preserves child constant order, duplicate child constant indices, and empty slices.
5. `q11Expr` `C6PEMatch` (`q11NominalIdentity.qnt:197`): `v.arms.slice(...).foldl(List(),(mappedRows,a)=>mappedRows.append({pattern:q11Pattern(v,a.pattern,budget),body:a.body}))` preserves arm declaration order required by first-match `cases` (`nominalPublicSimulation.qnt:268-276`).
6. `q11Program` parameter binders (`q11NominalIdentity.qnt:206`): `params.foldl(List(),(mappedRows,p)=>mappedRows.append(p.binder))` preserves binder order and duplicates so `distinct(f.parameters)` (`nominalPublicSimulation.qnt:244`) is not bypassed.
7. `q11Program` parameter sorts (`q11NominalIdentity.qnt:207`): `params.foldl(List(),(mappedRows,p)=>mappedRows.append(p.sort))` preserves positional parameter sort alignment even when multiple parameters share a sort.
8. `q11Program` layout fields (`q11NominalIdentity.qnt:212`): `v.ctor_fields.slice(...).foldl(List(),(mappedRows,f)=>mappedRows.append(f.type_id))` preserves duplicate field `type_id`s (such as `Fork` in `q11MirrorView` having two fields of `type_id: 4` at `ctor_fields` rows `6` and `7`).

---

## 3. Technical Assessment of Semantics and Five Actual `run` Tests

- **Row/ID/Declaration/Name/Field Order & Duplicates**: Builtin types `0..3` satisfy `C6PNone => id < 4` (`q11DeclaredTypeRelates`, `q11NominalIdentity.qnt:69`) and carry empty names (`id >= 4` / `owning_type >= 4` guards at `:17, :21, :26`). User nominal types (`4, 5`) link via `C6PSome(declId)` to `decl_templates`, while `name_id` indexes the sorted UTF-8 table `v.names` independently of declaration order (e.g., `Push` field `0` is `z_item` with `name_id: 12` before field `1` `a_tail` with `name_id: 6`). Duplicate constant rows (`q11MapView.constants` rows `4` and `5`) and empty slices (`Empty` with `field_start: 5, field_len: 0`) are preserved.
- **Full Retained `C6PProgram` (23 fields) & `C6HostImage` (23 fields)**: `q11Bindings` (`:29-32`), `q11ObserveEvidence` (`:145-148`), and `q11EvidencePreserved` (`:149-150`) retain the unmodified 23-field `C6PProgram` and 23-field `C6HostImage`.
- **`Const`/`Data` Traversal & Residual**: `q11ReadReference` (`:139-140`) and `q11ReadStep` (`:131-138`) perform bounded depth-first preorder unfolding (`node.children.concat(state.pending.tail())`, `state.tokens.append(node.token)`), mapping `Nil`/`Cons`/`Leaf`/`Branch` to `(2,0)`, `(2,1)`, `(3,2)`, `(3,3)` and preserving any unvisited `C6Ref`s in `state.pending` when `budget` is exhausted.
- **Source Projection & Reused Evaluator**: `q11Pattern`, `q11Expr`, `q11Program`, and `q11RunSource` (`:156-218`) project `C6PProgram` into the unchanged `nominalPublicSimulation` evaluator (`40459df5...`), keying `layouts` by `(c.owning_type, id)` using runtime constructor index `id`.
- **Five Actual `run` Declarations in `q11NominalIdentityTest.qnt`**:
  1. `q11CompleteInputBindingsTest` (`:10-20`): checks `q11Bindings` and `q11DeclarationRelation` on `q11MapView` and `q11MirrorView` against `q11MapExpectedTypes`, `q11MapExpectedCtors`, `q11MirrorExpectedTypes`, and `q11MirrorExpectedCtors`.
  2. `q11BothSourceReturnsUseRuntimeIndicesTest` (`:21-33`): runs `q11RunSource` (`fuel=256, readBudget=256`) on both views and checks `Done(q11MapExpectedReturn)` and `Done(q11MirrorExpectedReturn)`, `validValue`, empty final `environment`/`frames`, and `helperWork == 0`.
  3. `q11SameTypedFieldOrderDistinguishesTest` (`:34-52`): swaps same-typed fields (`type_id == 4`) of `Fork` (`ctor_id: 5`, rows `6` and `7` of `q11MirrorView.ctor_fields`) via `replaceAt`, verifying `not(q11DeclarationRelation(contrasting))` and `q11Bindings(contrasting).constructors != q11Bindings(q11MirrorView).constructors`.
  4. `q11AllOrderedInputTablesAndOptionsRetainedTest` (`:53-72`): checks full `retained_view` preservation, `C6PSome(0) != C6PNone`, `Empty` slice `{field_start: 5, field_len: 0}`, duplicate constants `nth(4) == nth(5)`, and retained `occurrences`/`plan_cases`/`decl_templates`.
  5. `q11ConstantReferenceReaderPreservesSourceValuesTest` (`:73-87`): checks `q11ReadConstant` on duplicate constructor constants `4` and `5`, `UIntToken(3)`, and `BoolToken(false)` with empty residual `pending == List()`.

---

## 4. Limitations & Eligibility of `02-fiveExactNominalIdentityTests`

1. **Unrun Tests & Unbound Final Host Image**: The five tests in `q11NominalIdentityTest.qnt` remain unrun on R3. `q11DataNode`, `q11DataChildren`, `q11ObserveEvidence`, and `q11EvidencePreserved` (`q11NominalIdentity.qnt:102-150`) are typechecked but not exercised against a live `C6HostImage` in `q11NominalIdentityTest.qnt`.
2. **Positive Partial Premises**: `nth`/`head`/`slice` calls and empty-candidate fallbacks (`C6PKOutOfRange` in `q11ConstNode` at `:118-120`, unexhausted `pending` in `q11Pattern` at `:172-173`, unsupported ops in `q11Expr` at `:187, :191`) are partial over externally qualified inputs and do not model Core refusal or admission policy.
3. **No Core Clock / Helper-Work / Host Refinement Claim**: `q11RunSource` projects into the finite token machine (`helperWork == 0`, `MAX_SCALAR == 255`, `FRAME_LIMIT == 32`), leaving `C6PEConstruct.constant`, `C6PEMatch.plan`, `plan_nodes`, `plan_cases`, `field_paths`, and `occurrences` as retained data rather than microstep execution.
4. **Parameter & Pattern Linearity Premises**: `q11TypeTokenMatches` (`:46`) assumes `C6PTParam` names exist uniquely in `decl.parameters`; `nestedPatternMatch` (`nominalPublicSimulation.qnt:118-141`) assumes linear pattern binders per arm.

**Eligibility Conclusion**: Proposed operation `02-fiveExactNominalIdentityTests` in `REMAINING-VALIDATION-PROPOSAL.json` is **ELIGIBLE** for Root selection once Root performs the data-only caller rebind (`ROOT-MANUAL-FIVE-TESTS.py`) to current R3 paths and pins, with no new helper or capabilities.
