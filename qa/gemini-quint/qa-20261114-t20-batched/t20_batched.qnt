// Independent soundness model for Sherlock row T20: batched Midnight writes.
// Reviews lane-src/sherlock-ledger.compact and lane-src/sherlock-claim-consumer.compact as built,
// together with rules W-1..W-5 and open registry rule N-8.
//
// Module parameters:
//   MUT:
//     "none"                     sound T20 design as built (+ W-4 batched registry updates, W-5 planner cap)
//     "per_job_accept_run"       W-1 violated: operator settles each proved job via acceptRun
//     "per_claim_open"           W-2 violated: consumer claim use writes openClaim on SherlockLedger
//     "nullifier_keyed_by_root"  W-2 control: consumer nullifier keyed by (claims_root, leaf)
//     "no_root_read"             W-2 control: consumer omits sherlock.rootAccepted(root)
//     "no_kind_tag"              W-3 control: foreign leaf omits SHK_CLAIM_FGN domain tag
//     "chunk_no_cap"             W-5 violated: planner chunks oversized calls with no K_MAX cap
//     "chunk_truncate"           W-5 violated: planner silently truncates effect words above K_MAX * W_CHUNK
//     "per_key_reg_write"        W-4 violated: each key add/revoke writes reg_root immediately
//     "no_reg_check"             W-4 violated: acceptCollection omits reg_root == reg_pub check
//     "descend_k_open"           Fix 5 violated: stale descend writes ceil(d / K_OPEN) sub-roots per deep claim
//     "nullifier_keyed_by_pos"   Extra Q2(b): consumer nullifier keyed by (slotIndex, leaf)
//     "unbound_claims_root"      Extra W-2/T8b: L allows unproved/premise leaves into claims_root
//
//   REG_MODE (N-8 is open; both options modelled without deciding N-8):
//     "strict"                   acceptCollection requires reg_pub == currentRegRoot (contract as built)
//     "grace_one"                acceptCollection accepts currentRegRoot or prevRegRoot (one-epoch grace)

module t20_batched_base {
  const MUT: str
  const REG_MODE: str

  // --------------------------------------------------------------------------
  // 1. Types
  // --------------------------------------------------------------------------

  type RunSpec = {
    cp: int,
    inp: int,
    outp: int,
  }

  type FgnSpec = {
    fpub: int,
    keyId: int,
  }

  type PendingJob = {
    isFgn: bool,
    runSpec: RunSpec,
    fgnSpec: FgnSpec,
    dagDepth: int,
    hasReceipt: bool,
  }

  type MerklePath = {
    sibs: List[int],
    bits: List[bool],
  }

  type AcceptedColl = {
    rootVal: int,
    epochNum: int,
    regNamed: int,
    regCurrentAtAccept: int,
    regPrevAtAccept: int,
    fgnKeysUsed: Set[int],
  }

  type UseRecord = {
    cid: int,
    usedAsFgn: bool,
    runSpec: RunSpec,
    fpubVal: int,
    fromRuns: bool,
    rootUsed: int,
  }

  type CallPlan = {
    effectWords: int,
    ttlSec: int,
    kMax: int,
    chunks: int,
    coveredWords: int,
    anchorWrites: int,
    refused: bool,
    isGeneralRun: bool,
  }

  type SysState = {
    // Collection epoch & operator batching state
    collEpoch: int,
    pendingJobs: List[PendingJob],
    stagedRegRoot: int,
    collRefusals: int,

    // SherlockLedger on-chain state
    runsMap: int -> int,
    claimRoots: Set[int],
    currentRegRoot: int,
    prevRegRoot: int,

    // Off-chain data availability, Merkle trees, and receipts
    rootLeaves: int -> List[int],
    rootEpochMap: int -> int,
    honestRootRuns: int -> Set[RunSpec],
    honestRootFgns: int -> Set[FgnSpec],
    rootMaxDepth: int -> int,
    daPublishedRoots: Set[int],
    receiptPairs: Set[(int, int)],

    // Compact consumers' own ledger state (cid -> Set of nullifiers)
    consumerUsed: int -> Set[int],

    // Registry epoch admin state (W-4, N-8)
    regEpoch: int,
    regTable: int -> Set[int],
    nextRegId: int,
    stagedRegKeys: Set[int],
    stagedKeyOps: int,
    regWritesPerEp: int -> int,
    batchedTwoKeyEpReached: bool,

    // Planner state for Compact calls and general Sherlock runs (W-5)
    callPlans: Set[CallPlan],
    userContractAnchorWrites: int,

    // Write-accounting & ghost audit counters
    ledgerWrites: int,
    collEpochsWithRoot: Set[int],
    userPaidRuns: int,
    regEpochsWithWrite: Set[int],
    descendWrites: int,
    acceptedColls: List[AcceptedColl],
    useLog: List[UseRecord],
  }

  // --------------------------------------------------------------------------
  // 2. Constants
  // --------------------------------------------------------------------------

  // Field constants and domain tags (atomic values in 0..9; hashes are >= 10)
  pure val ZERO_FIELD: int = 0
  pure val TAG_FGN: int = 8
  pure val TAG_NUL: int = 9

  // Fixed Merkle tree depth D_TREE = 2 (4 leaf slots 0..3), abstracting D_C = 20 (2^20 slots)
  pure val D_TREE: int = 2
  pure val TREE_CAP: int = 4
  pure val ALL_SLOTS: Set[int] = Set(0, 1, 2, 3)

  // Stale descend bound for Fix 5 mutant
  pure val K_OPEN: int = 2

  // Horizon bounds for finite simulation
  pure val MAX_COLL_EPOCH: int = 3
  pure val MAX_REG_EPOCH: int = 3
  pure val CONSUMERS: Set[int] = Set(1, 2)
  pure val ALL_KEYS: Set[int] = Set(1, 2, 3)
  pure val DAG_DEPTHS: Set[int] = Set(0, 1, 3)

  // Dummy placeholders for unused union fields in PendingJob / UseRecord
  pure val DUMMY_RUN: RunSpec = { cp: 0, inp: 0, outp: 0 }
  pure val DUMMY_FGN: FgnSpec = { fpub: 0, keyId: 0 }

  // Proved vs forged run statements and ADM/VIEW premise statement
  pure val RUN_A: RunSpec = { cp: 1, inp: 1, outp: 1 }
  pure val RUN_B: RunSpec = { cp: 1, inp: 2, outp: 2 }
  pure val RUN_C: RunSpec = { cp: 2, inp: 1, outp: 3 }
  pure val RUN_D: RunSpec = { cp: 2, inp: 2, outp: 2 }
  pure val RUN_E: RunSpec = { cp: 3, inp: 1, outp: 1 }
  pure val PROVED_RUNS: Set[RunSpec] = Set(RUN_A, RUN_B, RUN_C, RUN_D, RUN_E)

  pure val FORGED_RUN_A: RunSpec = { cp: 1, inp: 1, outp: 4 }
  pure val FORGED_RUN_B: RunSpec = { cp: 2, inp: 2, outp: 5 }
  pure val ADM_PREMISE_SPEC: RunSpec = { cp: 3, inp: 3, outp: 3 }
  pure val UNPROVED_RUNS: Set[RunSpec] = Set(FORGED_RUN_A, FORGED_RUN_B, ADM_PREMISE_SPEC)

  // Valid vs forged foreign claims
  pure val FGN_1: FgnSpec = { fpub: 1, keyId: 1 }
  pure val FGN_2: FgnSpec = { fpub: 2, keyId: 2 }
  pure val VALID_FGNS: Set[FgnSpec] = Set(FGN_1, FGN_2)
  pure val FORGED_FPUB: int = 6

