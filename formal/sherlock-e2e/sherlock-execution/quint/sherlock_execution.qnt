// -*- mode: Bluespec; -*-
// Independent Quint specification for bounded context Sherlock Execution (lane e2e-sherlock-execution).
// Sources:
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-universal-interpreter/spec.md (EUI-001..010)
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-rsl-kits/spec.md (EKT-001..007, 009..011)
//   - openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-rsl-statement/spec.md (ERS-001..003, 005..015)
//   - Adopted IDs: CORE-003,006,011,012,015; RFB-001,003,004; CF-003,005; CPR-003; ZK-008; UIREQ-001..035
//   - openspec/changes/sherlock-e2e-prototype-20261115/architecture/DDD.md (Sherlock Execution rows)

module sherlock_execution {
  // =========================================================================
  // Constants (numeric bounds named in EUI, EKT, ERS, CORE, and CF specs)
  // =========================================================================

  // EUI-001, EUI-010, RFB-003: total enabled step rules in the interpreted family (120).
  const INTERPRETED_RULE_COUNT: int
  // EKT-001: exact rule count in R_SL compile::RULES (16).
  const RSL_RULE_COUNT: int
  // EKT-002 Scenario, CF-005: R_SL app AIR count with FP-1 when no hash is called (6).
  const RSL_NO_HASH_AIRS_WITH_FP1: int
  // EKT-002, CF-005: R_SL app AIR count with FP-1 when SHA-256 or keccak256 is called (9).
  const RSL_SINGLE_HASH_AIRS_WITH_FP1: int
  // CORE-006: number of slot rule lanes S hashed into per-leaf rules_d (16).
  const CORE_SLOT_COUNT_S: int
  // ERS-001: exact field count of `rsl-call/1` JSON call record (`schema` + 14 fields = 15).
  const RSL_CALL_FIELD_COUNT: int
  // ERS-010: exact field count of `RslStatement` (`prog_d`, `input_d`, `outcome_d`, `comm`, `W` => 5).
  const RSL_STATEMENT_FIELD_COUNT: int
  // ERS-010 Scenario: public words projected on the proving box (`c_p`, `input_pub`, `outcome_pub`, `claim_pub` => 4).
  const STATEMENT_PUBLICS_WORD_COUNT: int
  // EKT-010: exact operation count in the Jubjub interface (7).
  const JUBJUB_OP_COUNT: int
  // ERS-012 Status: FP-1 BabyBear public-value capacity (54 words; entries like a01 have 81 > 54 effect words).
  const FP1_PUBLIC_WORDS: int
  // EKT-005: width of the midnight-circuits 7.3.0 Poseidon fixed-length sponge (3).
  const POSEIDON_SPONGE_WIDTH: int
  // Configurable fuel/effect-word bound varied across concrete instances (1..81).
  const MAX_RUN_FUEL_OR_EFFECTS: int
  // Mutant selector: 0 = honest specification; 1..27 = individual guard mutants.
  const MUTANT_ID: int

  // =========================================================================
  // Mutant identifiers
  // =========================================================================

  pure val MUT_NONE: int = 0
  pure val MUT_EUI001_WRONG_FAMILY_PROGD: int = 1
  pure val MUT_EUI002_SPLIT_UNIVERSAL_KEYS: int = 2
  pure val MUT_EUI003_NEW_KEY_PER_PROGRAM: int = 3
  pure val MUT_EUI004_OUTCOME_ON_REJECT: int = 4
  pure val MUT_EUI005_STEP_AFTER_TERMINAL: int = 5
  pure val MUT_EUI006_IGNORE_EXECUTOR_DIFF: int = 6
  pure val MUT_EUI007_PROVE_BAD_TAPE: int = 7
  pure val MUT_EUI008_DYNAMIC_TRACE_HEIGHT: int = 8
  pure val MUT_EUI009_OVERRIDE_PROCESSOR_FINAL: int = 9
  pure val MUT_EUI010_PASS_INCOMPLETE_120_RULES: int = 10
  pure val MUT_EKT002_INVALID_AIR_BUCKET: int = 11
  pure val MUT_EKT003_ADMIT_UNKNOWN_BUILTIN: int = 12
  pure val MUT_EKT004_ADMIT_MIXED_HASH: int = 13
  pure val MUT_EKT005_WRONG_SPONGE_INIT: int = 14
  pure val MUT_EKT007_ACCEPT_FORGED_KIT_TRACE: int = 15
  pure val MUT_EKT009_ACCEPT_NONCANONICAL_SECP: int = 16
  pure val MUT_EKT010_ACCEPT_OFF_SUBGROUP_JUBJUB: int = 17
  pure val MUT_EKT011_ACCEPT_OVERFLOW_ALU: int = 18
  pure val MUT_ERS002_SEND_RECORD_TO_UNOWNED_HOST: int = 19
  pure val MUT_ERS003_LEAK_TAPE_IN_PUBLIC_ROWS: int = 20
  pure val MUT_ERS005_PROVE_NATIVE_MISMATCH: int = 21
  pure val MUT_ERS007_ACCEPT_SWAPPED_TAPE: int = 22
  pure val MUT_ERS009_STATEMENT_FOR_REJECTED_CALL: int = 23
  pure val MUT_ERS012_ACCEPT_FORGED_OUTCOME_BIND: int = 24
  pure val MUT_ERS013_EMIT_DEBUG_STRING_PROGRAM: int = 25
  pure val MUT_ERS014_ADMIT_OR_PANIC_MALFORMED: int = 26
  pure val MUT_ERS015_ACCEPT_UNROOTED_APP_KEY: int = 27

  // =========================================================================
  // Domain types
  // =========================================================================

  type RuleFamily =
    | FamInterpreted
    | FamRsl

  type InterpOutcome =
    | InterpHalted
    | InterpRejected
    | InterpExhausted

  type HashProgramKind =
    | HashNone
    | HashSha256Only
    | HashKeccakOnly
    | HashMixedBoth

