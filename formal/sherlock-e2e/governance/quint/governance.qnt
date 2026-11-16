// -*- mode: Bluespec; -*-
// Independent Quint specification for bounded context Governance (lane e2e-governance).
// Sources:
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-governance/spec.md
//     (EGV-001..010)
//   - openspec/changes/sherlock-completion-20261115/specs/sherlock-loop-completion/spec.md
//     (LPC-012)
//   - t22-governance: openspec/changes/sherlock-governance/specs/
//     sherlock-era-table (ERT), sherlock-era-ledger (ELD),
//     sherlock-governance-verbs (GOV), sherlock-consumer-read (CRR)
//   - openspec/changes/sherlock-e2e-prototype-20261115/architecture/DDD.md
//     (Governance rows in sections 1..4 and 6..8)

module governance {
  // =========================================================================
  // Constants (governance parameters and bounds from the EARS specs)
  // =========================================================================

  // EGV-001, EGV-002, ERT-001, ERT-002, ELD-001: sealed authority threshold (1, never 0).
  const SEALED_THRESHOLD: int
  // EGV-001, EGV-002, ERT-001, ERT-002, ELD-001: initial authority counter (0).
  const SEALED_COUNTER: int
  // GOV-001, EGV-008 Scenario: Fenrir operator roster cardinality n (recommended 16).
  const ROSTER_SIZE_N: int
  // GOV-001, EGV-008 Scenario: required operator quorum Q (recommended 12 of 16).
  const QUORUM_Q: int
  // GOV-002, GOV-004, GOV-008, EGV-008 Scenario: announcement / attestation timelock TL.
  const TIMELOCK_TL: int
  // GOV-005, EGV-008: roster rotation timelock RTL.
  const ROTATE_TIMELOCK_RTL: int
  // ERT-009, EGV-008: activation grid step GRID (start % GRID_STEP == 0).
  const GRID_STEP: int
  // ELD-006: call-vector transaction lane ceiling L_TX (1,024; measured 85 and 91).
  const L_TX_LANE_LIMIT: int
  // Mutant selector: 0 = honest specification; 1..13 = individual guard mutants.
  const MUTANT_ID: int

  // =========================================================================
  // Mutant identifiers
  // =========================================================================

  pure val MUT_NONE: int = 0
  pure val MUT_EGV001_THRESHOLD_ZERO: int = 1
  pure val MUT_EGV002_KEEP_SDK_COMMITTEE: int = 2
  pure val MUT_EGV003_UNSEALED_SOUNDNESS_LABEL: int = 3
  pure val MUT_EGV004_REUSE_KEYSET: int = 4
  pure val MUT_EGV005_REKEY_NO_ERA_NO_FAIL: int = 5
  pure val MUT_EGV006_FALLBACK_ENDED_ERA_USE: int = 6
  pure val MUT_EGV007_IN_PLACE_VK_UPDATE: int = 7
  pure val MUT_EGV007_SUBQUORUM_OR_NO_TL: int = 8
  pure val MUT_EGV008_OMIT_OPEN_PARAMS: int = 9
  pure val MUT_EGV009_INCLUDE_AFTER_END: int = 10
  pure val MUT_EGV010_FORK_MUTATE_OR_NO_PRED: int = 11
  pure val MUT_ERT007_OVERWRITE_END: int = 12
  pure val MUT_CRR004_USE_SOUNDNESS_ENDED: int = 13

  // =========================================================================
  // Domain types
  // =========================================================================

  type EraStatus =
    | EraNormal
    | EraCancelled
    | EraSoundnessEnded

  type ReceiptLabel =
    | LabelUnreported
    | LabelTransactionPathOnly
    | LabelSoundnessEvidence

  type DeployedContractRecord = {
    contractId: int,
    isEraTable: bool,
    sealedEraId: int,
    committeeCount: int,
    threshold: int,
    authCounter: int,
    entryKeysMatchExpected: bool,
    wrapEntryLanes: int,
    markedUnusable: bool,
    vkUpdatedAfterDeploy: bool,
    reportLabel: ReceiptLabel
  }

  type EraRecord = {
    eraId: int,
    ledgerId: int,
    keysetD: int,
    startS: int,
    initialLedgerId: int,
    initialKeysetD: int,
    initialStartS: int,
    announcedAtS: int,
    signerCount: int,
    endIsSet: bool,
    endS: int,
    endWriteCount: int,
    eraStatus: EraStatus
  }

  type IncludedCallRecord = {
    callId: int,
    eraId: int,
    includedAtS: int,
    fallbackModeActive: bool,
    eraActiveAtInclusion: bool
  }

  type ConsumerUseRecord = {
    useId: int,
    eraId: int,
    usedAtS: int,
    fallbackModeAtUse: bool,
    eraActiveAtUse: bool,
    eraSoundnessEndedAtUse: bool,
    wroteSherlockState: bool
  }

  type GovState = {
    nowS: int,
    contracts: int -> DeployedContractRecord,
    knownContractIds: Set[int],

    eras: int -> EraRecord,
    knownEraIds: Set[int],
    currentEraId: int,
    hasPendingEra: bool,
    pendingEraId: int,

    rosterGen: int,
    activeRosterD: int,
    pendingRotation: bool,
    pendingRosterD: int,
    pendingRotationEffS: int,
    signedVerbNonce: int,

    premiseTestRan: bool,
    recordedPremiseCount: int,
    premiseA1Pass: bool,
    premiseA2Pass: bool,
    premiseA3Pass: bool,
    rekeyListHasEraRow: bool,
    rekeyStarted: bool,

    includedCalls: Set[IncludedCallRecord],
    consumerUses: Set[ConsumerUseRecord],

    ruledGovParams: Set[str],
    t15FreezeWritten: bool,
    t15OpenParamsListed: Set[str],
    t15StatedUnruledAsRule: bool,

    forkStopAnnounced: bool,
    forkMigratedBeforeHeight: bool,
    newTableHasPredecessorLink: bool,
    newSealedLedgersForNextProofVer: bool,
    oldSealedContractsMutatedVk: bool,
    snapshotRootMarkedTrustDowngrade: bool,

    lastRefusal: str
  }

  // =========================================================================
  // Observable state variables
  // =========================================================================

  var gov: GovState
  var lastOp: str

  // =========================================================================
  // Pure constants & helper predicates
  // =========================================================================

