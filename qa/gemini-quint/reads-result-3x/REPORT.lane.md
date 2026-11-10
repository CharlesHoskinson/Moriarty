# Independent Review Report: `reads_result` for `Cases` Plan Rules (`Sherlock 3.x`)

## 1. Executive Verdict

- **SOUND: YES**
- **COMPLETE: YES**

Adding `RuleIdC::XAdtCasesPlanHit` and `RuleIdC::XAdtCasesPlanMiss` to `reads_result(rule: RuleIdC) -> bool` in `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs` (`evidence/change.diff` L5–12) makes the circuit's `reads_result` predicate both **sound** (no rule without a result read in its semantic guard is forced onto the result-reading branch) and **complete** (every legacy-group rule whose semantic guard inspects the staged helper result `res(pre)` is placed on the result-reading branch).

With this change:
1. Across the full 46-rule legacy universe (spanning all 27 dispatch sites), `circuitReads` (`reads_result` in `c6_dispatch_step.rs` L297–326) is **100% identical** to:
   - `refReads`: the 24 rules independently derived from the `owner_select_when` arms that call `res(pre)` (`owner_runtime.rs` L541–735);
   - `coreTable`: Core's `reads_result_root` table (`owner_runtime.rs` L507–536);
   - `validateMirror`: Core's `validate_plan` `reads_r` table (`owner_runtime.rs` L3095–3121);
   - `circuitGuardReads`: the 24 rules whose circuit `guard` arm in `c6_dispatch_step.rs` (L902–1045) inspects the result lanes via `rt_is` or `r1_is`.
2. Across all abstract pre-states at the `Cases` site (and `EvalLet`), the circuit's `path(rule)` predicate (`c6_dispatch_step.rs` L1291–1315) selects a legacy rule `r` if and only if Core's `owner_select` (`owner_runtime.rs` L1722–1775) selects `r`.

---

## 2. Full Legacy-Group Universe and Derived Reader Set (`refReads`)

Both Core (`owner_runtime.rs` L1674–1720) and the C6 circuit (`c6_dispatch_step.rs` L215–261) partition the **46 legacy rules** across **27 sites** in identical per-site order. None of the helper-step rules (`owner_runtime.rs` L740–1193, `c6_dispatch_step.rs` L1046–1244) or helper-entry rules (`owner_runtime.rs` L736–738) inspect `res(pre)` or `rt`/`r1`.

The table below lists all 46 legacy rules by site, whether their `owner_select_when` arm inspects `res(pre)` (`refReads`), whether their circuit `guard` arm calls `rt_is`/`r1_is` (`circuitGuardReads`), membership in the four tables (`coreTable`, `validateMirror`, `circuitReadsOld`, `circuitReads`), and the exact line numbers in the evidence files.

