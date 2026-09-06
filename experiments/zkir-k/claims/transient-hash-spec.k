// Claim (c): a three-instruction program ending in transient_hash over
// symbolic native inputs. The hash register equals poseidonHash applied to
// the list of the hashed registers' values, with poseidonHash uninterpreted
// (its defining rule is [concrete]); the mul register is stated in reduced
// form, ((A + B) * C) mod r, which the modInt congruence lemma of
// ZKIR-SYMBOLIC identifies with the nested form the semantics builds. Every
// emitted gate holds on the witness, including the transient_hash gate, whose
// expected value is the same uninterpreted application.
requires "../semantics/zkir-symbolic.k"

module TRANSIENT-HASH-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%a", native()), typedId("%b", native()), typedId("%c", native()), .TypedIds), .IrTypes, false,
                        (add(var("%a"), var("%b"), "%s")
                       ; mul(var("%s"), var("%c"), "%m")
                       ; transientHash((var("%m"), var("%a"), .Operands), "%h")
                       ; .Instrs)),
                preimage(ListItem(A:Int) ListItem(B:Int) ListItem(C:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%a" |-> native(A)) ("%b" |-> native(B)) ("%c" |-> native(C))
                      ("%s" |-> native((A +Int B) modInt #r))
                      ("%m" |-> native(((A +Int B) *Int C) modInt #r))
                      ("%h" |-> native(poseidonHash(ListItem(((A +Int B) *Int C) modInt #r) ListItem(A)))) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(A) ListItem(B) ListItem(C), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%a", native())) ListItem(inputGate("%b", native())) ListItem(inputGate("%c", native())) ListItem(bindGate(0))
                               ListItem(gate(add(var("%a"), var("%b"), "%s")))
                               ListItem(gate(mul(var("%s"), var("%c"), "%m")))
                               ListItem(gate(transientHash((var("%m"), var("%a"), .Operands), "%h"))) </constraints>
        <chips> .Set => SetItem("poseidon") </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%a", native()), holds()))
                            ListItem(verdict(inputGate("%b", native()), holds()))
                            ListItem(verdict(inputGate("%c", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(add(var("%a"), var("%b"), "%s")), holds()))
                            ListItem(verdict(gate(mul(var("%s"), var("%c"), "%m")), holds()))
                            ListItem(verdict(gate(transientHash((var("%m"), var("%a"), .Operands), "%h")), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(ok(), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() </status>
    requires 0 <=Int A andBool A <Int #r andBool 0 <=Int B andBool B <Int #r andBool 0 <=Int C andBool C <Int #r
endmodule