  // Chunk cap constants (W-5, RESULTS.md & ARCHITECTURE.md)
  pure val L_TX: int = 1024
  pure val ANCHOR_PREFIX_WORDS: int = 9
  pure val ONE_CALL_MAX: int = L_TX - ANCHOR_PREFIX_WORDS
  pure val W_CHUNK: int = 990
  pure val DEFAULT_TTL_SEC: int = 3600
  pure val TEN_X_PROVE_SEC: int = 1755
  pure val BLOCK_MS: int = 2000
  pure val WRAP_VERIFY_HIGH_MS: int = 107
  pure val BLOCK_COMPUTE_CAP: int = BLOCK_MS / WRAP_VERIFY_HIGH_MS
  pure val PREVIEW_TX_LIMIT_BYTES: int = 1048576
  pure val BASE_ACCEPT_RUN_BYTES: int = 6783
  pure val BYTES_PER_EFFECT_WORD: int = 33
  pure val FULL_CHUNK_BYTES: int = BASE_ACCEPT_RUN_BYTES + BYTES_PER_EFFECT_WORD * W_CHUNK
  pure val TX_BYTE_CAP: int = PREVIEW_TX_LIMIT_BYTES / FULL_CHUNK_BYTES

  pure val MEASURED_SIZES: Set[int] = Set(10, 40, 46, 287, 328, 443, 474)
  pure val STATIC_MAX_WORDS: int = 699
  pure val CANDIDATE_CALL_SIZES: Set[int] = Set(40, 474, 699, 1015, 1016, 1980, 1981, 2970)

  // Typed empty collections
  pure val EMPTY_INT_SET: Set[int] = Set()
  pure val EMPTY_PAIR_SET: Set[(int, int)] = Set()
  pure val EMPTY_PLAN_SET: Set[CallPlan] = Set()
  pure val EMPTY_JOBS: List[PendingJob] = List()
  pure val EMPTY_COLLS: List[AcceptedColl] = List()
  pure val EMPTY_USES: List[UseRecord] = List()
  pure val EMPTY_INT_MAP: int -> int = Map()
  pure val EMPTY_LEAVES_MAP: int -> List[int] = Map()
  pure val EMPTY_RUNS_SET_MAP: int -> Set[RunSpec] = Map()
  pure val EMPTY_FGNS_SET_MAP: int -> Set[FgnSpec] = Map()
  pure val EMPTY_INT_LIST: List[int] = List()
  pure val EMPTY_RUN_SPEC_SET: Set[RunSpec] = Set()
  pure val EMPTY_FGN_SPEC_SET: Set[FgnSpec] = Set()
  pure val EMPTY_RUN_TRIPLE_SET: Set[(int, int, RunSpec)] = Set()
  pure val EMPTY_FGN_TRIPLE_SET: Set[(int, int, int)] = Set()

  // --------------------------------------------------------------------------
  // 3. Pure Cryptographic, Merkle Tree, and Protocol Logic
  // --------------------------------------------------------------------------

  pure def minInt(a: int, b: int): int = if (a < b) a else b
  pure def maxInt(a: int, b: int): int = if (a > b) a else b
  pure def ceilDivPos(num: int, den: int): int =
    if (num <= 0) 0 else (num + den - 1) / den

  // Total/safe map lookup helpers (avoid undefined Map.get on absent keys in any evaluation order)
  pure def getIntOr(m: int -> int, k: int, fallback: int): int =
    if (m.keys().contains(k)) m.get(k) else fallback

  pure def getIntSetOr(m: int -> Set[int], k: int): Set[int] =
    if (m.keys().contains(k)) m.get(k) else EMPTY_INT_SET

  pure def getLeavesOr(m: int -> List[int], k: int): List[int] =
    if (m.keys().contains(k)) m.get(k) else EMPTY_INT_LIST

  pure def getRunSetOr(m: int -> Set[RunSpec], k: int): Set[RunSpec] =
    if (m.keys().contains(k)) m.get(k) else EMPTY_RUN_SPEC_SET

  pure def getFgnSetOr(m: int -> Set[FgnSpec], k: int): Set[FgnSpec] =
    if (m.keys().contains(k)) m.get(k) else EMPTY_FGN_SPEC_SET

  // Cantor bijection N x N -> N. Collision-free model of Poseidon over Fr:
  //   hash2(l, r) is always even >= 10 (models transientHash<Vector<2, Field>>, cap(0, 2))
  //   hash3(a, b, c) is always odd >= 11 (models transientHash<Vector<3, Field>>, cap(0, 3))
  //   Atomic field constants (ZERO_FIELD = 0, small IDs, TAG_FGN = 8, TAG_NUL = 9) are in 0..9.
  pure def cantorPair(a: int, b: int): int =
    ((a + b) * (a + b + 1)) / 2 + b

  // LEAD REPAIR (2026-11-14): the Cantor pair overflows the simulator's integers (QNT601) on nested
  // hashes; keep the parity split (even H2, odd H3, both >= 10) and reduce modulo a 30-bit prime.
  // Not injective in general; collision freedom over the few values the model reaches is assumed
  // (a collision would show as a spurious violation, not a missed one, for the sound instances).
  pure val HP = 1000000007
  pure def mixH(a: int, b: int): int = ((a % HP) * 1000003 + (b % HP) * 999983 + 7) % HP
  pure def hash2(l: int, r: int): int =
    10 + 2 * mixH(l, r)

  pure def hash3(a: int, b: int, c: int): int =
    11 + 2 * mixH(a, mixH(b, c) + 1)

  // Run claim leaf: transientHash<Vector<3, Field>>([c_p, input_pub, outcome_pub])
  pure def runClaimPub(r: RunSpec): int =
    hash3(r.cp, r.inp, r.outp)

  // Foreign claim leaf: transientHash<Vector<2, Field>>([SHK_CLAIM_FGN, fclaim_pub])
  // Mutant no_kind_tag omits SHK_CLAIM_FGN (matching controls/consumer-control-no-kind-tag.compact:58).
  pure def fgnLeafOf(mut: str, fpub: int): int =
    if (mut == "no_kind_tag") fpub else hash2(TAG_FGN, fpub)

  // Merkle node: transientHash<Vector<2, Field>>([right ? sib : cur, right ? cur : sib])
  pure def merkleStep(cur: int, sib: int, isRight: bool): int =
    if (isRight) hash2(sib, cur) else hash2(cur, sib)

  // Consumer circuit rootOf(leaf): folds D_TREE steps along (sibs, bits)
  pure def rootFromPath(leaf: int, p: MerklePath): int =
    range(0, D_TREE).foldl(leaf, (cur, d) =>
      if (d < p.sibs.length() and d < p.bits.length())
        merkleStep(cur, p.sibs.nth(d), p.bits.nth(d))
      else
        cur
    )

  pure def pathWellFormed(p: MerklePath): bool =
    p.sibs.length() == D_TREE and p.bits.length() == D_TREE

  // Zero-padded 4-leaf array for D_TREE = 2
  pure def padLeaves4(rawLeaves: List[int]): List[int] =
    range(0, TREE_CAP).foldl(EMPTY_INT_LIST, (acc, idx) =>
      acc.append(if (idx < rawLeaves.length()) rawLeaves.nth(idx) else ZERO_FIELD))

  // Honest Merkle root of <= 4 leaves padded with ZERO_FIELD = 0
  pure def treeRoot4(rawLeaves: List[int]): int = {
    val padded = padLeaves4(rawLeaves)
    val n0 = hash2(padded.nth(0), padded.nth(1))
    val n1 = hash2(padded.nth(2), padded.nth(3))
    hash2(n0, n1)
  }

