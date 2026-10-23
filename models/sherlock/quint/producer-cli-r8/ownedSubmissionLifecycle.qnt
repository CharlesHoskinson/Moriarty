module ownedSubmissionLifecycle {
  import ownedNodeValidation.* from "./ownedNodeValidation"
  // Pure lifecycle adapter, not a second transport/SDK/submission engine.
  // Stage08 Request.Open includes CONNECTING: isConnected can still be false.
  type CloseOutcome = ClosePending | CloseComplete | CloseRefused
  type Lifecycle = {created:bool,closing:bool,consumed:bool,clientConstructed:bool,initSettled:bool,
    metadata:Request,current:Request,isConnected:bool,currentCloseProof:bool,
    sdkEntries:int,sdkSettled:bool,cleanupOkay:bool,closeOutcome:CloseOutcome}
  pure val dormant:Lifecycle={created:false,closing:false,consumed:false,
    clientConstructed:false,initSettled:false,metadata:unused,current:unused,isConnected:false,
    currentCloseProof:false,sdkEntries:0,sdkSettled:true,cleanupOkay:true,closeOutcome:ClosePending}
  pure def canCreate(s:Lifecycle):bool=not(s.created) and not(s.closing)
  pure def createOwned(s:Lifecycle):Lifecycle={...s,created:true,
    initSettled:false,metadata:opened(0),current:opened(0)}
  pure def canConsume(s:Lifecycle,finalizedTarget:bool):bool=
    s.created and not(s.closing) and not(s.consumed) and finalizedTarget
  // Actual used=true happens BEFORE await ready; init failure does not reset it.
  pure def consume(s:Lifecycle):Lifecycle={...s,consumed:true}
  pure def metadataDisconnectRequested(s:Lifecycle):Lifecycle={...s,
    metadata:{...s.metadata,socket:Closing,terminated:true},
    current:{...s.current,socket:Closing,terminated:true}}
  pure def closeWitness(r:Request):Request={...r,socket:Closed,settled:true}
  pure def metadataDisconnected(s:Lifecycle):Lifecycle={...s,
    metadata:closeWitness(s.metadata),current:closeWitness(s.current),
    isConnected:false,currentCloseProof:true}
  pure def canConstruct(s:Lifecycle):bool=s.created and not(s.clientConstructed)
    and not(s.initSettled) and s.metadata.socket==Closed and s.metadata.settled
  // Ready may finish constructing while close() waits for ready. Do not overstate the latch.
  pure def constructClient(s:Lifecycle):Lifecycle={...s,clientConstructed:true}
  // ready is a separate task: construction may finish before its promise settles.
  // A failed ready can settle without any client; its cleanup remains separately required.
  pure def canSettleInitSuccess(s:Lifecycle):bool=s.created and s.clientConstructed and not(s.initSettled)
  pure def settleInitSuccess(s:Lifecycle):Lifecycle={...s,initSettled:true}
  pure def canSettleInitFailure(s:Lifecycle):bool=s.created and not(s.clientConstructed) and not(s.initSettled)
  pure def settleInitFailure(s:Lifecycle):Lifecycle={...s,initSettled:true}
  pure def canEnterSdk(s:Lifecycle):bool=s.consumed and s.clientConstructed and s.initSettled
    and not(s.closing) and s.sdkEntries==0
  pure def enterSdk(s:Lifecycle):Lifecycle={...s,sdkEntries:s.sdkEntries+1,
    current:opened(0),currentCloseProof:false,isConnected:false,sdkSettled:false}
  pure def requestClose(s:Lifecycle):Lifecycle={...s,closing:true}
  pure def sdkTaskSettled(s:Lifecycle):Lifecycle={...s,sdkSettled:true}
  pure def currentDisconnected(s:Lifecycle):Lifecycle={...s,
    current:closeWitness(s.current),currentCloseProof:true,isConnected:false}
  pure def closeFastAllowed(s:Lifecycle):bool=s.currentCloseProof and not(s.isConnected)
  pure def canCompleteClose(s:Lifecycle):bool=s.closing and s.sdkSettled and s.cleanupOkay
    and (not(s.created) or (s.initSettled and closeFastAllowed(s)))
  pure def completeClose(s:Lifecycle):Lifecycle={...s,closeOutcome:CloseComplete}
  pure def refuseClose(s:Lifecycle):Lifecycle={...s,cleanupOkay:false,closeOutcome:CloseRefused}
  pure def lifecycleSafety(s:Lifecycle):bool=and {
    s.sdkEntries<=1,
    s.clientConstructed implies s.metadata.socket==Closed and s.metadata.settled,
    s.sdkEntries>0 implies s.consumed and s.clientConstructed and s.initSettled,
    s.closeOutcome==CloseComplete implies canCompleteClose(s),
    s.closeOutcome==CloseRefused implies not(s.cleanupOkay)}
  pure val readyForSdk:Lifecycle=settleInitSuccess(constructClient(metadataDisconnected(consume(createOwned(dormant)))))
  // Conditional read-only catch-up projection. Work/ticks are synthetic, not seconds/cursors.
  // An emission is not an applied-work event. No DUST progress/rate is inferred from R4.
  type CatchupOutcome = CatchupNotWaiting | CatchupWaiting | CatchupTimedOut | CatchupStrictObserved
  type Catchup = {open:bool,savedWork:int,ephemeralWork:int,neededWork:int,budgetTicks:int,
    elapsedTicks:int,emissions:int,strictObserved:bool,closeComplete:bool,
    sendCount:int,feeAssessed:bool,outcome:CatchupOutcome}
  pure val shortCatchup:Catchup={open:false,savedWork:0,ephemeralWork:0,neededWork:3,
    budgetTicks:2,elapsedTicks:0,emissions:0,strictObserved:false,closeComplete:false,
    sendCount:0,feeAssessed:false,outcome:CatchupNotWaiting}
  pure val longerCatchup:Catchup={...shortCatchup,budgetTicks:3}
  pure def canOpenCatchup(s:Catchup):bool=not(s.open)
  // Reopen restores the same unsaved baseline. This is an explicit conditional premise.
  pure def openCatchup(s:Catchup):Catchup={...s,open:true,ephemeralWork:s.savedWork,
    elapsedTicks:0,emissions:0,strictObserved:false,closeComplete:false,outcome:CatchupWaiting}
  pure def canCatchupTick(s:Catchup):bool=s.open and s.outcome==CatchupWaiting
    and s.elapsedTicks<s.budgetTicks
  // true means one ENVIRONMENT-ASSUMED apply event. false selects a stalling tick.
  pure def catchupTick(s:Catchup,assumedAdvance:bool):Catchup={...s,
    elapsedTicks:s.elapsedTicks+1,
    ephemeralWork:(if(assumedAdvance and s.ephemeralWork<s.neededWork) s.ephemeralWork+1 else s.ephemeralWork)}
  pure def canObserveCatchup(s:Catchup,strict:bool):bool=s.open and s.outcome==CatchupWaiting
    and (not(strict) or s.ephemeralWork==s.neededWork)
  // Strict means sync progress only, weaker than the full financial post-state guard.
  pure def observeCatchup(s:Catchup,strict:bool):Catchup={...s,emissions:s.emissions+1,
    strictObserved:strict,outcome:(if(strict) CatchupStrictObserved else CatchupWaiting)}
  pure def canTimeoutCatchup(s:Catchup):bool=s.open and s.outcome==CatchupWaiting
    and s.elapsedTicks>=s.budgetTicks and not(s.strictObserved)
  pure def timeoutCatchup(s:Catchup):Catchup={...s,outcome:CatchupTimedOut}
  pure def canCloseCatchup(s:Catchup):bool=s.open
    and Set(CatchupTimedOut,CatchupStrictObserved).contains(s.outcome)
  // No save or send operation exists in this projection. Ephemeral work is lost on reopen.
  pure def closeCatchup(s:Catchup):Catchup={...s,open:false,closeComplete:true}
  pure def catchupSafety(s:Catchup):bool=and {
    s.savedWork==0,s.ephemeralWork>=s.savedWork,s.ephemeralWork<=s.neededWork,
    s.elapsedTicks>=0,s.elapsedTicks<=s.budgetTicks,s.emissions>=0,
    s.strictObserved iff s.outcome==CatchupStrictObserved,
    s.strictObserved implies s.ephemeralWork==s.neededWork,
    s.closeComplete implies not(s.open) and s.outcome!=CatchupWaiting,
    s.sendCount==0,not(s.feeAssessed)}
  // Production receiver clock projection: exact source thresholds at integer-second instants.
  // Elapsed values and successful semantic checks below are explicit assumptions, not forecasts.
  type ReceiverStage = ReceiverBeforeProof | ReceiverProving | ReceiverAfterProof
    | ReceiverBeforeValidation | ReceiverBeforeSend | ReceiverAfterSend | ReceiverAfterOwnedClose
    | ReceiverAfterReceipt | ReceiverEffectsDone | ReceiverUncertain | ReceiverClosed
  type Receiver = {deadline:int,initialDeadline:int,now:int,ttl:int,initialTtl:int,
    stage:ReceiverStage,sendCount:int,receiptAttempts:int,signedExposed:bool,sendReturned:bool,
    ownedClosed:bool,receiptOkay:bool,effectsOkay:bool,walletClosed:bool,released:bool,
    ambiguous:bool,reverted:bool,postWalletDustKnown:bool,postWalletPendingKnown:bool}
  pure def initialReceiver(subject:int,ttlArg:int):Receiver={deadline:subject,initialDeadline:subject,
    now:0,ttl:ttlArg,initialTtl:ttlArg,stage:ReceiverBeforeProof,sendCount:0,receiptAttempts:0,
    signedExposed:false,sendReturned:false,ownedClosed:false,receiptOkay:false,effectsOkay:false,
    walletClosed:false,released:false,ambiguous:false,reverted:false,
    postWalletDustKnown:false,postWalletPendingKnown:false}
  pure def receiverRemaining(s:Receiver):int=s.deadline-s.now
  pure def receiverTtlRemaining(s:Receiver):int=s.ttl-s.now
  pure def canBeginReceiverProof(s:Receiver):bool=s.stage==ReceiverBeforeProof
    and s.initialDeadline>0 and s.initialDeadline<=1800 and receiverRemaining(s)>=110
    and receiverTtlRemaining(s)>=receiverRemaining(s)
  pure def beginReceiverProof(s:Receiver):Receiver={...s,stage:ReceiverProving}
  pure def canFinishReceiverProof(s:Receiver,cost:int):bool=s.stage==ReceiverProving and cost>=0
  pure def finishReceiverProof(s:Receiver,cost:int):Receiver={...s,now:s.now+cost,stage:ReceiverAfterProof}
  pure def canPassReceiverPostProof(s:Receiver):bool=s.stage==ReceiverAfterProof and receiverRemaining(s)>=150
  // Collapses only successful fresh-fee/full financial inspection/durable preparation.
  // Original financial predicates remain separate premises; TTL110 is checked after this cost.
  pure def canPrepareReceiverValidation(s:Receiver,cost:int):bool=canPassReceiverPostProof(s)
    and cost>=0 and receiverTtlRemaining(s)-cost>=110
  pure def prepareReceiverValidation(s:Receiver,cost:int):Receiver={...s,now:s.now+cost,stage:ReceiverBeforeValidation}
  pure def canValidateReceiver(s:Receiver,cost:int):bool=s.stage==ReceiverBeforeValidation
    and cost>=0 and receiverRemaining(s)>=130 and receiverTtlRemaining(s)>=130
  pure def validateReceiver(s:Receiver,cost:int,successful:bool):Receiver={...s,now:s.now+cost,
    signedExposed:true,ambiguous:not(successful),stage:(if(successful) ReceiverBeforeSend else ReceiverUncertain)}
  pure def canSendReceiver(s:Receiver,cost:int):bool=s.stage==ReceiverBeforeSend and cost>=0
    and receiverRemaining(s)>=110 and receiverTtlRemaining(s)>=110 and s.sendCount==0
  // cost20 in the witness is the source timer setting under explicit scheduling assumptions.
  pure def sendReceiver(s:Receiver,cost:int,returned:bool):Receiver={...s,now:s.now+cost,
    sendCount:s.sendCount+1,sendReturned:returned,ambiguous:true,stage:ReceiverAfterSend}
  pure def canCloseReceiverOwned(s:Receiver,cost:int):bool=s.stage==ReceiverAfterSend and cost>=0
  pure def closeReceiverOwned(s:Receiver,cost:int,successful:bool):Receiver={...s,now:s.now+cost,
    ownedClosed:successful,stage:ReceiverAfterOwnedClose}
  // The source performs one lookup whether send returned or threw; close failure does not skip it.
  pure def canReadReceiverReceipt(s:Receiver,cost:int):bool=s.stage==ReceiverAfterOwnedClose
    and cost>=0 and s.receiptAttempts==0
  pure def readReceiverReceipt(s:Receiver,cost:int,exactAccepted:bool):Receiver={...s,now:s.now+cost,
    receiptAttempts:s.receiptAttempts+1,receiptOkay:exactAccepted,ambiguous:not(exactAccepted),
    stage:(if(exactAccepted) ReceiverAfterReceipt else ReceiverUncertain)}
  pure def canReadReceiverEffects(s:Receiver,cost:int):bool=s.stage==ReceiverAfterReceipt
    and cost>=0 and s.receiptOkay
  pure def readReceiverEffects(s:Receiver,cost:int,completeAccepted:bool):Receiver={...s,now:s.now+cost,
    effectsOkay:completeAccepted,stage:(if(completeAccepted) ReceiverEffectsDone else ReceiverUncertain)}
  pure def canCloseReceiverWallet(s:Receiver,cost:int):bool=cost>=0
    and Set(ReceiverEffectsDone,ReceiverUncertain).contains(s.stage)
  pure def closeReceiverWallet(s:Receiver,cost:int,successful:bool,releaseSuccessful:bool):Receiver={...s,
    now:s.now+cost,walletClosed:successful,released:successful and releaseSuccessful,stage:ReceiverClosed}
  pure def receiverAccepted(s:Receiver):bool=s.stage==ReceiverClosed and s.receiptOkay and s.effectsOkay
    and s.ownedClosed and s.walletClosed and s.released
  // A separate scope observation, not an added product acceptance gate.
  pure def receiverWithinSubject(s:Receiver):bool=s.now<=s.deadline
  pure def receiverSafety(s:Receiver):bool=and {
    s.deadline==s.initialDeadline,s.ttl==s.initialTtl,s.now>=0,
    s.sendCount>=0,s.sendCount<=1,s.receiptAttempts>=0,s.receiptAttempts<=1,
    s.sendCount>0 implies s.signedExposed,
    s.sendReturned implies s.sendCount==1,
    s.receiptAttempts>0 implies s.sendCount==1,
    s.receiptOkay implies s.receiptAttempts==1 and not(s.ambiguous),
    s.effectsOkay implies s.receiptOkay,
    s.released implies s.walletClosed,
    not(s.reverted),not(s.postWalletDustKnown),not(s.postWalletPendingKnown)}

  // D3 selected whole-second clock projection: 300000ms wait within 600000ms subject.
  // Open cost includes dispatch/startup/open/receipt-read prefix through timer installation.
  // No SDK, rate, real-time fit or complete-R5 derivation is supplied by this model.
  type D3Readiness = {fullR5Filter:bool,syncFlag:bool,shielded:bool,
    unshielded:bool,dust:bool,aggregates:bool}
  pure val d3NotReady:D3Readiness={fullR5Filter:false,syncFlag:false,
    shielded:false,unshielded:false,dust:false,aggregates:false}
  pure val d3FullReady:D3Readiness={fullR5Filter:true,syncFlag:true,
    shielded:true,unshielded:true,dust:true,aggregates:true}
  pure def d3Ready(r:D3Readiness):bool=r.fullR5Filter and r.syncFlag
    and r.shielded and r.unshielded and r.dust and r.aggregates
  type D3WaitPhase = D3Opening | D3Waiting | D3Ready | D3TimedOut | D3Refused
  type D3Input = {now:int,waitStartedAt:int,phase:D3WaitPhase,readiness:D3Readiness,
    dustConnected:bool,dustAdvanced:bool,walletClosed:bool}
  pure val d3Input:D3Input={now:0,waitStartedAt:0,phase:D3Opening,
    readiness:d3NotReady,dustConnected:false,dustAdvanced:false,walletClosed:false}
  pure val d3WaitMilliseconds:int=300000
  pure val d3SubjectMilliseconds:int=600000
  pure def d3CanOpen(s:D3Input,cost:int):bool=s.phase==D3Opening
    and cost>=0 and (s.now+cost)*1000<=d3SubjectMilliseconds
  pure def d3Open(s:D3Input,cost:int):D3Input={...s,now:s.now+cost,
    waitStartedAt:s.now+cost,phase:D3Waiting}
  pure def d3CanAdvanceWait(s:D3Input,cost:int):bool=s.phase==D3Waiting
    and cost>=0 and (s.now+cost)*1000<=d3SubjectMilliseconds
  pure def d3AdvanceWait(s:D3Input,cost:int):D3Input={...s,now:s.now+cost}
  pure def d3CanObserve(s:D3Input):bool=s.phase==D3Waiting
  // Full filter/strict/aggregate success is collapsed into one assumed observation.
  // The timer and ready event can both be eligible: the first scheduled terminal wins.
  pure def d3Observe(s:D3Input,r:D3Readiness,connected:bool,advanced:bool):D3Input={...s,
    readiness:r,dustConnected:connected,dustAdvanced:advanced,
    phase:(if(d3Ready(r)) D3Ready else if(r.fullR5Filter and r.syncFlag) D3Refused else D3Waiting)}
  pure def d3CanTimeout(s:D3Input):bool=s.phase==D3Waiting
    and (s.now-s.waitStartedAt)*1000>=d3WaitMilliseconds
  pure def d3Timeout(s:D3Input):D3Input={...s,phase:D3TimedOut}
  pure def d3CanEnterFinancial(s:D3Input):bool=s.phase==D3Ready
    and d3Ready(s.readiness) and not(s.walletClosed)
  // Source creates deploy TTL after waitSynced. Absolute operation deadline stays600.
  pure def d3FinancialReceiver(s:D3Input):Receiver={...initialReceiver(600,s.now+4800),now:s.now}
  pure def d3CanCloseInput(s:D3Input):bool=Set(D3Ready,D3TimedOut,D3Refused).contains(s.phase)
    and not(s.walletClosed)
  pure def d3CloseInput(s:D3Input):D3Input={...s,walletClosed:true}
  pure def d3InputSafety(s:D3Input):bool=and {
    s.now>=0,s.now*1000<=d3SubjectMilliseconds,s.waitStartedAt<=s.now,
    s.phase==D3Ready implies d3Ready(s.readiness),
    s.walletClosed implies Set(D3Ready,D3TimedOut,D3Refused).contains(s.phase)}

  // Narrow outer publication projection. These are observed cleanup facts, not guesses.
  // provisionalComplete corresponds to the pre-postpins/scratch COMPLETE_ label in6241.
  type D3Publication = {terminal:bool,cgroupEmpty:bool,launcherReaped:bool,
    controlsReaped:bool,ownershipKnown:bool,postControlOkay:bool,provisionalComplete:bool,
    postPinsChecked:bool,postPinsOkay:bool,scratchChecked:bool,scratchEmpty:bool,
    unaccepted:bool,authenticDiagnostic:bool,costOutputAbsent:bool,published:bool}
  pure val d3PublicationPending:D3Publication={terminal:false,cgroupEmpty:false,
    launcherReaped:false,controlsReaped:false,ownershipKnown:false,postControlOkay:false,
    provisionalComplete:false,postPinsChecked:false,postPinsOkay:false,
    scratchChecked:false,scratchEmpty:false,unaccepted:true,authenticDiagnostic:true,
    costOutputAbsent:true,published:false}
  pure def d3OwnedCleanupObserved(s:D3Publication):bool=s.terminal and s.cgroupEmpty
    and s.launcherReaped and s.controlsReaped and s.ownershipKnown and s.postControlOkay
  // Handcrafted boundary fixture: actual observations remain external premises.
  pure val d3OwnedReaped:D3Publication={...d3PublicationPending,terminal:true,cgroupEmpty:true,
    launcherReaped:true,controlsReaped:true,ownershipKnown:true,postControlOkay:true,
    provisionalComplete:true}
  pure def d3CanCheckPostPins(s:D3Publication):bool=d3OwnedCleanupObserved(s)
    and s.provisionalComplete and not(s.postPinsChecked)
  pure def d3CheckPostPins(s:D3Publication,okay:bool):D3Publication={...s,
    postPinsChecked:true,postPinsOkay:okay}
  pure def d3CanCheckScratch(s:D3Publication):bool=s.postPinsChecked and not(s.scratchChecked)
  pure def d3CheckScratch(s:D3Publication,empty:bool):D3Publication={...s,
    scratchChecked:true,scratchEmpty:empty,provisionalComplete:s.provisionalComplete and empty}
  // Current6241 conditional successful projection attempt; intentionally too early.
  pure def d3LegacyPublicationAllowed(s:D3Publication):bool=s.provisionalComplete
    and s.costOutputAbsent and s.unaccepted and s.authenticDiagnostic
  // Corrected46b07229 source rule; source correspondence is not runtime observation.
  pure def d3PublicationPrerequisites(s:D3Publication):bool=d3OwnedCleanupObserved(s)
    and s.provisionalComplete and s.postPinsChecked and s.postPinsOkay
    and s.scratchChecked and s.scratchEmpty and s.unaccepted
    and s.authenticDiagnostic and s.costOutputAbsent
  pure def d3CanPublish(s:D3Publication):bool=d3PublicationPrerequisites(s) and not(s.published)
  pure def d3Publish(s:D3Publication):D3Publication={...s,published:true}
  pure def d3PublicationSafety(s:D3Publication):bool=s.published implies d3PublicationPrerequisites(s)

  // SDK04 failure-prefix counterpart; no SDK execution/refinement or cause attribution.
  // Stage transitions mean the named actual producer call/effect returned or occurred.
  // FinalCreated splits file creation from completion of write/fsync/parent fsync.
  type SDK04Local = SDK04BeforeBalance | SDK04BalanceReturned | SDK04SignReturned
    | SDK04FinalizeReturned | SDK04SerializeReturned | SDK04FinalCreated
    | SDK04FinalDurable | SDK04FreshDirectory
  type SDK04Failure = SDK04NoFailure | SDK04UnknownFailure | SDK04SyntheticA | SDK04SyntheticB
  type SDK04Trace = {local:SDK04Local,financialGuardPassed:bool,recoveryDurable:bool,
    signedRpcStarted:bool,ownedSubmitStarted:bool,originalFailure:SDK04Failure,
    closeAttempted:bool,closeSettled:bool,walletClosed:bool,snapshotCaptured:bool,
    snapshotOriginal:SDK04Failure,safeOutcomeCreated:bool,safeOutcomeDurable:bool,
    processQuiescent:bool,outerDeadline:bool}
  pure val sdk04Trace:SDK04Trace={local:SDK04BeforeBalance,financialGuardPassed:false,
    recoveryDurable:false,signedRpcStarted:false,ownedSubmitStarted:false,
    originalFailure:SDK04NoFailure,closeAttempted:false,closeSettled:false,walletClosed:false,
    snapshotCaptured:false,snapshotOriginal:SDK04NoFailure,safeOutcomeCreated:false,
    safeOutcomeDurable:false,processQuiescent:false,outerDeadline:false}
  pure def sdk04Live(s:SDK04Trace):bool=not(s.processQuiescent)
  // withWallet awaits its callback before finally-close; close latches new business entry.
  // Earlier RPC/send history remains valid after close and is never cleared.
  pure def sdk04BusinessOpen(s:SDK04Trace):bool=sdk04Live(s) and not(s.closeAttempted)
  pure def sdk04CanAdvanceLocal(s:SDK04Trace,postProofReserveOkay:bool):bool=
    sdk04BusinessOpen(s) and s.originalFailure==SDK04NoFailure and s.local!=SDK04FreshDirectory
      and (s.local!=SDK04FinalDurable or postProofReserveOkay)
  pure def sdk04AdvanceLocal(s:SDK04Trace):SDK04Trace={...s,local:match s.local {
    | SDK04BeforeBalance=>SDK04BalanceReturned
    | SDK04BalanceReturned=>SDK04SignReturned
    | SDK04SignReturned=>SDK04FinalizeReturned
    | SDK04FinalizeReturned=>SDK04SerializeReturned
    | SDK04SerializeReturned=>SDK04FinalCreated
    | SDK04FinalCreated=>SDK04FinalDurable
    | SDK04FinalDurable=>SDK04FreshDirectory
    | SDK04FreshDirectory=>SDK04FreshDirectory }}
  pure def sdk04PrefixReturned(s:SDK04Trace):bool=Set(SDK04SerializeReturned,SDK04FinalCreated,
    SDK04FinalDurable,SDK04FreshDirectory).contains(s.local)
  pure def sdk04FinalWriteReturned(s:SDK04Trace):bool=
    Set(SDK04FinalDurable,SDK04FreshDirectory).contains(s.local)
  pure def sdk04CanRecordGuard(s:SDK04Trace,fullGuardSucceeded:bool):bool=sdk04BusinessOpen(s)
    and s.originalFailure==SDK04NoFailure and s.local==SDK04FreshDirectory
    and not(s.financialGuardPassed) and fullGuardSucceeded
  pure def sdk04RecordGuard(s:SDK04Trace):SDK04Trace={...s,financialGuardPassed:true}
  // recoveryDurable means financial prepared/recovery/LEDGER + full tree/parent completed.
  pure def sdk04CanRecordRecovery(s:SDK04Trace,allDurable:bool):bool=sdk04BusinessOpen(s)
    and s.originalFailure==SDK04NoFailure and s.financialGuardPassed
    and not(s.recoveryDurable) and allDurable
  pure def sdk04RecordRecovery(s:SDK04Trace):SDK04Trace={...s,recoveryDurable:true}
  pure def sdk04CanStartSignedRpc(s:SDK04Trace,reserveAndTtlOkay:bool):bool=sdk04BusinessOpen(s)
    and s.originalFailure==SDK04NoFailure and s.financialGuardPassed and s.recoveryDurable
    and not(s.signedRpcStarted) and reserveAndTtlOkay
  pure def sdk04StartSignedRpc(s:SDK04Trace):SDK04Trace={...s,signedRpcStarted:true}
  pure def sdk04CanStartOwnedSubmit(s:SDK04Trace,validationAndReserveOkay:bool):bool=
    sdk04BusinessOpen(s) and s.originalFailure==SDK04NoFailure and s.financialGuardPassed
      and s.recoveryDurable and s.signedRpcStarted and not(s.ownedSubmitStarted)
      and validationAndReserveOkay
  pure def sdk04StartOwnedSubmit(s:SDK04Trace):SDK04Trace={...s,ownedSubmitStarted:true}
  // Actual observer records only the first failure; labels here are synthetic, never raw errors.
  pure def sdk04CanObserveFailure(s:SDK04Trace,f:SDK04Failure):bool=sdk04Live(s)
    and f!=SDK04NoFailure
  pure def sdk04ObserveFailure(s:SDK04Trace,f:SDK04Failure):SDK04Trace={...s,
    originalFailure:(if(s.originalFailure==SDK04NoFailure) f else s.originalFailure)}
  // Controller deadline is an independent observation, not the original failure cause or reap.
  pure def sdk04OuterDeadlineObserved(s:SDK04Trace):SDK04Trace={...s,outerDeadline:true}
  pure def sdk04CanBeginClose(s:SDK04Trace):bool=sdk04Live(s) and not(s.closeAttempted)
  pure def sdk04BeginClose(s:SDK04Trace):SDK04Trace={...s,closeAttempted:true}
  pure def sdk04CanSettleClose(s:SDK04Trace):bool=sdk04Live(s) and s.closeAttempted and not(s.closeSettled)
  pure def sdk04SettleClose(s:SDK04Trace,successful:bool):SDK04Trace={...s,
    closeSettled:true,walletClosed:successful}
  // Selected opened-wallet path: await close settles (success OR throw) before nested seal.
  pure def sdk04CanSeal(s:SDK04Trace):bool=sdk04Live(s) and s.closeSettled and not(s.snapshotCaptured)
  pure def sdk04Seal(s:SDK04Trace):SDK04Trace={...s,snapshotCaptured:true,snapshotOriginal:s.originalFailure}
  pure def sdk04CanCreateSafeOutcome(s:SDK04Trace):bool=sdk04Live(s)
    and s.snapshotCaptured and not(s.safeOutcomeCreated)
  pure def sdk04CreateSafeOutcome(s:SDK04Trace):SDK04Trace={...s,safeOutcomeCreated:true}
  pure def sdk04CanFinishSafeOutcome(s:SDK04Trace):bool=sdk04Live(s)
    and s.safeOutcomeCreated and not(s.safeOutcomeDurable)
  // Successful whole seal write/fsync is stronger than file presence, still weaker than quiescence.
  pure def sdk04FinishSafeOutcome(s:SDK04Trace):SDK04Trace={...s,safeOutcomeDurable:true}
  pure def sdk04ObserveQuiescence(s:SDK04Trace):SDK04Trace={...s,processQuiescent:true}
  type SDK04Metadata = {finalTransactionPresent:bool,freshParameterDirectoryPresent:bool,
    safeOutcomePresent:bool,originalFailureKnown:bool,validityEstablished:bool,
    financialAcceptance:bool,refundAuthorized:bool,overallExposureKnown:bool}
  // Conservative unread metadata projection, NOT a decoder of the optional safe payload.
  pure def sdk04Metadata(s:SDK04Trace):SDK04Metadata={
    finalTransactionPresent:Set(SDK04FinalCreated,SDK04FinalDurable,SDK04FreshDirectory).contains(s.local),
    freshParameterDirectoryPresent:s.local==SDK04FreshDirectory,safeOutcomePresent:s.safeOutcomeCreated,
    originalFailureKnown:false,validityEstablished:false,financialAcceptance:false,
    refundAuthorized:false,overallExposureKnown:false}
  pure def sdk04Safety(s:SDK04Trace):bool=and {
    s.signedRpcStarted implies s.financialGuardPassed and s.recoveryDurable,
    s.ownedSubmitStarted implies s.signedRpcStarted and s.financialGuardPassed and s.recoveryDurable,
    s.recoveryDurable implies s.financialGuardPassed,
    s.financialGuardPassed implies s.local==SDK04FreshDirectory,
    s.closeSettled implies s.closeAttempted,s.walletClosed implies s.closeSettled,
    s.snapshotCaptured implies s.closeSettled,
    s.safeOutcomeCreated implies s.snapshotCaptured,
    s.safeOutcomeDurable implies s.safeOutcomeCreated}
}
