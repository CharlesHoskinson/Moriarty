// Single local compiler/checker model. No messages, concurrency or crypto.
// Native field widths are authoritative premises. Claim/duty records below use
// abstract width 1; event/header widths use independently specified SDK literals.
// This model checks activity/consumption/merge and source guards, not ABI bytes.
module ledger_call_relation {
  type Group = { active: bool, width: int }
  type Request = { amounts: List[int], dutyPresent: bool, dutyAmount: int,
                   events: List[int], boolEvent: bool }
  type State = { request: Request, raw: List[int], skips: List[int],
                 processed: List[Group], accepted: bool, checked: bool }
  var state: State
  pure val emptyRequest: Request = { amounts: List(), dutyPresent: false,
    dutyAmount: 0, events: List(0), boolEvent: false }
  action init = state' = {request: emptyRequest, raw: List(), skips: List(),
                          processed: List(), accepted: false, checked: false}
  pure val maxUInt = 340282366920938463463374607431768211455
  pure val eventUpper = List(32,256,65536,16777216,4294967296,1099511627776,281474976710656,72057594037927936,18446744073709551616,4722366482869645213696,1208925819614629174706176,309485009821345068724781056,79228162514264337593543950336,20282409603651670423947251286016,5192296858534827628530496329220096,1329227995784915872903807060280344576,340282366920938463463374607431768211456)
  pure def op(active: bool, width: int): Group = {active: active, width: width}
  pure def branch(value: int, i: int): bool =
    value < eventUpper.nth(i) and (i == 0 or value >= eventUpper.nth(i - 1))
  pure def selected(value: int, i: int): int = if (branch(value, i)) value else 0
  pure def eventGroups(value: int, isBool: bool): List[Group] =
    if (isBool) List(op(true, 14), op(true, 1)) else
      List(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16).foldl(List(), (acc, i) =>
        acc.concat(List(op(branch(value, i), 14), op(branch(value, i), 1))))
  // A claim's first six and final four records require a positive source amount;
  // the three old-value lookup records also require a previous positive prefix.
  // Native claim field widths are abstract here; Rust checks actual SDK widths.
  pure def claimGroups(amounts: List[int]): List[Group] = {
    val result = amounts.foldl({total: 0, groups: List()}, (acc, amount) => {
      val positive = amount > 0
      val old = positive and acc.total > 0
      val groups = List(op(positive,1),op(positive,1),op(positive,1),op(positive,1),op(positive,1),op(positive,1),
        op(old,1),op(old,1),op(old,1),op(positive,1),op(positive,1),op(positive,1),op(positive,1))
      {total: acc.total + amount, groups: acc.groups.concat(groups)}
    })
    result.groups
  }
  // Membership records always execute; absent duties skip old-identity/value
  // reads. Amount range/identity constraints operate on selected-zero openings.
  pure def dutyGroups(present: bool): List[Group] = List(op(true,1),op(true,1),
    op(true,1),op(true,1),op(true,1),op(present,1),op(present,1),op(present,1))
  pure def skeleton(r: Request): List[Group] =
    List(op(true,1),op(true,1),op(true,4)).concat(claimGroups(r.amounts))
      .concat(dutyGroups(r.dutyPresent)).concat(r.events.foldl(List(),
        (acc,value) => acc.concat(eventGroups(value,r.boolEvent))))
  pure def activeWidths(groups: List[Group]): List[int] = groups.select(g => g.active).foldl(List(),(acc,g) => acc.append(g.width))
  pure def skipWidths(groups: List[Group]): List[int] = groups.foldl(List(),(acc,g) => acc.append(if(g.active) 0 else g.width))
  pure def fieldCount(groups: List[Group]): int = groups.foldl(0,(sum,g) => sum + g.width)
  pure def merged(groups: List[Group]): List[Group] = groups.foldl(List(),(acc,g) =>
    if (acc.length() == 0 or g.active) acc.append(g) else {
      val last = acc.nth(acc.length()-1)
      if (last.active) acc.append(g) else
        acc.replaceAt(acc.length()-1,op(false,last.width + g.width))
    })
  pure def domain(r: Request): bool = and {
    r.amounts.indices().forall(i => r.amounts.nth(i) >= 0),
    r.amounts.foldl(0,(sum,a) => sum + a) <= maxUInt,
    r.dutyAmount >= 0 and r.dutyAmount <= maxUInt,
    r.events.indices().forall(i => r.events.nth(i) >= 0 and
      r.events.nth(i) <= (if (r.boolEvent) 1 else maxUInt)),
  }
  pure def canProcess(r: Request, raw: List[int], skips: List[int]): bool =
    domain(r) and raw == activeWidths(skeleton(r)) and skips == skipWidths(skeleton(r))
  action inspect(r: Request, raw: List[int], skips: List[int]): bool = {
    val ok = canProcess(r,raw,skips)
    state' = {request: r,raw: raw,skips: skips,processed: if(ok) merged(skeleton(r)) else List(),
              accepted: ok,checked: true}
  }
  action inspectHonest(r: Request): bool = inspect(r,activeWidths(skeleton(r)),skipWidths(skeleton(r)))
  action step = {
    nondet amounts = Set(List(0),List(2),List(0,2,1),List(2,0,1),List(maxUInt),List(maxUInt,1)).oneOf()
    nondet present = Set(false,true).oneOf()
    nondet amount = Set(0,1,maxUInt).oneOf()
    nondet events = Set(List(0),List(1),List(31),List(32),List(255),List(256),List(65535),List(65536),
      List(18446744073709551616),List(maxUInt),List(0,maxUInt)).oneOf()
    nondet fault = Set(0,1,2,3,4).oneOf()
    val r = {amounts: amounts,dutyPresent: present,dutyAmount: amount,events: events,boolEvent: false}
    val raw = activeWidths(skeleton(r))
    val skips = skipWidths(skeleton(r))
    inspect(r,if(fault == 1) raw.tail() else if(fault == 2) raw.append(1) else raw,
      if(fault == 3) skips.tail() else if(fault == 4) skips.replaceAt(0,2) else skips)
  }
  val relation_projection_invariants = state.accepted implies and {
    canProcess(state.request,state.raw,state.skips),
    state.raw.length() == skeleton(state.request).select(g => g.active).length(),
    fieldCount(state.processed) == fieldCount(skeleton(state.request)),
    state.processed.indices().forall(i => i == 0 or state.processed.nth(i).active or state.processed.nth(i-1).active),
    state.request.events.indices().forall(j => state.request.boolEvent or and {
      0.to(16).filter(i => branch(state.request.events.nth(j),i)).size() == 1,
      0.to(16).forall(i => selected(state.request.events.nth(j),i) < eventUpper.nth(i)),
    }),
    (if (state.request.dutyPresent) state.request.dutyAmount else 0) <= maxUInt,
  }
  val refusedWitness = state.checked and not(state.accepted)
  val positiveClaimWitness = state.accepted and state.request.amounts.indices().exists(i => state.request.amounts.nth(i) == 2)
  val zeroClaimWitness = state.accepted and state.request.amounts.indices().exists(i => state.request.amounts.nth(i) == 0)
  val presentDutyWitness = state.accepted and state.request.dutyPresent
  val absentDutyWitness = state.accepted and not(state.request.dutyPresent)
  val longEventWitness = state.accepted and state.request.events.indices().exists(i => state.request.events.nth(i) == maxUInt)
}