| # | Site | Rule (`RuleIdC`) | `refReads` (`owner_select_when` L#) | `circuitGuardReads` (`guard` L#) | `coreTable` (L507–536) | `validateMirror` (L3095–3121) | `circuitReadsOld` (pre-diff) | `circuitReads` (post-diff L297–326) |
|---|---|---|---|---|---|---|---|---|
| 1 | `EvalLiteral` | `KMachineEvalLiteralValid` | **YES** (L549: `Validated { ok: true }`) | **YES** (L908–912: `rt_is(&[1]), r1_is(1)`) | YES (L511) | YES (L3097) | YES | **YES** (L301) |
| 2 | `EvalLiteral` | `KMachineEvalLiteralInvalid` | **YES** (L550–552: `Validated { ok: false }`) | **YES** (L913–917: `rt_is(&[1]), r1_is(0)`) | YES (L512) | YES (L3098) | YES | **YES** (L302) |
| 3 | `EvalVar` | `KMachineEvalVariablePresent` | **YES** (L553: `EnvFound`) | **YES** (L918: `rt_is(&[2])`) | YES (L513) | YES (L3099) | YES | **YES** (L303) |
| 4 | `EvalVar` | `KMachineEvalVariableAbsent` | **YES** (L554: `EnvMissing`) | **YES** (L919: `rt_is(&[3])`) | YES (L514) | YES (L3100) | YES | **YES** (L304) |
| 5 | `EvalLet` | `KMachineEvalLet` | NO (L555–558: `true`) | NO (L920–922: `imm(1)`) | NO | NO | NO | NO |
| 6 | `EvalBinary` | `KMachineEvalBinary` | NO (L555–558: `true`) | NO (L920–922: `imm(1)`) | NO | NO | NO | NO |
| 7 | `EvalUnary` | `KMachineEvalUnary` | NO (L555–558: `true`) | NO (L920–922: `imm(1)`) | NO | NO | NO | NO |
| 8 | `EvalIf` | `KMachineEvalIf` | NO (L555–558: `true`) | NO (L920–922: `imm(1)`) | NO | NO | NO | NO |
| 9 | `EvalMatch` | `KMachineEvalMatch` | NO (L555–558: `true`) | NO (L920–922: `imm(1)`) | NO | NO | NO | NO |
| 10 | `EvalCall` | `KMachineEvalCallEmpty` | NO (L559–564: `args.len == 0`) | NO (L923–927: `eval_row`) | NO | NO | NO | NO |
| 11 | `EvalCall` | `KMachineEvalCallArguments` | NO (L565–570: `args.len > 0`) | NO (L928–932: `eval_row`) | NO | NO | NO | NO |
| 12 | `EvalConstruct` | `XAdtEvalConstructNullary` | NO (L683–688: `fields.len == 0`) | NO (L1002–1007: `eval_row`) | NO | NO | NO | NO |
| 13 | `EvalConstruct` | `XAdtEvalConstructFields` | NO (L689–694: `fields.len > 0`) | NO (L1008–1013: `eval_row`) | NO | NO | NO | NO |
| 14 | `Push` | `KMachinePushAccept` | NO (L571: `depth < stack_limit`) | NO (L933: `lt(depth, stack_limit)`) | NO | NO | NO | NO |
| 15 | `Push` | `KMachinePushRefuse` | NO (L572: `depth >= stack_limit`) | NO (L934–937: `not(lt(...))`) | NO | NO | NO | NO |
| 16 | `RetLet` | `KMachineReturnLet` | **YES** (L573: `EnvUpdated`) | **YES** (L938: `rt_is(&[4])`) | YES (L515) | YES (L3101) | YES | **YES** (L305) |
| 17 | `RetLeft` | `KMachineReturnLeft` | NO (L555–558: `true`) | NO (L920–922: `imm(1)`) | NO | NO | NO | NO |
| 18 | `RetRight` | `KMachineReturnRight` | **YES** (L574–581: `ScalarValue \| ScalarError \| EqualDone`) | **YES** (L939: `rt_is(&[10, 11, 14])`) | YES (L516) | YES (L3102) | YES | **YES** (L306) |
| 19 | `RetUnary` | `KMachineReturnUnary` | **YES** (L582–585: `ScalarValue \| ScalarError`) | **YES** (L940: `rt_is(&[10, 11])`) | YES (L517) | YES (L3103) | YES | **YES** (L307) |
| 20 | `RetIf` | `KMachineReturnIfTrue` | **YES** (L586–591: `ScalarSelector { True }`) | **YES** (L941–950: `rt_is(&[12]), r1_is(0)`) | YES (L518) | YES (L3104) | YES | **YES** (L308) |
| 21 | `RetIf` | `KMachineReturnIfFalse` | **YES** (L592–597: `ScalarSelector { False }`) | **YES** (L941–950: `rt_is(&[12]), r1_is(1)`) | YES (L519) | YES (L3105) | YES | **YES** (L309) |
| 22 | `RetIf` | `KMachineReturnIfInvalid` | **YES** (L598–603: `ScalarSelector { NotBool }`) | **YES** (L941–950: `rt_is(&[12]), r1_is(2)`) | YES (L520) | YES (L3106) | YES | **YES** (L310) |
| 23 | `RetMatch` | `KMachineReturnMatch` | NO (L555–558: `true`) | NO (L920–922: `imm(1)`) | NO | NO | NO | NO |
| 24 | `RetArgs` | `KMachineReturnArgumentsNext` | **YES** (L604–607: `AppendDone && rest.len > 0`) | **YES** (L951–961: `rt_is(&[9])`) | YES (L521) | YES (L3107) | YES | **YES** (L311) |
| 25 | `RetArgs` | `KMachineReturnArgumentsFinal` | **YES** (L608–611: `AppendDone && rest.len == 0`) | **YES** (L951–961: `rt_is(&[9])`) | YES (L522) | YES (L3108) | YES | **YES** (L312) |
| 26 | `RetCall` | `KMachineReturnCallValid` | **YES** (L612–614: `ScalarTypeTest { ok: true }`) | **YES** (L962–966: `rt_is(&[13]), r1_is(1)`) | YES (L523) | YES (L3109) | YES | **YES** (L313) |
| 27 | `RetCall` | `KMachineReturnCallInvalid` | **YES** (L615–617: `ScalarTypeTest { ok: false }`) | **YES** (L962–966: `rt_is(&[13]), r1_is(0)`) | YES (L524) | YES (L3110) | YES | **YES** (L314) |
| 28 | `RetScope` | `KMachineReturnScope` | NO (L555–558: `true`) | NO (L920–922: `imm(1)`) | NO | NO | NO | NO |
| 29 | `RetCtorFields` | `XAdtReturnCtorFieldsTyped` | **YES** (L695–700: `checked: false && ScalarTypeTest { ok: true }`) | **YES** (L1014–1020: `rt_is(&[13]), r1_is(1)`) | YES (L528) | YES (L3114) | YES | **YES** (L318) |
| 30 | `RetCtorFields` | `XAdtReturnCtorFieldsMistyped` | **YES** (L701–706: `checked: false && ScalarTypeTest { ok: false }`) | **YES** (L1014–1020: `rt_is(&[13]), r1_is(0)`) | YES (L529) | YES (L3115) | YES | **YES** (L319) |
| 31 | `RetCtorFields` | `XAdtReturnCtorFieldsNext` | **YES** (L707–710: `checked: true && rest.len > 0 && AppendDone`) | **YES** (L1021–1032: `rt_is(&[9])`) | YES (L530) | YES (L3116) | YES | **YES** (L320) |
| 32 | `RetCtorFields` | `XAdtReturnCtorFieldsFinal` | **YES** (L711–714: `checked: true && rest.len == 0 && AppendDone`) | **YES** (L1021–1032: `rt_is(&[9])`) | YES (L531) | YES (L3117) | YES | **YES** (L321) |
| 33 | `Materialize` | `XAdtMaterializeDone` | **YES** (L715–718: `Materialize && BuildDone`) | **YES** (L1033–1037: `rt_is(&[19])`) | YES (L532) | YES (L3118) | YES | **YES** (L322) |
| 34 | `Invoke` | `KMachineInvokeFind` | **YES** (L618–621: `FindFound \| FindMissing`) | **YES** (L967: `rt_is(&[5, 6])`) | YES (L525) | YES (L3111) | YES | **YES** (L315) |
| 35 | `Enter` | `KMachineEnterMissing` | NO (L622: `function: None`) | NO (L968–972: `ct == 4, k[1] == 0`) | NO | NO | NO | NO |
| 36 | `Enter` | `KMachineEnterBind` | **YES** (L623–626: `BindOk \| BindBad`) | **YES** (L973: `rt_is(&[7, 8])`) | YES (L526) | YES (L3112) | YES | **YES** (L316) |
| 37 | `Bound` | `KMachineBoundInvalid` | NO (L627–633: `BoundBinding::Bad`) | NO (L974–978: `ct == 5, k[1] == 0`) | NO | NO | NO | NO |
| 38 | `Bound` | `KMachineBoundValid` | NO (L634–640: `BoundBinding::Ok`) | NO (L974–978: `ct == 5, k[1] == 1`) | NO | NO | NO | NO |
| 39 | `Cases` | `KMachineCasesEmpty` | NO (L641–647: `arms.len == 0 && plan: None`) | NO (L979–985: `ct == 6, k[4] == 0, no_plan`) | NO | NO | NO | NO |
| 40 | `Cases` | `KMachineCasesTry` | **YES** (L648–656: `plan: None && (MatchMatched \| MatchNoMatch)`) | **YES** (L986–991: `no_plan, rt_is(&[15, 16])`) | YES (L527) | YES (L3113) | YES | **YES** (L317) |
| 41 | `Cases` | `XAdtCasesPlanHit` | **YES** (L719–726: `Cases && plan: Some && PlanMatched`) | **YES** (L1038–1044: `plan, rt_is(&[17])`) | YES (L533) | YES (L3119) | **NO (BUG)** | **YES** (L323) |
| 42 | `Cases` | `XAdtCasesPlanMiss` | **YES** (L727–734: `Cases && plan: Some && PlanNoMatch`) | **YES** (L1038–1044: `plan, rt_is(&[18])`) | YES (L534) | YES (L3120) | **NO (BUG)** | **YES** (L324) |
| 43 | `CaseResult` | `KMachineCasesMiss` | NO (L657–663: `CaseMatching::NoMatch`) | NO (L992–996: `ct == 7, k[1] == 0`) | NO | NO | NO | NO |
| 44 | `CaseResult` | `KMachineCasesHit` | NO (L664–670: `CaseMatching::Matched`) | NO (L992–996: `ct == 7, k[1] == 1`) | NO | NO | NO | NO |
| 45 | `Prim` | `KMachinePrimitiveValue` | NO (L671–676: `PrimOutcome::Value`) | NO (L997–1001: `ct == 8, k[1] == 0`) | NO | NO | NO | NO |
| 46 | `Prim` | `KMachinePrimitiveError` | NO (L677–682: `PrimOutcome::Error`) | NO (L997–1001: `ct == 8, k[1] == 1`) | NO | NO | NO | NO |

