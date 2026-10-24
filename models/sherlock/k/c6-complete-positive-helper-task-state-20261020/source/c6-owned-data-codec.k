requires "c6-owned-data-syntax.k"
requires "c6-typed-root-codec.k"

module SHERLOCK-C6-OWNED-DATA
  imports SHERLOCK-C6-OWNED-DATA-SYNTAX
  imports SHERLOCK-C6-TYPED-ROOT
  syntax Int ::= dpNodesSize(DPNodes) [function, klabel(dpNodesSize)]
               | dpRefsSize(DPRefs) [function, klabel(dpRefsSize)]
               | dpEnvsSize(DPEnvs) [function, klabel(dpEnvsSize)]
  rule dpNodesSize(dpNoNodes()) => 0
  rule dpNodesSize(dpNode(_, R)) => 1 +Int dpNodesSize(R)
  rule dpRefsSize(dpNoRefs()) => 0
  rule dpRefsSize(dpRef(_, R)) => 1 +Int dpRefsSize(R)
  rule dpEnvsSize(dpNoEnvs()) => 0
  rule dpEnvsSize(dpEnv(_, R)) => 1 +Int dpEnvsSize(R)

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

endmodule
