// Function-evaluation harness for unit tests (tools/unit_values.py): the
// program is any K term; krun normalises it. ZKIR-CONSTRAINTS is imported so
// that witnessSpace and wellTyped are evaluable on a hand-built memory.
requires "zkir-values.k"
requires "zkir-constants.k"
requires "zkir-hash.k"
requires "zkir-ops.k"
requires "zkir-constraints.k"

module ZKIR-TEST
  imports ZKIR-VALUES
  imports ZKIR-CONSTANTS
  imports ZKIR-HASH
  imports ZKIR-OPS
  imports ZKIR-CONSTRAINTS

  configuration <zkir-test> <k> $PGM:KItem </k> </zkir-test>
endmodule
