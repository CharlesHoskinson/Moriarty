# Verdict: reads_result for XAdtCasesPlanHit / XAdtCasesPlanMiss (3.x branch)

Change: branch `worktree-agent-acf90d07b6b60767c`, commits 5782173bd, 7fadf00c3, 79a3fad8c.
In `crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs`, `reads_result` now also
returns true for `XAdtCasesPlanHit` and `XAdtCasesPlanMiss`.

**Verdict: SOUND and COMPLETE.** The circuit read set is now exactly the reference read set.
No rule can skip a needed read, and no rule is forced into a wrong read. Nothing in this lane
blocks the merge of the 3.x branch.

## Evidence

1. Reference read set. The lead derived it from the Core `owner_select_when` arms
   (`crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs` L541-1203 on the branch)
   with a script that lists every arm that reads `res(pre)`. The result is 24 rules. No arm reads
   `pre.result()` directly. The derived set is equal to the Core `reads_result_root` table
   (L507-536) and to the `validate_plan` mirror (L3095-3121).
2. Circuit side. After the change, `reads_result` (L297-326) has the same 24 rules. The set of
   circuit `guard` arms that test the result lanes (`rt_is` or `r1_is`, L902-1045) is the same
   24 rules. Before the change the circuit list had 22 rules: the two Cases plan rules were missing.
3. Rule universe. Circuit `legacy_group` (L215-261) and Core `legacy_group` (L1674-1720) are the
   same 46 rules in the same order over 27 sites.
4. `crates/mori-core` does not change between `main` and the branch. The Core table already had
   both rules on `main` (owner_runtime.rs L533-534). The circuit change brings the circuit in line
   with Core.
5. Old behaviour (mutant M0 below). Before the change, both rules were on the non-reader path. That
   path requires `rt == 0` and `not any_plan`, but their guards require `rt in {17, 18}` and
   `origin_row[4] == 1`. Thus both rules were unsatisfiable, and a PlanMatched or PlanNoMatch result
   at the Cases site had no selectable circuit rule. This was a completeness bug. It did not make the
   circuit unsound.
6. Cases-site dispatch. The Quint model abstracts the pre-state to the site (Cases, EvalLet), the
   result tag, the plan flag and arms-empty: 48 states. The test checks every state with `forall`.
   In each state, the circuit `path` holds for a rule iff Core `owner_select` selects that rule.
   The reader guards at Cases (CasesTry: plan None with rt 15/16; PlanHit: plan Some with rt 17;
   PlanMiss: plan Some with rt 18) are mutually exclusive.

## Lane

- Gemini 4 Argon (`gemini-4-argon-high`) through `argon-run`, inside `heavy-run`. The lane used the
  quint-modeling and quint-lang skills in `~/.gemini/skills`. Conversation
  `5e4e13a4-6936-46be-9b7a-75bf0c831055`, 4 turns, `DONE`.
- Workspace `~/.cache/gemini-lanes/reads-result-3x/` with line-numbered extracts of the branch
  sources (evidence files are not committed; this directory has the prompt that lists them).
- Files: `lane-prompt.md` (prompt), `lane-raw.log` (raw stream-json log),
  `REPORT.lane.md` (lane report, unedited), `reads_result.qnt`, `reads_result_test.qnt`
  (lane model and tests, unedited by the lead).
