# Quint Formal Specification Report — Bounded Context: Conformance (`e2e-conformance`)

- **Specification**: [conformance.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/conformance/quint/conformance.qnt)
- **Scenario Tests**: [conformance_test.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/conformance/quint/conformance_test.qnt)
- **ITF Replay Traces (`20` traces)**: [traces/conformance_main/](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/conformance/traces/conformance_main) (`t_0.itf.json` .. `t_19.itf.json`)

---

## 1. Requirements Coverage Table

Every own EARS ID (`14` IDs listed in `PROMPT.md`: `ECS-001` .. `ECS-014`; `0` completion IDs and `0` adopted IDs assigned to this context) appears below, followed by `ECS-015` (`sherlock-e2e-compact-suite/spec.md:185-197`).

| ID | Artifact (`inv_*`, action + test, consistency `val`) | Kind | Note |
|---|---|---|---|
| `ECS-001` | `inv_ECS_001`, `consistency_constants`, `doBuildManifestAndNativeRefs`, `cf_manifestNativeReferencesAndUniqueCallIdentityTest` | CONSTANT-CHECKED | Enforces schema `"t18-compact-suite-manifest/1"`, `MANIFEST_TOTAL_PROGRAMS == 36`, `COMPILED_SUITE_PROGRAMS == 33`, and `callsForUncompiledPrograms == 0` (no calls generated for programs refused by stock `compactc`). Killed by `conformance_m_ecs001_calls_for_uncompiled`. |
| `ECS-002` | `inv_ECS_002`, `gap_q_conf_3_ecs002_preimage_required_vs_ecs009_missing_62_tapes`, `doBuildManifestAndNativeRefs`, `cf_manifestNativeReferencesAndUniqueCallIdentityTest` | MODELED | Enforces schema `"mycroft-native-ref/1"` and `not(rejectedRefHasTranscriptWords)` on natively rejected calls. Killed by `conformance_m_ecs002_transcript_on_rejected_ref`. Missing preimages on the 62 rejected rows reported in `Q-conformance-3`. |
| `ECS-003` | `inv_ECS_003`, `gap_q_conf_1_compile_mismatch_exceeds_post_proof_denominator`, `doEvaluateSuiteCall`, `cf_sixCallClassesStageOrderingAndRefusalEvidenceLabelsTest` | MODELED | Enforces stage order `compile -> prove -> settle -> compare` and halts before `prove` with `ClsMismatch` if compile `W` differs from native. Killed by `conformance_m_ecs003_prove_after_compile_mismatch`. Denominator contradiction with `ECS-012` reported in `Q-conformance-1`. |
| `ECS-004` | `inv_ECS_004`, `consistency_constants`, `doEvaluateSuiteCall`, `cf_sixCallClassesStageOrderingAndRefusalEvidenceLabelsTest` | CONSTANT-CHECKED | Assigns each call `assignedClassCount == 1` of the `CALL_CLASSES_COUNT == 6` classes (`pass`, `refused-as-native`, `mismatch`, `blocked`, `prove-failed`, `n/a`) and maps pure `h02` to `ClsNa`. Killed by `conformance_m_ecs004_multi_or_bad_pure_class`. |
| `ECS-005` | `inv_ECS_005`, `doEvaluateSuiteCall`, `cf_sixCallClassesStageOrderingAndRefusalEvidenceLabelsTest` | MODELED | Requires every `ClsBlocked` row to carry a non-empty `blockedReason` text (e.g., `"no native Preview transcript"`, `"dispatch off"`, `"preview off"`). Killed by `conformance_m_ecs005_blocked_empty_reason`. |
| `ECS-006` | `inv_ECS_006`, `doEvaluateSuiteCall`, `cf_dispatchPreviewSwitchesKitReasonsAndDrySettleBlockedTest` | MODELED | Ensures calls whose kit (`jubjub`, `secp256k1`, `keccak256`) is held by the machine version are never classed as blocked for that kit (`machineHoldsKit implies not(blockedForEnabledKit)`). Killed by `conformance_m_ecs006_block_enabled_kit`. |
| `ECS-007` | `inv_ECS_007`, `doEvaluateSuiteCall`, `cf_dispatchPreviewSwitchesKitReasonsAndDrySettleBlockedTest` | MODELED | Sends `0` calls to a prover box while `T23_ALLOW_DISPATCH != 1` (`not(allowDispatch) implies not(dispatchedToProverBox)`). Killed by `conformance_m_ecs007_dispatch_without_switch`. |
| `ECS-008` | `inv_ECS_008`, `doEvaluateSuiteCall`, `cf_dispatchPreviewSwitchesKitReasonsAndDrySettleBlockedTest` | MODELED | Opens no wallet and submits `0` Preview transactions while `T23_ALLOW_PREVIEW != 1` (`not(allowPreview) implies (not(previewTxSubmitted) and not(walletOpened))`). Killed by `conformance_m_ecs008_preview_without_switch`. |
| `ECS-009` | `inv_ECS_009`, `doEvaluateSuiteCall`, `cf_sixCallClassesStageOrderingAndRefusalEvidenceLabelsTest` | MODELED | Labels `ClsRefusedAsNative` rows without an executed `R_SL` rejection as `EvDerived` (`DERIVED`), upgrading to `EvMeasured` (`MEASURED`) only when a tape exists and the `R_SL` executor rejects it. Killed by `conformance_m_ecs009_unexecuted_refusal_measured`. |
| `ECS-010` | `inv_ECS_010`, `gap_q_conf_4_ecs010_per_call_tx_vs_63_non_settling_calls`, `doWriteSuiteSummaryAndCheckCompletion`, `cf_suiteSummaryMismatchDenominatorLiveCompletionAndSpecGapsTest` | MODELED | Requires complete live-table fields and `mismatchCount == 0` whenever the `T23` live run is marked done. Killed by `conformance_m_ecs010_done_on_live_mismatch`. Per-call Preview tx/timing scope vs `63` non-settling rows reported in `Q-conformance-4`. |
| `ECS-011` | `inv_ECS_011`, `consistency_constants`, `doWriteSuiteSummaryAndCheckCompletion`, `cf_suiteSummaryMismatchDenominatorLiveCompletionAndSpecGapsTest` | CONSTANT-CHECKED | Enforces `33` compiled programs, `106` circuits (`105` proof + `1` pure `h02`), and `208` calls (`145 + 62 + 1 == 208`) summing identically across the suite table. Killed by `conformance_m_ecs011_inconsistent_suite_counts`. |
| `ECS-012` | `inv_ECS_012`, `gap_q_conf_1_compile_mismatch_exceeds_post_proof_denominator`, `doWriteSuiteSummaryAndCheckCompletion`, `cf_suiteSummaryMismatchDenominatorLiveCompletionAndSpecGapsTest` | MODELED | Reports `provedComparedCount` alongside `mismatchCount` (`denominatorReportedWithMismatch`, e.g., `0 of 0` on dry run). Killed by `conformance_m_ecs012_omit_mismatch_denominator`. |
| `ECS-013` | `inv_ECS_013`, `doBuildManifestAndNativeRefs`, `cf_manifestNativeReferencesAndUniqueCallIdentityTest` | MODELED | Enforces unique `(program, circuit, label, index)` call identities (`duplicateIdentityCount == 0`) and complete vector-to-call mapping (`unmappedExecutedVectorCount == 0`). Killed by `conformance_m_ecs013_duplicate_or_unmapped_id`. |
| `ECS-014` | `inv_ECS_014`, `consistency_constants`, `doWriteSuiteSummaryAndCheckCompletion`, `cf_suiteSummaryMismatchDenominatorLiveCompletionAndSpecGapsTest` | CONSTANT-CHECKED | Blocks `liveRunMarkedComplete` unless all `SUITE_ELIGIBLE_PROOF_CALLS == 145` eligible calls hold complete verification/Preview/readback/comparison evidence (`eligibleMissingEvidenceCount == 0`, `mismatchCount == 0`, `not(emptyVectorComparedEqual)`). Killed by `conformance_m_ecs014_complete_missing_evidence`. |

