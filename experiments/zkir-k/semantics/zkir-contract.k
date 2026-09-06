// The target contract, tier one (plan-iter3 M5a): the obligations a compiler
// must meet for its ZKIR output to be well formed and provable, decided from
// the program alone. targetContract(P) is a total function over Program that
// returns one obligation per line of the contract, each met(), failed(msg),
// notApplicable() (no instruction of that kind occurs) or info(fact) (a fact
// the later tiers quote rather than a check).
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
//   commitment                 the value of do_communications_commitment: the
//                              preimage-side half of the rule is tier two
//   inventory.instructions     instruction count by op
//   inventory.types            the types the program declares
//
// The chip and width obligations are the program-only half of the synthErr
// outcomes of ZKIR-CONSTRAINTS: the gates evaluate them on a witness, this
// module on the static types of ZKIR-STATIC. Tiers two (the off-circuit run
// succeeds) and three (the circuit accepts) need a preimage and are M5b.
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
  rule targetContract(program(Mi, Ins, _, DoComm, Is) #as P) =>
      obligation("wf", #wfStatus(wf(P))),
      obligation("version", info("3." +String Int2String(Mi))),
      obligation("surface", #surface(P)),
      obligation("chips.enabled", info(#chipsOrNone(usedChips(P)))),
      obligation("chips.gating", #chipGating(Is, inputEnv(Ins), usedChips(P), 0)),
      obligation("width.less_than", #walk(ltWidth(), Is, 0, false)),
      obligation("width.constrain_bits", #walk(cbWidth(), Is, 0, false)),
      obligation("width.div_mod_power_of_two", #walk(dmWidth(), Is, 0, false)),
      obligation("width.reconstitute_field", #walk(rfWidth(), Is, 0, false)),
      obligation("alignment.persistent_hash", #walk(align("persistent_hash"), Is, 0, false)),
      obligation("alignment.keccak256", #walk(align("keccak256"), Is, 0, false)),
      obligation("alignment.sha512", #walk(align("sha512"), Is, 0, false)),
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

  // --- widths and alignments: one walk per obligation -------------------------------------------------
  // applies(C, I) marks the instructions the obligation is about (none: notApplicable);
  // defect(C, I) is "" when I is within the limit and the failure text otherwise
  syntax Check ::= ltWidth() | cbWidth() | dmWidth() | rfWidth() | align(String)
  syntax Bool ::= applies(Check, Instr) [function, total]
  rule applies(ltWidth(), lessThan(_, _, _, _))         => true
  rule applies(cbWidth(), constrainBits(_, _))          => true
  rule applies(dmWidth(), divModPowerOfTwo(_, _, _))    => true
  rule applies(rfWidth(), reconstituteField(_, _, _, _))=> true
  rule applies(align(Op), I)                            => #opName(I) ==String Op
  rule applies(_, _) => false [owise]
  syntax String ::= defect(Check, Instr) [function, total]
  rule defect(ltWidth(), lessThan(_, _, N, _)) => "less_than: bits " +String Int2String(N) +String " pads to " +String Int2String(#ltBits(N)) +String ", above MAX_BOUND_IN_BITS = 253"
    requires #ltBits(N) >Int 253
  rule defect(cbWidth(), constrainBits(_, N)) => "constrain_bits: width " +String Int2String(N) +String " exceeds 255"
    requires N >Int 255
  rule defect(dmWidth(), divModPowerOfTwo(_, N, _)) => "div_mod_power_of_two: bits " +String Int2String(N) +String " exceeds FR_BYTES_STORED * 8 = 248"
    requires N >Int #frBytesStored *Int 8
  rule defect(rfWidth(), reconstituteField(_, _, N, _)) => "reconstitute_field: bits " +String Int2String(N) +String " exceeds FR_BYTES_STORED * 8 = 248"
    requires N >Int #frBytesStored *Int 8
  rule defect(align(Op), I) => #alignDefect(Op, alignmentOf(I)) requires #opName(I) ==String Op
  rule defect(_, _) => "" [owise]
  syntax String ::= #alignDefect(String, AlignOpt) [function, total]
  rule #alignDefect(Op, someAlign(alignment(Segs))) => Op +String ": alignment has an option segment, which the in-circuit decoder does not implement"
    requires #onlyAtoms(Segs) =/=K Segs
  rule #alignDefect(Op, someAlign(alignment(Segs))) => Op +String ": alignment has a compress atom, which cannot be decoded from field elements"
    requires #onlyAtoms(Segs) ==K Segs andBool #hasCompress(Segs)
  rule #alignDefect(_, _) => "" [owise]

  syntax ObligationStatus ::= #walk(Check, Instrs, Int, Bool) [function, total]
  rule #walk(_, .Instrs, _, false) => notApplicable()
  rule #walk(_, .Instrs, _, true)  => met()
  rule #walk(C, (I ; Is), Idx, Seen) => #walkStep(C, Is, Idx, Seen orBool applies(C, I), defect(C, I))
  syntax ObligationStatus ::= #walkStep(Check, Instrs, Int, Bool, String) [function, total]
  rule #walkStep(C, Is, Idx, Seen, "") => #walk(C, Is, Idx +Int 1, Seen)
  rule #walkStep(_, _, Idx, _, D) => failed("instruction " +String Int2String(Idx) +String ": " +String D) requires D =/=String ""

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
