// Design check for RPP-003/004/005. One proof, integer milliseconds and RSS MB.
// Intervals are half-open; diagnostics may overlap but earn no exclusive credit.
// CPU values abstract all-worker core-ms. Statistics are supplied, not estimated.
// No command reconciliation, concurrent proofs, allocator or prover is simulated.
module profile {
  type OptionalValue = Some(int) | None(str)
  type Counter = { supported: bool, value: OptionalValue }
  type Quality = BoundaryOnly | SampledLowerBound
  type Sample = { at: int, rss: int }
  type Memory = {
    entry: int, exit: int, samples: Set[Sample], sampledPeak: int,
    peakDelta: int, quality: Quality, certifiesInstantaneous: bool,
  }
  type Phase = {
    id: int, owner: str, start: int, end: int, exclusive: bool,
    cpu: Counter, memory: Memory,
  }
  type Verdict = Passed | Inconclusive | Failed
  // Basis points: 100 bp = 1%. Upper bound is one-sided 95%, for one stratum.
  type Overhead = { pairs: int, meanBp: int, upper95Bp: int, verdict: Verdict }
  type Outcome = ProofAccepted | ProofRejected
  type State = {
    wallStart: int, wallEnd: int, phases: Set[Phase], unexplained: int,
    closed: bool, coverageValid: bool, counter: Counter, overhead: Overhead,
    events: int, dropped: int, invalid: bool,
    unprofiledOutcome: Outcome, proofOutcome: Outcome,
  }
  var s: State

  pure val CAPACITY = 5
  pure val OWNERS = Set("quotient", "opening", "worker")
  pure val UNSUPPORTED: Counter = { supported: false, value: None("unsupported") }
  pure def maxInt(a: int, b: int): int = if (a > b) a else b
  pure def absInt(a: int): int = if (a < 0) -a else a
  pure def total(st: State): int = st.wallEnd - st.wallStart
  pure def exclusiveSum(st: State): int =
    st.phases.filter(p => p.exclusive).fold(0, (sum, p) => sum + p.end - p.start)

  // R1 / RPP-003: one owner per interval identity, including diagnostics.
  // A scalar owner plus unique IDs forbids two records claiming one interval.
  pure def ownershipOk(st: State): bool = and {
    total(st) > 0,
    st.phases.forall(p => OWNERS.contains(p.owner)
      and st.wallStart <= p.start and p.start < p.end and p.end <= st.wallEnd),
    st.phases.forall(p => st.phases.forall(q => p == q or
      (p.id != q.id and (not(p.exclusive and q.exclusive)
        or p.end <= q.start or q.end <= p.start)))),
  }
  // R2 / RPP-003: unexplained is an explicit stored bucket, not a second sum.
  pure def reconciliationOk(st: State): bool =
    st.unexplained >= 0 and exclusiveSum(st) + st.unexplained == total(st)
  // R3 / RPP-003: no percentage truncation; test discrepancy independently
  // even though a correctly reconciled closed profile has zero discrepancy.
  pure def coverageBounds(st: State): bool = and {
    total(st) > 0, st.unexplained >= 0,
    100 * absInt(exclusiveSum(st) + st.unexplained - total(st)) <= 5 * total(st),
    100 * st.unexplained <= 5 * total(st),
  }
  pure def eligible(st: State): bool = st.closed and not(st.invalid)
    and ownershipOk(st) and reconciliationOk(st) and coverageBounds(st)
  pure def coverageOk(st: State): bool = st.coverageValid == eligible(st)

  pure def sampledPeak(m: Memory): int =
    m.samples.fold(maxInt(m.entry, m.exit), (peak, sample) => maxInt(peak, sample.rss))
  pure def peakDelta(entry: int, peak: int): int = maxInt(0, peak - entry)
  pure def refreshMemory(m: Memory): Memory = { ...m,
    sampledPeak: sampledPeak(m), peakDelta: peakDelta(m.entry, sampledPeak(m)),
    quality: if (m.samples.size() < 2) BoundaryOnly else SampledLowerBound,
    certifiesInstantaneous: false,
  }
  // R4 / RPP-004: strictly interior samples count toward sampling quality.
  // Even a dense sampled lower bound does not certify an instantaneous peak.
  pure def memoryOk(p: Phase): bool = and {
    p.memory.entry >= 0, p.memory.exit >= 0,
    p.memory.samples.forall(x => p.start < x.at and x.at < p.end and x.rss >= 0),
    p.memory.samples.forall(x => p.memory.samples.forall(y => x == y or x.at != y.at)),
    p.memory.sampledPeak == sampledPeak(p.memory),
    p.memory.peakDelta == peakDelta(p.memory.entry, p.memory.sampledPeak),
    p.memory.quality == (if (p.memory.samples.size() < 2) BoundaryOnly else SampledLowerBound),
    not(p.memory.certifiesInstantaneous),
  }
  pure def peaksOk(st: State): bool = st.phases.forall(p => memoryOk(p))
  // R5 / RPP-004: None carries the absence reason; measured zero is distinct.
  pure def counterOk(c: Counter): bool = match c.value {
    | None(reason) => not(c.supported) and reason != ""
    | Some(n) => c.supported and n >= 0
  }
  pure def countersOk(st: State): bool = counterOk(st.counter)
    and st.phases.forall(p => counterOk(p.cpu))
  pure def qualifies(o: Overhead): bool = o.pairs >= 5 and o.upper95Bp <= 100
  pure def qualify(o: Overhead): Overhead =
    { ...o, verdict: if (qualifies(o)) Passed else Inconclusive }
  // R6 / RPP-005: the mean never licenses a pass. Both non-pass labels allowed.
  pure def overheadOk(st: State): bool = and {
    st.overhead.pairs >= 0, st.overhead.meanBp <= st.overhead.upper95Bp,
    (st.overhead.verdict == Passed) iff qualifies(st.overhead),
  }
  // R7 / RPP-005: invalidate at capacity (conservative full-buffer policy).
  // Subsequent attempted events are dropped; observer actions preserve outcome.
  pure def bufferOk(st: State): bool = and {
    0 <= st.events, st.events <= CAPACITY, st.dropped >= 0,
    (st.events == CAPACITY or st.dropped > 0) implies st.invalid,
    st.dropped > 0 implies st.events == CAPACITY,
    st.invalid implies not(st.coverageValid),
    st.proofOutcome == st.unprofiledOutcome,
  }
  pure def recordEvent(st: State): State = if (st.events < CAPACITY)
    { ...st, events: st.events + 1, invalid: st.invalid or st.events + 1 == CAPACITY }
    else { ...st, dropped: st.dropped + 1, invalid: true }

