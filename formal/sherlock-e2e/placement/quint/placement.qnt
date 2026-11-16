// -*- mode: Bluespec; -*-
// Independent Quint specification for bounded context Placement (settlement edge) (lane e2e-placement).
// Sources:
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-helper-machines/spec.md (HLP-001..008)
//   - openspec/changes/sherlock-processor-final-20261111/specs/sherlock-zero-knowledge/spec.md (adopted ZK-009)
//   - openspec/changes/sherlock-e2e-prototype-20261115/architecture/DDD.md (Placement rows in sections 1-4, 6-8)

module placement {
  // =========================================================================
  // Constants (numeric bounds, call-vector lane indices, and fleet bounds)
  // =========================================================================

  // HLP-003: exact count of public statement words returned with WrapOutput (c_p, input_pub, outcome_pub, claim_pub => 4).
  const PUBLIC_STATEMENT_WORDS: int
  // DDD section 2 & 3 (shared kernel Wrap <-> Settlement): wrap call vector length (1,026 lanes).
  const CALL_VECTOR_LANES: int
  // DDD section 2: call-vector lane index carrying anchor c_p (lane 10).
  const LANE_ANCHOR_CP: int
  // HLP-008 Scenario & DDD section 2: first call-vector lane carrying claim_pub (lane 20).
  const LANE_CLAIM_PUB_A: int
  // HLP-008 Scenario & DDD section 2: second call-vector lane carrying claim_pub (lane 34).
  const LANE_CLAIM_PUB_B: int
  // HLP-008 Scenario & DDD section 2: call-vector lane carrying outcome_pub (lane 39).
  const LANE_OUTCOME_PUB: int
  // HLP-005: SDK commands that send proving artifacts to unowned hosts (0).
  const SDK_OUTSOURCE_COMMANDS: int
  // HLP-005: SDK options that send proving artifacts to unowned hosts (0).
  const SDK_OUTSOURCE_OPTIONS: int
  // HLP-007 & DDD section 1: maximum number of wallet-holding settlement machines (1 = muninn / pinned wallet host).
  const MAX_WALLET_BOXES: int
  // Adopted ZK-009: maximum ContractCalls settled by one wrap proof (1).
  const MAX_CALLS_PER_WRAP: int
  // Number of user-owned remote proving boxes alongside the wallet/check host muninn (1..3; spec main = 2: huginn, odin).
  const OWNED_PROVER_BOX_COUNT: int
  // Mutant selector: 0 = honest specification; 1..11 = individual guard mutants.
  const MUTANT_ID: int

  // =========================================================================
  // Host, job, and mutant identifiers
  // =========================================================================

  pure val BOX_MUNINN: int = 1
  pure val BOX_HUGINN: int = 2
  pure val BOX_ODIN: int = 3
  pure val BOX_EXTRA_OWNED: int = 4
  pure val HOST_UNOWNED_HELPER: int = 99

  pure val JOB_IDS: Set[int] = Set(12, 13)

  pure val MUT_NONE: int = 0
  pure val MUT_HLP001_KEEP_UNVERIFIED: int = 1
  pure val MUT_HLP002_SUBMIT_REFUSED_WRAP: int = 2
  pure val MUT_HLP003_SHIP_ACCEPT_BUNDLE: int = 3
  pure val MUT_HLP004_LEAK_PRIVATE_IN_RESULT: int = 4
  pure val MUT_HLP005_ALLOW_UNOWNED_PROVER: int = 5
  pure val MUT_HLP007_PROVING_BOX_SUBMITS_TX: int = 6
  pure val MUT_HLP008_ACCEPT_BAD_SHA256: int = 7
  pure val MUT_HLP008_ACCEPT_OTHER_JOB_VEC: int = 8
  pure val MUT_ZK009_REMOTE_UNMASKED_MSM: int = 9
  pure val MUT_ZK009_REUSE_OR_HELPER_SLOT: int = 10
  pure val MUT_ZK009_MULTI_CALL_WRAP: int = 11

  // =========================================================================
  // Domain types
  // =========================================================================

  type ResultDisposition =
    | ResNone
    | ResReturnedUnchecked
    | ResKept
    | ResDropped

  type BoxReturnFault =
    | FaultNone
    | FaultBadSha
    | FaultWrongJobVec
    | FaultInvalidProof

  // Summary of the 4 public binding lanes in the 1,026-lane wrap call vector (lanes 10, 20, 34, 39).
  type CallVecSummary = {
    cpLane10: int,
    claimLane20: int,
    claimLane34: int,
    outcomeLane39: int
  }

  // Published language StatementPublics returned by the argument builder on the proving box (HLP-003).
  type StatementPublicsRec = {
    cp: int,
    inputPub: int,
    outcomePub: int,
    claimPub: int
  }

  type JobRecord = {
    jobId: int,
    expectedVec: CallVecSummary,
    resultPresent: bool,
    fromBox: int,
    reportedSha256: int,
    actualSha256: int,
    returnedPublics: StatementPublicsRec,
    returnedPublicWordCount: int,
    returnedVec: CallVecSummary,
    proofCryptoValid: bool,
    argBuilderRanOnBox: bool,
    acceptBundleLeftBox: bool,
    privatePartInResult: bool,
    wordsMatchCallVecLanes: bool,
    verifierInvoked: bool,
    muninnChecked: bool,
    ledger9101VerifiedOk: bool,
    disposition: ResultDisposition,
    reproveRequested: bool,
    submittedToChain: bool
  }

