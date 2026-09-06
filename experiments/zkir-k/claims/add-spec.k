// Claim (a), the baseline recovered from the second iteration: one `add` of
// two symbolic native inputs. After job(P, Pre) the run has status ok, the
// output register holds (A + B) mod r, the public-input vector is the binding
// input alone, and both emitted gates (binding input, add) hold on the witness.
// Every cell of the configuration is pinned: an unmentioned cell would be
// abstracted to a variable and the decoder could not choose a rule.
requires "../semantics/zkir-symbolic.k"

module ADD-SPEC
  imports ZKIR-SYMBOLIC

  claim <k> job(program(0, (typedId("%a", native()), typedId("%b", native()), .TypedIds), .IrTypes, false,
                        (add(var("%a"), var("%b"), "%o") ; .Instrs)),
                preimage(ListItem(A:Int) ListItem(B:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%a" |-> native(A)) ("%b" |-> native(B)) ("%o" |-> native((A +Int B) modInt #r)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(A) ListItem(B), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%a", native())) ListItem(inputGate("%b", native())) ListItem(bindGate(0)) ListItem(gate(add(var("%a"), var("%b"), "%o"))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%a", native()), holds()))
                            ListItem(verdict(inputGate("%b", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(add(var("%a"), var("%b"), "%o")), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => ?_ </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() </status>
    requires 0 <=Int A andBool A <Int #r andBool 0 <=Int B andBool B <Int #r
endmodule
