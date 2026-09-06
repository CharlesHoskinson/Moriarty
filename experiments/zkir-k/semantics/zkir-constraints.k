// Constraint checker (oracle 2, plan CLM-0716 item 2): does the witness in
// <mem> satisfy the in-circuit relation of every emitted constraint?
//
// A constraint is instruction-level: gate(I) is the relation that
// `Relation::circuit` in ir_vm.rs (midnight-ledger 92e8bdd3) and the
// `*_incircuit` functions of ir_instructions/*.rs enforce for instruction I,
// read off the chip calls they make; piGate(i, g, x) is pi[i] = g ? x : 0 with
// g boolean; bindGate(0) pins the binding input and commGate(1, inputs, rand) the commitment. The
// relation is evaluated with concrete values, so this establishes only that the
// honest witness satisfies the constraints (completeness), never that no other
// witness does (CLM-0721).
//
// Outcomes per constraint:
//   holds()            the relation is satisfied
//   violated(msg)      the relation is not satisfied (an unsatisfiable circuit)
//   synthErr(msg)      the circuit cannot be built: an in-circuit dispatch has
//                      no arm for these types (Error::Synthesis) or a chip the
//                      gate needs is not initialised (`used_chips`, finding 13)
//   unknown(msg)       a register the gate reads is absent (witness stopped early)
//
// Chip gating follows `IrSource::used_chips` (usedChips and chipOfType in
// ZKIR-STATIC, zkir-static.k, shared with the target contract): a foreign chip
// is initialised only when its types occur among the program inputs or
// public/private inputs, the Jubjub chip additionally by hash_to_curve,
// Poseidon by transient_hash, hash_to_curve or the communications commitment,
// SHA-256 by persistent_hash, Keccak by keccak256. from_bytes32 does not
// initialise anything (finding 13).
requires "zkir-ops.k"
requires "zkir-static.k"