### Unlisted Spec ID Modeled in `conformance.qnt`

| ID | Source | Artifact | Kind | Note |
|---|---|---|---|---|
| `ECS-015` | `sherlock-e2e-prototype-20261115` (`sherlock-e2e-compact-suite/spec.md:185-197`) | `inv_ECS_015`, `gap_q_conf_2_ecs015_unconditional_masks_dry_mismatch`, `doEvaluateSuiteCall`, `cf_dispatchPreviewSwitchesKitReasonsAndDrySettleBlockedTest` | MODELED | Classifies calls settled without a Preview transaction (`settledWithoutPreviewTx`) as `ClsBlocked` with reason `"preview off"` rather than `ClsPass`. Killed by `conformance_m_ecs015_pass_on_dry_settle`. Interaction with dry-settle mismatches and omission from `PROMPT.md` reported in `Q-conformance-2`. |

---

## 2. Findings

### `Q-conformance-1`: Stage-1 Compile Mismatch (`ECS-003` Scenario) Produces `n > m` (`"Mismatch: 1 of 0 compared after a proof"`) Against `ECS-012`'s Denominator

- **IDs**: `ECS-003`, `ECS-012`
- **Quoted EARS Sentences**:
  - `ECS-003`: *"ECS-003: When the T23 runner runs a call, the T23 runner SHALL run the stages compile, prove, settle and compare in that order."*
  - `ECS-003` Scenario (*Compile stage differs*): *"WHEN the compile stage gives W that differs from the native reference / THEN the call is a mismatch and the prove stage does not run"*
  - `ECS-012`: *"ECS-012: When the T23 runner reports the mismatch count, the T23 runner SHALL report with it the count of calls that reached the compare stage after a proof."* (Verification: *"the suite table line 'Mismatch: n of m compared after a proof'"*.)
