// N-4: the CONTINUATION kit (ARCHITECTURE "Kit: CONTINUATION") in mode 1, the retained-memory
// checkpoint, with the question of N-4: does the checkpoint domain tag have to be a circuit cell
// (option (b), an in-circuit continuation envelope) or only a word of the persisted record digest
// (option (a), no new circuit cell)?
//
// Objects:
//   - One genuine run of program PROG on input INP: NPROG machine steps to Halted, fuel F0. With
//     F0 < NPROG the uninterrupted run is Exhausted at h = F0 (a terminal state), so a resumed run
//     that refreshes its fuel would reach a different terminal state and a different claim.
//   - Machine states are their fields `{pc, inp, fuel}` (the height `h` is a SEG lane, not a state field) (BOUNDARY COMMITMENT digest assumed canonical
//     and injective, C3). A leaf proves ONE machine step from any state the prover holds a witness
//     for (genuine or tampered) at a witnessed height `h_pre`; the leaf relation is the transition
//     `step` (CORE assumed, C3) plus INIT's genesis gate (`h_pre == 0` implies the genesis state).
//   - Proven SEG statements `segs` (leaf and MERGE proofs; NODE-L1 assumed). MERGE is a circuit
//     relation: `a.hi == b.lo` (interval) and `a.post == b.pre` (stitch). SEAL is a circuit
//     relation: `lo == 0`, the post state terminal, and the genesis state at h = 0 (INIT's gate in
//     the genesis leaf). SEAL emits the SET root `LEAF(claim_d, n_steps)`.
//   - The tier 1 scheduler (host): a carry stack of proven SEGs by rank; workers prove leaves in
//     any order; the scheduler pushes the leaf at its height and merges equal ranks.
//   - Mode 1 checkpoint records (host artifacts): `{dig, prog, inp, h, post, stack}`. The record
//     digest is a hash with the checkpoint tag CKPT (R-7, option (a)). Mutant `recordTagRun` gives
//     it the run claim tag RUN instead (no domain separation of the record).
//   - The host is the adversary: it may resume from a tampered record (fuel refreshed, input
//     changed, a predecessor skipped, the height reset), merge any two proven SEGs, seal any proven
//     SEG, and offer any digest it holds (claims, SET roots, record digests) to a consumer.
//     Consumers (`L` and the GP-2 DL-STEP) accept only a verified SET root that opens as a LEAF.
//
// What the model decides for N-4: with NO checkpoint tag in any circuit relation, every attack
// that offers a checkpoint (record or prefix SEG) as a claim is refused by SEAL or by the GP-2
// consumer (`CheckpointRejectedBySealAndGP2`), even when the record digest itself carries the RUN
// tag (mutant `recordTagRun`). The tag is needed on the record digest only, so that no persisted
// record can parse as a claim in the host's claim namespace (`RecordNeverParsesAsClaim`, broken by
// `recordTagRun`). That is option (a).

module contTypes {
  type State = { pc: int, inp: int, fuel: int }
  type Seg = { lo: int, hi: int, pre: State, post: State, rank: int }
  // Digests are their preimages (an injective hash by construction). A claim is
  // RUN(prog, inp, outcome); a record digest is CKPT(prog, inp, code) (or RUN under recordTagRun).
  type Dig = { tag: str, a: int, b: int, c: int }
  type SetRoot = { tag: str, claim: Dig, n: int }
  type CkptRec = { dig: Dig, h: int, post: State, stack: List[Seg] }
  type Sched = { live: bool, h: int, cur: State, stack: List[Seg] }
  // Mutant flags: each drops one circuit or protocol check.
  type Mut = {
    noStitch: bool,      // MERGE skips a.post == b.pre        (cont_refresh_fuel, cont_change_input)
    noInterval: bool,    // MERGE skips a.hi == b.lo           (cont_skip_predecessor)
    noGenesis: bool,     // the leaf skips INIT's gate at h_pre = 0 (cont_reset_height)
    noTerminal: bool,    // SEAL skips the terminal check      (cont_publish_as_run)
    consumeAnyRun: bool, // the consumer accepts any digest with the RUN tag (cont_ckpt_as_gp2_claim)
    recordTagRun: bool,  // the record digest uses the RUN tag (no checkpoint domain tag at all)
    pauseTerminal: bool, // pause allowed at a terminal state (Exhausted as a checkpoint trigger)
  }
  pure val NOMUT: Mut = { noStitch: false, noInterval: false, noGenesis: false, noTerminal: false,
                          consumeAnyRun: false, recordTagRun: false, pauseTerminal: false }
}

module continuation {
  import contTypes.*
  const MUT: Mut

