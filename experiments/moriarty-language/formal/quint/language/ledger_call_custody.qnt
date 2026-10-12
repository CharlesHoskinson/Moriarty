// Preparation QA only. Byte/FAB/VM correctness, hashes, signatures, proofs and
// funding are abstract premises, not established by this model. No consumed
// ledger state, proof/settlement flag, network or distributed actors.
module ledger_call_custody {
  type Input = { id: int, owner: int, token: int, amount: int }
  type Output = { owner: int, token: int, amount: int }
  type Effect = { kind: str, token: int, from: int, to: int, amount: int }
  type Request = { opening: List[Input], selected: List[int], effects: List[Effect], claims: (int,int)->int,
    outputs: List[Output], retained: List[Input], initialDuties: int->int, dutyUpdates: List[(int,int)],
    expectedDuties: int->int, initialValue: int, nextValue: int, frozen: bool, events: List[int] }
  type State = { prepared: bool, refused: bool, postBalances: (int,int)->int, duties: int->int,
    value: int, frozen: bool, outputs: List[Output], claims: (int,int)->int, retained: List[Input],
    events: List[int], request: Request }
  var state: State
  pure val uintMax = 340282366920938463463374607431768211455
  pure val nativeMax = 170141183460469231731687303715884105727
  pure val balanceKeys = Set((0,0),(0,1),(1,0),(1,1),(2,0),(2,1),(3,0),(3,1),(4,0),(4,1))
  pure def coin(id: int, amount: int, owner: int): Input = { id: id, owner: owner, token: 0, amount: amount }
  pure def out(amount: int, owner: int): Output = { amount: amount, owner: owner, token: 0 }
  pure def transfer(amount: int, recipient: int): Effect = { kind: "Transfer", token: 0, from: 0, to: recipient, amount: amount }
  pure def fee(amount: int, recipient: int): Effect = { ...transfer(amount,recipient), kind: "Fee" }
  pure def claimsFor(es: List[Effect]): (int,int)->int = es.foldl(Map(), (m,e) =>
    if (e.amount == 0) m else {
      val k = (e.token,e.to)
      m.put(k, (if(m.keys().contains(k)) m.get(k) else 0) + e.amount)
    })
  pure def selection(r: Request): Set[int] = r.selected.foldl(Set(),(ids,id)=>ids.union(Set(id)))
  pure def selectedInputs(r: Request): List[Input] = r.opening.select(c => selection(r).contains(c.id))
  pure def budget(r: Request, token: int): int = selectedInputs(r).foldl(0, (n,c) => n + (if(c.token == token) c.amount else 0))
  pure def positiveTotal(r: Request, token: int): int = r.effects.foldl(0, (n,e) => n + (if(e.token == token) e.amount else 0))
  pure def expectedOutputs(r: Request): List[Output] = {
    val groups = claimsFor(r.effects)
    val payments = groups.keys().fold(List(), (l,k) => l.append({ token: k._1, owner: k._2, amount: groups.get(k) }))
    Set(0,1).fold(payments, (l,t) => {
      val change = budget(r,t) - positiveTotal(r,t)
      if (change > 0) l.append({ token: t, owner: 0, amount: change }) else l
    })
  }
  pure def occurrences(xs: List[Output], x: Output): int = xs.foldl(0,(n,o) => n+(if(o==x) 1 else 0))
  pure def sameMultiset(a: List[Output], b: List[Output]): bool = a.length()==b.length()
    and a.foldl(true,(ok,o) => ok and occurrences(a,o)==occurrences(b,o))
  pure def nativeSubset(claims: (int,int)->int, outputs: List[Output]): bool = claims.keys().forall(k =>
    occurrences(outputs,{ token:k._1,owner:k._2,amount:claims.get(k) }) >= 1)
  pure def precedes(a: Output,b: Output): bool = a.amount<b.amount or (a.amount==b.amount and
    (a.owner<b.owner or (a.owner==b.owner and a.token<=b.token)))
  pure def sorted(xs: List[Output]): bool = xs.indices().forall(i =>
    i==0 or precedes(xs.nth(i-1),xs.nth(i)))
  pure def balances(cs: List[Input]): (int,int)->int = balanceKeys.mapBy(k => cs.foldl(0,(n,c) =>
    n+(if(c.owner==k._1 and c.token==k._2) c.amount else 0)))
  pure def projected(r: Request): (int,int)->int = {
    val retained = balances(r.retained)
    r.outputs.foldl(retained,(bs,o) => bs.setBy((o.owner,o.token),n => n+o.amount))
  }
  pure def semanticBalances(r: Request): (int,int)->int = r.effects.foldl(balances(r.opening),(bs,e) =>
    if (e.from==e.to) bs else bs.setBy((e.from,e.token),n=>n-e.amount).setBy((e.to,e.token),n=>n+e.amount))
  pure def dutyResult(r: Request): int->int = r.dutyUpdates.foldl(r.initialDuties,(ds,u)=>ds.set(u._1,u._2))
  pure def canPrepare(r: Request): bool = and {
    r.selected.length() == r.selected.foldl(Set(),(ids,id)=>ids.union(Set(id))).size(),
    r.selected.foldl(true,(ok,id)=>ok and r.opening.foldl(false,(found,c)=>found or (c.id==id and c.owner==0))),
    r.opening.foldl(true,(ok,c)=>ok and c.amount>=0 and c.amount<=uintMax and Set(0,1).contains(c.token)),
    r.effects.foldl(true,(ok,e)=>ok and Set("Transfer","Fee").contains(e.kind) and e.from==0
      and Set(0,1,2,3,4).contains(e.to) and Set(0,1).contains(e.token) and e.amount>=0 and e.amount<=uintMax),
    Set(0,1).forall(t=>budget(r,t)>=positiveTotal(r,t) and budget(r,t)<=nativeMax),
    r.claims==claimsFor(r.effects), nativeSubset(r.claims,r.outputs),
    r.outputs.foldl(true,(ok,o)=>ok and o.amount>0 and o.amount<=nativeMax), sorted(r.outputs),
    sameMultiset(r.outputs,expectedOutputs(r)),
    Set(0,1).forall(t=>r.outputs.foldl(0,(n,o)=>n+(if(o.token==t) o.amount else 0))==budget(r,t)),
    r.retained==r.opening.select(c=>not(selection(r).contains(c.id))), projected(r)==semanticBalances(r),
    r.expectedDuties==dutyResult(r), r.expectedDuties.keys()==r.initialDuties.keys(),
    r.expectedDuties.keys().forall(n=>r.expectedDuties.get(n)>=0),
  }
  pure def request(es: List[Effect], outs: List[Output]): Request = {
    opening:List(coin(0,100,0),coin(1,10,1)),selected:List(0),effects:es,claims:claimsFor(es),outputs:outs,
    retained:List(coin(1,10,1)),initialDuties:Map(0->50,1->5,2->0,3->0),dutyUpdates:List(),
    expectedDuties:Map(0->50,1->5,2->0,3->0),initialValue:0,nextValue:0,frozen:true,events:List() }
  pure val paymentRequest: Request = { ...request(List(transfer(70,1),fee(2,2)),List(out(2,2),out(28,0),out(70,1))),
    dutyUpdates:List((0,30)),expectedDuties:Map(0->30,1->5,2->0,3->0),nextValue:70,events:List(70) }
  pure val collectionRequest: Request = { ...request(List(fee(3,2),transfer(40,3)),List(out(3,2),out(40,3),out(57,0))),
    opening:List(coin(0,100,0),coin(1,7,3),coin(2,1,2),coin(3,9,4)),retained:List(coin(1,7,3),coin(2,1,2),coin(3,9,4)),
    initialDuties:Map(0->12,1->5,2->0,3->0),dutyUpdates:List((3,4),(0,5)),expectedDuties:Map(0->5,1->5,2->0,3->4),
    nextValue:40,events:List(40) }
  pure val selfRequest=request(List(transfer(10,0),transfer(20,1),fee(2,0)),List(out(12,0),out(20,1),out(68,0)))
  pure val equalRequest=request(List(transfer(50,0)),List(out(50,0),out(50,0)))
  pure val repeatedRequest=request(List(transfer(10,1),transfer(15,1),fee(2,1)),List(out(27,1),out(73,0)))
  pure val zeroRequest=request(List(transfer(0,1),fee(0,2)),List(out(100,0)))
  pure val emptyRequest={ ...zeroRequest,selected:List(),outputs:List(),retained:zeroRequest.opening }
  pure val largeRequest={ ...request(List(transfer(18446744073709551616,1),transfer(18446744073709551616,1)),List(out(36893488147419103232,1))),
    opening:List(coin(0,36893488147419103232,0)),retained:List() }
  pure val initial: State={ prepared:false,refused:false,postBalances:balances(paymentRequest.opening),duties:paymentRequest.initialDuties,
    value:0,frozen:true,outputs:List(),claims:Map(),retained:paymentRequest.opening,events:List(),request:paymentRequest }
  action init=state'=initial
  action prepare(r: Request): bool=all { canPrepare(r),state'={ prepared:true,refused:false,postBalances:projected(r),duties:r.expectedDuties,
    value:r.nextValue,frozen:r.frozen,outputs:r.outputs,claims:r.claims,retained:r.retained,events:r.events,request:r } }
  action refuse(r: Request): bool=all { not(canPrepare(r)),state'={ ...state,refused:true } }
  action step={
    nondet r=Set(paymentRequest,collectionRequest,selfRequest,equalRequest,repeatedRequest,zeroRequest,emptyRequest,largeRequest,
      { ...equalRequest,outputs:List(out(50,0)) },{ ...paymentRequest,selected:List(0,0) }).oneOf()
    any { prepare(r),refuse(r) }
  }
  val preparation_invariants=state.prepared implies and {
    canPrepare(state.request), state.postBalances==semanticBalances(state.request),state.duties==dutyResult(state.request),
    state.outputs==state.request.outputs,state.claims==claimsFor(state.request.effects),state.events==state.request.events,
    state.value==state.request.nextValue,state.frozen==state.request.frozen,
    state.retained==state.request.opening.select(c=>not(selection(state.request).contains(c.id))) }
  val paymentWitness=state.prepared and state.value==70
  val collectionWitness=state.prepared and state.value==40
}
