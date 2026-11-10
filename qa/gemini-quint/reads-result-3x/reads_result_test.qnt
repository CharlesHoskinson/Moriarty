// -*- mode: Bluespec; -*-
// Deterministic forall verification and mutant tests (M0, M1, M2) for `reads_result`.

module reads_result_test {
  import reads_result.* from "./reads_result"

  // =========================================================================
  // 1. Exhaustive Universe & Dispatch Agreement Tests
  // =========================================================================

  run readSetsAgreeTest = {
    all {
      assert(allSites.size() == 27),
      assert(allLegacyRules.size() == 46),
      assert(refReads.size() == 24),
      assert(coreTable.size() == 24),
      assert(validateMirror.size() == 24),
      assert(circuitReads.size() == 24),
      assert(circuitGuardReads.size() == 24),
      assert(circuitReadsOld.size() == 22),
      assert(legacyGroupsAgree),
      assert(readSetsAgree),
      assert(allLegacyRules.forall(r =>
        (circuitReads.contains(r) == refReads.contains(r))
          and (circuitReads.contains(r) == coreTable.contains(r))
          and (circuitReads.contains(r) == validateMirror.contains(r))
          and (circuitReads.contains(r) == circuitGuardReads.contains(r))
      ))
    }
  }

  run dispatchAgreesTest = {
    val stCasesEmpty = {
      site: SiteCases,
      resultTag: ResNone,
      hasPlan: false,
      armsEmpty: true
    }
    val stCasesMatchEntry = {
      site: SiteCases,
      resultTag: ResNone,
      hasPlan: false,
      armsEmpty: false
    }
    val stCasesPlanEntry = {
      site: SiteCases,
      resultTag: ResNone,
      hasPlan: true,
      armsEmpty: false
    }
    val stCasesTryHit = {
      site: SiteCases,
      resultTag: ResMatchMatched,
      hasPlan: false,
      armsEmpty: false
    }
    val stCasesPlanHit = {
      site: SiteCases,
      resultTag: ResPlanMatched,
      hasPlan: true,
      armsEmpty: false
    }
    val stCasesPlanMiss = {
      site: SiteCases,
      resultTag: ResPlanNoMatch,
      hasPlan: true,
      armsEmpty: false
    }
    all {
      // Exhaustive check across all 48 abstract pre-states and all site rules
      assert(dispatchAgreesFor(circuitReads)),
      assert(allDispatchStates.forall(st => dispatchAgreesOnState(circuitReads, st))),
      assert(allCasesStates.forall(st => dispatchAgreesOnState(circuitReads, st))),

      // Spot-check key Cases transitions
      assert(refSelect(stCasesEmpty) == SelLegacyRule(KMachineCasesEmpty)),
      assert(circuitSelectable(KMachineCasesEmpty, stCasesEmpty)),

      assert(refSelect(stCasesMatchEntry) == SelHelperEntry(PlanMatchHelper)),
      assert(dispatchSiteRules.forall(r => not(circuitSelectable(r, stCasesMatchEntry)))),

      assert(refSelect(stCasesPlanEntry) == SelHelperEntry(PlanPlanHelper)),
      assert(dispatchSiteRules.forall(r => not(circuitSelectable(r, stCasesPlanEntry)))),

      assert(refSelect(stCasesTryHit) == SelLegacyRule(KMachineCasesTry)),
      assert(circuitSelectable(KMachineCasesTry, stCasesTryHit)),

      assert(refSelect(stCasesPlanHit) == SelLegacyRule(XAdtCasesPlanHit)),
      assert(circuitSelectable(XAdtCasesPlanHit, stCasesPlanHit)),

      assert(refSelect(stCasesPlanMiss) == SelLegacyRule(XAdtCasesPlanMiss)),
      assert(circuitSelectable(XAdtCasesPlanMiss, stCasesPlanMiss))
    }
  }

  // =========================================================================
  // 2. Mutant M0: Old circuit list (`circuitReadsOld`, before the change)
  //    Must FAIL `readSetsAgreeFor` and `dispatchAgreesFor` on the Cases site
  //    with a Plan result (`ResPlanMatched` / `ResPlanNoMatch`).
  // =========================================================================

