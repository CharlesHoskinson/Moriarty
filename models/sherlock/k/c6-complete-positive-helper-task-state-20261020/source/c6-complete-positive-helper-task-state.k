requires "c6-complete-helper-task-state-input-syntax.k"
requires "c6-helper-codec.k"
requires "c6-owned-data-codec.k"

module SHERLOCK-C6-COMPLETE-POSITIVE-HELPER-TASK-STATE
  imports SHERLOCK-C6-COMPLETE-HELPER-TASK-STATE-INPUT-SYNTAX
  imports SHERLOCK-C6-HELPER-RECORD
  imports SHERLOCK-C6-OWNED-DATA
  imports K-EQUAL
  // Positive canonical images and external native-fit/cap premises only.
  // No malformed/refusal/first-error policy, helper execution, admission or clock.
  syntax Int ::= csVisitTag(CSVisit) [function, klabel(csVisitTag)]
  syntax CSVisit ::= csVisitDecode(Int) [function, klabel(csVisitDecode)]
  rule csVisitTag(csUnvisited()) => 0
  rule csVisitDecode(0) => csUnvisited()
  rule csVisitTag(csFields()) => 1
  rule csVisitDecode(1) => csFields()
  syntax CTLanes ::= csEncodeTask(CSTask) [function, klabel(csEncodeTask)]
  syntax CSTask ::= csDecodeTask(CTLanes) [function, klabel(csDecodeTask)]
  rule csEncodeTask(csValidateTask(R, V, C)) => ctLane(0, ctLane(ctRefTag(R), ctLane(ctRefIndex(R), ctLane(csVisitTag(V), ctLane(C, ctLane(0, ctLane(0, ctNoLanes())))))))
  rule csEncodeTask(csEqualTask(A, B, V, C)) => ctLane(1, ctLane(ctRefTag(A), ctLane(ctRefIndex(A), ctLane(ctRefTag(B), ctLane(ctRefIndex(B), ctLane(csVisitTag(V), ctLane(C, ctNoLanes())))))))
  rule csEncodeTask(csPairTask(B, T, A)) => ctLane(2, ctLane(B, ctLane(T, ctLane(ctRefTag(A), ctLane(ctRefIndex(A), ctLane(0, ctLane(0, ctNoLanes())))))))
  rule csDecodeTask(ctLane(0, ctLane(T, ctLane(I, ctLane(V, ctLane(C, ctLane(0, ctLane(0, ctNoLanes())))))))) => csValidateTask(ctRefDecode(T, I), csVisitDecode(V), C)
  rule csDecodeTask(ctLane(1, ctLane(AT, ctLane(AI, ctLane(BT, ctLane(BI, ctLane(V, ctLane(C, ctNoLanes())))))))) => csEqualTask(ctRefDecode(AT, AI), ctRefDecode(BT, BI), csVisitDecode(V), C)
  rule csDecodeTask(ctLane(2, ctLane(B, ctLane(T, ctLane(AT, ctLane(AI, ctLane(0, ctLane(0, ctNoLanes())))))))) => csPairTask(B, T, ctRefDecode(AT, AI))
  syntax Int ::= csTaskCount(CSTasks) [function, klabel(csTaskCount)]
  rule csTaskCount(csNoTasks()) => 0
  rule csTaskCount(csTask(_, T)) => 1 +Int csTaskCount(T)
  syntax CTLanes ::= csEncodeTasks(CSTasks) [function, klabel(csEncodeTasks)]
  rule csEncodeTasks(csNoTasks()) => ctNoLanes()
  rule csEncodeTasks(csTask(H, T)) => ctJoin(csEncodeTask(H), csEncodeTasks(T))
  syntax CSTasks ::= csReadTasks(Int, CTLanes) [function, klabel(csReadTasks)]
  rule csReadTasks(0, ctNoLanes()) => csNoTasks()
  rule csReadTasks(N, L) => csTask(csDecodeTask(ctTake(7, L)), csReadTasks(N -Int 1, ctDrop(7, L))) requires N >Int 0
  syntax CTLanes ::= csPackHelper(CSHelperTables) [function, klabel(csPackHelper)]
  rule csPackHelper(csHelperTables(H, R, T)) => ctJoin(ctLane(18918541878711089, ctLane(1, ctLane(1212501072, ctLane(csTaskCount(T), ctNoLanes())))), ctJoin(chEncodeHelper(H), ctJoin(chEncodeResult(R), csEncodeTasks(T))))
  syntax CSHelperTables ::= csReadHelper(CTLanes) [function, klabel(csReadHelper)]
  rule csReadHelper(L) => csHelperTables(chDecodeHelper(ctTake(23, ctDrop(4, L))), chDecodeResult(ctTake(4, ctDrop(27, L))), csReadTasks(ctAt(3, L), ctDrop(31, L))) requires ctAt(0, L) ==Int 18918541878711089 andBool ctAt(1, L) ==Int 1 andBool ctAt(2, L) ==Int 1212501072 andBool ctSize(L) ==Int 31 +Int (7 *Int ctAt(3, L))
  syntax CTLanes ::= csPackOwnedData(DPTables) [function, klabel(csPackOwnedData)]
  rule csPackOwnedData(dpTables(D, F, E, V)) => ctJoin(dpHeader(dpTables(D, F, E, V)), ctJoin(dpEncodeNodes(D), ctJoin(dpEncodeRefs(F), ctJoin(dpEncodeEnvs(E), dpEncodeRefs(V)))))
  syntax Int ::= csDataFieldsBase(CTLanes) [function, klabel(csDataFieldsBase)]
  rule csDataFieldsBase(L) => 7 +Int (5 *Int ctAt(3, L))
  syntax Int ::= csDataEnvBase(CTLanes) [function, klabel(csDataEnvBase)]
  rule csDataEnvBase(L) => csDataFieldsBase(L) +Int (2 *Int ctAt(4, L))
  syntax Int ::= csDataValuesBase(CTLanes) [function, klabel(csDataValuesBase)]
  rule csDataValuesBase(L) => csDataEnvBase(L) +Int (5 *Int ctAt(5, L))
  syntax DPTables ::= csReadOwnedData(CTLanes) [function, klabel(csReadOwnedData)]
  rule csReadOwnedData(L) => dpTables(dpReadNodes(ctAt(3, L), ctTake(5 *Int ctAt(3, L), ctDrop(7, L))), dpReadRefs(ctAt(4, L), ctTake(2 *Int ctAt(4, L), ctDrop(csDataFieldsBase(L), L))), dpReadEnvs(ctAt(5, L), ctTake(5 *Int ctAt(5, L), ctDrop(csDataEnvBase(L), L))), dpReadRefs(ctAt(6, L), ctDrop(csDataValuesBase(L), L))) requires ctAt(0, L) ==Int 18918541878711089 andBool ctAt(1, L) ==Int 1 andBool ctAt(2, L) ==Int 1145132097 andBool ctSize(L) ==Int csDataValuesBase(L) +Int (2 *Int ctAt(6, L))
  syntax CTLanes ::= csCapsLanes(CSCaps) [function, klabel(csCapsLanes)]
  rule csCapsLanes(csCaps(C0, C1, C2, C3, C4, C5, C6, C7, C8, C9, C10, C11, C12, C13, C14, C15, C16, C17, C18, C19, C20, C21, C22, C23, C24, C25, C26, C27, C28)) => ctLane(C0, ctLane(C1, ctLane(C2, ctLane(C3, ctLane(C4, ctLane(C5, ctLane(C6, ctLane(C7, ctLane(C8, ctLane(C9, ctLane(C10, ctLane(C11, ctLane(C12, ctLane(C13, ctLane(C14, ctLane(C15, ctLane(C16, ctLane(C17, ctLane(C18, ctLane(C19, ctLane(C20, ctLane(C21, ctLane(C22, ctLane(C23, ctLane(C24, ctLane(C25, ctLane(C26, ctLane(C27, ctLane(C28, ctNoLanes())))))))))))))))))))))))))))))
  syntax CTLanes ::= csContextCaps(CSContext) [function, klabel(csContextCaps)]
  rule csContextCaps(csContext(C, _, _)) => csCapsLanes(C)
  syntax Int ::= csContextPresence(CSContext) [function, klabel(csContextPresence)]
  rule csContextPresence(csContext(_, B, _)) => ctBool(B)
  syntax CTLanes ::= csPackState(CSContext, CSOwnedState) [function, klabel(csPackState)]
  syntax CTLanes ::= csPackStateParts(CSContext, CTLanes, CTLanes, CTLanes) [function, klabel(csPackStateParts)]
  rule csPackState(C, csOwnedState(R, D, H)) => csPackStateParts(C, ctEncodeRoot(R), csPackOwnedData(D), csPackHelper(H))
  rule csPackStateParts(C, I, D, H) => ctJoin(ctJoin(ctLane(18918541878711089, ctLane(1, ctLane(1398030676, ctLane(3, ctLane(1229799751, ctLane(ctSize(I), ctLane(1145132097, ctLane(ctSize(D), ctLane(1212501072, ctLane(ctSize(H), ctLane(csContextPresence(C), ctLane(0, ctNoLanes())))))))))))), csContextCaps(C)), ctJoin(I, ctJoin(D, H)))
  syntax Int ::= csStateDataStart(CTLanes) [function, klabel(csStateDataStart)]
  rule csStateDataStart(L) => 41 +Int ctAt(5, L)
  syntax Int ::= csStateHelperStart(CTLanes) [function, klabel(csStateHelperStart)]
  rule csStateHelperStart(L) => csStateDataStart(L) +Int ctAt(7, L)
  syntax CSOwnedState ::= csReadState(CSContext, CTLanes) [function, klabel(csReadState)]
  rule csReadState(C, L) => csOwnedState(ctDecodeRoot(ctTake(ctAt(5, L), ctDrop(41, L))), csReadOwnedData(ctTake(ctAt(7, L), ctDrop(csStateDataStart(L), L))), csReadHelper(ctDrop(csStateHelperStart(L), L))) requires ctAt(0, L) ==Int 18918541878711089 andBool ctAt(1, L) ==Int 1 andBool ctAt(2, L) ==Int 1398030676 andBool ctAt(3, L) ==Int 3 andBool ctAt(4, L) ==Int 1229799751 andBool ctAt(6, L) ==Int 1145132097 andBool ctAt(8, L) ==Int 1212501072 andBool ctAt(10, L) ==Int csContextPresence(C) andBool ctAt(11, L) ==Int 0 andBool (ctTake(29, ctDrop(12, L)) ==K csContextCaps(C)) andBool ctSize(L) ==Int csStateHelperStart(L) +Int ctAt(9, L)
  syntax CSTaskSpans ::= csSpans(Int, Int, CTLanes) [function, klabel(csSpans)]
  rule csSpans(0, _, ctNoLanes()) => csNoTaskSpans()
  rule csSpans(N, I, L) => csTaskSpanRow(csTaskSpan(I, 31 +Int (7 *Int I), 7, 38 +Int (7 *Int I), ctTake(7, L)), csSpans(N -Int 1, I +Int 1, ctDrop(7, L))) requires N >Int 0
  syntax CSChildSections ::= csChildren(CTLanes) [function, klabel(csChildren)]
  rule csChildren(L) => csChildSectionRow(csChildSection("image", 1229799751, 41, ctAt(5, L), csStateDataStart(L), ctTake(ctAt(5, L), ctDrop(41, L))), csChildSectionRow(csChildSection("data", 1145132097, csStateDataStart(L), ctAt(7, L), csStateHelperStart(L), ctTake(ctAt(7, L), ctDrop(csStateDataStart(L), L))), csChildSectionRow(csChildSection("helper", 1212501072, csStateHelperStart(L), ctAt(9, L), ctSize(L), ctDrop(csStateHelperStart(L), L)), csNoChildSections())))
  syntax CSHelperRecord ::= csObserveHelperRecord(CHHelperOption) [function, klabel(csObserveHelperRecord)]
  syntax CSHelperRecord ::= csObserveHelperRecordEncoded(CTLanes) [function, klabel(csObserveHelperRecordEncoded)]
  rule csObserveHelperRecord(V) => csObserveHelperRecordEncoded(chEncodeHelper(V))
  rule csObserveHelperRecordEncoded(L) => csHelperRecord(L, chDecodeHelper(L))
  syntax CSResultRecord ::= csObserveResultRecord(CHResultOption) [function, klabel(csObserveResultRecord)]
  syntax CSResultRecord ::= csObserveResultRecordEncoded(CTLanes) [function, klabel(csObserveResultRecordEncoded)]
  rule csObserveResultRecord(V) => csObserveResultRecordEncoded(chEncodeResult(V))
  rule csObserveResultRecordEncoded(L) => csResultRecord(L, chDecodeResult(L))
  syntax CSTaskRecord ::= csObserveTaskRecord(CSTask) [function, klabel(csObserveTaskRecord)]
  syntax CSTaskRecord ::= csObserveTaskRecordEncoded(CTLanes) [function, klabel(csObserveTaskRecordEncoded)]
  rule csObserveTaskRecord(V) => csObserveTaskRecordEncoded(csEncodeTask(V))
  rule csObserveTaskRecordEncoded(L) => csTaskRecord(L, csDecodeTask(L))
  syntax CSHelperSection ::= csObserveHelperSection(CSHelperTables) [function, klabel(csObserveHelperSection)]
  syntax CSHelperSection ::= csObserveHelperSectionEncoded(CTLanes) [function, klabel(csObserveHelperSectionEncoded)]
  rule csObserveHelperSection(T) => csObserveHelperSectionEncoded(csPackHelper(T))
  rule csObserveHelperSectionEncoded(L) => csHelperSection(L, csReadHelper(L), csSpans(ctAt(3, L), 0, ctDrop(31, L)), ctSize(L))
  syntax CSStateObservation ::= csObserveState(CSContext, CSOwnedState) [function, klabel(csObserveState)]
  syntax CSStateObservation ::= csObserveStateEncoded(CSContext, CTLanes) [function, klabel(csObserveStateEncoded)]
  rule csObserveState(C, S) => csObserveStateEncoded(C, csPackState(C, S))
  rule csObserveStateEncoded(C, L) => csStateObservation(L, csReadState(C, L), C, csContextCaps(C), csChildren(L), ctSize(L))
  syntax CSStrings ::= csHelperIds(CSHelperCases) [function, klabel(csHelperIds)]
  rule csHelperIds(csNoHelperCases()) => csNoStrings()
  rule csHelperIds(csHelperCaseRow(csHelperCase(Id, _), T)) => csString(Id, csHelperIds(T))
  syntax CSHelperRecords ::= csObserveHelperCases(CSHelperCases) [function, klabel(csObserveHelperCases)]
  rule csObserveHelperCases(csNoHelperCases()) => csNoHelperRecords()
  rule csObserveHelperCases(csHelperCaseRow(csHelperCase(_, V), T)) => csHelperRecordRow(csObserveHelperRecord(V), csObserveHelperCases(T))
  syntax CSStrings ::= csResultIds(CSResultCases) [function, klabel(csResultIds)]
  rule csResultIds(csNoResultCases()) => csNoStrings()
  rule csResultIds(csResultCaseRow(csResultCase(Id, _), T)) => csString(Id, csResultIds(T))
  syntax CSResultRecords ::= csObserveResultCases(CSResultCases) [function, klabel(csObserveResultCases)]
  rule csObserveResultCases(csNoResultCases()) => csNoResultRecords()
  rule csObserveResultCases(csResultCaseRow(csResultCase(_, V), T)) => csResultRecordRow(csObserveResultRecord(V), csObserveResultCases(T))
  syntax CSStrings ::= csTaskIds(CSTaskCases) [function, klabel(csTaskIds)]
  rule csTaskIds(csNoTaskCases()) => csNoStrings()
  rule csTaskIds(csTaskCaseRow(csTaskCase(Id, _), T)) => csString(Id, csTaskIds(T))
  syntax CSTaskRecords ::= csObserveTaskCases(CSTaskCases) [function, klabel(csObserveTaskCases)]
  rule csObserveTaskCases(csNoTaskCases()) => csNoTaskRecords()
  rule csObserveTaskCases(csTaskCaseRow(csTaskCase(_, V), T)) => csTaskRecordRow(csObserveTaskRecord(V), csObserveTaskCases(T))
  syntax CSStrings ::= csHelperSectionIds(CSHelperSectionCases) [function, klabel(csHelperSectionIds)]
  rule csHelperSectionIds(csNoHelperSectionCases()) => csNoStrings()
  rule csHelperSectionIds(csHelperSectionCaseRow(csHelperSectionCase(Id, _), T)) => csString(Id, csHelperSectionIds(T))
  syntax CSHelperSections ::= csObserveHelperSectionCases(CSHelperSectionCases) [function, klabel(csObserveHelperSectionCases)]
  rule csObserveHelperSectionCases(csNoHelperSectionCases()) => csNoHelperSections()
  rule csObserveHelperSectionCases(csHelperSectionCaseRow(csHelperSectionCase(_, V), T)) => csHelperSectionRow(csObserveHelperSection(V), csObserveHelperSectionCases(T))
  syntax CSStrings ::= csStateIds(CSStateCases) [function, klabel(csStateIds)]
  rule csStateIds(csNoStateCases()) => csNoStrings()
  rule csStateIds(csStateCaseRow(csStateCase(Id, _, _), T)) => csString(Id, csStateIds(T))
  syntax CSStateObservations ::= csObserveStateCases(CSStateCases) [function, klabel(csObserveStateCases)]
  rule csObserveStateCases(csNoStateCases()) => csNoStateObservations()
  rule csObserveStateCases(csStateCaseRow(csStateCase(_, C, S), T)) => csStateObservationRow(csObserveState(C, S), csObserveStateCases(T))
  syntax CSObservations ::= csObserveAll(CSCohort) [function, klabel(csObserveAll)]
  rule csObserveAll(csCohort(_, Profile, _, H, R, T, HS, S)) => csObservations("independent-c6-positive-helper-task-state/1", Profile, csCaseIds(csHelperIds(H), csResultIds(R), csTaskIds(T), csHelperSectionIds(HS), csStateIds(S)), csObserveHelperCases(H), csObserveResultCases(R), csObserveTaskCases(T), csObserveHelperSectionCases(HS), csObserveStateCases(S))
endmodule