  type PlacementState = {
    ownedRoster: Set[int],
    walletBoxes: Set[int],
    jobs: int -> JobRecord,

    // HLP-004 & HLP-005 Case A / unowned helper tracking
    privatePartLeftOwnedBoxes: bool,
    unownedHostConfigAccepted: bool,
    unownedArtifactsSent: Set[str],
    caseAConfigsAccepted: int,
    unownedConfigRefusals: int,

    // HLP-007 wallet placement & transaction submission tracking
    boxesThatSignedOrSubmitted: Set[int],
    walletlessTxRefusalsOnBus: Set[int],

    // Adopted ZK-009 helper, wrap MSM/NTT, blinding slot, and single-call wrap tracking
    helperProvedStarkOrL: bool,
    remoteUnmaskedWrapOp: bool,
    helperHoldsFullSpongeState: bool,
    blindingSlotReused: bool,
    helperMadeBlindingSlot: bool,
    aggregatorReceivedStarkProofs: bool,
    callsPerWrap: int,
    userBatchOptInValid: bool,
    localBlindedWrapsCompleted: int,
    zk009Refusals: int,

    lastRefusal: str
  }

  // =========================================================================
  // Observable state variables
  // =========================================================================

  var plc: PlacementState
  var lastOp: str

  // =========================================================================
  // Pure functions & helper predicates
  // =========================================================================

  pure def computeProverBoxes(proverCnt: int): Set[int] =
    if (proverCnt <= 1) Set(BOX_HUGINN)
    else if (proverCnt == 2) Set(BOX_HUGINN, BOX_ODIN)
    else Set(BOX_HUGINN, BOX_ODIN, BOX_EXTRA_OWNED)

  pure def computeOwnedRoster(proverCnt: int): Set[int] =
    Set(BOX_MUNINN).union(computeProverBoxes(proverCnt))

  pure def jobExpectedPublics(jid: int): StatementPublicsRec =
    if (jid == 12) { cp: 500, inputPub: 1012, outcomePub: 220, claimPub: 120 }
    else { cp: 500, inputPub: 1013, outcomePub: 230, claimPub: 130 }

  pure def jobExpectedVec(jid: int): CallVecSummary = {
    val pubs = jobExpectedPublics(jid)
    {
      cpLane10: pubs.cp,
      claimLane20: pubs.claimPub,
      claimLane34: pubs.claimPub,
      outcomeLane39: pubs.outcomePub
    }
  }

  // Checks internal consistency between StatementPublics and the 3 values projected into lanes 10, 20, 34, 39.
  pure def wordsConsistentWithVec(pubs: StatementPublicsRec, vec: CallVecSummary): bool =
    pubs.cp == vec.cpLane10 and
    pubs.claimPub == vec.claimLane20 and
    pubs.claimPub == vec.claimLane34 and
    pubs.outcomePub == vec.outcomeLane39

  pure def blankJobRecord(jid: int): JobRecord = {
    val expPubs = jobExpectedPublics(jid)
    val expVec = jobExpectedVec(jid)
    {
      jobId: jid,
      expectedVec: expVec,
      resultPresent: false,
      fromBox: BOX_HUGINN,
      reportedSha256: 0,
      actualSha256: 0,
      returnedPublics: expPubs,
      returnedPublicWordCount: PUBLIC_STATEMENT_WORDS,
      returnedVec: expVec,
      proofCryptoValid: false,
      argBuilderRanOnBox: true,
      acceptBundleLeftBox: false,
      privatePartInResult: false,
      wordsMatchCallVecLanes: true,
      verifierInvoked: false,
      muninnChecked: false,
      ledger9101VerifiedOk: false,
      disposition: ResNone,
      reproveRequested: false,
      submittedToChain: false
    }
  }

  // Precondition for a box returning a prove result (HLP-003, HLP-004, HLP-005).
  pure def canBoxReturnResult(
    s: PlacementState,
    jid: int,
    boxId: int,
    shipBundleFlag: bool,
    leakPrivateFlag: bool
  ): bool = {
    val j = s.jobs.get(jid)
    val slotReady = (j.disposition == ResNone or (j.disposition == ResDropped and not(j.submittedToChain)))
    val ownerOk = s.ownedRoster.contains(boxId) and boxId != BOX_MUNINN
    val bundleOk = (MUTANT_ID == MUT_HLP003_SHIP_ACCEPT_BUNDLE) or not(shipBundleFlag)
    val privOk = (MUTANT_ID == MUT_HLP004_LEAK_PRIVATE_IN_RESULT) or not(leakPrivateFlag)
    slotReady and ownerOk and bundleOk and privOk
  }

  // Precondition for submitting a call carrying a box wrap proof to the ledger (HLP-001, HLP-002, HLP-008).
  pure def canSubmitJobCall(j: JobRecord, mutAllowRefused: bool): bool =
    if (MUTANT_ID == MUT_HLP002_SUBMIT_REFUSED_WRAP and mutAllowRefused)
      (j.resultPresent and j.muninnChecked and not(j.submittedToChain))
    else
      (j.resultPresent and j.disposition == ResKept and j.muninnChecked and j.ledger9101VerifiedOk and not(j.submittedToChain))

