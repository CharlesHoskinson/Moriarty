// ZKIR v3 virtual machine: the witness computation (`IrSource::preprocess`,
// zkir-v3/src/ir_vm.rs at midnight-ledger 92e8bdd3) and, in the same rewrite,
// the constraint set that `Relation::circuit` would synthesise for the same
// instruction (plan decisions CLM-0704 .. CLM-0706, CLM-0720).
//
// State (one cell per component of the seven-tuple on the VM wiki page):
//   <mem>        registers, String |-> Value, write-once
//   <pi>         public-input vector (binding input, optional commitment, impacts)
//   <skips>      pi_skips: skipNone() for an active impact, skipSome(n) for an inactive one
//   <pubInIdx>, <pubOutIdx>, <privIdx>   transcript cursors
//   <outputs>    values returned by `output`, in order
//   <constraints> instruction-level relations emitted by every instruction,
//                whether or not the witness computation is still alive
//   <status>     ok() or error(msg): the first witness failure freezes the
//                witness cells; the remaining instructions still emit constraints
//
// A run is job(program, preimage). The preimage mirrors `ProofPreimage`.
requires "zkir-syntax.k"
requires "zkir-ops.k"

module ZKIR-VM-SYNTAX
  imports ZKIR-SYNTAX
  imports LIST
  imports INT

  syntax Preimage ::= preimage(List, Int, CommOpt, List, List, List) [symbol(preimage)]
                      // inputs, binding_input, communications_commitment, private_transcript,
                      // public_transcript_inputs, public_transcript_outputs
  syntax CommOpt  ::= noComm()        [symbol(noComm)]
                    | comm(Int, Int)  [symbol(comm)]    // (commitment, opening randomness)
  syntax Job ::= job(Program, Preimage)    [symbol(job)]
               | genJob(Program, Preimage) [symbol(genJob)]   // generation mode, see <genMode>
endmodule