  type FamilyKeyState = {
    progFamily: RuleFamily,
    progdNamedFamily: RuleFamily,
    familyLeafAccepted: bool,
    machineVersion: int,
    priorVersionKeysValid: bool,
    universalKeyIdsUsed: Set[int],
    universalCeremoniesAdded: int,
    conflictWithProcessorFinal: bool,
    followedProcessorFinal: bool,
    uireqRetainedWhenUnmentioned: bool,
    coveredInterpRuleCount: int,
    gp1GatePassed: bool,
    rslRuleTableCount: int,
    appKeyInKeyRoot: bool,
    unrootedAppKeyAccepted: bool,
    distinctImagesDistinctProgd: bool,
    coreRulesDValid: bool
  }

  type InterpRunState = {
    executed: bool,
    outcomeKind: InterpOutcome,
    reportedOutcomeCount: int,
    remainingFramesAtHalt: int,
    stepAfterTerminalOccurred: bool,
    leafCloseMutatedState: bool,
    oneFuelPerStepCopied: bool,
    chunkPreservesSemantics: bool,
    cacheDeterministicBaseline: bool
  }

  type RslExecState = {
    evaluated: bool,
    recordSchema: str,
    recordFieldCount: int,
    programIsInstructionArray: bool,
    recordMalformed: bool,
    malformedRefusedNamed: bool,
    processPanicked: bool,
    recordMovedToUnownedHost: bool,
    tapeValid: bool,
    proofStartsCount: int,
    rslHasOutcome: bool,
    rslHasRejectCode: bool,
    openvmMatchesMycroftRef: bool,
    suiteBlockedOnExecMismatch: bool,
    openvmMatchesNative: bool,
    proofMadeOnNativeMismatch: bool,
    traceHeightTape1: int,
    traceHeightTape2: int,
    falseGuardWritesZeros: bool,
    runShapeKeptPrivate: bool,
    effectWordCount: int,
    publicRowCount: int,
    tapeWordInPublicRows: bool,
    boxACommW: int,
    boxBCommW: int
  }

  type KitVerifyState = {
    evaluated: bool,
    hashKind: HashProgramKind,
    admittedProgram: bool,
    appAirCountWithFp1: int,
    shaInEmpty: bool,
    unknownChipPairSeen: bool,
    unknownChipPairBlocked: bool,
    transientWordsN: int,
    spongeStartTriplet: (int, int, int),
    refCrateChanged: bool,
    kitSuiteReranBeforeSettle: bool,
    kitTraceViolatesRelation: bool,
    appVerifierAccepted: bool,
    secpScalarGeOrder: bool,
    jubjubPointOffSubgroup: bool,
    aluResultGeModulus: bool,
    badCryptoOrAluCallRejected: bool,
    traceBuiltOnAluOverflow: bool
  }

  type StatementBindState = {
    evaluated: bool,
    rslCallHalted: bool,
    statementEmitted: bool,
    statementFieldCount: int,
    outcomeKindIsHalted: bool,
    outcomePayloadBindsCommAndW: bool,
    projectedPublicWordCount: int,
    boundInputValD: int,
    consumedTapeValD: int,
    leafAcceptedInputBinding: bool,
    traceOutcomeDigest: int,
    suppliedRecursionOutcomeDigest: int,
    recursionAcceptedBinding: bool
  }

  type ExecutionState = {
    family: FamilyKeyState,
    interp: InterpRunState,
    rslExec: RslExecState,
    kits: KitVerifyState,
    stmt: StatementBindState,
    // Monotone reachability flags across random walks
    haltedInterpSeen: bool,
    rejectedInterpSeen: bool,
    exhaustedInterpSeen: bool,
    rslHaltedStmtSeen: bool,
    rslRefusedNoStmtSeen: bool,
    mixedHashRefusedSeen: bool,
    recursionForgeryRefusedSeen: bool
  }

  // =========================================================================
  // Observable state variables
  // =========================================================================

  var exec: ExecutionState
  var lastOp: str

  // =========================================================================
  // Pure helper functions & spec-gap witness predicates
  // =========================================================================

  pure def computeProgd(imgId: int, planId: int, keyRoot: int, machVer: int): int =
    imgId * 1000 + planId * 100 + keyRoot * 10 + machVer

  val consistency_constants: bool =
    INTERPRETED_RULE_COUNT == 120 and
    RSL_RULE_COUNT == 16 and
    RSL_NO_HASH_AIRS_WITH_FP1 == 6 and
    RSL_SINGLE_HASH_AIRS_WITH_FP1 == 9 and
    CORE_SLOT_COUNT_S == 16 and
    RSL_CALL_FIELD_COUNT == 15 and
    RSL_STATEMENT_FIELD_COUNT == 5 and
    STATEMENT_PUBLICS_WORD_COUNT == 4 and
    JUBJUB_OP_COUNT == 7 and
    FP1_PUBLIC_WORDS == 54 and
    POSEIDON_SPONGE_WIDTH == 3 and
    MAX_RUN_FUEL_OR_EFFECTS >= 1 and
    MAX_RUN_FUEL_OR_EFFECTS <= 81

  // Finding Q-sherlock-execution-1 witness:
  // EKT-001 fixes R_SL compile::RULES to exactly 16 named rules without secp256k1; adding R_SL_SECP yields 17 != 16.
  val gap_q_se_1_ekt001_16_rules_omits_secp256k1: bool = {
    val ekt001Rules = Set(
      "R_SL_FIELD", "R_SL_SELECT", "R_SL_EQ", "R_SL_LT",
      "R_SL_DIVMOD2", "R_SL_RANGE", "R_SL_ASSERT", "R_SL_TAPE",
      "R_SL_EFF", "R_SL_RET", "R_SL_HALT", "R_SL_SPONGE",
      "R_SL_BYTES", "R_SL_EC", "R_SL_SHA256", "R_SL_KECCAK256"
    )
    ekt001Rules.size() == RSL_RULE_COUNT and
      not(ekt001Rules.contains("R_SL_SECP")) and
      ekt001Rules.union(Set("R_SL_SECP")).size() != RSL_RULE_COUNT
  }

  // Finding Q-sherlock-execution-2 witness:
  // Base R_SL (5) + SHA-256 (2 unique) + Keccak (2 unique) + shared BITWISE8 (1) + FP-1 (1) = 11 AIRs,
  // falling outside the {6, 9} universal AIR bucket of EKT-002 and CF-005.
  val gap_q_se_2_mixed_hash_11_airs_outside_bucket: bool = {
    val baseAirs = 5
    val shaUniqueAirs = 2
    val keccakUniqueAirs = 2
    val sharedBitwiseAir = 1
    val fp1Air = 1
    val mixedTotal = baseAirs + shaUniqueAirs + keccakUniqueAirs + sharedBitwiseAir + fp1Air
    val allowedBuckets = Set(RSL_NO_HASH_AIRS_WITH_FP1, RSL_SINGLE_HASH_AIRS_WITH_FP1)
    mixedTotal == 11 and not(allowedBuckets.contains(mixedTotal))
  }