  // Precondition for accepting a prover host configuration (HLP-005).
  pure def canAcceptProverHostConfig(s: PlacementState, targetHost: int): bool =
    if (MUTANT_ID == MUT_HLP005_ALLOW_UNOWNED_PROVER) true
    else s.ownedRoster.contains(targetHost)

  // Precondition for signing or submitting a transaction on a box (HLP-007).
  pure def canBoxSignOrSubmitTx(s: PlacementState, boxId: int): bool =
    if (MUTANT_ID == MUT_HLP007_PROVING_BOX_SUBMITS_TX) true
    else s.walletBoxes.contains(boxId)

  // Precondition for wrap & blinding placement (adopted ZK-009).
  pure def canRunWrapPlacement(
    remoteMsm: bool,
    slotAbuse: bool,
    callsInWrap: int,
    optInBatchOk: bool
  ): bool = {
    val msmOk = (MUTANT_ID == MUT_ZK009_REMOTE_UNMASKED_MSM) or not(remoteMsm)
    val slotOk = (MUTANT_ID == MUT_ZK009_REUSE_OR_HELPER_SLOT) or not(slotAbuse)
    val countOk =
      (MUTANT_ID == MUT_ZK009_MULTI_CALL_WRAP) or (callsInWrap >= 1 and callsInWrap <= MAX_CALLS_PER_WRAP)
    msmOk and slotOk and countOk and optInBatchOk
  }

  // =========================================================================
  // Pure consistency & spec-gap witness predicates
  // =========================================================================

  val consistency_constants: bool =
    PUBLIC_STATEMENT_WORDS == 4 and
    CALL_VECTOR_LANES == 1026 and
    LANE_ANCHOR_CP == 10 and
    LANE_CLAIM_PUB_A == 20 and
    LANE_CLAIM_PUB_B == 34 and
    LANE_OUTCOME_PUB == 39 and
    LANE_ANCHOR_CP < CALL_VECTOR_LANES and
    LANE_CLAIM_PUB_A < CALL_VECTOR_LANES and
    LANE_CLAIM_PUB_B < CALL_VECTOR_LANES and
    LANE_OUTCOME_PUB < CALL_VECTOR_LANES and
    LANE_ANCHOR_CP != LANE_CLAIM_PUB_A and
    LANE_CLAIM_PUB_A != LANE_CLAIM_PUB_B and
    LANE_CLAIM_PUB_B != LANE_OUTCOME_PUB and
    SDK_OUTSOURCE_COMMANDS == 0 and
    SDK_OUTSOURCE_OPTIONS == 0 and
    MAX_WALLET_BOXES == 1 and
    MAX_CALLS_PER_WRAP == 1 and
    OWNED_PROVER_BOX_COUNT >= 1 and
    OWNED_PROVER_BOX_COUNT <= 3

  // Finding Q-placement-1 witness:
  // Job 12's StatementPublics and WrapOutput are internally lane-consistent with each other AND share c_p (500)
  // with job 13, while differing on lanes 20, 34 (claim_pub: 120 != 130) and 39 (outcome_pub: 220 != 230).
  // Without job 13's private inputs/salts (which HLP-003/HLP-004 forbid sending to muninn), muninn cannot
  // independently derive job 13's expectedVec, AND input_pub (1012) is not one of the 3 call-vector values.
  pure val gap_q_placement_1_job_swap_and_four_words_vs_three_lanes: bool = {
    val pubs12 = jobExpectedPublics(12)
    val vec12 = jobExpectedVec(12)
    val vec13 = jobExpectedVec(13)
    wordsConsistentWithVec(pubs12, vec12) and
      vec12.cpLane10 == vec13.cpLane10 and
      vec12.claimLane20 != vec13.claimLane20 and
      vec12.claimLane34 != vec13.claimLane34 and
      vec12.outcomeLane39 != vec13.outcomeLane39 and
      pubs12.inputPub != vec12.cpLane10 and
      pubs12.inputPub != vec12.claimLane20 and
      pubs12.inputPub != vec12.outcomeLane39
  }

  // Finding Q-placement-2 witness:
  // In a multi-box owned roster (muninn + huginn + odin), dispatching a taped call or co-proving shards across
  // two user-owned boxes crosses a single box boundary (singleBoxPrivate == false) while staying strictly inside
  // the user-owned Case A boundary (ownedRosterPrivate == true) and keeping returned prove results private-free.
  val gap_q_placement_2_box_private_vs_owned_roster_private: bool = {
    val senderBox = BOX_MUNINN
    val proverBox = BOX_HUGINN
    val roster = computeOwnedRoster(OWNED_PROVER_BOX_COUNT)
    senderBox != proverBox and roster.contains(senderBox) and roster.contains(proverBox)
  }

  // Finding Q-placement-3 witness:
  // Outsourcing an unmasked wrap MSM/NTT sends zero witnesses, STARK proofs, checkpoints, or shard plans
  // (satisfying literal HLP-005's 4-item list), yet violates adopted ZK-009 (and triggers HLP-006's scenario).
  pure val gap_q_placement_3_hlp005_misses_zk009_wrap_msm: bool = {
    val sentHlp005Items: Set[str] = Set()
    val remoteUnmaskedWrapMsm = true
    sentHlp005Items == Set() and remoteUnmaskedWrapMsm
  }

