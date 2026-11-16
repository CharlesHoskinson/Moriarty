# Quint Formal Specification Report — Bounded Context: Sherlock Execution (`e2e-sherlock-execution`)

- **Specification**: [sherlock_execution.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/sherlock-execution/quint/sherlock_execution.qnt)
- **Scenario Tests**: [sherlock_execution_test.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/sherlock-execution/quint/sherlock_execution_test.qnt)
- **ITF Replay Traces (`20` traces)**: [traces/sherlock_execution_main/](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/sherlock-execution/traces/sherlock_execution_main) (`t_0.itf.json` .. `t_19.itf.json`)

---

## 1. Requirements Coverage Table

Every own EARS ID (`34` IDs from `sherlock-e2e-prototype-20261115`: `EUI-001..010`, `EKT-001..007`, `EKT-009..011`, `ERS-001..003`, `ERS-005..015`; `0` completion IDs assigned to this context) appears below, followed by the full table of adopted IDs (`CF`, `CORE`, `CPR`, `RFB`, `UIREQ`, `ZK`).

| ID | Artifact (`inv_*`, action + test, consistency `val`) | Kind | Note |
|---|---|---|---|
| `EUI-001` | `inv_EUI_001`, `doEvaluateFamilyKeysAndGovernance`, `se_ruleFamiliesUniversalKeysGovernanceAndInterpretedOutcomesTest` | MODELED | Binds each program to one rule family (`FamInterpreted` or `FamRsl`) named in `prog_d` (`progFamily == progdNamedFamily`) and preserves prior-version program keys (`priorVersionKeysValid`). Killed by `sherlock_execution_m_eui001_wrong_family_progd`. |
| `EUI-002` | `inv_EUI_002`, `doEvaluateFamilyKeysAndGovernance`, `se_ruleFamiliesUniversalKeysGovernanceAndInterpretedOutcomesTest` | MODELED | Proves both interpreted-family (`Rejected` / `Halted`) and `R_SL` programs under a single universal key set (`universalKeyIdsUsed.size() == 1`). Killed by `sherlock_execution_m_eui002_split_universal_keys`. |
| `EUI-003` | `inv_EUI_003`, `doEvaluateFamilyKeysAndGovernance`, `se_ruleFamiliesUniversalKeysGovernanceAndInterpretedOutcomesTest` | MODELED | Ensures adding a program requires `0` new universal keys, contract classes, or ceremonies (`universalCeremoniesAdded == 0`). Killed by `sherlock_execution_m_eui003_new_key_per_program`. |
| `EUI-004` | `inv_EUI_004`, `doExecuteRslCallRecord`, `se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest` | MODELED | Enforces mutual exclusion between returning an `R_SL` outcome `(comm, W, outputs)` and returning a reject code (`rslHasOutcome iff not(rslHasRejectCode)`). Killed by `sherlock_execution_m_eui004_outcome_on_reject`. Unprovability of `R_SL` rejects reported in `Q-sherlock-execution-4`. |
| `EUI-005` | `inv_EUI_005`, `doRunInterpretedProgram`, `reach_all_three_interpreted_outcomes`, `se_ruleFamiliesUniversalKeysGovernanceAndInterpretedOutcomesTest` | MODELED | Reports exactly `1` outcome (`InterpHalted`, `InterpRejected`, or `InterpExhausted`) per interpreted-family run, with zero steps after a terminal outcome (`CORE-003`). Killed by `sherlock_execution_m_eui005_step_after_terminal`. |
| `EUI-006` | `inv_EUI_006`, `doExecuteRslCallRecord`, `se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest` | MODELED | Blocks the suite whenever openvm's `R_SL` `(comm, W)` differs from Mycroft `sem::sl_run` (`not(openvmMatchesMycroftRef) implies suiteBlockedOnExecMismatch`). Killed by `sherlock_execution_m_eui006_ignore_executor_diff`. |
| `EUI-007` | `inv_EUI_007`, `doExecuteRslCallRecord`, `se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest` | MODELED | Returns `refused` and starts `0` proofs (`proofStartsCount == 0`) whenever a tape has a wrong selector, non-canonical word, missing word, or leftover word (`not(tapeValid)`). Killed by `sherlock_execution_m_eui007_prove_bad_tape`. |
| `EUI-008` | `inv_EUI_008`, `doExecuteRslCallRecord`, `se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest` | MODELED | Enforces static micro-op trace height across distinct tapes of an entry (`traceHeightTape1 == traceHeightTape2`) and zero-writing on false guards (`falseGuardWritesZeros`). Killed by `sherlock_execution_m_eui008_dynamic_trace_height`. |
| `EUI-009` | `inv_EUI_009`, `doEvaluateFamilyKeysAndGovernance`, `se_ruleFamiliesUniversalKeysGovernanceAndInterpretedOutcomesTest` | MODELED | Follows `sherlock-processor-final-20261111` on demonstrated `UIREQ` conflicts (`followedProcessorFinal`) while keeping unmentioned `UIREQ` clauses in force. Killed by `sherlock_execution_m_eui009_override_processor_final`. Omission of `modular-leaf` precedence reported in `Q-sherlock-execution-5`. |
| `EUI-010` | `inv_EUI_010`, `consistency_constants`, `doEvaluateFamilyKeysAndGovernance`, `se_ruleFamiliesUniversalKeysGovernanceAndInterpretedOutcomesTest` | CONSTANT-CHECKED | Requires `coveredInterpRuleCount == INTERPRETED_RULE_COUNT (120)` before `gp1GatePassed` holds (`RFB-003`, `RFB-004`). Killed by `sherlock_execution_m_eui010_pass_incomplete_120_rules`. |
| `EKT-001` | `inv_EKT_001`, `consistency_constants`, `gap_q_se_1_ekt001_16_rules_omits_secp256k1`, `se_specGapWitnessesAndProgdSensitivityTest` | CONSTANT-CHECKED | Enforces `rslRuleTableCount == RSL_RULE_COUNT (16)` for chip set 3; omission of `secp256k1` from the 16-rule table and `rules_d` symbol collision with `CORE-006` reported in `Q-sherlock-execution-1`. |
| `EKT-002` | `inv_EKT_002`, `consistency_constants`, `doEvaluateRslKitsAndAlu`, `se_rslKitsAirBucketsMixedHashSpongeAndCanonicityRefusalsTest` | CONSTANT-CHECKED | Restricts admitted `R_SL` apps to `{RSL_NO_HASH_AIRS_WITH_FP1 (6), RSL_SINGLE_HASH_AIRS_WITH_FP1 (9)}` AIRs with `SHA_IN` empty when no hash is called. Killed by `sherlock_execution_m_ekt002_invalid_air_bucket`. |
| `EKT-003` | `inv_EKT_003`, `doEvaluateRslKitsAndAlu`, `se_rslKitsAirBucketsMixedHashSpongeAndCanonicityRefusalsTest` | MODELED | Refuses unknown `(kit, builtin_id)` pairs with `unsupported` and counts the call as blocked (`unknownChipPairBlocked`). Killed by `sherlock_execution_m_ekt003_admit_unknown_builtin`. |
| `EKT-004` | `inv_EKT_004`, `gap_q_se_2_mixed_hash_11_airs_outside_bucket`, `doEvaluateRslKitsAndAlu`, `se_rslKitsAirBucketsMixedHashSpongeAndCanonicityRefusalsTest` | MODELED | Refuses mixed `persistentHash` + `keccak256` programs (`HashMixedBoth`) with named capacity refusal (`not(admittedProgram)`). Killed by `sherlock_execution_m_ekt004_admit_mixed_hash`. Bucket contradiction (`11` AIRs vs `{6, 9}`) reported in `Q-sherlock-execution-2`. |
| `EKT-005` | `inv_EKT_005`, `consistency_constants`, `doEvaluateRslKitsAndAlu`, `se_rslKitsAirBucketsMixedHashSpongeAndCanonicityRefusalsTest` | CONSTANT-CHECKED | Enforces width-`3` Poseidon fixed-length sponge initialization `(0, 0, transientWordsN)` for `transientHash`. Killed by `sherlock_execution_m_ekt005_wrong_sponge_init`. |
| `EKT-006` | `inv_EKT_006`, `doEvaluateRslKitsAndAlu`, `se_rslKitsAirBucketsMixedHashSpongeAndCanonicityRefusalsTest` | MODELED | Requires rerunning pinned reference-crate tests (`sha2`, Ethereum vectors, `midnight-curves 0.3.1`, `k256 0.13.4`, `midnight-circuits 7.3.0`) whenever a reference version changes (`kitSuiteReranBeforeSettle`). |
| `EKT-007` | `inv_EKT_007`, `doEvaluateRslKitsAndAlu`, `se_rslKitsAirBucketsMixedHashSpongeAndCanonicityRefusalsTest` | MODELED | Refuses any `R_SL` app proof whose trace rows violate the kit relation for the admitted inputs (`kitTraceViolatesRelation implies not(appVerifierAccepted)`). Killed by `sherlock_execution_m_ekt007_accept_forged_kit_trace`. |
| `EKT-009` | `inv_EKT_009`, `doEvaluateRslKitsAndAlu`, `se_rslKitsAirBucketsMixedHashSpongeAndCanonicityRefusalsTest` | MODELED | Rejects `secp256k1` calls with non-canonical scalar `>=` group order (`secpScalarGeOrder implies badCryptoOrAluCallRejected`). Killed by `sherlock_execution_m_ekt009_accept_noncanonical_secp`. Uncommitted `SECP-INTERFACE.md` reported in `Q-sherlock-execution-6`. |
| `EKT-010` | `inv_EKT_010`, `consistency_constants`, `doEvaluateRslKitsAndAlu`, `se_rslKitsAirBucketsMixedHashSpongeAndCanonicityRefusalsTest` | CONSTANT-CHECKED | Checks `JUBJUB_OP_COUNT == 7` operations and rejects `FromCoordinates` points off the prime-order subgroup (`jubjubPointOffSubgroup implies badCryptoOrAluCallRejected`). Killed by `sherlock_execution_m_ekt010_accept_off_subgroup_jubjub`. Missing committed Jubjub interface file reported in `Q-sherlock-execution-6`. |
| `EKT-011` | `inv_EKT_011`, `doEvaluateRslKitsAndAlu`, `se_rslKitsAirBucketsMixedHashSpongeAndCanonicityRefusalsTest` | MODELED | Rejects exact ALU micro-ops or hints `>=` modulus before building a trace (`badCryptoOrAluCallRejected and not(traceBuiltOnAluOverflow)`). Killed by `sherlock_execution_m_ekt011_accept_overflow_alu`. |
| `ERS-001` | `inv_ERS_001`, `consistency_constants`, `gap_q_se_3_rsl_call_missing_statement_inputs_and_requires_native`, `doExecuteRslCallRecord`, `se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest` | CONSTANT-CHECKED | Enforces `RSL_CALL_FIELD_COUNT == 15` fields in `rsl-call/1`. Missing `prog_d` / `input_salt` and mandatory test-only `native_words` / `native_comm` reported in `Q-sherlock-execution-3`. |
| `ERS-002` | `inv_ERS_002`, `doExecuteRslCallRecord`, `se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest` | MODELED | Keeps call records and `RslStatement` records strictly on user-owned roster boxes (`not(recordMovedToUnownedHost)`). Killed by `sherlock_execution_m_ers002_send_record_to_unowned_host`. |
| `ERS-003` | `inv_ERS_003`, `doExecuteRslCallRecord`, `se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest` | MODELED | Emits `1 + effectWordCount` `PUBLIC` rows (`comm` followed by `W` in order) with zero private tape/witness/`rand` words (`not(tapeWordInPublicRows)`). Killed by `sherlock_execution_m_ers003_leak_tape_in_public_rows`. |
| `ERS-005` | `inv_ERS_005`, `doExecuteRslCallRecord`, `se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest` | MODELED | Requires openvm `R_SL` `(comm, W)` to equal `(native_comm, native_words)` (`CF-003`), returning `"different"` and building `0` proofs on mismatch. Killed by `sherlock_execution_m_ers005_prove_native_mismatch`. |
| `ERS-006` | `inv_ERS_006`, `computeProgd`, `se_specGapWitnessesAndProgdSensitivityTest` | MODELED | Binds `prog_d` deterministically to `(image, plan, key_root, machine_version)` while keeping universal keys unchanged across distinct images. |
| `ERS-007` | `inv_ERS_007`, `doBuildAndBindRslStatement`, `se_rslStatementBuilderInputBindingAndRecursionOutcomeBindingTest` | MODELED | Refuses leaf proofs if the consumed tape digest differs from `val_d` inside `input_d` (`boundInputValD != consumedTapeValD implies not(leafAcceptedInputBinding)`). Killed by `sherlock_execution_m_ers007_accept_swapped_tape`. |
| `ERS-008` | `inv_ERS_008`, `doBuildAndBindRslStatement`, `se_rslStatementBuilderInputBindingAndRecursionOutcomeBindingTest` | MODELED | Sets halted `R_SL` outcome kind to `Halted` with payload `vd([comm] ++ W)` (`outcomeKindIsHalted and outcomePayloadBindsCommAndW`). |
| `ERS-009` | `inv_ERS_009`, `gap_q_se_4_rsl_has_no_provable_rejected_outcome`, `doBuildAndBindRslStatement`, `se_rslStatementBuilderInputBindingAndRecursionOutcomeBindingTest` | MODELED | Suppresses `RslStatement` emission on rejected `T23` Compact calls (`not(rslCallHalted) implies not(statementEmitted)`). Killed by `sherlock_execution_m_ers009_statement_for_rejected_call`. Unprovable-`Rejected` contradiction in `ERS-009` Status reported in `Q-sherlock-execution-4`. |
| `ERS-010` | `inv_ERS_010`, `consistency_constants`, `doBuildAndBindRslStatement`, `se_rslStatementBuilderInputBindingAndRecursionOutcomeBindingTest` | CONSTANT-CHECKED | Emits `1` `RslStatement` with `RSL_STATEMENT_FIELD_COUNT == 5` fields (`prog_d`, `input_d`, `outcome_d`, `comm`, `W`) projecting on the box to `STATEMENT_PUBLICS_WORD_COUNT == 4` public words. |
| `ERS-011` | `inv_ERS_011`, `doExecuteRslCallRecord`, `se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest` | MODELED | Ensures cross-box (`aarch64` vs `x86_64`) execution produces byte-equal `(comm, W)` (`boxACommW == boxBCommW`). |
| `ERS-012` | `inv_ERS_012`, `consistency_constants`, `doBuildAndBindRslStatement`, `se_rslStatementBuilderInputBindingAndRecursionOutcomeBindingTest` | CONSTANT-CHECKED | Binds `PUBLIC` `(comm, W)` (tested at `81 > FP1_PUBLIC_WORDS == 54` effect words) to the recursion outcome digest (`traceOutcomeDigest == suppliedRecursionOutcomeDigest`). Killed by `sherlock_execution_m_ers012_accept_forged_outcome_bind`. |
| `ERS-013` | `inv_ERS_013`, `doExecuteRslCallRecord`, `se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest` | MODELED | Requires emitted prover call files to use schema `"rsl-call/1"` with `program` as an instruction-object array (`programIsInstructionArray`). Killed by `sherlock_execution_m_ers013_emit_debug_string_program`. |
| `ERS-014` | `inv_ERS_014`, `doExecuteRslCallRecord`, `se_rslCallRecordSchemaBadTapeStaticStepsAndNativeAgreementTest` | MODELED | Returns named `Refuse::Unsupported` without panicking and starts `0` proofs on malformed call records (`malformedRefusedNamed and not(processPanicked)`). Killed by `sherlock_execution_m_ers014_admit_or_panic_malformed`. |
| `ERS-015` | `inv_ERS_015`, `doEvaluateFamilyKeysAndGovernance`, `se_ruleFamiliesUniversalKeysGovernanceAndInterpretedOutcomesTest` | MODELED | Commits per-program `R_SL` app keys under `key_root` inside `prog_d` and refuses leaf proofs under unrooted app keys (`not(appKeyInKeyRoot) implies not(familyLeafAccepted)`). Killed by `sherlock_execution_m_ers015_accept_unrooted_app_key`. |

