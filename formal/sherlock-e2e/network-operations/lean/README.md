# Network Operations oracle (Lean)

Lean 4.35.0, no Mathlib. Shared codec: `../../codec` (`SherlockCodec`). Build on odin with
`lake build --wfail`; the library `NetworkOps` (Spec, Tests, Consistency, Audit) and the
executable `oracle` are default targets. Interface: `../ORACLE.md`.

- `NetworkOps/Spec.lean`: state, operations, `apply`, step domain, `Inv` and its preservation
  (one lemma per operation), requirement-level theorems, constants.
- `NetworkOps/Tests.lean`: pins (`decide +kernel` and `#guard`) written from the sentences.
- `NetworkOps/Consistency.lean`: SPEC-CHECKED rows (`<ID>_ok` predicates, `consistent`) and the
  decision functions of the privacy scan, push hold, suite skip, setup tool and labels.
- `NetworkOps/Audit.lean`: `#print axioms` of every theorem pinned (propext, Classical.choice,
  Quot.sound only) and `#audit_ns NetworkOps` pinned at `[]`.

## Requirement table

| ID | sentence | Lean name(s) | state |
|---|---|---|---|
| PVW-001 | If the genesis hash that the RPC node reports differs from the pinned Preview genesis `0x3c096de2...3796`, then the settle runner SHALL stop before any wallet step. | doChain, Inv.walletAfterChecks, PVW_001_no_wallet_on_other_chain, pin_chain | Lean done; replay pending |
| PVW-002 | The settle runner SHALL send a transaction to 2 chains only: the local devnet `undeployed1` and the pinned Preview genesis. | doChain, Inv.chainOk, PVW_002_two_chains, pin_chain, pin_devnet | Lean done; replay pending |
| PVW-003 | When the settle runner balances a Preview transaction, the settle runner SHALL price the fee with the ledger parameters read from the Preview chain. | PVW_003_ok | Lean consistency done |
| PVW-004 | When the Preview runner starts, the Preview runner SHALL start its own `proof-server:9.0.1` container on 127.0.0.1 port 6341. | PVW_004_ok | Lean consistency done |
| PVW-005 | The Preview runner SHALL expose the 7 steps `check`, `balance-before`, `deploy`, `call`, `refusals`, `balance-after` and `receipts` as one command each. | PVW_005_ok | Lean consistency done |
| PVW-006 | When the honest call is included, the Preview runner SHALL submit the 4 controls R1 (changed public input), R2 (flipped proof bit), R4 (other program) and R3 (replay). | doSubmit, Inv.controlsOk, Inv.controlCount, PVW_006_four_controls, pin_pass | Lean done; replay pending |
| PVW-007 | If a submitted control ends with no inclusion, no inclusion timeout and no JSON-RPC error from the node, then the Preview runner SHALL record `no-node-verdict` and fail the verdict. | doOutcome (k=3), PVW_012_failed_control_fails, pin_noverdict | Lean done; replay pending |
| PVW-008 | If a submitted call is not seen in a block within 600 s on Preview, then the Preview runner SHALL record `not-included-timeout` for the call. | doOutcome (k=2), pin_timeout, pin_ctrl_timeout | Lean done; replay pending |
| PVW-009 | When the T12 gate passes on the T25 key set, the deploy runner SHALL deploy the prototype SherlockLedger contracts on Preview with the existing Preview wallet. | doDeploy, Inv.finalGate, pin_pass | Lean done; replay pending |
| PVW-010 | When the final prototype call is proved, the Preview runner SHALL run the `L_Fr` prove and the wrap prove on a GPU box that the user owns. | doProve, Inv.proofHostOk, PVW_010_final_on_gpu, pin_final_cpu, pin_not_owned | Lean done; replay pending |
| PVW-011 | If a Sherlock claim offered to the validium contract has another program commitment or an input root that differs from the stored root, then the validium contract SHALL refuse the batch. | doBatch, PVW_011_validium_refuses, pin_validium | Lean done; replay pending |
| PVW-012 | If a tamper control is included in a block, then the Preview runner SHALL record `included` for the control and fail the verdict. | doOutcome (k=1), PVW_012_failed_control_fails, pin_r2 | Lean done; replay pending |
| PVW-013 | If the gate record of the wrap key to deploy lacks a pass for RKY-003, RKY-004, RKY-005, the T12 gate or the zero-knowledge checks ZK-012 to ZK-017, then the deploy runner SHALL refuse the final prototype deploy. | doDeploy, Inv.finalGate, PVW_013_final_gate, pin_gate | Lean done; replay pending |
| WAL-001 | When the settle runner needs the Preview wallet, the settle runner SHALL read the wallet phrase inside its own process from the mode-0600 phrase file of the wallet directory. | doModes (walletRead), Inv.readPhase, WAL_001_ok, pin_modes | Lean done; replay pending |
| WAL-002 | When the settle runner writes a message to a log, a state file, an error record or a child process, the settle runner SHALL replace each occurrence of the wallet phrase, of the master seed and of a derived private key with `<seed>`. | doLog, Inv.redact, WAL_002_redactions, pin_redact | Lean done; replay pending |
| WAL-003 | If the address derived from the wallet phrase differs from the wallet address pinned in the run configuration, then the settle runner SHALL stop before the wallet starts. | doAddr, Inv.startedPhase, pin_addr | Lean done; replay pending |
| WAL-004 | While the settle runner syncs, balances, submits or waits for a Preview transaction, the settle runner SHALL hold an exclusive `flock` on `~/.cache/preview-wallet.lock`. | enabled (lock), Inv.lockOk, WAL_004_lock, pin_lock | Lean done; replay pending |
| WAL-005 | When the receipts writer builds `receipts.json`, the receipts writer SHALL copy only the fields of its allow-list from the run state. | WAL_005_ok, ZK_002_ok | Lean consistency done |
| WAL-006 | When the balance phase runs, the settle runner SHALL write the public balances of the wallet and no key material. | WAL_006_ok | Lean consistency done |
| WAL-007 | The settle runner SHALL submit every Preview transaction from the 1 machine that holds the wallet directory. | doSubmit (host), pin_offhost | Lean done; replay pending |
| WAL-008 | The Moriarty repository SHALL hold 0 wallet phrases, seeds, private keys and deployment blinders `r_P`. | WAL_008_ok | Lean consistency done |
| WAL-009 | If a wallet phrase file exists in the wallet directory, then the wallet setup tool SHALL refuse to create a wallet in that directory. | setupCreates, WAL_009_refuses | Lean consistency done |
| WAL-010 | If the mode of the wallet directory is wider than 0700 or the mode of the phrase file is wider than 0600, then the settle runner SHALL stop before the phrase is read. | doModes, Inv.readPhase, pin_modes | Lean done; replay pending |
| SCL-001 | The Sherlock SDK SHALL provide the 7 settlement commands `sherlock_plan`, `lfr_call`, `wrap`, `sherlock-wrap-ledger-check`, `accept_args.py`, `t10a-ledger-run.mjs` and `t17.sh`. | SCL_001_ok | Lean consistency done |
| SCL-002 | When the ledger runner ends, the ledger runner SHALL exit with 0 for a passed verdict, 4 for a failed verdict and 1 for an error. | doFinish, Inv.exitOk, SCL_002_exit_codes, pin_pass, pin_r2, pin_exit1 | Lean done; replay pending |
| SCL-003 | Where the `dry-run` phase is selected, the ledger runner SHALL run the padding, the JS shape check and the privacy scan with 0 network connections. | enabled (chainCheck), Inv.dryNoNet, Inv.dryFlags, SCL_003_dry_run, pin_dry | Lean done; replay pending |
| SCL-004 | If the deployed key recorded in a state folder differs from the `--vk` argument, then the ledger runner SHALL refuse the state folder. | doState, Inv.workAfterChecks, pin_state | Lean done; replay pending |
| SCL-005 | When the wrap tool finishes a phase, the wrap tool SHALL print 1 line that starts with `E5JSON` and holds the phase record as JSON. | SCL_005_ok | Lean consistency done |
| SCL-006 | When the SDK builds the SherlockLedger contract, the SDK SHALL compile the committed source file with `compactc` 0.36.0. | SCL_006_ok | Lean consistency done |
| SCL-007 | If `T10A_TTL_S` is not above 0 and below 1,209,600, then the ledger runner SHALL stop before any transaction is built. | doStart, Inv.ttlOk, Inv.ttlNet, Inv.early, SCL_007_ttl, ttlMax_derived, pin_ttl0, pin_ttl_bound, pin_ttl_ok | Lean done; replay pending |
| SCL-008 | The T23 suite driver SHALL provide the 2 commands `run` and `selftest`. | SCL_008_ok | Lean consistency done |
| SCL-009 | If `SHERLOCK_SDK_ROOT` is not set, then the SDK node test suite SHALL stop with a message that names the variable. | suiteStarts, SCL_009_stops | Lean consistency done |
| SCL-010 | Where a user runs the Sherlock SDK outside the factory, the Sherlock SDK SHALL take the wallet directory, the pinned wallet address, the chain pin and the box roster from the user's configuration. | SCL_010_ok | Lean consistency done |
| OBS-001 | When a deploy or a call is included, the receipts writer SHALL record the contract address, the transaction hash, the block height, the status, the transaction bytes and the paid fee. | doOutcome (receipts), doResolve, pin_resolve | Lean done; replay pending |
| OBS-002 | When the node refuses a call, the receipts writer SHALL record the SHA-256 of the submitted bytes and the JSON-RPC error of the node. | doOutcome (errRecords), pin_refused | Lean done; replay pending |
| OBS-003 | When the ledger runner builds or reads back a call, the ledger runner SHALL scan the call vector, the transcript and the ledger state for the forbidden private words. | scanPass, OBS_003_ok, OBS_003_forbidden_fails, OBS_003_clean_passes | Lean consistency done |
| OBS-004 | If a control word is absent from a scanned surface, then the ledger runner SHALL fail the privacy scan. | scanPass, OBS_004_missing_control_fails | Lean consistency done |
| OBS-005 | When a runner step starts or ends, the Preview runner SHALL append 1 line with a timestamp and the step name to `runs.log`. | runsLogLines, OBS_005_lines | Lean consistency done |
| OBS-006 | When the `check` step runs, the Preview runner SHALL write the SHA-256 of each copied input file, of the `L_Fr` key and of each binary to `manifest.sha256`. | OBS_006_ok | Lean consistency done |
| OBS-007 | When a suite call is proved, the T23 suite driver SHALL record the box name and the seconds of the 5 stages leaf, node, `L`, `L_Fr` and wrap. | OBS_007_ok | Lean consistency done |
| OBS-008 | If wallet balancing fails, then the ledger runner SHALL record the wallet stage and the whole error chain in `calls.<tag>.walletAttempts`. | doBalance (errRecords), pin_attempts | Lean done; replay pending |
| OBS-009 | When a RESULTS file reports a figure, the RESULTS file SHALL label the figure as MEASURED, DERIVED or ESTIMATE. | labelOfSum, OBS_009_ok, OBS_009_sum_of_measured | Lean consistency done |
| OBS-010 | When the integration record changes, the tested-design loop SHALL run `openspec validate --strict` on every Sherlock change. | OBS_010_ok, loop_consistent | Lean consistency done |
| OBS-011 | When a lane starts, ends, decides or measures, the tested-design loop SHALL append 1 entry to `llm-wiki/wiki/log.md`. | OBS_011_ok, loop_consistent | Lean consistency done |
| OBS-012 | The prover GUI page SHALL show the proof stages, the program source and the schedule of each box for a run in 2 modes, live from the daemon bridge and replay from a recorded run. | OBS_012_ok, LPC_011_ok | Lean consistency done |
| FRC-001 | When a runner step ends with exit status 0, the Preview runner SHALL write the marker file `<step>.ok` in the state folder. | markerWritten, skipStep, FRC_001_marker | Lean consistency done |
| FRC-002 | If a deploy was sent and not confirmed, then the deploy runner SHALL read the recorded address back from the indexer and send no second deploy. | doDeploy (deployPending), Inv.oneDeploy, Inv.deployPhase, FRC_002_one_deploy, pin_deploy_resume | Lean done; replay pending |
| FRC-003 | While a proven, unsubmitted and unrefused call has more than 5 minutes of TTL left, the ledger runner SHALL reuse the saved call on a rerun. | doRerun, FRC_003_reuse, pin_reuse, pin_no_reuse_300 | Lean done; replay pending |
| FRC-004 | If wallet balancing fails with a transport error, then the ledger runner SHALL make at most 4 attempts in total, 30 s apart, with a wallet resync before each new attempt. | doBalance, Inv.attempts, FRC_004_attempts, pin_attempts, pin_resync, pin_other_error | Lean done; replay pending |
| FRC-005 | If the node refuses the honest call, then the ledger runner SHALL mark the saved call as refused and prove a new call on the next run. | doOutcome (savedRefused), doRerun, Inv.refusedNotReused, FRC_005_no_reuse_of_refused, pin_refused | Lean done; replay pending |
| FRC-006 | If the readback of the honest call is absent, then the Preview runner SHALL refuse to start the `refusals` step. | doRefusals, Inv.refusalsAfterReadback, FRC_006_refusals_need_readback, pin_no_readback | Lean done; replay pending |
| FRC-007 | When a run is planned to last more than 30 minutes, the tested-design loop SHALL start the run as a `systemd-run --user` unit with an append-mode log file. | FRC_007_ok | Lean consistency done |
| FRC-008 | When muninn starts after a crash, the tested-design loop SHALL restart each killed run from its resumable state as a new user unit. | FRC_008_ok, loop_consistent | Lean consistency done |
| FRC-009 | If a gate fails 2 times in a row, then the tested-design loop SHALL stop that row and keep the other rows running. | FRC_009_ok | Lean consistency done |
| FRC-010 | When a row reaches DONE, the tested-design loop SHALL hold every file of the row in a commit on the row's branch. | FRC_010_ok, loop_consistent | Lean consistency done |
| FRC-011 | If a push names a branch other than `t10q` before the owner signs T15, then the tested-design loop SHALL hold the push. | holdPush, FRC_011_ok, FRC_011_LPC_015_agree | Lean consistency done |
| FRC-012 | When the T23 settle run starts again after a stop, the T23 suite driver SHALL skip each call whose `runs[claim_pub]` readback equals its `outcome_pub`. | toSettle, FRC_012_no_call_twice, FRC_012_scenario | Lean consistency done |
| FRC-013 | If the inclusion state of a submitted call is unknown, then the ledger runner SHALL resolve the state from the node and the indexer before the ledger runner builds another submission of the same claim. | enabled (submit), doResolve, Inv.flight, FRC_013_no_submit_while_unknown, pin_timeout, pin_resolve | Lean done; replay pending |
| FRC-014 | When the ledger runner reuses a saved proof or state file, the ledger runner SHALL check the SHA-256 of the file, the deployed key and the genesis hash against the run state before the reuse. | doRerun (hashOk), FRC_014_hash_checked, pin_no_reuse_hash | Lean done; replay pending |
| FRC-015 | When a lane tip merges into the integration branch, the tested-design loop SHALL record a true result of `git merge-base --is-ancestor` for the tip and the pass result of each required suite on the merged tree. | FRC_015_ok, loop_consistent | Lean consistency done |
| LPC-008 | When T11 and T12 pass, the prototype runner SHALL run the honest call and the refusals on Preview with the prover on huginn and a second program. | LPC_008_ok, LPC_008_with_PVW_010 | Lean consistency done |
| LPC-009 | When T17 (b) passes, the validium runner SHALL settle at least 2 batches on Preview and refuse an invalid batch and a stale root. | doBatch, pin_validium | Lean done; replay pending |
| LPC-010 | When T10q closes, the tested-design loop SHALL merge `t20-batched` 5dc1be2ee into the integration branch. | LPC_010_ok | Lean consistency done |
| LPC-011 | When the five build lanes merge, the prover GUI SHALL run live against the T14b daemon and in replay mode. | LPC_011_ok | Lean consistency done |
| LPC-015 | If a push targets a branch other than `t10q` before T15, then the tested-design loop SHALL hold the push. | LPC_015_ok, FRC_011_LPC_015_agree | Lean consistency done |
| LPC-020 | If a box queue names a row that LOOP.md does not list, then the lead SHALL add the row to LOOP.md. | LPC_020_ok, loop_consistent | Lean consistency done |
| SWF-009 | The 2026-11-08 amendment (`MORIARTY-COMPLETION-PROGRAM.md:270-283`) supersedes the `:269` mainnet ban. While any of its conditions is unmet (local Docker at 9.1.0.1 passed, Preview qualification receipts committed, Preview rollback rehearsal receipt committed, pre-deploy checklist passed), no mainnet operation SHALL run. | SWF_009_ok | Lean consistency done |
| ZK-002 | The published view of a call SHALL be the call vector and one blinded wrap proof. The used log count, the deferred-or-emulated split, the shard plan, the AIR set and heights, and plain `prog_d` SHALL NOT be in that view. | ZK_002_ok | Lean consistency done |
| ZK-007 | The anchor lane SHALL carry `C_P` or a lock value `A_j`. Plain `prog_d` SHALL NOT reach the ledger through the anchor (ANC-001). | ZK_007_ok | Lean consistency done |