  // Finding Q-placement-4 witness:
  // (a) If a remote proving box (odin) were configured with holdsWalletDir = true, literal HLP-007's antecedent
  //     ("box that holds no wallet directory") is false, vacuously permitting odin to sign/submit.
  // (b) A valid proof of job 12 dropped pre-verification for job 13 (HLP-008) was never refused by the 9.1.0.1
  //     verifier (verifierRefused == false), so literal HLP-002 ("If the muninn ledger check refuses a wrap proof")
  //     requires a positive ResKept precondition on the settle runner.
  pure val gap_q_placement_4_negative_guards_vacuity: bool = {
    val odinHoldsWalletDir = true
    val odinSubmitsTx = true
    val literalHlp007Holds = (not(odinHoldsWalletDir) implies not(odinSubmitsTx))
    val verifierRefusedJob12ValidProof = false
    val droppedPreVerifyByHlp008 = true
    literalHlp007Holds and droppedPreVerifyByHlp008 and not(verifierRefusedJob12ValidProof)
  }

  // =========================================================================
  // Initialization
  // =========================================================================

  action init = all {
    plc' = {
      ownedRoster: computeOwnedRoster(OWNED_PROVER_BOX_COUNT),
      walletBoxes: Set(BOX_MUNINN),
      jobs: JOB_IDS.mapBy(jid => blankJobRecord(jid)),

      privatePartLeftOwnedBoxes: false,
      unownedHostConfigAccepted: false,
      unownedArtifactsSent: Set(),
      caseAConfigsAccepted: 0,
      unownedConfigRefusals: 0,

      boxesThatSignedOrSubmitted: Set(),
      walletlessTxRefusalsOnBus: Set(),

      helperProvedStarkOrL: false,
      remoteUnmaskedWrapOp: false,
      helperHoldsFullSpongeState: false,
      blindingSlotReused: false,
      helperMadeBlindingSlot: false,
      aggregatorReceivedStarkProofs: false,
      callsPerWrap: 1,
      userBatchOptInValid: true,
      localBlindedWrapsCompleted: 0,
      zk009Refusals: 0,

      lastRefusal: "none"
    },
    lastOp' = "init"
  }

  // =========================================================================
  // Parameterized actions
  // =========================================================================

  // Implements HLP-003, HLP-004, HLP-008:
  // Proving box runs argument builder locally, keeps accept-bundle and private parts (prog_d, r_P, salt, tape)
  // on the box, and returns StatementPublics (4 words) + WrapOutput + SHA-256 report to muninn.
  action doBoxFinishAndReturnResult(
    jid: int,
    boxId: int,
    faultKind: BoxReturnFault,
    mutShipBundle: bool,
    mutLeakPrivate: bool
  ): bool = all {
    JOB_IDS.contains(jid),
    canBoxReturnResult(plc, jid, boxId, mutShipBundle, mutLeakPrivate),
    val sourceJobId =
      if (faultKind == FaultWrongJobVec) (if (jid == 12) 13 else 12)
      else jid
    val pubs = jobExpectedPublics(sourceJobId)
    val vec = jobExpectedVec(sourceJobId)
    val repSha = 7000 + jid
    val actSha = if (faultKind == FaultBadSha) repSha + 1 else repSha
    val cryptoOk = (faultKind != FaultInvalidProof)
    val bundleShipped = (MUTANT_ID == MUT_HLP003_SHIP_ACCEPT_BUNDLE and mutShipBundle)
    val wordCnt = if (bundleShipped) PUBLIC_STATEMENT_WORDS + 4 else PUBLIC_STATEMENT_WORDS
    val leakedPriv = (MUTANT_ID == MUT_HLP004_LEAK_PRIVATE_IN_RESULT and mutLeakPrivate)
    val updatedJob: JobRecord = {
      ...plc.jobs.get(jid),
      resultPresent: true,
      fromBox: boxId,
      reportedSha256: repSha,
      actualSha256: actSha,
      returnedPublics: pubs,
      returnedPublicWordCount: wordCnt,
      returnedVec: vec,
      proofCryptoValid: cryptoOk,
      argBuilderRanOnBox: not(bundleShipped),
      acceptBundleLeftBox: bundleShipped,
      privatePartInResult: leakedPriv,
      wordsMatchCallVecLanes: wordsConsistentWithVec(pubs, vec),
      verifierInvoked: false,
      muninnChecked: false,
      ledger9101VerifiedOk: false,
      disposition: ResReturnedUnchecked,
      reproveRequested: false,
      submittedToChain: false
    }
    plc' = {
      ...plc,
      jobs: plc.jobs.put(jid, updatedJob)
    },
    lastOp' = "boxFinishAndReturnResult"
  }

  // Implements HLP-001, HLP-002, HLP-003, HLP-008:
  // muninn checks SHA-256 and job call-vector binding BEFORE invoking ledger 9.1.0.1 verifier (HLP-008);
  // if binding holds, runs ledger 9.1.0.1 verifier and marks KEEP (HLP-001) or drops for reprove (HLP-002).
  action doMuninnCheckBoxResult(jid: int, mutTrig: bool): bool = {
    val j = plc.jobs.get(jid)
    val shaMatches = (j.actualSha256 == j.reportedSha256)
    val jobVecMatches = (j.returnedVec == j.expectedVec)
    val lanesMatch = j.wordsMatchCallVecLanes
    val ignoreBadSha = (MUTANT_ID == MUT_HLP008_ACCEPT_BAD_SHA256 and not(shaMatches) and mutTrig)
    val ignoreBadVec = (MUTANT_ID == MUT_HLP008_ACCEPT_OTHER_JOB_VEC and not(jobVecMatches) and mutTrig)
    val skipVerifyKeep = (MUTANT_ID == MUT_HLP001_KEEP_UNVERIFIED and mutTrig)
    val bindingPassed = (shaMatches or ignoreBadSha) and (jobVecMatches or ignoreBadVec) and lanesMatch
    all {
      JOB_IDS.contains(jid),
      j.resultPresent,
      j.disposition == ResReturnedUnchecked,
      val updatedJob: JobRecord =
        if (skipVerifyKeep) {
          ...j,
          verifierInvoked: false,
          muninnChecked: false,
          ledger9101VerifiedOk: false,
          disposition: ResKept,
          reproveRequested: false
        } else if (not(bindingPassed)) {
          ...j,
          verifierInvoked: false,
          muninnChecked: true,
          ledger9101VerifiedOk: false,
          disposition: ResDropped,
          reproveRequested: true
        } else if (j.proofCryptoValid) {
          ...j,
          verifierInvoked: true,
          muninnChecked: true,
          ledger9101VerifiedOk: true,
          disposition: ResKept,
          reproveRequested: false
        } else {
          ...j,
          verifierInvoked: true,
          muninnChecked: true,
          ledger9101VerifiedOk: false,
          disposition: ResDropped,
          reproveRequested: true
        }
      val newRefusal =
        if (not(bindingPassed) and not(shaMatches)) "box_file_sha256_mismatch_dropped"
        else if (not(bindingPassed) and not(jobVecMatches)) "box_call_vector_job_mismatch_dropped"
        else if (bindingPassed and not(j.proofCryptoValid) and not(skipVerifyKeep)) "ledger_9101_refused_box_wrap_dropped"
        else plc.lastRefusal
      plc' = {
        ...plc,
        jobs: plc.jobs.put(jid, updatedJob),
        lastRefusal: newRefusal
      },
      lastOp' = "muninnCheckBoxResult"
    }
  }

  // Implements HLP-001, HLP-002:
  // Settle runner submits a call carrying a box wrap proof only when muninn checked and kept it.
  action doSubmitSettledCall(jid: int, mutAllowRefused: bool): bool = {
    val j = plc.jobs.get(jid)
    all {
      JOB_IDS.contains(jid),
      canSubmitJobCall(j, mutAllowRefused),
      plc' = {
        ...plc,
        jobs: plc.jobs.put(jid, { ...j, submittedToChain: true })
      },
      lastOp' = "submitSettledCall"
    }
  }

  // Implements HLP-005:
  // Prover reads configuration; accepts user-owned Case A hosts and refuses unowned helper hosts.
  action doEvaluateProverConfig(targetHost: int, artifactKind: str): bool = {
    val isOwned = plc.ownedRoster.contains(targetHost)
    val acceptUnowned = (MUTANT_ID == MUT_HLP005_ALLOW_UNOWNED_PROVER and not(isOwned))
    all {
      plc' =
        if (isOwned) {
          ...plc,
          caseAConfigsAccepted: plc.caseAConfigsAccepted + 1
        } else if (acceptUnowned) {
          ...plc,
          unownedHostConfigAccepted: true,
          unownedArtifactsSent: plc.unownedArtifactsSent.union(Set(artifactKind)),
          privatePartLeftOwnedBoxes: true
        } else {
          ...plc,
          unownedConfigRefusals: plc.unownedConfigRefusals + 1,
          lastRefusal: "unowned_prover_host_refused"
        },
      lastOp' = "evaluateProverConfig"
    }
  }

  // Implements HLP-007:
  // Box agent evaluates a job asking to sign or submit a transaction; walletless proving boxes refuse on the bus.
  action doRequestTxSignOrSubmitOnBox(boxId: int, mutTrig: bool): bool = {
    val hasWallet = plc.walletBoxes.contains(boxId)
    val allowBadSubmit = (MUTANT_ID == MUT_HLP007_PROVING_BOX_SUBMITS_TX and not(hasWallet) and mutTrig)
    all {
      plc.ownedRoster.contains(boxId),
      plc' =
        if (hasWallet or allowBadSubmit) {
          ...plc,
          boxesThatSignedOrSubmitted: plc.boxesThatSignedOrSubmitted.union(Set(boxId))
        } else {
          ...plc,
          walletlessTxRefusalsOnBus: plc.walletlessTxRefusalsOnBus.union(Set(boxId)),
          lastRefusal: "proving_box_has_no_wallet"
        },
      lastOp' = "requestTxSignOrSubmitOnBox"
    }
  }

  // Implements adopted ZK-009:
  // Enforces local blinded single-ContractCall wrap placement (refuses remote unmasked MSM/NTT/quotient/Merkle,
  // helper sponge state, blinding-slot reuse or helper-made slots, external STARK aggregators, and >1 call/wrap).
  action doCheckWrapAndBlindingPlacement(checkMode: int, mutTrig: bool): bool = {
    // checkMode: 0 = honest local single-call blinded wrap;
    //            1 = remote unmasked MSM/NTT request;
    //            2 = blinding-slot reuse / helper-made slot request;
    //            3 = multi-call (2 ContractCalls) in one wrap request.
    val reqRemoteMsm = (checkMode == 1)
    val reqSlotAbuse = (checkMode == 2)
    val reqCalls = if (checkMode == 3) 2 else 1
    val allowed = canRunWrapPlacement(reqRemoteMsm, reqSlotAbuse, reqCalls, true)
    val mutBypass = (
      (MUTANT_ID == MUT_ZK009_REMOTE_UNMASKED_MSM and reqRemoteMsm and mutTrig) or
      (MUTANT_ID == MUT_ZK009_REUSE_OR_HELPER_SLOT and reqSlotAbuse and mutTrig) or
      (MUTANT_ID == MUT_ZK009_MULTI_CALL_WRAP and checkMode == 3 and mutTrig)
    )
    all {
      plc' =
        if (checkMode == 0) {
          ...plc,
          callsPerWrap: 1,
          userBatchOptInValid: true,
          localBlindedWrapsCompleted: plc.localBlindedWrapsCompleted + 1
        } else if (allowed and mutBypass) {
          ...plc,
          remoteUnmaskedWrapOp: plc.remoteUnmaskedWrapOp or reqRemoteMsm,
          blindingSlotReused: plc.blindingSlotReused or reqSlotAbuse,
          helperMadeBlindingSlot: plc.helperMadeBlindingSlot or reqSlotAbuse,
          callsPerWrap: reqCalls
        } else {
          ...plc,
          zk009Refusals: plc.zk009Refusals + 1,
          lastRefusal: "zk009_helper_or_wrap_placement_refused"
        },
      lastOp' = "checkWrapAndBlindingPlacement"
    }
  }

  // =========================================================================
  // Nondeterministic step relation
  // =========================================================================

  action step = any {
    {
      nondet jid = JOB_IDS.oneOf()
      val provers = computeProverBoxes(OWNED_PROVER_BOX_COUNT)
      nondet boxId = provers.oneOf()
      nondet fKind = Set(FaultNone, FaultBadSha, FaultWrongJobVec, FaultInvalidProof).oneOf()
      nondet mBundle = Set(true, false).oneOf()
      nondet mLeak = Set(true, false).oneOf()
      doBoxFinishAndReturnResult(jid, boxId, fKind, mBundle, mLeak)
    },
    {
      nondet jid = JOB_IDS.oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doMuninnCheckBoxResult(jid, mutTrig)
    },
    {
      nondet jid = JOB_IDS.oneOf()
      nondet mutRef = Set(true, false).oneOf()
      doSubmitSettledCall(jid, mutRef)
    },
    {
      nondet tHost = Set(BOX_HUGINN, HOST_UNOWNED_HELPER).oneOf()
      nondet aKind = Set("witness", "stark_proof", "checkpoint", "shard_plan").oneOf()
      doEvaluateProverConfig(tHost, aKind)
    },
    {
      val roster = computeOwnedRoster(OWNED_PROVER_BOX_COUNT)
      nondet boxId = roster.oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doRequestTxSignOrSubmitOnBox(boxId, mutTrig)
    },
    {
      nondet cMode = Set(0, 1, 2, 3).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doCheckWrapAndBlindingPlacement(cMode, mutTrig)
    }
  }

  // =========================================================================
  // Safety invariants (one inv_<ID> per expressible requirement)
  // =========================================================================

  // HLP-001: Every kept box wrap result was verified on muninn with the ledger 9.1.0.1 verifier first.
  val inv_HLP_001: bool =
    JOB_IDS.forall(jid => {
      val j = plc.jobs.get(jid)
      (j.disposition == ResKept) implies (j.muninnChecked and j.ledger9101VerifiedOk and j.verifierInvoked)
    })

  // HLP-002: A box wrap refused by muninn's ledger check is dropped and never submitted by the settle runner.
  val inv_HLP_002: bool =
    JOB_IDS.forall(jid => {
      val j = plc.jobs.get(jid)
      val refusedDropped =
        (j.muninnChecked and not(j.ledger9101VerifiedOk)) implies
          (j.disposition == ResDropped and not(j.submittedToChain))
      val submittedValid =
        j.submittedToChain implies (j.disposition == ResKept and j.muninnChecked and j.ledger9101VerifiedOk)
      refusedDropped and submittedValid
    })

  // HLP-003: Argument bundle stays on the proving box; only the 4 StatementPublics words return with WrapOutput,
  // matching the call-vector lanes when kept.
  val inv_HLP_003: bool =
    JOB_IDS.forall(jid => {
      val j = plc.jobs.get(jid)
      j.resultPresent implies (
        j.argBuilderRanOnBox and
        not(j.acceptBundleLeftBox) and
        j.returnedPublicWordCount == PUBLIC_STATEMENT_WORDS and
        ((j.disposition == ResKept) implies j.wordsMatchCallVecLanes)
      )
    })

  // HLP-004: Private parts (prog_d, r_P, run salt, call tape) never appear in returned result files or leave owned boxes.
  val inv_HLP_004: bool =
    not(plc.privatePartLeftOwnedBoxes) and
    JOB_IDS.forall(jid => not(plc.jobs.get(jid).privatePartInResult))

  // HLP-005: Case A only — 0 commands/options and 0 artifacts sent to unowned hosts; unowned configs refused.
  val inv_HLP_005: bool =
    SDK_OUTSOURCE_COMMANDS == 0 and
    SDK_OUTSOURCE_OPTIONS == 0 and
    not(plc.unownedHostConfigAccepted) and
    plc.unownedArtifactsSent == Set()

  // HLP-007: Wallet stays on the designated wallet host (muninn); walletless proving boxes never sign or submit.
  val inv_HLP_007: bool =
    plc.walletBoxes.size() <= MAX_WALLET_BOXES and
    plc.walletBoxes == Set(BOX_MUNINN) and
    plc.boxesThatSignedOrSubmitted.forall(b => plc.walletBoxes.contains(b))

  // HLP-008: Any box file with mismatched SHA-256 or mismatched job call vector is dropped BEFORE proof verification.
  val inv_HLP_008: bool =
    JOB_IDS.forall(jid => {
      val j = plc.jobs.get(jid)
      val bindingBad = (
        j.actualSha256 != j.reportedSha256 or
        j.returnedVec != j.expectedVec or
        not(j.wordsMatchCallVecLanes)
      )
      (j.resultPresent and bindingBad) implies (
        not(j.verifierInvoked) and
        j.disposition != ResKept and
        (j.muninnChecked implies j.disposition == ResDropped)
      )
    })

  // Adopted ZK-009: No STARK/L helper, no remote unmasked MSM/NTT/quotient/Merkle, no helper sponge state,
  // no blinding-slot reuse or helper-made slot, no STARK aggregator, and at most 1 ContractCall per wrap.
  val inv_ZK_009: bool =
    not(plc.helperProvedStarkOrL) and
    not(plc.remoteUnmaskedWrapOp) and
    not(plc.helperHoldsFullSpongeState) and
    not(plc.blindingSlotReused) and
    not(plc.helperMadeBlindingSlot) and
    not(plc.aggregatorReceivedStarkProofs) and
    plc.callsPerWrap >= 1 and
    plc.callsPerWrap <= MAX_CALLS_PER_WRAP and
    plc.userBatchOptInValid

  // Master safety invariant.
  val inv: bool =
    consistency_constants and
    inv_HLP_001 and
    inv_HLP_002 and
    inv_HLP_003 and
    inv_HLP_004 and
    inv_HLP_005 and
    inv_HLP_007 and
    inv_HLP_008 and
    inv_ZK_009

  // =========================================================================
  // Reachability witnesses (negated goals — MUST be violated under placement_main)
  // =========================================================================

  // Witness 1: Honest box wrap proof passes muninn 9.1.0.1 check (ResKept) and settles on chain (HLP-001, HLP-003, HLP-004).
  val reach_honest_box_wrap_kept_and_submitted: bool = not(
    JOB_IDS.exists(jid => {
      val j = plc.jobs.get(jid)
      j.disposition == ResKept and j.submittedToChain
    })
  )

  // Witness 2: Invalid box wrap proof is refused by muninn 9.1.0.1 verifier, dropped, and queued for reprove (HLP-002).
  val reach_refused_box_wrap_dropped_and_reproved: bool = not(
    JOB_IDS.exists(jid => {
      val j = plc.jobs.get(jid)
      j.verifierInvoked and
      j.muninnChecked and
      not(j.ledger9101VerifiedOk) and
      j.disposition == ResDropped and
      j.reproveRequested
    })
  )

  // Witness 3: Box result with corrupted SHA-256 or swapped job call vector is dropped BEFORE proof verification (HLP-008).
  val reach_bad_sha_or_other_job_dropped_pre_verify: bool = not(
    JOB_IDS.exists(jid => {
      val j = plc.jobs.get(jid)
      j.muninnChecked and
      not(j.verifierInvoked) and
      j.disposition == ResDropped and
      (j.actualSha256 != j.reportedSha256 or j.returnedVec != j.expectedVec)
    })
  )

  // Witness 4: Unowned helper config and remote unmasked wrap MSM are refused while Case A local wrap succeeds (HLP-005, ZK-009).
  val reach_unowned_helper_and_remote_msm_refused: bool = not(
    plc.unownedConfigRefusals >= 1 and
    plc.zk009Refusals >= 1 and
    plc.localBlindedWrapsCompleted >= 1
  )

  // Witness 5: Walletless proving box refuses tx sign/submit on bus while muninn signs/submits (HLP-007).
  val reach_walletless_proving_box_refuses_tx_on_bus: bool = not(
    plc.walletlessTxRefusalsOnBus.size() >= 1 and
    plc.boxesThatSignedOrSubmitted.contains(BOX_MUNINN)
  )
}

