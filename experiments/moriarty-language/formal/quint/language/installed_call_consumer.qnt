// Installed local consumer boundary QA. One caller owns immutable snapshots;
// there are no distributed actors/messages, so this is plain Quint.
// EXTERNAL: source compiler/SDK VM/codec correctness, hash collision resistance,
// SRS pin/setup, deterministic native derivation/export, real crypto and ledger
// verifier compatibility, physical discovery/keygen work/fit, authenticated chain
// state/time/authority/funding/signatures/history/settlement. Nothing here computes
// or certifies those premises. External event tags are stipulated observations,
// never caller Boolean/hash certificates or authorization to execute native work.
// In particular optimal_k scans 9..=25 before a caller max_degree can refuse its
// discovered result; keygen/keygen_vk repeat k(). Physical costs remain external.
// Key/degree/SDK state/PI/gas/hash IDs are symbolic, not measured native results.
// The literal semantic states/effects are independently supplied QA expectations.
module installed_call_consumer {
  type Purpose = Keygen | ColdProve | ColdVerify
  type Source = Metered | Toggle
  type Case = A1 | A2 | B1 | B2 | FreezeCase | ResetCase | BadAmount | Disabled | Overflow
  type Entry = Settle | Freeze | Step | Reset
  type Phase = Idle | Preflight | Discovered | Regenerated | Compared | Installed
    | Prepared | FinalCompared | ExternalResultChecked | LeavesWritten | Published | Refused
  type Profile = LegacyKeyless | InstalledLocal | WrongProfile
  type DerivationPurpose = PublicOnly | SelectedFull
  type Observation = ExternalSourceDerivation | ImportedClaimsOnly | ExternalFailure
  type Discovery = ExternalSdkDiscovery | CallerClaimedDegree
  type Marker = ProofMarker | PreimageMarker | ErasedMarker
  type Mode = Real | Mock
  type Feature = Verifying | CompiledOut
  type Crypto = ExternalRealSuccess | ExternalRealFailure
  type Fault = SeedInstalled | MetadataMismatch | BadImportPurpose | EarlyKeyInit
    | WeakImportedKey | UnusedActionKey | WrongStoredOperation | FinalMutation | Collision | PartialWrite
  type Value = UIntValue(int) | BoolValue(bool)
  type Asset = { network: str, issuer: str, reference: str, scale: int }
  type Balance = { owner: str, asset: Asset, amount: int }
  type Duty = { name: str, asset: Asset, debtor: str, creditor: str, present: bool, amount: int }
  type Effect = StateEffect({name: str, before: Value, after: Value})
    | TransferEffect({asset: Asset, from: str, dst: str, amount: int})
    | FeeEffect({asset: Asset, from: str, dst: str, amount: int})
    | DutyEffect({name: str, asset: Asset, debtor: str, creditor: str, before: int, after: int})
    | EmitEffect({name: str, value: Value})
  type Snapshot = { values: str -> Value, balances: List[Balance], duties: List[Duty] }
  // These separate frame fields stand for complete admitted native structures.
  // Equality models preservation, not real-world authentication of their bytes.
  type Frame = { data: int, maintenance: int, contractBalance: int, utxos: List[int],
    extensions: List[int], bookkeeping: List[int], ledgerParameters: int, network: int, nightSupply: int }
  type Operation = { ir: int, vk: int, legacyVk: int }
  type Key = { ir: int, vk: int, manifest: int, degree: int, parameters: int }
  type Metadata = { source: Source, staticBinding: int, build: int, profile: Profile,
    parameterProfile: int, qualification: int, anchor: int, phasePolicy: int }
  type Manifest = { metadata: Metadata, names: List[Entry], keys: Entry -> Key,
    pkLeaf: int, candidateExpectedHash: int }
  type Limits = { degree: int, actions: int, srsBytes: int, keyBytes: int,
    aggregateBytes: int, inputBytes: int }
  type Envelope = { offers: List[int], ttl: int, randomness: int, actions: List[Entry],
    extraActions: List[int] }
  type Output = { owner: str, asset: Asset, amount: int, finalIntent: int, ordinal: int }
  type Statement = { post: Snapshot, effects: List[Effect], events: List[(str, Value)],
    parent: int, binding: int, communication: int, pi: List[int], transcript: List[int],
    chargedState: int, measuredGas: int, declaredGas: int, envelope: Envelope,
    finalIntent: int, outputs: List[Output], version: int }
  type State = { phase: Phase, purpose: Purpose, instance: Case, source: Source,
    limits: Limits, seedOps: Entry -> Operation, installedOps: Entry -> Operation,
    seed: Snapshot, frame: Frame, admittedFrame: Frame, manifest: Manifest,
    degrees: Entry -> int, derivedKeys: Entry -> Key, selectedPk: int,
    derivationPurpose: DerivationPurpose, observation: Observation,
    importMatched: bool, pkReads: int, importedKeyInitializations: int,
    selectedOperation: Operation, snapshotCommitment: (Frame, Entry -> Operation),
    descriptorCommitment: (Metadata, (Frame, Entry -> Operation)),rawOwner: int, expansions: int,
    statement: Statement, checkedStatement: Statement, leaves: Set[int],
    markerWritten: bool, existingFiles: Set[int], visited: Set[Phase], faults: Set[Fault],
    crypto: Crypto, marker: Marker, mode: Mode, feature: Feature }
  var state: State
  pure val qa: Asset = {network:"local",issuer:"next-native-qa",reference:"QA",scale:0}
  pure val frame: Frame = {data:1,maintenance:2,contractBalance:3,utxos:List(4,5),
    extensions:List(6,7),bookkeeping:List(8,9),ledgerParameters:10,network:11,nightSupply:12}
  pure val limits: Limits = {degree:19,actions:2,srsBytes:100,keyBytes:100,aggregateBytes:1000,inputBytes:100}
  pure def sourceOf(c: Case): Source = if(Set(A1,A2,FreezeCase,BadAmount).contains(c)) Metered else Toggle
  pure def selected(c: Case): Entry = match c {
    | FreezeCase => Freeze | ResetCase => Reset
    | A1 => Settle | A2 => Settle | BadAmount => Settle | _ => Step
  }
  pure def names(src: Source): List[Entry] = if(src == Metered) List(Settle,Freeze) else List(Step,Reset)
  pure def sourceEntries(src: Source): Set[Entry] = if(src == Metered) Set(Settle,Freeze) else Set(Step,Reset)
  pure def metadata(src: Source): Metadata = {source:src,staticBinding:21,build:22,
    profile:InstalledLocal,parameterProfile:23,qualification:24,anchor:25,phasePolicy:26}
  pure def key(entry: Entry): Key = match entry {
    | Settle => {ir:101,vk:201,manifest:301,degree:17,parameters:1017}
    | Freeze => {ir:102,vk:202,manifest:302,degree:16,parameters:1016}
    | Step => {ir:103,vk:203,manifest:303,degree:17,parameters:1017}
    | Reset => {ir:104,vk:204,manifest:304,degree:16,parameters:1016}
  }
  pure def sourceKeys(src: Source): Entry -> Key = sourceEntries(src).mapBy(e => key(e))
  pure def operation(k: Key): Operation = {ir:k.ir,vk:k.vk,legacyVk:0}
  pure def sourceOperations(src: Source): Entry -> Operation = sourceEntries(src).mapBy(e => operation(key(e)))
  pure def keyless(src: Source): Entry -> Operation = sourceEntries(src).mapBy(_ => {ir:0,vk:0,legacyVk:0})
  pure def pk(e: Entry): int = match e { | Settle => 501 | Freeze => 502 | Step => 503 | Reset => 504 }
  pure def exported(c: Case): Manifest = {metadata:metadata(sourceOf(c)),names:names(sourceOf(c)),
    keys:sourceKeys(sourceOf(c)),pkLeaf:pk(selected(c)),candidateExpectedHash:999}
  pure def balances(a: int,b: int,t: int): List[Balance] = List(
    {owner:"alice",asset:qa,amount:a},{owner:"bob",asset:qa,amount:b},{owner:"treasury",asset:qa,amount:t})
  pure def duty(n: int): Duty = {name:"charge",asset:qa,debtor:"alice",creditor:"bob",present:true,amount:n}
  pure def seed(c: Case): Snapshot = match c {
    | A2 => {values:Map("running" -> UIntValue(31),"enabled" -> BoolValue(true)),balances:balances(100,5,1),duties:List(duty(9))}
    | FreezeCase => {values:Map("running" -> UIntValue(4),"enabled" -> BoolValue(true)),balances:balances(100,5,1),duties:List(duty(9))}
    | A1 => {values:Map("running" -> UIntValue(4),"enabled" -> BoolValue(true)),balances:balances(100,5,1),duties:List()}
    | BadAmount => {values:Map("running" -> UIntValue(4),"enabled" -> BoolValue(true)),balances:balances(100,5,1),duties:List()}
    | B1 => {values:Map("count" -> UIntValue(31),"active" -> BoolValue(true)),balances:List(),duties:List()}
    | B2 => {values:Map("count" -> UIntValue(255),"active" -> BoolValue(true)),balances:List(),duties:List()}
    | ResetCase => {values:Map("count" -> UIntValue(256),"active" -> BoolValue(false)),balances:List(),duties:List()}
    | Disabled => {values:Map("count" -> UIntValue(31),"active" -> BoolValue(false)),balances:List(),duties:List()}
    | Overflow => {values:Map("count" -> UIntValue(340282366920938463463374607431768211455),"active" -> BoolValue(true)),balances:List(),duties:List()}
  }
  pure def validRequest(c: Case): bool = not(Set(BadAmount,Disabled,Overflow).contains(c))
  pure def post(c: Case): Snapshot = match c {
    | A1 => {values:Map("running" -> UIntValue(11),"enabled" -> BoolValue(true)),balances:balances(84,19,3),duties:List(duty(4))}
    | A2 => {values:Map("running" -> UIntValue(34),"enabled" -> BoolValue(true)),balances:balances(92,11,3),duties:List(duty(9))}
    | FreezeCase => {...seed(c),values:Map("running" -> UIntValue(4),"enabled" -> BoolValue(false))}
    | B1 => {values:Map("count" -> UIntValue(32),"active" -> BoolValue(false)),balances:List(),duties:List()}
    | B2 => {values:Map("count" -> UIntValue(256),"active" -> BoolValue(false)),balances:List(),duties:List()}
    | ResetCase => {values:Map("count" -> UIntValue(0),"active" -> BoolValue(true)),balances:List(),duties:List()}
    | _ => seed(c)
  }
  pure def effects(c: Case): List[Effect] = match c {
    | A1 => List(StateEffect({name:"running",before:UIntValue(4),after:UIntValue(11)}),
      TransferEffect({asset:qa,from:"alice",dst:"bob",amount:7}),TransferEffect({asset:qa,from:"alice",dst:"bob",amount:7}),
      FeeEffect({asset:qa,from:"alice",dst:"treasury",amount:2}),
      DutyEffect({name:"charge",asset:qa,debtor:"alice",creditor:"bob",before:0,after:7}),
      DutyEffect({name:"charge",asset:qa,debtor:"alice",creditor:"bob",before:7,after:4}),
      EmitEffect({name:"total",value:UIntValue(11)}),EmitEffect({name:"enabled",value:BoolValue(true)}))
    | A2 => List(StateEffect({name:"running",before:UIntValue(31),after:UIntValue(34)}),
      TransferEffect({asset:qa,from:"alice",dst:"bob",amount:3}),TransferEffect({asset:qa,from:"alice",dst:"bob",amount:3}),
      FeeEffect({asset:qa,from:"alice",dst:"treasury",amount:2}),
      DutyEffect({name:"charge",asset:qa,debtor:"alice",creditor:"bob",before:9,after:12}),
      DutyEffect({name:"charge",asset:qa,debtor:"alice",creditor:"bob",before:12,after:9}),
      EmitEffect({name:"total",value:UIntValue(34)}),EmitEffect({name:"enabled",value:BoolValue(true)}))
    | FreezeCase => List(StateEffect({name:"enabled",before:BoolValue(true),after:BoolValue(false)}),EmitEffect({name:"enabled",value:BoolValue(false)}))
    | B1 => List(StateEffect({name:"count",before:UIntValue(31),after:UIntValue(32)}),StateEffect({name:"active",before:BoolValue(true),after:BoolValue(false)}),
      EmitEffect({name:"count",value:UIntValue(32)}),EmitEffect({name:"active",value:BoolValue(false)}))
    | B2 => List(StateEffect({name:"count",before:UIntValue(255),after:UIntValue(256)}),StateEffect({name:"active",before:BoolValue(true),after:BoolValue(false)}),
      EmitEffect({name:"count",value:UIntValue(256)}),EmitEffect({name:"active",value:BoolValue(false)}))
    | ResetCase => List(StateEffect({name:"count",before:UIntValue(256),after:UIntValue(0)}),StateEffect({name:"active",before:BoolValue(false),after:BoolValue(true)}),
      EmitEffect({name:"count",value:UIntValue(0)}))
    | _ => List()
  }
  pure def events(c: Case): List[(str,Value)] = match c {
    | A1 => List(("total",UIntValue(11)),("enabled",BoolValue(true)))
    | A2 => List(("total",UIntValue(34)),("enabled",BoolValue(true)))
    | FreezeCase => List(("enabled",BoolValue(false)))
    | B1 => List(("count",UIntValue(32)),("active",BoolValue(false)))
    | B2 => List(("count",UIntValue(256)),("active",BoolValue(false)))
    | ResetCase => List(("count",UIntValue(0))) | _ => List()
  }
  pure def outputs(c: Case): List[Output] = if(c == A1) List(
    {owner:"bob",asset:qa,amount:14,finalIntent:601,ordinal:0},
    {owner:"treasury",asset:qa,amount:2,finalIntent:601,ordinal:1},
    {owner:"alice",asset:qa,amount:84,finalIntent:601,ordinal:2})
    else if(c == A2) List({owner:"bob",asset:qa,amount:6,finalIntent:601,ordinal:0},
    {owner:"treasury",asset:qa,amount:2,finalIntent:601,ordinal:1},
    {owner:"alice",asset:qa,amount:92,finalIntent:601,ordinal:2}) else List()
  pure def expected(c: Case): Statement = {post:post(c),effects:effects(c),events:events(c),
    parent:602,binding:603,communication:604,pi:List(603,604,605),transcript:List(606,0,607),
    chargedState:608,measuredGas:609,declaredGas:610,
    envelope:{offers:if(Set(A1,A2).contains(c)) List(611,612) else List(),ttl:613,randomness:614,
      actions:List(selected(c)),extraActions:List()},finalIntent:601,outputs:outputs(c),version:3}
  pure val initial: State = {phase:Idle,purpose:ColdVerify,instance:A1,source:Metered,limits:limits,
    seedOps:keyless(Metered),installedOps:keyless(Metered),seed:seed(A1),frame:frame,admittedFrame:frame,
    manifest:exported(A1),degrees:Map(),derivedKeys:Map(),selectedPk:0,
    derivationPurpose:PublicOnly,observation:ImportedClaimsOnly,importMatched:false,pkReads:0,
    importedKeyInitializations:0,selectedOperation:{ir:0,vk:0,legacyVk:0},
    snapshotCommitment:(frame,keyless(Metered)),descriptorCommitment:(metadata(Metered),(frame,keyless(Metered))),rawOwner:0,expansions:0,
    statement:expected(A1),checkedStatement:expected(A1),leaves:Set(),markerWritten:false,
    existingFiles:Set(),visited:Set(Idle),faults:Set(),crypto:ExternalRealFailure,
    marker:ErasedMarker,mode:Mock,feature:CompiledOut}
  pure def advance(s: State,p: Phase): State = {...s,phase:p,visited:s.visited.union(Set(p))}
  action init = state' = initial
  pure def positiveLimits(l: Limits): bool = and {l.degree > 0,l.actions > 0,l.srsBytes > 0,
    l.keyBytes > 0,l.aggregateBytes > 0,l.inputBytes > 0}
  pure def uintAdmitted(v: Value): bool = match v {
    | UIntValue(n) => n >= 0 and n <= 340282366920938463463374607431768211455
    | _ => false
  }
  pure def boolAdmitted(v: Value): bool = match v { | BoolValue(_) => true | _ => false }
  // Request-free complete fixture seed shape: no transition/inputs/gas/results
  // enter this guard. Full concrete native decoding, ABI/FAB/role/UTXO/context,
  // ledger supply invariants and resource work remain separately executed Rust
  // checks; this finite model does not authenticate any supplied snapshot.
  pure def seedAdmitted(src: Source,snap: Snapshot): bool = if(src == Metered) and {
    snap.values.keys() == Set("running","enabled"),uintAdmitted(snap.values.get("running")),
    boolAdmitted(snap.values.get("enabled")),snap.balances.length() == 3,
    snap.balances.foldl(Set(),(owners,row) => owners.union(Set(row.owner))) == Set("alice","bob","treasury"),
    snap.balances.foldl(true,(ok,row) => ok and row.asset == qa and row.amount >= 0
      and row.amount <= 340282366920938463463374607431768211455),
    snap.duties.length() <= 1,snap.duties.foldl(true,(ok,row) => ok and row.name == "charge"
      and row.asset == qa and row.debtor == "alice" and row.creditor == "bob" and row.present
      and row.amount >= 0 and row.amount <= 340282366920938463463374607431768211455),
  } else and {snap.values.keys() == Set("count","active"),uintAdmitted(snap.values.get("count")),
    boolAdmitted(snap.values.get("active")),snap.balances == List(),snap.duties == List()}
  // Parameter IDs abbreviate a fixed SDK-pinned degree table, never candidate
  // digests or computed cryptographic certificates. Checking these symbols
  // cannot validate actual SRS contents, unchecked parsing, or trusted setup.
  pure def pinnedParameterMetadata(k: Key): bool = k.degree > 0 and k.degree <= 19
    and k.parameters == 1000 + k.degree
  pure def completeManifest(src: Source,m: Manifest): bool =
    m.metadata == metadata(src) and m.names.length() == sourceEntries(src).size()
    and m.names.foldl(Set(),(entries,e) => entries.union(Set(e))) == sourceEntries(src)
    and m.keys.keys() == sourceEntries(src)
    and m.keys.keys().forall(e => m.keys.get(e).ir == key(e).ir
      and m.keys.get(e).manifest == key(e).manifest and pinnedParameterMetadata(m.keys.get(e)))
  pure def keylessSeed(src: Source,ops: Entry -> Operation): bool = ops == keyless(src)
  pure def legacyAdmits(src: Source,p: Profile,ops: Entry -> Operation): bool =
    p == LegacyKeyless and keylessSeed(src,ops)
  pure def canPreflight(s: State,p: Purpose,c: Case,m: Manifest,ops: Entry -> Operation,l: Limits): bool = and {
    s.phase == Idle,completeManifest(sourceOf(c),m),positiveLimits(l),l.actions >= sourceEntries(sourceOf(c)).size(),
    // Keygen's complete keyless seed admission is genuinely request-free and
    // early. Cold CLI parses/reads DTOs/bytes here; complete actual predecessor
    // frame/operation admission is later, after native rederivation/comparison.
    // Neither weak VK correspondence nor cheap full cold frame checks are
    // claimed before SDK discovery/key work.
    if(p == Keygen) seedAdmitted(sourceOf(c),seed(c)) and s.frame == s.admittedFrame
      and keylessSeed(sourceOf(c),ops) else true,
  }
  pure def applyPreflight(s: State,p: Purpose,c: Case,m: Manifest,ops: Entry -> Operation,l: Limits): State =
    {...advance(s,Preflight),purpose:p,instance:c,source:sourceOf(c),manifest:m,seedOps:ops,
     installedOps:ops,seed:seed(c),limits:l,statement:expected(c),checkedStatement:expected(c)}
  action preflight(p: Purpose,c: Case,m: Manifest,ops: Entry -> Operation,l: Limits): bool = all {
    canPreflight(state,p,c,m,ops,l),state' = applyPreflight(state,p,c,m,ops,l)
  }
  pure def canDiscover(s: State,d: Entry -> int,o: Discovery): bool =
    s.phase == Preflight and d.keys() == sourceEntries(s.source) and o == ExternalSdkDiscovery
  action discover(d: Entry -> int,o: Discovery): bool = all {
    canDiscover(state,d,o),state' = {...advance(state,Discovered),degrees:d}
  }
  pure def discoveredWithinLimits(s: State): bool = sourceEntries(s.source).forall(e =>
    s.degrees.get(e) > 0 and s.degrees.get(e) <= s.limits.degree and s.degrees.get(e) <= 19)
  pure def requiredPurpose(p: Purpose): DerivationPurpose = if(p == ColdVerify) PublicOnly else SelectedFull
  pure def canRegenerate(s: State,p: DerivationPurpose,o: Observation): bool = and {
    s.phase == Discovered,discoveredWithinLimits(s),p == requiredPurpose(s.purpose),o == ExternalSourceDerivation,
  }
  pure def applyRegenerate(s: State,p: DerivationPurpose,o: Observation): State =
    {...advance(s,Regenerated),derivedKeys:sourceEntries(s.source).mapBy(e => {...key(e),degree:s.degrees.get(e)}),
     selectedPk:if(p == PublicOnly) 0 else pk(selected(s.instance)),derivationPurpose:p,observation:o}
  action regenerate(p: DerivationPurpose,o: Observation): bool = all {
    canRegenerate(state,p,o),state' = applyRegenerate(state,p,o)
  }
  pure def canCompare(s: State,m: Manifest,p: Purpose,keyInits: int): bool = and {
    s.phase == Regenerated,p == s.purpose,keyInits == 0,completeManifest(s.source,m),
    m.keys == s.derivedKeys,
    if(p == ColdVerify) true else m.pkLeaf == s.selectedPk,
  }
  pure def applyCompare(s: State,m: Manifest): State = {...advance(s,Compared),manifest:m,
    importMatched:true,pkReads:if(s.purpose == ColdProve) 1 else 0}
  action compare(m: Manifest,p: Purpose,keyInits: int): bool = all {
    canCompare(state,m,p,keyInits),state' = applyCompare(state,m)
  }
  pure def exactMap(s: State,ops: Entry -> Operation): bool =
    ops.keys() == s.derivedKeys.keys() and ops.keys().forall(e => ops.get(e) == operation(s.derivedKeys.get(e)))
  pure def canInstall(s: State,ops: Entry -> Operation,f: Frame): bool = and {
    s.phase == Compared,s.importMatched,exactMap(s,ops),f == s.frame,f == s.admittedFrame,
    seedAdmitted(s.source,s.seed),
    if(s.purpose == Keygen) keylessSeed(s.source,s.seedOps) else s.installedOps == ops,
  }
  action install(ops: Entry -> Operation,f: Frame): bool = all {
    canInstall(state,ops,f),state' = {...advance(state,Installed),installedOps:ops,frame:f,
      selectedOperation:ops.get(selected(state.instance)),snapshotCommitment:(f,ops),
      descriptorCommitment:(metadata(state.source),(f,ops))}
  }
  pure def canPrepare(s: State,rawOwner: int): bool = and {
    s.phase == Installed,s.purpose != Keygen,validRequest(s.instance),rawOwner == 1,
    s.selectedOperation == s.installedOps.get(selected(s.instance)),exactMap(s,s.installedOps),
  }
  action prepare(rawOwner: int): bool = all {
    canPrepare(state,rawOwner),state' = {...advance(state,Prepared),rawOwner:rawOwner}
  }
  pure def canFinal(s: State,st: Statement,op: Operation,rawOwner: int,expansions: int): bool = and {
    s.phase == Prepared,st == expected(s.instance),op == s.selectedOperation,
    op == s.installedOps.get(selected(s.instance)),rawOwner == s.rawOwner,rawOwner == 1,
    expansions == 0,s.expansions == 0,
  }
  // ColdVerify imports a candidate for independently prepared comparison; it
  // does not itself invoke Intent::prove. ColdProve expands its raw owner once.
  action finalCompare(st: Statement,op: Operation,rawOwner: int,expansions: int): bool = all {
    canFinal(state,st,op,rawOwner,expansions),state' = {...advance(state,FinalCompared),statement:st,
      checkedStatement:st,expansions:if(state.purpose == ColdProve) 1 else 0}
  }
  pure def canCheckExternalResult(s: State,m: Marker,mode: Mode,f: Feature,c: Crypto): bool = and {
    s.phase == FinalCompared,m == ProofMarker,mode == Real,f == Verifying,c == ExternalRealSuccess,
  }
  action checkExternalResult(m: Marker,mode: Mode,f: Feature,c: Crypto): bool = all {
    canCheckExternalResult(state,m,mode,f,c),state' = {...advance(state,ExternalResultChecked),marker:m,
      mode:mode,feature:f,crypto:c}
  }
  pure val requiredLeaves: Set[int] = Set(1,2,3)
  pure val markerLeaf: int = 4
  // verify-call has no output directory. Its local result is reported after
  // external Real verification; only keygen/prove publish artifact leaves.
  pure def readyToPublish(s: State): bool = if(s.purpose == Keygen) s.phase == Installed
    else s.purpose == ColdProve and s.phase == ExternalResultChecked
  pure def canWriteLeaves(s: State,existing: Set[int],written: Set[int]): bool = and {
    readyToPublish(s),existing.intersect(requiredLeaves.union(Set(markerLeaf))).size() == 0,written == requiredLeaves,
  }
  action writeLeaves(existing: Set[int],written: Set[int]): bool = all {
    canWriteLeaves(state,existing,written),state' = {...advance(state,LeavesWritten),leaves:written,existingFiles:existing}
  }
  pure def canPublish(s: State,existing: Set[int]): bool = and {
    s.phase == LeavesWritten,s.leaves == requiredLeaves,not(existing.contains(markerLeaf)),not(s.markerWritten),
  }
  action publish(existing: Set[int]): bool = all {
    canPublish(state,existing),state' = {...advance(state,Published),markerWritten:true}
  }
  pure def unused(s: State): Entry = if(selected(s.instance) == Settle) Freeze
    else if(selected(s.instance) == Freeze) Settle else if(selected(s.instance) == Step) Reset else Step
  pure def weakManifest(s: State): Manifest = {...s.manifest,
    keys:s.manifest.keys.put(selected(s.instance),{...key(selected(s.instance)),vk:999})}
  pure def unusedMap(s: State): Entry -> Operation = sourceOperations(s.source).put(unused(s),{...operation(key(unused(s))),vk:999})
  pure def canRefuse(s: State,f: Fault): bool = match f {
    | SeedInstalled => s.phase == Idle and not(canPreflight(s,Keygen,A1,exported(A1),sourceOperations(Metered),limits))
    | MetadataMismatch => s.phase == Idle and not(canPreflight(s,ColdVerify,A1,
      {...exported(A1),metadata:{...metadata(Metered),build:999}},sourceOperations(Metered),limits))
    | BadImportPurpose => s.phase == Discovered and not(canRegenerate(s,
      if(requiredPurpose(s.purpose) == PublicOnly) SelectedFull else PublicOnly,ExternalSourceDerivation))
    | EarlyKeyInit => s.phase == Regenerated and not(canCompare(s,s.manifest,s.purpose,1))
    | WeakImportedKey => s.phase == Regenerated and not(canCompare(s,weakManifest(s),s.purpose,0))
    | UnusedActionKey => s.phase == Compared and not(canInstall(s,unusedMap(s),s.frame))
    | WrongStoredOperation => s.phase == Prepared and not(canFinal(s,s.statement,{ir:999,vk:999,legacyVk:0},1,0))
    | FinalMutation => s.phase == Prepared and not(canFinal(s,{...s.statement,pi:List(0)},s.selectedOperation,1,0))
    | Collision => readyToPublish(s) and not(canWriteLeaves(s,Set(1),requiredLeaves))
    | PartialWrite => readyToPublish(s) and not(canWriteLeaves(s,Set(),Set(1)))
  }
  // Refusal records an actual distinguishing invalid proposal, publishes no
  // candidate frame/effects, preserves preexisting files, and is terminal.
  // A partial file write may leave an incomplete leaf; it never leaves a
  // completion marker. Collision observations retain the occupied leaf.
  action refuse(f: Fault): bool = all {
    canRefuse(state,f),state' = {...advance(state,Refused),faults:state.faults.union(Set(f)),
      leaves:if(f == PartialWrite) Set(1) else state.leaves,
      existingFiles:if(f == Collision) state.existingFiles.union(Set(1)) else state.existingFiles}
  }
  pure def faultAtBoundary(s: State,choice: int): Fault = match s.phase {
    | Idle => if(choice == 0) SeedInstalled else MetadataMismatch
    | Discovered => BadImportPurpose
    | Regenerated => if(choice == 0) EarlyKeyInit else WeakImportedKey
    | Compared => UnusedActionKey
    | Prepared => if(choice == 0) WrongStoredOperation else FinalMutation
    | _ => if(choice == 0) Collision else PartialWrite
  }
  action step = {
    nondet p = Set(Keygen,ColdProve,ColdVerify).oneOf()
    nondet c = Set(A1,A2,B1,B2,FreezeCase,ResetCase).oneOf()
    nondet chance = 0.to(5).oneOf()
    nondet choice = Set(0,1).oneOf()
    val fault = faultAtBoundary(state,choice)
    if(chance == 1 and canRefuse(state,fault)) refuse(fault) else any {
      preflight(p,c,exported(c),if(p == Keygen) keyless(sourceOf(c)) else sourceOperations(sourceOf(c)),limits),
      discover(sourceEntries(state.source).mapBy(e => key(e).degree),ExternalSdkDiscovery),
      regenerate(requiredPurpose(state.purpose),ExternalSourceDerivation),compare(state.manifest,state.purpose,0),
      install(sourceOperations(state.source),frame),prepare(1),
      finalCompare(expected(state.instance),state.selectedOperation,1,0),
      checkExternalResult(ProofMarker,Real,Verifying,ExternalRealSuccess),
      writeLeaves(Set(),requiredLeaves),publish(Set()),
    }
  }
  val preflightWitness = state.visited.contains(Preflight)
  val discoveryWitness = state.visited.contains(Discovered)
  val regenerationWitness = state.visited.contains(Regenerated)
  val comparisonWitness = state.visited.contains(Compared)
  val installedWitness = state.visited.contains(Installed)
  val preparedWitness = state.visited.contains(Prepared)
  val finalWitness = state.visited.contains(FinalCompared)
  val externalResultWitness = state.visited.contains(ExternalResultChecked)
  val leavesWitness = state.visited.contains(LeavesWritten)
  val acceptedWitness = state.phase == Published or
    (state.purpose == ColdVerify and state.phase == ExternalResultChecked)
  val refusedWitness = state.phase == Refused
  val keygenWitness = acceptedWitness and state.purpose == Keygen
  val coldProveWitness = acceptedWitness and state.purpose == ColdProve
  val pkFreeVerifierWitness = acceptedWitness and state.purpose == ColdVerify and state.pkReads == 0
  val lazyInitRefusalWitness = state.faults.contains(EarlyKeyInit)
  val unusedActionRefusalWitness = state.faults.contains(UnusedActionKey)
  val purposeRefusalWitness = state.faults.contains(BadImportPurpose)
  val weakKeyRefusalWitness = state.faults.contains(WeakImportedKey)
  val fullMapInvariant = state.visited.contains(Installed) implies exactMap(state,state.installedOps)
  val framePreservation = state.visited.contains(Installed) implies state.frame == state.admittedFrame
  val purposeSeparation = state.visited.contains(Regenerated) implies and {
    state.derivationPurpose == requiredPurpose(state.purpose),
    state.purpose == ColdVerify implies state.selectedPk == 0 and state.pkReads == 0,
  }
  val importBeforeUse = and {state.importedKeyInitializations == 0,
    state.visited.contains(Installed) implies state.importMatched and state.observation == ExternalSourceDerivation}
  val selectedStoredOperation = state.visited.contains(Installed) implies
    state.selectedOperation == state.installedOps.get(selected(state.instance))
  // Tuples abstract hashes of actual byte structures. There is no descriptor's
  // own digest in its preimage; candidateExpectedHash never chooses either tuple.
  val acyclicCommitments = state.visited.contains(Installed) implies and {
    state.snapshotCommitment == (state.frame,state.installedOps),
    state.descriptorCommitment == (metadata(state.source),state.snapshotCommitment),
  }
  val singleRawExpansion = state.visited.contains(FinalCompared) implies
    state.rawOwner == 1 and state.expansions == (if(state.purpose == ColdProve) 1 else 0)
  val fullFinalBinding = state.visited.contains(FinalCompared) implies state.checkedStatement == expected(state.instance)
  val publicationInvariant = state.markerWritten implies and {
    state.phase == Published,state.leaves == requiredLeaves,state.importMatched,
    if(state.purpose == Keygen) state.visited.contains(Installed) else and {
      state.visited.contains(ExternalResultChecked),state.marker == ProofMarker,state.mode == Real,
      state.feature == Verifying,state.crypto == ExternalRealSuccess,
    },
  }
  val safety = and {fullMapInvariant,framePreservation,purposeSeparation,importBeforeUse,
    selectedStoredOperation,acyclicCommitments,singleRawExpansion,fullFinalBinding,publicationInvariant}
}