  // ---------------- the genuine run ----------------
  pure val PROG = 7
  pure val INP = 5
  pure val INP_OTHER = 6
  pure val NPROG = 7          // steps to Halted
  pure val F0 = 6             // fuel: the uninterrupted run is Exhausted at h = 6
  pure val HALTED = 1
  pure val EXHAUSTED = 3
  pure val LOG2L = 3          // ceil(log2 6): the uninterrupted run has 6 leaves
  pure val HMAX = 9           // heights a forged SEG may claim
  pure val GEN: State = { pc: 0, inp: INP, fuel: F0 }

  pure def terminal(s: State): bool = s.pc >= NPROG or s.fuel <= 0
  pure def next1(s: State): State = { pc: s.pc + 1, inp: s.inp, fuel: s.fuel - 1 }
  pure val RUNNING = 4
  pure def outcomeOf(s: State): int =
    if (s.pc >= NPROG) HALTED * 100 + s.inp else if (s.fuel <= 0) EXHAUSTED * 100 + s.inp else RUNNING * 100 + s.inp
  pure def exec(s: State, k: int): State = { pc: s.pc + k, inp: s.inp, fuel: s.fuel - k }
  pure def genuine(h: int): State = exec(GEN, h)
  pure val TRUE_N = F0
  pure val TRUE_CLAIM: Dig = { tag: "RUN", a: PROG, b: INP, c: outcomeOf(genuine(F0)) }

  // ---------------- state ----------------
  var segs: Set[Seg]           // proven SEG statements (leaf and MERGE proofs)
  var sets: Set[SetRoot]       // verified SET roots (SEAL outputs)
  var accepted: Set[Dig]       // claims a consumer accepted (`L` or a GP-2 DL-STEP)
  var recs: Set[CkptRec]           // persisted checkpoint records
  var held: Set[Dig]           // every digest the host holds and may offer as a claim
  var sched: Sched             // the tier 1 scheduler (carry stack); honest unless resumed from a tampered record
  var maxDepth: int            // ghost: the largest honest carry stack seen
  var pauses: int              // ghost: pauses so far (bounded)
  var adminOk: bool            // ghost: no pause or honest resume changed h, the state or the fuel

  // ---------------- circuit relations ----------------
  pure def leafOk(s: State, h: int): bool = not(terminal(s)) and (MUT.noGenesis or h != 0 or s == GEN)
  pure def leafSeg(s: State, h: int): Seg = { lo: h, hi: h + 1, pre: s, post: next1(s), rank: 0 }
  pure def mergeOk(a: Seg, b: Seg): bool = (MUT.noInterval or a.hi == b.lo) and (MUT.noStitch or a.post == b.pre)
  pure def merged(a: Seg, b: Seg): Seg = { lo: a.lo, hi: b.hi, pre: a.pre, post: b.post, rank: a.rank + 1 }
  pure def sealOk(s: Seg): bool = s.lo == 0 and (MUT.noTerminal or terminal(s.post))
  pure def claimOf(s: Seg): Dig = { tag: "RUN", a: PROG, b: s.post.inp, c: outcomeOf(s.post) }
  pure def leafRoot(s: Seg): SetRoot = { tag: "LEAF", claim: claimOf(s), n: s.hi }
  pure def recDig(h: int, post: State): Dig =
    if (MUT.recordTagRun) { tag: "RUN", a: PROG, b: post.inp, c: 1000 + h * 10 + post.fuel }
    else { tag: "CKPT", a: PROG, b: post.inp, c: 1000 + h * 10 + post.fuel }
  // a consumer accepts a claim only as the head of a verified LEAF SET root
  pure def consumerOk(d: Dig, ss: Set[SetRoot]): bool =
    ss.exists(r => r.tag == "LEAF" and r.claim == d) or (MUT.consumeAnyRun and d.tag == "RUN")

  // carry-stack fold: merge the top two while their ranks are equal; each merge is a MERGE proof,
  // so a fold that needs a refused MERGE cannot happen
  pure def mergeTop(x: { st: List[Seg], ok: bool }): { st: List[Seg], ok: bool } =
    val st = x.st
    val n = st.length()
    if (n >= 2 and st[n - 1].rank == st[n - 2].rank)
      { st: st.slice(0, n - 2).append(merged(st[n - 2], st[n - 1])), ok: x.ok and mergeOk(st[n - 2], st[n - 1]) }
    else x
  pure def foldAll(st: List[Seg]): { st: List[Seg], ok: bool } = mergeTop(mergeTop(mergeTop(mergeTop({ st: st, ok: true }))))
  pure def segsOf(st: List[Seg]): Set[Seg] = st.foldl(Set(), (a, x) => a.union(Set(x)))