// ===========================================================================
// Concrete instances: main, edge instances, and guard mutants
// ===========================================================================

// Main instance with exact spec constants (2 remote proving boxes: huginn, odin + wallet host muninn).
module placement_main {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4,
    CALL_VECTOR_LANES = 1026,
    LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20,
    LANE_CLAIM_PUB_B = 34,
    LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0,
    SDK_OUTSOURCE_OPTIONS = 0,
    MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1,
    OWNED_PROVER_BOX_COUNT = 2,
    MUTANT_ID = 0
  ).*
}

// Minimum legal prover roster (1 remote proving box: huginn + wallet host muninn).
module placement_edge_min {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4,
    CALL_VECTOR_LANES = 1026,
    LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20,
    LANE_CLAIM_PUB_B = 34,
    LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0,
    SDK_OUTSOURCE_OPTIONS = 0,
    MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1,
    OWNED_PROVER_BOX_COUNT = 1,
    MUTANT_ID = 0
  ).*
}

// Upper roster bound (3 remote owned proving boxes + wallet host muninn, with exact spec bounds).
module placement_edge_at_limit {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4,
    CALL_VECTOR_LANES = 1026,
    LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20,
    LANE_CLAIM_PUB_B = 34,
    LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0,
    SDK_OUTSOURCE_OPTIONS = 0,
    MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1,
    OWNED_PROVER_BOX_COUNT = 3,
    MUTANT_ID = 0
  ).*
}

