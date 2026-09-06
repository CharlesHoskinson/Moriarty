// ZKIR v3 abstract syntax and static well-formedness.
//
// Surface: the 34 instructions and 13 types of `zkir-v3/src/ir.rs` and
// `zkir-v3/src/ir_types.rs` in midnightntwrk/midnight-ledger at commit
// 92e8bdd3 (the surface mechanized by input-output-hk/arc-zkir). Every
// production carries a `symbol(_)` whose name is the JSON `op` of the
// instruction (or the JSON key of the type), so that the pyk preprocessor in
// tools/zkir_kast.py can build terms without a concrete K grammar.
//
// Programs do not enter K through a concrete syntax: tools/zkir_kast.py reads
// the JSON artifact and produces the `Program` term (plan decision CLM-0712).

module ZKIR-SYNTAX
  imports INT
  imports STRING
  imports BOOL

  // --- Program -------------------------------------------------------------
  // program(minorVersion, inputs, outputTypes, doCommunicationsCommitment, instructions)
  syntax Program ::= program(Int, TypedIds, IrTypes, Bool, Instrs) [symbol(program)]

  // Identifiers are the JSON strings verbatim (`Identifier(pub String)`).
  syntax TypedId  ::= typedId(String, IrType)                           [symbol(typedId)]
  syntax TypedIds ::= List{TypedId, ","}   [symbol(typedIds), terminator-symbol(.typedIds)]
  syntax IrTypes  ::= List{IrType, ","}    [symbol(irTypes),  terminator-symbol(.irTypes)]
  syntax Ids      ::= List{String, ","}    [symbol(ids),      terminator-symbol(.ids)]
  syntax Instrs   ::= List{Instr, ";"}     [symbol(instrs),   terminator-symbol(.instrs)]
  syntax Operands ::= List{Operand, ","}   [symbol(operands), terminator-symbol(.operands)]

  // --- Types (ir_types.rs, serde names in comments) -----------------------
  syntax IrType ::= native()          [symbol(Native)]          // "Scalar<BLS12-381>"
                  | bytes32()         [symbol(Bytes32)]         // "Bytes<32>"
                  | jubjubPoint()     [symbol(JubjubPoint)]     // "Point<Jubjub>"
                  | jubjubScalar()    [symbol(JubjubScalar)]    // "Scalar<Jubjub>"
                  | secp256k1Point()  [symbol(Secp256k1Point)]  // "Point<Secp256k1>"
                  | secp256k1Base()   [symbol(Secp256k1Base)]   // "Base<Secp256k1>"
                  | secp256k1Scalar() [symbol(Secp256k1Scalar)] // "Scalar<Secp256k1>"
                  | secp256r1Point()  [symbol(Secp256r1Point)]  // "Point<Secp256r1>"
                  | secp256r1Base()   [symbol(Secp256r1Base)]   // "Base<Secp256r1>"
                  | secp256r1Scalar() [symbol(Secp256r1Scalar)] // "Scalar<Secp256r1>"
                  | curve25519Point() [symbol(Curve25519Point)] // "Point<Curve25519>"
                  | curve25519Base()  [symbol(Curve25519Base)]  // "Base<Curve25519>"
                  | curve25519Scalar()[symbol(Curve25519Scalar)]// "Scalar<Curve25519>"

  // Number of raw field elements that encode a value of each type
  // (`IrType::encoded_len`).
  syntax Int ::= encodedLen(IrType) [function, total]
  rule encodedLen(native())          => 1
  rule encodedLen(bytes32())         => 2
  rule encodedLen(jubjubPoint())     => 2
  rule encodedLen(jubjubScalar())    => 1
  rule encodedLen(secp256k1Point())  => 5
  rule encodedLen(secp256k1Base())   => 2
  rule encodedLen(secp256k1Scalar()) => 2
  rule encodedLen(secp256r1Point())  => 5
  rule encodedLen(secp256r1Base())   => 2
  rule encodedLen(secp256r1Scalar()) => 2
  rule encodedLen(curve25519Point()) => 4
  rule encodedLen(curve25519Base())  => 2
  rule encodedLen(curve25519Scalar())=> 2

  // --- Operands -------------------------------------------------------------
  // `Operand::Variable(Identifier)` | `Operand::Immediate(Fr)`; the immediate is
  // already reduced into [0, r) by the preprocessor.
  syntax Operand ::= var(String) [symbol(var)]
                   | imm(Int)    [symbol(imm)]

  // Optional guard of public_input / private_input (`Option<Operand>`).
  syntax Guard ::= noGuard()       [symbol(noGuard)]
                 | guard(Operand)  [symbol(guard)]

  // --- Alignments (base_crypto::fab::Alignment) -----------------------------
  // JSON: [{"tag":"atom","value":{"tag":"bytes","length":32}}, ...]
  syntax Alignment  ::= alignment(Segments)   [symbol(alignment)]
  syntax Segments   ::= List{Segment, ","}    [symbol(segments),   terminator-symbol(.segments)]
  syntax Segment    ::= atom(Atom)            [symbol(atom)]
                      | option(Alignments)    [symbol(option)]
  syntax Alignments ::= List{Alignment, "|"}  [symbol(alignments), terminator-symbol(.alignments)]
  syntax Atom       ::= fieldAtom()           [symbol(fieldAtom)]
                      | bytesAtom(Int)        [symbol(bytesAtom)]
                      | compressAtom()        [symbol(compressAtom)]

  // --- Instructions (ir.rs `enum Instruction`, JSON `op` in the symbol) -----
  // Argument order follows the Rust field order; identifiers written by the
  // instruction come last.
  syntax Instr ::= encode(Operand, Ids)                            [symbol(encode)]
                 | assert(Operand)                                 [symbol(assert)]
                 | condSelect(Operand, Operand, Operand, String)   [symbol(cond_select)]   // bit, a, b, output
                 | constrainBits(Operand, Int)                     [symbol(constrain_bits)]
                 | constrainEq(Operand, Operand)                   [symbol(constrain_eq)]
                 | constrainToBoolean(Operand)                     [symbol(constrain_to_boolean)]
                 | copy(Operand, String)                           [symbol(copy)]
                 | impact(Operand, Operands)                       [symbol(impact)]        // guard, inputs
                 | ecMul(Operand, Operand, String)                 [symbol(ec_mul)]        // a, scalar, output
                 | ecMulGenerator(Operand, String)                 [symbol(ec_mul_generator)]
                 | hashToCurve(Operands, String)                   [symbol(hash_to_curve)]
                 | intoCoordinates(Operand, String, String)        [symbol(into_coordinates)] // point, x, y
                 | fromCoordinates(Operand, Operand, String)       [symbol(from_coordinates)] // x, y, output
                 | intoBytes32(Operand, String)                    [symbol(into_bytes32)]
                 | fromBytes32(Operand, IrType, String)            [symbol(from_bytes32)]  // bytes, type, output
                 | reverseBytes(Operand, String)                   [symbol(reverse_bytes)]
                 | bytes32IntoLowHigh(Operand, String, String)     [symbol(bytes32_into_low_high)] // bytes, low, high
                 | bytes32FromLowHigh(Operand, Operand, String)    [symbol(bytes32_from_low_high)] // low, high, output
                 | divModPowerOfTwo(Operand, Int, Ids)             [symbol(div_mod_power_of_two)] // val, bits, [div, mod]
                 | reconstituteField(Operand, Operand, Int, String)[symbol(reconstitute_field)]  // divisor, modulus, bits, output
                 | transientHash(Operands, String)                 [symbol(transient_hash)]
                 | persistentHash(Alignment, Operands, String)     [symbol(persistent_hash)]
                 | keccak256(Alignment, Operands, String)          [symbol(keccak256)]
                 | testEq(Operand, Operand, String)                [symbol(test_eq)]
                 | add(Operand, Operand, String)                   [symbol(add)]
                 | mul(Operand, Operand, String)                   [symbol(mul)]
                 | neg(Operand, String)                            [symbol(neg)]
                 | inv(Operand, String)                            [symbol(inv)]
                 | not(Operand, String)                            [symbol(not)]
                 | lessThan(Operand, Operand, Int, String)         [symbol(less_than)]     // a, b, bits, output
                 | jubjubScalarFromNative(Operand, String)         [symbol(jubjub_scalar_from_native)]
                 | publicInput(Guard, IrType, String)              [symbol(public_input)]
                 | privateInput(Guard, IrType, String)             [symbol(private_input)]
                 | output(Operands)                                [symbol(output)]

  // --- Field constants --------------------------------------------------------
  // BLS12-381 scalar field modulus r (the native field), FR_BITS = 255,
  // FR_BYTES_STORED = 31 (transient-crypto/src/curve.rs).
  syntax Int ::= "#blsScalarModulus" [function, total]
  rule #blsScalarModulus => 52435875175126190479447740508185965837690552500527637822603658699938581184513
  syntax Int ::= "#frBits" [function, total]
  rule #frBits => 255
  syntax Int ::= "#frBytesStored" [function, total]
  rule #frBytesStored => 31

  // --- Reads and writes of an instruction -----------------------------------
  // reads(I): every operand the instruction resolves off-circuit, in the order
  // `IrSource::preprocess` resolves them. writes(I): identifiers it defines.
  syntax Operands ::= reads(Instr) [function, total]
  rule reads(encode(X, _))                     => X, .Operands
  rule reads(assert(C))                        => C, .Operands
  rule reads(condSelect(B, A, C, _))           => B, A, C, .Operands
  rule reads(constrainBits(V, _))              => V, .Operands
  rule reads(constrainEq(A, B))                => A, B, .Operands
  rule reads(constrainToBoolean(V))            => V, .Operands
  rule reads(copy(V, _))                       => V, .Operands
  rule reads(impact(G, Is))                    => G, Is
  rule reads(ecMul(A, S, _))                   => A, S, .Operands
  rule reads(ecMulGenerator(S, _))             => S, .Operands
  rule reads(hashToCurve(Is, _))               => Is
  rule reads(intoCoordinates(P, _, _))         => P, .Operands
  rule reads(fromCoordinates(X, Y, _))         => X, Y, .Operands
  rule reads(intoBytes32(X, _))                => X, .Operands
  rule reads(fromBytes32(B, _, _))             => B, .Operands
  rule reads(reverseBytes(B, _))               => B, .Operands
  rule reads(bytes32IntoLowHigh(B, _, _))      => B, .Operands
  rule reads(bytes32FromLowHigh(L, H, _))      => L, H, .Operands
  rule reads(divModPowerOfTwo(V, _, _))        => V, .Operands
  rule reads(reconstituteField(D, M, _, _))    => M, D, .Operands
  rule reads(transientHash(Is, _))             => Is
  rule reads(persistentHash(_, Is, _))         => Is
  rule reads(keccak256(_, Is, _))              => Is
  rule reads(testEq(A, B, _))                  => A, B, .Operands
  rule reads(add(A, B, _))                     => A, B, .Operands
  rule reads(mul(A, B, _))                     => A, B, .Operands
  rule reads(neg(A, _))                        => A, .Operands
  rule reads(inv(A, _))                        => A, .Operands
  rule reads(not(A, _))                        => A, .Operands
  rule reads(lessThan(A, B, _, _))             => A, B, .Operands
  rule reads(jubjubScalarFromNative(N, _))     => N, .Operands
  rule reads(publicInput(noGuard(), _, _))     => .Operands
  rule reads(publicInput(guard(G), _, _))      => G, .Operands
  rule reads(privateInput(noGuard(), _, _))    => .Operands
  rule reads(privateInput(guard(G), _, _))     => G, .Operands
  rule reads(output(Vs))                       => Vs

  syntax Ids ::= writes(Instr) [function, total]
  rule writes(encode(_, Os))                   => Os
  rule writes(assert(_))                       => .Ids
  rule writes(condSelect(_, _, _, O))          => O, .Ids
  rule writes(constrainBits(_, _))             => .Ids
  rule writes(constrainEq(_, _))               => .Ids
  rule writes(constrainToBoolean(_))           => .Ids
  rule writes(copy(_, O))                      => O, .Ids
  rule writes(impact(_, _))                    => .Ids
  rule writes(ecMul(_, _, O))                  => O, .Ids
  rule writes(ecMulGenerator(_, O))            => O, .Ids
  rule writes(hashToCurve(_, O))               => O, .Ids
  rule writes(intoCoordinates(_, X, Y))        => X, Y, .Ids
  rule writes(fromCoordinates(_, _, O))        => O, .Ids
  rule writes(intoBytes32(_, O))               => O, .Ids
  rule writes(fromBytes32(_, _, O))            => O, .Ids
  rule writes(reverseBytes(_, O))              => O, .Ids
  rule writes(bytes32IntoLowHigh(_, L, H))     => L, H, .Ids
  rule writes(bytes32FromLowHigh(_, _, O))     => O, .Ids
  rule writes(divModPowerOfTwo(_, _, Os))      => Os
  rule writes(reconstituteField(_, _, _, O))   => O, .Ids
  rule writes(transientHash(_, O))             => O, .Ids
  rule writes(persistentHash(_, _, O))         => O, .Ids
  rule writes(keccak256(_, _, O))              => O, .Ids
  rule writes(testEq(_, _, O))                 => O, .Ids
  rule writes(add(_, _, O))                    => O, .Ids
  rule writes(mul(_, _, O))                    => O, .Ids
  rule writes(neg(_, O))                       => O, .Ids
  rule writes(inv(_, O))                       => O, .Ids
  rule writes(not(_, O))                       => O, .Ids
  rule writes(lessThan(_, _, _, O))            => O, .Ids
  rule writes(jubjubScalarFromNative(_, O))    => O, .Ids
  rule writes(publicInput(_, _, O))            => O, .Ids
  rule writes(privateInput(_, _, O))           => O, .Ids
  rule writes(output(_))                       => .Ids

  // --- Names ------------------------------------------------------------------
  // typeName(T): the Rust `IrType` variant name, used in run-time messages, in
  // the static contract inventory and by `tn` in ZKIR-OPS (base types here,
  // extension types in ZKIR-EXT-SYNTAX).
  syntax String ::= typeName(IrType) [function, total]
  rule typeName(native())          => "Native"
  rule typeName(bytes32())         => "Bytes32"
  rule typeName(jubjubPoint())     => "JubjubPoint"
  rule typeName(jubjubScalar())    => "JubjubScalar"
  rule typeName(secp256k1Point())  => "Secp256k1Point"
  rule typeName(secp256k1Base())   => "Secp256k1Base"
  rule typeName(secp256k1Scalar()) => "Secp256k1Scalar"
  rule typeName(secp256r1Point())  => "Secp256r1Point"
  rule typeName(secp256r1Base())   => "Secp256r1Base"
  rule typeName(secp256r1Scalar()) => "Secp256r1Scalar"
  rule typeName(curve25519Point()) => "Curve25519Point"
  rule typeName(curve25519Base())  => "Curve25519Base"
  rule typeName(curve25519Scalar())=> "Curve25519Scalar"

  // #opName(I): the JSON `op` of the instruction (its symbol), for messages and
  // the contract inventory; the [owise] arm keeps the function total on any
  // surface whose arms are missing.
  syntax String ::= #opName(Instr) [function, total]
  rule #opName(encode(_, _))                    => "encode"
  rule #opName(assert(_))                       => "assert"
  rule #opName(condSelect(_, _, _, _))          => "cond_select"
  rule #opName(constrainBits(_, _))             => "constrain_bits"
  rule #opName(constrainEq(_, _))               => "constrain_eq"
  rule #opName(constrainToBoolean(_))           => "constrain_to_boolean"
  rule #opName(copy(_, _))                      => "copy"
  rule #opName(impact(_, _))                    => "impact"
  rule #opName(ecMul(_, _, _))                  => "ec_mul"
  rule #opName(ecMulGenerator(_, _))            => "ec_mul_generator"
  rule #opName(hashToCurve(_, _))               => "hash_to_curve"
  rule #opName(intoCoordinates(_, _, _))        => "into_coordinates"
  rule #opName(fromCoordinates(_, _, _))        => "from_coordinates"
  rule #opName(intoBytes32(_, _))               => "into_bytes32"
  rule #opName(fromBytes32(_, _, _))            => "from_bytes32"
  rule #opName(reverseBytes(_, _))              => "reverse_bytes"
  rule #opName(bytes32IntoLowHigh(_, _, _))     => "bytes32_into_low_high"
  rule #opName(bytes32FromLowHigh(_, _, _))     => "bytes32_from_low_high"
  rule #opName(divModPowerOfTwo(_, _, _))       => "div_mod_power_of_two"
  rule #opName(reconstituteField(_, _, _, _))   => "reconstitute_field"
  rule #opName(transientHash(_, _))             => "transient_hash"
  rule #opName(persistentHash(_, _, _))         => "persistent_hash"
  rule #opName(keccak256(_, _, _))              => "keccak256"
  rule #opName(testEq(_, _, _))                 => "test_eq"
  rule #opName(add(_, _, _))                    => "add"
  rule #opName(mul(_, _, _))                    => "mul"
  rule #opName(neg(_, _))                       => "neg"
  rule #opName(inv(_, _))                       => "inv"
  rule #opName(not(_, _))                       => "not"
  rule #opName(lessThan(_, _, _, _))            => "less_than"
  rule #opName(jubjubScalarFromNative(_, _))    => "jubjub_scalar_from_native"
  rule #opName(publicInput(_, _, _))            => "public_input"
  rule #opName(privateInput(_, _, _))           => "private_input"
  rule #opName(output(_))                       => "output"
  rule #opName(_)                               => "instruction" [owise]

  // --- List lengths -----------------------------------------------------------
  syntax Int ::= lenOperands(Operands) [function, total]
  rule lenOperands(.Operands)  => 0
  rule lenOperands(_, Os)      => 1 +Int lenOperands(Os)
  syntax Int ::= lenIds(Ids) [function, total]
  rule lenIds(.Ids)            => 0
  rule lenIds(_, Xs)           => 1 +Int lenIds(Xs)
  syntax Int ::= lenIrTypes(IrTypes) [function, total]
  rule lenIrTypes(.IrTypes)    => 0
  rule lenIrTypes(_, Ts)       => 1 +Int lenIrTypes(Ts)
  syntax Int ::= lenInstrs(Instrs) [function, total]
  rule lenInstrs(.Instrs)      => 0
  rule lenInstrs(_ ; Is)       => 1 +Int lenInstrs(Is)

  // --- Field elements an alignment consumes -----------------------------------
  // `fab_decode_to_bytes_atom` (ir_vm.rs) takes one operand per field atom and
  // ceil(n / 31) operands per bytes atom, on both crates, and returns
  // Error::Synthesis ("cannot decode field element from no data", "cannot
  // decode bytes from to little data") when the operand list is shorter;
  // off-circuit `parse_field_repr` fails on the same programs for every
  // preimage. A compress atom and an option segment are rejected before any
  // count matters (ZKIR-STATIC #onlyAtoms, #hasCompress), so they count 0
  // here; surplus operands are accepted on both sides.
  syntax Int ::= #alignFields(Segments) [function, total]
  rule #alignFields(.Segments) => 0
  rule #alignFields((atom(fieldAtom()), Segs)) => 1 +Int #alignFields(Segs)
  rule #alignFields((atom(bytesAtom(N)), Segs)) => (N +Int 30) /Int 31 +Int #alignFields(Segs)
  rule #alignFields((atom(compressAtom()), Segs)) => #alignFields(Segs)
  rule #alignFields((option(_), Segs)) => #alignFields(Segs)
