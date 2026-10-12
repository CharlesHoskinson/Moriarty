module ledger_call_relation_test {
  import ledger_call_relation.* from "./ledger_call_relation"
  // These complete vectors are specified independently of skeleton(): three
  // header Ops, five membership Ops, three skipped old reads, one Bool event.
  pure val r: Request = {amounts: List(),dutyPresent: false,dutyAmount: 0,events: List(0),boolEvent: true}
  pure val raw = List(1,1,4,1,1,1,1,1,14,1)
  pure val skips = List(0,0,0,0,0,0,0,0,1,1,1,0,0)
  run literalPositiveTest = init.then(inspect(r,raw,skips)).expect(state.accepted and state.checked)
  run missingRawTest = init.then(inspect(r,List(1,1,4,1,1,1,1,1,14),skips)).expect(state.checked and not(state.accepted))
  run extraRawTest = init.then(inspect(r,List(1,1,4,1,1,1,1,1,14,1,1),skips)).expect(state.checked and not(state.accepted))
  run reorderedRawTest = init.then(inspect(r,List(4,1,1,1,1,1,1,1,14,1),skips)).expect(state.checked and not(state.accepted))
  run wrongRawWidthTest = init.then(inspect(r,List(1,1,4,1,1,1,1,1,13,1),skips)).expect(state.checked and not(state.accepted))
  run zeroInactiveSkipTest = init.then(inspect(r,raw,List(0,0,0,0,0,0,0,0,0,1,1,0,0))).expect(state.checked and not(state.accepted))
  run wrongInactiveSkipTest = init.then(inspect(r,raw,List(0,0,0,0,0,0,0,0,2,1,1,0,0))).expect(state.checked and not(state.accepted))
  run missingActivityTest = init.then(inspect(r,raw,List(0,0,0,0,0,0,0,0,1,1,1,0))).expect(state.checked and not(state.accepted))
  run extraActivityTest = init.then(inspect(r,raw,List(0,0,0,0,0,0,0,0,1,1,1,0,0,0))).expect(state.checked and not(state.accepted))
  run boolBoundaryTest = init.then(inspectHonest({...r,events: List(0,1)})).expect(state.accepted)
    .then(inspectHonest({...r,events: List(2)})).expect(state.checked and not(state.accepted))
  run claimOverflowTest = init.then(inspectHonest({...r,amounts: List(maxUInt)})).expect(state.accepted)
    .then(inspectHonest({...r,amounts: List(maxUInt,1)})).expect(state.checked and not(state.accepted))
  run sourceClaimPrefixTest = init.expect(and {
    claimGroups(List(0,2,1)).length() == 39,
    claimGroups(List(0)).select(g => g.active).length() == 0,
    claimGroups(List(2)).select(g => g.active).length() == 10,
    claimGroups(List(2,1)).select(g => g.active).length() == 23,
    claimGroups(List(2,0,1)).select(g => g.active).length() == 23,
  })
  run dutyPresenceTest = init.then(inspectHonest({...r,dutyPresent: false,dutyAmount: maxUInt})).expect(state.accepted)
    .then(inspectHonest({...r,dutyPresent: true,dutyAmount: 0})).expect(state.accepted)
    .expect(dutyGroups(false).select(g => g.active).length() == 5 and dutyGroups(true).select(g => g.active).length() == 8)
  run eventBoundaryTest = init.expect(and {
    branch(0,0),branch(1,0),branch(31,0),branch(32,1),branch(255,1),branch(256,2),
    branch(65535,2),branch(65536,3),branch(18446744073709551616,9),branch(maxUInt,16),
    selected(maxUInt,0) == 0,selected(maxUInt,16) == maxUInt,
    eventGroups(0,true) == List(op(true,14),op(true,1)),
    eventGroups(1,true) == List(op(true,14),op(true,1)),
  })
  pure val header = List(op(true,1),op(true,1),op(true,4))
  run independentEventNoopCountsTest = init.expect(and {
    header.concat(eventGroups(0,false)).length() == 37,
    fieldCount(header.concat(eventGroups(0,false))) + 2 == 263,
    merged(header.concat(eventGroups(0,false))).length() == 6,
    merged(header.concat(eventGroups(256,false))).length() == 7,
    merged(header.concat(eventGroups(maxUInt,false))).length() == 6,
    merged(header.concat(eventGroups(0,false))).select(g => not(g.active)) == List(op(false,240)),
    merged(header.concat(eventGroups(0,false)).concat(eventGroups(maxUInt,false))).length() == 8,
    merged(header.concat(eventGroups(0,false)).concat(eventGroups(maxUInt,false))).select(g => not(g.active)) == List(op(false,480)),
    fieldCount(header.concat(eventGroups(0,false)).concat(eventGroups(maxUInt,false))) + 2 == 518,
  })
}