// Guard mutants (each weakens or removes one refusal/binding guard and must break its named invariant)

module placement_m_hlp001_keep_unverified {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4, CALL_VECTOR_LANES = 1026, LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20, LANE_CLAIM_PUB_B = 34, LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0, SDK_OUTSOURCE_OPTIONS = 0, MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1, OWNED_PROVER_BOX_COUNT = 2, MUTANT_ID = 1
  ).*
}

module placement_m_hlp002_submit_refused_wrap {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4, CALL_VECTOR_LANES = 1026, LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20, LANE_CLAIM_PUB_B = 34, LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0, SDK_OUTSOURCE_OPTIONS = 0, MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1, OWNED_PROVER_BOX_COUNT = 2, MUTANT_ID = 2
  ).*
}

module placement_m_hlp003_ship_accept_bundle {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4, CALL_VECTOR_LANES = 1026, LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20, LANE_CLAIM_PUB_B = 34, LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0, SDK_OUTSOURCE_OPTIONS = 0, MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1, OWNED_PROVER_BOX_COUNT = 2, MUTANT_ID = 3
  ).*
}

module placement_m_hlp004_leak_private_in_result {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4, CALL_VECTOR_LANES = 1026, LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20, LANE_CLAIM_PUB_B = 34, LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0, SDK_OUTSOURCE_OPTIONS = 0, MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1, OWNED_PROVER_BOX_COUNT = 2, MUTANT_ID = 4
  ).*
}

