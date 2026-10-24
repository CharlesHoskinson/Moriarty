requires "c6-complete-helper-task-state-syntax.k"

module SHERLOCK-C6-COMPLETE-HELPER-TASK-STATE-INPUT-SYNTAX
  imports SHERLOCK-C6-COMPLETE-HELPER-TASK-STATE-SYNTAX
  syntax CSHelperCase ::= csHelperCase(String, CHHelperOption) [klabel(csHelperCase)]
  syntax CSResultCase ::= csResultCase(String, CHResultOption) [klabel(csResultCase)]
  syntax CSTaskCase ::= csTaskCase(String, CSTask) [klabel(csTaskCase)]
  syntax CSHelperSectionCase ::= csHelperSectionCase(String, CSHelperTables) [klabel(csHelperSectionCase)]
  syntax CSStateCase ::= csStateCase(String, CSContext, CSOwnedState) [klabel(csStateCase)]
  syntax CSHelperCases ::= csNoHelperCases() [klabel(csNoHelperCases)]
  syntax CSHelperCases ::= csHelperCaseRow(CSHelperCase, CSHelperCases) [klabel(csHelperCaseRow)]
  syntax CSResultCases ::= csNoResultCases() [klabel(csNoResultCases)]
  syntax CSResultCases ::= csResultCaseRow(CSResultCase, CSResultCases) [klabel(csResultCaseRow)]
  syntax CSTaskCases ::= csNoTaskCases() [klabel(csNoTaskCases)]
  syntax CSTaskCases ::= csTaskCaseRow(CSTaskCase, CSTaskCases) [klabel(csTaskCaseRow)]
  syntax CSHelperSectionCases ::= csNoHelperSectionCases() [klabel(csNoHelperSectionCases)]
  syntax CSHelperSectionCases ::= csHelperSectionCaseRow(CSHelperSectionCase, CSHelperSectionCases) [klabel(csHelperSectionCaseRow)]
  syntax CSStateCases ::= csNoStateCases() [klabel(csNoStateCases)]
  syntax CSStateCases ::= csStateCaseRow(CSStateCase, CSStateCases) [klabel(csStateCaseRow)]
  syntax CSCohort ::= csCohort(String, String, CSCaps, CSHelperCases, CSResultCases, CSTaskCases, CSHelperSectionCases, CSStateCases) [klabel(csCohort)]
endmodule