  pure val ALL_GOV_PARAMS: Set[str] = Set(
    "TL", "RTL", "Q", "n", "GRID", "SKEW"
  )

  // EGV-001, EGV-002, ERT-001, ERT-002, ELD-001: exact sealed authority {[], 1, 0}.
  pure def isContractSealedAndValid(c: DeployedContractRecord): bool =
    c.committeeCount == 0 and
    c.threshold == SEALED_THRESHOLD and
    c.authCounter == SEALED_COUNTER and
    c.entryKeysMatchExpected and
    c.wrapEntryLanes <= L_TX_LANE_LIMIT

  // EGV-005, EGV-006, ELD-007, CRR-006: whether premise A2 or A3 failed in the devnet test.
  pure def isUseTimeFallbackActive(s: GovState): bool =
    s.premiseTestRan and (not(s.premiseA2Pass) or not(s.premiseA3Pass))

  // ERT-004: effective end time end_eff(e) = min(end(e), start(P)) for non-cancelled pending P != e;
  // returns -1 when unbounded.
  pure def eraEndEff(s: GovState, e: EraRecord): int = {
    val pendingBound =
      if (s.hasPendingEra and s.pendingEraId != e.eraId and s.knownEraIds.contains(s.pendingEraId)) {
        val p = s.eras.get(s.pendingEraId)
        if (p.eraStatus != EraCancelled) p.startS else -1
      } else -1
    if (e.endIsSet and pendingBound >= 0)
      (if (e.endS <= pendingBound) e.endS else pendingBound)
    else if (e.endIsSet)
      e.endS
    else if (pendingBound >= 0)
      pendingBound
    else
      -1
  }

  // ERT-004: accepting interval [start(e), end_eff(e)) for non-cancelled, non-soundness-ended era e.
  pure def isEraActiveAt(s: GovState, eId: int, t: int): bool =
    s.knownEraIds.contains(eId) and {
      val e = s.eras.get(eId)
      val eff = eraEndEff(s, e)
      e.eraStatus == EraNormal and
        t >= e.startS and
        (eff < 0 or t < eff)
    }

  pure val GENESIS_TABLE_CONTRACT: DeployedContractRecord = {
    contractId: 100,
    isEraTable: true,
    sealedEraId: 0,
    committeeCount: 0,
    threshold: 1,
    authCounter: 0,
    entryKeysMatchExpected: true,
    wrapEntryLanes: 0,
    markedUnusable: false,
    vkUpdatedAfterDeploy: false,
    reportLabel: LabelSoundnessEvidence
  }

  pure def mkSealedEraLedger(cId: int, eId: int): DeployedContractRecord = {
    contractId: cId,
    isEraTable: false,
    sealedEraId: eId,
    committeeCount: 0,
    threshold: SEALED_THRESHOLD,
    authCounter: SEALED_COUNTER,
    entryKeysMatchExpected: true,
    wrapEntryLanes: 91,
    markedUnusable: false,
    vkUpdatedAfterDeploy: false,
    reportLabel: LabelSoundnessEvidence
  }

  // =========================================================================
  // Guard predicates (exposed for scenario tests in governance_test.qnt without .fail())
  // =========================================================================

  pure def canAnnounceEra(
    s: GovState,
    newEraId: int,
    ledgerContractId: int,
    keysetDigest: int,
    startT: int,
    signers: int
  ): bool = {
    val quorumAndGridAndTlOk =
      if (MUTANT_ID == MUT_EGV007_SUBQUORUM_OR_NO_TL)
        (signers >= 1 and startT >= s.nowS)
      else
        (signers >= QUORUM_Q and
          signers <= ROSTER_SIZE_N and
          startT >= s.nowS + TIMELOCK_TL and
          startT % GRID_STEP == 0)
    val uniqueKeysetOk =
      (MUTANT_ID == MUT_EGV004_REUSE_KEYSET) or
      not(s.knownEraIds.exists(id => s.eras.get(id).keysetD == keysetDigest))
    val uniqueLedgerOk =
      not(s.knownEraIds.exists(id => s.eras.get(id).ledgerId == ledgerContractId))
    val ledgerUsableOk =
      s.knownContractIds.contains(ledgerContractId) and {
        val c = s.contracts.get(ledgerContractId)
        not(c.isEraTable) and (
          if (MUTANT_ID == MUT_EGV002_KEEP_SDK_COMMITTEE) true
          else (not(c.markedUnusable) and isContractSealedAndValid(c))
        )
      }
    not(s.knownEraIds.contains(newEraId)) and
      not(s.hasPendingEra) and
      quorumAndGridAndTlOk and
      uniqueKeysetOk and
      uniqueLedgerOk and
      ledgerUsableOk
  }

  pure def canActivatePendingEra(s: GovState, targetEraId: int): bool =
    s.hasPendingEra and
    s.pendingEraId == targetEraId and
    s.knownEraIds.contains(targetEraId) and {
      val p = s.eras.get(targetEraId)
      p.eraStatus == EraNormal and s.nowS >= p.startS
    }

  pure def canSoundnessEnd(s: GovState, targetEraId: int, signers: int): bool =
    signers >= QUORUM_Q and
    signers <= ROSTER_SIZE_N and
    isEraActiveAt(s, targetEraId, s.nowS)

  pure def canStartT25Rekey(s: GovState): bool =
    not(s.rekeyStarted) and (
      (MUTANT_ID == MUT_EGV005_REKEY_NO_ERA_NO_FAIL) or
      s.rekeyListHasEraRow or
      isUseTimeFallbackActive(s)
    )

  pure def canIncludeLedgerCall(s: GovState, eId: int): bool =
    s.knownEraIds.contains(eId) and (
      if (isUseTimeFallbackActive(s)) true
      else if (MUTANT_ID == MUT_EGV009_INCLUDE_AFTER_END) true
      else isEraActiveAt(s, eId, s.nowS)
    )

  pure def canConsumerUseClaim(s: GovState, eId: int): bool =
    s.knownEraIds.contains(eId) and {
      val e = s.eras.get(eId)
      val soundnessOk =
        (MUTANT_ID == MUT_CRR004_USE_SOUNDNESS_ENDED) or (e.eraStatus != EraSoundnessEnded)
      val fallbackOk =
        if (not(isUseTimeFallbackActive(s))) true
        else if (MUTANT_ID == MUT_EGV006_FALLBACK_ENDED_ERA_USE) true
        else isEraActiveAt(s, eId, s.nowS)
      soundnessOk and fallbackOk
    }