- **Witness Predicate / State**:
  - Witnessed by `gap_q_conf_1_compile_mismatch_exceeds_post_proof_denominator` (`conformance.qnt:142-147`): when `1` call diverges in `W` at stage `1` (`compile`) and stops without running `prove` (`ECS-003` Scenario) while `0` calls complete `prove` and reach `compare`, the total mismatch count is `n = 1` while the count of calls that reached `compare` *after a proof* is `m = 0`, producing `"Mismatch: 1 of 0 compared after a proof"` (`totalMismatchesN > comparedAfterProofM`). Moreover, `ECS-003`'s normative sentence lists `compare` only as stage `4` (after `settle`), omitting the stage-`1` pre-proof comparison required by its own Scenario.
- **Closing Fix**:
  - Either split `ECS-012` into two explicit counters (`compile_mismatch: n_c of m_c compiled` and `post_proof_mismatch: n_p of m_p compared after a proof`), or define `ECS-012`'s denominator as *"calls compared at either compile or post-proof compare"*, and state the pre-proof `W`/`comm` check explicitly in `ECS-003`.

### `Q-conformance-2`: Unconditional `ECS-015` Overwrites a Proved Transcript `mismatch` as `blocked ("preview off")` When Preview Is Disabled (Plus `ECS-015` Coverage Omission)

- **IDs**: `ECS-015`, `ECS-004`, `ECS-010`, `ECS-012`
- **Quoted EARS Sentences**:
  - `ECS-015`: *"ECS-015: If the T23 runner settled a call without a Preview transaction, then the T23 runner SHALL class the call as blocked with the reason `preview off`."*
  - `ECS-004`: *"ECS-004: The T23 runner SHALL give each call exactly one of the classes pass, refused-as-native, mismatch, blocked, prove-failed and n/a."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_conf_2_ecs015_unconditional_masks_dry_mismatch` (`conformance.qnt:153-158`): with `T23_ALLOW_DISPATCH = 1` and `T23_ALLOW_PREVIEW = 0`, a call runs `prove`, dry-`settle`, and `compare` (`ECS-015` Scenario). If `compare` detects that the proved call's transcript/public words differ from the native reference (`postProofTranscriptDiffers == true`), literal `ECS-015`'s unconditional `If the T23 runner settled a call without a Preview transaction...` forces the call's single `ECS-004` class to `blocked ("preview off")` instead of `mismatch`, masking a genuine proving soundness/conformance failure! In addition, `ECS-015` (present in `sherlock-e2e-compact-suite/spec.md:185-197`) was absent from `PROMPT.md`'s own-ID list (`ECS-001..014`).
- **Closing Fix**:
  - Narrow `ECS-015`'s trigger to calls that **matched the native reference**: *"If a call proved and matched the native reference, and the T23 runner settled the call without a Preview transaction, then the T23 runner SHALL class the call as blocked with the reason `preview off` (whereas a native-reference difference SHALL remain `mismatch`)."* Add `ECS-015` to the formal coverage manifest.

### `Q-conformance-3`: `ECS-002` Requires Storing the Preimage for Every Suite Call, Contradicting `ECS-009`'s Absence of Preimages on All `62` Natively Rejected Calls (and Unspecified Halt-on-Rejected Mismatch)

- **IDs**: `ECS-002`, `ECS-004`, `ECS-009`
- **Quoted EARS Sentences**:
  - `ECS-002`: *"ECS-002: When the native driver runs a suite call, the native driver SHALL store the transcript, the preimage and one reference record of schema `mycroft-native-ref/1` under `native/<id>/transcripts/`."*
  - `ECS-002` Scenario (*Rejected native call*): *"WHEN the native circuit rejects a call / THEN the reference record holds the rejection and no transcript words"*
  - `ECS-009` & Status: *"ECS-009: While a refused-as-native row has no executed `R_SL` rejection, the T23 runner SHALL label the row DERIVED in the suite table."* / *"the 62 rows are DERIVED, not run, because the suite holds no native preimage or tape for a rejected call (`t23.py:398-457`)."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_conf_3_ecs002_preimage_required_vs_ecs009_missing_62_tapes` (`conformance.qnt:163-167`): `ECS-002` requires storing the preimage for every suite call (its Scenario exempts only *transcript words* on native rejection, not the input preimage), yet none of the `62` rejected calls stored a preimage. Moreover, `ECS-009` Scenario defines what happens when a tape exists for a natively rejected call and the `R_SL` executor *rejects* it (`MEASURED`), but neither `ECS-009` nor `ECS-004` normatively states that if the `R_SL` executor **halts (accepts)** a natively rejected call, the call MUST be classified as `mismatch`!
