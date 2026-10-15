// UNEXECUTED normal invocation examples; literal expectations from the oracle.
module continuation_lifecycle_test {
  import continuation_lifecycle.* from "./continuation_lifecycle"

  pure val escrowFirst: PendingState = {
    specimen: Escrow,
    balances: Map("alice" -> 90, "bob" -> 0, "escrow" -> 10, "observer" -> 42),
    programState: Map("marker" -> 77),
    dutyPresent: true, dutyAmount: 10,
    site: 1, captures: List(10), committed: 5,
  }
  pure val escrowDone: PendingState = {
    specimen: Escrow,
    balances: Map("alice" -> 90, "bob" -> 10, "escrow" -> 0, "observer" -> 42),
    programState: Map("marker" -> 77),
    dutyPresent: true, dutyAmount: 0,
    site: 0, captures: List(), committed: 11,
  }
  pure val reserveFirst: PendingState = {
    specimen: Reserve,
    balances: Map("observer" -> 42, "reserve" -> 100, "target" -> 0, "treasury" -> 0),
    programState: Map("allocated" -> 0, "basis" -> 35, "marker" -> 77),
    dutyPresent: true, dutyAmount: 47,
    site: 1, captures: List(2, 42), committed: 11,
  }
  pure val reserveSecond43: PendingState = {
    specimen: Reserve,
    balances: Map("observer" -> 42, "reserve" -> 58, "target" -> 42, "treasury" -> 0),
    programState: Map("allocated" -> 42, "basis" -> 78, "marker" -> 77),
    dutyPresent: true, dutyAmount: 5,
    site: 2, captures: List(2, 42, 43), committed: 26,
  }
  pure val reserveSecond51: PendingState = {
    specimen: Reserve,
    balances: Map("observer" -> 42, "reserve" -> 58, "target" -> 42, "treasury" -> 0),
    programState: Map("allocated" -> 42, "basis" -> 86, "marker" -> 77),
    dutyPresent: true, dutyAmount: 5,
    site: 2, captures: List(2, 42, 51), committed: 26,
  }
  pure val reserveDone: PendingState = {
    specimen: Reserve,
    balances: Map("observer" -> 42, "reserve" -> 56, "target" -> 42, "treasury" -> 2),
    programState: Map("allocated" -> 42, "basis" -> 78, "marker" -> 77),
    dutyPresent: true, dutyAmount: 5,
    site: 0, captures: List(), committed: 36,
  }

  run escrowFalseTest =
    initCase(Escrow)
      .expect(observation.c == escrowFirst)
      .then(resume(Delivery(false)))
      .expect(all {
        observation.c == escrowFirst,
        observation.last.status == Waiting,
        observation.last.attempted == 2,
        not(observation.last.advanced),
        not(observation.last.hasSegment),
        not(observation.last.newContinuation),
        observation.last.effects == List(),
        observation.last.sourceSteps == List(),
        observation.last.errorSteps == List(),
        observation.attemptedTotal == 7,
      })

  run escrowTrueTest =
    initCase(Escrow)
      .then(resume(Delivery(true)))
      .expect(all {
        observation.c == escrowDone,
        observation.last.status == Complete,
        observation.last.attempted == 6,
        observation.last.advanced,
        observation.last.hasSegment,
        not(observation.last.newContinuation),
        observation.last.effects == List(TransferAmount(10), DutyValue({ before: 10, after: 0 })),
        observation.last.sourceSteps == List(2, 3, 4),
        observation.last.errorSteps == List(),
        observation.attemptedTotal == 11,
      })

  run escrowFalseThenTrueTest =
    initCase(Escrow)
      .then(resume(Delivery(false)))
      .expect(observation.c == escrowFirst)
      .then(resume(Delivery(true)))
      .expect(all {
        observation.c == escrowDone,
        observation.last.attempted == 6,
        observation.attemptedTotal == 13,
      })

  run reserveFalseApprovalTest =
    initCase(Reserve)
      .expect(observation.c == reserveFirst)
      .then(resume(Approval(41)))
      .expect(all {
        observation.c == reserveFirst,
        observation.last.status == Waiting,
        observation.last.attempted == 4,
        not(observation.last.advanced),
        not(observation.last.hasSegment),
        not(observation.last.newContinuation),
        observation.last.effects == List(),
        observation.last.sourceSteps == List(),
        observation.last.errorSteps == List(),
        observation.attemptedTotal == 15,
      })

  run reserveTrueApprovalTest =
    initCase(Reserve)
      .then(resume(Approval(43)))
      .expect(all {
        observation.c == reserveSecond43,
        observation.last.status == Waiting,
        observation.last.attempted == 15,
        observation.last.advanced,
        observation.last.hasSegment,
        observation.last.newContinuation,
        observation.last.effects == List(
          StateValue({ name: "allocated", before: 0, after: 42 }),
          TransferAmount(42),
          DutyValue({ before: 47, after: 5 }),
          StateValue({ name: "basis", before: 35, after: 78 })
        ),
        observation.last.sourceSteps == List(3, 4, 5, 6, 7),
        observation.last.errorSteps == List(),
        observation.attemptedTotal == 26,
      })

  run reserveFalseSettlementTest =
    initCase(Reserve)
      .then(resume(Approval(43)))
      .then(resume(Settlement(false)))
      .expect(all {
        observation.c == reserveSecond43,
        observation.last.status == Waiting,
        observation.last.attempted == 2,
        not(observation.last.advanced),
        not(observation.last.hasSegment),
        not(observation.last.newContinuation),
        observation.last.effects == List(),
        observation.last.sourceSteps == List(),
        observation.last.errorSteps == List(),
        observation.attemptedTotal == 28,
      })

  run reserveTrueSettlementTest =
    initCase(Reserve)
      .then(resume(Approval(43)))
      .then(resume(Settlement(true)))
      .expect(all {
        observation.c == reserveDone,
        observation.last.status == Complete,
        observation.last.attempted == 10,
        observation.last.advanced,
        observation.last.hasSegment,
        not(observation.last.newContinuation),
        observation.last.effects == List(FeeAmount(2), EmitQuote(43)),
        observation.last.sourceSteps == List(8, 9, 10, 11),
        observation.last.errorSteps == List(),
        observation.attemptedTotal == 36,
      })

  run reserveFalseThenTrueTrajectoryTest =
    initCase(Reserve)
      .then(resume(Approval(41)))
      .expect(observation.c == reserveFirst)
      .then(resume(Approval(43)))
      .expect(observation.c == reserveSecond43)
      .then(resume(Settlement(false)))
      .expect(observation.c == reserveSecond43)
      .then(resume(Settlement(true)))
      .expect(all {
        observation.c == reserveDone,
        observation.last.attempted == 10,
        observation.attemptedTotal == 42,
      })

  run reserveBusinessRejectionTest =
    initCase(Reserve)
      .then(resume(Approval(51)))
      .expect(observation.c == reserveSecond51)
      .then(resume(Settlement(true)))
      .expect(all {
        observation.c == reserveSecond51,
        observation.last.status == BusinessRejected,
        observation.last.attempted == 6,
        not(observation.last.advanced),
        not(observation.last.hasSegment),
        not(observation.last.newContinuation),
        observation.last.effects == List(),
        observation.last.sourceSteps == List(),
        observation.last.errorSteps == List(9),
        observation.attemptedTotal == 32,
      })
}
