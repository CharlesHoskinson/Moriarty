requires "c6-helper-syntax.k"
requires "c6-owned-data-syntax.k"

module SHERLOCK-C6-COMPLETE-HELPER-TASK-STATE-SYNTAX
  imports SHERLOCK-C6-HELPER-RECORD-SYNTAX
  imports SHERLOCK-C6-OWNED-DATA-SYNTAX

  // DATA only: no function, rule, alias, evaluator or observation producer.
  // csOwnedState arg0 stores root16 plus raw frames in ctRoot arg16.
  syntax CSVisit ::= csUnvisited() [klabel(csUnvisited)]
  syntax CSVisit ::= csFields() [klabel(csFields)]
  syntax CSTask ::= csValidateTask(CTRef, CSVisit, Int) [klabel(csValidateTask)]
  syntax CSTask ::= csEqualTask(CTRef, CTRef, CSVisit, Int) [klabel(csEqualTask)]
  syntax CSTask ::= csPairTask(Int, Int, CTRef) [klabel(csPairTask)]
  syntax CSTasks ::= csNoTasks() [klabel(csNoTasks)]
  syntax CSTasks ::= csTask(CSTask, CSTasks) [klabel(csTask)]
  syntax CSHelperTables ::= csHelperTables(CHHelperOption, CHResultOption, CSTasks) [klabel(csHelperTables)]
  syntax CSCaps ::= csCaps(Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int, Int) [klabel(csCaps)]
  syntax CSLimits ::= csLimits(Int) [klabel(csLimits)]
  syntax CSContext ::= csContext(CSCaps, Bool, CSLimits) [klabel(csContext)]
  syntax CSOwnedState ::= csOwnedState(CTRoot, DPTables, CSHelperTables) [klabel(csOwnedState)]
  syntax CSStrings ::= csNoStrings() [klabel(csNoStrings)]
  syntax CSStrings ::= csString(String, CSStrings) [klabel(csString)]
  syntax CSCaseIds ::= csCaseIds(CSStrings, CSStrings, CSStrings, CSStrings, CSStrings) [klabel(csCaseIds)]
  syntax CSHelperRecord ::= csHelperRecord(CTLanes, CHHelperOption) [klabel(csHelperRecord)]
  syntax CSHelperRecords ::= csNoHelperRecords() [klabel(csNoHelperRecords)]
  syntax CSHelperRecords ::= csHelperRecordRow(CSHelperRecord, CSHelperRecords) [klabel(csHelperRecordRow)]
  syntax CSResultRecord ::= csResultRecord(CTLanes, CHResultOption) [klabel(csResultRecord)]
  syntax CSResultRecords ::= csNoResultRecords() [klabel(csNoResultRecords)]
  syntax CSResultRecords ::= csResultRecordRow(CSResultRecord, CSResultRecords) [klabel(csResultRecordRow)]
  syntax CSTaskRecord ::= csTaskRecord(CTLanes, CSTask) [klabel(csTaskRecord)]
  syntax CSTaskRecords ::= csNoTaskRecords() [klabel(csNoTaskRecords)]
  syntax CSTaskRecords ::= csTaskRecordRow(CSTaskRecord, CSTaskRecords) [klabel(csTaskRecordRow)]
  syntax CSTaskSpan ::= csTaskSpan(Int, Int, Int, Int, CTLanes) [klabel(csTaskSpan)]
  syntax CSTaskSpans ::= csNoTaskSpans() [klabel(csNoTaskSpans)]
  syntax CSTaskSpans ::= csTaskSpanRow(CSTaskSpan, CSTaskSpans) [klabel(csTaskSpanRow)]
  syntax CSHelperSection ::= csHelperSection(CTLanes, CSHelperTables, CSTaskSpans, Int) [klabel(csHelperSection)]
  syntax CSHelperSections ::= csNoHelperSections() [klabel(csNoHelperSections)]
  syntax CSHelperSections ::= csHelperSectionRow(CSHelperSection, CSHelperSections) [klabel(csHelperSectionRow)]
  syntax CSChildSection ::= csChildSection(String, Int, Int, Int, Int, CTLanes) [klabel(csChildSection)]
  syntax CSChildSections ::= csNoChildSections() [klabel(csNoChildSections)]
  syntax CSChildSections ::= csChildSectionRow(CSChildSection, CSChildSections) [klabel(csChildSectionRow)]
  syntax CSStateObservation ::= csStateObservation(CTLanes, CSOwnedState, CSContext, CTLanes, CSChildSections, Int) [klabel(csStateObservation)]
  syntax CSStateObservations ::= csNoStateObservations() [klabel(csNoStateObservations)]
  syntax CSStateObservations ::= csStateObservationRow(CSStateObservation, CSStateObservations) [klabel(csStateObservationRow)]
  syntax CSObservations ::= csObservations(String, String, CSCaseIds, CSHelperRecords, CSResultRecords, CSTaskRecords, CSHelperSections, CSStateObservations) [klabel(csObservations)]
endmodule