  // Finding Q-sherlock-execution-3 witness:
  // `rsl-call/1` (ERS-001) carries 15 fields without `prog_d`, `key_root`, `machine_version`, or 8-word `input_salt`,
  // while requiring test-only `native_words` and `native_comm` that production non-suite calls lack.
  val gap_q_se_3_rsl_call_missing_statement_inputs_and_requires_native: bool = {
    val ers001Fields = Set(
      "schema", "id", "circuit", "label", "index", "kits", "selector",
      "n_out", "l_e", "n_e", "tape_len", "program", "tape", "native_words", "native_comm"
    )
    ers001Fields.size() == RSL_CALL_FIELD_COUNT and
      not(ers001Fields.contains("prog_d")) and
      not(ers001Fields.contains("input_salt")) and
      ers001Fields.contains("native_words") and
      ers001Fields.contains("native_comm")
  }

  // Finding Q-sherlock-execution-4 witness:
  // Under EUI-001 (one rule family per program), EUI-004 (R_SL reject returns code and NO outcome), and ERS-008/010
  // (RslStatement emitted ONLY for halted calls), an R_SL program has 0 provable Rejected outcomes on any route.
  pure val gap_q_se_4_rsl_has_no_provable_rejected_outcome: bool = {
    val rslProgramFamiliesCount = 1
    val rslRejectedEmitsStatement = false
    rslProgramFamiliesCount == 1 and not(rslRejectedEmitsStatement)
  }

  // =========================================================================
  // Initialization
  // =========================================================================