module placement_m_hlp005_allow_unowned_prover {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4, CALL_VECTOR_LANES = 1026, LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20, LANE_CLAIM_PUB_B = 34, LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0, SDK_OUTSOURCE_OPTIONS = 0, MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1, OWNED_PROVER_BOX_COUNT = 2, MUTANT_ID = 5
  ).*
}

module placement_m_hlp007_proving_box_submits_tx {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4, CALL_VECTOR_LANES = 1026, LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20, LANE_CLAIM_PUB_B = 34, LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0, SDK_OUTSOURCE_OPTIONS = 0, MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1, OWNED_PROVER_BOX_COUNT = 2, MUTANT_ID = 6
  ).*
}

module placement_m_hlp008_accept_bad_sha256 {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4, CALL_VECTOR_LANES = 1026, LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20, LANE_CLAIM_PUB_B = 34, LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0, SDK_OUTSOURCE_OPTIONS = 0, MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1, OWNED_PROVER_BOX_COUNT = 2, MUTANT_ID = 7
  ).*
}

module placement_m_hlp008_accept_other_job_vec {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4, CALL_VECTOR_LANES = 1026, LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20, LANE_CLAIM_PUB_B = 34, LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0, SDK_OUTSOURCE_OPTIONS = 0, MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1, OWNED_PROVER_BOX_COUNT = 2, MUTANT_ID = 8
  ).*
}

