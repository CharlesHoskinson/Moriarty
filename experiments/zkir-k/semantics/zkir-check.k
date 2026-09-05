// Main module for milestone 2: parse (via pyk) and statically check a program.
// krun over a Program term leaves wfOk() or wfError(msg) in the <k> cell.
requires "zkir-syntax.k"

module ZKIR-CHECK
  imports ZKIR-WF

  configuration
    <zkir-check>
      <k> $PGM:Program </k>
    </zkir-check>

  rule <k> P:Program => wf(P) </k>
endmodule