### Adopted IDs Assessment (`PROMPT.md:45-47`)

| Adopted ID(s) | Owning Change | Disposition in `sherlock_execution.qnt` | Note |
|---|---|---|---|
| `CF-003` | `processor-final` | MODELED (`inv_ERS_005`) | Word-for-word equality of `W` (`effect_vector(preimage)`) with `native_words`. |
| `CF-005` | `processor-final` | CONSTANT-CHECKED (`inv_EKT_002`, `inv_EKT_005`) | `{6, 9}` AIR bucket with `FP-1`, fixed-length Poseidon `transientHash`, and `SHA-256` / `keccak256` / curve kits. |
| `CORE-003` | `modular-leaf` | MODELED (`inv_EUI_005`) | No step follows a terminal outcome (`not(stepAfterTerminalOccurred)`) and closing a leaf mutates nothing (`not(leafCloseMutatedState)`). |
| `CORE-006` | `modular-leaf` | CONSTANT-CHECKED (`inv_adopted_core_cf_cpr_zk_uireq`, `CORE_SLOT_COUNT_S = 16`) | `rules_d` hashes `S = 16` slot rule lanes; naming collision with `EKT-001` `rules_d` reported in `Q-sherlock-execution-1`. |
| `CORE-011`, `CORE-015` | `modular-leaf` | Not modeled here (Leaf Proving / AIR chip-row transition level) | Per-slot AIR register write-set and chip-set-2 ROM `vd` opening belong to Slice B `Leaf Proving`. |
| `CORE-012` | `modular-leaf` | MODELED (`inv_adopted_core_cf_cpr_zk_uireq`) | Enforces `not(oneFuelPerStepCopied)` (no spike 1-fuel-per-step rule); tension with `UIREQ-028` reported in `Q-sherlock-execution-5`. |
| `CPR-003` | `processor-final` | MODELED (`inv_EUI_002`, `inv_EUI_003`, `inv_ERS_015`) | Single universal key set across programs with per-program keys under `key_root`; missing `L_Fr` key in `CPR-003`'s 3-key list noted in `Q-sherlock-execution-5`. |
| `RFB-001`, `RFB-003`, `RFB-004` | `modular-leaf` | MODELED (`inv_EUI_010`) | `120`-rule projection coverage gate (`gp1GatePassed` fails on any uncovered rule or gap). |
| `ZK-008` | `processor-final` | MODELED (`inv_EUI_008`) | `n_steps`, per-AIR heights, and run shape kept private (`runShapeKeptPrivate`) with static `R_SL` step counts. |
| `UIREQ-001`, `002`, `010`, `011`, `012`, `016`, `035` | `universal-interpreter` | MODELED (`inv_EUI_001`, `inv_EUI_005`, `inv_adopted_core_cf_cpr_zk_uireq`) | Fixed versioned transition rules, `Halted` only at `remainingFramesAtHalt == 0`, bounded `Exhausted`/`Rejected` outcomes, chunk transparency (`chunkPreservesSemantics`), and cache-invariant deterministic witness baseline (`cacheDeterministicBaseline`). |
| `UIREQ-003..009`, `013..015`, `017..034` | `universal-interpreter` | Not modeled here | Host-expression micro-semantics (lexical scoping, left-to-right operand order, limb carry APIs), future suspension/financial-segment extensions (`UIREQ-014..020`), and Candidate B documentation/evidence-package clauses (`UIREQ-021..026`, `031`), governed by `EUI-009` (`Q-sherlock-execution-5`). |

