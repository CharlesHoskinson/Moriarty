// Literal ordinary oracle controls; no candidate-generated expected diagnostics.
// Symbols are payload-free, not real key bytes or native serialization fixtures.
module key_export_diagnostic_test {
  import key_export_diagnostic.* from "./key_export_diagnostic"
  pure val three: List[Token] = List(TagToken,PayloadToken,PayloadToken)
  pure val two: List[Token] = List(TagToken,PayloadToken)
  pure val context3: Context = {selectedAction:"settle",kind:Prover,degree:0,cap:ByteCap(3)}
  pure val cap3Accepted3: Diagnostic = {context:context3,reason:CapExceeded,accepted:AcceptedCount(3),rejectedChunk:Known(1),attemptedMinimum:Known(4)}
  pure val cap3Accepted2: Diagnostic = {context:context3,reason:CapExceeded,accepted:AcceptedCount(2),rejectedChunk:Known(2),attemptedMinimum:Known(4)}
  action ready(c: Context): bool = init.then(begin(c))
  action refusedTwo = ready(context3).then(admittedWrite(two)).then(reject(List(PayloadToken,PayloadToken)))
  run initializationTest = init.expect(safety and state.prefix == List() and state.serializerCalls == 0 and not(beginWitness))
  run exactCapTaggedSuccessTest = ready(context3).then(admittedWrite(three)).expect(state.prefix == three and state.firstFailure == NoFailure)
    .then(finish(ReturnedOk)).expect(state.result == Encoded(List(TagToken,PayloadToken,PayloadToken)) and state.serializerCalls == 1 and state.sizeProbeCalls == 0 and safety)
  run oneByteOverTest = ready(context3).then(admittedWrite(three)).then(reject(List(PayloadToken)))
    .expect(state.prefix == List(TagToken,PayloadToken,PayloadToken) and state.prefixAtFailure == three and safety)
    .then(finish(ReturnedError)).expect(state.result == Refused(cap3Accepted3) and safety)
  run rejectedMultiByteChunkTest = refusedTwo.expect(state.prefix == List(TagToken,PayloadToken))
    .then(finish(ReturnedError)).expect(state.result == Refused(cap3Accepted2) and safety)
  run zeroCapEmptyWriteTest = ready({selectedAction:"empty",kind:Verifier,degree:0,cap:ByteCap(0)}).then(admittedWrite(List()))
    .expect(state.prefix == List() and state.emptyWrites == 1).then(reject(List(PayloadToken))).then(finish(ReturnedOk))
    .expect(state.result == Refused({context:{selectedAction:"empty",kind:Verifier,degree:0,cap:ByteCap(0)},reason:CapExceeded,accepted:AcceptedCount(0),rejectedChunk:Known(1),attemptedMinimum:Known(1)}) and safety)
  run emptyWriteCountsNoBytesTest = ready(context3).then(admittedWrite(List())).then(admittedWrite(two)).then(admittedWrite(List()))
    .expect(state.prefix == List(TagToken,PayloadToken) and state.admittedWrites == 3 and state.emptyWrites == 2)
    .then(finish(ReturnedOk)).expect(state.result == Encoded(List(TagToken,PayloadToken)) and safety)
  run overflowPureCheckedLengthProbeTest = init.then(probeOverflow(Verifier,0)).expect(
    state.result == Refused({context:{selectedAction:"overflow-probe",kind:Verifier,degree:0,cap:UsizeMaximum},reason:LengthOverflow,
      accepted:UsizeMaximumMinusOne,rejectedChunk:Known(2),attemptedMinimum:Unavailable})
    and state.prefix == List() and state.serializerCalls == 0 and state.sizeProbeCalls == 0 and safety)
  run serializerFailureBeforeAnyWriteTest = ready(context3).then(finish(ReturnedError)).expect(
    state.result == Refused({context:context3,reason:Serialization,accepted:AcceptedCount(0),rejectedChunk:Unavailable,attemptedMinimum:Unavailable}) and safety)
  run serializerFailureAfterAdmittedPrefixTest = ready(context3).then(admittedWrite(List(TagToken))).then(finish(ReturnedError)).expect(
    state.result == Refused({context:context3,reason:Serialization,accepted:AcceptedCount(1),rejectedChunk:Unavailable,attemptedMinimum:Unavailable}) and safety)
  run misleadingMessageIsNotCapEvidenceTest = ready(context3).then(admittedWrite(two)).then(finish(MisleadingCapError)).expect(
    state.result == Refused({context:context3,reason:Serialization,accepted:AcceptedCount(2),rejectedChunk:Unavailable,attemptedMinimum:Unavailable}) and safety)
  run ignoredWriterErrorCannotFinishSuccessTest = refusedTwo.then(finish(ReturnedOk)).expect(state.result == Refused(cap3Accepted2) and ignoredFailureWitness and safety)
  run wrappedWriterErrorPreservesFirstReasonTest = refusedTwo.then(finish(WrappedError)).expect(state.result == Refused(cap3Accepted2) and wrappedFailureWitness and safety)
  run replacedWriterErrorPreservesFirstReasonTest = refusedTwo.then(finish(ReplacedError)).expect(state.result == Refused(cap3Accepted2) and replacedFailureWitness and safety)
  run misleadingAfterWriterFailureUsesWriterEvidenceTest = refusedTwo.then(finish(MisleadingCapError)).expect(state.result == Refused(cap3Accepted2) and safety)
  run laterSmallerFitMustNotAppendTest = refusedTwo.then(refuseAgain(List(PayloadToken)))
    .expect(state.prefix == List(TagToken,PayloadToken) and state.firstFailure == WriterFailed({reason:CapExceeded,accepted:AcceptedCount(2),rejectedChunk:2,attemptedMinimum:Known(4)}) and firstFailureSticky)
    .then(finish(ReturnedOk)).expect(state.result == Refused(cap3Accepted2) and safety)
  run smallerFitAdmitActionDisabledTest = refusedTwo.then(admittedWrite(List(PayloadToken)).fail())
  run emptyAfterFailureMustRefuseTest = refusedTwo.then(refuseAgain(List())).expect(state.prefix == List(TagToken,PayloadToken) and retryRefusalWitness and safety)
    .then(finish(ReturnedOk)).expect(state.result == Refused(cap3Accepted2) and safety)
  run emptyAfterFailureAdmitActionDisabledTest = refusedTwo.then(admittedWrite(List()).fail())
  run laterOversizedWritePreservesFirstMinimumTest = refusedTwo.then(refuseAgain(List(PayloadToken,PayloadToken,PayloadToken,PayloadToken)))
    .then(finish(WrappedError)).expect(state.result == Refused(cap3Accepted2) and safety)
  run exactMinimumIsNotFullSerializedSizeTest = refusedTwo.then(refuseAgain(List(PayloadToken,PayloadToken,PayloadToken)))
    .then(finish(ReturnedOk)).expect(state.result == Refused(cap3Accepted2) and state.retryWrites == 1 and safety)
  run headersCountAgainstCapTest = ready({selectedAction:"tag",kind:Prover,degree:1,cap:ByteCap(2)})
    .then(admittedWrite(List(TagToken,TagToken))).then(reject(List(PayloadToken))).then(finish(ReturnedOk)).expect(
      state.result == Refused({context:{selectedAction:"tag",kind:Prover,degree:1,cap:ByteCap(2)},reason:CapExceeded,accepted:AcceptedCount(2),rejectedChunk:Known(1),attemptedMinimum:Known(3)}) and safety)
  run actualContextMetadataPreservedTest = ready({selectedAction:"freeze",kind:Verifier,degree:2,cap:ByteCap(1)})
    .then(reject(two)).then(finish(ReturnedError)).expect(state.result == Refused({context:{selectedAction:"freeze",kind:Verifier,degree:2,cap:ByteCap(1)},reason:CapExceeded,
      accepted:AcceptedCount(0),rejectedChunk:Known(2),attemptedMinimum:Known(2)}) and safety)
  run opaqueActionApiIdentityTest = ready({selectedAction:"opaque-action",kind:Verifier,degree:0,cap:ByteCap(1)})
    .then(reject(two)).then(finish(ReturnedError)).expect(state.result == Refused({context:{selectedAction:"opaque-action",kind:Verifier,degree:0,cap:ByteCap(1)},
      reason:CapExceeded,accepted:AcceptedCount(0),rejectedChunk:Known(2),attemptedMinimum:Known(2)}) and safety)
  run unfinishedSerializerReturnCannotFinishTest = ready(context3).then(finish(NotReturned).fail())
  run finishedWriterCannotAppendTest = ready(context3).then(finish(ReturnedOk)).then(admittedWrite(List(TagToken)).fail())
  // Test-only ablations are outside the production step and routine Test set.
  // Removing the sticky-failure check returns a successful admitted prefix.
  action swallowedFailureInit = refusedTwo
  action swallowedFailureStep = all {state.phase == Writing,
    state' = {...state,phase:Finished,serializerReturn:ReturnedOk,result:Encoded(state.prefix)}}
  // Removing the terminal write check admits a smaller fitting chunk after error.
  action laterFitFaultInit = refusedTwo
  action laterFitFaultStep = all {state.phase == Writing,
    state' = {...state,prefix:state.prefix.concat(List(PayloadToken)),admittedWrites:state.admittedWrites + 1}}
}
