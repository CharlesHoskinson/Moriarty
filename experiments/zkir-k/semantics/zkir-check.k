// Main module for milestone 2: parse (via pyk) and statically check a program.
// krun over a Program term leaves wfOk() or wfError(msg) in the <k> cell.
// The extension syntax and its static-check arms (ZKIR-EXT-WF) are imported
// so that `zkir_kast.py check --ext` uses the same definition; the
// preprocessor decides which surface a file may use.
// The target contract (ZKIR-CONTRACT-MAIN, zkir-contract-main.k) is a separate
// main module over the same syntax and evaluates targetContract instead of wf.
requires "zkir-syntax.k"
requires "zkir-ext.k"

module ZKIR-CHECK
  imports ZKIR-WF
  imports ZKIR-EXT-WF

  configuration
    <zkir-check>
      <k> $PGM:Program </k>
    </zkir-check>

  rule <k> P:Program => wf(P) </k>
endmodule