  // ---------------- actions ----------------
  action init = all {
    segs' = Set(), sets' = Set(), accepted' = Set(), recs' = Set(), held' = Set(TRUE_CLAIM),
    sched' = { live: true, h: 0, cur: GEN, stack: List() }, maxDepth' = 0, pauses' = 0, adminOk' = true,
  }

  // a worker proves the leaf at any genuine height (out-of-order completion)
  action proveGenuineAt(h: int): bool = all {
    0 <= h, h < F0,
    segs' = segs.union(Set(leafSeg(genuine(h), h))),
    sets' = sets, accepted' = accepted, recs' = recs, held' = held, sched' = sched,
    maxDepth' = maxDepth, pauses' = pauses, adminOk' = adminOk,
  }

  // the adversary proves a leaf from a state of a record, tampered or not, after k more steps,
  // at any claimed height
  action proveForgedAt(r: CkptRec, t: str, k: int, h: int): bool = {
    val s0 = if (t == "fuel") { ...r.post, fuel: F0 } else if (t == "input") { ...r.post, inp: INP_OTHER } else r.post
    val s = exec(s0, k)
    all {
      recs.contains(r), leafOk(s, h),
      segs' = segs.union(Set(leafSeg(s, h))),
      sets' = sets, accepted' = accepted, recs' = recs, held' = held, sched' = sched,
      maxDepth' = maxDepth, pauses' = pauses, adminOk' = adminOk,
    }
  }

  // the scheduler pushes the proven leaf at its height and state, then folds
  action push = all {
    sched.live, segs.contains(leafSeg(sched.cur, sched.h)),
    val f = foldAll(sched.stack.append(leafSeg(sched.cur, sched.h)))
    all {
      f.ok,
      sched' = { ...sched, h: sched.h + 1, cur: next1(sched.cur), stack: f.st },
      segs' = segs.union(segsOf(f.st)),
      maxDepth' = if (f.st.length() > maxDepth) f.st.length() else maxDepth,
    },
    sets' = sets, accepted' = accepted, recs' = recs, held' = held, pauses' = pauses, adminOk' = adminOk,
  }

  // MERGE of any two proven SEGs (the adversary picks; the circuit checks)
  action mergeOf(a: Seg, b: Seg): bool = all {
    segs.contains(a), segs.contains(b),
    mergeOk(a, b), b.hi <= HMAX,
    segs' = segs.union(Set(merged(a, b))),
    sets' = sets, accepted' = accepted, recs' = recs, held' = held, sched' = sched,
    maxDepth' = maxDepth, pauses' = pauses, adminOk' = adminOk,
  }

  // SEAL of any proven SEG
  action sealOf(s: Seg): bool = all {
    segs.contains(s),
    sealOk(s),
    sets' = sets.union(Set(leafRoot(s))),
    held' = held.union(Set(claimOf(s))),
    segs' = segs, accepted' = accepted, recs' = recs, sched' = sched,
    maxDepth' = maxDepth, pauses' = pauses, adminOk' = adminOk,
  }

  // pause: persist a mode 1 record of the running prefix; the in-memory scheduler is lost
  action pause = all {
    sched.live, sched.h > 0, pauses < 3, MUT.pauseTerminal or not(terminal(sched.cur)),
    val r = { dig: recDig(sched.h, sched.cur), h: sched.h, post: sched.cur, stack: sched.stack }
    all { recs' = recs.union(Set(r)), held' = held.union(Set(r.dig)) },
    sched' = { live: false, h: 0, cur: GEN, stack: List() },
    pauses' = pauses + 1,
    segs' = segs, sets' = sets, accepted' = accepted, maxDepth' = maxDepth,
    adminOk' = adminOk and sched.cur == genuine(sched.h),
  }

  // honest resume: the scheduler is seeded at h with the record's carry stack and post state
  action resumeFrom(r: CkptRec): bool = all {
    recs.contains(r),
    not(sched.live),
    sched' = { live: true, h: r.h, cur: r.post, stack: r.stack },
    adminOk' = adminOk and r.post == genuine(r.h) and r.post.fuel == F0 - r.h,
    segs' = segs, sets' = sets, accepted' = accepted, recs' = recs, held' = held,
    maxDepth' = maxDepth, pauses' = pauses,
  }

  // a consumer (`L` or a GP-2 DL-STEP) offered any digest the host holds
  action consumeOf(d: Dig): bool = all {
    held.contains(d),
    consumerOk(d, sets),
    accepted' = accepted.union(Set(d)),
    segs' = segs, sets' = sets, recs' = recs, held' = held, sched' = sched,
    maxDepth' = maxDepth, pauses' = pauses, adminOk' = adminOk,
  }

