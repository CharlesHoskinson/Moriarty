// The target contract, tier one (plan-iter3 M5a, completed in M5b): the
// obligations a compiler must meet for its ZKIR output to be well formed and
// provable, decided from the program alone. targetContract(P) is a total
// function over Program that returns one obligation per line of the contract,
// each met(), failed(msg), notApplicable() (no instruction of that kind
// occurs) or info(fact) (a fact the later tiers quote rather than a check).
//
//   wf                         the static check of ZKIR-WF
//   version                    the version pin, 3.<minor>
//   surface                    base (midnight-ledger 92e8bdd3), extension
//                              (midnight-zkir 2ffe2d1) or common to both
//   chips.enabled              the chip set `IrSource::used_chips` initialises
//   chips.gating               no instruction needs a chip that set omits
//                              (finding 13, K4: keygen would panic)
//   width.less_than            padded bound at most MAX_BOUND_IN_BITS = 253
//   width.constrain_bits       width at most F::NUM_BITS = 255
//   width.div_mod_power_of_two bits at most FR_BYTES_STORED * 8 = 248
//   width.reconstitute_field   bits at most 248
//   alignment.persistent_hash  no option segment, no compress atom
//   alignment.keccak256        the same
//   alignment.sha512           the same (extension surface only)
//   circuit.static.output      every `output` operand whose static type is
//                              known has the declared output type, and the
//                              arity matches the signature
//   circuit.static.bool_gate   and / or / xor have at least one input, every
//                              input of known type a Bool (extension surface)
//   circuit.static.constant    every load_constant encoding decodes on the
//                              strict decoder (extension surface)
//   circuit.static.bytes_bounds nth and slice stay within the static length
//                              of their byte string, slice length at least 1
//                              (extension surface)
//   circuit.static.dispatch    no instruction's in-circuit dispatch is
//                              unsupported for the static types of its
//                              operands (the synthErr arms of
//                              ZKIR-CONSTRAINTS that do not read a value)
//   commitment                 the value of do_communications_commitment: the
//                              preimage-side half of the rule is tier two
//   inventory.instructions     instruction count by op
//   inventory.types            the types the program declares
//
// The chip, width, alignment and circuit.static obligations are the
// program-only half of the synthErr outcomes of ZKIR-CONSTRAINTS: the gates
// evaluate them on a witness, this module on the static type environment of
// ZKIR-STATIC (bindOuts, stype). Unknown-witness key generation
// (`keygen_vk` on `MidnightCircuit`) performs exactly these checks and no
// other before it has a witness, so a program with every obligation met or
// not applicable keys, and a failed obligation names the instruction whose
// synthesis fails (tools/provability.py). A check that needs a value (an
// unknown static type, a bound on a witness value) stays with the gates.
// Tiers two (the off-circuit run succeeds and the memory is in the modelled
// witness space) and three (the circuit accepts) need a preimage and run
// through ZKIR-VM and the circuit oracle (tools/zkir_kast.py `contract
// --preimage`). Arms for the extension surface are in ZKIR-EXT-CONTRACT
// (zkir-ext.k).
requires "zkir-syntax.k"
requires "zkir-static.k"

