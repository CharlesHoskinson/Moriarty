# Quint Formal Specification Report — Bounded Context: Call Flow (`e2e-call-flow`)

- **Specification**: [call_flow.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/call-flow/quint/call_flow.qnt)
- **Scenario Tests**: [call_flow_test.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/call-flow/quint/call_flow_test.qnt)
- **ITF Replay Traces (`20` traces)**: [traces/call_flow_main/](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/call-flow/traces/call_flow_main) (`t_0.itf.json` .. `t_19.itf.json`)

---

## 1. Requirements Coverage Table

Every own EARS ID (`12` IDs from `sherlock-e2e-prototype-20261115`: `EUF-001`, `EUF-002`, `EUF-005..014`; note `EUF-003` and `EUF-004` do not exist) and completion EARS ID (`2` IDs from `sherlock-completion-20261115`: `LPC-014`, `LPC-017`; `0` adopted IDs) assigned to this lane appears below (`14` rows).

| ID | Artifact (`inv_*`, action + test, consistency `val`) | Kind | Note |
|---|---|---|---|
| `EUF-001` | `inv_EUF_001`, `EUF001_STAGES`, `consistency_constants`, `gap_q_cf_1_euf001_vs_design_dep_table_mismatch`, `doRunEndToEndCallPipeline`, `cfl_firstCallSecondCallFromTapeAndReadbackTest` | CONSTANT-CHECKED | Enforces `EUF001_STAGE_COUNT == 11` and canonical stage ordering (`stagesInCanonicalOrder`). Killed by `call_flow_m_euf001_out_of_order_stage`. Stage-graph/dataflow contradictions against `design.md` and `EUF-002` reported in `Q-call-flow-1` and `Q-call-flow-2`. |
| `EUF-002` | `inv_EUF_002`, `consistency_constants`, `gap_q_cf_1_euf001_vs_design_dep_table_mismatch`, `gap_q_cf_2_first_call_11_stages_vs_second_call_7_stages`, `doRunEndToEndCallPipeline`, `cfl_firstCallSecondCallFromTapeAndReadbackTest`, `cfl_preSubmitStageStopsAndNativeCompactRejectionTest` | CONSTANT-CHECKED | Enforces stage input preconditions (`stageInputsSatisfied`, `missingArtifactStoppedCleanly` when `wrap` lacks `LfrProofReady`) and `deploymentsSentThisCall == 0` on second calls (`SECOND_CALL_STAGE_COUNT == 7`). Killed by `call_flow_m_euf002_missing_input_or_redeploy`. |
| `EUF-005` | `inv_EUF_005`, `doRunEndToEndCallPipeline`, `cfl_preSubmitStageStopsAndNativeCompactRejectionTest` | MODELED | Sends `0` transactions (`txSentCount == 0`, `not(submitStageStarted)`) and records the failing stage and error (`recordedStopStageAndError`) whenever any pre-submit stage fails. Killed by `call_flow_m_euf005_submit_after_failed_stage`. |
| `EUF-006` | `inv_EUF_006`, `doRunEndToEndCallPipeline`, `cfl_preSubmitStageStopsAndNativeCompactRejectionTest` | MODELED | Starts neither `prove` (`not(proveStageStarted)`) nor `submit` (`not(submitStageStarted)`, `txSentCount == 0`) when the `execute` stage rejects a Compact call. Killed by `call_flow_m_euf006_prove_rejected_compact_call`. |
| `EUF-007` | `inv_EUF_007`, `doCheckProfilePinsClaimAndMultiDeploy`, `cfl_boxProfileVersionPinsCrossBoxClaimAndSharedWrapVkTest` | MODELED | Refuses the `prove` stage on any box lacking a `sherlock-profile/1` matching the installed universal key set (`not(boxHasMatchingProfile) implies not(proveAllowedOnBox)`). Killed by `call_flow_m_euf007_prove_without_box_profile`. |
| `EUF-008` | `inv_EUF_008`, `doCheckProfilePinsClaimAndMultiDeploy`, `cfl_boxProfileVersionPinsCrossBoxClaimAndSharedWrapVkTest` | MODELED | Enforces byte-equal `claim_pub` across distinct user-owned boxes proving the same call (`claimPubBoxA == claimPubBoxB`). Killed by `call_flow_m_euf008_box_dependent_claim_pub`. |
| `EUF-009` | `inv_EUF_009`, `doCheckProfilePinsClaimAndMultiDeploy`, `cfl_boxProfileVersionPinsCrossBoxClaimAndSharedWrapVkTest` | MODELED | Requires a second deployed program (`a05`) to install the exact same wrap VK as the first (`firstProgWrapVk == secondProgWrapVk`) while differing in `C_P` (`progAnchorsDiffer`). Killed by `call_flow_m_euf009_second_prog_different_vk`. |
| `EUF-010` | `inv_EUF_010`, `gap_q_cf_3_euf010_requires_absent_native_transcript_on_user_call`, `doRunEndToEndCallPipeline`, `cfl_firstCallSecondCallFromTapeAndReadbackTest` | MODELED | Verifies `runs[claim_pub] == outcome_pub` and `W == native_words` upon settled read-back (`readbackVerifiedOk`). Killed by `call_flow_m_euf010_accept_mismatched_readback`. Non-existence of native transcripts on external user calls reported in `Q-call-flow-3`. |
| `EUF-011` | `inv_EUF_011`, `consistency_constants`, `doRunEndToEndCallPipeline`, `cfl_firstCallSecondCallFromTapeAndReadbackTest` | CONSTANT-CHECKED | Enforces `maintainerToolsUsed <= MAINTAINER_TOOLS_ALLOWED (0)` (no DoctorWatson, herdr, box bus, or agent tool on a clean user box). Killed by `call_flow_m_euf011_require_maintainer_tool`. |
| `EUF-012` | `inv_EUF_012`, `consistency_constants`, `doCheckProfilePinsClaimAndMultiDeploy`, `cfl_boxProfileVersionPinsCrossBoxClaimAndSharedWrapVkTest` | CONSTANT-CHECKED | Halts and names the differing pin (`stoppedAndNamedPin and not(continuedPastPinMismatch)`) across all `PIN_DIMENSIONS_COUNT == 5` pins (`compactc`, `ZKIR`, `machine_version`, `keys_d`, `ledger`). Killed by `call_flow_m_euf012_continue_on_pin_mismatch`. |
| `EUF-013` | `inv_EUF_013`, `consistency_constants`, `gap_q_cf_5_lpc014_owner_wait_vs_euf013_freeze_evidence`, `doEvaluatePrototypeFreezeAndDocs`, `cfl_reconcileUnreadSubmitPrototypeFreezeDocsAndSpecGapsTest` | CONSTANT-CHECKED | Requires all `ELIGIBLE_SUITE_CALLS == 145` eligible suite calls to complete under `universalKeySetsInEvidence == 1` before reporting the prototype complete. Killed by `call_flow_m_euf013_complete_partial_or_split_keys`. |
| `EUF-014` | `inv_EUF_014`, `gap_q_cf_4_euf014_blocks_retry_after_resolved_non_inclusion`, `doReconcileSubmittedCall`, `cfl_reconcileUnreadSubmitPrototypeFreezeDocsAndSpecGapsTest` | MODELED | Reconciles submitted calls lacking read-back by transaction hash (`reconciledByTxHash`) with `txCountForClaimPub == 1`. Killed by `call_flow_m_euf014_resubmit_unread_call`. Permanent retry deadlock after node refusal/TTL expiry reported in `Q-call-flow-4`. |
| `LPC-014` | `inv_LPC_014`, `consistency_constants`, `gap_q_cf_5_lpc014_owner_wait_vs_euf013_freeze_evidence`, `doEvaluatePrototypeFreezeAndDocs`, `cfl_reconcileUnreadSubmitPrototypeFreezeDocsAndSpecGapsTest` | CONSTANT-CHECKED | Blocks presenting the `T15` interface freeze (`freezePresented`) unless all `LOOP_ROW_COUNT` rows are `DONE` with a lead verdict or wait on the owner. Killed by `call_flow_m_lpc014_freeze_without_verdict`. Tension with `EUF-013` Scenario reported in `Q-call-flow-5`. |
| `LPC-017` | `inv_LPC_017`, `doEvaluatePrototypeFreezeAndDocs`, `cfl_reconcileUnreadSubmitPrototypeFreezeDocsAndSpecGapsTest` | MODELED | Requires `domLockHitsInDesignDocs == 0` in `FINAL-DESIGN.md` and `ARCHITECTURE.md` once the owner's `x_j` lock-form ruling holds. Killed by `call_flow_m_lpc017_keep_stale_dom_lock_docs`. Current repo violation noted in `Q-call-flow-5`. |

