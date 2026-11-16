// -*- mode: Bluespec; -*-
// Independent Quint specification for bounded context Compact Compilation (lane e2e-compact-compilation).
// Sources:
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-compact-compiler/spec.md (ECC-001..005, 007, 009..014)
//   - openspec/changes/sherlock-completion-20261115/specs/sherlock-compact-frontend/spec.md (CF-013, CF-014)
//   - openspec/changes/sherlock-completion-20261115/specs/sherlock-loop-completion/spec.md (LPC-013, LPC-019)
//   - openspec/changes/sherlock-processor-final-20261111/specs/sherlock-compact-frontend/spec.md (adopted CF-001, CF-002, CF-004, CF-012)
//   - openspec/changes/sherlock-e2e-prototype-20261115/architecture/DDD.md (Compact Compilation rows in sections 1-4, 6-8)

module compact_compilation {
  // =========================================================================
  // Constants (numeric bounds, versions, slots, and instruction/rule counts)
  // =========================================================================

  // ECC-005: exact count of straight-line SInstr shapes emitted by Mycroft (13).
  const SINSTR_SHAPE_COUNT: int
  // ECC-005 Status & DDD section 3: exact count of R_SL rules in compile::RULES (16).
  const RSL_RULE_COUNT: int
  // ECC-009: top-level ledger slot reserved as anchor_slot in deployable profiles (15).
  const DEPLOY_ANCHOR_SLOT: int
  // ECC-003 Scenario: fixed header words on a call tape (salt, selector, rand => 3; tape_len = 3 + slots).
  const TAPE_HEADER_WORDS: int
  // ECC-014 Scenario: raw tape words read for a Point<Jubjub> public-input decode (2).
  const JUBJUB_POINT_WORDS: int
  // ECC-001, ECC-013: supported stock compactc 0.36.0 ZKIR version encoded as 10*major + minor (3.1 => 31).
  const ZKIR_SUPPORTED_VERSION: int
  // Adopted CF-004 Scenario: upper bound on R_SL step count of `spend` at M1 (550).
  const MAX_SPEND_STEPS: int
  // ECC-012, ECC-013: number of requested proof circuits in a multi-circuit `sherlock compile` batch (1..4).
  const BATCH_CIRCUIT_COUNT: int
  // Mutant selector: 0 = honest specification; 1..16 = individual guard mutants.
  const MUTANT_ID: int

  // =========================================================================
  // Mutant identifiers
  // =========================================================================

  pure val MUT_NONE: int = 0
  pure val MUT_ECC001_PATCHED_OR_WRONG_ZKIR: int = 1
  pure val MUT_ECC002_SETTLE_SURVIVING_MUTANT: int = 2
  pure val MUT_ECC003_BAD_ABI_LAYOUT: int = 3
  pure val MUT_ECC004_WITNESS_DEPENDENT_BYTES: int = 4
  pure val MUT_ECC005_ILLEGAL_SINSTR_SHAPE: int = 5
  pure val MUT_ECC007_KECCAK_TIER3_OR_REFUSE: int = 6
  pure val MUT_ECC009_SPIKE_SLOT_OR_COLLISION: int = 7
  pure val MUT_ECC010_WRONG_REFUSAL_ORDER: int = 8
  pure val MUT_ECC011_IGNORE_ORACLE_DIFF: int = 9
  pure val MUT_ECC012_MISSING_ABI_OR_NONZERO: int = 10
  pure val MUT_ECC013_PARTIAL_OUTPUT_ON_REFUSAL: int = 11
  pure val MUT_ECC014_JUBJUB_MISSING_CURVE_CHECK: int = 12
  pure val MUT_CF013_REFUSE_SECP256K1_INPUT: int = 13
  pure val MUT_CF014_ADMIT_UNKNOWN_CURVE25519: int = 14
  pure val MUT_CF004_TAPE_DEPENDENT_STEPS: int = 15
  pure val MUT_LPC019_CLOSE_WITH_UNNAMED_S1: int = 16

  // =========================================================================
  // Pipeline stage order & domain types
  // =========================================================================

  // ECC-010 strict refusal check order: 1=read, 2=irVersion, 3=inputTypes, 4=lowering, 5=seal (0=none).
  pure val STAGE_NONE: int = 0
  pure val STAGE_READ: int = 1
  pure val STAGE_IR_VERSION: int = 2
  pure val STAGE_INPUT_TYPES: int = 3
  pure val STAGE_LOWERING: int = 4
  pure val STAGE_SEAL: int = 5

  pure val SINSTR_SHAPES: Set[str] = Set(
    "Field", "Mov", "Eq", "Select", "Lt", "Divmod2", "Range",
    "AssertEq", "Assert", "Tape", "Eff", "Chip", "Ret"
  )

  pure val CF002_FRONTEND_COMMANDS: Set[str] = Set(
    "sherlock compile", "sherlock plan", "sherlock simulate",
    "sherlock deploy", "sherlock prove", "sherlock call"
  )

  type CircuitCase =
    | CircCounterValid
    | CircKeccakValid
    | CircJubjubValid
    | CircSecp256k1Valid
    | CircUnknownCurve25519
    | CircMultiFailIrAndSeal
    | CircSlot15Collision