  action proveGenuine = nondet h = 0.to(F0 - 1).oneOf() proveGenuineAt(h)
  action proveForged = nondet r = recs.oneOf() nondet t = Set("none", "fuel", "input").oneOf()
                       nondet k = 0.to(NPROG).oneOf() nondet h = 0.to(HMAX).oneOf() proveForgedAt(r, t, k, h)
  action mergeAny = nondet a = segs.oneOf() nondet b = segs.oneOf() mergeOf(a, b)
  action sealAny = nondet x = segs.oneOf() sealOf(x)
  action resume = nondet r = recs.oneOf() resumeFrom(r)
  action consume = nondet d = held.oneOf() consumeOf(d)
  action step = any { proveGenuine, proveForged, push, mergeAny, sealAny, pause, resume, consume }

  // ---------------- helpers for the deterministic tests ----------------
  pure def gseg(lo: int, hi: int): Seg = { lo: lo, hi: hi, pre: genuine(lo), post: genuine(hi), rank: 0 }
  def top(i: int): Seg = sched.stack[i]
  def recAt(h: int): CkptRec = recs.filter(r => r.h == h).fold({ dig: { tag: "", a: 0, b: 0, c: 0 }, h: -1, post: GEN, stack: List() }, (a, r) => r)

  // ---------------- invariants (kit names) ----------------
  // every sealed claim and its n_steps equal the uninterrupted run's, whatever the pause cuts
  val ResumeProducesIdenticalClaimD = sets.forall(r => r.claim == TRUE_CLAIM and r.n == TRUE_N)
  // every proven SEG that starts at height 0 is an exact execution of its interval from genesis:
  // out-of-order completion, pauses and merges keep the interval (NODE-L1)
  val OutOfOrderWorkerCompletionPreservesSegInterval =
    segs.forall(s => s.lo == 0 implies (s.pre == GEN and s.post == genuine(s.hi)))
  // no record digest and no prefix SEG is ever accepted as a claim: only the sealed run claim
  val CheckpointRejectedBySealAndGP2 =
    accepted.forall(d => d == TRUE_CLAIM) and recs.forall(r => not(accepted.contains(r.dig)))
  // the honest carry stack stays within ceil(log2 L) + 2
  val MaxStackDepthLeLogLPlus2 = maxDepth <= LOG2L + 2
  // pause and resume execute no machine event: h, the state and the fuel are unchanged
  val AdminStepsKeptApart = adminOk
  // N-4 (a): a persisted record never parses as a claim (the record domain tag)
  val RecordNeverParsesAsClaim = recs.forall(r => r.dig.tag != "RUN")
  // Exhausted (any terminal state) is never a checkpoint trigger
  val NoTerminalCheckpoint = recs.forall(r => not(terminal(r.post)))

  // what an in-circuit tag (option (b)) would have to protect: it holds with no tag in any circuit
  val circuitSide = ResumeProducesIdenticalClaimD and OutOfOrderWorkerCompletionPreservesSegInterval
    and CheckpointRejectedBySealAndGP2
  val soundAll = circuitSide and MaxStackDepthLeLogLPlus2 and AdminStepsKeptApart
    and RecordNeverParsesAsClaim and NoTerminalCheckpoint

  // reach witnesses (negated: violated in the sound instance)
  val reachSealed = sets.size() == 0
  val reachSealedAfterResume = not(pauses > 0 and sets.size() > 0)
  val reachConsumed = accepted.size() == 0
}

module cont_sound { import contTypes.* from "./continuation" import continuation(MUT = NOMUT).* from "./continuation" }
module cont_noStitch { import contTypes.* from "./continuation" import continuation(MUT = { ...NOMUT, noStitch: true }).* from "./continuation" }
module cont_noInterval { import contTypes.* from "./continuation" import continuation(MUT = { ...NOMUT, noInterval: true }).* from "./continuation" }
module cont_noGenesis { import contTypes.* from "./continuation" import continuation(MUT = { ...NOMUT, noGenesis: true }).* from "./continuation" }
module cont_noTerminal { import contTypes.* from "./continuation" import continuation(MUT = { ...NOMUT, noTerminal: true }).* from "./continuation" }
module cont_consumeAnyRun { import contTypes.* from "./continuation" import continuation(MUT = { ...NOMUT, consumeAnyRun: true }).* from "./continuation" }
module cont_recordTagRun { import contTypes.* from "./continuation" import continuation(MUT = { ...NOMUT, recordTagRun: true }).* from "./continuation" }
module cont_anyRunUntagged { import contTypes.* from "./continuation" import continuation(MUT = { ...NOMUT, consumeAnyRun: true, recordTagRun: true }).* from "./continuation" }
module cont_pauseTerminal { import contTypes.* from "./continuation" import continuation(MUT = { ...NOMUT, pauseTerminal: true }).* from "./continuation" }