---

## 2. Findings

### `Q-call-flow-1`: `EUF-001`'s `11`-Stage Sequence Contradicts `design.md`'s `13`-Stage Dependency Table Bound by `EUF-002` (Stage Order Inversion, Missing `build arguments` Producer of `StatementPublics`, and Missing `compare` Stage)

- **IDs**: `EUF-001`, `EUF-002`, `EUF-010`
- **Quoted EARS Sentences**:
  - `EUF-001`: *"EUF-001: When a user runs one end-to-end call, the Sherlock toolchain SHALL run the stages in this order: compile, setup, plan, deploy, tape, execute, prove, wrap, ledger check, submit and read-back."*
  - `EUF-002`: *"EUF-002: The Sherlock toolchain SHALL give each stage only the artifacts and the private state references that the stage dependency table of `design.md` names for the stage."*
  - `design.md:56-70` (*Stage dependency table (EUF-002)*): Lists `13` rows in order `compile`, `plan and admit`, `deploy`, `setup`, `tape`, `execute`, `prove`, `wrap`, `ledger check`, `build arguments` (writes `StatementPublics`), `submit` (reads `StatementPublics`), `accept and read back` (reads `transaction hash; contract address`), `compare` (reads `SettlementReceipt; NativeCallReference; call identity`).
