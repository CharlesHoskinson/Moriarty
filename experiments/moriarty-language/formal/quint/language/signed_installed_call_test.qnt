// Independent fixture literals from root expectations SHA256
// 3ac8f6d7be94949a7384bc77672e5ad434a6cbd4c5c43e0d2062998efecd8c4a.
// Constructors only assemble values; no expected/effects/post/totals helper
// computes the literal outcomes below. ExternalAccepted is a premise, not crypto evidence.
module signed_installed_call_test {
  import signed_installed_call.* from "./signed_installed_call"
  pure val literalA1: Projection = {inputs:List(UInt(7)),
    pre:{values:Map(1 -> UInt(4),2 -> BoolValue(true)),balances:Map(1 -> 100,2 -> 5,3 -> 1),duties:Map(7 -> slot(false,0))},
    effects:List(write(1,UInt(4),UInt(11)),payment(Transfer,7),payment(Transfer,7),payment(Fee,2),liability(Liability,0,7,7),liability(Discharge,7,4,3),emit(3,UInt(11)),emit(2,BoolValue(true))),
    post:{values:Map(1 -> UInt(11),2 -> BoolValue(true)),balances:Map(1 -> 84,2 -> 19,3 -> 3),duties:Map(7 -> slot(true,4))}}
  pure val literalA2: Projection = {inputs:List(UInt(3)),
    pre:{values:Map(1 -> UInt(31),2 -> BoolValue(true)),balances:Map(1 -> 100,2 -> 5,3 -> 1),duties:Map(7 -> slot(true,9))},
    effects:List(write(1,UInt(31),UInt(34)),payment(Transfer,3),payment(Transfer,3),payment(Fee,2),liability(Liability,9,12,3),liability(Discharge,12,9,3),emit(3,UInt(34)),emit(2,BoolValue(true))),
    post:{values:Map(1 -> UInt(34),2 -> BoolValue(true)),balances:Map(1 -> 92,2 -> 11,3 -> 3),duties:Map(7 -> slot(true,9))}}
  pure val literalFreeze: Projection = {inputs:List(),
    pre:{values:Map(1 -> UInt(4),2 -> BoolValue(true)),balances:Map(1 -> 100,2 -> 5,3 -> 1),duties:Map(7 -> slot(true,9))},
    effects:List(write(2,BoolValue(true),BoolValue(false)),emit(2,BoolValue(false))),
    post:{values:Map(1 -> UInt(4),2 -> BoolValue(false)),balances:Map(1 -> 100,2 -> 5,3 -> 1),duties:Map(7 -> slot(true,9))}}
  pure val literalB1: Projection = {inputs:List(UInt(1)),pre:{values:Map(1 -> UInt(31),2 -> BoolValue(true)),balances:Map(),duties:Map()},
    effects:List(write(1,UInt(31),UInt(32)),write(2,BoolValue(true),BoolValue(false)),emit(1,UInt(32)),emit(2,BoolValue(false))),
    post:{values:Map(1 -> UInt(32),2 -> BoolValue(false)),balances:Map(),duties:Map()}}
  pure val literalB2: Projection = {inputs:List(UInt(1)),pre:{values:Map(1 -> UInt(255),2 -> BoolValue(true)),balances:Map(),duties:Map()},
    effects:List(write(1,UInt(255),UInt(256)),write(2,BoolValue(true),BoolValue(false)),emit(1,UInt(256)),emit(2,BoolValue(false))),
    post:{values:Map(1 -> UInt(256),2 -> BoolValue(false)),balances:Map(),duties:Map()}}
  pure val literalReset: Projection = {inputs:List(),pre:{values:Map(1 -> UInt(256),2 -> BoolValue(false)),balances:Map(),duties:Map()},
    effects:List(write(1,UInt(256),UInt(0)),write(2,BoolValue(false),BoolValue(true)),emit(1,UInt(0))),
    post:{values:Map(1 -> UInt(0),2 -> BoolValue(true)),balances:Map(),duties:Map()}}
  action compared(c: Case): bool = init.then(compare(c,keyMap(c),ExternalAccepted))
  action signingReady(c: Case,q: Request): bool = compared(c).then(prepare(package(c,q),q,observations(pre(c)),observations(post(c)),ExternalAccepted))
  action attached(c: Case,choice: int): bool = signingReady(c,request(c)).then(attach(attachment(state.package,choice),input(state.package,signature(choice),c),acceptedPredicates))
  action reported(c: Case,choice: int): bool = attached(c,choice).then(reportNative(proof(state.package,state.input),ExternalAccepted))
  action finish: bool = coldVerify(state.package,{payload:state.package.payload,signature:state.input.signature},state.keys,state.proof,0,ExternalAccepted)
  run initSafetyTest = init.expect(safety and state.phase == Idle and not(comparedWitness))
  run A1LiteralTest = reported(A1,0).expect(state.package.payload.projection == literalA1)
    .expect(totals(state.context,literalA1,11) == {outgoing:14,fees:2,credits:14,debits:0,gross:16,net:12,duty:4})
    .then(finish).expect(safety and coldVerifiedWitness and state.pkReads == 0 and state.expansions == 1)
  run A2LiteralTest = reported(A2,0).expect(state.package.payload.projection == literalA2)
    .expect(totals(state.context,literalA2,11) == {outgoing:6,fees:2,credits:6,debits:0,gross:8,net:4,duty:9})
    .then(finish).expect(safety and coldVerifiedWitness)
  run freezeLiteralTest = reported(Freeze,0).expect(state.package.payload.projection == literalFreeze)
    .expect(totals(state.context,literalFreeze,11) == {outgoing:0,fees:0,credits:0,debits:0,gross:0,net:0,duty:9})
    .then(finish).expect(safety and freezeWitness and untouchedDutyPreservation)
  run B1LiteralTest = reported(B1,0).expect(state.package.payload.projection == literalB1 and state.request.limits == List()).then(finish).expect(safety and stateOnlyWitness)
  run B2LiteralTest = reported(B2,0).expect(state.package.payload.projection == literalB2 and state.request.limits == List()).then(finish).expect(safety and stateOnlyWitness)
  run resetLiteralTest = reported(Reset,0).expect(state.package.payload.projection == literalReset and state.request.limits == List()).then(finish).expect(safety and stateOnlyWitness)
  run sharedSourceKeysTest = reported(A1,0).expect(state.keys == Map(1 -> {ir:101,vk:201,policy:policy,signed:true},2 -> {ir:102,vk:202,policy:policy,signed:true}))
    .then(reported(A2,0)).expect(state.keys == Map(1 -> {ir:101,vk:201,policy:policy,signed:true},2 -> {ir:102,vk:202,policy:policy,signed:true}))
  run unusedKeyRefusalTest = init.then(compare(A1,keyMap(A1).put(2,{ir:102,vk:999,policy:policy,signed:true}),ExternalAccepted).fail())
  run unsignedKeyRefusalTest = init.then(compare(A1,keyMap(A1).put(2,{ir:102,vk:202,policy:policy,signed:false}),ExternalAccepted).fail())
  run importedOnlyRefusalTest = init.then(compare(A1,keyMap(A1),ExternalRejected).fail())
  run packageBeforeDerivationRefusalTest = init.then(prepare(state.package,state.request,observations(pre(A1)),observations(post(A1)),ExternalAccepted).fail())
  run packageHasNoOwnerTest = signingReady(B1,request(B1)).expect(state.expansions == 0 and not(attachedWitness) and not(nativeResultWitness))
    .then(reportNative(state.proof,ExternalAccepted).fail())
  pure def changedPackage(c: Case,n: int): Package = {
    val p = package(c,request(c))
    if(n == 0) {...p,payload:{...p.payload,context:{...p.payload.context,source:Toggle}}}
    else if(n == 1) {...p,payload:{...p.payload,context:{...p.payload.context,network:999}}}
    else if(n == 2) {...p,payload:{...p.payload,context:{...p.payload.context,anchor:999}}}
    else if(n == 3) {...p,payload:{...p.payload,context:{...p.payload.context,address:999}}}
    else if(n == 4) {...p,payload:{...p.payload,context:{...p.payload.context,policy:{...policy,pk:999}}}}
    else if(n == 5) {...p,payload:{...p.payload,projection:{...literalA1,pre:{...literalA1.pre,values:Map(1 -> UInt(5),2 -> BoolValue(true))}}}}
    else if(n == 6) {...p,payload:{...p.payload,wires:p.payload.wires.replaceAt(0,UInt(8))}}
    else {...p,payload:{...p.payload,request:{...request(c),predecessor:999}}}
  }
  run sourceDomainWireBindingTest = compared(A1).expect(0.to(7).forall(i => not(canPrepare(state,changedPackage(A1,i),state.request,observations(pre(A1)),observations(post(A1)),ExternalAccepted))))
  run networkFailureActionTest = compared(A1).then(prepare(changedPackage(A1,1),state.request,observations(pre(A1)),observations(post(A1)),ExternalAccepted).fail())
  run unknownZeroDutyTest = compared(Freeze).expect(not(complete(state.context,{...pre(Freeze),duties:pre(Freeze).duties.put(8,slot(true,0))})))
    .then(prepare(state.package,state.request,{...observations(pre(Freeze)),dutyCount:2},observations(post(Freeze)),ExternalAccepted).fail())
  run omittedUntouchedDutyTest = compared(Freeze).then(prepare(state.package,state.request,{...observations(pre(Freeze)),memberships:Map(),reads:Map()},observations(post(Freeze)),ExternalAccepted).fail())
  run wrongDutyMetadataTest = compared(Freeze).then(prepare(state.package,state.request,{...observations(pre(Freeze)),reads:Map(7 -> {...slot(true,9),asset:12})},observations(post(Freeze)),ExternalAccepted).fail())
  run absentVersusPresentZeroTest = compared(A1).expect(complete(state.context,{...pre(A1),duties:Map(7 -> slot(true,0))}))
    .expect(endpointWires(state.context,pre(A1)) != endpointWires(state.context,{...pre(A1),duties:Map(7 -> slot(true,0))}))
    .then(prepare(state.package,state.request,{...observations(pre(A1)),memberships:Map(7 -> true),reads:Map(7 -> slot(true,0)),dutyCount:1},observations(post(A1)),ExternalAccepted).fail())
  run canonicalAbsentAmountTest = init.expect(not(complete(context(A1),{...pre(A1),duties:Map(7 -> {...slot(false,0),amount:1})})))
  run wrongStateCountTest = compared(A1).then(prepare(state.package,state.request,{...observations(pre(A1)),stateCount:3},observations(post(A1)),ExternalAccepted).fail())
  pure def tighter(c: Case,n: int): Request = {
    val q = request(c)
    val cap = q.limits.nth(0).caps
    {...q,limits:List({asset:11,caps:if(n == 0) {...cap,gross:cap.gross - 1} else if(n == 1) {...cap,fee:cap.fee - 1} else if(n == 2) {...cap,net:cap.net + 1} else {...cap,duty:cap.duty - 1}})}
  }
  // Independently accepted newly matching FENS is insufficient: actual cap guard refuses.
  run A1ResignedTighterCapsTest = {
    nondet n = 0.to(3).oneOf()
    signingReady(A1,tighter(A1,n)).then(attach(attachment(state.package,0),input(state.package,signature(0),A1),acceptedPredicates).fail())
  }
  run A2ResignedTighterCapsTest = {
    nondet n = 0.to(3).oneOf()
    signingReady(A2,tighter(A2,n)).then(attach(attachment(state.package,0),input(state.package,signature(0),A2),acceptedPredicates).fail())
  }
  run freezeResignedDuty8Test = signingReady(Freeze,tighter(Freeze,3)).expect(state.request.limits.nth(0).caps.duty == 8)
    .then(attach(attachment(state.package,0),input(state.package,signature(0),Freeze),acceptedPredicates).fail())
  run completeOrderedLimitsTest = init.expect(not(completeLimits(context(A1),List())))
    .expect(not(completeLimits(context(A1),List({asset:11,caps:tight(A1).nth(0).caps},{asset:11,caps:tight(A1).nth(0).caps}))))
    .expect(not(completeLimits({...context(A1),assets:List(11,12)},List({asset:12,caps:tight(A1).nth(0).caps},{asset:11,caps:tight(A1).nth(0).caps}))))
  run sourceWideAdmissionTest = init.expect(staticAdmission(context(A1),Set(1)))
    .expect(not(staticAdmission(context(A1),Set(1,9))))
    .expect(not(staticAdmission({...context(A1),registry:Map(7 -> {asset:11,debtor:9,creditor:2})},Set(1))))
    .expect(not(staticAdmission({...context(A1),assets:List(11,11)},Set(1))))
    .expect(not(staticAdmission({...context(A1),assets:range(0,17)},Set(1))))
    .expect(not(staticAdmission({...context(A1),registry:0.to(16).mapBy(_ => {asset:11,debtor:1,creditor:2}),dutyOrder:range(0,17)},Set(1))))
  run strictPredicateRefusalTest = signingReady(A1,request(A1)).expect(not(canAttach(state,attachment(state.package,0),input(state.package,signature(0),A1),{...acceptedPredicates,strictHostFens:ExternalRejected})))
    .expect(not(canAttach(state,attachment(state.package,0),input(state.package,signature(0),A1),{...acceptedPredicates,typedSubgroup:ExternalRejected})))
    .expect(not(canAttach(state,attachment(state.package,0),input(state.package,signature(0),A1),{...acceptedPredicates,scalarCanonical:ExternalRejected})))
    .expect(not(canAttach(state,attachment(state.package,0),input(state.package,signature(0),A1),{...acceptedPredicates,assertedFens:ExternalRejected})))
    .expect(not(canAttach(state,attachment(state.package,0),input(state.package,signature(0),A1),{...acceptedPredicates,sourceNativeWires:ExternalRejected})))
  run attackerMatchingValidSignatureTest = signingReady(A1,request(A1)).then(attach({payload:state.package.payload,signature:{...signature(0),pk:999}},input(state.package,{...signature(0),pk:999},A1),acceptedPredicates).fail())
  run scalarZeroIsNotPositivityTest = signingReady(B1,request(B1)).then(attach({payload:state.package.payload,signature:{...signature(0),response:0}},input(state.package,{...signature(0),response:0},B1),acceptedPredicates)).expect(safety)
  run emptyScopeStillRequiresFensTest = signingReady(B1,request(B1)).then(attach(attachment(state.package,0),input(state.package,signature(0),B1),{...acceptedPredicates,strictHostFens:ExternalRejected}).fail())
  run intervalEndpointsTest = init.expect(admittedRequest(A1,{...request(A1),at:10}) and admittedRequest(A1,{...request(A1),at:20}))
    .expect(not(admittedRequest(A1,{...request(A1),at:9})) and not(admittedRequest(A1,{...request(A1),at:21})))
    .expect(not(admittedRequest(A1,{...request(A1),start:20,end:10})) and not(admittedRequest(A1,{...request(A1),at:uintMax + 1})))
  run staleNoncePayloadTest = signingReady(A1,{...request(A1),nonce:2}).then(attach(attachment(package(A1,request(A1)),0),input(state.package,signature(0),A1),acceptedPredicates).fail())
  run localNonceDoesNotConsumeTest = reported(B1,0).then(finish).expect(coldVerifiedWitness).then(reported(B1,0)).then(finish).expect(coldVerifiedWitness and state.request.nonce == 1)
  pure def badLayouts(c: Case): List[Layout] = List({...layout(c),typed:layout(c).typed - 1},{...layout(c),flattened:layout(c).flattened + 1},
    {...layout(c),privateFields:1},{...layout(c),authImpacts:1},{...layout(c),outputs:2},{...layout(c),communications:false},{...layout(c),sharedAllocator:false})
  run typedTransportRefusalTest = signingReady(A1,request(A1)).expect(badLayouts(A1).indices().forall(i => not(canAttach(state,attachment(state.package,0),{...input(state.package,signature(0),A1),layout:badLayouts(A1).nth(i)},acceptedPredicates))))
  run typedCountsTest = init.expect(layout(A1).typed == 16 and layout(A1).flattened == 18 and layout(Freeze).typed == 15 and layout(Freeze).flattened == 17 and layout(B1).typed == 12 and layout(B1).flattened == 14 and layout(Reset).typed == 11 and layout(Reset).flattened == 13)
  run changedAttachmentSameOldInputTest = signingReady(A1,request(A1)).then(attach(attachment(state.package,1),input(state.package,signature(0),A1),acceptedPredicates).fail())
  run alternateValidAttachmentNewProofTest = reported(A1,1).expect(state.input.signature == {pk:31,r:72,response:82})
    .expect(state.package == package(A1,request(A1)))
    .expect(state.proof.final.pi != proof(state.package,input(state.package,signature(0),A1)).final.pi)
    .then(finish).expect(alternateAttachmentWitness and safety)
  run oldProofNewValidAttachmentTest = reported(A1,0).then(coldVerify(state.package,attachment(state.package,1),state.keys,state.proof,0,ExternalAccepted).fail())
  run nativeExternalFailureTest = attached(A1,0).then(reportNative(proof(state.package,state.input),ExternalRejected).fail())
  run processedMarkerRefusalTest = attached(A1,0).then(reportNative({...proof(state.package,state.input),genuineV3:false},ExternalAccepted).fail())
  run doubleNativeExpansionTest = reported(A1,0).then(reportNative(state.proof,ExternalAccepted).fail())
  run coldVerifierPkReadRefusalTest = reported(A1,0).then(coldVerify(state.package,attachment(state.package,0),state.keys,state.proof,1,ExternalAccepted).fail())
  run coldVerifierNativeFailureTest = reported(A1,0).then(coldVerify(state.package,attachment(state.package,0),state.keys,state.proof,0,ExternalRejected).fail())
  run wholeFinalReadbackMutationTest = reported(A1,0).expect(not(canColdVerify(state,state.package,attachment(state.package,0),state.keys,{...state.proof,final:{...state.proof.final,parent:999}},0,ExternalAccepted)))
    .expect(not(canColdVerify(state,state.package,attachment(state.package,0),state.keys,{...state.proof,final:{...state.proof.final,gas:999}},0,ExternalAccepted)))
    .expect(not(canColdVerify(state,state.package,attachment(state.package,0),state.keys,{...state.proof,final:{...state.proof.final,pi:{...state.proof.final.pi,processed:List(92,93)}}},0,ExternalAccepted)))
    .expect(not(canColdVerify(state,state.package,attachment(state.package,0),state.keys,{...state.proof,final:{...state.proof.final,amounts:List(14,2,85)}},0,ExternalAccepted)))
    .expect(not(canColdVerify(state,state.package,attachment(state.package,0),state.keys,{...state.proof,final:{...state.proof.final,actions:List(1,2)}},0,ExternalAccepted)))
  run refundGrossFeeNetTest = init.expect({
    val p = {...literalA1,effects:List(payment(Transfer,7),{...payment(Transfer,3),payer:2,recipient:1},payment(Fee,2))}
    totals(context(A1),p,11) == {outgoing:7,fees:2,credits:7,debits:3,gross:9,net:2,duty:4}
  })
  run selfBeneficiaryFeeUnderflowTest = init.expect({
    val c = {...context(A1),policy:{...policy,beneficiary:1}}
    val p = {...literalA1,effects:List({...payment(Transfer,10),recipient:1},payment(Fee,2))}
    val t = totals(c,p,11)
    t.gross == 12 and t.net == -2 and t.fees == 2 and t.debits == 10 and not(checkedTotals(t))
  })
  run checkedOverflowTest = init.expect({
    val p = {...literalA1,effects:List(payment(Transfer,uintMax),payment(Transfer,1))}
    not(checkedTotals(totals(context(A1),p,11))) and not(economic(context(A1),p,List({asset:11,caps:{gross:uintMax,fee:uintMax,net:0,duty:uintMax}})))
  })
  run closingDutyOverflowTest = init.expect({
    val c = {...context(A1),registry:Map(7 -> {asset:11,debtor:1,creditor:2},8 -> {asset:11,debtor:1,creditor:2}),dutyOrder:List(7,8)}
    val p = {...literalA1,post:{...literalA1.post,duties:Map(7 -> slot(true,uintMax),8 -> slot(true,1))}}
    not(checkedTotals(totals(c,p,11)))
  })
  run extraUntouchedDutyCountsAgainstCapTest = init.expect({
    val c = {...context(A1),registry:Map(7 -> {asset:11,debtor:1,creditor:2},8 -> {asset:11,debtor:1,creditor:2}),dutyOrder:List(7,8)}
    val p = {...literalA1,pre:{...literalA1.pre,duties:literalA1.pre.duties.put(8,slot(true,9))},post:{...literalA1.post,duties:literalA1.post.duties.put(8,slot(true,9))}}
    observed(c,p.pre,observations(p.pre)) and observed(c,p.post,observations(p.post))
      and totals(c,p,11).duty == 13 and not(economic(c,p,tight(A1)))
      and economic(c,p,List({asset:11,caps:{gross:16,fee:2,net:12,duty:13}}))
  })
  run faultBoundaryRecordingTest = init.then(refuse(0)).expect(refusedWitness and state.faults == Set(0) and safety)
    .then(compared(A1)).then(refuse(1)).expect(refusedWitness and safety)
    .then(signingReady(A1,request(A1))).then(refuse(2)).expect(refusedWitness and safety)
    .then(attached(A1,0)).then(refuse(3)).expect(refusedWitness and safety)
    .then(reported(A1,0)).then(refuse(4)).expect(refusedWitness and safety)
  // Negative fault demonstration: deliberately disconnected cap guard admits
  // freeze9 under cap8, so sharedSemanticWires/safety must report a violation.
  action ablateCap = all {state.phase == SigningPrepared,
    state' = {...advance(state,Attached),input:input(state.package,signature(0),state.instance)}}
  action faultyCapInit = signingReady(Freeze,tighter(Freeze,3))
  action faultyCapStep = ablateCap
  val capFaultDetectedWitness = state.phase == Attached and not(sharedSemanticWires)
  // A second diagnostic substitutes attachment while retaining the old proof.
  action faultyAttachmentInit = reported(A1,0)
  action ablateAttachmentBinding = all {state.phase == NativeResultReported,
    state' = {...advance(state,ColdVerified),input:input(state.package,signature(1),state.instance)}}
  action faultyAttachmentStep = ablateAttachmentBinding
  val attachmentFaultDetectedWitness = coldVerifiedWitness and not(attachmentSpecificFinal)
}
