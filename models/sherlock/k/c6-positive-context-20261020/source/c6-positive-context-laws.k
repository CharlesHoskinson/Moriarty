requires "c6-positive-context.k"

// Supplementary generic predicates, UNRUN/unproved. Not the independent oracle.
module SHERLOCK-C6-POSITIVE-CONTEXT-LAWS
  imports SHERLOCK-C6-POSITIVE-CONTEXT
  imports K-EQUAL
  syntax Int ::= cxLength(CTLanes) [function, klabel(cxLength)]
  rule cxLength(ctNoLanes()) => 0
  rule cxLength(ctLane(_, R)) => 1 +Int cxLength(R)
  syntax Bool ::= cxPositiveRoundtrip(CXPayload) [function, klabel(cxPositiveRoundtrip)]
                | cxEncodedWidth(CXPayload) [function, klabel(cxEncodedWidth)]
                | cxDescriptorWidth(CXInput) [function, klabel(cxDescriptorWidth)]
  rule cxPositiveRoundtrip(P) => cxUnpack(cxPack(P)) ==K P
  rule cxEncodedWidth(P) => cxLength(cxPack(P)) ==Int 22
  rule cxDescriptorWidth(I) => cxLength(cxDescriptor(cxCapsOf(cxDerive(I)))) ==Int 29
endmodule
