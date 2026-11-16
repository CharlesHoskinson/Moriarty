# Quint Formal Specification Report — Bounded Context: Network Operations (`e2e-network-operations`)

- **Specification**: [network_operations.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/network-operations/quint/network_operations.qnt)
- **Scenario Tests**: [network_operations_test.qnt](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/network-operations/quint/network_operations_test.qnt)
- **ITF Replay Traces (`20` traces)**: [traces/network_operations_main/](file:///home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/network-operations/traces/network_operations_main) (`t_0.itf.json` .. `t_19.itf.json`)

---

## 1. Requirements Coverage Table

Every own EARS ID (`60` IDs from `sherlock-e2e-prototype-20261115`: `PVW-001..013`, `WAL-001..010`, `SCL-001..010`, `OBS-001..012`, `FRC-001..015`) and completion EARS ID (`6` IDs from `sherlock-completion-20261115`: `LPC-008`, `LPC-009`, `LPC-010`, `LPC-011`, `LPC-015`, `LPC-020`) assigned to this lane appears below (`66` rows), followed by adopted `SWF-009` and `FRC-016` (present in `sherlock-e2e-recovery/spec.md` lines 213–225).

| ID | Artifact (`inv_*`, action + test, consistency `val`) | Kind | Note |
|---|---|---|---|
| `PVW-001` | `inv_PVW_001`, `canStartWallet`, `doStartWalletAndBalanceBefore`, `doStopPreWalletOrConfig`, `no_preWalletGenesisTtlFolderVkPermissionsAndWalletPinStopsTest` | MODELED | Stops before starting the wallet if RPC genesis differs from `PINNED_PREVIEW_GENESIS` (`3096`). Killed by `network_operations_m_pvw001_wrong_genesis_wallet`. |
| `PVW-002` | `inv_PVW_002`, `isLegalTargetChain`, `doDeployOrRecoverContract`, `no_preWalletGenesisTtlFolderVkPermissionsAndWalletPinStopsTest` | MODELED | Restricts settle-runner transactions to `LOCAL_DEVNET_GENESIS` (`1001`) and `PINNED_PREVIEW_GENESIS` (`3096`). Killed by `network_operations_m_pvw002_send_to_mainnet`. Tension with `SWF-009` and `SCL-010` reported in `Q-network-operations-1`. |
| `PVW-003` | `inv_PVW_003`, `doDeployOrRecoverContract`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Requires Preview fee balancing to use chain ledger parameters rather than initial parameters. Killed by `network_operations_m_pvw003_initial_fee_params`. |
| `PVW-004` | `inv_PVW_004`, `consistency_constants`, `doRunCheckStep`, `no_previewHappyPathSevenStepsAndReceiptsTest` | CONSTANT-CHECKED | Binds Preview proof server to `127.0.0.1:6341` (`PREVIEW_PROOF_SERVER_PORT`) while leaving devnet port `6340` (`DEVNET_PROOF_SERVER_PORT`) untouched. Killed by `network_operations_m_pvw004_touch_port_6340`. |
| `PVW-005` | `PREVIEW_STEPS`, `consistency_constants`, `no_previewHappyPathSevenStepsAndReceiptsTest` | CONSTANT-CHECKED | Verifies the ordered 7-step command surface (`check`, `balance-before`, `deploy`, `call`, `refusals`, `balance-after`, `receipts`) and full-pass marker progression. |
| `PVW-006` | `inv_PVW_006`, `consistency_constants`, `doRunTamperControlsAndVerdict`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Submits all `TAMPER_CONTROL_COUNT == 4` controls (`R1`, `R2`, `R4`, `R3`) once honest call readback is present. Control `R3` error-code/simulation conflict reported in `Q-network-operations-2`. |
| `PVW-007` | `inv_PVW_007`, `doRunTamperControlsAndVerdict`, `no_tamperControlOutcomesAndExitCode4Test` | MODELED | Records `CtrlNoNodeVerdict` (`no-node-verdict`) and fails the verdict when a control ends without inclusion, timeout, or JSON-RPC error. Killed by `network_operations_m_pvw007_pass_no_node_verdict`. Exit-code ambiguity reported in `Q-network-operations-6`. |
| `PVW-008` | `inv_PVW_008`, `consistency_constants`, `gap_q_netops_3_timeout_vs_ttl_expiry_reuse`, `doRunTamperControlsAndVerdict`, `no_tamperControlOutcomesAndExitCode4Test` | MODELED | Records `CtrlNotIncludedTimeout` after `INBLOCK_TIMEOUT_S` (`600 s`) and refuses to treat timeout as a node refusal. Killed by `network_operations_m_pvw008_timeout_as_refusal`. Interaction with `3,600 s` TTL reported in `Q-network-operations-3`. |
| `PVW-009` | `inv_PVW_009`, `gap_q_netops_4_pvw009_vs_pvw013`, `doRecordT25KeyGate`, `doDeployAndProveFinalPrototype`, `no_t25GateDeployGpuBoxValidiumAndSwf009Test` | MODELED | Requires `t12GatePass` on `T25` keys before final Preview prototype deploy; contradiction with `PVW-013` when non-T12 gates are missing is witnessed by `gap_q_netops_4_pvw009_vs_pvw013` (`Q-network-operations-4`). |
| `PVW-010` | `inv_PVW_010`, `doDeployAndProveFinalPrototype`, `no_t25GateDeployGpuBoxValidiumAndSwf009Test` | MODELED | Requires final prototype `L_Fr` and wrap proves to execute on an owner GPU box (`finalCallProvedOnOwnedGpuBox`). Killed by `network_operations_m_pvw010_final_without_gpu_box`. |
| `PVW-011` | `inv_PVW_011`, `doSubmitValidiumBatch`, `no_t25GateDeployGpuBoxValidiumAndSwf009Test` | MODELED | Refuses validium batches whose program commitment differs from `validiumExpectedProgCP` or whose input root differs from `validiumStoredRoot`, preserving stored state. Killed by `network_operations_m_pvw011_accept_stale_validium`. |
| `PVW-012` | `inv_PVW_012`, `doRunTamperControlsAndVerdict`, `no_tamperControlOutcomesAndExitCode4Test` | MODELED | Fails the verdict with exit code `EXIT_VERDICT_FAIL` (`4`) whenever any tamper control is included in a block (`CtrlIncluded`). Killed by `network_operations_m_pvw012_pass_included_control`. |
| `PVW-013` | `inv_PVW_013`, `fullKeyGatePassed`, `canDeployFinalPrototype`, `doDeployAndProveFinalPrototype`, `no_t25GateDeployGpuBoxValidiumAndSwf009Test` | MODELED | Refuses final prototype deploy unless the wrap key gate record holds passes for `RKY-003`, `RKY-004`, `RKY-005`, `T12`, `ZK-012..017`, and `WRP-012`. Killed by `network_operations_m_pvw013_deploy_missing_gate`. |
| `WAL-001` | `inv_WAL_001`, `doStartWalletAndBalanceBefore`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Ensures the wallet recovery phrase is read inside the wallet process (`phraseReadInProcess`) and never exposed to shell arguments or environment (`not(phraseExposedToShellOrEnv)`). |
| `WAL-002` | `inv_WAL_002`, `doIncludeHonestCallAndScan`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Ensures no unscrubbed secret (`seed=`, `mnemonic=`, `xprv`, `secretKey`, or 12+ lowercase word phrase) reaches logs or artifacts. Killed by `network_operations_m_wal002_unscrubbed_secret`. |
| `WAL-003` | `inv_WAL_003`, `canStartWallet`, `doStartWalletAndBalanceBefore`, `no_preWalletGenesisTtlFolderVkPermissionsAndWalletPinStopsTest` | MODELED | Refuses wallet startup if `derivedWalletAddr != pinnedWalletAddr`. Killed by `network_operations_m_wal003_unpinned_wallet`. |
| `WAL-004` | `inv_WAL_004`, `doStartWalletAndBalanceBefore`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Enforces exclusive `wallet.lock` (`activeWalletSenders <= 1`) across wallet runners. Killed by `network_operations_m_wal004_concurrent_senders`. |
| `WAL-005` | `inv_WAL_005`, `doIncludeHonestCallAndScan`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Ensures `receipts.json` writes only allow-listed public fields (`not(nonAllowlistFieldInReceipts)`). Killed by `network_operations_m_wal005_private_in_receipts`. |
| `WAL-006` | `inv_WAL_006`, `doStartWalletAndBalanceBefore`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Ensures `balance-before.json` and `balance-after.json` record only public balances and dust parameters (`not(keyMaterialInBalanceFile)`). |
| `WAL-007` | `inv_WAL_007`, `doDeployOrRecoverContract`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Ensures prover boxes (`huginn`/`odin`) never hold the wallet seed or submit transactions (`submittedFromWalletHost`). Killed by `network_operations_m_wal007_submit_from_prover`. |
| `WAL-008` | `inv_WAL_008`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Asserts zero wallet keys or secret patterns in tracked git files (`secretsInRepoCount == 0`). |
| `WAL-009` | `inv_WAL_009`, `doAttemptWalletSetupOverwrite`, `no_preWalletGenesisTtlFolderVkPermissionsAndWalletPinStopsTest` | MODELED | Refuses to overwrite an existing phrase file during wallet setup (`not(existingWalletOverwritten)`). Killed by `network_operations_m_wal009_overwrite_wallet`. |
| `WAL-010` | `inv_WAL_010`, `consistency_constants`, `canStartWallet`, `doStartWalletAndBalanceBefore`, `no_preWalletGenesisTtlFolderVkPermissionsAndWalletPinStopsTest` | CONSTANT-CHECKED | Refuses to read the wallet phrase when directory mode exceeds `MAX_WALLET_DIR_MODE` (`700`) or phrase file mode exceeds `MAX_PHRASE_FILE_MODE` (`600`). Killed by `network_operations_m_wal010_wide_mode_read`. |
| `SCL-001` | `SDK_COMMANDS`, `consistency_constants` | CONSTANT-CHECKED | Verifies the 7-command settlement surface (`sherlock_plan`, `lfr_call`, `wrap`, `sherlock-wrap-ledger-check`, `accept_args.py`, `t10a-ledger-run.mjs`, `t17.sh`); CLI binary packaging has no separate state transition. |
| `SCL-002` | `inv_SCL_002`, `consistency_constants`, `doRunTamperControlsAndVerdict`, `doStopPreWalletOrConfig`, `no_tamperControlOutcomesAndExitCode4Test` | MODELED | Maps `VerdictPassed -> 0`, `VerdictFailed -> EXIT_VERDICT_FAIL (4)`, and `VerdictError -> 1`. Killed by `network_operations_m_scl002_wrong_exit_code`. Interaction with `PVW-007` reported in `Q-network-operations-6`. |
| `SCL-003` | `inv_SCL_003`, `doRunSuiteDryRun`, `no_preWalletGenesisTtlFolderVkPermissionsAndWalletPinStopsTest` | MODELED | Enforces `dryRunNetworkConnections == 0` and `dryRunTxSent == 0` during `dry-run` execution. Killed by `network_operations_m_scl003_dryrun_network`. |
| `SCL-004` | `inv_SCL_004`, `canAcceptStateFolder`, `doRunCheckStep`, `no_preWalletGenesisTtlFolderVkPermissionsAndWalletPinStopsTest` | MODELED | Binds each state folder to a single wrap VK (`folderRecordedVkId == cliVkId`). Killed by `network_operations_m_scl004_mismatched_vk_folder`. |
| `SCL-005` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Stdout `E5JSON` phase-line serialization format of the `wrap` CLI binary is an I/O formatting rule without state-machine transitions in Network Operations. |
| `SCL-006` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Building `SherlockLedger` from committed source with `compactc 0.36.0` is a deterministic build-toolchain rule with no runtime state transition. |
| `SCL-007` | `inv_SCL_007`, `consistency_constants`, `canAcceptTtl`, `doRunCheckStep`, `no_preWalletGenesisTtlFolderVkPermissionsAndWalletPinStopsTest` | CONSTANT-CHECKED | Enforces `0 < configuredTtlS < MAX_TTL_EXCLUSIVE_S` (`1,209,600 s`) before transaction construction. Killed by `network_operations_m_scl007_out_of_range_ttl`. |
| `SCL-008` | `SUITE_DRIVER_COMMANDS`, `consistency_constants` | CONSTANT-CHECKED | Verifies the 2-command CLI surface (`run`, `selftest`) of `t23.py`. |
| `SCL-009` | `doStopPreWalletOrConfig`, `no_preWalletGenesisTtlFolderVkPermissionsAndWalletPinStopsTest` | MODELED | Event-driven startup check stopping cleanly with `SHERLOCK_SDK_ROOT_unset` (`sdkRootCheckStoppedCleanly`) when `SHERLOCK_SDK_ROOT` is absent. |
| `SCL-010` | `inv_SCL_010` | MODELED | Tracks clean external user configuration (`outsideFactoryCleanConfigOk`); conflict between external chain-pin configurability and `PVW-002`'s hardcoded 2-chain ban is reported in `Q-network-operations-1`. |
| `OBS-001` | `inv_OBS_001`, `doIncludeHonestCallAndScan`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Ensures included-call receipts carry txhash, block height, block hash, timestamp, fee, and circuit bytes (`includedReceiptComplete`). |
| `OBS-002` | `inv_OBS_002`, `doRunTamperControlsAndVerdict`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Ensures refused-control receipts carry control name, error code, message, and elapsed ms (`refusedReceiptComplete`). |
| `OBS-003` | `inv_OBS_003`, `doIncludeHonestCallAndScan`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Fails privacy scan if any private witness, trace, or memory word appears on the ledger call surface (`not(forbiddenPrivateWordOnSurface)`). Killed by `network_operations_m_obs003_dead_walker_or_leak`. |
| `OBS-004` | `inv_OBS_004`, `doIncludeHonestCallAndScan`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Verifies positive control words (`claim_pub`, `outcome_pub`) fire in the privacy walker so a dead walker cannot pass (`controlWordsPresentOnSurface`). |
| `OBS-005` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Appending ISO-8601 `run`, `verdict`, `incident`, and `ruling` lines to `llm-wiki/log.md` is an operational wiki-logging rule without protocol state transitions. |
| `OBS-006` | `inv_OBS_006`, `doRunCheckStep`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Requires `manifest.sha256` to be written whenever the `check` step completes (`manifestSha256Written`). |
| `OBS-007` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Reproducing every numeric claim in `RESULTS*.md` from committed benchmark logs is a documentation/evidence audit rule rather than a runtime state invariant. |
| `OBS-008` | `inv_OBS_008`, `doRecordWalletBalanceFailure`, `no_recoverySavedCallBalanceRetriesDoubleGateFailPushHoldsAndSpecGapsTest` | MODELED | Requires wallet failures to record the full `cause` chain and stack (`walletFailureChainRecorded`). |
| `OBS-009` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | T23 summary-table formatting (suite, family, status, elapsed time, settled count, failure reason) is a report-rendering rule with no state transition. |
| `OBS-010` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | `box-worker status` reporting remote `systemd --user` unit state, active job, GPU load, and log tail is a read-only CLI telemetry projection. |
| `OBS-011` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Recording per-phase RSS and GPU memory peaks on `L_Fr` and wrap benchmarks is a benchmark measurement rule with no state transition here. |
| `OBS-012` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Emitting a JSON receipt per `box-worker` job with git SHA, binary SHA-256, input/output hashes, exit code, and wall time is an external job-runner serialization rule. |
| `FRC-001` | `inv_FRC_001`, `doRunCheckStep`, `doStartWalletAndBalanceBefore`, `doDeployOrRecoverContract`, `doIncludeHonestCallAndScan`, `doRunTamperControlsAndVerdict`, `no_tamperControlOutcomesAndExitCode4Test` | MODELED | Writes `<step>.ok` markers only when a step exits `0` (`failedStepsMarkedOk == Set()`). Killed by `network_operations_m_frc001_mark_failed_step_ok`. |
| `FRC-002` | `inv_FRC_002`, `doDeployOrRecoverContract`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Recovers an unconfirmed deploy address from the indexer instead of sending a second deploy (`deploySendCount <= 1`). Killed by `network_operations_m_frc002_double_deploy`. |
| `FRC-003` | `inv_FRC_003`, `consistency_constants`, `gap_q_netops_3_timeout_vs_ttl_expiry_reuse`, `canReuseSavedCall`, `doReuseSavedCall`, `no_recoverySavedCallBalanceRetriesDoubleGateFailPushHoldsAndSpecGapsTest` | CONSTANT-CHECKED | Reuses a saved proven unrefused unincluded call only while `ttlLeftS > MIN_REUSE_TTL_LEFT_S` (`300 s`). Killed by `network_operations_m_frc003_reuse_low_ttl`. Post-expiry reuse contradiction reported in `Q-network-operations-3`. |
| `FRC-004` | `inv_FRC_004`, `consistency_constants`, `doRecordWalletBalanceFailure`, `no_recoverySavedCallBalanceRetriesDoubleGateFailPushHoldsAndSpecGapsTest` | CONSTANT-CHECKED | Bounds transport-error wallet balancing retries to `1..MAX_BALANCE_TRANSPORT_TRIES` (`<= 4`) with resync before each retry. Killed by `network_operations_m_frc004_excess_or_bad_retry`. |
| `FRC-005` | `inv_FRC_005`, `canReuseSavedCall`, `doReuseSavedCall`, `no_recoverySavedCallBalanceRetriesDoubleGateFailPushHoldsAndSpecGapsTest` | MODELED | Marks node-refused honest calls as refused and forbids reusing them (`not(reusedRefusedCall)`). Killed by `network_operations_m_frc005_reuse_refused_call`. |
| `FRC-006` | `inv_FRC_006`, `doRunTamperControlsAndVerdict`, `no_previewHappyPathSevenStepsAndReceiptsTest` | MODELED | Refuses to start the `refusals` step unless `honestCallReadbackPresent` holds. Killed by `network_operations_m_frc006_refusals_no_readback`. |
| `FRC-007` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Launching runs planned above 30 minutes under `systemd-run --user` with append-mode logs is a host process-supervision rule with no protocol state transition. |
| `FRC-008` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Restarting interrupted `muninn` jobs as new user units after a host reboot is an operational runbook recovery procedure building on `FRC-001..006`. |
| `FRC-009` | `inv_FRC_009`, `consistency_constants`, `doRecordGateFailureOnRow`, `no_recoverySavedCallBalanceRetriesDoubleGateFailPushHoldsAndSpecGapsTest` | CONSTANT-CHECKED | Stops a loop row upon `MAX_CONSECUTIVE_GATE_FAILS == 2` consecutive gate failures while keeping other rows running (`rowStoppedOnDoubleFail and otherRowsStillRunning`). |
| `FRC-010` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Holding every file of a `DONE` row in a branch commit (verified via `git status --short` and patch round-trip) is a git worktree hygiene rule. |
| `FRC-011` | `inv_FRC_011`, `canPushBranch`, `doAdvancePushGates`, `doPushBranch`, `no_recoverySavedCallBalanceRetriesDoubleGateFailPushHoldsAndSpecGapsTest` | MODELED | Holds pushes to any branch other than `t10q` before `t15OwnerSigned`. Killed by `network_operations_m_frc011_push_before_t15`. Missing `FP-3` gate in normative EARS text reported in `Q-network-operations-5`. |
| `FRC-012` | `inv_FRC_012` | MODELED | Preserves settled-call skip invariant (`suiteSkippedSettledCallsOk`) when `T23` settle resumes at calls whose `runs[claim_pub] == outcome_pub`. |
| `FRC-013` | `inv_FRC_013`, `canReuseSavedCall`, `doAttemptResubmitClaim`, `no_recoverySavedCallBalanceRetriesDoubleGateFailPushHoldsAndSpecGapsTest` | MODELED | Blocks resubmitting or reusing a call while `inclusionUnknown` holds (`not(resubmittedWhileInclusionUnknown)`). Killed by `network_operations_m_frc013_resubmit_unknown`. |
| `FRC-014` | `inv_FRC_014`, `canReuseSavedCall`, `doReuseSavedCall`, `no_recoverySavedCallBalanceRetriesDoubleGateFailPushHoldsAndSpecGapsTest` | MODELED | Verifies SHA-256, deployed VK, and genesis hash of saved call/state files before reuse (`not(reusedCorruptOrMismatchedCall)`). Killed by `network_operations_m_frc014_reuse_corrupt_file`. |
| `FRC-015` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Recording `git merge-base --is-ancestor` and post-merge suite passes on integration branch `tested` is a repository release-engineering rule. |
| `LPC-008` | `inv_PVW_009`, `inv_PVW_010`, `inv_PVW_013`, `doRecordT25KeyGate`, `doDeployAndProveFinalPrototype`, `doRunTamperControlsAndVerdict`, `no_t25GateDeployGpuBoxValidiumAndSwf009Test` | MODELED | Models Preview prototype `(b)`/`(c)` gate completion, GPU-box proving (`huginn`), honest call, and tamper-control refusals. |
| `LPC-009` | `inv_PVW_011`, `doSubmitValidiumBatch`, `reach_validium_two_chained_batches_and_stale_refusal`, `no_t25GateDeployGpuBoxValidiumAndSwf009Test` | MODELED | Settles `>= 2` chained validium batches on Preview (`validiumAcceptedBatches >= 2`) and refuses invalid/stale-root batches with state unchanged. |
| `LPC-010` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Merging branch `t20-batched` (`5dc1be2ee`) into `tested` after `T10q` closes is a one-shot git branch-merge checklist item. |
| `LPC-011` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Running the `prover-gui` Playwright harness live against the `T14b` daemon and in replay mode belongs to frontend UI verification without Network Operations state transitions. |
| `LPC-015` | `inv_LPC_015`, `canPushBranch`, `doAdvancePushGates`, `doPushBranch`, `reach_fp3_t10q_push_and_balance_failure_stops`, `no_recoverySavedCallBalanceRetriesDoubleGateFailPushHoldsAndSpecGapsTest` | MODELED | Enforces `FP-3` gate before pushing `t10q` and `T15` owner sign-off before pushing any other branch; normative-vs-scenario discrepancy reported in `Q-network-operations-5`. |
| `LPC-020` | `SPEC-CHECKED candidate` | SPEC-CHECKED candidate | Synchronizing row names between `~/.cache/box-bus/all.log` and `LOOP.md` (`T24`, `T25`) is a project-tracking checklist rule. |

### Adopted and Unlisted Spec IDs Modeled in `network_operations.qnt`

| ID | Source | Artifact | Kind | Note |
|---|---|---|---|---|
| `SWF-009` | `sherlock-vm-consensus-20261107` (`sherlock-delivery/spec.md`) | `inv_SWF_009`, `canRunMainnetT4Op`, `doRunMainnetT4Operation`, `no_t25GateDeployGpuBoxValidiumAndSwf009Test` | MODELED | Blocks Mainnet `T4` operations unless all 4 amendment preconditions hold, explicit per-operation owner+checklist authorization is present, and zero automatic retries occur. Killed by `network_operations_m_swf009_unauth_mainnet`. |
| `FRC-016` | `sherlock-e2e-prototype-20261115` (`sherlock-e2e-recovery/spec.md:213-225`) | `inv_FRC_004`, `doRecordWalletBalanceFailure`, `no_recoverySavedCallBalanceRetriesDoubleGateFailPushHoldsAndSpecGapsTest` | MODELED | Stops wallet balancing after `1` attempt (`balanceAttemptCount == 1`) with no retry when the balancing error is non-transport (e.g., insufficient funds). Omission from `PROMPT.md`'s own-ID list noted in `Q-network-operations-6`. |

---

## 2. Findings

### `Q-network-operations-1`: Hardcoded Two-Chain Restriction (`PVW-002`) Contradicts External SDK Chain Configuration (`SCL-010`) and Authorized Mainnet `T4` Operations (`SWF-009` / `FRC-004`)

- **IDs**: `PVW-002`, `SCL-010`, `SWF-009`, `FRC-004`
- **Quoted EARS Sentences**:
  - `PVW-002`: *"PVW-002: The settle runner SHALL send a transaction to 2 chains only: the local devnet `undeployed1` and the pinned Preview genesis."*
  - `SCL-010`: *"SCL-010: Where a user runs the Sherlock SDK outside the factory, the Sherlock SDK SHALL take the wallet directory, the pinned wallet address, the chain pin and the box roster from the user's configuration."*
  - `SWF-009`: *"While any of its conditions is unmet (local Docker at 9.1.0.1 passed, Preview qualification receipts committed, Preview rollback rehearsal receipt committed, pre-deploy checklist passed), no mainnet operation SHALL run. Each mainnet operation SHALL be a T4 operation with owner+checklist authorization, SHALL have explicit owner authorization at the time it runs, and SHALL stay within the owner-set per-operation spend ceiling with no automatic retry."*
  - `FRC-004`: *"FRC-004: If wallet balancing fails with a transport error, then the ledger runner SHALL make at most 4 attempts in total, 30 s apart, with a wallet resync before each new attempt."*
- **Witness State / Trace**:
  - In `network_operations.qnt`, `isLegalTargetChain(genesisId)` permits only `LOCAL_DEVNET_GENESIS (1001)` and `PINNED_PREVIEW_GENESIS (3096)`. Under `network_operations_m_pvw002_send_to_mainnet`, sending to any third chain `genesisId == 9999` (either an external user-pinned chain per `SCL-010` or an owner-authorized Mainnet chain per `SWF-009`) immediately violates `inv_PVW_002`. Furthermore, if `t10a-ledger-run.mjs` is invoked for `SWF-009`, `FRC-004` automatically retries balancing up to 4 times, violating `SWF-009`'s *"with no automatic retry"*.
- **Closing Fix**:
  - Scope `PVW-002` explicitly to the **factory prototype loop** (*"Where the settle runner runs inside the tested-design factory before T4 authorization, the settle runner SHALL send a transaction to 2 chains only..."*), allow user-supplied chain pins in external SDK mode (`SCL-010`), and force `T10A_BALANCE_TRIES = 1` whenever running a `T4` Mainnet operation under `SWF-009`.

### `Q-network-operations-2`: Control `R3` (Replay) Expects Node Proof Error `115` (`PVW-006`), But Local Wallet Balancing Fails Pre-Submission and Halts Without a Node Verdict (`FRC-016` & `PVW-007`)

- **IDs**: `PVW-006`, `PVW-007`, `FRC-016`
- **Quoted EARS Sentences**:
  - `PVW-006`: *"PVW-006: When the readback of the included honest call arrives, the Preview runner SHALL submit the 4 controls R1 (changed public input), R2 (flipped proof bit), R4 (other program) and R3 (replay)."*
  - `PVW-006` Scenario (*The node refuses each control*): *"GIVEN the included honest call A / WHEN the four controls are submitted / THEN each gets `1010: Invalid Transaction: Custom error: 115` and none is included"*
  - `FRC-016`: *"FRC-016: If wallet balancing fails with an error other than a transport error, then the ledger runner SHALL record the attempt and stop the call without a retry."*
  - `PVW-007`: *"PVW-007: If a submitted control ends with no inclusion, no inclusion timeout and no JSON-RPC error from the node, then the Preview runner SHALL record `no-node-verdict` and fail the verdict."*
- **Witness State / Trace**:
  - Controls `R1`, `R2`, and `R4` carry invalid PLONK wrap proofs, so they pass Compact circuit transcript evaluation locally during wallet balancing and are rejected at the node's verifier with `1010: Invalid Transaction: Custom error: 115`. Control `R3` (replay of already-settled `claim_pub`), by contrast, carries a *cryptographically valid* proof for an already-recorded `claim_pub`:
    1. If the runner builds a fresh transaction for `R3` through normal wallet balancing, local Compact contract execution (`!runs.member(claim_pub)`) throws a contract-assertion error *before* RPC submission; `FRC-016` stops without submitting, no JSON-RPC error arrives from the node, and `PVW-007` classifies `R3` as `no-node-verdict` and fails the entire Preview run (`no_tamperControlOutcomesAndExitCode4Test`).
    2. Even if raw pre-balanced bytes of honest call `A` are re-broadcast directly over RPC, the node rejects spent UTXOs / duplicate transaction hashes rather than PLONK proof error `Custom error: 115`.
- **Closing Fix**:
  - Specify in `PVW-006` how `R3` is constructed (either pre-built against a pre-inclusion ledger state snapshot and submitted raw to test on-chain contract/UTXO rejection, or distinguishing local contract-replay rejection from `Custom error: 115` in `PVW-006`'s acceptance predicate).

### `Q-network-operations-3`: `600 s` Inclusion Timeout (`PVW-008`) Leaves `3,000 s` of Live Mempool TTL (`SCL-007`), Making `FRC-013` Finality and `FRC-003` Post-Resolution Reuse Mutually Exclusive

- **IDs**: `PVW-008`, `FRC-003`, `FRC-013`, `SCL-007`
- **Quoted EARS Sentences**:
  - `PVW-008`: *"PVW-008: If a submitted call is not seen in a block within 600 s on Preview, then the Preview runner SHALL record `not-included-timeout` for the call."*
  - `SCL-007`: *"SCL-007: If `T10A_TTL_S` is not above 0 and below 1,209,600, then the ledger runner SHALL stop before any transaction is built."* (Status notes settlement TTL is `3,600 s`.)
  - `FRC-003`: *"FRC-003: While a proven, unrefused call that is not included has more than 5 minutes of TTL left, the ledger runner SHALL reuse the saved call on a rerun."* (Status notes: *"A call that was submitted and resolved as not included (FRC-013) is reused too."*)
  - `FRC-013`: *"FRC-013: If the inclusion state of a submitted call is unknown, then the ledger runner SHALL resolve the state from the node and the indexer before the ledger runner builds another submission of the same claim."*
- **Witness Predicate**:
  - Verified in `gap_q_netops_3_timeout_vs_ttl_expiry_reuse` (`network_operations.qnt:322-328`):
    - At `t = 600 s` (`INBLOCK_TIMEOUT_S`), a submitted transaction built with `defaultTtlS = 3,600 s` still has `afterInblockTimeoutLeftS = 3,000 s > 300 s` (`MIN_REUSE_TTL_LEFT_S`) of live validity in the Preview mempool; neither node nor indexer can resolve it as permanently unincluded (`FRC-013`), so building a second balanced submission races against the first on wallet UTXOs and fees.
    - Conversely, once the transaction's intent TTL actually expires (`t >= 3,600 s`) so `FRC-013` can definitively resolve it as `expired` / not-included, the saved unbound call's own TTL (`afterTrueExpiryLeftS = 0 s`) satisfies `not(0 > 300)`, so `FRC-003` forbids reusing it. Thus `FRC-003` Status's claim that a submitted call resolved as not-included via `FRC-013` is reused is unreachable whenever the saved unbound call and the balanced submission share `T10A_TTL_S`.
- **Closing Fix**:
  - Separate the unbound proof/intent TTL (`T10A_CALL_TTL_S`, e.g., `3,600 s`) from a shorter per-submission balancing/dust TTL or require `FRC-013` to wait until the submitted transaction's TTL expires before declaring `expired` and re-proving/re-balancing.

### `Q-network-operations-4`: `PVW-009` Mandates Final Preview Deploy Upon `T12` Pass Alone, Contradicting `PVW-013`'s Six-Gate Refusal Rule

- **IDs**: `PVW-009`, `PVW-013`
- **Quoted EARS Sentences**:
  - `PVW-009`: *"PVW-009: When the T12 gate passes on the T25 key set, the deploy runner SHALL deploy the prototype SherlockLedger contracts on Preview with the existing Preview wallet."*
  - `PVW-013`: *"PVW-013: If the gate record of the wrap key to deploy lacks a pass for RKY-003, RKY-004, RKY-005, the T12 gate, the zero-knowledge checks ZK-012 to ZK-017 or the 5 wrap checks of WRP-012, then the deploy runner SHALL refuse the final prototype deploy."*
- **Witness State**:
  - Verified in `gap_q_netops_4_pvw009_vs_pvw013` (`network_operations.qnt:332-342`) and mutant `network_operations_m_pvw013_deploy_missing_gate`: for `partialGate = { rky003RowsOk: true, rky004VkBytesOk: true, rky005Security100BitOk: false, t12GatePass: true, zk012To017Pass: true, wrp012FiveChecksPass: false }`, `partialGate.t12GatePass` is `true` (triggering `PVW-009`'s mandatory `SHALL deploy`), whereas `fullKeyGatePassed(partialGate)` is `false` (triggering `PVW-013`'s mandatory `SHALL refuse`).
- **Closing Fix**:
  - Amend `PVW-009`'s trigger to *"When the full wrap-key gate record of PVW-013 passes on the T25 key set, the deploy runner SHALL deploy..."*.

### `Q-network-operations-5`: Normative EARS Sentences of `FRC-011` and `LPC-015` Exempt `t10q` Unconditionally, Omitting the `FP-3` Gate Required by `LPC-015`'s Own Scenario

- **IDs**: `FRC-011`, `LPC-015`
- **Quoted EARS Sentences**:
  - `FRC-011`: *"FRC-011: If a push names a branch other than `t10q` before the owner signs T15, then the tested-design loop SHALL hold the push."*
  - `LPC-015`: *"LPC-015: If a push targets a branch other than `t10q` before T15, then the tested-design loop SHALL hold the push."*
  - `LPC-015` Scenario & Status: *"WHEN FP-3 passes / THEN the t10q branch push is allowed (owner order)"* and *"the status on 2026-11-15 is: t10q push held on FP-3."*
- **Witness State**:
  - In state `{ fp3Passed: false, t15OwnerSigned: false, pushedBranches: Set("t10q") }`, `inv_FRC_011` (`(br != "t10q") implies net.t15OwnerSigned`) evaluates to `true`, permitting `t10q` to be pushed *before* `FP-3` passes, while `inv_LPC_015` (`net.pushedBranches.contains("t10q") implies (net.fp3Passed or net.t15OwnerSigned)`) evaluates to `false`.
- **Closing Fix**:
  - Add the explicit `FP-3` guard to both normative EARS sentences: *"If a push targets `t10q` before FP-3 passes, or targets any other branch before the owner signs T15, then the tested-design loop SHALL hold the push."*

### `Q-network-operations-6`: Exit-Code Conflation Between `PVW-007` (`no-node-verdict`) and `SCL-002` (`4` Verdict Fail vs `1` Operational Error), Plus Unlisted `FRC-016`

- **IDs**: `PVW-007`, `PVW-012`, `SCL-002`, `FRC-016`
- **Quoted EARS Sentences**:
  - `SCL-002`: *"SCL-002: When the ledger runner ends, the ledger runner SHALL exit with 0 for a passed verdict, 4 for a failed verdict and 1 for an error."*
  - `PVW-007`: *"PVW-007: If a submitted control ends with no inclusion, no inclusion timeout and no JSON-RPC error from the node, then the Preview runner SHALL record `no-node-verdict` and fail the verdict."*
  - `PVW-012`: *"PVW-012: If a tamper control is included in a block, then the Preview runner SHALL record `included` for the control and fail the verdict."*
- **Witness State**:
  - In `doRunTamperControlsAndVerdict(CtrlNoNodeVerdict, false)`, a dropped socket during control submission triggers *"fail the verdict"* (`VerdictFailed`), which `SCL-002` maps to exit status `4`—the exact same exit status (`4`) used when a forged proof is accepted and included in a block (`PVW-012`), rather than `SCL-002`'s exit status `1` for operational/transport errors. Automation (`t17.sh`, `FRC-009`) cannot distinguish a transient RPC disconnect from a catastrophic soundness break by exit code alone.
  - Additionally, `openspec/changes/sherlock-e2e-prototype-20261115/specs/sherlock-e2e-recovery/spec.md` defines `FRC-016` (*"Other balancing errors stop the call"*), which was omitted from the `FRC-001..015` list in `PROMPT.md` (modeled in `inv_FRC_004` and `doRecordWalletBalanceFailure`).
- **Closing Fix**:
  - Specify explicitly in `SCL-002` and `PVW-007` whether `no-node-verdict` exits with `1` (operational transport error, incomplete run) or `4` (failed soundness verdict), and add `FRC-016` to the formal coverage manifest.

---

## 3. Mutant Table

All `32` guard mutants (`MUTANT_ID = 1..32`) were executed with `quint run network_operations.qnt --main <mutant_module> --invariant <target_inv> --max-samples 1000 --max-steps 15 --seed 0x20261115`. Every mutant was killed (`exit code 1`, `[violation] Found an issue`), and every mutant was verified **non-vacuous** by confirming `reach_full_preview_pass_verdict` is still violated (`exit code 1`) on that mutant module.

| Mutant Module | `MUTANT_ID` | Invariant It Must Break | Exit Code Seen | Non-Vacuous (`reach_full_preview_pass_verdict`) |
|---|---:|---|---:|---|
| `network_operations_m_pvw001_wrong_genesis_wallet` | `1` | `inv_PVW_001` | `1` | Yes (`exit 1`) |
| `network_operations_m_pvw002_send_to_mainnet` | `2` | `inv_PVW_002` | `1` | Yes (`exit 1`) |
| `network_operations_m_pvw003_initial_fee_params` | `3` | `inv_PVW_003` | `1` | Yes (`exit 1`) |
| `network_operations_m_pvw004_touch_port_6340` | `4` | `inv_PVW_004` | `1` | Yes (`exit 1`) |
| `network_operations_m_pvw007_pass_no_node_verdict` | `5` | `inv_PVW_007` | `1` | Yes (`exit 1`) |
| `network_operations_m_pvw008_timeout_as_refusal` | `6` | `inv_PVW_008` | `1` | Yes (`exit 1`) |
| `network_operations_m_pvw010_final_without_gpu_box` | `7` | `inv_PVW_010` | `1` | Yes (`exit 1`) |
| `network_operations_m_pvw011_accept_stale_validium` | `8` | `inv_PVW_011` | `1` | Yes (`exit 1`) |
| `network_operations_m_pvw012_pass_included_control` | `9` | `inv_PVW_012` | `1` | Yes (`exit 1`) |
| `network_operations_m_pvw013_deploy_missing_gate` | `10` | `inv_PVW_013` | `1` | Yes (`exit 1`) |
| `network_operations_m_wal002_unscrubbed_secret` | `11` | `inv_WAL_002` | `1` | Yes (`exit 1`) |
| `network_operations_m_wal003_unpinned_wallet` | `12` | `inv_WAL_003` | `1` | Yes (`exit 1`) |
| `network_operations_m_wal004_concurrent_senders` | `13` | `inv_WAL_004` | `1` | Yes (`exit 1`) |
| `network_operations_m_wal005_private_in_receipts` | `14` | `inv_WAL_005` | `1` | Yes (`exit 1`) |
| `network_operations_m_wal007_submit_from_prover` | `15` | `inv_WAL_007` | `1` | Yes (`exit 1`) |
| `network_operations_m_wal009_overwrite_wallet` | `16` | `inv_WAL_009` | `1` | Yes (`exit 1`) |
| `network_operations_m_wal010_wide_mode_read` | `17` | `inv_WAL_010` | `1` | Yes (`exit 1`) |
| `network_operations_m_scl002_wrong_exit_code` | `18` | `inv_SCL_002` | `1` | Yes (`exit 1`) |
| `network_operations_m_scl003_dryrun_network` | `19` | `inv_SCL_003` | `1` | Yes (`exit 1`) |
| `network_operations_m_scl004_mismatched_vk_folder` | `20` | `inv_SCL_004` | `1` | Yes (`exit 1`) |
| `network_operations_m_scl007_out_of_range_ttl` | `21` | `inv_SCL_007` | `1` | Yes (`exit 1`) |
| `network_operations_m_obs003_dead_walker_or_leak` | `22` | `inv_OBS_003` | `1` | Yes (`exit 1`) |
| `network_operations_m_frc001_mark_failed_step_ok` | `23` | `inv_FRC_001` | `1` | Yes (`exit 1`) |
| `network_operations_m_frc002_double_deploy` | `24` | `inv_FRC_002` | `1` | Yes (`exit 1`) |
| `network_operations_m_frc004_excess_or_bad_retry` | `25` | `inv_FRC_004` | `1` | Yes (`exit 1`) |
| `network_operations_m_frc005_reuse_refused_call` | `26` | `inv_FRC_005` | `1` | Yes (`exit 1`) |
| `network_operations_m_frc006_refusals_no_readback` | `27` | `inv_FRC_006` | `1` | Yes (`exit 1`) |
| `network_operations_m_frc011_push_before_t15` | `28` | `inv_FRC_011` | `1` | Yes (`exit 1`) |
| `network_operations_m_frc013_resubmit_unknown` | `29` | `inv_FRC_013` | `1` | Yes (`exit 1`) |
| `network_operations_m_frc014_reuse_corrupt_file` | `30` | `inv_FRC_014` | `1` | Yes (`exit 1`) |
| `network_operations_m_frc003_reuse_low_ttl` | `31` | `inv_FRC_003` | `1` | Yes (`exit 1`) |
| `network_operations_m_swf009_unauth_mainnet` | `32` | `inv_SWF_009` | `1` | Yes (`exit 1`) |

---

## 4. State Vocabulary for Replay

### 4.1 Top-Level State Variables (`var`) and ITF Keys

| Quint `var` | Exact ITF JSON Key in `t_{0..19}.itf.json` | Quint Type | Meaning |
|---|---|---|---|
| `net` | `network_operations_main::network_operations::net` | `NetOpsState` | Full observable state record of the Network Operations bounded context. |
| `lastOp` | `network_operations_main::network_operations::lastOp` | `str` | Action tag written by `init` (`"init"`) and every transition step to drive oracle replay. |

### 4.2 Sum Types and Nested Record Types

- `ControlOutcome`: `CtrlIncluded | CtrlNotIncludedTimeout | CtrlRefusedByNode | CtrlNoNodeVerdict` (exclusive outcomes in `PVW-007` / `PVW-008` / `PVW-012` order).
- `RunVerdictState`: `VerdictPending | VerdictPassed | VerdictFailed | VerdictError`.
- `KeyGateRecord`: `{ rky003RowsOk: bool, rky004VkBytesOk: bool, rky005Security100BitOk: bool, t12GatePass: bool, zk012To017Pass: bool, wrp012FiveChecksPass: bool }` (`PVW-009`, `PVW-013`).
- `SavedCallArtifact`: `{ present: bool, ttlLeftS: int, markedRefused: bool, inclusionUnknown: bool, includedOnChain: bool, sha256Valid: bool, vkMatchesFolder: bool, genesisMatchesFolder: bool }` (`FRC-003`, `FRC-005`, `FRC-013`, `FRC-014`).

### 4.3 `NetOpsState` Fields

| Field | Type | Meaning |
|---|---|---|
| `rpcGenesis` | `int` | Genesis identifier reported by the connected RPC node (`3096` = Preview, `1001` = `undeployed1`). |
| `configuredTtlS` | `int` | Configured `T10A_TTL_S` in seconds (`SCL-007`). |
| `ttlAccepted` | `bool` | Whether the runner accepted `configuredTtlS`. |
| `folderRecordedVkId` | `int` | Wrap VK identifier recorded in the state folder (`SCL-004`). |
| `cliVkId` | `int` | `--vk` argument supplied to the ledger runner (`SCL-004`). |
| `folderAccepted` | `bool` | Whether the state folder was accepted for this run (`SCL-004`). |
| `proofServerPort` | `int` | TCP port bound by the Preview runner's proof server (`6341`, `PVW-004`). |
| `devnetPort6340Untouched` | `bool` | True iff local devnet proof-server port `6340` was left untouched (`PVW-004`). |
| `usedChainFeeParams` | `bool` | True iff transaction balancing priced fees from chain ledger parameters (`PVW-003`). |
| `sentChainGenesises` | `Set[int]` | Set of chain genesis IDs to which the settle runner sent transactions (`PVW-002`). |
| `dryRunExecuted` | `bool` | True once suite/runner `dry-run` phase has run (`SCL-003`). |
| `dryRunNetworkConnections` | `int` | Count of network connections opened during `dry-run` (`0`, `SCL-003`). |
| `dryRunTxSent` | `int` | Count of transactions sent during `dry-run` (`0`, `SCL-003`). |
| `outsideFactoryCleanConfigOk` | `bool` | True iff external SDK runs read wallet/chain/roster from user config (`SCL-010`). |
| `sdkRootCheckStoppedCleanly` | `bool` | True when unset `SHERLOCK_SDK_ROOT` halted at startup naming the variable (`SCL-009`). |
| `walletDirMode` | `int` | Octal-encoded integer mode of wallet directory (e.g., `700` for `0700`, `WAL-010`). |
| `phraseFileMode` | `int` | Octal-encoded integer mode of phrase file (e.g., `600` for `0600`, `WAL-001`, `WAL-010`). |
| `phraseExistsInDir` | `bool` | True when `phrase.txt` already exists in the wallet directory (`WAL-009`). |
| `existingWalletOverwritten` | `bool` | True if wallet setup overwrote an existing phrase file (`WAL-009`). |
| `phraseReadInProcess` | `bool` | True once the wallet process read the phrase file directly (`WAL-001`, `WAL-010`). |
| `phraseExposedToShellOrEnv` | `bool` | True if phrase leaked to CLI args or environment (`WAL-001`). |
| `derivedWalletAddr` | `int` | Unshielded wallet address derived from the phrase (`WAL-003`). |
| `pinnedWalletAddr` | `int` | Pinned expected wallet address (`WAL-003`). |
| `walletStarted` | `bool` | True once wallet startup checks passed and `balance-before` ran (`PVW-001`, `WAL-003`, `WAL-010`). |
| `activeWalletSenders` | `int` | Concurrent processes holding `wallet.lock` (`<= 1`, `WAL-004`). |
| `submittedFromWalletHost` | `bool` | True iff transactions were submitted from the wallet host rather than prover boxes (`WAL-007`). |
| `unscrubbedSecretWritten` | `bool` | True if any unscrubbed secret pattern reached logs or files (`WAL-002`). |
| `nonAllowlistFieldInReceipts` | `bool` | True if `receipts.json` contained fields outside the public allow-list (`WAL-005`). |
| `keyMaterialInBalanceFile` | `bool` | True if `balance-before/after.json` held coins or key material (`WAL-006`). |
| `secretsInRepoCount` | `int` | Count of wallet keys/phrases in tracked repository files (`0`, `WAL-008`). |
| `t25KeyGate` | `KeyGateRecord` | Six-part gate status for the `T25` wrap key (`PVW-009`, `PVW-013`). |
| `finalPrototypeDeployed` | `bool` | True once final `T25` prototype contracts are deployed on Preview (`PVW-009`, `PVW-013`). |
| `finalCallProvedOnOwnedGpuBox` | `bool` | True iff final prototype prove ran on an owner GPU box (`PVW-010`, `LPC-008`). |
| `deploySendCount` | `int` | Number of contract deploy transactions broadcast (`<= 1`, `FRC-002`). |
| `deployConfirmed` | `bool` | True once contract deployment is confirmed or recovered from indexer (`FRC-002`). |
| `honestCallIncluded` | `bool` | True once honest call `A` is included in a Preview block (`PVW-006`, `OBS-001`). |
| `honestCallReadbackPresent` | `bool` | True once on-chain readback of honest call `A` is recorded (`PVW-006`, `FRC-006`). |
| `refusalsStepStarted` | `bool` | True once the `refusals` step has started (`PVW-006`, `FRC-006`). |
| `submittedControlCount` | `int` | Number of tamper controls submitted (`4`, `PVW-006`). |
| `controlOutcome` | `ControlOutcome` | Aggregated worst/test outcome among submitted tamper controls (`PVW-007`, `PVW-008`, `PVW-012`). |
| `verdictState` | `RunVerdictState` | Current ledger-runner verdict (`SCL-002`). |
| `runnerExitCode` | `int` | Process exit code (`0`, `4`, or `1`, `SCL-002`, `PVW-012`). |
| `validiumExpectedProgCP` | `int` | Expected program commitment pinned in the validium contract (`1007`, `PVW-011`). |
| `validiumStoredRoot` | `int` | Current L2 state root stored in the validium contract (`PVW-011`, `LPC-009`). |
| `validiumAcceptedBatches` | `int` | Count of accepted chained validium batches (`LPC-009`). |
| `validiumAcceptedBadBatch` | `bool` | True if a stale-root or wrong-program validium batch was accepted (`PVW-011`). |
| `manifestSha256Written` | `bool` | True once `manifest.sha256` was written during `check` (`OBS-006`). |
| `includedReceiptComplete` | `bool` | True iff included-call receipt holds all required fields (`OBS-001`). |
| `refusedReceiptComplete` | `bool` | True iff refused-control receipts hold all required fields (`OBS-002`). |
| `privacyScanPassed` | `bool` | True once privacy scan completed on the call surface (`OBS-003`, `OBS-004`). |
| `forbiddenPrivateWordOnSurface` | `bool` | True if any private witness/memory word leaked onto the call surface (`OBS-003`). |
| `controlWordsPresentOnSurface` | `bool` | True iff positive control words (`claim_pub`, `outcome_pub`) fired in the walker (`OBS-004`). |
| `walletFailureChainRecorded` | `bool` | True iff wallet error logs include full `cause` chain and stack (`OBS-008`). |
| `stepOkMarkers` | `Set[str]` | Set of `<step>.ok` markers written in the state folder (`FRC-001`, `PVW-005`). |
| `failedStepsMarkedOk` | `Set[str]` | Set of steps that exited non-zero yet wrote `<step>.ok` (`Set()`, `FRC-001`). |
| `savedCall` | `SavedCallArtifact` | Metadata of saved unbound/submitted call in the state folder (`FRC-003`, `FRC-005`, `FRC-013`, `FRC-014`). |
| `reusedLowTtlCall` | `bool` | True if a saved call with `ttlLeftS <= 300` was reused (`FRC-003`). |
| `reusedRefusedCall` | `bool` | True if a saved call marked refused was reused (`FRC-005`). |
| `reusedCorruptOrMismatchedCall` | `bool` | True if a saved call failing SHA-256, VK, or genesis checks was reused (`FRC-014`). |
| `resubmittedWhileInclusionUnknown` | `bool` | True if a claim was resubmitted before resolving unknown inclusion state (`FRC-013`). |
| `suiteSkippedSettledCallsOk` | `bool` | True iff `T23` resume skips calls whose `runs[claim_pub] == outcome_pub` (`FRC-012`). |
| `balanceIsTransportErr` | `bool` | True when the recorded wallet balancing error is a transport error (`FRC-004`, `FRC-016`). |
| `balanceAttemptCount` | `int` | Total balancing attempts made on the failing call (`1..4` for transport, `1` otherwise; `FRC-004`, `FRC-016`). |
| `resyncedBeforeEachRetry` | `bool` | True iff wallet resynced before each retry attempt (`FRC-004`). |
| `rowConsecutiveGateFails` | `int` | Consecutive gate failures on a loop row (`FRC-009`). |
| `rowStoppedOnDoubleFail` | `bool` | True once the row halted after `>= 2` consecutive gate failures (`FRC-009`). |
| `otherRowsStillRunning` | `bool` | True iff halting the failing row left unaffected rows running (`FRC-009`). |
| `t15OwnerSigned` | `bool` | True once the owner has signed the `T15` interface freeze (`FRC-011`, `LPC-015`). |
| `fp3Passed` | `bool` | True once formal gate `FP-3` has passed (`LPC-015`). |
| `pushedBranches` | `Set[str]` | Set of git branches pushed to remote (`FRC-011`, `LPC-015`). |
| `mainnetOpRan` | `bool` | True once a `T4` Mainnet operation executed (`SWF-009`). |
| `mainnetAll4PreconditionsMet` | `bool` | True iff all 4 `SWF-009` amendment preconditions held (`SWF-009`). |
| `mainnetExplicitPerOpOwnerAuth` | `bool` | True iff explicit per-operation owner+checklist authorization held (`SWF-009`). |
| `mainnetAutoRetried` | `bool` | True if a `T4` Mainnet operation automatically retried (`false`, `SWF-009`). |
| `lastStopReason` | `str` | Diagnostic reason recorded by the most recent refusal/stop guard. |

### 4.4 `lastOp` Values and Implemented Requirements

| `lastOp` Value | Action in `network_operations.qnt` | Implemented EARS IDs |
|---|---|---|
| `"init"` | `init` | Initial state prior to steps (`PVW-001..013`, `WAL-001..010`, `SCL-001..010`, `OBS-001..008`, `FRC-001..016`, `LPC-008..009`, `LPC-015`, `SWF-009`) |
| `"runCheckStep"` | `doRunCheckStep` | `PVW-004`, `SCL-004`, `SCL-007`, `OBS-006`, `FRC-001` |
| `"stopPreWalletOrConfig"` | `doStopPreWalletOrConfig` | `PVW-001`, `PVW-002`, `SCL-002`, `SCL-004`, `SCL-007`, `SCL-009`, `WAL-003`, `WAL-010` |
| `"attemptWalletSetupOverwrite"` | `doAttemptWalletSetupOverwrite` | `WAL-009` |
| `"startWalletAndBalanceBefore"` | `doStartWalletAndBalanceBefore` | `PVW-001`, `WAL-001`, `WAL-003`, `WAL-004`, `WAL-006`, `WAL-010`, `FRC-001` |
| `"runSuiteDryRun"` | `doRunSuiteDryRun` | `SCL-003` |
| `"deployOrRecoverContract"` | `doDeployOrRecoverContract` | `PVW-002`, `PVW-003`, `WAL-007`, `FRC-001`, `FRC-002` |
| `"recordT25KeyGate"` | `doRecordT25KeyGate` | `PVW-009`, `PVW-013`, `LPC-008` |
| `"deployAndProveFinalPrototype"` | `doDeployAndProveFinalPrototype` | `PVW-009`, `PVW-010`, `PVW-013`, `LPC-008` |
| `"includeHonestCallAndScan"` | `doIncludeHonestCallAndScan` | `OBS-001`, `OBS-003`, `OBS-004`, `WAL-002`, `WAL-005`, `FRC-001` |
| `"runTamperControlsAndVerdict"` | `doRunTamperControlsAndVerdict` | `PVW-006`, `PVW-007`, `PVW-008`, `PVW-012`, `SCL-002`, `OBS-002`, `FRC-001`, `FRC-006` |
| `"submitValidiumBatch"` | `doSubmitValidiumBatch` | `PVW-011`, `LPC-009` |
| `"setSavedCallState"` | `doSetSavedCallState` | `FRC-003`, `FRC-005`, `FRC-013`, `FRC-014` |
| `"reuseSavedCall"` | `doReuseSavedCall` | `FRC-003`, `FRC-005`, `FRC-013`, `FRC-014` |
| `"attemptResubmitClaim"` | `doAttemptResubmitClaim` | `FRC-013` |
| `"recordWalletBalanceFailure"` | `doRecordWalletBalanceFailure` | `FRC-004`, `FRC-016`, `OBS-008` |
| `"recordGateFailureOnRow"` | `doRecordGateFailureOnRow` | `FRC-009` |
| `"advancePushGates"` | `doAdvancePushGates` | `FRC-011`, `LPC-015` |
| `"pushBranch"` | `doPushBranch` | `FRC-011`, `LPC-015` |
| `"runMainnetT4Operation"` | `doRunMainnetT4Operation` | `SWF-009` |

### 4.5 Constants of Concrete Instances

| Constant | `network_operations_main` | `network_operations_edge_min` | `network_operations_edge_at_limit` | Mutant Modules (`m_*`) |
|---|---:|---:|---:|---:|
| `PINNED_PREVIEW_GENESIS` | `3096` | `3096` | `3096` | `3096` |
| `LOCAL_DEVNET_GENESIS` | `1001` | `1001` | `1001` | `1001` |
| `PREVIEW_PROOF_SERVER_PORT` | `6341` | `6341` | `6341` | `6341` |
| `DEVNET_PROOF_SERVER_PORT` | `6340` | `6340` | `6340` | `6340` |
| `TAMPER_CONTROL_COUNT` | `4` | `4` | `4` | `4` |
| `INBLOCK_TIMEOUT_S` | `600` | `60` | `600` | `600` |
| `MAX_WALLET_DIR_MODE` | `700` | `500` | `700` | `700` |
| `MAX_PHRASE_FILE_MODE` | `600` | `400` | `600` | `600` |
| `EXIT_VERDICT_FAIL` | `4` | `4` | `4` | `4` |
| `MAX_TTL_EXCLUSIVE_S` | `1209600` | `1209600` | `1209600` | `1209600` |
| `MIN_REUSE_TTL_LEFT_S` | `300` | `300` | `300` | `300` |
| `MAX_BALANCE_TRANSPORT_TRIES` | `4` | `1` | `4` | `4` |
| `MAX_CONSECUTIVE_GATE_FAILS` | `2` | `2` | `2` | `2` |
| `MUTANT_ID` | `0` | `0` | `0` | `1 .. 32` |

---

## 5. Model Abstractions

1. **Chain Genesises, Addresses, Keys, and Roots as Finite Integers**: 32-byte genesis hashes (`0x3c096de2...3796` -> `3096`, `undeployed1` -> `1001`, unauthorized chain -> `8888`/`9999`), Bech32m wallet addresses (`77`, `99`), wrap verifying keys (`1`, `2`), validium program commitments (`1007`), and Poseidon2 state roots (`10`, `20`, `30`, ...) are modeled as small integers preserving equality and inequality relations.
2. **POSIX Octal Modes as Monotone Decimal-Octal Integers**: File modes `0400`, `0500`, `0600`, `0640`, `0700`, `0755` are represented as integers `400`, `500`, `600`, `640`, `700`, `755`; for single-user vs group/other bits (`00xx`), integer `<=` matches no-group/no-other permission containment (`<= 700` for wallet dir, `<= 600` for phrase file).
3. **Aggregated Tamper-Control Outcome**: Rather than storing 4 separate identical outcome slots for `R1`, `R2`, `R4`, and `R3`, `submittedControlCount` tracks the cardinality (`4`) while `controlOutcome: ControlOutcome` represents the worst/decisive outcome across the 4 controls (`CtrlIncluded`, `CtrlNotIncludedTimeout`, `CtrlNoNodeVerdict`, or all 4 `CtrlRefusedByNode`).
4. ** Wall-Clock Time and Remaining TTL**: Wall-clock elapsed durations (`600 s` `InBlock` timeout, `30 s` retry spacing, `300 s` minimum reuse TTL, `< 1,209,600 s` configured TTL) are represented as explicit integer parameters/fields (`configuredTtlS`, `ttlLeftS`) and pure arithmetic predicates (`gap_q_netops_3_timeout_vs_ttl_expiry_reuse`) rather than a ticking global clock variable.
5. **Cryptographic Hashes, Regex Scrubbing, and Recursive Call-Tree Walking**: SHA-256 file integrity (`sha256Valid`), log secret scrubbing (`unscrubbedSecretWritten`), receipt field allow-listing (`nonAllowlistFieldInReceipts`), and recursive byte-buffer/TypedArray privacy scanning (`forbiddenPrivateWordOnSurface`, `controlWordsPresentOnSurface`) are abstracted into boolean outcome flags paired with guard mutants.
6. **Host Process Supervision and Git CLI Plumbing**: `systemd-run --user` unit lifetimes (`FRC-007`, `FRC-008`), `compactc` invocations (`SCL-006`), `E5JSON` stdout formatting (`SCL-005`), and git ancestry/worktree commands (`FRC-010`, `FRC-015`, `LPC-010`, `LPC-011`, `LPC-020`) are left unmodeled as `SPEC-CHECKED candidate` process/measurement rules per `PROMPT.md`.

---

## 6. Commands

Run all commands below from `/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/network-operations/quint`:

```bash
# 1. Typecheck base specification + instances + mutants (expected exit: 0; printed: empty)
quint typecheck network_operations.qnt
# Exit: 0 | Output: (none)

# 2. Typecheck deterministic scenario test module (expected exit: 0; printed: empty)
quint typecheck network_operations_test.qnt
# Exit: 0 | Output: (none)

# 3. Run deterministic scenario tests (expected exit: 0)
quint test --main=network_operations_test --match='no_.*Test' --seed=0x20261115 network_operations_test.qnt
# Exit: 0 | Printed: 5 passing (4493ms)

# 4. Honest simulation: main instance (expected exit: 0)
quint run network_operations.qnt --main network_operations_main --invariant inv --max-samples 2000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (18841ms at 106 traces/second).

# 5. Honest simulation: minimum-bound edge instance (expected exit: 0)
quint run network_operations.qnt --main network_operations_edge_min --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (3299ms at 303 traces/second).

# 6. Honest simulation: ceiling-bound edge instance (expected exit: 0)
quint run network_operations.qnt --main network_operations_edge_at_limit --invariant inv --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 0 | Printed: [ok] No violation found (2490ms at 402 traces/second).

# 7. Reachability witnesses on network_operations_main (all 6 MUST violate -> expected exit: 1)
quint run network_operations.qnt --main network_operations_main --invariant reach_full_preview_pass_verdict --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (528ms at 951 traces/second).

quint run network_operations.qnt --main network_operations_main --invariant reach_failed_control_verdict_exit_4 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (296ms at 169 traces/second).

quint run network_operations.qnt --main network_operations_main --invariant reach_final_t25_deploy_and_gpu_prove --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (308ms at 3 traces/second).

quint run network_operations.qnt --main network_operations_main --invariant reach_validium_two_chained_batches_and_stale_refusal --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (273ms at 7 traces/second).

quint run network_operations.qnt --main network_operations_main --invariant reach_dryrun_no_overwrite_and_double_fail_stop --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (440ms at 23 traces/second).

quint run network_operations.qnt --main network_operations_main --invariant reach_fp3_t10q_push_and_balance_failure_stops --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (273ms at 22 traces/second).

# 8. Guard mutants (all 32 MUST violate their target invariant -> expected exit: 1)
quint run network_operations.qnt --main network_operations_m_pvw001_wrong_genesis_wallet --invariant inv_PVW_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (476ms at 4 traces/second).

quint run network_operations.qnt --main network_operations_m_pvw002_send_to_mainnet --invariant inv_PVW_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (271ms at 4 traces/second).

quint run network_operations.qnt --main network_operations_m_pvw003_initial_fee_params --invariant inv_PVW_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (281ms at 4 traces/second).

quint run network_operations.qnt --main network_operations_m_pvw004_touch_port_6340 --invariant inv_PVW_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (381ms at 5 traces/second).

quint run network_operations.qnt --main network_operations_m_pvw007_pass_no_node_verdict --invariant inv_PVW_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (495ms at 752 traces/second).

quint run network_operations.qnt --main network_operations_m_pvw008_timeout_as_refusal --invariant inv_PVW_008 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (391ms at 581 traces/second).

quint run network_operations.qnt --main network_operations_m_pvw010_final_without_gpu_box --invariant inv_PVW_010 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (249ms at 4 traces/second).

quint run network_operations.qnt --main network_operations_m_pvw011_accept_stale_validium --invariant inv_PVW_011 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (855ms at 2 traces/second).

quint run network_operations.qnt --main network_operations_m_pvw012_pass_included_control --invariant inv_PVW_012 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (308ms at 162 traces/second).

quint run network_operations.qnt --main network_operations_m_pvw013_deploy_missing_gate --invariant inv_PVW_013 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (509ms at 43 traces/second).

quint run network_operations.qnt --main network_operations_m_wal002_unscrubbed_secret --invariant inv_WAL_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (648ms at 59 traces/second).

quint run network_operations.qnt --main network_operations_m_wal003_unpinned_wallet --invariant inv_WAL_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (264ms at 4 traces/second).

quint run network_operations.qnt --main network_operations_m_wal004_concurrent_senders --invariant inv_WAL_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (503ms at 2 traces/second).

quint run network_operations.qnt --main network_operations_m_wal005_private_in_receipts --invariant inv_WAL_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (303ms at 125 traces/second).

quint run network_operations.qnt --main network_operations_m_wal007_submit_from_prover --invariant inv_WAL_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (537ms at 2 traces/second).

quint run network_operations.qnt --main network_operations_m_wal009_overwrite_wallet --invariant inv_WAL_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (365ms at 8 traces/second).

quint run network_operations.qnt --main network_operations_m_wal010_wide_mode_read --invariant inv_WAL_010 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (380ms at 3 traces/second).

quint run network_operations.qnt --main network_operations_m_scl002_wrong_exit_code --invariant inv_SCL_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (313ms at 160 traces/second).

quint run network_operations.qnt --main network_operations_m_scl003_dryrun_network --invariant inv_SCL_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (274ms at 4 traces/second).

quint run network_operations.qnt --main network_operations_m_scl004_mismatched_vk_folder --invariant inv_SCL_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (325ms at 3 traces/second).

quint run network_operations.qnt --main network_operations_m_scl007_out_of_range_ttl --invariant inv_SCL_007 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (319ms at 3 traces/second).

quint run network_operations.qnt --main network_operations_m_obs003_dead_walker_or_leak --invariant inv_OBS_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (706ms at 54 traces/second).

quint run network_operations.qnt --main network_operations_m_frc001_mark_failed_step_ok --invariant inv_FRC_001 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (346ms at 145 traces/second).

quint run network_operations.qnt --main network_operations_m_frc002_double_deploy --invariant inv_FRC_002 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (253ms at 4 traces/second).

quint run network_operations.qnt --main network_operations_m_frc004_excess_or_bad_retry --invariant inv_FRC_004 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (416ms at 5 traces/second).

quint run network_operations.qnt --main network_operations_m_frc005_reuse_refused_call --invariant inv_FRC_005 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (434ms at 76 traces/second).

quint run network_operations.qnt --main network_operations_m_frc006_refusals_no_readback --invariant inv_FRC_006 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (538ms at 2 traces/second).

quint run network_operations.qnt --main network_operations_m_frc011_push_before_t15 --invariant inv_FRC_011 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (619ms at 2 traces/second).

quint run network_operations.qnt --main network_operations_m_frc013_resubmit_unknown --invariant inv_FRC_013 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (418ms at 5 traces/second).

quint run network_operations.qnt --main network_operations_m_frc014_reuse_corrupt_file --invariant inv_FRC_014 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (446ms at 47 traces/second).

quint run network_operations.qnt --main network_operations_m_frc003_reuse_low_ttl --invariant inv_FRC_003 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (773ms at 234 traces/second).

quint run network_operations.qnt --main network_operations_m_swf009_unauth_mainnet --invariant inv_SWF_009 --max-samples 1000 --max-steps 15 --seed 0x20261115
# Exit: 1 | Printed: [violation] Found an issue (409ms at 2 traces/second).

# 9. Generate 20 ITF replay traces (expected exit: 0)
mkdir -p /home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/network-operations/traces/network_operations_main
quint run network_operations.qnt --main network_operations_main --invariant inv --n-traces 20 --max-samples 20 --max-steps 15 --seed 0x20261115 --out-itf '/home/hoskinson/Projects/Moriarty-wt/sherlock-e2e-spec/formal/sherlock-e2e/network-operations/traces/network_operations_main/t_{seq}.itf.json'
# Exit: 0 | Printed: [ok] No violation found (850ms at 24 traces/second).
```