  // Height-1 internal nodes of the 4-leaf Merkle tree (for Q2 internal-node checks)
  pure def treeInternalNodes4(rawLeaves: List[int]): Set[int] = {
    val padded = padLeaves4(rawLeaves)
    Set(hash2(padded.nth(0), padded.nth(1)), hash2(padded.nth(2), padded.nth(3)))
  }

  // Deterministic Merkle sibling path for slotIdx in 0..3
  pure def treePath4(rawLeaves: List[int], slotIdx: int): MerklePath = {
    val padded = padLeaves4(rawLeaves)
    val n0 = hash2(padded.nth(0), padded.nth(1))
    val n1 = hash2(padded.nth(2), padded.nth(3))
    val sib0 =
      if (slotIdx == 0) padded.nth(1)
      else if (slotIdx == 1) padded.nth(0)
      else if (slotIdx == 2) padded.nth(3)
      else padded.nth(2)
    val sib1 = if (slotIdx < 2) n1 else n0
    val b0 = (slotIdx % 2) == 1
    val b1 = (slotIdx / 2) == 1
    { sibs: List(sib0, sib1), bits: List(b0, b1) }
  }

  pure def bitsToSlot(bits: List[bool]): int =
    if (bits.length() >= 2)
      (if (bits.nth(0)) 1 else 0) + (if (bits.nth(1)) 2 else 0)
    else
      0

  // Consumer useOnce nullifier computation (sherlock-claim-consumer.compact:34-38)
  pure def collectionNullifier(mut: str, rootVal: int, bits: List[bool], leafKey: int): int = {
    val innerKey =
      if (mut == "nullifier_keyed_by_root") hash2(rootVal, leafKey)
      else if (mut == "nullifier_keyed_by_pos") hash2(bitsToSlot(bits), leafKey)
      else leafKey
    hash2(TAG_NUL, innerKey)
  }

  pure def settledRunNullifier(claimPub: int): int =
    hash2(TAG_NUL, claimPub)

  // Planner cap derivation (W-5: least of TTL limit, block compute limit, tx byte limit)
  pure def derivedKMax(ttlSec: int): int =
    minInt(ttlSec / TEN_X_PROVE_SEC, minInt(BLOCK_COMPUTE_CAP, TX_BYTE_CAP))

  pure def planCompactCall(mut: str, effectWords: int, ttlSec: int): CallPlan = {
    val kMax = derivedKMax(ttlSec)
    if (effectWords <= ONE_CALL_MAX and kMax >= 1) {
      effectWords: effectWords,
      ttlSec: ttlSec,
      kMax: kMax,
      chunks: 1,
      coveredWords: effectWords,
      anchorWrites: 0,
      refused: false,
      isGeneralRun: false,
    } else {
      val neededChunks = ceilDivPos(effectWords, W_CHUNK)
      if (mut == "chunk_no_cap" or (neededChunks <= kMax and kMax >= 1)) {
        effectWords: effectWords,
        ttlSec: ttlSec,
        kMax: kMax,
        chunks: neededChunks,
        coveredWords: effectWords,
        anchorWrites: if (neededChunks > 1) neededChunks else 0,
        refused: false,
        isGeneralRun: false,
      } else if (mut == "chunk_truncate" and kMax >= 1) {
        effectWords: effectWords,
        ttlSec: ttlSec,
        kMax: kMax,
        chunks: kMax,
        coveredWords: kMax * W_CHUNK,
        anchorWrites: if (kMax > 1) kMax else 0,
        refused: false,
        isGeneralRun: false,
      } else {
        effectWords: effectWords,
        ttlSec: ttlSec,
        kMax: kMax,
        chunks: 0,
        coveredWords: 0,
        anchorWrites: 0,
        refused: true,
        isGeneralRun: false,
      }
    }
  }

  // General Sherlock runs always carry the fixed 3-word run statement [C_P, input_pub, outcome_pub]
  pure def planGeneralRun(ttlSec: int): CallPlan = {
    val kMax = derivedKMax(ttlSec)
    {
      effectWords: 3,
      ttlSec: ttlSec,
      kMax: kMax,
      chunks: 1,
      coveredWords: 3,
      anchorWrites: 0,
      refused: false,
      isGeneralRun: true,
    }
  }

  // Registry root check in SherlockLedger.acceptCollection
  pure def regRootAccepted(mut: str, regMode: str, currentReg: int, prevReg: int, candidateReg: int): bool =
    if (mut == "no_reg_check") true
    else if (regMode == "grace_one") (candidateReg == currentReg or candidateReg == prevReg)
    else (candidateReg == currentReg)

  // Helpers to extract leaves, members, keys, and max depth from pendingJobs
  pure def jobLeaf(mut: str, j: PendingJob): int =
    if (j.isFgn) fgnLeafOf(mut, j.fgnSpec.fpub) else runClaimPub(j.runSpec)

  pure def pendingLeaves(mut: str, jobs: List[PendingJob]): List[int] =
    range(0, jobs.length()).foldl(EMPTY_INT_LIST, (acc, i) => acc.append(jobLeaf(mut, jobs.nth(i))))

  pure def pendingHonestRuns(jobs: List[PendingJob]): Set[RunSpec] =
    jobs.indices().fold(EMPTY_RUN_SPEC_SET, (acc, i) => {
      val j = jobs.nth(i)
      if (not(j.isFgn) and PROVED_RUNS.contains(j.runSpec)) acc.union(Set(j.runSpec)) else acc
    })

  pure def pendingHonestFgns(jobs: List[PendingJob]): Set[FgnSpec] =
    jobs.indices().fold(EMPTY_FGN_SPEC_SET, (acc, i) => {
      val j = jobs.nth(i)
      if (j.isFgn and VALID_FGNS.contains(j.fgnSpec)) acc.union(Set(j.fgnSpec)) else acc
    })

  pure def pendingFgnKeys(jobs: List[PendingJob]): Set[int] =
    jobs.indices().fold(EMPTY_INT_SET, (acc, i) => {
      val j = jobs.nth(i)
      if (j.isFgn) acc.union(Set(j.fgnSpec.keyId)) else acc
    })

  pure def pendingMaxDepth(jobs: List[PendingJob]): int =
    jobs.indices().fold(0, (acc, i) => maxInt(acc, jobs.nth(i).dagDepth))

  pure def pendingReceiptPairs(rootVal: int, jobs: List[PendingJob]): Set[(int, int)] =
    jobs.indices().fold(EMPTY_PAIR_SET, (acc, i) =>
      if (jobs.nth(i).hasReceipt) acc.union(Set((rootVal, i))) else acc)

  // --------------------------------------------------------------------------
  // 4. Pure Transition Guards & Updates
  // --------------------------------------------------------------------------

  pure val initState: SysState = {
    collEpoch: 0,
    pendingJobs: EMPTY_JOBS,
    stagedRegRoot: 0,
    collRefusals: 0,
    runsMap: EMPTY_INT_MAP,
    claimRoots: EMPTY_INT_SET,
    currentRegRoot: 0,
    prevRegRoot: 0,
    rootLeaves: EMPTY_LEAVES_MAP,
    rootEpochMap: EMPTY_INT_MAP,
    honestRootRuns: EMPTY_RUNS_SET_MAP,
    honestRootFgns: EMPTY_FGNS_SET_MAP,
    rootMaxDepth: EMPTY_INT_MAP,
    daPublishedRoots: EMPTY_INT_SET,
    receiptPairs: EMPTY_PAIR_SET,
    consumerUsed: CONSUMERS.mapBy(_ => EMPTY_INT_SET),
    regEpoch: 0,
    regTable: Map(0 -> Set(1)),
    nextRegId: 1,
    stagedRegKeys: Set(1),
    stagedKeyOps: 0,
    regWritesPerEp: 0.to(MAX_REG_EPOCH).mapBy(_ => 0),
    batchedTwoKeyEpReached: false,
    callPlans: EMPTY_PLAN_SET,
    userContractAnchorWrites: 0,
    ledgerWrites: 0,
    collEpochsWithRoot: EMPTY_INT_SET,
    userPaidRuns: 0,
    regEpochsWithWrite: EMPTY_INT_SET,
    descendWrites: 0,
    acceptedColls: EMPTY_COLLS,
    useLog: EMPTY_USES,
  }