  action init = all {
    exec' = {
      family: {
        progFamily: FamRsl,
        progdNamedFamily: FamRsl,
        familyLeafAccepted: true,
        machineVersion: 2,
        priorVersionKeysValid: true,
        universalKeyIdsUsed: Set(1),
        universalCeremoniesAdded: 0,
        conflictWithProcessorFinal: false,
        followedProcessorFinal: true,
        uireqRetainedWhenUnmentioned: true,
        coveredInterpRuleCount: INTERPRETED_RULE_COUNT,
        gp1GatePassed: true,
        rslRuleTableCount: RSL_RULE_COUNT,
        appKeyInKeyRoot: true,
        unrootedAppKeyAccepted: false,
        distinctImagesDistinctProgd: (computeProgd(1, 1, 7, 2) != computeProgd(2, 1, 7, 2)),
        coreRulesDValid: true
      },
      interp: {
        executed: false,
        outcomeKind: InterpHalted,
        reportedOutcomeCount: 1,
        remainingFramesAtHalt: 0,
        stepAfterTerminalOccurred: false,
        leafCloseMutatedState: false,
        oneFuelPerStepCopied: false,
        chunkPreservesSemantics: true,
        cacheDeterministicBaseline: true
      },
      rslExec: {
        evaluated: false,
        recordSchema: "rsl-call/1",
        recordFieldCount: RSL_CALL_FIELD_COUNT,
        programIsInstructionArray: true,
        recordMalformed: false,
        malformedRefusedNamed: true,
        processPanicked: false,
        recordMovedToUnownedHost: false,
        tapeValid: true,
        proofStartsCount: 1,
        rslHasOutcome: true,
        rslHasRejectCode: false,
        openvmMatchesMycroftRef: true,
        suiteBlockedOnExecMismatch: false,
        openvmMatchesNative: true,
        proofMadeOnNativeMismatch: false,
        traceHeightTape1: 64,
        traceHeightTape2: 64,
        falseGuardWritesZeros: true,
        runShapeKeptPrivate: true,
        effectWordCount: MAX_RUN_FUEL_OR_EFFECTS,
        publicRowCount: 1 + MAX_RUN_FUEL_OR_EFFECTS,
        tapeWordInPublicRows: false,
        boxACommW: 501,
        boxBCommW: 501
      },
      kits: {
        evaluated: false,
        hashKind: HashNone,
        admittedProgram: true,
        appAirCountWithFp1: RSL_NO_HASH_AIRS_WITH_FP1,
        shaInEmpty: true,
        unknownChipPairSeen: false,
        unknownChipPairBlocked: true,
        transientWordsN: 3,
        spongeStartTriplet: (0, 0, 3),
        refCrateChanged: false,
        kitSuiteReranBeforeSettle: true,
        kitTraceViolatesRelation: false,
        appVerifierAccepted: true,
        secpScalarGeOrder: false,
        jubjubPointOffSubgroup: false,
        aluResultGeModulus: false,
        badCryptoOrAluCallRejected: false,
        traceBuiltOnAluOverflow: false
      },
      stmt: {
        evaluated: false,
        rslCallHalted: true,
        statementEmitted: true,
        statementFieldCount: RSL_STATEMENT_FIELD_COUNT,
        outcomeKindIsHalted: true,
        outcomePayloadBindsCommAndW: true,
        projectedPublicWordCount: STATEMENT_PUBLICS_WORD_COUNT,
        boundInputValD: 301,
        consumedTapeValD: 301,
        leafAcceptedInputBinding: true,
        traceOutcomeDigest: 401,
        suppliedRecursionOutcomeDigest: 401,
        recursionAcceptedBinding: true
      },
      haltedInterpSeen: false,
      rejectedInterpSeen: false,
      exhaustedInterpSeen: false,
      rslHaltedStmtSeen: false,
      rslRefusedNoStmtSeen: false,
      mixedHashRefusedSeen: false,
      recursionForgeryRefusedSeen: false
    },
    lastOp' = "init"
  }

  // =========================================================================
  // Parameterized actions
  // =========================================================================

  // Implements EUI-001, EUI-002, EUI-003, EUI-009, EUI-010, EKT-001, ERS-006, ERS-015, CPR-003, CORE-006, RFB-003, RFB-004:
  // Checks rule-family admission, universal key sharing across both families, key_root app-key binding,
  // processor-final conflict precedence, and 120-rule interpreted coverage gate.
  action doEvaluateFamilyKeysAndGovernance(
    progFam: RuleFamily,
    progdFam: RuleFamily,
    appKeyRooted: bool,
    full120Covered: bool,
    procFinalConflict: bool,
    mutTrig: bool
  ): bool = {
    val famMatches = (progFam == progdFam)
    val mutWrongFam = (MUTANT_ID == MUT_EUI001_WRONG_FAMILY_PROGD and not(famMatches) and mutTrig)
    val mutSplitKeys = (MUTANT_ID == MUT_EUI002_SPLIT_UNIVERSAL_KEYS and mutTrig)
    val mutCeremony = (MUTANT_ID == MUT_EUI003_NEW_KEY_PER_PROGRAM and mutTrig)
    val mutOverridePf = (MUTANT_ID == MUT_EUI009_OVERRIDE_PROCESSOR_FINAL and procFinalConflict and mutTrig)
    val mutPassIncomplete = (MUTANT_ID == MUT_EUI010_PASS_INCOMPLETE_120_RULES and not(full120Covered) and mutTrig)
    val mutUnrootedKey = (MUTANT_ID == MUT_ERS015_ACCEPT_UNROOTED_APP_KEY and not(appKeyRooted) and mutTrig)

    val leafOk = (famMatches or mutWrongFam) and (appKeyRooted or mutUnrootedKey)
    val coveredCnt = if (full120Covered) INTERPRETED_RULE_COUNT else 7
    val nextFam: FamilyKeyState = {
      progFamily: progFam,
      progdNamedFamily: progdFam,
      familyLeafAccepted: leafOk,
      machineVersion: 2,
      priorVersionKeysValid: true,
      universalKeyIdsUsed: if (mutSplitKeys) Set(1, 2) else Set(1),
      universalCeremoniesAdded: if (mutCeremony) 1 else 0,
      conflictWithProcessorFinal: procFinalConflict,
      followedProcessorFinal: not(mutOverridePf),
      uireqRetainedWhenUnmentioned: true,
      coveredInterpRuleCount: coveredCnt,
      gp1GatePassed: if (mutPassIncomplete) true else full120Covered,
      rslRuleTableCount: RSL_RULE_COUNT,
      appKeyInKeyRoot: appKeyRooted,
      unrootedAppKeyAccepted: mutUnrootedKey,
      distinctImagesDistinctProgd: (computeProgd(1, 1, 7, 2) != computeProgd(2, 1, 7, 2)),
      coreRulesDValid: (CORE_SLOT_COUNT_S == 16)
    }
    all {
      exec' = {
        ...exec,
        family: nextFam
      },
      lastOp' = "evaluateFamilyKeysAndGovernance"
    }
  }

  // Implements EUI-005, CORE-003, CORE-011, CORE-012, CORE-015, UIREQ-001..012, UIREQ-016, UIREQ-035:
  // Executes an interpreted-family program ending in Halted, Rejected (code 3), or Exhausted (fuel).
  action doRunInterpretedProgram(outKind: InterpOutcome, mutTrig: bool): bool = {
    val mutPostTerm = (MUTANT_ID == MUT_EUI005_STEP_AFTER_TERMINAL and mutTrig)
    val nextInterp: InterpRunState = {
      executed: true,
      outcomeKind: outKind,
      reportedOutcomeCount: if (mutPostTerm) 2 else 1,
      remainingFramesAtHalt: 0,
      stepAfterTerminalOccurred: mutPostTerm,
      leafCloseMutatedState: mutPostTerm,
      oneFuelPerStepCopied: false,
      chunkPreservesSemantics: true,
      cacheDeterministicBaseline: true
    }
    all {
      exec' = {
        ...exec,
        interp: nextInterp,
        haltedInterpSeen: exec.haltedInterpSeen or (outKind == InterpHalted),
        rejectedInterpSeen: exec.rejectedInterpSeen or (outKind == InterpRejected),
        exhaustedInterpSeen: exec.exhaustedInterpSeen or (outKind == InterpExhausted)
      },
      lastOp' = "runInterpretedProgram"
    }
  }

  // Implements EUI-004, EUI-006, EUI-007, EUI-008, ERS-001..003, ERS-005, ERS-011, ERS-013, ERS-014, CF-003, ZK-008:
  // Validates `rsl-call/1` record, owned-box privacy, bad-tape proof refusal, static step count, PUBLIC rows,
  // cross-box determinism, and openvm/Mycroft/native agreement.
  action doExecuteRslCallRecord(
    malformedRec: bool,
    validTape: bool,
    callSucceeds: bool,
    execDiffersRef: bool,
    nativeDiffers: bool,
    mutTrig: bool
  ): bool = {
    val mutOutOnRej = (MUTANT_ID == MUT_EUI004_OUTCOME_ON_REJECT and not(callSucceeds) and mutTrig)
    val mutIgnoreExecDiff = (MUTANT_ID == MUT_EUI006_IGNORE_EXECUTOR_DIFF and execDiffersRef and mutTrig)
    val mutProveBadTape = (MUTANT_ID == MUT_EUI007_PROVE_BAD_TAPE and not(validTape) and mutTrig)
    val mutDynHeight = (MUTANT_ID == MUT_EUI008_DYNAMIC_TRACE_HEIGHT and mutTrig)
    val mutUnownedMove = (MUTANT_ID == MUT_ERS002_SEND_RECORD_TO_UNOWNED_HOST and mutTrig)
    val mutLeakPubRow = (MUTANT_ID == MUT_ERS003_LEAK_TAPE_IN_PUBLIC_ROWS and mutTrig)
    val mutProveNativeDiff = (MUTANT_ID == MUT_ERS005_PROVE_NATIVE_MISMATCH and nativeDiffers and mutTrig)
    val mutEmitDebugStr = (MUTANT_ID == MUT_ERS013_EMIT_DEBUG_STRING_PROGRAM and mutTrig)
    val mutMalformedPanic = (MUTANT_ID == MUT_ERS014_ADMIT_OR_PANIC_MALFORMED and malformedRec and mutTrig)

    val effLen = MAX_RUN_FUEL_OR_EFFECTS
    val nextRsl: RslExecState = {
      evaluated: true,
      recordSchema: if (mutEmitDebugStr) "mycroft-call/1" else "rsl-call/1",
      recordFieldCount: RSL_CALL_FIELD_COUNT,
      programIsInstructionArray: not(mutEmitDebugStr),
      recordMalformed: malformedRec,
      malformedRefusedNamed: if (malformedRec) not(mutMalformedPanic) else true,
      processPanicked: mutMalformedPanic,
      recordMovedToUnownedHost: mutUnownedMove,
      tapeValid: validTape,
      proofStartsCount:
        if (malformedRec) 0
        else if (not(validTape)) (if (mutProveBadTape) 1 else 0)
        else if (nativeDiffers) (if (mutProveNativeDiff) 1 else 0)
        else 1,
      rslHasOutcome: if (callSucceeds and validTape and not(malformedRec)) true else mutOutOnRej,
      rslHasRejectCode: not(callSucceeds and validTape and not(malformedRec)),
      openvmMatchesMycroftRef: not(execDiffersRef),
      suiteBlockedOnExecMismatch: if (execDiffersRef) not(mutIgnoreExecDiff) else false,
      openvmMatchesNative: not(nativeDiffers),
      proofMadeOnNativeMismatch: mutProveNativeDiff,
      traceHeightTape1: 64,
      traceHeightTape2: if (mutDynHeight) 96 else 64,
      falseGuardWritesZeros: not(mutDynHeight),
      runShapeKeptPrivate: true,
      effectWordCount: effLen,
      publicRowCount: 1 + effLen,
      tapeWordInPublicRows: mutLeakPubRow,
      boxACommW: 501,
      boxBCommW: 501
    }
    all {
      exec' = {
        ...exec,
        rslExec: nextRsl
      },
      lastOp' = "executeRslCallRecord"
    }
  }

  // Implements EKT-002..007, EKT-009..011, CF-005:
  // Evaluates R_SL kit AIR buckets, mixed-hash refusal, Poseidon [0,0,n] sponge, forged kit traces,
  // secp256k1/Jubjub subgroup & scalar checks, and canonical ALU overflow rejection.
  action doEvaluateRslKitsAndAlu(
    hKind: HashProgramKind,
    unkChip: bool,
    forgedKitRow: bool,
    badSecp: bool,
    badJubjub: bool,
    badAlu: bool,
    mutTrig: bool
  ): bool = {
    val mutBadBucket = (MUTANT_ID == MUT_EKT002_INVALID_AIR_BUCKET and mutTrig)
    val mutUnkAdmit = (MUTANT_ID == MUT_EKT003_ADMIT_UNKNOWN_BUILTIN and unkChip and mutTrig)
    val mutMixAdmit = (MUTANT_ID == MUT_EKT004_ADMIT_MIXED_HASH and hKind == HashMixedBoth and mutTrig)
    val mutBadSponge = (MUTANT_ID == MUT_EKT005_WRONG_SPONGE_INIT and mutTrig)
    val mutAcceptForged = (MUTANT_ID == MUT_EKT007_ACCEPT_FORGED_KIT_TRACE and forgedKitRow and mutTrig)
    val mutAcceptSecp = (MUTANT_ID == MUT_EKT009_ACCEPT_NONCANONICAL_SECP and badSecp and mutTrig)
    val mutAcceptJub = (MUTANT_ID == MUT_EKT010_ACCEPT_OFF_SUBGROUP_JUBJUB and badJubjub and mutTrig)
    val mutAcceptAlu = (MUTANT_ID == MUT_EKT011_ACCEPT_OVERFLOW_ALU and badAlu and mutTrig)

    val admitted =
      if (hKind == HashMixedBoth) mutMixAdmit
      else if (unkChip) mutUnkAdmit
      else true
    val airCnt =
      if (mutBadBucket) 11
      else if (hKind == HashNone) RSL_NO_HASH_AIRS_WITH_FP1
      else RSL_SINGLE_HASH_AIRS_WITH_FP1
    val anyBadInput = (badSecp or badJubjub or badAlu)
    val rejectedCall =
      if (not(anyBadInput)) false
      else not(mutAcceptSecp or mutAcceptJub or mutAcceptAlu)

    val nextKits: KitVerifyState = {
      evaluated: true,
      hashKind: hKind,
      admittedProgram: admitted,
      appAirCountWithFp1: airCnt,
      shaInEmpty: (hKind == HashNone),
      unknownChipPairSeen: unkChip,
      unknownChipPairBlocked: if (unkChip) not(mutUnkAdmit) else true,
      transientWordsN: 3,
      spongeStartTriplet: if (mutBadSponge) (1, 0, 3) else (0, 0, 3),
      refCrateChanged: true,
      kitSuiteReranBeforeSettle: true,
      kitTraceViolatesRelation: forgedKitRow,
      appVerifierAccepted: if (forgedKitRow) mutAcceptForged else true,
      secpScalarGeOrder: badSecp,
      jubjubPointOffSubgroup: badJubjub,
      aluResultGeModulus: badAlu,
      badCryptoOrAluCallRejected: rejectedCall,
      traceBuiltOnAluOverflow: (badAlu and mutAcceptAlu)
    }
    all {
      exec' = {
        ...exec,
        kits: nextKits,
        mixedHashRefusedSeen:
          exec.mixedHashRefusedSeen or (hKind == HashMixedBoth and not(admitted))
      },
      lastOp' = "evaluateRslKitsAndAlu"
    }
  }

  // Implements ERS-007, ERS-008, ERS-009, ERS-010, ERS-012:
  // Builds `RslStatement` for halted calls only, verifies `input_d` tape binding, and binds PUBLIC `(comm, W)`
  // into the recursion outcome commitment (including entries with > 54 effect words).
  action doBuildAndBindRslStatement(
    callHalted: bool,
    swappedTape: bool,
    forgedRecursionOutcome: bool,
    mutTrig: bool
  ): bool = {
    val mutAcceptSwap = (MUTANT_ID == MUT_ERS007_ACCEPT_SWAPPED_TAPE and swappedTape and mutTrig)
    val mutStmtOnRej = (MUTANT_ID == MUT_ERS009_STATEMENT_FOR_REJECTED_CALL and not(callHalted) and mutTrig)
    val mutAcceptForgedOut = (MUTANT_ID == MUT_ERS012_ACCEPT_FORGED_OUTCOME_BIND and forgedRecursionOutcome and mutTrig)

    val emitStmt = if (callHalted) true else mutStmtOnRej
    val leafInOk = if (swappedTape) mutAcceptSwap else true
    val recOutOk = if (forgedRecursionOutcome) mutAcceptForgedOut else true

    val nextStmt: StatementBindState = {
      evaluated: true,
      rslCallHalted: callHalted,
      statementEmitted: emitStmt,
      statementFieldCount: RSL_STATEMENT_FIELD_COUNT,
      outcomeKindIsHalted: callHalted,
      outcomePayloadBindsCommAndW: callHalted,
      projectedPublicWordCount: STATEMENT_PUBLICS_WORD_COUNT,
      boundInputValD: 301,
      consumedTapeValD: if (swappedTape) 999 else 301,
      leafAcceptedInputBinding: leafInOk,
      traceOutcomeDigest: 401,
      suppliedRecursionOutcomeDigest: if (forgedRecursionOutcome) 888 else 401,
      recursionAcceptedBinding: recOutOk
    }
    all {
      exec' = {
        ...exec,
        stmt: nextStmt,
        rslHaltedStmtSeen: exec.rslHaltedStmtSeen or (callHalted and emitStmt),
        rslRefusedNoStmtSeen: exec.rslRefusedNoStmtSeen or (not(callHalted) and not(emitStmt)),
        recursionForgeryRefusedSeen:
          exec.recursionForgeryRefusedSeen or (forgedRecursionOutcome and not(recOutOk))
      },
      lastOp' = "buildAndBindRslStatement"
    }
  }

  // =========================================================================
  // Nondeterministic step relation
  // =========================================================================

  action step = any {
    {
      nondet pFam = Set(FamInterpreted, FamRsl).oneOf()
      nondet dFam = Set(FamInterpreted, FamRsl).oneOf()
      nondet rooted = Set(true, false).oneOf()
      nondet cov120 = Set(true, false).oneOf()
      nondet pfConf = Set(false, true).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doEvaluateFamilyKeysAndGovernance(pFam, dFam, rooted, cov120, pfConf, mutTrig)
    },
    {
      nondet oKind = Set(InterpHalted, InterpRejected, InterpExhausted).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doRunInterpretedProgram(oKind, mutTrig)
    },
    {
      nondet mRec = Set(false, true).oneOf()
      nondet vTape = Set(true, false).oneOf()
      nondet cOk = Set(true, false).oneOf()
      nondet eDiff = Set(false, true).oneOf()
      nondet nDiff = Set(false, true).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doExecuteRslCallRecord(mRec, vTape, cOk, eDiff, nDiff, mutTrig)
    },
    {
      nondet hKind = Set(HashNone, HashSha256Only, HashKeccakOnly, HashMixedBoth).oneOf()
      nondet uChip = Set(false, true).oneOf()
      nondet fRow = Set(false, true).oneOf()
      nondet bSecp = Set(false, true).oneOf()
      nondet bJub = Set(false, true).oneOf()
      nondet bAlu = Set(false, true).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doEvaluateRslKitsAndAlu(hKind, uChip, fRow, bSecp, bJub, bAlu, mutTrig)
    },
    {
      nondet halted = Set(true, false).oneOf()
      nondet swTape = Set(false, true).oneOf()
      nondet fgOut = Set(false, true).oneOf()
      nondet mutTrig = Set(true, false).oneOf()
      doBuildAndBindRslStatement(halted, swTape, fgOut, mutTrig)
    }
  }

  // =========================================================================
  // Safety invariants (one inv_<ID> per expressible requirement)
  // =========================================================================

  val inv_EUI_001: bool =
    (exec.family.familyLeafAccepted implies (exec.family.progFamily == exec.family.progdNamedFamily)) and
    exec.family.priorVersionKeysValid

  val inv_EUI_002: bool =
    exec.family.universalKeyIdsUsed.size() == 1

  val inv_EUI_003: bool =
    exec.family.universalCeremoniesAdded == 0

  val inv_EUI_004: bool =
    exec.rslExec.evaluated implies (exec.rslExec.rslHasOutcome iff not(exec.rslExec.rslHasRejectCode))

  val inv_EUI_005: bool =
    exec.interp.executed implies (
      exec.interp.reportedOutcomeCount == 1 and
      not(exec.interp.stepAfterTerminalOccurred) and
      not(exec.interp.leafCloseMutatedState)
    )

  val inv_EUI_006: bool =
    (exec.rslExec.evaluated and not(exec.rslExec.openvmMatchesMycroftRef)) implies
      exec.rslExec.suiteBlockedOnExecMismatch

  val inv_EUI_007: bool =
    (exec.rslExec.evaluated and not(exec.rslExec.tapeValid)) implies
      (exec.rslExec.proofStartsCount == 0)

  val inv_EUI_008: bool =
    exec.rslExec.evaluated implies (
      exec.rslExec.traceHeightTape1 == exec.rslExec.traceHeightTape2 and
      exec.rslExec.falseGuardWritesZeros and
      exec.rslExec.runShapeKeptPrivate
    )

  val inv_EUI_009: bool =
    (exec.family.conflictWithProcessorFinal implies exec.family.followedProcessorFinal) and
    (not(exec.family.conflictWithProcessorFinal) implies exec.family.uireqRetainedWhenUnmentioned)

  val inv_EUI_010: bool =
    exec.family.gp1GatePassed implies
      (exec.family.coveredInterpRuleCount == INTERPRETED_RULE_COUNT)

  val inv_EKT_001: bool =
    exec.family.rslRuleTableCount == RSL_RULE_COUNT

  val inv_EKT_002: bool =
    (exec.kits.evaluated and exec.kits.admittedProgram) implies (
      Set(RSL_NO_HASH_AIRS_WITH_FP1, RSL_SINGLE_HASH_AIRS_WITH_FP1).contains(exec.kits.appAirCountWithFp1) and
      ((exec.kits.hashKind == HashNone) implies exec.kits.shaInEmpty)
    )

  val inv_EKT_003: bool =
    (exec.kits.evaluated and exec.kits.unknownChipPairSeen) implies
      (not(exec.kits.admittedProgram) and exec.kits.unknownChipPairBlocked)

  val inv_EKT_004: bool =
    (exec.kits.evaluated and exec.kits.hashKind == HashMixedBoth) implies
      not(exec.kits.admittedProgram)

  val inv_EKT_005: bool =
    exec.kits.evaluated implies
      (exec.kits.spongeStartTriplet == (0, 0, exec.kits.transientWordsN))

  val inv_EKT_006: bool =
    (exec.kits.evaluated and exec.kits.refCrateChanged) implies
      exec.kits.kitSuiteReranBeforeSettle

  val inv_EKT_007: bool =
    (exec.kits.evaluated and exec.kits.kitTraceViolatesRelation) implies
      not(exec.kits.appVerifierAccepted)

  val inv_EKT_009: bool =
    (exec.kits.evaluated and exec.kits.secpScalarGeOrder) implies
      exec.kits.badCryptoOrAluCallRejected

  val inv_EKT_010: bool =
    (exec.kits.evaluated and exec.kits.jubjubPointOffSubgroup) implies
      exec.kits.badCryptoOrAluCallRejected

  val inv_EKT_011: bool =
    (exec.kits.evaluated and exec.kits.aluResultGeModulus) implies (
      exec.kits.badCryptoOrAluCallRejected and not(exec.kits.traceBuiltOnAluOverflow)
    )

  val inv_ERS_001: bool =
    exec.rslExec.recordFieldCount == RSL_CALL_FIELD_COUNT

  val inv_ERS_002: bool =
    not(exec.rslExec.recordMovedToUnownedHost)

  val inv_ERS_003: bool =
    exec.rslExec.evaluated implies (
      exec.rslExec.publicRowCount == 1 + exec.rslExec.effectWordCount and
      not(exec.rslExec.tapeWordInPublicRows)
    )

  val inv_ERS_005: bool =
    (exec.rslExec.evaluated and not(exec.rslExec.openvmMatchesNative)) implies
      (not(exec.rslExec.proofMadeOnNativeMismatch) and exec.rslExec.proofStartsCount == 0)

  val inv_ERS_006: bool =
    exec.family.distinctImagesDistinctProgd

  val inv_ERS_007: bool =
    (exec.stmt.evaluated and exec.stmt.boundInputValD != exec.stmt.consumedTapeValD) implies
      not(exec.stmt.leafAcceptedInputBinding)

  val inv_ERS_008: bool =
    (exec.stmt.evaluated and exec.stmt.rslCallHalted and exec.stmt.statementEmitted) implies (
      exec.stmt.outcomeKindIsHalted and exec.stmt.outcomePayloadBindsCommAndW
    )

  val inv_ERS_009: bool =
    (exec.stmt.evaluated and not(exec.stmt.rslCallHalted)) implies
      not(exec.stmt.statementEmitted)

  val inv_ERS_010: bool =
    (exec.stmt.evaluated and exec.stmt.rslCallHalted) implies (
      exec.stmt.statementEmitted and
      exec.stmt.statementFieldCount == RSL_STATEMENT_FIELD_COUNT and
      exec.stmt.projectedPublicWordCount == STATEMENT_PUBLICS_WORD_COUNT
    )

  val inv_ERS_011: bool =
    exec.rslExec.boxACommW == exec.rslExec.boxBCommW

  val inv_ERS_012: bool =
    (exec.stmt.evaluated and exec.stmt.traceOutcomeDigest != exec.stmt.suppliedRecursionOutcomeDigest) implies
      not(exec.stmt.recursionAcceptedBinding)

  val inv_ERS_013: bool =
    exec.rslExec.evaluated implies (
      exec.rslExec.recordSchema == "rsl-call/1" and
      exec.rslExec.programIsInstructionArray
    )

  val inv_ERS_014: bool =
    (exec.rslExec.evaluated and exec.rslExec.recordMalformed) implies (
      exec.rslExec.malformedRefusedNamed and
      not(exec.rslExec.processPanicked) and
      exec.rslExec.proofStartsCount == 0
    )

  val inv_ERS_015: bool =
    (not(exec.family.appKeyInKeyRoot)) implies
      (not(exec.family.familyLeafAccepted) and not(exec.family.unrootedAppKeyAccepted))

  // Adopted CORE, CF, CPR, ZK, and UIREQ invariants checkable on Sherlock Execution state.
  val inv_adopted_core_cf_cpr_zk_uireq: bool =
    exec.family.coreRulesDValid and
    not(exec.interp.oneFuelPerStepCopied) and
    ((exec.interp.executed and exec.interp.outcomeKind == InterpHalted) implies
      (exec.interp.remainingFramesAtHalt == 0)) and
    exec.interp.chunkPreservesSemantics and
    exec.interp.cacheDeterministicBaseline

  // Master safety invariant.
  val inv: bool =
    consistency_constants and
    inv_EUI_001 and
    inv_EUI_002 and
    inv_EUI_003 and
    inv_EUI_004 and
    inv_EUI_005 and
    inv_EUI_006 and
    inv_EUI_007 and
    inv_EUI_008 and
    inv_EUI_009 and
    inv_EUI_010 and
    inv_EKT_001 and
    inv_EKT_002 and
    inv_EKT_003 and
    inv_EKT_004 and
    inv_EKT_005 and
    inv_EKT_006 and
    inv_EKT_007 and
    inv_EKT_009 and
    inv_EKT_010 and
    inv_EKT_011 and
    inv_ERS_001 and
    inv_ERS_002 and
    inv_ERS_003 and
    inv_ERS_005 and
    inv_ERS_006 and
    inv_ERS_007 and
    inv_ERS_008 and
    inv_ERS_009 and
    inv_ERS_010 and
    inv_ERS_011 and
    inv_ERS_012 and
    inv_ERS_013 and
    inv_ERS_014 and
    inv_ERS_015 and
    inv_adopted_core_cf_cpr_zk_uireq

  // =========================================================================
  // Reachability witnesses (negated goals — MUST be violated under sherlock_execution_main)
  // =========================================================================

  // Witness 1: All three interpreted-family outcomes (Halted, Rejected, Exhausted) are reached (EUI-005).
  val reach_all_three_interpreted_outcomes: bool = not(
    exec.haltedInterpSeen and exec.rejectedInterpSeen and exec.exhaustedInterpSeen
  )

  // Witness 2: Both halted R_SL statement emission and rejected R_SL statement suppression occur (ERS-008..010).
  val reach_rsl_halted_statement_and_rejected_suppression: bool = not(
    exec.rslHaltedStmtSeen and exec.rslRefusedNoStmtSeen
  )

  // Witness 3: Mixed-hash capacity refusal and recursion outcome-binding forgery refusal both fire (EKT-004, ERS-012).
  val reach_mixed_hash_and_recursion_forgery_refusals: bool = not(
    exec.mixedHashRefusedSeen and exec.recursionForgeryRefusedSeen
  )

  // Witness 4: Valid R_SL call execution with static trace height, PUBLIC rows, and 9-AIR hash kit admission.
  val reach_valid_rsl_hash_kit_and_call_execution: bool = not(
    exec.rslExec.evaluated and
    exec.rslExec.rslHasOutcome and
    exec.kits.evaluated and
    exec.kits.admittedProgram and
    exec.kits.appAirCountWithFp1 == RSL_SINGLE_HASH_AIRS_WITH_FP1
  )

  // Witness 5: Both rule-family admission check and GP-1 120-rule coverage gate pass under one universal key set.
  val reach_universal_key_both_families_and_gp1_gate: bool = not(
    exec.family.familyLeafAccepted and
    exec.family.gp1GatePassed and
    exec.family.universalKeyIdsUsed.size() == 1 and
    exec.interp.executed
  )
}

