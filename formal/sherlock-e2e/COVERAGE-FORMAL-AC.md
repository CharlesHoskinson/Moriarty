# Formal coverage of slices A and C and the completion change (Formal-Lean-AC)

One row per EARS ID of the scope of Formal-Lean-AC: slice C (11 specs), slice A (6 specs),
change sherlock-completion-20261115 (62 IDs) and the reused IDs that those specs adopt
(inventory: `formal/sherlock-e2e/scripts/inventory_ac.py`). Verdicts per FORMAL-BRIEF.md.
MODELED rows are final only when the Quint replay agrees; until then the Quint/replay column
reads `pending`. Completion IDs that Formal-Quint placed in a slice B context are listed in
the last section with a pointer to COVERAGE-FORMAL-B.md (no verdict here, so no double row).

## Totals

| verdict | rows |
|---|---|
| MODELED | 211 |
| CONSTANT-CHECKED | 21 |
| SPEC-CHECKED | 204 |
| FINDING | 14 |
| total with a verdict here | 450 |
| completion IDs placed in slice B contexts (pointer rows) | 41 |

## Slice C (sherlock-e2e-prototype-20261115)

| ID | group | context | verdict | Lean artifact | Quint/replay | source / reason |
|---|---|---|---|---|---|---|
| WRP-001 | C | wrap | MODELED | `Wrap.Inv.oneProof, Inv.writtenOneProof, WRP_001_one_proof; pin_written` | pending (Formal-Quint wrap lane) | one proof per call; proofs <= 1 over every run |
| WRP-002 | C | wrap | MODELED | `Wrap.Inv.kOk, Inv.stopBeforeKeygen, WRP_002_never_k22; pin_k22, pin_k21; kMax_is_21` | pending (Formal-Quint wrap lane) | bound 21 accepted, 22 stops before keygen (stop code 1) |
| WRP-003 | C | wrap | MODELED | `Wrap.Inv.rowsOk, WRP_003_row_gate; pin_rows_over, pin_rows_gate; measured_t10m_rows` | pending (Formal-Quint wrap lane) | gate 1,782,573 (= rowGate_derived); bound and bound+1 pinned; T10m 1,685,845 rows LOOP.md:31 |
| WRP-004 | C | wrap | MODELED | `Wrap.Inv.vkOk, Inv.noFileAfterStop, WRP_004_vk_bound; pin_vk_over, pin_vk_bound; measured_t10m_vk` | pending (Formal-Quint wrap lane) | 50,000 accepted, 50,001 stops without wrap.vk; T10m VK 1,325 B LOOP.md:31 |
| WRP-006 | C | wrap | MODELED | `Wrap.Inv.fresh, WRP_006_fresh_blinders; pin_blinders, pin_seeded` | pending (Formal-Quint wrap lane) | blinder = CSPRNG draw counter; test-seed build repeats (ZK-014 mode) |
| WRP-007 | C | wrap | MODELED | `Wrap.Inv.decodeOk, WRP_007_anchor_slot; pin_slot0, pin_slot3` | pending (Formal-Quint wrap lane) | lanes 6 and 15 must hold 15, else stop code 5 |
| WRP-008 | C | wrap | MODELED | `Wrap.Inv.noFileAfterStop, Inv.writtenChecked; pin_keydiff` | pending (Formal-Quint wrap lane) | key differs -> stop code 4, files 0 |
| WRP-009 | C | wrap | MODELED | `Wrap.Inv.filesOk, WRP_009_three_files; pin_written, pin_verify_fail` | pending (Formal-Quint wrap lane) | files in {0,3}; written only after verifier accepts |
| WRP-010 | C | wrap | MODELED | `Wrap.Inv.oneKey, WRP_010_one_key; pin_second_key` | pending (Formal-Quint wrap lane) | second key in an era -> lastRefusal 9; keysDeployed <= 1 |
| WRP-011 | C | wrap | MODELED | `Wrap.Inv.gpuKey, WRP_011_gpu_key; pin_gpu_other, pin_gpu_same` | pending (Formal-Quint wrap lane) | GPU run with another key -> lastRefusal 10 |
| WRP-012 | C | wrap | MODELED | `Wrap.Inv.deployQual, Inv.qualRange, WRP_012_deploy_qualified; pin_deploy_unqualified, pin_deploy_four_bits, pin_deploy_qualified` | pending (Formal-Quint wrap lane) | deploy needs all 5 bits (LW-003, -004, -005, -006, -014) |
| LVR-001 | C | ledger-verification | MODELED | `Spec.apply` check branch, `Inv.exitZero`, `LVR_005_exit_status`, pin `pin_unpinned` | pending (Formal-Quint) | LEAD PICK: a check whose manifest gate did not pass exits nonzero (stock verifier = pinned crates) |
| LVR-002 | C | ledger-verification | MODELED | `Inv.gateCrates`, `LVR_002_gate`, pins `pin_gate_*` | pending (Formal-Quint) | the gate is MISSING in code (spec status) |
| LVR-003 | C | ledger-verification | MODELED | `Inv.keyNotG4`, `LVR_003_no_g4_key`, pin `pin_g4` | pending (Formal-Quint) | see ORACLE.md |
| LVR-004 | C | ledger-verification | MODELED | `Inv.controlsPerCheck`, `LVR_004_ten_controls`, `lvr004_lanes` (7 lanes 0, 10, 20, 34, 39, 513, 1025 distinct, 7 + 3 = 10) | pending (Formal-Quint) | see ORACLE.md |
| LVR-005 | C | ledger-verification | MODELED | `Inv.exitZero`, `LVR_005_exit_status`, pins `pin_lane_passed`, `pin_flip_passed`, `pin_honest_refused` | pending (Formal-Quint) | see ORACLE.md |
| LVR-006 | C | ledger-verification | MODELED | `Inv.submitAfterNative`, `LVR_006_native_before_submit`, pin `pin_submit_before_native` | pending (Formal-Quint) | see ORACLE.md |
| LVR-007 | C | ledger-verification | MODELED | `Inv.nativeSplit`, `LVR_007_refused_not_submitted`, `LVR_007_step`, pins `pin_native_refused`, `pin_reprove` | pending (Formal-Quint) | see ORACLE.md |
| LVR-008 | C | ledger-verification | MODELED | `Inv.genesisSet`, `LVR_008_genesis_stable`, pins `pin_first_connect`, `pin_other_chain`, `pin_same_chain` | pending (Formal-Quint) | LEAD PICK: a reconnect at the same genesis keeps the first version (the later-version record is STL-006's task per spec status) |
| SLC-001 | C | settlement | MODELED | `Inv.deployShape`, `SLC_001_004_deploy_shape`, `Consistency.SLC_001_ok`, `SLC_001_matches_oracle`, pin `pin_deploy` | pending (Formal-Quint) | t20-batched form (LEAD PICK: the adopted LDG text as amended on t20-batched, see FAL-SETTLEMENT-1) |
| SLC-002 | C | settlement | MODELED | `SLC_002_anchor_slot`, `SLC_002_anchor_constructor_only` | pending (Formal-Quint) | the compiler refusal of an entry that writes slot 15 is slice A |
| SLC-003 | C | settlement | MODELED | `Inv.deployShape` (keyRun = keyColl = wrap key) | pending (Formal-Quint) | see ORACLE.md |
| SLC-004 | C | settlement | MODELED | `Inv.deployShape` (keyRoot = keyRunRead = native key) | pending (Formal-Quint) | see ORACLE.md |
| SLC-005 | C | settlement | MODELED | `SLC_005_other_program`, pin `pin_other_program` | pending (Formal-Quint) | node custom error 115 = refusal bit 4 |
| SLC-006 | C | settlement | SPEC-CHECKED | `Consistency.SLC_006_ok`; `buildArgs` op writes the file | n/a | a data form (4 words, acceptRun takes the first 3); no state beyond the argument file |
| SLC-007 | C | settlement | SPEC-CHECKED | `Consistency.SLC_007_ok`, `SLC_007_changed_effect` | n/a | a data form of the outcome |
| SLC-008 | C | settlement | SPEC-CHECKED | `Consistency.SLC_008_ok` | n/a | a data form of the input (fuel 0, stack 0, 8-word salt); consistent with ZK-006 |
| SLC-009 | C | settlement | MODELED | `Inv.argsUsed`, pin `pin_mismatch`, `pin_no_args` | pending (Formal-Quint) | see ORACLE.md |
| SLC-010 | C | settlement | MODELED | `SLC_010_native_refused`, pin `pin_native` | pending (Formal-Quint) | see ORACLE.md |
| SLC-011 | C | settlement | MODELED | `apply` replayStart branch, pin `pin_reuse` | pending (Formal-Quint) | see ORACLE.md |
| SLC-012 | C | settlement | MODELED | `SLC_012_unbound_refused`, pin `pin_unbound` | pending (Formal-Quint) | see ORACLE.md |
| BAT-001 | C | settlement | MODELED | `Inv.epochBounds`, `Inv.onePerEpoch`, `LDG_006_one_write_per_epoch`, pins `pin_epoch`, `pin_no_overrun`, `pin_no_early_close` | pending (Formal-Quint) | LEAD PICK: the clock (tick) cannot pass an open epoch's close time; close is enabled at the close time or at 1,048,576 claims |
| BAT-002 | C | settlement | MODELED | `BAT_002_operator_no_acceptRun`, pin `pin_operator` | pending (Formal-Quint) | the contract cannot tell an operator (spec status); the runner routes |
| BAT-003 | C | settlement | CONSTANT-CHECKED | `bat003_rates`, `Consistency.BAT_003_ok` | n/a | 492 calls/day, 24 tx/day, 163,128 B and 3,337,236 B, 291 jobs/s (t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:11,13,14,54,56); MEASURED 6,783 B, ESTIMATE 6,797 B |
| BAT-004 | C | settlement | CONSTANT-CHECKED | `bat004_counts` | n/a | 208 - 62 - 1 = 145 acceptRun, at most 106 deploys (/home/hoskinson/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md:45) |
| BAT-005 | C | settlement | MODELED | `planRefuses`, `BAT_005_cap`, `Inv.planCap`, `bat005_pins`, `kmax_*`, pins `pin_plan`, `pin_short_ttl` | pending (Formal-Quint) | LEAD PICK: K_MAX = min(ANC-012 formula, 2) |
| BAT-006 | C | settlement | MODELED | `BAT_006_default_ttl`, pin `pin_ttl` | pending (Formal-Quint) | see ORACLE.md |
| BAT-007 | C | settlement | MODELED | `Inv.membership` (pathsGiven = jobsClosed), pin `pin_epoch` | pending (Formal-Quint) | see ORACLE.md |
| BAT-008 | C | settlement | MODELED | `Inv.membership` (leafBytes = 32 x jobsClosed), pin `pin_epoch` | pending (Formal-Quint) | see ORACLE.md |
| BAT-009 | C | settlement | MODELED | `Inv.formFirst`, pin `pin_no_form` | pending (Formal-Quint) | see ORACLE.md |
| BAT-010 | C | settlement | FINDING | `deadline`, `BAT_010_close_before_era`, pin `pin_era_close`, `Consistency.finding_bat010_boundary` | n/a (finding) | FAL-SETTLEMENT-3: closing at end_eff - proof - inclusion lands at end_eff, where ERT-004 accepts nothing |
| BAT-011 | C | settlement | FINDING | `BAT_011_era_refusal`, pin `pin_era_run`, `Consistency.finding_bat011_boundary` | n/a (finding) | FAL-SETTLEMENT-3: a run ending exactly at end_eff is admitted (refused only after) but ERT-004 is not active at end_eff |
| BAT-012 | C | settlement | MODELED | `Inv.freezeOpen`, pin `pin_freeze_open` | pending (Formal-Quint) | see ORACLE.md |
| EGV-001 | C | governance | MODELED | `Op.deploy` (readback args), `EGV_002_table_readback`, `pin_threshold2`, `pin_table_first` | pending (Formal-Quint) | deploy op: the runner deploys `{[], 1, 0}`; the op's arguments are the readback (spec: e2e/sherlock-e2e-governance) |
| EGV-002 | C | governance | MODELED | `EGV_002_table_readback`, `EGV_002_ledger_readback`, `ELD_002_ledger_gate`, `pin_threshold0`, `pin_threshold2` | pending (Formal-Quint) | LEAD PICK LP-2: readback threshold exactly 1; FAL-GOVERNANCE-1 (ERT-002 says >= 1) (spec: e2e/sherlock-e2e-governance) |
| EGV-003 | C | governance | MODELED | `EGV_003_path_evidence`, `pin_path_evidence` | pending (Formal-Quint) | reportLedger op; cell pathEvidence (spec: e2e/sherlock-e2e-governance) |
| EGV-004 | C | governance | MODELED | `ERT_003_keyset`, `GOV_002_announce`, `pin_announce`, `pin_keyset_reuse` | pending (Formal-Quint) | the T25 key set is era 1 (key set 7 in the pins); one era recorded; a reused key set is refused (spec: e2e/sherlock-e2e-governance) |
| EGV-005 | C | governance | MODELED | `EGV_005_rekey_gate` (from `inv_run`), `invB_rekeyStart`, `pin_rekey_refused`, `pin_rekey_row`, `pin_rekey_fallback`, `pin_rekey_a1` | pending (Formal-Quint) | InvB field 3; a failed A1 alone does not open the fallback (spec: e2e/sherlock-e2e-governance) |
| EGV-006 | C | governance | MODELED | `CRR_006_check_at_use`, `pin_fallback` | pending (Formal-Quint) | useClaim under fallback checks active at use (spec: e2e/sherlock-e2e-governance) |
| EGV-007 | C | governance | MODELED | `EGV_007_vk_change`, `pin_vk_change` | pending (Formal-Quint) | vkChange op is refused and changes no other cell (spec: e2e/sherlock-e2e-governance) |
| EGV-008 | C | governance | SPEC-CHECKED | `EGV_008_ok`, `consistent`, `EGV_008_refuses_short_rtl` | n/a (no transition) | constrains the T15 freeze record, not a state machine; witness at the T22 recommended values (TL 7 d, RTL = TL, Q 12 of n 16, GRID 3,600 s, SKEW 60 s), none ruled (spec: e2e/sherlock-e2e-governance) |
| EGV-009 | C | governance | MODELED | `EGV_009_premise_once`, `invB_premise`, `pin_premise` | pending (Formal-Quint) | LEAD PICK LP-6: a premise result is written once (spec: e2e/sherlock-e2e-governance) |
| EGV-010 | C | governance | SPEC-CHECKED | `EGV_010_ok`, `consistent`, `EGV_010_refuses_late` | n/a (no transition) | constrains a migration plan (new table and ledgers, predecessor link, before the fork height, no key change on the old contracts); no fork in the oracle (spec: e2e/sherlock-e2e-governance) |
| PVW-001 | C | network-operations | MODELED | doChain, Inv.walletAfterChecks, PVW_001_no_wallet_on_other_chain, pin_chain | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-002 | C | network-operations | MODELED | doChain, Inv.chainOk, PVW_002_two_chains, pin_chain, pin_devnet | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-003 | C | network-operations | SPEC-CHECKED | PVW_003_ok | n/a (consistency only) | pricing source is a tool parameter, no transition; consistent with PVW-002 (chain is devnet or Preview); spec: e2e/sherlock-e2e-preview-deployment |
| PVW-004 | C | network-operations | SPEC-CHECKED | PVW_004_ok | n/a (consistency only) | container and port layout, no transition; 6341 differs from the devnet 6340; spec: e2e/sherlock-e2e-preview-deployment |
| PVW-005 | C | network-operations | SPEC-CHECKED | PVW_005_ok | n/a (consistency only) | command surface of the runner; the 7 steps exist and FRC-006 orders refusals after call; spec: e2e/sherlock-e2e-preview-deployment |
| PVW-006 | C | network-operations | MODELED | doSubmit, Inv.controlsOk, Inv.controlCount, PVW_006_four_controls, pin_pass | pending (Formal-Quint) | trigger guarded by FRC-006 readback (LEAD PICK; FAL-NETOPS-1 GUARD HOLDS, spec text fix proposed); spec: e2e/sherlock-e2e-preview-deployment |
| PVW-007 | C | network-operations | MODELED | doOutcome (k=3), PVW_012_failed_control_fails, pin_noverdict | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-008 | C | network-operations | MODELED | doOutcome (k=2), pin_timeout, pin_ctrl_timeout | pending (Formal-Quint) | 600 s is the outcome code 2; the clock is outside the model; spec: e2e/sherlock-e2e-preview-deployment |
| PVW-009 | C | network-operations | MODELED | doDeploy, Inv.finalGate, pin_pass | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-010 | C | network-operations | MODELED | doProve, Inv.proofHostOk, PVW_010_final_on_gpu, pin_final_cpu, pin_not_owned | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-011 | C | network-operations | MODELED | doBatch, PVW_011_validium_refuses, pin_validium | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-012 | C | network-operations | MODELED | doOutcome (k=1), PVW_012_failed_control_fails, pin_r2 | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
| PVW-013 | C | network-operations | MODELED | doDeploy, Inv.finalGate, PVW_013_final_gate, pin_gate | pending (Formal-Quint) | spec: e2e/sherlock-e2e-preview-deployment |
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
| FRC-003 | C | network-operations | MODELED | doRerun, FRC_003_reuse, pin_reuse, pin_no_reuse_300 | pending (Formal-Quint) | reuse after a resolved non-inclusion is a LEAD PICK (FAL-NETOPS-2); spec: e2e/sherlock-e2e-recovery |
| FRC-004 | C | network-operations | MODELED | doBalance, Inv.attempts, FRC_004_attempts, pin_attempts, pin_resync, pin_other_error | pending (Formal-Quint) | a transport error is retried up to 4 attempts; another wallet error stops at once (LEAD PICK, FAL-NETOPS-3); 30 s spacing is outside the model; spec: e2e/sherlock-e2e-recovery |
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
| HLP-001 | C | placement | MODELED | `Placement.HLP_001_kept_after_ledger_check`, `Inv.keptChecked`, pins `pin_honest`, `pin_refused` | pending (Formal-Quint) | check op: verify before keep (spec: e2e/sherlock-e2e-helper-machines) |
| HLP-002 | C | placement | MODELED | `Placement.HLP_002_no_submit_of_refused`, `Inv.verifierDrop`, `Inv.submittedKept`, pin `pin_refused` | pending (Formal-Quint) | LP-1: submit after a drop records `lastRefusal = 5` and submits nothing (spec: e2e/sherlock-e2e-helper-machines) |
| HLP-003 | C | placement | MODELED | `Placement.HLP_003_four_public_words`, `Inv.result`, pin `pin_honest` | pending (Formal-Quint) | by construction of `receive`: the box result has 4 public words; the oracle cannot observe the argument builder's word values (abstracted) (spec: e2e/sherlock-e2e-helper-machines) |
| HLP-004 | C | placement | MODELED | `Placement.HLP_004_no_private_part`, `Inv.result` | pending (Formal-Quint) | by construction of `receive` (no private field in a box result); the private state of the box is not a cell (spec: e2e/sherlock-e2e-helper-machines) |
| HLP-005 | C | placement | MODELED | `Placement.HLP_005_case_A_only`, `Inv.owned`, `Inv.noHelper`, pins `pin_not_owned`, `pin_send_helper`; `Consistency.HLP_005_ok` | pending (Formal-Quint) | dispatch and sendOffBox refuse a host the user does not own (spec: e2e/sherlock-e2e-helper-machines) |
| HLP-006 | C | placement | SPEC-CHECKED | `Placement.HLP_006_ok`, `Placement.consistent`, `Placement.HLP_006_refuses_edit` | n/a (no state machine) | process rule on how a case-B design is written (a new OpenSpec change on Veritee seald Sessions); checked consistent with HLP-005, ZK-009 and PRD-002 by one witness design (spec: e2e/sherlock-e2e-helper-machines) |
| HLP-007 | C | placement | MODELED | `Placement.HLP_007_no_sign_without_wallet`, `Inv.noWalletSign`, pins `pin_no_wallet`, `pin_wallet` | pending (Formal-Quint) | sign op refuses on a box with no wallet directory (spec: e2e/sherlock-e2e-helper-machines) |
| HLP-008 | C | placement | MODELED | `Placement.HLP_008_binding_before_verify`, `Inv.bindingBeforeVerify`, `Inv.keptChecked`, pins `pin_cv_differs`, `pin_sha_differs` | pending (Formal-Quint) | check op drops on sha or call-vector mismatch before verifying (spec: e2e/sherlock-e2e-helper-machines) |

## Slice A (sherlock-e2e-prototype-20261115)

| ID | group | context | verdict | Lean artifact | Quint/replay | source / reason |
|---|---|---|---|---|---|---|
| ECC-001 | A | compact-compilation | MODELED | `ECC_001_carrier_refused`, `circuitRefusal` (code 1), Consistency `ECC_001_ok`, `cf001_ecc001_conflict` | pending (Formal-Quint) | e2e/sherlock-e2e-compact-compiler; a non-stock input is refused with code 1. See FAL-COMPACT-COMPILATION-1 (CF-001 conflict outside the prototype) |
| ECC-002 | A | compact-compilation | MODELED | `ECC_002_settle_needs_gates`, `Inv.settleGated`, `measured_gates`, Consistency `ECC_002_ok` | pending (Formal-Quint) | gates as six bits (KRS emit, oracle-equal, golden, coverage, mutations, compare); measured 58+4=62 mutations, 104+103=207 compare (mycroft-factory mycroft/factory/REPORT.md:16-18) |
| ECC-003 | A | compact-compilation | MODELED | `ECC_003_one_abi_per_image`, `Inv.okExit`, Consistency `ECC_003_abi_ok` | pending (Formal-Quint) | one image and one ABI per accepted circuit; ABI fields checked as a design predicate |
| ECC-004 | A | compact-compilation | MODELED | `ECC_004_bytes_witness_free`, `Inv.bytes`, pin `pin_bytes` | pending (Formal-Quint) | the bytes are a function of the source; the tape never enters |
| ECC-005 | A | compact-compilation | MODELED | `ECC_005_shape_refused`, `shapes_13`, `circuitRefusal` (code 4) | pending (Formal-Quint) | a form outside the 13 shapes is refused; shape count 13 by `decide` |
| ECC-007 | A | compact-compilation | MODELED | `ECC_007_no_tier3`, `Inv.noTier3`, `Inv.keccakOk` | pending (Formal-Quint) | one keccak chip instruction, never a tier 3 kit. Code today: s02-foreign keccak BLOCKED (compiler/SUITE-COVERAGE.md, FAL-COMPACT-COMPILATION-3) |
| ECC-009 | A | compact-compilation | MODELED | `ECC_009_deploy_slot_15`, `Inv.slot` | pending (Formal-Quint) | param `deploy` = 1 selects slot 15 |
| ECC-010 | A | compact-compilation | MODELED | `ECC_010_image_or_refusal`, `Inv.cover`, `Inv.codeOk`, `measured_circuits`, `measured_suite_calls` | pending (Formal-Quint) | one image or one code per circuit. LEAD PICK: precedence read, version, types, lowering, seal (FAL-COMPACT-COMPILATION-4). Measured counts: FAL-COMPACT-COMPILATION-2 |
| ECC-011 | A | compact-compilation | MODELED | `ECC_002_settle_needs_gates` (oracle-equal gate bit 1), `measured_oracle_equal` | pending (Formal-Quint) | oracle-equal 104/104 and Quint 1,226/1,226 (REPORT.md:10,14), T10m-era vectors |
| ECC-012 | A | compact-compilation | MODELED | `ECC_012_exit_zero_writes_all`, `Inv.okExit` | pending (Formal-Quint) | exitCode 1 = exit 0 with images = abis = accepted |
| ECC-013 | A | compact-compilation | MODELED | `ECC_013_refusal_no_image`, `Inv.refusedExit` | pending (Formal-Quint) | LEAD PICK: no partial output for the request (no image for any circuit of a refused request) |
| ECC-014 | A | compact-compilation | MODELED | `ECC_014_jubjub_tape_words`, `ECC_014_jubjub_admitted`, `Inv.jubjub` | pending (Formal-Quint) | point = two tape words with a curve check, scalar = one word. Code today: j02 verifySchnorrN3 2 calls BLOCKED (FAL-COMPACT-COMPILATION-3) |
| ERS-001 | A | sherlock-execution | CONSTANT-CHECKED | ERS_001_fields; load op (Spec `Op.load`) | n/a | e2e/sherlock-e2e-rsl-statement; 14 distinct fields and schema `rsl-call/1` (spec text); the load op checks presence and type (ERS-014) |
| ERS-002 | A | sherlock-execution | MODELED | Inv.noLeak, ERS_002_private_stays, op send; pins pin_private_out, pin_publics_out | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; a private record sent to a box the user does not own is refused (lastRefusal 3); only StatementPublics leave |
| ERS-003 | A | sherlock-execution | MODELED | Inv.native, ERS_003_public_rows; pin_halt | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; publicRows = 1 + nW (comm first, then W) |
| ERS-005 | A | sherlock-execution | MODELED | Inv.native, ERS_005_native_comm_and_W; pin_halt; measured_ers005 | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; also MEASURED 142 of 142 on odin r5 (spec status, LIST.md:30); 6 kit calls OPEN |
| ERS-006 | A | sherlock-execution | MODELED | Inv.admitted, ERS_006_prog_d; pin_admit | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; prog_d over image, plan, key_root, machine version; code has no prog_d for R_SL yet (status OPEN) |
| ERS-007 | A | sherlock-execution | MODELED | Inv.stmt, ERS_007_input_binds_tape; pin_statement | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; input value digest = tape digest; no R_SL INIT in code (status OPEN) |
| ERS-008 | A | sherlock-execution | MODELED | Inv.stmt, ERS_008_halted_payload; pin_statement | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; Halted, payload vd(comm ++ W) |
| ERS-009 | A | sherlock-execution | MODELED | Inv.rejectedNoStmt, Inv.stmtNotRejectedRsl, ERS_009_rejected_rsl_no_statement; pin_rsl_reject, pin_interp_reject | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; modelled as the spec states; tension with GP-001/CPR-005 recorded as FAL-EXEC-4 on the GP-001 row |
| ERS-010 | A | sherlock-execution | MODELED | Inv.stmt, ERS_010_at_most_one_statement, ERS_010_fields | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; at most one statement with exactly 5 fields |
| ERS-011 | A | sherlock-execution | MODELED | ERS_011_box_independent | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; public words of an execution are independent of the box cell |
| ERS-012 | A | sherlock-execution | MODELED | Inv.bound, ERS_012_bound_into_recursion; pin_unbound, pin_bound | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; modelled on the spec; known HIGH gap in code (air.rs:199-203 GAP, FP-1 probe zeros; spec status OPEN), no EARS conflict |
| ERS-013 | A | sherlock-execution | SPEC-CHECKED | ERS_013_ok | n/a (consistency only) | e2e/sherlock-e2e-rsl-statement; one schema for both writers, consistent with ERS-001; tool format, no call state |
| ERS-014 | A | sherlock-execution | MODELED | op load, Inv.stopIff; pin_missing_field, pin_wrong_type | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; stop 1 with refusedField = the field index |
| ERS-015 | A | sherlock-execution | MODELED | Inv.admitted, ERS_015_app_key_under_root; pin_app_key | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-statement; per-program app key committed under key_root |
| EKT-001 | A | sherlock-execution | CONSTANT-CHECKED | EKT_001_sixteen_rules | n/a | e2e/sherlock-e2e-rsl-kits; 16 distinct rule names (status compile.rs:73-90) |
| EKT-002 | A | sherlock-execution | CONSTANT-CHECKED | EKT_002_air_bucket | n/a | e2e/sherlock-e2e-rsl-kits; 5/8 app AIRs, 6/9 with FP-1; status MAX_APP_AIRS = 8 (trace.rs:49) |
| EKT-003 | A | sherlock-execution | SPEC-CHECKED | EKT_003_ok | n/a (consistency only) | e2e/sherlock-e2e-rsl-kits; compile-time lowering table, no call state |
| EKT-004 | A | sherlock-execution | MODELED | Inv.admitted, EKT_004_no_mixed_hash, EKT_004_mixed_outside_bucket; pin_mixed_hash | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-kits; refusal only for the R_SL family (pin_mixed_interp) |
| EKT-005 | A | sherlock-execution | SPEC-CHECKED | EKT_005_ok | n/a (consistency only) | e2e/sherlock-e2e-rsl-kits; sponge form; consistent with MYC-BLT-001 |
| EKT-006 | A | sherlock-execution | SPEC-CHECKED | EKT_006_ok | n/a (consistency only) | e2e/sherlock-e2e-rsl-kits; test-suite obligation |
| EKT-007 | A | sherlock-execution | MODELED | Inv.kitAccepted, EKT_007_kit_relation; pin_kit | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-kits; app verifier refuses (lastRefusal 1) |
| EKT-009 | A | sherlock-execution | SPEC-CHECKED | EKT_009_ok | n/a (consistency only) | e2e/sherlock-e2e-rsl-kits; interface document obligation |
| EKT-010 | A | sherlock-execution | SPEC-CHECKED | EKT_010_ok | n/a (consistency only) | e2e/sherlock-e2e-rsl-kits; interface document obligation; 7 ops include MYC-BLT-006's 6 |
| EKT-011 | A | sherlock-execution | MODELED | Inv.aluRejects, EKT_011_alu_canonical; pin_alu | pending (Formal-Quint) | e2e/sherlock-e2e-rsl-kits; ALU overflow gives Rejected |
| EUI-001 | A | sherlock-execution | MODELED | Inv.admitted, EUI_001_one_family; pin_admit, pin_version | pending (Formal-Quint) | e2e/sherlock-e2e-universal-interpreter; LEAD PICK: a machine version naming no family stops admission (stop 4) |
| EUI-002 | A | sherlock-execution | MODELED | EUI_003_program_is_data, pin_two_families | pending (Formal-Quint) | e2e/sherlock-e2e-universal-interpreter; both families reach recursion under one ukey |
| EUI-003 | A | sherlock-execution | MODELED | apply_ukey, runOps_ukey, EUI_003_program_is_data | pending (Formal-Quint) | e2e/sherlock-e2e-universal-interpreter; no operation writes the universal key set |
| EUI-004 | A | sherlock-execution | MODELED | Inv.outcomeRange, EUI_004_rsl_two_outcomes; pin_rsl_no_exhausted | pending (Formal-Quint) | e2e/sherlock-e2e-universal-interpreter; R_SL: Halted or Rejected only |
| EUI-005 | A | sherlock-execution | MODELED | Inv.outcomeRange, EUI_005_one_outcome_kind; pin_interp_exhausted | pending (Formal-Quint) | e2e/sherlock-e2e-universal-interpreter; exactly one of Halted, Rejected, Exhausted |
| EUI-006 | A | sherlock-execution | MODELED | Inv.native, ERS_005_native_comm_and_W | pending (Formal-Quint) | e2e/sherlock-e2e-universal-interpreter; executor comm and W equal the reference (abstract native functions) |
| EUI-007 | A | sherlock-execution | MODELED | Inv.proofNeedsTape, EUI_007_no_proof_without_tape; pin_tape_* | pending (Formal-Quint) | e2e/sherlock-e2e-universal-interpreter; each tape defect gives stop 3 and no proof |
| EUI-008 | A | sherlock-execution | MODELED | Inv.static, EUI_008_static_steps; pin_static | pending (Formal-Quint) | e2e/sherlock-e2e-universal-interpreter; microOps = staticOps(image), independent of the tape |
| EUI-009 | A | sherlock-execution | SPEC-CHECKED | EUI_009_ok | n/a (consistency only) | e2e/sherlock-e2e-universal-interpreter; precedence rule between two changes; no state |
| EUI-010 | A | sherlock-execution | CONSTANT-CHECKED | EUI_010_rule_gap | n/a | e2e/sherlock-e2e-universal-interpreter; bound 120 rules; MEASURED built 5 + 2 = 7 rules (EUI-010 status, dec.rs:23-29, :40-41, openvm rekey-chips dcf1b4c): bound NOT met, status TO BUILD (slice B gap) |
| ECS-001 | A | conformance | CONSTANT-CHECKED | `measured_manifest`, Consistency `ECS_001_ok` | n/a | 36 programs = 33 compile + 3 do not (mycroft-factory mycroft/tests/compact-suite/MANIFEST.json schema line 2, compactc line 3, 36 records); no state machine for the setup tool. Key set: none (native) |
| ECS-002 | A | conformance | MODELED | `Inv.rejectedNoWords`, `measured_native_refs`, pin `pin_rejected_ref` | pending (Formal-Quint) | 208 = 146 ok + 62 rejected (RESULTS.md:5) |
| ECS-003 | A | conformance | MODELED | `ECS_003_stage_order`, `Inv.order`, `Inv.stageRan`, `Inv.compileMismatch` | pending (Formal-Quint) | stages compile, prove, settle, compare in order; a compile mismatch skips prove |
| ECS-004 | A | conformance | MODELED | `ECS_004_table_sums`, `Inv.sums`, `Inv.oneClass`, `measured_dryrun_classes` | pending (Formal-Quint) | dry-run classes 0+62+0+145+0+1 = 208 (t23-suite@16836929b T23-DRYRUN.md:13-19, T10m-era tooling) |
| ECS-005 | A | conformance | MODELED | `ECS_005_blocked_reason`, `Inv.reasonOk`, `measured_blocked_reasons` | pending (Formal-Quint) | LEAD PICK: no R_SL chip set is classed before the dispatch check (first blocking stage). 139+3+2+1 = 145 (T23-DRYRUN.md:27-30) |
| ECS-006 | A | conformance | MODELED | `Inv.kitOk`, `measured_kit_blocked`, pins `pin_kit_absent`, `pin_kit_present` | pending (Formal-Quint) | kit-blocked 3+2+1 = 6 (T23-DRYRUN.md:28-30). Current suite: 3 blocked (FAL-COMPACT-COMPILATION-2) |
| ECS-007 | A | conformance | MODELED | `ECS_007_no_dispatch`, `Inv.dispatchOk` | pending (Formal-Quint) | no box message with dispatch off |
| ECS-008 | A | conformance | MODELED | `ECS_008_no_preview`, `Inv.previewOk`, `measured_dryrun_preview` | pending (Formal-Quint) | no Preview tx with preview off; dry run sent 0 (T23-DRYRUN.md:3). See FAL-CONFORMANCE-1 |
| ECS-009 | A | conformance | MODELED | `ECS_009_derived_label`, `Inv.derivedOk`, pin `pin_derived` | pending (Formal-Quint) | DERIVED exactly when no rejection was executed |
| ECS-010 | A | conformance | MODELED | `ECS_010_mismatch_not_done`, `Inv.completeOk`, Consistency `ECS_010_ok` | pending (Formal-Quint) | state part modeled (mismatch keeps the run not complete); table columns as a design predicate |
| ECS-011 | A | conformance | CONSTANT-CHECKED | `measured_counts`, Consistency `ECS_011_ok`, `ecs011_ecs004` | n/a | 33 programs, 106 = 105 + pure h02, 208 calls (mycroft-factory mycroft/factory/REPORT.md:8-9, RESULTS.md:5; also compiler/SUITE-COVERAGE.md:5). No state machine. Key set: none (native/T10m-era dry run). Sources differ in equal/blocked split (FAL-COMPACT-COMPILATION-2) |
| ECS-012 | A | conformance | MODELED | `ECS_012_denominator`, `Inv.denominator`, `measured_dryrun_denominator` | pending (Formal-Quint) | 0 mismatches of 0 compared (T23-DRYRUN.md:21) |
| ECS-013 | A | conformance | MODELED | `ECS_013_one_identity`, `Inv.identity`, pin `pin_duplicate` | pending (Formal-Quint) | a repeated identity is not counted again |
| ECS-014 | A | conformance | MODELED | `ECS_014_complete_needs_evidence`, `Inv.completeOk`, `Inv.evidenceRange`, pins `pin_settle_dry`, `pin_incomplete_report` | pending (Formal-Quint) | seven evidence items per eligible call; a pass without a Preview tx stays incomplete (FAL-CONFORMANCE-1) |
| EUF-001 | A | call-flow | MODELED | `EUF_001_order`, `Op.step` enabled only for stage + 1 | pending (Formal-Quint) | stages 1..11 in order |
| EUF-002 | A | call-flow | MODELED | `EUF_002_one_deploy_per_program`, `Inv.deployedFirst` | pending (Formal-Quint) | state part: deployment before any per-call stage; a second call starts after deploy. The artifact table itself is not a transition |
| EUF-005 | A | call-flow | MODELED | `EUF_005_no_tx_after_stop`, `Inv.stoppedNoTx`, `Inv.txAfterSubmit` | pending (Formal-Quint) | failedStage names the stage |
| EUF-006 | A | call-flow | MODELED | `EUF_006_native_refusal_no_prove`, `Inv.rejected` | pending (Formal-Quint) | stop at execute |
| EUF-007 | A | call-flow | MODELED | `EUF_007_prove_needs_profile`, `Inv.proved`, pin `pin_stale_profile` | pending (Formal-Quint) | prove stops with the key-set pin when the profile is stale |
| EUF-008 | A | call-flow | MODELED | `EUF_008_one_claim`, `Inv.oneClaim`, pin `pin_two_boxes` | pending (Formal-Quint) | claimA = claimB = claim |
| EUF-009 | A | call-flow | MODELED | `EUF_009_one_wrap_key`, `Inv.oneKey`, pin `pin_second_program` | pending (Formal-Quint) | a different key is refused (vkRefused) |
| EUF-010 | A | call-flow | MODELED | `EUF_010_outcome_after_readback`, `Inv.outcomeOk`, pin `pin_readback_differs` | pending (Formal-Quint) | outcome 1 equal, 2 differs |
| EUF-011 | A | call-flow | SPEC-CHECKED | Consistency `EUF_011_ok`, `euf011_today_open` | n/a | no state machine: a deployment property (no agent tooling on the user box). Today open: the EUF-011 status names t7_chain, t17.sh and t23.py on the box bus |
| EUF-012 | A | call-flow | MODELED | `EUF_012_pin_stops`, `Inv.pinNamed`, pin `pin_zkir_pin`, Consistency `EUF_012_ledger_ok` | pending (Formal-Quint) | lowest differing pin named |
| EUF-013 | A | call-flow | MODELED | `EUF_013_complete_joined`, `Inv.completeOk`, pins `pin_complete`, `pin_incomplete`, `pin_mixed` | pending (Formal-Quint) | complete only with every eligible call joined under one key set |
| EUF-014 | A | call-flow | MODELED | `EUF_014_one_tx_per_claim`, `Inv.oneTx`, pins `pin_reconcile`, `pin_no_resend` | pending (Formal-Quint) | reconcile by tx hash; resend refused |

## Completion change (sherlock-completion-20261115), placed in A and C contexts

| ID | group | context | verdict | Lean artifact | Quint/replay | source / reason |
|---|---|---|---|---|---|---|
| CF-013 | completion | completion | MODELED | `Inv.secp`, pin `pin_secp` | pending (Formal-Quint) | a secp256k1 input lowers to the RSL-005 kit (secpKit counter) |
| CF-014 | completion | completion | MODELED | `CF_014_no_kit_refused`, `circuitRefusal` (code 3) | pending (Formal-Quint) | kind 4 (no kit) is refused with the type code |
| LPC-008 | completion | network-operations | SPEC-CHECKED | LPC_008_ok, LPC_008_with_PVW_010 | n/a (consistency only) | loop row; consistent with PVW-010 (huginn is an owned GPU box); spec: completion/sherlock-loop-completion |
| LPC-009 | completion | network-operations | MODELED | doBatch, pin_validium | pending (Formal-Quint) | 2 batches settle, an invalid batch and a stale root are refused (reachable witness); spec: completion/sherlock-loop-completion |
| LPC-010 | completion | network-operations | SPEC-CHECKED | LPC_010_ok | n/a (consistency only) | loop merge row; spec: completion/sherlock-loop-completion |
| LPC-011 | completion | network-operations | SPEC-CHECKED | LPC_011_ok | n/a (consistency only) | loop row; same modes as OBS-012; spec: completion/sherlock-loop-completion |
| LPC-012 | completion | governance | SPEC-CHECKED | `LPC_012_ok`, `consistent` | n/a (process) | a review-process rule, no state machine; the witness has reviewed = false because the Carbon verdict is MISSING on 2026-11-15 (spec status); this oracle does not discharge it (open item) (spec: completion/sherlock-loop-completion) |
| LPC-013 | completion | completion | SPEC-CHECKED | Consistency `LPC_013_ok` | n/a | no state machine: a commit condition on two build results; witness gates pass (REPORT.md:12-18), lake build --wfail green |
| LPC-014 | completion | completion | SPEC-CHECKED | Consistency `LPC_014_ok` | n/a | no state machine: a loop-process condition |
| LPC-015 | completion | network-operations | SPEC-CHECKED | LPC_015_ok, FRC_011_LPC_015_agree | n/a (consistency only) | loop push rule; agrees with FRC-011; spec: completion/sherlock-loop-completion |
| LPC-017 | completion | completion | SPEC-CHECKED | Consistency `LPC_017_ok` | n/a | no state machine: a document-update condition |
| LPC-019 | completion | completion | SPEC-CHECKED | Consistency `LPC_019_ok` | n/a | no state machine: a closing condition on the Mycroft Lean library (S1 discharged or a named hypothesis) |
| LPC-020 | completion | network-operations | SPEC-CHECKED | LPC_020_ok, loop_consistent | n/a (consistency only) | loop process; spec: completion/sherlock-loop-completion |
| RKY-003 | completion | wrap | CONSTANT-CHECKED | `Wrap.measured_t10m_rows, rowGate_derived, dg002_forces_k21` | n/a (constant) | T10m wrap 1,685,845 rows <= 1,782,573 at k = 21 (LOOP.md:31). T25 measurement OPEN (rekey not run); the tool guard is WRP-003 MODELED |
| RKY-004 | completion | wrap | CONSTANT-CHECKED | `Wrap.measured_t10m_vk` | n/a (constant) | T10m VK 1,325 B <= 50,000 (LOOP.md:31). T25 measurement OPEN; guard WRP-004 MODELED |
| STL-001 | completion | settlement | SPEC-CHECKED | `Consistency.STL_001_ok` | n/a | a derivation of the 4 public words; same form as SLC-006 |
| STL-002 | completion | settlement | SPEC-CHECKED | `Consistency.STL_002_ok` | n/a | the prove chain app, leaf, N1, N2, N3, L, L_Fr, wrap at k = 21; proving is slice B |
| STL-003 | completion | settlement | MODELED | `Inv.deployShape`, `SLC_002_anchor_slot` | pending (Formal-Quint) | the trigger (rekey and T12 pass) is outside this context |
| STL-004 | completion | settlement | MODELED | `STL_004_runs_set`, pin `pin_settle` | pending (Formal-Quint) | see ORACLE.md |
| STL-005 | completion | settlement | MODELED | `Inv.mismatchSettled`, pin `pin_stl005` | pending (Formal-Quint) | see ORACLE.md |
| STL-006 | completion | settlement | SPEC-CHECKED | `Consistency.STL_006_ok` | n/a | 33 of 36 programs (LOOP.md:45); a run plan, no transition |

## Reused IDs adopted by slice A and C specs

| ID | group | context | verdict | Lean artifact | Quint/replay | source / reason |
|---|---|---|---|---|---|---|
| ANC-001 | reused | wrap | SPEC-CHECKED | `Wrap.ANC_001_ok`, `Wrap.consistent` | n/a (no state machine) | circuit constant and contract-slot content, no wrap-tool transition; slot 15 also MODELED in WRP-007 decode; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| ANC-012 | reused | settlement | FINDING | `chunks`, `kMax`, `planRefuses`, `kmax_default_ttl`, `measured_largest_calls`, `anc012_call_words`, `Consistency.finding_anc012_formula` | n/a (finding) | FAL-SETTLEMENT-2: the formula gives K_MAX 8 at 14,400 s, BAT-005 caps 2; constants 1,015 = 1,024 - 9, 474 and 699 <= 1,015 (t20-batched@5dc1be2ee:...RESULTS.md:116), floor(3,600/1,755) = 2 (RESULTS.md:122) |
| CF-001 | reused | compact-compilation | FINDING | Consistency `CF_001_ok`, `cf001_ecc001_conflict` | n/a | no state machine (a frontend tap choice). Holds for the prototype only by D-016 supersession; outside it CF-001 contradicts ECC-001 (FAL-COMPACT-COMPILATION-1, OPEN DESIGN) |
| CF-002 | reused | call-flow+compact-compilation | SPEC-CHECKED | Consistency `CF_002_ok`, `ECC_003_abi_ok` | n/a | no state machine: the command set and ABI fields as a design record |
| CF-003 | reused | sherlock-execution | MODELED | Inv.native, ERS_005_native_comm_and_W | pending (Formal-Quint) | sherlock-processor-final-20261111/sherlock-compact-frontend; effect vector equals native (abstract) |
| CF-004 | reused | sherlock-execution | MODELED | Inv.static, EUI_008_static_steps | pending (Formal-Quint) | sherlock-processor-final-20261111/sherlock-compact-frontend; N_e static per entry |
| CF-005 | reused | sherlock-execution | CONSTANT-CHECKED | EKT_002_air_bucket | n/a | sherlock-processor-final-20261111/sherlock-compact-frontend; 6 or 9 AIRs with FP-1 |
| CF-012 | reused | compact-compilation | SPEC-CHECKED | Consistency `CF_012_ok` | n/a | no state machine: a report condition on the CF-L1 axiom report |
| CLM-001 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_001_ok` | n/a | node circuit rule (slice B Aggregation owns the AIRs); kinds and domain words distinct |
| CLM-002 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_002_ok` | n/a | node circuit rule; no SEG or JOIN consumed |
| CLM-003 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_003_ok` | n/a | node circuit rule; openings of JOIN or DEP only (in circuit, LDG-004 t20) |
| CLM-004 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_004_ok` | n/a | deps chain keeps order and repetition |
| CLM-005 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_005_ok` | n/a | Rejected/Exhausted only as chain elements |
| CLM-006 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_006_ok` | n/a | build option AD-20 (a); a plan choice |
| CLM-007 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_007_ok` | n/a | DL fields and chain heads LEAF, DEP, FOREIGN |
| CLM-008 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_008_ok`, `SG_ok` | n/a | SG gate constants (2^19, 1,000, 200, 64, 3,000) at the bound; no measured value read here |
| CLM-009 | reused | settlement | SPEC-CHECKED | `Consistency.CLM_009_ok` | n/a | a test obligation (refusal twins) |
| CORE-003 | reused | sherlock-execution | SPEC-CHECKED | CORE_003_ok, CORE_003_no_step_after_outcome | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-chip-core; leaf chip rule; the oracle checks no execute after an outcome |
| CORE-006 | reused | sherlock-execution | SPEC-CHECKED | CORE_006_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-chip-core; leaf public value rules_d; circuit-level |
| CORE-011 | reused | sherlock-execution | SPEC-CHECKED | CORE_011_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-chip-core; sub-kit generation obligation |
| CORE-012 | reused | sherlock-execution | SPEC-CHECKED | CORE_012_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-chip-core; charging equals reference; leaf-level |
| CORE-015 | reused | sherlock-execution | SPEC-CHECKED | CORE_015_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-chip-core; value-set chip set 2 rules; agrees with EUI-010 status |
| CPR-003 | reused | sherlock-execution | MODELED | EUI_003_program_is_data, ERS_015_app_key_under_root | pending (Formal-Quint) | sherlock-processor-final-20261111/sherlock-general-purpose; one universal key set; per-program keys under key_root |
| CPR-004 | reused | wrap | SPEC-CHECKED | `Wrap.CPR_004_ok`, `Wrap.consistent` | n/a (no state machine) | ledger verifier unchanged is an external-system property; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| CPR-005 | reused | sherlock-execution | SPEC-CHECKED | CPR_005_ok | n/a (consistency only) | sherlock-processor-final-20261111/sherlock-general-purpose; no narrowing of GP; see FAL-EXEC-4 |
| CRR-001 | reused | governance | SPEC-CHECKED | `CRR_001_ok`, `consistent` | n/a (no transition) | a consumer's sealed configuration (profile A), not a state machine (spec: t22-governance:sherlock-governance/sherlock-consumer-read) |
| CRR-002 | reused | governance | SPEC-CHECKED | `CRR_002_ok`, `consistent`, `CRR_002_refuses_unsealed` | n/a (no transition) | a consumer's sealed era set (profile B), not a state machine (spec: t22-governance:sherlock-governance/sherlock-consumer-read) |
| CRR-003 | reused | governance | SPEC-CHECKED | `CRR_003_ok`, `consistent` | n/a (no transition) | which ledger a profile A consumer calls (ledgerOf or a usable attested leaf); the oracle has no consumer call graph (spec: t22-governance:sherlock-governance/sherlock-consumer-read) |
| CRR-004 | reused | governance | MODELED | `CRR_004_unsafe_era`, `pin_unsafe` | pending (Formal-Quint) | useClaim of a soundness-ended era sets lastRefusal 13 (spec: t22-governance:sherlock-governance/sherlock-consumer-read) |
| CRR-005 | reused | governance | SPEC-CHECKED | `CRR_005_ok`, `consistent`, `CRR_005_refuses_era_key` | n/a (no transition) | the nullifier's hash inputs; a fixed field list, no state (spec: t22-governance:sherlock-governance/sherlock-consumer-read) |
| CRR-006 | reused | governance | MODELED | `CRR_006_check_at_use`, `pin_fallback` | pending (Formal-Quint) | under a failed A2 or A3 the first use of an inactive era's claim is refused (spec: t22-governance:sherlock-governance/sherlock-consumer-read) |
| CRR-007 | reused | settlement | SPEC-CHECKED | `Consistency.CRR_007_ok` | n/a | consumer writes only its nullifier; consistent with LDG-004 |
| DCD-001 | reused | wrap | SPEC-CHECKED | `Wrap.DCD_001_ok`, `Wrap.consistent` | n/a (no state machine) | statement-shape rule met by LW-007; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| DCD-002 | reused | wrap | SPEC-CHECKED | `Wrap.DCD_002_ok`, `Wrap.consistent` | n/a (no state machine) | circuit composition rule; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| DCD-003 | reused | wrap | SPEC-CHECKED | `Wrap.DCD_003_ok`, `Wrap.consistent` | n/a (no state machine) | hash-argument structure (no n_steps); consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| DCD-004 | reused | wrap | SPEC-CHECKED | `Wrap.DCD_004_ok`, `Wrap.consistent` | n/a (no state machine) | collection statement shape (settlement side, LDG); consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| DCD-005 | reused | wrap | CONSTANT-CHECKED | `Wrap.DCD_005_ok, dg002_forces_k21` | n/a (constant) | same derived gate as DG-002, T10m rows LOOP.md:31 |
| DCD-006 | reused | wrap | SPEC-CHECKED | `Wrap.DCD_006_ok`, `Wrap.consistent` | n/a (no state machine) | root-opening set of L (circuit); consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| DCD-007 | reused | wrap | SPEC-CHECKED | `Wrap.DCD_007_ok`, `Wrap.consistent` | n/a (no state machine) | conditional on AD-20; both options checked (consistent, consistent_ad20_b); consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| DG-001 | reused | wrap | MODELED | `Wrap.Inv.kOk (WRP-002); DG_001_ok, k22_rejected; kMax_is_21` | pending (Formal-Quint wrap lane) | shared with slice B rows (COVERAGE-FORMAL-B.md) |
| DG-002 | reused | wrap | CONSTANT-CHECKED | `Wrap.dg002_forces_k21, wrp003_is_dg002, rowGate_derived` | n/a (constant) | 85% of 2^21-7 = 1,782,573; T10m 1,685,845 rows > gate at k=20 (891,283) so smallest k = 21 (LOOP.md:31); shared with B |
| DG-003 | reused | wrap | MODELED | `Wrap.Inv.vkOk (WRP-004); DG_003_ok, vk_over_rejected; measured_t10m_vk` | pending (Formal-Quint wrap lane) | shared with B |
| DG-010 | reused | wrap | SPEC-CHECKED | `Wrap.DG_010_ok`, `Wrap.consistent` | n/a (no state machine) | process rule (gate failure goes to owner); consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| ELD-001 | reused | governance | MODELED | `Op.deploy` target 1, `EGV_002_ledger_readback`, `pin_threshold0` | pending (Formal-Quint) | era ledger deploy with the readback gate; era_id and table address are the fixed slot of the ledger (no write verb) (spec: t22-governance:sherlock-governance/sherlock-era-ledger) |
| ELD-002 | reused | governance | MODELED | `ELD_002_ledger_gate`, `pin_ledger_not_deployed`, `pin_threshold0` | pending (Formal-Quint) | announce refuses a ledger that has not passed its gate (lastRefusal 7) (spec: t22-governance:sherlock-governance/sherlock-era-ledger) |
| ELD-003 | reused | governance | MODELED | `ELD_004_refuse_call`, `pin_eld_refuse`, `pin_eld_accept` | pending (Formal-Quint) | acceptCall checks active(era_id) at inclusion when A1 to A3 hold (fallback false) (spec: t22-governance:sherlock-governance/sherlock-era-ledger) |
| ELD-004 | reused | governance | MODELED | `ELD_004_refuse_call`, `pin_eld_refuse` | pending (Formal-Quint) | the whole call is refused: claims unchanged (spec: t22-governance:sherlock-governance/sherlock-era-ledger) |
| ELD-005 | reused | settlement | SPEC-CHECKED | `Consistency.ELD_005_ok`; native read keys in `Inv.deployShape` | n/a | read entries keep native keys and never write |
| ELD-006 | reused | governance | CONSTANT-CHECKED | `measured_ELD_006` | n/a (constant) | MEASURED compile only (compactc 0.36.0 `--skip-zk`, no keys): acceptCollection 91 lanes, acceptRun 85 lanes <= L_TX 1,024; t22-governance@fe7207819 research/sherlock-t22-governance-20261114/REPORT.md:10,32; key set: none (compile only, neither T10m nor T25); FAL-GOVERNANCE-3 (scenario says 1,026) (spec: t22-governance:sherlock-governance/sherlock-era-ledger) |
| ELD-007 | reused | governance | MODELED | `ELD_007_fallback_no_check`, `pin_fallback` | pending (Formal-Quint) | under the fallback acceptCall makes no era check (spec: t22-governance:sherlock-governance/sherlock-era-ledger) |
| ERT-001 | reused | governance | MODELED | `Op.deploy` target 0, `ERT_002_table_gate_first`, `pin_table_first` | pending (Formal-Quint) | table deploy once (enabled only while not deployed) (spec: t22-governance:sherlock-governance/sherlock-era-table) |
| ERT-002 | reused | governance | MODELED | `ERT_002_table_gate_first` (from `inv_run`), `EGV_002_table_readback`, `pin_table_first`, `pin_threshold2` | pending (Formal-Quint) | LEAD PICK LP-2 (threshold exactly 1); FAL-GOVERNANCE-1 (spec: t22-governance:sherlock-governance/sherlock-era-table) |
| ERT-003 | reused | governance | MODELED | `ERT_003_keyset`, `ERT_003_ledger`, `pin_keyset_reuse`, `pin_ledger_reuse`, `pin_cancel_holds` | pending (Formal-Quint) | LEAD PICK LP-3: a cancelled era still holds its key set and ledger (spec: t22-governance:sherlock-governance/sherlock-era-table) |
| ERT-004 | reused | settlement | SPEC-CHECKED | `Consistency.ERT_004_ok`, `ERT_004_consistent`, `active`, `endEff`, `finding_bat011_boundary`, `finding_bat010_boundary` | n/a | era activity is the governance context's model; here the half-open interval shows FAL-SETTLEMENT-3 against BAT-010/BAT-011 |
| ERT-005 | reused | governance | MODELED | `pin_handover`, `pin_before_handover`, `pin_end_once` (proof obligation OB-1 open, see ORACLE.md) | pending (Formal-Quint) | one accepting era at a block time; the Lean order-layer invariant is not proved (OB-1); the pins and the Quint replay check it; LEAD PICK LP-1 (FAL-GOVERNANCE-2) (spec: t22-governance:sherlock-governance/sherlock-era-table) |
| ERT-006 | reused | governance | MODELED | `GOV_005_rotation_frame`, `pin_rotate`, `pin_end_once` (OB-2 open) | pending (Formal-Quint) | era records keep ledger, key set and start; no general Lean theorem yet (OB-2) (spec: t22-governance:sherlock-governance/sherlock-era-table) |
| ERT-007 | reused | governance | MODELED | `SlotS` (fin = 0 while pending or current, in `inv_run`), `pin_end_once`, `pin_soundness_due` | pending (Formal-Quint) | end is written at supersession or soundness end only (spec: t22-governance:sherlock-governance/sherlock-era-table) |
| ERT-008 | reused | governance | SPEC-CHECKED | `ERT_008_ok`, `consistent`; reads `active`, `era`, `endEff`, `ledgerState` are pure functions | n/a (no transition) | the five read entries write no state and have distinct keys; a contract entry list, no state machine (spec: t22-governance:sherlock-governance/sherlock-era-table) |
| ERT-009 | reused | settlement | SPEC-CHECKED | `Consistency.ERT_009_ok` | n/a | GRID rule of the governance era table; no transition in this context |
| FREG-001 | reused | settlement | SPEC-CHECKED | `Consistency.FREG_001_ok` | n/a | registry entry form (9 fields, pi_count < 2^32) |
| FREG-002 | reused | settlement | SPEC-CHECKED | `Consistency.FREG_002_ok` | n/a | fclaim domain disjoint from SHK_RUN |
| FREG-003 | reused | settlement | SPEC-CHECKED | `Consistency.FREG_003_ok` | n/a | parsing refuses trailing data |
| FREG-004 | reused | settlement | SPEC-CHECKED | `Consistency.FREG_004_ok` | n/a | no accumulator |
| FREG-005 | reused | settlement | SPEC-CHECKED | `Consistency.FREG_005_ok`, `SG_ok` | n/a | SG gate constants |
| GOV-001 | reused | governance | MODELED | `GOV_001_quorum_announce`, `GOV_001_quorum_cancel`, `GOV_001_quorum_soundnessEnd`, `GOV_001_quorum_rotate`, `pin_quorum` | pending (Formal-Quint) | signature count below Q sets lastRefusal 1 and changes nothing else; signature validity is opaque (a count) (spec: t22-governance:sherlock-governance/sherlock-governance-verbs) |
| GOV-002 | reused | governance | MODELED | `GOV_002_announce`, `GOV_002_one_pending`, `GOV_004_too_early`, `pin_announce`, `pin_too_early` | pending (Formal-Quint) | announce adds era nextId as pending with start = T (spec: t22-governance:sherlock-governance/sherlock-governance-verbs) |
| GOV-003 | reused | governance | MODELED | `GOV_003_activate_at_start`, `GOV_003_one_current`, `pin_activate_early`, `pin_activate`, `pin_end_once` | pending (Formal-Quint) | activate writes end(N) = start(M), current = M, clears pending (spec: t22-governance:sherlock-governance/sherlock-governance-verbs) |
| GOV-004 | reused | governance | MODELED | `GOV_004_too_early`, `GOV_003_activate_at_start`, `pin_too_early`, `pin_soundness` | pending (Formal-Quint) | the accepting era changes only at a start announced TL earlier, or by soundness end (spec: t22-governance:sherlock-governance/sherlock-governance-verbs) |
| GOV-005 | reused | governance | MODELED | `GOV_005_rotation_frame`, `GOV_005_rotation_effect`, `pin_rotate` | pending (Formal-Quint) | rotation in force at now + RTL; no era, ledger, claim or nullifier change (spec: t22-governance:sherlock-governance/sherlock-governance-verbs) |
| GOV-006 | reused | settlement | SPEC-CHECKED | `Consistency.GOV_006_ok`, `GOV_006_consistent`; `eraEnd` cell (announceEra) feeds BAT-010/011 | n/a | the soundness_end verb is the governance context's; here only its end_eff is consumed |
| GOV-007 | reused | governance | MODELED | `GOV_007_ids_in_order` (from `inv_run`), `GOV_007_cancel`, `GOV_007_cancel_late`, `pin_cancel`, `pin_cancel_late` | pending (Formal-Quint) | cancelled id stays below nextId and is never reused (spec: t22-governance:sherlock-governance/sherlock-governance-verbs) |
| GOV-008 | reused | governance | MODELED | `GOV_008_attest_delay`, `pin_attest` | pending (Formal-Quint) | the root is usable from TL after it was recorded; the leaf contents (era_id, C_P, lineage, ledger) are opaque in the oracle (spec: t22-governance:sherlock-governance/sherlock-governance-verbs) |
| GOV-009 | reused | governance | SPEC-CHECKED | `GOV_009_ok`, `consistent`, `EGV_007_vk_change` | n/a (no transition) | the verb list of the table (Op codes 2 to 8), no MaintenanceUpdate, no freeze/seize/pause verb; a property of the entry list (spec: t22-governance:sherlock-governance/sherlock-governance-verbs) |
| GOV-010 | reused | governance | MODELED | `GOV_010_nonce`, `pin_nonce` | pending (Formal-Quint) | every accepted signed verb increments nonce; the message binding (table address, verb, roster_gen) is opaque (spec: t22-governance:sherlock-governance/sherlock-governance-verbs) |
| GOV-011 | reused | governance | MODELED | `GOV_011_cancel_rotation`, `pin_cancel_rotation`, `pin_no_rotation` | pending (Formal-Quint) | cancelRotation drops a pending rotation before it takes effect (spec: t22-governance:sherlock-governance/sherlock-governance-verbs) |
| GP-001 | reused | sherlock-execution | FINDING | pin_rsl_reject vs pin_interp_reject | see finding | sherlock-vm-consensus-20261107/sherlock-general-purpose; FAL-EXEC-4 (OPEN DESIGN): ERS-009/EUI-004 leave an R_SL Rejected outcome unprovable while GP-001 requires every outcome of every program; CPR-005 forbids narrowing |
| GPW-001 | reused | wrap | SPEC-CHECKED | `Wrap.GPW_001_ok`, `Wrap.consistent` | n/a (no state machine) | GPU/CPU path choice; key equality MODELED by WRP-011; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| GPW-002 | reused | wrap | SPEC-CHECKED | `Wrap.GPW_002_ok`, `Wrap.consistent` | n/a (no state machine) | build order of kernels; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| GPW-003 | reused | wrap | SPEC-CHECKED | `Wrap.GPW_003_ok`, `Wrap.consistent` | n/a (no state machine) | byte identity across configurations is a run check; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| GPW-004 | reused | wrap | SPEC-CHECKED | `Wrap.GPW_004_ok`, `Wrap.consistent` | n/a (no state machine) | MSM masking property of the prover; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| GPW-005 | reused | wrap | SPEC-CHECKED | `Wrap.GPW_005_ok`, `Wrap.consistent` | n/a (no state machine) | performance-conditioned build rule; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| GPW-006 | reused | wrap | SPEC-CHECKED | `Wrap.GPW_006_ok`, `Wrap.consistent` | n/a (no state machine) | build/fallback rule, threshold 8 s; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| GPW-007 | reused | wrap | SPEC-CHECKED | `Wrap.GPW_007_ok`, `Wrap.consistent` | n/a (no state machine) | reporting rule (MEASURED needs run record); consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| GPW-008 | reused | wrap | SPEC-CHECKED | `Wrap.GPW_008_ok`, `Wrap.consistent` | n/a (no state machine) | states the absence of a Lean obligation; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| INIT-001 | reused | sherlock-execution | SPEC-CHECKED | INIT_001_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-init-admission; INIT AIR, circuit-level |
| INIT-002 | reused | sherlock-execution | SPEC-CHECKED | INIT_002_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-init-admission; INIT selector, circuit-level |
| INIT-003 | reused | sherlock-execution | MODELED | Inv.stmt, ERS_007_input_binds_tape | pending (Formal-Quint) | sherlock-modular-leaf-20261111/sherlock-init-admission; input_d opens to val_d of the tape |
| INIT-004 | reused | sherlock-execution | SPEC-CHECKED | INIT_004_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-init-admission; genesis memory, circuit-level |
| INIT-005 | reused | sherlock-execution | SPEC-CHECKED | INIT_005_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-init-admission; admission obligations I1..I6 |
| INIT-006 | reused | sherlock-execution | SPEC-CHECKED | INIT_006_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-init-admission; form (b) view_root |
| INIT-007 | reused | sherlock-execution | MODELED | Inv.admitted, ERS_006_prog_d | pending (Formal-Quint) | sherlock-modular-leaf-20261111/sherlock-init-admission; prog_d formula |
| INIT-008 | reused | sherlock-execution | SPEC-CHECKED | INIT_008_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-init-admission; SG-1..SG-6 gate; no R_SL INIT measured |
| LDG-001 | reused | settlement | FINDING | `Consistency.LDG_001_ok` (t20 text); `Inv.deployShape` | n/a (finding) | FAL-SETTLEMENT-1: this branch holds the pre-T20 text (foreign map, verified_roots) that contradicts SLC-001; the oracle models the t20-batched text (LEAD PICK) |
| LDG-002 | reused | settlement | MODELED | `LDG_002_at_most_once`, `BAT_002_operator_no_acceptRun`, `Consistency.LDG_002_args_ok`, pins `pin_replay`, `pin_both_node` | pending (Formal-Quint) | see ORACLE.md |
| LDG-003 | reused | settlement | FINDING | `closeStep`, `collBad`, `Inv.rootsDistinct`, pin `pin_dup_root` | n/a (finding) | FAL-SETTLEMENT-1: in-tree text takes (reg_root, set_root, shape_d); the t20 text (modeled) takes (reg_pub, claims_root) |
| LDG-004 | reused | settlement | FINDING | `Consistency.LDG_004_ok` | n/a (finding) | FAL-SETTLEMENT-1: in-tree text has a descend entry; SLC-001 and the t20 text have none |
| LDG-005 | reused | settlement | FINDING | `Consistency.LDG_005_ok`, `epoch_cap_depth20` | n/a (finding) | FAL-SETTLEMENT-1: in-tree text is an openClaim entry; SLC-001 and the t20 text (claims tree depth 20, modeled) have none |
| LDG-006 | reused | settlement | MODELED | `Inv.onePerEpoch`, `LDG_006_one_write_per_epoch`, pins `pin_epoch`, `pin_empty_epoch` | pending (Formal-Quint) | see ORACLE.md |
| LDG-007 | reused | settlement | SPEC-CHECKED | `Consistency.LDG_007_ok` | n/a | no foreign map; run and foreign leaf tags differ |
| LDG-008 | reused | settlement | MODELED | `collBad` (registry bit 64), pin `pin_registry`, `Consistency.LDG_008_once` | pending (Formal-Quint) | registry epochs are not a transition here: `regWrites` shows one write per changed epoch |
| LW-001 | reused | wrap | SPEC-CHECKED | `Wrap.LW_001_ok`, `Wrap.consistent` | n/a (no state machine) | circuit statement of L; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-002 | reused | wrap | SPEC-CHECKED | `Wrap.LW_002_ok`, `Wrap.consistent` | n/a (no state machine) | circuit statement of L_Fr; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-003 | reused | wrap | CONSTANT-CHECKED | `Wrap.publicInputs_1026, LW_003_ok` | n/a (constant) | 2 + 1,024 = 1,026 public inputs (structural); lane content is WRP-012 qual bit 0 |
| LW-004 | reused | wrap | SPEC-CHECKED | `Wrap.LW_004_ok`, `Wrap.consistent` | n/a (no state machine) | in-circuit log checks; qualification bit 1 MODELED in WRP-012; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-005 | reused | wrap | SPEC-CHECKED | `Wrap.LW_005_ok`, `Wrap.consistent` | n/a (no state machine) | in-circuit anchor checks; qualification bit 2 MODELED in WRP-012; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-006 | reused | wrap | SPEC-CHECKED | `Wrap.LW_006_ok`, `Wrap.consistent` | n/a (no state machine) | in-circuit DAG key commitment; qualification bit 3 MODELED in WRP-012; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-007 | reused | wrap | SPEC-CHECKED | `Wrap.LW_007_ok`, `Wrap.consistent` | n/a (no state machine) | statement shape; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-008 | reused | wrap | SPEC-CHECKED | `Wrap.LW_008_ok`, `Wrap.consistent` | n/a (no state machine) | root-opening rule; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-009 | reused | settlement | FINDING | `Consistency.LW_009_ok` | n/a (finding) | FAL-SETTLEMENT-1: in-tree LW-009 publishes [reg_root, set_root]; the t20 text (modeled) publishes [reg_pub, claims_root] |
| LW-010 | reused | wrap | SPEC-CHECKED | `Wrap.DCD_003_ok`, `Wrap.consistent` | n/a (no state machine) | replay-identity shape (same predicate as DCD-003); consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-011 | reused | wrap | SPEC-CHECKED | `Wrap.DCD_007_ok`, `Wrap.consistent` | n/a (no state machine) | conditional on AD-20; both options checked; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-012 | reused | wrap | MODELED | `Wrap.Inv.fresh, WRP_006_fresh_blinders; LW_012_ok` | pending (Formal-Quint wrap lane) | same transition as WRP-006 |
| LW-013 | reused | wrap | CONSTANT-CHECKED | `Wrap.kMax_is_21, LW_013_ok, k22_rejected` | n/a (constant) | T10m key at k = 21 (LOOP.md:31) <= 21; chip exclusions SPEC-CHECKED in LW_013_ok |
| LW-014 | reused | wrap | SPEC-CHECKED | `Wrap.LW_014_ok`, `Wrap.consistent` | n/a (no state machine) | keygen refusals of L/L_Fr keys; qualification bit 4 MODELED in WRP-012; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-015 | reused | wrap | SPEC-CHECKED | `Wrap.LW_015_ok`, `Wrap.consistent` | n/a (no state machine) | proof-report rule of the L-and-wrap kit; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-016 | reused | wrap | CONSTANT-CHECKED | `Wrap.lw016_t4_rows, LW_016_ok, measured_t4_rows` | n/a (constant) | T4 1,676,222 rows with every check (LOOP.md:21) <= gate; 12,166 words / 5 pairs structural |
| LW-017 | reused | wrap | SPEC-CHECKED | `Wrap.LW_017_ok`, `Wrap.consistent` | n/a (no state machine) | external ledger property; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-018 | reused | wrap | SPEC-CHECKED | `Wrap.LW_018_ok`, `Wrap.consistent` | n/a (no state machine) | L_Fr field and hash parameters; consistent with WRP-001..012 and the other adopted rows on one design (theorem `consistent`) |
| LW-019 | reused | wrap | CONSTANT-CHECKED | `Wrap.measured_d3_weakest, LW_019_ok` | n/a (constant) | D3 weakest 101.0110 bits on T10m keys (LOOP.md:31) >= 100 and >= 101 grindable |
| MID-001 | reused | ledger-verification | MODELED | `apply` loadKey branch (the ledger reader refuses a G4 key), `LVR_003_no_g4_key` | pending (Formal-Quint) | the PLONK/KZG/Blake2b proof form is a constant of the wrap context |
| MID-002 | reused | ledger-verification | FINDING | `Settlement.Consistency.finding_mid002_read_entries` | n/a (finding) | FAL-LEDGER-VERIFICATION-1: MID-002 (every entry holds the wrap key) contradicts SLC-004 (native keys on read entries); spec text unamended |
| MID-003 | reused | ledger-verification | CONSTANT-CHECKED | `measured_vk_bytes`, `mid003_public_inputs`, `measured_tx_bytes`, `Consistency.MID_003_ok` | n/a | VK 1,325 B (t17-preview@9e1e7f398:research/sherlock-t17-preview-20261114/RESULTS.md:17), acceptRun 6,780 B (RESULTS.md:19), 6,783 B and limit 1,048,576 (t20-batched@5dc1be2ee:research/sherlock-t20-batched-20261114/RESULTS.md:11,15); T10m key set, T25 open |
| MID-004 | reused | ledger-verification | SPEC-CHECKED | `Consistency.MID_004_ok`, `consistent`, `MID_004_needs_preview` | n/a | a list of qualification records, no transition; consistent with MID-003 |
| MID-006 | reused | ledger-verification | SPEC-CHECKED | `Consistency.MID_006_ok`, `consistent`, `MID_006_needs_pin` | n/a | a configuration pin (build 2.1.101-bbd616b1), no transition |
| MID-007 | reused | ledger-verification | SPEC-CHECKED | `Consistency.MID_007_ok`, `consistent` | n/a | a statement about obligations; settlement rests on LDG, modeled in formal/sherlock-e2e/settlement |
| MYC-ACC-004 | reused | call-flow | SPEC-CHECKED | Consistency `MYC_ACC_004_ok`, `EUF_012_ledger_ok` | n/a | no state machine: transcript version 2.3 is a pin value (onchain-runtime/src/transcript.rs:89-90 per the sentence) |
| MYC-ACC-007 | reused | sherlock-execution | SPEC-CHECKED | MYC_ACC_007_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/acceptance; suite certification process |
| MYC-ACC-008 | reused | conformance | SPEC-CHECKED | Consistency `MYC_ACC_008_ok` | n/a | no state machine: a labelling property of the report |
| MYC-BLT-001 | reused | sherlock-execution | SPEC-CHECKED | MYC_BLT_001_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/builtins; consistent with EKT-005 |
| MYC-BLT-006 | reused | sherlock-execution | SPEC-CHECKED | MYC_BLT_006_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/builtins; consistent with EKT-010 |
| MYC-BLT-007 | reused | compact-compilation | MODELED | `ECC_007_no_tier3`, `Inv.noTier3`, `Inv.keccakOk` | pending (Formal-Quint) | same transition as ECC-007 |
| MYC-CHIP-001 | reused | sherlock-execution | FINDING | EKT_002_air_bucket, EKT_004_mixed_outside_bucket | see finding | mycroft-factory:mycroft/spec/specs/chipset; FAL-EXEC-3 (OPEN DESIGN, spec text): names one keccak AIR and 'at most 9 AIRs in all'; EKT-002 names KECCAK_REQ and KECCAK_PERM, 10 distinct AIRs in the chip set |
| MYC-CHIP-002 | reused | sherlock-execution | SPEC-CHECKED | MYC_CHIP_002_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/chipset; leaf soundness, circuit-level |
| MYC-CHIP-003 | reused | sherlock-execution | SPEC-CHECKED | MYC_CHIP_003_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/chipset; leaf soundness, circuit-level |
| MYC-CHIP-004 | reused | sherlock-execution | CONSTANT-CHECKED | MYC_CHIP_004_statement_length | n/a | mycroft-factory:mycroft/spec/specs/chipset; 2 + L_TX = 1,026 words; the prototype uses the run form (D-022) |
| MYC-CLM-009 | reused | sherlock-execution | MODELED | Inv.noLeak, ERS_002_private_stays, ZK_008_ok | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/claims; private digests never published |
| MYC-CLM-023 | reused | sherlock-execution | MODELED | Inv.admitted, ERS_006_prog_d | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/claims; prog_d formula |
| MYC-FE-001 | reused | compact-compilation | MODELED | `ECC_001_carrier_refused`, `circuitRefusal` (code 1) | pending (Formal-Quint) | same transition as ECC-001 |
| MYC-FE-002 | reused | call-flow+compact-compilation | MODELED | `MYC_FE_002_ir_version_refused`, `circuitRefusal` (code 2), pin `pin_ir32` | pending (Formal-Quint) | also pinned in mycroft-factory mycroft/lean/Mycroft/Oracle/Tests.lean:96-97 |
| MYC-FE-009 | reused | compact-compilation | MODELED | `ECC_004_bytes_witness_free`, `Inv.bytes` | pending (Formal-Quint) | same source gives the same bytes |
| MYC-FE-010 | reused | compact-compilation | MODELED | `ECC_004_bytes_witness_free`, `Inv.bytes` | pending (Formal-Quint) | bytes depend on source (and configuration) only |
| MYC-FE-011 | reused | compact-compilation | MODELED | `ECC_013_refusal_no_image`, `Inv.refusedExit` | pending (Formal-Quint) | refusal names the cause (`lastRefusal`) and writes no program |
| MYC-FE-013 | reused | sherlock-execution | SPEC-CHECKED | MYC_FE_013_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/frontend; frontend typing/refusal; compile-time, no call state |
| MYC-FE-014 | reused | sherlock-execution | SPEC-CHECKED | MYC_FE_014_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/frontend; frontend typing/refusal; compile-time, no call state |
| MYC-FE-015 | reused | sherlock-execution | SPEC-CHECKED | MYC_FE_015_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/frontend; frontend typing/refusal; compile-time, no call state |
| MYC-FE-016 | reused | sherlock-execution | SPEC-CHECKED | MYC_FE_016_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/frontend; frontend typing/refusal; compile-time, no call state |
| MYC-FE-017 | reused | sherlock-execution | SPEC-CHECKED | MYC_FE_017_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/frontend; frontend typing/refusal; compile-time, no call state |
| MYC-FE-018 | reused | sherlock-execution | SPEC-CHECKED | MYC_FE_018_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/frontend; frontend typing/refusal; compile-time, no call state |
| MYC-FE-019 | reused | sherlock-execution | SPEC-CHECKED | MYC_FE_019_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/frontend; frontend typing/refusal; compile-time, no call state |
| MYC-FE-020 | reused | sherlock-execution | SPEC-CHECKED | MYC_FE_020_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/frontend; frontend typing/refusal; compile-time, no call state |
| MYC-FE-021 | reused | sherlock-execution | SPEC-CHECKED | MYC_FE_021_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/frontend; frontend typing/refusal; compile-time, no call state |
| MYC-KEY-001 | reused | sherlock-execution | MODELED | EUI_003_program_is_data | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/keys; no key changes for any program |
| MYC-KEY-002 | reused | call-flow | MODELED | `EUF_009_one_wrap_key`, `Inv.oneKey` | pending (Formal-Quint) | every deployment stores universalVk |
| MYC-KEY-003 | reused | sherlock-execution | MODELED | ERS_006_prog_d, MYC_KEY_003_prog_d_ignores_ukey | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/keys; prog_d from image, plan, key_root, mv only |
| MYC-KEY-004 | reused | sherlock-execution | MODELED | MYC_KEY_004_prog_d_injective | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/keys; different images give different prog_d (abstract digest domain) |
| MYC-KEY-006 | reused | sherlock-execution | MODELED | ERS_015_app_key_under_root | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/keys; per-program key commitments under key_root |
| MYC-KEY-008 | reused | sherlock-execution | FINDING | MYC_KEY_003_prog_d_ignores_ukey | see finding | mycroft-factory:mycroft/spec/specs/keys; FAL-EXEC-1 (OPEN DESIGN): KEY-008 reports a new prog_d after a universal key change; KEY-003 makes prog_d independent of it unless key_root or machine version changes |
| MYC-LOW-005 | reused | sherlock-execution | MODELED | op execute kind 2, Inv.outcomeRange | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/lowering; a false assert is a Rejected outcome |
| MYC-LOW-006 | reused | sherlock-execution | SPEC-CHECKED | MYC_LOW_006_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/lowering; per-instruction lowering, compiler-level |
| MYC-LOW-007 | reused | sherlock-execution | MODELED | Inv.rejectedNoStmt, ERS_009_rejected_rsl_no_statement | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/lowering; rejected run: no statement, no proof |
| MYC-LOW-009 | reused | sherlock-execution | MODELED | Inv.native | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/lowering; comm equals the native reference |
| MYC-LOW-010 | reused | sherlock-execution | MODELED | Inv.native | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/lowering; halt with the native effect vector and comm |
| MYC-LOW-012 | reused | sherlock-execution | MODELED | Inv.static, EUI_008_static_steps | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/lowering; static counts |
| MYC-LOW-013 | reused | sherlock-execution | SPEC-CHECKED | MYC_LOW_013_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/lowering; fixed rule family, consistent with EUI-003 |
| MYC-LOW-018 | reused | conformance | SPEC-CHECKED | Consistency `MYC_LOW_018_ok` | n/a | no state machine: a record-keeping property of the conformance test |
| MYC-LOW-019 | reused | compact-compilation | SPEC-CHECKED | Consistency `MYC_LOW_019_ok` | n/a | no state machine: a documentation property of the Lean specification |
| MYC-REF-032 | reused | compact-compilation | MODELED | `ECC_001_carrier_refused`, `MYC_FE_002_ir_version_refused` | pending (Formal-Quint) | carrier (code 1) and IR version (code 2) refusals |
| MYC-REF-036 | reused | sherlock-execution | MODELED | op tape flag 2, EUI_007_no_proof_without_tape; pin_tape_noncanonical | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/refusals; non-canonical tape word refused |
| MYC-REF-037 | reused | sherlock-execution | MODELED | op execute kind 2 / alu, EKT_011_alu_canonical | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/refusals; failed run ends Rejected |
| MYC-REF-038 | reused | sherlock-execution | MODELED | op tape flags 1,4,8; pin_tape_selector, pin_tape_missing, pin_tape_leftover | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/refusals; bad tape refused |
| MYC-REF-052 | reused | sherlock-execution | FINDING | ERS_015_app_key_under_root | see finding | mycroft-factory:mycroft/spec/specs/refusals; FAL-EXEC-2 (CONFIRMED on spec text): refuses 'a preprocessing key per program', which the per-program R_SL app key of ERS-015/CF-005 is; e2e status says D-004 supersedes the clause but the MYC text is unchanged |
| MYC-SEAL-005 | reused | compact-compilation | MODELED | `ECC_009_deploy_slot_15`, `Inv.slot` | pending (Formal-Quint) | slot 15 (deploy = 1) or cell 0 (deploy = 0, T18 (c) spike) |
| MYC-SEAL-015 | reused | call-flow | MODELED | `EUF_009_one_wrap_key`, `Inv.oneKey` | pending (Formal-Quint) | one key `vk` as param, with no program input |
| MYC-SEAL-021 | reused | compact-compilation | MODELED | `MYC_SEAL_021_slot_clash_refused`, `circuitRefusal` (code 5), pin `pin_clash` | pending (Formal-Quint) | refused before deploy |
| MYC-TAPE-001 | reused | sherlock-execution | SPEC-CHECKED | MYC_TAPE_001_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/witness-tape; tape layout, compiler-level |
| MYC-TAPE-006 | reused | sherlock-execution | MODELED | op tape flags 4,8; pin_tape_missing, pin_tape_leftover | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/witness-tape; the program rejects, the prover refuses: no proof either way (ERS-009) |
| MYC-TAPE-007 | reused | sherlock-execution | MODELED | op tape flag 1; pin_tape_selector | pending (Formal-Quint) | mycroft-factory:mycroft/spec/specs/witness-tape; wrong selector: no proof |
| MYC-TAPE-008 | reused | sherlock-execution | SPEC-CHECKED | MYC_TAPE_008_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/witness-tape; tape header order |
| MYC-TAPE-010 | reused | sherlock-execution | SPEC-CHECKED | MYC_TAPE_010_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/witness-tape; no extra public word; consistent with ERS-002 |
| MYC-TAPE-012 | reused | sherlock-execution | SPEC-CHECKED | MYC_TAPE_012_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/witness-tape; tape words out of the effect vector |
| MYC-TAPE-015 | reused | sherlock-execution | SPEC-CHECKED | MYC_TAPE_015_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/witness-tape; secp256k1 tape layout |
| MYC-TRN-008 | reused | sherlock-execution | SPEC-CHECKED | MYC_TRN_008_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/transcript; harness reference source |
| MYC-TRN-011 | reused | sherlock-execution | SPEC-CHECKED | MYC_TRN_011_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/transcript; static L_e; agrees with EUI-008 |
| MYC-TRN-014 | reused | sherlock-execution | SPEC-CHECKED | MYC_TRN_014_ok | n/a (consistency only) | mycroft-factory:mycroft/spec/specs/transcript; comm formula; agrees with ERS-005 |
| MYC-TRN-017 | reused | compact-compilation | SPEC-CHECKED | Consistency `MYC_TRN_017_ok` | n/a | no state machine: the three X1 checks and the frame as the only declared difference |
| MYC-VAL-014 | reused | compact-compilation | MODELED | `ECC_014_jubjub_tape_words`, `Inv.jubjub` | pending (Formal-Quint) | a point is x then y, two words |
| PRD-002 | reused | placement | MODELED | `Placement.PRD_002_one_slot_per_wrap`, `Inv.slots`, pins `pin_helper_slot`, `pin_no_slot`, `pin_one_slot`, `pin_export`; `Consistency.PRD_002_ok` | pending (Formal-Quint) | shared with slice B (PDM, D-040: pool TO BUILD); the pool, one-use consumption, no export and helper-slot refusal are modelled (spec: sherlock-processor-final-20261111/sherlock-prover-daemon) |
| PRD-003 | reused | placement | MODELED | `Placement.HLP_005_case_A_only` (`Inv.noHelper`), pin `pin_send_helper` | pending (Formal-Quint) | sendOffBox of a STARK proof, checkpoint or shard plan to a host not owned is refused; owned hosts are the trust domain (LP-2) (spec: sherlock-processor-final-20261111/sherlock-prover-daemon) |
| RFB-001 | reused | sherlock-execution | SPEC-CHECKED | RFB_001_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-reference-bridge; host tool obligation |
| RFB-003 | reused | sherlock-execution | CONSTANT-CHECKED | EUI_010_rule_gap | n/a | sherlock-modular-leaf-20261111/sherlock-reference-bridge; corpus must fire all 120 enabled rules; built 7: bound not met |
| RFB-004 | reused | sherlock-execution | SPEC-CHECKED | RFB_004_ok | n/a (consistency only) | sherlock-modular-leaf-20261111/sherlock-reference-bridge; coverage gate |
| SPL-011 | reused | sherlock-execution | SPEC-CHECKED | SPL_011_ok | n/a (consistency only) | sherlock-processor-final-20261111/sherlock-shard-plan; schedule not in key_root |
| SWF-009 | reused | network-operations | SPEC-CHECKED | SWF_009_ok | n/a (consistency only) | Mainnet conditions; consistent with PVW-002 (the settle runner never targets Mainnet); spec: sherlock-vm-consensus-20261107/sherlock-delivery |
| UIREQ-001 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_001_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change; cross-checked with the oracle's family/outcome constants |
| UIREQ-002 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_002_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-003 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_003_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-004 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_004_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-005 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_005_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-006 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_006_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-007 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_007_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-008 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_008_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-009 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_009_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-010 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_010_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-011 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_011_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change; cross-checked with the oracle's family/outcome constants |
| UIREQ-012 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_012_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change; cross-checked with the oracle's family/outcome constants |
| UIREQ-013 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_013_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-014 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_014_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-015 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_015_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-016 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_016_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change; cross-checked with the oracle's family/outcome constants |
| UIREQ-017 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_017_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-018 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_018_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-019 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_019_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-020 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_020_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-021 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_021_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-022 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_022_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-023 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_023_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-024 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_024_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-025 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_025_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-026 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_026_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-027 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_027_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-028 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_028_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-029 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_029_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-030 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_030_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-031 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_031_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-032 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_032_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-033 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_033_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-034 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_034_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| UIREQ-035 | reused | sherlock-execution | SPEC-CHECKED | UIREQ_035_ok | n/a (consistency only) | sherlock-universal-interpreter-design/universal-interpreter; retained by reference (EUI-009 retention map, D-025); no state machine in this change |
| ZK-001 | reused | placement | MODELED | `Placement.HLP_005_case_A_only` (`Inv.owned`, `Inv.noHelper`) | pending (Formal-Quint) | case A placement part only; the PCS-masking and treatment clauses are proving-layer facts owned by slice B (ZKE); shared row (spec: sherlock-processor-final-20261111/sherlock-zero-knowledge) |
| ZK-002 | reused | network-operations | SPEC-CHECKED | ZK_002_ok | n/a (consistency only) | published view; consistent with WAL-005 and the OBS-003 forbidden list; spec: sherlock-processor-final-20261111/sherlock-zero-knowledge |
| ZK-003 | reused | wrap | MODELED | `Wrap.Inv.fresh, WRP_006_fresh_blinders, pin_seeded` | pending (Formal-Quint wrap lane) | wrap blinders per proof; seeded only under test-seed (testSeed cell) |
| ZK-006 | reused | settlement | SPEC-CHECKED | `Consistency.ZK_006_ok` | n/a | salt form; the CSPRNG draw is slice B (ZKE) |
| ZK-007 | reused | network-operations | SPEC-CHECKED | ZK_007_ok | n/a (consistency only) | anchor content; prog_d is a forbidden scan word; spec: sherlock-processor-final-20261111/sherlock-zero-knowledge |
| ZK-008 | reused | sherlock-execution | SPEC-CHECKED | ZK_008_ok | n/a (consistency only) | sherlock-processor-final-20261111/sherlock-zero-knowledge; n_steps not in the published projection |
| ZK-009 | reused | placement | SPEC-CHECKED | `Placement.ZK_009_ok`, `Placement.consistent`; slot clauses also by `Placement.PRD_002_one_slot_per_wrap` | n/a (design-space rule) | list of designs that are not allowed; one witness design meets it together with HLP-005, HLP-006, PRD-002; shared with slice B (spec: sherlock-processor-final-20261111/sherlock-zero-knowledge) |
| ZK-011 | reused | settlement | SPEC-CHECKED | `Consistency.ZK_011_ok` | n/a | public digest forms, distinct tags |

## Completion IDs placed in slice B contexts

| ID | group | context | verdict | note |
|---|---|---|---|---|
| ANC-013 | completion | bridge | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ANC-014 | completion | bridge | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ANC-015 | completion | bridge | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ANC-016 | completion | bridge | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ANC-017 | completion | bridge | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ANC-018 | completion | bridge | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ANC-019 | completion | bridge | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LW-020 | completion | bridge | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LW-021 | completion | bridge | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LW-022 | completion | bridge | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LPC-001 | completion | aggregation | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LPC-002 | completion | aggregation | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LPC-003 | completion | aggregation | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LPC-004 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LPC-005 | completion | box-fleet | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LPC-006 | completion | scheduling | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LPC-007 | completion | box-fleet | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LPC-016 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| LPC-018 | completion | leaf-proving | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| RKY-001 | completion | key-management | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| RKY-002 | completion | key-management | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| RKY-005 | completion | key-management | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| RSL-001 | completion | leaf-proving | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| RSL-002 | completion | leaf-proving | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| RSL-003 | completion | leaf-proving | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| RSL-004 | completion | leaf-proving | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| RSL-005 | completion | leaf-proving | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| RSL-006 | completion | leaf-proving | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| RSL-007 | completion | leaf-proving | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| RSL-008 | completion | leaf-proving | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| SQG-014 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| SQG-015 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| SQG-016 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ZK-012 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ZK-013 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ZK-014 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ZK-015 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ZK-016 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ZK-017 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ZK-018 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
| ZK-019 | completion | assurance | see COVERAGE-FORMAL-B.md | placed by Formal-Quint (bus/formal-findings.md, context folders table) |