---

## 2. Findings

### `Q-sherlock-execution-1`: `EKT-001`'s Exact `16`-Rule `compile::RULES` Table Omits `secp256k1` (`EKT-003`, `EKT-009`), and `rules_d` Collides With Adopted `CORE-006`

- **IDs**: `EKT-001`, `EKT-003`, `EKT-009`, `CORE-006`
- **Quoted EARS Sentences**:
  - `EKT-001`: *"EKT-001: The R_SL chip set SHALL have exactly the 16 rules of `compile::RULES`: `R_SL_FIELD`, `R_SL_SELECT`, `R_SL_EQ`, `R_SL_LT`, `R_SL_DIVMOD2`, `R_SL_RANGE`, `R_SL_ASSERT`, `R_SL_TAPE`, `R_SL_EFF`, `R_SL_RET`, `R_SL_HALT`, `R_SL_SPONGE`, `R_SL_BYTES`, `R_SL_EC`, `R_SL_SHA256` and `R_SL_KECCAK256`."*
  - `EKT-003`: *"EKT-003: When the R_SL compiler reads a `chip` instruction of a suite program, the R_SL compiler SHALL lower the pair of kit and builtin id to micro-ops."*
  - `EKT-009`: *"EKT-009: The secp256k1 kit SHALL accept exactly the operations, type tags and word layouts of `SECP-INTERFACE.md` sections 2 to 4."*
  - `CORE-006`: *"CORE SHALL compute `rules_d = Poseidon(dom("SHK_RULES"), rid_0, ..., rid_(S-1))` over exactly S lanes in slot order in one whole-sponge HASH slot, with `rid_j = 0` for a disabled slot, and SHALL bind it to the leaf public value `rules_d` (word 32, FP-1)."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_se_1_ekt001_16_rules_omits_secp256k1` (`sherlock_execution.qnt:213-222`): `EKT-001` enumerates all `16` entries of `compile::RULES` (`R_SL_EC` covers Jubjub, `R_SL_SHA256` covers SHA-256, `R_SL_KECCAK256` covers Keccak), leaving **no rule entry for `secp256k1`**. Either `secp256k1` executes outside the `rules_d`-digested rule table (so `EKT-001`'s `rules_d` fails to authenticate `secp256k1` rule semantics), or adding `R_SL_SECP` produces `17 != 16` rules and violates `EKT-001`. Additionally, `EKT-001` uses the identifier `rules_d` for the static chip-set rule-table digest in the key manifest, colliding with `CORE-006`'s dynamic per-leaf `rules_d` (`FP-1` word `32`).
- **Closing Fix**:
  - Explicitly state in `EKT-001` and `EKT-009` whether `secp256k1` adds `R_SL_SECP` as a `17`th rule before `T25` or lowers purely into existing `R_SL_FIELD` / `R_SL_RANGE` / `R_SL_ASSERT` rules with zero distinct rule ID, and rename chip set 3's static table digest (`chip_rules_d`) so it does not alias `CORE-006`'s per-leaf `rules_d`.

### `Q-sherlock-execution-2`: Fixed `{6, 9}` AIR Bucket (`EKT-002`, `CF-005`) Structurally Excludes Valid Compact Programs Calling Both `persistentHash` and `keccak256` (`EKT-004`)

- **IDs**: `EKT-002`, `EKT-004`, `CF-005`
- **Quoted EARS Sentences**:
  - `EKT-002`: *"EKT-002: The R_SL chip set SHALL prove every rule with the AIRs WORD, ALU, PUBLIC, RANGE and SHA_IN, plus SHA_REQ, SHA_HASHER and BITWISE8 for a SHA-256 program or KECCAK_REQ, KECCAK_PERM and BITWISE8 for a keccak256 program."*
  - `CF-005`: *"Every R_SL app SHALL have 6 or 9 AIRs with FP-1."*
  - `EKT-004`: *"EKT-004: While the R_SL chip set has no path that proves SHA-256 and keccak256 in one program, if a program calls both `persistentHash` and `keccak256`, then the R_SL compiler SHALL refuse the program with `Refuse::Unsupported` and the text "persistentHash and keccak256 in one program"."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_se_2_mixed_hash_11_airs_outside_bucket` (`sherlock_execution.qnt:227-236`): combining the `5` base `R_SL` AIRs, `2` unique SHA-256 AIRs (`SHA_REQ`, `SHA_HASHER`), `2` unique Keccak AIRs (`KECCAK_REQ`, `KECCAK_PERM`), `1` shared `BITWISE8` AIR, and `1` `FP-1` AIR requires **`11` AIRs**, which is outside `Set(6, 9)` mandated universally by `CF-005` and `EKT-002` (*"SHALL prove every rule..."*). Any valid Compact circuit calling both standard builtins is permanently unprovable unless `CF-005` / `EKT-002` admits an `11`-AIR bucket.
