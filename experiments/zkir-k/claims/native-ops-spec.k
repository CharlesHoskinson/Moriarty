// Claims (b): the native arithmetic and control instructions, one module per
// claim, each on a one-instruction program over symbolic native inputs. The
// arithmetic claims state the output register and that every emitted gate
// holds; the constrain_to_boolean and assert claims state the status after
// the run on the success branch and on the failure branch.
requires "../semantics/zkir-symbolic.k"

module MUL-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%a", native()), typedId("%b", native()), .TypedIds), .IrTypes, false,
                        (mul(var("%a"), var("%b"), "%o") ; .Instrs)),
                preimage(ListItem(A:Int) ListItem(B:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%a" |-> native(A)) ("%b" |-> native(B)) ("%o" |-> native((A *Int B) modInt #r)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(A) ListItem(B), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%a", native())) ListItem(inputGate("%b", native())) ListItem(bindGate(0)) ListItem(gate(mul(var("%a"), var("%b"), "%o"))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%a", native()), holds()))
                            ListItem(verdict(inputGate("%b", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(mul(var("%a"), var("%b"), "%o")), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(ok(), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() </status>
    requires 0 <=Int A andBool A <Int #r andBool 0 <=Int B andBool B <Int #r
endmodule

module NEG-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%a", native()), .TypedIds), .IrTypes, false,
                        (neg(var("%a"), "%o") ; .Instrs)),
                preimage(ListItem(A:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%a" |-> native(A)) ("%o" |-> native((0 -Int A) modInt #r)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(A), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%a", native())) ListItem(bindGate(0)) ListItem(gate(neg(var("%a"), "%o"))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%a", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(neg(var("%a"), "%o")), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(ok(), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() </status>
    requires 0 <=Int A andBool A <Int #r
endmodule

module COPY-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%a", native()), .TypedIds), .IrTypes, false,
                        (copy(var("%a"), "%o") ; .Instrs)),
                preimage(ListItem(A:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%a" |-> native(A)) ("%o" |-> native(A)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(A), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%a", native())) ListItem(bindGate(0)) ListItem(gate(copy(var("%a"), "%o"))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%a", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(copy(var("%a"), "%o")), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(ok(), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() </status>
    requires 0 <=Int A andBool A <Int #r
endmodule

// cond_select with the bit 1 selects the first operand, with the bit 0 the
// second; the selected values are symbolic.
module COND-SELECT-1-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%a", native()), typedId("%b", native()), .TypedIds), .IrTypes, false,
                        (condSelect(imm(1), var("%a"), var("%b"), "%o") ; .Instrs)),
                preimage(ListItem(A:Int) ListItem(B:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%a" |-> native(A)) ("%b" |-> native(B)) ("%o" |-> native(A)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(A) ListItem(B), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%a", native())) ListItem(inputGate("%b", native())) ListItem(bindGate(0)) ListItem(gate(condSelect(imm(1), var("%a"), var("%b"), "%o"))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%a", native()), holds()))
                            ListItem(verdict(inputGate("%b", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(condSelect(imm(1), var("%a"), var("%b"), "%o")), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(ok(), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() </status>
    requires 0 <=Int A andBool A <Int #r andBool 0 <=Int B andBool B <Int #r
endmodule

module COND-SELECT-0-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%a", native()), typedId("%b", native()), .TypedIds), .IrTypes, false,
                        (condSelect(imm(0), var("%a"), var("%b"), "%o") ; .Instrs)),
                preimage(ListItem(A:Int) ListItem(B:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%a" |-> native(A)) ("%b" |-> native(B)) ("%o" |-> native(B)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(A) ListItem(B), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%a", native())) ListItem(inputGate("%b", native())) ListItem(bindGate(0)) ListItem(gate(condSelect(imm(0), var("%a"), var("%b"), "%o"))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%a", native()), holds()))
                            ListItem(verdict(inputGate("%b", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(condSelect(imm(0), var("%a"), var("%b"), "%o")), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(ok(), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() </status>
    requires 0 <=Int A andBool A <Int #r andBool 0 <=Int B andBool B <Int #r
endmodule

// constrain_to_boolean: a symbolic input that is a boolean passes, stated as
// two claims under one module, one per boolean value (asBool has one rule per
// value, and a disjunctive `requires` would need a case split the prover
// does not make); a symbolic input that is neither 0 nor 1 ends the run with
// the decoder's message, and the gate is violated on that witness.
module CONSTRAIN-TO-BOOLEAN-OK-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%c", native()), .TypedIds), .IrTypes, false,
                        (constrainToBoolean(var("%c")) ; .Instrs)),
                preimage(ListItem(C:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%c" |-> native(C)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(C), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%c", native())) ListItem(bindGate(0)) ListItem(gate(constrainToBoolean(var("%c")))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%c", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(constrainToBoolean(var("%c"))), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(ok(), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() </status>
    requires C ==Int 0

  claim <k> job(program(0, (typedId("%c", native()), .TypedIds), .IrTypes, false,
                        (constrainToBoolean(var("%c")) ; .Instrs)),
                preimage(ListItem(C:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%c" |-> native(C)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(C), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%c", native())) ListItem(bindGate(0)) ListItem(gate(constrainToBoolean(var("%c")))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%c", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(constrainToBoolean(var("%c"))), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(ok(), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() </status>
    requires C ==Int 1
endmodule

module CONSTRAIN-TO-BOOLEAN-FAIL-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%c", native()), .TypedIds), .IrTypes, false,
                        (constrainToBoolean(var("%c")) ; .Instrs)),
                preimage(ListItem(C:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%c" |-> native(C)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(C), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%c", native())) ListItem(bindGate(0)) ListItem(gate(constrainToBoolean(var("%c")))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%c", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(constrainToBoolean(var("%c"))),
                                             violated("constrain_to_boolean is not boolean: " +String Int2String(C)))) </verdicts>
        <witnessSpace> false </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(error("Expected boolean, found: " +String Int2String(C)), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() => error("Expected boolean, found: " +String Int2String(C)) </status>
    requires 2 <=Int C andBool C <Int #r
endmodule

// assert: a symbolic input equal to 1 passes; equal to 0 it fails the run
// with "Failed direct assertion" and the gate reports the zero; an input in
// [2, r) fails the run with the boolean decoder's message while the gate,
// which only demands a non-zero value, holds (finding 2), so the memory is in
// the modelled witness space although `preprocess` rejects the preimage.
module ASSERT-OK-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%c", native()), .TypedIds), .IrTypes, false,
                        (assert(var("%c")) ; .Instrs)),
                preimage(ListItem(C:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%c" |-> native(C)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(C), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%c", native())) ListItem(bindGate(0)) ListItem(gate(assert(var("%c")))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%c", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(assert(var("%c"))), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(ok(), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() </status>
    requires C ==Int 1
endmodule

module ASSERT-FAIL-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%c", native()), .TypedIds), .IrTypes, false,
                        (assert(var("%c")) ; .Instrs)),
                preimage(ListItem(C:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%c" |-> native(C)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(C), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%c", native())) ListItem(bindGate(0)) ListItem(gate(assert(var("%c")))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%c", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(assert(var("%c"))), violated("assert of zero"))) </verdicts>
        <witnessSpace> false </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(error("Failed direct assertion"), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() => error("Failed direct assertion") </status>
    requires C ==Int 0
endmodule

module ASSERT-NON-BOOLEAN-SPEC
  imports ZKIR-SYMBOLIC
  claim <k> job(program(0, (typedId("%c", native()), .TypedIds), .IrTypes, false,
                        (assert(var("%c")) ; .Instrs)),
                preimage(ListItem(C:Int), 42, noComm(), .List, .List, .List)) => .K </k>
        <mem> .Map => ("%c" |-> native(C)) </mem>
        <pi> .List => ListItem(42) </pi>
        <skips> .List </skips>
        <pubInIdx> 0 </pubInIdx> <pubOutIdx> 0 </pubOutIdx> <privIdx> 0 </privIdx>
        <outputs> .List </outputs>
        <outTypes> .IrTypes </outTypes>
        <doComm> false </doComm>
        <pre> preimage(.List, 0, noComm(), .List, .List, .List)
           => preimage(ListItem(C), 42, noComm(), .List, .List, .List) </pre>
        <constraints> .List => ListItem(inputGate("%c", native())) ListItem(bindGate(0)) ListItem(gate(assert(var("%c")))) </constraints>
        <chips> .Set </chips>
        <verdicts> .List => ListItem(verdict(inputGate("%c", native()), holds()))
                            ListItem(verdict(bindGate(0), holds()))
                            ListItem(verdict(gate(assert(var("%c"))), holds())) </verdicts>
        <witnessSpace> false => true </witnessSpace> <unconstrainedRegs> .List </unconstrainedRegs>
        <observable> noObs() => obs(error("Expected boolean, found: " +String Int2String(C)), .List, ListItem(42), .List) </observable>
        <piIdx> 0 => 1 </piIdx>
        <genMode> false </genMode> <needs> .List </needs> <strictDecode> false </strictDecode>
        <status> ok() => error("Expected boolean, found: " +String Int2String(C)) </status>
    requires 2 <=Int C andBool C <Int #r
endmodule