  run mutantM0OldCircuitTest = {
    val stPlanHit = {
      site: SiteCases,
      resultTag: ResPlanMatched,
      hasPlan: true,
      armsEmpty: false
    }
    val stPlanMiss = {
      site: SiteCases,
      resultTag: ResPlanNoMatch,
      hasPlan: true,
      armsEmpty: false
    }
    all {
      // Invariants MUST fail on circuitReadsOld
      assert(not(readSetsAgreeFor(circuitReadsOld))),
      assert(not(dispatchAgreesFor(circuitReadsOld))),

      // Specifically fails on Cases with a Plan result
      assert(not(dispatchAgreesOnState(circuitReadsOld, stPlanHit))),
      assert(not(dispatchAgreesOnState(circuitReadsOld, stPlanMiss))),

      // Core selects XAdtCasesPlanHit / XAdtCasesPlanMiss, but old circuit cannot
      assert(refSelect(stPlanHit) == SelLegacyRule(XAdtCasesPlanHit)),
      assert(not(circuitSelectableWith(circuitReadsOld, XAdtCasesPlanHit, stPlanHit))),
      assert(refSelect(stPlanMiss) == SelLegacyRule(XAdtCasesPlanMiss)),
      assert(not(circuitSelectableWith(circuitReadsOld, XAdtCasesPlanMiss, stPlanMiss))),

      // In fact, under circuitReadsOld both rules are unsatisfiable in EVERY state
      assert(allDispatchStates.forall(st =>
        not(circuitSelectableWith(circuitReadsOld, XAdtCasesPlanHit, st))
          and not(circuitSelectableWith(circuitReadsOld, XAdtCasesPlanMiss, st))
      ))
    }
  }

  // =========================================================================
  // 3. Mutant M1: `circuitReads` with `XAdtCasesPlanMiss` dropped
  //    Must FAIL both invariants; shows the state where `XAdtCasesPlanMiss`
  //    becomes unselectable.
  // =========================================================================

  pure val mutantM1Reads: Set[RuleId] =
    circuitReads.exclude(Set(XAdtCasesPlanMiss))

  run mutantM1DropPlanMissTest = {
    val stPlanMissNonEmpty = {
      site: SiteCases,
      resultTag: ResPlanNoMatch,
      hasPlan: true,
      armsEmpty: false
    }
    val stPlanMissEmpty = {
      site: SiteCases,
      resultTag: ResPlanNoMatch,
      hasPlan: true,
      armsEmpty: true
    }
    all {
      // Invariants MUST fail on M1
      assert(not(readSetsAgreeFor(mutantM1Reads))),
      assert(not(dispatchAgreesFor(mutantM1Reads))),

      // Witness states where XAdtCasesPlanMiss is selected by Core but unselectable in M1
      assert(refSelect(stPlanMissNonEmpty) == SelLegacyRule(XAdtCasesPlanMiss)),
      assert(circuitSelectableWith(circuitReads, XAdtCasesPlanMiss, stPlanMissNonEmpty)),
      assert(not(circuitSelectableWith(mutantM1Reads, XAdtCasesPlanMiss, stPlanMissNonEmpty))),
      assert(not(dispatchAgreesOnState(mutantM1Reads, stPlanMissNonEmpty))),

      assert(refSelect(stPlanMissEmpty) == SelLegacyRule(XAdtCasesPlanMiss)),
      assert(circuitSelectableWith(circuitReads, XAdtCasesPlanMiss, stPlanMissEmpty)),
      assert(not(circuitSelectableWith(mutantM1Reads, XAdtCasesPlanMiss, stPlanMissEmpty))),
      assert(not(dispatchAgreesOnState(mutantM1Reads, stPlanMissEmpty))),

      // Under M1, XAdtCasesPlanMiss is unselectable in ALL abstract states
      assert(allDispatchStates.forall(st =>
        not(circuitSelectableWith(mutantM1Reads, XAdtCasesPlanMiss, st))
      ))
    }
  }

  // =========================================================================
  // 4. Mutant M2: `circuitReads` with an unrelated non-reader rule added
  //    (`KMachineCasesEmpty` or `KMachineEvalLet`).
  //    Must FAIL both invariants; shows both:
  //    (a) the state where the rule becomes unselectable (when `resultTag == ResNone`), and
  //    (b) the state where the rule is forced into a wrong read (when `resultTag != ResNone`).
  // =========================================================================

  pure val mutantM2CasesEmptyReads: Set[RuleId] =
    circuitReads.union(Set(KMachineCasesEmpty))

  pure val mutantM2EvalLetReads: Set[RuleId] =
    circuitReads.union(Set(KMachineEvalLet))