- **Closing Fix**:
  - Either add `11` (`10` app AIRs + `FP-1`) as a third universal leaf AIR bucket in `EKT-002` and `CF-005` before `T25`, or document the single-hash-family-per-circuit restriction as a normative Compact Compilation refusal in `ECC-010` (`lowering` stage) rather than an undocumented downstream prover failure.

### `Q-sherlock-execution-3`: `rsl-call/1` Schema (`ERS-001`, `ERS-013`, `ERS-014`) Omits `prog_d` and `input_d` Inputs Needed by `ERS-010` While Mandating Test-Only `native_words` and `native_comm`

- **IDs**: `ERS-001`, `ERS-006`, `ERS-007`, `ERS-010`, `ERS-013`, `ERS-014`
- **Quoted EARS Sentences**:
  - `ERS-001`: *"ERS-001: When the `mycroft-emit` tool writes one suite call, the `mycroft-emit` tool SHALL write one JSON file with `schema` equal to `rsl-call/1` and the fields `id`, `circuit`, `label`, `index`, `kits`, `selector`, `n_out`, `l_e`, `n_e`, `tape_len`, `program`, `tape`, `native_words` and `native_comm`."*
  - `ERS-013`: *"ERS-013: When the Mycroft compiler emits a call for the prover, the Mycroft compiler SHALL write the call in schema `rsl-call/1` with the program as an array of instruction objects."*
  - `ERS-014`: *"ERS-014: If a call record lacks a field or holds a field of a wrong type, then the R_SL compiler SHALL return `Refuse::Unsupported` with the field name."*
  - `ERS-010`: *"ERS-010: The R_SL statement builder SHALL write for each halted call one `RslStatement` record with exactly the fields `prog_d`, `input_d`, `outcome_d`, `comm` and `W`."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_se_3_rsl_call_missing_statement_inputs_and_requires_native` (`sherlock_execution.qnt:241-250`):
    1. `rsl-call/1` lacks `prog_d` (or `(plan, key_root, machine_version)` required by `ERS-006`) and lacks the 8-word `input_d` salt (`ERS-007` Status: *"The 8-word salt of `input_d` and the salt word of the tape are two fields"*), so an `R_SL` executor/statement-builder consuming `rsl-call/1` cannot construct `RslStatement` (`ERS-010`).
    2. Conversely, `rsl-call/1` includes test-oracle fields `native_words` and `native_comm`; when `ERS-013` reuses `rsl-call/1` as the production prover input format and `ERS-014` rejects any record lacking a field, every non-suite user call (which has no native oracle run) is rejected by `ERS-014`!
- **Closing Fix**:
  - Make `native_words` and `native_comm` optional conformance-only fields in `rsl-call/1` (`ERS-001`, `ERS-014`), and add `prog_d` (from `sherlock plan`, `ERS-006`) plus the `input_d` salt source to the prover input contract.

### `Q-sherlock-execution-4`: `ERS-009` Status Claims Rejected `R_SL` Programs Keep a Provable `Rejected` Outcome on the General Route, Contradicting `EUI-001`, `EUI-004`, `EUI-005`, `ERS-008`, and `ERS-010`

- **IDs**: `EUI-001`, `EUI-004`, `EUI-005`, `ERS-008`, `ERS-009`, `ERS-010`
- **Quoted EARS Sentences**:
  - `EUI-001`: *"EUI-001: The Sherlock machine SHALL run each program under exactly one rule family, the interpreted family of 120 step rules or the straight-line family `R_SL`, that the `machine_version` inside `prog_d` names."*
  - `EUI-004`: *"EUI-004: When the `R_SL` executor finishes a call, the `R_SL` executor SHALL return either one outcome with `comm`, the effect words and the outputs, or one reject code."*
  - `EUI-005`: *"EUI-005: When an interpreted-family run ends, the Sherlock machine SHALL report exactly one of `Halted`, `Rejected` and `Exhausted`."*
  - `ERS-009` & Status: *"ERS-009: If the R_SL executor rejects a Compact call on the T23 settlement route, then the R_SL statement builder SHALL emit no `RslStatement` for the call."* / *"the same program keeps a provable Rejected outcome under the general route (EUI-005, GP-001; FAL-EXEC-4, LEAD PICK for T15)."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_se_4_rsl_has_no_provable_rejected_outcome` (`sherlock_execution.qnt:255-259`): under `EUI-001`, an `R_SL` program runs under `R_SL` alone (not the interpreted family of `EUI-005`). When an `R_SL` `Assert` or `Range` instruction fails, `EUI-004` returns **no outcome**, `ERS-008` and `ERS-010` define `RslStatement` and `outcome_d` **only for halted calls**, and the `ASSERT`/`RANGE` AIR constraints are unsatisfied. Hence **no** `R_SL` program has a provable `Rejected` outcome on *any* route.