module ZKIR-CONSTRAINTS
  imports ZKIR-OPS
  imports ZKIR-WF
  imports ZKIR-STATIC
  imports MAP
  imports SET

  syntax Constraint ::= gate(Instr)                        [symbol(gate)]
                      | piGate(Int, Operand, Operand)      [symbol(piGate)]      // pi[i] = g ? x : 0
                      | guardGate(Operand)                 [symbol(guardGate)]   // impact guard is boolean (also for empty impacts)
                      | bindGate(Int)                      [symbol(bindGate)]    // pi[i] = binding input
                      | commGate(Int, TypedIds, Int)       [symbol(commGate)]    // pi[i] = poseidon(rand ++ enc(inputs) ++ enc(outputs))
                      | outputGate(Operands, IrTypes)      [symbol(outputGate)]  // arity and per-position types of the signature
  syntax Outcome ::= holds() [symbol(holds)] | violated(String) [symbol(violated)]
                   | synthErr(String) [symbol(synthErr)] | unknown(String) [symbol(unknown)]
                   | unsupported(String) [symbol(unsupported)]   // no in-circuit relation modelled for this gate
  syntax Verdict ::= verdict(Constraint, Outcome) [symbol(verdict)]

  // --- Evaluation of all constraints --------------------------------------------------
  // verdicts(constraints, mem, pi, chips, outputs, binding input)
  syntax List ::= verdicts(List, Map, List, Set, List, Int) [function, total]
  rule verdicts(.List, _, _, _, _, _) => .List
  rule verdicts(ListItem(C:Constraint) Cs, M, Pi, Chips, Os, B) => ListItem(verdict(C, eval(C, M, Pi, Chips, Os, B))) verdicts(Cs, M, Pi, Chips, Os, B)
  rule verdicts(ListItem(_) Cs, M, Pi, Chips, Os, B) => verdicts(Cs, M, Pi, Chips, Os, B) [owise]

  // gates that need the run's outputs or binding input
  syntax Outcome ::= eval(Constraint, Map, List, Set, List, Int) [function, total]
  rule eval(bindGate(I), _, Pi, _, _, B) => holds() requires I <Int size(Pi) andBool {Pi[I]}:>Int ==Int B
  rule eval(bindGate(I), _, Pi, _, _, _) => unknown("public input vector incomplete") requires I >=Int size(Pi)
  rule eval(bindGate(I), _, Pi, _, _, B) => violated("binding input differs") requires I <Int size(Pi) andBool {Pi[I]}:>Int =/=Int B
  // the circuit re-encodes the assigned input values and the outputs in order
  // (ir_vm.rs `circuit`, communications commitment block)
  rule eval(commGate(I, _, _), _, Pi, _, _, _) => unknown("public input vector incomplete") requires I >=Int size(Pi)
  rule eval(commGate(I, Ins, Rand), M, Pi, Chips, Os, _) => #commOutcome(#encInputs(Ins, M), Rand, Os, {Pi[I]}:>Int, Chips) requires I <Int size(Pi)
  syntax Outcome ::= #commOutcome(ListResultC, Int, List, Int, Set) [function, total]
  rule #commOutcome(lcErr(S), _, _, _, _) => unknown(S)
  rule #commOutcome(lcOk(_), _, _, _, Chips) => synthErr("chip not initialised: poseidon") requires notBool ("poseidon" in Chips)
  rule #commOutcome(lcOk(Enc), Rand, Os, C, Chips) => holds() requires "poseidon" in Chips andBool poseidonHash(ListItem(Rand) Enc #encodeAll(Os)) ==Int C
  rule #commOutcome(lcOk(Enc), Rand, Os, C, Chips) => violated("communications commitment differs from poseidon(rand ++ encode(inputs) ++ encode(outputs))")
    requires "poseidon" in Chips andBool poseidonHash(ListItem(Rand) Enc #encodeAll(Os)) =/=Int C
  syntax ListResultC ::= lcOk(List) | lcErr(String)
  syntax ListResultC ::= #encInputs(TypedIds, Map) [function, total]
  rule #encInputs(.TypedIds, _) => lcOk(.List)
  rule #encInputs((typedId(N, _), Rest), M) => #encInputs2(rdId(N, M), #encInputs(Rest, M))
  syntax ListResultC ::= #encInputs2(ValueResult, ListResultC) [function, total]
  rule #encInputs2(vOk(V), lcOk(L)) => lcOk(encodeValue(V) L)
  rule #encInputs2(vOk(_), lcErr(S)) => lcErr(S)
  rule #encInputs2(vErr(S), _) => lcErr(S)
  rule #encInputs2(vPanic(S), _) => lcErr(S)
  syntax List ::= #encodeAll(List) [function, total]
  rule #encodeAll(.List) => .List
  rule #encodeAll(ListItem(V:Value) L) => encodeValue(V) #encodeAll(L)
  rule #encodeAll(ListItem(_) L) => #encodeAll(L) [owise]
  // output: arity, then the runtime type of every operand against the signature
  rule eval(outputGate(Vs, Ts), _, _, _, _, _) => synthErr("Output: signature declares " +String Int2String(lenIrTypes(Ts)) +String " return values but instruction has " +String Int2String(lenOperands(Vs)))
    requires lenOperands(Vs) =/=Int lenIrTypes(Ts)
  rule eval(outputGate(Vs, Ts), M, _, _, _, _) => #outputTypes(Vs, Ts, M, 0) requires lenOperands(Vs) ==Int lenIrTypes(Ts)
  syntax Outcome ::= #outputTypes(Operands, IrTypes, Map, Int) [function, total]
  rule #outputTypes(.Operands, _, _, _) => holds()
  rule #outputTypes((V, Vs), (T, Ts), M, I) => #and(#typedOrSynth(rd(V, M), T, I), #outputTypes(Vs, Ts, M, I +Int 1))
  rule #outputTypes(_, _, _, _) => holds() [owise]
  syntax Outcome ::= #typedOrSynth(ValueResult, IrType, Int) [function, total]
  rule #typedOrSynth(vErr(S), _, _) => unknown(S)
  rule #typedOrSynth(vPanic(S), _, _) => unknown(S)
  rule #typedOrSynth(vOk(V), T, _) => holds() requires typeOf(V) ==K T
  rule #typedOrSynth(vOk(V), T, I) => synthErr("Output position " +String Int2String(I) +String ": signature declares " +String typeName(T) +String " but operand has runtime type " +String tn(V)) requires typeOf(V) =/=K T
  rule eval(C, M, Pi, Chips, _, _) => eval(C, M, Pi, Chips) [owise]

  syntax List ::= violations(List) [function, total]   // verdicts that are not holds()
  rule violations(.List) => .List
  rule violations(ListItem(verdict(_, holds())) Vs) => violations(Vs)
  rule violations(ListItem(V) Vs) => ListItem(V) violations(Vs) [owise]

  // --- Reading the witness ------------------------------------------------------------
  syntax ValueResult ::= rd(Operand, Map) [function, total]
  rule rd(imm(I), _) => vOk(native(I))
  rule rd(var(X), M) => vOk({M[X]}:>Value) requires X in_keys(M)
  rule rd(var(X), M) => vErr("register " +String X +String " is not in the witness") requires notBool X in_keys(M)
  syntax ValueResult ::= rdId(String, Map) [function, total]
  rule rdId(X, M) => rd(var(X), M)

  syntax Outcome ::= #need(ValueResult) [function, total]   // an absent register makes the outcome unknown
  rule #need(vErr(S)) => unknown(S)
  rule #need(vPanic(S)) => unknown(S)
  rule #need(vOk(_)) => holds()

  // value-level unsatisfiability of a circuit that exists (inv of zero, byte
  // bounds), as opposed to a missing dispatch arm
  syntax Outcome ::= #unsat(Outcome, ValueResult) [function, total]
  rule #unsat(O, _) => O

  syntax Outcome ::= #and(Outcome, Outcome) [function, total]   // first non-holds outcome wins
  rule #and(holds(), O) => O
  rule #and(O, _) => O requires O =/=K holds()

  syntax Outcome ::= #chipFor(IrType, Set) [function, total]
  rule #chipFor(T, Chips) => holds() requires chipOfType(T) <=Set Chips
  rule #chipFor(T, Chips) => synthErr("chip not initialised for " +String typeName(T)) requires notBool (chipOfType(T) <=Set Chips)
  syntax Outcome ::= #chipsForValues(List, Set) [function, total]
  rule #chipsForValues(.List, _) => holds()
  rule #chipsForValues(ListItem(V:Value) L, Chips) => #and(#chipFor(typeOf(V), Chips), #chipsForValues(L, Chips))
  rule #chipsForValues(ListItem(_) L, Chips) => #chipsForValues(L, Chips) [owise]

  syntax Outcome ::= #native(ValueResult, String) [function, total]
  rule #native(vErr(S), _) => unknown(S)
  rule #native(vPanic(S), _) => unknown(S)
  rule #native(vOk(native(_)), _) => holds()
  rule #native(vOk(V), What) => synthErr(What +String ": cannot convert " +String tn(V) +String " to Native") [owise]
  syntax Outcome ::= #boolean(ValueResult, String) [function, total]
  rule #boolean(vErr(S), _) => unknown(S)
  rule #boolean(vPanic(S), _) => unknown(S)
  rule #boolean(vOk(native(0)), _) => holds()
  rule #boolean(vOk(native(1)), _) => holds()
  rule #boolean(vOk(native(V)), What) => violated(What +String " is not boolean: " +String Int2String(V)) requires V =/=Int 0 andBool V =/=Int 1
  rule #boolean(vOk(V), What) => synthErr(What +String ": cannot convert " +String tn(V) +String " to Native") [owise]
  syntax Outcome ::= #eqValues(ValueResult, ValueResult, String) [function, total]
  rule #eqValues(vErr(S), _, _) => unknown(S)
  rule #eqValues(vPanic(S), _, _) => unknown(S)
  rule #eqValues(vOk(_), vErr(S), _) => unknown(S)
  rule #eqValues(vOk(_), vPanic(S), _) => unknown(S)
  rule #eqValues(vOk(A), vOk(B), _) => holds() requires A ==K B
  rule #eqValues(vOk(A), vOk(B), What) => violated(What) requires A =/=K B
  // expected value computed by an off-circuit function vs the register
  syntax Outcome ::= #matches(ValueResult, ValueResult, String) [function, total]
  rule #matches(vErr(S), _, What) => synthErr(What +String ": " +String S)   // the in-circuit op is undefined on these types
  rule #matches(vPanic(S), _, What) => synthErr(What +String ": " +String S)
  rule #matches(vOk(_), vErr(S), _) => unknown(S)
  rule #matches(vOk(_), vPanic(S), _) => unknown(S)
  rule #matches(vOk(A), vOk(B), _) => holds() requires A ==K B
  rule #matches(vOk(A), vOk(B), What) => violated(What) requires A =/=K B
  syntax Int ::= #int(ValueResult) [function, total]
  rule #int(vOk(native(X))) => X
  rule #int(_) => -1 [owise]

  // --- Gates --------------------------------------------------------------------------
  syntax Outcome ::= eval(Constraint, Map, List, Set) [function, total]

  rule eval(guardGate(G), M, _, _) => #boolean(rd(G, M), "impact guard")
  rule eval(piGate(I, G, X), M, Pi, _) => #and(#boolean(rd(G, M), "impact guard"), #and(#native(rd(X, M), "impact"), #piEq(I, rd(G, M), rd(X, M), Pi)))
  syntax Outcome ::= #piEq(Int, ValueResult, ValueResult, List) [function, total]
  // the witness stopped before this public input was produced: nothing to compare
  rule #piEq(I, _, _, Pi) => unknown("public input vector incomplete: witness stopped before index " +String Int2String(I)) requires I >=Int size(Pi)
  rule #piEq(I, G, X, Pi) => holds() requires I <Int size(Pi) andBool {Pi[I]}:>Int ==Int #guarded(#int(G), #int(X))
  rule #piEq(I, G, X, Pi) => violated("public input " +String Int2String(I) +String " differs from the guarded value") requires I <Int size(Pi) andBool {Pi[I]}:>Int =/=Int #guarded(#int(G), #int(X))
  syntax Int ::= #guarded(Int, Int) [function, total]
  rule #guarded(1, X) => X
  rule #guarded(_, _) => 0 [owise]

  // assert: cond != 0 (finding 2: booleanity is not enforced in circuit)
  rule eval(gate(assert(C)), M, _, _) => #and(#native(rd(C, M), "assert"), #nonZero(rd(C, M)))
  syntax Outcome ::= #nonZero(ValueResult) [function, total]
  rule #nonZero(vOk(native(0))) => violated("assert of zero")
  rule #nonZero(_) => holds() [owise]

  rule eval(gate(constrainToBoolean(V)), M, _, _) => #boolean(rd(V, M), "constrain_to_boolean")
  // assigned_to_le_bits asserts the requested width is at most 255
  rule eval(gate(constrainBits(_, N)), _, _, _) => synthErr("constrain_bits: width " +String Int2String(N) +String " exceeds 255") requires N >Int 255
  rule eval(gate(constrainBits(V, N)), M, _, _) => #and(#native(rd(V, M), "constrain_bits"), #bits(rd(V, M), N)) requires N <=Int 255
  syntax Outcome ::= #bits(ValueResult, Int) [function, total]
  rule #bits(vOk(native(X)), N) => holds() requires N >=Int #frBits orBool fitsBits(X, N)
  rule #bits(vOk(native(X)), N) => violated("value does not fit in " +String Int2String(N) +String " bits") requires N <Int #frBits andBool notBool fitsBits(X, N)
  rule #bits(_, _) => holds() [owise]

  // constrain_eq / test_eq / cond_select: in-circuit dispatch has no arm for
  // JubjubScalar (constrain_eq, findings 7) nor Bytes32 / JubjubScalar (cond_select, finding 6)
  rule eval(gate(constrainEq(A, B)), M, _, Chips) => #and(#need(rd(A, M)), #and(#need(rd(B, M)),
      #and(#chipsForValues(#vals(rd(A, M), rd(B, M)), Chips), #and(#eqSupported(rd(A, M), rd(B, M), "constrain_eq"), #eqValues(rd(A, M), rd(B, M), "constrain_eq fails")))))
  syntax List ::= #vals(ValueResult, ValueResult) [function, total]
  rule #vals(vOk(A), vOk(B)) => ListItem(A) ListItem(B)
  rule #vals(vOk(A), vErr(_)) => ListItem(A)
  rule #vals(vErr(_), vOk(B)) => ListItem(B)
  rule #vals(vErr(_), vErr(_)) => .List
  syntax Outcome ::= #eqSupported(ValueResult, ValueResult, String) [function, total]
  rule #eqSupported(vOk(A), vOk(B), What) => synthErr("Unsupported " +String What +String ": " +String tn(A) +String " == " +String tn(B)) requires notBool sameType(A, B)
  rule #eqSupported(vOk(jubjubScalar(_)), vOk(jubjubScalar(_)), What) => synthErr("Unsupported " +String What +String ": JubjubScalar == JubjubScalar")
  rule #eqSupported(vOk(bytes32(_)), vOk(bytes32(_)), "cond_select") => synthErr("Unsupported cond_select: Bytes32 ? Bytes32")
  rule #eqSupported(vOk(bytes32(_)), vOk(bytes32(_)), "constrain_eq") => holds()
  rule #eqSupported(vOk(bytes32(_)), vOk(bytes32(_)), "test_eq") => holds()
  rule #eqSupported(_, _, _) => holds() [owise]

  rule eval(gate(testEq(A, B, O)), M, _, Chips) => #and(#need(rd(A, M)), #and(#need(rd(B, M)),
      #and(#chipsForValues(#vals(rd(A, M), rd(B, M)), Chips), #and(#eqSupported(rd(A, M), rd(B, M), "test_eq"),
      #matches(#bin2(rd(A, M), rd(B, M), "test_eq"), rdId(O, M), "test_eq output")))))
  rule eval(gate(condSelect(Bit, A, B, O)), M, _, Chips) => #and(#boolean(rd(Bit, M), "cond_select bit"), #and(#need(rd(A, M)), #and(#need(rd(B, M)),
      #and(#chipsForValues(#vals(rd(A, M), rd(B, M)), Chips), #and(#eqSupported(rd(A, M), rd(B, M), "cond_select"),
      #matches(#selBit(#int(rd(Bit, M)), rd(A, M), rd(B, M)), rdId(O, M), "cond_select output"))))))
  syntax ValueResult ::= #selBit(Int, ValueResult, ValueResult) [function, total]
  rule #selBit(1, vOk(A), vOk(B)) => selectV(true, A, B)
  rule #selBit(0, vOk(A), vOk(B)) => selectV(false, A, B)
  rule #selBit(X, vOk(A), vOk(B)) => selectV(false, A, B) requires X =/=Int 0 andBool X =/=Int 1
  rule #selBit(_, _, _) => vErr("missing") [owise]

  syntax ValueResult ::= #bin2(ValueResult, ValueResult, String) [function, total]
  rule #bin2(vErr(S), _, _) => vErr(S)
  rule #bin2(vOk(_), vErr(S), _) => vErr(S)
  rule #bin2(vOk(A), vOk(B), "add") => addV(A, B)
  rule #bin2(vOk(A), vOk(B), "mul") => mulV(A, B)
  rule #bin2(vOk(A), vOk(B), "test_eq") => testEqV(A, B)
  rule #bin2(vOk(A), vOk(B), "ec_mul") => ecMulV(A, B)
  rule #bin2(vOk(A), vOk(B), "bytes32_from_low_high") => #lowHighInCircuit(A, B)
  rule #bin2(vOk(_), vOk(_), Op) => vErr("unknown op " +String Op) [owise]

  // arithmetic: output must equal the off-circuit result; unsupported types are
  // synthesis errors (same dispatch as off-circuit for add/mul/neg/inv/ec_mul)
  rule eval(gate(add(A, B, O)), M, _, Chips) => #and(#need(rd(A, M)), #and(#need(rd(B, M)), #and(#chipsForValues(#vals(rd(A, M), rd(B, M)), Chips),
      #matches(#bin2(rd(A, M), rd(B, M), "add"), rdId(O, M), "add output"))))
  rule eval(gate(mul(A, B, O)), M, _, Chips) => #and(#need(rd(A, M)), #and(#need(rd(B, M)), #and(#chipsForValues(#vals(rd(A, M), rd(B, M)), Chips),
      #matches(#bin2(rd(A, M), rd(B, M), "mul"), rdId(O, M), "mul output"))))
  rule eval(gate(ecMul(A, S, O)), M, _, Chips) => #and(#need(rd(A, M)), #and(#need(rd(S, M)), #and(#chipsForValues(#vals(rd(A, M), rd(S, M)), Chips),
      #matches(#bin2(rd(A, M), rd(S, M), "ec_mul"), rdId(O, M), "ec_mul output"))))
  rule eval(gate(neg(A, O)), M, _, Chips) => #and(#need(rd(A, M)), #and(#chipsForValues(#vals(rd(A, M), vErr("")), Chips), #matches(#un2(rd(A, M), "neg"), rdId(O, M), "neg output")))
  rule eval(gate(inv(A, O)), M, _, Chips) => #and(#need(rd(A, M)), #and(#chipsForValues(#vals(rd(A, M), vErr("")), Chips), #and(#invNonZero(rd(A, M)), #matches(#un2(rd(A, M), "inv"), rdId(O, M), "inv output (a * inv = 1)"))))
  syntax Outcome ::= #invNonZero(ValueResult) [function, total]
  rule #invNonZero(vOk(V)) => violated("inv: a * inv = 1 has no solution at zero") requires isZeroField(V)
  rule #invNonZero(_) => holds() [owise]
  syntax Bool ::= isZeroField(Value) [function, total]
  rule isZeroField(native(0)) => true
  rule isZeroField(secp256k1Base(0)) => true
  rule isZeroField(secp256k1Scalar(0)) => true
  rule isZeroField(secp256r1Base(0)) => true
  rule isZeroField(secp256r1Scalar(0)) => true
  rule isZeroField(curve25519Base(0)) => true
  rule isZeroField(curve25519Scalar(0)) => true
  rule isZeroField(_) => false [owise]
  rule eval(gate(copy(A, O)), M, _, _) => #eqValues(rd(A, M), rdId(O, M), "copy output")
  rule eval(gate(not(A, O)), M, _, _) => #and(#boolean(rd(A, M), "not"), #matches(#notOf(rd(A, M)), rdId(O, M), "not output"))
  syntax ValueResult ::= #notOf(ValueResult) [function, total]
  rule #notOf(vOk(native(X))) => vOk(native(1 -Int X)) requires X ==Int 0 orBool X ==Int 1
  rule #notOf(_) => vErr("not boolean") [owise]
  syntax ValueResult ::= #un2(ValueResult, String) [function, total]
  rule #un2(vErr(S), _) => vErr(S)
  rule #un2(vOk(A), "neg") => negV(A)
  rule #un2(vOk(A), "inv") => invV(A)
  rule #un2(vOk(A), "into_bytes32") => intoBytes32V(A)
  rule #un2(vOk(A), "ec_mul_generator") => ecMulGeneratorV(A)
  rule #un2(vOk(_), Op) => vErr("unknown op " +String Op) [owise]

  rule eval(gate(ecMulGenerator(S, O)), M, _, Chips) => #and(#need(rd(S, M)), #and(#chipsForValues(#vals(rd(S, M), vErr("")), Chips),
      #matches(#un2(rd(S, M), "ec_mul_generator"), rdId(O, M), "ec_mul_generator output")))

  // less_than: the chip rounds the bit width up to max(bits + bits % 2, 4) (finding 4)
  // the chip pads the width to max(bits + bits % 2, 4) and asserts it is at
  // most MAX_BOUND_IN_BITS = 253 (bounded_of_element); wider bounds cannot be built
  rule eval(gate(lessThan(_, _, N, _)), _, _, _) => synthErr("less_than: padded bound " +String Int2String(#ltBits(N)) +String " exceeds MAX_BOUND_IN_BITS = 253")
    requires #ltBits(N) >Int 253
  rule eval(gate(lessThan(A, B, N, O)), M, _, _) => #and(#native(rd(A, M), "less_than"), #and(#native(rd(B, M), "less_than"),
      #and(#bits(rd(A, M), #ltBits(N)), #and(#bits(rd(B, M), #ltBits(N)), #matches(#ltOf(rd(A, M), rd(B, M)), rdId(O, M), "less_than output")))))
    requires #ltBits(N) <=Int 253
  syntax ValueResult ::= #ltOf(ValueResult, ValueResult) [function, total]
  rule #ltOf(vOk(native(A)), vOk(native(B))) => vOk(native(bool2Word(A <Int B)))
  rule #ltOf(_, _) => vErr("not native") [owise]

  rule eval(gate(jubjubScalarFromNative(A, O)), M, _, Chips) => #and(#native(rd(A, M), "jubjub_scalar_from_native"), #and(#chipFor(jubjubScalar(), Chips),
      #matches(#jjOf(rd(A, M)), rdId(O, M), "jubjub_scalar_from_native output")))
  syntax ValueResult ::= #jjOf(ValueResult) [function, total]
  rule #jjOf(vOk(native(X))) => vOk(jubjubScalar(X modInt #rJ))
  rule #jjOf(_) => vErr("not native") [owise]

  // public_input / private_input: the register holds a value of the declared
  // type (assign_incircuit); the guard plays no part (finding 3)
  rule eval(gate(publicInput(_, T, O)), M, _, Chips) => #and(#need(rdId(O, M)), #and(#chipFor(T, Chips), #typed(rdId(O, M), T)))
  rule eval(gate(privateInput(_, T, O)), M, _, Chips) => #and(#need(rdId(O, M)), #and(#chipFor(T, Chips), #typed(rdId(O, M), T)))
  syntax Outcome ::= #typed(ValueResult, IrType) [function, total]
  rule #typed(vOk(V), T) => holds() requires typeOf(V) ==K T
  rule #typed(vOk(V), T) => violated("register has type " +String tn(V) +String ", declared " +String typeName(T)) requires typeOf(V) =/=K T
  rule #typed(vErr(S), _) => unknown(S)

  // div_mod_power_of_two: canonical decomposition of a native value
  rule eval(gate(divModPowerOfTwo(V, N, (Q, (Rm, .Ids)))), M, _, _) => #and(#native(rd(V, M), "div_mod_power_of_two"),
      #and(#matches(#hi(rd(V, M), N), rdId(Q, M), "div_mod quotient"), #matches(#lo(rd(V, M), N), rdId(Rm, M), "div_mod remainder")))
  rule eval(gate(divModPowerOfTwo(_, _, Os)), _, _, _) => synthErr("Unexpected output length of DivModPowerOfTwo instruction") requires lenIds(Os) =/=Int 2
  syntax ValueResult ::= #hi(ValueResult, Int) [function, total]
  rule #hi(vOk(native(X)), N) => vOk(native(highBits(X, N)))
  rule #hi(_, _) => vErr("not native") [owise]
  syntax ValueResult ::= #lo(ValueResult, Int) [function, total]
  rule #lo(vOk(native(X)), N) => vOk(native(lowBits(X, N)))
  rule #lo(_, _) => vErr("not native") [owise]

  // reconstitute_field: divisor < 2^(255 - bits), modulus < 2^bits, output = 2^bits divisor + modulus mod r
  rule eval(gate(reconstituteField(D, Mo, N, O)), M, _, _) => #and(#native(rd(D, M), "reconstitute_field"), #and(#native(rd(Mo, M), "reconstitute_field"),
      #and(#bits(rd(D, M), #frBits -Int N), #and(#bits(rd(Mo, M), N), #matches(#recOf(rd(D, M), rd(Mo, M), N), rdId(O, M), "reconstitute_field output")))))
  syntax ValueResult ::= #recOf(ValueResult, ValueResult, Int) [function, total]
  rule #recOf(vOk(native(D)), vOk(native(Mo)), N) => vOk(native(((D <<Int N) +Int Mo) modInt #r))
  rule #recOf(_, _, _) => vErr("not native") [owise]

  // encode
  rule eval(gate(encode(X, Os)), M, _, Chips) => #and(#need(rd(X, M)), #and(#chipsForValues(#vals(rd(X, M), vErr("")), Chips), #encMatch(rd(X, M), Os, M)))
  syntax Outcome ::= #encMatch(ValueResult, Ids, Map) [function, total]
  rule #encMatch(vOk(V), Os, M) => synthErr("Unexpected output length of encode instruction") requires size(encodeValue(V)) =/=Int lenIds(Os)
  rule #encMatch(vOk(V), Os, M) => #encEach(encodeValue(V), Os, M) requires size(encodeValue(V)) ==Int lenIds(Os)
  rule #encMatch(vErr(S), _, _) => unknown(S)
  syntax Outcome ::= #encEach(List, Ids, Map) [function, total]
  rule #encEach(.List, .Ids, _) => holds()
  rule #encEach(ListItem(X) L, (O, Os), M) => #and(#matches(vOk(native(X)), rdId(O, M), "encode output"), #encEach(L, Os, M))
  rule #encEach(_, _, _) => holds() [owise]

  // coordinates
  rule eval(gate(intoCoordinates(P, X, Y)), M, _, Chips) => #and(#need(rd(P, M)), #and(#chipsForValues(#vals(rd(P, M), vErr("")), Chips), #coordsMatch(rd(P, M), X, Y, M)))
  syntax Outcome ::= #coordsMatch(ValueResult, String, String, Map) [function, total]
  rule #coordsMatch(vOk(secp256k1Point(inf())), _, _, _) => violated("into_coordinates of the identity is unsatisfiable")
  rule #coordsMatch(vOk(secp256r1Point(inf())), _, _, _) => violated("into_coordinates of the identity is unsatisfiable")
  rule #coordsMatch(vOk(V), X, Y, M) => #pairMatch(intoCoordinatesV(V), X, Y, M) [owise]
  rule #coordsMatch(vErr(S), _, _, _) => unknown(S)
  syntax Outcome ::= #pairMatch(PairResult, String, String, Map) [function, total]
  rule #pairMatch(pErr(S), _, _, _) => synthErr(S)
  rule #pairMatch(pOk(A, B), X, Y, M) => #and(#matches(vOk(A), rdId(X, M), "first output"), #matches(vOk(B), rdId(Y, M), "second output"))

  // from_coordinates in circuit: point_from_coordinates constrains the exact
  // (x, y) to lie on the curve; the off-circuit Jubjub path goes through
  // decompression instead (only the parity of x is used)
  rule eval(gate(fromCoordinates(X, Y, O)), M, _, Chips) => #and(#need(rd(X, M)), #and(#need(rd(Y, M)), #and(#chipsForValues(#vals(rd(X, M), rd(Y, M)), Chips),
      #and(#fromCoordsChip(rd(X, M), rd(Y, M), Chips), #fromCoordsGate(rd(X, M), rd(Y, M), rdId(O, M))))))
  // curve and type dispatch of point_from_coordinates: the exact (x, y) must lie
  // on the curve and (after cofactor clearing) in the prime-order subgroup
  syntax Outcome ::= #fromCoordsGate(ValueResult, ValueResult, ValueResult) [function, total]
  rule #fromCoordsGate(vOk(native(X)), vOk(native(Y)), O) => #fromCoordsPt(fromXY(#jubjub, X, Y), #jubjub, jubjubPoint(), O)
  rule #fromCoordsGate(vOk(secp256k1Base(X)), vOk(secp256k1Base(Y)), O) => #fromCoordsPt(fromXY(#secp256k1, X, Y), #secp256k1, secp256k1Point(), O)
  rule #fromCoordsGate(vOk(secp256r1Base(X)), vOk(secp256r1Base(Y)), O) => #fromCoordsPt(fromXY(#secp256r1, X, Y), #secp256r1, secp256r1Point(), O)
  rule #fromCoordsGate(vOk(curve25519Base(X)), vOk(curve25519Base(Y)), O) => #fromCoordsPt(fromXY(#curve25519, X, Y), #curve25519, curve25519Point(), O)
  // the output point's chip must be initialised (`used_chips` does not enable it for this instruction)
  syntax Outcome ::= #fromCoordsChip(ValueResult, ValueResult, Set) [function, total]
  rule #fromCoordsChip(vOk(native(_)), vOk(native(_)), Chips) => #chipFor(jubjubPoint(), Chips)
  rule #fromCoordsChip(_, _, _) => holds() [owise]
  rule #fromCoordsGate(vOk(A), vOk(B), _) => synthErr("Unsupported `from_coordinates` on (" +String tn(A) +String ", " +String tn(B) +String ")") [owise]
  rule #fromCoordsGate(vErr(S), _, _) => unknown(S)
  rule #fromCoordsGate(vOk(_), vErr(S), _) => unknown(S)
  syntax Outcome ::= #fromCoordsPt(PointResult, Curve, IrType, ValueResult) [function, total]
  rule #fromCoordsPt(ptErr(_), _, _, _) => violated("from_coordinates: (x, y) is not on the curve")
  rule #fromCoordsPt(ptOk(Q), C, _, _) => violated("from_coordinates: point is not in the prime-order subgroup") requires notBool inSubgroup(C, Q)
  rule #fromCoordsPt(ptOk(Q), C, T, O) => #matches(vOk(#wrapPoint(Q, T)), O, "from_coordinates output") requires inSubgroup(C, Q)

  // bytes
  rule eval(gate(intoBytes32(A, O)), M, _, Chips) => #and(#need(rd(A, M)), #and(#chipsForValues(#vals(rd(A, M), vErr("")), Chips), #matches(#un2(rd(A, M), "into_bytes32"), rdId(O, M), "into_bytes32 output")))
  rule eval(gate(fromBytes32(B, T, O)), M, _, Chips) => #and(#need(rd(B, M)), #and(#chipFor(T, Chips), #matches(#fromB32c(rd(B, M), T), rdId(O, M), "from_bytes32 output")))
  syntax ValueResult ::= #fromB32c(ValueResult, IrType) [function, total]
  rule #fromB32c(vOk(bytes32(B)), T) => fromBytes32V(T, B)
  rule #fromB32c(vOk(V), _) => vErr("cannot convert " +String tn(V) +String " to Bytes32") [owise]
  rule #fromB32c(vErr(S), _) => vErr(S)
  rule eval(gate(reverseBytes(B, O)), M, _, _) => #and(#need(rd(B, M)), #matches(#revOf(rd(B, M)), rdId(O, M), "reverse_bytes output"))
  syntax ValueResult ::= #revOf(ValueResult) [function, total]
  rule #revOf(vOk(bytes32(B))) => vOk(bytes32(reverseBytes(B)))
  rule #revOf(vOk(V)) => vErr("cannot convert " +String tn(V) +String " to Bytes32") [owise]
  rule #revOf(vErr(S)) => vErr(S)
  rule eval(gate(bytes32IntoLowHigh(B, L, H)), M, _, _) => #and(#need(rd(B, M)), #lowHighMatch(rd(B, M), L, H, M))
  syntax Outcome ::= #lowHighMatch(ValueResult, String, String, Map) [function, total]
  rule #lowHighMatch(vOk(bytes32(B)), L, H, M) => #pairMatch(bytes32IntoLowHighV(B), L, H, M)
  rule #lowHighMatch(vOk(V), _, _, _) => synthErr("cannot convert " +String tn(V) +String " to Bytes32") [owise]
  rule #lowHighMatch(vErr(S), _, _, _) => unknown(S)
  // bytes32_from_low_high: high must be Native in circuit, low may be any
  // field element (finding 8); low's byte 31 must be zero
  rule eval(gate(bytes32FromLowHigh(L, H, O)), M, _, Chips) => #and(#need(rd(L, M)), #and(#chipsForValues(#vals(rd(L, M), vErr("")), Chips), #and(#native(rd(H, M), "bytes32_from_low_high high"),
      #and(#lowHighBounds(rd(L, M), rd(H, M)), #matches(#bin2(rd(L, M), rd(H, M), "bytes32_from_low_high"), rdId(O, M), "bytes32_from_low_high output")))))
  // assert_equal_to_fixed(bytes_low[31], 0) and the byte range check on high are unsatisfiable, not unbuildable
  syntax Outcome ::= #lowHighBounds(ValueResult, ValueResult) [function, total]
  rule #lowHighBounds(vOk(L), vOk(native(H))) => #lowHighBounds2(intoBytes32V(L), H)
  rule #lowHighBounds(_, _) => holds() [owise]
  syntax Outcome ::= #lowHighBounds2(ValueResult, Int) [function, total]
  rule #lowHighBounds2(vOk(bytes32(BL)), _) => violated("bytes32_from_low_high: low operand uses byte 31") requires BL[31] =/=Int 0
  rule #lowHighBounds2(vOk(bytes32(BL)), H) => violated("bytes32_from_low_high: high operand is not a byte") requires BL[31] ==Int 0 andBool notBool fitsBits(H, 8)
  rule #lowHighBounds2(_, _) => holds() [owise]
  syntax ValueResult ::= #lowHighInCircuit(Value, Value) [function, total]
  rule #lowHighInCircuit(L, native(H)) => #lowHighIC2(intoBytes32V(L), H)
  rule #lowHighInCircuit(_, H) => vErr("cannot convert " +String tn(H) +String " to Native") [owise]
  syntax ValueResult ::= #lowHighIC2(ValueResult, Int) [function, total]
  rule #lowHighIC2(vErr(S), _) => vErr(S)
  rule #lowHighIC2(vOk(bytes32(BL)), H) => vOk(bytes32(replaceAtBytes(BL, 31, Int2Bytes(1, H, LE)))) requires BL[31] ==Int 0 andBool fitsBits(H, 8)
  rule #lowHighIC2(vOk(bytes32(BL)), _) => vErr("low operand uses byte 31") requires BL[31] =/=Int 0
  rule #lowHighIC2(vOk(bytes32(_)), H) => vErr("high operand is not a byte") requires notBool fitsBits(H, 8)
  rule #lowHighIC2(vOk(_), _) => vErr("not bytes") [owise]

  // hashes
  rule eval(gate(transientHash(Xs, O)), M, _, Chips) => #and(#needAll(Xs, M), #and(#chipNamed("poseidon", Chips), #matches(#th3(#natives(Xs, M)), rdId(O, M), "transient_hash output")))
  rule eval(gate(hashToCurve(Xs, O)), M, _, Chips) => #and(#needAll(Xs, M), #and(#chipNamed("poseidon", Chips), #and(#chipNamed("jubjub", Chips), #matches(#htc3(#natives(Xs, M)), rdId(O, M), "hash_to_curve output"))))
  rule eval(gate(persistentHash(Al, Xs, O)), M, _, Chips) => #and(#needAll(Xs, M), #and(#chipNamed("sha2_256", Chips), #matches(#std3(#natives(Xs, M), Al, "sha256"), rdId(O, M), "persistent_hash output")))
  rule eval(gate(keccak256(Al, Xs, O)), M, _, Chips) => #and(#needAll(Xs, M), #and(#chipNamed("keccak_256", Chips), #matches(#std3(#natives(Xs, M), Al, "keccak256"), rdId(O, M), "keccak256 output")))
  syntax Outcome ::= #chipNamed(String, Set) [function, total]
  rule #chipNamed(N, Chips) => holds() requires N in Chips
  rule #chipNamed(N, Chips) => synthErr("chip not initialised: " +String N) requires notBool (N in Chips)
  syntax Outcome ::= #needAll(Operands, Map) [function, total]
  rule #needAll(.Operands, _) => holds()
  rule #needAll((X, Xs), M) => #and(#native(rd(X, M), "hash input"), #needAll(Xs, M))
  syntax IntsResult ::= #natives(Operands, Map) [function, total]
  rule #natives(.Operands, _) => isOk(.List)
  rule #natives((X, Xs), M) => #consNat(rd(X, M), #natives(Xs, M))
  syntax IntsResult ::= #consNat(ValueResult, IntsResult) [function, total]
  rule #consNat(vOk(native(X)), isOk(L)) => isOk(ListItem(X) L)
  rule #consNat(vOk(V), _) => isErr("cannot convert " +String tn(V) +String " to Native") requires notBool isNative(V)
  rule #consNat(vErr(S), _) => isErr(S)
  rule #consNat(_, isErr(S)) => isErr(S) [owise]
  syntax ValueResult ::= #th3(IntsResult) [function, total]
  rule #th3(isErr(S)) => vErr(S)
  rule #th3(isOk(L)) => vOk(native(poseidonHash(L)))
  syntax ValueResult ::= #htc3(IntsResult) [function, total]
  rule #htc3(isErr(S)) => vErr(S)
  rule #htc3(isOk(L)) => vOk(jubjubPoint(hashToCurve(L)))
  syntax ValueResult ::= #std3(IntsResult, Alignment, String) [function, total]
  rule #std3(isErr(S), _, _) => vErr(S)
  rule #std3(isOk(L), Al, H) => #std4(alignedBytesCircuit(Al, L), H)
  // fab_decode_to_bytes: atoms as off-circuit (byte range constraints), no check
  // of surplus fields, options "not yet implemented" and compress atoms rejected
  // at synthesis (#onlyAtoms and #hasCompress in ZKIR-STATIC)
  syntax BytesResult ::= alignedBytesCircuit(Alignment, List) [function, total]
  rule alignedBytesCircuit(alignment(Segs), Fs) => #abcOk(#alignedBytes(#onlyAtoms(Segs), Fs, .Bytes)) requires #onlyAtoms(Segs) ==K Segs andBool notBool #hasCompress(Segs)
  rule alignedBytesCircuit(alignment(Segs), _) => bErr("synthesis: in-circuit decoding of alignment options is not yet implemented") requires #onlyAtoms(Segs) =/=K Segs
  rule alignedBytesCircuit(alignment(Segs), _) => bErr("synthesis: Cannot decode compressed value from field elements") requires #onlyAtoms(Segs) ==K Segs andBool #hasCompress(Segs)
  syntax BytesResult ::= #abcOk(ABResult) [function, total]
  rule #abcOk(abOk(B, _)) => bOk(B)
  rule #abcOk(abErr(S)) => bErr(S)
  syntax ValueResult ::= #std4(BytesResult, String) [function, total]
  rule #std4(bErr(S), _) => vErr(S)
  rule #std4(bOk(B), "sha256") => vOk(bytes32(sha256Bytes(B)))
  rule #std4(bOk(B), _) => vOk(bytes32(keccak256Bytes(B))) [owise]

  // output is emitted as outputGate(vals, signature); a bare gate(output) or
  // any gate without a relation here is reported, not assumed to hold
  rule eval(gate(I), _, _, _) => unsupported("no in-circuit relation modelled for this instruction: " +String #opName(I)) [owise]
endmodule
