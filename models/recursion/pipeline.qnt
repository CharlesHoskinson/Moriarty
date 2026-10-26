// Recursive proving pipeline, grounded in ../../wiki-llm/recursion-perf/stages.json.
// Shared-memory scheduler: no message transport or cryptography is simulated.
// Decimal MB and integer milliseconds. See RESULTS.md for scope and provenance.
module pipeline_model {
  const LEAVES: int
  const ARITY: int

  type Phase = Queued | KeysReady | Witnessed | Committed | LookupsArgued
    | Quotiented | Opened | Proved | Verified | Consumed | Decided
    | Submitted | Finalized | Failed
  type Role = Leaf | Node | Decider | Ledger
  type Job = { phase: Phase, ram: int, cores: int, gpu: bool, vk: int, uses: int }
  type Consumption = { parent: int, child: int, wasVerified: bool }
  type State = {
    jobs: int -> Job,
    parentVk: int -> int,
    installedVk: int,
    finalized: List[int],
    receipts: List[Consumption],
    clockMs: int,
    otherHostMb: int,
  }
  var s: State

  pure val CPU_CORES = 20
  pure val RUN_RAM_MB = 32000
  pure val HOST_RAM_MB = 90000
  pure val GPU_VRAM_MB = 32000
  pure val BATCHES = Set(0, 1)

  // At most four reduction levels for 16 leaves and arity >= 2.
  // A single leaf gets one unary wrapper node. Partial groups are allowed.
  pure val WIDTHS = 1.to(4).fold([LEAVES], (ws, _) =>
    if (ws[ws.length() - 1] > 1 or ws.length() == 1)
      ws.append((ws[ws.length() - 1] + ARITY - 1) / ARITY) else ws)
  pure val TREE = WIDTHS.indices().fold(Set(), (ids, level) =>
    ids.union(0.to(WIDTHS[level] - 1).map(slot => 16 * level + slot)))
  pure def batch(j: int): int = j / 1000
  pure def role(j: int): Role =
    if (j % 1000 == 100) Decider else if (j % 1000 == 101) Ledger
    else if (j % 1000 < 16) Leaf else Node
  pure def root(b: int): int = b * 1000 + 16 * (WIDTHS.length() - 1)
  pure def decider(b: int): int = b * 1000 + 100
  pure def ledger(b: int): int = b * 1000 + 101
  pure val JOBS = BATCHES.fold(Set(), (ids, b) =>
    ids.union(TREE.map(j => b * 1000 + j)).union(Set(decider(b), ledger(b))))
  pure def children(j: int): Set[int] =
    if (role(j) == Leaf) Set()
    else if (role(j) == Decider) Set(root(batch(j)))
    else if (role(j) == Ledger) Set(decider(batch(j)))
    else {
      val level = (j % 1000) / 16
      val slot = j % 16
      0.to(WIDTHS[level - 1] - 1)
        .filter(i => i / ARITY == slot)
        .map(i => batch(j) * 1000 + 16 * (level - 1) + i)
    }
  pure val configurationOk = Set(1, 2, 4, 16).contains(LEAVES)
    and 2.to(4).contains(ARITY) and WIDTHS[WIDTHS.length() - 1] == 1