  // =========================================================================
  // Pure consistency & spec-gap witness checks
  // =========================================================================

  val consistency_constants: bool =
    SEALED_THRESHOLD == 1 and
    SEALED_COUNTER == 0 and
    QUORUM_Q >= 1 and
    QUORUM_Q <= ROSTER_SIZE_N and
    2 * QUORUM_Q > ROSTER_SIZE_N and
    TIMELOCK_TL >= GRID_STEP and
    TIMELOCK_TL % GRID_STEP == 0 and
    ROTATE_TIMELOCK_RTL >= 1 and
    GRID_STEP >= 1 and
    L_TX_LANE_LIMIT >= 91 and
    L_TX_LANE_LIMIT <= 1024 and
    ALL_GOV_PARAMS.size() == 6

  // Finding Q-governance-4 witness: when premise A1 fails while A2 and A3 pass,
  // NEITHER ELD-003 (requires A1 and A2 and A3) NOR ELD-007/EGV-006/CRR-006 (triggers only when A2 or A3 fails)
  // applies — leaving zero active era check on either ledger inclusion or consumer use!
  pure val gap_q_gov_4_premise_a1_fail_neither_branch: bool = {
    val a1 = false
    val a2 = true
    val a3 = true
    val primaryEld003Applies = (a1 and a2 and a3)
    val fallbackEgv006Applies = (not(a2) or not(a3))
    not(primaryEld003Applies) and not(fallbackEgv006Applies)
  }

  // Finding Q-governance-2 witness: ERT-003 forbids reusing ledger_addr across eras, so using
  // ledger_addr == 0 when per-program ledgers are attested via GOV-008 collides on the second era.
  pure val gap_q_gov_2_zero_ledger_addr_collides_under_ert003: bool = {
    val era0LedgerAddr = 0
    val era1LedgerAddr = 0
    era0LedgerAddr == era1LedgerAddr
  }

  // =========================================================================
  // Initialization
  // =========================================================================

