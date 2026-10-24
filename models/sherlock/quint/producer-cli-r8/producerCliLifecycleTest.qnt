module producerCliLifecycleTest {
  import producerCliLifecycle.* from "./producerCliLifecycle"
  import ownedSubmissionLifecycle.* from "./ownedSubmissionLifecycle"
  import ownedNodeValidation.* from "./ownedNodeValidation"

  pure val parameterWithViews = parameterOwnView(parameterOwnView(parameterDeserialize(parameterStart), 2), 3)
  pure def cliReady(accepted:bool, failedBusiness:bool, recorded:int):CliTrace = {
    val tracked = cliTrackBusiness(cliTrace(accepted, false, recorded, true))
    val drained = cliSettleBusiness(cliBeginDrain(tracked), not(failedBusiness))
    val closed = cliSettleClose(cliBeginClose(drained), true)
    val released = cliReleaseTransactions(closed, true)
    cliMintLogicalSeal(cliFinishSafe(cliCreateSafe(cliSealSafe(released))))
  }
  pure def flushed(s:CliTrace):CliTrace =
    cliFinishStderr(cliFinishStdout(cliBeginCompletion(s), CliFlushed), CliFlushed)

  run producerTransferWaitsForSuccessfulViewDrainTest = initWith(good).expect({
    val registered = parameterOwnView(parameterOwnView(parameterOwnView(parameterWithViews, 2), 1), 0)
    val started = parameterBeginDrain(registered, SDK04NoFailure)
    val runtimeFreed = parameterFreeView(started, true)
    val viewsFreed = parameterFreeView(runtimeFreed, true)
    val returned = parameterTransfer(viewsFreed)
    val callerReleased = parameterReleaseCaller(returned, true)
    parameterCanDeserialize(parameterStart) and parameterCanOwnView(parameterWithViews, 2)
      and registered == parameterWithViews and registered.views.owned == List(2, 3)
      and registered.views.seen == Set(2, 3) and not(parameterCanTransfer(registered))
      and parameterCanBeginDrain(registered) and not(parameterCanTransfer(started))
      and parameterCanFreeView(started) and runtimeFreed.views.attempted == List(3)
      and not(parameterCanTransfer(runtimeFreed)) and parameterCanFreeView(runtimeFreed)
      and viewsFreed.views.attempted == List(3, 2) and parameterCanTransfer(viewsFreed)
      and not(parameterCanReleaseProducer(viewsFreed)) and returned.owner == CallerParams
      and returned.transferred and returned.paramsFreeAttempts == 0
      and parameterCanReleaseCaller(returned) and callerReleased.owner == ReleasedParams
      and callerReleased.transferred and callerReleased.paramsFreeAttempts == 1
      and not(parameterCanReleaseCaller(callerReleased)) and parameterSafety(callerReleased)
  })
  run producerPreReturnFailuresReleaseOwnedParamsAndDrainSiblingsTest = initWith(good).expect({
    val noViews = parameterDeserialize(parameterStart)
    val oneView = parameterOwnView(noViews, 2)
    val emptyDrain = parameterBeginDrain(noViews, SDK04SyntheticA)
    val oneDrain = parameterFreeView(parameterBeginDrain(oneView, SDK04SyntheticA), true)
    val twoStart = parameterBeginDrain(parameterWithViews, SDK04SyntheticA)
    val twoDrain = parameterFreeView(parameterFreeView(twoStart, false), true)
    Set(emptyDrain, oneDrain, twoDrain).forall(s =>
      parameterCanReleaseProducer(s) and not(parameterCanTransfer(s))
        and not(s.transferred) and parameterReportFailure(s) == SDK04SyntheticA
        and parameterReleaseProducer(s, true).owner == ReleasedParams
        and parameterReleaseProducer(s, true).paramsFreeAttempts == 1
        and not(parameterCanReleaseProducer(parameterReleaseProducer(s, true)))
        and parameterSafety(parameterReleaseProducer(s, true)))
      and oneDrain.views.attempted == List(2) and twoDrain.views.attempted == List(3, 2)
      and twoDrain.views.cleanupRefused and not(emptyDrain.views.cleanupRefused)
      and twoDrain.views.attempted.indices().forall(i =>
        not(Set(0, 1).contains(twoDrain.views.attempted.nth(i))))
  })
  run producerCleanupRefusalPreservesPrimaryBeforeOpaqueWrapperTest = initWith(good).expect({
    val failed = parameterFreeView(parameterFreeView(
      parameterBeginDrain(parameterWithViews, SDK04SyntheticA), false), true)
    val released = parameterReleaseProducer(failed, true)
    val observed = producerObserve(producerObservation, parameterReportFailure(released),
      parameterReportCode(released), parameterCleanupRefused(released))
    val opaque = producerObserve(observed, SDK04SyntheticB, ProducerUnclassifiedCode, false)
    val cleanupOnly = parameterFreeView(parameterFreeView(
      parameterBeginDrain(parameterWithViews, SDK04NoFailure), true), false)
    val refusedParams = parameterReleaseProducer(cleanupOnly, false)
    observed.primary == SDK04SyntheticA and observed.code == ProducerAssertionCode and observed.cleanupRefused
      and opaque == observed and failed.views.attempted == List(3, 2)
      and not(parameterCanTransfer(cleanupOnly)) and parameterCanReleaseProducer(cleanupOnly)
      and cleanupOnly.views.primary == SDK04NoFailure
      and parameterReportFailure(refusedParams) == SDK04UnknownFailure
      and parameterReportCode(refusedParams) == ProducerCleanupRefusedCode
      and refusedParams.owner == RefusedParams and refusedParams.paramsFreeAttempts == 1
      and parameterCleanupRefused(refusedParams) and parameterSafety(refusedParams)
  })
  run managedCliZeroWaitsForEveryLogicalBarrierTest = initWith(good).expect({
    val tracked = cliTrackBusiness(cliTrackBusiness(cliTrace(true, false, 0, true)))
    val draining = cliBeginDrain(tracked)
    val first = cliSettleBusiness(draining, true)
    val drained = cliSettleBusiness(first, true)
    val closing = cliBeginClose(drained)
    val closed = cliSettleClose(closing, true)
    val released = cliReleaseTransactions(closed, true)
    val sealed = cliSealSafe(released)
    val created = cliCreateSafe(sealed)
    val durable = cliFinishSafe(created)
    val ready = cliMintLogicalSeal(durable)
    val starting = cliBeginCompletion(ready)
    val oneStream = cliFinishStdout(starting, CliFlushed)
    val both = cliFinishStderr(oneStream, CliFlushed)
    val done = cliPersistCompletion(both, true)
    cliCanTrackBusiness(cliTrace(true, false, 0, true))
      and not(cliCanBeginClose(cliTrace(true, false, 0, true)))
      and not(cliCanTrackBusiness(draining)) and not(cliCanBeginClose(draining)) and not(cliCanBeginClose(first))
      and cliCanBeginClose(drained) and not(cliCanSealSafe(closing))
      and cliCanSettleClose(closing) and cliCanReleaseTransactions(closed)
      and not(cliCanMintLogicalSeal(released)) and cliCanSealSafe(released)
      and cliCanCreateSafe(sealed) and not(cliCanMintLogicalSeal(created))
      and cliCanFinishSafe(created) and cliCanMintLogicalSeal(durable)
      and cliCanBeginCompletion(ready) and cliCanFinishStdout(starting, CliFlushed)
      and not(cliCanFinishStdout(oneStream, CliFlushed)) and cliCanFinishStderr(oneStream, CliFlushed)
      and not(cliCanPersistCompletion(oneStream))
      and cliCanPersistCompletion(both) and done.terminationRequested
      and done.completionDurable and done.exitCode == 0 and cliSafety(done)
      and not(done.sdk.processQuiescent) and not(cliCanPersistCompletion(done))
      and not(cliCanBeginCompletion({...ready, sdk:{...ready.sdk, walletClosed:false}}))
      and not(cliCanBeginCompletion({...ready, transactionsReleased:false}))
      and not(cliCanBeginCompletion({...ready, sdk:{...ready.sdk, safeOutcomeDurable:false}}))
      and not(cliCanBeginCompletion({...ready, pendingBusiness:1}))
  })
  run managedCliRejectedBusinessCannotBecomeZeroTest = initWith(good).expect({
    val ready = cliReady(true, true, 0)
    val both = flushed(ready)
    val done = cliPersistCompletion(both, true)
    cliBusinessSettled(ready) and ready.businessFailed and ready.transactionsReleased
      and cliLogicalSealValid(ready) and cliCanPersistCompletion(both)
      and done.terminationRequested and done.exitCode == 1 and cliSafety(done)
      and cliPersistCompletion(flushed(cliReady(false, false, 0)), true).exitCode == 1
      and cliPersistCompletion({...flushed(cliReady(true, false, 0)), operationFailed:true}, true).exitCode == 1
      and cliPersistCompletion({...flushed(cliReady(true, false, 0)),
        sdk:{...ready.sdk, originalFailure:SDK04SyntheticA}}, true).exitCode == 1
  })
  run managedCliLateBusinessInvalidatesSealDuringStdioTest = initWith(good).expect({
    val ready = cliReady(true, false, 0)
    val one = cliFinishStdout(cliBeginCompletion(ready), CliFlushed)
    val late = cliObserveLateBusiness(one)
    val both = cliFinishStderr(late, CliFlushed)
    cliCanObserveLateBusiness(one) and late.pendingBusiness == 0 and late.lateBusiness
      and late.businessFailed and late.logicalSeal and not(cliLogicalSealValid(late))
      and not(cliCanPersistCompletion(both)) and not(both.terminationRequested)
      and not(both.completionDurable) and cliSafety(both)
  })
  run managedCliLateStreamErrorStaysLatchedAcrossPeerBarrierTest = initWith(good).expect({
    val first = cliFinishStdout(cliBeginCompletion(cliReady(true, false, 0)), CliFlushed)
    val errored = cliObserveStreamError(first)
    val both = cliFinishStderr(errored, CliFlushed)
    val done = cliPersistCompletion(both, true)
    first.stdout == CliFlushed and first.stderr == CliPending
      and not(cliCanPersistCompletion(first)) and cliCanObserveStreamError(first)
      and errored.stdout == CliFlushed and errored.streamFailure
      and both.stdout == CliFlushed and both.stderr == CliFlushed
      and cliCanPersistCompletion(both) and done.exitCode == 1
      and done.terminationRequested and cliSafety(done)
  })
  run managedCliTimeoutRecordedExitAndPersistenceRefusalStaySeparateTest = initWith(good).expect({
    val ready = cliReady(true, false, 0)
    val timed = cliFinishStderr(cliFinishStdout(cliBeginCompletion(ready), CliTimedOut), CliFlushed)
    val timedDone = cliPersistCompletion(timed, true)
    val refused = cliPersistCompletion(flushed(ready), false)
    val recorded = cliPersistCompletion(flushed(cliReady(true, false, 7)), true)
    cliCanPersistCompletion(timed) and timedDone.terminationRequested and timedDone.exitCode == 1
      and recorded.terminationRequested and recorded.exitCode == 7
      and refused.completionAttempted and not(refused.completionDurable)
      and not(refused.terminationRequested) and refused.exitCode == -1
      and not(cliCanPersistCompletion(refused)) and cliSafety(refused)
      and not(cliCanBeginCompletion({...ready, managed:false}))
      and not(cliCanBeginCompletion({...ready, recordedExit:256}))
  })
  run managedCliWalletOrTransactionReleaseRefusalCannotCreateSealTest = initWith(good).expect({
    val closing = cliBeginClose(cliBeginDrain(cliTrace(true, false, 0, true)))
    val closeRefused = cliSettleClose(closing, false)
    val closeOutcome = cliFinishSafe(cliCreateSafe(cliSealSafe(closeRefused)))
    val closed = cliSettleClose(closing, true)
    val releaseRefused = cliReleaseTransactions(closed, false)
    val releaseOutcome = cliFinishSafe(cliCreateSafe(cliSealSafe(releaseRefused)))
    not(cliCanSealSafe(closing)) and cliCanSealSafe(closeRefused)
      and closeOutcome.sdk.safeOutcomeDurable and not(closeOutcome.sdk.walletClosed)
      and not(cliCanMintLogicalSeal(closeOutcome)) and not(cliCanBeginCompletion(closeOutcome))
      and cliCanReleaseTransactions(closed) and releaseOutcome.transactionReleaseAttempted
      and not(releaseOutcome.transactionsReleased) and releaseOutcome.sdk.safeOutcomeDurable
      and not(cliCanMintLogicalSeal(releaseOutcome)) and not(cliCanBeginCompletion(releaseOutcome))
      and cliSafety(closeOutcome) and cliSafety(releaseOutcome)
  })
}