- **Witness Predicate / State**:
  - Witnessed by `gap_q_cf_1_euf001_vs_design_dep_table_mismatch` (`call_flow.qnt:147-156`):
    1. **Ordering inversion (`setup` vs `plan`/`deploy`)**: `EUF-001` places `setup` second (`compile -> setup -> plan -> deploy`), whereas `design.md:58-61` places `setup` fourth (`compile -> plan and admit -> deploy -> setup`), and `setup` is per-box (`1` run per box per universal key set, independent of per-program `compile/plan/deploy`).
    2. **Broken `StatementPublics` dataflow into `submit`**: `EUF-001` omits `build arguments` (`design.md:67`), which is the **only** stage in `design.md` that produces `StatementPublics`. Under `EUF-002`'s strict input rule, `submit` (`design.md:68`, which requires `StatementPublics`) has no producer in `EUF-001`'s 11-stage pipeline!
    3. **Forbidden `NativeCallReference` access in `read-back`**: `EUF-001` omits `compare` (`design.md:70`), yet `EUF-010` mandates comparing against the native transcript during `read-back`—an input (`NativeCallReference`) that `design.md:69` (`accept and read back`) does **not** grant under `EUF-002`!
- **Closing Fix**:
  - Unify `EUF-001` and `design.md:56-70` on one stage DAG with three explicit scopes: per-box (`setup`), per-program (`compile -> plan -> deploy`), and per-call (`tape -> execute -> prove -> wrap -> ledger check -> build arguments -> submit -> read-back [-> optional conformance compare]`).

### `Q-call-flow-2`: Unconditional `EUF-001` Requires Running All `11` Stages (Including `deploy`) on Every Call, Contradicting `EUF-002` Scenario *Second call*

