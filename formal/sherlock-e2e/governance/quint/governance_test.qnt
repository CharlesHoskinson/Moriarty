// -*- mode: Bluespec; -*-
// Deterministic scenario tests for bounded context Governance (lane e2e-governance).

module governance_test {
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
  ).* from "./governance"

  // EGV-001, EGV-002, EGV-003, EGV-007, ERT-001, ERT-002, ELD-001, ELD-002:
  // Sealed contracts deploy with {[], 1, 0}; compact-js 1-key committee and threshold-0 deploys
  // are caught by readback, marked unusable, refused in announce, and labelled transaction-path evidence;
  // verifier-key updates on deployed contracts are refused.
  run gv_sealingReadbackPathEvidenceAndNoInPlaceVkUpdateTest =
    init
      .expect(inv and lastOp == "init")
      .then(doDeploySealedEraLedger(3, 2, false))
      .expect(
        inv and
        isContractSealedAndValid(gov.contracts.get(3)) and
        not(gov.contracts.get(3).markedUnusable)
      )
      .then(doCatchUnsealedContractAtReadback(50, 1, 1, true, false))
      .then(doCatchUnsealedContractAtReadback(51, 0, 0, true, false))
      .expect(
        inv and
        gov.contracts.get(50).markedUnusable and
        gov.contracts.get(50).reportLabel == LabelTransactionPathOnly and
        gov.contracts.get(51).markedUnusable and
        gov.contracts.get(51).reportLabel == LabelTransactionPathOnly and
        not(canAnnounceEra(gov, 1, 50, 20, 70, 12)) and
        not(canAnnounceEra(gov, 1, 51, 20, 70, 12))
      )
      .then(doAttemptVkUpdateOnDeployedContract(1, false))
      .expect(
        inv and
        not(gov.contracts.get(1).vkUpdatedAfterDeploy) and
        gov.lastRefusal == "vk_change_refused_new_era_required"
      )

  // EGV-004, EGV-007, ERT-003..007, ERT-009, GOV-001..004:
  // Duplicate keyset_d, sub-quorum (< 12), early start (< now + TL), and off-grid start are refused;
  // lazy pending transition at start(1) switches active(0)->false and active(1)->true before activate(1) runs;
  // activate(1) writes end(0) = start(1) once and keeps append-only era 0 fields.
  run gv_eraAnnounceQuorumGridLazyTransitionAndActivateTest =
    init
      .expect(
        not(canAnnounceEra(gov, 1, 2, 10, 70, 12)) and
        not(canAnnounceEra(gov, 1, 2, 20, 70, 11)) and
        not(canAnnounceEra(gov, 1, 2, 20, 60, 12)) and
        not(canAnnounceEra(gov, 1, 2, 20, 71, 12)) and
        canAnnounceEra(gov, 1, 2, 20, 70, 12)
      )
      .then(doAnnounceEra(1, 2, 20, 70, 12))
      .expect(
        inv and
        gov.hasPendingEra and
        isEraActiveAt(gov, 0, 0) and
        not(isEraActiveAt(gov, 1, 0)) and
        not(canActivatePendingEra(gov, 1))
      )
      .then(doAdvanceTime(70))
      .expect(
        inv and
        not(isEraActiveAt(gov, 0, gov.nowS)) and
        isEraActiveAt(gov, 1, gov.nowS) and
        canActivatePendingEra(gov, 1)
      )
      .then(doActivateEra(1))
      .expect(
        inv and
        gov.currentEraId == 1 and
        not(gov.hasPendingEra) and
        gov.eras.get(0).endIsSet and
        gov.eras.get(0).endS == 70 and
        gov.eras.get(0).endWriteCount == 1 and
        gov.eras.get(0).keysetD == 10
      )

  // GOV-005, GOV-006, GOV-007, GOV-011, CRR-004:
  // Roster rotation can be cancelled before RTL or mature after RTL while preserving era keyset_d;
  // soundness_end refuses old era after pending start has passed, applies lazy pending activation,
  // marks era 1 soundness_ended permanently, and blocks consumer use of era 1 claims.
  run gv_rosterRotationAndSoundnessEndAfterLazyPendingStartTest =
    init
      .then(doScheduleRosterRotation(501, 12))
      .then(doCancelRosterRotation(12))
      .expect(inv and not(gov.pendingRotation) and gov.activeRosterD == 500)
      .then(doScheduleRosterRotation(502, 12))
      .then(doAnnounceEra(1, 2, 20, 70, 12))
      .then(doAdvanceTime(70))
      .expect(
        inv and
        gov.activeRosterD == 502 and
        gov.eras.get(0).keysetD == 10 and
        not(canSoundnessEnd(gov, 0, 12)) and
        canSoundnessEnd(gov, 1, 12)
      )
      .then(doSoundnessEndEra(1, 12, false))
      .expect(
        inv and
        gov.eras.get(1).eraStatus == EraSoundnessEnded and
        not(isEraActiveAt(gov, 1, gov.nowS)) and
        not(canConsumerUseClaim(gov, 1)) and
        canConsumerUseClaim(gov, 0)
      )
      .then(doRefuseConsumerUseClaim(1))
      .expect(inv and gov.lastRefusal == "consumer_claim_use_refused")
      .then(doConsumerUseClaim(1, 0))
      .expect(inv)

  // EGV-005, EGV-006, EGV-009, ELD-003..005, ELD-007, CRR-006, CRR-007:
  // Today's rekey list (no era row, no failed A2/A3 premise) refuses T25 rekey;
  // in primary mode (A1..A3 pass), calls included after end_eff are refused while consumer reads of
  // peacefully ended era 0 succeed; when A2 fails, fallback activates, unlocks rekey, and refuses
  // consumer use of ended era 0.
  run gv_rekeyGatePrimaryInclusionAndUseTimeFallbackTest =
    init
      .expect(not(canStartT25Rekey(gov)))
      .then(doRefuseT25Rekey)
      .expect(inv and gov.lastRefusal == "rekey_refused_missing_era_row_and_premise_test")
      .then(doRunPremiseTest(true, true, true, false))
      .expect(inv and gov.recordedPremiseCount == 3 and not(canStartT25Rekey(gov)))
      .then(doAnnounceEra(1, 2, 20, 70, 12))
      .then(doIncludeLedgerCall(1, 0))
      .then(doAdvanceTime(70))
      .expect(
        inv and
        not(canIncludeLedgerCall(gov, 0)) and
        canIncludeLedgerCall(gov, 1) and
        canConsumerUseClaim(gov, 0)
      )
      .then(doRefuseInactiveEraInclusion(0))
      .expect(inv and gov.lastRefusal == "era_inactive_at_inclusion_no_write")
      .then(doConsumerUseClaim(1, 0))
      .expect(inv)
      .then(doRunPremiseTest(true, false, true, false))
      .expect(
        inv and
        isUseTimeFallbackActive(gov) and
        canStartT25Rekey(gov) and
        not(canConsumerUseClaim(gov, 0)) and
        canConsumerUseClaim(gov, 1)
      )
      .then(doStartT25Rekey)
      .then(doRefuseConsumerUseClaim(0))
      .then(doConsumerUseClaim(2, 1))
      .expect(inv and gov.rekeyStarted and gov.lastRefusal == "consumer_claim_use_refused")

  // EGV-008, EGV-010, and pure spec-gap witnesses:
  // Unruled governance parameters among {TL, RTL, Q, n, GRID, SKEW} are listed as open at T15;
  // proof-version hard fork deploys new table with predecessor link, new sealed v3 ledgers, and
  // trust-downgraded snapshot root without mutating old sealed contracts.
  run gv_t15OpenGovernanceParamsForkMigrationAndSpecGapsTest =
    init
      .expect(
        consistency_constants and
        gap_q_gov_4_premise_a1_fail_neither_branch and
        gap_q_gov_2_zero_ledger_addr_collides_under_ert003
      )
      .then(doWriteT15GovernanceFreeze(Set("GRID"), false))
      .expect(
        inv and
        gov.t15FreezeWritten and
        gov.t15OpenParamsListed == Set("TL", "RTL", "Q", "n", "SKEW") and
        not(gov.t15StatedUnruledAsRule)
      )
      .then(doAnnounceProofVersionFork)
      .then(doMigrateBeforeProofVersionFork(false))
      .expect(
        inv and
        gov.forkMigratedBeforeHeight and
        gov.newTableHasPredecessorLink and
        gov.newSealedLedgersForNextProofVer and
        not(gov.oldSealedContractsMutatedVk) and
        gov.snapshotRootMarkedTrustDowngrade
      )
}
