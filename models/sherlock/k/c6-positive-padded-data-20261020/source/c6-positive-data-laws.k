requires "c6-positive-data.k"

// Supplementary generic finite-query predicates, not proofs or the independent
// complete-observation comparison. No malformed/refusal/security judgment.
module SHERLOCK-C6-POSITIVE-PADDED-DATA-LAWS
  imports SHERLOCK-C6-POSITIVE-PADDED-DATA
  imports K-EQUAL

  syntax Bool ::= dpInverse(DPCaps, DPTables) [function, klabel(dpInverse)]
                | dpExtent(DPCaps, DPTables) [function, klabel(dpExtent)]
                | dpHeaderCounts(DPCaps, DPTables) [function, klabel(dpHeaderCounts)]
                | dpInactiveZeros(DPCaps, DPTables) [function, klabel(dpInactiveZeros)]
                | dpAllZero(CTLanes) [function, klabel(dpAllZero)]
                | dpPaddingIn(DPCaps, DPTables, CTLanes) [function, klabel(dpPaddingIn)]
  rule dpInverse(C, T) => dpUnpack(C, dpPack(C, T)) ==K T
  rule dpExtent(C, T) => ctSize(dpPack(C, T)) ==Int dpTotal(C)
  rule dpHeaderCounts(C, dpTables(D, F, E, V)) =>
    (ctAt(3, dpPack(C, dpTables(D, F, E, V))) ==Int dpNodesSize(D)) andBool
    (ctAt(4, dpPack(C, dpTables(D, F, E, V))) ==Int dpRefsSize(F)) andBool
    (ctAt(5, dpPack(C, dpTables(D, F, E, V))) ==Int dpEnvsSize(E)) andBool
    (ctAt(6, dpPack(C, dpTables(D, F, E, V))) ==Int dpRefsSize(V))
  rule dpAllZero(ctNoLanes()) => true
  rule dpAllZero(ctLane(I, R)) => (I ==Int 0) andBool dpAllZero(R)
  rule dpInactiveZeros(C, T) => dpPaddingIn(C, T, dpPack(C, T))
  rule dpPaddingIn(C, dpTables(D, F, E, V), L) =>
    dpAllZero(ctTake((dpNodeCap(C) -Int dpNodesSize(D)) *Int 5,
      ctDrop(7 +Int (dpNodesSize(D) *Int 5), L))) andBool
    dpAllZero(ctTake((dpFieldCap(C) -Int dpRefsSize(F)) *Int 2,
      ctDrop(dpFieldsBase(C) +Int (dpRefsSize(F) *Int 2), L))) andBool
    dpAllZero(ctTake((dpEnvCap(C) -Int dpEnvsSize(E)) *Int 5,
      ctDrop(dpEnvBase(C) +Int (dpEnvsSize(E) *Int 5), L))) andBool
    dpAllZero(ctTake((dpValueCap(C) -Int dpRefsSize(V)) *Int 2,
      ctDrop(dpValuesBase(C) +Int (dpRefsSize(V) *Int 2), L)))
endmodule
