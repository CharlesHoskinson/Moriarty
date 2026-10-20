requires "c6-positive-meta.k"

module SHERLOCK-C6-COMPLETE-POSITIVE-META-LAWS
  imports SHERLOCK-C6-COMPLETE-POSITIVE-META
  imports K-EQUAL
  syntax Bool ::= cmDecodedWhole(CMMetadata, CMMetadataRead, Int) [function, klabel(cmDecodedWhole)]
  rule cmDecodedWhole(M, cmMetadataRead(D, cmReader(ctNoLanes(), C, _, _, _)), N) => (M ==K D) andBool (C ==Int N)
  syntax Bool ::= cmRoundtrip(CMMetadata) [function, klabel(cmRoundtrip)]
  rule cmRoundtrip(M) => cmDecodedWhole(M, cmDecodeMetadata(cmEncodeMetadata(M)), ctSize(cmEncodeMetadata(M)))
endmodule