  run mutantM2AddUnrelatedRuleTest = {
    // Witness states for KMachineCasesEmpty added to circuitReads:
    // (a) No result present, no plan, empty arms -> Core selects KMachineCasesEmpty,
    //     but M2 requires resultTag != ResNone, making KMachineCasesEmpty unselectable.
    val stCasesEmptyNoRes = {
      site: SiteCases,
      resultTag: ResNone,
      hasPlan: false,
      armsEmpty: true
    }
    // (b) Stale/unrelated result present, no plan, empty arms -> Core rejects (SelNone),
    //     but M2 selects KMachineCasesEmpty because its guard never checks `rt`.
    val stCasesEmptyWrongRead = {
      site: SiteCases,
      resultTag: ResOther,
      hasPlan: false,
      armsEmpty: true
    }
    // (c) Valid Match result present with empty arms -> Core selects KMachineCasesTry,
    //     but M2 puts KMachineCasesEmpty first in the reader scan of legacy_group(Cases),
    //     selecting KMachineCasesEmpty and blocking KMachineCasesTry!
    val stCasesEmptyShadowsTry = {
      site: SiteCases,
      resultTag: ResMatchMatched,
      hasPlan: false,
      armsEmpty: true
    }

    // Witness states for KMachineEvalLet added to circuitReads:
    // (a) No result present at EvalLet -> Core selects KMachineEvalLet,
    //     but M2 makes KMachineEvalLet unselectable.
    val stEvalLetNoRes = {
      site: SiteEvalLet,
      resultTag: ResNone,
      hasPlan: false,
      armsEmpty: false
    }
    // (b) Result present at EvalLet -> Core rejects as stale result (SelNone),
    //     but M2 forces KMachineEvalLet to fire on a present result.
    val stEvalLetWrongRead = {
      site: SiteEvalLet,
      resultTag: ResOther,
      hasPlan: false,
      armsEmpty: false
    }

    all {
      // M2a (KMachineCasesEmpty added) MUST fail both invariants
      assert(not(readSetsAgreeFor(mutantM2CasesEmptyReads))),
      assert(not(dispatchAgreesFor(mutantM2CasesEmptyReads))),

      // Witness (a): KMachineCasesEmpty becomes unselectable when resultTag == ResNone
      assert(refSelect(stCasesEmptyNoRes) == SelLegacyRule(KMachineCasesEmpty)),
      assert(circuitSelectableWith(circuitReads, KMachineCasesEmpty, stCasesEmptyNoRes)),
      assert(not(circuitSelectableWith(mutantM2CasesEmptyReads, KMachineCasesEmpty, stCasesEmptyNoRes))),
      assert(not(dispatchAgreesOnState(mutantM2CasesEmptyReads, stCasesEmptyNoRes))),

      // Witness (b): KMachineCasesEmpty is forced into a wrong read when resultTag == ResOther
      assert(refSelect(stCasesEmptyWrongRead) == SelNone),
      assert(not(circuitSelectableWith(circuitReads, KMachineCasesEmpty, stCasesEmptyWrongRead))),
      assert(circuitSelectableWith(mutantM2CasesEmptyReads, KMachineCasesEmpty, stCasesEmptyWrongRead)),
      assert(not(dispatchAgreesOnState(mutantM2CasesEmptyReads, stCasesEmptyWrongRead))),

      // Witness (c): KMachineCasesEmpty shadows KMachineCasesTry when armsEmpty == true and resultTag == ResMatchMatched
      assert(refSelect(stCasesEmptyShadowsTry) == SelLegacyRule(KMachineCasesTry)),
      assert(circuitSelectableWith(mutantM2CasesEmptyReads, KMachineCasesEmpty, stCasesEmptyShadowsTry)),
      assert(not(circuitSelectableWith(mutantM2CasesEmptyReads, KMachineCasesTry, stCasesEmptyShadowsTry))),
      assert(not(dispatchAgreesOnState(mutantM2CasesEmptyReads, stCasesEmptyShadowsTry))),

      // M2b (KMachineEvalLet added) MUST fail both invariants
      assert(not(readSetsAgreeFor(mutantM2EvalLetReads))),
      assert(not(dispatchAgreesFor(mutantM2EvalLetReads))),

      // Witness (a): KMachineEvalLet becomes unselectable when resultTag == ResNone
      assert(refSelect(stEvalLetNoRes) == SelLegacyRule(KMachineEvalLet)),
      assert(circuitSelectableWith(circuitReads, KMachineEvalLet, stEvalLetNoRes)),
      assert(not(circuitSelectableWith(mutantM2EvalLetReads, KMachineEvalLet, stEvalLetNoRes))),
      assert(not(dispatchAgreesOnState(mutantM2EvalLetReads, stEvalLetNoRes))),

      // Witness (b): KMachineEvalLet is forced into a wrong read when resultTag != ResNone
      assert(refSelect(stEvalLetWrongRead) == SelNone),
      assert(not(circuitSelectableWith(circuitReads, KMachineEvalLet, stEvalLetWrongRead))),
      assert(circuitSelectableWith(mutantM2EvalLetReads, KMachineEvalLet, stEvalLetWrongRead)),
      assert(not(dispatchAgreesOnState(mutantM2EvalLetReads, stEvalLetWrongRead)))
    }
  }

}