- **Closing Fix**:
  - Enforce `ECS-002` in `ref_vectors.py` so input preimages are saved before native execution (even when native execution throws a rejection), and add an explicit `ECS-009` clause: *"If a natively rejected call halts in the `R_SL` executor, then the T23 runner SHALL class the call as `mismatch`."*

### `Q-conformance-4`: `ECS-010` Requires Per-Call Preview Transaction and Prove/Wrap Timing Fields Across All `208` Rows, Including `63` Rows (`62` `refused-as-native` + `1` `n/a`) That Never Prove or Settle

- **IDs**: `ECS-010`, `ECS-011`, `ECS-014`
- **Quoted EARS Sentences**:
  - `ECS-010`: *"ECS-010: When the T23 live run ends, the T23 runner SHALL write one suite table with, per call, the class, the Preview transaction hash and block, the comparison with the native transcript, and the prove and wrap time per box."*
  - `ECS-011`: *"ECS-011: The T23 runner SHALL report 33 programs, 106 circuits and 208 calls, where the 106 circuits include the pure circuit of h02."*
  - `ECS-014`: *"ECS-014: When the T23 runner reports the live run complete, the T23 runner SHALL hold for each eligible call of the pinned manifest the proof and key-set identity, the stock and ledger 9.1.0.1 verification results, the included Preview transaction and block, the read-back and the native comparison."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_conf_4_ecs010_per_call_tx_vs_63_non_settling_calls` (`conformance.qnt:172-175`): `63` of the `208` suite calls (`62` `refused-as-native` + `1` pure `h02` `n/a`) never produce a proof, wrap, Preview transaction, or native transcript comparison, so `ECS-010`'s unqualified *"per call, the class, the Preview transaction hash and block..."* cannot be populated with non-null transaction/timing values on those `63` rows. Conversely, `ECS-014` scopes live-run completion to *"each eligible call of the pinned manifest"* without normatively defining `eligible` (`145` non-pure natively-passing calls).
- **Closing Fix**:
  - Scope `ECS-010`'s Preview transaction, readback, and prove/wrap timing columns to **eligible proof calls** (with explicit `null`/`n/a` sentinels on `refused-as-native` and `n/a` rows), and define `eligible call` in `ECS-014` as *"every call of a non-pure circuit in the pinned manifest whose `mycroft-native-ref/1` record succeeded (`145` calls)"*.

---

## 3. Mutant Table

All `15` guard mutants (`MUTANT_ID = 1..15`, one 1-to-1 mutant for each of `ECS-001` .. `ECS-015`) were executed with `quint run conformance.qnt --main <mutant_module> --invariant <target_inv> --max-samples 1000 --max-steps 15 --seed 0x20261115`. Every mutant was killed (`exit code 1`, `[violation] Found an issue`), and every mutant was verified **non-vacuous** by confirming `reach_live_eligible_call_passes` is still violated (`exit code 1`) on that mutant module.

| Mutant Module | `MUTANT_ID` | Invariant It Must Break | Exit Code Seen | Non-Vacuous (`reach_live_eligible_call_passes`) |
|---|---:|---|---:|---|
| `conformance_m_ecs001_calls_for_uncompiled` | `1` | `inv_ECS_001` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs002_transcript_on_rejected_ref` | `2` | `inv_ECS_002` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs003_prove_after_compile_mismatch` | `3` | `inv_ECS_003` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs004_multi_or_bad_pure_class` | `4` | `inv_ECS_004` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs005_blocked_empty_reason` | `5` | `inv_ECS_005` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs006_block_enabled_kit` | `6` | `inv_ECS_006` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs007_dispatch_without_switch` | `7` | `inv_ECS_007` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs008_preview_without_switch` | `8` | `inv_ECS_008` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs009_unexecuted_refusal_measured` | `9` | `inv_ECS_009` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs010_done_on_live_mismatch` | `10` | `inv_ECS_010` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs011_inconsistent_suite_counts` | `11` | `inv_ECS_011` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs012_omit_mismatch_denominator` | `12` | `inv_ECS_012` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs013_duplicate_or_unmapped_id` | `13` | `inv_ECS_013` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs014_complete_missing_evidence` | `14` | `inv_ECS_014` | `1` | Yes (`exit 1`) |
| `conformance_m_ecs015_pass_on_dry_settle` | `15` | `inv_ECS_015` | `1` | Yes (`exit 1`) |

