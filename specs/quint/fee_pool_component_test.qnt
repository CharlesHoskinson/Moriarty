// Deterministic model QA. Public time inputs are not authenticated ledger observations.
module fee_pool_component_test {
 import fee_pool_component.* from "./fee_pool_component"
 run golden111Test = init
   .then(intake(0,111,86400,17,10))
   .then(registerOperatorA(1,true,1)).then(registerOperatorB(2,true,2))
   .then(registerDa(0,false,0)).then(registerWatcher(1,true,1))
   .then(submitOrdinary(5)).then(submitStipend(5))
   .then(settle(55,55,10,10,5,5,15,15,15))
   .expect(allInvariants and world.s.midnightAmount==16 and world.s.backstop==15 and world.s.watcherHeld==5)
   .then(allocateOperatorA(3,18,1)).then(allocateOperatorB(3,36,2)).then(allocateDa(1,0,0))
   .expect(world.s.carried==11 and world.duties.get("reward_da").present and world.duties.get("reward_da").amount==0)
   .then(finish(2)).then(spendWatcher(7,3))
   .expect(allInvariants and world.s.dustAmount==9 and world.s.backstop==17 and world.s.watcherHeld==2)
   .then(claimOrdinary(86400)).then(claimStipend(86400))
   .then(claimOperatorA(2678400)).then(claimOperatorB(2678400))
   .then(claimMidnight(86400)).then(claimDust(86400)).then(claimWatcherSpend(86400))
   .expect(allInvariants and world.s.claimedTotal==92 and world.cash.get("fee-pool")==19 and wLateClaim)
   .expect(not(canClaimOperatorA(world,2678400))).expect(not(canClaimDa(world,2678400))).expect(not(canClaimL2(world,86400)))
   .expect(world.s.claimedTotal==92 and allInvariants)
 run residual99Test = init.then(intake(0,99,86400,0,10))
   .then(settle(54,45,9,90,4,95,14,85,0))
   .expect(allInvariants and world.s.midnightAmount==18 and world.s.midnightAmount-(99*15/100)==4 and not(world.s.midnightAmount<=99*15/100+3))
 run originalWeightReverseOrderTest = init.then(intake(0,100,86400,0,10))
   .then(registerOperatorA(1,true,1)).then(registerOperatorB(2,true,2))
   .then(settle(55,0,10,0,5,0,15,0,0))
   .then(allocateOperatorB(3,36,2)).then(allocateOperatorA(3,18,1))
   .expect(allInvariants and world.s.opAAmount==18 and world.s.opBAmount==36 and world.s.opRemaining==1)
   .then(finish(0)).expect(world.s.dustAmount==11 and allInvariants)
 run targetLoweringTest = init.then(intake(0,100,86400,17,10))
   .then(settle(55,0,10,0,5,0,15,0,15)).then(setTarget(0))
   .expect(world.s.backstop==15 and world.s.backstop>world.s.target and allInvariants)
   .then(finish(0)).expect(world.s.dustAmount==65 and world.s.backstop==15 and allInvariants)
 run allCostsAndZeroDutyTest = init.then(intake(0,10,86400,17,10)).then(submitOrdinary(10))
   .expect(not(canClaimOrdinary(world,86400)))
   .then(settle(0,0,0,0,0,0,0,0,0)).then(finish(0))
   .then(claimOrdinary(86400))
   .expect(allInvariants and world.cash.get("fee-pool")==0 and world.s.claimedTotal==10 and world.s.gross==10 and world.duties.get("treasury_dust").present)
   .expect(not(canClaimMidnight(world,86400))).expect(not(canClaimDust(world,86400)))
 run invalidWitnessAndRefillTest = init.then(intake(0,100,86400,17,10))
   .expect(not(canSettle(world,56,0,10,0,5,0,15,0,15)))
   .expect(not(canSettle(world,55,100,10,0,5,0,15,0,15)))
   .expect(not(canSettle(world,55,0,10,0,5,0,15,0,14)))
   .expect(not(world.s.settled) and world.s.carried==100 and world.history.length()==1 and allInvariants)
 run rosterAndMeterRejectionTest = init.then(intake(0,100,86400,17,5))
   .expect(not(canRegisterOperatorA(world,86401,false,0))).expect(not(canRegisterOperatorA(world,2,false,2)))
   .then(registerOperatorA(2,false,0)).expect(not(canRegisterOperatorA(world,1,true,1)))
   .expect(not(canSubmitStipend(world,1))).then(registerWatcher(0,true,0)).expect(not(canSubmitStipend(world,1)))
   .expect(not(canSubmitOrdinary(world,101))).then(submitOrdinary(1)).expect(not(canSubmitOrdinary(world,1)))
   .expect(allInvariants and world.s.metered==1 and world.s.opAWeight==0)
 run vestingAndLateClaimTest = init.then(intake(0,100,86400,0,5)).then(registerOperatorA(1,true,1))
   .then(settle(55,0,10,0,5,0,15,0,0)).then(allocateOperatorA(1,55,0))
   .expect(not(canClaimOperatorA(world,2678399))).expect(not(canClaimL2(world,86399)))
   .then(finish(0)).then(claimOperatorA(2678400))
   .expect(allInvariants and world.s.opAAmount==55 and world.duties.get("reward_operator_a").present and world.duties.get("reward_operator_a").amount==0 and wLateClaim)
 run finishAndPostSettlementRejectionTest = init.then(intake(0,100,86400,0,5)).then(registerDa(0,false,0))
   .then(settle(55,0,10,0,5,0,15,0,0))
   .expect(not(canFinish(world,0))).expect(not(canRegisterOperatorA(world,1,true,1))).expect(not(canSubmitOrdinary(world,1)))
   .then(allocateDa(1,0,0)).then(finish(0)).expect(not(canFinish(world,0))).expect(not(canAllocateDa(world,1,0,0)))
   .expect(allInvariants and world.s.dustAmount==65)
 run watcherIndependenceTest = init.then(intake(0,100,86400,0,5))
   .then(settle(55,0,10,0,5,0,15,0,0))
   .expect(not(canSpendWatcher(world,3,1))).expect(not(canSpendWatcher(world,7,6)))
   .then(spendWatcher(8,3)).expect(not(canSpendWatcher(world,7,1)))
   .expect(allInvariants and not(world.s.watcherRegistered) and world.s.watcherHeld==2 and world.s.metered==0)
 run noSecondStipendTest = init.then(intake(0,100,86400,0,5)).then(registerWatcher(1,true,1))
   .expect(not(canSubmitStipend(world,6))).then(submitStipend(5)).expect(not(canSubmitStipend(world,1)))
   .expect(allInvariants and world.s.stipendAmount==5 and world.s.metered==5 and world.s.watcherHeld==0)
 run pureMaximumArithmeticTest = all {
   M==18446744073709551615,
   M*M==340282366920938463426481119284349108225,
   division(M,M,M,M,0),
   not(division(M,M,M,M,M-1)),
   not(division(M,1,0,0,0)),
   not(division(M,1,1,M+1,0)),
   not(u64(M+1)),
   canIntake(emptyWorld,0,M,86400,M,M),
   not(canIntake(emptyWorld,0,M+1,86400,0,0)),
   not(canIntake(emptyWorld,0,1,86399,0,0)),
   not(canIntake({...emptyWorld,cash:emptyWorld.cash.set("funding-source",0)},0,1,86400,0,0)),
 }
 run maximumGrossSettlementTest = init.then(intake(0,M,86400,0,0))
   .then(settle(M*55/100,M*55%100,M*10/100,M*10%100,M*5/100,M*5%100,M*15/100,M*15%100,0))
   .expect(allInvariants and world.s.gross==M and world.s.midnightAmount>=M*15/100)
 run positiveDaAndL2ClaimsTest = init
   .then(intake(0,100,86400,0,0))
   .then(registerDa(1,true,1))
   .then(settle(55,0,10,0,5,0,15,0,0))
   .then(allocateDa(1,10,0))
   .expect(not(canClaimDa(world,2678399)) and not(canClaimL2(world,86399)))
   .then(claimDa(2678400)).then(claimL2(86400))
   .expect(allInvariants and world.s.claimedTotal==25
     and world.cash.get("da-provider")==10 and world.cash.get("l2-treasury")==15
     and world.duties.get("reward_da").amount==0
     and world.duties.get("treasury_l2").amount==0
     and wClaimDa and wClaimL2
     and not(canClaimDa(world,2678400)) and not(canClaimL2(world,86400)))
}
