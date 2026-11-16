// -*- mode: Bluespec; -*-
// Deterministic scenario tests for bounded context Placement (settlement edge) (lane e2e-placement).

module placement_test {
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
  ).* from "./placement"

  // 1. Honest box wrap from huginn -> muninn ledger 9.1.0.1 check -> KEEP -> settle submit
  // Covers HLP-001, HLP-002, HLP-003, HLP-004, HLP-008.
  run pl_honestBoxWrapKeepAndSettleTest =
    init
      .expect(inv and lastOp == "init")
      .then(doBoxFinishAndReturnResult(12, BOX_HUGINN, FaultNone, false, false))
      .expect(
        inv and
        plc.jobs.get(12).resultPresent and
        plc.jobs.get(12).argBuilderRanOnBox and
        not(plc.jobs.get(12).acceptBundleLeftBox) and
        plc.jobs.get(12).returnedPublicWordCount == 4 and
        not(plc.jobs.get(12).privatePartInResult) and
        plc.jobs.get(12).disposition == ResReturnedUnchecked and
        // Unchecked box wrap cannot be submitted before muninn check (HLP-001, HLP-002)
        not(canSubmitJobCall(plc.jobs.get(12), false))
      )
      .then(doMuninnCheckBoxResult(12, false))
      .expect(
        inv and
        plc.jobs.get(12).verifierInvoked and
        plc.jobs.get(12).muninnChecked and
        plc.jobs.get(12).ledger9101VerifiedOk and
        plc.jobs.get(12).disposition == ResKept and
        canSubmitJobCall(plc.jobs.get(12), false)
      )
      .then(doSubmitSettledCall(12, false))
      .expect(
        inv and
        plc.jobs.get(12).submittedToChain and
        not(canSubmitJobCall(plc.jobs.get(12), false))
      )

  // 2. Refused box wrap from odin is dropped (no submit) and reproved cleanly
  // Covers HLP-001, HLP-002.
  run pl_refusedBoxWrapDroppedAndReprovedTest =
    init
      .then(doBoxFinishAndReturnResult(13, BOX_ODIN, FaultInvalidProof, false, false))
      .then(doMuninnCheckBoxResult(13, false))
      .expect(
        inv and
        plc.jobs.get(13).verifierInvoked and
        plc.jobs.get(13).muninnChecked and
        not(plc.jobs.get(13).ledger9101VerifiedOk) and
        plc.jobs.get(13).disposition == ResDropped and
        plc.jobs.get(13).reproveRequested and
        plc.lastRefusal == "ledger_9101_refused_box_wrap_dropped" and
        not(canSubmitJobCall(plc.jobs.get(13), false))
      )
      // Reprove job 13 on odin with a valid wrap proof, verify on muninn, and submit
      .then(doBoxFinishAndReturnResult(13, BOX_ODIN, FaultNone, false, false))
      .then(doMuninnCheckBoxResult(13, false))
      .expect(
        inv and
        plc.jobs.get(13).disposition == ResKept and
        plc.jobs.get(13).ledger9101VerifiedOk
      )
      .then(doSubmitSettledCall(13, false))
      .expect(inv and plc.jobs.get(13).submittedToChain)

  // 3. Corrupted SHA-256 and swapped valid proof of job 12 for job 13 dropped BEFORE proof verification
  // Covers HLP-008 (both SHA-256 mismatch and lanes 20, 34, 39 call-vector mismatch).
  run pl_badSha256AndSwappedJobCallVectorDroppedPreVerifyTest =
    init
      .then(doBoxFinishAndReturnResult(12, BOX_HUGINN, FaultBadSha, false, false))
      .then(doMuninnCheckBoxResult(12, false))
      .expect(
        inv and
        plc.jobs.get(12).muninnChecked and
        not(plc.jobs.get(12).verifierInvoked) and
        plc.jobs.get(12).disposition == ResDropped and
        plc.lastRefusal == "box_file_sha256_mismatch_dropped" and
        not(canSubmitJobCall(plc.jobs.get(12), false))
      )
      // Valid proof of job 12 returned for job 13: lanes 20, 34, and 39 differ -> dropped pre-verify
      .then(doBoxFinishAndReturnResult(13, BOX_ODIN, FaultWrongJobVec, false, false))
      .expect(
        plc.jobs.get(13).returnedVec.cpLane10 == plc.jobs.get(13).expectedVec.cpLane10 and
        plc.jobs.get(13).returnedVec.claimLane20 != plc.jobs.get(13).expectedVec.claimLane20 and
        plc.jobs.get(13).returnedVec.claimLane34 != plc.jobs.get(13).expectedVec.claimLane34 and
        plc.jobs.get(13).returnedVec.outcomeLane39 != plc.jobs.get(13).expectedVec.outcomeLane39
      )
      .then(doMuninnCheckBoxResult(13, false))
      .expect(
        inv and
        plc.jobs.get(13).muninnChecked and
        not(plc.jobs.get(13).verifierInvoked) and
        plc.jobs.get(13).disposition == ResDropped and
        plc.lastRefusal == "box_call_vector_job_mismatch_dropped"
      )

  // 4. Case A host ownership, private bundle/tape confinement, and walletless proving-box tx refusal
  // Covers HLP-003, HLP-004, HLP-005, HLP-007.
  run pl_caseAOnlyUnownedProverAndWalletPlacementRefusalsTest =
    init
      .expect(
        inv and
        // Shipping accept-bundle or leaking private words from proving box is blocked (HLP-003, HLP-004)
        not(canBoxReturnResult(plc, 12, BOX_HUGINN, true, false)) and
        not(canBoxReturnResult(plc, 12, BOX_HUGINN, false, true)) and
        // Unowned host cannot produce results or pass prover config guard (HLP-005)
        not(canBoxReturnResult(plc, 12, HOST_UNOWNED_HELPER, false, false)) and
        not(canAcceptProverHostConfig(plc, HOST_UNOWNED_HELPER)) and
        canAcceptProverHostConfig(plc, BOX_HUGINN) and
        // Proving boxes hold no wallet directory and cannot sign/submit transactions (HLP-007)
        not(canBoxSignOrSubmitTx(plc, BOX_ODIN)) and
        canBoxSignOrSubmitTx(plc, BOX_MUNINN)
      )
      .then(doEvaluateProverConfig(BOX_HUGINN, "witness"))
      .expect(inv and plc.caseAConfigsAccepted == 1)
      .then(doEvaluateProverConfig(HOST_UNOWNED_HELPER, "witness"))
      .expect(
        inv and
        plc.unownedConfigRefusals == 1 and
        not(plc.unownedHostConfigAccepted) and
        plc.unownedArtifactsSent == Set() and
        plc.lastRefusal == "unowned_prover_host_refused"
      )
      .then(doRequestTxSignOrSubmitOnBox(BOX_ODIN, false))
      .expect(
        inv and
        plc.walletlessTxRefusalsOnBus == Set(BOX_ODIN) and
        plc.boxesThatSignedOrSubmitted == Set() and
        plc.lastRefusal == "proving_box_has_no_wallet"
      )
      .then(doRequestTxSignOrSubmitOnBox(BOX_MUNINN, false))
      .expect(
        inv and
        plc.boxesThatSignedOrSubmitted == Set(BOX_MUNINN)
      )

  // 5. Adopted ZK-009 placement guards and pure spec-gap witnesses (Q-placement-1..4)
  run pl_zk009PlacementGuardsAndSpecGapWitnessesTest =
    init
      .expect(
        inv and
        consistency_constants and
        gap_q_placement_1_job_swap_and_four_words_vs_three_lanes and
        gap_q_placement_2_box_private_vs_owned_roster_private and
        gap_q_placement_3_hlp005_misses_zk009_wrap_msm and
        gap_q_placement_4_negative_guards_vacuity and
        not(canRunWrapPlacement(true, false, 1, true)) and
        not(canRunWrapPlacement(false, true, 1, true)) and
        not(canRunWrapPlacement(false, false, 2, true)) and
        canRunWrapPlacement(false, false, 1, true)
      )
      .then(doCheckWrapAndBlindingPlacement(1, false))
      .expect(inv and plc.zk009Refusals == 1 and not(plc.remoteUnmaskedWrapOp))
      .then(doCheckWrapAndBlindingPlacement(2, false))
      .expect(inv and plc.zk009Refusals == 2 and not(plc.blindingSlotReused) and not(plc.helperMadeBlindingSlot))
      .then(doCheckWrapAndBlindingPlacement(3, false))
      .expect(inv and plc.zk009Refusals == 3 and plc.callsPerWrap == 1)
      .then(doCheckWrapAndBlindingPlacement(0, false))
      .expect(inv and plc.localBlindedWrapsCompleted == 1)
}