  // Operator proves a valid Sherlock run job
  pure def canProveRunJob(mut: str, s: SysState, r: RunSpec, depth: int): bool = and {
    s.collEpoch < MAX_COLL_EPOCH,
    PROVED_RUNS.contains(r),
    DAG_DEPTHS.contains(depth),
    s.pendingJobs.length() < TREE_CAP,
    mut == "per_job_accept_run" implies not(s.runsMap.keys().contains(runClaimPub(r))),
  }

  pure def applyProveRunJob(mut: str, s: SysState, r: RunSpec, depth: int, giveReceipt: bool): SysState = {
    val bindReg = if (s.pendingJobs.length() == 0) s.currentRegRoot else s.stagedRegRoot
    if (mut == "per_job_accept_run") {
      val k = runClaimPub(r)
      {
        ...s,
        runsMap: s.runsMap.put(k, r.outp),
        ledgerWrites: s.ledgerWrites + 1,
        collEpochsWithRoot: s.collEpochsWithRoot.union(Set(s.collEpoch)),
      }
    } else {
      val job: PendingJob = {
        isFgn: false,
        runSpec: r,
        fgnSpec: DUMMY_FGN,
        dagDepth: depth,
        hasReceipt: giveReceipt,
      }
      {
        ...s,
        pendingJobs: s.pendingJobs.append(job),
        stagedRegRoot: bindReg,
      }
    }
  }

  // Operator proves a foreign claim job under the epoch's staged registry root
  pure def canProveFgnJob(s: SysState, f: FgnSpec, depth: int): bool = {
    val bindReg = if (s.pendingJobs.length() == 0) s.currentRegRoot else s.stagedRegRoot
    and {
      s.collEpoch < MAX_COLL_EPOCH,
      VALID_FGNS.contains(f),
      DAG_DEPTHS.contains(depth),
      s.pendingJobs.length() < TREE_CAP,
      s.regTable.keys().contains(bindReg),
      getIntSetOr(s.regTable, bindReg).contains(f.keyId),
    }
  }

  pure def applyProveFgnJob(s: SysState, f: FgnSpec, depth: int, giveReceipt: bool): SysState = {
    val bindReg = if (s.pendingJobs.length() == 0) s.currentRegRoot else s.stagedRegRoot
    val job: PendingJob = {
      isFgn: true,
      runSpec: DUMMY_RUN,
      fgnSpec: f,
      dagDepth: depth,
      hasReceipt: giveReceipt,
    }
    {
      ...s,
      pendingJobs: s.pendingJobs.append(job),
      stagedRegRoot: bindReg,
    }
  }

  // Mutant-only: unconstrained L circuit injects an unproved or ADM-premise leaf into pendingJobs
  pure def canInjectUnboundLeaf(mut: str, s: SysState, badRun: RunSpec): bool = and {
    mut == "unbound_claims_root",
    s.collEpoch < MAX_COLL_EPOCH,
    UNPROVED_RUNS.contains(badRun),
    s.pendingJobs.length() < TREE_CAP,
  }

  pure def applyInjectUnboundLeaf(s: SysState, badRun: RunSpec): SysState = {
    val bindReg = if (s.pendingJobs.length() == 0) s.currentRegRoot else s.stagedRegRoot
    val job: PendingJob = {
      isFgn: false,
      runSpec: badRun,
      fgnSpec: DUMMY_FGN,
      dagDepth: 0,
      hasReceipt: true,
    }
    {
      ...s,
      pendingJobs: s.pendingJobs.append(job),
      stagedRegRoot: bindReg,
    }
  }

  // Empty epoch closes with 0 ledger writes (W-1, LDG-006)
  pure def canCloseEmptyEpoch(s: SysState): bool = and {
    s.collEpoch < MAX_COLL_EPOCH,
    s.pendingJobs.length() == 0,
    not(s.collEpochsWithRoot.contains(s.collEpoch)),
  }

  pure def applyCloseEmptyEpoch(s: SysState): SysState = {
    ...s,
    collEpoch: s.collEpoch + 1,
    stagedRegRoot: s.currentRegRoot,
  }

  // SherlockLedger.acceptCollection(regPub, claimsRoot) at epoch close
  pure def canAcceptCollection(mut: str, regMode: str, s: SysState, regPub: int): bool = {
    val leaves = pendingLeaves(mut, s.pendingJobs)
    val rootVal = treeRoot4(leaves)
    and {
      s.collEpoch < MAX_COLL_EPOCH,
      s.pendingJobs.length() > 0,
      mut == "no_reg_check" or regPub == s.stagedRegRoot,
      regRootAccepted(mut, regMode, s.currentRegRoot, s.prevRegRoot, regPub),
      not(s.claimRoots.contains(rootVal)),
    }
  }

  pure def applyAcceptCollection(mut: str, s: SysState, regPub: int, publishDa: bool): SysState = {
    val leaves = pendingLeaves(mut, s.pendingJobs)
    val rootVal = treeRoot4(leaves)
    val honestRuns = pendingHonestRuns(s.pendingJobs)
    val honestFgns = pendingHonestFgns(s.pendingJobs)
    val usedKeys = pendingFgnKeys(s.pendingJobs)
    val maxD = pendingMaxDepth(s.pendingJobs)
    val recPairs = pendingReceiptPairs(rootVal, s.pendingJobs)
    val rec: AcceptedColl = {
      rootVal: rootVal,
      epochNum: s.collEpoch,
      regNamed: regPub,
      regCurrentAtAccept: s.currentRegRoot,
      regPrevAtAccept: s.prevRegRoot,
      fgnKeysUsed: usedKeys,
    }
    {
      ...s,
      collEpoch: s.collEpoch + 1,
      pendingJobs: EMPTY_JOBS,
      stagedRegRoot: s.currentRegRoot,
      claimRoots: s.claimRoots.union(Set(rootVal)),
      rootLeaves: s.rootLeaves.put(rootVal, leaves),
      rootEpochMap: s.rootEpochMap.put(rootVal, s.collEpoch),
      honestRootRuns: s.honestRootRuns.put(rootVal, honestRuns),
      honestRootFgns: s.honestRootFgns.put(rootVal, honestFgns),
      rootMaxDepth: s.rootMaxDepth.put(rootVal, maxD),
      daPublishedRoots: if (publishDa) s.daPublishedRoots.union(Set(rootVal)) else s.daPublishedRoots,
      receiptPairs: s.receiptPairs.union(recPairs),
      ledgerWrites: s.ledgerWrites + 1,
      collEpochsWithRoot: s.collEpochsWithRoot.union(Set(s.collEpoch)),
      acceptedColls: s.acceptedColls.append(rec),
    }
  }

  // Operator attempts acceptCollection with stale stagedRegRoot and is refused on chain;
  // or re-proves pending jobs under currentRegRoot if all foreign keys remain admitted.
  pure def canRecordStaleRefusal(mut: str, regMode: str, s: SysState): bool = and {
    s.collEpoch < MAX_COLL_EPOCH,
    s.pendingJobs.length() > 0,
    not(regRootAccepted(mut, regMode, s.currentRegRoot, s.prevRegRoot, s.stagedRegRoot)),
  }

  pure def applyRecordStaleRefusal(s: SysState): SysState = {
    ...s,
    collRefusals: s.collRefusals + 1,
  }