  type SingleCompileState = {
    evaluated: bool,
    circuitCase: CircuitCase,
    isPrototypeBuild: bool,
    usedPatchedCompactcPass: bool,
    usesLflattenedTap: bool,
    zkirVersion: int,
    deployProfile: bool,
    userUsesSlot15: bool,
    firstFailingStage: int,
    accepted: bool,
    reportedStage: int,
    refusalCode: str,
    unsupportedCount: int,
    // Emitted image & ABI properties when accepted (ECC-003, ECC-005, ECC-007, ECC-009, ECC-014, CF-013)
    emittedImageCount: int,
    emittedAbiCount: int,
    imageSelector: int,
    abiSelector: int,
    slotsCount: int,
    abiTapeLen: int,
    codeEffectWords: int,
    abiLE: int,
    abiNE: int,
    maxEmittedShapeId: int,
    universalKeysChanged: bool,
    anchorSlotUsed: int,
    hasSpikeConstants: bool,
    effectAddressesRoot15: bool,
    keccakUsesChipRule: bool,
    keccakUsedTier3OrRefused: bool,
    jubjubPointWordsRead: int,
    jubjubCurveAndSubgroupChecked: bool,
    secp256k1LoweredToKit: bool,
    secp256k1RefusedInputType: bool,
    unknownInputTypeAdmitted: bool
  }

  type DeterminismState = {
    checked: bool,
    imageShaTape1: int,
    imageShaTape2: int,
    abiShaTape1: int,
    abiShaTape2: int,
    stepCountTape1: int,
    stepCountTape2: int,
    spendStepCount: int,
    planReadNoWitness: bool
  }

  type BatchCompileState = {
    executed: bool,
    requestedCount: int,
    refusedCircuitCount: int,
    writtenImageCount: int,
    writtenAbiCount: int,
    cliExitCode: int,
    printedRefusalCode: str
  }

  type FactoryGateState = {
    evaluated: bool,
    hasUnarguedSurvivingMutant: bool,
    gatesShPassed: bool,
    usableForSettledCall: bool,
    hasGoldenOracleDiff: bool,
    oracleGatePassed: bool,
    namedDiffVector: str,
    sorriesRemainingCount: int,
    lakeBuildWfailPassed: bool
  }

  type PremiseClosureState = {
    mycroftDesignClosed: bool,
    s1Discharged: bool,
    s1KeptAsNamedHypothesis: bool,
    s1NamedInReport: bool,
    cfL1Proved: bool,
    onlyStandardLeanAxioms: bool,
    cfKitReportedComplete: bool
  }

  type CompilerState = {
    lastCompile: SingleCompileState,
    det: DeterminismState,
    batch: BatchCompileState,
    gates: FactoryGateState,
    premises: PremiseClosureState,
    // Monotone reachability counters across steps
    acceptedCompileRuns: int,
    refusedCompileRuns: int,
    keccakCompileOk: bool,
    jubjubCompileOk: bool,
    secp256k1CompileOk: bool,
    allValidBatchCompleted: bool,
    refusedBatchCompleted: bool
  }

  // =========================================================================
  // Observable state variables
  // =========================================================================

  var cc: CompilerState
  var lastOp: str

  // =========================================================================
  // Pure functions (5-stage refusal order & compilation semantics)
  // =========================================================================

  // Computes the earliest failing check in ECC-010's strict order:
  // 1=read, 2=irVersion, 3=inputTypes, 4=lowering, 5=seal (0=all checks pass).
  pure def computeFirstFailingStage(
    readOk: bool,
    zkirVer: int,
    inputTypeSupported: bool,
    loweringOk: bool,
    deployProf: bool,
    slot15OccupiedByUser: bool
  ): int =
    if (not(readOk)) STAGE_READ
    else if (zkirVer != ZKIR_SUPPORTED_VERSION) STAGE_IR_VERSION
    else if (not(inputTypeSupported)) STAGE_INPUT_TYPES
    else if (not(loweringOk)) STAGE_LOWERING
    else if (deployProf and slot15OccupiedByUser) STAGE_SEAL
    else STAGE_NONE

  pure def refusalCodeForStage(stageId: int, circ: CircuitCase): str =
    if (stageId == STAGE_READ) "MYC-REF-READ/io"
    else if (stageId == STAGE_IR_VERSION) "MYC-REF-032/irVersion"
    else if (stageId == STAGE_INPUT_TYPES)
      (if (circ == CircUnknownCurve25519) "InputType:Scalar<Curve25519>" else "InputType:Unsupported")
    else if (stageId == STAGE_LOWERING) "MYC-REF-LOW/lowering"
    else if (stageId == STAGE_SEAL) "MYC-SEAL-021/seal"
    else "none"

  // =========================================================================
  // Pure consistency & spec-gap witness predicates
  // =========================================================================

  val consistency_constants: bool =
    SINSTR_SHAPE_COUNT == 13 and
    SINSTR_SHAPES.size() == SINSTR_SHAPE_COUNT and
    RSL_RULE_COUNT == 16 and
    SINSTR_SHAPE_COUNT <= RSL_RULE_COUNT and
    DEPLOY_ANCHOR_SLOT == 15 and
    TAPE_HEADER_WORDS == 3 and
    JUBJUB_POINT_WORDS == 2 and
    ZKIR_SUPPORTED_VERSION == 31 and
    MAX_SPEND_STEPS == 550 and
    CF002_FRONTEND_COMMANDS.size() == 6 and
    BATCH_CIRCUIT_COUNT >= 1 and
    BATCH_CIRCUIT_COUNT <= 4

  // Finding Q-compact-compilation-1 witness:
  // ECC-003's ABI schema {selector, tape_len, slots, l_e, n_e, kits} omits CF-002's per-builtin count map.
  pure val gap_q_cc_1_ecc003_vs_cf002_abi_fields: bool = {
    val ecc003Fields = Set("selector", "tape_len", "slots", "l_e", "n_e", "kits")
    val cf002RequiredField = "builtin_counts"
    not(ecc003Fields.contains(cf002RequiredField))
  }