  action init = {
    val genesisLedger = mkSealedEraLedger(1, 0)
    val stagedNextEraLedger = mkSealedEraLedger(2, 1)
    val era0: EraRecord = {
      eraId: 0,
      ledgerId: 1,
      keysetD: 10,
      startS: 0,
      initialLedgerId: 1,
      initialKeysetD: 10,
      initialStartS: 0,
      announcedAtS: 0,
      signerCount: QUORUM_Q,
      endIsSet: false,
      endS: 0,
      endWriteCount: 0,
      eraStatus: EraNormal
    }
    all {
      gov' = {
        nowS: 0,
        contracts: Map(100 -> GENESIS_TABLE_CONTRACT, 1 -> genesisLedger, 2 -> stagedNextEraLedger),
        knownContractIds: Set(100, 1, 2),
        eras: Map(0 -> era0),
        knownEraIds: Set(0),
        currentEraId: 0,
        hasPendingEra: false,
        pendingEraId: 0,
        rosterGen: 0,
        activeRosterD: 500,
        pendingRotation: false,
        pendingRosterD: 0,
        pendingRotationEffS: 0,
        signedVerbNonce: 1,
        premiseTestRan: false,
        recordedPremiseCount: 0,
        premiseA1Pass: false,
        premiseA2Pass: false,
        premiseA3Pass: false,
        rekeyListHasEraRow: false,
        rekeyStarted: false,
        includedCalls: Set(),
        consumerUses: Set(),
        ruledGovParams: Set(),
        t15FreezeWritten: false,
        t15OpenParamsListed: Set(),
        t15StatedUnruledAsRule: false,
        forkStopAnnounced: false,
        forkMigratedBeforeHeight: false,
        newTableHasPredecessorLink: false,
        newSealedLedgersForNextProofVer: false,
        oldSealedContractsMutatedVk: false,
        snapshotRootMarkedTrustDowngrade: false,
        lastRefusal: "none"
      },
      lastOp' = "init"
    }
  }

  // =========================================================================
  // Parameterized actions
  // =========================================================================

  // Advance block time (applying any matured roster rotation per GOV-005 without touching era/claim keys).
  action doAdvanceTime(deltaS: int): bool = all {
    deltaS > 0,
    val nextNow = gov.nowS + deltaS
    val rotMatured = (gov.pendingRotation and nextNow >= gov.pendingRotationEffS)
    gov' = {
      ...gov,
      nowS: nextNow,
      rosterGen: if (rotMatured) gov.rosterGen + 1 else gov.rosterGen,
      activeRosterD: if (rotMatured) gov.pendingRosterD else gov.activeRosterD,
      pendingRotation: if (rotMatured) false else gov.pendingRotation,
      pendingRosterD: if (rotMatured) 0 else gov.pendingRosterD,
      pendingRotationEffS: if (rotMatured) 0 else gov.pendingRotationEffS
    },
    lastOp' = "advanceTime"
  }

  // Implements EGV-001, EGV-002, EGV-003, ERT-001, ERT-002, ELD-001, ELD-002, ELD-006:
  // Deploy a sealed Sherlock era ledger and run the readback gate.
  action doDeploySealedEraLedger(cId: int, eId: int, mutTrig: bool): bool = all {
    not(gov.knownContractIds.contains(cId)),
    val usedThreshold =
      if (MUTANT_ID == MUT_EGV001_THRESHOLD_ZERO and mutTrig) 0 else SEALED_THRESHOLD
    val cRec: DeployedContractRecord = {
      ...mkSealedEraLedger(cId, eId),
      threshold: usedThreshold
    }
    gov' = {
      ...gov,
      contracts: gov.contracts.put(cId, cRec),
      knownContractIds: gov.knownContractIds.union(Set(cId))
    },
    lastOp' = "deploySealedEraLedger"
  }

  // Implements EGV-002, EGV-003, ERT-002, ELD-002:
  // Readback gate catches an unsealed deploy (e.g. compact-js 1-key committee or threshold 0),
  // marks it unusable, and labels any historical receipts as transaction-path evidence only.
  action doCatchUnsealedContractAtReadback(
    cId: int,
    commCnt: int,
    thresh: int,
    keysMatch: bool,
    mutTrig: bool
  ): bool = all {
    not(gov.knownContractIds.contains(cId)),
    commCnt != 0 or thresh != SEALED_THRESHOLD or not(keysMatch),
    val unusableFlag =
      not(MUTANT_ID == MUT_EGV002_KEEP_SDK_COMMITTEE and mutTrig)
    val label =
      if (MUTANT_ID == MUT_EGV003_UNSEALED_SOUNDNESS_LABEL and mutTrig)
        LabelSoundnessEvidence
      else
        LabelTransactionPathOnly
    val badRec: DeployedContractRecord = {
      contractId: cId,
      isEraTable: false,
      sealedEraId: 99,
      committeeCount: commCnt,
      threshold: thresh,
      authCounter: SEALED_COUNTER,
      entryKeysMatchExpected: keysMatch,
      wrapEntryLanes: 85,
      markedUnusable: unusableFlag,
      vkUpdatedAfterDeploy: false,
      reportLabel: label
    }
    gov' = {
      ...gov,
      contracts: gov.contracts.put(cId, badRec),
      knownContractIds: gov.knownContractIds.union(Set(cId)),
      lastRefusal: "readback_gate_failed_contract_unusable"
    },
    lastOp' = "catchUnsealedContractAtReadback"
  }

  // Implements EGV-007, EGV-001, GOV-009: refuse any verifier-key update on a deployed contract.
  action doAttemptVkUpdateOnDeployedContract(cId: int, mutTrig: bool): bool = all {
    gov.knownContractIds.contains(cId),
    val prev = gov.contracts.get(cId)
    val allowMutUpdate = (MUTANT_ID == MUT_EGV007_IN_PLACE_VK_UPDATE and mutTrig)
    val updated: DeployedContractRecord = {
      ...prev,
      vkUpdatedAfterDeploy: prev.vkUpdatedAfterDeploy or allowMutUpdate
    }
    gov' = {
      ...gov,
      contracts: gov.contracts.put(cId, updated),
      lastRefusal: if (allowMutUpdate) gov.lastRefusal else "vk_change_refused_new_era_required"
    },
    lastOp' = "attemptVkUpdateOnDeployedContract"
  }

  // Implements EGV-004, EGV-007, ERT-003, ERT-006, ERT-009, GOV-001, GOV-002, GOV-004, GOV-010:
  // Quorum-signed announce(newEraId, ledgerContractId, keysetDigest, startT).
  action doAnnounceEra(
    newEraId: int,
    ledgerContractId: int,
    keysetDigest: int,
    startT: int,
    signers: int
  ): bool = all {
    canAnnounceEra(gov, newEraId, ledgerContractId, keysetDigest, startT, signers),
    val eRec: EraRecord = {
      eraId: newEraId,
      ledgerId: ledgerContractId,
      keysetD: keysetDigest,
      startS: startT,
      initialLedgerId: ledgerContractId,
      initialKeysetD: keysetDigest,
      initialStartS: startT,
      announcedAtS: gov.nowS,
      signerCount: signers,
      endIsSet: false,
      endS: 0,
      endWriteCount: 0,
      eraStatus: EraNormal
    }
    gov' = {
      ...gov,
      eras: gov.eras.put(newEraId, eRec),
      knownEraIds: gov.knownEraIds.union(Set(newEraId)),
      hasPendingEra: true,
      pendingEraId: newEraId,
      signedVerbNonce: gov.signedVerbNonce + 1
    },
    lastOp' = "announceEra"
  }

  // Implements EGV-004, ERT-003, ERT-009, GOV-001, GOV-002: explicit refusal of invalid announce.
  action doRefuseAnnounceEra(reasonTag: str): bool = all {
    gov' = {
      ...gov,
      lastRefusal: reasonTag
    },
    lastOp' = "refuseAnnounceEra"
  }

  // Implements GOV-003, ERT-004, ERT-005, ERT-007:
  // Permissionless activate(targetEraId) once nowS >= start(targetEraId).
  action doActivateEra(targetEraId: int): bool = all {
    canActivatePendingEra(gov, targetEraId),
    val p = gov.eras.get(targetEraId)
    val oldCur = gov.eras.get(gov.currentEraId)
    val updatedOld: EraRecord =
      if (oldCur.endIsSet) oldCur
      else {
        ...oldCur,
        endIsSet: true,
        endS: p.startS,
        endWriteCount: oldCur.endWriteCount + 1
      }
    gov' = {
      ...gov,
      eras: gov.eras.put(gov.currentEraId, updatedOld),
      currentEraId: targetEraId,
      hasPendingEra: false,
      pendingEraId: 0
    },
    lastOp' = "activateEra"
  }

  // Implements GOV-007: quorum cancel(targetEraId) before start(targetEraId) (clearing pending).
  action doCancelPendingEra(targetEraId: int, signers: int): bool = all {
    signers >= QUORUM_Q,
    signers <= ROSTER_SIZE_N,
    gov.hasPendingEra,
    gov.pendingEraId == targetEraId,
    val p = gov.eras.get(targetEraId)
    all {
      gov.nowS < p.startS,
      val cancelledP: EraRecord = {
        ...p,
        eraStatus: EraCancelled
      }
      gov' = {
        ...gov,
        eras: gov.eras.put(targetEraId, cancelledP),
        hasPendingEra: false,
        pendingEraId: 0,
        signedVerbNonce: gov.signedVerbNonce + 1
      },
      lastOp' = "cancelPendingEra"
    }
  }

  // Implements GOV-006, ERT-004, ERT-007, CRR-004:
  // Quorum soundness_end(targetEraId) for the era that active accepts now:
  // first applies any passed pending activation, sets end(targetEraId) = nowS and status = soundness_ended,
  // and cancels any unstarted pending era.
  action doSoundnessEndEra(targetEraId: int, signers: int, mutOverwriteEnd: bool): bool = all {
    canSoundnessEnd(gov, targetEraId, signers),
    val passedPending =
      gov.hasPendingEra and
      gov.pendingEraId == targetEraId and
      gov.nowS >= gov.eras.get(gov.pendingEraId).startS
    val unstartedPending =
      gov.hasPendingEra and
      gov.pendingEraId != targetEraId and
      gov.nowS < gov.eras.get(gov.pendingEraId).startS
    val curRec = gov.eras.get(gov.currentEraId)
    val erasAfterLazyActivate =
      if (passedPending and not(curRec.endIsSet))
        gov.eras.put(gov.currentEraId, {
          ...curRec,
          endIsSet: true,
          endS: gov.eras.get(targetEraId).startS,
          endWriteCount: curRec.endWriteCount + 1
        })
      else
        gov.eras
    val erasAfterUnstartedCancel =
      if (unstartedPending) {
        val pRec = erasAfterLazyActivate.get(gov.pendingEraId)
        erasAfterLazyActivate.put(gov.pendingEraId, { ...pRec, eraStatus: EraCancelled })
      } else
        erasAfterLazyActivate
    val targetRec = erasAfterUnstartedCancel.get(targetEraId)
    val extraWrite = if (MUTANT_ID == MUT_ERT007_OVERWRITE_END and mutOverwriteEnd) 1 else 0
    val endedTarget: EraRecord = {
      ...targetRec,
      endIsSet: true,
      endS: if (targetRec.endIsSet and targetRec.endS < gov.nowS) targetRec.endS else gov.nowS,
      endWriteCount: (if (targetRec.endIsSet) targetRec.endWriteCount else targetRec.endWriteCount + 1) + extraWrite,
      eraStatus: EraSoundnessEnded
    }
    gov' = {
      ...gov,
      eras: erasAfterUnstartedCancel.put(targetEraId, endedTarget),
      currentEraId: targetEraId,
      hasPendingEra: false,
      pendingEraId: 0,
      signedVerbNonce: gov.signedVerbNonce + 1
    },
    lastOp' = "soundnessEndEra"
  }

  // Implements GOV-005, GOV-011, EGV-007: schedule or cancel a key-preserving Fenrir roster rotation.
  action doScheduleRosterRotation(newRosterD: int, signers: int): bool = all {
    signers >= QUORUM_Q,
    signers <= ROSTER_SIZE_N,
    not(gov.pendingRotation),
    gov' = {
      ...gov,
      pendingRotation: true,
      pendingRosterD: newRosterD,
      pendingRotationEffS: gov.nowS + ROTATE_TIMELOCK_RTL,
      signedVerbNonce: gov.signedVerbNonce + 1
    },
    lastOp' = "scheduleRosterRotation"
  }

  action doCancelRosterRotation(signers: int): bool = all {
    signers >= QUORUM_Q,
    signers <= ROSTER_SIZE_N,
    gov.pendingRotation,
    gov.nowS < gov.pendingRotationEffS,
    gov' = {
      ...gov,
      pendingRotation: false,
      pendingRosterD: 0,
      pendingRotationEffS: 0,
      signedVerbNonce: gov.signedVerbNonce + 1
    },
    lastOp' = "cancelRosterRotation"
  }

  // Implements EGV-009: run T22 premise test on ledger 9.1.0.1 devnet, recording A1, A2, A3 verdicts.
  action doRunPremiseTest(a1: bool, a2: bool, a3: bool, mutIncomplete: bool): bool = all {
    not(gov.rekeyStarted),
    val cnt = if (MUTANT_ID == MUT_EGV009_INCLUDE_AFTER_END and mutIncomplete) 2 else 3
    gov' = {
      ...gov,
      premiseTestRan: true,
      recordedPremiseCount: cnt,
      premiseA1Pass: a1,
      premiseA2Pass: a2,
      premiseA3Pass: a3
    },
    lastOp' = "runPremiseTest"
  }

  // Implements EGV-005: add era-check row to T25 rekey list, start rekey when gated, or refuse rekey.
  action doAddEraRowToRekeyList: bool = all {
    not(gov.rekeyListHasEraRow),
    gov' = {
      ...gov,
      rekeyListHasEraRow: true
    },
    lastOp' = "addEraRowToRekeyList"
  }

  action doStartT25Rekey: bool = all {
    canStartT25Rekey(gov),
    gov' = {
      ...gov,
      rekeyStarted: true
    },
    lastOp' = "startT25Rekey"
  }

  action doRefuseT25Rekey: bool = all {
    not(gov.rekeyListHasEraRow),
    not(isUseTimeFallbackActive(gov)),
    gov' = {
      ...gov,
      lastRefusal: "rekey_refused_missing_era_row_and_premise_test"
    },
    lastOp' = "refuseT25Rekey"
  }

  // Implements EGV-009 Scenario, ELD-003, ELD-004: include a wrap-keyed call on an era ledger
  // (refusing outside [start, end_eff) when primary mode is active).
  action doIncludeLedgerCall(callId: int, eId: int): bool = all {
    canIncludeLedgerCall(gov, eId),
    val activeNow = isEraActiveAt(gov, eId, gov.nowS)
    val rec: IncludedCallRecord = {
      callId: callId,
      eraId: eId,
      includedAtS: gov.nowS,
      fallbackModeActive: isUseTimeFallbackActive(gov),
      eraActiveAtInclusion: activeNow
    }
    gov' = {
      ...gov,
      includedCalls: gov.includedCalls.union(Set(rec))
    },
    lastOp' = "includeLedgerCall"
  }

  action doRefuseInactiveEraInclusion(eId: int): bool = all {
    gov.knownEraIds.contains(eId),
    not(isEraActiveAt(gov, eId, gov.nowS)),
    gov' = {
      ...gov,
      lastRefusal: "era_inactive_at_inclusion_no_write"
    },
    lastOp' = "refuseInactiveEraInclusion"
  }

  // Implements EGV-006, ELD-005, ELD-007, CRR-004, CRR-006, CRR-007:
  // Consumer contract uses a claim (checking soundness_end always, and checking active(era_id) under fallback).
  action doConsumerUseClaim(useId: int, eId: int): bool = all {
    canConsumerUseClaim(gov, eId),
    val e = gov.eras.get(eId)
    val rec: ConsumerUseRecord = {
      useId: useId,
      eraId: eId,
      usedAtS: gov.nowS,
      fallbackModeAtUse: isUseTimeFallbackActive(gov),
      eraActiveAtUse: isEraActiveAt(gov, eId, gov.nowS),
      eraSoundnessEndedAtUse: (e.eraStatus == EraSoundnessEnded),
      wroteSherlockState: false
    }
    gov' = {
      ...gov,
      consumerUses: gov.consumerUses.union(Set(rec))
    },
    lastOp' = "consumerUseClaim"
  }

  action doRefuseConsumerUseClaim(eId: int): bool = all {
    gov.knownEraIds.contains(eId),
    val e = gov.eras.get(eId)
    all {
      (e.eraStatus == EraSoundnessEnded) or
        (isUseTimeFallbackActive(gov) and not(isEraActiveAt(gov, eId, gov.nowS))),
      gov' = {
        ...gov,
        lastRefusal: "consumer_claim_use_refused"
      },
      lastOp' = "refuseConsumerUseClaim"
    }
  }

  // Implements EGV-008: write T15 freeze record listing any unruled governance parameters among
  // {TL, RTL, Q, n, GRID, SKEW} as open with their recommended values, never stating them as rules.
  action doWriteT15GovernanceFreeze(ruledSubset: Set[str], mutTrig: bool): bool = all {
    ruledSubset.subseteq(ALL_GOV_PARAMS),
    val openExpected = ALL_GOV_PARAMS.exclude(ruledSubset)
    val listedSet =
      if (MUTANT_ID == MUT_EGV008_OMIT_OPEN_PARAMS and mutTrig) Set()
      else openExpected
    val statedAsRule =
      (MUTANT_ID == MUT_EGV008_OMIT_OPEN_PARAMS and mutTrig)
    gov' = {
      ...gov,
      ruledGovParams: ruledSubset,
      t15FreezeWritten: true,
      t15OpenParamsListed: listedSet,
      t15StatedUnruledAsRule: statedAsRule
    },
    lastOp' = "writeT15GovernanceFreeze"
  }

  // Implements EGV-010: announce a Midnight proof-version-stopping hard fork and deploy a new era table
  // and new sealed ledgers with a predecessor link and trust-downgraded snapshot root before fork height.
  action doAnnounceProofVersionFork: bool = all {
    not(gov.forkStopAnnounced),
    gov' = {
      ...gov,
      forkStopAnnounced: true
    },
    lastOp' = "announceProofVersionFork"
  }

  action doMigrateBeforeProofVersionFork(mutTrig: bool): bool = all {
    gov.forkStopAnnounced,
    not(gov.forkMigratedBeforeHeight),
    val badMigration = (MUTANT_ID == MUT_EGV010_FORK_MUTATE_OR_NO_PRED and mutTrig)
    gov' = {
      ...gov,
      forkMigratedBeforeHeight: true,
      newTableHasPredecessorLink: not(badMigration),
      newSealedLedgersForNextProofVer: true,
      oldSealedContractsMutatedVk: badMigration,
      snapshotRootMarkedTrustDowngrade: not(badMigration)
    },
    lastOp' = "migrateBeforeProofVersionFork"
  }

  // =========================================================================
  // Nondeterministic step relation
  // =========================================================================

  action step = any {
    // Advance clock on GRID_STEP increments so lazy & explicit activations both occur.
    {
      nondet dS = Set(GRID_STEP, TIMELOCK_TL).oneOf()
      doAdvanceTime(dS)
    },

    // Contract deployments, readback gate catches, and refused in-place VK updates (EGV-001..003, EGV-007)
    {
      nondet cId = Set(3, 4).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doDeploySealedEraLedger(cId, cId - 1, mutTrig)
    },
    {
      nondet badKind = Set(1, 2).oneOf()
      val comm = if (badKind == 1) 1 else 0
      val thr = if (badKind == 1) 1 else 0
      nondet mutTrig = Set(true, false).oneOf()
      doCatchUnsealedContractAtReadback(50, comm, thr, true, mutTrig)
    },
    {
      nondet mutTrig = Set(true, false).oneOf()
      doAttemptVkUpdateOnDeployedContract(1, mutTrig)
    },

    // Era table quorum verbs: announce, activate, cancel, soundness_end, roster rotation (EGV-004, EGV-007, ERT, GOV)
    {
      nondet eId = Set(1, 2).oneOf()
      val ldgPool =
        if (MUTANT_ID == MUT_EGV002_KEEP_SDK_COMMITTEE) Set(2, 50)
        else Set(2, 3)
      nondet lId = ldgPool.oneOf()
      val ksPool =
        if (MUTANT_ID == MUT_EGV004_REUSE_KEYSET) Set(10, 20)
        else Set(20, 30)
      nondet ksD = ksPool.oneOf()
      val alignedNow =
        if (gov.nowS % GRID_STEP == 0) gov.nowS
        else gov.nowS + (GRID_STEP - (gov.nowS % GRID_STEP))
      val startPool =
        if (MUTANT_ID == MUT_EGV007_SUBQUORUM_OR_NO_TL) Set(gov.nowS, alignedNow + TIMELOCK_TL)
        else Set(alignedNow + TIMELOCK_TL)
      nondet sT = startPool.oneOf()
      val sigPool =
        if (MUTANT_ID == MUT_EGV007_SUBQUORUM_OR_NO_TL) Set(1, QUORUM_Q)
        else Set(QUORUM_Q)
      nondet sigs = sigPool.oneOf()
      doAnnounceEra(eId, lId, ksD, sT, sigs)
    },
    {
      nondet rTag = Set("duplicate_keyset_d", "below_quorum_Q", "off_grid_start").oneOf()
      doRefuseAnnounceEra(rTag)
    },
    {
      nondet eId = Set(1, 2).oneOf()
      doActivateEra(eId)
    },
    {
      nondet eId = Set(1, 2).oneOf()
      doCancelPendingEra(eId, QUORUM_Q)
    },
    {
      nondet eId = Set(0, 1, 2).oneOf()
      nondet mutOw = Set(true, false).oneOf()
      doSoundnessEndEra(eId, QUORUM_Q, mutOw)
    },
    doScheduleRosterRotation(501, QUORUM_Q),
    doCancelRosterRotation(QUORUM_Q),

    // Devnet premise test, T25 rekey gate, call inclusion, and consumer use (EGV-005, EGV-006, EGV-009)
    {
      nondet a2 = Set(true, false).oneOf()
      nondet a3 = Set(true, false).oneOf()
      nondet mutInc = Set(true, false).oneOf()
      doRunPremiseTest(true, a2, a3, mutInc)
    },
    doAddEraRowToRekeyList,
    doStartT25Rekey,
    doRefuseT25Rekey,
    {
      nondet eId = Set(0, 1).oneOf()
      doIncludeLedgerCall(1, eId)
    },
    {
      nondet eId = Set(0, 1).oneOf()
      doRefuseInactiveEraInclusion(eId)
    },
    {
      nondet eId = Set(0, 1).oneOf()
      doConsumerUseClaim(1, eId)
    },
    {
      nondet eId = Set(0, 1).oneOf()
      doRefuseConsumerUseClaim(eId)
    },

    // T15 governance freeze & proof-version hard fork migration (EGV-008, EGV-010)
    {
      nondet ruled = Set(Set(), Set("GRID")).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doWriteT15GovernanceFreeze(ruled, mutTrig)
    },
    doAnnounceProofVersionFork,
    {
      nondet mutTrig = Set(true, false).oneOf()
      doMigrateBeforeProofVersionFork(mutTrig)
    }
  }

  // =========================================================================
  // Safety invariants (one inv_<ID> per expressible requirement)
  // =========================================================================

  // EGV-001 (and ERT-001, ELD-001): Every usable deployed Sherlock contract has sealed maintenance
  // authority {committee: [], threshold: 1, counter: 0}.
  val inv_EGV_001: bool =
    gov.knownContractIds.forall(cId => {
      val c = gov.contracts.get(cId)
      not(c.markedUnusable) implies (
        c.committeeCount == 0 and
        c.threshold == SEALED_THRESHOLD and
        c.authCounter == SEALED_COUNTER
      )
    })

  // EGV-002 (and ERT-002, ELD-002): Any contract whose readback differs from {[], 1, 0} or expected entry
  // keys is marked unusable, and no era announcement names an unusable or unsealed contract.
  val inv_EGV_002: bool =
    gov.knownContractIds.forall(cId => {
      val c = gov.contracts.get(cId)
      (c.committeeCount != 0 or c.threshold != SEALED_THRESHOLD or
        c.authCounter != SEALED_COUNTER or not(c.entryKeysMatchExpected)) implies c.markedUnusable
    }) and
    gov.knownEraIds.forall(eId => {
      val lId = gov.eras.get(eId).ledgerId
      gov.knownContractIds.contains(lId) and
      not(gov.contracts.get(lId).markedUnusable) and
      isContractSealedAndValid(gov.contracts.get(lId))
    })

  // EGV-003: Any unsealed SherlockLedger listed in the prototype report has its receipts labelled
  // solely as transaction-path evidence (never soundness evidence).
  val inv_EGV_003: bool =
    gov.knownContractIds.forall(cId => {
      val c = gov.contracts.get(cId)
      val isSealed = (c.committeeCount == 0 and c.threshold == SEALED_THRESHOLD and c.authCounter == SEALED_COUNTER)
      (not(c.isEraTable) and not(isSealed) and c.reportLabel != LabelUnreported) implies (
        c.reportLabel == LabelTransactionPathOnly
      )
    })

  // EGV-004 (and ERT-003, ERT-006): One era per keyset_d and per ledger_addr, and era table records
  // keep their announced (ledger_addr, keyset_d, start) values append-only.
  val inv_EGV_004: bool =
    gov.knownEraIds.forall(e1 => {
      val r1 = gov.eras.get(e1)
      r1.ledgerId == r1.initialLedgerId and
      r1.keysetD == r1.initialKeysetD and
      r1.startS == r1.initialStartS and
      gov.knownEraIds.forall(e2 =>
        (e1 != e2) implies (
          r1.keysetD != gov.eras.get(e2).keysetD and
          r1.ledgerId != gov.eras.get(e2).ledgerId
        )
      )
    })

  // EGV-005: T25 rekey starts only when the rekey list holds the era-check row OR the premise test
  // recorded a failure for premise A2 or A3.
  val inv_EGV_005: bool =
    gov.rekeyStarted implies (gov.rekeyListHasEraRow or isUseTimeFallbackActive(gov))

  // EGV-006 (and CRR-004, CRR-006, CRR-007): Consumer claim use refuses soundness-ended eras always,
  // checks active(era_id) when premise A2 or A3 failed, and writes nothing on Sherlock contracts.
  val inv_EGV_006: bool =
    gov.consumerUses.forall(u =>
      not(u.eraSoundnessEndedAtUse) and
      not(u.wroteSherlockState) and
      (u.fallbackModeAtUse implies u.eraActiveAtUse)
    )

  // EGV-007 (and GOV-001, GOV-002, GOV-004, GOV-005, GOV-009, ERT-009):
  // No verifier-key update ever mutates a deployed contract; new key sets enter only as quorum-signed
  // new eras respecting TIMELOCK_TL and GRID_STEP, and roster rotations keep all era keyset_d/ledgerId values.
  val inv_EGV_007: bool =
    gov.knownContractIds.forall(cId => not(gov.contracts.get(cId).vkUpdatedAfterDeploy)) and
    gov.knownEraIds.forall(eId => {
      val e = gov.eras.get(eId)
      (eId > 0) implies (
        e.signerCount >= QUORUM_Q and
        e.startS >= e.announcedAtS + TIMELOCK_TL and
        e.startS % GRID_STEP == 0
      )
    })

  // EGV-008: Any parameter among {TL, RTL, Q, n, GRID, SKEW} with no owner ruling at T15 is listed
  // as open in the T15 freeze record, and none is stated as a fixed rule without a ruling.
  val inv_EGV_008: bool =
    gov.t15FreezeWritten implies (
      ALL_GOV_PARAMS.exclude(gov.ruledGovParams).subseteq(gov.t15OpenParamsListed) and
      not(gov.t15StatedUnruledAsRule)
    )

  // EGV-009 (and ELD-003, ELD-004, ELD-006): Devnet premise test records verdicts for all 3 premises
  // A1, A2, A3; in primary mode, no call included outside [start, end_eff) writes to an era ledger;
  // wrap-keyed entries fit L_TX <= 1,024 lanes.
  val inv_EGV_009: bool =
    (gov.premiseTestRan implies (gov.recordedPremiseCount == 3)) and
    gov.includedCalls.forall(ic =>
      not(ic.fallbackModeActive) implies ic.eraActiveAtInclusion
    ) and
    gov.knownContractIds.forall(cId =>
      gov.contracts.get(cId).wrapEntryLanes <= L_TX_LANE_LIMIT
    )

  // EGV-010: Proof-version-stopping hard fork migration deploys a new era table with predecessor link
  // and new sealed ledgers, mutates no old sealed contract key, and marks the snapshot root as a trust downgrade.
  val inv_EGV_010: bool =
    gov.forkMigratedBeforeHeight implies (
      gov.forkStopAnnounced and
      gov.newTableHasPredecessorLink and
      gov.newSealedLedgersForNextProofVer and
      not(gov.oldSealedContractsMutatedVk) and
      gov.snapshotRootMarkedTrustDowngrade
    )

  // Adopted ERT-004, ERT-005, ERT-007: At most one era accepts at gov.nowS, cancelled/soundness-ended
  // eras never accept, and end(e) is written at most once per era.
  val inv_ERT_004_005_007: bool = {
    val activeNowSet = gov.knownEraIds.filter(eId => isEraActiveAt(gov, eId, gov.nowS))
    activeNowSet.size() <= 1 and
    gov.knownEraIds.forall(eId => {
      val e = gov.eras.get(eId)
      e.endWriteCount <= 1 and
      ((e.eraStatus != EraNormal) implies not(activeNowSet.contains(eId)))
    })
  }

  // Master safety invariant.
  val inv: bool =
    consistency_constants and
    inv_EGV_001 and
    inv_EGV_002 and
    inv_EGV_003 and
    inv_EGV_004 and
    inv_EGV_005 and
    inv_EGV_006 and
    inv_EGV_007 and
    inv_EGV_008 and
    inv_EGV_009 and
    inv_EGV_010 and
    inv_ERT_004_005_007

  // =========================================================================
  // Reachability witnesses (negated goals — MUST be violated under governance_main)
  // =========================================================================

  // Witness 1: Deploy sealed era ledger, quorum-announce era 1, advance past TIMELOCK_TL, and activate era 1.
  val reach_era_announce_and_activate: bool = not(
    gov.currentEraId == 1 and
    gov.eras.get(0).endIsSet and
    isEraActiveAt(gov, 1, gov.nowS)
  )

  // Witness 2: Soundness-end an active era and refuse consumer claim use on it (GOV-006, CRR-004).
  val reach_soundness_end_and_consumer_refusal: bool = not(
    gov.knownEraIds.exists(eId => gov.eras.get(eId).eraStatus == EraSoundnessEnded) and
    gov.lastRefusal == "consumer_claim_use_refused"
  )

  // Witness 3: Readback gate catches an unsealed SDK deploy and labels receipts as path evidence only (EGV-002, EGV-003).
  val reach_readback_catches_unsealed_sdk_deploy: bool = not(
    gov.knownContractIds.exists(cId => {
      val c = gov.contracts.get(cId)
      c.markedUnusable and c.reportLabel == LabelTransactionPathOnly
    })
  )

  // Witness 4: Premise test runs, fallback consumer era check activates, and T25 rekey starts (EGV-005, EGV-006, EGV-009).
  val reach_premise_test_fallback_and_rekey: bool = not(
    gov.premiseTestRan and
    isUseTimeFallbackActive(gov) and
    gov.rekeyStarted and
    gov.consumerUses.exists(u => u.fallbackModeAtUse and u.eraActiveAtUse)
  )

  // Witness 5: Roster rotation takes effect without changing era keys, and VK update request is refused (EGV-007, GOV-005).
  val reach_roster_rotation_and_vk_update_refusal: bool = not(
    gov.rosterGen >= 1 and
    gov.activeRosterD == 501 and
    gov.lastRefusal == "vk_change_refused_new_era_required"
  )

  // Witness 6: T15 open governance parameters recorded and proof-version fork migration completed (EGV-008, EGV-010).
  val reach_t15_open_params_and_fork_migration: bool = not(
    gov.t15FreezeWritten and
    gov.t15OpenParamsListed.contains("TL") and
    gov.t15OpenParamsListed.contains("SKEW") and
    gov.forkMigratedBeforeHeight
  )
}