  pure val EMPTY_JOB: Job = { phase: Queued, ram: 0, cores: 0, gpu: false, vk: 1, uses: 0 }
  pure def initial(otherMb: int): State = {
    jobs: JOBS.mapBy(_ => EMPTY_JOB), parentVk: BATCHES.mapBy(_ => 1),
    installedVk: 1, finalized: [], receipts: [], clockMs: 0, otherHostMb: otherMb,
  }
  // Other workloads are abstracted as a fixed 0 or 60 GB host reservation.
  // At least one largest job must fit, otherwise the environment itself can
  // prevent progress. The honest scheduler cannot control other workloads.
  action initWithHostLoad(otherMb: int): bool = all {
    configurationOk, otherMb >= 0, otherMb + 16500 <= HOST_RAM_MB,
    s' = initial(otherMb),
  }
  action init = {
    nondet otherMb = Set(0, 60000).oneOf()
    initWithHostLoad(otherMb)
  }
  // RSS: 15.8 GB Sherlock parent rounded up; 16.5 GB Sherlock decider
  // dominates recl9's 14.99/14.97 GB. Ledger B4 reports 4.2 GB.
  // Leaf 1.5 GB is a user-supplied assumption, NOT present in stages.json.
  pure def peakMb(j: int): int =
    if (role(j) == Leaf) 1500 else if (role(j) == Node) 16000
    else if (role(j) == Decider) 16500 else 4200
  // CPU reservations and GPU footprints are placeholders: the source reports
  // utilization / thread counts, not a per-phase resource profile.
  pure def coreNeed(j: int): int =
    if (role(j) == Leaf) 2 else if (role(j) == Node) 3
    else if (role(j) == Decider) 6 else 2
  pure def vramMb(j: int): int = if (role(j) == Leaf) 1500 else 16000
  pure def ramUsed(st: State): int = JOBS.fold(0, (n, j) => n + st.jobs.get(j).ram)
  pure def coresUsed(st: State): int = JOBS.fold(0, (n, j) => n + st.jobs.get(j).cores)
  pure def gpuUsers(st: State): Set[int] = JOBS.filter(j => st.jobs.get(j).gpu)
  pure def vramUsed(st: State): int = gpuUsers(st).fold(0, (n, j) => n + vramMb(j))

  // Whole-run observations (s1, s7, s8). No new measurements.
  pure val SHERLOCK_SETUP_MS = 62300
  pure val RECL9_NODE_SETUP_MS = 622800
  pure val RECL9_NODE_PROVE_MS = 867300
  pure val RECL9_DECIDER_PROVE_MS = 586400
  pure val SHERLOCK_PARENT_PROVE_MS = 382000
  pure val SHERLOCK_DECIDER_PROVE_MS = 325000
  pure val LEDGER_STOCK_VERIFY_MS = 155000
  pure val LEDGER_VERIFY_MS = 21
  // s3's k17 12.3 s is a Nocy PLONK benchmark, NOT a recl9 leaf timing.
  // It is only a proxy. Leaf setup and settlement/readback are placeholders.
  pure val LEAF_PROVE_PROXY_MS = 12300
  pure val LEAF_SETUP_PLACEHOLDER_MS = 1000
  pure val SETTLE_PLACEHOLDER_MS = 1000
  pure def setupMs(j: int): int =
    if (role(j) == Leaf) LEAF_SETUP_PLACEHOLDER_MS
    else if (role(j) == Node) RECL9_NODE_SETUP_MS
    else SHERLOCK_SETUP_MS // proxy for decider setup; unmeasured separately
  pure def proveMs(j: int): int =
    if (role(j) == Leaf) LEAF_PROVE_PROXY_MS
    else if (role(j) == Node) RECL9_NODE_PROVE_MS else RECL9_DECIDER_PROVE_MS

