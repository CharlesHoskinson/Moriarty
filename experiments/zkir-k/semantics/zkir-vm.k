// ZKIR v3 virtual machine: the witness computation (`IrSource::preprocess`,
// zkir-v3/src/ir_vm.rs at midnight-ledger 92e8bdd3) and, in the same rewrite,
// the constraint set that `Relation::circuit` would synthesise for the same
// instruction (plan decisions CLM-0704 .. CLM-0706, CLM-0720).
//
// State (one cell per component of the seven-tuple on the VM wiki page):
//   <mem>        registers, String |-> Value; like `preprocess`, a later write
//                overwrites (single assignment is a static check, see ZKIR-WF)
//   <pi>         public-input vector (binding input, optional commitment, impacts)
//   <skips>      pi_skips: skipNone() for an active impact, skipSome(n) for an inactive one
//   <pubInIdx>, <pubOutIdx>, <privIdx>   transcript cursors
//   <outputs>    values returned by `output`, in order
//   <constraints> instruction-level relations emitted by every instruction,
//                whether or not the witness computation is still alive
//   <status>     ok(), error(msg) or panic(msg): the first witness failure freezes
//                the witness cells; the remaining instructions still emit
//                constraints. panic() marks the places where the Rust code aborts
//                (index out of range, failed assertion) rather than returning Err.
//   <verdicts>   one verdict per emitted constraint, evaluated on the final witness
//   <witnessSpace>       witnessSpace(verdicts, mem) of ZKIR-CONSTRAINTS: the final
//                memory is a point of the modelled witness space
//   <unconstrainedRegs>  the registers whose assigning relation is unconstrained
//   <observable> the observable result of the run, obs(status, encoded outputs,
//                public inputs): [[P]](pre) as the compiler-correctness statement
//                quotes it (observable(...), after #witnessSpace)
//
// Entry points: job(P, Pre) runs P exactly as `IrSource::preprocess` would,
// including its run-time checks, on any program `IrSource::load` accepts;
// checkedJob(P, Pre) first evaluates the static check `wf` (ZKIR-WF) and
// reports its failure as the run's error, so it accepts strictly fewer programs.
//
// A run is job(program, preimage). The preimage mirrors `ProofPreimage`.
requires "zkir-syntax.k"
requires "zkir-ops.k"
requires "zkir-constraints.k"

module ZKIR-VM-SYNTAX
  imports ZKIR-SYNTAX
  imports LIST
  imports INT

  syntax Preimage ::= preimage(List, Int, CommOpt, List, List, List) [symbol(preimage)]
                      // inputs, binding_input, communications_commitment, private_transcript,
                      // public_transcript_inputs, public_transcript_outputs
  syntax CommOpt  ::= noComm()        [symbol(noComm)]
                    | comm(Int, Int)  [symbol(comm)]    // (commitment, opening randomness)
  syntax Job ::= job(Program, Preimage)         [symbol(job)]
               | checkedJob(Program, Preimage)  [symbol(checkedJob)]   // wf first
               | genJob(Program, Preimage)      [symbol(genJob)]       // generation mode, see <genMode>
endmodule