- **IDs**: `EUF-001`, `EUF-002`
- **Quoted EARS Sentences**:
  - `EUF-001`: *"EUF-001: When a user runs one end-to-end call, the Sherlock toolchain SHALL run the stages in this order: compile, setup, plan, deploy, tape, execute, prove, wrap, ledger check, submit and read-back."*
  - `EUF-002` Scenario (*Second call*): *"WHEN a user calls a deployed program a second time / THEN the call starts at the tape stage and no deployment is sent"*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_cf_2_first_call_11_stages_vs_second_call_7_stages` (`call_flow.qnt:161-162`): `EUF-001` states unconditionally that running *"one end-to-end call"* runs all `11` stages (`compile` through `read-back`, sending a deployment), whereas `EUF-002` Scenario requires subsequent calls of an already-deployed program to run only the per-call tail (`tape` through `read-back`, `deploymentsSentThisCall == 0`). Also note `EUF-002` Verification (`spec.md:19`) lists *"no second compile, setup or deploy"*, inadvertently omitting `plan` even though `plan` is also per-program (`design.md:59`).
- **Closing Fix**:
  - Qualify `EUF-001` with *"On the first end-to-end call of an uncompiled program on an unprofiled box..."*, and include `plan` alongside `compile, setup, and deploy` in `EUF-002` Verification.

### `Q-call-flow-3`: Unconditional `EUF-010` Requires a Native Reference Transcript That Does Not Exist on Clean User Runs (`EUF-011`) or General Interpreted Runs (`EUF-006` Status)

- **IDs**: `EUF-010`, `EUF-011`, `EUF-006`
- **Quoted EARS Sentences**:
  - `EUF-010`: *"EUF-010: When a settled call is read back, the Sherlock toolchain SHALL compare `runs[claim_pub]` with `outcome_pub` and the outcome with the native transcript of the call."*
  - `EUF-011`: *"EUF-011: The Sherlock toolchain SHALL run the flow of EUF-001 with no DoctorWatson, herdr, box bus or agent tool on the user's box."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_cf_3_euf010_requires_absent_native_transcript_on_user_call` (`call_flow.qnt:167-171`): outside the `mycroft/tests/compact-suite/` conformance harness (e.g., an external developer settling a contract on Preview under `EUF-011`, or proving a general interpreted-family Sherlock program under `EUF-006` Status), no `mycroft-native-ref/1` native transcript exists (`hasNativeTranscriptRef == false`). Furthermore, on-chain indexer read-back returns only `runs[claim_pub] = outcome_pub` (a salted digest), not the raw effect vector `W`.
- **Closing Fix**:
  - Split `EUF-010` into: (1) an unconditional product read-back check (`runs[claim_pub] == outcome_pub`), and (2) a `Where a native reference transcript is provided` conformance check (`W == native_words`).

### `Q-call-flow-4`: `EUF-014`'s Unconditional Ban on a Second Transaction for `claim_pub` Deadlocks Retry After Node Refusal or Intent TTL Expiry (`EUF-008`)

- **IDs**: `EUF-014`, `EUF-008`
- **Quoted EARS Sentences**:
  - `EUF-014`: *"EUF-014: If a submitted call has no read-back, then the Sherlock toolchain SHALL reconcile the call by its transaction hash and send no second transaction for the same `claim_pub`."*
  - `EUF-014` Status: *"Three post-submit outcomes are kept apart: node refusal (a JSON-RPC error), unknown submission status, and included but unread."*
  - `EUF-008`: *"EUF-008: When two boxes that the user owns prove the same call, the Sherlock prover SHALL give byte-equal `claim_pub` values."*
- **Witness Predicate / State**:
  - Witnessed by `gap_q_cf_4_euf014_blocks_retry_after_resolved_non_inclusion` (`call_flow.qnt:177-182`): if a submitted transaction fails due to a transient fee/UTXO error (JSON-RPC node refusal) or expires its intent TTL without inclusion, it has no read-back (`not(hasReadback)`). Because `claim_pub` is deterministic for the prepared run (`EUF-008`), literal `EUF-014`'s permanent ban (*"send no second transaction for the same `claim_pub`"*) makes that prepared call permanently unsettleable even after reconciliation confirms the first transaction was never included on chain!
- **Closing Fix**:
  - Scope `EUF-014`'s no-second-transaction rule to **while the transaction's inclusion state is unknown or resolved as included**; once reconciliation confirms node refusal or TTL expiry (not-included), allow building a fresh balanced transaction for the unsettled `claim_pub`.