  // Finding Q-compact-compilation-2 witness:
  // Open sorry entries in mycroft/lean/SORRIES.md (S2, builtin params, wide args, secp256k1 layout; sorries > 0)
  // fail `lake build --wfail` (LPC-013) even if S1 alone is kept as a named hypothesis (LPC-019), while turning
  // sorries into Lean `axiom` declarations violates CF-012's standard-axioms-only gate.
  pure val gap_q_cc_2_sorries_vs_wfail_and_cf012_axioms: bool = {
    val s1AsNamedHypothesis = true
    val remainingSorriesAfterS1 = 4
    val lakeBuildWfailPasses = (remainingSorriesAfterS1 == 0)
    s1AsNamedHypothesis and not(lakeBuildWfailPasses)
  }

  // Finding Q-compact-compilation-3 witness:
  // Normative ECC-013 ("write no image file for the request") does not mention ABI files emitted by ECC-012,
  // and defines no cross-circuit deterministic order when 2 requested circuits fail with distinct codes.
  pure val gap_q_cc_3_ecc013_abi_omission_and_multi_refusal_order: bool = {
    val writtenImages = 0
    val writtenPartialAbis = 1
    val literalEcc013NoImageHolds = (writtenImages == 0)
    val twoCircuitRefusalCodes = Set("MYC-REF-032/irVersion", "InputType:Scalar<Curve25519>")
    literalEcc013NoImageHolds and writtenPartialAbis > 0 and twoCircuitRefusalCodes.size() == 2
  }

  // Finding Q-compact-compilation-4 witness:
  // A contract with a user ledger field at slot 15 passes non-deployable suite compilation (SPIKE_SLOT = 0)
  // under ECC-010/ECC-011, yet collides with DEPLOY_ANCHOR_SLOT == 15 in deployable profile (ECC-009).
  val gap_q_cc_4_slot15_passes_spike_fails_deploy: bool = {
    val nonDeployStage = computeFirstFailingStage(true, ZKIR_SUPPORTED_VERSION, true, true, false, true)
    val deployStage = computeFirstFailingStage(true, ZKIR_SUPPORTED_VERSION, true, true, true, true)
    nonDeployStage == STAGE_NONE and deployStage == STAGE_SEAL
  }

  // =========================================================================
  // Initialization
  // =========================================================================

