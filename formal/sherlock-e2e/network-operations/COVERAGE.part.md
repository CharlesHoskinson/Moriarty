# Coverage of network-operations (Formal-Lean-AC)

| ID | group | context | verdict | Lean artifact | Quint/replay | source / reason |
|---|---|---|---|---|---|---|
| PVW-001 | C | network-operations | MODELED | doChain, Inv.walletAfterChecks, PVW_001_no_wallet_on_other_chain, pin_chain | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-002 | C | network-operations | MODELED | doChain, Inv.chainOk, PVW_002_two_chains, pin_chain, pin_devnet | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-003 | C | network-operations | SPEC-CHECKED | PVW_003_ok | n/a (consistency only) | pricing source is a tool parameter, no transition; consistent with PVW-002 (chain is devnet or Preview); spec: e2e/sherlock-e2e-preview-deployment |
| PVW-004 | C | network-operations | SPEC-CHECKED | PVW_004_ok | n/a (consistency only) | container and port layout, no transition; 6341 differs from the devnet 6340; spec: e2e/sherlock-e2e-preview-deployment |
| PVW-005 | C | network-operations | SPEC-CHECKED | PVW_005_ok | n/a (consistency only) | command surface of the runner; the 7 steps exist and FRC-006 orders refusals after call; spec: e2e/sherlock-e2e-preview-deployment |
| PVW-006 | C | network-operations | MODELED | doSubmit, Inv.controlsOk, Inv.controlCount, PVW_006_four_controls, pin_pass | pending (Formal-Quint) | trigger guarded by FRC-006 readback (LEAD PICK; FAL-NETOPS-1 GUARD HOLDS, spec text fix proposed); spec: e2e/sherlock-e2e-preview-deployment; text FIXED: trigger is the readback of the included call |
| PVW-007 | C | network-operations | MODELED | doOutcome (k=3), PVW_012_failed_control_fails, pin_noverdict | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-008 | C | network-operations | MODELED | doOutcome (k=2), pin_timeout, pin_ctrl_timeout | pending (Formal-Quint) | 600 s is the outcome code 2; the clock is outside the model; spec: e2e/sherlock-e2e-preview-deployment |
| PVW-009 | C | network-operations | MODELED | doDeploy, Inv.finalGate, pin_pass | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-010 | C | network-operations | MODELED | doProve, Inv.proofHostOk, PVW_010_final_on_gpu, pin_final_cpu, pin_not_owned | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-011 | C | network-operations | MODELED | doBatch, PVW_011_validium_refuses, pin_validium | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-012 | C | network-operations | MODELED | doOutcome (k=1), PVW_012_failed_control_fails, pin_r2 | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-013 | C | network-operations | MODELED | doDeploy, Inv.finalGate, PVW_013_final_gate, pin_gate | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment; FAL-WRAP-1 FIXED: one gate record holds the 5 WRP-012 passes too |
| WAL-001 | C | network-operations | MODELED | doModes (walletRead), Inv.readPhase, WAL_001_ok, pin_modes | pending (Formal-Quint) | order of the read MODELED; "inside its own process" is WAL_001_ok (SPEC-CHECKED part); spec: e2e/sherlock-e2e-wallet |
| WAL-002 | C | network-operations | MODELED | doLog, Inv.redact, WAL_002_redactions, pin_redact | pending (Formal-Quint) | redaction as a counter; text content is outside the model; spec: e2e/sherlock-e2e-wallet |
| WAL-003 | C | network-operations | MODELED | doAddr, Inv.startedPhase, pin_addr | pending (Formal-Quint) | spec: e2e/sherlock-e2e-wallet |
| WAL-004 | C | network-operations | MODELED | enabled (lock), Inv.lockOk, WAL_004_lock, pin_lock | pending (Formal-Quint) | spec: e2e/sherlock-e2e-wallet |
| WAL-005 | C | network-operations | SPEC-CHECKED | WAL_005_ok, ZK_002_ok | n/a (consistency only) | field selection of a file writer; receipts fields within the allow-list, consistent with ZK-002; spec: e2e/sherlock-e2e-wallet |
| WAL-006 | C | network-operations | SPEC-CHECKED | WAL_006_ok | n/a (consistency only) | balance file content; balance fields disjoint from key material; spec: e2e/sherlock-e2e-wallet |
| WAL-007 | C | network-operations | MODELED | doSubmit (host), pin_offhost | pending (Formal-Quint) | spec: e2e/sherlock-e2e-wallet |
| WAL-008 | C | network-operations | SPEC-CHECKED | WAL_008_ok | n/a (consistency only) | repository content rule, no transition; spec: e2e/sherlock-e2e-wallet |
| WAL-009 | C | network-operations | SPEC-CHECKED | setupCreates, WAL_009_refuses | n/a (consistency only) | separate setup tool; decision function only; spec: e2e/sherlock-e2e-wallet |
| WAL-010 | C | network-operations | MODELED | doModes, Inv.readPhase, pin_modes | pending (Formal-Quint) | spec: e2e/sherlock-e2e-wallet |
| SCL-001 | C | network-operations | SPEC-CHECKED | SCL_001_ok | n/a (consistency only) | command inventory, no transition; spec: e2e/sherlock-e2e-sdk-cli |
| SCL-002 | C | network-operations | MODELED | doFinish, Inv.exitOk, SCL_002_exit_codes, pin_pass, pin_r2, pin_exit1 | pending (Formal-Quint) | a guard stop is an error, exit 1 (LEAD PICK); spec: e2e/sherlock-e2e-sdk-cli |
| SCL-003 | C | network-operations | MODELED | enabled (chainCheck), Inv.dryNoNet, Inv.dryFlags, SCL_003_dry_run, pin_dry | pending (Formal-Quint) | spec: e2e/sherlock-e2e-sdk-cli |
| SCL-004 | C | network-operations | MODELED | doState, Inv.workAfterChecks, pin_state | pending (Formal-Quint) | spec: e2e/sherlock-e2e-sdk-cli |
| SCL-005 | C | network-operations | SPEC-CHECKED | SCL_005_ok | n/a (consistency only) | print format of the wrap tool; 7 keys of the prove record; spec: e2e/sherlock-e2e-sdk-cli |
| SCL-006 | C | network-operations | SPEC-CHECKED | SCL_006_ok | n/a (consistency only) | build tool version pin; spec: e2e/sherlock-e2e-sdk-cli |
| SCL-007 | C | network-operations | MODELED | doStart, Inv.ttlOk, Inv.ttlNet, Inv.early, SCL_007_ttl, ttlMax_derived, pin_ttl0, pin_ttl_bound, pin_ttl_ok | pending (Formal-Quint) | bound 1,209,600 = 14 days (ttlMax_derived); spec: e2e/sherlock-e2e-sdk-cli |
| SCL-008 | C | network-operations | SPEC-CHECKED | SCL_008_ok | n/a (consistency only) | command inventory of the suite driver; spec: e2e/sherlock-e2e-sdk-cli |
| SCL-009 | C | network-operations | SPEC-CHECKED | suiteStarts, SCL_009_stops | n/a (consistency only) | start check of a test suite; spec: e2e/sherlock-e2e-sdk-cli |
| SCL-010 | C | network-operations | SPEC-CHECKED | SCL_010_ok | n/a (consistency only) | configuration sources; 4 user items, 0 factory names; spec: e2e/sherlock-e2e-sdk-cli |
| OBS-001 | C | network-operations | MODELED | doOutcome (receipts), doResolve, pin_resolve | pending (Formal-Quint) | receipt fields as a counter; field content is WAL_005_ok; spec: e2e/sherlock-e2e-observability |
| OBS-002 | C | network-operations | MODELED | doOutcome (errRecords), pin_refused | pending (Formal-Quint) | spec: e2e/sherlock-e2e-observability |
| OBS-003 | C | network-operations | SPEC-CHECKED | scanPass, OBS_003_ok, OBS_003_forbidden_fails, OBS_003_clean_passes | n/a (consistency only) | scan is a pure function over 3 surfaces; spec: e2e/sherlock-e2e-observability |
| OBS-004 | C | network-operations | SPEC-CHECKED | scanPass, OBS_004_missing_control_fails | n/a (consistency only) | scan is a pure function; a missing control word fails it; spec: e2e/sherlock-e2e-observability |
| OBS-005 | C | network-operations | SPEC-CHECKED | runsLogLines, OBS_005_lines | n/a (consistency only) | log line count; consistent with FRC-001 skip lines; spec: e2e/sherlock-e2e-observability |
| OBS-006 | C | network-operations | SPEC-CHECKED | OBS_006_ok | n/a (consistency only) | manifest content; spec: e2e/sherlock-e2e-observability |
| OBS-007 | C | network-operations | SPEC-CHECKED | OBS_007_ok | n/a (consistency only) | 5 stage timings of the suite driver; spec: e2e/sherlock-e2e-observability |
| OBS-008 | C | network-operations | MODELED | doBalance (errRecords), pin_attempts | pending (Formal-Quint) | one record per failed attempt; the error-chain text stays under WAL-002 redaction; spec: e2e/sherlock-e2e-observability |
| OBS-009 | C | network-operations | SPEC-CHECKED | labelOfSum, OBS_009_ok, OBS_009_sum_of_measured | n/a (consistency only) | report labelling rule; spec: e2e/sherlock-e2e-observability |
| OBS-010 | C | network-operations | SPEC-CHECKED | OBS_010_ok, loop_consistent | n/a (consistency only) | loop process; spec: e2e/sherlock-e2e-observability |
| OBS-011 | C | network-operations | SPEC-CHECKED | OBS_011_ok, loop_consistent | n/a (consistency only) | loop process; spec: e2e/sherlock-e2e-observability |
| OBS-012 | C | network-operations | SPEC-CHECKED | OBS_012_ok, LPC_011_ok | n/a (consistency only) | GUI page; same 2 modes as LPC-011; spec: e2e/sherlock-e2e-observability |
| FRC-001 | C | network-operations | SPEC-CHECKED | markerWritten, skipStep, FRC_001_marker | n/a (consistency only) | marker files of the runner; decision functions; spec: e2e/sherlock-e2e-recovery |
| FRC-002 | C | network-operations | MODELED | doDeploy (deployPending), Inv.oneDeploy, Inv.deployPhase, FRC_002_one_deploy, pin_deploy_resume | pending (Formal-Quint) | spec: e2e/sherlock-e2e-recovery |
| FRC-003 | C | network-operations | MODELED | doRerun, FRC_003_reuse, pin_reuse, pin_no_reuse_300 | pending (Formal-Quint) | reuse after a resolved non-inclusion is a LEAD PICK (FAL-NETOPS-2); spec: e2e/sherlock-e2e-recovery; text FIXED: a proven, unrefused call that is not included |
| FRC-004 | C | network-operations | MODELED | doBalance, Inv.attempts, FRC_004_attempts, pin_attempts, pin_resync, pin_other_error | pending (Formal-Quint) | a transport error is retried up to 4 attempts; another wallet error stops at once (LEAD PICK, FAL-NETOPS-3); 30 s spacing is outside the model; spec: e2e/sherlock-e2e-recovery; FAL-NETOPS-3: the other-error rule is FRC-016 |
| FRC-005 | C | network-operations | MODELED | doOutcome (savedRefused), doRerun, Inv.refusedNotReused, FRC_005_no_reuse_of_refused, pin_refused | pending (Formal-Quint) | spec: e2e/sherlock-e2e-recovery |
| FRC-006 | C | network-operations | MODELED | doRefusals, Inv.refusalsAfterReadback, FRC_006_refusals_need_readback, pin_no_readback | pending (Formal-Quint) | spec: e2e/sherlock-e2e-recovery |
| FRC-007 | C | network-operations | SPEC-CHECKED | FRC_007_ok | n/a (consistency only) | loop process (systemd unit for runs over 30 min); spec: e2e/sherlock-e2e-recovery |
| FRC-008 | C | network-operations | SPEC-CHECKED | FRC_008_ok, loop_consistent | n/a (consistency only) | loop process; spec: e2e/sherlock-e2e-recovery |
| FRC-009 | C | network-operations | SPEC-CHECKED | FRC_009_ok | n/a (consistency only) | loop process; spec: e2e/sherlock-e2e-recovery |
| FRC-010 | C | network-operations | SPEC-CHECKED | FRC_010_ok, loop_consistent | n/a (consistency only) | loop process; spec: e2e/sherlock-e2e-recovery |
| FRC-011 | C | network-operations | SPEC-CHECKED | holdPush, FRC_011_ok, FRC_011_LPC_015_agree | n/a (consistency only) | loop push rule; agrees with LPC-015; spec: e2e/sherlock-e2e-recovery |
| FRC-012 | C | network-operations | SPEC-CHECKED | toSettle, FRC_012_no_call_twice, FRC_012_scenario | n/a (consistency only) | suite driver skip as a pure function; no call sent twice; spec: e2e/sherlock-e2e-recovery |
| FRC-013 | C | network-operations | MODELED | enabled (submit), doResolve, Inv.flight, FRC_013_no_submit_while_unknown, pin_timeout, pin_resolve | pending (Formal-Quint) | code diverges, known TO BUILD gap: FAL-NETOPS-4 CONFIRMED; spec: e2e/sherlock-e2e-recovery |
| FRC-014 | C | network-operations | MODELED | doRerun (hashOk), FRC_014_hash_checked, pin_no_reuse_hash | pending (Formal-Quint) | the three checks (hash, key, genesis) are one Bool; key and genesis are also SCL-004 and PVW-001; spec: e2e/sherlock-e2e-recovery |
| FRC-015 | C | network-operations | SPEC-CHECKED | FRC_015_ok, loop_consistent | n/a (consistency only) | loop process; spec: e2e/sherlock-e2e-recovery |
| FRC-016 | C | network-operations | MODELED | doBalance, Inv.attempts, pin_other_error | pending (Formal-Quint) | new for FAL-NETOPS-3 (LEAD PICK): another balancing error records one attempt and stops the call (stop = 7, balanceAttempts = 1); code t17-preview@9e1e7f398 t10a-ledger-run.mjs:764-775; spec: e2e/sherlock-e2e-recovery |
| LPC-008 | completion | network-operations | SPEC-CHECKED | LPC_008_ok, LPC_008_with_PVW_010 | n/a (consistency only) | loop row; consistent with PVW-010 (huginn is an owned GPU box); spec: completion/sherlock-loop-completion |
| LPC-009 | completion | network-operations | MODELED | doBatch, pin_validium | pending (Formal-Quint) | 2 batches settle, an invalid batch and a stale root are refused (reachable witness); spec: completion/sherlock-loop-completion |
| LPC-010 | completion | network-operations | SPEC-CHECKED | LPC_010_ok | n/a (consistency only) | loop merge row; spec: completion/sherlock-loop-completion |
| LPC-011 | completion | network-operations | SPEC-CHECKED | LPC_011_ok | n/a (consistency only) | loop row; same modes as OBS-012; spec: completion/sherlock-loop-completion |
| LPC-015 | completion | network-operations | SPEC-CHECKED | LPC_015_ok, FRC_011_LPC_015_agree | n/a (consistency only) | loop push rule; agrees with FRC-011; spec: completion/sherlock-loop-completion |
| LPC-020 | completion | network-operations | SPEC-CHECKED | LPC_020_ok, loop_consistent | n/a (consistency only) | loop process; spec: completion/sherlock-loop-completion |
| SWF-009 | reused | network-operations | SPEC-CHECKED | SWF_009_ok | n/a (consistency only) | Mainnet conditions; consistent with PVW-002 (the settle runner never targets Mainnet); spec: sherlock-vm-consensus-20261107/sherlock-delivery |
| ZK-002 | reused | network-operations | SPEC-CHECKED | ZK_002_ok | n/a (consistency only) | published view; consistent with WAL-005 and the OBS-003 forbidden list; spec: sherlock-processor-final-20261111/sherlock-zero-knowledge |
| ZK-007 | reused | network-operations | SPEC-CHECKED | ZK_007_ok | n/a (consistency only) | anchor content; prog_d is a forbidden scan word; spec: sherlock-processor-final-20261111/sherlock-zero-knowledge |