// ===========================================================================
// Concrete instances: main, edge instances, and guard mutants
// ===========================================================================

module governance_main {
  import governance(
    SEALED_THRESHOLD = 1,
    SEALED_COUNTER = 0,
    ROSTER_SIZE_N = 16,
    QUORUM_Q = 12,
    TIMELOCK_TL = 70,
    ROTATE_TIMELOCK_RTL = 30,
    GRID_STEP = 10,
    L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 0
  ).*
}

// Smallest legal quorum/timelock/grid/lane values (Q = 2 of n = 3, TL = 10, RTL = 10, GRID = 1, L_TX = 91).
module governance_edge_min {
  import governance(
    SEALED_THRESHOLD = 1,
    SEALED_COUNTER = 0,
    ROSTER_SIZE_N = 3,
    QUORUM_Q = 2,
    TIMELOCK_TL = 10,
    ROTATE_TIMELOCK_RTL = 10,
    GRID_STEP = 1,
    L_TX_LANE_LIMIT = 91,
    MUTANT_ID = 0
  ).*
}

// Unanimous quorum at limit (Q = 16 of n = 16, L_TX = 1,024).
module governance_edge_at_limit {
  import governance(
    SEALED_THRESHOLD = 1,
    SEALED_COUNTER = 0,
    ROSTER_SIZE_N = 16,
    QUORUM_Q = 16,
    TIMELOCK_TL = 70,
    ROTATE_TIMELOCK_RTL = 30,
    GRID_STEP = 10,
    L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 0
  ).*
}