- **Closing Fix**:
  - Either specify that only **interpreted-family** programs support provable `Rejected` and `Exhausted` outcomes (`GP-001`) while `R_SL` is a `Halted`-only straight-line circuit family (mirroring native ZKIR PLONK), or add a fault-absorbing `R_SL` mode where failed `Assert`/`Range` sets a fault latch and emits `outcome_d` of kind `Rejected`.

### `Q-sherlock-execution-5`: `EUI-009` Governs `UIREQ` Conflicts Only Against `processor-final`, Leaving Conflicts With `modular-leaf` (`CORE-003`, `CORE-012`) and `CPR-003`'s Missing `L_Fr` Key Unresolved

- **IDs**: `EUI-009`, `EUI-002`, `UIREQ-012`, `UIREQ-013`, `UIREQ-028`, `CORE-003`, `CORE-012`, `CPR-003`
- **Quoted EARS Sentences**:
  - `EUI-009`: *"EUI-009: Where a requirement of change `sherlock-universal-interpreter-design` conflicts with a named requirement of `sherlock-processor-final-20261111`, the Sherlock machine SHALL follow `sherlock-processor-final-20261111`."*
  - `CORE-003`: *"A step that does not fit the remaining capacity SHALL NOT be enabled and SHALL NOT produce `Rejected` or `Exhausted` (SVM-010)."*
  - `CORE-012`: *"The spike's one-fuel-per-step rule SHALL NOT be copied."*
  - `UIREQ-028` Scenario: *"The proposed schedule charges one per completed legacy machine transition and one per helper transition"*
  - `CPR-003`: *"One universal key set SHALL accept every program: the internal-recursive key, the `L` key and the wrap key."*
