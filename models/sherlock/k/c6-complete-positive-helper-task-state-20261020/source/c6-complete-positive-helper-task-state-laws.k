requires "c6-complete-positive-helper-task-state.k"

module SHERLOCK-C6-COMPLETE-POSITIVE-HELPER-TASK-STATE-LAWS
  imports SHERLOCK-C6-COMPLETE-POSITIVE-HELPER-TASK-STATE
  imports K-EQUAL
  syntax Bool ::= csTaskRoundtrip(CSTask) [function, klabel(csTaskRoundtrip)]
  rule csTaskRoundtrip(T) => csDecodeTask(csEncodeTask(T)) ==K T
  syntax Bool ::= csHelperRoundtrip(CSHelperTables) [function, klabel(csHelperRoundtrip)]
  rule csHelperRoundtrip(H) => csReadHelper(csPackHelper(H)) ==K H
  syntax Bool ::= csDataRoundtrip(DPTables) [function, klabel(csDataRoundtrip)]
  rule csDataRoundtrip(D) => csReadOwnedData(csPackOwnedData(D)) ==K D
  syntax Bool ::= csStateRoundtrip(CSContext, CSOwnedState) [function, klabel(csStateRoundtrip)]
  rule csStateRoundtrip(C, S) => csReadState(C, csPackState(C, S)) ==K S
endmodule
