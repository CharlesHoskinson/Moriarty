// Main module for the target contract (plan-iter3 M5a): krun over a Program
// term leaves targetContract(P), the list of tier-one obligations, in the <k>
// cell. Like ZKIR-CHECK it imports the extension syntax and the extension's
// static arms, so `zkir_kast.py contract --ext` uses the same definition; the
// preprocessor decides which surface a file may use. Nothing from ZKIR-VM or
// ZKIR-OPS is imported: the contract reads the program only.
requires "zkir-contract.k"
requires "zkir-ext.k"

module ZKIR-CONTRACT-MAIN
  imports ZKIR-CONTRACT
  imports ZKIR-EXT-STATIC

  configuration
    <zkir-contract>
      <k> $PGM:Program </k>
    </zkir-contract>

  rule <k> P:Program => targetContract(P) </k>
endmodule
