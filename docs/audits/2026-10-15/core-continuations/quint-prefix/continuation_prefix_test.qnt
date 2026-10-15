// Normal numeric examples only. UNEXECUTED; expectations are frozen oracle rows.
module continuation_prefix_test {
  import continuation_prefix.* from "./continuation_prefix"

  run escrowStartTest =
    initCase(Escrow)
      .expect(all { not(boundary.dutyPresent), boundary.dutyAmount == 0 })
      .then(start)
      .expect(all {
        boundary.balances == Map("alice" -> 90, "bob" -> 0, "escrow" -> 10, "observer" -> 42),
        boundary.programState == Map("marker" -> 77),
        boundary.dutyPresent,
        boundary.dutyAmount == 10,
        boundary.pendingSite == 1,
        boundary.captureValues == List(10),
        boundary.logicalAttempted == 5,
        boundary.logicalCommitted == 5,
        boundary.effects == List(Transfer(10), LiabilityChange({ before: 0, after: 10 })),
      })

  run reserveStartTest =
    initCase(Reserve)
      .expect(all { boundary.dutyPresent, boundary.dutyAmount == 5 })
      .then(start)
      .expect(all {
        boundary.balances == Map("observer" -> 42, "reserve" -> 100, "target" -> 0, "treasury" -> 0),
        boundary.programState == Map("allocated" -> 0, "basis" -> 35, "marker" -> 77),
        boundary.dutyPresent,
        boundary.dutyAmount == 47,
        boundary.pendingSite == 1,
        boundary.captureValues == List(2, 42),
        boundary.logicalAttempted == 11,
        boundary.logicalCommitted == 11,
        boundary.effects == List(
          StateChange({ before: 40, after: 35 }),
          LiabilityChange({ before: 5, after: 47 })
        ),
      })
}
