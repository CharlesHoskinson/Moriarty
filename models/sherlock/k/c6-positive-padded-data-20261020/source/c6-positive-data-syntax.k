requires "c6-typed-root-syntax.k"

module SHERLOCK-C6-POSITIVE-PADDED-DATA-SYNTAX
  imports SHERLOCK-C6-TYPED-ROOT-SYNTAX

  // Logical, positive-only carriers. Int widths/caps are external premises.
  syntax DPData ::= dpU128(Int) [klabel(dpU128)]
                  | dpBool(Bool) [klabel(dpBool)]
                  | dpNil() [klabel(dpNil)]
                  | dpCons(CTSlice) [klabel(dpCons)]
                  | dpLeaf(CTSlice) [klabel(dpLeaf)]
                  | dpBranch(CTSlice) [klabel(dpBranch)]
                  | dpConstructor(Int, Int, CTSlice) [klabel(dpConstructor)]
  syntax DPNodes ::= dpNoNodes() [klabel(dpNoNodes)]
                   | dpNode(DPData, DPNodes) [klabel(dpNode)]
  syntax DPRefs ::= dpNoRefs() [klabel(dpNoRefs)]
                  | dpRef(CTRef, DPRefs) [klabel(dpRef)]
  syntax DPEnvCell ::= dpEnvCell(Int, CTRef, CTEnv) [klabel(dpEnvCell)]
  syntax DPEnvs ::= dpNoEnvs() [klabel(dpNoEnvs)]
                  | dpEnv(DPEnvCell, DPEnvs) [klabel(dpEnv)]
  syntax DPTables ::= dpTables(DPNodes, DPRefs, DPEnvs, DPRefs) [klabel(dpTables)]
  // data/fields/env/values/constants/types/constructors/names/arity capacities.
  syntax DPCaps ::= dpCaps(Int, Int, Int, Int, Int, Int, Int, Int, Int) [klabel(dpCaps)]
  // data_base, fields_base, env_base, values_base, total.
  syntax DPGeometry ::= dpGeometry(Int, Int, Int, Int, Int) [klabel(dpGeometry)]
  // The exact shared four-field observation; no profile/case wrapper.
  syntax DPObservation ::= dpObservation(DPGeometry, CTActive, CTLanes, DPTables) [klabel(dpObservation)]
endmodule
