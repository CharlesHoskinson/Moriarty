module recoveryExposure {
  // PUBLIC_SYNTHETIC. Exposure flag is POSSIBILITY at ADMISSION/PENDING, not observed delivery.
  // modeledExplicitAttempts counts abstract model actions, never observed SDK/network calls.
  type Route = Source28ExposureFirst | RequiredRecoveryFirst
  type Fault = NoFault | FinancialReject | RecordWriteFailure | FsyncFailure
    | NodeInvalid | NodeUnknown | NodeTimeout | OwnDispatchAmbiguous | PostValidationFailure
  type Phase = FinancialGuard | WriteRecord | PersistRecovery | ArmOwnership | Expose
    | AwaitVerdict | OwnSubmit | Finished | RefusedBefore | RefusedAfter | Crashed | Ambiguous
  type Config = {route:Route,fault:Fault,allowCrash:bool}
  type State = {cfg:Config,phase:Phase,financialPassed:bool,recordsWritten:bool,
    boundPreparedIdentity:bool,recoveryDurable:bool,ownedUncertainty:bool,
    signedBytesExposurePossible:bool,signedAuthorityLive:bool,thirdPartyRelayPossible:bool,
    nodeAccepted:bool,ownSubmitMayHaveStarted:bool,modeledExplicitAttempts:int,ownsExplicitSubmit:bool,
    localBookkeepingReverted:bool,registered:bool,finalized:bool}
  pure val corrected:Config={route:RequiredRecoveryFirst,fault:NoFault,allowCrash:false}
  pure def initial(c:Config):State={cfg:c,phase:FinancialGuard,financialPassed:false,
    recordsWritten:false,boundPreparedIdentity:false,recoveryDurable:false,ownedUncertainty:false,
    signedBytesExposurePossible:false,signedAuthorityLive:true,thirdPartyRelayPossible:false,nodeAccepted:false,
    ownSubmitMayHaveStarted:false,modeledExplicitAttempts:0,ownsExplicitSubmit:false,
    localBookkeepingReverted:false,registered:false,finalized:false}
  pure def terminal(s:State):bool=Set(Finished,RefusedBefore,RefusedAfter,Crashed,Ambiguous).contains(s.phase)
  pure def refused(s:State):State={...s,phase:(if(s.signedBytesExposurePossible) RefusedAfter else RefusedBefore)}
  pure def nextState(s:State):State=match s.phase {
    | FinancialGuard=>if(s.cfg.fault==FinancialReject) refused(s)
      else {...s,financialPassed:true,phase:(if(s.cfg.route==Source28ExposureFirst) Expose else WriteRecord)}
    | WriteRecord=>if(s.cfg.fault==RecordWriteFailure) refused(s)
      else {...s,recordsWritten:true,boundPreparedIdentity:true,phase:PersistRecovery}
    | PersistRecovery=>if(s.cfg.fault==FsyncFailure) refused(s)
      else {...s,recoveryDurable:true,phase:ArmOwnership}
    | ArmOwnership=>{...s,ownedUncertainty:true,
      ownSubmitMayHaveStarted:(s.cfg.route==Source28ExposureFirst or s.ownSubmitMayHaveStarted),
      phase:(if(s.cfg.route==Source28ExposureFirst) OwnSubmit else Expose)}
    | Expose=>{...s,signedBytesExposurePossible:true,thirdPartyRelayPossible:s.signedAuthorityLive,phase:AwaitVerdict}
    | AwaitVerdict=>if(Set(NodeInvalid,NodeUnknown,NodeTimeout).contains(s.cfg.fault)) refused(s)
      else {...s,nodeAccepted:true,ownSubmitMayHaveStarted:s.cfg.route==RequiredRecoveryFirst,
        phase:(if(s.cfg.route==Source28ExposureFirst) WriteRecord else OwnSubmit)}
    | OwnSubmit=>if(s.cfg.fault==PostValidationFailure) refused(s)
      else {...s,modeledExplicitAttempts:s.modeledExplicitAttempts+1,ownsExplicitSubmit:true,
        phase:(if(s.cfg.fault==OwnDispatchAmbiguous) Ambiguous else Finished)}
    | _=>List().head() }
  pure def exposureProtected(s:State):bool=s.signedBytesExposurePossible implies
    s.recoveryDurable and s.boundPreparedIdentity and s.ownedUncertainty
  // ownedUncertainty is logical external ownership, not a claim volatile armed survives a crash.
  // Durable records must support conservative operator recovery; no automatic resume action exists.
  var state:State
  action initWith(c:Config):bool=state'=initial(c)
  action initSource28:bool=initWith({...corrected,route:Source28ExposureFirst})
  action init:bool={
    nondet routeArg=Set(Source28ExposureFirst,RequiredRecoveryFirst).oneOf()
    nondet faultArg=Set(NoFault,FinancialReject,RecordWriteFailure,FsyncFailure,
      NodeInvalid,NodeUnknown,NodeTimeout,OwnDispatchAmbiguous,PostValidationFailure).oneOf()
    nondet crashArg=Set(true,false).oneOf()
    initWith({route:routeArg,fault:faultArg,allowCrash:crashArg}) }
  action advance:bool=all {not(terminal(state)),state'=nextState(state)}
  action crash:bool=all {not(terminal(state)),state.cfg.allowCrash,state'={...state,phase:Crashed}}
  action revertBookkeeping:bool=all {terminal(state),state.signedBytesExposurePossible,not(state.localBookkeepingReverted),
    state'={...state,localBookkeepingReverted:true}}
  action authorityExpires:bool=all {terminal(state),state.signedAuthorityLive,
    state'={...state,signedAuthorityLive:false,thirdPartyRelayPossible:false}}
  action step:bool=any {advance,crash,revertBookkeeping,authorityExpires}
  val exposureSafety:bool=exposureProtected(state)
  val correctedSafety:bool=and {
    state.cfg.route==RequiredRecoveryFirst implies exposureProtected(state),
    state.modeledExplicitAttempts<=1,state.ownsExplicitSubmit==(state.modeledExplicitAttempts==1),
    state.modeledExplicitAttempts>0 implies state.nodeAccepted and state.financialPassed
      and state.recoveryDurable and state.boundPreparedIdentity and state.ownedUncertainty,
    state.thirdPartyRelayPossible==(state.signedBytesExposurePossible and state.signedAuthorityLive),
    state.phase==RefusedAfter implies state.modeledExplicitAttempts==0,
    not(state.registered),not(state.finalized) }
  val oldUnrecoveredExposure:bool=state.cfg.route==Source28ExposureFirst and state.signedBytesExposurePossible
    and not(state.recoveryDurable) and state.thirdPartyRelayPossible and state.modeledExplicitAttempts==0
  val correctedProtectedExposure:bool=state.cfg.route==RequiredRecoveryFirst and state.signedBytesExposurePossible
    and exposureProtected(state) and state.modeledExplicitAttempts==0
  val correctedSingleOwnSubmit:bool=state.cfg.route==RequiredRecoveryFirst and state.modeledExplicitAttempts==1
  val refusedAfterProtectedExposure:bool=state.cfg.route==RequiredRecoveryFirst and state.phase==RefusedAfter
    and state.signedBytesExposurePossible and exposureProtected(state) and state.thirdPartyRelayPossible
  val crashAfterProtectedExposure:bool=state.cfg.route==RequiredRecoveryFirst and state.phase==Crashed
    and state.signedBytesExposurePossible and exposureProtected(state)
  val fsyncFailureBeforeExposure:bool=state.cfg.route==RequiredRecoveryFirst and state.cfg.fault==FsyncFailure
    and state.phase==RefusedBefore and not(state.recoveryDurable) and not(state.signedBytesExposurePossible)
}