// ===========================================================================
// Concrete instances: main, edge instances, and guard mutants
// ===========================================================================

// Main instance with exact spec bounds (81 effect words as in a01 `test_all` > 54 FP-1 words).
module sherlock_execution_main {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120,
    RSL_RULE_COUNT = 16,
    RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9,
    CORE_SLOT_COUNT_S = 16,
    RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5,
    STATEMENT_PUBLICS_WORD_COUNT = 4,
    JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54,
    POSEIDON_SPONGE_WIDTH = 3,
    MAX_RUN_FUEL_OR_EFFECTS = 81,
    MUTANT_ID = 0
  ).*
}

// Minimum legal effect-word/fuel count (MAX_RUN_FUEL_OR_EFFECTS = 1).
module sherlock_execution_edge_min {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120,
    RSL_RULE_COUNT = 16,
    RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9,
    CORE_SLOT_COUNT_S = 16,
    RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5,
    STATEMENT_PUBLICS_WORD_COUNT = 4,
    JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54,
    POSEIDON_SPONGE_WIDTH = 3,
    MAX_RUN_FUEL_OR_EFFECTS = 1,
    MUTANT_ID = 0
  ).*
}

// Exact FP-1 boundary effect-word count (MAX_RUN_FUEL_OR_EFFECTS = 54 == FP1_PUBLIC_WORDS).
module sherlock_execution_edge_at_limit {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120,
    RSL_RULE_COUNT = 16,
    RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9,
    CORE_SLOT_COUNT_S = 16,
    RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5,
    STATEMENT_PUBLICS_WORD_COUNT = 4,
    JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54,
    POSEIDON_SPONGE_WIDTH = 3,
    MAX_RUN_FUEL_OR_EFFECTS = 54,
    MUTANT_ID = 0
  ).*
}

