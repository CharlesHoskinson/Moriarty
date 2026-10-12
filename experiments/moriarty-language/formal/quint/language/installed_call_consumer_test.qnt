// Independent literal expectations, not constructed by model expected/post/effects.
// Opaque SDK/key/PI/gas/hash symbols below are EXTERNAL premises, not crypto positives.
module installed_call_consumer_test {
  import installed_call_consumer.* from "./installed_call_consumer"
  pure val literalAsset: Asset = {network:"local",issuer:"next-native-qa",reference:"QA",scale:0}
  pure val literalFrame: Frame = {data:1,maintenance:2,contractBalance:3,utxos:List(4,5),
    extensions:List(6,7),bookkeeping:List(8,9),ledgerParameters:10,network:11,nightSupply:12}
  pure val literalA1: Statement = {
    post:{values:Map("running" -> UIntValue(11),"enabled" -> BoolValue(true)),
      balances:List({owner:"alice",asset:literalAsset,amount:84},{owner:"bob",asset:literalAsset,amount:19},
        {owner:"treasury",asset:literalAsset,amount:3}),
      duties:List({name:"charge",asset:literalAsset,debtor:"alice",creditor:"bob",present:true,amount:4})},
    effects:List(StateEffect({name:"running",before:UIntValue(4),after:UIntValue(11)}),
      TransferEffect({asset:literalAsset,from:"alice",dst:"bob",amount:7}),
      TransferEffect({asset:literalAsset,from:"alice",dst:"bob",amount:7}),
      FeeEffect({asset:literalAsset,from:"alice",dst:"treasury",amount:2}),
      DutyEffect({name:"charge",asset:literalAsset,debtor:"alice",creditor:"bob",before:0,after:7}),
      DutyEffect({name:"charge",asset:literalAsset,debtor:"alice",creditor:"bob",before:7,after:4}),
      EmitEffect({name:"total",value:UIntValue(11)}),EmitEffect({name:"enabled",value:BoolValue(true)})),
    events:List(("total",UIntValue(11)),("enabled",BoolValue(true))),parent:602,binding:603,
    communication:604,pi:List(603,604,605),transcript:List(606,0,607),chargedState:608,
    measuredGas:609,declaredGas:610,
    envelope:{offers:List(611,612),ttl:613,randomness:614,actions:List(Settle),extraActions:List()},
    finalIntent:601,outputs:List({owner:"bob",asset:literalAsset,amount:14,finalIntent:601,ordinal:0},
      {owner:"treasury",asset:literalAsset,amount:2,finalIntent:601,ordinal:1},
      {owner:"alice",asset:literalAsset,amount:84,finalIntent:601,ordinal:2}),version:3}
  pure val literalA2: Statement = {...literalA1,
    post:{values:Map("running" -> UIntValue(34),"enabled" -> BoolValue(true)),
      balances:List({owner:"alice",asset:literalAsset,amount:92},{owner:"bob",asset:literalAsset,amount:11},
        {owner:"treasury",asset:literalAsset,amount:3}),
      duties:List({name:"charge",asset:literalAsset,debtor:"alice",creditor:"bob",present:true,amount:9})},
    effects:List(StateEffect({name:"running",before:UIntValue(31),after:UIntValue(34)}),
      TransferEffect({asset:literalAsset,from:"alice",dst:"bob",amount:3}),
      TransferEffect({asset:literalAsset,from:"alice",dst:"bob",amount:3}),
      FeeEffect({asset:literalAsset,from:"alice",dst:"treasury",amount:2}),
      DutyEffect({name:"charge",asset:literalAsset,debtor:"alice",creditor:"bob",before:9,after:12}),
      DutyEffect({name:"charge",asset:literalAsset,debtor:"alice",creditor:"bob",before:12,after:9}),
      EmitEffect({name:"total",value:UIntValue(34)}),EmitEffect({name:"enabled",value:BoolValue(true)})),
    events:List(("total",UIntValue(34)),("enabled",BoolValue(true))),
    outputs:List({owner:"bob",asset:literalAsset,amount:6,finalIntent:601,ordinal:0},
      {owner:"treasury",asset:literalAsset,amount:2,finalIntent:601,ordinal:1},
      {owner:"alice",asset:literalAsset,amount:92,finalIntent:601,ordinal:2})}
  pure val literalB1: Statement = {...literalA1,
    post:{values:Map("count" -> UIntValue(32),"active" -> BoolValue(false)),balances:List(),duties:List()},
    effects:List(StateEffect({name:"count",before:UIntValue(31),after:UIntValue(32)}),
      StateEffect({name:"active",before:BoolValue(true),after:BoolValue(false)}),
      EmitEffect({name:"count",value:UIntValue(32)}),EmitEffect({name:"active",value:BoolValue(false)})),
    events:List(("count",UIntValue(32)),("active",BoolValue(false))),
    envelope:{offers:List(),ttl:613,randomness:614,actions:List(Step),extraActions:List()},outputs:List()}
  pure val literalB2: Statement = {...literalB1,
    post:{values:Map("count" -> UIntValue(256),"active" -> BoolValue(false)),balances:List(),duties:List()},
    effects:List(StateEffect({name:"count",before:UIntValue(255),after:UIntValue(256)}),
      StateEffect({name:"active",before:BoolValue(true),after:BoolValue(false)}),
      EmitEffect({name:"count",value:UIntValue(256)}),EmitEffect({name:"active",value:BoolValue(false)})),
    events:List(("count",UIntValue(256)),("active",BoolValue(false)))}
  pure val literalFreeze: Statement = {...literalA1,
    post:{values:Map("running" -> UIntValue(4),"enabled" -> BoolValue(false)),
      balances:List({owner:"alice",asset:literalAsset,amount:100},{owner:"bob",asset:literalAsset,amount:5},
        {owner:"treasury",asset:literalAsset,amount:1}),
      duties:List({name:"charge",asset:literalAsset,debtor:"alice",creditor:"bob",present:true,amount:9})},
    effects:List(StateEffect({name:"enabled",before:BoolValue(true),after:BoolValue(false)}),
      EmitEffect({name:"enabled",value:BoolValue(false)})),events:List(("enabled",BoolValue(false))),
    envelope:{offers:List(),ttl:613,randomness:614,actions:List(Freeze),extraActions:List()},outputs:List()}
  pure val literalReset: Statement = {...literalB1,
    post:{values:Map("count" -> UIntValue(0),"active" -> BoolValue(true)),balances:List(),duties:List()},
    effects:List(StateEffect({name:"count",before:UIntValue(256),after:UIntValue(0)}),
      StateEffect({name:"active",before:BoolValue(false),after:BoolValue(true)}),EmitEffect({name:"count",value:UIntValue(0)})),
    events:List(("count",UIntValue(0))),
    envelope:{offers:List(),ttl:613,randomness:614,actions:List(Reset),extraActions:List()},outputs:List()}
  pure val literalMeteredMap: Entry -> Operation = Map(
    Settle -> {ir:101,vk:201,legacyVk:0},Freeze -> {ir:102,vk:202,legacyVk:0})
  pure val literalToggleMap: Entry -> Operation = Map(
    Step -> {ir:103,vk:203,legacyVk:0},Reset -> {ir:104,vk:204,legacyVk:0})
  // Guard assertions preserve the trace; separate terminal fail() controls
  // below exercise disabled actions. fail() does not supply a next state.
  action beforeDerivation(p: Purpose,c: Case): bool = init
    .then(preflight(p,c,exported(c),if(p == Keygen) keyless(sourceOf(c)) else sourceOperations(sourceOf(c)),limits))
    .then(discover(sourceEntries(sourceOf(c)).mapBy(e => key(e).degree),ExternalSdkDiscovery))
  action beforeComparison(p: Purpose,c: Case): bool = beforeDerivation(p,c)
    .then(regenerate(requiredPurpose(p),ExternalSourceDerivation))
  action beforeInstallation(p: Purpose,c: Case): bool = beforeComparison(p,c).then(compare(exported(c),p,0))
  action admitted(p: Purpose,c: Case): bool = beforeInstallation(p,c).then(install(sourceOperations(sourceOf(c)),literalFrame))
  action beforeFinal(p: Purpose,c: Case): bool = admitted(p,c).then(prepare(1))
  action finish(st: Statement): bool = if(state.purpose == ColdVerify)
    finalCompare(st,state.selectedOperation,1,0).then(checkExternalResult(ProofMarker,Real,Verifying,ExternalRealSuccess))
    else finalCompare(st,state.selectedOperation,1,0)
      .then(checkExternalResult(ProofMarker,Real,Verifying,ExternalRealSuccess))
      .then(writeLeaves(Set(),Set(1,2,3))).then(publish(Set()))
  run literalA1Test = beforeFinal(ColdProve,A1).expect(state.statement == literalA1 and state.frame == literalFrame)
    .then(finish(literalA1)).expect(acceptedWitness and safety and state.checkedStatement.effects.length() == 8)
  run literalA2Test = beforeFinal(ColdProve,A2).expect(state.statement == literalA2)
    .then(finish(literalA2)).expect(acceptedWitness and safety)
  run literalB1Test = beforeFinal(ColdProve,B1).expect(state.statement == literalB1)
    .then(finish(literalB1)).expect(acceptedWitness and safety and state.checkedStatement.effects.length() == 4)
  run literalB2Test = beforeFinal(ColdProve,B2).expect(state.statement == literalB2)
    .then(finish(literalB2)).expect(acceptedWitness and safety)
  run unusedFreezeLiteralTest = beforeFinal(ColdVerify,FreezeCase).expect(state.statement == literalFreeze)
    .expect(state.installedOps == literalMeteredMap and state.selectedOperation == {ir:102,vk:202,legacyVk:0})
    .expect(not(canFinal(state,literalFreeze,{ir:101,vk:201,legacyVk:0},1,0)))
    .then(finish(literalFreeze)).expect(pkFreeVerifierWitness and safety)
  run unusedResetLiteralTest = beforeFinal(ColdVerify,ResetCase).expect(state.statement == literalReset)
    .expect(state.installedOps == literalToggleMap and state.selectedOperation == {ir:104,vk:204,legacyVk:0})
    .expect(not(canFinal(state,literalReset,{ir:103,vk:203,legacyVk:0},1,0)))
    .then(finish(literalReset)).expect(pkFreeVerifierWitness and safety)
  run sameDurableKeysTest = admitted(ColdProve,A1).expect(state.installedOps == literalMeteredMap and state.selectedPk == 501)
    .then(admitted(ColdProve,A2)).expect(state.installedOps == literalMeteredMap and state.selectedPk == 501)
    .then(admitted(ColdProve,B1)).expect(state.installedOps == literalToggleMap and state.selectedPk == 503)
    .then(admitted(ColdProve,B2)).expect(state.installedOps == literalToggleMap and state.selectedPk == 503)
  run requestFreeKeygenTest = 3.reps(i => {
    val c = List(BadAmount,Disabled,Overflow).nth(i)
    admitted(Keygen,c).expect(not(validRequest(c)) and state.seed == seed(c) and state.rawOwner == 0)
      .then(writeLeaves(Set(),Set(1,2,3))).then(publish(Set()))
      .expect(keygenWitness and safety and not(state.visited.contains(Prepared)))
  })
  run invalidRequestsCannotPrepareTest = 3.reps(i => {
    val c = List(BadAmount,Disabled,Overflow).nth(i)
    admitted(ColdProve,c).expect(not(canPrepare(state,1))).expect(state.phase == Installed and state.seed == seed(c))
  })
  run requestFreeSeedShapeTest = init
    .expect(seedAdmitted(Metered,seed(BadAmount)) and seedAdmitted(Toggle,seed(Disabled)) and seedAdmitted(Toggle,seed(Overflow)))
    .expect(not(seedAdmitted(Metered,{...seed(A1),values:Map("running" -> BoolValue(true),"enabled" -> BoolValue(true))})))
    .expect(not(seedAdmitted(Metered,{...seed(A1),values:Map("running" -> UIntValue(-1),"enabled" -> BoolValue(true))})))
    .expect(not(seedAdmitted(Metered,{...seed(A1),values:Map("running" -> UIntValue(0),"enabled" -> BoolValue(true),"hidden" -> UIntValue(1))})))
    .expect(not(seedAdmitted(Metered,{...seed(A1),balances:List({owner:"alice",asset:literalAsset,amount:100})})))
    .expect(not(seedAdmitted(Metered,{...seed(A1),duties:List({name:"charge",asset:literalAsset,debtor:"bob",creditor:"alice",present:true,amount:9})})))
    .expect(not(seedAdmitted(Toggle,{...seed(B1),balances:List({owner:"alice",asset:literalAsset,amount:1})})))
    .expect(not(canPreflight({...state,frame:{...literalFrame,ledgerParameters:999}},Keygen,A1,exported(A1),keyless(Metered),limits)))
  run coldFullFrameAdmissionIsLateTest = init
    .then(all {state' = {...state,frame:{...literalFrame,ledgerParameters:999}}})
    .then(preflight(ColdVerify,A1,exported(A1),literalMeteredMap,limits))
    .expect(preflightWitness and not(installedWitness))
    .then(discover(Map(Settle -> 17,Freeze -> 16),ExternalSdkDiscovery))
    .then(regenerate(PublicOnly,ExternalSourceDerivation)).then(compare(exported(A1),ColdVerify,0))
    .expect(comparisonWitness and not(canInstall(state,literalMeteredMap,state.frame)))
    .expect(not(canInstall(state,literalMeteredMap,literalFrame)))
    .expect(safety and not(acceptedWitness))
  run acyclicActualSnapshotCommitmentsTest = admitted(Keygen,A1)
    .expect(state.snapshotCommitment == (literalFrame,literalMeteredMap))
    .expect(state.descriptorCommitment == ({source:Metered,staticBinding:21,build:22,profile:InstalledLocal,
      parameterProfile:23,qualification:24,anchor:25,phasePolicy:26},(literalFrame,literalMeteredMap)))
    .expect(acyclicCommitments and state.manifest.candidateExpectedHash == 999)
  run keylessSeedRefusalTest = init
    .expect(not(canPreflight(state,Keygen,A1,exported(A1),literalMeteredMap,limits)))
    .expect(not(canPreflight(state,Keygen,A1,exported(A1),keyless(Metered).put(Freeze,{ir:1,vk:0,legacyVk:0}),limits)))
    .expect(not(canPreflight(state,Keygen,A1,exported(A1),keyless(Metered).put(Settle,{ir:0,vk:0,legacyVk:1}),limits)))
    .then(refuse(SeedInstalled)).expect(refusedWitness and not(state.markerWritten) and state.frame == literalFrame)
  run profileSeparationTest = init
    .expect(legacyAdmits(Metered,LegacyKeyless,keyless(Metered)))
    .expect(not(legacyAdmits(Metered,InstalledLocal,keyless(Metered))))
    .expect(not(legacyAdmits(Metered,LegacyKeyless,literalMeteredMap)))
    .expect(not(canPreflight(state,ColdVerify,A1,{...exported(A1),metadata:{...metadata(Metered),profile:LegacyKeyless}},literalMeteredMap,limits)))
  run completeMetadataRefusalTest = init.expect(List(
    {...metadata(Metered),source:Toggle},{...metadata(Metered),staticBinding:999},
    {...metadata(Metered),build:999},{...metadata(Metered),profile:WrongProfile},
    {...metadata(Metered),parameterProfile:999},{...metadata(Metered),qualification:999},
    {...metadata(Metered),anchor:999},{...metadata(Metered),phasePolicy:999})
    .foldl(true,(ok,m) => ok and not(canPreflight(state,ColdVerify,A1,{...exported(A1),metadata:m},literalMeteredMap,limits))))
  run manifestNamesRefusalTest = init
    .expect(not(canPreflight(state,ColdVerify,A1,{...exported(A1),names:List(Settle)},literalMeteredMap,limits)))
    .expect(not(canPreflight(state,ColdVerify,A1,{...exported(A1),names:List(Settle,Freeze,Freeze)},literalMeteredMap,limits)))
    .expect(not(canPreflight(state,ColdVerify,A1,{...exported(A1),names:List(Settle,Step)},literalMeteredMap,limits)))
    .expect(not(canPreflight(state,ColdVerify,A1,{...exported(A1),keys:sourceKeys(Metered).put(Step,key(Step))},literalMeteredMap,limits)))
  run operationMapOrderingNotAuthorityTest = init
    .expect(canPreflight(state,ColdVerify,A1,{...exported(A1),names:List(Freeze,Settle)},literalMeteredMap,limits))
  run staticSourceAndPinnedMetadataBeforeNativeTest = init.expect(List(
    {...key(Settle),ir:999},{...key(Settle),manifest:999},{...key(Settle),parameters:999},
    {...key(Settle),degree:20,parameters:1020}).foldl(true,(ok,k) => ok and
      not(canPreflight(state,ColdVerify,A1,{...exported(A1),keys:sourceKeys(Metered).put(Settle,k)},literalMeteredMap,limits))))
    .expect(not(discoveryWitness) and state.importedKeyInitializations == 0)
  run workLimitsRefusalTest = init.expect(List({...limits,degree:0},{...limits,actions:0},
    {...limits,srsBytes:0},{...limits,keyBytes:0},{...limits,aggregateBytes:0},{...limits,inputBytes:0},
    {...limits,actions:1}).foldl(true,(ok,l) => ok and not(canPreflight(state,ColdVerify,A1,exported(A1),literalMeteredMap,l))))
  run discoveredDegreeOnlyLimitTest = init
    .then(preflight(ColdVerify,A1,exported(A1),literalMeteredMap,{...limits,degree:9}))
    .expect(not(canDiscover(state,Map(Settle -> 17,Freeze -> 16),CallerClaimedDegree)))
    .then(discover(Map(Settle -> 17,Freeze -> 16),ExternalSdkDiscovery))
    .expect(discoveryWitness and not(discoveredWithinLimits(state)))
    .expect(not(canRegenerate(state,PublicOnly,ExternalSourceDerivation)))
    .expect(state.phase == Discovered and state.selectedPk == 0)
  run unsupportedPinnedDegreeTest = init.then(preflight(ColdVerify,A1,exported(A1),literalMeteredMap,{...limits,degree:25}))
    .then(discover(Map(Settle -> 20,Freeze -> 16),ExternalSdkDiscovery))
    .expect(not(canRegenerate(state,PublicOnly,ExternalSourceDerivation)))
  run importedHashCannotDeriveTest = beforeDerivation(ColdVerify,A1)
    .expect(not(canRegenerate(state,PublicOnly,ImportedClaimsOnly)))
    .expect(not(canRegenerate(state,PublicOnly,ExternalFailure))).expect(not(state.markerWritten))
  run derivationPurposeRefusalTest = beforeDerivation(ColdVerify,A1)
    .expect(not(canRegenerate(state,SelectedFull,ExternalSourceDerivation))).then(refuse(BadImportPurpose))
    .expect(purposeRefusalWitness and state.selectedPk == 0 and state.pkReads == 0)
  run publicReaderIgnoresPkLeavesTest = beforeComparison(ColdVerify,A1)
    .then(compare({...exported(A1),pkLeaf:999},ColdVerify,0))
    .expect(state.pkReads == 0 and state.selectedPk == 0 and state.importMatched)
    .then(install(literalMeteredMap,literalFrame)).then(prepare(1)).then(finish(literalA1))
    .expect(pkFreeVerifierWitness and safety)
  run readOnlyVerifierDoesNotPublishTest = beforeFinal(ColdVerify,A1).then(finish(literalA1))
    .expect(pkFreeVerifierWitness and state.phase == ExternalResultChecked and not(state.markerWritten) and state.leaves == Set())
    .expect(not(canWriteLeaves(state,Set(),Set(1,2,3))))
  run wrongColdImportPurposeTest = beforeComparison(ColdVerify,A1).expect(not(canCompare(state,exported(A1),ColdProve,0)))
  run selectedPkRefusalTest = beforeComparison(ColdProve,A1)
    .expect(not(canCompare(state,{...exported(A1),pkLeaf:999},ColdProve,0))).expect(not(state.importMatched))
  run lazyNativeInitializationOrderTest = init.expect(not(canCompare(state,exported(A1),ColdVerify,0)))
    .then(beforeComparison(ColdVerify,A1)).expect(not(canCompare(state,exported(A1),ColdVerify,1)))
    .then(refuse(EarlyKeyInit)).expect(lazyInitRefusalWitness and state.importedKeyInitializations == 0)
  run weakOpaqueVkRefusalTest = beforeComparison(ColdVerify,A1)
    .expect(not(canCompare(state,weakManifest(state),ColdVerify,0))).then(refuse(WeakImportedKey))
    .expect(weakKeyRefusalWitness and not(state.importMatched) and not(state.markerWritten))
  run completeImportedKeyIdentityTest = beforeComparison(ColdVerify,A1).expect(List(
    {...key(Settle),ir:999},{...key(Settle),vk:999},{...key(Settle),manifest:999},
    {...key(Settle),degree:18},{...key(Settle),parameters:999}).foldl(true,(ok,k) =>
      ok and not(canCompare(state,{...exported(A1),keys:sourceKeys(Metered).put(Settle,k)},ColdVerify,0))))
  run wholeMapUnusedActionRefusalTest = beforeInstallation(ColdVerify,A1)
    .expect(not(canInstall(state,literalMeteredMap.put(Freeze,{ir:102,vk:999,legacyVk:0}),literalFrame)))
    .then(refuse(UnusedActionKey)).expect(unusedActionRefusalWitness and not(state.markerWritten))
  run allOperationFieldsRefusalTest = beforeInstallation(ColdVerify,A1)
    .expect(List({ir:0,vk:201,legacyVk:0},{ir:101,vk:0,legacyVk:0},{ir:101,vk:201,legacyVk:9},
      {ir:999,vk:201,legacyVk:0}).foldl(true,(ok,op) => ok and not(canInstall(state,literalMeteredMap.put(Settle,op),literalFrame))))
    .expect(not(canInstall(state,Map(Settle -> {ir:101,vk:201,legacyVk:0}),literalFrame)))
    .expect(not(canInstall(state,literalMeteredMap.put(Step,{ir:103,vk:203,legacyVk:0}),literalFrame)))
  run lateVkCorrespondenceTest = init.then(preflight(ColdVerify,A1,
    {...exported(A1),keys:sourceKeys(Metered).put(Freeze,{...key(Freeze),vk:999})},
    literalMeteredMap.put(Freeze,{ir:102,vk:999,legacyVk:0}),limits))
    .expect(preflightWitness)
    .then(discover(Map(Settle -> 17,Freeze -> 16),ExternalSdkDiscovery))
    .then(regenerate(PublicOnly,ExternalSourceDerivation)).expect(not(canCompare(state,state.manifest,ColdVerify,0)))
  run completeFramePreservationTest = beforeInstallation(Keygen,A1).expect(List(
    {...literalFrame,data:999},{...literalFrame,maintenance:999},{...literalFrame,contractBalance:999},
    {...literalFrame,utxos:List(999)},{...literalFrame,extensions:List(999)},
    {...literalFrame,bookkeeping:List(999)},{...literalFrame,ledgerParameters:999},
    {...literalFrame,network:999},{...literalFrame,nightSupply:999})
    .foldl(true,(ok,f) => ok and not(canInstall(state,literalMeteredMap,f))))
    .then(install(literalMeteredMap,literalFrame)).expect(framePreservation and state.seed == seed(A1))
  run rawOwnerAndExpansionRefusalTest = beforeFinal(ColdProve,A1)
    .expect(not(canFinal(state,literalA1,state.selectedOperation,2,0)))
    .expect(not(canFinal(state,literalA1,state.selectedOperation,1,1)))
    .then(finalCompare(literalA1,state.selectedOperation,1,0))
    .expect(not(canFinal(state,literalA1,state.selectedOperation,1,0))).expect(singleRawExpansion)
  run wrongStoredOperationTest = beforeFinal(ColdVerify,A1)
    .expect(not(canFinal(state,literalA1,{ir:102,vk:202,legacyVk:0},1,0)))
    .then(refuse(WrongStoredOperation)).expect(refusedWitness and state.rawOwner == 1 and state.expansions == 0)
  run completeFinalStatementMutationTest = beforeFinal(ColdVerify,A1).expect(List(
    {...literalA1,parent:999},{...literalA1,binding:999},{...literalA1,communication:999},
    {...literalA1,pi:List(603,604)},{...literalA1,pi:List(604,603,605)},
    {...literalA1,transcript:List(606,0,0,607)},{...literalA1,chargedState:999},
    {...literalA1,measuredGas:999},{...literalA1,declaredGas:999},{...literalA1,version:2},
    {...literalA1,finalIntent:999},{...literalA1,events:List(("enabled",BoolValue(true)),("total",UIntValue(11)))},
    {...literalA1,effects:literalA1.effects.tail()},
    {...literalA1,effects:literalA1.effects.replaceAt(3,FeeEffect({asset:literalAsset,from:"alice",dst:"treasury",amount:3}))},
    {...literalA1,post:{...literalA1.post,values:Map("running" -> UIntValue(12),"enabled" -> BoolValue(true))}},
    {...literalA1,post:{...literalA1.post,duties:List()}},
    {...literalA1,post:{...literalA1.post,balances:List()}},
    {...literalA1,outputs:literalA1.outputs.replaceAt(0,{owner:"bob",asset:literalAsset,amount:14,finalIntent:999,ordinal:0})},
    {...literalA1,envelope:{...literalA1.envelope,offers:List()}},
    {...literalA1,envelope:{...literalA1.envelope,ttl:999}},
    {...literalA1,envelope:{...literalA1.envelope,randomness:999}},
    {...literalA1,envelope:{...literalA1.envelope,actions:List(Settle,Freeze)}},
    {...literalA1,envelope:{...literalA1.envelope,extraActions:List(999)}},literalA2)
    .foldl(true,(ok,st) => ok and not(canFinal(state,st,state.selectedOperation,1,0))))
    .then(refuse(FinalMutation)).expect(refusedWitness and not(state.markerWritten))
  run externalCryptoFailureTest = beforeFinal(ColdVerify,A1).then(finalCompare(literalA1,state.selectedOperation,1,0))
    .expect(not(canCheckExternalResult(state,ProofMarker,Real,Verifying,ExternalRealFailure))).expect(not(state.markerWritten))
  run markerModeFeatureRefusalTest = beforeFinal(ColdVerify,A1).then(finalCompare(literalA1,state.selectedOperation,1,0))
    .expect(not(canCheckExternalResult(state,PreimageMarker,Real,Verifying,ExternalRealSuccess)))
    .expect(not(canCheckExternalResult(state,ErasedMarker,Real,Verifying,ExternalRealSuccess)))
    .expect(not(canCheckExternalResult(state,ProofMarker,Mock,Verifying,ExternalRealSuccess)))
    .expect(not(canCheckExternalResult(state,ProofMarker,Real,CompiledOut,ExternalRealSuccess)))
  run markerLastAndCollisionTest = admitted(Keygen,A1).expect(not(canPublish(state,Set())))
    .expect(not(canWriteLeaves(state,Set(1),Set(1,2,3)))).expect(not(canWriteLeaves(state,Set(4),Set(1,2,3))))
    .expect(not(canWriteLeaves(state,Set(),Set(1)))).expect(not(state.markerWritten) and state.leaves == Set())
    .then(writeLeaves(Set(99),Set(1,2,3))).expect(not(canPublish(state,Set(4))))
    .expect(state.existingFiles == Set(99) and not(state.markerWritten)).then(publish(Set(99)))
    .expect(keygenWitness and safety and state.existingFiles == Set(99))
  run publicationFailureNeverSucceedsTest = admitted(Keygen,A1).then(refuse(PartialWrite))
    .expect(refusedWitness and state.leaves == Set(1) and not(state.markerWritten)).expect(not(canPublish(state,Set())))
  run publicationCollisionPreservesOccupiedLeafTest = admitted(Keygen,A1).then(refuse(Collision))
    .expect(refusedWitness and state.existingFiles == Set(1) and state.leaves == Set() and not(state.markerWritten))
  // Deliberately inadequate independent validators demonstrate sensitivity.
  pure def weakCandidateHashOnly(m: Manifest): bool = m.candidateExpectedHash == 999
  pure def weakSelectedOnly(ops: Entry -> Operation): bool = ops.get(Settle) == {ir:101,vk:201,legacyVk:0}
  run distinguishingWeakGuardsTest = beforeComparison(ColdVerify,A1)
    .expect(weakCandidateHashOnly(weakManifest(state)))
    .expect(not(canCompare(state,weakManifest(state),ColdVerify,0)))
    .then(compare(exported(A1),ColdVerify,0))
    .expect(weakSelectedOnly(literalMeteredMap.put(Freeze,{ir:102,vk:999,legacyVk:0})))
    .expect(not(canInstall(state,literalMeteredMap.put(Freeze,{ir:102,vk:999,legacyVk:0}),literalFrame)))
  run witnessesNotTrueAtInitTest = init.expect(safety and not(acceptedWitness) and not(refusedWitness)
    and not(keygenWitness) and not(pkFreeVerifierWitness) and not(lazyInitRefusalWitness)
    and not(unusedActionRefusalWitness) and not(purposeRefusalWitness) and not(weakKeyRefusalWitness))
  run blockedPreflightActionTest = init.then(preflight(Keygen,A1,exported(A1),literalMeteredMap,limits).fail())
  run blockedRegenerationActionTest = beforeDerivation(ColdVerify,A1).then(regenerate(SelectedFull,ExternalSourceDerivation).fail())
  run blockedImportActionTest = beforeComparison(ColdVerify,A1).then(compare(weakManifest(state),ColdVerify,0).fail())
  run blockedInstallActionTest = beforeInstallation(ColdVerify,A1).then(install(literalMeteredMap.put(Freeze,{ir:102,vk:999,legacyVk:0}),literalFrame).fail())
  run blockedRawExpansionActionTest = beforeFinal(ColdProve,A1).then(finalCompare(literalA1,state.selectedOperation,1,1).fail())
  run blockedCryptoActionTest = beforeFinal(ColdVerify,A1).then(finalCompare(literalA1,state.selectedOperation,1,0))
    .then(checkExternalResult(ProofMarker,Real,Verifying,ExternalRealFailure).fail())
  run blockedPublicationActionTest = admitted(Keygen,A1).then(writeLeaves(Set(1),Set(1,2,3)).fail())
  // Explicit reaching controls; these are expected failure-predicate violations
  // when selected as --invariant, not failures of safety or crypto experiments.
  val neverAcceptedControl = not(acceptedWitness)
  val neverRefusedControl = not(refusedWitness)
  val neverPkFreeVerifierControl = not(pkFreeVerifierWitness)
}
