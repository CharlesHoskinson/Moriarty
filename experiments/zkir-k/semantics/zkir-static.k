// Program-only facts about the circuit, shared by the constraint checker
// (ZKIR-CONSTRAINTS, zkir-constraints.k) and the target contract
// (ZKIR-CONTRACT, zkir-contract.k): the chip set `IrSource::used_chips`
// enables, the chips each instruction's gate demands, the alignment shapes the
// in-circuit byte decoder accepts, the padded width of less_than, and a static
// type environment for registers. Nothing here reads a value or a preimage, so
// a check-only definition can import this module without ZKIR-OPS. Arms for
// the extension surface are in ZKIR-EXT-STATIC (zkir-ext.k).
requires "zkir-syntax.k"

module ZKIR-STATIC
  imports ZKIR-SYNTAX
  imports SET
  imports LIST
  imports MAP
  imports K-EQUAL

  // --- Chips (`IrSource::used_chips`, ir_vm.rs) -----------------------------------------
  // A foreign chip is initialised only when its types occur among the program
  // inputs or public/private inputs, the Jubjub chip additionally by
  // hash_to_curve, Poseidon by transient_hash, hash_to_curve or the
  // communications commitment, SHA-256 by persistent_hash, Keccak by keccak256
  // (sha512 enables SHA-512 on the extension surface only). from_bytes32,
  // jubjub_scalar_from_native, native from_coordinates and load_constant
  // initialise nothing (finding 13, K4).
  syntax Set ::= usedChips(Program) [function, total]
  rule usedChips(program(_, Ins, _, DoComm, Is))
    => #chipsOfTypes(#inputTypes(Ins) #ioTypes(Is)) |Set #chipsOfInstrs(Is) |Set #commChip(DoComm)
  syntax Set ::= #commChip(Bool) [function, total]
  rule #commChip(true) => SetItem("poseidon")
  rule #commChip(false) => .Set
  syntax List ::= #inputTypes(TypedIds) [function, total]
  rule #inputTypes(.TypedIds) => .List
  rule #inputTypes((typedId(_, T), Rest)) => ListItem(T) #inputTypes(Rest)
  syntax List ::= #ioTypes(Instrs) [function, total]
  rule #ioTypes(.Instrs) => .List
  rule #ioTypes((publicInput(_, T, _) ; Is)) => ListItem(T) #ioTypes(Is)
  rule #ioTypes((privateInput(_, T, _) ; Is)) => ListItem(T) #ioTypes(Is)
  rule #ioTypes((_ ; Is)) => #ioTypes(Is) [owise]
  syntax Set ::= #chipsOfTypes(List) [function, total]
  rule #chipsOfTypes(.List) => .Set
  rule #chipsOfTypes(ListItem(T:IrType) L) => chipOfType(T) |Set #chipsOfTypes(L)
  rule #chipsOfTypes(ListItem(_) L) => #chipsOfTypes(L) [owise]
  syntax Set ::= #chipsOfInstrs(Instrs) [function, total]
  rule #chipsOfInstrs(.Instrs) => .Set
  rule #chipsOfInstrs((hashToCurve(_, _) ; Is)) => (SetItem("jubjub") SetItem("poseidon")) |Set #chipsOfInstrs(Is)
  rule #chipsOfInstrs((transientHash(_, _) ; Is)) => SetItem("poseidon") |Set #chipsOfInstrs(Is)
  rule #chipsOfInstrs((persistentHash(_, _, _) ; Is)) => SetItem("sha2_256") |Set #chipsOfInstrs(Is)
  rule #chipsOfInstrs((keccak256(_, _, _) ; Is)) => SetItem("keccak_256") |Set #chipsOfInstrs(Is)
  rule #chipsOfInstrs((_ ; Is)) => #chipsOfInstrs(Is) [owise]

  // chip needed to hold a value of a type (.Set = none)
  syntax Set ::= chipOfType(IrType) [function, total]
  rule chipOfType(jubjubPoint())     => SetItem("jubjub")
  rule chipOfType(jubjubScalar())    => SetItem("jubjub")
  rule chipOfType(secp256k1Point())  => SetItem("secp256k1")
  rule chipOfType(secp256k1Base())   => SetItem("secp256k1")
  rule chipOfType(secp256k1Scalar()) => SetItem("secp256k1")
  rule chipOfType(secp256r1Point())  => SetItem("p256")
  rule chipOfType(secp256r1Base())   => SetItem("p256")
  rule chipOfType(secp256r1Scalar()) => SetItem("p256")
  rule chipOfType(curve25519Point()) => SetItem("curve25519")
  rule chipOfType(curve25519Base())  => SetItem("curve25519")
  rule chipOfType(curve25519Scalar())=> SetItem("curve25519")
  rule chipOfType(_) => .Set [owise]

  // --- Static register types ----------------------------------------------------------------
  // A forward type environment (String |-> IrType) over the instruction list:
  // inputs carry their declared type, and every instruction whose output type
  // is fixed by its operand types binds its outputs. A register whose type
  // cannot be told without a value (an unsupported operand combination, or
  // concat on the extension surface) stays absent, and stype reports unknownT.
  syntax SType ::= known(IrType) [symbol(known)] | unknownT() [symbol(unknownT)]
  syntax SType ::= stype(Operand, Map) [function, total]
  rule stype(imm(_), _) => known(native())
  rule stype(var(X), E) => known({E[X]}:>IrType) requires X in_keys(E)
  rule stype(var(X), E) => unknownT() requires notBool X in_keys(E)

  syntax Map ::= inputEnv(TypedIds) [function, total]
  rule inputEnv(.TypedIds) => .Map
  rule inputEnv((typedId(X, T), Rest)) => inputEnv(Rest)[X <- T]

  syntax Map ::= #bind(String, SType, Map) [function, total]
  rule #bind(X, known(T), E) => E[X <- T]
  rule #bind(_, unknownT(), E) => E
  syntax Map ::= #bindAll(Ids, IrType, Map) [function, total]
  rule #bindAll(.Ids, _, E) => E
  rule #bindAll((X, Xs), T, E) => #bindAll(Xs, T, E[X <- T])
  syntax SType ::= #firstKnown(SType, SType) [function, total]
  rule #firstKnown(known(T), _) => known(T)
  rule #firstKnown(unknownT(), S) => S

  // curve maps between point, coordinate and scalar types (Jubjub coordinates are Native)
  syntax SType ::= #baseOf(SType) [function, total]
  rule #baseOf(known(jubjubPoint()))     => known(native())
  rule #baseOf(known(secp256k1Point()))  => known(secp256k1Base())
  rule #baseOf(known(secp256r1Point()))  => known(secp256r1Base())
  rule #baseOf(known(curve25519Point())) => known(curve25519Base())
  rule #baseOf(_) => unknownT() [owise]
  syntax SType ::= #pointOfBase(SType) [function, total]
  rule #pointOfBase(known(native()))         => known(jubjubPoint())
  rule #pointOfBase(known(secp256k1Base()))  => known(secp256k1Point())
  rule #pointOfBase(known(secp256r1Base()))  => known(secp256r1Point())
  rule #pointOfBase(known(curve25519Base())) => known(curve25519Point())
  rule #pointOfBase(_) => unknownT() [owise]
  syntax SType ::= #pointOfScalar(SType) [function, total]
  rule #pointOfScalar(known(jubjubScalar()))     => known(jubjubPoint())
  rule #pointOfScalar(known(secp256k1Scalar()))  => known(secp256k1Point())
  rule #pointOfScalar(known(secp256r1Scalar()))  => known(secp256r1Point())
  rule #pointOfScalar(known(curve25519Scalar())) => known(curve25519Point())
  rule #pointOfScalar(_) => unknownT() [owise]

  // bindOuts(I, E): E extended with the outputs of I whose type is fixed
  syntax Map ::= bindOuts(Instr, Map) [function, total]
  rule bindOuts(encode(_, Os), E)                  => #bindAll(Os, native(), E)
  rule bindOuts(condSelect(_, A, B, O), E)         => #bind(O, #firstKnown(stype(A, E), stype(B, E)), E)
  rule bindOuts(copy(V, O), E)                     => #bind(O, stype(V, E), E)
  rule bindOuts(ecMul(A, _, O), E)                 => #bind(O, stype(A, E), E)
  rule bindOuts(ecMulGenerator(S, O), E)           => #bind(O, #pointOfScalar(stype(S, E)), E)
  rule bindOuts(hashToCurve(_, O), E)              => E[O <- jubjubPoint()]
  rule bindOuts(intoCoordinates(P, X, Y), E)       => #bind(Y, #baseOf(stype(P, E)), #bind(X, #baseOf(stype(P, E)), E))
  rule bindOuts(fromCoordinates(X, _, O), E)       => #bind(O, #pointOfBase(stype(X, E)), E)
  rule bindOuts(intoBytes32(_, O), E)              => E[O <- bytes32()]
  rule bindOuts(fromBytes32(_, T, O), E)           => E[O <- T]
  rule bindOuts(reverseBytes(_, O), E)             => E[O <- bytes32()]
  rule bindOuts(bytes32IntoLowHigh(_, L, H), E)    => (E[L <- bytes32()])[H <- native()]
  rule bindOuts(bytes32FromLowHigh(_, _, O), E)    => E[O <- bytes32()]
  rule bindOuts(divModPowerOfTwo(_, _, Os), E)     => #bindAll(Os, native(), E)
  rule bindOuts(reconstituteField(_, _, _, O), E)  => E[O <- native()]
  rule bindOuts(transientHash(_, O), E)            => E[O <- native()]
  rule bindOuts(persistentHash(_, _, O), E)        => E[O <- bytes32()]
  rule bindOuts(keccak256(_, _, O), E)             => E[O <- bytes32()]
  rule bindOuts(testEq(_, _, O), E)                => E[O <- native()]
  rule bindOuts(add(A, B, O), E)                   => #bind(O, #firstKnown(stype(A, E), stype(B, E)), E)
  rule bindOuts(mul(A, B, O), E)                   => #bind(O, #firstKnown(stype(A, E), stype(B, E)), E)
  rule bindOuts(neg(A, O), E)                      => #bind(O, stype(A, E), E)
  rule bindOuts(inv(A, O), E)                      => #bind(O, stype(A, E), E)
  rule bindOuts(not(_, O), E)                      => E[O <- native()]
  rule bindOuts(lessThan(_, _, _, O), E)           => E[O <- native()]
  rule bindOuts(jubjubScalarFromNative(_, O), E)   => E[O <- jubjubScalar()]
  rule bindOuts(publicInput(_, T, O), E)           => E[O <- T]
  rule bindOuts(privateInput(_, T, O), E)          => E[O <- T]
  rule bindOuts(_, E)                              => E [owise]   // assert, constrain_*, impact, output write nothing

  // --- Chips a gate demands ------------------------------------------------------------------
  // chipNeeds(I, E): the chips the in-circuit relation of I asks `ZkStdLib` for
  // (ZKIR-CONSTRAINTS `#chipFor`, `#chipsForValues`, `#chipNamed`), read off the
  // static types of its operands and its own constructor. A chip in this set
  // that usedChips does not enable is a keygen panic (finding 13, K4).
  syntax Set ::= chipNeeds(Instr, Map) [function, total]
  rule chipNeeds(I, E) => #chipsOfReads(reads(I), E) |Set #chipNeedsOf(I, E)
  syntax Set ::= #chipsOfReads(Operands, Map) [function, total]
  rule #chipsOfReads(.Operands, _) => .Set
  rule #chipsOfReads((imm(_), Os), E) => #chipsOfReads(Os, E)
  rule #chipsOfReads((var(X), Os), E) => chipOfType({E[X]}:>IrType) |Set #chipsOfReads(Os, E) requires X in_keys(E)
  rule #chipsOfReads((var(X), Os), E) => #chipsOfReads(Os, E) requires notBool X in_keys(E)
  syntax Set ::= #chipNeedsOf(Instr, Map) [function, total]
  rule #chipNeedsOf(fromBytes32(_, T, _), _)         => chipOfType(T)
  rule #chipNeedsOf(jubjubScalarFromNative(_, _), _) => SetItem("jubjub")
  rule #chipNeedsOf(fromCoordinates(X, Y, _), E)     => SetItem("jubjub")
    requires stype(X, E) ==K known(native()) andBool stype(Y, E) ==K known(native())
  rule #chipNeedsOf(hashToCurve(_, _), _)            => SetItem("jubjub") SetItem("poseidon")
  rule #chipNeedsOf(transientHash(_, _), _)          => SetItem("poseidon")
  rule #chipNeedsOf(persistentHash(_, _, _), _)      => SetItem("sha2_256")
  rule #chipNeedsOf(keccak256(_, _, _), _)           => SetItem("keccak_256")
  rule #chipNeedsOf(publicInput(_, T, _), _)         => chipOfType(T)
  rule #chipNeedsOf(privateInput(_, T, _), _)        => chipOfType(T)
  rule #chipNeedsOf(_, _)                            => .Set [owise]

  // --- Types a program declares -----------------------------------------------------------------
  // inputs, outputs, and the types named inside instructions (public_input,
  // private_input, from_bytes32; load_constant on the extension surface)
  syntax List ::= declaredTypes(Program) [function, total]
  rule declaredTypes(program(_, Ins, Outs, _, Is)) => #inputTypes(Ins) #outTypeList(Outs) instrTypes(Is)
  syntax List ::= #outTypeList(IrTypes) [function, total]
  rule #outTypeList(.IrTypes) => .List
  rule #outTypeList((T, Ts)) => ListItem(T) #outTypeList(Ts)
  syntax List ::= instrTypes(Instrs) [function, total]
  rule instrTypes(.Instrs) => .List
  rule instrTypes((I ; Is)) => #instrTypes(I) instrTypes(Is)
  syntax List ::= #instrTypes(Instr) [function, total]
  rule #instrTypes(publicInput(_, T, _))  => ListItem(T)
  rule #instrTypes(privateInput(_, T, _)) => ListItem(T)
  rule #instrTypes(fromBytes32(_, T, _))  => ListItem(T)
  rule #instrTypes(_) => .List [owise]

  // --- Surface membership ---------------------------------------------------------------------------
  // true only for constructors ZKIR-EXT-SYNTAX adds (arms in ZKIR-EXT-STATIC)
  syntax Bool ::= isExtInstr(Instr) [function, total]
  rule isExtInstr(_) => false [owise]
  syntax Bool ::= isExtType(IrType) [function, total]
  rule isExtType(_) => false [owise]
  syntax Bool ::= anyExtInstr(Instrs) [function, total]
  rule anyExtInstr(.Instrs) => false
  rule anyExtInstr((I ; Is)) => isExtInstr(I) orBool anyExtInstr(Is)
  syntax Bool ::= anyExtType(List) [function, total]
  rule anyExtType(.List) => false
  rule anyExtType(ListItem(T:IrType) L) => isExtType(T) orBool anyExtType(L)
  rule anyExtType(ListItem(_) L) => anyExtType(L) [owise]
  syntax Bool ::= hasOp(Instrs, String) [function, total]
  rule hasOp(.Instrs, _) => false
  rule hasOp((I ; Is), Op) => #opName(I) ==String Op orBool hasOp(Is, Op)

  // --- Widths and alignments the circuit accepts ----------------------------------------------------
  // less_than pads the width to max(bits + bits % 2, 4) (`std.lower_than`),
  // and bounded_of_element asserts the padded width is at most 253 (finding 4, K5)
  syntax Int ::= #ltBits(Int) [function, total]
  rule #ltBits(N) => maxInt(N +Int (N modInt 2), 4)

  // fab_decode_to_bytes: options are "not yet implemented" and compress atoms
  // cannot be decoded from field elements; both are synthesis errors
  syntax Segments ::= #onlyAtoms(Segments) [function, total]
  rule #onlyAtoms(.Segments) => .Segments
  rule #onlyAtoms((atom(A), Segs)) => atom(A), #onlyAtoms(Segs)
  rule #onlyAtoms((option(_), Segs)) => #onlyAtoms(Segs)
  syntax Bool ::= #hasCompress(Segments) [function, total]
  rule #hasCompress(.Segments) => false
  rule #hasCompress((atom(compressAtom()), _)) => true
  rule #hasCompress((_, Segs)) => #hasCompress(Segs) [owise]

  // the alignment a byte-hash instruction decodes through that path
  syntax AlignOpt ::= someAlign(Alignment) [symbol(someAlign)] | noAlign() [symbol(noAlign)]
  syntax AlignOpt ::= alignmentOf(Instr) [function, total]
  rule alignmentOf(persistentHash(Al, _, _)) => someAlign(Al)
  rule alignmentOf(keccak256(Al, _, _))      => someAlign(Al)
  rule alignmentOf(_) => noAlign() [owise]
endmodule