  pure def canReprovePendingUnderCurrentReg(s: SysState): bool = and {
    s.pendingJobs.length() > 0,
    s.stagedRegRoot != s.currentRegRoot,
    pendingFgnKeys(s.pendingJobs).subseteq(getIntSetOr(s.regTable, s.currentRegRoot)),
  }

  pure def applyReprovePendingUnderCurrentReg(s: SysState): SysState = {
    ...s,
    stagedRegRoot: s.currentRegRoot,
  }

  // User-submitted, user-paid SherlockLedger.acceptRun(c_p, input_pub, outcome_pub)
  pure def canUserAcceptRun(s: SysState, r: RunSpec): bool = and {
    PROVED_RUNS.contains(r),
    not(s.runsMap.keys().contains(runClaimPub(r))),
  }

  pure def applyUserAcceptRun(s: SysState, r: RunSpec): SysState = {
    val k = runClaimPub(r)
    {
      ...s,
      runsMap: s.runsMap.put(k, r.outp),
      ledgerWrites: s.ledgerWrites + 1,
      userPaidRuns: s.userPaidRuns + 1,
    }
  }

  // Later off-chain publication of a withheld claim list
  pure def canPublishDa(s: SysState, rootVal: int): bool = and {
    s.claimRoots.contains(rootVal),
    not(s.daPublishedRoots.contains(rootVal)),
  }

  pure def applyPublishDa(s: SysState, rootVal: int): SysState = {
    ...s,
    daPublishedRoots: s.daPublishedRoots.union(Set(rootVal)),
  }

  // Compact consumer: useRunClaim(c_p, input_pub, outcome_pub) with witness (sibs, bits)
  pure def canUseRunClaim(mut: str, s: SysState, cid: int, r: RunSpec, p: MerklePath): bool = {
    val claimPub = runClaimPub(r)
    if (not(CONSUMERS.contains(cid)) or not(pathWellFormed(p))) false
    else {
      val computedRoot = rootFromPath(claimPub, p)
      val nul = collectionNullifier(mut, computedRoot, p.bits, claimPub)
      and {
        mut == "no_root_read" or s.claimRoots.contains(computedRoot),
        not(getIntSetOr(s.consumerUsed, cid).contains(nul)),
      }
    }
  }

  pure def applyUseRunClaim(mut: str, s: SysState, cid: int, r: RunSpec, p: MerklePath): SysState = {
    val claimPub = runClaimPub(r)
    val computedRoot = rootFromPath(claimPub, p)
    val nul = collectionNullifier(mut, computedRoot, p.bits, claimPub)
    val depthVal = getIntOr(s.rootMaxDepth, computedRoot, 0)
    val extraDescend = if (mut == "descend_k_open") ceilDivPos(depthVal, K_OPEN) else 0
    val extraOpen = if (mut == "per_claim_open") 1 else 0
    val uRec: UseRecord = {
      cid: cid,
      usedAsFgn: false,
      runSpec: r,
      fpubVal: 0,
      fromRuns: false,
      rootUsed: computedRoot,
    }
    {
      ...s,
      consumerUsed: s.consumerUsed.put(cid, getIntSetOr(s.consumerUsed, cid).union(Set(nul))),
      ledgerWrites: s.ledgerWrites + extraOpen + extraDescend,
      descendWrites: s.descendWrites + extraDescend,
      useLog: s.useLog.append(uRec),
    }
  }

  // Compact consumer: useSettledRun(c_p, input_pub, outcome_pub) reading SherlockLedger.runAccepted
  pure def canUseSettledRun(s: SysState, cid: int, r: RunSpec): bool = {
    val claimPub = runClaimPub(r)
    val nul = settledRunNullifier(claimPub)
    and {
      CONSUMERS.contains(cid),
      s.runsMap.keys().contains(claimPub),
      getIntOr(s.runsMap, claimPub, -1) == r.outp,
      not(getIntSetOr(s.consumerUsed, cid).contains(nul)),
    }
  }

  pure def applyUseSettledRun(s: SysState, cid: int, r: RunSpec): SysState = {
    val claimPub = runClaimPub(r)
    val nul = settledRunNullifier(claimPub)
    val uRec: UseRecord = {
      cid: cid,
      usedAsFgn: false,
      runSpec: r,
      fpubVal: 0,
      fromRuns: true,
      rootUsed: -1,
    }
    {
      ...s,
      consumerUsed: s.consumerUsed.put(cid, getIntSetOr(s.consumerUsed, cid).union(Set(nul))),
      useLog: s.useLog.append(uRec),
    }
  }

  // Compact consumer: useForeignClaim(fclaim_pub) with witness (sibs, bits)
  pure def canUseForeignClaim(mut: str, s: SysState, cid: int, fpubArg: int, p: MerklePath): bool = {
    if (not(CONSUMERS.contains(cid)) or fpubArg < 0 or not(pathWellFormed(p))) false
    else {
      val leaf = fgnLeafOf(mut, fpubArg)
      val computedRoot = rootFromPath(leaf, p)
      val nul = collectionNullifier(mut, computedRoot, p.bits, leaf)
      and {
        mut == "no_root_read" or s.claimRoots.contains(computedRoot),
        not(getIntSetOr(s.consumerUsed, cid).contains(nul)),
      }
    }
  }

  pure def applyUseForeignClaim(mut: str, s: SysState, cid: int, fpubArg: int, p: MerklePath): SysState = {
    val leaf = fgnLeafOf(mut, fpubArg)
    val computedRoot = rootFromPath(leaf, p)
    val nul = collectionNullifier(mut, computedRoot, p.bits, leaf)
    val depthVal = getIntOr(s.rootMaxDepth, computedRoot, 0)
    val extraDescend = if (mut == "descend_k_open") ceilDivPos(depthVal, K_OPEN) else 0
    val extraOpen = if (mut == "per_claim_open") 1 else 0
    val uRec: UseRecord = {
      cid: cid,
      usedAsFgn: true,
      runSpec: DUMMY_RUN,
      fpubVal: fpubArg,
      fromRuns: false,
      rootUsed: computedRoot,
    }
    {
      ...s,
      consumerUsed: s.consumerUsed.put(cid, getIntSetOr(s.consumerUsed, cid).union(Set(nul))),
      ledgerWrites: s.ledgerWrites + extraOpen + extraDescend,
      descendWrites: s.descendWrites + extraDescend,
      useLog: s.useLog.append(uRec),
    }
  }

  // Registry key change staging (or immediate write under mutant per_key_reg_write)
  pure def canStageKeyChange(s: SysState, keyId: int, isAdd: bool): bool = and {
    s.regEpoch < MAX_REG_EPOCH,
    ALL_KEYS.contains(keyId),
    isAdd != s.stagedRegKeys.contains(keyId),
  }

  pure def applyStageKeyChange(mut: str, s: SysState, keyId: int, isAdd: bool): SysState = {
    val updatedKeys = if (isAdd) s.stagedRegKeys.union(Set(keyId)) else s.stagedRegKeys.exclude(Set(keyId))
    if (mut == "per_key_reg_write") {
      val newId = s.nextRegId
      val prevWrites = getIntOr(s.regWritesPerEp, s.regEpoch, 0)
      {
        ...s,
        prevRegRoot: s.currentRegRoot,
        currentRegRoot: newId,
        regTable: s.regTable.put(newId, updatedKeys),
        nextRegId: newId + 1,
        stagedRegKeys: updatedKeys,
        stagedKeyOps: s.stagedKeyOps + 1,
        regWritesPerEp: s.regWritesPerEp.put(s.regEpoch, prevWrites + 1),
        ledgerWrites: s.ledgerWrites + 1,
        regEpochsWithWrite: s.regEpochsWithWrite.union(Set(s.regEpoch)),
      }
    } else {
      {
        ...s,
        stagedRegKeys: updatedKeys,
        stagedKeyOps: s.stagedKeyOps + 1,
      }
    }
  }

