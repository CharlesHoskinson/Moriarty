// The target contract, tier one (plan-iter3 M5a, completed in M5b): the
// obligations a compiler must meet for its ZKIR output to be well formed and
// provable, decided from the program alone. targetContract(P) is a total
// function over Program that returns one obligation per line of the contract,
// obligation(name, stage, status), the status met(), failed(msg),
// notApplicable() (no instruction of that kind occurs) or info(fact) (a fact
// the later tiers quote rather than a check).
//
// The stage names the crate function that rejects a program failing the
// obligation, witness-independently:
//   keygen      `keygen_vk` on `MidnightCircuit` (unknown-witness synthesis)
//               rejects it, by a panic or an Error::Synthesis; `preprocess`
//               accepts it on some or every preimage
//   preprocess  `IrSource::load` or `IrSource::preprocess` rejects it on every
//               preimage; keygen keys it
//   both        keygen and preprocess reject it
//   static      the K static layer ZKIR-WF, which is strictly stricter than
//               the crate: its single-assignment check is K-only (CLM-0711),
//               the other checks are load or preprocess checks
//   none        an info fact, nothing to reject
//
//   wf                         static     the static check of ZKIR-WF
//   version                    none       the version pin, 3.<minor> (wf and
//                              the preprocessor reject another major or minor)
//   surface                    preprocess base (midnight-ledger 92e8bdd3),
//                              extension (midnight-zkir 2ffe2d1) or common;
//                              failed when reverse_bytes meets an extension
//                              type or instruction (no crate loads it)
//   chips.enabled              none       the chip set `IrSource::used_chips`
//                              initialises
//   chips.gating               keygen     no instruction needs a chip that set
//                              omits (finding 13, K4, K7: the accessor panics)
//   width.less_than            keygen     padded bound at most
//                              MAX_BOUND_IN_BITS = 253 (`bounded_of_element`
//                              asserts; `preprocess` accepts 253 and 254, K5)
//   width.constrain_bits       keygen     width at most F::NUM_BITS = 255
//                              (`assigned_to_le_bits` asserts); unreachable
//                              behind wf, which rejects width 255 and above as
//                              `preprocess` does
//   width.div_mod_power_of_two preprocess bits at most FR_BYTES_STORED * 8 =
//                              248 ("Excessive bit count", ir_vm.rs); keygen
//                              keys 249 (case e21a)
//   width.reconstitute_field   preprocess bits at most 248 (the same); keygen
//                              keys 249 to 254 (case e21b)
//   width.reconstitute_field.assertion both  bits in [1, 254]: bits 0 makes
//                              the divisor bound FR_BITS - bits = 255, bits
//                              255 makes the modulus bound 255 and bits above
//                              255 wrap the divisor bound; `preprocess`
//                              rejects every such program on every preimage
//                              (an excessive bit bound at 0, "Excessive bit
//                              count" above 248) and `assign_less_than_pow2`
//                              (midnight-circuits field/decomposition/chip.rs)
//                              rejects it at keygen by
//                              `assert!(bit_length < F::NUM_BITS)` (K8 at 0,
//                              case f10 at 256)
//   alignment.persistent_hash  keygen     no option segment, no compress atom,
//                              and at least #alignFields operands
//                              (`fab_decode_to_bytes_atom`; the compress and
//                              operand-count failures are preprocess failures
//                              as well, the option failure is not)
//   alignment.keccak256        keygen     the same
//   alignment.sha512           keygen     the same (extension surface only)
//   circuit.static.defined     both       every operand the circuit resolves
//                              names a declared input or an earlier output
//                              (`resolve_operand` in `Relation::circuit`:
//                              "value Identifier(..) not in memory";
//                              `preprocess`: "variable not found"); the guard
//                              of public_input / private_input is not read in
//                              circuit and is left to wf
//   circuit.static.output      both       every `output` operand whose static
//                              type is known has the declared output type,
//                              and the arity matches the signature
//   circuit.static.div_mod_outputs both   div_mod_power_of_two names exactly
//                              two outputs ("Unexpected output length of
//                              DivModPowerOfTwo instruction" in circuit,
//                              "DivModPowerOfTwo requires exactly 2 outputs"
//                              in `preprocess`)
//   circuit.static.bool_gate   both       and / or / xor have at least one
//                              input, every input of known type a Bool
//                              (extension surface)
//   circuit.static.constant    both       every load_constant encoding decodes
//                              on the strict decoder (extension surface)
//   circuit.static.bytes_bounds both      nth and slice stay within the static
//                              length of their byte string, slice length at
//                              least 1 (extension surface)
//   circuit.static.dispatch    keygen     no instruction's in-circuit dispatch
//                              is unsupported for the static types of its
//                              operands (the synthErr arms of
//                              ZKIR-CONSTRAINTS that do not read a value);
//                              `preprocess` accepts some of these pairs
//   commitment                 none       the value of
//                              do_communications_commitment: the preimage-side
//                              half of the rule is tier two
//   inventory.instructions     none       instruction count by op
//   inventory.types            none       the types the program declares
//
// The chip, width, alignment and circuit.static obligations are the
// program-only half of the synthErr outcomes of ZKIR-CONSTRAINTS: the gates
// evaluate them on a witness, this module on the static type environment of
// ZKIR-STATIC (bindOuts, stype). A program with every keygen-stage and
// both-stage obligation met or not applicable keys; a program with a failed
// preprocess-stage or both-stage obligation is rejected on every preimage; a
// failed keygen-stage or both-stage obligation names the instruction whose
// synthesis fails (tools/provability.py keys every parseable program and
// checks the stage against the keygen outcome). A check that needs a value
// (an unknown static type, a bound on a witness value) stays with the gates.
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
  syntax Stage ::= keygen()     [symbol(keygen)]
                 | preprocess() [symbol(preprocess)]
                 | both()       [symbol(both)]
                 | static()     [symbol(static)]
                 | none()       [symbol(none)]
  syntax Obligation  ::= obligation(String, Stage, ObligationStatus) [symbol(obligation)]
  syntax Obligations ::= List{Obligation, ","} [symbol(obligations), terminator-symbol(.obligations)]

  syntax Obligations ::= targetContract(Program) [function, total]
  rule targetContract(program(Mi, Ins, Outs, DoComm, Is) #as P) =>
      obligation("wf", static(), #wfStatus(wf(P))),
      obligation("version", none(), info("3." +String Int2String(Mi))),
      obligation("surface", preprocess(), #surface(P)),
      obligation("chips.enabled", none(), info(#chipsOrNone(usedChips(P)))),
      obligation("chips.gating", keygen(), #chipGating(Is, inputEnv(Ins), usedChips(P), 0)),
      obligation("width.less_than", keygen(), #walk(ltWidth(), Is, inputEnv(Ins), 0, false)),
      obligation("width.constrain_bits", keygen(), #walk(cbWidth(), Is, inputEnv(Ins), 0, false)),
      obligation("width.div_mod_power_of_two", preprocess(), #walk(dmWidth(), Is, inputEnv(Ins), 0, false)),
      obligation("width.reconstitute_field", preprocess(), #walk(rfWidth(), Is, inputEnv(Ins), 0, false)),
      obligation("width.reconstitute_field.assertion", both(), #walk(rfAssert(), Is, inputEnv(Ins), 0, false)),
      obligation("alignment.persistent_hash", keygen(), #walk(align("persistent_hash"), Is, inputEnv(Ins), 0, false)),
      obligation("alignment.keccak256", keygen(), #walk(align("keccak256"), Is, inputEnv(Ins), 0, false)),
      obligation("alignment.sha512", keygen(), #walk(align("sha512"), Is, inputEnv(Ins), 0, false)),
      obligation("circuit.static.defined", both(), #defined(Is, #inputSet(Ins), 0)),
      obligation("circuit.static.output", both(), #walk(outputSig(Outs), Is, inputEnv(Ins), 0, false)),
      obligation("circuit.static.div_mod_outputs", both(), #walk(dmOutputs(), Is, inputEnv(Ins), 0, false)),
      obligation("circuit.static.bool_gate", both(), #walk(boolGate(), Is, inputEnv(Ins), 0, false)),
      obligation("circuit.static.constant", both(), #walk(constant(), Is, inputEnv(Ins), 0, false)),
      obligation("circuit.static.bytes_bounds", both(), #walk(bytesBounds(), Is, inputEnv(Ins), 0, false)),
      obligation("circuit.static.dispatch", keygen(), #walk(dispatch(), Is, inputEnv(Ins), 0, false)),
      obligation("commitment", none(), info(#commInfo(DoComm))),
      obligation("inventory.instructions", none(), info(#instrInventory(Is))),
      obligation("inventory.types", none(), info(#typeInventory(P))),
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
  syntax Check ::= ltWidth() | cbWidth() | dmWidth() | rfWidth() | rfAssert() | dmOutputs() | align(String)
                 | outputSig(IrTypes) | boolGate() | constant() | bytesBounds() | dispatch()
  syntax Bool ::= applies(Check, Instr) [function, total]
  rule applies(ltWidth(), lessThan(_, _, _, _))         => true
  rule applies(cbWidth(), constrainBits(_, _))          => true
  rule applies(dmWidth(), divModPowerOfTwo(_, _, _))    => true
  rule applies(rfWidth(), reconstituteField(_, _, _, _))=> true
  rule applies(rfAssert(), reconstituteField(_, _, _, _)) => true
  rule applies(dmOutputs(), divModPowerOfTwo(_, _, _))  => true
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
  // `assert_lower_than_fixed(divisor, 1 << (FR_BITS - bits))` and
  // `assert_lower_than_fixed(modulus, 1 << bits)` (ir_vm.rs) reach
  // assign_less_than_pow2 (midnight-circuits field/decomposition/chip.rs),
  // whose `assert!((bit_length as u32) < F::NUM_BITS)` fails at keygen when a
  // bound is 255 bits or more: bits 0 (the divisor bound), bits 255 (the
  // modulus bound) and bits above 255 (FR_BITS - bits wraps). `preprocess`
  // rejects the same programs on every preimage; 249 to 254 key (case e21b)
  rule defect(rfAssert(), reconstituteField(_, _, N, _), _) => "reconstitute_field: bits " +String Int2String(N) +String ": assertion failed: (bit_length as u32) < F::NUM_BITS"
    requires N ==Int 0 orBool N >=Int #frBits
  // the `I::DivModPowerOfTwo` arm of `Relation::circuit` checks the output
  // count before resolving the operand; `preprocess` checks it as well
  rule defect(dmOutputs(), divModPowerOfTwo(_, _, Os), _) => "div_mod_power_of_two: Unexpected output length of DivModPowerOfTwo instruction"
    requires lenIds(Os) =/=Int 2
  rule defect(align(Op), I, _) => #alignDefect(Op, alignmentOf(I), reads(I)) requires #opName(I) ==String Op
  rule defect(outputSig(Ts), output(Vs), E) => #outputDefect(Vs, Ts, E)
  rule defect(dispatch(), I, E) => dispatchDefect(I, E)
  rule defect(_, _, _) => "" [owise]
  // options and compress atoms first, as `fab_decode_to_bytes` rejects them;
  // then the operand count against #alignFields (`fab_decode_to_bytes_atom`,
  // ir_vm.rs: the first atom that runs out gives the Error::Synthesis text)
  syntax String ::= #alignDefect(String, AlignOpt, Operands) [function, total]
  rule #alignDefect(Op, someAlign(alignment(Segs)), _) => Op +String ": alignment has an option segment, which the in-circuit decoder does not implement"
    requires #onlyAtoms(Segs) =/=K Segs
  rule #alignDefect(Op, someAlign(alignment(Segs)), _) => Op +String ": alignment has a compress atom, which cannot be decoded from field elements"
    requires #onlyAtoms(Segs) ==K Segs andBool #hasCompress(Segs)
  rule #alignDefect(Op, someAlign(alignment(Segs)), Xs) => Op +String ": " +String #alignShort(Segs, lenOperands(Xs))
    requires #onlyAtoms(Segs) ==K Segs andBool notBool #hasCompress(Segs) andBool lenOperands(Xs) <Int #alignFields(Segs)
  rule #alignDefect(_, _, _) => "" [owise]

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

  // --- circuit.static.defined (`resolve_operand` in ir_vm.rs `circuit`) ------------------------------
  // The circuit resolves every operand of an instruction from its memory
  // before doing anything else with it, and an identifier that no declared
  // input or earlier output bound is `Error::Synthesis("value {id:?} not in
  // memory")`, witness-independently. The guard of public_input and
  // private_input is the exception: the `PublicInput | PrivateInput` arm
  // matches `guard: _` (finding 3), so an undefined guard keys and is left to
  // wf and to `preprocess` ("variable not found"). The walk carries the set
  // of bound identifiers as wf does; the first undefined operand in program
  // order names the instruction.
  syntax Set ::= #inputSet(TypedIds) [function, total]
  rule #inputSet(.TypedIds) => .Set
  rule #inputSet((typedId(X, _), Rest)) => SetItem(X) |Set #inputSet(Rest)
  syntax Set ::= #idSet(Ids) [function, total]
  rule #idSet(.Ids) => .Set
  rule #idSet((X, Xs)) => SetItem(X) |Set #idSet(Xs)
  syntax Operands ::= circuitReads(Instr) [function, total]
  rule circuitReads(publicInput(_, _, _))  => .Operands
  rule circuitReads(privateInput(_, _, _)) => .Operands
  rule circuitReads(I) => reads(I) [owise]
  // the first variable operand outside the bound set, or ""
  syntax String ::= #undefinedRead(Operands, Set) [function, total]
  rule #undefinedRead(.Operands, _) => ""
  rule #undefinedRead((imm(_), Os), D) => #undefinedRead(Os, D)
  rule #undefinedRead((var(X), Os), D) => #undefinedRead(Os, D) requires X in D
  rule #undefinedRead((var(X), _), D)  => X requires notBool (X in D)
  syntax ObligationStatus ::= #defined(Instrs, Set, Int) [function, total]
  rule #defined(.Instrs, _, 0)   => notApplicable()
  rule #defined(.Instrs, _, Idx) => met() requires Idx >Int 0
  rule #defined((I ; Is), D, Idx) => #definedStep(#undefinedRead(circuitReads(I), D), I, Is, D, Idx)
  syntax ObligationStatus ::= #definedStep(String, Instr, Instrs, Set, Int) [function, total]
  rule #definedStep("", I, Is, D, Idx) => #defined(Is, D |Set #idSet(writes(I)), Idx +Int 1)
  rule #definedStep(X, _, _, _, Idx) => failed("instruction " +String Int2String(Idx) +String ": value Identifier(\"" +String X +String "\") not in memory")
    requires X =/=String ""

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