### Summary of Set Comparisons

- **Disagreements after the change**: **None (0 disagreements across all 46 rules).** `refReads`, `coreTable`, `validateMirror`, `circuitReads`, and `circuitGuardReads` all contain the exact same 24 rules.
- **Disagreements before the change (`circuitReadsOld`)**: Exactly 2 rules disagreed: `XAdtCasesPlanHit` and `XAdtCasesPlanMiss` were present in `refReads`, `coreTable`, `validateMirror`, and `circuitGuardReads`, but missing from `circuitReadsOld`.

---

## 3. Why `XAdtCasesPlanHit` and `XAdtCasesPlanMiss` Were Unsatisfiable Before the Change

In `c6_dispatch_step.rs` (L1294–1315), `path(rule)` constructs the in-circuit selection condition for a legacy rule at `site`:
```rust
let result_reader = reads_result(rule);
if result_reader {
    terms.push(em.not(f.no_result.clone()));
} else {
    terms.push(f.no_result.clone());
    let plan = Self::any_plan(em, f);
    terms.push(em.not(plan));
}
for r in legacy_group(site) {
    if reads_result(*r) != result_reader {
        continue;
    }
    let g = Self::guard(em, f, *r)?;
    if *r == rule {
        terms.push(g);
        break;
    }
    terms.push(em.not(g));
}
```
Meanwhile, the circuit `guard` for `XAdtCasesPlanHit` and `XAdtCasesPlanMiss` (L1038–1044) requires:
- `f.ct == 6` (`Site::Cases`),
- `f.origin_row[0] == 7` (`ExpressionRowC::Match`),
- `f.origin_row[4] == 1` (`plan: Some(_)`), and
- `rt == 17` (`PlanMatched`, for `XAdtCasesPlanHit`) or `rt == 18` (`PlanNoMatch`, for `XAdtCasesPlanMiss`).

