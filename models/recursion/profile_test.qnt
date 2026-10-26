// Honest traces, threshold edges and one seeded mutant per requested rule.
// Mutants live only in this harness and never enter profile::step.
module profile_test {
  import profile.* from "./profile"

  pure val FIRST = phase(0, 50, UNSUPPORTED)
  pure val SECOND = phase(1, 100, { supported: true, value: Some(20) })
  pure val DIAGNOSTIC = phase(2, 100, UNSUPPORTED)
  pure def rulesOk(st: State): bool = ownershipOk(st) and reconciliationOk(st)
    and coverageOk(st) and peaksOk(st) and countersOk(st) and overheadOk(st) and bufferOk(st)
  run twoPhases = initWith(ProofAccepted, 5, 100)
    .then(addPhase(FIRST)).then(addPhase(SECOND))
    .expect(exclusiveSum(s) == 100 and s.unexplained == 0 and rulesOk(s))

  run honestClosedTest = twoPhases.then(closeProfile)
    .expect(s.closed and s.coverageValid and rulesOk(s))
  run honestDiagnosticsTest = twoPhases.then(addPhase(DIAGNOSTIC)).then(closeProfile)
    .expect(s.phases.size() == 3 and exclusiveSum(s) == 100
      and s.coverageValid and rulesOk(s))
  run honestOwnershipGuardsTest = twoPhases
    .expect(not(canAdd(s, { ...DIAGNOSTIC, exclusive: true }))
      and not(canAdd(s, { ...DIAGNOSTIC, id: 0 }))
      and not(canAdd(s, { ...DIAGNOSTIC, owner: "" })) and rulesOk(s))
  run honestFivePercentTest = initWith(ProofAccepted, 5, 100)
    .then(addPhase(FIRST)).then(addPhase(phase(1, 95, UNSUPPORTED))).then(closeProfile)
    .expect(s.unexplained == 5 and s.coverageValid and rulesOk(s))
  run honestSixPercentRejectedTest = initWith(ProofAccepted, 5, 100)
    .then(addPhase(FIRST)).then(addPhase(phase(1, 94, UNSUPPORTED))).then(closeProfile)
    .expect(s.unexplained == 6 and not(s.coverageValid) and rulesOk(s))
  // Exercise the discrepancy limb separately from the unexplained limb.
  run discrepancyEdgesTest = twoPhases.expect(and {
    coverageBounds({ ...s, wallEnd: 105 }),
    not(coverageBounds({ ...s, wallEnd: 106 })),
    coverageBounds({ ...s, wallEnd: 96 }),
    not(coverageBounds({ ...s, wallEnd: 95 })),
    not(coverageBounds({ ...s, wallEnd: 0 })),
    not(coverageBounds({ ...s, unexplained: -1 })),
  })
  run exactDiscrepancyThresholdTest = initWith(ProofAccepted, 5, 100).expect(and {
    coverageBounds({ ...s, phases: Set(phase(0, 95, UNSUPPORTED)), unexplained: 0 }),
    not(coverageBounds({ ...s, phases: Set(phase(0, 94, UNSUPPORTED)), unexplained: 0 })),
    not(coverageBounds({ ...s, phases: Set(phase(0, 93, UNSUPPORTED)), unexplained: 0 })),
  })
  run honestMemoryTest = twoPhases
    .expect(s.phases.forall(p => p.memory.quality == BoundaryOnly
      and p.memory.peakDelta == 0 and not(p.memory.certifiesInstantaneous)))
    .then(sampleRss(0, { at: 1, rss: 8 }))
    .expect(s.phases.exists(p => p.id == 0 and p.memory.samples.size() == 1
      and p.memory.quality == BoundaryOnly and p.memory.peakDelta == 0) and rssHonesty)
    .then(sampleRss(0, { at: 2, rss: 12 }))
    .expect(s.phases.exists(p => p.id == 0 and p.memory.quality == SampledLowerBound
      and p.memory.sampledPeak == 12 and p.memory.peakDelta == 2
      and not(p.memory.certifiesInstantaneous)) and rssHonesty)
    .then(closeProfile).expect(s.coverageValid and rulesOk(s))
  run memoryEdgesTest = twoPhases
    .expect(not(canSample(s, FIRST, { at: 0, rss: 12 }))
      and not(canSample(s, FIRST, { at: 50, rss: 12 })))
    .then(sampleRss(0, { at: 1, rss: 12 }))
    .expect(s.phases.filter(p => p.id == 0).forall(p => not(canSample(s, p, { at: 1, rss: 8 })))
      and peakDelta(12, 8) == 0 and peakDelta(8, 12) == 4 and rulesOk(s))
  run honestCountersTest = twoPhases.expect(and {
    countersOk(s), counterOk(UNSUPPORTED),
    counterOk({ supported: true, value: Some(0) }),
    not(counterOk({ supported: false, value: None("") })),
    not(counterOk({ supported: true, value: Some(-1) })),
    not(countersOk({ ...s, phases: Set({ ...FIRST,
      cpu: { supported: false, value: Some(0) } }) })),
  })
  run honestOverheadPassTest = initWith(ProofAccepted, 5, 100)
    .expect(s.overhead.verdict == Passed and overheadQualification)
  run honestOverheadNoiseTest = initWith(ProofAccepted, 5, 150)
    .expect(s.overhead.meanBp < 100 and s.overhead.verdict == Inconclusive and overheadQualification
      and overheadOk({ ...s, overhead: { ...s.overhead, verdict: Failed } }))
  run honestTooFewPairsTest = initWith(ProofAccepted, 4, 50)
    .expect(s.overhead.upper95Bp < 100 and s.overhead.verdict == Inconclusive
      and overheadQualification
      and not(overheadOk({ ...s, overhead: { ...s.overhead, verdict: Passed } })))

