requires "c6-positive-context-syntax.k"

// General positive representation only. Widths/usize/raw128 are external premises.
// No fixture IDs, new guards, refusal, admission, auth or transition semantics.
module SHERLOCK-C6-POSITIVE-CONTEXT
  imports SHERLOCK-C6-POSITIVE-CONTEXT-SYNTAX

  syntax CXExtFields ::= cxEffective(CXExt) [function, klabel(cxEffective)]
  rule cxEffective(cxAbsent()) => cxExt(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0)
  rule cxEffective(cxPresent(E)) => E
  syntax Bool ::= cxExtPresent(CXExt) [function, klabel(cxExtPresent)]
  rule cxExtPresent(cxAbsent()) => false
  rule cxExtPresent(cxPresent(_)) => true

  syntax CTLanes ::= cxPack(CXPayload) [function, klabel(cxPack)]
  rule cxPack(cxPayload(cxAbsent(), cxBudgets(U, V))) => ctLane(18918541878711089, ctLane(1, ctLane(1129601108, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(U, ctLane(V, ctNoLanes()))))))))))))))))))))))
  rule cxPack(cxPayload(cxPresent(cxExt(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15)), cxBudgets(U, V))) => ctLane(18918541878711089, ctLane(1, ctLane(1129601108, ctLane(1, ctLane(E0, ctLane(E1, ctLane(E2, ctLane(E3, ctLane(E4, ctLane(E5, ctLane(E6, ctLane(E7, ctLane(E8, ctLane(E9, ctLane(E10, ctLane(E11, ctLane(E12, ctLane(E13, ctLane(E14, ctLane(E15, ctLane(U, ctLane(V, ctNoLanes()))))))))))))))))))))))

  // Inverse only on exact produced22-lane positive images, including whole terminator.
  syntax CXPayload ::= cxUnpack(CTLanes) [function, klabel(cxUnpack)]
  rule cxUnpack(ctLane(18918541878711089, ctLane(1, ctLane(1129601108, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(0, ctLane(U, ctLane(V, ctNoLanes()))))))))))))))))))))))) => cxPayload(cxAbsent(), cxBudgets(U, V))
  rule cxUnpack(ctLane(18918541878711089, ctLane(1, ctLane(1129601108, ctLane(1, ctLane(E0, ctLane(E1, ctLane(E2, ctLane(E3, ctLane(E4, ctLane(E5, ctLane(E6, ctLane(E7, ctLane(E8, ctLane(E9, ctLane(E10, ctLane(E11, ctLane(E12, ctLane(E13, ctLane(E14, ctLane(E15, ctLane(U, ctLane(V, ctNoLanes()))))))))))))))))))))))) => cxPayload(cxPresent(cxExt(E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15)), cxBudgets(U, V))

  syntax Int ::= cxDeclTemplates(Int) [function, klabel(cxDeclTemplates)]
  rule cxDeclTemplates(T) => T -Int 4 requires T >=Int 4
  rule cxDeclTemplates(T) => 0 requires T <Int 4

  syntax CXStateView ::= cxDerive(CXInput) [function, klabel(cxDerive)]
                        | cxDeriveEffective(CXExtFields, Bool, CXBudgets, CXBounds, CXLayoutCaps, CXHostLimits) [function, klabel(cxDeriveEffective)]
  rule cxDerive(cxInput(cxPayload(E, U), B, C, H)) => cxDeriveEffective(cxEffective(E), cxExtPresent(E), U, B, C, H)
  rule cxDeriveEffective(cxExt(E0, E1, E2, _, _, _, _, _, _, _, _, _, _, _, _, _), PRESENT, cxBudgets(U, V), cxBounds(B0, B1, B2, B3, B4, B5, B6, B7, B8, B9, B10, B11, B12, B13, B14, B15, B16, B17, _, _, _, _), cxLayoutCaps(C0, C1, C2, C3), cxHostLimits(_, H1, _, _, _)) =>
    cxStateView(cxStateCaps(cxPhysCaps(B16, B0, C0, B6, B2, C3, B11, B13, B15, B1, B10, B7, B0, C2, B3, B4, E2, B8, B9, cxDeclTemplates(B7), B5, E0, E1, B12, C1, U, V), B14, B17), PRESENT, cxStateLimits(H1))

  syntax CXStateCaps ::= cxCapsOf(CXStateView) [function, klabel(cxCapsOf)]
  rule cxCapsOf(cxStateView(C, _, _)) => C
  syntax CTLanes ::= cxDescriptor(CXStateCaps) [function, klabel(cxDescriptor)]
  rule cxDescriptor(cxStateCaps(cxPhysCaps(P0, P1, P2, P3, P4, P5, P6, P7, P8, P9, P10, P11, P12, P13, P14, P15, P16, P17, P18, P19, P20, P21, P22, P23, P24, P25, P26), DF, HT)) => ctLane(P0, ctLane(P1, ctLane(P2, ctLane(P3, ctLane(P4, ctLane(P5, ctLane(P6, ctLane(P7, ctLane(P8, ctLane(P9, ctLane(P10, ctLane(P11, ctLane(P12, ctLane(P13, ctLane(P14, ctLane(P15, ctLane(P16, ctLane(P17, ctLane(P18, ctLane(P19, ctLane(P20, ctLane(P21, ctLane(P22, ctLane(P23, ctLane(P24, ctLane(P25, ctLane(P26, ctLane(DF, ctLane(HT, ctNoLanes())))))))))))))))))))))))))))))

  syntax CXObservation ::= cxObserve(CXInput) [function, klabel(cxObserve)]
                          | cxObservePacked(CXInput, CTLanes) [function, klabel(cxObservePacked)]
                          | cxObserveDecoded(CTLanes, CXPayload, CXBounds, CXLayoutCaps, CXHostLimits) [function, klabel(cxObserveDecoded)]
                          | cxObserveDerived(CTLanes, CXPayload, CXStateView) [function, klabel(cxObserveDerived)]
  rule cxObserve(cxInput(P, B, C, H)) => cxObservePacked(cxInput(P, B, C, H), cxPack(P))
  rule cxObservePacked(cxInput(_, B, C, H), L) => cxObserveDecoded(L, cxUnpack(L), B, C, H)
  rule cxObserveDecoded(L, P, B, C, H) => cxObserveDerived(L, P, cxDerive(cxInput(P, B, C, H)))
  rule cxObserveDerived(L, cxPayload(E, B), S) => cxObservation(L, cxPayload(E, B), cxEffective(E), S, cxDescriptor(cxCapsOf(S)))

  syntax CXObservations ::= cxObserveAll(CXCases) [function, klabel(cxObserveAll)]
  rule cxObserveAll(cxNoCases()) => cxNoObservations()
  rule cxObserveAll(cxCase(_, I, R)) => cxObservationCons(cxObserve(I), cxObserveAll(R))
endmodule