  // Registry epoch close: at most 1 reg_root update on SherlockLedger if membership changed; 0 if unchanged
  pure def canCloseRegEpoch(s: SysState): bool =
    s.regEpoch < MAX_REG_EPOCH

  pure def applyCloseRegEpoch(mut: str, s: SysState): SysState = {
    val curSet = getIntSetOr(s.regTable, s.currentRegRoot)
    val netChanged = (s.stagedRegKeys != curSet)
    val symDiffCount =
      s.stagedRegKeys.exclude(curSet).size() + curSet.exclude(s.stagedRegKeys).size()
    if (mut != "per_key_reg_write" and netChanged) {
      val newId = s.nextRegId
      {
        ...s,
        regEpoch: s.regEpoch + 1,
        prevRegRoot: s.currentRegRoot,
        currentRegRoot: newId,
        regTable: s.regTable.put(newId, s.stagedRegKeys),
        nextRegId: newId + 1,
        stagedKeyOps: 0,
        regWritesPerEp: s.regWritesPerEp.put(s.regEpoch, 1),
        batchedTwoKeyEpReached: s.batchedTwoKeyEpReached or (symDiffCount >= 2),
        ledgerWrites: s.ledgerWrites + 1,
        regEpochsWithWrite: s.regEpochsWithWrite.union(Set(s.regEpoch)),
      }
    } else {
      {
        ...s,
        regEpoch: s.regEpoch + 1,
        stagedKeyOps: 0,
      }
    }
  }

  // Planner actions (W-5: Compact calls on user contracts vs general Sherlock runs)
  pure def applySubmitCompactPlan(mut: str, s: SysState, effectWords: int, ttlSec: int): SysState = {
    val p = planCompactCall(mut, effectWords, ttlSec)
    {
      ...s,
      callPlans: s.callPlans.union(Set(p)),
      userContractAnchorWrites: s.userContractAnchorWrites + p.anchorWrites,
    }
  }

  pure def applySubmitGeneralRunPlan(s: SysState, ttlSec: int): SysState = {
    val p = planGeneralRun(ttlSec)
    {
      ...s,
      callPlans: s.callPlans.union(Set(p)),
    }
  }

  // --------------------------------------------------------------------------
  // 5. Pure Invariant Predicates (reusable in exhaustive tests)
  // --------------------------------------------------------------------------

  pure def checkWritesBounded(s: SysState): bool =
    s.ledgerWrites <= s.collEpochsWithRoot.size() + s.userPaidRuns + s.regEpochsWithWrite.size()

  pure def checkUsedOnlyMembers(s: SysState): bool =
    s.useLog.indices().forall(i => {
      val u = s.useLog.nth(i)
      if (u.fromRuns) and {
        not(u.usedAsFgn),
        PROVED_RUNS.contains(u.runSpec),
        s.runsMap.keys().contains(runClaimPub(u.runSpec)),
        getIntOr(s.runsMap, runClaimPub(u.runSpec), -1) == u.runSpec.outp,
      } else and {
        s.claimRoots.contains(u.rootUsed),
        if (u.usedAsFgn)
          VALID_FGNS.exists(f => f.fpub == u.fpubVal and getFgnSetOr(s.honestRootFgns, u.rootUsed).contains(f))
        else
          (PROVED_RUNS.contains(u.runSpec) and getRunSetOr(s.honestRootRuns, u.rootUsed).contains(u.runSpec)),
      }
    })

  pure def countConsumerUses(logList: List[UseRecord], cid: int, asFgn: bool, claimId: int): int =
    logList.foldl(0, (acc, u) => {
      val uId = if (u.usedAsFgn) u.fpubVal else runClaimPub(u.runSpec)
      if (u.cid == cid and u.usedAsFgn == asFgn and uId == claimId) acc + 1 else acc
    })

  pure def checkUseOnce(s: SysState): bool =
    s.useLog.indices().forall(i => {
      val u = s.useLog.nth(i)
      val uId = if (u.usedAsFgn) u.fpubVal else runClaimPub(u.runSpec)
      countConsumerUses(s.useLog, u.cid, u.usedAsFgn, uId) <= 1
    })

  pure def checkKindSeparated(s: SysState): bool =
    s.useLog.indices().forall(i => {
      val u = s.useLog.nth(i)
      if (u.usedAsFgn)
        VALID_FGNS.exists(f => f.fpub == u.fpubVal)
      else
        PROVED_RUNS.contains(u.runSpec)
    })

  pure def checkNoUseBeforeClose(s: SysState): bool =
    s.useLog.indices().forall(i => {
      val u = s.useLog.nth(i)
      u.fromRuns or (
        s.claimRoots.contains(u.rootUsed) and
        s.rootEpochMap.keys().contains(u.rootUsed) and
        getIntOr(s.rootEpochMap, u.rootUsed, MAX_COLL_EPOCH + 1) < s.collEpoch
      )
    })

  pure def checkChunkCap(s: SysState): bool =
    s.callPlans.forall(p => and {
      p.chunks <= p.kMax,
      p.chunks * TEN_X_PROVE_SEC <= p.ttlSec,
      (MEASURED_SIZES.contains(p.effectWords) or p.effectWords == STATIC_MAX_WORDS or p.isGeneralRun)
        implies (p.chunks == 1 and not(p.refused) and p.anchorWrites == 0),
    })

  pure def checkNoTruncation(s: SysState): bool =
    s.callPlans.forall(p =>
      (not(p.refused) implies p.coveredWords >= p.effectWords) and
      (p.refused iff p.chunks == 0)
    )

  pure def checkRegWritesPerEpoch(s: SysState): bool =
    s.regWritesPerEp.keys().forall(ep =>
      getIntOr(s.regWritesPerEp, ep, 0) <= 1 and
      (not(s.regEpochsWithWrite.contains(ep)) implies getIntOr(s.regWritesPerEp, ep, 0) == 0)
    )

  pure def checkStaleRegRefusedStrict(s: SysState): bool =
    s.acceptedColls.indices().forall(i => {
      val c = s.acceptedColls.nth(i)
      and {
        c.regNamed == c.regCurrentAtAccept,
        s.regTable.keys().contains(c.regNamed),
        c.fgnKeysUsed.subseteq(getIntSetOr(s.regTable, c.regNamed)),
      }
    })

  pure def checkGraceRegRefused(s: SysState): bool =
    s.acceptedColls.indices().forall(i => {
      val c = s.acceptedColls.nth(i)
      and {
        c.regNamed == c.regCurrentAtAccept or c.regNamed == c.regPrevAtAccept,
        s.regTable.keys().contains(c.regNamed),
        c.fgnKeysUsed.subseteq(getIntSetOr(s.regTable, c.regNamed)),
      }
    })

  pure def checkNoDescendCalls(s: SysState): bool =
    s.descendWrites == 0

  // --------------------------------------------------------------------------
  // 6. State Variable, Named Invariants, and Non-Vacuity Witnesses
  // --------------------------------------------------------------------------

  var st: SysState

  val invWritesBounded: bool = checkWritesBounded(st)
  val invUsedOnlyMembers: bool = checkUsedOnlyMembers(st)
  val invUseOnce: bool = checkUseOnce(st)
  val invKindSeparated: bool = checkKindSeparated(st)
  val invNoUseBeforeClose: bool = checkNoUseBeforeClose(st)
  val invChunkCap: bool = checkChunkCap(st)
  val invNoTruncation: bool = checkNoTruncation(st)
  val invRegWritesPerEpoch: bool = checkRegWritesPerEpoch(st)
  val invStaleRegRefused: bool = checkStaleRegRefusedStrict(st)
  val invGraceRegRefused: bool = checkGraceRegRefused(st)
  val invNoDescendCalls: bool = checkNoDescendCalls(st)