module ZKIR-VM
  imports ZKIR-VM-SYNTAX
  imports ZKIR-WF
  imports ZKIR-OPS
  imports ZKIR-CONSTRAINTS
  imports MAP
  imports SET
  imports K-EQUAL

  syntax Status ::= ok() [symbol(ok)] | error(String) [symbol(error)] | panic(String) [symbol(panic)]
  syntax Skip   ::= skipNone() [symbol(skipNone)] | skipSome(Int) [symbol(skipSome)]

  // Constraints (sort Constraint, ZKIR-CONSTRAINTS): the in-circuit relation
  // of one instruction over register names and immediates; piGate(i, g, x)
  // states pi[i] = g ? x : 0; guardGate(g) states g is boolean; bindGate(i)
  // pins the binding input; commGate(i, inputs, rand) pins the commitment;
  // outputGate(vals, types) checks the output signature.

  configuration
    <zkir>
      <k> $PGM:Job </k>
      <mem> .Map </mem>
      <pi> .List </pi>
      <skips> .List </skips>
      <pubInIdx> 0 </pubInIdx>
      <pubOutIdx> 0 </pubOutIdx>
      <privIdx> 0 </privIdx>
      <outputs> .List </outputs>
      <outTypes> .IrTypes </outTypes>
      <doComm> false </doComm>
      <pre> preimage(.List, 0, noComm(), .List, .List, .List) </pre>
      <constraints> .List </constraints>
      <chips> .Set </chips>
      <verdicts> .List </verdicts>
      <witnessSpace> false </witnessSpace>
      <unconstrainedRegs> .List </unconstrainedRegs>
      <observable> noObs() </observable>
      <piIdx> 0 </piIdx>          // public inputs emitted so far on the constraint side
      <genMode> false </genMode>
      <needs> .List </needs>
      <strictDecode> false </strictDecode>   // true for the ZKIR-EXT surface (canonical-form check on decoding)
      <status> ok() </status>
    </zkir>

  // Generation mode (test harness only): a transcript that runs out yields the
  // type's default value and records what was needed; impact values and the
  // communications commitment are recorded instead of checked. The harness
  // (tools/diff_test.py) uses this to build a consistent preimage before the
  // real run.
  syntax Need ::= needPubOut(IrType) [symbol(needPubOut)]
                | needPriv(IrType)   [symbol(needPriv)]
                | needPubIn(Int)     [symbol(needPubIn)]
                | needComm(Int)      [symbol(needComm)]
  rule <k> genJob(P, Pre) => job(P, Pre) ... </k> <genMode> _ => true </genMode>

  // --- Start: decode inputs, seed the public-input vector ------------------------------
  rule <k> checkedJob(P, Pre) => #wfGate(wf(P)) ~> job(P, Pre) ... </k>
  syntax KItem ::= #wfGate(WfResult)
  rule <k> #wfGate(wfOk()) => .K ... </k>
  rule <k> #wfGate(wfError(S)) => .K ... </k> <status> ok() => error("well-formedness: " +String S) </status>
  // a failed static check ends the run before anything executes; the
  // observable result is still recorded
  rule <k> job(_, _) => #observable ... </k> <status> error(_) </status>

  // the circuit assigns every declared input from the witness before the
  // binding input: one inputGate per input, emitted whether or not decoding succeeds
  rule <k> job(program(Mi, Ins, Outs, DoComm, Is), Pre) => #loadInputs(#preInputs(Pre), Ins, 0) ~> #seedPi(Ins) ~> Is ~> #verdicts </k>
       <pre> _ => Pre </pre> <status> ok() </status>
       <chips> _ => usedChips(program(Mi, Ins, Outs, DoComm, Is)) </chips>
       <outTypes> _ => Outs </outTypes>
       <doComm> _ => DoComm </doComm>
       <constraints> Cs => Cs #inputGates(Ins) </constraints>
  syntax List ::= #inputGates(TypedIds) [function, total]
  rule #inputGates(.TypedIds) => .List
  rule #inputGates((typedId(N, T), Rest)) => ListItem(inputGate(N, T)) #inputGates(Rest)

  syntax List ::= #preInputs(Preimage) [function, total]
  rule #preInputs(preimage(I, _, _, _, _, _)) => I
  syntax Int ::= #preBinding(Preimage) [function, total]
  rule #preBinding(preimage(_, B, _, _, _, _)) => B
  syntax CommOpt ::= #preComm(Preimage) [function, total]
  rule #preComm(preimage(_, _, C, _, _, _)) => C
  syntax List ::= #prePrivate(Preimage) [function, total]
  rule #prePrivate(preimage(_, _, _, P, _, _)) => P
  syntax List ::= #prePubIn(Preimage) [function, total]
  rule #prePubIn(preimage(_, _, _, _, P, _)) => P
  syntax List ::= #prePubOut(Preimage) [function, total]
  rule #prePubOut(preimage(_, _, _, _, _, P)) => P

  // Inputs are decoded and bound one at a time, as `preprocess` does, so a
  // failure leaves the earlier inputs in memory.
  syntax KItem ::= #loadInputs(List, TypedIds, Int) | #seedPi(TypedIds)
  rule <k> #loadInputs(Raw, .TypedIds, Idx) => .K ... </k> <status> ok() </status> requires Idx ==Int size(Raw)
  rule <k> #loadInputs(Raw, .TypedIds, Idx) => #fail("Expected " +String Int2String(Idx) +String " raw inputs, received " +String Int2String(size(Raw))) ... </k> <status> ok() </status>
    requires Idx =/=Int size(Raw)
  rule <k> #loadInputs(Raw, (typedId(N, T), _), Idx) => #fail("Not enough raw inputs: ran out at index " +String Int2String(Idx) +String " while decoding " +String N) ... </k> <status> ok() </status>
    requires Idx +Int encodedLen(T) >Int size(Raw)
  rule <k> #loadInputs(Raw, (typedId(N, T), Rest), Idx) => #put(N, #toValue(decodeStrict(range(Raw, Idx, size(Raw) -Int Idx -Int encodedLen(T)), T, Strict))) ~> #loadInputs(Raw, Rest, Idx +Int encodedLen(T)) ... </k>
       <strictDecode> Strict </strictDecode> <status> ok() </status>
    requires Idx +Int encodedLen(T) <=Int size(Raw)
  rule <k> #loadInputs(_, _, _) => .K ... </k> <status> error(_) </status>
  rule <k> #loadInputs(_, _, _) => .K ... </k> <status> panic(_) </status>

  // pis = [binding_input] ++ [commitment if do_communications_commitment]
  // The constraint side always pins pi[0] (and pi[1] under the commitment
  // flag); the witness side seeds them only while alive, as `preprocess`
  // returns before creating `pis` when input decoding failed and before
  // pushing the commitment when it is absent.
  rule <k> #seedPi(_) => .K ... </k> <pre> Pre </pre> <doComm> false </doComm> <status> ok() </status>
       <pi> _ => ListItem(#preBinding(Pre)) </pi> <piIdx> _ => 1 </piIdx>
       <constraints> Cs => Cs ListItem(bindGate(0)) </constraints>
  rule <k> #seedPi(Ins) => .K ... </k> <pre> Pre </pre> <doComm> true </doComm> <status> ok() </status>
       <pi> _ => ListItem(#preBinding(Pre)) ListItem(#commValue(#preComm(Pre))) </pi> <piIdx> _ => 2 </piIdx>
       <constraints> Cs => Cs ListItem(bindGate(0)) ListItem(commGate(1, Ins, #commRand(#preComm(Pre)))) </constraints>
    requires #preComm(Pre) =/=K noComm()
  rule <k> #seedPi(Ins) => #fail("Expected communications commitment") ... </k> <pre> Pre </pre> <doComm> true </doComm> <status> ok() </status>
       <pi> _ => ListItem(#preBinding(Pre)) </pi> <piIdx> _ => 2 </piIdx>
       <constraints> Cs => Cs ListItem(bindGate(0)) ListItem(commGate(1, Ins, 0)) </constraints>
    requires #preComm(Pre) ==K noComm()
  rule <k> #seedPi(_) => .K ... </k> <doComm> false </doComm> <status> St </status> <piIdx> _ => 1 </piIdx>
       <constraints> Cs => Cs ListItem(bindGate(0)) </constraints>
    requires St =/=K ok()
  rule <k> #seedPi(Ins) => .K ... </k> <pre> Pre </pre> <doComm> true </doComm> <status> St </status> <piIdx> _ => 2 </piIdx>
       <constraints> Cs => Cs ListItem(bindGate(0)) ListItem(commGate(1, Ins, #commRand(#preComm(Pre)))) </constraints>
    requires St =/=K ok()
  syntax Int ::= #commValue(CommOpt) [function, total]
  rule #commValue(comm(C, _)) => C
  rule #commValue(noComm()) => 0

  // --- Instruction sequencing ------------------------------------------------------------
  rule <k> (I:Instr ; Is:Instrs) => I ~> Is ... </k>
  rule <k> .Instrs => #finish ... </k>

  // Every instruction emits its constraint, then runs its witness half (if alive).
  // The witness half is written as #exec(I); when the status is an error only
  // the constraint half fires.
  syntax KItem ::= #exec(Instr)
  rule <k> I:Instr => #exec(I) ... </k> <constraints> Cs => Cs ListItem(gate(I)) </constraints>
    requires notBool isSpecialEmit(I)
  rule <k> #exec(_) => .K ... </k> <status> error(_) </status>
  rule <k> #exec(_) => .K ... </k> <status> panic(_) </status>

  // impact and output carry state into their gates: the absolute public-input
  // positions (from <piIdx>, advanced at emission time so that they never
  // depend on how far the witness got) and the output signature.
  syntax Bool ::= isSpecialEmit(Instr) [function, total]
  rule isSpecialEmit(impact(_, _)) => true
  rule isSpecialEmit(output(_)) => true
  rule isSpecialEmit(_) => false [owise]
  rule <k> impact(G, Xs) => #exec(impact(G, Xs)) ... </k>
       <piIdx> N => N +Int lenOperands(Xs) </piIdx>
       <constraints> Cs => Cs ListItem(guardGate(G)) #piGates(N, G, Xs) </constraints>
  rule <k> output(Vs) => #exec(output(Vs)) ... </k> <outTypes> Ts </outTypes>
       <constraints> Cs => Cs ListItem(outputGate(Vs, Ts)) </constraints>

  syntax List ::= #piGates(Int, Operand, Operands) [function, total]
  rule #piGates(_, _, .Operands) => .List
  rule #piGates(N, G, (X, Xs)) => ListItem(piGate(N, G, X)) #piGates(N +Int 1, G, Xs)

  // --- Operand resolution ----------------------------------------------------------------
  syntax ValueResult ::= resolve(Operand, Map) [function, total]
  rule resolve(imm(I), _) => vOk(native(I))
  rule resolve(var(X), M) => vOk({M[X]}:>Value) requires X in_keys(M)
  rule resolve(var(X), M) => vErr("variable not found: Identifier(\"" +String X +String "\")") requires notBool X in_keys(M)

  syntax BoolResult ::= resolveBool(Operand, Map) [function, total]
  rule resolveBool(O, M) => #toBool(resolve(O, M))
  syntax BoolResult ::= #toBool(ValueResult) [function, total]
  rule #toBool(vErr(S)) => boolErr(S)
  rule #toBool(vOk(V)) => asBool(V)

  syntax ValuesResult ::= vsOk(List) | vsErr(String)
  syntax ValuesResult ::= resolveAll(Operands, Map) [function, total]
  rule resolveAll(.Operands, _) => vsOk(.List)
  rule resolveAll((O, Os), M) => #consValue(resolve(O, M), resolveAll(Os, M))
  syntax ValuesResult ::= #consValue(ValueResult, ValuesResult) [function, total]
  rule #consValue(vErr(S), _) => vsErr(S)
  rule #consValue(vOk(_), vsErr(S)) => vsErr(S)
  rule #consValue(vOk(V), vsOk(L)) => vsOk(ListItem(V) L)

  // Write helpers: #put(name, result) stores or fails.
  syntax KItem ::= #put(String, ValueResult) | #put2(String, String, PairResult) | #fail(String)
  rule <k> #put(_, vErr(S)) => #fail(S) ... </k>
  rule <k> #put(X, vOk(V)) => .K ... </k> <mem> M => M[X <- V] </mem>
  rule <k> #put2(_, _, pErr(S)) => #fail(S) ... </k>
  rule <k> #put2(X, Y, pOk(V, W)) => .K ... </k> <mem> M => M[X <- V][Y <- W] </mem>
  rule <k> #fail(S) => .K ... </k> <status> ok() => error(S) </status>
  rule <k> #put(_, vPanic(S)) => #panicNow(S) ... </k>
  syntax KItem ::= #panicNow(String)
  rule <k> #panicNow(S) => .K ... </k> <status> ok() => panic(S) </status>

  // Resolve and convert operands one at a time, in program order, so the
  // first failing operand determines the error as in `preprocess`.
  syntax IntsResult ::= resolveNatives(Operands, Map) [function, total]
  rule resolveNatives(.Operands, _) => isOk(.List)
  rule resolveNatives((O, Os), M) => #rn(resolve(O, M), Os, M)
  syntax IntsResult ::= #rn(ValueResult, Operands, Map) [function, total]
  rule #rn(vErr(S), _, _) => isErr(S)
  rule #rn(vPanic(S), _, _) => isErr(S)
  rule #rn(vOk(native(X)), Os, M) => #consNative(iOk(X), resolveNatives(Os, M))
  rule #rn(vOk(V), _, _) => isErr("cannot convert " +String tn(V) +String " to Native") [owise]
  syntax KItem ::= #check(CheckResult)
  rule <k> #check(cOk()) => .K ... </k>
  rule <k> #check(cErr(S)) => #fail(S) ... </k>

  // --- Witness semantics, one rule per instruction (alive only) --------------------------
  rule <k> #exec(add(A, B, O)) => #put(O, #bin(resolve(A, M), resolve(B, M), "add")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(mul(A, B, O)) => #put(O, #bin(resolve(A, M), resolve(B, M), "mul")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(testEq(A, B, O)) => #put(O, #bin(resolve(A, M), resolve(B, M), "test_eq")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(ecMul(A, S, O)) => #put(O, #bin(resolve(A, M), resolve(S, M), "ec_mul")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(fromCoordinates(X, Y, O)) => #put(O, #bin(resolve(X, M), resolve(Y, M), "from_coordinates")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(bytes32FromLowHigh(L, H, O)) => #put(O, #bin(resolve(L, M), resolve(H, M), "bytes32_from_low_high")) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #bin(ValueResult, ValueResult, String) [function, total]
  rule #bin(vErr(S), _, _) => vErr(S)
  rule #bin(vOk(_), vErr(S), _) => vErr(S)
  rule #bin(vOk(A), vOk(B), "add") => addV(A, B)
  rule #bin(vOk(A), vOk(B), "mul") => mulV(A, B)
  rule #bin(vOk(A), vOk(B), "test_eq") => testEqV(A, B)
  rule #bin(vOk(A), vOk(B), "ec_mul") => ecMulV(A, B)
  rule #bin(vOk(A), vOk(B), "from_coordinates") => fromCoordinatesV(A, B)
  rule #bin(vOk(A), vOk(B), "bytes32_from_low_high") => bytes32FromLowHighV(A, B)
  rule #bin(vOk(_), vOk(_), Op) => vErr("unknown binary op " +String Op) [owise]

  rule <k> #exec(neg(A, O)) => #put(O, #un(resolve(A, M), "neg")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(inv(A, O)) => #put(O, #un(resolve(A, M), "inv")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(copy(A, O)) => #put(O, resolve(A, M)) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(ecMulGenerator(S, O)) => #put(O, #un(resolve(S, M), "ec_mul_generator")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(intoBytes32(A, O)) => #put(O, #un(resolve(A, M), "into_bytes32")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(reverseBytes(A, O)) => #put(O, #un(resolve(A, M), "reverse_bytes")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(jubjubScalarFromNative(A, O)) => #put(O, #un(resolve(A, M), "jubjub_scalar_from_native")) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #un(ValueResult, String) [function, total]
  rule #un(vErr(S), _) => vErr(S)
  rule #un(vOk(A), "neg") => negV(A)
  rule #un(vOk(A), "inv") => invV(A)
  rule #un(vOk(A), "ec_mul_generator") => ecMulGeneratorV(A)
  rule #un(vOk(A), "into_bytes32") => intoBytes32V(A)
  rule #un(vOk(A), "reverse_bytes") => #reverse(asBytes32(A))
  rule #un(vOk(A), "jubjub_scalar_from_native") => #jjScalar(asNative(A))
  rule #un(vOk(_), Op) => vErr("unknown unary op " +String Op) [owise]
  syntax ValueResult ::= #reverse(BytesResult) [function, total]
  rule #reverse(bErr(S)) => vErr(S)
  rule #reverse(bOk(B)) => vOk(bytes32(reverseBytes(B)))
  syntax ValueResult ::= #jjScalar(IntResult) [function, total]
  rule #jjScalar(iErr(S)) => vErr(S)
  rule #jjScalar(iOk(X)) => vOk(jubjubScalar(X modInt #rJ))

  rule <k> #exec(not(A, O)) => #put(O, #notV(resolveBool(A, M))) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #notV(BoolResult) [function, total]
  rule #notV(boolErr(S)) => vErr(S)
  rule #notV(boolOk(B)) => vOk(native(bool2Word(notBool B)))

  rule <k> #exec(assert(C)) => #assert(resolveBool(C, M)) ... </k> <mem> M </mem> <status> ok() </status>
  syntax KItem ::= #assert(BoolResult)
  rule <k> #assert(boolErr(S)) => #fail(S) ... </k>
  rule <k> #assert(boolOk(true)) => .K ... </k>
  rule <k> #assert(boolOk(false)) => #fail("Failed direct assertion") ... </k>

  rule <k> #exec(constrainToBoolean(V)) => #check(#boolCheck(resolveBool(V, M))) ... </k> <mem> M </mem> <status> ok() </status>
  syntax CheckResult ::= #boolCheck(BoolResult) [function, total]
  rule #boolCheck(boolErr(S)) => cErr(S)
  rule #boolCheck(boolOk(_)) => cOk()

  rule <k> #exec(constrainBits(V, N)) => #check(#bitsCheck(resolve(V, M), N)) ... </k> <mem> M </mem> <status> ok() </status>
  syntax CheckResult ::= #bitsCheck(ValueResult, Int) [function, total]
  rule #bitsCheck(vErr(S), _) => cErr(S)
  rule #bitsCheck(vOk(V), N) => checkBits(V, N)

  rule <k> #exec(constrainEq(A, B)) => #check(#eqCheck(resolve(A, M), resolve(B, M))) ... </k> <mem> M </mem> <status> ok() </status>
  syntax CheckResult ::= #eqCheck(ValueResult, ValueResult) [function, total]
  rule #eqCheck(vErr(S), _) => cErr(S)
  rule #eqCheck(vOk(_), vErr(S)) => cErr(S)
  rule #eqCheck(vOk(A), vOk(B)) => constrainEqV(A, B)

  rule <k> #exec(condSelect(Bit, A, B, O)) => #put(O, #sel(resolveBool(Bit, M), resolve(A, M), resolve(B, M))) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #sel(BoolResult, ValueResult, ValueResult) [function, total]
  rule #sel(boolErr(S), _, _) => vErr(S)
  rule #sel(boolOk(_), vErr(S), _) => vErr(S)
  rule #sel(boolOk(_), vOk(_), vErr(S)) => vErr(S)
  rule #sel(boolOk(Bit), vOk(A), vOk(B)) => selectV(Bit, A, B)

  rule <k> #exec(encode(X, Os)) => #encode(resolve(X, M), Os) ... </k> <mem> M </mem> <status> ok() </status>
  syntax KItem ::= #encode(ValueResult, Ids)
  rule <k> #encode(vErr(S), _) => #fail(S) ... </k>
  rule <k> #encode(vOk(V), Os) => #fail("Unexpected output length of encode instruction: " +String tn(V)) ... </k>
    requires size(encodeValue(V)) =/=Int lenIds(Os)
  rule <k> #encode(vOk(V), Os) => #bindEncoded(Os, encodeValue(V)) ... </k>
    requires size(encodeValue(V)) ==Int lenIds(Os)
  syntax KItem ::= #bindEncoded(Ids, List)
  rule <k> #bindEncoded(.Ids, _) => .K ... </k>
  rule <k> #bindEncoded((O, Os), ListItem(X) L) => #bindEncoded(Os, L) ... </k> <mem> M => M[O <- native(X)] </mem>

  rule <k> #exec(intoCoordinates(P, X, Y)) => #put2(X, Y, #coords(resolve(P, M))) ... </k> <mem> M </mem> <status> ok() </status>
  syntax PairResult ::= #coords(ValueResult) [function, total]
  rule #coords(vErr(S)) => pErr(S)
  rule #coords(vOk(V)) => intoCoordinatesV(V)

  rule <k> #exec(bytes32IntoLowHigh(B, L, H)) => #put2(L, H, #lowHigh(resolve(B, M))) ... </k> <mem> M </mem> <status> ok() </status>
  syntax PairResult ::= #lowHigh(ValueResult) [function, total]
  rule #lowHigh(vErr(S)) => pErr(S)
  rule #lowHigh(vOk(V)) => #lowHigh2(asBytes32(V))
  syntax PairResult ::= #lowHigh2(BytesResult) [function, total]
  rule #lowHigh2(bErr(S)) => pErr(S)
  rule #lowHigh2(bOk(B)) => bytes32IntoLowHighV(B)

  rule <k> #exec(fromBytes32(B, T, O)) => #put(O, #fromB32(resolve(B, M), T)) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #fromB32(ValueResult, IrType) [function, total]
  rule #fromB32(vErr(S), _) => vErr(S)
  rule #fromB32(vOk(V), T) => #fromB32b(asBytes32(V), T)
  syntax ValueResult ::= #fromB32b(BytesResult, IrType) [function, total]
  rule #fromB32b(bErr(S), _) => vErr(S)
  rule #fromB32b(bOk(B), T) => fromBytes32V(T, B)

  // div_mod_power_of_two: the two run-time checks of `preprocess` come first
  rule <k> #exec(divModPowerOfTwo(_, _, Os)) => #fail("DivModPowerOfTwo requires exactly 2 outputs") ... </k> <status> ok() </status>
    requires lenIds(Os) =/=Int 2
  rule <k> #exec(divModPowerOfTwo(_, N, Os)) => #fail("Excessive bit count") ... </k> <status> ok() </status>
    requires lenIds(Os) ==Int 2 andBool N >Int #frBytesStored *Int 8
  rule <k> #exec(divModPowerOfTwo(V, N, (Q, (R, .Ids)))) => #divMod(resolve(V, M), N, Q, R) ... </k> <mem> M </mem> <status> ok() </status>
    requires N <=Int #frBytesStored *Int 8
  syntax KItem ::= #divMod(ValueResult, Int, String, String)
  rule <k> #divMod(vErr(S), _, _, _) => #fail(S) ... </k>
  rule <k> #divMod(vOk(native(X)), N, Q, R) => .K ... </k> <mem> M => M[Q <- native(highBits(X, N))][R <- native(lowBits(X, N))] </mem>
  rule <k> #divMod(vOk(V), _, _, _) => #fail("cannot convert " +String tn(V) +String " to Native") ... </k> requires notBool isNative(V)

  // reconstitute_field: modulus < 2^bits, divisor < 2^(255-bits), and the
  // combination must not exceed r - 1 (compared bit by bit against r - 1).
  rule <k> #exec(reconstituteField(_, _, N, _)) => #fail("Excessive bit count") ... </k> <status> ok() </status>
    requires N >Int #frBytesStored *Int 8
  rule <k> #exec(reconstituteField(D, Mo, N, O)) => #recon(resolve(Mo, M), D, N, O) ... </k> <mem> M </mem> <status> ok() </status>
    requires N <=Int #frBytesStored *Int 8
  syntax KItem ::= #recon(ValueResult, Operand, Int, String)
  rule <k> #recon(vErr(S), _, _, _) => #fail(S) ... </k>
  rule <k> #recon(vOk(MV), D, N, O) => #recon2(checkBits(MV, N), D, MV, N, O) ... </k>
  syntax KItem ::= #recon2(CheckResult, Operand, Value, Int, String)
  rule <k> #recon2(cErr(S), _, _, _, _) => #fail(S) ... </k>
  rule <k> #recon2(cOk(), D, MV, N, O) => #recon3(resolve(D, M), MV, N, O) ... </k> <mem> M </mem>
  syntax KItem ::= #recon3(ValueResult, Value, Int, String)
  rule <k> #recon3(vErr(S), _, _, _) => #fail(S) ... </k>
  rule <k> #recon3(vOk(DV), MV, N, O) => #recon4(checkBits(DV, #frBits -Int N), DV, MV, N, O) ... </k>
  syntax KItem ::= #recon4(CheckResult, Value, Value, Int, String)
  rule <k> #recon4(cErr(S), _, _, _, _) => #fail(S) ... </k>
  rule <k> #recon4(cOk(), native(DV), native(MV), N, _) => #fail("Reconstituted element overflows field") ... </k>
    requires (DV <<Int N) +Int MV >Int #r -Int 1
  rule <k> #recon4(cOk(), native(DV), native(MV), N, O) => .K ... </k>
    <mem> M => M[O <- native(((DV <<Int N) +Int MV) modInt #r)] </mem>
    requires (DV <<Int N) +Int MV <=Int #r -Int 1

  rule <k> #exec(lessThan(A, B, N, O)) => #lt(resolve(A, M), B, N, O) ... </k> <mem> M </mem> <status> ok() </status>
  syntax KItem ::= #lt(ValueResult, Operand, Int, String)
  rule <k> #lt(vErr(S), _, _, _) => #fail(S) ... </k>
  rule <k> #lt(vOk(AV), B, N, O) => #lt2(checkBits(AV, N), AV, B, N, O) ... </k>
  syntax KItem ::= #lt2(CheckResult, Value, Operand, Int, String)
  rule <k> #lt2(cErr(S), _, _, _, _) => #fail(S) ... </k>
  rule <k> #lt2(cOk(), AV, B, N, O) => #lt3(resolve(B, M), AV, N, O) ... </k> <mem> M </mem>
  syntax KItem ::= #lt3(ValueResult, Value, Int, String)
  rule <k> #lt3(vErr(S), _, _, _) => #fail(S) ... </k>
  rule <k> #lt3(vOk(BV), AV, N, O) => #lt4(checkBits(BV, N), AV, BV, O) ... </k>
  syntax KItem ::= #lt4(CheckResult, Value, Value, String)
  rule <k> #lt4(cErr(S), _, _, _) => #fail(S) ... </k>
  rule <k> #lt4(cOk(), native(AV), native(BV), O) => .K ... </k> <mem> M => M[O <- native(bool2Word(AV <Int BV))] </mem>

  // hashes
  rule <k> #exec(transientHash(Xs, O)) => #put(O, #th2(resolveNatives(Xs, M))) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #th2(IntsResult) [function, total]
  rule #th2(isErr(S)) => vErr(S)
  rule #th2(isOk(L)) => vOk(native(poseidonHash(L)))

  rule <k> #exec(hashToCurve(Xs, O)) => #put(O, #htcV2(resolveNatives(Xs, M))) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #htcV2(IntsResult) [function, total]
  rule #htcV2(isErr(S)) => vErr(S)
  rule #htcV2(isOk(L)) => vOk(jubjubPoint(hashToCurve(L)))

  rule <k> #exec(persistentHash(Al, Xs, O)) => #put(O, #stdHash2(resolveNatives(Xs, M), Al, "sha256")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(keccak256(Al, Xs, O)) => #put(O, #stdHash2(resolveNatives(Xs, M), Al, "keccak256")) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #stdHash2(IntsResult, Alignment, String) [function, total]
  rule #stdHash2(isErr(S), _, _) => vErr(S)
  rule #stdHash2(isOk(L), Al, H) => #stdHash3(alignedBytes(Al, L), H)
  syntax ValueResult ::= #stdHash3(BytesResult, String) [function, total]
  rule #stdHash3(bErr(S), _) => vErr(S)
  rule #stdHash3(bOk(B), "sha256") => vOk(bytes32(sha256Bytes(B)))
  rule #stdHash3(bOk(B), "keccak256") => vOk(bytes32(keccak256Bytes(B)))
  rule #stdHash3(bOk(_), H) => vErr("unknown hash " +String H) [owise]

  // public_input / private_input: consume encodedLen(t) elements of the
  // transcript unless the guard is present and false (then the default value).
  rule <k> #exec(publicInput(G, T, O)) => #input(#guardActive(G, M), T, O, "public") ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(privateInput(G, T, O)) => #input(#guardActive(G, M), T, O, "private") ... </k> <mem> M </mem> <status> ok() </status>
  syntax BoolResult ::= #guardActive(Guard, Map) [function, total]
  rule #guardActive(noGuard(), _) => boolOk(true)
  rule #guardActive(guard(G), M) => resolveBool(G, M)
  syntax KItem ::= #input(BoolResult, IrType, String, String)
  rule <k> #input(boolErr(S), _, _, _) => #fail(S) ... </k>
  rule <k> #input(boolOk(false), T, O, _) => .K ... </k> <mem> M => M[O <- defaultValue(T)] </mem>
  rule <k> #input(boolOk(true), T, O, "public") => #put(O, #decodeSlice(#prePubOut(Pre), Idx, T, Strict)) ... </k>
       <pre> Pre </pre> <pubOutIdx> Idx => Idx +Int encodedLen(T) </pubOutIdx> <strictDecode> Strict </strictDecode>
    requires Idx +Int encodedLen(T) <=Int size(#prePubOut(Pre))
  // `preprocess` slices the transcript before advancing the cursor and the
  // slice panics (index out of range) when the transcript is too short
  rule <k> #input(boolOk(true), T, O, "public") => #panicNow("range end index out of range: public transcript outputs too short") ... </k>
       <pre> Pre </pre> <pubOutIdx> Idx </pubOutIdx> <genMode> false </genMode>
    requires Idx +Int encodedLen(T) >Int size(#prePubOut(Pre))
  rule <k> #input(boolOk(true), T, O, "public") => .K ... </k> <mem> M => M[O <- defaultValue(T)] </mem>
       <pre> Pre </pre> <pubOutIdx> Idx => Idx +Int encodedLen(T) </pubOutIdx> <genMode> true </genMode>
       <needs> N => N ListItem(needPubOut(T)) </needs>
    requires Idx +Int encodedLen(T) >Int size(#prePubOut(Pre))
  rule <k> #input(boolOk(true), T, O, "private") => #put(O, #decodeSlice(#prePrivate(Pre), Idx, T, Strict)) ... </k>
       <pre> Pre </pre> <privIdx> Idx => Idx +Int encodedLen(T) </privIdx> <strictDecode> Strict </strictDecode>
    requires Idx +Int encodedLen(T) <=Int size(#prePrivate(Pre))
  rule <k> #input(boolOk(true), T, O, "private") => #panicNow("range end index out of range: private transcript too short") ... </k>
       <pre> Pre </pre> <privIdx> Idx </privIdx> <genMode> false </genMode>
    requires Idx +Int encodedLen(T) >Int size(#prePrivate(Pre))
  rule <k> #input(boolOk(true), T, O, "private") => .K ... </k> <mem> M => M[O <- defaultValue(T)] </mem>
       <pre> Pre </pre> <privIdx> Idx => Idx +Int encodedLen(T) </privIdx> <genMode> true </genMode>
       <needs> N => N ListItem(needPriv(T)) </needs>
    requires Idx +Int encodedLen(T) >Int size(#prePrivate(Pre))
  // The Rust code indexes the transcript slice directly and panics with an
  // index-out-of-range when it is too short; the #input rules above report
  // the same event as panic(...) before this function is reached.
  syntax ValueResult ::= #decodeSlice(List, Int, IrType, Bool) [function, total]
  rule #decodeSlice(L, Idx, T, _) => vPanic("range end index out of range") requires Idx +Int encodedLen(T) >Int size(L)
  rule #decodeSlice(L, Idx, T, Strict) => #toValue(decodeStrict(range(L, Idx, size(L) -Int Idx -Int encodedLen(T)), T, Strict))
    requires Idx +Int encodedLen(T) <=Int size(L)
  syntax ValueResult ::= #toValue(DecodeResult) [function, total]
  rule #toValue(decOk(V)) => vOk(V)
  rule #toValue(decErr(S)) => vErr(S)
  rule #toValue(decPanic(S)) => vPanic(S)

  // impact
  rule <k> #exec(impact(G, Xs)) => #impact(resolveBool(G, M), Xs) ... </k> <mem> M </mem> <status> ok() </status>
  syntax KItem ::= #impact(BoolResult, Operands)
  rule <k> #impact(boolErr(S), _) => #fail(S) ... </k>
  rule <k> #impact(boolOk(false), Xs) => .K ... </k>
       <pi> Pi => Pi #zeros(lenOperands(Xs)) </pi>
       <skips> Sk => Sk ListItem(skipSome(lenOperands(Xs))) </skips>
  // active: each input is resolved, converted and pushed in turn (a failure
  // leaves the earlier pushes in place); the skip marker and the comparison
  // with the expected public transcript inputs follow the loop, as in Rust
  rule <k> #impact(boolOk(true), Xs) => #impactPush(Xs, 0) ... </k>
  syntax KItem ::= #impactPush(Operands, Int)   // operands left, pushed so far
  rule <k> #impactPush(.Operands, C) => #impactCheck(0, C) ... </k> <skips> Sk => Sk ListItem(skipNone()) </skips>
  rule <k> #impactPush((X, Xs), C) => #impactOne(resolve(X, M), Xs, C) ... </k> <mem> M </mem>
  syntax KItem ::= #impactOne(ValueResult, Operands, Int)
  rule <k> #impactOne(vErr(S), _, _) => #fail(S) ... </k>
  rule <k> #impactOne(vOk(native(V)), Xs, C) => #impactPush(Xs, C +Int 1) ... </k>
       <pi> Pi => Pi ListItem(V) </pi> <pubInIdx> Idx => Idx +Int 1 </pubInIdx>
  rule <k> #impactOne(vOk(V), _, _) => #fail("cannot convert " +String tn(V) +String " to Native") ... </k> requires notBool isNative(V)
  // the C values just pushed must equal public_transcript_inputs[pubInIdx - C + i]
  syntax KItem ::= #impactCheck(Int, Int)
  rule <k> #impactCheck(I, C) => .K ... </k> requires I >=Int C
  rule <k> #impactCheck(I, C) => #impactCheck(I +Int 1, C) ... </k> <pi> Pi </pi> <pubInIdx> Idx </pubInIdx> <pre> Pre </pre> <genMode> false </genMode>
    requires I <Int C andBool Idx -Int C +Int I <Int size(#prePubIn(Pre))
     andBool {#prePubIn(Pre)[Idx -Int C +Int I]}:>Int ==Int {Pi[size(Pi) -Int C +Int I]}:>Int
  rule <k> #impactCheck(I, C) => #fail("Public transcript input mismatch for input " +String Int2String(Idx -Int C +Int I)) ... </k>
       <pi> Pi </pi> <pubInIdx> Idx </pubInIdx> <pre> Pre </pre> <genMode> false </genMode>
    requires I <Int C andBool notBool (Idx -Int C +Int I <Int size(#prePubIn(Pre))
     andBool {#prePubIn(Pre)[Idx -Int C +Int I]}:>Int ==Int {Pi[size(Pi) -Int C +Int I]}:>Int)
  rule <k> #impactCheck(I, C) => #impactCheck(I +Int 1, C) ... </k> <pi> Pi </pi> <genMode> true </genMode>
       <needs> N => N ListItem(needPubIn({Pi[size(Pi) -Int C +Int I]}:>Int)) </needs>
    requires I <Int C
  syntax List ::= #zeros(Int) [function, total]
  rule #zeros(N) => .List requires N <=Int 0
  rule #zeros(N) => ListItem(0) #zeros(N -Int 1) requires N >Int 0

  // output
  rule <k> #exec(output(Vs)) => #fail("Output: signature declares " +String Int2String(lenIrTypes(Ts)) +String " return values but instruction has " +String Int2String(lenOperands(Vs))) ... </k>
       <outTypes> Ts </outTypes> <status> ok() </status>
    requires lenOperands(Vs) =/=Int lenIrTypes(Ts)
  rule <k> #exec(output(Vs)) => #output(Vs, Ts, 0) ... </k> <outTypes> Ts </outTypes> <status> ok() </status>
    requires lenOperands(Vs) ==Int lenIrTypes(Ts)
  syntax KItem ::= #output(Operands, IrTypes, Int)
  rule <k> #output(.Operands, _, _) => .K ... </k>
  rule <k> #output((V, Vs), (T, Ts), I) => #output1(resolve(V, M), T, Vs, Ts, I) ... </k> <mem> M </mem>
  rule <k> #output(_, .IrTypes, _) => .K ... </k> [owise]
  syntax KItem ::= #output1(ValueResult, IrType, Operands, IrTypes, Int)
  rule <k> #output1(vErr(S), _, _, _, _) => #fail(S) ... </k>
  rule <k> #output1(vOk(V), T, Vs, Ts, I) => #output(Vs, Ts, I +Int 1) ... </k> <outputs> Os => Os ListItem(V) </outputs>
    requires typeOf(V) ==K T
  rule <k> #output1(vOk(V), T, _, _, I) => #fail("Output position " +String Int2String(I) +String ": signature declares " +String typeName(T) +String " but operand has runtime type " +String tn(V)) ... </k>
    requires typeOf(V) =/=K T

  // --- After the run: evaluate every emitted constraint against the witness ---------------------
  syntax KItem ::= "#verdicts" | "#witnessSpace"
  rule <k> #verdicts => #witnessSpace ... </k> <constraints> Cs </constraints> <mem> M </mem> <pi> Pi </pi> <chips> Chips </chips>
       <outputs> Os </outputs> <pre> Pre </pre>
       <verdicts> _ => verdicts(Cs, M, Pi, Chips, Os, #preBinding(Pre)) </verdicts>
  // then decide membership of the final memory in the modelled witness space
  rule <k> #witnessSpace => #observable ... </k> <verdicts> Vs </verdicts> <mem> M </mem>
       <witnessSpace> _ => witnessSpace(Vs, M) </witnessSpace>
       <unconstrainedRegs> _ => unconstrainedRegs(Vs) </unconstrainedRegs>

  // --- The observable semantics --------------------------------------------------------
  // [[P]](pre) = observable(status, outputs, pi) = obs(status, encoded outputs,
  // public inputs), one function over the final configuration: the status of
  // the run, the values returned by `output` in order, each encoded as
  // `encode_offcircuit` encodes it (encodeValue, the encoding the
  // communications commitment hashes) and concatenated, and the public-input
  // vector. A compiler-correctness statement quotes this triple and nothing
  // else of the configuration; the memory, the verdicts and the witness space
  // are the model's evidence for the triple, not part of the observation.
  syntax Observable ::= obs(Status, List, List) [symbol(obs)] | noObs() [symbol(noObs)]
  syntax Observable ::= observable(Status, List, List) [function, total]
  rule observable(St, Os, Pi) => obs(St, #encodeAll(Os), Pi)
  syntax KItem ::= "#observable"
  rule <k> #observable => .K ... </k> <status> St </status> <outputs> Os </outputs> <pi> Pi </pi>
       <observable> _ => observable(St, Os, Pi) </observable>

  // --- End of program: transcript exhaustion and communications commitment ------------------
  syntax KItem ::= "#finish"
  rule <k> #finish => .K ... </k> <status> error(_) </status>
  rule <k> #finish => .K ... </k> <status> panic(_) </status>
  rule <k> #finish => .K ... </k> <status> ok() </status> <genMode> true </genMode> <doComm> false </doComm>
  rule <k> #finish => .K ... </k> <status> ok() </status> <genMode> true </genMode> <doComm> true </doComm> <pre> Pre </pre> <outputs> Os </outputs>
       <needs> N => N ListItem(needComm(transientCommit(#preInputs(Pre) #encodeAll(Os), #commRand(#preComm(Pre))))) </needs>
  syntax Int ::= #commRand(CommOpt) [function, total]
  rule #commRand(comm(_, Rand)) => Rand
  rule #commRand(noComm()) => 0
  rule <k> #finish => #fail("Transcripts not fully consumed") ... </k> <status> ok() </status> <pre> Pre </pre> <genMode> false </genMode>
       <pubInIdx> A </pubInIdx> <pubOutIdx> B </pubOutIdx> <privIdx> C </privIdx>
    requires size(#prePubIn(Pre)) =/=Int A orBool size(#prePubOut(Pre)) =/=Int B orBool size(#prePrivate(Pre)) =/=Int C
  rule <k> #finish => .K ... </k> <status> ok() </status> <pre> Pre </pre> <doComm> false </doComm> <genMode> false </genMode>
       <pubInIdx> A </pubInIdx> <pubOutIdx> B </pubOutIdx> <privIdx> C </privIdx>
    requires size(#prePubIn(Pre)) ==Int A andBool size(#prePubOut(Pre)) ==Int B andBool size(#prePrivate(Pre)) ==Int C
  rule <k> #finish => #commCheck(#preComm(Pre), Os) ... </k> <status> ok() </status> <pre> Pre </pre> <doComm> true </doComm> <outputs> Os </outputs> <genMode> false </genMode>
       <pubInIdx> A </pubInIdx> <pubOutIdx> B </pubOutIdx> <privIdx> C </privIdx>
    requires size(#prePubIn(Pre)) ==Int A andBool size(#prePubOut(Pre)) ==Int B andBool size(#prePrivate(Pre)) ==Int C
  syntax KItem ::= #commCheck(CommOpt, List)
  // unreachable (an absent commitment already failed at #seedPi); kept because
  // `preprocess` has the same dead branch (ir_vm.rs "Expected communications randomness")
  rule <k> #commCheck(noComm(), _) => #fail("Expected communications randomness") ... </k>
  rule <k> #commCheck(comm(Cm, Rand), Os) => .K ... </k> <pre> Pre </pre>
    requires Cm ==Int transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)
  rule <k> #commCheck(comm(Cm, Rand), Os) => #fail("Communications commitment mismatch") ... </k> <pre> Pre </pre>
    requires Cm =/=Int transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)
  // #encodeAll is defined in ZKIR-CONSTRAINTS
endmodule
