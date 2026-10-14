// Accepted finite FeePool monetary model: 54 source fields, 23 actions, 9 duties.
// Semantics, reference provenance and evidence limits: fee_pool_component.README.md.
// Atomic shared state; opaque fixed slots abstract hash membership. No cryptography.
// Time/auth/import/recipient authority are EXTERNAL premises. Public observedAt
// inputs are numerical thresholds, never authenticated ledger clock evidence.
module fee_pool_component {
  pure val M = 18446744073709551615
  pure val HOLD = 2592000
  pure val DUTIES = Set("cost_ordinary","cost_stipend","reward_operator_a","reward_operator_b","reward_da","treasury_l2","treasury_midnight","treasury_dust","watcher_spend")
  pure val ACTORS = Set("funding-source","fee-pool","operator-a","operator-b","da-provider","watcher","cost-provider","l2-treasury","midnight-treasury")
  type State = {
    epoch: int,
    epochClose: int,
    gross: int,
    metered: int,
    target: int,
    backstop: int,
    watcherHeld: int,
    stipendLimit: int,
    opWeight: int,
    daWeight: int,
    rewardSeatCount: int,
    allocatedCount: int,
    opTotal: int,
    daTotal: int,
    watcherTotal: int,
    opRemaining: int,
    daRemaining: int,
    opAWeight: int,
    opBWeight: int,
    daSeatWeight: int,
    watcherWeight: int,
    ordinaryAmount: int,
    stipendAmount: int,
    opAAmount: int,
    opBAmount: int,
    daAmount: int,
    l2Amount: int,
    midnightAmount: int,
    dustAmount: int,
    spendAmount: int,
    claimedTotal: int,
    carried: int,
    recorded: bool,
    settled: bool,
    finished: bool,
    opARegistered: bool,
    opBRegistered: bool,
    daRegistered: bool,
    watcherRegistered: bool,
    ordinaryRecorded: bool,
    stipendRecorded: bool,
    opAAllocated: bool,
    opBAllocated: bool,
    daAllocated: bool,
    spendIssued: bool,
    ordinaryClaimed: bool,
    stipendClaimed: bool,
    opAClaimed: bool,
    opBClaimed: bool,
    daClaimed: bool,
    l2Claimed: bool,
    midnightClaimed: bool,
    dustClaimed: bool,
    spendClaimed: bool,
  }
  type Duty = {present:bool, amount:int, original:int}
  type FinancialEffect = {kind:str,network:str,issuer:str,reference:str,scale:int,source:str,recipient:str,duty:str,amount:int}
  type World = {history:List[FinancialEffect],s:State, duties:str -> Duty, cash:str -> int, hits:Set[str], claims:str -> int, claimTimes:str -> int, maxTarget:int, watcherCredit:int, watcherSpent:int, sentinelOtherAsset:int}
  var world:World
  pure def u64(n:int):bool = 0 <= n and n <= M
  pure def rangeState(s:State):bool = all {
    u64(s.epoch),
    u64(s.epochClose),
    u64(s.gross),
    u64(s.metered),
    u64(s.target),
    u64(s.backstop),
    u64(s.watcherHeld),
    u64(s.stipendLimit),
    u64(s.opWeight),
    u64(s.daWeight),
    u64(s.rewardSeatCount),
    u64(s.allocatedCount),
    u64(s.opTotal),
    u64(s.daTotal),
    u64(s.watcherTotal),
    u64(s.opRemaining),
    u64(s.daRemaining),
    u64(s.opAWeight),
    u64(s.opBWeight),
    u64(s.daSeatWeight),
    u64(s.watcherWeight),
    u64(s.ordinaryAmount),
    u64(s.stipendAmount),
    u64(s.opAAmount),
    u64(s.opBAmount),
    u64(s.daAmount),
    u64(s.l2Amount),
    u64(s.midnightAmount),
    u64(s.dustAmount),
    u64(s.spendAmount),
    u64(s.claimedTotal),
    u64(s.carried)
  }
  pure val emptyState:State = {
    epoch: 0,
    epochClose: 0,
    gross: 0,
    metered: 0,
    target: 0,
    backstop: 0,
    watcherHeld: 0,
    stipendLimit: 0,
    opWeight: 0,
    daWeight: 0,
    rewardSeatCount: 0,
    allocatedCount: 0,
    opTotal: 0,
    daTotal: 0,
    watcherTotal: 0,
    opRemaining: 0,
    daRemaining: 0,
    opAWeight: 0,
    opBWeight: 0,
    daSeatWeight: 0,
    watcherWeight: 0,
    ordinaryAmount: 0,
    stipendAmount: 0,
    opAAmount: 0,
    opBAmount: 0,
    daAmount: 0,
    l2Amount: 0,
    midnightAmount: 0,
    dustAmount: 0,
    spendAmount: 0,
    claimedTotal: 0,
    carried: 0,
    recorded: false,
    settled: false,
    finished: false,
    opARegistered: false,
    opBRegistered: false,
    daRegistered: false,
    watcherRegistered: false,
    ordinaryRecorded: false,
    stipendRecorded: false,
    opAAllocated: false,
    opBAllocated: false,
    daAllocated: false,
    spendIssued: false,
    ordinaryClaimed: false,
    stipendClaimed: false,
    opAClaimed: false,
    opBClaimed: false,
    daClaimed: false,
    l2Claimed: false,
    midnightClaimed: false,
    dustClaimed: false,
    spendClaimed: false
  }
  pure val emptyWorld:World = {history:List(),s:emptyState, duties:DUTIES.mapBy(_ => {present:false,amount:0,original:0}),cash:ACTORS.mapBy(a => if (a == "funding-source") M else 0),hits:Set(),claims:DUTIES.mapBy(_ => 0),claimTimes:DUTIES.mapBy(_ => 0),maxTarget:0,watcherCredit:0,watcherSpent:0,sentinelOtherAsset:777}
  pure def mark(w:World,s:State,name:str):World = {...w,s:s,hits:w.hits.union(Set(name))}
  pure def fresh(w:World,k:str):bool = not(w.duties.get(k).present)
  pure def effect(kind:str,source:str,recipient:str,duty:str,amount:int):FinancialEffect = {kind:kind,network:"local",issuer:"fee-pool-source",reference:"SERVICE",scale:0,source:source,recipient:recipient,duty:duty,amount:amount}
  pure def beneficiary(k:str):str =
    if (k == "cost_ordinary") "cost-provider" else if (k == "cost_stipend") "watcher" else if (k == "reward_operator_a") "operator-a" else if (k == "reward_operator_b") "operator-b" else if (k == "reward_da") "da-provider" else if (k == "treasury_l2") "l2-treasury" else if (k == "treasury_midnight") "midnight-treasury" else if (k == "treasury_dust") "l2-treasury" else "watcher"
  pure def create(w:World,k:str,n:int):World = {...w,duties:w.duties.set(k,{present:true,amount:n,original:n}),history:w.history.append(effect("Liability","fee-pool",beneficiary(k),k,n))}
  pure def canIntake(w:World,epochId:int,grossAmount:int,closeObservedAt:int,initialTarget:int,watcherStipendLimit:int):bool = all {
    rangeState(w.s),w.s == emptyState,DUTIES.forall(k => fresh(w,k)),u64(epochId),u64(grossAmount),u64(closeObservedAt),u64(initialTarget),u64(watcherStipendLimit),u64(epochId+1),u64((epochId+1)*86400),closeObservedAt >= (epochId+1)*86400,w.cash.get("funding-source") >= grossAmount,
  }
  pure def applyIntake(w:World,epochId:int,grossAmount:int,closeObservedAt:int,initialTarget:int,watcherStipendLimit:int):World = {
    val n = mark(w,{...w.s,epoch:epochId,epochClose:(epochId+1)*86400,gross:grossAmount,target:initialTarget,stipendLimit:watcherStipendLimit,carried:grossAmount,recorded:true},"intake")
    {...n,cash:w.cash.set("funding-source",w.cash.get("funding-source")-grossAmount).set("fee-pool",w.cash.get("fee-pool")+grossAmount),maxTarget:initialTarget,history:w.history.append(effect("Transfer","funding-source","fee-pool","",grossAmount))}
  }
  action init = world' = emptyWorld
  action intake(epochId:int,grossAmount:int,closeObservedAt:int,initialTarget:int,watcherStipendLimit:int):bool = all { canIntake(world,epochId,grossAmount,closeObservedAt,initialTarget,watcherStipendLimit),world'=applyIntake(world,epochId,grossAmount,closeObservedAt,initialTarget,watcherStipendLimit) }
  val wIntake = world.hits.contains("intake")
  val intakeConservation = world.cash.get("funding-source") + world.cash.get("fee-pool") == M

  pure def canRegisterOperatorA(w:World,weight:int,attested:bool,effectiveWeight:int):bool = all {rangeState(w.s),w.s.recorded,not(w.s.settled),not(w.s.opARegistered),u64(weight),u64(effectiveWeight),weight <= 86400, (attested and effectiveWeight==weight) or (not(attested) and effectiveWeight==0),u64(w.s.opWeight+effectiveWeight),u64(w.s.rewardSeatCount+1),}
  pure def applyRegisterOperatorA(w:World,weight:int,attested:bool,effectiveWeight:int):World = mark(w,{...w.s,opAWeight:effectiveWeight,opARegistered:true,opWeight:w.s.opWeight+effectiveWeight,rewardSeatCount:w.s.rewardSeatCount+1},"registerOperatorA")
  action registerOperatorA(weight:int,attested:bool,effectiveWeight:int):bool = all {canRegisterOperatorA(world,weight,attested,effectiveWeight),world'=applyRegisterOperatorA(world,weight,attested,effectiveWeight)}
  pure def canRegisterOperatorB(w:World,weight:int,attested:bool,effectiveWeight:int):bool = all {rangeState(w.s),w.s.recorded,not(w.s.settled),not(w.s.opBRegistered),u64(weight),u64(effectiveWeight),weight <= 86400, (attested and effectiveWeight==weight) or (not(attested) and effectiveWeight==0),u64(w.s.opWeight+effectiveWeight),u64(w.s.rewardSeatCount+1),}
  pure def applyRegisterOperatorB(w:World,weight:int,attested:bool,effectiveWeight:int):World = mark(w,{...w.s,opBWeight:effectiveWeight,opBRegistered:true,opWeight:w.s.opWeight+effectiveWeight,rewardSeatCount:w.s.rewardSeatCount+1},"registerOperatorB")
  action registerOperatorB(weight:int,attested:bool,effectiveWeight:int):bool = all {canRegisterOperatorB(world,weight,attested,effectiveWeight),world'=applyRegisterOperatorB(world,weight,attested,effectiveWeight)}
  pure def canRegisterDa(w:World,weight:int,attested:bool,effectiveWeight:int):bool = all {rangeState(w.s),w.s.recorded,not(w.s.settled),not(w.s.daRegistered),u64(weight),u64(effectiveWeight), (attested and effectiveWeight==weight) or (not(attested) and effectiveWeight==0),u64(w.s.daWeight+effectiveWeight),u64(w.s.rewardSeatCount+1),}
  pure def applyRegisterDa(w:World,weight:int,attested:bool,effectiveWeight:int):World = mark(w,{...w.s,daSeatWeight:effectiveWeight,daRegistered:true,daWeight:w.s.daWeight+effectiveWeight,rewardSeatCount:w.s.rewardSeatCount+1},"registerDa")
  action registerDa(weight:int,attested:bool,effectiveWeight:int):bool = all {canRegisterDa(world,weight,attested,effectiveWeight),world'=applyRegisterDa(world,weight,attested,effectiveWeight)}
  pure def canRegisterWatcher(w:World,weight:int,attested:bool,effectiveWeight:int):bool = all {rangeState(w.s),w.s.recorded,not(w.s.settled),not(w.s.watcherRegistered),u64(weight),u64(effectiveWeight), (attested and effectiveWeight==weight) or (not(attested) and effectiveWeight==0),}
  pure def applyRegisterWatcher(w:World,weight:int,attested:bool,effectiveWeight:int):World = mark(w,{...w.s,watcherWeight:effectiveWeight,watcherRegistered:true},"registerWatcher")
  action registerWatcher(weight:int,attested:bool,effectiveWeight:int):bool = all {canRegisterWatcher(world,weight,attested,effectiveWeight),world'=applyRegisterWatcher(world,weight,attested,effectiveWeight)}
  pure def canSubmitOrdinary(w:World,amount:int):bool = all {rangeState(w.s),w.s.recorded,not(w.s.settled),not(w.s.ordinaryRecorded),fresh(w,"cost_ordinary"),u64(amount),amount>0,w.s.metered+amount<=w.s.gross,amount<=w.s.carried,}
  pure def applySubmitOrdinary(w:World,amount:int):World = create(mark(w,{...w.s,ordinaryAmount:amount,ordinaryRecorded:true,metered:w.s.metered+amount,carried:w.s.carried-amount},"submitOrdinary"),"cost_ordinary",amount)
  action submitOrdinary(amount:int):bool = all {canSubmitOrdinary(world,amount),world'=applySubmitOrdinary(world,amount)}
  pure def canSubmitStipend(w:World,amount:int):bool = all {rangeState(w.s),w.s.recorded,not(w.s.settled),not(w.s.stipendRecorded),fresh(w,"cost_stipend"),u64(amount),amount>0,w.s.metered+amount<=w.s.gross,amount<=w.s.carried,w.s.watcherRegistered,w.s.watcherWeight>0,amount<=w.s.stipendLimit,}
  pure def applySubmitStipend(w:World,amount:int):World = create(mark(w,{...w.s,stipendAmount:amount,stipendRecorded:true,metered:w.s.metered+amount,carried:w.s.carried-amount},"submitStipend"),"cost_stipend",amount)
  action submitStipend(amount:int):bool = all {canSubmitStipend(world,amount),world'=applySubmitStipend(world,amount)}
  pure def refillOf(balance:int,target:int,amount:int):int = if (balance >= target) 0 else if (amount <= target-balance) amount else target-balance
  pure def division(n:int,weight:int,d:int,q:int,r:int):bool = all {u64(n),u64(weight),u64(d),u64(q),u64(r),d>0,r<d,q*d+r==n*weight}
  pure def canSettle(w:World,opQ:int,opR:int,daQ:int,daR:int,watcherQ:int,watcherR:int,l2Q:int,l2R:int,refill:int):bool = {
    val surplus=w.s.gross-w.s.metered
    all {rangeState(w.s),w.s.recorded,not(w.s.settled),division(surplus,55,100,opQ,opR),division(surplus,10,100,daQ,daR),division(surplus,5,100,watcherQ,watcherR),division(surplus,15,100,l2Q,l2R),u64(refill),refill==refillOf(w.s.backstop,w.s.target,l2Q),u64(w.s.watcherHeld+watcherQ),u64(w.s.backstop+refill),opQ+daQ+watcherQ+l2Q<=surplus,fresh(w,"treasury_l2"),fresh(w,"treasury_midnight")}
  }
  pure def applySettle(w:World,opQ:int,opR:int,daQ:int,daR:int,watcherQ:int,watcherR:int,l2Q:int,l2R:int,refill:int):World = {
    val midnight=w.s.gross-w.s.metered-opQ-daQ-watcherQ-l2Q
    val n=mark(w,{...w.s,settled:true,opTotal:opQ,daTotal:daQ,watcherTotal:watcherQ,opRemaining:opQ,daRemaining:daQ,carried:opQ+daQ,watcherHeld:w.s.watcherHeld+watcherQ,backstop:w.s.backstop+refill,l2Amount:l2Q-refill,midnightAmount:midnight},"settle")
    val c=create(create(n,"treasury_l2",l2Q-refill),"treasury_midnight",midnight)
    {...c,watcherCredit:w.watcherCredit+watcherQ}
  }
  action settle(opQ:int,opR:int,daQ:int,daR:int,watcherQ:int,watcherR:int,l2Q:int,l2R:int,refill:int):bool = all {canSettle(world,opQ,opR,daQ,daR,watcherQ,watcherR,l2Q,l2R,refill),world'=applySettle(world,opQ,opR,daQ,daR,watcherQ,watcherR,l2Q,l2R,refill)}
  pure def outstanding(w:World):int = DUTIES.fold(0,(a,k)=>a+w.duties.get(k).amount)
  pure def accounting(w:World):bool = w.s.gross==w.s.claimedTotal+outstanding(w)+w.s.backstop+w.s.watcherHeld+w.s.carried
  val conservation = accounting(world)
  val wRegisterOperatorA=world.hits.contains("registerOperatorA")
  val wRegisterOperatorB=world.hits.contains("registerOperatorB")
  val wRegisterDa=world.hits.contains("registerDa")
  val wRegisterWatcher=world.hits.contains("registerWatcher")
  val wSubmitOrdinary=world.hits.contains("submitOrdinary")
  val wSubmitStipend=world.hits.contains("submitStipend")
  val wSettle=world.hits.contains("settle")
  pure def canAllocateOperatorA(w:World,denominator:int,quotient:int,remainder:int):bool = all {rangeState(w.s),w.s.settled,not(w.s.finished),w.s.opARegistered,not(w.s.opAAllocated),fresh(w,"reward_operator_a"),denominator==(if (w.s.opWeight==0) 1 else w.s.opWeight),division(w.s.opTotal,w.s.opAWeight,denominator,quotient,remainder),quotient<=w.s.opRemaining,quotient<=w.s.carried,w.s.allocatedCount+1<=w.s.rewardSeatCount,u64(w.s.epochClose+HOLD)}
  pure def applyAllocateOperatorA(w:World,denominator:int,quotient:int,remainder:int):World = create(mark(w,{...w.s,opAAllocated:true,opAAmount:quotient,opRemaining:w.s.opRemaining-quotient,carried:w.s.carried-quotient,allocatedCount:w.s.allocatedCount+1},"allocateOperatorA"),"reward_operator_a",quotient)
  action allocateOperatorA(denominator:int,quotient:int,remainder:int):bool = all {canAllocateOperatorA(world,denominator,quotient,remainder),world'=applyAllocateOperatorA(world,denominator,quotient,remainder)}
  pure def canAllocateOperatorB(w:World,denominator:int,quotient:int,remainder:int):bool = all {rangeState(w.s),w.s.settled,not(w.s.finished),w.s.opBRegistered,not(w.s.opBAllocated),fresh(w,"reward_operator_b"),denominator==(if (w.s.opWeight==0) 1 else w.s.opWeight),division(w.s.opTotal,w.s.opBWeight,denominator,quotient,remainder),quotient<=w.s.opRemaining,quotient<=w.s.carried,w.s.allocatedCount+1<=w.s.rewardSeatCount,u64(w.s.epochClose+HOLD)}
  pure def applyAllocateOperatorB(w:World,denominator:int,quotient:int,remainder:int):World = create(mark(w,{...w.s,opBAllocated:true,opBAmount:quotient,opRemaining:w.s.opRemaining-quotient,carried:w.s.carried-quotient,allocatedCount:w.s.allocatedCount+1},"allocateOperatorB"),"reward_operator_b",quotient)
  action allocateOperatorB(denominator:int,quotient:int,remainder:int):bool = all {canAllocateOperatorB(world,denominator,quotient,remainder),world'=applyAllocateOperatorB(world,denominator,quotient,remainder)}
  pure def canAllocateDa(w:World,denominator:int,quotient:int,remainder:int):bool = all {rangeState(w.s),w.s.settled,not(w.s.finished),w.s.daRegistered,not(w.s.daAllocated),fresh(w,"reward_da"),denominator==(if (w.s.daWeight==0) 1 else w.s.daWeight),division(w.s.daTotal,w.s.daSeatWeight,denominator,quotient,remainder),quotient<=w.s.daRemaining,quotient<=w.s.carried,w.s.allocatedCount+1<=w.s.rewardSeatCount,u64(w.s.epochClose+HOLD)}
  pure def applyAllocateDa(w:World,denominator:int,quotient:int,remainder:int):World = create(mark(w,{...w.s,daAllocated:true,daAmount:quotient,daRemaining:w.s.daRemaining-quotient,carried:w.s.carried-quotient,allocatedCount:w.s.allocatedCount+1},"allocateDa"),"reward_da",quotient)
  action allocateDa(denominator:int,quotient:int,remainder:int):bool = all {canAllocateDa(world,denominator,quotient,remainder),world'=applyAllocateDa(world,denominator,quotient,remainder)}
  pure def canFinish(w:World,refill:int):bool = all {rangeState(w.s),w.s.settled,not(w.s.finished),w.s.allocatedCount==w.s.rewardSeatCount,w.s.carried==w.s.opRemaining+w.s.daRemaining,u64(refill),refill==refillOf(w.s.backstop,w.s.target,w.s.carried),fresh(w,"treasury_dust"),u64(w.s.backstop+refill)}
  pure def applyFinish(w:World,refill:int):World = create(mark(w,{...w.s,finished:true,dustAmount:w.s.carried-refill,backstop:w.s.backstop+refill,opRemaining:0,daRemaining:0,carried:0},"finish"),"treasury_dust",w.s.carried-refill)
  action finish(refill:int):bool = all {canFinish(world,refill),world'=applyFinish(world,refill)}
  pure def canSpendWatcher(w:World,kind:int,amount:int):bool = all {rangeState(w.s),w.s.recorded,w.s.settled,not(w.s.spendIssued),u64(kind),kind==7 or kind==8,u64(amount),amount>0,amount<=w.s.watcherHeld,fresh(w,"watcher_spend")}
  pure def applySpendWatcher(w:World,kind:int,amount:int):World = {
    val n=create(mark(w,{...w.s,watcherHeld:w.s.watcherHeld-amount,spendAmount:amount,spendIssued:true},"spendWatcher"),"watcher_spend",amount)
    {...n,watcherSpent:w.watcherSpent+amount}
  }
  action spendWatcher(kind:int,amount:int):bool = all {canSpendWatcher(world,kind,amount),world'=applySpendWatcher(world,kind,amount)}
  pure def canSetTarget(w:World,newTarget:int):bool = all {rangeState(w.s),w.s.recorded,u64(newTarget)}
  pure def applySetTarget(w:World,newTarget:int):World = {...mark(w,{...w.s,target:newTarget},"setTarget"),maxTarget:if (newTarget>w.maxTarget) newTarget else w.maxTarget}
  action setTarget(newTarget:int):bool = all {canSetTarget(world,newTarget),world'=applySetTarget(world,newTarget)}
  pure def canClaimOrdinary(w:World,observedAt:int):bool = all {rangeState(w.s),w.s.recorded,w.s.settled,w.s.ordinaryRecorded,not(w.s.ordinaryClaimed),w.s.ordinaryAmount>0,w.duties.get("cost_ordinary").present,w.duties.get("cost_ordinary").amount==w.s.ordinaryAmount,u64(observedAt),u64(w.s.epochClose),observedAt>=w.s.epochClose,u64(w.s.claimedTotal+w.s.ordinaryAmount),w.cash.get("fee-pool")>=w.s.ordinaryAmount}
  pure def applyClaimOrdinary(w:World,observedAt:int):World = {
    val amount=w.s.ordinaryAmount
    val n=mark(w,{...w.s,ordinaryClaimed:true,claimedTotal:w.s.claimedTotal+amount},"claimOrdinary")
    {...n,duties:w.duties.set("cost_ordinary",{...w.duties.get("cost_ordinary"),amount:0}),cash:w.cash.set("fee-pool",w.cash.get("fee-pool")-amount).set("cost-provider",w.cash.get("cost-provider")+amount),claims:w.claims.set("cost_ordinary",w.claims.get("cost_ordinary")+1),claimTimes:w.claimTimes.set("cost_ordinary",observedAt),hits:if (w.s.finished) n.hits.union(Set("lateClaim")) else n.hits,history:w.history.append(effect("Transfer","fee-pool","cost-provider","cost_ordinary",amount)).append(effect("Discharge","fee-pool","cost-provider","cost_ordinary",amount))}
  }
  action claimOrdinary(observedAt:int):bool = all {canClaimOrdinary(world,observedAt),world'=applyClaimOrdinary(world,observedAt)}
  pure def canClaimStipend(w:World,observedAt:int):bool = all {rangeState(w.s),w.s.recorded,w.s.settled,w.s.stipendRecorded,not(w.s.stipendClaimed),w.s.stipendAmount>0,w.duties.get("cost_stipend").present,w.duties.get("cost_stipend").amount==w.s.stipendAmount,u64(observedAt),u64(w.s.epochClose),observedAt>=w.s.epochClose,u64(w.s.claimedTotal+w.s.stipendAmount),w.cash.get("fee-pool")>=w.s.stipendAmount}
  pure def applyClaimStipend(w:World,observedAt:int):World = {
    val amount=w.s.stipendAmount
    val n=mark(w,{...w.s,stipendClaimed:true,claimedTotal:w.s.claimedTotal+amount},"claimStipend")
    {...n,duties:w.duties.set("cost_stipend",{...w.duties.get("cost_stipend"),amount:0}),cash:w.cash.set("fee-pool",w.cash.get("fee-pool")-amount).set("watcher",w.cash.get("watcher")+amount),claims:w.claims.set("cost_stipend",w.claims.get("cost_stipend")+1),claimTimes:w.claimTimes.set("cost_stipend",observedAt),hits:if (w.s.finished) n.hits.union(Set("lateClaim")) else n.hits,history:w.history.append(effect("Transfer","fee-pool","watcher","cost_stipend",amount)).append(effect("Discharge","fee-pool","watcher","cost_stipend",amount))}
  }
  action claimStipend(observedAt:int):bool = all {canClaimStipend(world,observedAt),world'=applyClaimStipend(world,observedAt)}
  pure def canClaimOperatorA(w:World,observedAt:int):bool = all {rangeState(w.s),w.s.recorded,w.s.settled,w.s.opAAllocated,not(w.s.opAClaimed),w.s.opAAmount>0,w.duties.get("reward_operator_a").present,w.duties.get("reward_operator_a").amount==w.s.opAAmount,u64(observedAt),u64(w.s.epochClose+HOLD),observedAt>=w.s.epochClose+HOLD,u64(w.s.claimedTotal+w.s.opAAmount),w.cash.get("fee-pool")>=w.s.opAAmount}
  pure def applyClaimOperatorA(w:World,observedAt:int):World = {
    val amount=w.s.opAAmount
    val n=mark(w,{...w.s,opAClaimed:true,claimedTotal:w.s.claimedTotal+amount},"claimOperatorA")
    {...n,duties:w.duties.set("reward_operator_a",{...w.duties.get("reward_operator_a"),amount:0}),cash:w.cash.set("fee-pool",w.cash.get("fee-pool")-amount).set("operator-a",w.cash.get("operator-a")+amount),claims:w.claims.set("reward_operator_a",w.claims.get("reward_operator_a")+1),claimTimes:w.claimTimes.set("reward_operator_a",observedAt),hits:if (w.s.finished) n.hits.union(Set("lateClaim")) else n.hits,history:w.history.append(effect("Transfer","fee-pool","operator-a","reward_operator_a",amount)).append(effect("Discharge","fee-pool","operator-a","reward_operator_a",amount))}
  }
  action claimOperatorA(observedAt:int):bool = all {canClaimOperatorA(world,observedAt),world'=applyClaimOperatorA(world,observedAt)}
  pure def canClaimOperatorB(w:World,observedAt:int):bool = all {rangeState(w.s),w.s.recorded,w.s.settled,w.s.opBAllocated,not(w.s.opBClaimed),w.s.opBAmount>0,w.duties.get("reward_operator_b").present,w.duties.get("reward_operator_b").amount==w.s.opBAmount,u64(observedAt),u64(w.s.epochClose+HOLD),observedAt>=w.s.epochClose+HOLD,u64(w.s.claimedTotal+w.s.opBAmount),w.cash.get("fee-pool")>=w.s.opBAmount}
  pure def applyClaimOperatorB(w:World,observedAt:int):World = {
    val amount=w.s.opBAmount
    val n=mark(w,{...w.s,opBClaimed:true,claimedTotal:w.s.claimedTotal+amount},"claimOperatorB")
    {...n,duties:w.duties.set("reward_operator_b",{...w.duties.get("reward_operator_b"),amount:0}),cash:w.cash.set("fee-pool",w.cash.get("fee-pool")-amount).set("operator-b",w.cash.get("operator-b")+amount),claims:w.claims.set("reward_operator_b",w.claims.get("reward_operator_b")+1),claimTimes:w.claimTimes.set("reward_operator_b",observedAt),hits:if (w.s.finished) n.hits.union(Set("lateClaim")) else n.hits,history:w.history.append(effect("Transfer","fee-pool","operator-b","reward_operator_b",amount)).append(effect("Discharge","fee-pool","operator-b","reward_operator_b",amount))}
  }
  action claimOperatorB(observedAt:int):bool = all {canClaimOperatorB(world,observedAt),world'=applyClaimOperatorB(world,observedAt)}
  pure def canClaimDa(w:World,observedAt:int):bool = all {rangeState(w.s),w.s.recorded,w.s.settled,w.s.daAllocated,not(w.s.daClaimed),w.s.daAmount>0,w.duties.get("reward_da").present,w.duties.get("reward_da").amount==w.s.daAmount,u64(observedAt),u64(w.s.epochClose+HOLD),observedAt>=w.s.epochClose+HOLD,u64(w.s.claimedTotal+w.s.daAmount),w.cash.get("fee-pool")>=w.s.daAmount}
  pure def applyClaimDa(w:World,observedAt:int):World = {
    val amount=w.s.daAmount
    val n=mark(w,{...w.s,daClaimed:true,claimedTotal:w.s.claimedTotal+amount},"claimDa")
    {...n,duties:w.duties.set("reward_da",{...w.duties.get("reward_da"),amount:0}),cash:w.cash.set("fee-pool",w.cash.get("fee-pool")-amount).set("da-provider",w.cash.get("da-provider")+amount),claims:w.claims.set("reward_da",w.claims.get("reward_da")+1),claimTimes:w.claimTimes.set("reward_da",observedAt),hits:if (w.s.finished) n.hits.union(Set("lateClaim")) else n.hits,history:w.history.append(effect("Transfer","fee-pool","da-provider","reward_da",amount)).append(effect("Discharge","fee-pool","da-provider","reward_da",amount))}
  }
  action claimDa(observedAt:int):bool = all {canClaimDa(world,observedAt),world'=applyClaimDa(world,observedAt)}
  pure def canClaimL2(w:World,observedAt:int):bool = all {rangeState(w.s),w.s.recorded,w.s.settled,w.s.settled,not(w.s.l2Claimed),w.s.l2Amount>0,w.duties.get("treasury_l2").present,w.duties.get("treasury_l2").amount==w.s.l2Amount,u64(observedAt),u64(w.s.epochClose),observedAt>=w.s.epochClose,u64(w.s.claimedTotal+w.s.l2Amount),w.cash.get("fee-pool")>=w.s.l2Amount}
  pure def applyClaimL2(w:World,observedAt:int):World = {
    val amount=w.s.l2Amount
    val n=mark(w,{...w.s,l2Claimed:true,claimedTotal:w.s.claimedTotal+amount},"claimL2")
    {...n,duties:w.duties.set("treasury_l2",{...w.duties.get("treasury_l2"),amount:0}),cash:w.cash.set("fee-pool",w.cash.get("fee-pool")-amount).set("l2-treasury",w.cash.get("l2-treasury")+amount),claims:w.claims.set("treasury_l2",w.claims.get("treasury_l2")+1),claimTimes:w.claimTimes.set("treasury_l2",observedAt),hits:if (w.s.finished) n.hits.union(Set("lateClaim")) else n.hits,history:w.history.append(effect("Transfer","fee-pool","l2-treasury","treasury_l2",amount)).append(effect("Discharge","fee-pool","l2-treasury","treasury_l2",amount))}
  }
  action claimL2(observedAt:int):bool = all {canClaimL2(world,observedAt),world'=applyClaimL2(world,observedAt)}
  pure def canClaimMidnight(w:World,observedAt:int):bool = all {rangeState(w.s),w.s.recorded,w.s.settled,w.s.settled,not(w.s.midnightClaimed),w.s.midnightAmount>0,w.duties.get("treasury_midnight").present,w.duties.get("treasury_midnight").amount==w.s.midnightAmount,u64(observedAt),u64(w.s.epochClose),observedAt>=w.s.epochClose,u64(w.s.claimedTotal+w.s.midnightAmount),w.cash.get("fee-pool")>=w.s.midnightAmount}
  pure def applyClaimMidnight(w:World,observedAt:int):World = {
    val amount=w.s.midnightAmount
    val n=mark(w,{...w.s,midnightClaimed:true,claimedTotal:w.s.claimedTotal+amount},"claimMidnight")
    {...n,duties:w.duties.set("treasury_midnight",{...w.duties.get("treasury_midnight"),amount:0}),cash:w.cash.set("fee-pool",w.cash.get("fee-pool")-amount).set("midnight-treasury",w.cash.get("midnight-treasury")+amount),claims:w.claims.set("treasury_midnight",w.claims.get("treasury_midnight")+1),claimTimes:w.claimTimes.set("treasury_midnight",observedAt),hits:if (w.s.finished) n.hits.union(Set("lateClaim")) else n.hits,history:w.history.append(effect("Transfer","fee-pool","midnight-treasury","treasury_midnight",amount)).append(effect("Discharge","fee-pool","midnight-treasury","treasury_midnight",amount))}
  }
  action claimMidnight(observedAt:int):bool = all {canClaimMidnight(world,observedAt),world'=applyClaimMidnight(world,observedAt)}
  pure def canClaimDust(w:World,observedAt:int):bool = all {rangeState(w.s),w.s.recorded,w.s.settled,w.s.finished,not(w.s.dustClaimed),w.s.dustAmount>0,w.duties.get("treasury_dust").present,w.duties.get("treasury_dust").amount==w.s.dustAmount,u64(observedAt),u64(w.s.epochClose),observedAt>=w.s.epochClose,u64(w.s.claimedTotal+w.s.dustAmount),w.cash.get("fee-pool")>=w.s.dustAmount}
  pure def applyClaimDust(w:World,observedAt:int):World = {
    val amount=w.s.dustAmount
    val n=mark(w,{...w.s,dustClaimed:true,claimedTotal:w.s.claimedTotal+amount},"claimDust")
    {...n,duties:w.duties.set("treasury_dust",{...w.duties.get("treasury_dust"),amount:0}),cash:w.cash.set("fee-pool",w.cash.get("fee-pool")-amount).set("l2-treasury",w.cash.get("l2-treasury")+amount),claims:w.claims.set("treasury_dust",w.claims.get("treasury_dust")+1),claimTimes:w.claimTimes.set("treasury_dust",observedAt),hits:if (w.s.finished) n.hits.union(Set("lateClaim")) else n.hits,history:w.history.append(effect("Transfer","fee-pool","l2-treasury","treasury_dust",amount)).append(effect("Discharge","fee-pool","l2-treasury","treasury_dust",amount))}
  }
  action claimDust(observedAt:int):bool = all {canClaimDust(world,observedAt),world'=applyClaimDust(world,observedAt)}
  pure def canClaimWatcherSpend(w:World,observedAt:int):bool = all {rangeState(w.s),w.s.recorded,w.s.settled,w.s.spendIssued,not(w.s.spendClaimed),w.s.spendAmount>0,w.duties.get("watcher_spend").present,w.duties.get("watcher_spend").amount==w.s.spendAmount,u64(observedAt),u64(w.s.epochClose),observedAt>=w.s.epochClose,u64(w.s.claimedTotal+w.s.spendAmount),w.cash.get("fee-pool")>=w.s.spendAmount}
  pure def applyClaimWatcherSpend(w:World,observedAt:int):World = {
    val amount=w.s.spendAmount
    val n=mark(w,{...w.s,spendClaimed:true,claimedTotal:w.s.claimedTotal+amount},"claimWatcherSpend")
    {...n,duties:w.duties.set("watcher_spend",{...w.duties.get("watcher_spend"),amount:0}),cash:w.cash.set("fee-pool",w.cash.get("fee-pool")-amount).set("watcher",w.cash.get("watcher")+amount),claims:w.claims.set("watcher_spend",w.claims.get("watcher_spend")+1),claimTimes:w.claimTimes.set("watcher_spend",observedAt),hits:if (w.s.finished) n.hits.union(Set("lateClaim")) else n.hits,history:w.history.append(effect("Transfer","fee-pool","watcher","watcher_spend",amount)).append(effect("Discharge","fee-pool","watcher","watcher_spend",amount))}
  }
  action claimWatcherSpend(observedAt:int):bool = all {canClaimWatcherSpend(world,observedAt),world'=applyClaimWatcherSpend(world,observedAt)}
  pure def flag(b:bool):int = if (b) 1 else 0
  val custodyConservation = world.cash.get("fee-pool")+world.s.claimedTotal==world.s.gross and ACTORS.fold(0,(a,k)=>a+world.cash.get(k))==M
  val costsFirst = world.s.metered==world.s.ordinaryAmount+world.s.stipendAmount and world.s.metered<=world.s.gross
  val splitsExact = world.s.settled implies {
    val surplus=world.s.gross-world.s.metered
    all {world.s.opTotal==surplus*55/100,world.s.daTotal==surplus*10/100,world.s.watcherTotal==surplus*5/100,world.s.midnightAmount==surplus-world.s.opTotal-world.s.daTotal-world.s.watcherTotal-surplus*15/100,world.s.midnightAmount>=surplus*15/100,world.s.midnightAmount<=surplus*15/100+4}
  }
  val reserveHeld = world.s.watcherHeld+world.watcherSpent==world.watcherCredit and world.watcherCredit==world.s.watcherTotal and world.watcherSpent==world.s.spendAmount
  val noDoubleClaim = DUTIES.forall(k=>world.claims.get(k)<=1)
  val backstopBound = world.s.backstop<=world.maxTarget
  val residualSweep = world.s.finished implies all {world.s.settled,world.s.carried==0,world.s.opRemaining==0,world.s.daRemaining==0,world.s.allocatedCount==world.s.rewardSeatCount}
  val counters = world.s.rewardSeatCount==flag(world.s.opARegistered)+flag(world.s.opBRegistered)+flag(world.s.daRegistered) and world.s.allocatedCount==flag(world.s.opAAllocated)+flag(world.s.opBAllocated)+flag(world.s.daAllocated) and world.s.allocatedCount<=world.s.rewardSeatCount and world.s.opWeight==world.s.opAWeight+world.s.opBWeight and world.s.daWeight==world.s.daSeatWeight
  val fixedOriginalWeight = all {
    world.s.opAAllocated implies world.s.opAAmount==world.s.opTotal*world.s.opAWeight/(if (world.s.opWeight==0) 1 else world.s.opWeight),
    world.s.opBAllocated implies world.s.opBAmount==world.s.opTotal*world.s.opBWeight/(if (world.s.opWeight==0) 1 else world.s.opWeight),
    world.s.daAllocated implies world.s.daAmount==world.s.daTotal*world.s.daSeatWeight/(if (world.s.daWeight==0) 1 else world.s.daWeight),
    not(world.s.finished) implies world.s.opRemaining==world.s.opTotal-world.s.opAAmount-world.s.opBAmount and world.s.daRemaining==world.s.daTotal-world.s.daAmount,
  }
  val immutableEntitlements = all {
    (if (not(world.s.ordinaryRecorded)) not(world.duties.get("cost_ordinary").present) and world.s.ordinaryAmount==0 and not(world.s.ordinaryClaimed) else world.duties.get("cost_ordinary").present and world.duties.get("cost_ordinary").original==world.s.ordinaryAmount and (if (world.s.ordinaryClaimed) world.s.ordinaryAmount>0 and world.duties.get("cost_ordinary").amount==0 else world.duties.get("cost_ordinary").amount==world.s.ordinaryAmount)),
    (if (not(world.s.stipendRecorded)) not(world.duties.get("cost_stipend").present) and world.s.stipendAmount==0 and not(world.s.stipendClaimed) else world.duties.get("cost_stipend").present and world.duties.get("cost_stipend").original==world.s.stipendAmount and (if (world.s.stipendClaimed) world.s.stipendAmount>0 and world.duties.get("cost_stipend").amount==0 else world.duties.get("cost_stipend").amount==world.s.stipendAmount)),
    (if (not(world.s.opAAllocated)) not(world.duties.get("reward_operator_a").present) and world.s.opAAmount==0 and not(world.s.opAClaimed) else world.duties.get("reward_operator_a").present and world.duties.get("reward_operator_a").original==world.s.opAAmount and (if (world.s.opAClaimed) world.s.opAAmount>0 and world.duties.get("reward_operator_a").amount==0 else world.duties.get("reward_operator_a").amount==world.s.opAAmount)),
    (if (not(world.s.opBAllocated)) not(world.duties.get("reward_operator_b").present) and world.s.opBAmount==0 and not(world.s.opBClaimed) else world.duties.get("reward_operator_b").present and world.duties.get("reward_operator_b").original==world.s.opBAmount and (if (world.s.opBClaimed) world.s.opBAmount>0 and world.duties.get("reward_operator_b").amount==0 else world.duties.get("reward_operator_b").amount==world.s.opBAmount)),
    (if (not(world.s.daAllocated)) not(world.duties.get("reward_da").present) and world.s.daAmount==0 and not(world.s.daClaimed) else world.duties.get("reward_da").present and world.duties.get("reward_da").original==world.s.daAmount and (if (world.s.daClaimed) world.s.daAmount>0 and world.duties.get("reward_da").amount==0 else world.duties.get("reward_da").amount==world.s.daAmount)),
    (if (not(world.s.settled)) not(world.duties.get("treasury_l2").present) and world.s.l2Amount==0 and not(world.s.l2Claimed) else world.duties.get("treasury_l2").present and world.duties.get("treasury_l2").original==world.s.l2Amount and (if (world.s.l2Claimed) world.s.l2Amount>0 and world.duties.get("treasury_l2").amount==0 else world.duties.get("treasury_l2").amount==world.s.l2Amount)),
    (if (not(world.s.settled)) not(world.duties.get("treasury_midnight").present) and world.s.midnightAmount==0 and not(world.s.midnightClaimed) else world.duties.get("treasury_midnight").present and world.duties.get("treasury_midnight").original==world.s.midnightAmount and (if (world.s.midnightClaimed) world.s.midnightAmount>0 and world.duties.get("treasury_midnight").amount==0 else world.duties.get("treasury_midnight").amount==world.s.midnightAmount)),
    (if (not(world.s.finished)) not(world.duties.get("treasury_dust").present) and world.s.dustAmount==0 and not(world.s.dustClaimed) else world.duties.get("treasury_dust").present and world.duties.get("treasury_dust").original==world.s.dustAmount and (if (world.s.dustClaimed) world.s.dustAmount>0 and world.duties.get("treasury_dust").amount==0 else world.duties.get("treasury_dust").amount==world.s.dustAmount)),
    (if (not(world.s.spendIssued)) not(world.duties.get("watcher_spend").present) and world.s.spendAmount==0 and not(world.s.spendClaimed) else world.duties.get("watcher_spend").present and world.duties.get("watcher_spend").original==world.s.spendAmount and (if (world.s.spendClaimed) world.s.spendAmount>0 and world.duties.get("watcher_spend").amount==0 else world.duties.get("watcher_spend").amount==world.s.spendAmount))
  }
  val claimedTotals = world.s.claimedTotal==flag(world.s.ordinaryClaimed)*world.s.ordinaryAmount+flag(world.s.stipendClaimed)*world.s.stipendAmount+flag(world.s.opAClaimed)*world.s.opAAmount+flag(world.s.opBClaimed)*world.s.opBAmount+flag(world.s.daClaimed)*world.s.daAmount+flag(world.s.l2Claimed)*world.s.l2Amount+flag(world.s.midnightClaimed)*world.s.midnightAmount+flag(world.s.dustClaimed)*world.s.dustAmount+flag(world.s.spendClaimed)*world.s.spendAmount
  val vestingThresholdConditional = all {(world.s.ordinaryClaimed implies world.claimTimes.get("cost_ordinary")>=world.s.epochClose),(world.s.stipendClaimed implies world.claimTimes.get("cost_stipend")>=world.s.epochClose),(world.s.opAClaimed implies world.claimTimes.get("reward_operator_a")>=world.s.epochClose+HOLD),(world.s.opBClaimed implies world.claimTimes.get("reward_operator_b")>=world.s.epochClose+HOLD),(world.s.daClaimed implies world.claimTimes.get("reward_da")>=world.s.epochClose+HOLD),(world.s.l2Claimed implies world.claimTimes.get("treasury_l2")>=world.s.epochClose),(world.s.midnightClaimed implies world.claimTimes.get("treasury_midnight")>=world.s.epochClose),(world.s.dustClaimed implies world.claimTimes.get("treasury_dust")>=world.s.epochClose),(world.s.spendClaimed implies world.claimTimes.get("watcher_spend")>=world.s.epochClose)}
  val exactAssetAndActors = world.sentinelOtherAsset==777 and world.history.foldl(true,(ok,e)=>ok and e.network=="local" and e.issuer=="fee-pool-source" and e.reference=="SERVICE" and e.scale==0 and (if (e.source=="funding-source") e.kind=="Transfer" and e.recipient=="fee-pool" and e.duty=="" else e.source=="fee-pool" and DUTIES.contains(e.duty) and e.recipient==beneficiary(e.duty)))
  val refundGross = world.history.select(e=>e.kind=="Transfer" and e.source=="funding-source").foldl(0,(a,e)=>a+e.amount)==world.s.gross and world.history.foldl(true,(ok,e)=>ok and not(e.source=="fee-pool" and e.recipient=="funding-source"))
  val allInvariants = rangeState(world.s) and conservation and custodyConservation and costsFirst and splitsExact and reserveHeld and noDoubleClaim and backstopBound and residualSweep and counters and fixedOriginalWeight and immutableEntitlements and claimedTotals and vestingThresholdConditional and exactAssetAndActors and refundGross
  val wLateClaim = world.hits.contains("lateClaim")
  val wAllocateOperatorA = world.hits.contains("allocateOperatorA")
  val wAllocateOperatorB = world.hits.contains("allocateOperatorB")
  val wAllocateDa = world.hits.contains("allocateDa")
  val wFinish = world.hits.contains("finish")
  val wSpendWatcher = world.hits.contains("spendWatcher")
  val wSetTarget = world.hits.contains("setTarget")
  val wClaimOrdinary = world.hits.contains("claimOrdinary")
  val wClaimStipend = world.hits.contains("claimStipend")
  val wClaimOperatorA = world.hits.contains("claimOperatorA")
  val wClaimOperatorB = world.hits.contains("claimOperatorB")
  val wClaimDa = world.hits.contains("claimDa")
  val wClaimL2 = world.hits.contains("claimL2")
  val wClaimMidnight = world.hits.contains("claimMidnight")
  val wClaimDust = world.hits.contains("claimDust")
  val wClaimWatcherSpend = world.hits.contains("claimWatcherSpend")
  action step = {
    nondet g=Set(99,100,111).oneOf()
    nondet weight=Set(0,1,2).oneOf()
    nondet target=Set(0,17,500).oneOf()
    val surplus=world.s.gross-world.s.metered
    val opDen=if (world.s.opWeight==0) 1 else world.s.opWeight
    val daDen=if (world.s.daWeight==0) 1 else world.s.daWeight
    any {
      intake(0,g,86400,target,10),registerOperatorA(weight,true,weight),registerOperatorB(weight,true,weight),registerDa(weight,true,weight),registerWatcher(1,true,1),submitOrdinary(5),submitStipend(5),
      settle(surplus*55/100,surplus*55%100,surplus*10/100,surplus*10%100,surplus*5/100,surplus*5%100,surplus*15/100,surplus*15%100,refillOf(world.s.backstop,world.s.target,surplus*15/100)),
      allocateOperatorA(opDen,world.s.opTotal*world.s.opAWeight/opDen,world.s.opTotal*world.s.opAWeight%opDen),
      allocateOperatorB(opDen,world.s.opTotal*world.s.opBWeight/opDen,world.s.opTotal*world.s.opBWeight%opDen),
      allocateDa(daDen,world.s.daTotal*world.s.daSeatWeight/daDen,world.s.daTotal*world.s.daSeatWeight%daDen),
      finish(refillOf(world.s.backstop,world.s.target,world.s.carried)),spendWatcher(7,1),
      all {not(world.hits.contains("setTarget")),setTarget(target)},
      claimOrdinary(2678400),claimStipend(2678400),claimOperatorA(2678400),claimOperatorB(2678400),claimDa(2678400),claimL2(2678400),claimMidnight(2678400),claimDust(2678400),claimWatcherSpend(2678400),
    }
  }
}