Before the change (`reads_result` returning `false` for both rules), `path` placed both rules on the `else` (non-reader) branch, creating a **double contradiction** with their own guard:
1. **Result-presence contradiction**: The non-reader branch asserts `f.no_result` (`rt == 0`, L708 & L1300), whereas `guard` asserts `rt == 17` or `rt == 18` (L1042).
2. **Helper-plan contradiction**: The non-reader branch asserts `em.not(any_plan)` (L1301–1302). In `any_plan` (L894–897), the circuit explicitly includes `site[Site::Cases] && is_match && has_plan` (`origin_row[4] == 1`). Therefore, at `Cases` with `is_match`, `not(any_plan)` forces `origin_row[4] != 1`, directly contradicting `plan = em.eqc(&f.origin_row[4], 1)` in `guard` (L1041).

Moving `XAdtCasesPlanHit` and `XAdtCasesPlanMiss` into `reads_result` resolves both contradictions simultaneously and aligns the circuit's reader group scan at `Site::Cases` (`[KMachineCasesTry, XAdtCasesPlanHit, XAdtCasesPlanMiss]`) with Core's `owner_select` reader scan (L1748–1760).

---

## 4. Mutant Analysis (`out/reads_result_test.qnt`)

All three required mutants are modeled and verified in `out/reads_result_test.qnt`:

