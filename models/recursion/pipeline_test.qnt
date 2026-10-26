// Deterministic end-to-end traces, guard checks and four mutation witnesses.
module pipeline_case {
  const CASE_LEAVES: int
  const CASE_ARITY: int
  import pipeline_model(LEAVES = CASE_LEAVES, ARITY = CASE_ARITY) as machine from "./pipeline"

  pure def first(ids: Set[int]): int = ids.fold(100000, (a, b) => if (a < b) a else b)

  // Test harness only: deterministic scheduling without re-keying. The padding
  // at machine::terminal states permits bounded reps; production machine::step has no stutter.
  action drain =
    if (machine::terminal(machine::s)) machine::s' = machine::s
    else if (machine::JOBS.exists(j => machine::canAdmit(machine::s, j))) machine::admit(first(machine::JOBS.filter(j => machine::canAdmit(machine::s, j))))
    else if (machine::JOBS.exists(j => machine::canAdvance(machine::s, j))) machine::advance(first(machine::JOBS.filter(j => machine::canAdvance(machine::s, j))))
    else if (machine::BATCHES.exists(b => machine::canDecide(machine::s, b))) machine::decide(first(machine::BATCHES.filter(b => machine::canDecide(machine::s, b))))
    else if (machine::BATCHES.exists(b => machine::canSubmit(machine::s, b))) machine::submit(first(machine::BATCHES.filter(b => machine::canSubmit(machine::s, b))))
    else if (machine::BATCHES.exists(b => machine::canFinalize(machine::s, b))) machine::finalize(first(machine::BATCHES.filter(b => machine::canFinalize(machine::s, b))))
    else all { false, machine::s' = machine::s }

  run proveJob(j: int): bool = machine::admit(j).then(7.reps(_ => machine::advance(j)))
  run proveTree(b: int): bool = machine::WIDTHS.length().reps(level =>
    machine::WIDTHS[level].reps(slot => proveJob(b * 1000 + level * 16 + slot)))
  run proveDecider(b: int): bool = proveTree(b).then(proveJob(machine::decider(b)))
  run prepareSubmission(b: int): bool = proveDecider(b).then(machine::decide(b)).then(machine::submit(b))

  run complete = machine::initWithHostLoad(60000)
    .expect(machine::LEAVES == CASE_LEAVES and machine::ARITY == CASE_ARITY and machine::configurationOk and machine::TREE.filter(j => machine::role(j) == machine::Leaf).size() == machine::LEAVES
      and machine::TREE.forall(j => machine::children(j).size() <= machine::ARITY)
      and machine::TREE.forall(j => machine::JOBS.filter(p => machine::children(p).contains(j)).size() == 1))
    .then((10 * machine::JOBS.size()).reps(_ => drain.expect(machine::allInvariants)))
    .expect(machine::terminal(machine::s) and machine::s.finalized == [0, 1]
      and machine::ramUsed(machine::s) == 0 and machine::coresUsed(machine::s) == 0 and machine::gpuUsers(machine::s) == Set())

  run phaseTimingAndRelease = machine::initWithHostLoad(0).then(machine::admit(0))
    .expect(machine::s.jobs.get(0).phase == machine::KeysReady and machine::s.clockMs == 1000 and machine::ramUsed(machine::s) == 1500)
    .then(machine::advance(0)).expect(machine::s.jobs.get(0).phase == machine::Witnessed and machine::s.clockMs == 3460)
    .then(machine::advance(0)).expect(machine::s.jobs.get(0).phase == machine::Committed and machine::gpuUsers(machine::s) == Set(0))
    .then(machine::advance(0)).expect(machine::s.jobs.get(0).phase == machine::LookupsArgued and machine::gpuUsers(machine::s) == Set())
    .then(machine::advance(0)).expect(machine::s.jobs.get(0).phase == machine::Quotiented and machine::gpuUsers(machine::s) == Set(0))
    .then(machine::advance(0)).expect(machine::s.jobs.get(0).phase == machine::Opened)
    .then(machine::advance(0)).expect(machine::s.jobs.get(0).phase == machine::Proved and machine::s.clockMs == 13300)
    .then(machine::advance(0)).expect(machine::s.jobs.get(0).phase == machine::Verified and machine::s.clockMs == 13321
      and machine::ramUsed(machine::s) == 0 and machine::coresUsed(machine::s) == 0 and machine::allInvariants)

  run queuedChildrenRefused = machine::initWithHostLoad(0).then(machine::admit(machine::root(0)).fail())
  run unverifiedLeafRefused = machine::initWithHostLoad(0).then(machine::admit(0))
    .then(6.reps(_ => machine::advance(0))).expect(machine::s.jobs.get(0).phase == machine::Proved)
    .then(machine::admit(machine::root(0)).fail())
  run unverifiedParentRefused = machine::initWithHostLoad(0).then(proveJob(0))
    .then(machine::admit(machine::root(0))).then(6.reps(_ => machine::advance(machine::root(0))))
    .expect(machine::s.jobs.get(machine::root(0)).phase == machine::Proved).then(machine::admit(machine::decider(0)).fail())
  run consumedChildCannotBeClaimedAgain = machine::initWithHostLoad(0).then(proveJob(0))
    .then(machine::admit(machine::root(0))).expect(machine::s.jobs.get(0).phase == machine::Consumed
      and machine::s.jobs.get(0).uses == 1 and machine::verifiedChildren and machine::singleUse)
    .then(machine::admit(machine::root(0)).fail())

  run singleGpuContention = machine::initWithHostLoad(0).then(machine::admit(0)).then(machine::admit(1000))
    .then(machine::advance(0)).then(machine::advance(0)).then(machine::advance(1000))
    .expect(machine::gpuUsers(machine::s) == Set(0) and machine::cpuGpuCaps and not(machine::canAdvance(machine::s, 1000))).then(machine::advance(0)).then(machine::advance(1000))
    .expect(machine::gpuUsers(machine::s) == Set(1000) and machine::cpuGpuCaps and machine::memoryCaps)
  run exactRunRamBoundary = machine::initWithHostLoad(0).then(proveJob(0)).then(proveJob(1000))
    .then(machine::admit(machine::root(0))).then(machine::admit(machine::root(1)))
    .expect(machine::ramUsed(machine::s) == machine::RUN_RAM_MB and machine::allInvariants)
  run hostRamBoundary = machine::initWithHostLoad(73500).then(proveJob(0)).then(machine::admit(machine::root(0)))
    .expect(machine::ramUsed(machine::s) + machine::peakMb(1000) < machine::RUN_RAM_MB
      and machine::s.otherHostMb + machine::ramUsed(machine::s) + machine::peakMb(1000) > machine::HOST_RAM_MB and machine::allInvariants)
    .then(machine::admit(1000).fail())
  run cpuBoundary = machine::initWithHostLoad(0).then(10.reps(i => machine::admit(i)))
    .expect(machine::coresUsed(machine::s) == 20 and machine::ramUsed(machine::s) == 15000
      and machine::allInvariants).then(machine::admit(10).fail())

  // Reach the RAM edge via real proof completions, rather than an invented state.
  run ramEdge = machine::initWithHostLoad(60000).then(proveTree(0)).then(proveJob(1000))
    .then(machine::admit(machine::decider(0))).expect(machine::ramUsed(machine::s) == 16500 and machine::allInvariants)
  run overAdmissionRefused = ramEdge.then(machine::admit(machine::root(1)).fail())
  run mutantOverAdmissionCaught = ramEdge.then(machine::mutantOverAdmission(machine::root(1)))
    .expect(machine::ramUsed(machine::s) == 32500 and machine::s.otherHostMb + machine::ramUsed(machine::s) == 92500 and not(machine::memoryCaps)
      and machine::cpuGpuCaps and machine::verifiedChildren and machine::singleUse and machine::vkBinding
      and machine::orderedBatches and machine::noDeadlock and machine::resourceAccounting)
  run mutantChildReuseCaught = machine::initWithHostLoad(0).then(proveJob(0))
    .then(machine::mutantChildReuse(machine::root(0), 0))
    .expect(machine::s.jobs.get(0).uses == 2 and not(machine::singleUse) and machine::verifiedChildren
      and machine::memoryCaps and machine::cpuGpuCaps and machine::vkBinding and machine::orderedBatches
      and machine::noDeadlock and machine::resourceAccounting)

  run changedParent = machine::initWithHostLoad(0).then(proveTree(0)).then(machine::reproveParent(0))
    .then(7.reps(_ => machine::advance(machine::root(0)))).then(proveJob(machine::decider(0)))
    .expect(machine::s.parentVk.get(0) == 2 and machine::s.installedVk == 1
      and machine::s.jobs.get(machine::decider(0)).vk == 2 and machine::allInvariants)
  run changedVkRefusedAndFailed = changedParent.expect(not(machine::canDecide(machine::s, 0)))
    .then(machine::rejectVk(0)).expect(machine::terminal(machine::s) and machine::s.finalized == []
      and machine::ramUsed(machine::s) == 0 and machine::gpuUsers(machine::s) == Set() and machine::allInvariants)
  run mutantVkRebindingCaught = changedParent.then(machine::mutantVkRebinding(0))
    .expect(not(machine::vkBinding) and machine::memoryCaps and machine::cpuGpuCaps and machine::verifiedChildren
      and machine::singleUse and machine::orderedBatches and machine::noDeadlock and machine::resourceAccounting)
    .then(machine::submit(0)).then(machine::finalize(0))
    .expect(machine::s.jobs.get(machine::ledger(0)).phase == machine::Finalized and machine::s.parentVk.get(0) == 2
      and machine::s.installedVk == 1 and not(machine::vkBinding))
  run explicitReinstallAllowsNewVk = machine::initWithHostLoad(0).then(proveTree(0))
    .then(machine::reproveParent(0)).then(7.reps(_ => machine::advance(machine::root(0)))).then(machine::installVk(2))
    .then(proveJob(machine::decider(0))).then(machine::decide(0)).then(machine::submit(0)).then(machine::finalize(0))
    .expect(machine::s.finalized == [0] and machine::s.installedVk == 2 and machine::allInvariants)
  run reinstallationWhileDecidingRefused = machine::initWithHostLoad(0)
    .then(prepareSubmission(0)).then(machine::installVk(2).fail())
  run consumedParentCannotChangeVk = machine::initWithHostLoad(0).then(proveDecider(0))
    .then(machine::reproveParent(0).fail())

  run reorderedSettlementRefused = machine::initWithHostLoad(0).then(prepareSubmission(1))
    .expect(machine::s.jobs.get(machine::ledger(1)).phase == machine::Submitted and machine::noDeadlock and machine::orderedBatches)
    .then(machine::finalize(1).fail())
  run mutantReorderedBatchCaught = machine::initWithHostLoad(0).then(prepareSubmission(1))
    .then(machine::mutantReorderedBatch(1))
    .expect(machine::s.finalized == [1] and not(machine::orderedBatches) and machine::memoryCaps and machine::cpuGpuCaps
      and machine::verifiedChildren and machine::singleUse and machine::vkBinding and machine::noDeadlock and machine::resourceAccounting)
  run outOfOrderSubmissionThenOrderedFinalization = machine::initWithHostLoad(0)
    .then(prepareSubmission(1)).then(prepareSubmission(0)).then(machine::finalize(0)).then(machine::finalize(1))
    .expect(machine::s.finalized == [0, 1] and machine::terminal(machine::s) and machine::allInvariants)

  run proofFailureReleasesEverything = machine::initWithHostLoad(0).then(machine::admit(0))
    .then(6.reps(_ => machine::advance(0))).then(machine::rejectProof(0))
    .expect(machine::terminal(machine::s) and machine::ramUsed(machine::s) == 0 and machine::coresUsed(machine::s) == 0
      and machine::gpuUsers(machine::s) == Set() and machine::s.finalized == [] and machine::allInvariants)
  run failurePreservesFinalizedPrefix = machine::initWithHostLoad(0)
    .then(prepareSubmission(0)).then(machine::finalize(0)).then(machine::admit(1000))
    .then(6.reps(_ => machine::advance(1000))).then(machine::rejectProof(1000))
    .expect(machine::terminal(machine::s) and machine::s.finalized == [0] and machine::allInvariants)
  run failedPredecessorAbortsSubmittedSuffix = machine::initWithHostLoad(0)
    .then(prepareSubmission(1)).then(machine::admit(0)).then(6.reps(_ => machine::advance(0)))
    .then(machine::rejectProof(0)).expect(machine::terminal(machine::s) and machine::ramUsed(machine::s) == 0
      and machine::s.jobs.get(machine::ledger(1)).phase == machine::Failed and machine::allInvariants)

  // A deliberately stuck fixture checks that the enabledness predicate can
  // detect deadlock; just counting unfinished jobs would miss this state.
  run deadlockPredicateDetectsStuckState = machine::initWithHostLoad(0)
    .then(machine::s' = { ...machine::s, jobs: machine::JOBS.mapBy(_ => { ...machine::EMPTY_JOB, phase: machine::Consumed }) })
    .expect(not(machine::terminal(machine::s)) and not(machine::noDeadlock))
}

module pipeline_test {
  import pipeline_case(CASE_LEAVES = 1, CASE_ARITY = 2) as p12
  import pipeline_case(CASE_LEAVES = 1, CASE_ARITY = 3) as p13
  import pipeline_case(CASE_LEAVES = 1, CASE_ARITY = 4) as p14
  import pipeline_case(CASE_LEAVES = 2, CASE_ARITY = 2) as p22
  import pipeline_case(CASE_LEAVES = 2, CASE_ARITY = 3) as p23
  import pipeline_case(CASE_LEAVES = 2, CASE_ARITY = 4) as p24
  import pipeline_case(CASE_LEAVES = 4, CASE_ARITY = 2) as p42
  import pipeline_case(CASE_LEAVES = 4, CASE_ARITY = 3) as p43
  import pipeline_case(CASE_LEAVES = 4, CASE_ARITY = 4) as p44
  import pipeline_case(CASE_LEAVES = 16, CASE_ARITY = 2) as p162
  import pipeline_case(CASE_LEAVES = 16, CASE_ARITY = 3) as p163
  import pipeline_case(CASE_LEAVES = 16, CASE_ARITY = 4) as p164

  run leaves1arity2Test = p12::complete
  run leaves1arity3Test = p13::complete
  run leaves1arity4Test = p14::complete
  run leaves2arity2Test = p22::complete
  run leaves2arity3Test = p23::complete
  run leaves2arity4Test = p24::complete
  run leaves4arity2Test = p42::complete
  run leaves4arity3Test = p43::complete
  run leaves4arity4Test = p44::complete
  run leaves16arity2Test = p162::complete
  run leaves16arity3Test = p163::complete
  run leaves16arity4Test = p164::complete

  run phaseTimingAndReleaseTest = p12::phaseTimingAndRelease
  run queuedChildrenRefusedTest = p12::queuedChildrenRefused
  run unverifiedLeafRefusedTest = p12::unverifiedLeafRefused
  run unverifiedParentRefusedTest = p12::unverifiedParentRefused
  run consumedChildCannotBeClaimedAgainTest = p12::consumedChildCannotBeClaimedAgain
  run singleGpuContentionTest = p12::singleGpuContention
  run exactRunRamBoundaryTest = p12::exactRunRamBoundary
  run hostRamBoundaryTest = p12::hostRamBoundary
  run cpuBoundaryTest = p162::cpuBoundary
  run overAdmissionRefusedTest = p12::overAdmissionRefused
  run mutantOverAdmissionCaughtTest = p12::mutantOverAdmissionCaught
  run mutantChildReuseCaughtTest = p12::mutantChildReuseCaught
  run changedVkRefusedAndFailedTest = p12::changedVkRefusedAndFailed
  run mutantVkRebindingCaughtTest = p12::mutantVkRebindingCaught
  run explicitReinstallAllowsNewVkTest = p12::explicitReinstallAllowsNewVk
  run reinstallationWhileDecidingRefusedTest = p12::reinstallationWhileDecidingRefused
  run consumedParentCannotChangeVkTest = p12::consumedParentCannotChangeVk
  run reorderedSettlementRefusedTest = p12::reorderedSettlementRefused
  run mutantReorderedBatchCaughtTest = p12::mutantReorderedBatchCaught
  run outOfOrderSubmissionThenOrderedFinalizationTest = p12::outOfOrderSubmissionThenOrderedFinalization
  run proofFailureReleasesEverythingTest = p12::proofFailureReleasesEverything
  run failurePreservesFinalizedPrefixTest = p12::failurePreservesFinalizedPrefix
  run failedPredecessorAbortsSubmittedSuffixTest = p12::failedPredecessorAbortsSubmittedSuffix
  run deadlockPredicateDetectsStuckStateTest = p12::deadlockPredicateDetectsStuckState
}