- **Witness State**:
  - `EUI-009` names only `sherlock-processor-final-20261111`, omitting `sherlock-modular-leaf-20261111` (`CORE-003` vs `UIREQ-012` capacity outcomes; `CORE-012` vs `UIREQ-013`/`028`'s three competing unchosen fuel schedules). In addition, adopted `CPR-003` omits the **`L_Fr` key** from the universal key set, whereas `EUI-002` Scenario (`line 23`) and `ERS-015` Status (`line 183`) enumerate `4` universal keys (`node`, `L`, `L_Fr`, `wrap`).
- **Closing Fix**:
  - Extend `EUI-009` so both `sherlock-processor-final-20261111` and `sherlock-modular-leaf-20261111` (`CORE`, `RFB`) supersede conflicting `UIREQ` clauses, publish the `UIREQ` retention map, and add `L_Fr` to `CPR-003`'s universal key list.

### `Q-sherlock-execution-6`: `EKT-009` (`secp256k1`) and `EKT-010` (`Jubjub`) Delegate Normative Word Layouts to Uncommitted or Non-Existent Interface Files

- **IDs**: `EKT-009`, `EKT-010`
- **Quoted EARS Sentences**:
  - `EKT-009`: *"EKT-009: The secp256k1 kit SHALL accept exactly the operations, type tags and word layouts of `SECP-INTERFACE.md` sections 2 to 4."* (Status: *"The interface file is in `~/.cache/rekey-chips/`, outside every repository; gap: commit it with T24."*)
  - `EKT-010`: *"EKT-010: The Jubjub kit SHALL accept exactly the operations and word layouts of one committed interface file that names `EcAdd`, `EcMul`, `EcMulGenerator`, `HashToCurve`, `FromCoordinates`, `IntoCoordinates` and `JubjubScalarFromNative`."* (Status: *"OPEN. `gir.rs:52` states 'no interface file exists' for `EcAdd`"*.)
- **Witness State**:
  - Neither `SECP-INTERFACE.md` nor the Jubjub interface file exists in any committed repository tree, leaving the shared kernel between `Compact Compilation` and `Sherlock Execution` (`DDD.md:62`) undefined in version-controlled specs.
- **Closing Fix**:
  - Inline or commit both `SECP-INTERFACE.md` and `JUBJUB-INTERFACE.md` (with exact opcode numbers, limb widths, endianness, and subgroup/order rejection preconditions) before `T15` / `T25`.

---

## 3. Mutant Table

All `27` guard mutants (`MUTANT_ID = 1..27`) were executed with `quint run sherlock_execution.qnt --main <mutant_module> --invariant <target_inv> --max-samples 1000 --max-steps 15 --seed 0x20261115`. Every mutant was killed (`exit code 1`, `[violation] Found an issue`), and every mutant was verified **non-vacuous** by confirming `reach_all_three_interpreted_outcomes` is still violated (`exit code 1`) on that mutant module.

| Mutant Module | `MUTANT_ID` | Invariant It Must Break | Exit Code Seen | Non-Vacuous (`reach_all_three_interpreted_outcomes`) |
|---|---:|---|---:|---|
| `sherlock_execution_m_eui001_wrong_family_progd` | `1` | `inv_EUI_001` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_eui002_split_universal_keys` | `2` | `inv_EUI_002` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_eui003_new_key_per_program` | `3` | `inv_EUI_003` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_eui004_outcome_on_reject` | `4` | `inv_EUI_004` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_eui005_step_after_terminal` | `5` | `inv_EUI_005` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_eui006_ignore_executor_diff` | `6` | `inv_EUI_006` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_eui007_prove_bad_tape` | `7` | `inv_EUI_007` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_eui008_dynamic_trace_height` | `8` | `inv_EUI_008` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_eui009_override_processor_final` | `9` | `inv_EUI_009` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_eui010_pass_incomplete_120_rules` | `10` | `inv_EUI_010` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ekt002_invalid_air_bucket` | `11` | `inv_EKT_002` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ekt003_admit_unknown_builtin` | `12` | `inv_EKT_003` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ekt004_admit_mixed_hash` | `13` | `inv_EKT_004` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ekt005_wrong_sponge_init` | `14` | `inv_EKT_005` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ekt007_accept_forged_kit_trace` | `15` | `inv_EKT_007` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ekt009_accept_noncanonical_secp` | `16` | `inv_EKT_009` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ekt010_accept_off_subgroup_jubjub` | `17` | `inv_EKT_010` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ekt011_accept_overflow_alu` | `18` | `inv_EKT_011` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ers002_send_record_to_unowned_host` | `19` | `inv_ERS_002` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ers003_leak_tape_in_public_rows` | `20` | `inv_ERS_003` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ers005_prove_native_mismatch` | `21` | `inv_ERS_005` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ers007_accept_swapped_tape` | `22` | `inv_ERS_007` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ers009_statement_for_rejected_call` | `23` | `inv_ERS_009` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ers012_accept_forged_outcome_bind` | `24` | `inv_ERS_012` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ers013_emit_debug_string_program` | `25` | `inv_ERS_013` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ers014_admit_or_panic_malformed` | `26` | `inv_ERS_014` | `1` | Yes (`exit 1`) |
| `sherlock_execution_m_ers015_accept_unrooted_app_key` | `27` | `inv_ERS_015` | `1` | Yes (`exit 1`) |

---

## 4. State Vocabulary for Replay

### 4.1 Top-Level State Variables (`var`) and ITF Keys

| Quint `var` | Exact ITF JSON Key in `t_{0..19}.itf.json` | Quint Type | Meaning |
|---|---|---|---|
| `exec` | `sherlock_execution_main::sherlock_execution::exec` | `ExecutionState` | Cohesive observable state of the Sherlock Execution bounded context. |
| `lastOp` | `sherlock_execution_main::sherlock_execution::lastOp` | `str` | Action name written by `init` (`"init"`) and every transition step to drive oracle replay. |

### 4.2 Sum Types and Nested Sub-Record Types

- `RuleFamily`: `FamInterpreted | FamRsl` (`EUI-001`).
- `InterpOutcome`: `InterpHalted | InterpRejected | InterpExhausted` (`EUI-005`).
- `HashProgramKind`: `HashNone | HashSha256Only | HashKeccakOnly | HashMixedBoth` (`EKT-002`, `EKT-004`).
- `FamilyKeyState` (`exec.family`):
  - `progFamily: RuleFamily`, `progdNamedFamily: RuleFamily`, `familyLeafAccepted: bool` (`EUI-001`).
  - `machineVersion: int`, `priorVersionKeysValid: bool` (`EUI-001`, `EKT-001`).
  - `universalKeyIdsUsed: Set[int]`, `universalCeremoniesAdded: int` (`EUI-002`, `EUI-003`, `CPR-003`).
  - `conflictWithProcessorFinal: bool`, `followedProcessorFinal: bool`, `uireqRetainedWhenUnmentioned: bool` (`EUI-009`).
  - `coveredInterpRuleCount: int`, `gp1GatePassed: bool` (`EUI-010`, `RFB-003`, `RFB-004`).
  - `rslRuleTableCount: int` (`EKT-001`), `appKeyInKeyRoot: bool`, `unrootedAppKeyAccepted: bool` (`ERS-015`).
  - `distinctImagesDistinctProgd: bool` (`ERS-006`), `coreRulesDValid: bool` (`CORE-006`).
- `InterpRunState` (`exec.interp`):
  - `executed: bool`, `outcomeKind: InterpOutcome`, `reportedOutcomeCount: int` (`EUI-005`).
  - `remainingFramesAtHalt: int` (`UIREQ-011`), `stepAfterTerminalOccurred: bool`, `leafCloseMutatedState: bool` (`CORE-003`).
  - `oneFuelPerStepCopied: bool` (`CORE-012`), `chunkPreservesSemantics: bool` (`UIREQ-016`), `cacheDeterministicBaseline: bool` (`UIREQ-035`).
- `RslExecState` (`exec.rslExec`):
  - `evaluated: bool`, `recordSchema: str`, `recordFieldCount: int`, `programIsInstructionArray: bool` (`ERS-001`, `ERS-013`).
  - `recordMalformed: bool`, `malformedRefusedNamed: bool`, `processPanicked: bool` (`ERS-014`).
  - `recordMovedToUnownedHost: bool` (`ERS-002`).
  - `tapeValid: bool`, `proofStartsCount: int` (`EUI-007`), `rslHasOutcome: bool`, `rslHasRejectCode: bool` (`EUI-004`).
  - `openvmMatchesMycroftRef: bool`, `suiteBlockedOnExecMismatch: bool` (`EUI-006`).
  - `openvmMatchesNative: bool`, `proofMadeOnNativeMismatch: bool` (`ERS-005`, `CF-003`).
  - `traceHeightTape1: int`, `traceHeightTape2: int`, `falseGuardWritesZeros: bool`, `runShapeKeptPrivate: bool` (`EUI-008`, `ZK-008`).
  - `effectWordCount: int`, `publicRowCount: int`, `tapeWordInPublicRows: bool` (`ERS-003`).
  - `boxACommW: int`, `boxBCommW: int` (`ERS-011`).
- `KitVerifyState` (`exec.kits`):
  - `evaluated: bool`, `hashKind: HashProgramKind`, `admittedProgram: bool`, `appAirCountWithFp1: int`, `shaInEmpty: bool` (`EKT-002`, `EKT-004`, `CF-005`).
  - `unknownChipPairSeen: bool`, `unknownChipPairBlocked: bool` (`EKT-003`).
  - `transientWordsN: int`, `spongeStartTriplet: (int, int, int)` (`EKT-005`).
  - `refCrateChanged: bool`, `kitSuiteReranBeforeSettle: bool` (`EKT-006`).
  - `kitTraceViolatesRelation: bool`, `appVerifierAccepted: bool` (`EKT-007`).
  - `secpScalarGeOrder: bool`, `jubjubPointOffSubgroup: bool`, `aluResultGeModulus: bool`, `badCryptoOrAluCallRejected: bool`, `traceBuiltOnAluOverflow: bool` (`EKT-009`, `EKT-010`, `EKT-011`).
- `StatementBindState` (`exec.stmt`):
  - `evaluated: bool`, `rslCallHalted: bool`, `statementEmitted: bool` (`ERS-009`).
  - `statementFieldCount: int`, `outcomeKindIsHalted: bool`, `outcomePayloadBindsCommAndW: bool`, `projectedPublicWordCount: int` (`ERS-008`, `ERS-010`).
  - `boundInputValD: int`, `consumedTapeValD: int`, `leafAcceptedInputBinding: bool` (`ERS-007`).
  - `traceOutcomeDigest: int`, `suppliedRecursionOutcomeDigest: int`, `recursionAcceptedBinding: bool` (`ERS-012`).
- Monotone top-level `ExecutionState` reachability flags:
  - `haltedInterpSeen: bool`, `rejectedInterpSeen: bool`, `exhaustedInterpSeen: bool`, `rslHaltedStmtSeen: bool`, `rslRefusedNoStmtSeen: bool`, `mixedHashRefusedSeen: bool`, `recursionForgeryRefusedSeen: bool`.

### 4.3 `lastOp` Values and Implemented Requirements

| `lastOp` Value | Action in `sherlock_execution.qnt` | Implemented EARS IDs |
|---|---|---|
| `"init"` | `init` | Initial execution context state (`EUI-001..010`, `EKT-001..007`, `EKT-009..011`, `ERS-001..003`, `ERS-005..015`) |
| `"evaluateFamilyKeysAndGovernance"` | `doEvaluateFamilyKeysAndGovernance` | `EUI-001`, `EUI-002`, `EUI-003`, `EUI-009`, `EUI-010`, `EKT-001`, `ERS-006`, `ERS-015`, `CPR-003`, `CORE-006`, `RFB-003`, `RFB-004` |
| `"runInterpretedProgram"` | `doRunInterpretedProgram` | `EUI-005`, `CORE-003`, `CORE-012`, `UIREQ-001..012`, `UIREQ-016`, `UIREQ-035` |
| `"executeRslCallRecord"` | `doExecuteRslCallRecord` | `EUI-004`, `EUI-006`, `EUI-007`, `EUI-008`, `ERS-001`, `ERS-002`, `ERS-003`, `ERS-005`, `ERS-011`, `ERS-013`, `ERS-014`, `CF-003`, `ZK-008` |
| `"evaluateRslKitsAndAlu"` | `doEvaluateRslKitsAndAlu` | `EKT-002`, `EKT-003`, `EKT-004`, `EKT-005`, `EKT-006`, `EKT-007`, `EKT-009`, `EKT-010`, `EKT-011`, `CF-005` |
| `"buildAndBindRslStatement"` | `doBuildAndBindRslStatement` | `ERS-007`, `ERS-008`, `ERS-009`, `ERS-010`, `ERS-012` |

### 4.4 Constants of Concrete Instances

| Constant | `sherlock_execution_main` | `sherlock_execution_edge_min` | `sherlock_execution_edge_at_limit` | Mutant Modules (`m_*`) |
|---|---:|---:|---:|---:|
| `INTERPRETED_RULE_COUNT` | `120` | `120` | `120` | `120` |
| `RSL_RULE_COUNT` | `16` | `16` | `16` | `16` |
| `RSL_NO_HASH_AIRS_WITH_FP1` | `6` | `6` | `6` | `6` |
| `RSL_SINGLE_HASH_AIRS_WITH_FP1` | `9` | `9` | `9` | `9` |
| `CORE_SLOT_COUNT_S` | `16` | `16` | `16` | `16` |
| `RSL_CALL_FIELD_COUNT` | `15` | `15` | `15` | `15` |
| `RSL_STATEMENT_FIELD_COUNT` | `5` | `5` | `5` | `5` |
| `STATEMENT_PUBLICS_WORD_COUNT` | `4` | `4` | `4` | `4` |
| `JUBJUB_OP_COUNT` | `7` | `7` | `7` | `7` |
| `FP1_PUBLIC_WORDS` | `54` | `54` | `54` | `54` |
| `POSEIDON_SPONGE_WIDTH` | `3` | `3` | `3` | `3` |
| `MAX_RUN_FUEL_OR_EFFECTS` | `81` | `1` | `54` | `81` |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 27` |

---

## 5. Model Abstractions

1. **Digests, Keys, and Transcript Vectors as Finite Integers**: `prog_d` (`computeProgd`), `input_d` / `val_d` (`301` vs `999` on tape swap), `outcome_d` (`401` vs `888` on recursion outcome forgery), cross-box `(comm, W)` summaries (`501`), and universal key-set identities (`Set(1)` vs `Set(1, 2)`) are modeled as collision-free finite integers.
2. **Elliptic Curve, Hash, and ALU Micro-Op Semantics via Validity Predicates**: SHA-256, Keccak-256, Poseidon width-3 sponge, `secp256k1` scalar range (`< n`), `Jubjub` prime-order subgroup checks, and exact modular ALU bounds are abstracted into explicit input-validity flags (`kitTraceViolatesRelation`, `secpScalarGeOrder`, `jubjubPointOffSubgroup`, `aluResultGeModulus`) paired with verifier/executor acceptance guards and dedicated mutants.
3. **Poseidon Sponge State Initialization**: `EKT-005`'s width-3 sponge initialization `[0, 0, n]` is modeled directly as a 3-tuple `(0, 0, transientWordsN)` checked against `(1, 0, n)` domain-tag/state corruption.
4. **Effect Vector Scaling Across `FP-1` Capacity (`54` vs `81` Words)**: `MAX_RUN_FUEL_OR_EFFECTS` is instantiated at `1` (`edge_min`), `54` (`edge_at_limit`, exact `FP-1` word count), and `81` (`main`, matching `a01` `test_all` in `ERS-012` Verification) to exercise digest-based recursion binding above `54` words without allocating full BabyBear limb arrays.

---

## 6. Commands

Run all commands below from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/sherlock-execution/quint`:

```bash
# 1. Typecheck base specification + instances + mutants (expected exit: 0; printed: empty)
quint typecheck sherlock_execution.qnt
# Exit: 0 | Output: (none)

# 2. Typecheck deterministic scenario test module (expected exit: 0; printed: empty)
quint typecheck sherlock_execution_test.qnt
# Exit: 0 | Output: (none)

# 3. Run deterministic scenario tests (expected exit: 0)
quint test --main=sherlock_execution_test --match='se_.*Test' --seed=0x20261115 sherlock_execution_test.qnt
# Exit: 0 | Printed: 5 passing (1128ms)

# 4. Honest simulation: main instance (expected exit: 0)
quint run sherlock_execution.qnt --main sherlock_execution_main --invariant inv --max-samples 2000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (2526ms at 792 traces/second).

# 5. Honest simulation: minimum-effect/fuel edge instance (expected exit: 0)
quint run sherlock_execution.qnt --main sherlock_execution_edge_min --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (1245ms at 803 traces/second).

# 6. Honest simulation: FP-1-boundary (54 words) edge instance (expected exit: 0)
quint run sherlock_execution.qnt --main sherlock_execution_edge_at_limit --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (1261ms at 793 traces/second).

# 7. Reachability witnesses on sherlock_execution_main (all 5 MUST violate -> expected exit: 1)
quint run sherlock_execution.qnt --main sherlock_execution_main --invariant reach_all_three_interpreted_outcomes --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (157ms at 6 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_main --invariant reach_rsl_halted_statement_and_rejected_suppression --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (234ms at 9 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_main --invariant reach_mixed_hash_and_recursion_forgery_refusals --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (222ms at 5 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_main --invariant reach_valid_rsl_hash_kit_and_call_execution --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (259ms at 46 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_main --invariant reach_universal_key_both_families_and_gp1_gate --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (202ms at 15 traces/second).

# 8. Guard mutants (all 27 MUST violate their target invariant -> expected exit: 1)
quint run sherlock_execution.qnt --main sherlock_execution_m_eui001_wrong_family_progd --invariant inv_EUI_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (156ms at 13 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_eui002_split_universal_keys --invariant inv_EUI_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (184ms at 11 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_eui003_new_key_per_program --invariant inv_EUI_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (185ms at 11 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_eui004_outcome_on_reject --invariant inv_EUI_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (245ms at 4 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_eui005_step_after_terminal --invariant inv_EUI_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (410ms at 2 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_eui006_ignore_executor_diff --invariant inv_EUI_006 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (401ms at 2 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_eui007_prove_bad_tape --invariant inv_EUI_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (348ms at 9 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_eui008_dynamic_trace_height --invariant inv_EUI_008 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (210ms at 5 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_eui009_override_processor_final --invariant inv_EUI_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (449ms at 4 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_eui010_pass_incomplete_120_rules --invariant inv_EUI_010 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (213ms at 9 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ekt002_invalid_air_bucket --invariant inv_EKT_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (350ms at 9 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ekt003_admit_unknown_builtin --invariant inv_EKT_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (732ms at 4 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ekt004_admit_mixed_hash --invariant inv_EKT_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (682ms at 1 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ekt005_wrong_sponge_init --invariant inv_EKT_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (231ms at 4 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ekt007_accept_forged_kit_trace --invariant inv_EKT_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (243ms at 4 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ekt009_accept_noncanonical_secp --invariant inv_EKT_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (695ms at 1 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ekt010_accept_off_subgroup_jubjub --invariant inv_EKT_010 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (153ms at 20 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ekt011_accept_overflow_alu --invariant inv_EKT_011 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (184ms at 16 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ers002_send_record_to_unowned_host --invariant inv_ERS_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (200ms at 5 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ers003_leak_tape_in_public_rows --invariant inv_ERS_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (232ms at 4 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ers005_prove_native_mismatch --invariant inv_ERS_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (240ms at 4 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ers007_accept_swapped_tape --invariant inv_ERS_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (164ms at 6 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ers009_statement_for_rejected_call --invariant inv_ERS_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (197ms at 5 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ers012_accept_forged_outcome_bind --invariant inv_ERS_012 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (156ms at 6 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ers013_emit_debug_string_program --invariant inv_ERS_013 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (467ms at 2 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ers014_admit_or_panic_malformed --invariant inv_ERS_014 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (272ms at 4 traces/second).

quint run sherlock_execution.qnt --main sherlock_execution_m_ers015_accept_unrooted_app_key --invariant inv_ERS_015 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (296ms at 7 traces/second).

# 9. Generate 20 ITF replay traces (expected exit: 0)
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/sherlock-execution/traces/sherlock_execution_main
quint run sherlock_execution.qnt --main sherlock_execution_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/sherlock-execution/traces/sherlock_execution_main/t_{seq}.itf.json'
# Exit: 0 | Printed: [ok] No violation found (1757ms at 11 traces/second).
```
