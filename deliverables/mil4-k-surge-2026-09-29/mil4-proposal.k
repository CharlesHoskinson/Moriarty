// Isolated, specified-only MIL/4 K proposal. No accepted Core or native claim.
requires "sol-static.k"
requires "sol-transition.k"
requires "sol-defi.k"
requires "sol-formula.k"
requires "sol-contract.k"
requires "sol-family-effects.k"

module MIL4-PROPOSAL-SYNTAX
  imports M4S-STATIC
  imports M4T-TRANSITION
  imports M4D-DEFI
  imports M4F-FORMULA
  imports M4C-CONTRACT
  imports M4E-EFFECTS
  syntax M4Request ::= m4Check(M4SEnv,M4SFormula,M4SPhase,Int)
                     | m4Step(M4TRequest)
                     | m4FamilyProjection(M4DReq)
                     | m4AdmitFamily(M4DReq)
                     | m4Evaluate(M4SEnv,M4SFormula,M4SPhase,M4FFrame,Int)
                     | m4Accept(M4CCandidate)
                     | m4FamilyEffects(M4EPacket)
endmodule

module MIL4-PROPOSAL
  imports MIL4-PROPOSAL-SYNTAX
  configuration <k> $PGM:M4Request </k>
  rule <k> m4Check(E,F,P,N) => m4sStatic(E,F,P,N) ... </k>
  rule <k> m4Step(R) => m4tEvaluate(R) ... </k>
  rule <k> m4FamilyProjection(R) => m4dEval(R) ... </k>
  rule <k> m4AdmitFamily(R) => m4dAdmit(R) ... </k>
  rule <k> m4Evaluate(E,F,P,S,N) => m4fEvaluate(E,F,P,S,N) ... </k>
  rule <k> m4Accept(C) => m4cAccept(C) ... </k>
  rule <k> m4FamilyEffects(P) => m4eCheck(P) ... </k>
endmodule
