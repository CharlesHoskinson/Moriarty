requires "c6-typed-root-syntax.k"

module SHERLOCK-C6-OWNED-DATA-SYNTAX
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
endmodule