module ZKIR-VM
  imports ZKIR-VM-SYNTAX
  imports ZKIR-WF
  imports ZKIR-OPS
  imports MAP
  imports SET
  imports K-EQUAL

  syntax Status ::= ok() [symbol(ok)] | error(String) [symbol(error)]
  syntax Skip   ::= skipNone() [symbol(skipNone)] | skipSome(Int) [symbol(skipSome)]

  // Constraints: the in-circuit relation of one instruction over register
  // names and immediates. `piGate(i, guard, x)` states pi[i] = guard ? x : 0.
  syntax Constraint ::= gate(Instr)                       [symbol(gate)]
                      | piGate(Int, Operand, Operand)     [symbol(piGate)]
                      | piFixed(Int, Operand)             [symbol(piFixed)]   // pi[i] = x (binding input, commitment)

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
      <deadPis> 0 </deadPis>
      <genMode> false </genMode>
      <needs> .List </needs>
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
  rule <k> job(program(_, Ins, Outs, DoComm, Is), Pre) => #loadInputs(decodeInputs(#preInputs(Pre), Ins, 0)) ~> #seedPi ~> Is </k>
       <pre> _ => Pre </pre>
       <outTypes> _ => Outs </outTypes>
       <doComm> _ => DoComm </doComm>

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

  syntax KItem ::= #loadInputs(DecodeMany) | "#seedPi" | #bind(List)
  rule <k> #loadInputs(decManyErr(S)) => .K ... </k> <status> ok() => error(S) </status>
  rule <k> #loadInputs(decManyOk(L)) => #bind(L) ... </k>
  rule <k> #bind(.List) => .K ... </k>
  rule <k> #bind(ListItem(binding(N, V)) L) => #bind(L) ... </k> <mem> M => M[N <- V] </mem>

  // pis = [binding_input] ++ [commitment if do_communications_commitment]
  rule <k> #seedPi => .K ... </k> <pre> Pre </pre> <doComm> false </doComm>
       <pi> _ => ListItem(#preBinding(Pre)) </pi>
       <constraints> Cs => Cs ListItem(piFixed(0, var("$binding"))) </constraints>
  rule <k> #seedPi => .K ... </k> <pre> Pre </pre> <doComm> true </doComm>
       <pi> _ => ListItem(#preBinding(Pre)) ListItem(#commValue(#preComm(Pre))) </pi>
       <constraints> Cs => Cs ListItem(piFixed(0, var("$binding"))) ListItem(piFixed(1, var("$commitment"))) </constraints>
       <status> St => #requireComm(#preComm(Pre), St) </status>
  syntax Int ::= #commValue(CommOpt) [function, total]
  rule #commValue(comm(C, _)) => C
  rule #commValue(noComm()) => 0
  syntax Status ::= #requireComm(CommOpt, Status) [function, total]
  rule #requireComm(noComm(), ok()) => error("Expected communications commitment")
  rule #requireComm(_, St) => St [owise]

  // --- Instruction sequencing ------------------------------------------------------------
  rule <k> (I:Instr ; Is:Instrs) => I ~> Is ... </k>
  rule <k> .Instrs => #finish ... </k>

  // Every instruction emits its constraint, then runs its witness half (if alive).
  // The witness half is written as #exec(I); when the status is an error only
  // the constraint half fires.
  syntax KItem ::= #exec(Instr)
  rule <k> I:Instr => #exec(I) ... </k> <constraints> Cs => Cs #constraintsOf(I) </constraints>
    requires notBool isImpact(I)
  rule <k> #exec(_) => .K ... </k> <status> error(_) </status>

  syntax Bool ::= isImpact(Instr) [function, total]
  rule isImpact(impact(_, _)) => true
  rule isImpact(_) => false [owise]
  syntax List ::= #constraintsOf(Instr) [function, total]
  rule #constraintsOf(I) => ListItem(gate(I))

  // impact: constraints reference absolute public-input positions, so they
  // depend on how many public inputs precede them; with a dead witness the
  // count continues from the constraint side (piIdx cell would be needed for
  // full independence; here we use the length of <pi>, which is frozen on
  // error, plus the impacts seen since). The dead-witness case is tracked by
  // <deadPis>.
  rule <k> impact(G, Xs) => #exec(impact(G, Xs)) ... </k>
       <pi> Pi </pi> <deadPis> D </deadPis>
       <constraints> Cs => Cs #piGates(size(Pi) +Int D, G, Xs) </constraints>
  syntax List ::= #piGates(Int, Operand, Operands) [function, total]
  rule #piGates(_, _, .Operands) => .List
  rule #piGates(N, G, (X, Xs)) => ListItem(piGate(N, G, X)) #piGates(N +Int 1, G, Xs)

  rule <k> #exec(impact(_, Xs)) => .K ... </k> <status> error(_) </status> <deadPis> D => D +Int lenOperands(Xs) </deadPis>
    [priority(30)]

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
  rule <k> #bindEncoded((_, _), .List) => .K ... </k>

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

  // div_mod_power_of_two: bits <= 248 checked statically; val must be Native
  rule <k> #exec(divModPowerOfTwo(V, N, (Q, (R, .Ids)))) => #divMod(resolve(V, M), N, Q, R) ... </k> <mem> M </mem> <status> ok() </status>
  syntax KItem ::= #divMod(ValueResult, Int, String, String)
  rule <k> #divMod(vErr(S), _, _, _) => #fail(S) ... </k>
  rule <k> #divMod(vOk(native(X)), N, Q, R) => .K ... </k> <mem> M => M[Q <- native(highBits(X, N))][R <- native(lowBits(X, N))] </mem>
  rule <k> #divMod(vOk(V), _, _, _) => #fail("cannot convert " +String tn(V) +String " to Native") ... </k> requires notBool isNative(V)

  // reconstitute_field: modulus < 2^bits, divisor < 2^(255-bits), and the
  // combination must not exceed r - 1 (compared bit by bit against r - 1).
  rule <k> #exec(reconstituteField(D, Mo, N, O)) => #recon(resolve(D, M), resolve(Mo, M), N, O) ... </k> <mem> M </mem> <status> ok() </status>
  syntax KItem ::= #recon(ValueResult, ValueResult, Int, String)
  rule <k> #recon(_, vErr(S), _, _) => #fail(S) ... </k>
  rule <k> #recon(vErr(S), vOk(_), _, _) => #fail(S) ... </k>
  rule <k> #recon(vOk(DV), vOk(MV), N, O) => #recon2(checkBits(MV, N), checkBits(DV, #frBits -Int N), DV, MV, N, O) ... </k>
  syntax KItem ::= #recon2(CheckResult, CheckResult, Value, Value, Int, String)
  rule <k> #recon2(cErr(S), _, _, _, _, _) => #fail(S) ... </k>
  rule <k> #recon2(cOk(), cErr(S), _, _, _, _) => #fail(S) ... </k>
  rule <k> #recon2(cOk(), cOk(), native(DV), native(MV), N, O) => #fail("Reconstituted element overflows field") ... </k>
    requires (DV <<Int N) +Int MV >Int #r -Int 1
  rule <k> #recon2(cOk(), cOk(), native(DV), native(MV), N, O) => .K ... </k>
    <mem> M => M[O <- native(((DV <<Int N) +Int MV) modInt #r)] </mem>
    requires (DV <<Int N) +Int MV <=Int #r -Int 1

  rule <k> #exec(lessThan(A, B, N, O)) => #lt(resolve(A, M), resolve(B, M), N, O) ... </k> <mem> M </mem> <status> ok() </status>
  syntax KItem ::= #lt(ValueResult, ValueResult, Int, String)
  rule <k> #lt(vErr(S), _, _, _) => #fail(S) ... </k>
  rule <k> #lt(vOk(_), vErr(S), _, _) => #fail(S) ... </k>
  rule <k> #lt(vOk(AV), vOk(BV), N, O) => #lt2(checkBits(AV, N), checkBits(BV, N), AV, BV, O) ... </k>
  syntax KItem ::= #lt2(CheckResult, CheckResult, Value, Value, String)
  rule <k> #lt2(cErr(S), _, _, _, _) => #fail(S) ... </k>
  rule <k> #lt2(cOk(), cErr(S), _, _, _) => #fail(S) ... </k>
  rule <k> #lt2(cOk(), cOk(), native(AV), native(BV), O) => .K ... </k> <mem> M => M[O <- native(bool2Word(AV <Int BV))] </mem>

  // hashes
  rule <k> #exec(transientHash(Xs, O)) => #put(O, #th(resolveAll(Xs, M))) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #th(ValuesResult) [function, total]
  rule #th(vsErr(S)) => vErr(S)
  rule #th(vsOk(Vs)) => #th2(asNatives(Vs))
  syntax ValueResult ::= #th2(IntsResult) [function, total]
  rule #th2(isErr(S)) => vErr(S)
  rule #th2(isOk(L)) => vOk(native(poseidonHash(L)))

  rule <k> #exec(hashToCurve(Xs, O)) => #put(O, #htcV(resolveAll(Xs, M))) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #htcV(ValuesResult) [function, total]
  rule #htcV(vsErr(S)) => vErr(S)
  rule #htcV(vsOk(Vs)) => #htcV2(asNatives(Vs))
  syntax ValueResult ::= #htcV2(IntsResult) [function, total]
  rule #htcV2(isErr(S)) => vErr(S)
  rule #htcV2(isOk(L)) => vOk(jubjubPoint(hashToCurve(L)))

  rule <k> #exec(persistentHash(Al, Xs, O)) => #put(O, #stdHash(resolveAll(Xs, M), Al, "sha256")) ... </k> <mem> M </mem> <status> ok() </status>
  rule <k> #exec(keccak256(Al, Xs, O)) => #put(O, #stdHash(resolveAll(Xs, M), Al, "keccak256")) ... </k> <mem> M </mem> <status> ok() </status>
  syntax ValueResult ::= #stdHash(ValuesResult, Alignment, String) [function, total]
  rule #stdHash(vsErr(S), _, _) => vErr(S)
  rule #stdHash(vsOk(Vs), Al, H) => #stdHash2(asNatives(Vs), Al, H)
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
  rule <k> #input(boolOk(true), T, O, "public") => #put(O, #decodeSlice(#prePubOut(Pre), Idx, T)) ... </k>
       <pre> Pre </pre> <pubOutIdx> Idx => Idx +Int encodedLen(T) </pubOutIdx>
    requires Idx +Int encodedLen(T) <=Int size(#prePubOut(Pre))
  rule <k> #input(boolOk(true), T, O, "public") => #put(O, #decodeSlice(#prePubOut(Pre), Idx, T)) ... </k>
       <pre> Pre </pre> <pubOutIdx> Idx => Idx +Int encodedLen(T) </pubOutIdx> <genMode> false </genMode>
    requires Idx +Int encodedLen(T) >Int size(#prePubOut(Pre))
  rule <k> #input(boolOk(true), T, O, "public") => .K ... </k> <mem> M => M[O <- defaultValue(T)] </mem>
       <pre> Pre </pre> <pubOutIdx> Idx => Idx +Int encodedLen(T) </pubOutIdx> <genMode> true </genMode>
       <needs> N => N ListItem(needPubOut(T)) </needs>
    requires Idx +Int encodedLen(T) >Int size(#prePubOut(Pre))
  rule <k> #input(boolOk(true), T, O, "private") => #put(O, #decodeSlice(#prePrivate(Pre), Idx, T)) ... </k>
       <pre> Pre </pre> <privIdx> Idx => Idx +Int encodedLen(T) </privIdx>
    requires Idx +Int encodedLen(T) <=Int size(#prePrivate(Pre))
  rule <k> #input(boolOk(true), T, O, "private") => #put(O, #decodeSlice(#prePrivate(Pre), Idx, T)) ... </k>
       <pre> Pre </pre> <privIdx> Idx => Idx +Int encodedLen(T) </privIdx> <genMode> false </genMode>
    requires Idx +Int encodedLen(T) >Int size(#prePrivate(Pre))
  rule <k> #input(boolOk(true), T, O, "private") => .K ... </k> <mem> M => M[O <- defaultValue(T)] </mem>
       <pre> Pre </pre> <privIdx> Idx => Idx +Int encodedLen(T) </privIdx> <genMode> true </genMode>
       <needs> N => N ListItem(needPriv(T)) </needs>
    requires Idx +Int encodedLen(T) >Int size(#prePrivate(Pre))
  // The Rust code indexes the slice directly and panics past the end; we report it.
  syntax ValueResult ::= #decodeSlice(List, Int, IrType) [function, total]
  rule #decodeSlice(L, Idx, T) => vErr("transcript index out of range") requires Idx +Int encodedLen(T) >Int size(L)
  rule #decodeSlice(L, Idx, T) => #toValue(decodeValue(range(L, Idx, size(L) -Int Idx -Int encodedLen(T)), T))
    requires Idx +Int encodedLen(T) <=Int size(L)
  syntax ValueResult ::= #toValue(DecodeResult) [function, total]
  rule #toValue(decOk(V)) => vOk(V)
  rule #toValue(decErr(S)) => vErr(S)

  // impact
  rule <k> #exec(impact(G, Xs)) => #impact(resolveBool(G, M), Xs) ... </k> <mem> M </mem> <status> ok() </status>
  syntax KItem ::= #impact(BoolResult, Operands)
  rule <k> #impact(boolErr(S), _) => #fail(S) ... </k>
  rule <k> #impact(boolOk(false), Xs) => .K ... </k>
       <pi> Pi => Pi #zeros(lenOperands(Xs)) </pi>
       <skips> Sk => Sk ListItem(skipSome(lenOperands(Xs))) </skips>
  rule <k> #impact(boolOk(true), Xs) => #impactPush(resolveAll(Xs, M)) ... </k> <mem> M </mem>
  syntax KItem ::= #impactPush(ValuesResult)
  rule <k> #impactPush(vsErr(S)) => #fail(S) ... </k>
  rule <k> #impactPush(vsOk(Vs)) => #impactNatives(asNatives(Vs)) ... </k>
  syntax KItem ::= #impactNatives(IntsResult)
  rule <k> #impactNatives(isErr(S)) => #fail(S) ... </k>
  rule <k> #impactNatives(isOk(L)) => #impactCheck(L, Idx) ... </k>
       <pi> Pi => Pi L </pi>
       <skips> Sk => Sk ListItem(skipNone()) </skips>
       <pubInIdx> Idx => Idx +Int size(L) </pubInIdx>
  // every pushed value must equal the expected public transcript input at its index
  syntax KItem ::= #impactCheck(List, Int)
  rule <k> #impactCheck(.List, _) => .K ... </k>
  rule <k> #impactCheck(ListItem(X) L, Idx) => #impactCheck(L, Idx +Int 1) ... </k> <pre> Pre </pre> <genMode> false </genMode>
    requires Idx <Int size(#prePubIn(Pre)) andBool {#prePubIn(Pre)[Idx]}:>Int ==Int X
  rule <k> #impactCheck(ListItem(X) _, Idx) => #fail("Public transcript input mismatch for input " +String Int2String(Idx)) ... </k> <pre> Pre </pre> <genMode> false </genMode>
    requires notBool (Idx <Int size(#prePubIn(Pre)) andBool {#prePubIn(Pre)[Idx]}:>Int ==Int X)
  rule <k> #impactCheck(ListItem(X) L, Idx) => #impactCheck(L, Idx +Int 1) ... </k> <genMode> true </genMode>
       <needs> N => N ListItem(needPubIn(X)) </needs>
  syntax List ::= #zeros(Int) [function, total]
  rule #zeros(N) => .List requires N <=Int 0
  rule #zeros(N) => ListItem(0) #zeros(N -Int 1) requires N >Int 0

  // output
  rule <k> #exec(output(Vs)) => #output(resolveAll(Vs, M), Ts, 0) ... </k> <mem> M </mem> <outTypes> Ts </outTypes> <status> ok() </status>
  syntax KItem ::= #output(ValuesResult, IrTypes, Int)
  rule <k> #output(vsErr(S), _, _) => #fail(S) ... </k>
  rule <k> #output(vsOk(L), Ts, _) => #fail("Output: signature declares " +String Int2String(lenIrTypes(Ts)) +String " return values but instruction has " +String Int2String(size(L))) ... </k>
    requires size(L) =/=Int lenIrTypes(Ts)
  rule <k> #output(vsOk(.List), .IrTypes, _) => .K ... </k>
  rule <k> #output(vsOk(ListItem(V) L), (T, Ts), I) => #output(vsOk(L), Ts, I +Int 1) ... </k> <outputs> Os => Os ListItem(V) </outputs>
    requires size(L) ==Int lenIrTypes(Ts) andBool typeOf(V) ==K T
  rule <k> #output(vsOk(ListItem(V) L), (T, Ts), I) => #fail("Output position " +String Int2String(I) +String ": signature declares " +String typeName(T) +String " but operand has runtime type " +String tn(V)) ... </k>
    requires size(L) ==Int lenIrTypes(Ts) andBool typeOf(V) =/=K T

  // --- End of program: transcript exhaustion and communications commitment ------------------
  syntax KItem ::= "#finish"
  rule <k> #finish => .K ... </k> <status> error(_) </status>
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
  rule <k> #commCheck(noComm(), _) => #fail("Expected communications randomness") ... </k>
  rule <k> #commCheck(comm(Cm, Rand), Os) => .K ... </k> <pre> Pre </pre>
    requires Cm ==Int transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)
  rule <k> #commCheck(comm(Cm, Rand), Os) => #fail("Communications commitment mismatch") ... </k> <pre> Pre </pre>
    requires Cm =/=Int transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)
  syntax List ::= #encodeAll(List) [function, total]
  rule #encodeAll(.List) => .List
  rule #encodeAll(ListItem(V:Value) L) => encodeValue(V) #encodeAll(L)
  rule #encodeAll(ListItem(_) L) => #encodeAll(L) [owise]
endmodule
