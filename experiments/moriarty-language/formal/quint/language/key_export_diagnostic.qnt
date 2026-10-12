// Independent expected contract SHA25657f466a7109397576836c924f87dfe39ff532d640d6d5cf89b1ecc023c72cc7e.
// One ordinary writer/serializer, no native keys, SRS, owners or cryptography.
// Payload-free tokens model admitted stream positions, including tagged headers.
// Symbolic overflow is a pure checked-length probe, never a huge buffer/stream.
module key_export_diagnostic {
  type Phase = Idle | Writing | Finished
  type Mode = Encoding | CheckedLengthProbe
  type KeyKind = Prover | Verifier
  type Cap = ByteCap(int) | UsizeMaximum
  type Count = AcceptedCount(int) | UsizeMaximumMinusOne
  type Measurement = Known(int) | Unavailable
  type Reason = CapExceeded | LengthOverflow | Serialization
  type Token = TagToken | PayloadToken
  type Context = {selectedAction: str, kind: KeyKind, degree: int, cap: Cap}
  type WriteEvidence = {reason: Reason, accepted: Count, rejectedChunk: int, attemptedMinimum: Measurement}
  type Failure = NoFailure | WriterFailed(WriteEvidence)
  type SerializerReturn = NotReturned | ReturnedOk | ReturnedError | WrappedError | ReplacedError | MisleadingCapError
  type Diagnostic = {context: Context, reason: Reason, accepted: Count, rejectedChunk: Measurement, attemptedMinimum: Measurement}
  type Result = Pending | Encoded(List[Token]) | Refused(Diagnostic)
  type State = {phase: Phase, mode: Mode, context: Context, prefix: List[Token], prefixAtFailure: List[Token], firstFailure: Failure,
    refusals: List[WriteEvidence], serializerReturn: SerializerReturn, result: Result,
    serializerCalls: int, sizeProbeCalls: int, admittedWrites: int, emptyWrites: int, retryWrites: int}
  var state: State
  pure def context(cap: int,k: KeyKind,d: int,a: str): Context = {selectedAction:a,kind:k,degree:d,cap:ByteCap(cap)}
  pure val initial: State = {phase:Idle,mode:Encoding,context:context(3,Prover,0,"settle"),prefix:List(),prefixAtFailure:List(),firstFailure:NoFailure,
    refusals:List(),serializerReturn:NotReturned,result:Pending,serializerCalls:0,sizeProbeCalls:0,admittedWrites:0,emptyWrites:0,retryWrites:0}
  pure def ordinaryContext(c: Context): bool = match c.cap { | ByteCap(n) => n >= 0 and n <= 4 and c.degree >= 0 and c.degree <= 2 | UsizeMaximum => false }
  pure def canBegin(s: State,c: Context): bool = s.phase == Idle and ordinaryContext(c)
  pure def applyBegin(s: State,c: Context): State = {...s,phase:Writing,context:c,serializerCalls:1}
  action init = state' = initial
  action begin(c: Context): bool = all {canBegin(state,c),state' = applyBegin(state,c)}
  pure def finiteCap(c: Cap): int = match c { | ByteCap(n) => n | UsizeMaximum => 4 }
  pure def withinDomain(chunk: List[Token]): bool = chunk.length() <= 4
  pure def canAppend(s: State,chunk: List[Token]): bool = and {
    s.phase == Writing,s.mode == Encoding,s.firstFailure == NoFailure,withinDomain(chunk),
    s.prefix.length() + chunk.length() <= finiteCap(s.context.cap),
  }
  pure def applyAppend(s: State,chunk: List[Token]): State = {...s,prefix:s.prefix.concat(chunk),
    admittedWrites:s.admittedWrites + 1,emptyWrites:s.emptyWrites + (if(chunk.length() == 0) 1 else 0)}
  action admittedWrite(chunk: List[Token]): bool = all {canAppend(state,chunk),state' = applyAppend(state,chunk)}
  pure def firstRejected(s: State,chunk: List[Token]): WriteEvidence = {
    reason:CapExceeded,accepted:AcceptedCount(s.prefix.length()),rejectedChunk:chunk.length(),attemptedMinimum:Known(s.prefix.length() + chunk.length())}
  pure def canReject(s: State,chunk: List[Token]): bool = and {
    s.phase == Writing,s.mode == Encoding,s.firstFailure == NoFailure,withinDomain(chunk),
    s.prefix.length() + chunk.length() > finiteCap(s.context.cap),
  }
  pure def applyReject(s: State,chunk: List[Token]): State = {
    val failure = firstRejected(s,chunk)
    {...s,firstFailure:WriterFailed(failure),prefixAtFailure:s.prefix,refusals:s.refusals.append(failure)}
  }
  action reject(chunk: List[Token]): bool = all {canReject(state,chunk),state' = applyReject(state,chunk)}
  // The writer's fatal first failure freezes the exact accepted prefix, even
  // for a later empty or smaller fitting write. No new length check replaces it.
  pure def canRefuseAgain(s: State,chunk: List[Token]): bool =
    s.phase == Writing and s.mode == Encoding and s.firstFailure != NoFailure and withinDomain(chunk)
  pure def applyRefuseAgain(s: State): State = {...s,retryWrites:s.retryWrites + 1}
  action refuseAgain(chunk: List[Token]): bool = all {canRefuseAgain(state,chunk),state' = applyRefuseAgain(state)}
  pure def writerDiagnostic(c: Context,f: WriteEvidence): Diagnostic = {context:c,reason:f.reason,accepted:f.accepted,
    rejectedChunk:Known(f.rejectedChunk),attemptedMinimum:f.attemptedMinimum}
  pure def serializerDiagnostic(s: State): Diagnostic = {context:s.context,reason:Serialization,accepted:AcceptedCount(s.prefix.length()),rejectedChunk:Unavailable,attemptedMinimum:Unavailable}
  pure def finishResult(s: State,r: SerializerReturn): Result = match s.firstFailure {
    | WriterFailed(f) => Refused(writerDiagnostic(s.context,f))
    | NoFailure => if(r == ReturnedOk) Encoded(s.prefix) else Refused(serializerDiagnostic(s))
  }
  pure def canFinish(s: State,r: SerializerReturn): bool = s.phase == Writing and r != NotReturned
  pure def applyFinish(s: State,r: SerializerReturn): State = {...s,phase:Finished,serializerReturn:r,result:finishResult(s,r)}
  action finish(r: SerializerReturn): bool = all {canFinish(state,r),state' = applyFinish(state,r)}
  // Dedicated finite symbolic probe for usize::MAX-1 +2. It represents the
  // pure Rust checked-length control, not any allocated prefix or serializer.
  pure val overflowEvidence: WriteEvidence = {reason:LengthOverflow,accepted:UsizeMaximumMinusOne,rejectedChunk:2,attemptedMinimum:Unavailable}
  pure def canProbeOverflow(s: State): bool = s.phase == Idle
  pure def applyProbeOverflow(s: State,k: KeyKind,d: int): State = {
    val c = {selectedAction:"overflow-probe",kind:k,degree:d,cap:UsizeMaximum}
    {...s,phase:Finished,mode:CheckedLengthProbe,context:c,firstFailure:WriterFailed(overflowEvidence),refusals:List(overflowEvidence),
      result:Refused(writerDiagnostic(c,overflowEvidence))}
  }
  action probeOverflow(k: KeyKind,d: int): bool = all {
    canProbeOverflow(state),d >= 0,d <= 2,state' = applyProbeOverflow(state,k,d)
  }
  action step = {
    nondet cap = 0.to(4).oneOf()
    nondet k = Set(Prover,Verifier).oneOf()
    nondet d = 0.to(2).oneOf()
    nondet chunkLen = 0.to(4).oneOf()
    nondet token = Set(TagToken,PayloadToken).oneOf()
    nondet r = Set(ReturnedOk,ReturnedError,WrappedError,ReplacedError,MisleadingCapError).oneOf()
    val chunk = range(0,chunkLen).foldl(List(),(xs,_) => xs.append(token))
    any {begin(context(cap,k,d,"settle")),admittedWrite(chunk),reject(chunk),refuseAgain(chunk),finish(r),probeOverflow(k,d)}
  }
  val beginWitness = state.serializerCalls == 1
  val appendWitness = state.admittedWrites > 0
  val emptyWriteWitness = state.emptyWrites > 0
  val retryRefusalWitness = state.retryWrites > 0
  val capWitness = match state.result { | Refused(d) => d.reason == CapExceeded | _ => false }
  val overflowWitness = match state.result { | Refused(d) => d.reason == LengthOverflow | _ => false }
  val serializerWitness = match state.result { | Refused(d) => d.reason == Serialization | _ => false }
  val successWitness = match state.result { | Encoded(_) => true | _ => false }
  val ignoredFailureWitness = capWitness and state.serializerReturn == ReturnedOk
  val wrappedFailureWitness = capWitness and state.serializerReturn == WrappedError
  val replacedFailureWitness = capWitness and state.serializerReturn == ReplacedError
  val misleadingSerializerWitness = serializerWitness and state.serializerReturn == MisleadingCapError
  val verifierWitness = state.phase == Finished and state.context.kind == Verifier
  val zeroDegreeWitness = state.phase == Finished and state.context.degree == 0
  val acceptedWithinCap = state.mode == Encoding implies state.prefix.length() <= finiteCap(state.context.cap)
  val onePass = if(state.mode == CheckedLengthProbe) state.serializerCalls == 0 and state.sizeProbeCalls == 0
    else (state.phase != Idle implies state.serializerCalls == 1 and state.sizeProbeCalls == 0)
  val firstFailureSticky = state.firstFailure != NoFailure implies and {
    state.refusals.length() == 1,state.firstFailure == WriterFailed(state.refusals.head()),state.prefix == state.prefixAtFailure,
  }
  val noTruncatedSuccess = match state.result {
    | Encoded(bytes) => state.mode == Encoding and state.firstFailure == NoFailure and state.serializerReturn == ReturnedOk and bytes == state.prefix
    | _ => true
  }
  val faithfulFailure = state.phase == Finished implies (match state.firstFailure {
    | WriterFailed(f) => state.result == Refused(writerDiagnostic(state.context,f))
    | NoFailure => state.result == (if(state.serializerReturn == ReturnedOk) Encoded(state.prefix) else Refused(serializerDiagnostic(state)))
  })
  val truthfulAttemptedMinimum = match state.result {
    | Refused(d) => d.context == state.context and (match d.reason {
      | CapExceeded => match d.accepted {
        | AcceptedCount(n) => d.attemptedMinimum == Known(n + state.refusals.head().rejectedChunk)
          and n == state.prefix.length() and n <= finiteCap(d.context.cap)
          and n + state.refusals.head().rejectedChunk > finiteCap(d.context.cap)
        | _ => false
      }
      | LengthOverflow => d.accepted == UsizeMaximumMinusOne and d.attemptedMinimum == Unavailable and d.rejectedChunk == Known(2)
      | Serialization => d.attemptedMinimum == Unavailable and d.rejectedChunk == Unavailable and d.accepted == AcceptedCount(state.prefix.length())
    })
    | _ => true
  }
  val safety = acceptedWithinCap and onePass and firstFailureSticky and noTruncatedSuccess and faithfulFailure and truthfulAttemptedMinimum
}
