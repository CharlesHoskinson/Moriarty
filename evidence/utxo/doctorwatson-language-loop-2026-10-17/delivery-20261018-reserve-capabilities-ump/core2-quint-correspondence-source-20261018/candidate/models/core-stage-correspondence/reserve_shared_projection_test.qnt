// One existing normal reserve trajectory; no model actions or domains change.
module reserve_shared_projection_test {
  import continuation_lifecycle.* from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_lifecycle"
  import continuation_prefix.Reserve from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"
  import continuation_prefix.FinancialFrame from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"
  import continuation_prefix.initialFrame from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"
  import continuation_prefix.applyStart from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"
  import continuation_prefix.StateChange from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"
  import continuation_prefix.LiabilityChange from "../../docs/audits/2026-10-15/core-continuations/quint-lifecycle/continuation_prefix"

  pure val expectedInitial: FinancialFrame = {
    specimen: Reserve,
    inputValue: 2,
    balances: Map("observer" -> 42, "reserve" -> 100, "target" -> 0, "treasury" -> 0),
    programState: Map("allocated" -> 0, "basis" -> 40, "marker" -> 77),
    dutyPresent: true,
    dutyAmount: 5,
    pendingSite: 0,
    captureValues: List(),
    logicalAttempted: 0,
    logicalCommitted: 0,
    effects: List(),
  }

  pure val expectedPrefix: FinancialFrame = {
    specimen: Reserve,
    inputValue: 2,
    balances: Map("observer" -> 42, "reserve" -> 100, "target" -> 0, "treasury" -> 0),
    programState: Map("allocated" -> 0, "basis" -> 35, "marker" -> 77),
    dutyPresent: true,
    dutyAmount: 47,
    pendingSite: 1,
    captureValues: List(2, 42),
    logicalAttempted: 11,
    logicalCommitted: 11,
    effects: List(StateChange({ before: 40, after: 35 }), LiabilityChange({ before: 5, after: 47 })),
  }

  pure val expected0: Observation = {
    c: {
      specimen: Reserve,
      balances: Map("observer" -> 42, "reserve" -> 100, "target" -> 0, "treasury" -> 0),
      programState: Map("allocated" -> 0, "basis" -> 35, "marker" -> 77),
      dutyPresent: true,
      dutyAmount: 47,
      site: 1,
      captures: List(2, 42),
      committed: 11,
    },
    last: {
      status: NotInvoked,
      attempted: 0,
      advanced: false,
      hasSegment: false,
      newContinuation: false,
      effects: List(),
      sourceSteps: List(),
      errorSteps: List(),
    },
    attemptedTotal: 11,
  }

  pure val expected1: Observation = {
    c: {
      specimen: Reserve,
      balances: Map("observer" -> 42, "reserve" -> 100, "target" -> 0, "treasury" -> 0),
      programState: Map("allocated" -> 0, "basis" -> 35, "marker" -> 77),
      dutyPresent: true,
      dutyAmount: 47,
      site: 1,
      captures: List(2, 42),
      committed: 11,
    },
    last: {
      status: Waiting,
      attempted: 4,
      advanced: false,
      hasSegment: false,
      newContinuation: false,
      effects: List(),
      sourceSteps: List(),
      errorSteps: List(),
    },
    attemptedTotal: 15,
  }

  pure val expected2: Observation = {
    c: {
      specimen: Reserve,
      balances: Map("observer" -> 42, "reserve" -> 58, "target" -> 42, "treasury" -> 0),
      programState: Map("allocated" -> 42, "basis" -> 78, "marker" -> 77),
      dutyPresent: true,
      dutyAmount: 5,
      site: 2,
      captures: List(2, 42, 43),
      committed: 26,
    },
    last: {
      status: Waiting,
      attempted: 15,
      advanced: true,
      hasSegment: true,
      newContinuation: true,
      effects: List(StateValue({ name: "allocated", before: 0, after: 42 }), TransferAmount(42), DutyValue({ before: 47, after: 5 }), StateValue({ name: "basis", before: 35, after: 78 })),
      sourceSteps: List(3, 4, 5, 6, 7),
      errorSteps: List(),
    },
    attemptedTotal: 30,
  }

  pure val expected3: Observation = {
    c: {
      specimen: Reserve,
      balances: Map("observer" -> 42, "reserve" -> 58, "target" -> 42, "treasury" -> 0),
      programState: Map("allocated" -> 42, "basis" -> 78, "marker" -> 77),
      dutyPresent: true,
      dutyAmount: 5,
      site: 2,
      captures: List(2, 42, 43),
      committed: 26,
    },
    last: {
      status: Waiting,
      attempted: 2,
      advanced: false,
      hasSegment: false,
      newContinuation: false,
      effects: List(),
      sourceSteps: List(),
      errorSteps: List(),
    },
    attemptedTotal: 32,
  }

  pure val expected4: Observation = {
    c: {
      specimen: Reserve,
      balances: Map("observer" -> 42, "reserve" -> 56, "target" -> 42, "treasury" -> 2),
      programState: Map("allocated" -> 42, "basis" -> 78, "marker" -> 77),
      dutyPresent: true,
      dutyAmount: 5,
      site: 0,
      captures: List(),
      committed: 36,
    },
    last: {
      status: Complete,
      attempted: 10,
      advanced: true,
      hasSegment: true,
      newContinuation: false,
      effects: List(FeeAmount(2), EmitQuote(43)),
      sourceSteps: List(8, 9, 10, 11),
      errorSteps: List(),
    },
    attemptedTotal: 42,
  }

  run reserveSharedProjectionTraceTest =
    initCase(Reserve)
      .expect(all {
        initialFrame(Reserve) == expectedInitial,
        applyStart(initialFrame(Reserve)) == expectedPrefix,
        observation == expected0,
      })
      .then(resume(Approval(41)))
      .expect(observation == expected1)
      .then(resume(Approval(43)))
      .expect(observation == expected2)
      .then(resume(Settlement(false)))
      .expect(observation == expected3)
      .then(resume(Settlement(true)))
      .expect(observation == expected4)
}