### `Q-call-flow-5`: `LPC-014`'s Per-Row `"or waits on the owner"` Disjunction Allows Presenting `T15` Before `EUF-013`'s Joined Suite Evidence Exists, and `LPC-017` Is Open in Design Docs

- **IDs**: `LPC-014`, `EUF-013`, `LPC-017`
- **Quoted EARS Sentences**:
  - `LPC-014`: *"LPC-014: When every row is DONE with a lead verdict or waits on the owner, the tested-design loop SHALL present the interface freeze to the owner."*
  - `EUF-013` Scenario (*Interface freeze*): *"WHEN the lead presents T15 / THEN the suite table, the gate logs and the Preview receipts name one universal key-set identity (the key manifest of UKS-004), and each row records the `prog_d` and `key_root` of its own program"*
  - `LPC-017`: *"LPC-017: When an owner ruling changes a formula, the tested-design loop SHALL update FINAL-DESIGN.md and ARCHITECTURE.md to the ruled form."* (Status: *"GAP: `FINAL-DESIGN.md` and `ARCHITECTURE.md` keep the `P2(DOM_LOCK)` form."*)
- **Witness Predicate / State**:
  - Witnessed by `gap_q_cf_5_lpc014_owner_wait_vs_euf013_freeze_evidence` (`call_flow.qnt:188-194`): if row `T23` or `T25` is marked *"waits on the owner"*, `LPC-014`'s antecedent holds (`canPresentT15ByLpc014 == true`) even when `joinedSuiteCallsCompleted == 0 < 145`, contradicting `EUF-013` Scenario's precondition on presenting `T15`. Meanwhile `LPC-017` Status confirms `DOM_LOCK` still appears in `FINAL-DESIGN.md` and `ARCHITECTURE.md` (`domLockHitsInDesignDocs > 0` in mutant `call_flow_m_lpc017_keep_stale_dom_lock_docs`).
- **Closing Fix**:
  - Restrict *"or waits on the owner"* in `LPC-014` strictly to post-`T15` owner-gated rows (`T4` Mainnet, branch pushes), requiring technical rows (`T0`–`T14b`, `T16`–`T25`) and `LPC-017` doc updates to be `DONE` with lead verdicts before presenting `T15`.

---

## 3. Mutant Table

All `14` guard mutants (`MUTANT_ID = 1..14`, one 1-to-1 mutant for each of the `14` own + completion IDs) were executed with `quint run call_flow.qnt --main <mutant_module> --invariant <target_inv> --max-samples 1000 --max-steps 15 --seed 0x20261115`. Every mutant was killed (`exit code 1`, `[violation] Found an issue`), and every mutant was verified **non-vacuous** by confirming `reach_first_and_second_call_settle` is still violated (`exit code 1`) on that mutant module.

| Mutant Module | `MUTANT_ID` | Invariant It Must Break | Exit Code Seen | Non-Vacuous (`reach_first_and_second_call_settle`) |
|---|---:|---|---:|---|
| `call_flow_m_euf001_out_of_order_stage` | `1` | `inv_EUF_001` | `1` | Yes (`exit 1`) |
| `call_flow_m_euf002_missing_input_or_redeploy` | `2` | `inv_EUF_002` | `1` | Yes (`exit 1`) |
| `call_flow_m_euf005_submit_after_failed_stage` | `3` | `inv_EUF_005` | `1` | Yes (`exit 1`) |
| `call_flow_m_euf006_prove_rejected_compact_call` | `4` | `inv_EUF_006` | `1` | Yes (`exit 1`) |
| `call_flow_m_euf007_prove_without_box_profile` | `5` | `inv_EUF_007` | `1` | Yes (`exit 1`) |
| `call_flow_m_euf008_box_dependent_claim_pub` | `6` | `inv_EUF_008` | `1` | Yes (`exit 1`) |
| `call_flow_m_euf009_second_prog_different_vk` | `7` | `inv_EUF_009` | `1` | Yes (`exit 1`) |
| `call_flow_m_euf010_accept_mismatched_readback` | `8` | `inv_EUF_010` | `1` | Yes (`exit 1`) |
| `call_flow_m_euf011_require_maintainer_tool` | `9` | `inv_EUF_011` | `1` | Yes (`exit 1`) |
| `call_flow_m_euf012_continue_on_pin_mismatch` | `10` | `inv_EUF_012` | `1` | Yes (`exit 1`) |
| `call_flow_m_euf013_complete_partial_or_split_keys` | `11` | `inv_EUF_013` | `1` | Yes (`exit 1`) |
| `call_flow_m_euf014_resubmit_unread_call` | `12` | `inv_EUF_014` | `1` | Yes (`exit 1`) |
| `call_flow_m_lpc014_freeze_without_verdict` | `13` | `inv_LPC_014` | `1` | Yes (`exit 1`) |
| `call_flow_m_lpc017_keep_stale_dom_lock_docs` | `14` | `inv_LPC_017` | `1` | Yes (`exit 1`) |