// Guard mutants (27 mutants, each weakening one refusal/binding guard and breaking its named invariant)

module sherlock_execution_m_eui001_wrong_family_progd {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 1
  ).*
}

module sherlock_execution_m_eui002_split_universal_keys {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 2
  ).*
}

module sherlock_execution_m_eui003_new_key_per_program {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 3
  ).*
}

module sherlock_execution_m_eui004_outcome_on_reject {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 4
  ).*
}

module sherlock_execution_m_eui005_step_after_terminal {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 5
  ).*
}

module sherlock_execution_m_eui006_ignore_executor_diff {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 6
  ).*
}

module sherlock_execution_m_eui007_prove_bad_tape {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 7
  ).*
}

module sherlock_execution_m_eui008_dynamic_trace_height {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 8
  ).*
}

module sherlock_execution_m_eui009_override_processor_final {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 9
  ).*
}

module sherlock_execution_m_eui010_pass_incomplete_120_rules {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 10
  ).*
}

module sherlock_execution_m_ekt002_invalid_air_bucket {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 11
  ).*
}

module sherlock_execution_m_ekt003_admit_unknown_builtin {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 12
  ).*
}

module sherlock_execution_m_ekt004_admit_mixed_hash {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 13
  ).*
}

module sherlock_execution_m_ekt005_wrong_sponge_init {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 14
  ).*
}

