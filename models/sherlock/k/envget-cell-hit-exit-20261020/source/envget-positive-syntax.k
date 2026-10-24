requires "c6-complete-helper-task-state-syntax.k"

module SHERLOCK-ENVGET-POSITIVE-SYNTAX
  imports SHERLOCK-C6-COMPLETE-HELPER-TASK-STATE-SYNTAX

  // Full immutable native JSON tree; Int and Bool remain distinct.
  // Positive premise: exact declared field sets and unique object keys.
  // This carrier is not an admission validator or an opaque hash substitute.
  syntax EGJSON ::= egNull() | egInteger(Int) | egBoolean(Bool)
                  | egText(String) | egArray(EGValues) | egObject(EGFields)
  syntax EGValues ::= egNoValues() | egValue(EGJSON, EGValues)
  syntax EGFields ::= egNoFields() | egField(String, EGJSON, EGFields)

  // admission(all4 fields, full23-field view) and run_id are immutable.
  // CSOwnedState contains all remaining22 image fields, including raw frames.
  syntax EGSnapshot ::= egSnapshot(EGJSON, EGJSON, CSOwnedState)
  syntax EGStepId ::= egStepId(EGJSON, Int)
  syntax EGRule ::= egEnvGetCellRule() | egEnvGetExitRule()
  syntax EGKind ::= egHelperVisit() | egHelperExit()
  syntax EGSiteOption ::= egSiteSome(CHSite)
  syntax EGHelperKindOption ::= egSomeEnvGet()
  syntax EGDisposition ::= egApplied()
  syntax EGPhaseOption ::= egNoPhase() | egTriPhase(CHTri)
  syntax EGResultTagOption ::= egNoResultTag() | egEnvFoundTag()
  syntax EGNatOption ::= egNatNone() | egNatSome(Int)
  syntax EGOccurrence ::= egOccurrenceLocator(Bool, Int, EGValues)
  syntax EGExhaustOption ::= egNoExhaustion()
  syntax EGOutcomeTag ::= egRunningTag()
  syntax EGBoundary ::= egNoBoundary()

  // Complete StepEventC declaration order: 28 fields, never a three-counter event.
  syntax EGEvent ::= egEvent(EGStepId, EGRule, EGKind, EGSiteOption,
    EGHelperKindOption, EGDisposition, Int, Int, Int, Int, CTAlloc, Int,
    Int, Int, Int, Int, EGPhaseOption, EGPhaseOption, CHIndex, CHIndex,
    EGResultTagOption, EGNatOption, Int, EGOccurrence, EGExhaustOption,
    EGOutcomeTag, CTOutcome, EGBoundary)
  syntax EGWindow ::= egWindow(EGSnapshot, EGSnapshot, EGSnapshot, EGEvent, EGEvent)
endmodule
