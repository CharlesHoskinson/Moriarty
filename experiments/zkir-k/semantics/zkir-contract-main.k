// Main module for the target contract (plan-iter3 M5a, M5b): krun over a
// Program term leaves targetContract(P), the list of tier-one obligations, in
// the <k> cell. Like ZKIR-CHECK it imports the extension syntax and the
// extension's static and contract arms, so `zkir_kast.py contract --ext` uses
// the same definition; the preprocessor decides which surface a file may use.
// Nothing from ZKIR-VM or ZKIR-OPS is imported: the contract reads the program
// only. ZKIR-VALUES and ZKIR-EXT-VALUES are imported for one purpose, the
// strict decoder that circuit.static.constant applies to a load_constant
// encoding; the tiers that need a preimage run through ZKIR-VM in
// tools/zkir_kast.py (`contract --preimage`).
requires "zkir-contract.k"
requires "zkir-ext.k"

module ZKIR-CONTRACT-MAIN
  imports ZKIR-CONTRACT
  imports ZKIR-EXT-CONTRACT

  configuration
    <zkir-contract>
      <k> $PGM:Program </k>
    </zkir-contract>

  rule <k> P:Program => targetContract(P) </k>
endmodule