  val invAll: bool = and {
    invWritesBounded,
    invUsedOnlyMembers,
    invUseOnce,
    invKindSeparated,
    invNoUseBeforeClose,
    invChunkCap,
    invNoTruncation,
    invRegWritesPerEpoch,
    if (REG_MODE == "grace_one") invGraceRegRefused else invStaleRegRefused,
    invNoDescendCalls,
  }

  // Positive target predicates for reachability
  val seenUsedRunClaim: bool =
    st.useLog.indices().exists(i => not(st.useLog.nth(i).usedAsFgn) and not(st.useLog.nth(i).fromRuns))

  val seenUsedForeignClaim: bool =
    st.useLog.indices().exists(i => st.useLog.nth(i).usedAsFgn)

  val seenChunkedCall: bool =
    st.callPlans.exists(p => not(p.refused) and p.chunks == 2)

  val seenRefusedPlan: bool =
    st.callPlans.exists(p => p.refused)

  val seenRegTwoKeyChanges: bool =
    st.batchedTwoKeyEpReached

  val seenCollectionUnderNewReg: bool =
    st.acceptedColls.indices().exists(i => st.acceptedColls.nth(i).regNamed > 0)

  val seenFallbackSettledRunUsed: bool =
    st.useLog.indices().exists(i => {
      val u = st.useLog.nth(i)
      u.fromRuns and st.claimRoots.exists(r =>
        not(st.daPublishedRoots.contains(r)) and getRunSetOr(st.honestRootRuns, r).contains(u.runSpec)
      )
    })

  val seenDeepDagClaimUsed: bool =
    st.useLog.indices().exists(i => {
      val u = st.useLog.nth(i)
      not(u.fromRuns) and st.rootMaxDepth.keys().contains(u.rootUsed) and getIntOr(st.rootMaxDepth, u.rootUsed, 0) >= 3
    })

  // Negated reach* invariants: each MUST be violated (exit 1) in the sound instance
  val reachUsedRunClaim: bool = not(seenUsedRunClaim)
  val reachUsedForeignClaim: bool = not(seenUsedForeignClaim)
  val reachChunkedCall: bool = not(seenChunkedCall)
  val reachRefusedPlan: bool = not(seenRefusedPlan)
  val reachRegTwoKeyChanges: bool = not(seenRegTwoKeyChanges)
  val reachCollectionUnderNewReg: bool = not(seenCollectionUnderNewReg)
  val reachFallbackSettledRunUsed: bool = not(seenFallbackSettledRunUsed)
  val reachDeepDagClaimUsed: bool = not(seenDeepDagClaimUsed)

  // --------------------------------------------------------------------------
  // 7. Guarded Actions & Nondeterministic Environment Step
  // --------------------------------------------------------------------------

  action init: bool = all {
    st' = initState,
  }

  action proveRunJob(r: RunSpec, depth: int, giveReceipt: bool): bool = all {
    canProveRunJob(MUT, st, r, depth),
    st' = applyProveRunJob(MUT, st, r, depth, giveReceipt),
  }

  action proveFgnJob(f: FgnSpec, depth: int, giveReceipt: bool): bool = all {
    canProveFgnJob(st, f, depth),
    st' = applyProveFgnJob(st, f, depth, giveReceipt),
  }

  action injectUnboundLeaf(badRun: RunSpec): bool = all {
    canInjectUnboundLeaf(MUT, st, badRun),
    st' = applyInjectUnboundLeaf(st, badRun),
  }

  action closeEmptyCollEpoch: bool = all {
    canCloseEmptyEpoch(st),
    st' = applyCloseEmptyEpoch(st),
  }

  action acceptCollectionEpoch(regPub: int, publishDa: bool): bool = all {
    canAcceptCollection(MUT, REG_MODE, st, regPub),
    st' = applyAcceptCollection(MUT, st, regPub, publishDa),
  }

  action recordStaleRefusal: bool = all {
    canRecordStaleRefusal(MUT, REG_MODE, st),
    st' = applyRecordStaleRefusal(st),
  }

  action reprovePendingUnderCurrentReg: bool = all {
    canReprovePendingUnderCurrentReg(st),
    st' = applyReprovePendingUnderCurrentReg(st),
  }

  action userAcceptRun(r: RunSpec): bool = all {
    canUserAcceptRun(st, r),
    st' = applyUserAcceptRun(st, r),
  }

  action publishDa(rootVal: int): bool = all {
    canPublishDa(st, rootVal),
    st' = applyPublishDa(st, rootVal),
  }

  action consumerUseRunFromRoot(cid: int, r: RunSpec, p: MerklePath): bool = all {
    canUseRunClaim(MUT, st, cid, r, p),
    st' = applyUseRunClaim(MUT, st, cid, r, p),
  }

  action consumerUseSettledRun(cid: int, r: RunSpec): bool = all {
    canUseSettledRun(st, cid, r),
    st' = applyUseSettledRun(st, cid, r),
  }

  action consumerUseForeignFromRoot(cid: int, fpubArg: int, p: MerklePath): bool = all {
    canUseForeignClaim(MUT, st, cid, fpubArg, p),
    st' = applyUseForeignClaim(MUT, st, cid, fpubArg, p),
  }

  action stageKeyChange(keyId: int, isAdd: bool): bool = all {
    canStageKeyChange(st, keyId, isAdd),
    st' = applyStageKeyChange(MUT, st, keyId, isAdd),
  }

  action closeRegEpoch: bool = all {
    canCloseRegEpoch(st),
    st' = applyCloseRegEpoch(MUT, st),
  }

  action submitCompactPlan(effectWords: int, ttlSec: int): bool = all {
    st' = applySubmitCompactPlan(MUT, st, effectWords, ttlSec),
  }

  action submitGeneralRunPlan(ttlSec: int): bool = all {
    st' = applySubmitGeneralRunPlan(st, ttlSec),
  }

  pure def firstPendingRunOrDummy(jobs: List[PendingJob]): RunSpec =
    if (jobs.length() > 0 and not(jobs.nth(0).isFgn)) jobs.nth(0).runSpec else RUN_A

  // Honest off-chain path availability check: DA published, or receipt held for that slot
  pure def slotPathAvailable(s: SysState, rootVal: int, slotIdx: int): bool =
    s.daPublishedRoots.contains(rootVal) or s.receiptPairs.contains((rootVal, slotIdx))

  pure def availRunTriples(s: SysState): Set[(int, int, RunSpec)] =
    s.claimRoots.fold(EMPTY_RUN_TRIPLE_SET, (accR, rootVal) => {
      val leaves = getLeavesOr(s.rootLeaves, rootVal)
      ALL_SLOTS.fold(accR, (accS, slotIdx) =>
        if (slotIdx < leaves.length() and slotPathAvailable(s, rootVal, slotIdx))
          PROVED_RUNS.union(UNPROVED_RUNS).fold(accS, (accC, r) =>
            if (runClaimPub(r) == leaves.nth(slotIdx)) accC.union(Set((rootVal, slotIdx, r))) else accC
          )
        else
          accS
      )
    })

  pure def availFgnTriples(mut: str, s: SysState): Set[(int, int, int)] =
    s.claimRoots.fold(EMPTY_FGN_TRIPLE_SET, (accR, rootVal) => {
      val leaves = getLeavesOr(s.rootLeaves, rootVal)
      ALL_SLOTS.fold(accR, (accS, slotIdx) =>
        if (slotIdx < leaves.length() and slotPathAvailable(s, rootVal, slotIdx))
          VALID_FGNS.fold(accS, (accC, f) =>
            if (fgnLeafOf(mut, f.fpub) == leaves.nth(slotIdx)) accC.union(Set((rootVal, slotIdx, f.fpub))) else accC
          )
        else
          accS
      )
    })