  // This clock adds transition service times; overlapping reservations are
  // modelled, but this is NOT a concurrent makespan/throughput prediction.
  // All internal phase allocations (20/20/10/30/10/10%) are placeholders.
  // They sum exactly to the observed whole-proof time for nodes/deciders.
  pure def transitionMs(j: int, p: Phase): int =
    if (p == KeysReady or p == Witnessed) proveMs(j) * 2 / 10
    else if (p == Committed or p == Quotiented or p == Opened) proveMs(j) / 10
    else if (p == LookupsArgued) proveMs(j) * 3 / 10
    else LEDGER_VERIFY_MS // off-chain verification proxy; only ledger 21 ms is measured
  pure def nextPhase(p: Phase): Phase = match p {
    | KeysReady => Witnessed | Witnessed => Committed
    | Committed => LookupsArgued | LookupsArgued => Quotiented
    | Quotiented => Opened | Opened => Proved | _ => Verified
  }
  pure def gpuPhase(p: Phase): bool = Set(Committed, Quotiented, Opened).contains(p)
  pure def proofPhase(p: Phase): bool =
    Set(KeysReady, Witnessed, Committed, LookupsArgued, Quotiented, Opened, Proved).contains(p)
  pure def terminal(st: State): bool =
    JOBS.forall(j => Set(Finalized, Failed).contains(st.jobs.get(j).phase))
  pure def room(st: State, j: int): bool = and {
    ramUsed(st) + peakMb(j) <= RUN_RAM_MB,
    st.otherHostMb + ramUsed(st) + peakMb(j) <= HOST_RAM_MB,
    coresUsed(st) + coreNeed(j) <= CPU_CORES,
  }
  pure def inputsReady(st: State, j: int): bool = children(j).forall(c =>
    st.jobs.get(c).phase == Verified and st.jobs.get(c).uses == 0)
  pure def canAdmit(st: State, j: int): bool = and {
    role(j) != Ledger, st.jobs.get(j).phase == Queued,
    inputsReady(st, j), room(st, j),
  }
  pure def consumeInputs(st: State, j: int): State = {
    val js = children(j).fold(st.jobs, (jobs, c) =>
      jobs.set(c, { ...jobs.get(c), phase: Consumed, uses: jobs.get(c).uses + 1 }))
    val rs = children(j).fold(st.receipts, (rs, c) =>
      rs.append({ parent: j, child: c, wasVerified: st.jobs.get(c).phase == Verified }))
    { ...st, jobs: js, receipts: rs }
  }
  pure def applyAdmission(st: State, j: int): State = {
    val consumed = consumeInputs(st, j)
    val job = { ...st.jobs.get(j), phase: KeysReady, ram: peakMb(j), cores: coreNeed(j),
      vk: if (role(j) == Decider) st.parentVk.get(batch(j)) else st.jobs.get(j).vk }
    { ...consumed, jobs: consumed.jobs.set(j, job), clockMs: st.clockMs + setupMs(j) }
  }
  action admit(j: int): bool = all { canAdmit(s, j), s' = applyAdmission(s, j) }

  pure def canAdvance(st: State, j: int): bool =
    proofPhase(st.jobs.get(j).phase) and
    (not(gpuPhase(nextPhase(st.jobs.get(j).phase)))
      or gpuUsers(st).exclude(Set(j)) == Set())
  pure def applyAdvance(st: State, j: int): State = {
    val old = st.jobs.get(j)
    val p = nextPhase(old.phase)
    val job = { ...old, phase: p, gpu: gpuPhase(p),
      ram: if (p == Verified) 0 else old.ram,
      cores: if (p == Verified) 0 else old.cores }
    { ...st, jobs: st.jobs.set(j, job), clockMs: st.clockMs + transitionMs(j, old.phase) }
  }
  action advance(j: int): bool = all { canAdvance(s, j), s' = applyAdvance(s, j) }
  // A new root proof retains ownership of its already-consumed inputs. This
  // is a second attempt of the SAME logical job, not a second consumer.
  // The concrete re-proof runs KeysReady..Verified and reserves RAM again.
  pure def canReprove(st: State, b: int): bool = and {
    st.jobs.get(root(b)).phase == Verified,
    st.jobs.get(decider(b)).phase == Queued,
    st.parentVk.get(b) == 1, room(st, root(b)),
  }
  pure def applyReproof(st: State, b: int): State = {
    val j = root(b)
    val job = { ...st.jobs.get(j), phase: KeysReady, vk: 2,
      ram: peakMb(j), cores: coreNeed(j) }
    { ...st, jobs: st.jobs.set(j, job), parentVk: st.parentVk.set(b, 2),
      clockMs: st.clockMs + setupMs(j) }
  }
  action reproveParent(b: int): bool = all { canReprove(s, b), s' = applyReproof(s, b) }
  // Deployment is explicit and excluded from step. No accepted/in-flight
  // decider may be invalidated by replacing its installed contract key.
  action installVk(vk: int): bool = all {
    Set(1, 2).contains(vk), vk != s.installedVk,
    BATCHES.forall(b => s.jobs.get(decider(b)).phase == Queued),
    s' = { ...s, installedVk: vk },
  }
  pure def boundVk(st: State, b: int): bool =
    st.jobs.get(decider(b)).vk == st.parentVk.get(b)
      and st.jobs.get(decider(b)).vk == st.installedVk
  pure def canDecide(st: State, b: int): bool =
    st.jobs.get(decider(b)).phase == Verified and boundVk(st, b)
  pure def applyDecision(st: State, b: int): State =
    { ...st, jobs: st.jobs.set(decider(b), { ...st.jobs.get(decider(b)), phase: Decided }) }
  action decide(b: int): bool = all { canDecide(s, b), s' = applyDecision(s, b) }

  // Invalid proofs / incompatible keys are fail-stop. Abort the failed batch
  // AND its suffix, since a failed predecessor cannot support ordered settle.
  // Proof artifacts/receipts are retained, all resource reservations released.
  pure def applyFailure(st: State, b: int): State = {
    val js = JOBS.fold(st.jobs, (js, j) =>
      if (batch(j) >= b and js.get(j).phase != Finalized)
        js.set(j, { ...js.get(j), phase: Failed, ram: 0, cores: 0, gpu: false }) else js)
    { ...st, jobs: js }
  }
  pure def canRejectVk(st: State, b: int): bool =
    st.jobs.get(decider(b)).phase == Verified and not(boundVk(st, b))
  action rejectVk(b: int): bool = all { canRejectVk(s, b), s' = applyFailure(s, b) }
  // Environmental verification failure, explicitly exercised by tests.
  // step models successful off-chain verification and key rejection only.
  action rejectProof(j: int): bool = all {
    role(j) != Ledger, s.jobs.get(j).phase == Proved,
    s' = applyFailure(s, batch(j)),
  }

  pure def canSubmit(st: State, b: int): bool = and {
    st.jobs.get(ledger(b)).phase == Queued,
    st.jobs.get(decider(b)).phase == Decided, room(st, ledger(b)),
  }
  pure def applySubmission(st: State, b: int): State = {
    val j = ledger(b)
    val consumed = consumeInputs(st, j)
    val job = { ...st.jobs.get(j), phase: Submitted, ram: peakMb(j), cores: coreNeed(j),
      vk: st.jobs.get(decider(b)).vk }
    { ...consumed, jobs: consumed.jobs.set(j, job),
      clockMs: st.clockMs + LEDGER_STOCK_VERIFY_MS }
  }
  action submit(b: int): bool = all { canSubmit(s, b), s' = applySubmission(s, b) }
  pure def canFinalize(st: State, b: int): bool =
    st.jobs.get(ledger(b)).phase == Submitted and b == st.finalized.length()
  pure def applyFinalization(st: State, b: int): State = {
    val js = JOBS.filter(j => batch(j) == b).fold(st.jobs, (js, j) =>
      js.set(j, { ...js.get(j), phase: Finalized, ram: 0, cores: 0, gpu: false }))
    { ...st, jobs: js, finalized: st.finalized.append(b),
      clockMs: st.clockMs + LEDGER_VERIFY_MS + SETTLE_PLACEHOLDER_MS }
  }
  action finalize(b: int): bool = all { canFinalize(s, b), s' = applyFinalization(s, b) }

  // Four seeded faults, EXCLUDED from the honest step. Each changes just one
  // protocol rule; tests reach its precondition using honest actions.
  // M1: trust a newly rebound parent/decider VK without reinstalling it.
  action mutantVkRebinding(b: int): bool = all {
    s.jobs.get(decider(b)).phase == Verified, not(boundVk(s, b)),
    s' = applyDecision(s, b),
  }
  // M2: the same Verified child appears twice in the parent's input list.
  pure def applyChildReuse(st: State, j: int, c: int): State = {
    val admitted = applyAdmission(st, j)
    { ...admitted,
      jobs: admitted.jobs.set(c, { ...admitted.jobs.get(c), uses: admitted.jobs.get(c).uses + 1 }),
      receipts: admitted.receipts.append({ parent: j, child: c, wasVerified: st.jobs.get(c).phase == Verified }) }
  }
  action mutantChildReuse(j: int, c: int): bool = all {
    canAdmit(s, j), children(j).contains(c), s' = applyChildReuse(s, j, c),
  }
  // M3: preserve CPU admission and dependency checks, omit both RAM checks.
  action mutantOverAdmission(j: int): bool = all {
    role(j) != Ledger, s.jobs.get(j).phase == Queued, inputsReady(s, j),
    coresUsed(s) + coreNeed(j) <= CPU_CORES,
    s' = applyAdmission(s, j),
  }
  // M4: settle a submitted batch without checking the predecessor.
  action mutantReorderedBatch(b: int): bool = all {
    s.jobs.get(ledger(b)).phase == Submitted, s' = applyFinalization(s, b),
  }

  action step = any {
    nondet j = JOBS.filter(j => canAdmit(s, j)).oneOf()
    admit(j),
    nondet j = JOBS.filter(j => canAdvance(s, j)).oneOf()
    advance(j),
    nondet b = BATCHES.filter(b => canReprove(s, b)).oneOf()
    reproveParent(b),
    nondet b = BATCHES.filter(b => canDecide(s, b)).oneOf()
    decide(b),
    nondet b = BATCHES.filter(b => canRejectVk(s, b)).oneOf()
    rejectVk(b),
    nondet b = BATCHES.filter(b => canSubmit(s, b)).oneOf()
    submit(b),
    nondet b = BATCHES.filter(b => canFinalize(s, b)).oneOf()
    finalize(b),
  }
  val memoryCaps = ramUsed(s) <= RUN_RAM_MB and s.otherHostMb + ramUsed(s) <= HOST_RAM_MB
    and vramUsed(s) <= GPU_VRAM_MB
  val cpuGpuCaps = coresUsed(s) <= CPU_CORES and gpuUsers(s).size() <= 1
  val verifiedChildren = s.receipts.indices().forall(i => {
    val r = s.receipts[i]
    children(r.parent).contains(r.child)
      and (role(r.parent) == Ledger or r.wasVerified)
  })
  val singleUse = JOBS.forall(j => s.jobs.get(j).uses <= 1
    and s.jobs.get(j).uses == s.receipts.select(r => r.child == j).length())
  val vkBinding = BATCHES.forall(b => {
    val p = s.jobs.get(decider(b)).phase
    and {
      Set(Decided, Consumed, Finalized).contains(p) implies boundVk(s, b),
      Set(Submitted, Finalized).contains(s.jobs.get(ledger(b)).phase)
        implies s.jobs.get(ledger(b)).vk == s.installedVk,
    }
  })
  val orderedBatches = s.finalized.indices().forall(i => s.finalized[i] == i)
    and BATCHES.forall(b => (s.jobs.get(ledger(b)).phase == Finalized)
      iff s.finalized.indices().exists(i => s.finalized[i] == b))
  // Explicit enabledness: quint run alone silently accepts truncated deadlocks.
  // No failure escape or unconditional stutter is counted as progress here.
  val noDeadlock = terminal(s)
    or JOBS.exists(j => canAdmit(s, j) or canAdvance(s, j))
    or BATCHES.exists(b => canDecide(s, b) or canRejectVk(s, b)
      or canSubmit(s, b) or canFinalize(s, b))
  val resourceAccounting = s.clockMs >= 0 and s.otherHostMb >= 0
    and JOBS.forall(j => {
      val job = s.jobs.get(j)
      val active = proofPhase(job.phase) or job.phase == Submitted
      job.ram == (if (active) peakMb(j) else 0)
        and job.cores == (if (active) coreNeed(j) else 0)
        and job.gpu == gpuPhase(job.phase)
    })
  val allInvariants = memoryCaps and cpuGpuCaps and verifiedChildren and singleUse
    and vkBinding and orderedBatches and noDeadlock and resourceAccounting

  val wAdmitted = JOBS.exists(j => s.jobs.get(j).phase == KeysReady)
  val wAdvanced = JOBS.exists(j => s.jobs.get(j).phase == Witnessed)
  val wConsumed = s.receipts.length() > 0
  val wGpu = gpuUsers(s).size() == 1
  val wReproved = BATCHES.exists(b => s.parentVk.get(b) == 2)
  val wDecided = BATCHES.exists(b => s.jobs.get(decider(b)).phase == Decided)
  val wSubmitted = BATCHES.exists(b => s.jobs.get(ledger(b)).phase == Submitted)
  val wFinalized = s.finalized.length() > 0
  val wAllFinalized = s.finalized.length() == BATCHES.size()
  val wRejected = JOBS.exists(j => s.jobs.get(j).phase == Failed)
}

// Runnable default; parameterized instances may import pipeline_model directly.
module pipeline {
  import pipeline_model(LEAVES = 1, ARITY = 2).*
}

// Instances for predicate M1 (N in {1,2,4,16} leaves, arity 2..4). Added by the lead on 2026-10-26.
module pipeline_l2a2 { import pipeline_model(LEAVES = 2, ARITY = 2).* }
module pipeline_l4a2 { import pipeline_model(LEAVES = 4, ARITY = 2).* }
module pipeline_l4a4 { import pipeline_model(LEAVES = 4, ARITY = 4).* }
module pipeline_l16a4 { import pipeline_model(LEAVES = 16, ARITY = 4).* }