- The lane could not run `quint` itself; the host refused the command ("user denied permission to
  run command"). The lane report therefore has no tool output. All results below are from the lead.
- The lane report is input, not evidence. The lead checked its claims against the sources (above)
  and re-ran every check (below).

## Lead re-run (2026-11-10, quint 0.32.0, in this directory)

```
$ quint typecheck reads_result.qnt
exit=0
$ quint typecheck reads_result_test.qnt
exit=0

$ quint test --main=reads_result_test reads_result_test.qnt
  reads_result_test
    ok readSetsAgreeTest passed 1 test(s)
    ok dispatchAgreesTest passed 1 test(s)
    ok mutantM0OldCircuitTest passed 1 test(s)
    ok mutantM1DropPlanMissTest passed 1 test(s)
    ok mutantM2AddUnrelatedRuleTest passed 1 test(s)

  5 passing (241ms)
exit=0

$ quint run --main=reads_result --invariant=allInvariants --max-samples=2000 --max-steps=20 reads_result.qnt
[ok] No violation found (5654ms at 354 traces/second).
exit=0
$ quint run --main=reads_result --invariant=readSetsAgree --max-samples=2000 --max-steps=20 --seed=0x1 reads_result.qnt
exit=0 (no violation)
$ quint run --main=reads_result --invariant=dispatchAgrees --max-samples=2000 --max-steps=20 --seed=0x1 reads_result.qnt
exit=0 (no violation)
```

The test module checks the mutants inside Quint: each mutant test asserts that `readSetsAgreeFor`
and `dispatchAgreesFor` are false for the mutant list, and names a witness state.

- M0 (old list, both rules missing): PlanHit and PlanMiss are unselectable in all 48 states.
- M1 (PlanMiss dropped): Core selects PlanMiss at (Cases, PlanNoMatch, plan, any arms), and the
  circuit cannot select it.
- M2a (`KMachineCasesEmpty` added): the rule cannot be selected when there is no result. It fires on
  a stale result that Core refuses. When arms are empty, it shadows `KMachineCasesTry`.
- M2b (`KMachineEvalLet` added): the rule cannot be selected when there is no result, and it fires on
  a stale result.

The lead also made mutated copies of `reads_result.qnt`. Each copy changes only the
`circuitReads` set (L469-494) with `sed`. The lead ran the plain invariants on each copy. All must
be violated:

```
$ quint run --main=reads_result --invariant=<inv> --max-samples=200 --max-steps=5 <mutant>.qnt
m0  (old 22-rule list)          readSetsAgree  exit=1 [violation] Found an issue
m0                              dispatchAgrees exit=1 [violation] Found an issue
m1  (drop XAdtCasesPlanMiss)    readSetsAgree  exit=1 [violation] Found an issue
m1                              dispatchAgrees exit=1 [violation] Found an issue
m2a (add KMachineCasesEmpty)    readSetsAgree  exit=1 [violation] Found an issue
m2a                             dispatchAgrees exit=1 [violation] Found an issue
m2b (add KMachineEvalLet)       readSetsAgree  exit=1 [violation] Found an issue
m2b                             dispatchAgrees exit=1 [violation] Found an issue
```

Transcription check: the lead extracted the rule names from the model and from the Rust sources and
compared them with `diff`. The model `circuitReads` is equal to the circuit `reads_result` (24 rules).
The script-derived reference set is equal to the circuit `reads_result`. The model rule universe is
equal to the circuit `legacy_group` (46 rules). There were no differences.

## Limits of this check

- The model abstracts the lanes. For the Cases and EvalLet rules, `refSelectWhen` and
  `circuitGuard` are copied by hand from the Rust guard arms. The model checks the reader/non-reader
  partition and the group-scan order. It does not check that each guard arm encodes its
  `select_when` correctly on the real lanes. The partition test on the branch
  (`c6_dispatch_partition.rs`, `testdata/dispatch/partition.json`) covers that. The lead did not
  run that test, because the heavy lock is busy.
- No cargo, build, test or proving was run.

## Notes for the merge reviewer (not blocking)

- At Cases with a plan and no result, Core selects `XAdtHelperPlanEntry`. The C6 dispatch does not
  support this rule (module doc, L30-31). In the circuit, `any_plan` (L888-899) includes the Cases
  plan term, so no non-reader rule can fire there. No reader can fire either, because no result is
  present. The circuit selects nothing in that state. This agrees with Core within the documented
  scope.
- The PlanMatched and PlanNoMatch results (rt 17/18) come only from the `XAdtHelperPlan*` group,
  which the C6 dispatch does not yet support. Thus the two fixed rules can only be reached end to end
  after Plan helper support is added. The same applies to `XAdtMaterializeDone` and the Build group.
