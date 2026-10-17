requires "domains.md"

module SHERLOCK-B-SCALAR-TAG-SYNTAX
  imports DOMAINS-SYNTAX
  // Logical, exact mathematical integers. No JS Number or packed-field carrier.
  syntax BTUInt128 ::= btUInt128(Int)
  syntax BTValueSpace ::= btConstSpace() | btDataSpace()
  syntax BTValueRef ::= btValueRef(BTValueSpace, Int)
  syntax BTValueTag ::= btUIntTag() | btOtherTag(String)
  // Abstract carriers retain supplied terms exactly; they are never decoded or
  // treated as authenticated physical image cells by this source projection.
  syntax BTOpaque ::= btOpaque(String)
  syntax BTValueRow ::= btValueRow(BTValueTag, BTUInt128, BTOpaque)
  syntax BTValueRows ::= btRowsNil() | btRowsCons(BTValueRow, BTValueRows)
  // capacity, active_rows, rows (each row retains its uninspected rest).
  syntax BTValueTableRead ::= btValueTableRead(Int, Int, BTValueRows)
  syntax BTPhase ::= btFirst() | btSecond()
  syntax BTResume ::= btNoResume() | btResume(Int)
  syntax BTOptValueRef ::= btNoRef() | btSomeRef(BTValueRef)
  syntax BTOptSort ::= btNoSort() | btSomeSort(String)
  syntax BTStaged ::= btStagedNone() | btStagedOpaque(String)
  // origin_reference_cursor, parent_resume, operand_b, expected, staged,
  // further_lanes. These are the complete named remaining helper partition;
  // further_lanes is an abstract tuple, not a selected B register layout.
  syntax BTScalarRest ::= btScalarRest(BTUInt128, BTResume, BTOptValueRef,
                                      BTOptSort, BTStaged, BTOpaque)
  // present, kind, site, mode, phase, operand_a, reg_a, rest.
  syntax BTScalarRegister ::= btScalarRegister(Bool, String, String, String,
                              BTPhase, BTValueRef, BTUInt128, BTScalarRest)
  // remaining_work, consumed_work, helper_work, helper_visits.
  syntax BTChargedCounters ::= btChargedCounters(BTUInt128, BTUInt128,
                                                BTUInt128, BTUInt128)
  syntax BTAlloc ::= btAlloc(Int, Int, Int)
  // active.data is data.active_rows; these are dataFields, env, valueSlots.
  syntax BTActiveRest ::= btActiveRest(Int, Int, Int)
  // active(data,dataFields,env,valueSlots), alloc(dataNodes,dataFields,envCells),
  // consumedWork. The source preserves every supplied component.
  syntax BTSegBase ::= btSegBase(Int, Int, Int, Int, Int, Int, Int, BTUInt128)
  // img.dataFields, img.env, img.valueSlots, img.frames, img.tasks.
  syntax BTOutsideTables ::= btOutsideTables(BTOpaque, BTOpaque, BTOpaque,
                                              BTOpaque, BTOpaque)
  syntax BTOutcome ::= btRunning() | btOutcomeOpaque(String)
  // control, controlOrigin, envRoot, depth, stackLimit, outcome, initialFuel,
  // referenceCursor, alloc, activeRest, taskLive, result, segBase, outsideTables,
  // admission, runId, programRest. All are unchanged, by identical term reuse.
  syntax BTFraming ::= btFraming(BTOpaque, Int, BTOpaque, Int, Int, BTOutcome,
                         BTUInt128, BTUInt128, BTAlloc, BTActiveRest, Int,
                         BTOpaque, BTSegBase, BTOutsideTables,
                         BTOpaque, BTOpaque, BTOpaque)
  // Applied-domain parameter max_helper_work, already required by owner P3.
  syntax BTParams ::= btParams(Int)
  // params, constants, data, helper, counters, framing.
  syntax BTState ::= btState(BTParams, BTValueTableRead, BTValueTableRead,
                             BTScalarRegister, BTChargedCounters, BTFraming)
  syntax BTRequest ::= btRequest(BTState)
  // A normal semantic step result, not a circuit witness/constraint judgment,
  // StepEvent, full B transition certificate, or public toolchain API.
  syntax BTApplied ::= btApplied(String, BTState, BTState)
  syntax KItem ::= BTRequest | BTApplied
endmodule

