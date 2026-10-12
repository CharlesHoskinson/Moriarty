// PROPOSED CONTRACT ONLY; no production correspondence or native proof claim.
module prover_key_domain {
  type Profile = V1 | V2
  type Purpose = Keyset | GenericPackage
  type Reader = Cold | Proving | Export
  type KeyName = Selected | Lookalike | Extra
  type Phase = Idle | Admitted | PublicRead | PkChecked | PkRead | Snapshot | Writing | LeavesWritten | ProvingMarkerWritten | Published | Refused
  type Access = PublicMarker | PublicPayload | ProvingMarker | SelectedPk
  type Limits = { leaf: int, public: int, pk: int, input: int, vk: int }
  type Fixture = { leaves: List[int], publicMarker: int, provingMarker: int, declaredPk: int,
    actualPk: int, declaredHash: int, actualHash: int, keyName: KeyName,
    pkCount: int, markerIdentity: int, provingIdentity: int, input: int, vk: int }
  type Request = { profile: Profile, purpose: Purpose, reader: Reader, signed: bool, limits: Limits, fixture: Fixture }
  type Operation = { phase: Phase, request: Request, publicBytes: int, pkBytes: int,
    physicalBytes: int, accessed: Set[Access], written: int, publicCommitted: bool,
    provingCommitted: bool, refusal: str }
  pure val WORD_MAX = 31
  pure val PUBLIC_HARD = 8
  pure val PK_HARD = 16
  pure val INPUT_HARD = 2
  pure val VK_HARD = 1
  pure val limits: Limits = { leaf: 4, public: 8, pk: 16, input: 2, vk: 1 }
  pure val fixture: Fixture = { leaves: List(2, 1, 1), publicMarker: 1, provingMarker: 1,
    declaredPk: 12, actualPk: 12, declaredHash: 7, actualHash: 7, keyName: Selected,
    pkCount: 1, markerIdentity: 9, provingIdentity: 9, input: 1, vk: 1 }
  pure val request: Request = { profile: V2, purpose: Keyset, reader: Cold,
    signed: false, limits: limits, fixture: fixture }
  var op: Operation
  action init = op' = { phase: Idle, request: request, publicBytes: 0, pkBytes: 0,
    physicalBytes: 0, accessed: Set(), written: 0, publicCommitted: false,
    provingCommitted: false, refusal: "" }
  pure def checkedAdd(a: int, b: int, max: int): bool =
    a >= 0 and b >= 0 and a <= max and b <= max - a
  pure def min(a: int, b: int): int = if (a < b) a else b
  pure def independent(r: Request): bool = r.profile == V2 and r.purpose == Keyset
  pure def hasPk(r: Request): bool = r.purpose == Keyset and r.reader != Cold
  pure def publicTotal(r: Request): int = r.fixture.leaves.foldl(0, (a, b) => a + b)
    + r.fixture.publicMarker + (if (r.reader == Cold) 0 else r.fixture.provingMarker)
    + (if (r.purpose == GenericPackage) r.fixture.declaredPk else 0)
  pure def pkTotal(r: Request): int = if (hasPk(r)) r.fixture.declaredPk else 0
  pure def profileValid(r: Request): bool = {
    val l = r.limits
    and {
      Set(l.leaf, l.public, l.pk, l.input, l.vk).forall(n => n > 0 and n <= WORD_MAX),
      if (r.profile == V2) and {
        l.leaf <= PUBLIC_HARD, l.public <= PUBLIC_HARD, l.pk <= PK_HARD,
        l.input <= INPUT_HARD, l.vk <= VK_HARD,
      } else if (r.signed) l.public <= PUBLIC_HARD else true,
    }
  }
  pure def publicBounded(r: Request): bool = and {
    r.fixture.leaves.foldl(true, (ok, n) => ok and n >= 0 and n <= r.limits.leaf),
    r.fixture.publicMarker >= 0 and r.fixture.publicMarker <= r.limits.leaf,
    if (r.reader != Cold) r.fixture.provingMarker >= 0 and r.fixture.provingMarker <= r.limits.leaf else true,
    if (r.purpose == GenericPackage) r.fixture.declaredPk >= 0 and r.fixture.declaredPk <= r.limits.leaf else true,
    publicTotal(r) <= r.limits.public,
    publicTotal(r) <= WORD_MAX,
    r.fixture.input >= 0 and r.fixture.input <= r.limits.input,
    r.fixture.vk >= 0 and r.fixture.vk <= r.limits.vk,
  }
  pure def pkBounded(r: Request): bool = if (not(hasPk(r))) true else and {
    r.fixture.keyName == Selected and r.fixture.pkCount == 1,
    r.fixture.declaredPk >= 0,
    r.fixture.declaredPk <= (if (independent(r)) r.limits.pk else min(r.limits.pk, r.limits.leaf)),
    checkedAdd(publicTotal(r), pkTotal(r), WORD_MAX),
    if (independent(r)) true else publicTotal(r) + pkTotal(r) <= r.limits.public,
  }
  pure def publicRefusal(r: Request): str =
    if (not(profileValid(r))) "profile" else if (not(publicBounded(r))) "public" else ""
  pure def pkRefusal(r: Request): str = if (not(pkBounded(r))) "pk-declaration" else ""
  action begin(r: Request): bool = all {
    op.phase == Idle,
    publicRefusal(r) == "",
    if (r.reader == Export) pkBounded(r) else true,
    op' = {...op, request: r, phase: Admitted},
  }
  action refuseAdmission(r: Request): bool = all {
    op.phase == Idle,
    publicRefusal(r) != "" or (r.reader == Export and not(pkBounded(r))),
    op' = {...op, request: r, phase: Refused,
      refusal: if (publicRefusal(r) != "") publicRefusal(r) else "pk-declaration"},
  }
  action acquirePublic: bool = all {
    op.phase == Admitted,
    op.request.reader != Export,
    op' = {...op, phase: PublicRead, publicBytes: publicTotal(op.request),
      physicalBytes: publicTotal(op.request),
      accessed: Set(PublicMarker, PublicPayload).union(
        if (op.request.reader == Proving) Set(ProvingMarker) else Set())},
  }
  action coldSnapshot: bool = all {
    op.phase == PublicRead, op.request.reader == Cold,
    op' = {...op, phase: Snapshot},
  }
  action checkPkDeclaration: bool = all {
    op.phase == PublicRead, op.request.reader == Proving,
    pkBounded(op.request),
    op.request.fixture.markerIdentity == op.request.fixture.provingIdentity,
    op' = {...op, phase: PkChecked},
  }
  action refusePkDeclaration: bool = all {
    op.phase == PublicRead, op.request.reader == Proving,
    not(pkBounded(op.request)),
    op' = {...op, phase: Refused, refusal: "pk-declaration"},
  }
  action refuseMarkerAgreement: bool = all {
    op.phase == PublicRead, op.request.reader == Proving,
    pkBounded(op.request),
    op.request.fixture.markerIdentity != op.request.fixture.provingIdentity,
    op' = {...op, phase: Refused, refusal: "marker-agreement"},
  }
  action acquirePk: bool = all {
    op.phase == PkChecked,
    op' = {...op, phase: PkRead, pkBytes: pkTotal(op.request),
      physicalBytes: op.publicBytes + pkTotal(op.request),
      accessed: op.accessed.union(Set(SelectedPk))},
  }
  action provingSnapshot: bool = all {
    op.phase == PkRead,
    op.request.fixture.actualPk == op.request.fixture.declaredPk,
    op.request.fixture.actualHash == op.request.fixture.declaredHash,
    op' = {...op, phase: Snapshot},
  }
  action refusePkLength: bool = all {
    op.phase == PkRead,
    op.request.fixture.actualPk != op.request.fixture.declaredPk,
    op' = {...op, phase: Refused, refusal: "pk-length"},
  }
  action refusePkHash: bool = all {
    op.phase == PkRead,
    op.request.fixture.actualPk == op.request.fixture.declaredPk,
    op.request.fixture.actualHash != op.request.fixture.declaredHash,
    op' = {...op, phase: Refused, refusal: "pk-hash"},
  }
  pure def payloadPublic(r: Request): int = r.fixture.leaves.foldl(0, (a,b) => a+b)
    + (if (r.purpose == GenericPackage) r.fixture.declaredPk else 0)
  action startWrite: bool = all {
    op.phase == Admitted, op.request.reader == Export,
    op' = {...op, phase: Writing},
  }
  action writeLeaves: bool = all {
    op.phase == Writing,
    op' = {...op, phase: LeavesWritten, publicBytes: payloadPublic(op.request),
      pkBytes: pkTotal(op.request), physicalBytes: payloadPublic(op.request) + pkTotal(op.request), written: 1},
  }
  action refuseLeafWrite: bool = all {
    op.phase == Writing,
    op' = {...op, phase: Refused, refusal: "leaf-write"},
  }
  action writeProvingMarker: bool = all {
    op.phase == LeavesWritten,
    op' = {...op, phase: ProvingMarkerWritten,
      publicBytes: op.publicBytes + op.request.fixture.provingMarker,
      physicalBytes: op.physicalBytes + op.request.fixture.provingMarker,
      provingCommitted: true, written: 2},
  }
  action refuseProvingMarkerWrite: bool = all {
    op.phase == LeavesWritten,
    op' = {...op, phase: Refused, refusal: "proving-marker-write"},
  }
  action publishPublicMarker: bool = all {
    op.phase == ProvingMarkerWritten,
    op' = {...op, phase: Published,
      publicBytes: op.publicBytes + op.request.fixture.publicMarker,
      physicalBytes: op.physicalBytes + op.request.fixture.publicMarker,
      publicCommitted: true, written: 3},
  }
  action refusePublicMarkerWrite: bool = all {
    op.phase == ProvingMarkerWritten,
    op' = {...op, phase: Refused, refusal: "public-marker-write"},
  }
  // Terminal states have no outgoing transition. No blanket stutter.
  action chooseRequest: bool = {
    nondet profile = Set(V1, V2).oneOf()
    nondet purpose = Set(Keyset, GenericPackage).oneOf()
    nondet reader = Set(Cold, Proving, Export).oneOf()
    nondet signed = Set(false, true).oneOf()
    nondet publicCap = Set(5, 6, 8).oneOf()
    nondet pkCap = Set(4, 12, 16).oneOf()
    nondet declared = Set(4, 12, 16, 17, 31).oneOf()
    nondet actual = Set(4, 12, 16, 17, 31).oneOf()
    nondet hash = Set(7, 8).oneOf()
    nondet identity = Set(9, 10).oneOf()
    nondet name = Set(Selected, Lookalike, Extra).oneOf()
    nondet count = Set(1, 2).oneOf()
    val r = {profile: profile, purpose: purpose, reader: reader, signed: signed,
      limits: {...limits, public: publicCap, pk: pkCap},
      fixture: {...fixture, declaredPk: declared, actualPk: actual, actualHash: hash,
        provingIdentity: identity, keyName: name, pkCount: count}}
    any { begin(r), refuseAdmission(r) }
  }
  // Disabled terminal action is false, so it introduces no self-transition.
  action terminal: bool = all { false, op' = op }
  action step = match op.phase {
    | Idle => chooseRequest
    | Admitted => if (op.request.reader == Export) startWrite else acquirePublic
    | PublicRead => if (op.request.reader == Cold) coldSnapshot else any {
      checkPkDeclaration, refusePkDeclaration, refuseMarkerAgreement }
    | PkChecked => acquirePk
    | PkRead => any { provingSnapshot, refusePkLength, refusePkHash }
    | Writing => any { writeLeaves, refuseLeafWrite }
    | LeavesWritten => any { writeProvingMarker, refuseProvingMarkerWrite }
    | ProvingMarkerWritten => any { publishPublicMarker, refusePublicMarkerWrite }
    | Snapshot => terminal
    | Published => terminal
    | Refused => terminal
  }
  val physicalChecked = checkedAdd(op.publicBytes, op.pkBytes, WORD_MAX)
    and op.physicalBytes == op.publicBytes + op.pkBytes
  val coldNoOptionalAccess = (op.request.purpose == Keyset and op.request.reader == Cold)
    implies not(op.accessed.contains(SelectedPk) or op.accessed.contains(ProvingMarker))
  val publicWithinCap = op.publicBytes <= op.request.limits.public
  val pkWithinCap = op.pkBytes <= op.request.limits.pk
  val markerLast = (op.phase == Refused implies not(op.publicCommitted))
    and (op.publicCommitted implies op.phase == Published and op.provingCommitted and op.written == 3)
  val snapshotExact = (op.phase == Snapshot and op.request.reader == Proving) implies and {
    op.request.fixture.actualPk == op.request.fixture.declaredPk,
    op.request.fixture.actualHash == op.request.fixture.declaredHash,
    op.request.fixture.markerIdentity == op.request.fixture.provingIdentity,
    op.accessed.contains(SelectedPk),
  }
  val markerBeforePk = op.accessed.contains(SelectedPk) implies
    op.request.fixture.markerIdentity == op.request.fixture.provingIdentity
  // Independent raw-field checks remain fixed when fault accounting functions change.
  val genericNoException = (op.request.purpose == GenericPackage and
    op.phase != Idle and op.phase != Refused) implies
    op.request.fixture.declaredPk <= op.request.limits.leaf
  val safe = physicalChecked and coldNoOptionalAccess and publicWithinCap and pkWithinCap
    and markerLast and snapshotExact and markerBeforePk and genericNoException
  val begun = op.phase == Admitted
  val acquiredPublic = op.phase == PublicRead
  val coldComplete = op.phase == Snapshot and op.request.reader == Cold
  val pkDeclarationChecked = op.phase == PkChecked
  val pkAcquired = op.phase == PkRead
  val provingComplete = op.phase == Snapshot and op.request.reader == Proving
  val writingStarted = op.phase == Writing
  val leavesWritten = op.phase == LeavesWritten
  val provingMarkerWritten = op.phase == ProvingMarkerWritten
  val published = op.phase == Published
  val refused = op.phase == Refused
  val failedLeafWrite = op.refusal == "leaf-write"
  val failedProvingMarker = op.refusal == "proving-marker-write"
  val failedPublicMarker = op.refusal == "public-marker-write"
}