  action step: bool = any {
    // 1. Operator proves a run job (including deep DAGs and duplicate diamond leaves)
    {
      nondet r = PROVED_RUNS.oneOf()
      nondet d = DAG_DEPTHS.oneOf()
      nondet rec = Set(true, false).oneOf()
      proveRunJob(r, d, rec)
    },
    // 2. Operator proves a foreign claim job
    {
      nondet f = VALID_FGNS.oneOf()
      nondet d = DAG_DEPTHS.oneOf()
      nondet rec = Set(true, false).oneOf()
      proveFgnJob(f, d, rec)
    },
    // 3. Mutant-only: unconstrained L injects an unproved/ADM leaf
    {
      nondet badRun = UNPROVED_RUNS.oneOf()
      injectUnboundLeaf(badRun)
    },
    // 4. Operator settles open epoch via acceptCollection (or skips empty epoch)
    {
      nondet pub = Set(true, false).oneOf()
      acceptCollectionEpoch(st.stagedRegRoot, pub)
    },
    closeEmptyCollEpoch,
    recordStaleRefusal,
    reprovePendingUnderCurrentReg,
    // 5. User submits and pays for acceptRun directly (or as DA fallback)
    {
      nondet r = PROVED_RUNS.oneOf()
      userAcceptRun(r)
    },
    // 6. DA store publishes a previously withheld list
    {
      nondet rootVal = (if (st.claimRoots.size() > 0) st.claimRoots else Set(0)).oneOf()
      publishDa(rootVal)
    },
    // 7. Consumer uses a run claim from an accepted collection (when path is available via DA or receipt)
    {
      nondet cid = CONSUMERS.oneOf()
      nondet tr = (if (availRunTriples(st).size() > 0) availRunTriples(st) else Set((0, 0, RUN_A))).oneOf()
      all {
        availRunTriples(st).size() > 0,
        consumerUseRunFromRoot(cid, tr._3, treePath4(getLeavesOr(st.rootLeaves, tr._1), tr._2)),
      }
    },
    // 8. Consumer uses a foreign claim from an accepted collection (when path is available)
    {
      nondet cid = CONSUMERS.oneOf()
      nondet tr = (if (availFgnTriples(MUT, st).size() > 0) availFgnTriples(MUT, st) else Set((0, 0, 1))).oneOf()
      all {
        availFgnTriples(MUT, st).size() > 0,
        consumerUseForeignFromRoot(cid, tr._3, treePath4(getLeavesOr(st.rootLeaves, tr._1), tr._2)),
      }
    },
    // 9. Consumer uses a user-settled run from runsMap (same nullifier)
    {
      nondet cid = CONSUMERS.oneOf()
      nondet r = PROVED_RUNS.oneOf()
      consumerUseSettledRun(cid, r)
    },
    // 10. Registry admin stages key additions/revocations and closes registry epochs
    {
      nondet k = ALL_KEYS.oneOf()
      nondet addFlag = Set(true, false).oneOf()
      stageKeyChange(k, addFlag)
    },
    closeRegEpoch,
    // 11. Compact call planner and general Sherlock run planner
    {
      nondet w = CANDIDATE_CALL_SIZES.oneOf()
      submitCompactPlan(w, DEFAULT_TTL_SEC)
    },
    submitGeneralRunPlan(DEFAULT_TTL_SEC),
    // 12. Attacker: forged/unaccepted-root or pre-close run claim attempt (blocked in sound; caught in no_root_read)
    {
      nondet cid = CONSUMERS.oneOf()
      nondet badRun = UNPROVED_RUNS.oneOf()
      consumerUseRunFromRoot(cid, badRun, { sibs: List(0, 0), bits: List(false, false) })
    },
    {
      nondet cid = CONSUMERS.oneOf()
      val hasOpenRun = if (st.pendingJobs.length() > 0) not(st.pendingJobs.nth(0).isFgn) else false
      val openRun = firstPendingRunOrDummy(st.pendingJobs)
      all {
        hasOpenRun,
        consumerUseRunFromRoot(cid, openRun, treePath4(pendingLeaves(MUT, st.pendingJobs), 0)),
      }
    },
    // 13. Attacker: cross-kind / empty-slot-0 foreign claim attempt using real slot leaf & path
    //     (blocked in sound; caught in no_kind_tag)
    {
      nondet cid = CONSUMERS.oneOf()
      nondet slotIdx = ALL_SLOTS.oneOf()
      nondet rootVal = (if (st.claimRoots.size() > 0) st.claimRoots else Set(0)).oneOf()
      val leaves = getLeavesOr(st.rootLeaves, rootVal)
      val padded = padLeaves4(leaves)
      all {
        st.claimRoots.contains(rootVal),
        consumerUseForeignFromRoot(cid, padded.nth(slotIdx), treePath4(leaves, slotIdx)),
      }
    },
    // 14. Attacker: submit acceptCollection under stale/foreign reg_pub = 99 or 0
    //     (blocked in sound; caught in no_reg_check)
    {
      nondet badReg = Set(0, 99).oneOf()
      nondet pub = Set(true, false).oneOf()
      all {
        badReg != st.currentRegRoot,
        acceptCollectionEpoch(badReg, pub),
      }
    },
  }
}

// ----------------------------------------------------------------------------
// Concrete Sound Instances (strict as-built rule, plus open N-8 grace option)
// ----------------------------------------------------------------------------

module t20_batched {
  import t20_batched_base(MUT = "none", REG_MODE = "strict").*
}

module t20_batched_grace {
  import t20_batched_base(MUT = "none", REG_MODE = "grace_one").*
}

// ----------------------------------------------------------------------------
// Concrete Mutant Instances (every mutant MUST violate invAll under quint run)
// ----------------------------------------------------------------------------

module t20_batched_m_per_job_accept_run {
  import t20_batched_base(MUT = "per_job_accept_run", REG_MODE = "strict").*
}

module t20_batched_m_per_claim_open {
  import t20_batched_base(MUT = "per_claim_open", REG_MODE = "strict").*
}

module t20_batched_m_nullifier_keyed_by_root {
  import t20_batched_base(MUT = "nullifier_keyed_by_root", REG_MODE = "strict").*
}

module t20_batched_m_no_root_read {
  import t20_batched_base(MUT = "no_root_read", REG_MODE = "strict").*
}

module t20_batched_m_no_kind_tag {
  import t20_batched_base(MUT = "no_kind_tag", REG_MODE = "strict").*
}

module t20_batched_m_chunk_no_cap {
  import t20_batched_base(MUT = "chunk_no_cap", REG_MODE = "strict").*
}

module t20_batched_m_chunk_truncate {
  import t20_batched_base(MUT = "chunk_truncate", REG_MODE = "strict").*
}

module t20_batched_m_per_key_reg_write {
  import t20_batched_base(MUT = "per_key_reg_write", REG_MODE = "strict").*
}

module t20_batched_m_no_reg_check {
  import t20_batched_base(MUT = "no_reg_check", REG_MODE = "strict").*
}

module t20_batched_m_descend_k_open {
  import t20_batched_base(MUT = "descend_k_open", REG_MODE = "strict").*
}

module t20_batched_m_nullifier_keyed_by_pos {
  import t20_batched_base(MUT = "nullifier_keyed_by_pos", REG_MODE = "strict").*
}

module t20_batched_m_unbound_claims_root {
  import t20_batched_base(MUT = "unbound_claims_root", REG_MODE = "strict").*
}
