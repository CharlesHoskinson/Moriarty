// Function-evaluation harness for unit tests (tools/unit_values.py): the
// program is any K term; krun normalises it.
requires "zkir-values.k"
requires "zkir-constants.k"
requires "zkir-hash.k"
requires "zkir-ops.k"

module ZKIR-TEST
  imports ZKIR-VALUES
  imports ZKIR-CONSTANTS
  imports ZKIR-HASH
  imports ZKIR-OPS

  configuration <zkir-test> <k> $PGM:KItem </k> </zkir-test>
endmodule