---

## 4. State Vocabulary for Replay

### 4.1 Top-Level State Variables (`var`) and ITF Keys

| Quint `var` | Exact ITF JSON Key in `t_{0..19}.itf.json` | Quint Type | Meaning |
|---|---|---|---|
| `flow` | `call_flow_main::call_flow::flow` | `CallFlowState` | Cohesive observable state of the Call Flow cross-boundary process. |
| `lastOp` | `call_flow_main::call_flow::lastOp` | `str` | Action label set by `init` (`"init"`) and every transition step to drive oracle replay. |

### 4.2 Sum Types and Nested Record Types

- `FlowOutcomeCase`: `FlowFirstCallHappy | FlowSecondCallFromTape | FlowMissingWrapArtifactStop | FlowLedgerCheckRefusedStop | FlowCompactExecuteRejected | FlowReadbackMismatch`.
- `PipelineRunState` (`flow.pipeline`):
  - `evaluated: bool`, `flowCase: FlowOutcomeCase`, `isSecondCallOfDeployed: bool`.
  - `stagesInCanonicalOrder: bool` (`EUF-001`), `stageInputsSatisfied: bool`, `missingArtifactStoppedCleanly: bool`, `deploymentsSentThisCall: int` (`EUF-002`).
  - `executeRejectedCompact: bool` (`EUF-006`), `preSubmitFailed: bool`, `proveStageStarted: bool`, `submitStageStarted: bool`, `txSentCount: int`, `recordedStopStageAndError: bool` (`EUF-005`, `EUF-006`).
  - `readbackArrived: bool`, `readbackEqualsOutcomePub: bool`, `outcomeEqualsNativeWords: bool`, `readbackVerifiedOk: bool` (`EUF-010`).
  - `maintainerToolsUsed: int` (`0`, `EUF-011`).
- `PinsAndBoxesState` (`flow.pinsBoxes`):
  - `evaluated: bool`, `installedKeySetId: int` (`25`), `boxProfileKeySetId: int` (`25` vs `10`), `boxHasMatchingProfile: bool`, `proveAllowedOnBox: bool` (`EUF-007`).
  - `differingPinName: str`, `stoppedAndNamedPin: bool`, `continuedPastPinMismatch: bool` (`EUF-012`).
  - `claimPubBoxA: int`, `claimPubBoxB: int` (`701`, `EUF-008`).
  - `firstProgWrapVk: int`, `secondProgWrapVk: int` (`901`), `progAnchorsDiffer: bool` (`EUF-009`).
- `ReconcileState` (`flow.reconcile`):
  - `evaluated: bool`, `submittedHasNoReadback: bool`, `reconciledByTxHash: bool`, `txCountForClaimPub: int` (`1`, `EUF-014`).
- `FreezeAndDocsState` (`flow.completion`):
  - `evaluated: bool`, `joinedEligibleCallsCount: int` (`145`), `universalKeySetsInEvidence: int` (`1`), `prototypeReportedComplete: bool` (`EUF-013`).
  - `totalRows: int`, `readyOrOwnerWaitingRows: int`, `freezePresented: bool` (`LPC-014`).
  - `ownerRulingChangedLockForm: bool`, `domLockHitsInDesignDocs: int` (`0`, `LPC-017`).
