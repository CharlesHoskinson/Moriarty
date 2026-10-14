module liability_reads_test {
  import liability_reads.* from "./liability_reads"
  action ready(c: Case): bool = init.then(begin(c))
  action complete(c: Case): bool = ready(c).then(c.program.length().reps(_ => execute)).then(commit)
  run boundedCasesTest = init.expect(CASES.forall(c => wellFormed(c)) and safety)
  run initializationTest = init.expect(state.phase == Idle and not(beginWitness) and state.committedEffects == 0)
  run absenceTest = complete(missing).expect(state.history.nth(0).reads == List({duty:0,presence:true,value:boolean(false)})
    and state.committed.x == uint(0) and not(state.committed.duties.get(0).present) and safety)
  run tombstoneTest = complete(tombstone).expect(state.history.nth(0).reads == List({duty:0,presence:true,value:boolean(true)})
    and state.committed.x == uint(0) and state.committed.duties.get(0).present and safety)
  run orderedCaptureTest = ready(ordered).then(execute).then(execute)
    .expect(state.env.get("saved") == uint(7))
    .then(execute).then(execute).expect(state.current.x == uint(4) and state.env.get("saved") == uint(7))
    .then(4.reps(_ => execute)).then(commit)
    .expect(state.committed.duties.get(0) == row(true,0) and state.committed.x == uint(4)
      and state.env.get("saved") == uint(7) and state.committedEffects == 7 and safety)
  run selfReadBeforeEffectTest = ready(selfRead).then(execute)
    .expect(state.current.duties.get(0) == row(true,14)
      and state.history.nth(0).reads == List({duty:0,presence:false,value:uint(7)}))
    .then(execute).then(commit).expect(state.committed.duties.get(0) == row(true,0)
      and state.history.nth(1).reads == List({duty:0,presence:false,value:uint(14)}) and safety)
  run sharedHandleFreshStatementTest = complete(ordered).expect(
    state.history.nth(1).reads == List({duty:0,presence:false,value:uint(7)})
    and state.history.nth(3).reads == List({duty:0,presence:false,value:uint(4)})
    and state.history.nth(7).reads == List({duty:0,presence:false,value:uint(0)}))
  run duplicateHandleOnceTest = complete(duplicate).expect(state.committed.x == uint(14)
    and state.history.nth(0).reads == List({duty:0,presence:false,value:uint(7)}) and safety)
  run twoDutyLeftFirstTest = complete(twoDuties).expect(state.committed.x == uint(10)
    and state.history.nth(0).reads == List({duty:0,presence:false,value:uint(7)},{duty:1,presence:false,value:uint(3)}) and safety)
  run unusedOverflowRollsBackEarlierEffectTest = ready(overflow).then(execute)
    .expect(state.current.duties.get(0) == row(true,7) and state.committed == EMPTY)
    .then(refuse).expect(state.phase == Refused and state.error == "expression"
      and state.committed == EMPTY and state.committedEffects == 0 and state.effects == 1 and safety)
  run eagerAndCannotHideOverflowTest = ready(eager).then(refuse)
    .expect(state.error == "expression" and state.committed == EMPTY and safety)
  run eagerOrCannotHideOverflowTest = ready(eagerOr).then(refuse)
    .expect(state.error == "expression" and state.committed == EMPTY and safety)
  run absentZeroDischargeRefusesTest = ready(absentDischarge).then(refuse)
    .expect(state.error == "absent-discharge" and state.committed == EMPTY and safety)
  run excessiveDischargeRefusesTest = ready(excessiveDischarge).then(refuse)
    .expect(state.error == "underflow" and state.committed == snapshot(true,3) and safety)
  run arithmeticUnderflowRefusesTest = ready(expressionUnderflow).then(refuse)
    .expect(state.error == "expression" and state.committed == EMPTY and safety)
  run multiplyOverflowRefusesTest = ready(multiplyOverflow).then(refuse)
    .expect(state.error == "expression" and state.committed == EMPTY and safety)
  run mutationOverflowRefusesTest = ready(mutationOverflow).then(refuse)
    .expect(state.error == "overflow" and state.committed == snapshot(true,U128_MAX) and safety)
  run wideExactAmountsTest = {
    nondet n = Set(W64,W127,U128_MAX).oneOf()
    complete({...wide,pre:snapshot(true,n)}).expect(state.committed.x == uint(n)
      and state.committed.y == uint(0) and state.committed.duties.get(0) == row(true,0) and safety)
  }
  run presenceGuardsTest = complete(guards).expect(state.executed == Set(Requires,Ensures) and state.committedEffects == 0 and safety)
  run falseGuardRefusesTest = ready({...guards,pre:EMPTY}).then(refuse)
    .expect(state.error == "predicate" and state.pc == 0 and state.committed == EMPTY and safety)
  run falseEnsuresRollsBackTest = ready({label:"false-ensures",pre:EMPTY,arena:List(lit(7),blit(false)),
    program:List(stmt(Accrue,0,0,""),stmt(Ensures,1,0,""))}).then(execute).then(refuse)
    .expect(state.phase == Refused and state.committed == EMPTY and state.committedEffects == 0 and safety)
  run identitySubstitutionRefusesBeforeFirstStatementTest = init.then(refuseAdmission(forgedIdentity))
    .expect(state.phase == Refused and state.error == "identity-admission" and state.pc == 0
      and state.history == List() and state.committed == forgedIdentity.pre and state.committedEffects == 0 and safety)
  run overflowCommitDisabledTest = ready(overflow).then(execute).then(execute.fail())
  run absentDischargeDisabledTest = ready(absentDischarge).then(execute.fail())
  run eagerAndDisabledTest = ready(eager).then(execute.fail())
  run forgedIdentityAdmissionDisabledTest = init.then(begin(forgedIdentity).fail())
  run noPrematureCommitTest = ready(ordered).then(commit.fail())
  run committedTerminalTest = complete(missing).then(step.fail())
  run refusedTerminalTest = ready(eager).then(refuse).then(step.fail())
  run capturedValueConsumerTest = complete(ordered).expect(state.env.get("saved") == uint(7)
    and state.history.nth(4).reads == List() and state.committedEvents == List(uint(7),boolean(true),uint(0))
    and state.executed.contains(Emit) and safety)
  run unaryNotLiteralTest = complete(unaryNot).expect(state.committedEvents == List(boolean(false),boolean(true))
    and state.history.nth(0).reads == List({duty:0,presence:true,value:boolean(true)}) and safety)
  run unaryNotSampledPresenceTest = {
    nondet p = Set(false,true).oneOf()
    complete({...unaryNot,pre:snapshot(p,0)}).expect(state.committedEvents == List(boolean(not(p)),boolean(p)) and safety)
  }
  run successfulEqualityLiteralTest = complete(equality).expect(
    state.committedEvents == List(boolean(true),boolean(false),boolean(true),boolean(false)) and safety)
  run successfulEqualitySampledUIntAndBoolTest = {
    nondet n = Set(0,1,W64,W127,U128_MAX).oneOf()
    complete({...equality,arena:List(lit(n),lit(if (n == U128_MAX) 0 else n+1),binary(Eq,0,0),binary(Eq,0,1),
      blit(true),blit(false),binary(Eq,4,4),binary(Eq,4,5))}).expect(
      state.committedEvents == List(boolean(true),boolean(false),boolean(true),boolean(false)) and safety)
  }
  run emittedEventRollsBackAfterLaterFailureTest = ready(eventFailure).then(execute)
    .expect(state.events == List(uint(7)) and state.committedEvents == List() and state.effects == 1)
    .then(refuse).expect(state.phase == Refused and state.error == "expression"
      and state.events == List(uint(7)) and state.committedEvents == List()
      and state.committed == EMPTY and state.committedEffects == 0 and refusalIsAtomic and safety)
  // Test-only faults challenge the invariants; excluded from the production step.
  action staleReadFault = all {state.phase == Running,state.pc == 3,
    state' = {...state,history:state.history.append({pc:3,snapshot:state.current,
      reads:List({duty:0,presence:false,value:uint(7)})})}}
  run staleReadDetectedTest = ready(ordered).then(3.reps(_ => execute)).then(staleReadFault).expect(not(freshReads))
  action recaptureFault = state' = {...state,env:state.env.put("saved",uint(4))}
  run recaptureDetectedTest = ready(ordered).then(3.reps(_ => execute)).then(recaptureFault).expect(not(capturedLets))
  action identityFault = state' = {...state,current:{...state.current,
    duties:state.current.duties.set(0,{present:true,amount:7,identity:99})}}
  run changedIdentityDetectedTest = ready(ordered).then(execute).then(identityFault).expect(not(fixedIdentity))
  action committedEventLeakFault = state' = {...state,committedEvents:state.events}
  run leakedRefusalEventsDetectedTest = ready(eventFailure).then(execute).then(refuse).then(committedEventLeakFault)
    .expect(state.committed == EMPTY and state.committedEffects == 0 and state.committedEvents == List(uint(7))
      and not(refusalIsAtomic))
  action leakCommitFault = state' = {...state,committed:state.current,committedEffects:state.effects}
  run leakedRefusalEffectsDetectedTest = ready(overflow).then(execute).then(refuse).then(leakCommitFault).expect(not(refusalIsAtomic))
}