module ZKIR-CONTRACT
  imports ZKIR-WF
  imports ZKIR-STATIC
  imports STRING

  syntax ObligationStatus ::= met()                 [symbol(met)]
                            | failed(String)        [symbol(failed)]
                            | notApplicable()       [symbol(notApplicable)]
                            | info(String)          [symbol(info)]
  syntax Obligation  ::= obligation(String, ObligationStatus) [symbol(obligation)]
  syntax Obligations ::= List{Obligation, ","} [symbol(obligations), terminator-symbol(.obligations)]

  syntax Obligations ::= targetContract(Program) [function, total]
  rule targetContract(program(Mi, Ins, Outs, DoComm, Is) #as P) =>
      obligation("wf", #wfStatus(wf(P))),
      obligation("version", info("3." +String Int2String(Mi))),
      obligation("surface", #surface(P)),
      obligation("chips.enabled", info(#chipsOrNone(usedChips(P)))),
      obligation("chips.gating", #chipGating(Is, inputEnv(Ins), usedChips(P), 0)),
      obligation("width.less_than", #walk(ltWidth(), Is, inputEnv(Ins), 0, false)),
      obligation("width.constrain_bits", #walk(cbWidth(), Is, inputEnv(Ins), 0, false)),
      obligation("width.div_mod_power_of_two", #walk(dmWidth(), Is, inputEnv(Ins), 0, false)),
      obligation("width.reconstitute_field", #walk(rfWidth(), Is, inputEnv(Ins), 0, false)),
      obligation("alignment.persistent_hash", #walk(align("persistent_hash"), Is, inputEnv(Ins), 0, false)),
      obligation("alignment.keccak256", #walk(align("keccak256"), Is, inputEnv(Ins), 0, false)),
      obligation("alignment.sha512", #walk(align("sha512"), Is, inputEnv(Ins), 0, false)),
      obligation("circuit.static.output", #walk(outputSig(Outs), Is, inputEnv(Ins), 0, false)),
      obligation("circuit.static.bool_gate", #walk(boolGate(), Is, inputEnv(Ins), 0, false)),
      obligation("circuit.static.constant", #walk(constant(), Is, inputEnv(Ins), 0, false)),
      obligation("circuit.static.bytes_bounds", #walk(bytesBounds(), Is, inputEnv(Ins), 0, false)),
      obligation("circuit.static.dispatch", #walk(dispatch(), Is, inputEnv(Ins), 0, false)),
      obligation("commitment", info(#commInfo(DoComm))),
      obligation("inventory.instructions", info(#instrInventory(Is))),
      obligation("inventory.types", info(#typeInventory(P))),
      .Obligations

  // --- wf ---------------------------------------------------------------------------------
  syntax ObligationStatus ::= #wfStatus(WfResult) [function, total]
  rule #wfStatus(wfOk()) => met()
  rule #wfStatus(wfError(S)) => failed(S)

  // --- surface ---------------------------------------------------------------------------------
  // reverse_bytes exists only at 92e8bdd3; the extension types and instructions only at 2ffe2d1
  syntax ObligationStatus ::= #surface(Program) [function, total]
  rule #surface(program(_, _, _, _, Is) #as P) => #surface2(anyExtInstr(Is) orBool anyExtType(declaredTypes(P)), hasOp(Is, "reverse_bytes"))
  syntax ObligationStatus ::= #surface2(Bool, Bool) [function, total]
  rule #surface2(true, true)   => failed("uses reverse_bytes, which midnight-zkir 2ffe2d1 removed, together with extension types or instructions")
  rule #surface2(true, false)  => info("extension: midnight-zkir 2ffe2d1 (ZKIR-EXT)")
  rule #surface2(false, true)  => info("base: midnight-ledger 92e8bdd3 (reverse_bytes is base only)")
  rule #surface2(false, false) => info("common: loads on midnight-ledger 92e8bdd3 and midnight-zkir 2ffe2d1")

  // --- chips ----------------------------------------------------------------------------------------
  syntax ObligationStatus ::= #chipGating(Instrs, Map, Set, Int) [function, total]
  rule #chipGating(.Instrs, _, _, _) => met()
  rule #chipGating((I ; Is), E, Chips, Idx) => #chipGating(Is, bindOuts(I, E), Chips, Idx +Int 1)
    requires chipNeeds(I, E) <=Set Chips
  rule #chipGating((I ; _), E, Chips, Idx) => failed("instruction " +String Int2String(Idx) +String " (" +String #opName(I) +String ") needs chip "
      +String #chipNames(chipNeeds(I, E) -Set Chips) +String ", which used_chips does not enable")
    requires notBool (chipNeeds(I, E) <=Set Chips)

  // chip names in the order of the ZkStdLibArch fields
  syntax String ::= #chipsOrNone(Set) [function, total]
  rule #chipsOrNone(S) => "none" requires size(S) ==Int 0
  rule #chipsOrNone(S) => #chipNames(S) requires size(S) >Int 0
  syntax String ::= #chipNames(Set) [function, total]
  rule #chipNames(S) => #joinPresent(#chipOrder, S, "")
  syntax List ::= "#chipOrder" [function, total]
  rule #chipOrder => ListItem("jubjub") ListItem("poseidon") ListItem("sha2_256") ListItem("sha2_512") ListItem("keccak_256") ListItem("secp256k1") ListItem("p256") ListItem("curve25519")
  syntax String ::= #joinPresent(List, Set, String) [function, total]
  rule #joinPresent(.List, _, Acc) => Acc
  rule #joinPresent(ListItem(N:String) L, S, Acc) => #joinPresent(L, S, #comma(Acc, N)) requires N in S
  rule #joinPresent(ListItem(N:String) L, S, Acc) => #joinPresent(L, S, Acc) requires notBool (N in S)
  rule #joinPresent(ListItem(_) L, S, Acc) => #joinPresent(L, S, Acc) [owise]
  syntax String ::= #comma(String, String) [function, total]
  rule #comma("", N) => N
  rule #comma(Acc, N) => Acc +String ", " +String N requires Acc =/=String ""

  // --- one walk per obligation, over the static type environment ------------------------------------
  // applies(C, I) marks the instructions the obligation is about (none:
  // notApplicable); defect(C, I, E) is "" when I passes under the environment
  // E of the registers defined before it, and the failure text otherwise. The
  // walk extends E with bindOuts after each instruction.
  syntax Check ::= ltWidth() | cbWidth() | dmWidth() | rfWidth() | align(String)
                 | outputSig(IrTypes) | boolGate() | constant() | bytesBounds() | dispatch()
  syntax Bool ::= applies(Check, Instr) [function, total]
  rule applies(ltWidth(), lessThan(_, _, _, _))         => true
  rule applies(cbWidth(), constrainBits(_, _))          => true
  rule applies(dmWidth(), divModPowerOfTwo(_, _, _))    => true
  rule applies(rfWidth(), reconstituteField(_, _, _, _))=> true
  rule applies(align(Op), I)                            => #opName(I) ==String Op
  rule applies(outputSig(_), output(_))                 => true
  rule applies(dispatch(), _)                           => true
  rule applies(_, _) => false [owise]
  syntax String ::= defect(Check, Instr, Map) [function, total]
  rule defect(ltWidth(), lessThan(_, _, N, _), _) => "less_than: bits " +String Int2String(N) +String " pads to " +String Int2String(#ltBits(N)) +String ", above MAX_BOUND_IN_BITS = 253"
    requires #ltBits(N) >Int 253
  rule defect(cbWidth(), constrainBits(_, N), _) => "constrain_bits: width " +String Int2String(N) +String " exceeds 255"
    requires N >Int 255
  rule defect(dmWidth(), divModPowerOfTwo(_, N, _), _) => "div_mod_power_of_two: bits " +String Int2String(N) +String " exceeds FR_BYTES_STORED * 8 = 248"
    requires N >Int #frBytesStored *Int 8
  rule defect(rfWidth(), reconstituteField(_, _, N, _), _) => "reconstitute_field: bits " +String Int2String(N) +String " exceeds FR_BYTES_STORED * 8 = 248"
    requires N >Int #frBytesStored *Int 8
  rule defect(align(Op), I, _) => #alignDefect(Op, alignmentOf(I)) requires #opName(I) ==String Op
  rule defect(outputSig(Ts), output(Vs), E) => #outputDefect(Vs, Ts, E)
  rule defect(dispatch(), I, E) => dispatchDefect(I, E)
  rule defect(_, _, _) => "" [owise]
  syntax String ::= #alignDefect(String, AlignOpt) [function, total]
  rule #alignDefect(Op, someAlign(alignment(Segs))) => Op +String ": alignment has an option segment, which the in-circuit decoder does not implement"
    requires #onlyAtoms(Segs) =/=K Segs
  rule #alignDefect(Op, someAlign(alignment(Segs))) => Op +String ": alignment has a compress atom, which cannot be decoded from field elements"
    requires #onlyAtoms(Segs) ==K Segs andBool #hasCompress(Segs)
  rule #alignDefect(_, _) => "" [owise]

  syntax ObligationStatus ::= #walk(Check, Instrs, Map, Int, Bool) [function, total]
  rule #walk(_, .Instrs, _, _, false) => notApplicable()
  rule #walk(_, .Instrs, _, _, true)  => met()
  rule #walk(C, (I ; Is), E, Idx, Seen) => #walkStep(C, Is, bindOuts(I, E), Idx, Seen orBool applies(C, I), defect(C, I, E))
  syntax ObligationStatus ::= #walkStep(Check, Instrs, Map, Int, Bool, String) [function, total]
  rule #walkStep(C, Is, E, Idx, Seen, "") => #walk(C, Is, E, Idx +Int 1, Seen)
  rule #walkStep(_, _, _, Idx, _, D) => failed("instruction " +String Int2String(Idx) +String ": " +String D) requires D =/=String ""

  // the first non-empty defect
  syntax String ::= #first(String, String) [function, total]
  rule #first("", B) => B
  rule #first(A, _) => A requires A =/=String ""
  // the type as the messages print it (Bytes<n> on the extension surface)
  syntax String ::= tyStr(IrType) [function, total]
  rule tyStr(T) => typeName(T) [owise]

  // --- circuit.static.output (`I::Output`, ir_vm.rs `circuit`) ---------------------------------------
  // the arity is checked first, then each operand's runtime type against the
  // signature; here the static type, where known
  syntax String ::= #outputDefect(Operands, IrTypes, Map) [function, total]
  rule #outputDefect(Vs, Ts, _) => "output: signature declares " +String Int2String(lenIrTypes(Ts)) +String " return values but instruction has " +String Int2String(lenOperands(Vs))
    requires lenOperands(Vs) =/=Int lenIrTypes(Ts)
  rule #outputDefect(Vs, Ts, E) => #outputPositions(Vs, Ts, E, 0) requires lenOperands(Vs) ==Int lenIrTypes(Ts)
  syntax String ::= #outputPositions(Operands, IrTypes, Map, Int) [function, total]
  rule #outputPositions((V, Vs), (T, Ts), E, I) => #first(#outputPos(stype(V, E), T, I), #outputPositions(Vs, Ts, E, I +Int 1))
  rule #outputPositions(_, _, _, _) => "" [owise]
  syntax String ::= #outputPos(SType, IrType, Int) [function, total]
  rule #outputPos(known(S), T, I) => "output position " +String Int2String(I) +String ": signature declares " +String tyStr(T) +String " but operand has static type " +String tyStr(S)
    requires S =/=K T
  rule #outputPos(_, _, _) => "" [owise]

  // --- circuit.static.dispatch (`*_incircuit` in ir_instructions/*.rs, the conversions of ir_types.rs) ----
  // The type sets of the in-circuit match arms. Each `_ =>` arm of an
  // `*_incircuit` function is a synthesis error, and `TryFrom<CircuitValue>`
  // for AssignedNative / [AssignedByte; 32] fails on any other variant; both
  // depend on the types alone. The extension surface adds arms in
  // ZKIR-EXT-CONTRACT.
  syntax Bool ::= arithT(IrType) [function, total]   // add.rs: Native, the four point types, the six foreign fields
  rule arithT(native())           => true
  rule arithT(jubjubPoint())      => true
  rule arithT(secp256k1Point())   => true
  rule arithT(secp256k1Base())    => true
  rule arithT(secp256k1Scalar())  => true
  rule arithT(secp256r1Point())   => true
  rule arithT(secp256r1Base())    => true
  rule arithT(secp256r1Scalar())  => true
  rule arithT(curve25519Point())  => true
  rule arithT(curve25519Base())   => true
  rule arithT(curve25519Scalar()) => true
  rule arithT(_) => false [owise]
  syntax Bool ::= fieldT(IrType) [function, total]   // mul.rs, inv.rs, into_bytes32.rs, from_bytes32.rs: Native and the six foreign fields
  rule fieldT(native())           => true
  rule fieldT(secp256k1Base())    => true
  rule fieldT(secp256k1Scalar())  => true
  rule fieldT(secp256r1Base())    => true
  rule fieldT(secp256r1Scalar())  => true
  rule fieldT(curve25519Base())   => true
  rule fieldT(curve25519Scalar()) => true
  rule fieldT(_) => false [owise]
  syntax Bool ::= negT(IrType) [function, total]     // neg.rs: arithT (and Bool at 2ffe2d1)
  rule negT(T) => arithT(T) [owise]
  syntax Bool ::= selectT(IrType) [function, total]  // select.rs: arithT (and Bool, Byte at 2ffe2d1); no byte strings, no JubjubScalar (finding 6)
  rule selectT(T) => arithT(T) [owise]
  syntax Bool ::= eqT(IrType) [function, total]      // constrain_eq.rs, eq.rs: arithT and Bytes32 (and Bool, Byte, Bytes<n> at 2ffe2d1); no JubjubScalar (finding 7)
  rule eqT(bytes32()) => true
  rule eqT(T) => arithT(T) [owise]
  syntax Bool ::= generatorT(IrType) [function, total]   // `I::EcMulGenerator`: JubjubScalar and Secp256k1Scalar
  rule generatorT(jubjubScalar())    => true
  rule generatorT(secp256k1Scalar()) => true
  rule generatorT(_) => false [owise]

  syntax TPred ::= arithP() | fieldP() | negP() | selectP() | eqP()
  syntax Bool ::= predT(TPred, IrType) [function, total]
  rule predT(arithP(), T)  => arithT(T)
  rule predT(fieldP(), T)  => fieldT(T)
  rule predT(negP(), T)    => negT(T)
  rule predT(selectP(), T) => selectT(T)
  rule predT(eqP(), T)     => eqT(T)

  // a binary arm of equal operand types: the defect when both types are known and the arm is missing
  syntax String ::= #pairDefect(String, String, SType, SType, TPred) [function, total]
  rule #pairDefect(Pre, Sep, known(A), known(B), P) => Pre +String tyStr(A) +String Sep +String tyStr(B)
    requires A =/=K B orBool notBool predT(P, A)
  rule #pairDefect(_, _, _, _, _) => "" [owise]
  syntax String ::= #unDefect(String, SType, TPred) [function, total]
  rule #unDefect(Pre, known(A), P) => Pre +String tyStr(A) requires notBool predT(P, A)
  rule #unDefect(_, _, _) => "" [owise]
  // `TryFrom<CircuitValue> for AssignedNative`
  syntax String ::= #nativeDefect(String, SType) [function, total]
  rule #nativeDefect(What, known(T)) => What +String ": cannot convert " +String tyStr(T) +String " to Native" requires T =/=K native()
  rule #nativeDefect(_, _) => "" [owise]
  syntax String ::= #nativesDefect(String, Operands, Map) [function, total]
  rule #nativesDefect(_, .Operands, _) => ""
  rule #nativesDefect(What, (X, Xs), E) => #first(#nativeDefect(What, stype(X, E)), #nativesDefect(What, Xs, E))
  // `TryFrom<CircuitValue> for [AssignedByte; 32]` (circuit_value_to_bytes32 at 2ffe2d1)
  syntax String ::= #bytes32Defect(String, SType) [function, total]
  rule #bytes32Defect(What, known(T)) => What +String ": cannot convert " +String tyStr(T) +String " to Bytes32" requires T =/=K bytes32()
  rule #bytes32Defect(_, _) => "" [owise]

  syntax String ::= dispatchDefect(Instr, Map) [function, total]
  rule dispatchDefect(add(A, B, _), E)  => #pairDefect("Unsupported addition: ", " + ", stype(A, E), stype(B, E), arithP())
  rule dispatchDefect(mul(A, B, _), E)  => #pairDefect("Unsupported multiplication: ", " x ", stype(A, E), stype(B, E), fieldP())
  rule dispatchDefect(neg(A, _), E)     => #unDefect("Unsupported negation of ", stype(A, E), negP())
  rule dispatchDefect(inv(A, _), E)     => #unDefect("Unsupported inversion of ", stype(A, E), fieldP())
  rule dispatchDefect(ecMul(A, S, _), E) => #ecMulDefect(stype(A, E), stype(S, E))
  rule dispatchDefect(ecMulGenerator(S, _), E) => #generatorDefect(stype(S, E))
  rule dispatchDefect(intoBytes32(A, _), E) => #unDefect("Unsupported into_bytes32 for ", stype(A, E), fieldP())
  rule dispatchDefect(fromBytes32(B, T, _), E) => #first(#bytes32Defect("from_bytes32", stype(B, E)), #fromBytes32Defect(T))
  rule dispatchDefect(intoCoordinates(P, _, _), E) => #coordsDefect(stype(P, E))
  rule dispatchDefect(fromCoordinates(X, Y, _), E) => #fromCoordsDefect(stype(X, E), stype(Y, E))
  rule dispatchDefect(condSelect(Bit, A, B, _), E) => #first(#nativeDefect("cond_select bit", stype(Bit, E)), #pairDefect("Unsupported cond_select: ", " ? ", stype(A, E), stype(B, E), selectP()))
  rule dispatchDefect(constrainEq(A, B), E) => #pairDefect("Unsupported constrain_eq: ", " == ", stype(A, E), stype(B, E), eqP())
  rule dispatchDefect(testEq(A, B, _), E)   => #pairDefect("Unsupported test_eq: ", " == ", stype(A, E), stype(B, E), eqP())
  rule dispatchDefect(bytes32FromLowHigh(L, H, _), E) => #first(#nativeDefect("bytes32_from_low_high high", stype(H, E)), #unDefect("Unsupported into_bytes32 for ", stype(L, E), fieldP()))
  rule dispatchDefect(bytes32IntoLowHigh(B, _, _), E) => #bytes32Defect("bytes32_into_low_high", stype(B, E))
  rule dispatchDefect(reverseBytes(B, _), E) => #bytes32Defect("reverse_bytes", stype(B, E))
  rule dispatchDefect(encode(X, Os), E)       => #encodeDefect(stype(X, E), lenIds(Os))
  rule dispatchDefect(assert(C), E)           => #nativeDefect("assert", stype(C, E))
  rule dispatchDefect(constrainBits(V, _), E) => #nativeDefect("constrain_bits", stype(V, E))
  rule dispatchDefect(constrainToBoolean(V), E) => #nativeDefect("constrain_to_boolean", stype(V, E))
  rule dispatchDefect(lessThan(A, B, _, _), E) => #first(#nativeDefect("less_than", stype(A, E)), #nativeDefect("less_than", stype(B, E)))
  rule dispatchDefect(divModPowerOfTwo(V, _, _), E) => #nativeDefect("div_mod_power_of_two", stype(V, E))
  rule dispatchDefect(reconstituteField(D, Mo, _, _), E) => #first(#nativeDefect("reconstitute_field", stype(D, E)), #nativeDefect("reconstitute_field", stype(Mo, E)))
  rule dispatchDefect(not(A, _), E)           => #nativeDefect("not", stype(A, E))
  rule dispatchDefect(jubjubScalarFromNative(N, _), E) => #nativeDefect("jubjub_scalar_from_native", stype(N, E))
  rule dispatchDefect(impact(G, Xs), E)       => #first(#nativeDefect("impact guard", stype(G, E)), #nativesDefect("impact", Xs, E))
  rule dispatchDefect(transientHash(Xs, _), E)      => #nativesDefect("transient_hash", Xs, E)
  rule dispatchDefect(hashToCurve(Xs, _), E)        => #nativesDefect("hash_to_curve", Xs, E)
  rule dispatchDefect(persistentHash(_, Xs, _), E)  => #nativesDefect("persistent_hash", Xs, E)
  rule dispatchDefect(keccak256(_, Xs, _), E)       => #nativesDefect("keccak256", Xs, E)
  // copy, output (circuit.static.output), public_input and private_input (the
  // guard is not read in circuit, finding 3) have no typed dispatch
  rule dispatchDefect(_, _) => "" [owise]

  syntax String ::= #ecMulDefect(SType, SType) [function, total]
  rule #ecMulDefect(known(A), known(S)) => "Unsupported EC multiplication: " +String tyStr(A) +String " x " +String tyStr(S)
    requires #pointOfScalar(known(S)) =/=K known(A)
  rule #ecMulDefect(_, _) => "" [owise]
  syntax String ::= #generatorDefect(SType) [function, total]
  rule #generatorDefect(known(S)) => "Unsupported EcMulGenerator for scalar of type " +String tyStr(S) requires notBool generatorT(S)
  rule #generatorDefect(_) => "" [owise]
  syntax String ::= #fromBytes32Defect(IrType) [function, total]
  rule #fromBytes32Defect(T) => "Unsupported from_bytes32 for " +String tyStr(T) requires notBool fieldT(T)
  rule #fromBytes32Defect(_) => "" [owise]
  syntax String ::= #coordsDefect(SType) [function, total]
  rule #coordsDefect(known(T)) => "Unsupported coordinate extraction of " +String tyStr(T) requires #baseOf(known(T)) ==K unknownT()
  rule #coordsDefect(_) => "" [owise]
  syntax String ::= #fromCoordsDefect(SType, SType) [function, total]
  rule #fromCoordsDefect(known(X), known(Y)) => "Unsupported `from_coordinates` on (" +String tyStr(X) +String ", " +String tyStr(Y) +String ")"
    requires X =/=K Y orBool #pointOfBase(known(X)) ==K unknownT()
  rule #fromCoordsDefect(_, _) => "" [owise]
  // `I::Encode`: the in-circuit encoding has encodedLen(type) elements, one per output register
  syntax String ::= #encodeDefect(SType, Int) [function, total]
  rule #encodeDefect(known(T), N) => "Unexpected output length of encode instruction: " +String tyStr(T) +String " encodes to " +String Int2String(encodedLen(T)) +String " elements, " +String Int2String(N) +String " outputs named"
    requires encodedLen(T) =/=Int N
  rule #encodeDefect(_, _) => "" [owise]

  // --- commitment flag ------------------------------------------------------------------------------------
  syntax String ::= #commInfo(Bool) [function, total]
  rule #commInfo(true)  => "do_communications_commitment = true: the preimage must carry the (commitment, opening) pair, pi[1] is the commitment, poseidon is enabled"
  rule #commInfo(false) => "do_communications_commitment = false: no commitment pair is expected and the public-input vector has no commitment slot"

  // --- inventories ------------------------------------------------------------------------------------------
  syntax String ::= #instrInventory(Instrs) [function, total]
  rule #instrInventory(Is) => Int2String(lenInstrs(Is)) +String " instructions" +String #countsSuffix(#opCounts(Is, .List, .Map))
  syntax Counts ::= counts(List, Map)   // op names in first-seen order, and their counts
  syntax Counts ::= #opCounts(Instrs, List, Map) [function, total]
  rule #opCounts(.Instrs, Names, M) => counts(Names, M)
  rule #opCounts((I ; Is), Names, M) => #opCounts(Is, Names, M[#opName(I) <- {M[#opName(I)]}:>Int +Int 1]) requires #opName(I) in_keys(M)
  rule #opCounts((I ; Is), Names, M) => #opCounts(Is, Names ListItem(#opName(I)), M[#opName(I) <- 1]) requires notBool (#opName(I) in_keys(M))
  syntax String ::= #countsSuffix(Counts) [function, total]
  rule #countsSuffix(counts(.List, _)) => ""
  rule #countsSuffix(counts(Names, M)) => ": " +String #renderCounts(Names, M, "") requires size(Names) >Int 0
  syntax String ::= #renderCounts(List, Map, String) [function, total]
  rule #renderCounts(.List, _, Acc) => Acc
  rule #renderCounts(ListItem(N:String) L, M, Acc) => #renderCounts(L, M, #comma(Acc, N +String " " +String Int2String({M[N]}:>Int)))
  rule #renderCounts(ListItem(_) L, M, Acc) => #renderCounts(L, M, Acc) [owise]

  syntax String ::= #typeInventory(Program) [function, total]
  rule #typeInventory(program(_, Ins, Outs, _, Is)) =>
      "inputs: " +String #typeNamesOrNone(#inputTypes(Ins)) +String "; outputs: " +String #typeNamesOrNone(#outTypeList(Outs))
      +String "; declared in instructions: " +String #typeNamesOrNone(instrTypes(Is))
  syntax String ::= #typeNamesOrNone(List) [function, total]
  rule #typeNamesOrNone(.List) => "none"
  rule #typeNamesOrNone(L) => #typeNames(L, "") requires size(L) >Int 0
  syntax String ::= #typeNames(List, String) [function, total]
  rule #typeNames(.List, Acc) => Acc
  rule #typeNames(ListItem(T:IrType) L, Acc) => #typeNames(L, #comma(Acc, typeName(T)))
  rule #typeNames(ListItem(_) L, Acc) => #typeNames(L, Acc) [owise]
endmodule
