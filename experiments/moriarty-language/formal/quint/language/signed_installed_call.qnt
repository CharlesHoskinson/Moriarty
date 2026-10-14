// Adopted signed-installed design 2026-10-12, SHA256 8f08c1952459f6eae7a61da84d6c1577a0c9eab4fddef25ce11d1bf0b17d4309.
// One local caller, shared immutable artifacts; no distributed/network model.
// Structural tuples stand for collision-resistant commitments; no hashes run here.
// EXTERNAL: strict curve/subgroup/FENS, source-to-wire/native correspondence,
// SDK commitment/PI/proof processing, public SRS and source-derived keygen.
// Those results are explicit external observations, never model-produced proofs.
module signed_installed_call {
  // Consuming-v2 supplied-state component. Authentic current ledger acceptance
  // remains external; adopting a supplied queried snapshot does not establish it.
  type SignedVersion = LegacyV1 | ConsumingV2
  type NoncePackage = {selectedAction: int, domain: int, predecessor: int, nonce: int, preCount: int, postCount: int, preMember: bool, postMember: bool}
  type NonceState = {version: SignedVersion, domain: int, head: int, consumed: Set[int], packages: int -> NoncePackage, authenticAcceptance: bool}
  pure val authorizationCapacity = 12153
  pure def nonceInitial(v: SignedVersion): NonceState = {version:v,domain:1,head:41,consumed:Set(),packages:Map(),authenticAcceptance:false}
  pure def noncePackage(a: NonceState,n: int): NoncePackage = {selectedAction:1,domain:a.domain,predecessor:a.head,nonce:n,preCount:a.consumed.size()+1,postCount:a.consumed.size()+2,preMember:a.consumed.contains(n),postMember:true}
  pure def nonceCanPrepare(a: NonceState,n: int,budgets: bool): bool = a.version == ConsumingV2 and n >= 0 and n <= 340282366920938463463374607431768211455
    and not(a.consumed.contains(n)) and a.consumed.size()+1 < authorizationCapacity and budgets
  pure def nonceTranscriptOnCurrent(a: NonceState,p: NoncePackage): bool = p.domain == a.domain and p.predecessor == a.head
    and p.preCount == a.consumed.size()+1 and p.postCount == p.preCount+1 and not(p.preMember) and p.postMember
    and nonceCanPrepare(a,p.nonce,true)
  pure def nonceAttachmentMatches(a: NonceState,p: NoncePackage): bool = p == {...noncePackage(a,p.nonce),selectedAction:p.selectedAction}
  type Phase = Idle | PublicMapCompared | SigningPrepared | Attached | NativeResultReported | ColdVerified | Refused
  type Case = A1 | A2 | Freeze | B1 | B2 | Reset
  type Source = Metered | Toggle
  type Value = UInt(int) | BoolValue(bool)
  type Kind = Write | Transfer | Fee | Liability | Discharge | Emit
  type Slot = {present: bool, amount: int, asset: int, debtor: int, creditor: int}
  type Registry = int -> {asset: int, debtor: int, creditor: int}
  type Snapshot = {values: int -> Value, balances: int -> int, duties: int -> Slot}
  type Effect = {kind: Kind, name: int, before: Value, after: Value, asset: int, payer: int, recipient: int, amount: int}
  type Caps = {gross: int, fee: int, net: int, duty: int}
  type Limits = List[{asset: int, caps: Caps}]
  type Policy = {owner: int, beneficiary: int, pk: int}
  // This static type deliberately cannot contain IR/keys/request/state/signature/final PI.
  type Context = {source: Source, closure: int, network: int, anchor: int, address: int,
    policy: Policy, assets: List[int], registry: Registry, dutyOrder: List[int], selectedAction: int, stateKeys: List[int], storageAbi: int, signedAbi: int}
  type Request = {inputs: List[Value], limits: Limits, nonce: int, start: int, end: int, at: int,
    predecessor: int, parameters: int, selectedSpends: List[int], ttl: int, randomness: int, bindingRandomness: int}
  type Projection = {inputs: List[Value], pre: Snapshot, effects: List[Effect], post: Snapshot}
  // Signature-free readback cannot contain full typed input, parent binding, comm,
  // final PI, intent/output IDs, signature/proof or authorization-status fields.
  type SigningReadback = {projection: Projection, operations: List[int], events: List[(int,Value)],
    queryState: int, queryEffects: int, measuredReadGas: int, declaredGas: int, preservedFrame: List[int], prospectiveAmounts: List[int]}
  type Payload = {authorityVersion: SignedVersion, authority: NoncePackage, context: Context, ir: int, request: Request, projection: Projection, wires: List[Value], readback: SigningReadback}
  type Package = {payload: Payload, messageDomain: int, messageVersion: int}
  type Signature = {pk: int, r: int, response: int}
  type Attachment = {payload: Payload, signature: Signature}
  type ExternalCheck = ExternalAccepted | ExternalRejected
  type Predicates = {strictHostFens: ExternalCheck, typedSubgroup: ExternalCheck, scalarCanonical: ExternalCheck,
    assertedFens: ExternalCheck, sourceNativeWires: ExternalCheck}
  type Key = {ir: int, vk: int, policy: Policy, signed: bool}
  type KeyMap = int -> Key
  type Layout = {typed: int, flattened: int, privateFields: int, authImpacts: int, outputs: int, communications: bool, sharedAllocator: bool}
  type Observations = {stateCount: int, dutyCount: int, memberships: int -> bool, reads: int -> Slot}
  type Totals = {outgoing: int, fees: int, credits: int, debits: int, gross: int, net: int, duty: int}
  type FullInput = {payload: Payload, signature: Signature, layout: Layout}
  type Communication = {input: FullInput, randomness: int}
  type PI = {binding: int, communication: Communication, processed: List[int]}
  type FinalReadback = {semantic: SigningReadback, pi: PI, parent: int, offers: List[int], ttl: int,
    actions: List[int], gas: int, version: int, outputIdentity: (int,Communication), amounts: List[int]}
  type Proof = {final: FinalReadback, operation: Key, attachment: Attachment, genuineV3: bool}
  type State = {phase: Phase, instance: Case, context: Context, request: Request, keys: KeyMap,
    package: Package, input: FullInput, proof: Proof, visited: Set[Phase], pkReads: int,
    expansions: int, faults: Set[int], signatureChecks: Predicates, nativeResult: ExternalCheck, nonce: NonceState}
  var state: State
  pure val uintMax = 340282366920938463463374607431768211455
  pure val policy: Policy = {owner: 1, beneficiary: 2, pk: 31}
  pure val acceptedPredicates: Predicates = {strictHostFens: ExternalAccepted, typedSubgroup: ExternalAccepted,
    scalarCanonical: ExternalAccepted, assertedFens: ExternalAccepted, sourceNativeWires: ExternalAccepted}
  pure def source(c: Case): Source = if (Set(A1,A2,Freeze).contains(c)) Metered else Toggle
  pure def actionId(c: Case): int = if (Set(A1,A2).contains(c)) 1 else if (c == Freeze) 2 else if (Set(B1,B2).contains(c)) 3 else 4
  pure def context(c: Case): Context = {source: source(c), closure: 20, network: 21, anchor: 22, address: 23,
    policy: policy, assets: if (source(c) == Metered) List(11) else List(),
    registry: if (source(c) == Metered) Map(7 -> {asset:11,debtor:1,creditor:2}) else Map(),
    selectedAction: actionId(c), dutyOrder:if(source(c) == Metered) List(7) else List(), stateKeys: List(1,2), storageAbi: 1, signedAbi: 2}
  pure def uint(n: int): bool = n >= 0 and n <= uintMax
  pure def slot(p: bool,n: int): Slot = {present:p,amount:n,asset:if(p) 11 else 0,debtor:if(p) 1 else 0,creditor:if(p) 2 else 0}
  pure def pre(c: Case): Snapshot = {values:Map(1 -> UInt(if(c == A2 or c == B1) 31 else if(c == B2 or c == Reset) 255 + (if(c == Reset) 1 else 0) else 4),2 -> BoolValue(c != Reset)),
    balances:if(source(c) == Metered) Map(1 -> 100,2 -> 5,3 -> 1) else Map(),
    duties:if(source(c) == Metered) Map(7 -> slot(c != A1,if(c == A1) 0 else 9)) else Map()}
  pure def post(c: Case): Snapshot = if(c == A1) {values:Map(1 -> UInt(11),2 -> BoolValue(true)),balances:Map(1 -> 84,2 -> 19,3 -> 3),duties:Map(7 -> slot(true,4))}
    else if(c == A2) {values:Map(1 -> UInt(34),2 -> BoolValue(true)),balances:Map(1 -> 92,2 -> 11,3 -> 3),duties:Map(7 -> slot(true,9))}
    else if(c == Freeze) {...pre(c),values:Map(1 -> UInt(4),2 -> BoolValue(false))}
    else {values:Map(1 -> UInt(if(c == B1) 32 else if(c == B2) 256 else 0),2 -> BoolValue(c == Reset)),balances:Map(),duties:Map()}
  pure def write(n: int,b: Value,a: Value): Effect = {kind:Write,name:n,before:b,after:a,asset:0,payer:0,recipient:0,amount:0}
  pure def payment(k: Kind,n: int): Effect = {kind:k,name:0,before:UInt(0),after:UInt(0),asset:11,payer:1,recipient:if(k == Fee) 3 else 2,amount:n}
  pure def liability(k: Kind,b: int,a: int,n: int): Effect = {kind:k,name:7,before:UInt(b),after:UInt(a),asset:11,payer:1,recipient:2,amount:n}
  pure def emit(n: int,v: Value): Effect = {kind:Emit,name:n,before:v,after:v,asset:0,payer:0,recipient:0,amount:0}
  pure def effects(c: Case): List[Effect] = if(Set(A1,A2).contains(c)) {
    val before = if(c == A1) 4 else 31
    val amount = if(c == A1) 7 else 3
    val duty = if(c == A1) 0 else 9
    List(write(1,UInt(before),UInt(before + amount)),payment(Transfer,amount),payment(Transfer,amount),payment(Fee,2),
      liability(Liability,duty,duty + amount,amount),liability(Discharge,duty + amount,duty + amount - 3,3),emit(3,UInt(before + amount)),emit(2,BoolValue(true)))
  } else if(c == Freeze) List(write(2,BoolValue(true),BoolValue(false)),emit(2,BoolValue(false)))
    else List(write(1,pre(c).values.get(1),post(c).values.get(1)),write(2,pre(c).values.get(2),post(c).values.get(2)),emit(1,post(c).values.get(1)))
      .concat(if(c == Reset) List() else List(emit(2,BoolValue(false))))
  pure def tight(c: Case): Limits = if(source(c) == Toggle) List() else List({asset:11,caps:
    if(c == A1) {gross:16,fee:2,net:12,duty:4} else if(c == A2) {gross:8,fee:2,net:4,duty:9} else {gross:0,fee:0,net:0,duty:9}})
  pure def request(c: Case): Request = {inputs:if(c == A1) List(UInt(7)) else if(c == A2) List(UInt(3)) else if(Set(B1,B2).contains(c)) List(UInt(1)) else List(),
    limits:tight(c),nonce:1,start:10,end:20,at:15,predecessor:41,parameters:42,selectedSpends:if(Set(A1,A2).contains(c)) List(51) else List(),ttl:52,randomness:53,bindingRandomness:54}
  pure def projection(c: Case): Projection = {inputs:request(c).inputs,pre:pre(c),effects:effects(c),post:post(c)}
  pure def readback(c: Case): SigningReadback = {projection:projection(c),operations:List(61,62),events:effects(c).foldl(List(),(es,e) => if(e.kind == Emit) es.append((e.name,e.after)) else es),
    queryState:63,queryEffects:64,measuredReadGas:65,declaredGas:66,preservedFrame:List(67,68),prospectiveAmounts:if(c == A1) List(14,2,84) else if(c == A2) List(6,2,92) else List()}
  pure def keyMap(c: Case): KeyMap = (if(source(c) == Metered) Set(1,2) else Set(3,4)).mapBy(i => {ir:100 + i,vk:200 + i,policy:policy,signed:true})
  pure def effectWires(e: Effect): List[Value] = if(e.kind == Write or e.kind == Liability) List(e.before,e.after)
    else if(e.kind == Discharge) List(UInt(e.amount),e.before,e.after)
    else if(e.kind == Emit) List(e.after) else List(UInt(e.amount))
  pure def endpointWires(c: Context,s: Snapshot): List[Value] = c.stateKeys.foldl(List(),(xs,i) => xs.append(s.values.get(i))).concat(c.dutyOrder.foldl(List(),(xs,i) => xs.concat(List(BoolValue(s.duties.get(i).present),UInt(s.duties.get(i).amount)))))
  pure def wires(c: Context,p: Projection): List[Value] = p.inputs.concat(endpointWires(c,p.pre)).concat(p.effects.foldl(List(),(xs,e) => xs.concat(effectWires(e)))).concat(endpointWires(c,p.post))
  pure def package(c: Case,q: Request): Package = {payload:{authorityVersion:LegacyV1,authority:{...noncePackage(nonceInitial(LegacyV1),q.nonce),selectedAction:actionId(c)},context:context(c),ir:100 + actionId(c),request:q,projection:projection(c),wires:wires(context(c),projection(c)),readback:readback(c)},messageDomain:826494029,messageVersion:1}
  pure def contextFor(a: NonceState,c: Case): Context = if(a.version == LegacyV1) context(c) else {...context(c),storageAbi:2,signedAbi:3}
  pure def keyMapFor(a: NonceState,c: Case): KeyMap = if(a.version == LegacyV1) keyMap(c) else keyMap(c).keys().mapBy(i => {...keyMap(c).get(i),ir:300+i,vk:400+i})
  pure def authorityFor(a: NonceState,c: Case,q: Request): NoncePackage = {...noncePackage(a,q.nonce),selectedAction:actionId(c)}
  pure def authorityWires(c: Context,p: Projection,a: NoncePackage): List[Value] = p.inputs.concat(endpointWires(c,p.pre))
    .concat(List(UInt(a.preCount),BoolValue(a.preMember)))
    .concat(p.effects.foldl(List(),(xs,e) => xs.concat(effectWires(e))))
    .concat(endpointWires(c,p.post)).concat(List(UInt(a.postCount),BoolValue(a.postMember)))
  pure def packageFor(a: NonceState,c: Case,q: Request): Package = if(a.version == LegacyV1) package(c,q) else {
    val auth = authorityFor(a,c,q)
    val ctx = contextFor(a,c)
    {...package(c,q),payload:{...package(c,q).payload,authorityVersion:ConsumingV2,authority:auth,context:ctx,ir:300+actionId(c),wires:authorityWires(ctx,projection(c),auth)}}
  }
  pure def authorityBound(s: State,p: Package,q: Request): bool = if(s.nonce.version == LegacyV1) p.payload.authorityVersion == LegacyV1 else and {
    p.payload.authorityVersion == ConsumingV2,s.nonce.domain == (if(source(s.instance) == Metered) 1 else 2),p.payload.authority == authorityFor(s.nonce,s.instance,q),
    p.payload.context == contextFor(s.nonce,s.instance),p.payload.context.selectedAction == p.payload.authority.selectedAction,
    q.predecessor == s.nonce.head,p.payload.request == q,nonceTranscriptOnCurrent(s.nonce,p.payload.authority),
    p.payload.wires == authorityWires(p.payload.context,p.payload.projection,p.payload.authority),
  }
  pure def signature(sigChoice: int): Signature = {pk:31,r:71 + sigChoice,response:81 + sigChoice}
  pure def attachment(p: Package,sigChoice: int): Attachment = {payload:p.payload,signature:signature(sigChoice)}
  pure def layout(c: Case): Layout = {typed:request(c).inputs.length() + 11 + 4 * context(c).assets.length(),flattened:request(c).inputs.length() + 13 + 4 * context(c).assets.length(),
    privateFields:0,authImpacts:0,outputs:1,communications:true,sharedAllocator:true}
  pure def input(p: Package,s: Signature,c: Case): FullInput = {payload:p.payload,signature:s,layout:layout(c)}
  pure def finalReadback(p: Package,i: FullInput): FinalReadback = {
    val comm = {input:i,randomness:p.payload.request.randomness}
    {semantic:p.payload.readback,pi:{binding:91,communication:comm,processed:List(92,0,93)},parent:94,
      offers:p.payload.request.selectedSpends,ttl:p.payload.request.ttl,actions:List(p.payload.context.selectedAction),gas:95,version:3,
      outputIdentity:(96,comm),amounts:p.payload.readback.prospectiveAmounts}
  }
  pure def proof(p: Package,i: FullInput): Proof = {final:finalReadback(p,i),operation:{ir:p.payload.ir,vk:(if(p.payload.authorityVersion == ConsumingV2) 400 else 200) + p.payload.context.selectedAction,policy:policy,signed:true},attachment:{payload:p.payload,signature:i.signature},genuineV3:true}
  pure def initialFor(a: NonceState,c: Case): State = {
    val q = {...request(c),predecessor:a.head}
    val p = packageFor(a,c,q)
    val i = input(p,signature(0),c)
    {phase:Idle,instance:c,context:contextFor(a,c),request:q,keys:Map(),package:p,input:i,proof:proof(p,i),visited:Set(Idle),pkReads:0,expansions:0,faults:Set(),signatureChecks:acceptedPredicates,nativeResult:ExternalRejected,nonce:a}
  }
  pure def initial(c: Case): State = initialFor(nonceInitial(LegacyV1),c)
  pure def advance(s: State,p: Phase): State = {...s,phase:p,visited:s.visited.union(Set(p))}
  action init = state' = initial(A1)
  pure def canCompare(s: State,c: Case,k: KeyMap,result: ExternalCheck): bool = s.phase == Idle and (s.nonce.version == LegacyV1 or source(c) == source(s.instance)) and k == keyMapFor(s.nonce,c) and result == ExternalAccepted
  action compare(c: Case,k: KeyMap,result: ExternalCheck): bool = all {
    canCompare(state,c,k,result),state' = {...advance(initialFor(state.nonce,c),PublicMapCompared),keys:k}
  }
  pure def valueUint(v: Value): bool = match v { | UInt(n) => uint(n) | BoolValue(_) => true }
  pure def staticAdmission(c: Context,payers: Set[int]): bool = and {
    c.assets.length() <= 16,c.assets.foldl(Set(),(a,i) => a.union(Set(i))).size() == c.assets.length(),
    c.registry.keys().size() <= 16,c.dutyOrder.length() == c.registry.keys().size(),
    c.dutyOrder.foldl(Set(),(d,i) => d.union(Set(i))) == c.registry.keys(),payers.forall(i => i == c.policy.owner),
    c.registry.keys().forall(i => c.registry.get(i).debtor == c.policy.owner and c.assets.indices().exists(j => c.assets.nth(j) == c.registry.get(i).asset)),
  }
  pure def complete(c: Context,s: Snapshot): bool = if(s.duties.keys() != c.registry.keys()) false else and {
    s.values.keys() == c.stateKeys.foldl(Set(),(xs,i) => xs.union(Set(i))),s.values.keys().forall(i => valueUint(s.values.get(i))),
    s.balances.keys().forall(i => uint(s.balances.get(i))),
    s.duties.keys().forall(i => {
      val d = s.duties.get(i)
      val metadata = c.registry.get(i)
      uint(d.amount) and (if(d.present) d.asset == metadata.asset and d.debtor == metadata.debtor and d.creditor == metadata.creditor
        else d.amount == 0 and d.asset == 0 and d.debtor == 0 and d.creditor == 0)
    }),
  }
  pure def observations(s: Snapshot): Observations = {stateCount:s.values.keys().size(),dutyCount:s.duties.keys().filter(i => s.duties.get(i).present).size(),
    memberships:s.duties.keys().mapBy(i => s.duties.get(i).present),reads:s.duties}
  // Size/member/read evidence agrees separately; this models the necessary
  // native link, rather than proving SDK wire soundness from a host frame.
  pure def observed(c: Context,s: Snapshot,o: Observations): bool = if(not(complete(c,s))) false else and {
    o.stateCount == c.stateKeys.length(),o.memberships.keys() == c.registry.keys(),
    o.reads == s.duties,o.memberships == s.duties.keys().mapBy(i => s.duties.get(i).present),
    o.dutyCount == o.memberships.keys().fold(0,(n,i) => n + (if(o.memberships.get(i)) 1 else 0)),
  }
  pure def completeLimits(c: Context,l: Limits): bool = l.length() == c.assets.length()
    and l.indices().forall(i => l.nth(i).asset == c.assets.nth(i) and {
      val cap = l.nth(i).caps
      uint(cap.gross) and uint(cap.fee) and uint(cap.net) and uint(cap.duty)
    })
  pure def totals(c: Context,p: Projection,a: int): Totals = {
    val sums = p.effects.foldl({outgoing:0,fees:0,credits:0,debits:0},(s,e) => if(e.asset != a) s else {
      outgoing:s.outgoing + (if(e.kind == Transfer and e.payer == c.policy.owner) e.amount else 0),
      fees:s.fees + (if(e.kind == Fee and e.payer == c.policy.owner) e.amount else 0),
      credits:s.credits + (if(e.kind == Transfer and e.recipient == c.policy.beneficiary) e.amount else 0),
      debits:s.debits + (if(e.payer == c.policy.beneficiary and (e.kind == Transfer or (e.kind == Fee and c.policy.beneficiary != c.policy.owner))) e.amount else 0),
    })
    {outgoing:sums.outgoing,fees:sums.fees,credits:sums.credits,debits:sums.debits,gross:sums.outgoing + sums.fees,net:sums.credits - sums.debits - sums.fees,
      duty:c.registry.keys().fold(0,(n,i) => n + (if(c.registry.get(i).asset == a) p.post.duties.get(i).amount else 0))}
  }
  pure def checkedTotals(t: Totals): bool = uint(t.outgoing) and uint(t.fees) and uint(t.credits) and uint(t.debits) and uint(t.gross) and uint(t.net) and uint(t.duty)
  pure def economic(c: Context,p: Projection,l: Limits): bool = if(not(complete(c,p.pre) and complete(c,p.post) and completeLimits(c,l))) false else
    p.effects.foldl(true,(ok,e) => ok and uint(e.amount) and valueUint(e.before) and valueUint(e.after)) and l.indices().forall(i => {
      val t = totals(c,p,l.nth(i).asset)
      val cap = l.nth(i).caps
      checkedTotals(t) and t.gross <= cap.gross and t.fees <= cap.fee and t.net >= cap.net and t.duty <= cap.duty
    })
  pure def admittedRequest(c: Case,q: Request): bool = q.inputs == request(c).inputs and completeLimits(context(c),q.limits)
    and uint(q.nonce) and uint(q.start) and uint(q.end) and uint(q.at) and q.start <= q.at and q.at <= q.end
  pure def canPrepare(s: State,p: Package,q: Request,oPre: Observations,oPost: Observations,result: ExternalCheck): bool = and {
    s.phase == PublicMapCompared,s.keys == keyMapFor(s.nonce,s.instance),staticAdmission(s.context,Set(1)),admittedRequest(s.instance,q),
    p == packageFor(s.nonce,s.instance,q),authorityBound(s,p,q),observed(s.context,p.payload.projection.pre,oPre),observed(s.context,p.payload.projection.post,oPost),result == ExternalAccepted,
  }
  action prepare(p: Package,q: Request,oPre: Observations,oPost: Observations,result: ExternalCheck): bool = all {
    canPrepare(state,p,q,oPre,oPost,result),state' = {...advance(state,SigningPrepared),package:p,request:q,nonce:if(state.nonce.version == ConsumingV2) {...state.nonce,packages:state.nonce.packages.put(q.nonce,p.payload.authority)} else state.nonce}
  }
  pure def allPredicates(p: Predicates): bool = p == acceptedPredicates
  pure def canAttach(s: State,a: Attachment,i: FullInput,p: Predicates): bool = and {
    s.phase == SigningPrepared,authorityBound(s,s.package,s.request),a.payload == s.package.payload,a.signature.pk == s.context.policy.pk,
    i == input(s.package,a.signature,s.instance),allPredicates(p),economic(s.context,s.package.payload.projection,s.request.limits),
  }
  action attach(a: Attachment,i: FullInput,p: Predicates): bool = all {
    canAttach(state,a,i,p),state' = {...advance(state,Attached),input:i,signatureChecks:p}
  }
  // Consumes an EXTERNAL genuine result; no native prover executes in Quint.
  pure def canReportNative(s: State,candidate: Proof,result: ExternalCheck): bool = and {
    s.phase == Attached,authorityBound(s,s.package,s.request),s.expansions == 0,candidate == proof(s.package,s.input),
    candidate.operation == s.keys.get(s.context.selectedAction),result == ExternalAccepted,
  }
  action reportNative(candidate: Proof,result: ExternalCheck): bool = all {
    canReportNative(state,candidate,result),state' = {...advance(state,NativeResultReported),proof:candidate,expansions:1,nativeResult:result}
  }
  pure def canColdVerify(s: State,p: Package,a: Attachment,k: KeyMap,candidate: Proof,pkReads: int,result: ExternalCheck): bool = and {
    s.phase == NativeResultReported,pkReads == 0,k == keyMapFor(s.nonce,s.instance),p == packageFor(s.nonce,s.instance,s.request),authorityBound(s,p,s.request),
    a.payload == p.payload,a.signature.pk == s.context.policy.pk,allPredicates(s.signatureChecks),
    candidate == proof(p,input(p,a.signature,s.instance)),candidate == s.proof,
    candidate.operation == k.get(s.context.selectedAction),result == ExternalAccepted,
  }
  action coldVerify(p: Package,a: Attachment,k: KeyMap,candidate: Proof,pkReads: int,result: ExternalCheck): bool = all {
    canColdVerify(state,p,a,k,candidate,pkReads,result),state' = {...advance(state,ColdVerified),pkReads:pkReads}
  }
  pure def distinguishingFault(s: State,f: int): bool = if(s.phase == Idle) f == 0 and
    not(canCompare(s,s.instance,keyMapFor(s.nonce,s.instance).put(if(source(s.instance) == Metered) 2 else 4,{ir:999,vk:999,policy:policy,signed:true}),ExternalAccepted))
    else if(s.phase == PublicMapCompared) f == 1 and not(canPrepare(s,{...s.package,payload:{...s.package.payload,context:{...s.context,network:999}}},s.request,observations(pre(s.instance)),observations(post(s.instance)),ExternalAccepted))
    else if(s.phase == SigningPrepared) f == 2 and not(canAttach(s,attachment(s.package,0),{...input(s.package,signature(0),s.instance),layout:{...layout(s.instance),authImpacts:1}},acceptedPredicates))
    else if(s.phase == Attached) f == 3 and not(canReportNative(s,{...proof(s.package,s.input),genuineV3:false},ExternalAccepted))
    else if(s.phase == NativeResultReported) f == 4 and not(canColdVerify(s,s.package,attachment(s.package,1 - (s.input.signature.r - 71)),s.keys,s.proof,0,ExternalAccepted)) else false
  action refuse(f: int): bool = all {
    distinguishingFault(state,f),state' = {...advance(state,Refused),faults:state.faults.union(Set(f))}
  }
  action step = {
    nondet c = Set(A1,A2,Freeze,B1,B2,Reset).oneOf()
    nondet sigChoice = Set(0,1).oneOf()
    nondet faultChance = 0.to(9).oneOf()
    val a = attachment(state.package,sigChoice)
    any {
      compare(c,keyMapFor(state.nonce,c),ExternalAccepted),
      prepare(state.package,state.request,observations(pre(state.instance)),observations(post(state.instance)),ExternalAccepted),
      attach(a,input(state.package,a.signature,state.instance),acceptedPredicates),reportNative(proof(state.package,state.input),ExternalAccepted),
      coldVerify(state.package,{payload:state.package.payload,signature:state.input.signature},state.keys,state.proof,0,ExternalAccepted),
      all {faultChance == 1,refuse(if(state.phase == Idle) 0 else if(state.phase == PublicMapCompared) 1 else if(state.phase == SigningPrepared) 2 else if(state.phase == Attached) 3 else 4)},
    }
  }
  action initNonce = state' = initialFor(nonceInitial(ConsumingV2),A1)
  action prepareNonceAction(c: Case,n: int,budgets: bool): bool = {
    val base = initialFor(state.nonce,c)
    val q = {...base.request,nonce:n}
    val p = packageFor(state.nonce,c,q)
    val compared = {...advance(base,PublicMapCompared),keys:keyMapFor(state.nonce,c)}
    all {
      source(c) == source(state.instance),nonceCanPrepare(state.nonce,n,budgets),
      canPrepare(compared,p,q,observations(pre(c)),observations(post(c)),ExternalAccepted),
      state' = {...advance(compared,SigningPrepared),package:p,request:q,
        nonce:{...state.nonce,packages:state.nonce.packages.put(n,p.payload.authority)}}
    }
  }
  action prepareNonce(n: int,budgets: bool): bool = prepareNonceAction(A1,n,budgets)
  action refuseNonce(n: int,budgets: bool): bool = all {not(nonceCanPrepare(state.nonce,n,budgets)),state' = state}
  // Explicit supplied-frame adoption; never a native proof or ledger apply.
  action deriveNonceFrame(n: int): bool = {
    val successor = {...state.nonce,head:state.nonce.head+1,consumed:state.nonce.consumed.union(Set(n))}
    all {
      state.nonce.packages.keys().contains(n),nonceTranscriptOnCurrent(state.nonce,state.nonce.packages.get(n)),
      state' = initialFor(successor,state.instance)
    }
  }
  // The existing compare/sign/attach/external-native/cold pipeline is sampled in v2.
  action stepConsuming = {
    nondet c = Set(A1,A2,Freeze).oneOf()
    nondet n = 0.to(7).oneOf()
    val base = initialFor(state.nonce,c)
    val q = {...base.request,nonce:n}
    any {
      step,
      all {canCompare(state,c,keyMapFor(state.nonce,c),ExternalAccepted),
        state' = {...advance(base,PublicMapCompared),keys:keyMapFor(state.nonce,c),request:q,package:packageFor(state.nonce,c,q)}},
      all {state.phase == ColdVerified,deriveNonceFrame(state.request.nonce)}
    }
  }
  pure def nonceSafety(a: NonceState): bool = a.consumed.size()+1 <= authorizationCapacity and not(a.authenticAcceptance)
    and a.packages.keys().forall(n => {
      val p = a.packages.get(n)
      p.nonce == n and p.postCount == p.preCount+1 and p.preCount < authorizationCapacity and not(p.preMember) and p.postMember
    })
  val comparedWitness = state.visited.contains(PublicMapCompared)
  val preparedWitness = state.visited.contains(SigningPrepared)
  val attachedWitness = state.visited.contains(Attached)
  val nativeResultWitness = state.visited.contains(NativeResultReported)
  val coldVerifiedWitness = state.visited.contains(ColdVerified)
  val refusedWitness = state.phase == Refused
  val alternateAttachmentWitness = coldVerifiedWitness and state.input.signature == signature(1)
  val freezeWitness = coldVerifiedWitness and state.instance == Freeze
  val stateOnlyWitness = coldVerifiedWitness and source(state.instance) == Toggle
  val sourceMapInvariant = state.visited.contains(PublicMapCompared) implies state.keys == keyMapFor(state.nonce,state.instance)
  val signingInvariant = state.visited.contains(SigningPrepared) implies state.package == packageFor(state.nonce,state.instance,state.request) and authorityBound(state,state.package,state.request)
  val sharedSemanticWires = state.visited.contains(Attached) implies and {
    state.input.payload == state.package.payload,state.input.layout == layout(state.instance),state.package.payload.wires == (if(state.nonce.version == LegacyV1) wires(state.context,state.package.payload.projection) else authorityWires(state.context,state.package.payload.projection,state.package.payload.authority)),
    economic(state.context,state.package.payload.projection,state.request.limits),allPredicates(state.signatureChecks),
  }
  val untouchedDutyPreservation = state.visited.contains(SigningPrepared) implies
    state.context.registry.keys().filter(i => not(state.package.payload.projection.effects.foldl(false,(found,e) => found or ((e.kind == Liability or e.kind == Discharge) and e.name == i)))).forall(i =>
      state.package.payload.projection.pre.duties.get(i) == state.package.payload.projection.post.duties.get(i))
  val attachmentSpecificFinal = state.visited.contains(NativeResultReported) implies and {
    state.expansions == 1,state.proof == proof(state.package,state.input),state.nativeResult == ExternalAccepted,state.proof.final.pi.communication.input.signature == state.input.signature,
  }
  val publicOnlyColdVerification = coldVerifiedWitness implies state.pkReads == 0 and state.proof.operation == state.keys.get(state.context.selectedAction)
  val safety = nonceSafety(state.nonce) and sourceMapInvariant and signingInvariant and sharedSemanticWires and untouchedDutyPreservation and attachmentSpecificFinal and publicOnlyColdVerification
}