module sherlock_execution_m_ekt007_accept_forged_kit_trace {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 15
  ).*
}

module sherlock_execution_m_ekt009_accept_noncanonical_secp {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 16
  ).*
}

module sherlock_execution_m_ekt010_accept_off_subgroup_jubjub {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 17
  ).*
}

module sherlock_execution_m_ekt011_accept_overflow_alu {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 18
  ).*
}

module sherlock_execution_m_ers002_send_record_to_unowned_host {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 19
  ).*
}

module sherlock_execution_m_ers003_leak_tape_in_public_rows {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 20
  ).*
}

module sherlock_execution_m_ers005_prove_native_mismatch {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 21
  ).*
}

module sherlock_execution_m_ers007_accept_swapped_tape {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 22
  ).*
}

module sherlock_execution_m_ers009_statement_for_rejected_call {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 23
  ).*
}

module sherlock_execution_m_ers012_accept_forged_outcome_bind {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 24
  ).*
}

module sherlock_execution_m_ers013_emit_debug_string_program {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 25
  ).*
}

module sherlock_execution_m_ers014_admit_or_panic_malformed {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 26
  ).*
}

module sherlock_execution_m_ers015_accept_unrooted_app_key {
  import sherlock_execution(
    INTERPRETED_RULE_COUNT = 120, RSL_RULE_COUNT = 16, RSL_NO_HASH_AIRS_WITH_FP1 = 6,
    RSL_SINGLE_HASH_AIRS_WITH_FP1 = 9, CORE_SLOT_COUNT_S = 16, RSL_CALL_FIELD_COUNT = 15,
    RSL_STATEMENT_FIELD_COUNT = 5, STATEMENT_PUBLICS_WORD_COUNT = 4, JUBJUB_OP_COUNT = 7,
    FP1_PUBLIC_WORDS = 54, POSEIDON_SPONGE_WIDTH = 3, MAX_RUN_FUEL_OR_EFFECTS = 81, MUTANT_ID = 27
  ).*
}