---

## 4. State Vocabulary for Replay

### 4.1 Top-Level State Variables (`var`) and ITF Keys

| Quint `var` | Exact ITF JSON Key in `t_{0..19}.itf.json` | Quint Type | Meaning |
|---|---|---|---|
| `conf` | `conformance_main::conformance::conf` | `ConformanceState` | Cohesive observable state of the Conformance bounded context. |
| `lastOp` | `conformance_main::conformance::lastOp` | `str` | Action label set by `init` (`"init"`) and every transition step to drive oracle replay. |

### 4.2 Sum Types and Nested Record Types

- `CallResultClass`: `ClsPass | ClsRefusedAsNative | ClsMismatch | ClsBlocked | ClsProveFailed | ClsNa` (`ECS-004`).
- `EvidenceLabel`: `EvDerived | EvMeasured | EvNotApplicable` (`ECS-009`).
- `SuiteCallScenario`: `ScenEligibleLivePass | ScenEligibleCompileMismatch | ScenEligibleProveFailed | ScenEligibleNoNativePreviewRef | ScenEligibleKitCall | ScenNativeRejectedDerived | ScenNativeRejectedExecuted | ScenPureH02`.
- `ManifestRefState` (`conf.manifest`):
  - `evaluated: bool` — True once manifest and native reference setup ran.
  - `manifestSchema: str` — Schema string (`"t18-compact-suite-manifest/1"`, `ECS-001`).
  - `manifestProgramCount: int` (`36`), `compiledProgramCount: int` (`33`), `callsForUncompiledPrograms: int` (`0`, `ECS-001`).
  - `nativeRefSchema: str` (`"mycroft-native-ref/1"`), `rejectedRefHasTranscriptWords: bool` (`false`, `ECS-002`).
  - `duplicateIdentityCount: int` (`0`), `unmappedExecutedVectorCount: int` (`0`, `ECS-013`).
- `CallRunState` (`conf.callRun`):
  - `evaluated: bool`, `scenario: SuiteCallScenario`.
  - `allowDispatch: bool` (`T23_ALLOW_DISPATCH == 1`, `ECS-007`), `allowPreview: bool` (`T23_ALLOW_PREVIEW == 1`, `ECS-008`), `machineHoldsKit: bool` (`ECS-006`).
  - `compileRan: bool`, `proveRan: bool`, `settleRan: bool`, `compareRan: bool`, `stageOrderValid: bool`, `compileMatchedNative: bool` (`ECS-003`).
  - `dispatchedToProverBox: bool` (`ECS-007`), `previewTxSubmitted: bool`, `walletOpened: bool` (`ECS-008`), `settledWithoutPreviewTx: bool` (`ECS-015`).
  - `assignedClassCount: int` (`1`), `resultClass: CallResultClass` (`ECS-004`), `blockedReason: str` (`ECS-005`), `blockedForEnabledKit: bool` (`ECS-006`).
  - `hasExecutedRslRejection: bool`, `evidenceKind: EvidenceLabel` (`ECS-009`).