  run fillBuffer = twoPhases.then(sampleRss(0, { at: 1, rss: 8 }))
    .then(sampleRss(0, { at: 2, rss: 12 }))
    .then(sampleRss(1, { at: 51, rss: 8 }))
    .expect(s.events == CAPACITY and s.invalid and s.dropped == 0 and bufferIsolation)
  run honestOverflowTest = fillBuffer.then(sampleRss(1, { at: 52, rss: 12 }))
    .expect(s.events == CAPACITY and s.dropped == 1 and bufferIsolation)
    .then(closeProfile).expect(not(s.coverageValid)
      and s.proofOutcome == ProofAccepted and rulesOk(s))
  run honestRejectedProofUnaffectedTest = initWith(ProofRejected, 5, 100)
    .then(addPhase(FIRST)).then(addPhase(SECOND)).then(addPhase(DIAGNOSTIC))
    .then(sampleRss(0, { at: 1, rss: 8 })).then(sampleRss(0, { at: 2, rss: 12 }))
    .then(sampleRss(1, { at: 51, rss: 8 })).then(closeProfile)
    .expect(s.invalid and s.dropped == 1 and s.proofOutcome == ProofRejected and rulesOk(s))
  run closedIsTerminalTest = twoPhases.then(closeProfile)
    .expect(not(canAdd(s, DIAGNOSTIC))
      and s.phases.forall(p => not(canSample(s, p, { at: 1, rss: 8 }))) and rulesOk(s))
    .then(closeProfile.fail())

  // M1: grant an overlapping diagnostic exclusive accounting credit.
  action mutant1 = s' = { ...s, phases: s.phases.union(Set({ ...DIAGNOSTIC, exclusive: true })) }
  run mutant1CaughtTest = twoPhases.then(mutant1)
    .expect(not(ownershipOk(s)) and not(exclusiveOwnership))
  // M2: silently discard unexplained time before reconciling.
  action mutant2 = s' = { ...s, unexplained: 0 }
  run mutant2CaughtTest = initWith(ProofAccepted, 5, 100).then(addPhase(FIRST))
    .then(mutant2).expect(not(reconciliationOk(s)) and not(wallReconciliation))
  // M3: label a profile with 6% unexplained time coverage-valid.
  action mutant3 = s' = { ...s, coverageValid: true }
  run mutant3CaughtTest = initWith(ProofAccepted, 5, 100)
    .then(addPhase(FIRST)).then(addPhase(phase(1, 94, UNSUPPORTED))).then(closeProfile)
    .then(mutant3).expect(not(coverageOk(s)) and not(coverageQualification)
      and reconciliationOk(s) and not(coverageBounds(s)))
  // M4: promote boundary-only data into an instantaneous peak certificate.
  action mutant4 = s' = { ...s, phases: s.phases.map(p =>
    if (p.id == 0) { ...p, memory: { ...p.memory, certifiesInstantaneous: true } } else p) }
  run mutant4CaughtTest = twoPhases.then(mutant4)
    .expect(not(peaksOk(s)) and not(rssHonesty))
  // Independently ensure the numeric delta and quality limbs are not vacuous.
  run memoryPredicateEdgesTest = twoPhases.expect(and {
    not(memoryOk({ ...FIRST, memory: { ...FIRST.memory, peakDelta: -2 } })),
    not(memoryOk({ ...FIRST, memory: { ...FIRST.memory, quality: SampledLowerBound } })),
    not(memoryOk({ ...FIRST, memory: { ...FIRST.memory, sampledPeak: 12 } })),
  })
  // M5: serialize unsupported hardware as a measured zero.
  action mutant5 = s' = { ...s, counter: { supported: false, value: Some(0) } }
  run mutant5CaughtTest = twoPhases.then(mutant5)
    .expect(not(countersOk(s)) and not(unsupportedCounters))
  // M6: pass based on the sub-1% mean despite a 1.5% upper bound.
  action mutant6 = s' = { ...s, overhead: { ...s.overhead, verdict: Passed } }
  run mutant6CaughtTest = initWith(ProofAccepted, 5, 150).then(mutant6)
    .expect(not(overheadOk(s)) and not(overheadQualification))
  // M7: conceal a full event buffer, allowing a false coverage claim.
  action mutant7 = s' = { ...s, invalid: false }
  run mutant7CaughtTest = fillBuffer.then(mutant7)
    .expect(not(bufferOk(s)) and not(bufferIsolation))
  run outcomePredicateEdgeTest = twoPhases.expect(not(bufferOk({ ...s, proofOutcome: ProofRejected })))
}