endmodule


// Static well-formedness, checked before execution. The checks are those that
// hold for every run regardless of inputs:
//   1. minor version is 0 (`IrSource::load` accepts 3.0 only);
//   2. no identifier is defined twice: not among the inputs, and not by an
//      instruction once defined (single assignment, plan decision CLM-0711;
//      the Rust preprocess does not enforce this and silently overwrites);
//   3. every variable operand is defined by an earlier input or instruction
//      (the Rust preprocess reports "variable not found" at run time);
//   4. immediates lie in [0, r);
//   5. arities that the Rust code checks structurally: `output` supplies
//      exactly as many values as the signature declares, `div_mod_power_of_two`
//      names exactly two outputs, `encode` names exactly encodedLen(type)
//      outputs (checked at run time, since the type is dynamic; not here);
//   6. bit widths: constrain_bits and less_than below FR_BITS = 255;
//      div_mod_power_of_two and reconstitute_field at most
//      FR_BYTES_STORED * 8 = 248 (the Rust bounds, `ir_vm.rs`);
//      reconstitute_field at least 1 (bits 0 makes the divisor bound 255,
//      which `preprocess` rejects as an excessive bit bound on every value and
//      keygen rejects by an assertion, plan-iter3 candidate K8);
//   7. the byte-hash operand lists are at least as long as their alignment
//      needs (#alignFields): `parse_field_repr` rejects a shorter list for
//      every preimage and `fab_decode_to_bytes` at synthesis.
// Everything else (types of operands, guard values, transcript lengths) is
// dynamic and belongs to ZKIR-VM.
module ZKIR-WF
  imports ZKIR-SYNTAX
  imports SET
  imports K-EQUAL

  syntax WfResult ::= wfOk()            [symbol(wfOk)]
                    | wfError(String)   [symbol(wfError)]

  syntax WfResult ::= wf(Program) [function, total]
  rule wf(program(Minor, _, _, _, _)) => wfError("unsupported minor version " +String Int2String(Minor))
    requires Minor =/=Int 0
  rule wf(program(0, Ins, Outs, _, Is)) => #wfInputs(Ins, .Set, Is, lenIrTypes(Outs))

  // Inputs: distinct identifiers, then walk the instructions.
  syntax WfResult ::= #wfInputs(TypedIds, Set, Instrs, Int) [function, total]
  rule #wfInputs(.TypedIds, Defined, Is, N) => #wfInstrs(Is, Defined, N)
  rule #wfInputs((typedId(X, _), _), Defined, _, _) => wfError("duplicate input " +String X)
    requires X in Defined
  rule #wfInputs((typedId(X, _), Rest), Defined, Is, N) => #wfInputs(Rest, Defined |Set SetItem(X), Is, N)
    requires notBool (X in Defined)

  syntax WfResult ::= #wfInstrs(Instrs, Set, Int) [function, total]
  rule #wfInstrs(.Instrs, _, _) => wfOk()
  rule #wfInstrs((I ; Is), Defined, N) => #wfAfterReads(#checkReads(reads(I), Defined), I, Is, Defined, N)

  syntax WfResult ::= #wfAfterReads(WfResult, Instr, Instrs, Set, Int) [function, total]
  rule #wfAfterReads(wfError(S), _, _, _, _) => wfError(S)
  rule #wfAfterReads(wfOk(), I, Is, Defined, N) => #wfAfterArity(#checkArity(I, N), I, Is, Defined, N)

  syntax WfResult ::= #wfAfterArity(WfResult, Instr, Instrs, Set, Int) [function, total]
  rule #wfAfterArity(wfError(S), _, _, _, _) => wfError(S)
  rule #wfAfterArity(wfOk(), I, Is, Defined, N) => #wfWrites(writes(I), Is, Defined, N)

  syntax WfResult ::= #wfWrites(Ids, Instrs, Set, Int) [function, total]
  rule #wfWrites(.Ids, Is, Defined, N) => #wfInstrs(Is, Defined, N)
  rule #wfWrites((X, _), _, Defined, _) => wfError("reassignment of " +String X)
    requires X in Defined
  rule #wfWrites((X, Xs), Is, Defined, N) => #wfWrites(Xs, Is, Defined |Set SetItem(X), N)
    requires notBool (X in Defined)

  syntax WfResult ::= #checkReads(Operands, Set) [function, total]
  rule #checkReads(.Operands, _) => wfOk()
  rule #checkReads((imm(V), Os), Defined) => #checkReads(Os, Defined)
    requires V >=Int 0 andBool V <Int #blsScalarModulus
  rule #checkReads((imm(V), _), _) => wfError("immediate out of field range: " +String Int2String(V))
    requires V <Int 0 orBool V >=Int #blsScalarModulus
  rule #checkReads((var(X), Os), Defined) => #checkReads(Os, Defined)
    requires X in Defined
  rule #checkReads((var(X), _), Defined) => wfError("undefined variable " +String X)
    requires notBool (X in Defined)

  syntax WfResult ::= #checkArity(Instr, Int) [function, total]
  rule #checkArity(output(Vs), N) => wfError("output: signature declares " +String Int2String(N) +String " return values but instruction has " +String Int2String(lenOperands(Vs)))
    requires lenOperands(Vs) =/=Int N
  rule #checkArity(divModPowerOfTwo(_, _, Os), _) => wfError("div_mod_power_of_two requires exactly 2 outputs")
    requires lenIds(Os) =/=Int 2
  rule #checkArity(divModPowerOfTwo(_, B, Os), _) => wfError("div_mod_power_of_two: excessive bit count")
    requires lenIds(Os) ==Int 2 andBool B >Int #frBytesStored *Int 8
  rule #checkArity(reconstituteField(_, _, B, _), _) => wfError("reconstitute_field: excessive bit count")
    requires B >Int #frBytesStored *Int 8
  rule #checkArity(reconstituteField(_, _, 0, _), _) => wfError("reconstitute_field: bits 0 makes the divisor bound 255, an excessive bit bound")
  rule #checkArity(persistentHash(alignment(Segs), Xs, _), _) => wfError(#alignArity("persistent_hash", Segs, Xs))
    requires lenOperands(Xs) <Int #alignFields(Segs)
  rule #checkArity(keccak256(alignment(Segs), Xs, _), _) => wfError(#alignArity("keccak256", Segs, Xs))
    requires lenOperands(Xs) <Int #alignFields(Segs)
  rule #checkArity(constrainBits(_, B), _) => wfError("constrain_bits: excessive bit bound")
    requires B >=Int #frBits
  rule #checkArity(lessThan(_, _, B, _), _) => wfError("less_than: excessive bit bound")
    requires B >=Int #frBits
  rule #checkArity(_, _) => wfOk() [owise]
  syntax String ::= #alignArity(String, Segments, Operands) [function, total]
  rule #alignArity(Op, Segs, Xs) => Op +String ": alignment needs " +String Int2String(#alignFields(Segs)) +String " field elements but instruction has " +String Int2String(lenOperands(Xs))
endmodule
