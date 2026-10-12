// Source correspondence: mori-ledger prepare.rs (typed raw owner), proving.rs
// (source constructor / check / resolver / split / binding / Intent::prove /
// exact final statement / ProofMarker Real verification), relation/mod.rs.
// One caller, immutable shared snapshot; no messages, retries or network.
// ASSUMPTIONS: original-source compilation and SDK keygen produce sourceKey;
// Separately assumed: independently trusted correct/compatible SRS, correct SDK
// keygen and compatibility with ledger PARAMS_VERIFIER. Degree/type alone does
// not establish any of these; none is validated by this model or a Boolean flag.
// private constructors and immutable Arc snapshots cannot be forged or mutated.
// SDK supplies final binding and processes raw Ops once; independent checked
// preparation + ChargedState VM supplies expected transcript/state/effects/gas/PI.
// CryptoResult denotes an external SDK result, never a modeled proof checker.
// Opaque IDs below are oracle symbols, not cryptographic computations or measured
// native quantities. A/B financial effects, running, charge, balances and events are literals.
// This models necessary trust-boundary checks; it proves no SDK, crypto, native
// fit, installed authority, signature, funding, application or settlement claim.
// Update this model when the listed source contract or external premises change.
module source_call_proving {
  type Phase = Idle | Prepared | Derived | Checked | Resolved | Split | Proven | Verified | Refused
  type Case = A | B
  type Origin = SourceRecompiled | ImportedClaim
  type ProofMode = Real | Mock
  type Marker = ProofMarker | PreimageMarker | ErasedMarker
  type Feature = Verifying | CompiledOut
  type Binding = Missing | Supplied(int)
  type CryptoResult = VerifierReturnedSuccess | VerifierReturnedFailure
  type Identity = { source: int, ir: int, manifest: int, closure: int,
    entrypoint: int, pk: int, vk: int, degree: int, origin: Origin }
  type Operation = { ir: int, vk: int, legacyKeyPresent: bool }
  type Effect = StateChange({name:int,before:int,after:int})
    | TransferEffect({asset:int,from:int,dst:int,amount:int})
    | FeeEffect({asset:int,from:int,dst:int,amount:int})
    | DutyChange({name:int,asset:int,debtor:int,creditor:int,before:int,after:int})
    | UIntEmit({name:int,value:int}) | BoolEmit({name:int,value:bool})
  type Envelope = { offers: int, ttl: int, randomness: int, actions: List[int] }
  type Statement = { parent: int, binding: int, communication: int, pi: List[int],
    transcript: List[int], effects: List[Effect], running: int, charge: int,
    balances: List[int], chargedState: int, events: List[int],
    measuredGas: int, declaredGas: int, envelope: Envelope, version: int }
  type State = { phase: Phase, instance: Case, source: int, key: Identity,
    checkedKey: Identity, resolvedKey: Identity, splitKey: Identity,
    rawOwner: int, expansions: int, statement: Statement, verifiedStatement: Statement,
    visited: Set[Phase], changedUntrustedArtifact: bool,
    marker: Marker, mode: ProofMode, feature: Feature, cryptoResult: CryptoResult }
  var state: State
  pure val sourceKey: Identity = {source:11,ir:21,manifest:31,closure:41,
    entrypoint:51,pk:61,vk:71,degree:17,origin:SourceRecompiled}
  pure val envelope: Envelope = {offers:81,ttl:91,randomness:101,actions:List(51)}
  // Transcript/PI/gas/state IDs are independent SDK/VM oracle premises.
  // Event sequence order matters; identity fields are opaque.
  pure def expected(c: Case): Statement = if(c == A)
    {parent:111,binding:121,communication:131,pi:List(121,131,141),
     transcript:List(151,0,152),effects:List(StateChange({name:205,before:4,after:11}),TransferEffect({asset:201,from:202,dst:203,amount:7}),TransferEffect({asset:201,from:202,dst:203,amount:7}),FeeEffect({asset:201,from:202,dst:204,amount:2}),DutyChange({name:206,asset:201,debtor:202,creditor:203,before:0,after:7}),DutyChange({name:206,asset:201,debtor:202,creditor:203,before:7,after:4}),UIntEmit({name:207,value:11}),BoolEmit({name:208,value:true})),running:11,
     charge:4,balances:List(84,19,3),chargedState:161,events:List(11,1),
     measuredGas:171,declaredGas:181,envelope:envelope,version:3}
    else {parent:112,binding:122,communication:132,pi:List(122,132,142),
     transcript:List(151,0,0,152),effects:List(StateChange({name:205,before:31,after:34}),TransferEffect({asset:201,from:202,dst:203,amount:3}),TransferEffect({asset:201,from:202,dst:203,amount:3}),FeeEffect({asset:201,from:202,dst:204,amount:2}),DutyChange({name:206,asset:201,debtor:202,creditor:203,before:9,after:12}),DutyChange({name:206,asset:201,debtor:202,creditor:203,before:12,after:9}),UIntEmit({name:207,value:34}),BoolEmit({name:208,value:true})),running:34,
     charge:9,balances:List(92,11,3),chargedState:162,events:List(34,1),
     measuredGas:172,declaredGas:182,envelope:envelope,version:3}
  pure val initial: State = {phase:Idle,instance:A,source:11,key:sourceKey,
    checkedKey:sourceKey,resolvedKey:sourceKey,splitKey:sourceKey,
    rawOwner:0,expansions:0,statement:expected(A),verifiedStatement:expected(A),
    visited:Set(Idle),changedUntrustedArtifact:false,marker:ErasedMarker,
    mode:Mock,feature:CompiledOut,cryptoResult:VerifierReturnedFailure}
  pure def advance(s: State,p: Phase): State = {...s,phase:p,visited:s.visited.union(Set(p))}
  pure def canPrepare(s: State,source: int): bool = s.phase == Idle and source == 11
  pure def applyPrepare(s: State,c: Case): State =
    {...advance(s,Prepared),instance:c,rawOwner:1,statement:expected(c)}
  action init = state' = initial
  action prepare(c: Case,source: int): bool = all {
    canPrepare(state,source),state' = applyPrepare(state,c)
  }
  // Original source drives compilation; no mutable artifact/manifest input exists.
  pure def canDerive(s: State,originalSource: int,origin: Origin): bool =
    s.phase == Prepared and originalSource == s.source and origin == SourceRecompiled
  pure def applyDerive(s: State): State = {...advance(s,Derived),key:sourceKey}
  action derive(originalSource: int,origin: Origin): bool = all {
    canDerive(state,originalSource,origin),state' = applyDerive(state)
  }
  pure def canChangeClaim(s: State): bool = s.phase == Derived and not(s.changedUntrustedArtifact)
  action changeUntrustedArtifact = all {
    canChangeClaim(state),state' = {...state,changedUntrustedArtifact:true}
  }
  pure def canCheck(s: State,location: int): bool =
    s.phase == Derived and location == s.key.entrypoint and s.key == sourceKey
  pure def applyCheck(s: State): State = {...advance(s,Checked),checkedKey:s.key}
  action check(location: int): bool = all {canCheck(state,location),state' = applyCheck(state)}
  pure def canResolve(s: State,location: int): bool = s.phase == Checked and location == s.key.entrypoint
  pure def applyResolve(s: State): State = {...advance(s,Resolved),resolvedKey:s.key}
  action resolve(location: int): bool = all {canResolve(state,location),state' = applyResolve(state)}
  pure def canSplit(s: State): bool = s.phase == Resolved
  pure def applySplit(s: State): State = {...advance(s,Split),splitKey:s.key}
  action split = all {canSplit(state),state' = applySplit(state)}
  pure def bindingPresent(b: Binding): bool = match b {
    | Missing => false
    | Supplied(_) => true
  }
  pure def canSdkProve(s: State,rawOwner: int,expansions: int,b: Binding): bool = and {
    s.phase == Split,s.rawOwner == rawOwner,rawOwner == 1,expansions == 0,
    s.checkedKey == s.resolvedKey,s.resolvedKey == s.splitKey,
    bindingPresent(b),
  }
  pure def overwriteBinding(st: Statement,b: Binding): Statement = match b {
    | Missing => st
    | Supplied(value) => {...st,binding:value,pi:st.pi.replaceAt(0,value)}
  }
  pure def applySdkProve(s: State,b: Binding): State =
    {...advance(s,Proven),statement:overwriteBinding(expected(s.instance),b),expansions:1}
  action sdkProve(rawOwner: int,expansions: int,b: Binding): bool = all {
    canSdkProve(state,rawOwner,expansions,b),state' = applySdkProve(state,b)
  }
  pure def exactOperation(s: State,operation: Operation): bool =
    operation.ir == s.key.ir and operation.vk == s.key.vk and not(operation.legacyKeyPresent)
  // Selection + exact statement checks are necessary conditions only. External
  // VerifierReturnedSuccess is not evidence that this model executes native crypto.
  pure def canVerify(s: State,candidate: Statement,operation: Operation,
    marker: Marker,mode: ProofMode,feature: Feature,result: CryptoResult): bool = and {
    s.phase == Proven,exactOperation(s,operation),candidate == expected(s.instance),
    candidate == s.statement,marker == ProofMarker,mode == Real,feature == Verifying,
    result == VerifierReturnedSuccess,
  }
  pure def applyVerify(s: State,candidate: Statement): State =
    {...advance(s,Verified),verifiedStatement:candidate,marker:ProofMarker,
     mode:Real,feature:Verifying,cryptoResult:VerifierReturnedSuccess}
  action verify(candidate: Statement,operation: Operation,marker: Marker,
    mode: ProofMode,feature: Feature,result: CryptoResult): bool = all {
    canVerify(state,candidate,operation,marker,mode,feature,result),state' = applyVerify(state,candidate)
  }
  pure def canRefuse(s: State): bool = Set(Prepared,Derived,Checked,Split,Proven).contains(s.phase)
  pure def distinguishingFault(s: State): bool = match s.phase {
    | Prepared => not(canDerive(s,12,ImportedClaim))
    | Derived => not(canCheck(s,52))
    | Checked => not(canResolve(s,52))
    | Split => not(canSdkProve(s,1,1,Missing))
    | Proven => not(canVerify(s,{...s.statement,parent:999},{ir:21,vk:71,legacyKeyPresent:false},
                             ProofMarker,Real,Verifying,VerifierReturnedSuccess))
    | _ => false
  }
  // Observed error; successful transitions remain disabled for invalid proposals.
  action refuse = all {
    canRefuse(state),distinguishingFault(state),state' = advance(state,Refused)
  }
  action step = {
    nondet c = Set(A,B).oneOf()
    nondet fault = 0.to(19).oneOf()
    any {
      prepare(c,11),derive(11,SourceRecompiled),changeUntrustedArtifact,
      check(51),resolve(51),split,
      sdkProve(1,0,Supplied(expected(state.instance).binding)),
      verify(state.statement,{ir:21,vk:71,legacyKeyPresent:false},ProofMarker,Real,Verifying,VerifierReturnedSuccess),
      all {fault == 1,refuse},
    }
  }
  val preparedWitness = state.visited.contains(Prepared)
  val derivedWitness = state.visited.contains(Derived)
  val checkedWitness = state.visited.contains(Checked)
  val resolvedWitness = state.visited.contains(Resolved)
  val splitWitness = state.visited.contains(Split)
  val provenWitness = state.visited.contains(Proven)
  val verifiedWitness = state.visited.contains(Verified)
  val refusedWitness = state.visited.contains(Refused)
  val changedClaimWitness = state.changedUntrustedArtifact
  val verifiedAWitness = state.phase == Verified and state.instance == A
  val verifiedBWitness = state.phase == Verified and state.instance == B
  val rawOwnership = state.visited.contains(Prepared) implies state.rawOwner == 1
  val snapshotIdentity = and {
    state.visited.contains(Derived) implies state.key == sourceKey,
    state.visited.contains(Checked) implies state.checkedKey == state.key,
    state.visited.contains(Resolved) implies state.resolvedKey == state.checkedKey,
    state.visited.contains(Split) implies state.splitKey == state.resolvedKey,
  }
  val singleExpansion = state.visited.contains(Proven) implies state.expansions == 1
  val finalStatementExactness = state.phase == Verified implies and {
    state.verifiedStatement == expected(state.instance),
    state.verifiedStatement.parent == state.statement.parent,
    state.verifiedStatement.pi == state.statement.pi,
    state.verifiedStatement.effects.length() == 8,
    state.verifiedStatement.balances == (if(state.instance == A) List(84,19,3) else List(92,11,3)),
    state.marker == ProofMarker,state.mode == Real,state.feature == Verifying,
    state.cryptoResult == VerifierReturnedSuccess,
  }
}