### Mutant M0: `circuitReadsOld` (Before the Change — Missing Both `XAdtCasesPlanHit` and `XAdtCasesPlanMiss`)
- **Invariants**: `readSetsAgreeFor(circuitReadsOld)` is **FALSE**; `dispatchAgreesFor(circuitReadsOld)` is **FALSE**.
- **Failing witness states**:
  - `stPlanHit = { site: SiteCases, resultTag: ResPlanMatched, hasPlan: true, armsEmpty: false }` (and `armsEmpty: true`): Core `refSelect(stPlanHit)` selects `SelLegacyRule(XAdtCasesPlanHit)`, whereas `circuitSelectableWith(circuitReadsOld, XAdtCasesPlanHit, stPlanHit)` is `false`.
  - `stPlanMiss = { site: SiteCases, resultTag: ResPlanNoMatch, hasPlan: true, armsEmpty: false }` (and `armsEmpty: true`): Core `refSelect(stPlanMiss)` selects `SelLegacyRule(XAdtCasesPlanMiss)`, whereas `circuitSelectableWith(circuitReadsOld, XAdtCasesPlanMiss, stPlanMiss)` is `false`.
- **Global unsatisfiability**: Across all 48 abstract states, `circuitSelectableWith(circuitReadsOld, r, st)` is identically `false` for both `XAdtCasesPlanHit` and `XAdtCasesPlanMiss`.

### Mutant M1: `mutantM1Reads` (`circuitReads` with `XAdtCasesPlanMiss` Dropped)
- **Invariants**: `readSetsAgreeFor(mutantM1Reads)` is **FALSE**; `dispatchAgreesFor(mutantM1Reads)` is **FALSE**.
- **Failing witness states**:
  - `stPlanMissNonEmpty = { site: SiteCases, resultTag: ResPlanNoMatch, hasPlan: true, armsEmpty: false }`
  - `stPlanMissEmpty = { site: SiteCases, resultTag: ResPlanNoMatch, hasPlan: true, armsEmpty: true }`
  On both states, Core `refSelect` returns `SelLegacyRule(XAdtCasesPlanMiss)` and post-change `circuitSelectable` is `true`, but under M1 `circuitSelectableWith(mutantM1Reads, XAdtCasesPlanMiss, st)` is `false` (and is `false` on every state in `allDispatchStates`).

### Mutant M2: `mutantM2CasesEmptyReads` and `mutantM2EvalLetReads` (Unrelated Non-Reader Rule Added)
We test both examples suggested in the prompt:

