// Claim (d), the observable instance of the compiler-obligation template
// spec_compiled_observable (tools/run_claims.py). Generated; edit the template,
// not this file. The program add %a 1 -> %b ; impact 1 [%b] on a symbolic
// input A, with the public transcript input equal to the impacted value, has
// the observable result obs(ok, no outputs, public inputs [binding input 7,
// (A + 1) mod r], skips [skipNone()]) in the <observable> cell (zkir-vm.k
// `observable`) and its final memory in the modelled witness space with no
// unconstrained register. targetContract(P) is discharged separately.
requires "../semantics/zkir-symbolic.k"

module SPEC-COMPILED-OBSERVABLE
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%a", native()), .TypedIds), .IrTypes, false, (add(var("%a"), imm(1), "%b") ; impact(imm(1), (var("%b"), .Operands)) ; .Instrs)),
                preimage(ListItem(A:Int), 7, noComm(), .List, ListItem((A +Int 1) modInt #r), .List)) => .K </k>
        <observable> noObs() => obs(ok(), .List, ListItem(7) ListItem((A +Int 1) modInt #r), ListItem(skipNone())) </observable>
        <status> ok() => ?_ </status>
        <outputs> .List => ?_ </outputs>
        <pi> .List => ?_ </pi>
        <mem> .Map => ?_ </mem>
        <skips> .List => ?_ </skips>
        <pubInIdx> 0 => ?_ </pubInIdx> <pubOutIdx> 0 => ?_ </pubOutIdx> <privIdx> 0 => ?_ </privIdx>
        <outTypes> .IrTypes => ?_ </outTypes>
        <doComm> false => ?_ </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List) => ?_ </pre>
        <constraints> .List => ?_ </constraints>
        <chips> .Set => ?_ </chips>
        <verdicts> .List => ?_ </verdicts>
        <piIdx> 0 => ?_ </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List => .List </unconstrainedRegs>
    requires 0 <=Int A andBool A <Int #r
endmodule
