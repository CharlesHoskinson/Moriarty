// Claim (e): the communications commitment on a symbolic input. The program
// copy %a -> %o with do_communications_commitment true, on a preimage whose
// commitment C is transientCommit([A], 5) with the opening 5, ends with status
// ok, pi = [42, C], and every emitted gate holds: the commitment gate
// recomputes poseidonHash([5, A]) over the re-encoded input register and the
// (empty) outputs and meets the witness-side transientCommit([A], 5) because
// transientCommit is a definitional alias that unfolds to the same
// poseidonHash term (zkir-hash.k; the alias carries no [concrete]). The
// digest itself stays uninterpreted. This is the shape of every Moriarty
// artifact, all of which set the flag.
requires "../semantics/zkir-symbolic.k"

module COMMITMENT-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%a", native()), .TypedIds), .IrTypes, true,
                        (copy(var("%a"), "%o") ; .Instrs)),
                preimage(ListItem(A:Int), 42, comm(C:Int, 5), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%a" |-> native(A)) ("%o" |-> native(A)) </mem>
        <pi> .List => ListItem(42) ListItem(C) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false => true </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(A), 42, comm(C, 5), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%a", native())) ListItem(bindGate(0)) ListItem(commGate(1, (typedId("%a", native()), .TypedIds), 5)) ListItem(gate(copy(var("%a"), "%o"))) </constraints>
        <chips> .Set => SetItem("poseidon") </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%a", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(commGate(1, (typedId("%a", native()), .TypedIds), 5), holds()))
                            ListItem(verdict(gate(copy(var("%a"), "%o")), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(ok(), .List, ListItem(42) ListItem(C), .List) </observable>
        <piIdx> 0 => 2 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() </status>
    requires 0 <=Int A andBool A <Int #r andBool C ==Int transientCommit(ListItem(A), 5)
endmodule