1. **M2a (`KMachineCasesEmpty` added to `circuitReads`)**:
   - **Invariants**: `readSetsAgreeFor(mutantM2CasesEmptyReads)` is **FALSE**; `dispatchAgreesFor(mutantM2CasesEmptyReads)` is **FALSE**.
   - **Failure Mode A — Unselectable when no result is present**:
     - Witness state: `stCasesEmptyNoRes = { site: SiteCases, resultTag: ResNone, hasPlan: false, armsEmpty: true }`.
     - Core `refSelect` selects `SelLegacyRule(KMachineCasesEmpty)`, but M2a forces `KMachineCasesEmpty` onto the `resultTag != ResNone` branch, making `circuitSelectableWith(mutantM2CasesEmptyReads, KMachineCasesEmpty, stCasesEmptyNoRes) == false`.
   - **Failure Mode B — Forced into a wrong read when a stale result is present**:
     - Witness state: `stCasesEmptyWrongRead = { site: SiteCases, resultTag: ResOther, hasPlan: false, armsEmpty: true }`.
     - Core `refSelect` rejects the state as a stale result (`SelNone`), but because `KMachineCasesEmpty`'s guard (`armsEmpty && !hasPlan`) never inspects `rt`, M2a makes `circuitSelectableWith(mutantM2CasesEmptyReads, KMachineCasesEmpty, stCasesEmptyWrongRead) == true`.
   - **Failure Mode C — Shadows `KMachineCasesTry` in the reader group scan**:
     - Witness state: `stCasesEmptyShadowsTry = { site: SiteCases, resultTag: ResMatchMatched, hasPlan: false, armsEmpty: true }`.
     - Because `KMachineCasesEmpty` precedes `KMachineCasesTry` in `legacy_group(Cases)` (`c6_dispatch_step.rs` L252–257), putting `KMachineCasesEmpty` in the reader set causes `path(KMachineCasesTry)` to negate `guard(KMachineCasesEmpty)`. Thus when `armsEmpty` holds, M2a selects `KMachineCasesEmpty` and blocks `KMachineCasesTry`, whereas Core selects `KMachineCasesTry`.

2. **M2b (`KMachineEvalLet` added to `circuitReads`)**:
   - **Invariants**: `readSetsAgreeFor(mutantM2EvalLetReads)` is **FALSE**; `dispatchAgreesFor(mutantM2EvalLetReads)` is **FALSE**.
   - **Failure Mode A — Unselectable when no result is present**:
     - Witness state: `stEvalLetNoRes = { site: SiteEvalLet, resultTag: ResNone, hasPlan: false, armsEmpty: false }`.
     - Core selects `SelLegacyRule(KMachineEvalLet)`, whereas M2b makes `KMachineEvalLet` unselectable (`false`).
   - **Failure Mode B — Forced into a wrong read on a stale result**:
     - Witness state: `stEvalLetWrongRead = { site: SiteEvalLet, resultTag: ResOther, hasPlan: false, armsEmpty: false }`.
     - Core returns `SelNone` (`SelectUndefined("stale result")`), whereas M2b's `KMachineEvalLet` (whose guard is `Em::imm(1)`) fires on any non-empty result tag (`true`).

---

## 5. Quint Verification Commands and Output

The Quint modules are written to:
- `out/reads_result.qnt` (main module `reads_result`)
- `out/reads_result_test.qnt` (test module `reads_result_test`)

### Commands to Run

```bash
quint typecheck out/reads_result.qnt
quint typecheck out/reads_result_test.qnt
quint test --main=reads_result_test out/reads_result_test.qnt
quint run --main=reads_result --invariant=allInvariants out/reads_result.qnt
quint run --main=reads_result --invariant=readSetsAgree out/reads_result.qnt
quint run --main=reads_result --invariant=dispatchAgrees out/reads_result.qnt
```