  pure def phase(id: int, finish: int, cpu: Counter): Phase = {
    id: id, owner: if (id == 0) "quotient" else if (id == 1) "opening" else "worker",
    start: if (id == 1) 50 else 0, end: finish, exclusive: id != 2, cpu: cpu,
    memory: { entry: 10, exit: 8, samples: Set(), sampledPeak: 10,
      peakDelta: 0, quality: BoundaryOnly, certifiesInstantaneous: false },
  }
  pure val PHASES = Set(phase(0, 50, UNSUPPORTED),
    phase(1, 94, UNSUPPORTED), phase(1, 95, UNSUPPORTED),
    phase(1, 100, { supported: true, value: Some(20) }),
    phase(2, 100, UNSUPPORTED))
  pure val SAMPLES = Set({ at: 1, rss: 8 }, { at: 2, rss: 12 },
    { at: 51, rss: 8 }, { at: 52, rss: 12 })
  pure def initial(outcome: Outcome, pairs: int, upper: int): State = {
    wallStart: 0, wallEnd: 100, phases: Set(), unexplained: 100,
    closed: false, coverageValid: false, counter: UNSUPPORTED,
    overhead: qualify({ pairs: pairs, meanBp: 50, upper95Bp: upper, verdict: Inconclusive }),
    events: 0, dropped: 0, invalid: false,
    unprofiledOutcome: outcome, proofOutcome: outcome,
  }
  action initWith(outcome: Outcome, pairs: int, upper: int): bool = all {
    Set(4, 5, 6).contains(pairs), Set(50, 100, 150).contains(upper),
    s' = initial(outcome, pairs, upper),
  }
  action init = {
    nondet outcome = Set(ProofAccepted, ProofRejected).oneOf()
    nondet pairs = Set(4, 5, 6).oneOf()
    nondet upper = Set(50, 100, 150).oneOf()
    initWith(outcome, pairs, upper)
  }
  pure def canAdd(st: State, p: Phase): bool = not(st.closed)
    and not(st.phases.exists(q => p.id == q.id))
    and ownershipOk({ ...st, phases: st.phases.union(Set(p)) })
    and memoryOk(p) and counterOk(p.cpu)
  pure def applyAdd(st: State, p: Phase): State = {
    // Logical observations remain available to this design check after a drop;
    // events counts only retained buffer slots. Such profiles cannot qualify.
    val recorded = recordEvent(st)
    { ...recorded, phases: st.phases.union(Set(p)),
      unexplained: st.unexplained - (if (p.exclusive) p.end - p.start else 0) }
  }
  action addPhase(p: Phase): bool = all { canAdd(s, p), s' = applyAdd(s, p) }

  pure def canSample(st: State, p: Phase, sample: Sample): bool = and {
    not(st.closed), st.phases.contains(p), p.start < sample.at, sample.at < p.end,
    sample.rss >= 0, not(p.memory.samples.exists(x => x.at == sample.at)),
  }
  pure def applySample(st: State, p: Phase, sample: Sample): State = {
    val updated = { ...p, memory: refreshMemory({ ...p.memory,
      samples: p.memory.samples.union(Set(sample)) }) }
    val recorded = recordEvent(st)
    { ...recorded, phases: st.phases.exclude(Set(p)).union(Set(updated)) }
  }
  action sampleRss(id: int, sample: Sample): bool = {
    nondet p = s.phases.filter(p => p.id == id and canSample(s, p, sample)).oneOf()
    s' = applySample(s, p, sample)
  }
  pure def applyClose(st: State): State = {
    val closed = { ...st, closed: true }
    { ...closed, coverageValid: eligible(closed) }
  }
  action closeProfile: bool = all { not(s.closed), s' = applyClose(s) }

  // Closed is terminal; closeProfile is always enabled while open.
  action step = any {
    nondet p = PHASES.filter(p => canAdd(s, p)).oneOf()
    addPhase(p),
    nondet sample = SAMPLES.oneOf()
    nondet p = s.phases.filter(p => canSample(s, p, sample)).oneOf()
    sampleRss(p.id, sample),
    closeProfile,
  }
  val exclusiveOwnership = ownershipOk(s)
  val wallReconciliation = reconciliationOk(s)
  val coverageQualification = coverageOk(s)
  val rssHonesty = peaksOk(s)
  val unsupportedCounters = countersOk(s)
  val overheadQualification = overheadOk(s)
  val bufferIsolation = bufferOk(s)
  val wAdded = s.phases.size() > 0
  val wSampled = s.phases.exists(p => p.memory.samples.size() > 0)
  val wDense = s.phases.exists(p => p.memory.quality == SampledLowerBound)
  val wClosed = s.closed
  val wCovered = s.coverageValid
  val wDropped = s.dropped > 0
}
