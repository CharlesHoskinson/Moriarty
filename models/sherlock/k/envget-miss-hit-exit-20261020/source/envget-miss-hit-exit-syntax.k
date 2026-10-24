requires "envget-positive-syntax.k"

module SHERLOCK-ENVGET-MISS-HIT-EXIT-SYNTAX
  imports SHERLOCK-ENVGET-POSITIVE-SYNTAX

  syntax EGTraversalWindow ::= egTraversalWindow(EGSnapshot, EGSnapshot, EGSnapshot, EGSnapshot, EGEvent, EGEvent, EGEvent) [klabel(egTraversalWindow)]
endmodule