  action init = all {
    cc' = {
      lastCompile: {
        evaluated: false,
        circuitCase: CircCounterValid,
        isPrototypeBuild: true,
        usedPatchedCompactcPass: false,
        usesLflattenedTap: false,
        zkirVersion: ZKIR_SUPPORTED_VERSION,
        deployProfile: true,
        userUsesSlot15: false,
        firstFailingStage: STAGE_NONE,
        accepted: false,
        reportedStage: STAGE_NONE,
        refusalCode: "none",
        unsupportedCount: 0,
        emittedImageCount: 0,
        emittedAbiCount: 0,
        imageSelector: 1,
        abiSelector: 1,
        slotsCount: 4,
        abiTapeLen: TAPE_HEADER_WORDS + 4,
        codeEffectWords: 5,
        abiLE: 5,
        abiNE: 120,
        maxEmittedShapeId: SINSTR_SHAPE_COUNT,
        universalKeysChanged: false,
        anchorSlotUsed: DEPLOY_ANCHOR_SLOT,
        hasSpikeConstants: false,
        effectAddressesRoot15: false,
        keccakUsesChipRule: true,
        keccakUsedTier3OrRefused: false,
        jubjubPointWordsRead: JUBJUB_POINT_WORDS,
        jubjubCurveAndSubgroupChecked: true,
        secp256k1LoweredToKit: true,
        secp256k1RefusedInputType: false,
        unknownInputTypeAdmitted: false
      },
      det: {
        checked: false,
        imageShaTape1: 9001,
        imageShaTape2: 9001,
        abiShaTape1: 9002,
        abiShaTape2: 9002,
        stepCountTape1: 120,
        stepCountTape2: 120,
        spendStepCount: MAX_SPEND_STEPS,
        planReadNoWitness: true
      },
      batch: {
        executed: false,
        requestedCount: BATCH_CIRCUIT_COUNT,
        refusedCircuitCount: 0,
        writtenImageCount: 0,
        writtenAbiCount: 0,
        cliExitCode: 0,
        printedRefusalCode: "none"
      },
      gates: {
        evaluated: false,
        hasUnarguedSurvivingMutant: false,
        gatesShPassed: true,
        usableForSettledCall: false,
        hasGoldenOracleDiff: false,
        oracleGatePassed: true,
        namedDiffVector: "none",
        sorriesRemainingCount: 0,
        lakeBuildWfailPassed: true
      },
      premises: {
        mycroftDesignClosed: false,
        s1Discharged: false,
        s1KeptAsNamedHypothesis: true,
        s1NamedInReport: true,
        cfL1Proved: false,
        onlyStandardLeanAxioms: true,
        cfKitReportedComplete: false
      },
      acceptedCompileRuns: 0,
      refusedCompileRuns: 0,
      keccakCompileOk: false,
      jubjubCompileOk: false,
      secp256k1CompileOk: false,
      allValidBatchCompleted: false,
      refusedBatchCompleted: false
    },
    lastOp' = "init"
  }

  // =========================================================================
  // Parameterized actions
  // =========================================================================

  // Implements ECC-001, ECC-003, ECC-005, ECC-007, ECC-009, ECC-010, ECC-014, CF-013, CF-014, CF-001:
  // Compiles one proof circuit through the 5-stage pipeline (read -> irVersion -> inputTypes -> lowering -> seal).
  action doCompileSingleCircuit(circ: CircuitCase, mutTrig: bool): bool = {
    val zkirVer = if (circ == CircMultiFailIrAndSeal) 32 else ZKIR_SUPPORTED_VERSION
    val userSlot15 = (circ == CircMultiFailIrAndSeal or circ == CircSlot15Collision)
    val mutRefuseSecp = (MUTANT_ID == MUT_CF013_REFUSE_SECP256K1_INPUT and circ == CircSecp256k1Valid and mutTrig)
    val mutAdmitCurve25519 = (MUTANT_ID == MUT_CF014_ADMIT_UNKNOWN_CURVE25519 and circ == CircUnknownCurve25519 and mutTrig)
    val inputTypeOk =
      if (circ == CircUnknownCurve25519) mutAdmitCurve25519
      else if (circ == CircSecp256k1Valid) not(mutRefuseSecp)
      else true
    val trueFirstFail = computeFirstFailingStage(true, zkirVer, inputTypeOk, true, true, userSlot15)

    val mutAcceptWrongZkir = (MUTANT_ID == MUT_ECC001_PATCHED_OR_WRONG_ZKIR and mutTrig)
    val mutAllowSlot15 = (MUTANT_ID == MUT_ECC009_SPIKE_SLOT_OR_COLLISION and circ == CircSlot15Collision and mutTrig)
    val mutWrongOrder = (MUTANT_ID == MUT_ECC010_WRONG_REFUSAL_ORDER and circ == CircMultiFailIrAndSeal and mutTrig)

    val isAccepted =
      if (mutAcceptWrongZkir and circ == CircMultiFailIrAndSeal) true
      else if (mutAllowSlot15) true
      else (trueFirstFail == STAGE_NONE)

    val repStage =
      if (isAccepted) STAGE_NONE
      else if (mutWrongOrder) STAGE_SEAL
      else trueFirstFail

    val badAbi = (MUTANT_ID == MUT_ECC003_BAD_ABI_LAYOUT and isAccepted and mutTrig)
    val badShape = (MUTANT_ID == MUT_ECC005_ILLEGAL_SINSTR_SHAPE and isAccepted and mutTrig)
    val badKeccak = (MUTANT_ID == MUT_ECC007_KECCAK_TIER3_OR_REFUSE and circ == CircKeccakValid and mutTrig)
    val badAnchor = (MUTANT_ID == MUT_ECC009_SPIKE_SLOT_OR_COLLISION and isAccepted and mutTrig)
    val badJubjub = (MUTANT_ID == MUT_ECC014_JUBJUB_MISSING_CURVE_CHECK and circ == CircJubjubValid and mutTrig)

    val slotsVal = 4
    val effVal = 5
    val updatedSingle: SingleCompileState = {
      evaluated: true,
      circuitCase: circ,
      isPrototypeBuild: true,
      usedPatchedCompactcPass: mutAcceptWrongZkir,
      usesLflattenedTap: false,
      zkirVersion: zkirVer,
      deployProfile: true,
      userUsesSlot15: userSlot15,
      firstFailingStage: computeFirstFailingStage(
        true,
        zkirVer,
        (circ != CircUnknownCurve25519),
        true,
        true,
        userSlot15
      ),
      accepted: isAccepted and not(badKeccak),
      reportedStage: if (badKeccak) STAGE_LOWERING else repStage,
      refusalCode: if (badKeccak) "MYC-BLT-007/tier3" else refusalCodeForStage(repStage, circ),
      unsupportedCount: if (mutWrongOrder) 1 else 0,
      emittedImageCount: if (isAccepted and not(badKeccak)) 1 else 0,
      emittedAbiCount: if (isAccepted and not(badKeccak)) 1 else 0,
      imageSelector: 42,
      abiSelector: if (badAbi) 99 else 42,
      slotsCount: slotsVal,
      abiTapeLen: if (badAbi) slotsVal else (TAPE_HEADER_WORDS + slotsVal),
      codeEffectWords: effVal,
      abiLE: if (badAbi) (effVal + 1) else effVal,
      abiNE: 120,
      maxEmittedShapeId: if (badShape) (SINSTR_SHAPE_COUNT + 1) else SINSTR_SHAPE_COUNT,
      universalKeysChanged: badShape,
      anchorSlotUsed: if (badAnchor) 0 else DEPLOY_ANCHOR_SLOT,
      hasSpikeConstants: badAnchor,
      effectAddressesRoot15: badAnchor,
      keccakUsesChipRule: not(badKeccak),
      keccakUsedTier3OrRefused: badKeccak,
      jubjubPointWordsRead: if (badJubjub) 1 else JUBJUB_POINT_WORDS,
      jubjubCurveAndSubgroupChecked: not(badJubjub),
      secp256k1LoweredToKit: not(mutRefuseSecp),
      secp256k1RefusedInputType: mutRefuseSecp,
      unknownInputTypeAdmitted: mutAdmitCurve25519
    }
    all {
      cc' = {
        ...cc,
        lastCompile: updatedSingle,
        acceptedCompileRuns: if (updatedSingle.accepted) (cc.acceptedCompileRuns + 1) else cc.acceptedCompileRuns,
        refusedCompileRuns: if (not(updatedSingle.accepted)) (cc.refusedCompileRuns + 1) else cc.refusedCompileRuns,
        keccakCompileOk: cc.keccakCompileOk or (circ == CircKeccakValid and updatedSingle.accepted),
        jubjubCompileOk: cc.jubjubCompileOk or (circ == CircJubjubValid and updatedSingle.accepted),
        secp256k1CompileOk: cc.secp256k1CompileOk or (circ == CircSecp256k1Valid and updatedSingle.accepted)
      },
      lastOp' = "compileSingleCircuit"
    }
  }

  // Implements ECC-004, adopted CF-004, adopted CF-002 (Scenario: sherlock plan without witness):
  // Serializes one circuit across two distinct witness tapes and checks byte-equal image/ABI and static step count N_e.
  action doCheckWitnessInvarianceAndStepBound(spendSteps: int, mutTrig: bool): bool = {
    val mutBytes = (MUTANT_ID == MUT_ECC004_WITNESS_DEPENDENT_BYTES and mutTrig)
    val mutSteps = (MUTANT_ID == MUT_CF004_TAPE_DEPENDENT_STEPS and mutTrig)
    val legalSpend = (spendSteps > 0 and spendSteps <= MAX_SPEND_STEPS)
    all {
      legalSpend or mutSteps,
      cc' = {
        ...cc,
        det: {
          checked: true,
          imageShaTape1: 9001,
          imageShaTape2: if (mutBytes) 9999 else 9001,
          abiShaTape1: 9002,
          abiShaTape2: if (mutBytes) 9998 else 9002,
          stepCountTape1: 120,
          stepCountTape2: if (mutSteps) 185 else 120,
          spendStepCount: if (mutSteps) (MAX_SPEND_STEPS + 50) else spendSteps,
          planReadNoWitness: true
        }
      },
      lastOp' = "checkWitnessInvarianceAndStepBound"
    }
  }

  // Implements ECC-012, ECC-013:
  // Executes `sherlock compile` on a batch of BATCH_CIRCUIT_COUNT requested proof circuits (all valid vs 1 ZKIR 3.2 refusal).
  action doRunCompileCliBatch(hasRefusedCircuit: bool, mutTrig: bool): bool = {
    val mutBadAdmit = (MUTANT_ID == MUT_ECC012_MISSING_ABI_OR_NONZERO and not(hasRefusedCircuit) and mutTrig)
    val mutPartialOut = (MUTANT_ID == MUT_ECC013_PARTIAL_OUTPUT_ON_REFUSAL and hasRefusedCircuit and mutTrig)
    val nextBatch: BatchCompileState =
      if (not(hasRefusedCircuit)) {
        executed: true,
        requestedCount: BATCH_CIRCUIT_COUNT,
        refusedCircuitCount: 0,
        writtenImageCount: BATCH_CIRCUIT_COUNT,
        writtenAbiCount: if (mutBadAdmit) 0 else BATCH_CIRCUIT_COUNT,
        cliExitCode: if (mutBadAdmit) 1 else 0,
        printedRefusalCode: "none"
      } else {
        executed: true,
        requestedCount: BATCH_CIRCUIT_COUNT,
        refusedCircuitCount: 1,
        writtenImageCount: if (mutPartialOut) 1 else 0,
        writtenAbiCount: if (mutPartialOut) 1 else 0,
        cliExitCode: if (mutPartialOut) 0 else 1,
        printedRefusalCode: if (mutPartialOut) "none" else "MYC-REF-032/irVersion"
      }
    all {
      cc' = {
        ...cc,
        batch: nextBatch,
        allValidBatchCompleted: cc.allValidBatchCompleted or (not(hasRefusedCircuit) and nextBatch.cliExitCode == 0),
        refusedBatchCompleted: cc.refusedBatchCompleted or (hasRefusedCircuit and nextBatch.cliExitCode != 0)
      },
      lastOp' = "runCompileCliBatch"
    }
  }

  // Implements ECC-002, ECC-011, LPC-013:
  // Runs `mycroft/factory/gates.sh` (mutation gate + OracleMain golden vector replay) and `lake build --wfail`.
  action doRunFactoryGatesAndOracle(
    unarguedMutantSurvived: bool,
    goldenVectorDiffers: bool,
    remainingSorries: int,
    mutTrig: bool
  ): bool = {
    val mutAllowSurvivor = (MUTANT_ID == MUT_ECC002_SETTLE_SURVIVING_MUTANT and unarguedMutantSurvived and mutTrig)
    val mutIgnoreOracle = (MUTANT_ID == MUT_ECC011_IGNORE_ORACLE_DIFF and goldenVectorDiffers and mutTrig)
    val oracleOk = if (mutIgnoreOracle) true else not(goldenVectorDiffers)
    val gatesOk = not(unarguedMutantSurvived) and oracleOk
    val wfailOk = (remainingSorries == 0)
    all {
      cc' = {
        ...cc,
        gates: {
          evaluated: true,
          hasUnarguedSurvivingMutant: unarguedMutantSurvived,
          gatesShPassed: gatesOk,
          usableForSettledCall: if (mutAllowSurvivor) true else gatesOk,
          hasGoldenOracleDiff: goldenVectorDiffers,
          oracleGatePassed: oracleOk,
          namedDiffVector: if (goldenVectorDiffers and not(mutIgnoreOracle)) "golden_vec_07" else "none",
          sorriesRemainingCount: remainingSorries,
          lakeBuildWfailPassed: wfailOk
        }
      },
      lastOp' = "runFactoryGatesAndOracle"
    }
  }

  // Implements LPC-019, adopted CF-012:
  // Evaluates Mycroft design closure (premise S1 discharged or named hypothesis) and CF-L1 kit completion.
  action doEvaluatePremiseAndKitClosure(
    dischargeS1: bool,
    keepS1Named: bool,
    cfL1ProvedFlag: bool,
    stdAxiomsOnly: bool,
    mutTrig: bool
  ): bool = {
    val s1Handled = (dischargeS1 or keepS1Named)
    val mutBadClose = (MUTANT_ID == MUT_LPC019_CLOSE_WITH_UNNAMED_S1 and not(s1Handled) and mutTrig)
    val canCloseMycroft = (s1Handled or mutBadClose)
    val canCompleteCfKit = (cfL1ProvedFlag and stdAxiomsOnly)
    all {
      cc' = {
        ...cc,
        premises: {
          mycroftDesignClosed: canCloseMycroft,
          s1Discharged: dischargeS1,
          s1KeptAsNamedHypothesis: keepS1Named,
          s1NamedInReport: keepS1Named,
          cfL1Proved: cfL1ProvedFlag,
          onlyStandardLeanAxioms: stdAxiomsOnly,
          cfKitReportedComplete: canCompleteCfKit
        }
      },
      lastOp' = "evaluatePremiseAndKitClosure"
    }
  }

  // =========================================================================
  // Nondeterministic step relation
  // =========================================================================

  action step = any {
    {
      nondet cCase = Set(
        CircCounterValid,
        CircKeccakValid,
        CircJubjubValid,
        CircSecp256k1Valid,
        CircUnknownCurve25519,
        CircMultiFailIrAndSeal,
        CircSlot15Collision
      ).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doCompileSingleCircuit(cCase, mutTrig)
    },
    {
      nondet spSteps = Set(120, MAX_SPEND_STEPS).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doCheckWitnessInvarianceAndStepBound(spSteps, mutTrig)
    },
    {
      nondet hasRef = Set(false, true).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doRunCompileCliBatch(hasRef, mutTrig)
    },
    {
      nondet survMut = Set(false, true).oneOf()
      nondet oraDiff = Set(false, true).oneOf()
      nondet sCnt = Set(0, 4).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doRunFactoryGatesAndOracle(survMut, oraDiff, sCnt, mutTrig)
    },
    {
      nondet disS1 = Set(true, false).oneOf()
      nondet keepS1 = Set(true, false).oneOf()
      nondet cfL1 = Set(false, true).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doEvaluatePremiseAndKitClosure(disS1, keepS1, cfL1, true, mutTrig)
    }
  }

  // =========================================================================
  // Safety invariants (one inv_<ID> per expressible requirement)
  // =========================================================================

  // ECC-001: Stage 1 reads only stock compactc 0.36.0 ZKIR 3.1 + contract-info.json (no patched compactc pass).
  val inv_ECC_001: bool =
    cc.lastCompile.evaluated implies (
      not(cc.lastCompile.usedPatchedCompactcPass) and
      (cc.lastCompile.accepted implies (cc.lastCompile.zkirVersion == ZKIR_SUPPORTED_VERSION))
    )

  // ECC-002: Compiler is not usable for a settled call if an unargued mutant survives or gates.sh fails.
  val inv_ECC_002: bool =
    cc.gates.usableForSettledCall implies (
      cc.gates.gatesShPassed and not(cc.gates.hasUnarguedSurvivingMutant)
    )

  // ECC-003: Each accepted proof circuit emits 1 image and 1 ABI with matching selector, tape_len = 3 + slots, and l_e = effect words.
  val inv_ECC_003: bool =
    (cc.lastCompile.evaluated and cc.lastCompile.accepted) implies (
      cc.lastCompile.emittedImageCount == 1 and
      cc.lastCompile.emittedAbiCount == 1 and
      cc.lastCompile.imageSelector == cc.lastCompile.abiSelector and
      cc.lastCompile.abiTapeLen == TAPE_HEADER_WORDS + cc.lastCompile.slotsCount and
      cc.lastCompile.abiLE == cc.lastCompile.codeEffectWords
    )

  // ECC-004: Serialized image and ABI bytes have equal SHA-256 across different call tapes of one entry.
  val inv_ECC_004: bool =
    cc.det.checked implies (
      cc.det.imageShaTape1 == cc.det.imageShaTape2 and
      cc.det.abiShaTape1 == cc.det.abiShaTape2
    )

  // ECC-005: Emitted instructions use only the 13 SInstr shapes and change no universal key.
  val inv_ECC_005: bool =
    (cc.lastCompile.evaluated and cc.lastCompile.accepted) implies (
      cc.lastCompile.maxEmittedShapeId >= 1 and
      cc.lastCompile.maxEmittedShapeId <= SINSTR_SHAPE_COUNT and
      not(cc.lastCompile.universalKeysChanged)
    )

  // ECC-007: keccak256 lowers to one Chip instruction proved by R_SL_KECCAK256 (not Tier 3 or refusal).
  val inv_ECC_007: bool =
    (cc.lastCompile.evaluated and cc.lastCompile.circuitCase == CircKeccakValid) implies (
      cc.lastCompile.accepted and
      cc.lastCompile.keccakUsesChipRule and
      not(cc.lastCompile.keccakUsedTier3OrRefused)
    )

  // ECC-009: Deployable profile uses anchor slot 15, carries no spike constants, addresses no root key 15 in W,
  // and refuses user ledger fields at slot 15.
  val inv_ECC_009: bool =
    (cc.lastCompile.evaluated and cc.lastCompile.deployProfile and cc.lastCompile.accepted) implies (
      cc.lastCompile.anchorSlotUsed == DEPLOY_ANCHOR_SLOT and
      not(cc.lastCompile.hasSpikeConstants) and
      not(cc.lastCompile.effectAddressesRoot15) and
      not(cc.lastCompile.userUsesSlot15)
    )

  // ECC-010: Every circuit either compiles to an image or gets the refusal code of its FIRST failing check
  // in order read -> irVersion -> inputTypes -> lowering -> seal, with 0 unsupported circuits.
  val inv_ECC_010: bool =
    cc.lastCompile.evaluated implies (
      cc.lastCompile.unsupportedCount == 0 and
      (if (cc.lastCompile.firstFailingStage == STAGE_NONE)
        (cc.lastCompile.accepted and cc.lastCompile.emittedImageCount == 1)
       else
        (not(cc.lastCompile.accepted) and
         cc.lastCompile.reportedStage == cc.lastCompile.firstFailingStage and
         cc.lastCompile.refusalCode != "none"))
    )

  // ECC-011: Any golden vector mismatch against Lean OracleMain fails the oracle gate and names the vector.
  val inv_ECC_011: bool =
    (cc.gates.evaluated and cc.gates.hasGoldenOracleDiff) implies (
      not(cc.gates.oracleGatePassed) and
      not(cc.gates.gatesShPassed) and
      cc.gates.namedDiffVector != "none"
    )

  // ECC-012: Admitting every requested proof circuit writes 1 image + 1 ABI per circuit and exits 0.
  val inv_ECC_012: bool =
    (cc.batch.executed and cc.batch.refusedCircuitCount == 0) implies (
      cc.batch.writtenImageCount == cc.batch.requestedCount and
      cc.batch.writtenAbiCount == cc.batch.requestedCount and
      cc.batch.cliExitCode == 0
    )

  // ECC-013: Refusing any requested circuit exits non-zero, prints refusal_code, and writes 0 partial files.
  val inv_ECC_013: bool =
    (cc.batch.executed and cc.batch.refusedCircuitCount >= 1) implies (
      cc.batch.cliExitCode != 0 and
      cc.batch.printedRefusalCode != "none" and
      cc.batch.writtenImageCount == 0 and
      cc.batch.writtenAbiCount == 0
    )

  // ECC-014: Jubjub point input reads 2 raw tape words and checks on-curve + subgroup membership.
  val inv_ECC_014: bool =
    (cc.lastCompile.evaluated and cc.lastCompile.circuitCase == CircJubjubValid) implies (
      cc.lastCompile.accepted and
      cc.lastCompile.jubjubPointWordsRead == JUBJUB_POINT_WORDS and
      cc.lastCompile.jubjubCurveAndSubgroupChecked
    )

  // Completion CF-013: Scalar<Secp256k1> and Point<Secp256k1> lower to the Secp256k1 R_SL kit without InputType refusal.
  val inv_CF_013: bool =
    (cc.lastCompile.evaluated and cc.lastCompile.circuitCase == CircSecp256k1Valid) implies (
      cc.lastCompile.accepted and
      cc.lastCompile.secp256k1LoweredToKit and
      not(cc.lastCompile.secp256k1RefusedInputType)
    )

  // Completion CF-014: Unknown ZKIR input type with no R_SL kit (e.g., Curve25519) is refused naming InputType.
  val inv_CF_014: bool =
    (cc.lastCompile.evaluated and cc.lastCompile.circuitCase == CircUnknownCurve25519) implies (
      not(cc.lastCompile.accepted) and
      not(cc.lastCompile.unknownInputTypeAdmitted) and
      cc.lastCompile.reportedStage == STAGE_INPUT_TYPES and
      cc.lastCompile.refusalCode == "InputType:Scalar<Curve25519>"
    )

  // Completion LPC-013: lake build --wfail passes only when 0 raw sorries remain, and gates.sh requires mutant/oracle gates.
  val inv_LPC_013: bool =
    cc.gates.evaluated implies (
      (cc.gates.lakeBuildWfailPassed iff (cc.gates.sorriesRemainingCount == 0)) and
      (cc.gates.gatesShPassed implies (cc.gates.oracleGatePassed and not(cc.gates.hasUnarguedSurvivingMutant)))
    )

  // Completion LPC-019: Closing Mycroft design requires S1 either discharged or kept as a named hypothesis in the report.
  val inv_LPC_019: bool =
    cc.premises.mycroftDesignClosed implies (
      cc.premises.s1Discharged or
      (cc.premises.s1KeptAsNamedHypothesis and cc.premises.s1NamedInReport)
    )

  // Adopted CF-001: Non-prototype build taps Lflattened; prototype build reads ZKIR 3.1 only (ECC-001).
  val inv_CF_001: bool =
    (not(cc.lastCompile.isPrototypeBuild) implies cc.lastCompile.usesLflattenedTap) and
    (cc.lastCompile.isPrototypeBuild implies not(cc.lastCompile.usesLflattenedTap))

  // Adopted CF-002 (Scenario): `sherlock plan` runs on an image without reading a witness.
  val inv_CF_002: bool =
    cc.det.checked implies cc.det.planReadNoWitness

  // Adopted CF-004: Step count N_e is independent of the tape and `spend` takes at most 550 steps.
  val inv_CF_004: bool =
    cc.det.checked implies (
      cc.det.stepCountTape1 == cc.det.stepCountTape2 and
      cc.det.spendStepCount > 0 and
      cc.det.spendStepCount <= MAX_SPEND_STEPS
    )

  // Adopted CF-012: Compact Frontend kit is reported complete only when CF-L1 is proved under standard Lean axioms.
  val inv_CF_012: bool =
    cc.premises.cfKitReportedComplete implies (
      cc.premises.cfL1Proved and cc.premises.onlyStandardLeanAxioms
    )

  // Master safety invariant.
  val inv: bool =
    consistency_constants and
    inv_ECC_001 and
    inv_ECC_002 and
    inv_ECC_003 and
    inv_ECC_004 and
    inv_ECC_005 and
    inv_ECC_007 and
    inv_ECC_009 and
    inv_ECC_010 and
    inv_ECC_011 and
    inv_ECC_012 and
    inv_ECC_013 and
    inv_ECC_014 and
    inv_CF_013 and
    inv_CF_014 and
    inv_LPC_013 and
    inv_LPC_019 and
    inv_CF_001 and
    inv_CF_002 and
    inv_CF_004 and
    inv_CF_012

  // =========================================================================
  // Reachability witnesses (negated goals — MUST be violated under compact_compilation_main)
  // =========================================================================

  // Witness 1: Keccak256, Jubjub, and Secp256k1 circuits all compile cleanly (ECC-007, ECC-014, CF-013).
  val reach_keccak_jubjub_and_secp256k1_compiled: bool = not(
    cc.keccakCompileOk and cc.jubjubCompileOk and cc.secp256k1CompileOk
  )

  // Witness 2: Both an all-admitted batch (exit 0) and a refused batch (exit != 0, 0 files) execute (ECC-012, ECC-013).
  val reach_batch_compile_admit_and_refuse: bool = not(
    cc.allValidBatchCompleted and cc.refusedBatchCompleted
  )

  // Witness 3: Unknown Curve25519 input type and multi-fail IR/seal ordering refusal fire (ECC-010, CF-014).
  val reach_ordered_refusals_and_unknown_type: bool = not(
    cc.refusedCompileRuns >= 2 and
    cc.lastCompile.reportedStage == STAGE_INPUT_TYPES and
    cc.lastCompile.refusalCode == "InputType:Scalar<Curve25519>"
  )

  // Witness 4: Witness-invariance check, passing factory gates (`usableForSettledCall`), and `lake build --wfail` pass.
  val reach_witness_invariance_and_factory_gates_pass: bool = not(
    cc.det.checked and
    cc.gates.usableForSettledCall and
    cc.gates.lakeBuildWfailPassed
  )

  // Witness 5: Mycroft design closes with S1 named and CF-L1 proved under standard axioms (LPC-019, CF-012).
  val reach_premise_s1_and_cfl1_closure: bool = not(
    cc.premises.mycroftDesignClosed and cc.premises.cfKitReportedComplete
  )
}