- `SuiteSummaryState` (`conf.summary`):
  - `evaluated: bool`, `reportedPrograms: int` (`33`), `reportedCircuits: int` (`106`), `reportedCalls: int` (`208`), `tableRowSum: int` (`208`, `ECS-011`).
  - `provedComparedCount: int`, `mismatchCount: int`, `denominatorReportedWithMismatch: bool` (`ECS-012`).
  - `eligibleMissingEvidenceCount: int`, `emptyVectorComparedEqual: bool`, `liveTableFieldsPresent: bool`, `liveRunMarkedComplete: bool` (`ECS-010`, `ECS-014`).
- Monotone top-level `ConformanceState` reachability flags:
  - `livePassSeen: bool`, `dryPreviewBlockedSeen: bool`, `compileMismatchEarlyStopSeen: bool`, `derivedAndMeasuredRefusalsSeen: bool`, `liveRunCompleteSeen: bool`.

### 4.3 `lastOp` Values and Implemented Requirements

| `lastOp` Value | Action in `conformance.qnt` | Implemented EARS IDs |
|---|---|---|
| `"init"` | `init` | Initial suite conformance state (`ECS-001..015`) |
| `"buildManifestAndNativeRefs"` | `doBuildManifestAndNativeRefs` | `ECS-001`, `ECS-002`, `ECS-013` |
| `"evaluateSuiteCall"` | `doEvaluateSuiteCall` | `ECS-003`, `ECS-004`, `ECS-005`, `ECS-006`, `ECS-007`, `ECS-008`, `ECS-009`, `ECS-015` |
| `"writeSuiteSummaryAndCheckCompletion"` | `doWriteSuiteSummaryAndCheckCompletion` | `ECS-010`, `ECS-011`, `ECS-012`, `ECS-014` |

### 4.4 Constants of Concrete Instances

| Constant | `conformance_main` | `conformance_edge_min` | `conformance_edge_at_limit` | Mutant Modules (`m_*`) |
|---|---:|---:|---:|---:|
| `MANIFEST_TOTAL_PROGRAMS` | `36` | `36` | `36` | `36` |
| `COMPILED_SUITE_PROGRAMS` | `33` | `33` | `33` | `33` |
| `SUITE_TOTAL_CIRCUITS` | `106` | `106` | `106` | `106` |
| `SUITE_PURE_CIRCUITS` | `1` | `1` | `1` | `1` |
| `SUITE_TOTAL_CALLS` | `208` | `208` | `208` | `208` |
| `SUITE_ELIGIBLE_PROOF_CALLS` | `145` | `145` | `145` | `145` |
| `SUITE_NATIVE_REJECTED_CALLS` | `62` | `62` | `62` | `62` |
| `CALL_CLASSES_COUNT` | `6` | `6` | `6` | `6` |
| `ACTIVE_BATCH_SIZE` | `4` | `1` | `6` | `4` |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 15` |

---

## 5. Model Abstractions

1. **Eight Call Archetypes Spanning All `208` Suite Rows**: Rather than storing a 208-entry map in every ITF state, `SuiteCallScenario` represents the 8 distinct call equivalence classes (`ScenEligibleLivePass`, `ScenEligibleCompileMismatch`, `ScenEligibleProveFailed`, `ScenEligibleNoNativePreviewRef`, `ScenEligibleKitCall`, `ScenNativeRejectedDerived`, `ScenNativeRejectedExecuted`, `ScenPureH02`), while `SuiteSummaryState` enforces the exact corpus partition `145 + 62 + 1 == 208` across `33` of `36` manifest programs and `106` circuits (`ECS-001`, `ECS-011`).
2. **Evidence Completeness as Missing-Item Cardinality**: `ECS-014`'s per-call evidence bundle (proof + key-set ID, stock + `9.1.0.1` verification, Preview tx + block, readback, and non-empty native vector comparison) is modeled via `eligibleMissingEvidenceCount` (`0`, `1`, `4`, `6`, `145`) and `emptyVectorComparedEqual: bool`.
3. **Call Identity Uniqueness via Violation Counters**: `ECS-013`'s `(program, circuit, label, index)` bijection across recorded scenarios and executed vectors is abstracted into `duplicateIdentityCount` and `unmappedExecutedVectorCount`.

---

## 6. Commands

Run all commands below from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/conformance/quint`:

```bash
# 1. Typecheck base specification + instances + mutants (expected exit: 0; printed: empty)
quint typecheck conformance.qnt
# Exit: 0 | Output: (none)

# 2. Typecheck deterministic scenario test module (expected exit: 0; printed: empty)
quint typecheck conformance_test.qnt
# Exit: 0 | Output: (none)

# 3. Run deterministic scenario tests (expected exit: 0)
quint test --main=conformance_test --match='cf_.*Test' --seed=0x20261115 conformance_test.qnt
# Exit: 0 | Printed: 4 passing (1243ms)

# 4. Honest simulation: main instance (expected exit: 0)
quint run conformance.qnt --main conformance_main --invariant inv --max-samples 2000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (6274ms at 319 traces/second).

# 5. Honest simulation: single-missing-item edge instance (expected exit: 0)
quint run conformance.qnt --main conformance_edge_min --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (1675ms at 597 traces/second).

# 6. Honest simulation: six-class-batch edge instance (expected exit: 0)
quint run conformance.qnt --main conformance_edge_at_limit --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (1091ms at 917 traces/second).

# 7. Reachability witnesses on conformance_main (all 4 MUST violate -> expected exit: 1)
quint run conformance.qnt --main conformance_main --invariant reach_live_eligible_call_passes --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (208ms at 14 traces/second).

quint run conformance.qnt --main conformance_main --invariant reach_dispatch_on_preview_off_blocked --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (513ms at 18 traces/second).

quint run conformance.qnt --main conformance_main --invariant reach_compile_mismatch_stop_and_measured_refusal --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (534ms at 7 traces/second).

quint run conformance.qnt --main conformance_main --invariant reach_full_145_live_run_complete --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (244ms at 4 traces/second).

# 8. Guard mutants (all 15 MUST violate their target invariant -> expected exit: 1)
quint run conformance.qnt --main conformance_m_ecs001_calls_for_uncompiled --invariant inv_ECS_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (203ms at 5 traces/second).

quint run conformance.qnt --main conformance_m_ecs002_transcript_on_rejected_ref --invariant inv_ECS_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (219ms at 5 traces/second).

quint run conformance.qnt --main conformance_m_ecs003_prove_after_compile_mismatch --invariant inv_ECS_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (158ms at 25 traces/second).

quint run conformance.qnt --main conformance_m_ecs004_multi_or_bad_pure_class --invariant inv_ECS_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (335ms at 3 traces/second).

quint run conformance.qnt --main conformance_m_ecs005_blocked_empty_reason --invariant inv_ECS_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (130ms at 8 traces/second).

quint run conformance.qnt --main conformance_m_ecs006_block_enabled_kit --invariant inv_ECS_006 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (173ms at 69 traces/second).

quint run conformance.qnt --main conformance_m_ecs007_dispatch_without_switch --invariant inv_ECS_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (186ms at 11 traces/second).

quint run conformance.qnt --main conformance_m_ecs008_preview_without_switch --invariant inv_ECS_008 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (237ms at 59 traces/second).

quint run conformance.qnt --main conformance_m_ecs009_unexecuted_refusal_measured --invariant inv_ECS_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (262ms at 8 traces/second).

quint run conformance.qnt --main conformance_m_ecs010_done_on_live_mismatch --invariant inv_ECS_010 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (122ms at 16 traces/second).

quint run conformance.qnt --main conformance_m_ecs011_inconsistent_suite_counts --invariant inv_ECS_011 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (272ms at 4 traces/second).

quint run conformance.qnt --main conformance_m_ecs012_omit_mismatch_denominator --invariant inv_ECS_012 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (131ms at 8 traces/second).

quint run conformance.qnt --main conformance_m_ecs013_duplicate_or_unmapped_id --invariant inv_ECS_013 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (160ms at 6 traces/second).

quint run conformance.qnt --main conformance_m_ecs014_complete_missing_evidence --invariant inv_ECS_014 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (157ms at 13 traces/second).

quint run conformance.qnt --main conformance_m_ecs015_pass_on_dry_settle --invariant inv_ECS_015 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (297ms at 47 traces/second).

# 9. Generate 20 ITF replay traces (expected exit: 0)
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/conformance/traces/conformance_main
quint run conformance.qnt --main conformance_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/conformance/traces/conformance_main/t_{seq}.itf.json'
# Exit: 0 | Printed: [ok] No violation found (532ms at 38 traces/second).
```
