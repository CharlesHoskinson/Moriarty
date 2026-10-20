requires "c6-positive-data-syntax.k"
requires "c6-typed-root-codec.k"

// Positive gate-one representation equations only. The SAFE contract supplies
// finite counts<=caps, raw128/usize/index/slice and canonical-padding premises.
// No error/refusal/gate-zero/admission/security policy is defined here.
module SHERLOCK-C6-POSITIVE-PADDED-DATA
  imports SHERLOCK-C6-POSITIVE-PADDED-DATA-SYNTAX
  imports SHERLOCK-C6-TYPED-ROOT

  syntax Int ::= dpNodeCap(DPCaps) [function, klabel(dpNodeCap)]
               | dpFieldCap(DPCaps) [function, klabel(dpFieldCap)]
               | dpEnvCap(DPCaps) [function, klabel(dpEnvCap)]
               | dpValueCap(DPCaps) [function, klabel(dpValueCap)]
  rule dpNodeCap(dpCaps(N, _, _, _, _, _, _, _, _)) => N
  rule dpFieldCap(dpCaps(_, F, _, _, _, _, _, _, _)) => F
  rule dpEnvCap(dpCaps(_, _, E, _, _, _, _, _, _)) => E
  rule dpValueCap(dpCaps(_, _, _, V, _, _, _, _, _)) => V

  syntax Int ::= dpFieldsBase(DPCaps) [function, klabel(dpFieldsBase)]
               | dpEnvBase(DPCaps) [function, klabel(dpEnvBase)]
               | dpValuesBase(DPCaps) [function, klabel(dpValuesBase)]
               | dpTotal(DPCaps) [function, klabel(dpTotal)]
  rule dpFieldsBase(C) => 7 +Int (5 *Int dpNodeCap(C))
  rule dpEnvBase(C) => dpFieldsBase(C) +Int (2 *Int dpFieldCap(C))
  rule dpValuesBase(C) => dpEnvBase(C) +Int (5 *Int dpEnvCap(C))
  rule dpTotal(C) => dpValuesBase(C) +Int (2 *Int dpValueCap(C))
  syntax DPGeometry ::= dpGeometryOf(DPCaps) [function, klabel(dpGeometryOf)]
  rule dpGeometryOf(C) => dpGeometry(7, dpFieldsBase(C), dpEnvBase(C), dpValuesBase(C), dpTotal(C))

  syntax Int ::= dpNodesSize(DPNodes) [function, klabel(dpNodesSize)]
               | dpRefsSize(DPRefs) [function, klabel(dpRefsSize)]
               | dpEnvsSize(DPEnvs) [function, klabel(dpEnvsSize)]
  rule dpNodesSize(dpNoNodes()) => 0
  rule dpNodesSize(dpNode(_, R)) => 1 +Int dpNodesSize(R)
  rule dpRefsSize(dpNoRefs()) => 0
  rule dpRefsSize(dpRef(_, R)) => 1 +Int dpRefsSize(R)
  rule dpEnvsSize(dpNoEnvs()) => 0
  rule dpEnvsSize(dpEnv(_, R)) => 1 +Int dpEnvsSize(R)

  syntax CTLanes ::= dpZeros(Int) [function, klabel(dpZeros)]
  rule dpZeros(0) => ctNoLanes()
  rule dpZeros(N) => ctLane(0, dpZeros(N -Int 1)) requires N >Int 0

  syntax CTLanes ::= dpEncodeData(DPData) [function, klabel(dpEncodeData)]
  rule dpEncodeData(dpU128(N)) => ctLane(0, ctLane(N, ctLane(0, ctLane(0, ctLane(0, ctNoLanes())))))
  rule dpEncodeData(dpBool(B)) => ctLane(1, ctLane(ctBool(B), ctLane(0, ctLane(0, ctLane(0, ctNoLanes())))))
  rule dpEncodeData(dpNil()) => ctLane(2, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctNoLanes())))))
  rule dpEncodeData(dpCons(ctSlice(S, L))) => ctLane(3, ctLane(0, ctLane(0, ctLane(S, ctLane(L, ctNoLanes())))))
  rule dpEncodeData(dpLeaf(ctSlice(S, L))) => ctLane(4, ctLane(0, ctLane(0, ctLane(S, ctLane(L, ctNoLanes())))))
  rule dpEncodeData(dpBranch(ctSlice(S, L))) => ctLane(5, ctLane(0, ctLane(0, ctLane(S, ctLane(L, ctNoLanes())))))
  rule dpEncodeData(dpConstructor(T, C, ctSlice(S, L))) => ctLane(6, ctLane(T, ctLane(C, ctLane(S, ctLane(L, ctNoLanes())))))

  syntax CTLanes ::= dpEncodeRef(CTRef) [function, klabel(dpEncodeRef)]
                  | dpEncodeEnv(DPEnvCell) [function, klabel(dpEncodeEnv)]
  rule dpEncodeRef(R) => ctLane(ctRefTag(R), ctLane(ctRefIndex(R), ctNoLanes()))
  rule dpEncodeEnv(dpEnvCell(B, R, P)) => ctLane(B, ctLane(ctRefTag(R), ctLane(ctRefIndex(R), ctLane(ctEnvTag(P), ctLane(ctEnvIndex(P), ctNoLanes())))))

  syntax CTLanes ::= dpEncodeNodes(DPNodes) [function, klabel(dpEncodeNodes)]
                  | dpEncodeRefs(DPRefs) [function, klabel(dpEncodeRefs)]
                  | dpEncodeEnvs(DPEnvs) [function, klabel(dpEncodeEnvs)]
  rule dpEncodeNodes(dpNoNodes()) => ctNoLanes()
  rule dpEncodeNodes(dpNode(N, R)) => ctJoin(dpEncodeData(N), dpEncodeNodes(R))
  rule dpEncodeRefs(dpNoRefs()) => ctNoLanes()
  rule dpEncodeRefs(dpRef(V, R)) => ctJoin(dpEncodeRef(V), dpEncodeRefs(R))
  rule dpEncodeEnvs(dpNoEnvs()) => ctNoLanes()
  rule dpEncodeEnvs(dpEnv(E, R)) => ctJoin(dpEncodeEnv(E), dpEncodeEnvs(R))

  // Counts are physical table lengths, not root allocation/segment counters.
  syntax CTLanes ::= dpHeader(DPTables) [function, klabel(dpHeader)]
  rule dpHeader(dpTables(D, F, E, V)) =>
    ctLane(18918541878711089, ctLane(1, ctLane(1145132097,
      ctLane(dpNodesSize(D), ctLane(dpRefsSize(F), ctLane(dpEnvsSize(E), ctLane(dpRefsSize(V), ctNoLanes())))))))

  syntax CTLanes ::= dpPack(DPCaps, DPTables) [function, klabel(dpPack)]
  rule dpPack(C, dpTables(D, F, E, V)) =>
    ctJoin(dpHeader(dpTables(D, F, E, V)),
      ctJoin(ctJoin(dpEncodeNodes(D), dpZeros((dpNodeCap(C) -Int dpNodesSize(D)) *Int 5)),
        ctJoin(ctJoin(dpEncodeRefs(F), dpZeros((dpFieldCap(C) -Int dpRefsSize(F)) *Int 2)),
          ctJoin(ctJoin(dpEncodeEnvs(E), dpZeros((dpEnvCap(C) -Int dpEnvsSize(E)) *Int 5)),
                 ctJoin(dpEncodeRefs(V), dpZeros((dpValueCap(C) -Int dpRefsSize(V)) *Int 2))))))

  // Positive canonical row inverse only, not a decoder for hostile raw images.
  syntax DPData ::= dpDecodeData(Int, Int, Int, Int, Int) [function, klabel(dpDecodeData)]
  rule dpDecodeData(0, N, 0, 0, 0) => dpU128(N)
  rule dpDecodeData(1, B, 0, 0, 0) => dpBool(ctBoolDecode(B))
  rule dpDecodeData(2, 0, 0, 0, 0) => dpNil()
  rule dpDecodeData(3, 0, 0, S, L) => dpCons(ctSlice(S, L))
  rule dpDecodeData(4, 0, 0, S, L) => dpLeaf(ctSlice(S, L))
  rule dpDecodeData(5, 0, 0, S, L) => dpBranch(ctSlice(S, L))
  rule dpDecodeData(6, T, C, S, L) => dpConstructor(T, C, ctSlice(S, L))

  syntax DPNodes ::= dpReadNodes(Int, CTLanes) [function, klabel(dpReadNodes)]
  syntax DPRefs ::= dpReadRefs(Int, CTLanes) [function, klabel(dpReadRefs)]
  syntax DPEnvs ::= dpReadEnvs(Int, CTLanes) [function, klabel(dpReadEnvs)]
  rule dpReadNodes(0, _) => dpNoNodes()
  rule dpReadNodes(N, L) => dpNode(dpDecodeData(ctAt(0, L), ctAt(1, L), ctAt(2, L), ctAt(3, L), ctAt(4, L)), dpReadNodes(N -Int 1, ctDrop(5, L))) requires N >Int 0
  rule dpReadRefs(0, _) => dpNoRefs()
  rule dpReadRefs(N, L) => dpRef(ctRefDecode(ctAt(0, L), ctAt(1, L)), dpReadRefs(N -Int 1, ctDrop(2, L))) requires N >Int 0
  rule dpReadEnvs(0, _) => dpNoEnvs()
  rule dpReadEnvs(N, L) => dpEnv(dpEnvCell(ctAt(0, L), ctRefDecode(ctAt(1, L), ctAt(2, L)), ctEnvDecode(ctAt(3, L), ctAt(4, L))), dpReadEnvs(N -Int 1, ctDrop(5, L))) requires N >Int 0

  syntax DPTables ::= dpUnpack(DPCaps, CTLanes) [function, klabel(dpUnpack)]
  rule dpUnpack(C, L) => dpTables(
    dpReadNodes(ctAt(3, L), ctDrop(7, L)),
    dpReadRefs(ctAt(4, L), ctDrop(dpFieldsBase(C), L)),
    dpReadEnvs(ctAt(5, L), ctDrop(dpEnvBase(C), L)),
    dpReadRefs(ctAt(6, L), ctDrop(dpValuesBase(C), L)))

  syntax DPObservation ::= dpObserve(DPCaps, DPTables) [function, klabel(dpObserve)]
                        | dpObservePacked(DPCaps, CTLanes) [function, klabel(dpObservePacked)]
  rule dpObserve(C, T) => dpObservePacked(C, dpPack(C, T))
  rule dpObservePacked(C, L) => dpObservation(dpGeometryOf(C),
    ctActive(ctAt(3, L), ctAt(4, L), ctAt(5, L), ctAt(6, L)), L, dpUnpack(C, L))
endmodule