// ===========================================================================
// Concrete instances: main, edge instances, and guard mutants
// ===========================================================================

module compact_compilation_main {
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
  ).*
}

// Minimum batch size (1 proof circuit per `sherlock compile` invocation).
module compact_compilation_edge_min {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13,
    RSL_RULE_COUNT = 16,
    DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3,
    JUBJUB_POINT_WORDS = 2,
    ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550,
    BATCH_CIRCUIT_COUNT = 1,
    MUTANT_ID = 0
  ).*
}

// Upper batch bound (4 proof circuits per `sherlock compile` invocation, exact spec limits).
module compact_compilation_edge_at_limit {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13,
    RSL_RULE_COUNT = 16,
    DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3,
    JUBJUB_POINT_WORDS = 2,
    ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550,
    BATCH_CIRCUIT_COUNT = 4,
    MUTANT_ID = 0
  ).*
}

// Guard mutants (each weakens or removes one refusal/binding guard and must break its named invariant)

module compact_compilation_m_ecc001_patched_or_wrong_zkir {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 1
  ).*
}

module compact_compilation_m_ecc002_settle_surviving_mutant {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 2
  ).*
}

module compact_compilation_m_ecc003_bad_abi_layout {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 3
  ).*
}

module compact_compilation_m_ecc004_witness_dependent_bytes {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 4
  ).*
}

