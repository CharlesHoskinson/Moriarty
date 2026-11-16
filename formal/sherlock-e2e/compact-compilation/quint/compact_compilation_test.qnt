// -*- mode: Bluespec; -*-
// Deterministic scenario tests for bounded context Compact Compilation (lane e2e-compact-compilation).

module compact_compilation_test {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13,
    RSL_RULE_COUNT = 16,
    DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3,
    JUBJUB_POINT_WORDS = 2,
    ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550,
    BATCH_CIRCUIT_COUNT = 2,
    MUTANT_ID = 0
  ).* from "./compact_compilation"

  // 1. Accepted proof circuits: counter, keccak256 chip rule, Jubjub 2-word decode, and Secp256k1 R_SL kit
  // Covers ECC-001, ECC-003, ECC-005, ECC-007, ECC-009, ECC-014, CF-013, CF-001.
  run cc_counterKeccakJubjubAndSecp256k1CompileTest =
    init
      .expect(inv and lastOp == "init")
      .then(doCompileSingleCircuit(CircCounterValid, false))
      .expect(
        inv and
        cc.lastCompile.accepted and
        cc.lastCompile.zkirVersion == 31 and
        not(cc.lastCompile.usedPatchedCompactcPass) and
        cc.lastCompile.emittedImageCount == 1 and
        cc.lastCompile.emittedAbiCount == 1 and
        cc.lastCompile.abiTapeLen == 3 + cc.lastCompile.slotsCount and
        cc.lastCompile.abiLE == cc.lastCompile.codeEffectWords and
        cc.lastCompile.maxEmittedShapeId == 13 and
        cc.lastCompile.anchorSlotUsed == 15 and
        not(cc.lastCompile.hasSpikeConstants)
      )
      .then(doCompileSingleCircuit(CircKeccakValid, false))
      .expect(
        inv and
        cc.lastCompile.accepted and
        cc.lastCompile.keccakUsesChipRule and
        not(cc.lastCompile.keccakUsedTier3OrRefused) and
        cc.keccakCompileOk
      )
      .then(doCompileSingleCircuit(CircJubjubValid, false))
      .expect(
        inv and
        cc.lastCompile.accepted and
        cc.lastCompile.jubjubPointWordsRead == 2 and
        cc.lastCompile.jubjubCurveAndSubgroupChecked and
        cc.jubjubCompileOk
      )
      .then(doCompileSingleCircuit(CircSecp256k1Valid, false))
      .expect(
        inv and
        cc.lastCompile.accepted and
        cc.lastCompile.secp256k1LoweredToKit and
        not(cc.lastCompile.secp256k1RefusedInputType) and
        cc.secp256k1CompileOk
      )

  // 2. Five-stage ordered refusal pipeline: read -> irVersion -> inputTypes -> lowering -> seal
  // Covers ECC-009, ECC-010, CF-014.
  run cc_orderedRefusalPipelineAndUnknownCurve25519Test =
    init
      .expect(
        computeFirstFailingStage(false, 32, false, false, true, true) == STAGE_READ and
        computeFirstFailingStage(true, 32, false, false, true, true) == STAGE_IR_VERSION and
        computeFirstFailingStage(true, 31, false, false, true, true) == STAGE_INPUT_TYPES and
        computeFirstFailingStage(true, 31, true, false, true, true) == STAGE_LOWERING and
        computeFirstFailingStage(true, 31, true, true, true, true) == STAGE_SEAL and
        computeFirstFailingStage(true, 31, true, true, true, false) == STAGE_NONE
      )
      // Circuit failing both irVersion (3.2) and seal (slot 15) reports irVersion first
      .then(doCompileSingleCircuit(CircMultiFailIrAndSeal, false))
      .expect(
        inv and
        not(cc.lastCompile.accepted) and
        cc.lastCompile.reportedStage == STAGE_IR_VERSION and
        cc.lastCompile.refusalCode == "MYC-REF-032/irVersion" and
        cc.lastCompile.unsupportedCount == 0
      )
      // Unknown Curve25519 input type refused at inputTypes stage naming the type
      .then(doCompileSingleCircuit(CircUnknownCurve25519, false))
      .expect(
        inv and
        not(cc.lastCompile.accepted) and
        cc.lastCompile.reportedStage == STAGE_INPUT_TYPES and
        cc.lastCompile.refusalCode == "InputType:Scalar<Curve25519>"
      )
      // User ledger field at slot 15 refused at seal stage in deployable profile
      .then(doCompileSingleCircuit(CircSlot15Collision, false))
      .expect(
        inv and
        not(cc.lastCompile.accepted) and
        cc.lastCompile.reportedStage == STAGE_SEAL and
        cc.lastCompile.refusalCode == "MYC-SEAL-021/seal"
      )

  // 3. Witness invariance of serialized image/ABI bytes and static step count N_e <= 550 on spend
  // Covers ECC-004, CF-002, CF-004.
  run cc_witnessInvarianceStaticStepCountAndPlanWithoutWitnessTest =
    init
      .then(doCheckWitnessInvarianceAndStepBound(550, false))
      .expect(
        inv and
        cc.det.checked and
        cc.det.imageShaTape1 == cc.det.imageShaTape2 and
        cc.det.abiShaTape1 == cc.det.abiShaTape2 and
        cc.det.stepCountTape1 == cc.det.stepCountTape2 and
        cc.det.spendStepCount == 550 and
        cc.det.planReadNoWitness
      )

  // 4. `sherlock compile` CLI batch: all admitted (exit 0) vs 1 ZKIR 3.2 circuit refused (exit 1, 0 files)
  // Covers ECC-012, ECC-013.
  run cc_compileCliBatchAdmitAndAllOrNothingRefusalTest =
    init
      .then(doRunCompileCliBatch(false, false))
      .expect(
        inv and
        cc.batch.writtenImageCount == 2 and
        cc.batch.writtenAbiCount == 2 and
        cc.batch.cliExitCode == 0 and
        cc.allValidBatchCompleted
      )
      .then(doRunCompileCliBatch(true, false))
      .expect(
        inv and
        cc.batch.cliExitCode == 1 and
        cc.batch.printedRefusalCode == "MYC-REF-032/irVersion" and
        cc.batch.writtenImageCount == 0 and
        cc.batch.writtenAbiCount == 0 and
        cc.refusedBatchCompleted
      )

  // 5. Factory gates, OracleMain golden replay, lake build --wfail, S1/CF-L1 closure, and spec-gap witnesses
  // Covers ECC-002, ECC-011, LPC-013, LPC-019, CF-012, Q-compact-compilation-1..4.
  run cc_factoryGatesOracleReplayPremiseS1CfL1AndSpecGapsTest =
    init
      .expect(
        inv and
        consistency_constants and
        gap_q_cc_1_ecc003_vs_cf002_abi_fields and
        gap_q_cc_2_sorries_vs_wfail_and_cf012_axioms and
        gap_q_cc_3_ecc013_abi_omission_and_multi_refusal_order and
        gap_q_cc_4_slot15_passes_spike_fails_deploy
      )
      // Surviving unargued mutant blocks usableForSettledCall
      .then(doRunFactoryGatesAndOracle(true, false, 4, false))
      .expect(
        inv and
        not(cc.gates.gatesShPassed) and
        not(cc.gates.usableForSettledCall) and
        not(cc.gates.lakeBuildWfailPassed)
      )
      // Golden vector difference against OracleMain fails gate and names the vector
      .then(doRunFactoryGatesAndOracle(false, true, 0, false))
      .expect(
        inv and
        not(cc.gates.oracleGatePassed) and
        not(cc.gates.gatesShPassed) and
        cc.gates.namedDiffVector == "golden_vec_07" and
        cc.gates.lakeBuildWfailPassed
      )
      // Clean gates + 0 sorries passes both gates.sh and lake build --wfail
      .then(doRunFactoryGatesAndOracle(false, false, 0, false))
      .expect(
        inv and
        cc.gates.gatesShPassed and
        cc.gates.usableForSettledCall and
        cc.gates.lakeBuildWfailPassed
      )
      // S1 kept as named hypothesis allows Mycroft design close while CF-L1 open keeps CF kit incomplete
      .then(doEvaluatePremiseAndKitClosure(false, true, false, true, false))
      .expect(
        inv and
        cc.premises.mycroftDesignClosed and
        cc.premises.s1NamedInReport and
        not(cc.premises.cfKitReportedComplete)
      )
      // Proving CF-L1 under standard Lean axioms completes CF kit
      .then(doEvaluatePremiseAndKitClosure(true, false, true, true, false))
      .expect(
        inv and
        cc.premises.mycroftDesignClosed and
        cc.premises.cfKitReportedComplete
      )
}