module SHERLOCK-B-SCALAR-TAG-FUNCTIONS
  imports SHERLOCK-B-SCALAR-TAG-SYNTAX
  imports DOMAINS
  syntax Bool ::= btIsUInt128(Int) [function]
  rule btIsUInt128(N) => N >=Int 0 andBool N <Int (2 ^Int 128)

  // Pure indexed reads have no event/charge of their own. The one modeled
  // inspected value occurrence is the Scalar.tag event, not K reduction count.
  syntax BTValueRow ::= btAt(BTValueRows, Int) [function]
                     | btReadRef(BTValueRef, BTValueTableRead, BTValueTableRead) [function]
  rule btAt(btRowsCons(Row, _), 0) => Row
  rule btAt(btRowsCons(_, Tail), I) => btAt(Tail, I -Int 1) requires I >Int 0
  rule btReadRef(btValueRef(btConstSpace(), I), btValueTableRead(_, Active, Rows), _)
    => btAt(Rows, I) requires I >=Int 0 andBool I <Int Active
  rule btReadRef(btValueRef(btDataSpace(), I), _, btValueTableRead(_, Active, Rows))
    => btAt(Rows, I) requires I >=Int 0 andBool I <Int Active

  syntax Bool ::= btOutcomeRunning(BTFraming) [function]
  rule btOutcomeRunning(btFraming(_, _, _, _, _, btRunning(), _, _, _, _, _, _, _, _, _, _, _)) => true
  rule btOutcomeRunning(_) => false [owise]

  syntax BTApplied ::= btStep(BTState) [function]
                    | btCopySelected(BTState, BTValueRow) [function]
  // Exactly the declared positive RetRight/Add/First/UInt Applied branch.
  // AB, pre-StepInv, dispatch derivation/uniqueness, complete binding, and the
  // surrounding relation are external owner obligations, not new K policies.
  rule btStep(btState(btParams(MaxH), Constants, Data,
          btScalarRegister(true, "Scalar", "RetRight", "Add", btFirst(), A, OldA, Rest),
          btChargedCounters(btUInt128(F), btUInt128(C), btUInt128(H), btUInt128(V)), Frame))
    => btCopySelected(
         btState(btParams(MaxH), Constants, Data,
           btScalarRegister(true, "Scalar", "RetRight", "Add", btFirst(), A, OldA, Rest),
           btChargedCounters(btUInt128(F), btUInt128(C), btUInt128(H), btUInt128(V)), Frame),
         btReadRef(A, Constants, Data))
    requires btOutcomeRunning(Frame)
       andBool F >Int 0 andBool btIsUInt128(F)
       andBool C >=Int 0 andBool C +Int 1 <Int (2 ^Int 128)
       andBool H >=Int 0 andBool H +Int 1 <Int (2 ^Int 128)
       andBool V >=Int 0 andBool V +Int 1 <Int (2 ^Int 128)
       andBool H <Int MaxH

  // /helper_events/1/effects/1/then/0/then in the pinned R4 rows:
  // regA := u128(deref(operandA)); phase := Second.
  // T-HELPER/P4/P7: work -1/+1, helperWork +1, helperVisits +1.
  // Every other named component, including row payloads and helper rest, is
  // structurally identical to the supplied pre component in the result.
  rule btCopySelected(btState(Params, Constants, Data,
          btScalarRegister(Present, Kind, Site, Mode, btFirst(), A, OldA, Rest),
          btChargedCounters(btUInt128(F), btUInt128(C), btUInt128(H), btUInt128(V)), Frame),
          btValueRow(btUIntTag(), btUInt128(N), RowRest))
    => btApplied("b.helper.Scalar.tag",
         btState(Params, Constants, Data,
           btScalarRegister(Present, Kind, Site, Mode, btFirst(), A, OldA, Rest),
           btChargedCounters(btUInt128(F), btUInt128(C), btUInt128(H), btUInt128(V)), Frame),
         btState(Params, Constants, Data,
           btScalarRegister(Present, Kind, Site, Mode, btSecond(), A, btUInt128(N), Rest),
           btChargedCounters(btUInt128(F -Int 1), btUInt128(C +Int 1),
                            btUInt128(H +Int 1), btUInt128(V +Int 1)), Frame))
    requires btIsUInt128(N)
endmodule

module SHERLOCK-B-SCALAR-TAG
  imports SHERLOCK-B-SCALAR-TAG-FUNCTIONS
  configuration <k> $PGM:BTRequest </k>
  rule <k> btRequest(Pre) => btStep(Pre) </k>
endmodule
