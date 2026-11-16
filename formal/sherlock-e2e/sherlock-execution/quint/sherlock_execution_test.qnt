// -*- mode: Bluespec; -*-
// Deterministic scenario tests for bounded context Sherlock Execution (lane e2e-sherlock-execution).

module sherlock_execution_test {
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
  ).* from "./sherlock_execution"

  // 1. Rule-family separation, single universal key set, key_root binding, 120-rule gate, and 3 interpreted outcomes
  // Covers EUI-001..003, EUI-005, EUI-009, EUI-010, EKT-001, ERS-006, ERS-015, CORE-003, CORE-006, CPR-003, UIREQ.
  run se_ruleFamiliesUniversalKeysGovernanceAndInterpretedOutcomesTest =
    init
      .expect(inv and lastOp == "init")
      // Mismatched rule family in prog_d is refused (EUI-001)
      .then(doEvaluateFamilyKeysAndGovernance(FamRsl, FamInterpreted, true, true, false, false))
      .expect(inv and not(exec.family.familyLeafAccepted))
      // App key outside key_root is refused (ERS-015)
      .then(doEvaluateFamilyKeysAndGovernance(FamRsl, FamRsl, false, true, false, false))
      .expect(inv and not(exec.family.familyLeafAccepted))
      // Incomplete 120-rule coverage fails GP-1 gate (EUI-010, RFB-003, RFB-004)
      .then(doEvaluateFamilyKeysAndGovernance(FamInterpreted, FamInterpreted, true, false, true, false))
      .expect(
        inv and
        not(exec.family.gp1GatePassed) and
        exec.family.followedProcessorFinal
      )
      // Matching family + rooted app key + all 120 rules covered passes under 1 universal key set (EUI-002, EUI-003)
      .then(doEvaluateFamilyKeysAndGovernance(FamInterpreted, FamInterpreted, true, true, false, false))
      .expect(
        inv and
        exec.family.familyLeafAccepted and
        exec.family.gp1GatePassed and
        exec.family.universalKeyIdsUsed == Set(1) and
        exec.family.universalCeremoniesAdded == 0
      )
      // All three interpreted-family outcomes: Halted, Rejected (code 3), Exhausted (fuel) (EUI-005)
      .then(doRunInterpretedProgram(InterpHalted, false))
      .then(doRunInterpretedProgram(InterpRejected, false))
      .then(doRunInterpretedProgram(InterpExhausted, false))
      .expect(
        inv and
        exec.haltedInterpSeen and
        exec.rejectedInterpSeen and
        exec.exhaustedInterpSeen
      )

  // 2. Call record schema `rsl-call/1`, malformed/bad-tape refusals, static step height, PUBLIC rows, and executor equality
  // Covers EUI-004, EUI-006..008, ERS-001..003, ERS-005, ERS-011, ERS-013, ERS-014, CF-003, ZK-008.
  run se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest =
    init
      // Malformed record lacking `tape` is refused without panic and starts 0 proofs (ERS-014)
      .then(doExecuteRslCallRecord(true, true, true, false, false, false))
      .expect(
        inv and
        exec.rslExec.malformedRefusedNamed and
        not(exec.rslExec.processPanicked) and
        exec.rslExec.proofStartsCount == 0
      )
      // Leftover/bad tape returns refused and starts 0 proofs (EUI-007)
      .then(doExecuteRslCallRecord(false, false, true, false, false, false))
      .expect(
        inv and
        exec.rslExec.proofStartsCount == 0 and
        exec.rslExec.rslHasRejectCode and
        not(exec.rslExec.rslHasOutcome)
      )
      // Executor mismatch against Mycroft reference blocks suite (EUI-006); native mismatch makes 0 proofs (ERS-005)
      .then(doExecuteRslCallRecord(false, true, true, true, true, false))
      .expect(
        inv and
        exec.rslExec.suiteBlockedOnExecMismatch and
        exec.rslExec.proofStartsCount == 0 and
        not(exec.rslExec.proofMadeOnNativeMismatch)
      )
      // Clean halted R_SL call: 15 fields, instruction array, 1 + 81 PUBLIC rows, static height, cross-box equal
      .then(doExecuteRslCallRecord(false, true, true, false, false, false))
      .expect(
        inv and
        exec.rslExec.recordSchema == "rsl-call/1" and
        exec.rslExec.recordFieldCount == 15 and
        exec.rslExec.programIsInstructionArray and
        exec.rslExec.rslHasOutcome and
        not(exec.rslExec.rslHasRejectCode) and
        exec.rslExec.publicRowCount == 82 and
        not(exec.rslExec.tapeWordInPublicRows) and
        exec.rslExec.traceHeightTape1 == exec.rslExec.traceHeightTape2 and
        exec.rslExec.boxACommW == exec.rslExec.boxBCommW
      )

  // 3. R_SL kits: 6/9 AIR bucket, mixed-hash refusal, [0,0,n] sponge, forged trace refusal, secp/Jubjub/ALU canonicity
  // Covers EKT-002..007, EKT-009..011, CF-005.
  run se_rslKitsAirBucketsMixedHashSpongeAndCanonicityRefusalsTest =
    init
      // No-hash program: 6 AIRs with FP-1, SHA_IN empty, Poseidon start register (0, 0, 3)
      .then(doEvaluateRslKitsAndAlu(HashNone, false, false, false, false, false, false))
      .expect(
        inv and
        exec.kits.admittedProgram and
        exec.kits.appAirCountWithFp1 == 6 and
        exec.kits.shaInEmpty and
        exec.kits.spongeStartTriplet == (0, 0, 3)
      )
      // Single-hash SHA-256 or keccak256 program: 9 AIRs with FP-1
      .then(doEvaluateRslKitsAndAlu(HashKeccakOnly, false, false, false, false, false, false))
      .expect(
        inv and
        exec.kits.admittedProgram and
        exec.kits.appAirCountWithFp1 == 9 and
        not(exec.kits.shaInEmpty)
      )
      // Mixed SHA-256 + keccak256 program refused with named capacity error (EKT-004)
      .then(doEvaluateRslKitsAndAlu(HashMixedBoth, false, false, false, false, false, false))
      .expect(inv and not(exec.kits.admittedProgram) and exec.mixedHashRefusedSeen)
      // Unknown builtin chip pair refused and blocked (EKT-003)
      .then(doEvaluateRslKitsAndAlu(HashNone, true, false, false, false, false, false))
      .expect(inv and not(exec.kits.admittedProgram) and exec.kits.unknownChipPairBlocked)
      // Forged kit row refused by app verifier; non-canonical secp, off-subgroup Jubjub, and overflow ALU rejected
      .then(doEvaluateRslKitsAndAlu(HashSha256Only, false, true, true, true, true, false))
      .expect(
        inv and
        not(exec.kits.appVerifierAccepted) and
        exec.kits.badCryptoOrAluCallRejected and
        not(exec.kits.traceBuiltOnAluOverflow)
      )

  // 4. RslStatement emission on halted calls, suppression on rejected calls, input_d tape binding, and recursion binding
  // Covers ERS-007..010, ERS-012.
  run se_rslStatementBuilderInputBindingAndRecursionOutcomeBindingTest =
    init
      // Halted call emits 5-field RslStatement projecting to 4 public words
      .then(doBuildAndBindRslStatement(true, false, false, false))
      .expect(
        inv and
        exec.stmt.statementEmitted and
        exec.stmt.statementFieldCount == 5 and
        exec.stmt.projectedPublicWordCount == 4 and
        exec.stmt.leafAcceptedInputBinding and
        exec.stmt.recursionAcceptedBinding and
        exec.rslHaltedStmtSeen
      )
      // Rejected Compact call on T23 route emits no RslStatement (ERS-009)
      .then(doBuildAndBindRslStatement(false, false, false, false))
      .expect(inv and not(exec.stmt.statementEmitted) and exec.rslRefusedNoStmtSeen)
      // Swapped tape after input_d fixed and forged recursion outcome digest both refused (ERS-007, ERS-012)
      .then(doBuildAndBindRslStatement(true, true, true, false))
      .expect(
        inv and
        not(exec.stmt.leafAcceptedInputBinding) and
        not(exec.stmt.recursionAcceptedBinding) and
        exec.recursionForgeryRefusedSeen
      )

  // 5. Pure prog_d sensitivity checks (ERS-006) and spec-gap witnesses (Q-sherlock-execution-1..4)
  run se_specGapWitnessesAndProgdSensitivityTest =
    init
      .expect(
        inv and
        consistency_constants and
        computeProgd(1, 2, 3, 2) == computeProgd(1, 2, 3, 2) and
        computeProgd(1, 2, 3, 2) != computeProgd(2, 2, 3, 2) and
        computeProgd(1, 2, 3, 2) != computeProgd(1, 3, 3, 2) and
        computeProgd(1, 2, 3, 2) != computeProgd(1, 2, 4, 2) and
        computeProgd(1, 2, 3, 2) != computeProgd(1, 2, 3, 3) and
        gap_q_se_1_ekt001_16_rules_omits_secp256k1 and
        gap_q_se_2_mixed_hash_11_airs_outside_bucket and
        gap_q_se_3_rsl_call_missing_statement_inputs_and_requires_native and
        gap_q_se_4_rsl_has_no_provable_rejected_outcome
      )
}