module compact_compilation_m_ecc005_illegal_sinstr_shape {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 5
  ).*
}

module compact_compilation_m_ecc007_keccak_tier3_or_refuse {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 6
  ).*
}

module compact_compilation_m_ecc009_spike_slot_or_collision {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 7
  ).*
}

module compact_compilation_m_ecc010_wrong_refusal_order {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 8
  ).*
}

module compact_compilation_m_ecc011_ignore_oracle_diff {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 9
  ).*
}

module compact_compilation_m_ecc012_missing_abi_or_nonzero {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 10
  ).*
}

module compact_compilation_m_ecc013_partial_output_on_refusal {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 11
  ).*
}

module compact_compilation_m_ecc014_jubjub_missing_curve_check {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 12
  ).*
}

module compact_compilation_m_cf013_refuse_secp256k1_input {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 13
  ).*
}

module compact_compilation_m_cf014_admit_unknown_curve25519 {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 14
  ).*
}

module compact_compilation_m_cf004_tape_dependent_steps {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 15
  ).*
}

module compact_compilation_m_lpc019_close_with_unnamed_s1 {
  import compact_compilation(
    SINSTR_SHAPE_COUNT = 13, RSL_RULE_COUNT = 16, DEPLOY_ANCHOR_SLOT = 15,
    TAPE_HEADER_WORDS = 3, JUBJUB_POINT_WORDS = 2, ZKIR_SUPPORTED_VERSION = 31,
    MAX_SPEND_STEPS = 550, BATCH_CIRCUIT_COUNT = 2, MUTANT_ID = 16
  ).*
}
