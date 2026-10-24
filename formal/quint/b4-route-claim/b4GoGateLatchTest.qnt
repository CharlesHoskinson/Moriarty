module b4GoGateLatchTest {
  import b4GoGateLatch.* from "./b4GoGateLatch"

  run toLinked = initWith(true).then(proveReturn).then(observeSealed).expect(b.st == Linked and safety)

  // Clean control: no raise; gate true after the full turn; exposure point reached.
  run cleanControlTest = toLinked.then(callGate).then(tickCheck(true)).then(resume(true))
    .expect(w.c == Exposed and w.g == GTrue and not(w.chk) and safety)

  // Raise while goGate is suspended: still 'linked' until the latch; callback waits (P1); gate refuses; unexposed.
  run lateRaiseRefusedTest = toLinked.then(callGate).then(lateRaise)
    .expect(b.st == Linked and b.first == NoErr and b.pend == Reentry)
    .then(tickCheck(true).fail())
    .expect(b == { st: Linked, first: NoErr, cl: NoClean, pend: Reentry, raised: true }
      and w == { pub: true, wf: false, g: Ticking, c: Await, chk: false } and safety)
    .then(guardLatch).then(tickCheck(true)).then(resume(true))
    .expect(w.g == GThrow and w.wf and w.c == Refused and b.first == Reentry and safety)

  // Codec site: cleanup latched at the throw while first is null; prove never returns; gate refuses.
  run codecCleanupBeforeFirstTest = initWith(true).then(codecRaise)
    .expect(b.cl == CodecFree and b.first == NoErr and b.st == Routed and safety)
    .then(proveReturn.fail())
    .expect(b == { st: Routed, first: NoErr, cl: CodecFree, pend: Codec, raised: true }
      and w == { pub: true, wf: false, g: GIdle, c: Prep, chk: false } and safety)
    .then(guardLatch).then(callGate).then(tickCheck(true)).then(resume(true))
    .expect(b.first == Codec and b.cl == CodecFree and w.c == Refused and safety)

  // Sync sealed refusal with free failure: both latches in one step.
  run sealedSyncTest = initWith(true).then(proveReturn).then(observeFail(true))
    .expect(b.st == Failed and b.first == Sealed and b.cl == SealedFree and safety)
    .then(callGate).then(tickCheck(true)).then(resume(true)).expect(w.c == Refused and safety)

  // Unpublished, or a post-await deadline/TTL refusal: unexposed.
  run unpublishedTest = initWith(false).then(proveReturn).then(observeSealed).then(callGate).then(tickCheck(true))
    .then(resume(true)).expect(w.g == GThrow and w.c == Refused and safety)
  run postDeadlineTest = toLinked.then(callGate).then(tickCheck(true)).then(resume(false))
    .expect(w.g == GTrue and w.c == Refused and safety)

  // Residual window, not a refusal claim: a raise after the reread is unseen; the gate is point-in-time.
  run postCheckRaiseTest = toLinked.then(callGate).then(tickCheck(true)).then(lateRaise).then(guardLatch).then(resume(true))
    .expect(w.c == Exposed and b.st == Failed and not(w.chk) and safety)

  // ExpectedBad W1 (witness reached, not a safety pass): callback before the pending catch returns true over a raise.
  run expectedBadEarlyTickTest = toLinked.then(callGate).then(lateRaise).then(tickEarly(true)).then(resume(true))
    .expect(w.c == Exposed and w.chk and not(trueSawEveryEarlierRaise))
    .then(guardLatch).expect(b.st == Failed and w.c == Exposed)

  // ExpectedBad W2 (witness reached, not a safety pass): exposure while goGate is still suspended.
  run expectedBadUnawaitedTest = toLinked.then(callGate).then(lateRaise).then(exposeUnawaited)
    .expect(w.g == Ticking and not(exposedOnlyAfterTrue))
}