module placement_m_zk009_remote_unmasked_msm {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4, CALL_VECTOR_LANES = 1026, LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20, LANE_CLAIM_PUB_B = 34, LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0, SDK_OUTSOURCE_OPTIONS = 0, MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1, OWNED_PROVER_BOX_COUNT = 2, MUTANT_ID = 9
  ).*
}

module placement_m_zk009_reuse_or_helper_slot {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4, CALL_VECTOR_LANES = 1026, LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20, LANE_CLAIM_PUB_B = 34, LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0, SDK_OUTSOURCE_OPTIONS = 0, MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1, OWNED_PROVER_BOX_COUNT = 2, MUTANT_ID = 10
  ).*
}

module placement_m_zk009_multi_call_wrap {
  import placement(
    PUBLIC_STATEMENT_WORDS = 4, CALL_VECTOR_LANES = 1026, LANE_ANCHOR_CP = 10,
    LANE_CLAIM_PUB_A = 20, LANE_CLAIM_PUB_B = 34, LANE_OUTCOME_PUB = 39,
    SDK_OUTSOURCE_COMMANDS = 0, SDK_OUTSOURCE_OPTIONS = 0, MAX_WALLET_BOXES = 1,
    MAX_CALLS_PER_WRAP = 1, OWNED_PROVER_BOX_COUNT = 2, MUTANT_ID = 11
  ).*
}