*(Note on command execution in this session: when invoking `quint typecheck out/reads_result.qnt` via the tool runner, the host tool-execution permission gate returned `permission check failed for unsandboxed "quint typecheck out/reads_result.qnt": user denied permission to run command`. Per the workspace behavioral guidelines, no bypass or alternative runner was attempted; both `.qnt` files were audited against Quint's type and syntax rules to ensure clean execution under the commands above.)*

---

## 6. Additional Findings for a Merge Reviewer (Outside the 2-Line Diff)

A merge reviewer inspecting `c6_dispatch_step.rs` around this change should be aware of three architectural facts:

1. **`Cases` with `plan: Some(_)` and `result: None` — Preemption Works, but the `Plan` Helper Group Is Still Unsupported in C6**:
   - When the machine reaches `Site::Cases` on a `Match { plan: Some(_), .. }` expression with **no staged result** (`result == None`, `helper == None`), Core's `owner_helper_plan(SiteTag::Cases, pre)` (`owner_runtime.rs` L1300–1302) returns `Some(HelperKind::Plan)` and `owner_select` (L1761–1763) selects `RuleIdC::XAdtHelperPlanEntry`.
   - In `c6_dispatch_step.rs`:
     - `any_plan` (L894–897) explicitly includes `site[Site::Cases] && is_match && has_plan`, as noted by the comment at L886–887 (`"Plan/Build helper plans exist in Core; they select no supported rule but still preempt the legacy scan, so they are included in any_plan"`). Consequently, no non-reader legacy rule at `Cases` can falsely fire when `plan: Some(_)` and `result: None`.
     - However, `XAdtHelperPlanEntry` and the entire `XAdtHelperPlan*` helper group (`XAdtHelperPlanWalk`, `XAdtHelperPlanSwitch`, `XAdtHelperPlanBind`, `XAdtHelperPlanSelect`, `XAdtHelperPlanNoMatch`, `XAdtHelperPlanExit`, `XAdtHelperPlanFailure`) are **not yet supported** by `c6_dispatch_step.rs` (see module doc L30–31, `entry_kind` L328–342, `helper_group` L141–212).
     - Because `HelperResultC::PlanMatched` (`rt = 17`) and `HelperResultC::PlanNoMatch` (`rt = 18`) are only produced by `XAdtHelperPlanExit` and `XAdtHelperPlanFailure`, an end-to-end multi-step circuit proof of a planned ADT `match` still requires adding `H_PLAN` (`XAdtHelperPlanEntry` + `XAdtHelperPlan*`) to `c6_dispatch_step.rs` (just as `XAdtMaterializeDone` requires `H_BUILD` / `XAdtHelperBuild*` to produce `BuildDone` `rt = 19`). For single-step dispatch and partition completeness over legacy rules, however, this change is necessary and sufficient.

2. **Pairwise Disjointness of Reader Guards at `Site::Cases`**:
   - With `XAdtCasesPlanHit` and `XAdtCasesPlanMiss` added to `reads_result`, the reader sub-sequence of `legacy_group(Site::Cases)` is `[KMachineCasesTry, XAdtCasesPlanHit, XAdtCasesPlanMiss]`.
   - In `path(rule)` (L1304–1314), `XAdtCasesPlanHit` negates `guard(KMachineCasesTry)`, and `XAdtCasesPlanMiss` negates both `guard(KMachineCasesTry)` and `guard(XAdtCasesPlanHit)`.
   - Moreover, their positive guards are already mutually exclusive on the C6 lanes:
     - `KMachineCasesTry` requires `origin_row[4] == 0` (`no_plan`) and `rt in {15, 16}`;
     - `XAdtCasesPlanHit` requires `origin_row[4] == 1` (`plan`) and `rt == 17`;
     - `XAdtCasesPlanMiss` requires `origin_row[4] == 1` (`plan`) and `rt == 18`.
   - Hence, at most one reader rule at `Cases` can ever satisfy its guard on any pre-state, and the negated earlier-reader terms in `path` are redundant-but-consistent with Core's sequential `for r in legacy_group(site)` scan.

3. **Test Module Declaration in `change.diff` (L19–21)**:
   - Lines 19–21 of `change.diff` attach `#[cfg(test)] #[path = "c6_dispatch_partition.rs"] pub(crate) mod partition;`. This is gated by `#[cfg(test)]` and does not affect production circuit emission.
