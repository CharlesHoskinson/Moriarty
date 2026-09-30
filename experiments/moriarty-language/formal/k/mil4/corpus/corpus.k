// Finite experiment harness: injects the explicitly supplied trusted premise.
// This does not verify authentication, state provenance, or ledger admission.
requires "../s0.k"

module MIL4-S0-CORPUS
  imports MIL4-S0
  syntax Request ::= withAuthenticated(Request, Premise) [symbol(withAuthenticated)]
  rule <k> withAuthenticated(R, P) => R </k>
       <external> unavailable() => P </external>
endmodule
