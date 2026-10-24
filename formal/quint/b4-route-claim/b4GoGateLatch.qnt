module b4GoGateLatch {
  // Source-only finite counterpart of B4 14c7b754 guard/owned latches, wrapper 845aee76 goTick/gate/goGate and the
  // submit c33bea33 CALL_GO_GATE await before exposed=true. Not an event-loop, Promise or JS refinement.
  // P1 (bounded scheduling premise, unproven): a guard catch queued before the goTick setImmediate callback runs first.
  // P2: published, deadline, inputs and finalTyped are opaque booleans. At most one deferred raise per run.
  // Exposed is source reachability of the exposure point only: no proof, acceptance or send.
  type St = Routed | Returned | Linked | Failed
  type Err = NoErr | Codec | Reentry | Sealed
  type Cl = NoClean | CodecFree | SealedFree
  type G = GIdle | Ticking | GTrue | GThrow
  type C = Prep | Await | Exposed | Refused
  type B = { st: St, first: Err, cl: Cl, pend: Err, raised: bool }
  type W = { pub: bool, wf: bool, g: G, c: C, chk: bool }
  var b: B
  var w: W

  // Safety expectations first. chk: a deferred raise had occurred when the callback reread.
  val exposedOnlyAfterTrue = w.c == Exposed implies w.g == GTrue
  val trueSawEveryEarlierRaise = w.g == GTrue implies not(w.chk)
  val refusalRecorded = w.g == GThrow implies w.wf
  val latchPair = (b.st == Failed) iff (b.first != NoErr)
  val cleanupAtRaise = b.cl != NoClean implies (b.st == Failed or b.pend == Codec)
  val safety = and { exposedOnlyAfterTrue, trueSawEveryEarlierRaise, refusalRecorded, latchPair, cleanupAtRaise }

  action initWith(p: bool): bool = all {
    b' = { st: Routed, first: NoErr, cl: NoClean, pend: NoErr, raised: false },
    w' = { pub: p, wf: false, g: GIdle, c: Prep, chk: false },
  }
  action init = {
    nondet p = Set(true, false).oneOf()
    initWith(p)
  }
  // Disabled observation frame for the two .fail() probes below: this remains false, so it adds no
  // enabled model transition. Boolean 'and' keeps the full speculative frame when returning false;
  // a surrounding actionAll would roll it back. .fail() can then observe refusal in one trace.
  action blockedProbeFrame = all { b' = b, w' = w } and false
  // prove passes parse/codec/seen and sets 'returned'.
  action proveReturn = if (b.st == Routed and b.pend == NoErr)
    all { b' = { ...b, st: Returned }, w' = w }
    else blockedProbeFrame
  // owned(): cleanup latched at the throw; guard's catch ('failed', first) is still a pending microtask.
  action codecRaise = all { b.st == Routed, not(b.raised), b' = { ...b, cl: CodecFree, pend: Codec, raised: true }, w' = w }
  // sync observeSealed: linked, or refusal with first/'failed'/optional SEALED_PROOF_FREE in one step.
  action observeSealed = all { b.st == Returned, b' = { ...b, st: Linked }, w' = w }
  action observeFail(free: bool): bool = all {
    b.st == Returned, w' = w,
    b' = { ...b, st: Failed, first: Sealed, cl: if (free) SealedFree else b.cl },
  }
  // A later async B4 entry (prove LINK_REUSED, claimed route.prove) throws in its async prefix.
  action lateRaise = all { b.st == Linked, not(b.raised), b' = { ...b, pend: Reentry, raised: true }, w' = w }
  // guard catch -> failed(): 'failed', first kept.
  action guardLatch = all {
    b.pend != NoErr, w' = w,
    b' = { ...b, st: Failed, first: if (b.first == NoErr) b.pend else b.first, pend: NoErr },
  }
  // submit CALL_GO_GATE: await goGate(); goGate suspends on goTick().
  action callGate = all { w.c == Prep, w.g == GIdle, b' = b, w' = { ...w, c: Await, g: Ticking } }
  // goGate after goTick: s.published and gate() rereads; ok abstracts left/assertInputs/finalTyped; true or fail(GO_GATE).
  val gatePass = w.pub and not(w.wf) and b.st == Linked and b.first == NoErr and b.cl == NoClean
  action check(ok: bool): bool = all {
    w.g == Ticking, b' = b,
    w' = { ...w, g: if (gatePass and ok) GTrue else GThrow, wf: w.wf or not(gatePass and ok), chk: b.raised },
  }
  action tickCheck(ok: bool): bool = if (b.pend == NoErr) check(ok) else blockedProbeFrame
  // submit resumes: strict true, then deadline/TTL (post), then exposed=true; anything else propagates unexposed.
  action resume(post: bool): bool = all {
    w.c == Await, w.g == GTrue or w.g == GThrow, b' = b,
    w' = { ...w, c: if (w.g == GTrue and post) Exposed else Refused },
  }
  action step = {
    nondet x = Set(true, false).oneOf()
    any { proveReturn, codecRaise, observeSealed, observeFail(x), lateRaise, guardLatch, callGate, tickCheck(x), resume(x) }
  }

  // ExpectedBad weakenings: hypothetical, outside step, not an observed bug. W1 drops P1 (callback before the pending
  // guard catch, e.g. no full-turn await); W2 exposes without awaiting a strict true.
  action tickEarly(ok: bool): bool = check(ok)
  action exposeUnawaited = all { w.c == Await, b' = b, w' = { ...w, c: Exposed } }
}