- Monotone top-level `CallFlowState` reachability flags:
  - `firstCallSettledSeen: bool`, `secondCallFromTapeSeen: bool`, `preSubmitAndExecRejectStopsSeen: bool`, `reconciledUnreadSubmitSeen: bool`, `prototypeAndFreezeCompleteSeen: bool`.

### 4.3 `lastOp` Values and Implemented Requirements

| `lastOp` Value | Action in `call_flow.qnt` | Implemented EARS IDs |
|---|---|---|
| `"init"` | `init` | Initial call-flow process state (`EUF-001..002`, `EUF-005..014`, `LPC-014`, `LPC-017`) |
| `"runEndToEndCallPipeline"` | `doRunEndToEndCallPipeline` | `EUF-001`, `EUF-002`, `EUF-005`, `EUF-006`, `EUF-010`, `EUF-011` |
| `"checkProfilePinsClaimAndMultiDeploy"` | `doCheckProfilePinsClaimAndMultiDeploy` | `EUF-007`, `EUF-008`, `EUF-009`, `EUF-012` |
| `"reconcileSubmittedCall"` | `doReconcileSubmittedCall` | `EUF-014` |
| `"evaluatePrototypeFreezeAndDocs"` | `doEvaluatePrototypeFreezeAndDocs` | `EUF-013`, `LPC-014`, `LPC-017` |

### 4.4 Constants of Concrete Instances

| Constant | `call_flow_main` | `call_flow_edge_min` | `call_flow_edge_at_limit` | Mutant Modules (`m_*`) |
|---|---:|---:|---:|---:|
| `EUF001_STAGE_COUNT` | `11` | `11` | `11` | `11` |
| `SECOND_CALL_STAGE_COUNT` | `7` | `7` | `7` | `7` |
| `PIN_DIMENSIONS_COUNT` | `5` | `5` | `5` | `5` |
| `ELIGIBLE_SUITE_CALLS` | `145` | `145` | `145` | `145` |
| `MAINTAINER_TOOLS_ALLOWED` | `0` | `0` | `0` | `0` |
| `LOOP_ROW_COUNT` | `5` | `1` | `25` | `5` |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 14` |

---

## 5. Model Abstractions

1. **Cross-Context Process Modeled by Trajectory Archetypes (`FlowOutcomeCase`)**: Per `DDD.md` Section 5 (*"The Call Flow context owns one process and no aggregate"*), the end-to-end call flow is modeled through its 6 canonical execution/failure outcomes (`FlowFirstCallHappy`, `FlowSecondCallFromTape`, `FlowMissingWrapArtifactStop`, `FlowLedgerCheckRefusedStop`, `FlowCompactExecuteRejected`, `FlowReadbackMismatch`) and explicit stage-ordering/input-satisfaction predicates.
2. **Version Pins, Key-Set Identities, `claim_pub`, and Wrap VKs as Finite Identifiers**: Key-set IDs (`25` vs `10`), `claim_pub` digests (`701` vs `799`), wrap VKs (`901` vs `902`), and pin dimension names (`"none"`, `"zkir"`, `"keys_d"`, `"ledger"`) are represented by small finite integers and symbolic strings.
3. **Suite and Loop Readiness Counts**: `EUF-013`'s 145-call joined completion and `LPC-014`'s loop-row readiness (`1`, `5`, or all `25` LOOP rows) are tracked by exact integer counts against `ELIGIBLE_SUITE_CALLS` and `LOOP_ROW_COUNT`.

---

## 6. Commands

Run all commands below from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/call-flow/quint`:

```bash
# 1. Typecheck base specification + instances + mutants (expected exit: 0; printed: empty)
quint typecheck call_flow.qnt
# Exit: 0 | Output: (none)

# 2. Typecheck deterministic scenario test module (expected exit: 0; printed: empty)
quint typecheck call_flow_test.qnt
# Exit: 0 | Output: (none)

# 3. Run deterministic scenario tests (expected exit: 0)
quint test --main=call_flow_test --match='cfl_.*Test' --seed=0x20261115 call_flow_test.qnt
# Exit: 0 | Printed: 4 passing (928ms)

# 4. Honest simulation: main instance (expected exit: 0)
quint run call_flow.qnt --main call_flow_main --invariant inv --max-samples 2000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (9108ms at 220 traces/second).

# 5. Honest simulation: single-row minimum edge instance (expected exit: 0)
quint run call_flow.qnt --main call_flow_edge_min --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (4212ms at 237 traces/second).

# 6. Honest simulation: 25-row LOOP table limit edge instance (expected exit: 0)
quint run call_flow.qnt --main call_flow_edge_at_limit --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (3051ms at 328 traces/second).

# 7. Reachability witnesses on call_flow_main (all 4 MUST violate -> expected exit: 1)
quint run call_flow.qnt --main call_flow_main --invariant reach_first_and_second_call_settle --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (169ms at 6 traces/second).

quint run call_flow.qnt --main call_flow_main --invariant reach_pre_submit_and_execute_reject_stops --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (516ms at 2 traces/second).

quint run call_flow.qnt --main call_flow_main --invariant reach_reconciled_unread_submission --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (215ms at 5 traces/second).

quint run call_flow.qnt --main call_flow_main --invariant reach_prototype_complete_and_freeze_presented --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (244ms at 4 traces/second).

# 8. Guard mutants (all 14 MUST violate their target invariant -> expected exit: 1)
quint run call_flow.qnt --main call_flow_m_euf001_out_of_order_stage --invariant inv_EUF_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (86ms at 12 traces/second).

quint run call_flow.qnt --main call_flow_m_euf002_missing_input_or_redeploy --invariant inv_EUF_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (217ms at 5 traces/second).

quint run call_flow.qnt --main call_flow_m_euf005_submit_after_failed_stage --invariant inv_EUF_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (272ms at 4 traces/second).

quint run call_flow.qnt --main call_flow_m_euf006_prove_rejected_compact_call --invariant inv_EUF_006 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (301ms at 17 traces/second).

quint run call_flow.qnt --main call_flow_m_euf007_prove_without_box_profile --invariant inv_EUF_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (251ms at 24 traces/second).

quint run call_flow.qnt --main call_flow_m_euf008_box_dependent_claim_pub --invariant inv_EUF_008 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (127ms at 16 traces/second).

quint run call_flow.qnt --main call_flow_m_euf009_second_prog_different_vk --invariant inv_EUF_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (88ms at 23 traces/second).

quint run call_flow.qnt --main call_flow_m_euf010_accept_mismatched_readback --invariant inv_EUF_010 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (209ms at 33 traces/second).

quint run call_flow.qnt --main call_flow_m_euf011_require_maintainer_tool --invariant inv_EUF_011 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (81ms at 12 traces/second).

quint run call_flow.qnt --main call_flow_m_euf012_continue_on_pin_mismatch --invariant inv_EUF_012 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (277ms at 7 traces/second).

quint run call_flow.qnt --main call_flow_m_euf013_complete_partial_or_split_keys --invariant inv_EUF_013 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (219ms at 5 traces/second).

quint run call_flow.qnt --main call_flow_m_euf014_resubmit_unread_call --invariant inv_EUF_014 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (214ms at 9 traces/second).

quint run call_flow.qnt --main call_flow_m_lpc014_freeze_without_verdict --invariant inv_LPC_014 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (270ms at 7 traces/second).

quint run call_flow.qnt --main call_flow_m_lpc017_keep_stale_dom_lock_docs --invariant inv_LPC_017 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (106ms at 9 traces/second).

# 9. Generate 20 ITF replay traces (expected exit: 0)
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/call-flow/traces/call_flow_main
quint run call_flow.qnt --main call_flow_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/call-flow/traces/call_flow_main/t_{seq}.itf.json'
# Exit: 0 | Printed: [ok] No violation found (235ms at 85 traces/second).
```