// Mutants (each removes or weakens one refusal/binding guard and must break its named invariant)

module governance_m_egv001_threshold_zero {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 1
  ).*
}

module governance_m_egv002_keep_sdk_committee {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 2
  ).*
}

module governance_m_egv003_unsealed_soundness_label {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 3
  ).*
}

module governance_m_egv004_reuse_keyset {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 4
  ).*
}

module governance_m_egv005_rekey_no_era_no_fail {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 5
  ).*
}

module governance_m_egv006_fallback_ended_era_use {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 6
  ).*
}

module governance_m_egv007_in_place_vk_update {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 7
  ).*
}

module governance_m_egv007_subquorum_or_no_tl {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 8
  ).*
}

module governance_m_egv008_omit_open_params {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 9
  ).*
}

module governance_m_egv009_include_after_end {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 10
  ).*
}

module governance_m_egv010_fork_mutate_or_no_pred {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 11
  ).*
}

module governance_m_ert007_overwrite_end {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 12
  ).*
}

module governance_m_crr004_use_soundness_ended {
  import governance(
    SEALED_THRESHOLD = 1, SEALED_COUNTER = 0, ROSTER_SIZE_N = 16, QUORUM_Q = 12,
    TIMELOCK_TL = 70, ROTATE_TIMELOCK_RTL = 30, GRID_STEP = 10, L_TX_LANE_LIMIT = 1024,
    MUTANT_ID = 13
  ).*
}
