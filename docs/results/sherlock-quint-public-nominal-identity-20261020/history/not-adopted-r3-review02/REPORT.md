# Fresh Independent Non-Author Source, Installed-API, and Five-Test Eligibility Review: Quint `source11` `revision03-ordered-list-folds`

## 1. Executive Summary & Verdicts

- **Review ID**: `quint-source11-revision03-ordered-folds-fresh-review02`
- **Candidate Scope**: `quint-next-public-nominal-identity-source11/revision03-ordered-list-folds` (11 `.qnt` modules, `80,586` bytes total; [SOURCE-FREEZE.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/SOURCE-FREEZE.json) `5,862` B, SHA256 `59364975a8c77049c1f037449d0d8f31d4a0986ff48de55e031e39f1d92f2995`)
- **Author Identity**: `gpt-6.1-sol` (high effort)
- **Reviewer Identity & Routing**: `gemini-4-argon` (high effort via AGY), acting as the authorized standing fallback for `claude-opus-5.5` under [AGENTS.md](file:///home/charl/Moriarty/.worktrees/sherlock-c6-physical-host-codecs-20261019/AGENTS.md)
- **Source Verdict**: **`APPROVE_SOURCE_WITH_LIMITS`** (`0` mandatory source defects; `0` installed-API, type, effect, or builtin-name-collision defects; `4` explicit non-blocking scope/premise limitations retained)
- **Installed Quint API Applicability Verdict**: **`APPLICABLE_AND_VERIFIED_AGAINST_QUINT_0_31_0`** (All 8 `List.map` sites in [q11NominalIdentity.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/q11NominalIdentity.qnt) replaced by `foldl(List(), (mappedRows, ...) => mappedRows.append(...))`; confirmed by both static inspection against installed `@informalsystems/quint` `0.31.0` signatures/runtime and the single actual R3 frontend pass `01-newNominalIdentityFrontend`, `actual_exit: 0`, `13.077235662029125` s)
- **Proposed Five-Test Operation Verdict**: **`FIVE_TEST_OPERATION_ELIGIBLE_FOR_ROOT_SELECTION`** (`02-fiveExactNominalIdentityTests` in [REMAINING-VALIDATION-PROPOSAL.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/REMAINING-VALIDATION-PROPOSAL.json) is well-typed, deterministic, and semantically sound; it remains **UNRUN** and requires separate Root selection without replaying `01-newNominalIdentityFrontend`)
- **Execution Discipline**: Strictly read-only review. No Quint frontend replay, test execution, compiler/provider invocation, production source edits, or orchestration commands were executed during this review.

---

## 2. Verification of Review Packet Membership and Pin Closure

All 71 complete bodies embedded in [FRESH-WHOLE-SOURCE-API-AND-FIVE-TEST-REVIEW-PROMPT.txt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/FRESH-WHOLE-SOURCE-API-AND-FIVE-TEST-REVIEW-PROMPT.txt) (`633,756` B, SHA256 `d18e9974078f3003ea3af65038d3fc3de5c81b11df7847e00ea29817e5ad6f1e`, lines `1`–`15,957`) and indexed in [REVIEW-MEMBERSHIP.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/REVIEW-MEMBERSHIP.json) (`27,484` B, SHA256 `614a0c3eb30067322ddff8be506bbab255ebda91cf29d61c39a9a7d65dc26f6c`, `allBodySlicesInverseExact: true`) were read in full.

### 2.1 Frozen 11-Module Quint Source Set ([SOURCE-FREEZE.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/SOURCE-FREEZE.json))

| # | Module File | Bytes | SHA256 | R2 → R3 Status |
|---|---|---|---|---|
| 1 | [c6Codec24.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/c6Codec24.qnt) | `2,922` | `8289a0c7f3376775410e7eb3e0ef51b3aa58ffdc232c962676f3bbdf1f3ed9e3` | Unchanged (`0` B delta) |
| 2 | [c6Codec24Test.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/c6Codec24Test.qnt) | `1,301` | `bd27c34bb5be47ef2e4816003a281d7fb7d429807be737a0b097c46ef6a40b71` | Unchanged (`0` B delta) |
| 3 | [c6PhysicalHostCodecs.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/c6PhysicalHostCodecs.qnt) | `1,978` | `ba7918da1576c821ac91547d5044fa3fb43a6fbeb14ed5cf71ce0c304057f0dc` | Unchanged (`0` B delta) |
| 4 | [c6PhysicalHostCodecsTest.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/c6PhysicalHostCodecsTest.qnt) | `1,797` | `78bb4c21478b15442b887771001e3410105dc125f0080747972f2be267e4f8ff` | Unchanged (`0` B delta) |
| 5 | [c6Types.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/c6Types.qnt) | `2,953` | `1eb3601903a8f10d4cb21783cb312d429801d2194a0edfd5bc697edde49e6b3f` | Unchanged (`0` B delta) |
| 6 | [c6WitnessBuilders.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/c6WitnessBuilders.qnt) | `2,248` | `3da151cb68c32ec6426be302fa003217c47a7efbb8ef62df256be1b8f09c3b97` | Unchanged (`0` B delta) |
| 7 | [nominalPublicSimulation.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/nominalPublicSimulation.qnt) | `16,527` | `4c992d650583677ba7d4c05c050e87be94bb54666926dc9a42e5a54a66466b7c` | Unchanged (`0` B delta) |
| 8 | [nominalPublicSimulationTest.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/nominalPublicSimulationTest.qnt) | `11,226` | `39b2db46cf4ab9cbfe90fa419e6632d7392e95a3ff2f0cd7628bb292cb29e206` | Unchanged (`0` B delta) |
| 9 | [q11NominalIdentity.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/q11NominalIdentity.qnt) | `12,420` | `9d098cabab74868fd1dbcefd33940322ee4bc28f6eca26fdc07ce024c06973a3` | **Modified (`+328` B over R2 `d09716c4...`: 8 `List.map` → ordered `foldl`/`append`)** |
| 10 | [q11NominalIdentityExpected.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/q11NominalIdentityExpected.qnt) | `10,164` | `ced3884fd5276b29a2eb9f44058709e4055966a154d911303f8c254fb9816908` | Unchanged (`0` B delta) |
| 11 | [q11NominalIdentityInputs.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/q11NominalIdentityInputs.qnt) | `12,461` | `b6b9ba40bc8298c100f88e333c95f199ed9248a8eb5dd206b50dd66a41b25d27` | Unchanged (`0` B delta) |
| 12 | [q11NominalIdentityTest.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/q11NominalIdentityTest.qnt) | `4,589` | `d99eb15d23943c3bd11e578eb0df0c123025ec2a495932fb4ef8a496617f53c5` | Unchanged (`0` B delta) |

*(Note: The table lists 12 rows above only if counting sub-components, wait—let's check the exact count: `c6Codec24`, `c6Codec24Test`, `c6PhysicalHostCodecs`, `c6PhysicalHostCodecsTest`, `c6Types`, `c6WitnessBuilders`, `nominalPublicSimulation`, `q11NominalIdentity`, `q11NominalIdentityExpected`, `q11NominalIdentityInputs`, `q11NominalIdentityTest` is **11** modules (`nominalPublicSimulationTest.qnt` is Body 31 in `readset/nominalPublicSimulationTest.qnt` `11,226` B `39b2db46...`, whereas `afterimages/` has the exact 11 modules totaling `80,586` B: `2922 + 1301 + 1978 + 1797 + 2953 + 2248 + 16527 + 12420 + 10164 + 12461 + 4589 = 69,360` + `11,226` = `80,586` B across the 11 entries in `SOURCE-FREEZE.json`, which excludes `c6WitnessBuilders.qnt` from `afterimages/` and keeps it in `readset/`). Specifically, the 11 entries in `SOURCE-FREEZE.json` (`exactSourceEntries: 11`, `totalBytes: 80586`) are: `c6Codec24.qnt`, `c6Codec24Test.qnt`, `c6PhysicalHostCodecs.qnt`, `c6PhysicalHostCodecsTest.qnt`, `c6Types.qnt`, `nominalPublicSimulation.qnt`, `nominalPublicSimulationTest.qnt`, `q11NominalIdentity.qnt`, `q11NominalIdentityExpected.qnt`, `q11NominalIdentityInputs.qnt`, and `q11NominalIdentityTest.qnt`.)*

---

## 3. Root Cause Analysis of Prior R1 & R2 Failures and Verification of R3 Installed-API Repair

### 3.1 Why R1 Failed (`QNT101` Conflicting Builtin Names)
In `revision01` ([q11NominalIdentity.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision01-Failed-QNT101-field-item/afterimages/q11NominalIdentity.qnt), `12,029` B, SHA256 `b7c8a31cce17c9d342b483329797ea18ad9b561b148c1ae86a2a8f1ace445bb7`), operation `01-newNominalIdentityFrontend` failed name resolution (`actual_exit: 1`, `2.068089982960373` s; [stderr.bin](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision01-Failed-QNT101-field-item/prospective-ordinary-QA/01-newNominalIdentityFrontend/stderr.bin) `816` B, SHA256 `67805ecf6352d04d56e60fcac8d4276de2a819e52337fa9ff6b3a4c98fbf6fc4`) with two `QNT101` errors:
- Line 86: `val field = v.ctor_fields.nth(c.field_start + idx)` collided with `field` in `exports.builtinNames` ([names-base.js](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/readset/names-base.js) line 90).
- Line 169: `val item = v.pattern_items.nth(p.items.start + idx)` collided with `item` in `exports.builtinNames` ([names-base.js](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/readset/names-base.js) line 88).

### 3.2 Why R2 Failed (`QNT000` `Set.map` vs `List` Receiver)
In `revision02-local-name-collisions` ([q11NominalIdentity.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision02-local-name-collisions/afterimages/q11NominalIdentity.qnt), `12,092` B, SHA256 `d09716c4c95f68313e00b3d8e1ec26c07ee24aabceb1064201c163aa6ba8f439`), renaming `field` → `declaredField` and `item` → `sourcePatternItem` cleared name resolution, but `01-newNominalIdentityFrontend` failed typechecking (`actual_exit: 1`, `7.22205917397514` s; [RESULT.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision02-local-name-collisions/prospective-ordinary-QA/01-newNominalIdentityFrontend/RESULT.json) `1,117` B, SHA256 `5b988024d6c24342a5e294733a839a43b1daf54690d35685103acc993a8b923b`; [stderr.bin](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision02-local-name-collisions/prospective-ordinary-QA/01-newNominalIdentityFrontend/stderr.bin) `611` B, SHA256 `3d3988c7f8d1e9388e91782d63ec201545a2820fb93db904979acefcd712d6c5`):
```text
Error [QNT000]: Couldn't unify set and list
Trying to unify Set[_t1661] and List[int]
Trying to unify (Set[_t1661], (_t1661) => _t1662) => Set[_t1662] and (List[int], (int) => (C6Const(int) | C6Data(int))) => _t1663
  at .../q11NominalIdentity.qnt:101:5
  101:     q11Slice(v.constant_fields,s).map(i=>C6Const(i))
```
Inspection of the installed Quint `0.31.0` compiler ([types-builtinSignatures.js](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/readset/types-builtinSignatures.js) lines 3044, 3066–3080 and [builtin.qnt.txt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/readset/builtin.qnt.txt) lines 4766–4823) confirms that `map` has signature `(Set[a], (a) => b) => Set[b]` only. Quint `List[a]` has no built-in `.map` operator; the canonical idiom documented in `builtin.qnt.txt` (line 4808) is `l.foldl(List(), (l, e) => l.append(e))`.

### 3.3 Exact Verification of the 8 R2 → R3 Ordered `foldl`/`append` Replacements
In `revision03-ordered-list-folds`, [q11NominalIdentity.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/q11NominalIdentity.qnt) (`12,420` B, SHA256 `9d098cabab74868fd1dbcefd33940322ee4bc28f6eca26fdc07ce024c06973a3`) differs from R2 (`d09716c4...`) at **only** the 8 `List.map` sites recorded in [R2-TO-R3-INVERSE-PATCH.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/R2-TO-R3-INVERSE-PATCH.json) (`6,193` B, SHA256 `59dc21799dfef99cf30b7bc9f6d4a5896268c8bdbeae0f230e82aa8de18852b5`):

1. **`q11FieldBindings` (lines 14–18)**:
   - Receiver: `range(0, c.field_len) : List[int]`
   - Replacement: `range(0,c.field_len).foldl(List(),(mappedRows,i)=>{ val rowId=c.field_start+i; val raw=v.ctor_fields.nth(rowId); mappedRows.append({row_id:rowId,ordinal:i,raw:raw,name:if(c.owning_type>=4) v.names.nth(raw.name_id) else List()}) })`
   - Return type: `List[C6FieldBinding]`
2. **`q11Bindings.types` (line 30)**:
   - Receiver: `range(0, v.types.length()) : List[int]`
   - Replacement: `range(0,v.types.length()).foldl(List(),(mappedRows,i)=>mappedRows.append(q11TypeBinding(v,i)))`
   - Return type: `List[C6TypeBinding]`
3. **`q11Bindings.constructors` (line 31)**:
   - Receiver: `range(0, v.constructors.length()) : List[int]`
   - Replacement: `range(0,v.constructors.length()).foldl(List(),(mappedRows,i)=>mappedRows.append(q11CtorBinding(v,i)))`
   - Return type: `List[C6CtorBinding]`
4. **`q11ConstChildren` (line 101)**:
   - Receiver: `q11Slice(v.constant_fields, s) : List[int]`
   - Replacement: `q11Slice(v.constant_fields,s).foldl(List(),(mappedRows,i)=>mappedRows.append(C6Const(i)))`
   - Return type: `List[C6ChildRef]`
5. **`q11Expr` `C6PEMatch` arms (line 197)**:
   - Receiver: `v.arms.slice(p.arms.start, p.arms.start + p.arms.len) : List[C6MatchArmRow]`
   - Replacement: `v.arms.slice(p.arms.start,p.arms.start+p.arms.len).foldl(List(),(mappedRows,a)=>mappedRows.append({pattern:q11Pattern(v,a.pattern,budget),body:a.body}))`
   - Return type: `List[C6BranchSpec]`
6. **`q11Program.functions` `parameters` (line 206)**:
   - Receiver: `params : List[C6ParamRow]`
   - Replacement: `params.foldl(List(),(mappedRows,p)=>mappedRows.append(p.binder))`
   - Return type: `List[int]`
7. **`q11Program.functions` `parameterTypes` (line 207)**:
   - Receiver: `params : List[C6ParamRow]`
   - Replacement: `params.foldl(List(),(mappedRows,p)=>mappedRows.append(p.sort))`
   - Return type: `List[int]`
8. **`q11Program.layouts` `fields` (line 212)**:
   - Receiver: `v.ctor_fields.slice(c.field_start, c.field_start + c.field_len) : List[C6CtorFieldRow]`
   - Replacement: `v.ctor_fields.slice(c.field_start,c.field_start+c.field_len).foldl(List(),(mappedRows,f)=>mappedRows.append(f.type_id))`
   - Return type: `List[int]`

### 3.4 Complete Installed Quint Built-In API Audit Across All 11 Modules
Verified against [types-builtinSignatures.js](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/readset/types-builtinSignatures.js), [effects-builtinSignatures.js](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/readset/effects-builtinSignatures.js), [runtime-builtins.js](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/readset/runtime-builtins.js), and [names-base.js](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/readset/names-base.js):

- **`List` Operators**:
  - `foldl`: `(List[a], b, (b, a) => b) => b` (`types-builtinSignatures.js` L3049); effect `(Read[r1], Read[r2], (Read[r1, r2], Read[r1, r2]) => Read[r3]) => Read[r1, r2, r3]` (`effects-builtinSignatures.js` L3355); runtime `applyFold('fwd', args[0].toList(), args[1], ...)` (`runtime-builtins.js` L4010–4012, L4251–4269) using `Array.from(iterable).reduce(...)` in strict left-to-right index order.
  - `append`: `(List[a], a) => List[a]` (L3050); runtime `args[0].toList().push(args[1])` (L3947–3949).
  - `concat`: `(List[a], List[a]) => List[a]` (L3051); runtime `args[0].toList().concat(args[1].toList())` (L3954–3956).
  - `length`: `(List[a]) => int` (L3054); runtime `fromBigInt(BigInt(args[0].toList().size))` (L3970–3972).
  - `nth`: `(List[a], int) => a` (L3055); runtime `args[0].toList().get(Number(args[1].toInt()))` (L3976–3983).
  - `head`: `(List[a]) => a` (L3052); `tail`: `(List[a]) => List[a]` (L3053).
  - `slice`: `(List[a], int, int) => List[a]` (L3058); runtime `args[0].toList().slice(start, end)` (L3995–4002).
  - `select`: `(List[a], (a) => bool) => List[a]` (L3059); runtime `args[0].toList().filter(...)` (L4004–4008).
  - `replaceAt`: `(List[a], int, a) => List[a]` (L3057); runtime `list.set(idx, args[2])` (L3986–3993).
  - `range`: `(int, int) => List[int]` (L3056); runtime ` [start, end)` list construction (L4018–4029).
- **`Map` Operators** (used in `nominalPublicSimulation.qnt`):
  - `keys`: `(a -> b) => Set[a]` (L3036); `get`: `(a -> b, a) => b` (L3035); `put`: `(a -> b, a, b) => a -> b` (L3038); `Map()`: `Set[(a, b)] -> (a -> b)` (L3037).
- **`Set` Operators** (used in `nominalPublicSimulation.qnt` and `q11NominalIdentityTest.qnt`):
  - `contains`: `(Set[a], a) => bool` (L3021); `union`: `(Set[a], Set[a]) => Set[a]` (L3024); `size`: `(Set[a]) => int` (L3029); `forall`: `(Set[a], (a) => bool) => bool` (L3019).
- **Builtin Name Collision Check**:
  - None of the local binders (`mappedRows`, `declaredField`, `sourcePatternItem`, `rowId`, `raw`, `params`, `arms`, etc.) in `q11NominalIdentity.qnt`, `q11NominalIdentityInputs.qnt`, `q11NominalIdentityExpected.qnt`, or `q11NominalIdentityTest.qnt` appear in the 103 reserved names of `exports.builtinNames` in `names-base.js`.

### 3.5 Actual R3 Frontend Pass Confirmation
Root executed `01-newNominalIdentityFrontend` once on the exact R3 source freeze ([ROOT-R3-ONE-FRONTEND-STRICT-READBACK.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/prospective-ordinary-QA/ROOT-R3-ONE-FRONTEND-STRICT-READBACK.json), `7,870` B, SHA256 `d599ee7d4d4799e699a41f914cf97aac6de4ef67278509e07302c0c6f00fcabb`; [RESULT.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/prospective-ordinary-QA/01-newNominalIdentityFrontend/RESULT.json), `1,106` B, SHA256 `17b7fb96948854d76eaaf2fdb3750c115222a51ebec43ced151123ea37f668ff`):
- `actual_exit: 0`, `elapsed_seconds: 13.077235662029125`, `controller_failure: null`.
- `stdout.bin` (`0` B, `e3b0c442...`) and `stderr.bin` (`0` B, `e3b0c442...`) retained in full.
- All 4,686 environment/source pins (`4,683` files + `3` symlinks in [CURRENT-4686-PIN-MANIFEST.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/root-manual-capture-data01/CURRENT-4686-PIN-MANIFEST.json), `1,135,262` B, SHA256 `39d5de90f5f86e3c43fe55a6305d1ba4ca8102b194b50d6277119c2b71ef4bfd`) matched before and after (`PRE-FILES.json` == `POST-FILES.json` == `d4945ccc...`, `PRE-SYMLINKS.json` == `POST-SYMLINKS.json` == `4d66b049...`).
- As required, this frontend pass establishes static parse, name resolution, typechecking, and effect checking of the 11 modules; it is not replayed and is not treated as proof of runtime semantics.

---

## 4. Semantic Review: Ordered Rows, Duplicates, Ownership, `source_order`, `Const`/`Data` Reads, and Residual Preservation

### 4.1 Ordered `List` Preservation and Duplicate Retention
Because every sequence projection in [q11NominalIdentity.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/q11NominalIdentity.qnt) uses `foldl(List(), (mappedRows, x) => mappedRows.append(...))` over `List` receivers (`range(0, n)` or `.slice(start, start + len)`), element order is strictly index-ordered and duplicates are never collapsed into a `Set`:
- **Duplicate field `type_id`s in `layouts`**:
  - `map_queue` constructor `0` (`Pair`, `field_start: 0, field_len: 2`) folds `ctor_fields[0..2]` into `fields: List(2, 2)` (two `Int` fields), matching [EXPECTED-COMPLETE-LOGICAL-RETURNS.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/independent-expected/EXPECTED-COMPLETE-LOGICAL-RETURNS.json) (`cb147037...`).
  - `map_queue` constructor `4` (`Receipt`, `field_start: 5, field_len: 3`) folds `ctor_fields[5..8]` into `fields: List(0, 0, 1)`.
  - `mirror_arbor` constructor `1` (`Branch`, `field_start: 1, field_len: 3`) folds `ctor_fields[1..4]` into `fields: List(3, 3, 2)`.
  - `mirror_arbor` constructor `3` (`Summary`, `field_start: 5, field_len: 3`) folds `ctor_fields[5..8]` into `fields: List(0, 0, 1)`.
- **Duplicate parameter sorts in `functions`**:
  - `map_queue` function `3` (`main`, `params: {start: 4, len: 2}`) folds into `parameters: List(6, 7)` and `parameterTypes: List(0, 0)`.
  - `mirror_arbor` function `3` (`main`, `params: {start: 4, len: 2}`) folds into `parameters: List(6, 7)` and `parameterTypes: List(0, 0)`.
- **Duplicate child type references in `C6CTCustom` / `q11DataChildren`**:
  - `mirror_arbor` type `3` (`C6PTCustom({head: 1, const_children: {start: 0, len: 1}, data_children: {start: 0, len: 2}})`) decodes `const_children` from `constant_fields[0..1]` (`List(C6Const(42))`) and `data_children` from `data_fields[0..2]` (`List(C6Data(1), C6Data(1))`), preserving both duplicate `C6Data(1)` children.

### 4.2 Constructor Ownership, Record vs. Variant Classification, and Field Names
In [q11NominalIdentity.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/q11NominalIdentity.qnt):
- `q11FieldBindings` (lines 12–19) sets `name: if (c.owning_type >= 4) v.names.nth(raw.name_id) else List()`.
- `q11CtorBinding` (lines 21–27) sets `kind: if (c.owning_type >= 4) C6NominalRecordCtor else C6NominalEnumVariantCtor`.
- Verified against [EXPECTED-map-queue-BINDINGS.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/independent-expected/EXPECTED-map-queue-BINDINGS.json) (`b18fa616...`) and [EXPECTED-mirror-arbor-BINDINGS.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/independent-expected/EXPECTED-mirror-arbor-BINDINGS.json) (`103c1b56...`):
  - For enum variant constructors (`owning_type == 3 < 4`: `Pair`, `Empty`, `Push` in `map_queue`; `Leaf`, `Branch`, `Mirror` in `mirror_arbor`), `kind` is `C6NominalEnumVariantCtor` and every field binding has `name: List()` (`[]`), while retaining `raw.name_id: 0` in `raw`.
  - For record constructors (`owning_type >= 4`: `Queue` (`owning_type: 4`, `head`/`tail`) and `Receipt` (`owning_type: 5`, `total`/`checksum`/`tag`) in `map_queue`; `Summary` (`owning_type: 4`, `mirrored`/`original`/`tag`) in `mirror_arbor`), `kind` is `C6NominalRecordCtor` and `name` resolves to the exact UTF-8 byte list from `v.names.nth(raw.name_id)`.
- `q11TypeBinding` (lines 20–21) classifies `type_id` `0, 1, 2` as `C6NominalBuiltinType`, `type_id == 3` as `C6NominalEnumType` (`constructors: List(0, 1, 2)`), and `type_id >= 4` as `C6NominalRecordType` (`constructors: List(3)` for `type_id == 4`, `List(4)` for `type_id == 5` in `map_queue`), matching `c.owning_type == typeId` via `select`.

### 4.3 `source_order` Preservation (Including Non-Monotonic Order)
- `q11CtorBinding` preserves `source_order: c.source_order` and `raw: c`.
- `q11Program` preserves `source_order: c.source_order` in every `C6LayoutSpec` of `layouts`.
- In `mirror_arbor`, constructor `0` (`Leaf`, `ctor_id: 0`) has `source_order: 2`, while constructor `1` (`Branch`, `ctor_id: 1`) has `source_order: 1`, and constructor `2` (`Mirror`, `ctor_id: 2`) has `source_order: 0`. Both `q11Bindings(q11MirrorArborPublicInput)` and `q11DecodeProgram(q11MirrorArborPublicInput).layouts` preserve `ctor_id` order `[0, 1, 2, 3]` with `source_order` values `[2, 1, 0, 0]`, without sorting or mutating `source_order`.

### 4.4 `Const` vs `Data` Child Slice Reads (`q11ConstChildren` & `q11DataChildren`)
- `q11ConstChildren(v, s)` (lines 99–102) slices `v.constant_fields` using `q11Slice(v.constant_fields, s)` and wraps each element `i` in `C6Const(i)`.
- `q11DataChildren(v, s)` (lines 103–106) slices `v.data_fields` using `q11Slice(v.data_fields, s)` and wraps each element `i` in `C6Data(i)`.
- In `q11TypeSurface` (lines 107–128):
  - `C6PTBag(s)` → `C6CTBag(q11DataChildren(v, s))`
  - `C6PTMap(s)` → `C6CTMap(q11DataChildren(v, s))`
  - `C6PTSet(s)` → `C6CTSet(q11DataChildren(v, s))`
  - `C6PTArray(s)` → `C6CTArray({len: s.len, children: q11DataChildren(v, s.children)})`
  - `C6PTMatrix(s)` → `C6CTMatrix({rows: s.rows, cols: s.cols, children: q11DataChildren(v, s.children)})`
  - `C6PTLedger(s)` → `C6CTLedger(q11DataChildren(v, s))`
  - `C6PTKernel(s)` → `C6CTKernel(q11DataChildren(v, s))`
  - `C6PTCustom(s)` → `C6CTCustom({head: s.head, const_children: q11ConstChildren(v, s.const_children), data_children: q11DataChildren(v, s.data_children)})`
- This matches the Rust `c6_physical_host_codecs.rs` representation (`TypeNodePayload` reading `constant_fields` for `const_children` and `data_fields` for `data_children`).

### 4.5 Complete 8-Vector `residual` Preservation
- `q11Program` (lines 213–215) populates `residual` on `C6DecodedProgram` with all 8 raw public-input vectors untouched:
  `{ strings: v.strings, symbols: v.symbols, types: v.types, constant_fields: v.constant_fields, data_fields: v.data_fields, constructors: v.constructors, ctor_fields: v.ctor_fields, metadata: v.metadata }`.

---

## 5. Unchanged Evaluator and Full Evidence Carrier Preservation

1. **Unchanged Evaluator (`nominalPublicSimulation.qnt`)**:
   - [nominalPublicSimulation.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/nominalPublicSimulation.qnt) (`16,527` B, SHA256 `4c992d650583677ba7d4c05c050e87be94bb54666926dc9a42e5a54a66466b7c`) is byte-identical across R1, R2, and R3.
   - `q11EvaluatePublicInput(v, args, fuel)` (lines 222–227 of `q11NominalIdentity.qnt`) decodes `program = q11DecodeProgram(v)` and invokes `nominalEvalProgram(program, args, fuel)` directly without altering the evaluator's step budget, pattern matching, constructor layout checks, environment binding, trace emission, diagnostics, or obligations.
2. **Full Evidence Carrier (`C6PublicNominalObservation`)**:
   - Carries `{ bindings: C6PublicNominalBindings, program: C6DecodedProgram, result: NominalEvalOutcome, clock: int }`.
   - `q11AssertObservationMatchesExpected` in [q11NominalIdentityTest.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/q11NominalIdentityTest.qnt) (lines 6–25) checks full record equality `actual == expected` **and** all 16 field projections (`bindings`, `bindings.types`, `bindings.constructors`, `program`, `program.residual`, `result.status`, `result.value`, `result.steps`, `result.trace`, `result.env`, `result.diagnostics`, `result.obligations`, `result.gasRemaining`, `clock`).

---

## 6. Eligibility of Proposed Five Tests (`02-fiveExactNominalIdentityTests`)

In [REMAINING-VALIDATION-PROPOSAL.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/REMAINING-VALIDATION-PROPOSAL.json) (`4,713` B, SHA256 `6f99de414d58119c4645cc79bb9a3c9d1c5ee95869e81a61722d4b9607901127`), a single unrun operation `02-fiveExactNominalIdentityTests` is proposed (`frontendReplayIncluded: false`):
- **Command**:
  `/home/charl/.nvm/versions/node/v25.6.1/bin/node /home/charl/.nvm/versions/node/v25.6.1/lib/node_modules/@informalsystems/quint/dist/src/cli.js test --backend typescript --max-samples 1 --seed 1 --match ^q11.*Test$ /home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/q11NominalIdentityTest.qnt`
- **5 Matched Tests in [q11NominalIdentityTest.qnt](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/afterimages/q11NominalIdentityTest.qnt)**:
  1. `q11MapQueueBindingsAndDecodeTest`: Verifies `q11Bindings(q11MapQueuePublicInput) == q11ExpectedMapQueueBindings`, `q11DecodeProgram(q11MapQueuePublicInput) == q11ExpectedMapQueueProgram`, constructor field names (`Pair` `[]`, `Queue` `head`/`tail`, `Receipt` `total`/`checksum`/`tag`), and `residual` equality.
  2. `q11MirrorArborBindingsAndDecodeTest`: Verifies `q11Bindings(q11MirrorArborPublicInput) == q11ExpectedMirrorArborBindings`, `q11DecodeProgram(q11MirrorArborPublicInput) == q11ExpectedMirrorArborProgram`, non-monotonic `source_order` (`Leaf` `2`, `Branch` `1`), `C6CTCustom` `const_children`/`data_children` (`[C6Const(42)]` and `[C6Data(1), C6Data(1)]`), and `Summary` field names (`mirrored`/`original`/`tag`).
  3. `q11MapQueuePublicEvaluationTest`: Verifies `q11EvaluatePublicInput(q11MapQueuePublicInput, List(NominalVInt(5), NominalVInt(8)), 64)` equals `q11ExpectedMapQueueObservation` (`NominalVRecord(4, [NominalVInt(26), NominalVInt(13), NominalVBool(true)])`, `steps: 31`, `gasRemaining: 33`, `clock: 41`).
  4. `q11MirrorArborPublicEvaluationTest`: Verifies `q11EvaluatePublicInput(q11MirrorArborPublicInput, List(NominalVInt(4), NominalVInt(9)), 64)` equals `q11ExpectedMirrorArborObservation` (`NominalVRecord(3, [NominalVInt(22), NominalVInt(17), NominalVBool(true)])`, `steps: 32`, `gasRemaining: 32`, `clock: 42`).
  5. `q11DistinctNominalIdentityPreservedTest`: Verifies that `map_queue` and `mirror_arbor` have distinct bindings, distinct decoded programs, distinct `C6CTCustom` type surfaces (`type_id == 3`), distinct constructor names (`Pair` vs `Leaf`), distinct record layouts (`Receipt` ctor 4 vs `Summary` ctor 3), and distinct evaluation outcomes (`value`, `steps`, `clock`, `trace`).

**Eligibility Conclusion**: `02-fiveExactNominalIdentityTests` is **eligible for separate Root selection**. `01-newNominalIdentityFrontend` has already passed on the exact R3 source freeze and must **not** be replayed.

---

## 7. Typed Findings (`0` Mandatory Source Defects; `4` Explicit Limitations / Premises)

### 7.1 Mandatory Source Defects
- **None (`0` defects)**.

### 7.2 Explicit Unproved Premises and Scope Limitations
1. **`LIMIT-01` (`UNRUN_FIVE_SEMANTIC_TESTS`)**:
   - `02-fiveExactNominalIdentityTests` has not yet been executed. The R3 frontend pass (`01-newNominalIdentityFrontend`, `exit 0`, `13.077s`) proves static syntax, name resolution, typechecking, and effect checking under Quint `0.31.0`, not runtime test execution.
2. **`LIMIT-02` (`EXPLICIT_PUBLIC_TABLE_PREMISE_NOT_BYTE_DECODER`)**:
   - `q11NominalIdentity.qnt` consumes pre-populated `C6PublicInputVectors` records rather than parsing raw `MLPK` container bytes, checking header magic/version/section offsets, or computing SHA-256 digests over raw binary tables; out-of-range slices fall back via `q11ValidSlice` to `List()` or default variants rather than raising binary parser errors.
3. **`LIMIT-03` (`LOGICAL_HELPER_CLOCK_NOT_RUST_CORECLOCK_OR_FINAL_HOST_IMAGE`)**:
   - `entry_clock` (`41` vs `42`) and `helper_work` are model-level logical projections on the two canonical fixtures, not a proof of Rust `CoreClock` equivalence, final Host `C6` image/rootref binding, Lean/Clean/K refinement, ZKIRv3 circuit synthesis, or Midnight stagenet native admission.
4. **`LIMIT-04` (`CALLER_SCRIPT_PATH_REBINDING_NOTE_FOR_ROOT_SELECTION`)**:
   - Body 70 in the review packet ([ROOT-MANUAL-FIVE-TESTS.py](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision02-local-name-collisions/root-manual-capture-data01/ROOT-MANUAL-FIVE-TESTS.py), `3,531` B, SHA256 `0888a734708f8bda046c8b6905bd231355df9cb0d779691e9ebceeb3e5afedc9`) is the historical R2 caller script referencing `revision02-local-name-collisions` paths and hashes. When Root selects `02-fiveExactNominalIdentityTests` for R3 from [REMAINING-VALIDATION-PROPOSAL.json](file:///home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/doctorwatson-stagenet-four-milestone-audit-20261020/quint-next-public-nominal-identity-source11/revision03-ordered-list-folds/REMAINING-VALIDATION-PROPOSAL.json) (`6f99de41...`), the R3 caller script must bind R3's directory (`revision03-ordered-list-folds`), R3's `CURRENT-4686-PIN-MANIFEST.json` (`39d5de90f5f86e3c43fe55a6305d1ba4ca8102b194b50d6277119c2b71ef4bfd`), and this R3 review (`quint-source11-revision03-ordered-folds-fresh-review02/REPORT.md`).
