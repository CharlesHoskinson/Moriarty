module producerCliLifecycle {
  import ownedSubmissionLifecycle.* from "./ownedSubmissionLifecycle"

  // Source-only supplied-record counterpart. Can guards are caller obligations.
  // IDs are representative identities, not SDK pointers or a complete resource census.
  type ProducerScope = {borrowed:Set[int], owned:List[int], seen:Set[int], attempted:List[int],
    draining:bool, primary:SDK04Failure, cleanupRefused:bool}
  pure def producerScope(borrowed:Set[int]):ProducerScope = {borrowed:borrowed, owned:List(),
    seen:Set(), attempted:List(), draining:false, primary:SDK04NoFailure, cleanupRefused:false}
  pure def producerCanOwn(s:ProducerScope, id:int):bool = not(s.draining) and id >= 0 and id <= 3
  pure def producerOwn(s:ProducerScope, id:int):ProducerScope =
    if (s.borrowed.contains(id) or s.seen.contains(id)) s
    else {...s, owned:s.owned.append(id), seen:s.seen.union(Set(id))}
  pure def producerCanBeginDrain(s:ProducerScope):bool = not(s.draining)
  pure def producerBeginDrain(s:ProducerScope, primary:SDK04Failure):ProducerScope =
    {...s, draining:true, primary:primary}
  pure def producerCanFree(s:ProducerScope):bool = s.draining and s.attempted.length() < s.owned.length()
  // A failed free/getter is an attempted cleanup; it never skips the next sibling.
  pure def producerFree(s:ProducerScope, successful:bool):ProducerScope = {...s,
    attempted:s.attempted.append(s.owned.nth(s.owned.length() - s.attempted.length() - 1)),
    cleanupRefused:s.cleanupRefused or not(successful)}
  pure def producerDrained(s:ProducerScope):bool = s.draining and s.attempted.length() == s.owned.length()
  pure def producerReturned(s:ProducerScope):bool = producerDrained(s)
    and s.primary == SDK04NoFailure and not(s.cleanupRefused)
  pure def producerSafety(s:ProducerScope):bool = and {
    s.owned.length() == s.seen.size(), s.seen.intersect(s.borrowed).size() == 0,
    s.seen.forall(id => id >= 0 and id <= 3),
    s.owned.indices().forall(i => s.seen.contains(s.owned.nth(i))),
    s.seen.forall(id => s.owned.indices().exists(i => s.owned.nth(i) == id)),
    not(s.draining) implies s.attempted.length() == 0 and s.primary == SDK04NoFailure and not(s.cleanupRefused),
    s.cleanupRefused implies s.draining and s.attempted.length() > 0,
    if (s.attempted.length() <= s.owned.length())
      s.attempted.indices().forall(i => s.attempted.nth(i) == s.owned.nth(s.owned.length() - i - 1))
    else false }

  type ParameterOwner = NoParams | ProducerParams | CallerParams | ReleasedParams | RefusedParams
  type ParameterRecord = {owner:ParameterOwner, views:ProducerScope, transferred:bool, paramsFreeAttempts:int}
  pure val parameterStart:ParameterRecord = {owner:NoParams, views:producerScope(Set(0, 1)),
    transferred:false, paramsFreeAttempts:0}
  pure def parameterCanDeserialize(s:ParameterRecord):bool = s.owner == NoParams
  // Owner installation immediately follows a successfully returned deserialize.
  pure def parameterDeserialize(s:ParameterRecord):ParameterRecord = {...s, owner:ProducerParams}
  pure def parameterCanOwnView(s:ParameterRecord, id:int):bool = s.owner == ProducerParams
    and producerCanOwn(s.views, id)
  pure def parameterOwnView(s:ParameterRecord, id:int):ParameterRecord =
    {...s, views:producerOwn(s.views, id)}
  pure def parameterCanBeginDrain(s:ParameterRecord):bool = s.owner == ProducerParams
    and producerCanBeginDrain(s.views)
  // NoFailure means the complete record body returned; other labels mean pre-return failure.
  pure def parameterBeginDrain(s:ParameterRecord, primary:SDK04Failure):ParameterRecord =
    {...s, views:producerBeginDrain(s.views, primary)}
  pure def parameterCanFreeView(s:ParameterRecord):bool = s.owner == ProducerParams and producerCanFree(s.views)
  pure def parameterFreeView(s:ParameterRecord, successful:bool):ParameterRecord =
    {...s, views:producerFree(s.views, successful)}
  pure def parameterCanTransfer(s:ParameterRecord):bool = s.owner == ProducerParams and producerReturned(s.views)
  pure def parameterTransfer(s:ParameterRecord):ParameterRecord = {...s, owner:CallerParams, transferred:true}
  pure def parameterCanReleaseProducer(s:ParameterRecord):bool = s.owner == ProducerParams
    and producerDrained(s.views) and not(producerReturned(s.views))
  pure def parameterReleaseProducer(s:ParameterRecord, successful:bool):ParameterRecord = {...s,
    owner:(if (successful) ReleasedParams else RefusedParams), paramsFreeAttempts:s.paramsFreeAttempts + 1}
  // Later withFreshParameters caller scope owns only a successfully transferred record.
  pure def parameterCanReleaseCaller(s:ParameterRecord):bool = s.owner == CallerParams
  pure def parameterReleaseCaller(s:ParameterRecord, successful:bool):ParameterRecord =
    parameterReleaseProducer(s, successful)
  pure def parameterCleanupRefused(s:ParameterRecord):bool = s.views.cleanupRefused or s.owner == RefusedParams
  // This reports an abstract identity; cleanup-only's public code is known separately below.
  pure def parameterReportFailure(s:ParameterRecord):SDK04Failure =
    if (s.views.primary != SDK04NoFailure) s.views.primary
    else if (parameterCleanupRefused(s)) SDK04UnknownFailure else SDK04NoFailure
  pure def parameterSafety(s:ParameterRecord):bool = and {
    producerSafety(s.views), s.paramsFreeAttempts >= 0, s.paramsFreeAttempts <= 1,
    s.transferred implies producerReturned(s.views),
    s.owner == CallerParams implies s.transferred and s.paramsFreeAttempts == 0,
    Set(ReleasedParams, RefusedParams).contains(s.owner) iff s.paramsFreeAttempts == 1,
    s.paramsFreeAttempts == 1 implies producerDrained(s.views),
    s.owner == ProducerParams implies not(s.transferred) and s.paramsFreeAttempts == 0,
    s.owner == NoParams implies not(s.transferred) and s.paramsFreeAttempts == 0 and s.views.owned.length() == 0 }
  type ProducerFixedCode = ProducerNoCode | ProducerAssertionCode | ProducerUnclassifiedCode | ProducerCleanupRefusedCode
  // SyntheticA stands for the directed assertion fixture; identity is not a raw error decoder.
  pure def parameterReportCode(s:ParameterRecord):ProducerFixedCode =
    if (s.views.primary == SDK04SyntheticA) ProducerAssertionCode
    else if (s.views.primary != SDK04NoFailure) ProducerUnclassifiedCode
    else if (parameterCleanupRefused(s)) ProducerCleanupRefusedCode else ProducerNoCode
  type ProducerObservation = {primary:SDK04Failure, code:ProducerFixedCode, cleanupRefused:bool}
  pure val producerObservation:ProducerObservation =
    {primary:SDK04NoFailure, code:ProducerNoCode, cleanupRefused:false}
  // This is the typed callback BEFORE the opaque vendor wrapper, not SDK cause traversal.
  pure def producerObserve(s:ProducerObservation, primary:SDK04Failure,
      code:ProducerFixedCode, cleanup:bool):ProducerObservation =
    {primary:(if (s.primary == SDK04NoFailure) primary else s.primary),
    code:(if (s.code == ProducerNoCode) code else s.code), cleanupRefused:s.cleanupRefused or cleanup}

  type CliStream = CliPending | CliFlushed | CliFailed | CliTimedOut
  type CliTrace = {sdk:SDK04Trace, pendingBusiness:int, drainStarted:bool, lateBusiness:bool,
    businessFailed:bool, transactionReleaseAttempted:bool, transactionsReleased:bool,
    logicalSeal:bool, acceptedOutcome:bool, operationFailed:bool, recordedExit:int, managed:bool,
    stdioStarted:bool, stdout:CliStream, stderr:CliStream, streamFailure:bool,
    completionAttempted:bool, completionDurable:bool, terminationRequested:bool, exitCode:int}
  // acceptedOutcome is an EXTERNAL full receipt/effects/owned-close premise, not financial verification.
  pure def cliTrace(accepted:bool, operationFailed:bool, recorded:int, managed:bool):CliTrace =
    {sdk:sdk04Trace, pendingBusiness:0, drainStarted:false, lateBusiness:false, businessFailed:false,
    transactionReleaseAttempted:false, transactionsReleased:false, logicalSeal:false,
    acceptedOutcome:accepted, operationFailed:operationFailed, recordedExit:recorded, managed:managed,
    stdioStarted:false, stdout:CliPending, stderr:CliPending, streamFailure:false,
    completionAttempted:false, completionDurable:false, terminationRequested:false, exitCode:-1}
  // Two pending callbacks is a directed-fixture bound, not a production restriction.
  pure def cliCanTrackBusiness(s:CliTrace):bool = not(s.drainStarted) and s.pendingBusiness < 2
  pure def cliTrackBusiness(s:CliTrace):CliTrace = {...s, pendingBusiness:s.pendingBusiness + 1}
  pure def cliCanBeginDrain(s:CliTrace):bool = not(s.drainStarted)
  pure def cliBeginDrain(s:CliTrace):CliTrace = {...s, drainStarted:true}
  pure def cliCanSettleBusiness(s:CliTrace):bool = s.pendingBusiness > 0
  pure def cliSettleBusiness(s:CliTrace, successful:bool):CliTrace = {...s,
    pendingBusiness:s.pendingBusiness - 1, businessFailed:s.businessFailed or not(successful)}
  pure def cliCanObserveLateBusiness(s:CliTrace):bool = s.drainStarted and not(s.completionAttempted)
  pure def cliObserveLateBusiness(s:CliTrace):CliTrace = {...s, lateBusiness:true, businessFailed:true}
  pure def cliBusinessSettled(s:CliTrace):bool = s.drainStarted and s.pendingBusiness == 0 and not(s.lateBusiness)
  // drain() waits pending tasks even when a rejection/late attempt makes successful seal impossible.
  pure def cliCanBeginClose(s:CliTrace):bool = s.drainStarted and s.pendingBusiness == 0 and sdk04CanBeginClose(s.sdk)
  pure def cliBeginClose(s:CliTrace):CliTrace = {...s, sdk:sdk04BeginClose(s.sdk)}
  pure def cliCanSettleClose(s:CliTrace):bool = sdk04CanSettleClose(s.sdk)
  pure def cliSettleClose(s:CliTrace, successful:bool):CliTrace = {...s, sdk:sdk04SettleClose(s.sdk, successful)}
  pure def cliCanReleaseTransactions(s:CliTrace):bool = s.sdk.walletClosed and cliBusinessSettled(s)
    and not(s.transactionReleaseAttempted)
  pure def cliReleaseTransactions(s:CliTrace, successful:bool):CliTrace = {...s,
    transactionReleaseAttempted:true, transactionsReleased:successful}
  pure def cliCanSealSafe(s:CliTrace):bool = sdk04CanSeal(s.sdk)
    and (not(s.sdk.walletClosed and cliBusinessSettled(s)) or s.transactionReleaseAttempted)
  pure def cliSealSafe(s:CliTrace):CliTrace = {...s, sdk:sdk04Seal(s.sdk)}
  pure def cliCanCreateSafe(s:CliTrace):bool = sdk04CanCreateSafeOutcome(s.sdk)
  pure def cliCreateSafe(s:CliTrace):CliTrace = {...s, sdk:sdk04CreateSafeOutcome(s.sdk)}
  pure def cliCanFinishSafe(s:CliTrace):bool = sdk04CanFinishSafeOutcome(s.sdk)
  pure def cliFinishSafe(s:CliTrace):CliTrace = {...s, sdk:sdk04FinishSafeOutcome(s.sdk)}
  pure def cliCanMintLogicalSeal(s:CliTrace):bool = s.sdk.walletClosed and s.transactionsReleased
    and cliBusinessSettled(s) and s.sdk.safeOutcomeDurable and not(s.logicalSeal)
  pure def cliMintLogicalSeal(s:CliTrace):CliTrace = {...s, logicalSeal:true}
  pure def cliLogicalSealValid(s:CliTrace):bool = s.logicalSeal and s.sdk.walletClosed
    and s.transactionsReleased and cliBusinessSettled(s) and s.sdk.safeOutcomeDurable
  pure def cliRecordedExitValid(s:CliTrace):bool = s.recordedExit >= 0 and s.recordedExit <= 255
  pure def cliCanBeginCompletion(s:CliTrace):bool = s.managed and cliRecordedExitValid(s)
    and cliLogicalSealValid(s) and not(s.stdioStarted)
  pure def cliBeginCompletion(s:CliTrace):CliTrace = {...s, stdioStarted:true}
  pure def cliCanFinishStdout(s:CliTrace, status:CliStream):bool = s.stdioStarted
    and s.stdout == CliPending and status != CliPending and not(s.completionAttempted)
  pure def cliFinishStdout(s:CliTrace, status:CliStream):CliTrace = {...s, stdout:status,
    streamFailure:s.streamFailure or status == CliFailed}
  pure def cliCanFinishStderr(s:CliTrace, status:CliStream):bool = s.stdioStarted
    and s.stderr == CliPending and status != CliPending and not(s.completionAttempted)
  pure def cliFinishStderr(s:CliTrace, status:CliStream):CliTrace = {...s, stderr:status,
    streamFailure:s.streamFailure or status == CliFailed}
  // Late error after a successful local callback; initial errors use Finish*(CliFailed).
  pure def cliCanObserveStreamError(s:CliTrace):bool = s.stdioStarted and not(s.completionAttempted)
    and (s.stdout == CliFlushed or s.stderr == CliFlushed)
  pure def cliObserveStreamError(s:CliTrace):CliTrace = {...s, streamFailure:true}
  pure def cliCanPersistCompletion(s:CliTrace):bool = s.managed and cliRecordedExitValid(s)
    and s.stdioStarted and s.stdout != CliPending and s.stderr != CliPending
    and cliLogicalSealValid(s) and not(s.completionAttempted)
  pure def cliZeroPrerequisites(s:CliTrace):bool = cliLogicalSealValid(s) and s.acceptedOutcome
    and not(s.operationFailed) and not(s.businessFailed) and s.sdk.originalFailure == SDK04NoFailure
    and s.stdout == CliFlushed and s.stderr == CliFlushed and not(s.streamFailure) and s.recordedExit == 0
  pure def cliSelectedExit(s:CliTrace):int =
    if (s.recordedExit != 0) s.recordedExit else if (cliZeroPrerequisites(s)) 0 else 1
  // Synchronous persist-return then terminate has no awaited/interleavable gap in the helper.
  // A refusal (including non-undefined return) requests no explicit termination.
  pure def cliPersistCompletion(s:CliTrace, synchronousDurableSuccess:bool):CliTrace = {...s,
    completionAttempted:true, completionDurable:synchronousDurableSuccess,
    terminationRequested:synchronousDurableSuccess,
    exitCode:(if (synchronousDurableSuccess) cliSelectedExit(s) else -1)}
  pure def cliSafety(s:CliTrace):bool = and {
    sdk04Safety(s.sdk), s.pendingBusiness >= 0, s.pendingBusiness <= 2,
    s.transactionsReleased implies s.transactionReleaseAttempted,
    s.completionDurable implies s.completionAttempted,
    s.terminationRequested implies s.completionDurable and cliLogicalSealValid(s)
      and s.stdioStarted and s.stdout != CliPending and s.stderr != CliPending,
    s.terminationRequested implies s.exitCode >= 0 and s.exitCode <= 255,
    s.terminationRequested and s.exitCode == 0 implies cliZeroPrerequisites(s) }
}
